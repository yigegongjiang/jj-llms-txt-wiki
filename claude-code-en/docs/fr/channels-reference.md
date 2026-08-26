> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Référence des canaux

> Créez un serveur MCP qui envoie des webhooks, des alertes et des messages de chat dans une session Claude Code. Référence du contrat de canal : déclaration de capacité, événements de notification, outils de réponse, contrôle de l'expéditeur et relais de permission.

<Note>
  Les canaux sont en [aperçu de recherche](/docs/fr/channels#research-preview). Les organisations Team et Enterprise doivent [les activer explicitement](/docs/fr/channels#enterprise-controls).
</Note>

Un canal est un serveur MCP qui envoie des événements dans une session Claude Code afin que Claude puisse réagir aux choses qui se produisent en dehors du terminal.

Vous pouvez créer un canal unidirectionnel ou bidirectionnel. Les canaux unidirectionnels transmettent les alertes, webhooks ou événements de surveillance pour que Claude agisse. Les canaux bidirectionnels comme les passerelles de chat [exposent également un outil de réponse](#expose-a-reply-tool) afin que Claude puisse renvoyer des messages. Un canal avec un chemin d'expéditeur de confiance peut également opter pour [relayer les invites de permission](#relay-permission-prompts) afin que vous puissiez approuver ou refuser l'utilisation d'outils à distance.

Cette page couvre :

* [Aperçu](#overview) : comment fonctionnent les canaux
* [Ce dont vous avez besoin](#what-you-need) : exigences et étapes générales
* [Exemple : créer un récepteur de webhook](#example-build-a-webhook-receiver) : une procédure pas à pas unidirectionnelle minimale
* [Options du serveur](#server-options) : les champs du constructeur
* [Format de notification](#notification-format) : la charge utile de l'événement et le comportement de livraison
* [Exposer un outil de réponse](#expose-a-reply-tool) : permettre à Claude d'envoyer des messages en retour
* [Contrôler les messages entrants](#gate-inbound-messages) : vérifications de l'expéditeur pour prévenir l'injection de requête
* [Relayer les invites de permission](#relay-permission-prompts) : transmettre les invites d'approbation d'outils aux canaux distants

Pour utiliser un canal existant au lieu d'en créer un, consultez [Canaux](/docs/fr/channels). Telegram, Discord, iMessage et fakechat sont inclus dans l'aperçu de recherche.

<h2 id="overview">
  Aperçu
</h2>

Un canal est un serveur [MCP](https://modelcontextprotocol.io) qui s'exécute sur la même machine que Claude Code. Claude Code le lance en tant que sous-processus et communique via stdio. Votre serveur de canal est le pont entre les systèmes externes et la session Claude Code :

* **Plateformes de chat** (Telegram, Discord) : votre plugin s'exécute localement et interroge l'API de la plateforme pour les nouveaux messages. Quand quelqu'un envoie un message direct à votre bot, le plugin reçoit le message et le transmet à Claude. Aucune URL à exposer.
* **Webhooks** (CI, surveillance) : votre serveur écoute sur un port HTTP local. Les systèmes externes envoient des POST à ce port, et votre serveur envoie la charge utile à Claude.

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-architecture.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=9a037b7da80184ae49015c0256b21a1f" alt="Diagramme d'architecture montrant les systèmes externes se connectant à votre serveur de canal local, qui communique avec Claude Code via stdio" width="600" height="220" data-path="images/channel-architecture.svg" />

<h2 id="what-you-need">
  Ce dont vous avez besoin
</h2>

La seule exigence stricte est le package [`@modelcontextprotocol/sdk`](https://www.npmjs.com/package/@modelcontextprotocol/sdk) et un runtime compatible Node.js. [Bun](https://bun.sh), [Node](https://nodejs.org) et [Deno](https://deno.com) fonctionnent tous. Les plugins pré-construits dans l'aperçu de recherche utilisent Bun, mais votre canal n'a pas besoin de le faire.

Votre serveur doit :

1. Déclarer la capacité `claude/channel` afin que Claude Code enregistre un écouteur de notification
2. Émettre des événements `notifications/claude/channel` quand quelque chose se produit
3. Se connecter via [transport stdio](https://modelcontextprotocol.io/docs/concepts/transports#standard-io) (Claude Code lance votre serveur en tant que sous-processus)

Les sections [Options du serveur](#server-options) et [Format de notification](#notification-format) couvrent chacune de ces points en détail. Consultez [Exemple : créer un récepteur de webhook](#example-build-a-webhook-receiver) pour une procédure pas à pas complète.

Pendant l'aperçu de recherche, les canaux personnalisés ne sont pas sur la [liste d'approbation](/docs/fr/channels#supported-channels). Utilisez `--dangerously-load-development-channels` pour tester localement. Consultez [Tester pendant l'aperçu de recherche](#test-during-the-research-preview) pour plus de détails.

<h2 id="example-build-a-webhook-receiver">
  Exemple : créer un récepteur de webhook
</h2>

Cette procédure pas à pas crée un serveur d'un seul fichier qui écoute les requêtes HTTP et les transmet dans votre session Claude Code. À la fin, tout ce qui peut envoyer un HTTP POST, comme un pipeline CI, une alerte de surveillance ou une commande `curl`, peut envoyer des événements à Claude.

Cet exemple utilise [Bun](https://bun.sh) comme runtime pour son serveur HTTP intégré et son support TypeScript. Vous pouvez utiliser [Node](https://nodejs.org) ou [Deno](https://deno.com) à la place ; la seule exigence est le [SDK MCP](https://www.npmjs.com/package/@modelcontextprotocol/sdk).

<Steps>
  <Step title="Créer le projet">
    Créez un nouveau répertoire et installez le SDK MCP :

    ```bash theme={null}
    mkdir webhook-channel && cd webhook-channel
    bun add @modelcontextprotocol/sdk
    ```
  </Step>

  <Step title="Écrire le serveur de canal">
    Créez un fichier appelé `webhook.ts`. C'est votre serveur de canal entier : il se connecte à Claude Code via stdio, et il écoute les POST HTTP sur le port 8788. Quand une requête arrive, il envoie le corps à Claude en tant qu'événement de canal.

    ```ts title="webhook.ts" theme={null}
    #!/usr/bin/env bun
    import { Server } from '@modelcontextprotocol/sdk/server/index.js'
    import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'

    // Créer le serveur MCP et le déclarer comme canal
    const mcp = new Server(
      { name: 'webhook', version: '0.0.1' },
      {
        // cette clé est ce qui en fait un canal — Claude Code enregistre un écouteur pour elle
        capabilities: { experimental: { 'claude/channel': {} } },
        // ajouté à l'invite système de Claude afin qu'il sache comment gérer ces événements
        instructions: 'Les événements du canal webhook arrivent sous la forme <channel source="webhook" ...>. Ils sont unidirectionnels : lisez-les et agissez, aucune réponse attendue.',
      },
    )

    // Se connecter à Claude Code via stdio (Claude Code lance ce processus)
    await mcp.connect(new StdioServerTransport())

    // Démarrer un serveur HTTP qui transmet chaque POST à Claude
    Bun.serve({
      port: 8788,  // n'importe quel port ouvert fonctionne
      // localhost uniquement : rien en dehors de cette machine ne peut envoyer de POST
      hostname: '127.0.0.1',
      async fetch(req) {
        const body = await req.text()
        await mcp.notification({
          method: 'notifications/claude/channel',
          params: {
            content: body,  // devient le corps de la balise <channel>
            // chaque clé devient un attribut de balise, par ex. <channel path="/" method="POST">
            meta: { path: new URL(req.url).pathname, method: req.method },
          },
        })
        return new Response('ok')
      },
    })
    ```

    Le fichier fait trois choses dans l'ordre :

    * **Configuration du serveur** : crée le serveur MCP avec `claude/channel` dans ses capacités, ce qui indique à Claude Code que c'est un canal. La chaîne [`instructions`](#server-options) va dans l'invite système de Claude : dites à Claude quels événements attendre, s'il faut répondre et comment router les réponses si c'est le cas.
    * **Connexion stdio** : se connecte à Claude Code via stdin/stdout. C'est standard pour tout [serveur MCP](https://modelcontextprotocol.io/docs/concepts/transports#standard-io) : Claude Code le lance en tant que sous-processus.
    * **Écouteur HTTP** : démarre un serveur web local sur le port 8788. Chaque corps POST est transmis à Claude en tant qu'événement de canal via `mcp.notification()`. Le `content` devient le corps de l'événement, et chaque entrée `meta` devient un attribut sur la balise `<channel>`. L'écouteur a besoin d'accès à l'instance `mcp`, donc il s'exécute dans le même processus. Vous pourriez le diviser en modules séparés pour un projet plus grand.
  </Step>

  <Step title="Enregistrer votre serveur avec Claude Code">
    Ajoutez le serveur à votre configuration MCP afin que Claude Code sache comment le démarrer. Pour un `.mcp.json` au niveau du projet dans le même répertoire, utilisez un chemin relatif. Pour la configuration au niveau de l'utilisateur dans `~/.claude.json`, utilisez le chemin absolu complet afin que le serveur puisse être trouvé à partir de n'importe quel projet :

    ```json title=".mcp.json" theme={null}
    {
      "mcpServers": {
        "webhook": { "command": "bun", "args": ["./webhook.ts"] }
      }
    }
    ```

    Claude Code lit votre configuration MCP au démarrage et lance chaque serveur en tant que sous-processus.
  </Step>

  <Step title="Le tester">
    Pendant l'aperçu de recherche, les canaux personnalisés ne sont pas sur la liste d'approbation, donc démarrez Claude Code avec le drapeau de développement :

    ```bash theme={null}
    claude --dangerously-load-development-channels server:webhook
    ```

    La première fois que vous démarrez une session dans ce projet, Claude Code demande le consentement avant d'utiliser le nouveau serveur de `.mcp.json`. La boîte de dialogue signale ' Nouveau serveur MCP trouvé dans ce projet : webhook '. Sélectionnez **Utiliser ce serveur MCP** pour continuer.

    Quand Claude Code démarre, il lit votre configuration MCP, lance votre `webhook.ts` en tant que sous-processus, et l'écouteur HTTP démarre automatiquement sur le port que vous avez configuré (8788 dans cet exemple). Vous n'avez pas besoin de lancer le serveur vous-même.

    Un avis atténué sous la bannière de démarrage confirme que le canal est enregistré : `Channels (experimental) messages from server:webhook inject directly in this session · restart without --dangerously-load-development-channels to stop`.

    Si vous voyez ' bloqué par la politique de l'organisation ', votre administrateur d'organisation doit d'abord [activer les canaux](/docs/fr/channels#enterprise-controls).

    Dans un terminal séparé, simulez un webhook en envoyant un HTTP POST avec un message à votre serveur. Cet exemple envoie une alerte d'échec CI au port 8788 (ou quel que soit le port que vous avez configuré) :

    ```bash theme={null}
    curl -X POST localhost:8788 -d "build failed on main: https://ci.example.com/run/1234"
    ```

    La charge utile arrive dans votre session Claude Code en tant que balise `<channel>` :

    ```text theme={null}
    <channel source="webhook" path="/" method="POST">build failed on main: https://ci.example.com/run/1234</channel>
    ```

    Dans votre terminal Claude Code, vous verrez Claude recevoir le message et commencer à répondre : lire des fichiers, exécuter des commandes ou tout ce que le message demande. C'est un canal unidirectionnel, donc Claude agit dans votre session mais ne renvoie rien via le webhook. Pour ajouter des réponses, consultez [Exposer un outil de réponse](#expose-a-reply-tool).

    Si l'événement n'arrive pas, le diagnostic dépend de ce que `curl` a retourné :

    * **`curl` réussit mais rien n'atteint Claude** : exécutez `/mcp` dans votre session pour vérifier l'état du serveur. « Impossible de se connecter » signifie généralement une erreur de dépendance ou d'importation dans votre fichier serveur ; vérifiez le journal de débogage à `~/.claude/debug/<session-id>.txt` pour la trace stderr.
    * **`curl` échoue avec « connexion refusée »** : le port n'est pas encore lié ou un processus obsolète d'une exécution antérieure le maintient. `lsof -i :<port>` montre ce qui écoute ; `kill` le processus obsolète avant de redémarrer votre session.
  </Step>
</Steps>

Le [serveur fakechat](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/fakechat) étend ce modèle avec une interface web, des pièces jointes et un outil de réponse pour le chat bidirectionnel.

<h2 id="test-during-the-research-preview">
  Tester pendant l'aperçu de recherche
</h2>

Pendant l'aperçu de recherche, chaque canal doit être sur la [liste d'approbation](/docs/fr/channels#research-preview) pour s'enregistrer. Le drapeau de développement contourne la liste d'approbation pour des entrées spécifiques après une invite de confirmation. Cet exemple montre les deux types d'entrée :

```bash theme={null}
# Tester un plugin que vous développez
claude --dangerously-load-development-channels plugin:yourplugin@yourmarketplace

# Tester un serveur .mcp.json nu (pas encore d'enveloppe de plugin)
claude --dangerously-load-development-channels server:webhook
```

Le contournement est par entrée. Combiner ce drapeau avec `--channels` n'étend pas le contournement aux entrées `--channels`. Pendant l'aperçu de recherche, la liste d'approbation est organisée par Anthropic, donc votre canal reste sur le drapeau de développement pendant que vous le construisez et le testez.

<Note>
  Ce drapeau ignore uniquement la liste d'approbation. La politique d'organisation `channelsEnabled` s'applique toujours. Ne l'utilisez pas pour exécuter des canaux de sources non fiables.
</Note>

<h2 id="server-options">
  Options du serveur
</h2>

Un canal définit ces options dans le constructeur [`Server`](https://modelcontextprotocol.io/docs/concepts/servers). Les champs `instructions` et `capabilities.tools` sont [MCP standard](https://modelcontextprotocol.io/docs/concepts/servers) ; `capabilities.experimental['claude/channel']` et `capabilities.experimental['claude/channel/permission']` sont les ajouts spécifiques au canal :

| Champ                                                    | Type     | Description                                                                                                                                                                                                                                                                                                                       |
| :------------------------------------------------------- | :------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `capabilities.experimental['claude/channel']`            | `object` | Requis. Toujours `{}`. La présence enregistre l'écouteur de notification.                                                                                                                                                                                                                                                         |
| `capabilities.experimental['claude/channel/permission']` | `object` | Optionnel. Toujours `{}`. Déclare que ce canal peut recevoir des demandes de relais de permission. Quand déclaré, Claude Code transmet les invites d'approbation d'outils à votre canal afin que vous puissiez les approuver ou les refuser à distance. Consultez [Relayer les invites de permission](#relay-permission-prompts). |
| `capabilities.tools`                                     | `object` | Bidirectionnel uniquement. Toujours `{}`. Capacité d'outil MCP standard. Consultez [Exposer un outil de réponse](#expose-a-reply-tool).                                                                                                                                                                                           |
| `instructions`                                           | `string` | Recommandé. Ajouté à l'invite système de Claude. Dites à Claude quels événements attendre, ce que les attributs de la balise `<channel>` signifient, s'il faut répondre et, le cas échéant, quel outil utiliser et quel attribut repasser (comme `chat_id`).                                                                      |

Pour créer un canal unidirectionnel, omettez `capabilities.tools`. Cet exemple montre une configuration bidirectionnelle avec la capacité de canal, les outils et les instructions définis :

```ts theme={null}
import { Server } from '@modelcontextprotocol/sdk/server/index.js'

const mcp = new Server(
  { name: 'your-channel', version: '0.0.1' },
  {
    capabilities: {
      experimental: { 'claude/channel': {} },  // enregistre l'écouteur de canal
      tools: {},  // omettez pour les canaux unidirectionnels
    },
    // ajouté à l'invite système de Claude afin qu'il sache comment gérer vos événements
    instructions: 'Les messages arrivent sous la forme <channel source="your-channel" ...>. Répondez avec l'outil de réponse.',
  },
)
```

Pour envoyer un événement, appelez `mcp.notification()` avec la méthode `notifications/claude/channel`. Les paramètres sont dans la section suivante.

<h2 id="notification-format">
  Format de notification
</h2>

Votre serveur émet `notifications/claude/channel` avec deux paramètres :

| Champ     | Type                     | Description                                                                                                                                                                                                                                                                                                                                                     |
| :-------- | :----------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content` | `string`                 | Le corps de l'événement. Livré en tant que corps de la balise `<channel>`.                                                                                                                                                                                                                                                                                      |
| `meta`    | `Record<string, string>` | Optionnel. Chaque entrée devient un attribut sur la balise `<channel>` pour le contexte de routage comme l'ID de chat, le nom de l'expéditeur ou la gravité de l'alerte. Les clés doivent être des identifiants : lettres, chiffres et traits de soulignement uniquement. Les clés contenant des tirets ou d'autres caractères sont silencieusement supprimées. |

Votre serveur envoie des événements en appelant `mcp.notification()` sur l'instance `Server`. Cet exemple envoie une alerte d'échec CI avec deux clés meta :

```ts theme={null}
await mcp.notification({
  method: 'notifications/claude/channel',
  params: {
    content: 'build failed on main: https://ci.example.com/run/1234',
    meta: { severity: 'high', run_id: '1234' },
  },
})
```

L'événement arrive dans le contexte de Claude enveloppé dans une balise `<channel>`. L'attribut `source` est défini automatiquement à partir du nom configuré de votre serveur :

```text theme={null}
<channel source="your-channel" severity="high" run_id="1234">
build failed on main: https://ci.example.com/run/1234
</channel>
```

Les notifications ne sont pas reconnues. L'`await` sur `mcp.notification()` se résout quand le message est écrit au transport, pas quand Claude l'a traité. Si la session n'a pas chargé votre serveur en tant que canal, ou si la politique d'organisation le bloque, les événements sont supprimés silencieusement sans erreur retournée à votre serveur.

Si vous avez besoin d'une confirmation de livraison, suivez l'état de l'événement dans votre serveur et exposez un [outil de réponse](#expose-a-reply-tool) que Claude peut appeler pour signaler le statut en retour.

Les événements s'accumulent dans la session et sont traités dans l'ordre. Si plusieurs notifications arrivent pendant que Claude est occupé, elles sont livrées ensemble au prochain tour et Claude les traite comme un groupe. Pour traiter les flux d'événements indépendants en parallèle, exécutez des sessions séparées.

<h2 id="expose-a-reply-tool">
  Exposer un outil de réponse
</h2>

Si votre canal est bidirectionnel, comme une passerelle de chat plutôt qu'un transmetteur d'alerte, exposez un [outil MCP](https://modelcontextprotocol.io/docs/concepts/tools) standard que Claude peut appeler pour envoyer des messages en retour. Rien dans l'enregistrement de l'outil n'est spécifique au canal. Un outil de réponse a trois composants :

1. Une entrée `tools: {}` dans les capacités du constructeur `Server` afin que Claude Code découvre l'outil
2. Des gestionnaires d'outils qui définissent le schéma de l'outil et implémentent la logique d'envoi
3. Une chaîne `instructions` dans le constructeur `Server` qui indique à Claude quand et comment appeler l'outil

Pour ajouter ceux-ci au [récepteur de webhook ci-dessus](#example-build-a-webhook-receiver) :

<Steps>
  <Step title="Activer la découverte d'outils">
    Dans votre constructeur `Server` dans `webhook.ts`, ajoutez `tools: {}` aux capacités afin que Claude Code sache que votre serveur offre des outils :

    ```ts theme={null}
    capabilities: {
      experimental: { 'claude/channel': {} },
      tools: {},  // active la découverte d'outils
    },
    ```
  </Step>

  <Step title="Enregistrer l'outil de réponse">
    Ajoutez ce qui suit à `webhook.ts`. L'`import` va en haut du fichier avec vos autres imports ; les deux gestionnaires vont entre le constructeur `Server` et `mcp.connect()`. Cela enregistre un outil `reply` que Claude peut appeler avec un `chat_id` et un `text` :

    ```ts theme={null}
    // Ajoutez cet import en haut de webhook.ts
    import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'

    // Claude interroge ceci au démarrage pour découvrir quels outils votre serveur offre
    mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
      tools: [{
        name: 'reply',
        description: 'Envoyer un message en retour sur ce canal',
        // inputSchema indique à Claude quels arguments passer
        inputSchema: {
          type: 'object',
          properties: {
            chat_id: { type: 'string', description: 'La conversation dans laquelle répondre' },
            text: { type: 'string', description: 'Le message à envoyer' },
          },
          required: ['chat_id', 'text'],
        },
      }],
    }))

    // Claude appelle ceci quand il veut invoquer un outil
    mcp.setRequestHandler(CallToolRequestSchema, async req => {
      if (req.params.name === 'reply') {
        const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
        // send() est votre sortie : POST à votre plateforme de chat, ou pour les tests locaux
        // la diffusion SSE montrée dans l'exemple complet ci-dessous.
        send(`Reply to ${chat_id}: ${text}`)
        return { content: [{ type: 'text', text: 'sent' }] }
      }
      throw new Error(`unknown tool: ${req.params.name}`)
    })
    ```
  </Step>

  <Step title="Mettre à jour les instructions">
    Mettez à jour la chaîne `instructions` dans votre constructeur `Server` afin que Claude sache router les réponses via l'outil. Cet exemple indique à Claude de passer `chat_id` à partir de la balise entrante :

    ```ts theme={null}
    instructions: 'Les messages arrivent sous la forme <channel source="webhook" chat_id="...">. Répondez avec l'outil de réponse, en passant le chat_id de la balise.'
    ```
  </Step>
</Steps>

Voici le `webhook.ts` complet avec support bidirectionnel. Les réponses sortantes se diffusent via `GET /events` en utilisant [Server-Sent Events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events) (SSE), donc `curl -N localhost:8788/events` peut les regarder en direct ; le chat entrant arrive sur `POST /` :

```ts title="webhook.ts complet avec outil de réponse' expandable theme={null}
#!/usr/bin/env bun
import { Server } from '@modelcontextprotocol/sdk/server/index.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'

// --- Sortant : écrire à n'importe quel écouteur curl -N sur /events ---
// Un vrai pont enverrait un POST à votre plateforme de chat à la place.
const listeners = new Set<(chunk: string) => void>()
function send(text: string) {
  const chunk = text.split('\n').map(l => `data: ${l}\n`).join('') + '\n'
  for (const emit of listeners) emit(chunk)
}

const mcp = new Server(
  { name: 'webhook', version: '0.0.1' },
  {
    capabilities: {
      experimental: { 'claude/channel': {} },
      tools: {},
    },
    instructions: 'Les messages arrivent sous la forme <channel source="webhook" chat_id="...">. Répondez avec l'outil de réponse, en passant le chat_id de la balise.',
  },
)

mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [{
    name: 'reply',
    description: 'Envoyer un message en retour sur ce canal',
    inputSchema: {
      type: 'object',
      properties: {
        chat_id: { type: 'string', description: 'La conversation dans laquelle répondre' },
        text: { type: 'string', description: 'Le message à envoyer' },
      },
      required: ['chat_id', 'text'],
    },
  }],
}))

mcp.setRequestHandler(CallToolRequestSchema, async req => {
  if (req.params.name === 'reply') {
    const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
    send(`Reply to ${chat_id}: ${text}`)
    return { content: [{ type: 'text', text: 'sent' }] }
  }
  throw new Error(`unknown tool: ${req.params.name}`)
})

await mcp.connect(new StdioServerTransport())

let nextId = 1
Bun.serve({
  port: 8788,
  hostname: '127.0.0.1',
  idleTimeout: 0,  // ne pas fermer les flux SSE inactifs
  async fetch(req) {
    const url = new URL(req.url)

    // GET /events : flux SSE afin que curl -N puisse regarder les réponses de Claude en direct
    if (req.method === 'GET' && url.pathname === '/events') {
      const stream = new ReadableStream({
        start(ctrl) {
          ctrl.enqueue(': connected\n\n')  // afin que curl affiche quelque chose immédiatement
          const emit = (chunk: string) => ctrl.enqueue(chunk)
          listeners.add(emit)
          req.signal.addEventListener('abort', () => listeners.delete(emit))
        },
      })
      return new Response(stream, {
        headers: { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-cache' },
      })
    }

    // POST : transmettre à Claude en tant qu'événement de canal
    const body = await req.text()
    const chat_id = String(nextId++)
    await mcp.notification({
      method: 'notifications/claude/channel',
      params: {
        content: body,
        meta: { chat_id, path: url.pathname, method: req.method },
      },
    })
    return new Response('ok')
  },
})
```

Le [serveur fakechat](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/fakechat) montre un exemple plus complet avec pièces jointes et édition de messages.

<h2 id="gate-inbound-messages">
  Contrôler les messages entrants
</h2>

Un canal non contrôlé est un vecteur d'injection de requête. Quiconque peut atteindre votre point de terminaison peut mettre du texte devant Claude. Un canal écoutant une plateforme de chat ou un point de terminaison public a besoin d'une vérification d'expéditeur réelle avant d'émettre quoi que ce soit.

Vérifiez l'expéditeur par rapport à une liste d'approbation avant d'appeler `mcp.notification()`. Cet exemple supprime tout message d'un expéditeur qui n'est pas dans l'ensemble :

```ts theme={null}
const allowed = new Set(loadAllowlist())  // à partir de votre access.json ou équivalent

// à l'intérieur de votre gestionnaire de messages, avant d'émettre :
if (!allowed.has(message.from.id)) {  // expéditeur, pas salle
  return  // supprimer silencieusement
}
await mcp.notification({ ... })
```

Contrôlez sur l'identité de l'expéditeur, pas l'identité du chat ou de la salle : `message.from.id` dans l'exemple, pas `message.chat.id`. Dans les chats de groupe, ceux-ci diffèrent, et contrôler sur la salle laisserait quiconque dans un groupe approuvé injecter des messages dans la session.

Les canaux [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram) et [Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) contrôlent sur une liste d'approbation d'expéditeur de la même manière. Ils amorçent la liste par appairage : l'utilisateur envoie un message direct au bot, le bot répond avec un code d'appairage, l'utilisateur l'approuve dans sa session Claude Code, et son ID de plateforme est ajouté. Consultez l'une ou l'autre implémentation pour le flux d'appairage complet. Le canal [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage) adopte une approche différente : il détecte les propres adresses de l'utilisateur à partir de la base de données Messages au démarrage et les laisse passer automatiquement, avec d'autres expéditeurs ajoutés par poignée.

<h2 id="relay-permission-prompts">
  Relayer les invites de permission
</h2>

Quand Claude appelle un outil qui a besoin d'approbation, la boîte de dialogue du terminal local s'ouvre et la session attend. Un canal bidirectionnel peut opter pour recevoir la même invite en parallèle et la relayer vers vous sur un autre appareil. Les deux restent actifs : vous pouvez répondre dans le terminal ou sur votre téléphone, et Claude Code applique la réponse qui arrive en premier et ferme l'autre.

Le relais couvre les approbations d'utilisation d'outils comme `Bash`, `Write` et `Edit`. La confiance du projet et les boîtes de dialogue de consentement du serveur MCP ne relaient pas ; celles-ci n'apparaissent que dans le terminal local.

<h3 id="how-relay-works">
  Comment fonctionne le relais
</h3>

Quand une invite de permission s'ouvre, la boucle de relais a quatre étapes :

1. Claude Code génère un court ID de demande et notifie votre serveur
2. Votre serveur transmet l'invite et l'ID à votre application de chat
3. L'utilisateur distant répond par oui ou non et cet ID
4. Votre gestionnaire entrant analyse la réponse en un verdict, et Claude Code l'applique uniquement si l'ID correspond à une demande ouverte

La boîte de dialogue du terminal local reste ouverte pendant tout cela. Si quelqu'un au terminal répond avant l'arrivée du verdict distant, cette réponse est appliquée à la place et la demande distante en attente est supprimée.

<img src="https://mintcdn.com/claude-code/9FG0ZKj9uKYiHmbi/images/channel-permission-relay.svg?fit=max&auto=format&n=9FG0ZKj9uKYiHmbi&q=85&s=97d57f128f0da55f105ab1e3a7e10240" alt="Diagramme de séquence : Claude Code envoie une notification permission_request au serveur de canal, le serveur formate et envoie l'invite à l'application de chat, l'humain répond avec un verdict, et le serveur analyse cette réponse en une notification de permission vers Claude Code" width="600" height="230" data-path="images/channel-permission-relay.svg" />

<h3 id="permission-request-fields">
  Champs de demande de permission
</h3>

La notification sortante de Claude Code est `notifications/claude/channel/permission_request`. Comme la [notification de canal](#notification-format), le transport est MCP standard mais la méthode et le schéma sont des extensions Claude Code. L'objet `params` a quatre champs de chaîne que votre serveur formate dans l'invite sortante :

| Champ           | Description                                                                                                                                                                                                                                                                                                                                                                                                                |
| --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `request_id`    | Cinq lettres minuscules tirées de `a`-`z` sans `l`, donc cela ne se lit jamais comme un `1` ou un `I` quand tapé sur un téléphone. Incluez-le dans votre invite sortante afin qu'il puisse être répété dans la réponse. Claude Code n'accepte un verdict que s'il porte un ID qu'il a émis. La boîte de dialogue du terminal local n'affiche pas cet ID, donc votre gestionnaire sortant est le seul moyen de l'apprendre. |
| `tool_name`     | Nom de l'outil que Claude veut utiliser, par exemple `Bash` ou `Write`.                                                                                                                                                                                                                                                                                                                                                    |
| `description`   | Résumé lisible par l'homme de ce que cet appel d'outil spécifique fait, le même texte que la boîte de dialogue du terminal local affiche. Pour un appel Bash c'est la description de Claude de la commande, ou la commande elle-même si aucune n'a été donnée.                                                                                                                                                             |
| `input_preview` | Les arguments de l'outil sous forme de chaîne JSON, tronqués à 200 caractères. Pour Bash c'est la commande ; pour Write c'est le chemin du fichier et un préfixe du contenu. Omettez-le de votre invite si vous n'avez de la place que pour un message d'une ligne. Votre serveur décide ce qu'il faut afficher.                                                                                                           |

Le verdict que votre serveur renvoie est `notifications/claude/channel/permission` avec deux champs : `request_id` répétant l'ID ci-dessus, et `behavior` défini sur `'allow'` ou `'deny'`. Allow laisse l'appel d'outil procéder ; deny le rejette, comme répondre Non dans la boîte de dialogue locale. Aucun verdict n'affecte les appels futurs.

<h3 id="add-relay-to-a-chat-bridge">
  Ajouter le relais à une passerelle de chat
</h3>

Ajouter le relais de permission à un canal bidirectionnel prend trois composants :

1. Une entrée `claude/channel/permission: {}` sous les capacités `experimental` dans votre constructeur `Server` afin que Claude Code sache transmettre les invites
2. Un gestionnaire de notification pour `notifications/claude/channel/permission_request` qui formate l'invite et l'envoie via votre API de plateforme
3. Une vérification dans votre gestionnaire de message entrant qui reconnaît `yes <id>` ou `no <id>` et émet une notification de verdict `notifications/claude/channel/permission` au lieu de transmettre le texte à Claude

Déclarez uniquement la capacité si votre canal [authentifie l'expéditeur](#gate-inbound-messages), car quiconque peut répondre via votre canal peut approuver ou refuser l'utilisation d'outils dans votre session.

Pour ajouter ceux-ci à une passerelle de chat bidirectionnelle comme celle assemblée dans [Exposer un outil de réponse](#expose-a-reply-tool) :

<Steps>
  <Step title="Déclarer la capacité de permission">
    Dans votre constructeur `Server`, ajoutez `claude/channel/permission: {}` à côté de `claude/channel` sous `experimental` :

    ```ts theme={null}
    capabilities: {
      experimental: {
        'claude/channel': {},
        'claude/channel/permission': {},  // opter pour le relais de permission
      },
      tools: {},
    },
    ```
  </Step>

  <Step title="Gérer la demande entrante">
    Enregistrez un gestionnaire de notification entre votre constructeur `Server` et `mcp.connect()`. Claude Code l'appelle avec les [quatre champs de demande](#permission-request-fields) quand une boîte de dialogue de permission s'ouvre. Votre gestionnaire formate l'invite pour votre plateforme et inclut des instructions pour répondre avec l'ID :

    ```ts theme={null}
    import { z } from 'zod'

    // setNotificationHandler route par z.literal sur le champ method,
    // donc ce schéma est à la fois le validateur et la clé de dispatch
    const PermissionRequestSchema = z.object({
      method: z.literal('notifications/claude/channel/permission_request'),
      params: z.object({
        request_id: z.string(),     // cinq lettres minuscules, inclure verbatim dans votre invite
        tool_name: z.string(),      // par ex. "Bash", "Write"
        description: z.string(),    // résumé lisible par l'homme de cet appel
        input_preview: z.string(),  // arguments d'outil en JSON, tronqués à ~200 caractères
      }),
    })

    mcp.setNotificationHandler(PermissionRequestSchema, async ({ params }) => {
      // send() est votre sortie : POST à votre plateforme de chat, ou pour les tests locaux
      // la diffusion SSE montrée dans l'exemple complet ci-dessous.
      send(
        `Claude wants to run ${params.tool_name}: ${params.description}\n\n` +
        // l'ID dans l'instruction est ce que votre gestionnaire entrant analyse à l'étape 3
        `Reply "yes ${params.request_id}" or "no ${params.request_id}"`,
      )
    })
    ```
  </Step>

  <Step title="Intercepter le verdict dans votre gestionnaire entrant">
    Votre gestionnaire entrant est la boucle ou le rappel qui reçoit les messages de votre plateforme : le même endroit où vous [contrôlez sur l'expéditeur](#gate-inbound-messages) et émettez `notifications/claude/channel` pour transmettre le chat à Claude. Ajoutez une vérification avant l'appel de transmission de chat qui reconnaît le format du verdict et émet la notification de permission à la place.

    La regex correspond au format d'ID que Claude Code génère : cinq lettres, jamais `l`. Le drapeau `/i` tolère la correction automatique du téléphone en mettant en majuscules la réponse ; mettez en minuscules l'ID capturé avant de le renvoyer.

    ```ts theme={null}
    // correspond à "y abcde", "yes abcde", "n abcde", "no abcde"
    // [a-km-z] est l'alphabet d'ID que Claude Code utilise (minuscules, ignore 'l')
    // /i tolère la correction automatique du téléphone ; mettez en minuscules la capture avant d'envoyer
    const PERMISSION_REPLY_RE = /^\s*(y|yes|n|no)\s+([a-km-z]{5})\s*$/i

    async function onInbound(message: PlatformMessage) {
      if (!allowed.has(message.from.id)) return  // contrôler sur l'expéditeur d'abord

      const m = PERMISSION_REPLY_RE.exec(message.text)
      if (m) {
        // m[1] est le mot du verdict, m[2] est l'ID de demande
        // émettre la notification du verdict vers Claude Code au lieu du chat
        await mcp.notification({
          method: 'notifications/claude/channel/permission',
          params: {
            request_id: m[2].toLowerCase(),  // normaliser en cas de majuscules de correction automatique
            behavior: m[1].toLowerCase().startsWith('y') ? 'allow' : 'deny',
          },
        })
        return  // géré comme verdict, ne pas aussi transmettre comme chat
      }

      // ne correspondait pas au format du verdict : passer au chemin de chat normal
      await mcp.notification({
        method: 'notifications/claude/channel',
        params: { content: message.text, meta: { chat_id: String(message.chat.id) } },
      })
    }
    ```
  </Step>
</Steps>

Claude Code garde également la boîte de dialogue du terminal local ouverte, donc vous pouvez répondre dans l'un ou l'autre endroit, et la première réponse à arriver est appliquée. Une réponse distante qui ne correspond pas exactement au format attendu échoue de l'une de deux façons, et dans les deux cas la boîte de dialogue reste ouverte :

* **Format différent** : la regex de votre gestionnaire entrant ne correspond pas, donc du texte comme `approve it` ou `yes` sans ID tombe comme un message normal à Claude.
* **Format correct, ID incorrect** : votre serveur émet un verdict, mais Claude Code ne trouve aucune demande ouverte avec cet ID et le supprime silencieusement.

<h3 id="full-example">
  Exemple complet
</h3>

Le `webhook.ts` assemblé ci-dessous combine les trois extensions de cette page : l'outil de réponse, le contrôle de l'expéditeur et le relais de permission. Si vous commencez ici, vous aurez également besoin de la [configuration du projet et de l'entrée `.mcp.json`](#example-build-a-webhook-receiver) de la procédure pas à pas initiale.

Pour rendre les deux directions testables à partir de curl, l'écouteur HTTP sert deux chemins :

* **`GET /events`** : maintient un flux SSE ouvert et envoie chaque message sortant en tant que ligne `data:`, donc `curl -N` peut regarder les réponses et les invites de permission de Claude arriver en direct.
* **`POST /`** : le côté entrant, le même gestionnaire qu'avant, maintenant avec la vérification du format du verdict insérée avant la branche de transmission de chat.

```ts title="webhook.ts complet avec relais de permission' expandable theme={null}
#!/usr/bin/env bun
import { Server } from '@modelcontextprotocol/sdk/server/index.js'
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js'
import { ListToolsRequestSchema, CallToolRequestSchema } from '@modelcontextprotocol/sdk/types.js'
import { z } from 'zod'

// --- Sortant : écrire à n'importe quel écouteur curl -N sur /events ---
// Un vrai pont enverrait un POST à votre plateforme de chat à la place.
const listeners = new Set<(chunk: string) => void>()
function send(text: string) {
  const chunk = text.split('\n').map(l => `data: ${l}\n`).join('') + '\n'
  for (const emit of listeners) emit(chunk)
}

// Liste d'approbation de l'expéditeur. Pour la procédure pas à pas locale nous faisons confiance à la valeur d'en-tête X-Sender unique
// "dev" ; un vrai pont vérifierait l'ID utilisateur de la plateforme.
const allowed = new Set(['dev'])

const mcp = new Server(
  { name: 'webhook', version: '0.0.1' },
  {
    capabilities: {
      experimental: {
        'claude/channel': {},
        'claude/channel/permission': {},  // opter pour le relais de permission
      },
      tools: {},
    },
    instructions:
      'Les messages arrivent sous la forme <channel source="webhook" chat_id="...">. ' +
      'Répondez avec l'outil de réponse, en passant le chat_id de la balise.',
  },
)

// --- outil de réponse : Claude appelle ceci pour envoyer un message en retour ---
mcp.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: [{
    name: 'reply',
    description: 'Envoyer un message en retour sur ce canal',
    inputSchema: {
      type: 'object',
      properties: {
        chat_id: { type: 'string', description: 'La conversation dans laquelle répondre' },
        text: { type: 'string', description: 'Le message à envoyer' },
      },
      required: ['chat_id', 'text'],
    },
  }],
}))

mcp.setRequestHandler(CallToolRequestSchema, async req => {
  if (req.params.name === 'reply') {
    const { chat_id, text } = req.params.arguments as { chat_id: string; text: string }
    send(`Reply to ${chat_id}: ${text}`)
    return { content: [{ type: 'text', text: 'sent' }] }
  }
  throw new Error(`unknown tool: ${req.params.name}`)
})

// --- relais de permission : Claude Code (pas Claude) appelle ceci quand une boîte de dialogue s'ouvre
const PermissionRequestSchema = z.object({
  method: z.literal('notifications/claude/channel/permission_request'),
  params: z.object({
    request_id: z.string(),
    tool_name: z.string(),
    description: z.string(),
    input_preview: z.string(),
  }),
})

mcp.setNotificationHandler(PermissionRequestSchema, async ({ params }) => {
  send(
    `Claude wants to run ${params.tool_name}: ${params.description}\n\n` +
    `Reply "yes ${params.request_id}" or "no ${params.request_id}"`,
  )
})

await mcp.connect(new StdioServerTransport())

// --- HTTP sur :8788 : GET /events diffuse la sortie, POST route l'entrée ---
const PERMISSION_REPLY_RE = /^\s*(y|yes|n|no)\s+([a-km-z]{5})\s*$/i
let nextId = 1

Bun.serve({
  port: 8788,
  hostname: '127.0.0.1',
  idleTimeout: 0,  // ne pas fermer les flux SSE inactifs
  async fetch(req) {
    const url = new URL(req.url)

    // GET /events : flux SSE afin que curl -N puisse regarder les réponses et les invites en direct
    if (req.method === 'GET' && url.pathname === '/events') {
      const stream = new ReadableStream({
        start(ctrl) {
          ctrl.enqueue(': connected\n\n')  // afin que curl affiche quelque chose immédiatement
          const emit = (chunk: string) => ctrl.enqueue(chunk)
          listeners.add(emit)
          req.signal.addEventListener('abort', () => listeners.delete(emit))
        },
      })
      return new Response(stream, {
        headers: { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-cache' },
      })
    }

    // tout le reste est entrant : contrôler sur l'expéditeur d'abord
    const body = await req.text()
    const sender = req.headers.get('X-Sender') ?? ''
    if (!allowed.has(sender)) return new Response('forbidden', { status: 403 })

    // vérifier le format du verdict avant de traiter comme chat
    const m = PERMISSION_REPLY_RE.exec(body)
    if (m) {
      await mcp.notification({
        method: 'notifications/claude/channel/permission',
        params: {
          request_id: m[2].toLowerCase(),
          behavior: m[1].toLowerCase().startsWith('y') ? 'allow' : 'deny',
        },
      })
      return new Response('verdict recorded')
    }

    // chat normal : transmettre à Claude en tant qu'événement de canal
    const chat_id = String(nextId++)
    await mcp.notification({
      method: 'notifications/claude/channel',
      params: { content: body, meta: { chat_id, path: url.pathname } },
    })
    return new Response('ok')
  },
})
```

Testez le chemin du verdict dans trois terminaux. Le premier est votre session Claude Code, démarrée avec le [drapeau de développement](#test-during-the-research-preview) afin qu'elle lance `webhook.ts` :

```bash theme={null}
claude --dangerously-load-development-channels server:webhook
```

Dans le second, diffusez le côté sortant afin que vous puissiez voir les réponses de Claude et toutes les invites de permission au fur et à mesure qu'elles se déclenchent :

```bash theme={null}
curl -N localhost:8788/events
```

Dans le troisième, envoyez un message qui fera que Claude essaiera d'exécuter une commande :

```bash theme={null}
curl -d "list the files in this directory" -H "X-Sender: dev" localhost:8788
```

L'énumération de fichiers est en lecture seule, donc Claude l'exécute sans approbation. La boîte de dialogue de permission s'ouvre quand Claude appelle l'outil `reply` pour envoyer sa réponse en retour. La boîte de dialogue locale s'ouvre dans votre terminal Claude Code, et un moment plus tard l'invite pour `mcp__webhook__reply` apparaît dans le flux `/events`, y compris l'ID à cinq lettres. Approuvez-le du côté distant :

```bash theme={null}
curl -d "yes <id>" -H "X-Sender: dev" localhost:8788
```

La boîte de dialogue locale se ferme, l'outil `reply` s'exécute, et la réponse de Claude atterrit dans le flux.

Les trois pièces spécifiques au canal dans ce fichier :

* **Capacités** dans le constructeur `Server` : `claude/channel` enregistre l'écouteur de notification, `claude/channel/permission` opte pour le relais de permission, `tools` laisse Claude découvrir l'outil de réponse.
* **Chemins sortants** : le gestionnaire de l'outil `reply` est ce que Claude appelle pour les réponses conversationnelles ; le gestionnaire de notification `PermissionRequestSchema` est ce que Claude Code appelle quand une boîte de dialogue de permission s'ouvre. Les deux appellent `send()` pour diffuser sur `/events`, mais ils sont déclenchés par différentes parties du système.
* **Gestionnaire HTTP** : `GET /events` maintient un flux SSE ouvert afin que curl puisse regarder la sortie en direct ; `POST` est entrant, contrôlé sur l'en-tête `X-Sender`. Un corps `yes <id>` ou `no <id>` va à Claude Code en tant que notification de verdict et n'atteint jamais Claude ; tout le reste est transmis à Claude en tant qu'événement de canal.

<h2 id="package-as-a-plugin">
  Empaqueter en tant que plugin
</h2>

Pour rendre votre canal installable et partageable, enveloppez-le dans un [plugin](/docs/fr/plugins) et publiez-le sur un [marketplace](/docs/fr/plugin-marketplaces). Les utilisateurs l'installent avec `/plugin install`, puis l'activent par session avec `--channels plugin:<name>@<marketplace>`.

Un canal publié sur votre propre marketplace a toujours besoin de `--dangerously-load-development-channels` pour s'exécuter, car il n'est pas sur la [liste d'approbation](/docs/fr/channels#supported-channels). La liste d'approbation par défaut est les plugins de canal dans `claude-plugins-official`, que Anthropic organise à sa discrétion. Les [formulaires de soumission intégrés à l'application](/docs/fr/plugins#submit-your-plugin-to-the-community-marketplace) ajoutent des plugins au marketplace communautaire, qui n'est pas sur la liste d'approbation des canaux.

Si vous travaillez avec un contact partenaire Anthropic, contactez-le pour coordonner une inscription au marketplace officiel. Sur les plans Team et Enterprise, un administrateur peut plutôt inclure votre plugin dans la liste [`allowedChannelPlugins`](/docs/fr/channels#restrict-which-channel-plugins-can-run) de l'organisation, qui remplace la liste d'approbation Anthropic par défaut.

<h2 id="see-also">
  Voir aussi
</h2>

* [Canaux](/docs/fr/channels) pour installer et utiliser Telegram, Discord, iMessage ou la démo fakechat, et pour activer les canaux pour une organisation Team ou Enterprise
* [Implémentations de canaux fonctionnels](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins) pour le code serveur complet avec flux d'appairage, outils de réponse et pièces jointes
* [MCP](/docs/fr/mcp) pour le protocole sous-jacent que les serveurs de canal implémentent
* [Plugins](/docs/fr/plugins) pour empaqueter votre canal afin que les utilisateurs puissent l'installer avec `/plugin install`
