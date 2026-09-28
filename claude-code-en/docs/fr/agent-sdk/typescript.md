> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Référence du SDK Agent - TypeScript

> Référence API complète du SDK Agent TypeScript, incluant toutes les fonctions, types et interfaces.

<script src="/docs/components/typescript-sdk-type-links.js" defer />

<h2 id="installation">
  Installation
</h2>

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

<Note>
  Le SDK regroupe un binaire Claude Code natif pour votre plateforme en tant que dépendance optionnelle telle que `@anthropic-ai/claude-agent-sdk-darwin-arm64`. La plupart des installations n'ont pas besoin d'installation Claude Code séparée. La version du SDK suit la version du binaire Claude Code fourni. Le SDK v0.3.191 regroupe Claude Code v2.1.191, donc une fonctionnalité sur cette page qui nécessite une version Claude Code a besoin de la version du SDK avec le même numéro de correctif ou ultérieur. Si votre gestionnaire de paquets ignore les dépendances optionnelles, le SDK lève `Native CLI binary for <platform>-<arch> not found` ; définissez [`pathToClaudeCodeExecutable`](#options) sur un binaire `claude` installé séparément à la place.

  Si votre gestionnaire de paquets n'applique pas le champ `libc` de npm, comme Yarn 1.x ne le fait pas, vous obtenez à la fois les packages de plateforme glibc et musl sur Linux, ce qui double à peu près la taille d'installation. Sur Agent SDK v0.2.141 ou ultérieur, le SDK lance toujours la variante correcte. Pour récupérer l'espace dans une image conteneur, supprimez le package de plateforme qui ne correspond pas au libc où votre application s'exécute ; pour un runtime glibc sur x64, c'est `rm -rf node_modules/@anthropic-ai/claude-agent-sdk-linux-x64-musl`. Sur une machine de développement, la suppression est temporaire, car Yarn réinstalle le package lors du prochain changement de dépendance.
</Note>

<h3 id="compile-to-a-single-executable">
  Compiler en un seul exécutable
</h3>

Lorsque vous compilez votre application en un exécutable à fichier unique avec `bun build --compile`, le SDK ne peut pas résoudre le binaire CLI fourni au moment de l'exécution. `require.resolve` ne fonctionne pas à l'intérieur du système de fichiers virtuel `$bunfs` de l'exécutable compilé, donc le SDK lève `Native CLI binary for <platform>-<arch> not found`.

Pour contourner ce problème, intégrez le binaire de plateforme en tant que ressource de fichier, extrayez-le vers un chemin réel au démarrage avec `extractFromBunfs()`, et transmettez ce chemin à [`pathToClaudeCodeExecutable`](#options).

L'assistant `extractFromBunfs()` nécessite `@anthropic-ai/claude-agent-sdk` v0.3.144 ou ultérieur. L'exemple ci-dessous compile pour macOS sur Apple Silicon :

```typescript theme={null}
import binPath from "@anthropic-ai/claude-agent-sdk-darwin-arm64/claude" with { type: "file" };
import { extractFromBunfs } from "@anthropic-ai/claude-agent-sdk/extract";
import { query } from "@anthropic-ai/claude-agent-sdk";

const cliPath = extractFromBunfs(binPath);

for await (const message of query({
  prompt: "Hello",
  options: { pathToClaudeCodeExecutable: cliPath },
})) {
  console.log(message);
}
```

`extractFromBunfs()` copie le binaire intégré hors du système de fichiers virtuel de l'exécutable compilé vers un répertoire temporaire par utilisateur et retourne le chemin réel. En dehors d'un exécutable compilé, il retourne le chemin d'entrée inchangé, donc le même code s'exécute en développement sans modification.

Chaque exécutable compilé intègre le binaire d'une seule plateforme. Faites correspondre le package de plateforme dans l'importation à votre `--target` :

* Pour la compilation croisée, installez le package de plateforme non correspondant, par exemple `npm install @anthropic-ai/claude-agent-sdk-linux-x64 --force`.
* Sur Windows, le sous-chemin binaire est `claude.exe`, par exemple `@anthropic-ai/claude-agent-sdk-win32-x64/claude.exe`.

<h2 id="functions">
  Fonctions
</h2>

<h3 id="query">
  `query()`
</h3>

La fonction principale pour interagir avec Claude Code. Crée un générateur asynchrone qui diffuse les messages au fur et à mesure de leur arrivée.

```typescript theme={null}
function query({
  prompt,
  options
}: {
  prompt: string | AsyncIterable<SDKUserMessage>;
  options?: Options;
}): Query;
```

<h4 id="parameters">
  Paramètres
</h4>

| Paramètre | Type                                                             | Description                                                                               |
| :-------- | :--------------------------------------------------------------- | :---------------------------------------------------------------------------------------- |
| `prompt`  | `string \| AsyncIterable<`[`SDKUserMessage`](#sdkusermessage)`>` | L'invite d'entrée sous forme de chaîne ou d'itérable asynchrone pour le mode de diffusion |
| `options` | [`Options`](#options)                                            | Objet de configuration optionnel (voir le type Options ci-dessous)                        |

<h4 id="returns">
  Retours
</h4>

Retourne un objet [`Query`](#query-object) qui étend `AsyncGenerator<`[`SDKMessage`](#sdkmessage)`, void>` avec des méthodes supplémentaires.

<h3 id="startup">
  `startup()`
</h3>

Préconfigure le sous-processus CLI en le générant et en complétant la poignée de main d'initialisation avant qu'une invite soit disponible. Le handle [`WarmQuery`](#warmquery) retourné accepte une invite plus tard et l'écrit dans un processus déjà prêt, de sorte que le premier appel `query()` se résout sans payer le coût de génération et d'initialisation du sous-processus en ligne.

```typescript theme={null}
function startup(params?: {
  options?: Options;
  initializeTimeoutMs?: number;
}): Promise<WarmQuery>;
```

<h4 id="parameters-2">
  Paramètres
</h4>

| Paramètre             | Type                  | Description                                                                                                                                                                                                         |
| :-------------------- | :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `options`             | [`Options`](#options) | Objet de configuration optionnel. Identique au paramètre `options` de `query()`                                                                                                                                     |
| `initializeTimeoutMs` | `number`              | Temps maximum en millisecondes à attendre pour l'initialisation du sous-processus. Par défaut `60000`. Si l'initialisation ne se termine pas à temps, la promesse est rejetée avec une erreur de délai d'expiration |

<h4 id="returns-2">
  Retours
</h4>

Retourne une `Promise<`[`WarmQuery`](#warmquery)`>` qui se résout une fois que le sous-processus a été généré et a complété sa poignée de main d'initialisation.

<h4 id="example">
  Exemple
</h4>

Appelez `startup()` tôt, par exemple au démarrage de l'application, puis appelez `.query()` sur le handle retourné une fois qu'une invite est prête. Cela déplace la génération du sous-processus et l'initialisation en dehors du chemin critique.

```typescript theme={null}
import { startup } from "@anthropic-ai/claude-agent-sdk";

// Payez le coût de démarrage à l'avance
const warm = await startup({ options: { maxTurns: 3 } });

// Plus tard, quand une invite est prête, c'est immédiat
for await (const message of warm.query("What files are here?")) {
  console.log(message);
}
```

<h3 id="tool">
  `tool()`
</h3>

Crée une définition d'outil MCP type-safe pour une utilisation avec les serveurs MCP du SDK.

```typescript theme={null}
function tool<Schema extends AnyZodRawShape>(
  name: string,
  description: string,
  inputSchema: Schema,
  handler: (args: InferShape<Schema>, extra: unknown) => Promise<CallToolResult>,
  extras?: { annotations?: ToolAnnotations; searchHint?: string; alwaysLoad?: boolean }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-3">
  Paramètres
</h4>

| Paramètre     | Type                                                                                                   | Description                                                                                                                                                                                                                                                                                                                                                     |
| :------------ | :----------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | `string`                                                                                               | Le nom de l'outil                                                                                                                                                                                                                                                                                                                                               |
| `description` | `string`                                                                                               | Une description de ce que fait l'outil                                                                                                                                                                                                                                                                                                                          |
| `inputSchema` | `Schema extends AnyZodRawShape`                                                                        | Schéma Zod définissant les paramètres d'entrée de l'outil (supporte Zod 3 et Zod 4)                                                                                                                                                                                                                                                                             |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>`                                      | Fonction asynchrone qui exécute la logique de l'outil                                                                                                                                                                                                                                                                                                           |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)`; searchHint?: string; alwaysLoad?: boolean }` | Extras optionnels. `annotations` fournit des indices comportementaux MCP aux clients. `searchHint` est une phrase de capacité d'une ligne affichée dans la liste des outils différés quand la [recherche d'outils](/docs/fr/agent-sdk/tool-search) est active. `alwaysLoad: true` garde le schéma complet de cet outil dans l'invite initiale au lieu de le différer |

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

Réexportée depuis `@modelcontextprotocol/sdk/types.js`. Tous les champs sont des indices optionnels ; les clients ne doivent pas s'y fier pour les décisions de sécurité.

| Champ             | Type      | Par défaut  | Description                                                                                                                                                         |
| :---------------- | :-------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `title`           | `string`  | `undefined` | Titre lisible par l'homme pour l'outil                                                                                                                              |
| `readOnlyHint`    | `boolean` | `false`     | Si `true`, l'outil ne modifie pas son environnement                                                                                                                 |
| `destructiveHint` | `boolean` | `true`      | Si `true`, l'outil peut effectuer des mises à jour destructrices (uniquement significatif quand `readOnlyHint` est `false`)                                         |
| `idempotentHint`  | `boolean` | `false`     | Si `true`, les appels répétés avec les mêmes arguments n'ont aucun effet supplémentaire (uniquement significatif quand `readOnlyHint` est `false`)                  |
| `openWorldHint`   | `boolean` | `true`      | Si `true`, l'outil interagit avec des entités externes (par exemple, recherche web). Si `false`, le domaine de l'outil est fermé (par exemple, un outil de mémoire) |

```typescript theme={null}
import { tool } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

const searchTool = tool(
  "search",
  "Search the web",
  { query: z.string() },
  async ({ query }) => {
    return { content: [{ type: "text", text: `Results for: ${query}` }] };
  },
  { annotations: { readOnlyHint: true, openWorldHint: true } }
);
```

<h3 id="createsdkmcpserver">
  `createSdkMcpServer()`
</h3>

Crée une instance de serveur MCP qui s'exécute dans le même processus que votre application.

```typescript theme={null}
function createSdkMcpServer(options: {
  name: string;
  version?: string;
  instructions?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
  alwaysLoad?: boolean;
  timeout?: number;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  Paramètres
</h4>

| Paramètre              | Type                          | Description                                                                                                                                                                                                                                                                                            |
| :--------------------- | :---------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.name`         | `string`                      | Le nom du serveur MCP                                                                                                                                                                                                                                                                                  |
| `options.version`      | `string`                      | Chaîne de version optionnelle                                                                                                                                                                                                                                                                          |
| `options.instructions` | `string`                      | Instructions optionnelles du serveur, retournées depuis `initialize` et exposées au modèle comme un bloc d'instructions MCP                                                                                                                                                                            |
| `options.tools`        | `Array<SdkMcpToolDefinition>` | Tableau de définitions d'outils créées avec [`tool()`](#tool)                                                                                                                                                                                                                                          |
| `options.alwaysLoad`   | `boolean`                     | Quand `true`, chaque outil de ce serveur reste dans l'invite initiale et n'est jamais différé derrière la [recherche d'outils](/docs/fr/agent-sdk/tool-search). Se combine avec `alwaysLoad` par outil dans [`tool()`](#tool)                                                                               |
| `options.timeout`      | `number`                      | Délai d'expiration en millisecondes pour les appels d'outils de ce serveur. Claude Code l'applique à ce serveur à la place de [`MCP_TOOL_TIMEOUT`](/docs/fr/env-vars). Passez un nombre entier d'au moins 1000. Claude Code ignore les autres valeurs. Nécessite TypeScript Agent SDK v0.3.248 ou ultérieur |

<h3 id="listsessions">
  `listSessions()`
</h3>

Découvre et répertorie les sessions passées avec des métadonnées légères. Filtrez par répertoire de projet ou répertoriez les sessions dans tous les projets.

```typescript theme={null}
function listSessions(options?: ListSessionsOptions): Promise<SDKSessionInfo[]>;
```

<h4 id="parameters-5">
  Paramètres
</h4>

| Paramètre                  | Type      | Par défaut  | Description                                                                                                      |
| :------------------------- | :-------- | :---------- | :--------------------------------------------------------------------------------------------------------------- |
| `options.dir`              | `string`  | `undefined` | Répertoire pour lequel répertorier les sessions. Lorsqu'il est omis, retourne les sessions dans tous les projets |
| `options.limit`            | `number`  | `undefined` | Nombre maximum de sessions à retourner                                                                           |
| `options.includeWorktrees` | `boolean` | `true`      | Quand `dir` est à l'intérieur d'un référentiel git, inclure les sessions de tous les chemins worktree            |

<h4 id="return-type-sdksessioninfo">
  Type de retour : `SDKSessionInfo`
</h4>

| Propriété      | Type                  | Description                                                                                        |
| :------------- | :-------------------- | :------------------------------------------------------------------------------------------------- |
| `sessionId`    | `string`              | Identifiant de session unique (UUID)                                                               |
| `summary`      | `string`              | Titre d'affichage : titre personnalisé, résumé généré automatiquement ou première invite           |
| `lastModified` | `number`              | Heure de dernière modification en millisecondes depuis l'époque                                    |
| `fileSize`     | `number \| undefined` | Taille du fichier de session en octets. Rempli uniquement pour le stockage JSONL local             |
| `customTitle`  | `string \| undefined` | Titre de session défini par l'utilisateur (via `/rename`)                                          |
| `firstPrompt`  | `string \| undefined` | Première invite utilisateur significative dans la session                                          |
| `gitBranch`    | `string \| undefined` | Branche Git à la fin de la session                                                                 |
| `cwd`          | `string \| undefined` | Répertoire de travail pour la session                                                              |
| `tag`          | `string \| undefined` | Étiquette de session définie par l'utilisateur (voir [`tagSession()`](#tagsession))                |
| `createdAt`    | `number \| undefined` | Heure de création en millisecondes depuis l'époque, à partir de l'horodatage de la première entrée |

<h4 id="example-2">
  Exemple
</h4>

Imprimez les 10 sessions les plus récentes pour un projet. Les résultats sont triés par `lastModified` décroissant, donc le premier élément est le plus récent. Omettez `dir` pour rechercher dans tous les projets.

```typescript theme={null}
import { listSessions } from "@anthropic-ai/claude-agent-sdk";

const sessions = await listSessions({ dir: "/path/to/project", limit: 10 });

for (const session of sessions) {
  console.log(`${session.summary} (${session.sessionId})`);
}
```

<h3 id="getsessionmessages">
  `getSessionMessages()`
</h3>

Lit les messages utilisateur et assistant à partir d'une transcription de session passée.

```typescript theme={null}
function getSessionMessages(
  sessionId: string,
  options?: GetSessionMessagesOptions
): Promise<SessionMessage[]>;
```

<h4 id="parameters-6">
  Paramètres
</h4>

| Paramètre        | Type     | Par défaut  | Description                                                                                       |
| :--------------- | :------- | :---------- | :------------------------------------------------------------------------------------------------ |
| `sessionId`      | `string` | requis      | UUID de session à lire (voir `listSessions()`)                                                    |
| `options.dir`    | `string` | `undefined` | Répertoire de projet pour trouver la session. Lorsqu'il est omis, recherche dans tous les projets |
| `options.limit`  | `number` | `undefined` | Nombre maximum de messages à retourner                                                            |
| `options.offset` | `number` | `undefined` | Nombre de messages à ignorer à partir du début                                                    |

<h4 id="return-type-sessionmessage">
  Type de retour : `SessionMessage`
</h4>

| Propriété            | Type                    | Description                                                                                                                                                                                                                                                                                                                   |
| :------------------- | :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | Rôle du message                                                                                                                                                                                                                                                                                                               |
| `uuid`               | `string`                | Identifiant de message unique                                                                                                                                                                                                                                                                                                 |
| `session_id`         | `string`                | Session à laquelle ce message appartient                                                                                                                                                                                                                                                                                      |
| `message`            | `unknown`               | Charge utile de message brute de la transcription                                                                                                                                                                                                                                                                             |
| `parent_tool_use_id` | `string \| null`        | Pour les messages de sous-agent, le `tool_use_id` de l'appel d'outil `Agent` ou `Skill` qui l'a généré. `null` pour les messages de session principale et les sessions plus anciennes                                                                                                                                         |
| `parent_agent_id`    | `string \| null`        | Pour les messages d'un [sous-agent imbriqué](/docs/fr/sub-agents#let-subagents-spawn-their-own-subagents), le `agentId` du sous-agent qui l'a généré. `null` pour les messages de session principale, les messages des sous-agents de niveau supérieur et les sessions plus anciennes. Nécessite Claude Code v2.1.202 ou ultérieur |

<h4 id="example-3">
  Exemple
</h4>

```typescript theme={null}
import { listSessions, getSessionMessages } from "@anthropic-ai/claude-agent-sdk";

const [latest] = await listSessions({ dir: "/path/to/project", limit: 1 });

if (latest) {
  const messages = await getSessionMessages(latest.sessionId, {
    dir: "/path/to/project",
    limit: 20
  });

  for (const msg of messages) {
    console.log(`[${msg.type}] ${msg.uuid}`);
  }
}
```

<h3 id="getsessioninfo">
  `getSessionInfo()`
</h3>

Lit les métadonnées d'une seule session par ID sans analyser le répertoire de projet complet.

```typescript theme={null}
function getSessionInfo(
  sessionId: string,
  options?: GetSessionInfoOptions
): Promise<SDKSessionInfo | undefined>;
```

<h4 id="parameters-7">
  Paramètres
</h4>

| Paramètre     | Type     | Par défaut  | Description                                                                                       |
| :------------ | :------- | :---------- | :------------------------------------------------------------------------------------------------ |
| `sessionId`   | `string` | requis      | UUID de la session à rechercher                                                                   |
| `options.dir` | `string` | `undefined` | Chemin du répertoire de projet. Lorsqu'il est omis, recherche dans tous les répertoires de projet |

Retourne [`SDKSessionInfo`](#return-type-sdksessioninfo), ou `undefined` si la session n'est pas trouvée.

<h3 id="renamesession">
  `renameSession()`
</h3>

Renomme une session en ajoutant une entrée de titre personnalisé. Les appels répétés sont sûrs ; le titre le plus récent gagne.

```typescript theme={null}
function renameSession(
  sessionId: string,
  title: string,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-8">
  Paramètres
</h4>

| Paramètre     | Type     | Par défaut  | Description                                                                                       |
| :------------ | :------- | :---------- | :------------------------------------------------------------------------------------------------ |
| `sessionId`   | `string` | requis      | UUID de la session à renommer                                                                     |
| `title`       | `string` | requis      | Nouveau titre. Doit être non vide après suppression des espaces blancs                            |
| `options.dir` | `string` | `undefined` | Chemin du répertoire de projet. Lorsqu'il est omis, recherche dans tous les répertoires de projet |

<h3 id="tagsession">
  `tagSession()`
</h3>

Étiquette une session. Passez `null` pour effacer l'étiquette. Les appels répétés sont sûrs ; l'étiquette la plus récente gagne.

```typescript theme={null}
function tagSession(
  sessionId: string,
  tag: string | null,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-9">
  Paramètres
</h4>

| Paramètre     | Type             | Par défaut  | Description                                                                                       |
| :------------ | :--------------- | :---------- | :------------------------------------------------------------------------------------------------ |
| `sessionId`   | `string`         | requis      | UUID de la session à étiqueter                                                                    |
| `tag`         | `string \| null` | requis      | Chaîne d'étiquette, ou `null` pour effacer                                                        |
| `options.dir` | `string`         | `undefined` | Chemin du répertoire de projet. Lorsqu'il est omis, recherche dans tous les répertoires de projet |

<h3 id="resolvesettings">
  `resolveSettings()`
</h3>

Résout les paramètres Claude Code effectifs pour un répertoire donné en utilisant le même moteur de fusion que l'interface CLI, sans générer l'interface CLI Claude. Utilisez-le pour inspecter quelle configuration un appel `query()` verrait avant d'en invoquer un.

<Note>
  Cette fonction est en version alpha et son API peut changer avant la stabilisation.
</Note>

L'instantané diffère de ce qu'une session `query()` en direct applique :

* **`policyHelper`** : `resolveSettings()` lit les sources MDM, y compris la liste de propriétés macOS et Windows HKLM/HKCU, mais n'exécute pas le sous-processus `policyHelper` configuré par l'administrateur.
* **Paramètres gérés par le serveur** : `resolveSettings()` ne récupère pas les [paramètres gérés par le serveur](/docs/fr/server-managed-settings#fetch-and-caching-behavior). Passez-les comme `options.serverManagedSettings` pour les inclure.
* **`defaultMode`** : l'instantané retourne `permissions.defaultMode` tel quel de chaque niveau, de sorte qu'il peut inclure les valeurs `'auto'` et `'bypassPermissions'` des paramètres de projet et locaux, que [une session en direct ignore](/docs/fr/permission-modes#which-mode-a-session-starts-in).

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  Paramètres
</h4>

`resolveSettings()` accepte un seul objet d'options. Tous les champs sont optionnels.

| Paramètre                       | Type                                  | Par défaut         | Description                                                                                                                                                                                                                                                                                                                                                                |
| :------------------------------ | :------------------------------------ | :----------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.cwd`                   | `string`                              | `process.cwd()`    | Répertoire pour résoudre les paramètres de projet et locaux par rapport à                                                                                                                                                                                                                                                                                                  |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | Toutes les sources | Quelles sources du système de fichiers charger. Passez `[]` pour ignorer les paramètres utilisateur, projet et locaux. La [politique gérée par le point de terminaison](/docs/fr/managed-settings#delivery-mechanisms) se charge dans tous les cas. `resolveSettings()` inclut les paramètres gérés par le serveur uniquement quand vous passez `options.serverManagedSettings` |
| `options.managedSettings`       | `Settings`                            | `undefined`        | Paramètres de politique fournis par l'hôte d'intégration. Suit les mêmes règles que [`managedSettings` dans `Options`](#options), sauf que `resolveSettings()` n'exécute pas un [`policyHelper`](/docs/fr/settings-reference#policyhelper) configuré, de sorte que l'instantané peut inclure des paramètres qu'une session en direct supprime                                   |
| `options.serverManagedSettings` | `Settings`                            | `undefined`        | Charge utile de paramètres gérés par le serveur depuis `/api/claude_code/settings`. Les clés non restrictives passent sans filtre                                                                                                                                                                                                                                          |

<h4 id="return-type-resolvedsettings">
  Type de retour : `ResolvedSettings`
</h4>

`resolveSettings()` retourne un objet décrivant les paramètres fusionnés et la source qui a contribué à chaque clé.

| Propriété    | Type                                                | Description                                                                                      |
| :----------- | :-------------------------------------------------- | :----------------------------------------------------------------------------------------------- |
| `effective`  | `Settings`                                          | Paramètres fusionnés après application de toutes les sources activées dans l'ordre de précédence |
| `provenance` | `Partial<Record<keyof Settings, ProvenanceEntry>>`  | Pour chaque clé de niveau supérieur dans `effective`, quelle source a fourni la valeur           |
| `sources`    | `Array<{ source, settings, path?, policyOrigin? }>` | Paramètres bruts par source, ordonnés de la plus basse à la plus haute précédence                |

<h4 id="example-4">
  Exemple
</h4>

L'exemple ci-dessous résout les paramètres pour un répertoire de projet et imprime la source qui contrôle la période de nettoyage. Sur une machine où aucun fichier de paramètres ne définit `cleanupPeriodDays`, les deux lignes imprimées affichent `undefined` pour la valeur, ce qui est le résultat attendu plutôt qu'une erreur.

```typescript theme={null}
import { resolveSettings } from "@anthropic-ai/claude-agent-sdk";

const { effective, provenance } = await resolveSettings({
  cwd: "/path/to/project",
  settingSources: ["user", "project", "local"],
});

console.log(`Cleanup period: ${effective.cleanupPeriodDays} days`);
console.log(`Set by: ${provenance.cleanupPeriodDays?.source}`);
```

<h2 id="types">
  Types
</h2>

<h3 id="options">
  `Options`
</h3>

Objet de configuration pour la fonction `query()`.

| Propriété                         | Type                                                                                                                                                                                                           | Par défaut                                              | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abortController`                 | `AbortController`                                                                                                                                                                                              | `new AbortController()`                                 | Contrôleur pour annuler les opérations                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `additionalDirectories`           | `string[]`                                                                                                                                                                                                     | `[]`                                                    | Répertoires supplémentaires auxquels Claude peut accéder. Le SDK transmet chaque entrée à Claude Code en tant que `--add-dir`, donc avec le paramètre `project`, Claude Code charge également [les compétences, commandes et sous-agents du répertoire](/docs/fr/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `agent`                           | `string`                                                                                                                                                                                                       | `undefined`                                             | Nom de l'agent pour le thread principal. L'agent doit être défini dans l'option `agents` ou dans les paramètres                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                                                                                                                        | `undefined`                                             | Définir programmatiquement les sous-agents                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `agentProgressSummaries`          | `boolean`                                                                                                                                                                                                      | `false`                                                 | Quand `true`, générer des résumés de progression d'une ligne pour les sous-agents et les transférer sur les événements [`task_progress`](#sdktaskprogressmessage) via le champ `summary`. S'applique aux sous-agents de premier plan et d'arrière-plan                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                                                                                                                      | `false`                                                 | Activer le contournement des permissions. Requis lors de l'utilisation de `permissionMode: 'bypassPermissions'`, au démarrage ou ultérieurement via `setPermissionMode()`. Voir [mode plan](/docs/fr/agent-sdk/permissions#plan-mode-plan) pour savoir comment il interagit avec `permissionMode: 'plan'`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `allowedTools`                    | `string[]`                                                                                                                                                                                                     | `[]`                                                    | Outils à approuver automatiquement sans demander. Cela ne restreint pas Claude à seulement ces outils. Si vous nommez l'un des [outils de suivi des tâches](/docs/fr/agent-sdk/todo-tracking#model-availability) ici, Claude Code opte également la session. Les autres outils non répertoriés passent à `permissionMode` et `canUseTool`. Utilisez `disallowedTools` pour bloquer les outils. Voir [Permissions](/docs/fr/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                                                                                                                      | `[]`                                                    | Activer les fonctionnalités bêta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                                                                                                                                    | `undefined`                                             | Fonction de permission personnalisée, invoquée uniquement quand le [flux de permission](/docs/fr/agent-sdk/permissions#how-permissions-are-evaluated) se termine par une invite. Non invoquée pour les appels pré-approuvés par `allowedTools`, les règles d'autorisation, ou `permissionMode`. Une règle d'autorisation ne pré-approuve pas les [actions qu'aucun mode n'approuve automatiquement](/docs/fr/permission-modes#actions-no-mode-auto-approves). Voir [`CanUseTool`](#canusetool) pour les détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `continue`                        | `boolean`                                                                                                                                                                                                      | `false`                                                 | Continuer la conversation la plus récente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `cwd`                             | `string`                                                                                                                                                                                                       | `process.cwd()`                                         | Répertoire de travail actuel                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `debug`                           | `boolean`                                                                                                                                                                                                      | `false`                                                 | Activer le mode débogage pour le processus Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `debugFile`                       | `string`                                                                                                                                                                                                       | `undefined`                                             | Écrire les journaux de débogage dans un chemin de fichier spécifique. Active implicitement le mode débogage                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `disallowedTools`                 | `string[]`                                                                                                                                                                                                     | `[]`                                                    | Outils à refuser. Un nom simple tel que `"Bash"` supprime l'outil du contexte de Claude. Une règle délimitée telle que `"Bash(rm *)"` laisse l'outil disponible et refuse les appels correspondants dans chaque mode de permission, y compris `bypassPermissions`, pour la commande [telle qu'écrite](/docs/fr/permissions#bash-rule-limits). Voir [Permissions](/docs/fr/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                                                                                                                              | `undefined`                                             | Contrôle l'effort que Claude met dans sa réponse. Fonctionne avec la réflexion adaptative pour guider la profondeur de réflexion. Voir [ajuster le niveau d'effort](/docs/fr/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `enableFileCheckpointing`         | `boolean`                                                                                                                                                                                                      | `false`                                                 | Activer le suivi des modifications de fichiers pour le rembobinage. Voir [Sauvegarde de fichiers](/docs/fr/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `env`                             | `Record<string, string \| undefined>`                                                                                                                                                                          | `process.env`                                           | Variables d'environnement. Quand défini, cela remplace l'environnement du sous-processus au lieu de fusionner avec `process.env`, donc passez `{ ...process.env, YOUR_VAR: 'value' }` pour conserver les variables héritées comme `PATH`. Voir [Gérer les réponses API lentes ou bloquées](#handle-slow-or-stalled-api-responses) pour un exemple de ce modèle, et [Variables d'environnement](/docs/fr/env-vars) pour les variables que la CLI sous-jacente lit. Définissez `CLAUDE_AGENT_SDK_CLIENT_APP` pour identifier votre application dans l'en-tête User-Agent                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                                                                                                                                    | Détection automatique                                   | Runtime JavaScript à utiliser                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `executableArgs`                  | `string[]`                                                                                                                                                                                                     | `[]`                                                    | Arguments à passer à l'exécutable                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `extraArgs`                       | `Record<string, string \| null>`                                                                                                                                                                               | `{}`                                                    | Arguments supplémentaires                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `fallbackModel`                   | `string`                                                                                                                                                                                                       | `undefined`                                             | Modèle à utiliser si le principal échoue. Accepte une liste séparée par des virgules. Pour l'ordre et le plafond, voir [Chaînes de modèle de secours](/docs/fr/model-config#fallback-model-chains). Pour des conseils, voir [Choisir un modèle](/docs/fr/agent-sdk/configuration#choose-a-model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `forkSession`                     | `boolean`                                                                                                                                                                                                      | `false`                                                 | Lors de la reprise avec `resume`, bifurquer vers un nouvel ID de session au lieu de continuer la session d'origine                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `forwardSubagentText`             | `boolean`                                                                                                                                                                                                      | `false`                                                 | Transférer les blocs de texte et de réflexion des sous-agents en tant que messages assistant et utilisateur avec `parent_tool_use_id` défini, pour que les consommateurs puissent afficher une transcription imbriquée. Sans cette option, Claude Code émet les blocs `tool_use` et `tool_result` des sous-agents mais pas le texte ou la réflexion. Les messages des sous-agents à chaque profondeur d'imbrication sont transférés sur Claude Code v2.1.219 et ultérieur ; avant v2.1.219, seuls les messages des sous-agents de profondeur 1 apparaissaient. Les messages des sous-agents qu'une compétence bifurquée génère, et des compétences bifurquées imbriquées, nécessitent v2.1.275 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`                                                                                                              | `{}`                                                    | Rappels de hook pour les événements                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `includeHookEvents`               | `boolean`                                                                                                                                                                                                      | `false`                                                 | Inclure les événements du cycle de vie du hook dans le flux de messages en tant que [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage), et [`SDKHookResponseMessage`](#sdkhookresponsemessage). Les événements du cycle de vie pour les hooks `SessionStart` et `Setup` sont toujours inclus et n'ont pas besoin de cette option. Certains événements de hook, tels que `Notification`, `SessionEnd`, `PreCompact`, et `PostCompact`, ne produisent jamais un `SDKHookStartedMessage`, même avec cette option. Pour ceux-ci, Claude Code émet toujours un `SDKHookProgressMessage` tandis qu'un hook de commande qui s'exécute pendant plus d'une seconde produit une sortie, et émet un `SDKHookResponseMessage` uniquement quand un hook [qui s'exécute en arrière-plan](/docs/fr/hooks#run-hooks-in-the-background) se termine                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `includePartialMessages`          | `boolean`                                                                                                                                                                                                      | `false`                                                 | Inclure les événements de message partiel                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `loadTimeoutMs`                   | `number`                                                                                                                                                                                                       | `60000`                                                 | *Alpha.* Délai d'expiration en millisecondes pour chaque appel `sessionStore.load()` et `sessionStore.listSubkeys()` lors de la matérialisation de la reprise. Si l'adaptateur ne se règle pas dans cette fenêtre, la requête échoue au lieu de rester bloquée. Ignoré quand `sessionStore` n'est pas défini                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `managedSettings`                 | `Settings`                                                                                                                                                                                                     | `undefined`                                             | Paramètres de niveau politique que votre processus hôte fournit à la session générée. Sur les machines avec des paramètres gérés déployés par l'administrateur, Claude Code ignore ceux-ci sauf si la source gérée de priorité la plus élevée de l'administrateur définit `parentSettingsBehavior: 'merge'`, et ne les fusionne jamais tandis qu'un [`policyHelper`](/docs/fr/settings-reference#policyhelper) fournit des paramètres gérés. Les valeurs fusionnées passent par un filtre restrictif uniquement ; [Restreindre les paramètres parents](/docs/fr/claude-apps-gateway#restrict-parent-settings) couvre ce que le filtre admet et les verrous `allowManaged*Only`. Un hôte qui définit [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/fr/env-vars) a trois clés lues directement à partir de cette charge utile à la place : sa [configuration de modèle](/docs/fr/model-config#restrict-model-selection) sur Claude Code v2.1.222 ou ultérieur, [`modelPricing`](/docs/fr/settings-reference#modelpricing) quand aucune source gérée ne la définit sur v2.1.246 ou ultérieur, et son entrée `ENABLE_TOOL_SEARCH` env sur v2.1.247 ou ultérieur                                                                                                                                                                                                                            |
| `maxBudgetUsd`                    | `number`                                                                                                                                                                                                       | `undefined`                                             | Arrêter la requête quand l'estimation du coût côté client atteint cette valeur en USD. Comparé à la même estimation que `total_cost_usd`. Pour les avertissements de précision et le comportement de réinitialisation, voir [Suivi des coûts et de l'utilisation](/docs/fr/agent-sdk/cost-tracking)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `maxThinkingTokens`               | `number`                                                                                                                                                                                                       | `undefined`                                             | *Déprécié :* Utilisez `thinking` à la place. Tokens maximum pour le processus de réflexion                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `maxTurns`                        | `number`                                                                                                                                                                                                       | `undefined`                                             | Tours agentiques maximum (allers-retours d'utilisation d'outils)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                                                                                                                        | `{}`                                                    | Configurations de serveur MCP                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `model`                           | `string`                                                                                                                                                                                                       | Par défaut de CLI                                       | Alias de modèle Claude ou nom de modèle complet. Voir [valeurs acceptées et ID spécifiques au fournisseur](/docs/fr/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`                                                                                                                | `undefined`                                             | Rappel pour gérer les demandes d'élicitation MCP. Appelé quand un serveur MCP demande une entrée utilisateur et aucun hook ne la gère en premier. Quand non fourni, les demandes d'élicitation non gérées sont automatiquement refusées                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                                                                                                                                  | `undefined`                                             | Définir le format de sortie pour les résultats de l'agent. Voir [Sorties structurées](/docs/fr/agent-sdk/structured-outputs) pour les détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `outputStyle`                     | `string`                                                                                                                                                                                                       | `undefined`                                             | Pas un champ `Options`. Définissez `outputStyle` dans l'objet [`settings`](/docs/fr/settings) en ligne ou un fichier de paramètres à la place. Voir [Activer un style de sortie](/docs/fr/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                                                                                                                       | Résolu automatiquement à partir du binaire natif groupé | Chemin vers l'exécutable Claude Code. Nécessaire uniquement si les dépendances optionnelles ont été ignorées lors de l'installation ou si votre plateforme ne figure pas dans l'ensemble pris en charge                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                                                                                                                            | `'default'`                                             | Mode de permission pour la session                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `permissionPromptToolName`        | `string`                                                                                                                                                                                                       | `undefined`                                             | Nom de l'outil MCP pour les invites de permission                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `permissionPrompts`               | `'host' \| 'none'`                                                                                                                                                                                             | `'host'`                                                | Qui répond aux invites de permission : `'host'` les achemine vers votre rappel [`canUseTool`](#canusetool) ou l'outil `permissionPromptToolName`, et `'none'` [refuse les appels qui auraient invité](/docs/fr/agent-sdk/permissions#how-permissions-are-evaluated). Nécessite Claude Code v2.1.259 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `persistSession`                  | `boolean`                                                                                                                                                                                                      | `true`                                                  | Quand `false`, désactive la persistance de session sur disque. Les sessions ne peuvent pas être reprises plus tard                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `planModeInstructions`            | `string`                                                                                                                                                                                                       | `undefined`                                             | Instructions de flux de travail personnalisées pour le mode plan. Quand `permissionMode` est `'plan'`, cette chaîne remplace le corps du flux de travail du mode plan par défaut. La CLI l'enveloppe toujours avec le préambule d'application en lecture seule et le pied de page du protocole ExitPlanMode                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                                                                                                                      | `[]`                                                    | Charger les plugins personnalisés à partir de chemins locaux. Voir [Plugins](/docs/fr/agent-sdk/plugins) pour les détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `projectConfigRoot`               | `string`                                                                                                                                                                                                       | `undefined`                                             | Chemin absolu du checkout de confiance dont `cwd` est une worktree. Claude Code lit les paramètres du projet, `.mcp.json`, et les commandes, agents, compétences, workflows, routines et styles de sortie du projet `.claude/` à partir de ce répertoire au lieu de `cwd`, et définit `CLAUDE_PROJECT_DIR` sur celui-ci. Les hooks, les scripts d'aide tels que `apiKeyHelper`, et les serveurs MCP stdio commencent avec ce répertoire comme répertoire de travail. Les fichiers `CLAUDE.md` et `.claude/rules/` se chargent toujours à partir de `cwd`. Nécessite Claude Code v2.1.275 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `promptSuggestions`               | `boolean`                                                                                                                                                                                                      | `false`                                                 | Activer les suggestions d'invite. Après un tour, Claude Code émet un message `prompt_suggestion` portant une invite utilisateur suivante prédite. Claude Code ne génère aucune suggestion pour certains tours, comme quand votre compte est proche ou à sa limite d'utilisation. Voir [Quand Claude Code ignore les suggestions](/docs/fr/interactive-mode#when-claude-code-skips-suggestions)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `resume`                          | `string`                                                                                                                                                                                                       | `undefined`                                             | ID de session à reprendre                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `resumeDropsTurn`                 | `string`                                                                                                                                                                                                       | `undefined`                                             | Avec `resumeSessionAt` : l'UUID du tour que la reprise tronquée a l'intention de rejeter. Claude Code refuse la reprise quand la plage rejetée contient quelque chose non attribuable à ce tour, comme des messages en attente absorbés ou des notifications de tâche, et nomme le drapeau `--resume-drops-turn` dans le message de rejet. Seul l'Agent SDK et les reprises en mode impression lisent la paire. Nécessite Claude Code v2.1.223 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `resumeSessionAt`                 | `string`                                                                                                                                                                                                       | `undefined`                                             | Reprendre la session à un UUID de message spécifique                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                                                                                                                          | `undefined`                                             | Configurer le comportement du sandbox par programmation. Voir [Paramètres du sandbox](#sandboxsettings) pour les détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `sessionId`                       | `string`                                                                                                                                                                                                       | Généré automatiquement                                  | Utiliser un UUID spécifique pour la session au lieu d'en générer un automatiquement                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `sessionStore`                    | [`SessionStore`](/docs/fr/agent-sdk/session-storage#the-sessionstore-interface)                                                                                                                                     | `undefined`                                             | Refléter les transcriptions de session vers un backend externe pour qu'un autre hôte puisse les reprendre. Voir [Persister les sessions vers un stockage externe](/docs/fr/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                                                                                                                         | `'batched'`                                             | *Alpha.* Mode de vidage pour `sessionStore`. Ignoré quand `sessionStore` n'est pas défini                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `settings`                        | `string \| Settings`                                                                                                                                                                                           | `undefined`                                             | Objet [paramètres](/docs/fr/settings) en ligne, chemin vers un fichier de paramètres, ou chaîne JSON en ligne. Remplit la couche de paramètres d'indicateur dans l'[ordre de précédence](/docs/fr/settings#settings-precedence). Modifiez à l'exécution avec [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                                                                                                                          | Paramètres par défaut de CLI (toutes les sources)       | Contrôler les paramètres du système de fichiers à charger. Passez `[]` pour désactiver les paramètres utilisateur, projet et locaux. [La politique gérée par endpoint](/docs/fr/managed-settings#delivery-mechanisms) se charge indépendamment ; les paramètres gérés par le serveur sont récupérés quand la session s'authentifie avec une credential d'organisation sur une [configuration éligible](/docs/fr/server-managed-settings#platform-availability). Voir [Utiliser les fonctionnalités Claude Code](/docs/fr/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `skills`                          | `string[] \| 'all'`                                                                                                                                                                                            | `undefined`                                             | Compétences disponibles pour la session. Passez `'all'` pour activer chaque compétence découverte, ou une liste de noms de compétences. Passez uniquement les noms exacts. Sur Agent SDK v0.3.221 ou ultérieur, le SDK rejette les noms mal formés et de forme wildcard avec une erreur avant de démarrer le processus Claude Code. Quand défini, le SDK ajoute l'outil Skill à `allowedTools` automatiquement. Si vous passez également `tools`, incluez `'Skill'` dans cette liste. Voir [Compétences](/docs/fr/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                                                                                                                                    | `undefined`                                             | Fonction personnalisée pour générer le processus Claude Code. Utilisez pour exécuter Claude Code dans des VM, des conteneurs ou des environnements distants                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `stderr`                          | `(data: string) => void`                                                                                                                                                                                       | `undefined`                                             | Rappel pour la sortie stderr                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `strictMcpConfig`                 | `boolean`                                                                                                                                                                                                      | `false`                                                 | Utiliser uniquement les serveurs passés dans `mcpServers` et ignorer le projet `.mcp.json`, les paramètres utilisateur, les serveurs MCP fournis par les plugins, et les [connecteurs claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `systemPrompt`                    | `string \| string[] \| { type: 'custom'; prompt: string \| string[]; snapshot?: boolean } \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean; snapshot?: boolean }` | `undefined` (invite minimale)                           | Configuration de l'invite système. Passez une chaîne pour une invite personnalisée, ou `{ type: 'preset', preset: 'claude_code' }` pour utiliser l'invite système de Claude Code. Passez un tableau de chaînes avec la constante exportée `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` entre les parties statiques et par requête pour [mettre en cache la partie statique d'une invite personnalisée](/docs/fr/agent-sdk/modifying-system-prompts#cache-the-static-part-of-a-custom-prompt). Lors de l'utilisation de la forme d'objet prédéfini, ajoutez `append` pour l'étendre avec des instructions supplémentaires, et définissez `excludeDynamicSections: true` pour déplacer le contexte par session dans le premier message utilisateur pour une [meilleure réutilisation du cache d'invite sur les machines](/docs/fr/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines). Définissez `snapshot: false` pour reconstruire l'invite à chaque requête au lieu de [réutiliser l'invite que la session a enregistrée à sa première requête](/docs/fr/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session). Pour définir `snapshot` sur une invite personnalisée, passez la forme `{ type: 'custom', prompt }`. La forme `{ type: 'custom' }` et le champ `snapshot` nécessitent TypeScript Agent SDK v0.3.257 ou ultérieur |
| `taskBudget`                      | `{ total: number }`                                                                                                                                                                                            | `undefined`                                             | *Alpha.* Budget de tâche côté API en tokens. Quand défini, le modèle est informé de son budget de tokens restant pour qu'il puisse adapter l'utilisation des outils et terminer avant la limite                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                                                                                                                            | `{ type: 'adaptive' }` pour les modèles pris en charge  | Contrôle le comportement de réflexion/raisonnement de Claude. Voir [`ThinkingConfig`](#thinkingconfig) pour les options                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `title`                           | `string`                                                                                                                                                                                                       | `undefined`                                             | Titre d'affichage pour la session. Lors de la reprise via `resume` ou `continue`, le titre persistant de la session reprise a la priorité ; utilisez [`renameSession()`](#renamesession) pour renommer une session existante                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `toolAliases`                     | `Record<string, string>`                                                                                                                                                                                       | `undefined`                                             | Mapper les noms d'outils intégrés aux noms d'outils MCP pour que Claude appelle votre implémentation MCP à la place de l'intégrée. Par exemple, `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                                                                                                                                    | `undefined`                                             | Configuration pour le comportement des outils intégrés. Voir [`ToolConfig`](#toolconfig) pour les détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                                                                                                                        | `undefined`                                             | Configuration des outils. Passez un tableau de noms d'outils ou utilisez le prédéfini pour obtenir les outils par défaut de Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |

<h4 id="handle-slow-or-stalled-api-responses">
  Gérer les réponses API lentes ou bloquées
</h4>

Le sous-processus CLI lit plusieurs variables d'environnement qui contrôlent les délais d'expiration de l'API et la détection de blocage. Transmettez-les via l'option `env` :

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const result = query({
  prompt: "Analyze this code",
  options: {
    env: {
      ...process.env,
      API_TIMEOUT_MS: "120000",
      CLAUDE_CODE_MAX_RETRIES: "2",
      CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS: "120000",
    },
  },
});
```

* `API_TIMEOUT_MS` : délai d'expiration par requête sur le client Anthropic, en millisecondes. Par défaut `600000`. S'applique à la boucle principale et à tous les sous-agents.
* `CLAUDE_CODE_MAX_RETRIES` : tentatives API maximales. Par défaut `10`, limité à `15`. Chaque tentative obtient sa propre fenêtre `API_TIMEOUT_MS`, donc le pire cas de temps mural est approximativement `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` plus le backoff. Pour les exécutions sans surveillance qui doivent attendre des pannes plus longues, définissez [`CLAUDE_CODE_RETRY_WATCHDOG=1`](/docs/fr/errors#tune-retry-behavior) : il réessaye les erreurs de capacité transitoires indéfiniment et, sur Claude Code v2.1.199 ou ultérieur, augmente la valeur par défaut pour les autres erreurs transitoires à `300` et supprime le plafond sur cette variable.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` : chien de garde de blocage pour les sous-agents. Tandis que le chien de garde de flux est activé, la valeur par défaut est `CLAUDE_STREAM_IDLE_TIMEOUT_MS` plus 5 minutes, ce qui donne `600000` sauf si vous augmentez cette variable. Avec le chien de garde de flux désactivé, la valeur par défaut est `600000`. Avant v2.1.257, la valeur par défaut était toujours `600000`.

  Le minuteur se réinitialise à chaque événement de flux. En cas de blocage, Claude Code abandonne le sous-agent et signale le blocage au parent. Pour un sous-agent d'arrière-plan, il marque également la tâche comme échouée et joint tout résultat partiel.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` avec `CLAUDE_STREAM_IDLE_TIMEOUT_MS` : chien de garde de flux qui abandonne la requête quand les en-têtes sont arrivés mais que le corps de la réponse cesse de diffuser. Le chien de garde est activé par défaut pour tous les fournisseurs ; définissez `CLAUDE_ENABLE_STREAM_WATCHDOG=0` pour le désactiver. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` par défaut à `300000` et est limité à ce minimum. Après l'abandon, [Tentatives automatiques](/docs/fr/errors#automatic-retries) couvre ce que Claude Code fait, en fonction de la progression de la réponse.

  Tandis que le chien de garde attend une réponse qu'une passerelle derrière `ANTHROPIC_BASE_URL` maintient ouverte avec des pings de maintien de connexion, un hôte qui définit `includePartialMessages` continue de recevoir des événements de flux `ping` [](#sdkpartialassistantmessage), donc lisez ces cadres comme une vivacité plutôt que de mettre fin à la session sur le silence. Avant v2.1.257, les cadres s'arrêtaient 5 minutes après le dernier événement de flux réel.

<h3 id="query-object">
  Objet `Query`
</h3>

Interface retournée par la fonction `query()`.

```typescript theme={null}
interface Query extends AsyncGenerator<SDKMessage, void> {
  interrupt(): Promise<SDKControlInterruptResponse | undefined>;
  rewindFiles(
    userMessageId: string,
    options?: { dryRun?: boolean }
  ): Promise<RewindFilesResult>;
  setPermissionMode(mode: PermissionMode): Promise<void>;
  setModel(model?: string): Promise<void>;
  setMaxThinkingTokens(maxThinkingTokens: number | null): Promise<void>;
  applyFlagSettings(settings: {
    [K in keyof Settings]?: K extends 'effortLevel'
      ? 'low' | 'medium' | 'high' | 'xhigh' | 'max' | null
      : Settings[K] | null;
  }): Promise<void>;
  updateSettings(
    source: 'localSettings' | 'userSettings',
    settings: Record<string, unknown>,
  ): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  getContextUsage(opts?: {
    detail?: 'summary' | 'full';
  }): Promise<SDKControlGetContextUsageResponse>;
  readFile(
    path: string,
    options?: { maxBytes?: number; encoding?: 'utf-8' | 'base64' }
  ): Promise<SDKControlReadFileResponse | null>;
  reloadSkills(): Promise<SDKControlReloadSkillsResponse>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  readMcpResource(serverName: string, uri: string): Promise<SDKControlMcpReadResourceResponse>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  close(): void;
}
```

<h4 id="methods">
  Méthodes
</h4>

| Méthode                                | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | Interrompt la requête. Disponible uniquement en mode d'entrée en diffusion. Quand la CLI annonce la capacité `interrupt_receipt_v1` dans [`SDKSystemMessage.capabilities`](#sdksystemmessage), se résout avec une [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) listant les messages qui étaient en attente quand l'interruption est arrivée. Se résout à `undefined` sur les CLI antérieures à v2.1.205                                                                                                                                                                  |
| `rewindFiles(userMessageId, options?)` | Restaure les fichiers à leur état au message utilisateur spécifié. Passez `{ dryRun: true }` pour prévisualiser les modifications. Nécessite `enableFileCheckpointing: true`. Voir [Sauvegarde de fichiers](/docs/fr/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                             |
| `setPermissionMode()`                  | Change le mode de permission (disponible uniquement en mode d'entrée en diffusion)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `setModel()`                           | Change le modèle (disponible uniquement en mode d'entrée en diffusion). Passer `undefined` ou la chaîne `"default"` réinitialise au [modèle par défaut de Claude Code](/docs/fr/model-config)                                                                                                                                                                                                                                                                                                                                                                                                  |
| `setMaxThinkingTokens()`               | *Déprécié :* Utilisez l'option `thinking` à la place. Change les tokens de réflexion maximum. Passer `null` réinitialise la réflexion à la valeur par défaut de la session : un remplacement en milieu de session est effacé, et la réflexion reste désactivée pour les sessions qui l'ont désactivée                                                                                                                                                                                                                                                                                     |
| `applyFlagSettings(settings)`          | Fusionne les paramètres dans la couche de paramètres d'indicateur de la session à l'exécution (disponible uniquement en mode d'entrée en diffusion). Voir [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                     |
| `updateSettings(source, settings)`     | Écrit une clé autorisée dans le fichier de paramètres locaux du projet ou votre fichier de paramètres utilisateur, pour que la valeur persiste pour les sessions ultérieures. Voir [`updateSettings()`](#updatesettings). Nécessite TypeScript SDK v0.3.257 ou ultérieur, qui regroupe Claude Code v2.1.257                                                                                                                                                                                                                                                                               |
| `initializationResult()`               | Retourne le résultat d'initialisation complet incluant les commandes prises en charge, les modèles, les informations de compte et la configuration du style de sortie                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `reinitialize()`                       | Renvoie la demande de contrôle `initialize` au CLI en cours d'exécution et retourne un résultat frais au lieu du résultat de première connexion mis en cache. Utilisez-le après une interruption de transport, comme se reconnecter à une session après une déconnexion, pour que les demandes de permission en attente atteignent à nouveau votre rappel `canUseTool`. Rendez le rappel idempotent par ID de requête, car une requête dont la réponse a été perdue est distribuée à nouveau. Nécessite Claude Code v2.1.195 ou ultérieur                                                 |
| `supportedCommands()`                  | Retourne les commandes disponibles. À partir d'Agent SDK v0.3.216, la liste reflète les changements de commande en milieu de session ; voir [`SDKCommandsChangedMessage`](#sdkcommandschangedmessage)                                                                                                                                                                                                                                                                                                                                                                                     |
| `supportedModels()`                    | Retourne les modèles disponibles avec les informations d'affichage                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `supportedAgents()`                    | Retourne les sous-agents disponibles en tant que [`AgentInfo`](#agentinfo)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `mcpServerStatus()`                    | Retourne l'état des serveurs MCP connectés en tant que [`McpServerStatus`](#mcpserverstatus)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `getContextUsage(opts?)`               | Retourne une [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse) ventilant l'utilisation de la fenêtre de contexte de la session par catégorie, compétence et outil. Avec la valeur par défaut `detail`, c'est les mêmes données que `/context` affiche dans une session interactive. L'[option `detail`](#sdkcontrolgetcontextusageresponse) nécessite Agent SDK v0.3.257 ou ultérieur                                                                                                                                                                             |
| `readFile(path, options?)`             | Lit un fichier du système de fichiers de la session. Claude Code résout le chemin par rapport à `cwd` ; [Ce que `readFile()` peut lire](#what-readfile-can-read) liste les fichiers qu'il sert. Passez `{ maxBytes }` pour modifier le plafond de lecture (par défaut 1 Mo, plafond 10 Mo) et `{ encoding: 'base64' }` pour les fichiers binaires tels que les images. Se résout avec une [`SDKControlReadFileResponse`](#sdkcontrolreadfileresponse), ou `null` sur refus de permission, un fichier manquant, ou une erreur de transport. Nécessite TypeScript SDK v0.2.121 ou ultérieur |
| `reloadSkills()`                       | Recharge les compétences à partir du disque, donc les compétences que vous ajoutez ou modifiez en milieu de session deviennent disponibles pour la session en cours d'exécution. Se résout avec une [`SDKControlReloadSkillsResponse`](#sdkcontrolreloadskillsresponse) listant les compétences disponibles après le rechargement. Nécessite Agent SDK v0.3.163 ou ultérieur                                                                                                                                                                                                              |
| `accountInfo()`                        | Retourne les informations de compte                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `reconnectMcpServer(serverName)`       | Reconnecter un serveur MCP par nom. Si le nom correspond également à une entrée dans un fichier de paramètres tel que `.mcp.json` ou `~/.claude.json`, Claude Code reconnecte le serveur que vous avez configuré via [`mcpServers`](#options) ou `setMcpServers()`, pas l'entrée du fichier de paramètres. Cet ordre de résolution nécessite Claude Code v2.1.257 ou ultérieur                                                                                                                                                                                                            |
| `toggleMcpServer(serverName, enabled)` | Activer ou désactiver un serveur MCP par nom, avec la même résolution de nom que `reconnectMcpServer()`. La désactivation déconnecte le serveur                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `setMcpServers(servers)`               | Remplacer dynamiquement l'ensemble des serveurs MCP pour cette session. Se résout avec un [`McpSetServersResult`](#mcpsetserversresult) nommant les serveurs qui ont été ajoutés et supprimés, et toute erreur                                                                                                                                                                                                                                                                                                                                                                            |
| `readMcpResource(serverName, uri)`     | *Alpha.* Lit une ressource MCP Apps `ui://` à partir d'un serveur MCP connecté pour que votre application puisse afficher le widget d'un outil. Se résout avec une [`SDKControlMcpReadResourceResponse`](#sdkcontrolmcpreadresourceresponse). Nécessite TypeScript Agent SDK v0.3.280 ou ultérieur                                                                                                                                                                                                                                                                                        |
| `streamInput(stream)`                  | Diffuser les messages d'entrée vers la requête pour les conversations multi-tours                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `stopTask(taskId)`                     | Arrêter une tâche de fond en cours d'exécution par ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `close()`                              | Fermer la requête et terminer le processus sous-jacent. Termine de force la requête et nettoie toutes les ressources                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

Change les [paramètres](/docs/fr/settings) sur une session en cours d'exécution sans redémarrer la requête. Utilisez-le quand un paramètre qui n'a pas de setter dédié doit changer en milieu de session, comme resserrer `permissions` après que l'agent ait lu une entrée non fiable. `setModel()` et `setPermissionMode()` sont des setters dédiés pour ces deux clés ; `applyFlagSettings()` est la forme générale qui accepte n'importe quel sous-ensemble des clés de paramètres, et passer `model` ici se comporte de la même manière que `setModel()`.

Seules certaines clés prennent effet en milieu de session :

* **Appliquées au tour suivant** : `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. Basculer `agent` applique également le remplacement de modèle et les hooks de cet agent au tour suivant. Son invite système s'applique au tour suivant, ou, dans une session qui [réutilise une invite système enregistrée](/docs/fr/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session), une fois que la session est compactée.
* **Appliquées pendant le tour actuel** : `model`. Si vous basculez `model` tandis que Claude travaille sur un tour, la réponse que Claude génère déjà se termine sur l'ancien modèle, et le reste du tour, commençant par le prochain appel que Claude Code fait au modèle, utilise le nouveau. Les sous-agents conservent leur propre modèle. Avant v2.1.212, un basculement en milieu de tour attendait le tour suivant.
* **Aucun effet en milieu de session** : les options d'invite système. Celles-ci sont résolues une fois au démarrage, donc la session en cours d'exécution conserve la valeur d'origine même si l'appel réussit. Pour les modifier, démarrez une nouvelle session.

`effortLevel` accepte un nom de [niveau d'effort](/docs/fr/model-config#adjust-effort-level). Il accepte également `"ultracode"`, qui exécute la session au niveau d'effort `xhigh` et active [ultracode](/docs/fr/workflows#let-claude-decide-with-ultracode). `applyFlagSettings()` déclare `effortLevel` sans cette valeur, donc passez l'équivalent `{ ultracode: true }` en TypeScript. La valeur `ultracode` nécessite Claude Code v2.1.203 ou ultérieur et n'est acceptée que par `applyFlagSettings()`, pas par la clé `effortLevel` dans un fichier de paramètres.

Les valeurs sont écrites dans la couche de paramètres d'indicateur, la même couche que l'option `settings` en ligne de `query()` remplit au démarrage. C'est le même niveau que la [section de précédence sur la page](#settings-precedence) appelle les options programmatiques.

Les appels successifs fusionnent superficiellement les clés de niveau supérieur. Un deuxième appel avec `{ permissions: {...} }` remplace l'objet `permissions` entier de l'appel précédent plutôt que de le fusionner profondément.

Pour effacer une clé que vous avez définie avec `applyFlagSettings()`, passez `null` pour cette clé. La plupart des clés reviennent alors d'abord à une valeur que l'option `settings` de `query()` a définie au démarrage, puis aux sources de précédence inférieure. Un `model` effacé réinitialise au [modèle par défaut de Claude Code](/docs/fr/model-config), même quand un fichier de paramètres définit `model`. Passer `undefined` n'a aucun effet car la sérialisation JSON le supprime.

Trois clés en plus de `model` réinitialisent l'état de session au lieu de revenir :

* `effortLevel: null` retourne la session au niveau d'effort par défaut du modèle, pas à l'option `effort` de `query()` ou un `effortLevel` d'un fichier de paramètres.
* `agent: null` exécute le thread principal sans agent, à partir du tour suivant, plutôt que de restaurer l'option `agent` de `query()` ou un `agent` d'un fichier de paramètres. Si l'agent effacé avait appliqué son propre modèle, la session revient au modèle qu'elle a résolu au démarrage.
* `ultracode: null` désactive ultracode, comme `false` le fait, plutôt que de restaurer une valeur `ultracode` d'un fichier de paramètres. La session conserve son niveau d'effort actuel, donc passez `effortLevel` dans le même appel pour le modifier.

Disponible uniquement en mode d'entrée en diffusion, la même contrainte que `setModel()` et `setPermissionMode()`.

L'exemple ci-dessous bascule le modèle actif en milieu de session, puis efface le remplacement pour que le modèle revienne au [modèle par défaut de Claude Code](/docs/fr/model-config).

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const q = query({ prompt: messageStream });

// Remplacer le modèle pour le reste de la session
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Plus tard : effacer le remplacement ; le modèle réinitialise au modèle par défaut de Claude Code
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` est TypeScript uniquement. Le SDK Python n'expose pas de méthode équivalente.
</Note>

<h4 id="updatesettings">
  `updateSettings()`
</h4>

Écrit une clé autorisée dans un fichier de paramètres sur disque, pour que la valeur persiste pour les sessions ultérieures qui chargent cette source. Chaque source accepte une clé, avec une valeur de chaîne :

* **`"localSettings"`** : accepte `outputStyle` et le fusionne dans le fichier de paramètres locaux du projet, `.claude/settings.local.json`. Le nouveau style prend effet à la requête suivante de la session.
* **`"userSettings"`** : accepte `effortLevel` et l'enregistre comme le [niveau d'effort](/docs/fr/model-config#adjust-effort-level) par défaut pour le modèle actuel de la session, sous [`modelSettings`](/docs/fr/settings-reference#modelsettings) dans votre fichier de paramètres utilisateur. Passer `max` n'écrit rien, car `max` est session uniquement. La session en cours d'exécution conserve son niveau d'effort actuel de toute façon, donc appelez [`applyFlagSettings()`](#applyflagsettings) quand vous voulez aussi changer cela. Cette source nécessite TypeScript SDK v0.3.277 ou ultérieur, qui regroupe Claude Code v2.1.277.

L'appel rejette quand la requête porte une autre clé, quand la session s'exécute sur un transport distant, et quand [`settingSources`](#options) de la session excluent la source que vous nommez. La suppression d'une clé n'est pas prise en charge.

<h3 id="warmquery">
  `WarmQuery`
</h3>

Handle retourné par [`startup()`](#startup). Le sous-processus est déjà généré et initialisé, donc appeler `query()` sur ce handle écrit l'invite directement dans un processus prêt sans latence de démarrage.

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

<h4 id="methods-2">
  Méthodes
</h4>

| Méthode         | Description                                                                                                                                |
| :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| `query(prompt)` | Envoyer une invite au sous-processus préchauffé et retourner une [`Query`](#query-object). Ne peut être appelé qu'une fois par `WarmQuery` |
| `close()`       | Fermer le sous-processus sans envoyer d'invite. Utilisez ceci pour abandonner une requête chaude qui n'est plus nécessaire                 |

`WarmQuery` implémente `AsyncDisposable`, il peut donc être utilisé avec `await using` pour le nettoyage automatique.

<h3 id="sdkcontrolinitializeresponse">
  `SDKControlInitializeResponse`
</h3>

Type de retour de `initializationResult()`. Contient les données d'initialisation de session.

```typescript theme={null}
type SDKControlInitializeResponse = {
  commands: SlashCommand[];
  agents: AgentInfo[];
  output_style: string;
  available_output_styles: string[];
  models: ModelInfo[];
  account: AccountInfo;
  fast_mode_state?: "off" | "cooldown" | "on";
  fast_mode_disabled_reason?: FastModeDisabledReason;
  hooks_applied?: boolean;
};
```

`hooks_applied` signale si Claude Code a enregistré les `hooks` que la demande `initialize` portait. Le SDK envoie cette demande une fois quand la session démarre et à nouveau à chaque appel [`reinitialize()`](#query-object). Le champ nécessite Agent SDK v0.3.238 ou ultérieur.

Claude Code omet le champ quand la demande ne portait aucun hook. Quand la demande portait des hooks, la valeur dépend de si c'est la première initialisation de la session et, pour une répétée, de comment elle a atteint la session :

* `true` : Claude Code a enregistré les hooks. La première initialisation d'une session retourne cette valeur. Une initialisation répétée envoyée sur stdin de la CLI retourne également cette valeur. Dans ce cas, les hooks dans la nouvelle demande remplacent les hooks enregistrés plus tôt.
* `false` : Claude Code a ignoré les hooks. Une initialisation répétée envoyée à une session distante retourne cette valeur, donc un deuxième client qui rejoint une session ne peut pas remplacer les hooks que le premier client a enregistrés.

Avant Agent SDK v0.3.238, la réponse ne portait jamais le champ, et Claude Code ignorait `hooks` à chaque initialisation répétée.

La réponse signale toujours `fast_mode_state`, et quand quelque chose bloque le [mode rapide](/docs/fr/fast-mode), `fast_mode_disabled_reason` porte le code de raison à côté, pour que vous puissiez expliquer l'état bloqué au lieu de le redériver. Les deux comportements nécessitent Claude Code v2.1.219 ou ultérieur. Avant v2.1.219, la réponse omettait `fast_mode_state` quand le mode rapide n'était pas disponible et ne portait jamais de raison. Pour les codes de raison et leurs significations, voir [`fast_mode_disabled_reason`](#sdkresultmessage) sur le message de résultat.

Le wrapper de réponse de contrôle pour une `initialize` réussie porte également un tableau `pending_permission_requests`. Le champ se trouve sur le wrapper de réponse lui-même, pas dans la charge utile `SDKControlInitializeResponse` ci-dessus. Chaque entrée est un message `control_request` complet avec la même forme `{ type: "control_request", request_id, request }` que la session diffuse pour les demandes de permission lors de l'exécution.

Le tableau liste les demandes de permission que ce processus Claude Code a émises et n'a pas encore résolues. Le SDK lit le tableau pour vous et distribue chaque entrée à votre rappel [`canUseTool`](#canusetool), la même redistribution que [`reinitialize()`](#query-object) déclenche après une interruption de transport. Gérez les ID de requête répétés de manière idempotente, car une entrée peut répéter une requête que le rappel a déjà reçue avant que la connexion ne soit interrompue.

Le tableau est toujours présent sur une réponse `initialize` réussie et est vide quand ce processus n'a aucune demande de permission non résolue. Nécessite Claude Code v2.1.268 ou ultérieur. Les versions antérieures pourraient omettre le champ, donc si vous analysez le protocole de fil vous-même, traitez un champ manquant comme une CLI plus ancienne plutôt que comme une preuve que rien n'est en attente.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

Le reçu d'interruption : la valeur que [`interrupt()`](#query-object) se résout avec sur une CLI qui annonce la capacité `interrupt_receipt_v1` dans [`SDKSystemMessage.capabilities`](#sdksystemmessage). Nécessite Claude Code v2.1.205 ou ultérieur. Les CLI antérieures répondent à l'interruption avec une charge utile de succès vide, donc `interrupt()` se résout à `undefined`.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
  cancelled?: string[];
};
```

`still_queued` liste les UUID des messages utilisateur qui étaient en attente quand l'interruption est arrivée : messages toujours en attente, plus tout message que Claude Code avait déjà retiré de la file d'attente pour le tour suivant. Une fois que le premier tour de la session a commencé, Claude Code traite les messages listés après l'interruption sauf si vous les annulez d'abord, et peut en fusionner plusieurs en un tour. Si vous interrompez avant que le premier tour ne commence, Claude Code abandonne ce tour dès qu'il commence, et les messages listés dans ce tour ne reçoivent aucune réponse.

Utilisez le reçu pour décider si vous devez renvoyer quelque chose. Un message listé que vous ne cancélez pas entre dans la conversation que ce soit ou non il reçoit une réponse, donc le renvoyer livre à Claude deux fois.

Interprétez la liste avec ces avertissements :

* Seuls les messages qui ont été mis en attente avec un UUID apparaissent. Un tableau vide ne signifie pas que rien d'autre ne s'exécutera.
* Seuls les messages du thread principal sont listés. Les messages adressés à un sous-agent sont hors de portée.
* La liste peut inclure des UUID que votre client n'a jamais envoyés, comme les déclencheurs de [tâche programmée](/docs/fr/scheduled-tasks). Ignorez les UUID que vous ne reconnaissez pas au lieu de les traiter comme une erreur.

Un client qui pilote le protocole de contrôle de la CLI directement, plutôt que via `interrupt()`, peut définir `cancel_queued: true` sur la demande de contrôle `interrupt`. Claude Code v2.1.219 et ultérieur annonce le support avec la capacité `interrupt_cancel_queued_v1` dans [`SDKSystemMessage.capabilities`](#sdksystemmessage) ; les CLI plus anciennes ignorent le champ et laissent les messages en attente s'exécuter comme d'habitude. Une telle interruption annule également chaque message qui serait autrement listé sous `still_queued` : le reçu les liste sous `cancelled` à la place, `still_queued` est vide, et aucun d'eux ne s'exécute.

La liste `cancelled` porte les mêmes avertissements que `still_queued`. La méthode `interrupt()` n'envoie jamais `cancel_queued`, donc les reçus qu'elle se résout avec ne portent pas `cancelled`.

Le reçu est un instantané pris au moment où l'interruption est traitée, et sur une interruption propre, il arrive avant le [`SDKResultMessage`](#sdkresultmessage) du tour interrompu. Lisez le reçu plutôt que d'inspecter la file d'attente après ce résultat : la boucle démarre immédiatement le tour en attente suivant, donc la file d'attente que vous inspectez après le résultat a déjà changé.

<h3 id="sdkcontrolgetcontextusageresponse">
  `SDKControlGetContextUsageResponse`
</h3>

Type de retour de [`getContextUsage()`](#query-object). Avec la valeur par défaut `detail`, c'est la même charge utile que Claude Code affiche pour la commande `/context` dans une session interactive, donc à côté des comptes de tokens, elle porte des champs d'affichage tels que `color` et `gridRows` que Claude Code utilise pour dessiner la grille d'utilisation `/context`.

L'argument optionnel `detail` de la méthode choisit comment Claude Code compte chaque catégorie. Avec la valeur par défaut, `'full'`, Claude Code compte chaque catégorie avec des demandes d'API de comptage de tokens. Passez `{ detail: 'summary' }` pour obtenir une réponse de l'utilisation de la dernière réponse et des estimations locales à la place. Aucune demande de comptage de tokens ne sort, et les nombres par catégorie sont approximatifs. L'argument `detail` nécessite Agent SDK v0.3.257 ou ultérieur.

Quand vous envoyez `/context` comme invite au lieu d'appeler la méthode, Claude Code joint une charge utile [`SDKContextUsage`](#sdkcontextusage) au champ `context_usage` du message assistant qui livre le résultat. Ce champ nécessite Agent SDK v0.3.232 ou ultérieur.

```typescript theme={null}
type SDKControlGetContextUsageResponse = {
  categories: {
    name: string;
    tokens: number;
    color: string;
    isDeferred?: boolean;
  }[];
  totalTokens: number;
  maxTokens: number;
  rawMaxTokens: number;
  percentage: number;
  gridRows: {
    color: string;
    isFilled: boolean;
    categoryName: string;
    tokens: number;
    percentage: number;
    squareFullness: number;
  }[][];
  model: string;
  memoryFiles: {
    path: string;
    type: string;
    tokens: number;
  }[];
  mcpTools: {
    name: string;
    serverName: string;
    tokens: number;
    isLoaded?: boolean;
  }[];
  deferredBuiltinTools?: {
    name: string;
    tokens: number;
    isLoaded: boolean;
  }[];
  systemTools?: {
    name: string;
    tokens: number;
  }[];
  systemPromptSections?: {
    name: string;
    tokens: number;
  }[];
  agents: {
    agentType: string;
    source: string;
    tokens: number;
  }[];
  slashCommands?: {
    totalCommands: number;
    includedCommands: number;
    tokens: number;
  };
  skills?: {
    totalSkills: number;
    includedSkills: number;
    tokens: number;
    skillFrontmatter: {
      name: string;
      source: string;
      tokens: number;
    }[];
  };
  autoCompactThreshold?: number;
  isAutoCompactEnabled: boolean;
  messageBreakdown?: {
    toolCallTokens: number;
    toolResultTokens: number;
    attachmentTokens: number;
    assistantMessageTokens: number;
    userMessageTokens: number;
    redirectedContextTokens: number;
    unattributedTokens: number;
    toolCallsByType: {
      name: string;
      callTokens: number;
      resultTokens: number;
    }[];
    attachmentsByType: {
      name: string;
      tokens: number;
    }[];
  };
  apiUsage: {
    input_tokens: number;
    output_tokens: number;
    cache_creation_input_tokens: number;
    cache_read_input_tokens: number;
  } | null;
};
```

Lisez l'attribution de tokens à partir des champs de collection :

* `categories` contient les totaux par catégorie.
* `mcpTools` et `agents` attribuent les tokens aux outils MCP individuels et aux sous-agents.
* `memoryFiles` liste chaque fichier de mémoire chargé avec son coût.
* `skills.skillFrontmatter` attribue les tokens de la liste des compétences à chaque compétence incluse. Les comptes par compétence mesurent chaque entrée de liste de compétences comme Claude Code l'envoie réellement, ce qui peut être plus court que le frontmatter complet de la compétence. Comparez `skills.totalSkills` avec `skills.includedSkills` pour voir si chaque compétence découverte a fait son chemin dans la liste.

`totalTokens` est l'utilisation de contexte actuelle de la session, et `maxTokens` est la fenêtre par rapport à laquelle l'utilisation est mesurée. Cette fenêtre est la fenêtre de contexte du modèle, ou la fenêtre de compaction automatique inférieure quand une s'applique. `rawMaxTokens` porte la même valeur que `maxTokens`, et `percentage` est `totalTokens` en pourcentage arrondi de cette fenêtre.

Claude Code laisse les diagnostics optionnels `deferredBuiltinTools`, `systemTools`, et `systemPromptSections` non définis, donc attendez-vous à ce qu'ils soient absents même si le type les déclare.

<h3 id="sdkcontrolreadfileresponse">
  `SDKControlReadFileResponse`
</h3>

Type de retour de [`readFile()`](#query-object).

```typescript theme={null}
type SDKControlReadFileResponse = {
  contents: string;
  absPath: string;
  truncated?: boolean;
  encoding?: 'base64';
};
```

`contents` contient le texte du fichier, ou les données base64 quand vous avez demandé `encoding: 'base64'` ; le champ `encoding` de la réponse est défini à `'base64'` dans ce cas. `absPath` est le chemin absolu résolu. `truncated` est défini quand le fichier était plus long que le plafond `maxBytes` et le contenu a été coupé à cette limite.

<h4 id="what-readfile-can-read">
  Ce que `readFile()` peut lire
</h4>

`readFile()` sert un ensemble plus étroit de fichiers que l'outil Read :

* Un fichier régulier à l'intérieur de l'un des répertoires de travail de la session, comme `cwd` et `additionalDirectories`
* Quelques fichiers propres à Claude Code pour la session, comme les résultats des outils

Les règles de refus et de demande de Read bloquent toujours un chemin correspondant, et une règle d'autorisation Read large n'ouvre pas le reste du système de fichiers à `readFile()`. Pour tout le reste, l'appel se résout avec `null`.

<h3 id="sdkcontrolreloadskillsresponse">
  `SDKControlReloadSkillsResponse`
</h3>

Type de retour de [`reloadSkills()`](#query-object).

```typescript theme={null}
type SDKControlReloadSkillsResponse = {
  skills: SlashCommand[];
};
```

`skills` liste les compétences disponibles après le rechargement, dans la même forme [`SlashCommand`](#slashcommand) que `supportedCommands()` retourne.

<h3 id="sdkcontrolmcpreadresourceresponse">
  `SDKControlMcpReadResourceResponse`
</h3>

Type de retour de [`readMcpResource()`](#query-object), portant le résultat `resources/read` du serveur MCP. Nécessite TypeScript Agent SDK v0.3.280 ou ultérieur.

```typescript theme={null}
type SDKControlMcpReadResourceResponse = {
  contents: {
    uri: string;
    mimeType?: string;
    text?: string;
    blob?: string;
    _meta?: Record<string, unknown>;
  }[];
};
```

Passez à `readMcpResource()` le nom du serveur tel que `mcpServerStatus()` le signale et un URI `ui://`, comme le `ui.resourceUri` qu'un outil déclare dans son [`_meta`](#mcpserverstatus). L'appel rejette pour tout autre schéma d'URI, pour un [serveur MCP SDK](#createsdkmcpserver) que votre application héberge elle-même, et pour un serveur qui n'est pas connecté. C'est disponible quand le message init's [`capabilities`](#sdksystemmessage) incluent `mcp_read_resource_v1`.

Chaque entrée `contents` est un élément de contenu tel que le serveur l'a envoyé. `blob` contient les données base64 pour un élément binaire, et `_meta` est le propre `_meta` de l'élément, où un serveur MCP Apps met le `ui.csp` et `ui.permissions` de la ressource. Le contenu est du HTML tiers non fiable, donc rendez-le dans un sandbox.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Configuration pour un sous-agent défini par programmation.

```typescript theme={null}
type AgentDefinition = {
  description: string;
  tools?: string[];
  disallowedTools?: string[];
  prompt: string;
  model?: string;
  mcpServers?: AgentMcpServerSpec[];
  skills?: string[];
  initialPrompt?: string;
  maxTurns?: number;
  background?: boolean;
  omitClaudeMd?: boolean;
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| Champ                                 | Requis | Description                                                                                                                                                                                                                                                                                                                                                                                                         |
| :------------------------------------ | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `description`                         | Oui    | Description en langage naturel de quand utiliser cet agent                                                                                                                                                                                                                                                                                                                                                          |
| `tools`                               | Non    | Tableau de noms d'outils autorisés. S'il est omis, hérite chaque [outil disponible pour les sous-agents](/docs/fr/sub-agents#available-tools). Pour précharger les compétences dans le contexte de l'agent, utilisez le champ `skills` plutôt que de lister `'Skill'` ici                                                                                                                                                |
| `disallowedTools`                     | Non    | Tableau de noms d'outils à explicitement interdire pour cet agent. Les modèles au niveau du serveur MCP sont également acceptés : `mcp__server` ou `mcp__server__*` supprime chaque outil de ce serveur, et `mcp__*` supprime chaque outil MCP de n'importe quel serveur                                                                                                                                            |
| `prompt`                              | Oui    | L'invite système de l'agent                                                                                                                                                                                                                                                                                                                                                                                         |
| `model`                               | Non    | Remplacement de modèle pour cet agent. Accepte un alias tel que `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, ou un ID de modèle complet. `'inherit'` utilise le modèle principal. Quand vous l'omettez, Claude Code choisit le modèle dans l'[ordre de modèle de sous-agent](/docs/fr/sub-agents#choose-a-model)                                                                                             |
| `mcpServers`                          | Non    | Spécifications de serveur MCP pour cet agent                                                                                                                                                                                                                                                                                                                                                                        |
| `skills`                              | Non    | Tableau de noms de compétences à précharger dans le contexte de l'agent                                                                                                                                                                                                                                                                                                                                             |
| `initialPrompt`                       | Non    | Auto-soumis comme le premier tour utilisateur quand cet agent s'exécute en tant qu'agent du thread principal                                                                                                                                                                                                                                                                                                        |
| `maxTurns`                            | Non    | Nombre maximum de tours agentiques (allers-retours API) avant arrêt                                                                                                                                                                                                                                                                                                                                                 |
| `background`                          | Non    | Exécuter cet agent en tant que tâche de fond non-bloquante quand invoqué                                                                                                                                                                                                                                                                                                                                            |
| `omitClaudeMd`                        | Non    | Exécuter cet agent sans les fichiers CLAUDE.md utilisateur, projet et locaux quand il s'exécute en tant que sous-agent ; les fichiers de politique gérée se chargent toujours. Utilisez-le pour les agents qui prennent tout ce dont ils ont besoin à partir de l'invite du tool Agent. Ignoré quand cet agent s'exécute en tant qu'agent du thread principal. Nécessite TypeScript Agent SDK v0.3.271 ou ultérieur |
| `memory`                              | Non    | Source de mémoire pour cet agent : `'user'`, `'project'`, ou `'local'`                                                                                                                                                                                                                                                                                                                                              |
| `effort`                              | Non    | Niveau d'effort de raisonnement pour cet agent. Accepte un niveau nommé ou un entier                                                                                                                                                                                                                                                                                                                                |
| `permissionMode`                      | Non    | Mode de permission pour l'exécution des outils dans cet agent. Les [règles d'héritage de sous-agent](/docs/fr/agent-sdk/permissions#available-modes) décident quand il s'applique. Voir [`PermissionMode`](#permissionmode)                                                                                                                                                                                              |
| `criticalSystemReminder_EXPERIMENTAL` | Non    | Expérimental : Rappel critique ajouté à l'invite système                                                                                                                                                                                                                                                                                                                                                            |

<h3 id="agentmcpserverspec">
  `AgentMcpServerSpec`
</h3>

Spécifie les serveurs MCP disponibles pour un sous-agent. Peut être un nom de serveur (chaîne référençant un serveur de la configuration `mcpServers` du parent) ou une configuration de serveur en ligne enregistrant les noms de serveur aux configurations.

```typescript theme={null}
type AgentMcpServerSpec = string | Record<string, McpServerConfigForProcessTransport>;
```

Où `McpServerConfigForProcessTransport` est `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig`.

<h3 id="settingsource">
  `SettingSource`
</h3>

Contrôle les sources de configuration basées sur le système de fichiers que le SDK charge les paramètres à partir de.

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| Valeur      | Description                                                                       | Emplacement                   |
| :---------- | :-------------------------------------------------------------------------------- | :---------------------------- |
| `'user'`    | Paramètres utilisateur globaux                                                    | `~/.claude/settings.json`     |
| `'project'` | Paramètres de projet partagés (contrôle de version)                               | `.claude/settings.json`       |
| `'local'`   | Paramètres de projet locaux, gitignorés quand Claude Code enregistre un paramètre | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Comportement par défaut
</h4>

Quand `settingSources` est omis ou `undefined`, `query()` charge les mêmes paramètres du système de fichiers que la CLI Claude Code : utilisateur, projet et local. Voir [Ce que settingSources ne contrôle pas](/docs/fr/agent-sdk/claude-code-features#what-settingsources-does-not-control) pour les entrées qui sont lues indépendamment de cette option, et comment les désactiver.

<h4 id="why-use-settingsources">
  Pourquoi utiliser settingSources
</h4>

**Désactiver les paramètres du système de fichiers :**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Ne pas charger les paramètres utilisateur, projet ou locaux à partir du disque
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**Charger uniquement des sources de paramètres spécifiques :**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Charger uniquement les paramètres de projet, ignorer utilisateur et local
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Uniquement .claude/settings.json
  }
});
```

Pour charger les instructions de projet CLAUDE.md, incluez `"project"` dans `settingSources`. Voir [Modifier les invites système](/docs/fr/agent-sdk/modifying-system-prompts#claude-md-files-for-project-level-instructions) pour comment le chargement de CLAUDE.md interagit avec les options d'invite système.

<h4 id="settings-precedence">
  Précédence des paramètres
</h4>

Quand plusieurs sources sont chargées, les paramètres sont fusionnés avec cette précédence (la plus haute à la plus basse) :

1. Paramètres locaux (`.claude/settings.local.json`)
2. Paramètres de projet (`.claude/settings.json`)
3. Paramètres utilisateur (`~/.claude/settings.json`)

Les options programmatiques telles que `agents`, `allowedTools`, et `settings` remplacent les paramètres du système de fichiers utilisateur, projet et local. Les paramètres de politique gérée ont la priorité sur les options programmatiques.

<h3 id="permissionmode">
  `PermissionMode`
</h3>

```typescript theme={null}
type PermissionMode =
  | "default" // Comportement de permission standard
  | "acceptEdits" // Accepter automatiquement les modifications de fichiers
  | "bypassPermissions" // Contourner les contrôles de permission ; les règles d'ask explicites demandent toujours
  | "plan" // Mode de planification - explorer sans modifier
  | "dontAsk" // Ne pas demander les permissions, refuser si non pré-approuvé
  | "auto"; // Classificateur de modèle approuve ou refuse les invites de permission
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Type de fonction de permission personnalisée pour contrôler l'utilisation des outils.

La fonction est le remplacement SDK pour l'invite de permission interactive : elle est invoquée uniquement quand le [flux d'évaluation de permission](/docs/fr/agent-sdk/permissions#how-permissions-are-evaluated) se termine par une invite. Les appels d'outils déjà approuvés par une entrée `allowedTools`, une règle d'autorisation de paramètres, ou le mode de permission, comme `acceptEdits` ou `bypassPermissions`, ne l'invoquent jamais. Pour contrôler chaque appel d'outil, utilisez un [hook `PreToolUse`](/docs/fr/agent-sdk/hooks) à la place.

Une règle d'autorisation ne pré-approuve pas les [actions qu'aucun mode n'approuve automatiquement](/docs/fr/permission-modes#actions-no-mode-auto-approves) ; voir [Comment les permissions sont évaluées](/docs/fr/agent-sdk/permissions#how-permissions-are-evaluated) pour lesquelles d'entre elles atteignent le rappel et ce qui se passe en mode `dontAsk` et `auto`.

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    mcpServer?: { name: string; source: string };
    decisionReason?: string;
    toolUseID: string;
    agentID?: string;
    requestId: string;
  }
) => Promise<PermissionResult | null>;
```

| Option           | Type                                        | Description                                                                                                                                                                                                                                                                                                                                    |
| :--------------- | :------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`         | `AbortSignal`                               | Signalé si l'opération doit être abandonnée                                                                                                                                                                                                                                                                                                    |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | Mises à jour de permission suggérées pour que l'utilisateur ne soit pas invité à nouveau pour cet outil. Les invites Bash incluent une suggestion avec la destination [`localSettings`](#permissionupdatedestination), donc retourner dans `updatedPermissions` écrit la règle à `.claude/settings.local.json` et persiste entre les sessions. |
| `blockedPath`    | `string`                                    | Le chemin de fichier qui a déclenché la demande de permission, le cas échéant                                                                                                                                                                                                                                                                  |
| `mcpServer`      | `{ name: string; source: string }`          | Pour un outil `mcp__*`, le serveur MCP qui le sert et d'où la définition de ce serveur provient, avec les champs de [`McpServerProvenance`](#mcpserverprovenance). Absent pour les autres outils. Nécessite Agent SDK v0.3.274 ou ultérieur                                                                                                    |
| `decisionReason` | `string`                                    | Explique pourquoi cette demande de permission a été déclenchée                                                                                                                                                                                                                                                                                 |
| `toolUseID`      | `string`                                    | Identifiant unique pour cet appel d'outil spécifique dans le message assistant                                                                                                                                                                                                                                                                 |
| `agentID`        | `string`                                    | Si exécuté dans un sous-agent, l'ID du sous-agent                                                                                                                                                                                                                                                                                              |
| `requestId`      | `string`                                    | L'`request_id` du wrapper d'enveloppe `control_request`. Une `control_response` que votre application envoie en dehors du SDK, comme un POST HTTP signé, doit répéter cette valeur pour que le processus Claude Code puisse faire correspondre la réponse à la demande                                                                         |

Le rappel résout normalement la demande en retournant un [`PermissionResult`](#permissionresult), que le SDK écrit en retour sur son transport en tant que `control_response`. Retournez `null` uniquement quand votre application a déjà envoyé la `control_response` pour cette demande sur son propre canal, en répétant `requestId` ; le SDK saute alors l'écriture de la réponse sur son transport. Retourner `null` dans tout autre cas laisse l'appel d'outil bloqué indéfiniment, car aucune `control_response` n'est jamais envoyée et les invites de permission ne s'écoulent pas.

L'option `requestId` et la valeur de retour `null` nécessitent Claude Code v2.1.199 ou ultérieur.

<h3 id="permissionresult">
  `PermissionResult`
</h3>

Résultat d'une vérification de permission.

```typescript theme={null}
type PermissionResult =
  | {
      behavior: "allow";
      updatedInput?: Record<string, unknown>;
      updatedPermissions?: PermissionUpdate[];
      toolUseID?: string;
    }
  | {
      behavior: "deny";
      message: string;
      interrupt?: boolean;
      toolUseID?: string;
    };
```

<h3 id="toolconfig">
  `ToolConfig`
</h3>

Configuration pour le comportement des outils intégrés.

```typescript theme={null}
type ToolConfig = {
  askUserQuestion?: {
    previewFormat?: "markdown" | "html";
  };
};
```

| Champ                           | Type                   | Description                                                                                                                                                                                           |
| :------------------------------ | :--------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | Opte dans le champ `preview` sur les options [`AskUserQuestion`](/docs/fr/agent-sdk/user-input#question-format) et définit son format de contenu. Quand non défini, Claude n'émet pas de prévisualisations |

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

Configuration pour les serveurs MCP.

```typescript theme={null}
type McpServerConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfigWithInstance;
```

<h4 id="mcpstdioserverconfig">
  `McpStdioServerConfig`
</h4>

```typescript theme={null}
type McpStdioServerConfig = {
  type?: "stdio";
  command: string;
  args?: string[];
  env?: Record<string, string>;
};
```

<h4 id="mcpsseserverconfig">
  `McpSSEServerConfig`
</h4>

```typescript theme={null}
type McpSSEServerConfig = {
  type: "sse";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcphttpserverconfig">
  `McpHttpServerConfig`
</h4>

```typescript theme={null}
type McpHttpServerConfig = {
  type: "http";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcpsdkserverconfigwithinstance">
  `McpSdkServerConfigWithInstance`
</h4>

```typescript theme={null}
type McpSdkServerConfigWithInstance = {
  type: "sdk";
  name: string;
  timeout?: number;
  instance: McpServer;
};
```

<h4 id="mcpclaudeaiproxyserverconfig">
  `McpClaudeAIProxyServerConfig`
</h4>

```typescript theme={null}
type McpClaudeAIProxyServerConfig = {
  type: "claudeai-proxy";
  url: string;
  id: string;
};
```

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

Configuration pour charger les plugins dans le SDK.

```typescript theme={null}
type SdkPluginConfig = {
  type: "local";
  path: string;
  skipMcpDiscovery?: boolean;
};
```

| Champ              | Type      | Description                                                                                                                                                                                                                                  |
| :----------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`             | `'local'` | Doit être `'local'` (seuls les plugins locaux sont actuellement pris en charge)                                                                                                                                                              |
| `path`             | `string`  | Chemin absolu ou relatif au répertoire du plugin                                                                                                                                                                                             |
| `skipMcpDiscovery` | `boolean` | Quand `true`, le SDK charge les compétences, les hooks, les agents et les commandes de ce plugin mais ne lit pas son `.mcp.json` ou le manifeste `mcpServers`. Définissez ceci quand votre application possède les connexions MCP du plugin. |

**Exemple :**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

Pour des informations complètes sur la création et l'utilisation de plugins, voir [Plugins](/docs/fr/agent-sdk/plugins).

<h2 id="message-types">
  Types de messages
</h2>

<h3 id="sdkmessage">
  `SDKMessage`
</h3>

Type union de tous les messages possibles retournés par la requête.

```typescript theme={null}
type SDKMessage =
  | SDKAssistantMessage
  | SDKUserMessage
  | SDKUserMessageReplay
  | SDKResultMessage
  | SDKSystemMessage
  | SDKPartialAssistantMessage
  | SDKCompactBoundaryMessage
  | SDKStatusMessage
  | SDKLocalCommandOutputMessage
  | SDKHookStartedMessage
  | SDKHookProgressMessage
  | SDKHookResponseMessage
  | SDKPluginInstallMessage
  | SDKToolProgressMessage
  | SDKAuthStatusMessage
  | SDKTaskNotificationMessage
  | SDKTaskStartedMessage
  | SDKTaskProgressMessage
  | SDKTaskUpdatedMessage
  | SDKBackgroundTasksChangedMessage
  | SDKThinkingTokensMessage
  | SDKSessionStateChangedMessage
  | SDKWorkerShuttingDownMessage
  | SDKCommandsChangedMessage
  | SDKNotificationMessage
  | SDKFilesPersistedEvent
  | SDKToolUseSummaryMessage
  | SDKMemoryRecallMessage
  | SDKRateLimitEvent
  | SDKElicitationCompleteMessage
  | SDKPermissionDeniedMessage
  | SDKPromptSuggestionMessage
  | SDKAPIRetryMessage
  | SDKMirrorErrorMessage
  | SDKInformationalMessage
  | SDKConversationResetMessage;
```

<h3 id="sdkassistantmessage">
  `SDKAssistantMessage`
</h3>

Message de réponse de l'assistant.

```typescript theme={null}
type SDKAssistantMessage = {
  type: "assistant";
  uuid: UUID;
  session_id: string;
  message: BetaMessage; // From Anthropic SDK
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
  aborted?: true;
  timestamp?: string;
  context_usage?: SDKContextUsage;
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

Le champ `message` est un [`BetaMessage`](https://platform.claude.com/docs/fr/api/messages/create) du SDK Anthropic. Il inclut des champs comme `id`, `content`, `model`, `stop_reason` et `usage`.

`SDKAssistantMessageError` est l'un des suivants : `'authentication_failed'`, `'oauth_org_not_allowed'`, `'account_on_hold'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'`, `'cloud_credential_error'`, ou `'unknown'`. Quatre de ces valeurs signifient plus que leurs noms ne l'indiquent :

* `'model_not_found'` : le modèle sélectionné n'existe pas ou n'est pas disponible pour votre compte ou déploiement
* `'overloaded'` : l'API a retourné un 529 parce que le serveur est à capacité, contrairement à `'rate_limit'`, qui est un 429 contre votre quota
* `'account_on_hold'` : [votre compte est suspendu](/docs/fr/errors#your-account-is-on-hold)
* `'cloud_credential_error'` : Claude Code n'a pas pu obtenir des identifiants AWS ou Google Cloud utilisables sur la machine sur laquelle il s'exécute, donc aucune requête n'a atteint le fournisseur cloud. La cause habituelle est une connexion cloud qui a expiré ou n'a jamais été complétée sur cette machine, bien qu'un service d'identifiants brièvement inaccessible rapporte la même valeur. Voir [Impossible de charger les identifiants AWS ou Google Cloud](/docs/fr/errors#could-not-load-aws-or-google-cloud-credentials). Nécessite TypeScript Agent SDK v0.3.267 ou ultérieur, qui regroupe Claude Code v2.1.267

`aborted` est `true` quand une interruption ou un abandon a tronqué le message de l'assistant avant la fin du flux : le message n'a pas de `stop_reason` et le contenu peut se terminer au milieu d'un mot. Le champ est absent sur les messages normalement complétés. Il nécessite Agent SDK v0.3.214 ou ultérieur.

Claude Code définit `user_message_uuid` et `user_message_uuids` sur le premier message de l'assistant du tour, selon les conditions dans [`user_message_uuid`](#user_message_uuid).

`timestamp` est l'heure ISO 8601 à laquelle le contenu du message a fini de générer sur le processus qui l'a produit. La valeur provient de l'horloge de cette machine, donc utilisez-la uniquement pour l'affichage et ne triez pas les messages par elle. Un tour API peut produire plusieurs messages d'assistant qui partagent un `message.id`, chacun avec son propre `timestamp`. Quand le champ est absent, revenez à l'heure à laquelle vous avez reçu le message.

`context_usage` est une copie structurée du rapport `/context`, typée comme [`SDKContextUsage`](#sdkcontextusage), et nécessite Agent SDK v0.3.232 ou ultérieur. Quand vous envoyez `/context` comme invite, Claude Code livre le rapport comme un message d'assistant dont `message.content` contient le tableau markdown, et attache `context_usage` à ce même message. Claude Code ne définit pas le champ sur aucun autre message d'assistant, et les versions antérieures livrent le tableau `/context` sans lui, donc lisez la ventilation du champ quand il est présent et revenez au texte markdown quand il ne l'est pas.

<h3 id="sdkusermessage">
  `SDKUserMessage`
</h3>

Message d'entrée utilisateur.

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // From Anthropic SDK
  pasted_content?: MessageParam["content"][];
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  inline_pastes?: string[];
};
```

Définissez `pasted_content` pour envoyer du contenu que l'utilisateur a collé dans votre interface d'invite plutôt que tapé, une entrée par collage, chacune étant une chaîne ou un tableau de blocs de contenu. Claude Code ajoute le texte de chaque entrée après le texte tapé, dans l'ordre, et peut envelopper chaque collage dans des balises `<pasted_content>`. Les blocs autres que le texte sont ignorés, donc envoyez les images et les documents dans `message.content`. Nécessite Agent SDK v0.3.277 ou ultérieur.

Définissez `shouldQuery` à `false` pour ajouter le message à la transcription sans déclencher un tour d'assistant. Le message est conservé et fusionné dans le prochain message utilisateur qui déclenche un tour. Utilisez ceci pour injecter du contexte, comme la sortie d'une commande que vous avez exécutée hors bande, sans dépenser un appel de modèle pour cela.

Sur un message qui porte un bloc `tool_result`, `tool_use_result` est l'objet de sortie structuré de l'outil plutôt que le texte envoyé au modèle. Sa forme dépend de l'outil nommé par le bloc `tool_use` correspondant, donc le champ est typé `unknown` ; les formes intégrées sont listées sous [Types de sortie d'outil](#tool-output-types).

Pour l'outil `Agent`, `tool_use_result` est [`AgentOutput`](#agent-2). Sur un résultat `completed`, `content` contient le rapport du sous-agent sans l'ID d'agent et la remorque d'utilisation que Claude Code ajoute au texte `tool_result`, donc rendez à partir de `tool_use_result` au lieu d'analyser ce texte.

Pour un outil MCP dont le résultat contient des blocs `resource_link`, `tool_use_result` est un objet avec un tableau `resourceLinks` d'entrées [`SDKMcpResourceLink`](#sdkmcpresourcelink). Claude reçoit chaque lien comme une ligne de texte dans le bloc `tool_result`, donc lisez `resourceLinks` pour rendre les fichiers que le serveur a retournés au lieu d'analyser ce texte. Claude Code omet `resourceLinks` quand le résultat n'a pas de liens et sur les résultats des sous-agents, conserve au maximum 50 liens par résultat, et arrête d'ajouter des liens une fois que le tableau atteint 64 KiB de JSON sérialisé. `resourceLinks` nécessite Agent SDK v0.3.257 ou ultérieur.

Définissez `inline_pastes` pour dire à Claude Code quelles parties de `message.content` l'utilisateur a collées plutôt que tapées, une chaîne par collage. Le texte d'invite reste où l'utilisateur l'a mis. Claude Code peut envelopper chaque collage listé dans des balises `<pasted_content>` où il se tient, donc Claude peut distinguer le matériel collé des propres paroles de l'utilisateur. Seuls les collages du dernier bloc de texte de l'invite sont enveloppés. Nécessite TypeScript Agent SDK v0.3.280 ou ultérieur.

<h3 id="sdkusermessagereplay">
  `SDKUserMessageReplay`
</h3>

Message utilisateur rejoué avec UUID requis.

```typescript theme={null}
type SDKUserMessageReplay = {
  type: "user";
  uuid: UUID;
  session_id: string;
  message: MessageParam;
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  isReplay: true;
};
```

Un tour utilisateur injecté de l'extérieur de la session, dont le [`origin`](#sdkmessageorigin) est de type `peer` ou `channel`, arrive sur le flux comme un rejeu qu'il ait été livré pendant un tour actif ou ait démarré un nouveau tour alors que la session était inactive. Avant v2.1.207, un tour injecté livré alors que la session était inactive ne produisait aucun message sur le flux et n'apparaissait que quand vous relisiez la transcription.

<h3 id="sdkresultmessage">
  `SDKResultMessage`
</h3>

Message de résultat final.

```typescript theme={null}
type SDKResultMessage =
  | {
      type: "result";
      subtype: "success";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      api_error_status?: number | null;
      num_turns: number;
      result: string;
      stop_reason: string | null;
      ttft_ms?: number;
      ttft_stream_ms?: number;
      user_message_uuid?: string;
      user_message_uuids?: string[];
      request_sent_wall_ms?: number;
      first_content_frame_ms?: number;
      first_stream_post_ms?: number;
      first_stream_post_ack_ms?: number;
      first_stream_post_wall_ms?: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      queued_turn_count?: number;
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
      origin?: SDKMessageOrigin;
    }
  | {
      type: "result";
      subtype:
        | "error_max_turns"
        | "error_during_execution"
        | "error_max_budget_usd"
        | "error_max_structured_output_retries";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      num_turns: number;
      stop_reason: string | null;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      queued_turn_count?: number;
      errors: string[];
      startup_failure_reason?: SDKStartupFailureReason;
      user_message_uuid?: string;
      user_message_uuids?: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
      origin?: SDKMessageOrigin;
    };
```

Plusieurs champs sur le résultat portent des détails de diagnostic au-delà de `subtype` :

* `api_error_status` : le code de statut HTTP de l'erreur API qui a terminé la conversation. Absent ou `null` quand le tour s'est terminé sans erreur API.
* `ttft_ms` : temps jusqu'au premier jeton en millisecondes, mesuré quand le premier message d'assistant complet arrive. Présent sur le bras de succès uniquement.
* `ttft_stream_ms` : temps en millisecondes jusqu'au premier événement de flux `message_start`, quand le flux de réponse s'ouvre. Inférieur à `ttft_ms` ; l'écart entre les deux est le temps passé à diffuser le premier message. Présent sur le bras de succès uniquement.
* `user_message_uuid` : l'`uuid` du message que vous avez envoyé que ce tour a répondu. Voir [`user_message_uuid`](#user_message_uuid) pour savoir quels résultats le portent.
* `user_message_uuids` : les `uuid`s de chaque message que vous avez envoyé que Claude Code a répondu dans ce tour. Voir [`user_message_uuids`](#user_message_uuids).
* `request_sent_wall_ms` : millisecondes d'époque auxquelles Claude Code a envoyé la requête API, pour les jointures contre les horodatages côté serveur. Présent uniquement avec [`user_message_uuid`](#user_message_uuid), sur un résultat de succès avec `is_error` false dont le tour a envoyé une requête API.
* `first_content_frame_ms` : temps en millisecondes jusqu'au premier événement de flux `content_block_start` ou `content_block_delta`, en comptant les blocs de réflexion comme du contenu. Présent sur le bras de succès uniquement, quand `is_error` est false. Nécessite Agent SDK v0.3.260 ou ultérieur.
* `first_stream_post_ms`, `first_stream_post_ack_ms`, `first_stream_post_wall_ms` : chronométrages pour télécharger le premier événement de flux du tour. Claude Code les enregistre uniquement dans les sessions qu'il diffuse à claude.ai, comme les [sessions cloud](/docs/fr/claude-code-on-the-web), et les résultats que `query()` produit ne les portent pas. Nécessite Agent SDK v0.3.260 ou ultérieur.
* `usage` : boucle d'agent principal uniquement. Exclut les appels de sous-agent et de modèle auxiliaire, et est par tour dans les sessions d'entrée en flux. Préférez `modelUsage` pour la comptabilité des jetons/coûts.
* `modelUsage` : totaux par modèle pour chaque appel de modèle effectué via le pipeline de requête pendant cet appel `query()`, y compris la boucle principale, les sous-agents et les appels internes tels que la compaction et les agents Workflow. Les appels d'assistance en dehors de ce pipeline, comme le classificateur de permissions et les demandes de comptage de jetons, sont exclus. Un appel qui reprend une session compte également les [totaux par modèle restaurés à partir des appels antérieurs de la session](/docs/fr/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Dans les sessions d'entrée en flux, les totaux sont cumulatifs entre les tours, donc lisez le dernier résultat plutôt que de faire la somme entre les résultats. Voir [Suivre les coûts en mode d'entrée en flux](/docs/fr/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) pour les réinitialisations et [Récupérer les totaux après un crash de session](/docs/fr/agent-sdk/cost-tracking#recover-totals-after-a-session-crash) pour les résultats mis à zéro.
* `total_cost_usd` : coût estimé cumulatif en USD, couvrant les mêmes appels que `modelUsage` et réinitialisé aux mêmes points. Un appel qui reprend une session compte également les [totaux restaurés à partir des appels antérieurs de la session](/docs/fr/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). C'est une estimation, pas un relevé de facturation. Voir [Suivre le coût et l'utilisation](/docs/fr/agent-sdk/cost-tracking) pour les avertissements de précision.
* `queued_turn_count` : le nombre de messages que vous avez envoyés avec `origin: { kind: "human" }` qui attendent toujours quand Claude Code a produit le résultat. Voir [`queued_turn_count`](#queued_turn_count) pour ce que `0` et un champ absent vous disent.
* `startup_failure_reason` : pourquoi Claude Code a refusé de démarrer, sur le résultat `error_during_execution` qu'il écrit avant de quitter sur une défaillance de démarrage connue. Voir [`startup_failure_reason`](#startup_failure_reason) pour les valeurs et quelles défaillances la portent. Nécessite Agent SDK v0.3.274 ou ultérieur.
* `terminal_reason` : pourquoi la boucle s'est terminée. L'un de `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"`, ou `"turn_setup_failed"`.
* `fast_mode_state` : l'un de `"on"`, `"off"`, ou `"cooldown"`.
* `fast_mode_disabled_reason` : pourquoi le [mode rapide](/docs/fr/fast-mode) n'est pas disponible en ce moment. Absent quand rien ne bloque le mode rapide, bien qu'une requête puisse toujours s'exécuter à vitesse standard. Pendant le refroidissement après une limite de débit du mode rapide, Claude Code rapporte `fast_mode_state: "cooldown"` sans code de raison et réactive le mode rapide quand le refroidissement expire. Nécessite Claude Code v2.1.219 ou ultérieur.

Utilisez le code de raison pour expliquer pourquoi le mode rapide est désactivé dans votre propre interface utilisateur au lieu de redériver la disponibilité. Chaque code nomme la vérification qui a bloqué le mode rapide :

| Code de raison         | Signification                                                                                                                                                 |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `free`                 | Le compte n'a pas l'abonnement payant ou les crédits d'utilisation que le mode rapide nécessite                                                               |
| `preference`           | L'organisation a désactivé le mode rapide                                                                                                                     |
| `extra_usage_disabled` | Les crédits d'utilisation sont désactivés pour le compte                                                                                                      |
| `network_error`        | La [vérification de disponibilité](/docs/fr/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) n'a pas pu atteindre `api.anthropic.com`                      |
| `unknown`              | Claude Code n'a pas pu déterminer la disponibilité                                                                                                            |
| `not_first_party`      | La session utilise un fournisseur autre que l'API Anthropic                                                                                                   |
| `disabled_by_env`      | [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/fr/env-vars) est défini                                                                                                    |
| `model_not_allowed`    | Le modèle Opus du mode rapide ne figure pas dans la liste d'autorisation [`availableModels`](/docs/fr/model-config#restrict-model-selection) de l'organisation     |
| `sdk_opt_in_required`  | La session n'a pas opté pour le mode rapide : passez `fastMode: true` dans l'option [`settings`](#options) ou via [`applyFlagSettings()`](#applyflagsettings) |
| `pending`              | La vérification de disponibilité n'a pas encore été complétée                                                                                                 |

La même paire de champs apparaît sur [`SDKSystemMessage`](#sdksystemmessage) et sur [`SDKControlInitializeResponse`](#sdkcontrolinitializeresponse), donc vous pouvez lire l'état du mode rapide avant le premier tour.

Le champ `origin` transmet le [`SDKMessageOrigin`](#sdkmessageorigin) du message utilisateur qui a déclenché ce résultat. Quand le SDK injecte un tour de suivi synthétique, comme pour une tâche de fond terminée, le `SDKResultMessage` résultant porte `origin: { kind: "task-notification" }`. Les routines dont le déclencheur s'est déclenché et les messages vérifiés par le serveur de vos autres sessions arrivent avec ce type aussi, chacun avec le `subkind` décrit dans [Sous-types de notification de tâche](#task-notification-subkinds). Vérifiez `kind` pour distinguer les résultats qui répondent à votre invite des suivis injectés avant de les router ou de les supprimer. Si votre application [déclare des exécutions planifiées](#declare-a-scheduled-run), leurs résultats portent `kind: "task-notification"` aussi, donc ne supprimez pas sur `kind` seul.

Quand plusieurs complétions de tâche de fond sont mises en file d'attente ensemble, Claude Code peut les répondre en un seul tour plutôt qu'un tour chacun. Chaque complétion produit toujours son propre résultat avec cette origine. Tous sauf le dernier des complétions que Claude Code répond ensemble produisent des résultats vides avec `num_turns: 0`, dans l'ordre, et le résultat du dernier porte le tour qui les répond tous.

Le champ est absent pour les résultats émis avant tout tour utilisateur, comme les erreurs de démarrage.

Quand un hook `PreToolUse` retourne `permissionDecision: "defer"`, le résultat a `stop_reason: "tool_deferred"` et `deferred_tool_use` porte l'`id`, le `name` et l'`input` de l'outil en attente. Lisez ce champ pour afficher la requête dans votre propre interface utilisateur, puis reprenez avec le même `session_id` pour continuer. Voir [Différer un appel d'outil pour plus tard](/docs/fr/hooks#defer-a-tool-call-for-later) pour le trajet complet.

<h4 id="user_message_uuid">
  `user_message_uuid`
</h4>

L'`uuid` du [`SDKUserMessage`](#sdkusermessage) auquel le tour répond, répété pour que vous puissiez faire correspondre la réponse de Claude Code au message que vous avez envoyé. Claude Code répète un `uuid` uniquement si vous en définissez un sur le message. Le champ est optionnel sur `SDKUserMessage`, et une invite de chaîne passée à `query()` n'en porte aucune.

Quel message de votre part un tour répond dépend de la façon dont le tour a commencé :

* **Un message régulier que vous avez envoyé**, c'est-à-dire sans `isSynthetic: true` : le tour répond à ce message pour toute sa durée. Quand vous envoyez plusieurs messages rapprochés, Claude Code peut les fusionner en un seul tour, et le champ porte alors uniquement l'`uuid` du dernier message. Pour faire correspondre la réponse à l'un des messages fusionnés, utilisez [`user_message_uuids`](#user_message_uuids).
* **Un message que vous avez envoyé avec `isSynthetic: true`** : le tour répond d'abord à ce message. Si Claude Code reprend un message régulier de votre part entre les appels d'outil, le tour répond au message repris à partir de là. Répéter l'`uuid` d'un message synthétique nécessite Agent SDK v0.3.265 ou ultérieur ; les versions antérieures ne répètent rien sur les tours synthétiques.
* **Une invite que Claude Code a générée lui-même**, comme le tour qui continue le travail interrompu après un redémarrage de session : le tour ne répond d'abord à aucun message de votre part et ses cadres ne portent aucun écho. Si Claude Code reprend un message régulier de votre part entre les appels d'outil, le tour répond à ce message à partir de là. L'écho de reprise nécessite Agent SDK v0.3.265 ou ultérieur ; les versions antérieures ne répètent rien sur ces tours.

Claude Code répète l'`uuid` du message répondu sur trois types de cadre :

* **Le résultat** : chaque résultat d'un tour qui a répondu à un message que vous avez envoyé. Chaque tel résultat le porte sur Agent SDK v0.3.265 ou ultérieur. Avant v0.3.265, le résultat de succès d'un tour qu'un message régulier a démarré lui manquait quand le tour n'a envoyé aucune requête API ou s'est terminé avec un appel d'outil différé. Avant v0.3.246, les résultats d'erreur lui manquaient aussi, et avant v0.3.216 chaque résultat le faisait.
* **La première réponse du tour** : le premier [message d'assistant](#sdkassistantmessage), ou avec `includePartialMessages` le premier [événement de flux](#sdkpartialassistantmessage) dont `event.type` n'est pas `ping`, pour que vous puissiez lier la réponse avant l'arrivée du résultat. Quand un tour ne diffuse rien, Claude Code le définit sur le premier message d'assistant à la place. L'écho de première réponse nécessite Agent SDK v0.3.246 ou ultérieur. Quand le message auquel le tour répond change en cours de tour, la première réponse après le changement porte le champ aussi, sur Agent SDK v0.3.265 ou ultérieur ; les versions antérieures le définissent sur un cadre de réponse par tour.
* **Chaque cadre [`thinking_tokens`](#sdkthinkingtokensmessage) du tour** : pour que vous puissiez attribuer la progression de la réflexion au message que vous avez envoyé sans attendre la première réponse du tour. Nécessite Agent SDK v0.3.260 ou ultérieur.

Claude Code omet le champ dans ces cas :

* Cadres de réponse autres que ces premières réponses
* Cadres de sous-agent
* Tours qui ne répondent à aucun message avec un `uuid` : le tour a répondu à un message que vous avez envoyé sans en avoir un, ou Claude Code a démarré le tour lui-même et n'a repris aucun message régulier qui en a un
* Résultats qui ne répondent à aucun message que vous avez envoyé, comme le résultat mis à zéro après un crash de processus de travail

<h4 id="user_message_uuids">
  `user_message_uuids`
</h4>

Les `uuid`s de chaque message que vous avez envoyé que Claude Code a répondu dans ce tour. Quand vous envoyez plusieurs messages rapprochés, Claude Code peut les fusionner en un seul tour, et `user_message_uuid` nomme alors uniquement le dernier d'entre eux. Pour faire correspondre la réponse à l'un des messages fusionnés, cherchez l'`uuid` de ce message n'importe où dans cette liste. Nécessite Agent SDK v0.3.259 ou ultérieur.

Claude Code définit la liste avec `user_message_uuid` sur chaque cadre de réponse qui porte ce champ et sur le résultat. Pour l'ensemble complet des cadres qui portent `user_message_uuid`, et la version que chacun nécessite, voir [`user_message_uuid`](#user_message_uuid). La liste contient toujours `user_message_uuid` et contient au maximum 64 entrées.

Quand Claude Code reprend un message régulier que vous avez envoyé pendant qu'un tour s'exécutait, il ajoute l'`uuid` de ce message à la liste du résultat.

Quand une première réponse ou un résultat porte `user_message_uuid` sans la liste, il provient d'une version antérieure de Claude Code, donc revenez au champ unique.

<h4 id="queued_turn_count">
  `queued_turn_count`
</h4>

Le nombre de messages que vous avez envoyés avec [`origin: { kind: "human" }`](#sdkmessageorigin) qui attendent toujours dans la file de commandes quand Claude Code a produit le résultat. Nécessite Agent SDK v0.3.242 ou ultérieur.

Ce que `0` et un champ absent vous disent :

* **`0`** : Claude Code ne compte pas les messages que vous avez envoyés sans ce `origin`, et ne compte pas les notifications de tâche, donc un tour peut toujours suivre.
* **Absent** : le résultat final que Claude Code émet après un crash ou une erreur de démarrage fatale omet le champ, et [peut porter des totaux mis à zéro](/docs/fr/agent-sdk/cost-tracking#recover-totals-after-a-session-crash).

<h4 id="startup_failure_reason">
  `startup_failure_reason`
</h4>

Pourquoi Claude Code a refusé de démarrer, pour que votre application puisse offrir la correction au lieu d'une nouvelle tentative. Claude Code le définit sur le résultat `error_during_execution` qu'il écrit avant de quitter sur une défaillance de démarrage connue. Ce résultat porte des totaux mis à zéro, et son tableau `errors` porte le même texte que stderr. Le champ est absent sur tous les autres résultats. Nécessite Agent SDK v0.3.274 ou ultérieur.

Définissez `CLAUDE_CODE_STARTUP_FAILURE_RESULTS` à `1` dans [`env`](#options) pour recevoir ce résultat pour chaque valeur `SDKStartupFailureReason`. Sans cette variable, Claude Code écrit le résultat uniquement pour ces défaillances, et le reste se termine par une sortie stderr, un code de sortie non nul et aucun message de résultat :

* Une reprise que Claude Code arrête parce qu'elle [ne peut pas retourner la session à son worktree](/docs/fr/worktrees#the-session-resumes-outside-its-worktree), avec `worktree_unverified` ou `worktree_resume_refused`. Cette section dit quelle erreur porte quelle valeur.
* Une [`continue`](#options) refusée d'une conversation qu'une session de fond tient, avec `session_held_by_background`. Pour une [`resume`](#options) refusée d'une telle conversation, Claude Code écrit le résultat uniquement quand la variable est définie.

```typescript theme={null}
type SDKStartupFailureReason =
  | "org_pin_api_key_conflict"
  | "org_verify_failed"
  | "org_pin_mismatch"
  | "managed_settings_invalid"
  | "remote_settings_required_unavailable"
  | "gateway_signin_required"
  | "gateway_access_denied"
  | "proxy_invalid"
  | "temp_dir_unusable"
  | "cwd_unavailable"
  | "shell_tool_missing"
  | "session_held_by_background"
  | "worktree_resume_refused"
  | "worktree_unverified"
  | "cli_version_too_old"
  | "bypass_root";
```

Chaque valeur nomme un refus :

| Valeur                                 | Ce qui a arrêté la session                                                                                                                                                                                                                        |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `org_pin_api_key_conflict`             | Les paramètres gérés [nécessitent une connexion de première partie ou Cloud gateway](/docs/fr/authentication#restrict-login-to-your-organization), et une clé API Anthropic, un jeton d'authentification ou un `apiKeyHelper` est configuré à la place |
| `org_verify_failed`                    | L'organisation de la connexion n'a pas pu être vérifiée par rapport à la broche, par exemple en raison d'une défaillance réseau ou d'un jeton révoqué                                                                                             |
| `org_pin_mismatch`                     | La connexion appartient à une organisation que la broche ne permet pas                                                                                                                                                                            |
| `managed_settings_invalid`             | Les paramètres de politique gérés n'ont pas pu être lus, ou la broche ne nomme aucune organisation                                                                                                                                                |
| `remote_settings_required_unavailable` | Les paramètres gérés que l'organisation nécessite n'ont pas pu être chargés                                                                                                                                                                       |
| `gateway_signin_required`              | La [Cloud gateway](/docs/fr/claude-apps-gateway) a terminé cette connexion                                                                                                                                                                             |
| `gateway_access_denied`                | La demande de paramètres gérés à la Cloud gateway est revenue avec un 403, que la [table de dépannage](/docs/fr/claude-apps-gateway-deploy#troubleshooting) de la gateway couvre                                                                       |
| `proxy_invalid`                        | Un paramètre de proxy n'est pas une URL complète                                                                                                                                                                                                  |
| `temp_dir_unusable`                    | Le répertoire temporaire par utilisateur n'est pas sûr ou n'a pas pu être créé                                                                                                                                                                    |
| `cwd_unavailable`                      | Le répertoire de travail a été supprimé, déplacé ou ne peut pas être lu                                                                                                                                                                           |
| `shell_tool_missing`                   | Sur Windows, aucun outil shell n'est disponible : Git Bash manque, et PowerShell manque ou est désactivé avec `CLAUDE_CODE_USE_POWERSHELL_TOOL`                                                                                                   |
| `session_held_by_background`           | La conversation à reprendre ou continuer s'exécute comme une [session de fond](/docs/fr/agent-view)                                                                                                                                                    |
| `worktree_resume_refused`              | Le worktree de la session a échoué ses vérifications de sécurité, ou la reprise a été lancée de l'intérieur. `errors` dit si l'exécution de la même reprise continue sans le worktree                                                             |
| `worktree_unverified`                  | Le worktree de la session n'a pas pu être vérifié en ce moment, et une nouvelle tentative peut réussir                                                                                                                                            |
| `cli_version_too_old`                  | Cette version de Claude Code est inférieure au minimum qu'Anthropic nécessite                                                                                                                                                                     |
| `bypass_root`                          | Le mode de permissions de contournement a été demandé lors de l'exécution en tant que root                                                                                                                                                        |

<h3 id="sdksystemmessage">
  `SDKSystemMessage`
</h3>

Message d'initialisation du système.

```typescript theme={null}
type SDKSystemMessage = {
  type: "system";
  subtype: "init";
  uuid: UUID;
  session_id: string;
  agents?: string[];
  apiKeySource: ApiKeySource;
  betas?: string[];
  claude_code_version: string;
  cwd: string;
  tools: string[];
  mcp_servers: {
    name: string;
    status: string;
    source?: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  terminal_slash_commands?: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  fast_mode_state?: FastModeState;
  fast_mode_disabled_reason?: FastModeDisabledReason;
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | null;
  capabilities?: string[];
};
```

`fast_mode_state` rapporte l'état du [mode rapide](/docs/fr/fast-mode) de la session. Quand quelque chose bloque le mode rapide, `fast_mode_disabled_reason` nomme la vérification qui l'a bloqué ; le champ nécessite Claude Code v2.1.219 ou ultérieur. Pour les codes de raison et leurs significations, voir [`fast_mode_disabled_reason`](#sdkresultmessage) sur le message de résultat.

`terminal_slash_commands` nomme les entrées dans `slash_commands` dont l'interface est liée au terminal local, comme `exit`. Vous pouvez les envoyer comme n'importe quelle autre entrée dans `slash_commands` ; le champ existe pour qu'un client distant ou mobile puisse les masquer de ses menus de commandes. Le champ est présent uniquement quand non vide, et nécessite Agent SDK v0.3.229 ou ultérieur.

*

`source` sur chaque entrée `mcp_servers` : d'où provient la définition du serveur, avec les mêmes valeurs que `source` de [`McpServerStatus`](#mcpserverstatus). Nécessite Agent SDK v0.3.274 ou ultérieur.

*

`effort` : le [niveau d'effort](/docs/fr/model-config#adjust-effort-level) que Claude Code envoie sur la prochaine requête de la session, ou `null` quand il n'en envoie aucun. Claude Code définit le champ uniquement sur le message d'initialisation qu'il envoie aux clients [Remote Control](/docs/fr/remote-control), et l'omet du message d'initialisation que votre application lit. Nécessite Agent SDK v0.3.234 ou ultérieur.

Le tableau `capabilities` nomme les comportements de protocole que cette CLI implémente, pour que vous puissiez faire de la détection de fonctionnalités au lieu de comparer les chaînes `claude_code_version`. C'est un ensemble ouvert : ignorez les valeurs que vous ne reconnaissez pas, et vérifiez la capacité spécifique dont vous dépendez du comportement. Le champ nécessite Claude Code v2.1.205 ou ultérieur et est absent sur les CLI antérieures.

| Capacité                     | Signification                                                                                                                                                                                                                                                                                          |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `interrupt_receipt_v1`       | [`interrupt()`](#query-object) se résout avec un reçu [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) listant les messages qui étaient en attente quand l'interruption est arrivée                                                                                                       |
| `interrupt_cancel_queued_v1` | La demande de contrôle `interrupt` honore `cancel_queued: true`, annulant les messages que le reçu listerait autrement sous `still_queued` et les listant sous `cancelled` à la place. Voir [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse). Nécessite Claude Code v2.1.219 ou ultérieur |

<h3 id="sdkpartialassistantmessage">
  `SDKPartialAssistantMessage`
</h3>

Message partiel en flux (uniquement quand `includePartialMessages` est true). Le champ `parent_tool_use_id` est toujours `null` : les événements de flux sont émis pour la session principale uniquement. Pour l'attribution de sous-agent, utilisez les messages complets, qui portent `parent_tool_use_id`, ou activez [`forwardSubagentText`](#options) pour recevoir le texte et la réflexion du sous-agent comme des messages complets.

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // From Anthropic SDK
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // Time to first token in ms, present only on message_start events
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

Claude Code définit `user_message_uuid` et `user_message_uuids` sur le premier événement de flux non-ping du tour, et à nouveau quand le message auquel le tour répond change, selon les conditions dans [`user_message_uuid`](#user_message_uuid).

<h3 id="sdkcompactboundarymessage">
  `SDKCompactBoundaryMessage`
</h3>

Message indiquant une limite de compaction de conversation.

```typescript theme={null}
type SDKCompactBoundaryMessage = {
  type: "system";
  subtype: "compact_boundary";
  uuid: UUID;
  session_id: string;
  compact_metadata: {
    trigger: "manual" | "auto";
    pre_tokens: number;
  };
};
```

<h3 id="sdkinformationalmessage">
  `SDKInformationalMessage`
</h3>

Bannière de texte générique émise par la boucle. Porte les lignes d'état non-erreur, les retours de hook comme la raison du blocage d'un hook `UserPromptSubmit`, et la sortie de commande. Sur Claude Code v2.1.227 ou ultérieur, le [`systemMessage`](/docs/fr/hooks#json-output) d'un hook peut arriver comme ce message, avec chaque ligne préfixée par le nom du hook, comme `PostToolUse:Bash says:`. Qu'un `systemMessage` d'un hook arrive comme ce message dépend de l'événement. Chaque [section d'événement](/docs/fr/hooks#hook-events) sur la page des hooks dit comment la sortie s'affiche. Rendez `content` comme texte brut au `level` donné.

```typescript theme={null}
type SDKInformationalMessage = {
  type: "system";
  subtype: "informational";
  content: string;
  level: "info" | "notice" | "suggestion" | "warning";
  tool_use_id?: string;
  prevent_continuation?: boolean;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkworkershuttingdownmessage">
  `SDKWorkerShuttingDownMessage`
</h3>

Émis lors d'un arrêt gracieux du travailleur pour que les clients de contrôle à distance puissent montrer pourquoi le travailleur a quitté au lieu d'attendre l'expiration du délai d'attente du battement de cœur. La `reason` est une courte chaîne snake\_case définie par la CLI hôte, comme `"host_exit"` ou `"remote_control_disabled"`. Agissez sur ceci uniquement lors de la diffusion en direct. Une session reprise rejoue les instances passées de ce message, donc ignorez-les dans ce cas.

```typescript theme={null}
type SDKWorkerShuttingDownMessage = {
  type: "system";
  subtype: "worker_shutting_down";
  reason: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkplugininstallmessage">
  `SDKPluginInstallMessage`
</h3>

Événement de progression d'installation de plugin. Émis quand [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/fr/env-vars) est défini, pour que votre application Agent SDK puisse suivre l'installation du plugin de marketplace avant le premier tour. Les statuts `started` et `completed` encadrent l'installation globale. Les statuts `installed` et `failed` rapportent les marketplaces individuels et incluent `name`.

```typescript theme={null}
type SDKPluginInstallMessage = {
  type: "system";
  subtype: "plugin_install";
  status: "started" | "installed" | "failed" | "completed";
  name?: string;
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpermissiondeniedmessage">
  `SDKPermissionDeniedMessage`
</h3>

Événement de flux émis quand le système de permissions refuse un appel d'outil sans invite interactive. Utilisez-le pour rendre le refus dans votre interface utilisateur au fur et à mesure, plutôt que d'observer uniquement le résultat d'outil `is_error` qui suit. Quels refus il rapporte dépend de la façon dont l'exécution gère les invites de permissions :

* **Avec un rappel [`canUseTool`](#canusetool) et le [`permissionPrompts: 'host'`](#options) par défaut** : les invites de permissions vont à votre rappel, et cet événement rapporte les refus que Claude Code décide par lui-même sans l'appeler.
*

**Avec aucun des deux** : une exécution `-p` nue, ou `query()` qui ne définit ni `canUseTool` ni `permissionPromptToolName`, refuse tout appel d'outil qui aurait invité, et cet événement rapporte ces refus ainsi que ceux que Claude Code décide par lui-même. Avant v2.1.223, Claude Code n'émettait pas cet événement dans les exécutions sans rappel.

* **Avec un outil d'invite MCP**, défini avec `permissionPromptToolName` ou le drapeau [`--permission-prompt-tool`](/docs/fr/cli-reference#cli-flags), et le `permissionPrompts: 'host'` par défaut : Claude Code n'émet pas du tout cet événement, pas même pour les refus de règle qu'il décide par lui-même.
*

**Avec [`permissionPrompts: 'none'`](#options)** : Claude Code refuse les appels qui auraient invité, même quand `canUseTool` ou un outil d'invite MCP est aussi défini, et cet événement rapporte ces refus ainsi que ceux que Claude Code décide par lui-même. Nécessite Claude Code v2.1.259 ou ultérieur.

Dans chaque configuration, cet événement ignore tout refus décidé sur le chemin du hook `PreToolUse`, que le hook ait refusé l'appel lui-même ou qu'une règle de refus ait remplacé la décision d'autorisation ou de demande du hook. L'événement est aussi du meilleur effort : occasionnellement Claude Code enregistre un refus sans émettre cet événement, donc `permission_denials` sur le [message de résultat](#sdkresultmessage) est le registre faisant autorité.

```typescript theme={null}
type SDKPermissionDeniedMessage = {
  type: "system";
  subtype: "permission_denied";
  tool_name: string;
  tool_use_id: string;
  agent_id?: string;
  decision_reason_type?: string;
  decision_reason?: string;
  message: string;
  uuid: UUID;
  session_id: string;
};
```

| Champ                  | Type     | Description                                                                                                                                  |
| ---------------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `tool_name`            | `string` | Nom de l'outil qui a été refusé                                                                                                              |
| `tool_use_id`          | `string` | ID du bloc `tool_use` auquel ce refus répond                                                                                                 |
| `agent_id`             | `string` | ID du sous-agent quand l'appel refusé provient de l'intérieur d'un sous-agent. Reflète le champ sur `can_use_tool` pour le routage côté hôte |
| `decision_reason_type` | `string` | Discriminateur pour le composant qui a décidé, comme `"rule"`, `"mode"`, `"classifier"`, ou `"asyncAgent"`                                   |
| `decision_reason`      | `string` | Raison lisible par l'homme du composant décideur, quand disponible                                                                           |
| `message`              | `string` | Message de rejet retourné au modèle dans le `tool_result`                                                                                    |

<h3 id="sdkpermissiondenial">
  `SDKPermissionDenial`
</h3>

Information sur un usage d'outil refusé.

```typescript theme={null}
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: Record<string, unknown>;
};
```

<h3 id="sdkcontextusage">
  `SDKContextUsage`
</h3>

Forme structurée du rapport `/context`, portée comme `context_usage` sur le [`SDKAssistantMessage`](#sdkassistantmessage) qui livre un résultat `/context`. Agent SDK v0.3.232 et ultérieur exportent le type. Contrairement à [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse), il porte uniquement les données nécessaires pour rendre la ventilation d'utilisation, sans champs d'affichage comme `color` et `gridRows`.

```typescript theme={null}
type SDKContextUsage = {
  model: string;
  total_tokens: number;
  raw_max_tokens: number;
  percentage: number;
  over_limit?: {
    tokens_over: number;
    kind: "hard_limit" | "compaction_window";
  };
  categories: SDKContextUsageCategory[];
  mcp_tools: {
    name: string;
    server_name: string;
    tokens: number;
  }[];
  memory_files: {
    path: string;
    type: string;
    tokens: number;
  }[];
  agents: {
    agent_type: string;
    source: string;
    tokens: number;
  }[];
  skills?: {
    name: string;
    source: string;
    plugin_name?: string;
    tokens: number;
  }[];
};
```

Le tableau liste ce que Claude Code met dans chaque champ. Les champs de `model` à `over_limit` décrivent la session dans son ensemble, et les champs de collection attribuent les jetons aux éléments individuels.

| Champ            | Type                                                      | Description                                                                                                                                                                                                                                                                                                                                                      |
| ---------------- | --------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `model`          | `string`                                                  | Le modèle de la boucle principale pour lequel Claude Code a calculé l'utilisation, pas celui d'un sous-agent                                                                                                                                                                                                                                                     |
| `total_tokens`   | `number`                                                  | L'estimation de Claude Code des jetons en utilisation. Non limité à la fenêtre, donc il peut dépasser `raw_max_tokens` quand la session dépasse la limite                                                                                                                                                                                                        |
| `raw_max_tokens` | `number`                                                  | La fenêtre de contexte du modèle, ou la [fenêtre de compaction automatique](/docs/fr/model-config#context-window-and-auto-compaction) inférieure quand une s'applique, comme une que vous définissez ou la limite de 200K que Claude Code applique à certains modèles avec une fenêtre de 1M de jetons. Claude Code mesure `total_tokens` par rapport à cette fenêtre |
| `percentage`     | `number`                                                  | `total_tokens` en pourcentage arrondi de `raw_max_tokens`, donc il peut dépasser 100 quand la session dépasse la limite                                                                                                                                                                                                                                          |
| `over_limit`     | `object`                                                  | Présent uniquement quand `total_tokens` dépasse `raw_max_tokens`. `tokens_over` est le montant au-dessus, et `kind` dit comment Claude Code a résolu la fenêtre                                                                                                                                                                                                  |
| `categories`     | [`SDKContextUsageCategory`](#sdkcontextusagecategory)`[]` | Une entrée par ligne de la ventilation d'utilisation par catégorie                                                                                                                                                                                                                                                                                               |
| `mcp_tools`      | `object[]`                                                | Jetons attribués à chaque outil MCP, avec son nom de fil, comme `mcp__linear__create_issue`, et son `server_name`                                                                                                                                                                                                                                                |
| `memory_files`   | `object[]`                                                | Jetons attribués à chaque fichier de mémoire chargé, avec son `path` et une étiquette source comme `Project` ou `User` dans `type`                                                                                                                                                                                                                               |
| `agents`         | `object[]`                                                | Jetons attribués à chaque définition de sous-agent personnalisé, avec un identifiant source comme `projectSettings`, `userSettings`, ou `plugin`. Les sous-agents intégrés ne sont pas listés                                                                                                                                                                    |
| `skills`         | `object[]`                                                | Jetons attribués à chaque compétence dans la liste des compétences, avec un identifiant source et, pour les compétences de plugin, le nom du plugin dans `plugin_name`. Absent quand aucune compétence ne contribue de jetons                                                                                                                                    |

`over_limit.kind` enregistre comment Claude Code a résolu la fenêtre, pas si l'API accepte la prochaine requête :

* `hard_limit` : la fenêtre est ce que Claude Code croit être la limite propre du modèle, au-delà de laquelle l'API refuse les requêtes
* `compaction_window` : la fenêtre est une fenêtre de politique de compaction, qui peut ou non coïncider avec la limite du modèle

Claude Code évolue le type de manière additive, ajoutant de nouvelles données comme des champs optionnels plutôt que de remodeler les existants. Lisez les champs que vous connaissez et ignorez ceux que vous ne reconnaissez pas.

<h3 id="sdkcontextusagecategory">
  `SDKContextUsageCategory`
</h3>

Une ligne de la ventilation d'utilisation par catégorie `/context`.

```typescript theme={null}
type SDKContextUsageCategory = {
  name: string;
  tokens: number;
  kind: "used" | "free" | "buffer" | "deferred";
};
```

Le tableau liste ce que Claude Code met dans chaque champ d'une ligne.

| Champ    | Type     | Description                                                                                                                |
| -------- | -------- | -------------------------------------------------------------------------------------------------------------------------- |
| `name`   | `string` | Le nom d'affichage de la ligne comme `/context` l'imprime, comme `Messages`. Classifiez les lignes par `kind`, pas par nom |
| `tokens` | `number` | Le nombre de jetons de la ligne. Les lignes peuvent porter zéro jeton                                                      |
| `kind`   | `string` | Ce que la ligne représente : `used`, `free`, `buffer`, ou `deferred`                                                       |

Chaque valeur `kind` dit ce que les jetons de la ligne sont :

* `used` : contenu qui occupe la fenêtre de contexte
* `free` : la fenêtre restante
* `buffer` : la réserve de compaction
* `deferred` : schémas d'outil que Claude Code tient hors de la fenêtre et exclut du calcul d'utilisation, listés pour la sensibilisation

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

Provenance d'un message de rôle utilisateur. Ceci apparaît comme `origin` sur [`SDKUserMessage`](#sdkusermessage) et est transféré sur le [`SDKResultMessage`](#sdkresultmessage) correspondant pour que vous puissiez dire ce qui a déclenché un tour donné.

```typescript theme={null}
type SDKMessageOrigin =
  | { kind: "human" }
  | { kind: "channel"; server: string }
  | {
      kind: "peer";
      from: string;
      fromMode?: "bypass" | "prompting";
      name?: string;
      fromSession?: string;
      senderTaskId?: string;
      body?: string;
      verifiedPeerPid?: number;
    }
  | {
      kind: "task-notification";
      subkind?: "scheduled-trigger" | "peer-send-message";
      fireReason?: string;
    }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" }
  | { kind: "unclassified" };
```

| `kind`              | Signification                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | Entrée directe de l'utilisateur final. Si votre application transmet ce que l'utilisateur a tapé comme un message utilisateur, définissez son `origin` à `{ kind: "human" }` explicitement : Claude Code traite un message utilisateur sans `origin` comme non attribué, et vérifie que les vérifications qui nécessitent une invite tapée par l'humain, comme le [mot-clé de flux de travail `ultracode`](/docs/fr/workflows#ask-for-a-workflow-in-your-prompt), ne l'acceptent pas. Avant v2.1.210, Claude Code traitait un `origin` absent sur un message utilisateur comme une entrée humaine. |
| `channel`           | Message arrivant sur un [canal](/docs/fr/channels). `server` est le nom du serveur MCP source.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `peer`              | Message d'un autre agent : un [coéquipier](/docs/fr/agent-teams) en processus ou un [pair entre sessions](/docs/fr/cross-session-messaging), une autre de vos sessions Claude Code. Voir [Champs d'origine pair](#peer-origin-fields) pour la sémantique par champ et le modèle de confiance.                                                                                                                                                                                                                                                                                                           |
| `task-notification` | Tour synthétique injecté pour une livraison qui arrive sans une invite utilisateur fraîche, comme une tâche de fond terminée ; voir [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) pour ce bras. Une invite que votre application [déclare comme une exécution planifiée](#declare-a-scheduled-run) porte ce type aussi. Le `subkind` optionnel marque ce qui a levé la notification. Voir [Sous-types de notification de tâche](#task-notification-subkinds).                                                                                                                   |
| `coordinator`       | Message d'un coordinateur d'équipe dans une [équipe d'agents](/docs/fr/agent-teams).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `auto-continuation` | Tour synthétique injecté quand la session continue sans entrée utilisateur fraîche, comme un résultat de commande qui déclenche une invite de suivi.                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `unclassified`      | Tour injecté dont l'origine n'a pas pu être déterminée. Nécessite Claude Code v2.1.223 ou ultérieur. Quand Claude Code reçoit un [`SDKUserMessage`](#sdkusermessage) avec `isSynthetic: true` et ne peut pas le classer comme un autre `kind`, il définit ce type à l'arrivée du message et encadre le tour au modèle comme une source non-utilisateur plutôt que de le traiter comme une entrée humaine. Votre application ne devrait pas définir cette valeur.                                                                                                                              |

<h3 id="task-notification-subkinds">
  Sous-types de notification de tâche
</h3>

Quand Claude Code livre une notification de tâche dans une session, il définit `subkind` sur le `origin` de la notification si les serveurs Anthropic ont vérifié d'où provenait cette notification. Il le définit aussi quand votre application [déclare le message comme une exécution planifiée](#declare-a-scheduled-run) elle-même, ce qui nécessite TypeScript Agent SDK v0.3.280 ou ultérieur. `subkind` nécessite Claude Code v2.1.213 ou ultérieur, et prend l'une de deux valeurs :

* `scheduled-trigger` : la notification est une invite stockée d'une [routine](/docs/fr/routines), livrée parce que l'un des déclencheurs de la routine s'est déclenché : son horaire, son [déclencheur API](/docs/fr/routines#add-an-api-trigger), son [déclencheur GitHub](/docs/fr/routines#add-a-github-trigger), ou **Exécuter maintenant**. Une invite que votre application [déclare comme une exécution planifiée](#declare-a-scheduled-run) porte cette valeur aussi. Claude Code encadre celles-ci au modèle comme la tâche assignée de la session, avec un avis différent de l'[avis que les autres notifications de tâche portent](#sdktasknotificationmessage).
*

`peer-send-message` : la notification est un message qu'une autre de vos sessions a envoyé avec l'outil côté serveur `send_message` que les [sessions cloud](/docs/fr/claude-code-on-the-web) utilisent pour se messagerie mutuellement, pas l'[outil `SendMessage` entre sessions](/docs/fr/cross-session-messaging), et les serveurs Anthropic ont vérifié que les deux sessions appartiennent au même groupe privé de sessions. Nécessite Claude Code v2.1.224 ou ultérieur. Une livraison `send_message` que les serveurs n'ont pas vérifiée de cette façon n'obtient pas de subkind.

Chaque autre notification de tâche n'a pas de `subkind`. Cela inclut l'[activité PR](/docs/fr/claude-code-on-the-web#how-claude-responds-to-pr-activity) livrée dans une session et les événements de fond comme une tâche terminée. Les messages de l'[outil `SendMessage` entre sessions](/docs/fr/cross-session-messaging) ne sont pas du tout des notifications de tâche : qu'ils proviennent d'une session sur la même machine ou via les serveurs Anthropic d'une autre machine, Claude Code leur donne `kind: "peer"` et les [champs d'origine pair](#peer-origin-fields).

`fireReason` dit pourquoi une notification `scheduled-trigger` s'est déclenchée, comme un jeton minuscule court comme `scheduled`, `manual`, `retry`, `catch_up`, ou `api`. Les serveurs Anthropic le définissent sur les livraisons d'une [routine](/docs/fr/routines), et votre application le définit quand elle déclare une exécution planifiée. Il est absent quand aucun des deux n'en a envoyé un. Nécessite TypeScript Agent SDK v0.3.280 ou ultérieur.

<h4 id="declare-a-scheduled-run">
  Déclarer une exécution planifiée
</h4>

Si votre application exécute des invites selon son propre horaire, déclarez chaque exécution pour que Claude Code encadre le tour au modèle comme une tâche planifiée plutôt que comme une entrée en direct de l'utilisateur. Démarrez la session avec `CLAUDE_CODE_HOST_SCHEDULED_RUN` défini à `1` dans [`env`](#options), puis envoyez le [`SDKUserMessage`](#sdkusermessage) de l'exécution avec `origin: { kind: "task-notification", subkind: "scheduled-trigger", fireReason: "scheduled" }` et sans `isSynthetic`. Claude Code ignore la déclaration dans un processus démarré sans cette variable. Il l'ignore aussi dans un processus dont l'environnement porte [`CLAUDECODE`](/docs/fr/env-vars) ou `CLAUDE_CODE_CHILD_SESSION`. Claude Code conserve `fireReason` uniquement quand la valeur est 1 à 32 lettres minuscules ou traits de soulignement. Nécessite TypeScript Agent SDK v0.3.280 ou ultérieur.

<h3 id="peer-origin-fields">
  Champs d'origine pair
</h3>

Une origine `peer` identifie quel agent a envoyé le message : un [coéquipier](/docs/fr/agent-teams) en processus envoyant à `main` avec `SendMessage`, ou un [pair entre sessions](/docs/fr/cross-session-messaging), une autre de vos sessions Claude Code. Les pairs entre sessions nécessitent Claude Code v2.1.224 ou ultérieur sur macOS et Linux ; voir [disponibilité de la messagerie entre sessions](/docs/fr/cross-session-messaging#availability) pour l'exigence Windows native. Un pair entre sessions peut s'exécuter sur la même machine, ou sur [une autre de vos machines](/docs/fr/cross-session-messaging#message-sessions-on-other-machines) ou [dans le cloud](/docs/fr/claude-code-on-the-web) quand son message arrive via Remote Control. Les deux types d'expéditeur remplissent les champs différemment :

* `from` : le nom du coéquipier, ou l'adresse de l'expéditeur pour un pair entre sessions. Pour un [message entre machines unidirectionnel](/docs/fr/cross-session-messaging#message-sessions-on-other-machines), l'expéditeur n'a pas d'adresse de réponse et `from` est `"unknown"`. La valeur est créée par l'expéditeur ; `verifiedPeerPid` est l'identité vérifiée.
*

`fromMode` : la classe de permissions de la session d'envoi, `bypass` ou `prompting`, déclarée par un hôte qui relaie un message pair entre vos sessions, comme l'[application de bureau](/docs/fr/desktop#work-across-sessions). Claude Code la lit dans la session de réception quand il applique les [contrôles entrants](/docs/fr/cross-session-messaging#control-inbound-messages). Nécessite Agent SDK v0.3.234 ou ultérieur.

* `senderTaskId` : l'ID de tâche du coéquipier. Absent pour un pair entre sessions.
*

`name` : le nom d'affichage de l'expéditeur, normalisé par Claude Code : il supprime les points de code de contrôle, format, substitut et séparateur de ligne ou paragraphe Unicode, puis coupe le résultat et le limite à 64 points de code avec une ellipse. Nécessite Claude Code v2.1.205 ou ultérieur.

*

`body` : le corps du message décodé avec l'enveloppe pair supprimée, octet-exact avec ce que le modèle voit. Toujours présent pour un message de coéquipier ; pour un pair entre sessions, présent uniquement quand le tour est exactement une enveloppe pair formée par Claude Code. Rendez `name` et `body` au lieu de réanalyser le texte du message. Nécessite Claude Code v2.1.205 ou ultérieur.

*

`fromSession` : l'ID de session ouvrable par l'hôte de l'expéditeur, défini par l'hôte de l'expéditeur pour que votre interface utilisateur puisse se lier à la session d'envoi. Comme `from`, il est affirmé par l'expéditeur : utilisez-le uniquement comme cible de navigation, et ne le traitez pas comme une preuve de l'identité de l'expéditeur. Nécessite Claude Code v2.1.216 ou ultérieur.

*

`verifiedPeerPid` : l'ID de processus du processus qui s'est connecté à la prise de messagerie entre sessions de cette session, vérifié par le noyau et lu de la connexion elle-même, jamais de la charge utile. Utilisez-le, pas `from`, pour identifier l'expéditeur : `from` est forgeable par n'importe quel processus du même utilisateur. Le champ est absent quand Claude Code ne peut pas le vérifier, comme sur Windows ou l'entrée non-socket, donc une valeur absente signifie que l'expéditeur n'est pas vérifié. Pour le trafic relayé, il identifie le relais plutôt que l'auteur du message, et les ID de processus sont recyclables, donc traitez-le comme la provenance plutôt que comme un jeton d'authentification. Nécessite Claude Code v2.1.216 ou ultérieur.

<h2 id="hook-types">
  Types de hook
</h2>

Pour un guide complet sur l'utilisation des hooks avec des exemples et des modèles courants, voir le [guide des hooks](/docs/fr/agent-sdk/hooks).

<h3 id="hookevent">
  `HookEvent`
</h3>

Événements de hook disponibles.

```typescript theme={null}
type HookEvent =
  | "PreToolUse"
  | "PostToolUse"
  | "PostToolUseFailure"
  | "PostToolBatch"
  | "Notification"
  | "UserPromptSubmit"
  | "UserPromptExpansion"
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "StopFailure"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PostCompact"
  | "PreModelSwitch"
  | "PostModelSwitch"
  | "PermissionRequest"
  | "PermissionDenied"
  | "Setup"
  | "TeammateIdle"
  | "TaskCreated"
  | "TaskCompleted"
  | "Elicitation"
  | "ElicitationResult"
  | "ConfigChange"
  | "DirectoryAdded"
  | "WorktreeCreate"
  | "WorktreeRemove"
  | "InstructionsLoaded"
  | "CwdChanged"
  | "FileChanged"
  | "MessageDisplay";
```

<h3 id="hookcallback">
  `HookCallback`
</h3>

Type de fonction de rappel de hook.

```typescript theme={null}
type HookCallback = (
  input: HookInput, // Union de tous les types d'entrée de hook
  toolUseID: string | undefined,
  options: { signal: AbortSignal }
) => Promise<HookJSONOutput>;
```

<h3 id="hookcallbackmatcher">
  `HookCallbackMatcher`
</h3>

Configuration de hook avec matcher optionnel.

```typescript theme={null}
interface HookCallbackMatcher {
  matcher?: string;
  hooks: HookCallback[];
  timeout?: number; // Délai d'expiration en secondes pour tous les hooks dans ce matcher
}
```

<h3 id="hookinput">
  `HookInput`
</h3>

Type union de tous les types d'entrée de hook.

```typescript theme={null}
type HookInput =
  | PreToolUseHookInput
  | PostToolUseHookInput
  | PostToolUseFailureHookInput
  | PostToolBatchHookInput
  | PermissionDeniedHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | UserPromptExpansionHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | StopFailureHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PostCompactHookInput
  | PreModelSwitchHookInput
  | PostModelSwitchHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCreatedHookInput
  | TaskCompletedHookInput
  | ElicitationHookInput
  | ElicitationResultHookInput
  | ConfigChangeHookInput
  | InstructionsLoadedHookInput
  | DirectoryAddedHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
  | CwdChangedHookInput
  | FileChangedHookInput
  | MessageDisplayHookInput;
```

<h3 id="basehookinput">
  `BaseHookInput`
</h3>

Interface de base que tous les types d'entrée de hook étendent.

```typescript theme={null}
type BaseHookInput = {
  session_id: string;
  transcript_path: string;
  cwd: string;
  prompt_id?: string;
  permission_mode?: string;
  effort?: { level: string };
  agent_id?: string;
  agent_type?: string;
};
```

Le champ `prompt_id` est un UUID identifiant l'invite utilisateur actuellement traitée. Il correspond à l'[attribut `prompt.id` sur les événements OpenTelemetry](/docs/fr/monitoring-usage#event-correlation-attributes) et est absent jusqu'à la première entrée utilisateur. Nécessite Claude Code v2.1.196 ou ultérieur.

<h4 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h4>

```typescript theme={null}
type PreToolUseHookInput = BaseHookInput & {
  hook_event_name: "PreToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  mcp_server?: McpServerProvenance;
};
```

`mcp_server` est présent quand l'outil provient d'un serveur MCP ; voir [`McpServerProvenance`](#mcpserverprovenance). Les entrées `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` et `PermissionDenied` portent le même champ. Le champ nécessite Agent SDK v0.3.274 ou ultérieur.

<h4 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h4>

```typescript theme={null}
type PostToolUseHookInput = BaseHookInput & {
  hook_event_name: "PostToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_response: unknown;
  tool_use_id: string;
  duration_ms?: number;
  mcp_server?: McpServerProvenance;
};
```

<h4 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h4>

```typescript theme={null}
type PostToolUseFailureHookInput = BaseHookInput & {
  hook_event_name: "PostToolUseFailure";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  error: string;
  is_interrupt?: boolean;
  duration_ms?: number;
  mcp_server?: McpServerProvenance;
};
```

<h4 id="posttoolbatchhookinput">
  `PostToolBatchHookInput`
</h4>

S'exécute une fois après que chaque appel d'outil dans un lot ait été résolu, avant la prochaine demande de modèle. `tool_response` porte le contenu `tool_result` sérialisé que le modèle voit ; la forme diffère de l'objet `Output` structuré de `PostToolUseHookInput`.

```typescript theme={null}
type PostToolBatchHookInput = BaseHookInput & {
  hook_event_name: "PostToolBatch";
  tool_calls: PostToolBatchToolCall[];
};

type PostToolBatchToolCall = {
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  tool_response?: unknown;
};
```

<h4 id="permissiondeniedhookinput">
  `PermissionDeniedHookInput`
</h4>

```typescript theme={null}
type PermissionDeniedHookInput = BaseHookInput & {
  hook_event_name: "PermissionDenied";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  reason: string;
  mcp_server?: McpServerProvenance;
};
```

<h4 id="notificationhookinput">
  `NotificationHookInput`
</h4>

```typescript theme={null}
type NotificationHookInput = BaseHookInput & {
  hook_event_name: "Notification";
  message: string;
  title?: string;
  notification_type: string;
};
```

<h4 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h4>

```typescript theme={null}
type UserPromptSubmitHookInput = BaseHookInput & {
  hook_event_name: "UserPromptSubmit";
  prompt: string;
  session_title?: string;
};
```

<h4 id="userpromptexpansionhookinput">
  `UserPromptExpansionHookInput`
</h4>

```typescript theme={null}
type UserPromptExpansionHookInput = BaseHookInput & {
  hook_event_name: "UserPromptExpansion";
  expansion_type: "slash_command" | "mcp_prompt";
  command_name: string;
  command_args: string;
  command_source?: string;
  prompt: string;
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact" | "fork";
  agent_type?: string;
  model?: string;
  session_title?: string;
};
```

<h4 id="sessionendhookinput">
  `SessionEndHookInput`
</h4>

```typescript theme={null}
type SessionEndHookInput = BaseHookInput & {
  hook_event_name: "SessionEnd";
  reason: ExitReason; // Chaîne du tableau EXIT_REASONS
};
```

<h4 id="stophookinput">
  `StopHookInput`
</h4>

```typescript theme={null}
type StopHookInput = BaseHookInput & {
  hook_event_name: "Stop";
  stop_hook_active: boolean;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};
```

<h4 id="stopfailurehookinput">
  `StopFailureHookInput`
</h4>

```typescript theme={null}
type StopFailureHookInput = BaseHookInput & {
  hook_event_name: "StopFailure";
  error: SDKAssistantMessageError;
  error_details?: string;
  last_assistant_message?: string;
};
```

<h4 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h4>

```typescript theme={null}
type SubagentStartHookInput = BaseHookInput & {
  hook_event_name: "SubagentStart";
  agent_id: string;
  agent_type: string;
};
```

<h4 id="subagentstophookinput">
  `SubagentStopHookInput`
</h4>

```typescript theme={null}
type SubagentStopHookInput = BaseHookInput & {
  hook_event_name: "SubagentStop";
  stop_hook_active: boolean;
  agent_id: string;
  agent_transcript_path: string;
  agent_type: string;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};

type BackgroundTaskSummary = {
  id: string;
  type: string;
  status: string;
  description: string;
  command?: string;
  agent_type?: string;
  server?: string;
  tool?: string;
  name?: string;
};

type SessionCronSummary = {
  id: string;
  schedule: string;
  recurring: boolean;
  prompt: string;
};
```

<h4 id="precompacthookinput">
  `PreCompactHookInput`
</h4>

```typescript theme={null}
type PreCompactHookInput = BaseHookInput & {
  hook_event_name: "PreCompact";
  trigger: "manual" | "auto";
  custom_instructions: string | null;
};
```

<h4 id="postcompacthookinput">
  `PostCompactHookInput`
</h4>

```typescript theme={null}
type PostCompactHookInput = BaseHookInput & {
  hook_event_name: "PostCompact";
  trigger: "manual" | "auto";
  compact_summary: string;
};
```

<h4 id="premodelswitchhookinput">
  `PreModelSwitchHookInput`
</h4>

S'exécute avant qu'un changement de modèle demandé ne prenne effet. `context_tokens` et les champs qui le suivent estiment le coût de renvoi de la conversation au nouveau modèle. Pour les descriptions complètes des champs et la sémantique de blocage, voir [PreModelSwitch](/docs/fr/hooks#premodelswitch).

```typescript theme={null}
type PreModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PreModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
};
```

<h4 id="postmodelswitchhookinput">
  `PostModelSwitchHookInput`
</h4>

S'exécute après que le modèle de la session change. Il porte les mêmes champs que `PreModelSwitchHookInput`, avec deux valeurs `source` supplémentaires. Voir [PostModelSwitch](/docs/fr/hooks#postmodelswitch).

```typescript theme={null}
type PostModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PostModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk" | "auto" | "resume";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
};
```

<h4 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h4>

```typescript theme={null}
type PermissionRequestHookInput = BaseHookInput & {
  hook_event_name: "PermissionRequest";
  tool_name: string;
  tool_input: unknown;
  permission_suggestions?: PermissionUpdate[];
  mcp_server?: McpServerProvenance;
};
```

<h4 id="setuphookinput">
  `SetupHookInput`
</h4>

```typescript theme={null}
type SetupHookInput = BaseHookInput & {
  hook_event_name: "Setup";
  trigger: "init" | "maintenance";
};
```

<h4 id="teammateidlehookinput">
  `TeammateIdleHookInput`
</h4>

```typescript theme={null}
type TeammateIdleHookInput = BaseHookInput & {
  hook_event_name: "TeammateIdle";
  teammate_name: string;
  /** @deprecated depuis v2.1.178. Porte le nom d'équipe dérivé de la session ; sera supprimé. */
  team_name: string;
};
```

<h4 id="taskcreatedhookinput">
  `TaskCreatedHookInput`
</h4>

```typescript theme={null}
type TaskCreatedHookInput = BaseHookInput & {
  hook_event_name: "TaskCreated";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated depuis v2.1.178. Porte le nom d'équipe dérivé de la session ; sera supprimé. */
  team_name?: string;
};
```

<h4 id="taskcompletedhookinput">
  `TaskCompletedHookInput`
</h4>

```typescript theme={null}
type TaskCompletedHookInput = BaseHookInput & {
  hook_event_name: "TaskCompleted";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated depuis v2.1.178. Porte le nom d'équipe dérivé de la session ; sera supprimé. */
  team_name?: string;
};
```

<h4 id="elicitationhookinput">
  `ElicitationHookInput`
</h4>

```typescript theme={null}
type ElicitationHookInput = BaseHookInput & {
  hook_event_name: "Elicitation";
  mcp_server_name: string;
  message: string;
  mode?: "form" | "url";
  url?: string;
  elicitation_id?: string;
  requested_schema?: Record<string, unknown>;
};
```

<h4 id="elicitationresulthookinput">
  `ElicitationResultHookInput`
</h4>

```typescript theme={null}
type ElicitationResultHookInput = BaseHookInput & {
  hook_event_name: "ElicitationResult";
  mcp_server_name: string;
  elicitation_id?: string;
  mode?: "form" | "url";
  action: "accept" | "decline" | "cancel";
  content?: Record<string, unknown>;
};
```

<h4 id="configchangehookinput">
  `ConfigChangeHookInput`
</h4>

```typescript theme={null}
type ConfigChangeHookInput = BaseHookInput & {
  hook_event_name: "ConfigChange";
  source:
    | "user_settings"
    | "project_settings"
    | "local_settings"
    | "policy_settings"
    | "skills";
  file_path?: string;
};
```

<h4 id="instructionsloadedhookinput">
  `InstructionsLoadedHookInput`
</h4>

```typescript theme={null}
type InstructionsLoadedHookInput = BaseHookInput & {
  hook_event_name: "InstructionsLoaded";
  file_path: string;
  memory_type: "User" | "Project" | "Local" | "Managed";
  load_reason:
    | "session_start"
    | "nested_traversal"
    | "path_glob_match"
    | "include"
    | "compact";
  globs?: string[];
  trigger_file_path?: string;
  parent_file_path?: string;
};
```

<h4 id="directoryaddedhookinput">
  `DirectoryAddedHookInput`
</h4>

```typescript theme={null}
type DirectoryAddedHookInput = BaseHookInput & {
  hook_event_name: "DirectoryAdded";
  directory: string;
  source: "slash_command" | "register_repo_root";
};
```

`directory` est le chemin absolu du répertoire qui a été ajouté. `source` est `"slash_command"` quand `/add-dir` l'a ajouté et `"register_repo_root"` quand la demande de contrôle du SDK l'a fait.

<h4 id="worktreecreatehookinput">
  `WorktreeCreateHookInput`
</h4>

```typescript theme={null}
type WorktreeCreateHookInput = BaseHookInput & {
  hook_event_name: "WorktreeCreate";
  name: string;
};
```

<h4 id="worktreeremovehookinput">
  `WorktreeRemoveHookInput`
</h4>

```typescript theme={null}
type WorktreeRemoveHookInput = BaseHookInput & {
  hook_event_name: "WorktreeRemove";
  worktree_path: string;
};
```

<h4 id="cwdchangedhookinput">
  `CwdChangedHookInput`
</h4>

```typescript theme={null}
type CwdChangedHookInput = BaseHookInput & {
  hook_event_name: "CwdChanged";
  old_cwd: string;
  new_cwd: string;
};
```

<h4 id="filechangedhookinput">
  `FileChangedHookInput`
</h4>

```typescript theme={null}
type FileChangedHookInput = BaseHookInput & {
  hook_event_name: "FileChanged";
  file_path: string;
  event: "change" | "add" | "unlink";
};
```

<h4 id="messagedisplayhookinput">
  `MessageDisplayHookInput`
</h4>

```typescript theme={null}
type MessageDisplayHookInput = BaseHookInput & {
  hook_event_name: "MessageDisplay";
  turn_id: string;
  message_id: string;
  index: number;
  final: boolean;
  delta: string;
};
```

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

Valeur de retour du hook.

```typescript theme={null}
type HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput;
```

<h4 id="asynchookjsonoutput">
  `AsyncHookJSONOutput`
</h4>

```typescript theme={null}
type AsyncHookJSONOutput = {
  async: true;
  asyncTimeout?: number;
};
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

```typescript theme={null}
type SyncHookJSONOutput = {
  continue?: boolean;
  suppressOutput?: boolean;
  stopReason?: string;
  decision?: "approve" | "block";
  systemMessage?: string;
  /**
   * Une séquence d'échappement de terminal (par exemple OSC 9 / OSC 777 desktop-notification)
   * pour que Claude Code émette en votre nom. Seuls les OSCs de notification/titre
   * (0, 1, 2, 9, 99, 777) et BEL sont autorisés ; une valeur contenant
   * autre chose est ignorée dans son ensemble. Seul l'interface CLI interactive l'émet ;
   * le SDK ignore le champ.
   */
  terminalSequence?: string;
  reason?: string;
  hookSpecificOutput?:
    | {
        hookEventName: "PreToolUse";
        permissionDecision?: "allow" | "deny" | "ask" | "defer";
        permissionDecisionReason?: string;
        updatedInput?: Record<string, unknown>;
        additionalContext?: string;
      }
    | {
        hookEventName: "UserPromptSubmit";
        additionalContext?: string;
        sessionTitle?: string;
        /** Quand la décision est « block », omettez l'invite originale du message de blocage. */
        suppressOriginalPrompt?: boolean;
      }
    | {
        hookEventName: "UserPromptExpansion";
        additionalContext?: string;
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
        initialUserMessage?: string;
        sessionTitle?: string;
        watchPaths?: string[];
        /**
         * Rescannez les répertoires de compétences et de commandes après que les hooks SessionStart
         * se terminent, afin que les compétences installées par le hook soient disponibles dans la
         * même session.
         */
        reloadSkills?: boolean;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "PreModelSwitch";
        /**
         * Même contrat que PreToolUse : « allow » procède, « deny » annule
         * le changement, « ask » demande à l'utilisateur de confirmer. Seul /model dans une
         * session interactive affiche cette invite ; chaque autre surface,
         * y compris les demandes set_model, traite « ask » comme un refus.
         */
        permissionDecision?: "allow" | "deny" | "ask";
        permissionDecisionReason?: string;
      }
    | {
        hookEventName: "PostModelSwitch";
        /** Atteint le modèle avec la prochaine demande que le nouveau modèle traite. */
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
        /**
         * Note courte sur le résultat de cet appel d'outil pour le classificateur de permissions
         * en mode automatique. Limité à 2000 caractères, partagé entre
         * tous les hooks qui répondent au même appel ; honoré sur les réponses de hooks synchrones uniquement.
         * Ne copiez pas la sortie d'outil non fiable dedans.
         */
        classifierContext?: string;
        updatedToolOutput?: unknown;
        /** @deprecated Utilisez `updatedToolOutput`, qui fonctionne pour tous les outils. */
        updatedMCPToolOutput?: unknown;
      }
    | {
        hookEventName: "PostToolUseFailure";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolBatch";
        additionalContext?: string;
      }
    | {
        hookEventName: "Stop";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStop";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionDenied";
        retry?: boolean;
      }
    | {
        hookEventName: "Notification";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionRequest";
        decision:
          | {
              behavior: "allow";
              updatedInput?: Record<string, unknown>;
              updatedPermissions?: PermissionUpdate[];
            }
          | {
              behavior: "deny";
              message?: string;
              interrupt?: boolean;
            };
      }
    | {
        hookEventName: "Elicitation";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "ElicitationResult";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "CwdChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "FileChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "WorktreeCreate";
        worktreePath: string;
      }
    | {
        hookEventName: "MessageDisplay";
        /** Texte affiché à la place du delta. Omettez (ou retournez le delta inchangé) pour afficher l'original. */
        displayContent?: string;
      };
};
```

<h2 id="tool-input-types">
  Types d'entrée d'outil
</h2>

Documentation des schémas d'entrée pour tous les outils Claude Code intégrés. Ces types sont exportés depuis `@anthropic-ai/claude-agent-sdk` et peuvent être utilisés pour les interactions d'outils type-safe.

<h3 id="toolinputschemas">
  `ToolInputSchemas`
</h3>

Union de types d'entrée d'outil exportée depuis `@anthropic-ai/claude-agent-sdk` ; les membres incluent :

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | ArtifactInput
  | AskUserQuestionInput
  | BashInput
  | CronCreateInput
  | CronDeleteInput
  | CronListInput
  | EnterPlanModeInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | ExitWorktreeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ProjectsInput
  | PushNotificationInput
  | ReadMcpResourceDirInput
  | ReadMcpResourceInput
  | RefreshMcpToolsInput
  | RemoteTriggerInput
  | ReportFindingsInput
  | ScheduleWakeupInput
  | ShowOnboardingRolePickerInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**Nom de l'outil :** `Agent`. Le nom précédent `Task` est toujours accepté comme alias, et le tableau `tools` dans le message d'initialisation [`SDKSystemMessage`](#sdksystemmessage) répertorie actuellement cet outil comme `Task` pour la compatibilité rétroactive.

<Note>
  Le champ `mode` est déprécié et ignoré sur Claude Code v2.1.212 ou version ultérieure. Un sous-agent s'exécute soit en mode de permission de la session parent, soit en mode de sa définition [`permissionMode`](#agentdefinition), et les [règles d'héritage des sous-agents](/docs/fr/agent-sdk/permissions#available-modes) décident lequel.
</Note>

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  team_name?: string; // Déprécié ; ignoré
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan"; // Déprécié ; ignoré. Les règles d'héritage des sous-agents décident du mode de permission d'un sous-agent
  isolation?: "worktree" | "remote";
};
```

Lance un nouvel agent pour gérer les tâches complexes et multi-étapes de manière autonome.

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**Nom de l'outil :** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionInput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
  answers?: Record<string, string>;
  annotations?: Record<string, { preview?: string; notes?: string }>;
  metadata?: { source?: string };
};
```

Pose des questions de clarification à l'utilisateur pendant l'exécution. Voir [Gérer les approbations et l'entrée utilisateur](/docs/fr/agent-sdk/user-input#handle-clarifying-questions) pour les détails d'utilisation.

<h3 id="bash">
  Bash
</h3>

**Nom de l'outil :** `Bash`

```typescript theme={null}
type BashInput = {
  command: string;
  timeout?: number; // milliseconds, max 600000; higher values are clamped to the max
  description?: string;
  run_in_background?: boolean;
  dangerouslyDisableSandbox?: boolean;
};
```

Exécute les commandes Bash avec délai d'expiration optionnel et exécution en arrière-plan. Le répertoire de travail persiste entre les commandes, y compris les commandes exécutées dans les tours ultérieurs d'une session multi-tour ; l'état du shell tel que les variables d'environnement exportées ne persiste pas. Pour les limites sur les changements de répertoire qui persistent, voir [Ce qui persiste entre les commandes](/docs/fr/tools-reference#what-persists-between-commands).

<h3 id="monitor">
  Monitor
</h3>

**Nom de l'outil :** `Monitor`

```typescript theme={null}
type MonitorInput = {
  description: string;
  timeout_ms: number;
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
};
```

Exécute une source de fond et livre chaque événement à Claude pour qu'il puisse réagir sans interrogation : `command` exécute un script et émet un événement par ligne stdout, et `ws` ouvre une WebSocket et émet un événement par trame texte. Fournissez exactement l'un de `command` ou `ws`. La source `ws` nécessite Claude Code v2.1.195 ou version ultérieure.

`timeout_ms` est la date limite de la montre en millisecondes. Elle est par défaut 300000 et accepte les valeurs jusqu'à 3600000. La date limite effective est au maximum 1800000, soit 30 minutes, donc une valeur acceptée plus grande est raccourcie à cela. À la date limite, la montre se termine et Claude reçoit un avis pour qu'il puisse démarrer une nouvelle montre s'il en a toujours besoin.

Le type exporté marque `timeout_ms` comme requis car le schéma remplit la valeur par défaut ; un appel qui l'omet valide.

Lorsque Monitor exécute une commande, il suit les mêmes règles de permission que Bash ; une montre WebSocket demande une approbation séparément. Voir la [référence de l'outil Monitor](/docs/fr/tools-reference#monitor-tool) pour le comportement et la disponibilité du fournisseur.

<h3 id="taskoutput">
  TaskOutput
</h3>

Supprimé dans Claude Code v2.1.277, ainsi que son type `TaskOutputInput`. Récupérait précédemment la sortie d'une tâche de fond en cours d'exécution ou terminée ; Claude lit le fichier de sortie d'une tâche de fond avec `Read` à la place.

Une entrée `disallowedTools` ou une règle de refus qui nomme toujours `TaskOutput` est ignorée sans avertissement.

<h3 id="edit">
  Edit
</h3>

**Nom de l'outil :** `Edit`

```typescript theme={null}
type FileEditInput = {
  file_path: string;
  old_string: string;
  new_string: string;
  replace_all?: boolean;
};
```

Effectue des remplacements de chaînes exacts dans les fichiers.

<h3 id="read">
  Read
</h3>

**Nom de l'outil :** `Read`

```typescript theme={null}
type FileReadInput = {
  file_path: string;
  offset?: number;
  limit?: number;
  pages?: string;
};
```

Lit les fichiers du système de fichiers local, y compris le texte, les images, les PDF et les carnets Jupyter. Utilisez `pages` pour les plages de pages PDF (par exemple, `"1-5"`).

Pour un PDF, Claude reçoit le contenu du fichier dans le `tool_result` de l'appel Read. Une lecture qui retourne la sortie `pdf` [output](#tool-output-types) porte un bloc `text` de résumé suivi d'un bloc `document`. Une qui retourne la sortie `parts` porte le bloc `text` de résumé suivi d'un bloc par page extraite : un bloc `image`, ou un bloc `text` nommant la page lorsque Claude Code n'a pas pu la rendre en tant qu'image. Avant Agent SDK v0.3.242, Claude Code livrait le contenu du fichier en tant que message `user` séparé après le résultat de l'outil.

<h3 id="write">
  Write
</h3>

**Nom de l'outil :** `Write`

```typescript theme={null}
type FileWriteInput = {
  file_path: string;
  content: string;
};
```

Écrit un fichier dans le système de fichiers local, en écrasant s'il existe.

<h3 id="glob">
  Glob
</h3>

**Nom de l'outil :** `Glob`

```typescript theme={null}
type GlobInput = {
  pattern: string;
  path?: string;
};
```

Correspondance de motif de fichier rapide qui fonctionne avec n'importe quelle taille de base de code.

<h3 id="grep">
  Grep
</h3>

**Nom de l'outil :** `Grep`

```typescript theme={null}
type GrepInput = {
  pattern: string;
  path?: string;
  glob?: string;
  type?: string;
  output_mode?: "content" | "files_with_matches" | "count";
  "-i"?: boolean;
  "-o"?: boolean; // print only the matched parts of each line; requires output_mode: "content"
  "-n"?: boolean;
  "-B"?: number;
  "-A"?: number;
  "-C"?: number;
  context?: number;
  head_limit?: number;
  offset?: number;
  multiline?: boolean;
};
```

Outil de recherche puissant construit sur ripgrep avec support regex.

<h3 id="taskstop">
  TaskStop
</h3>

**Nom de l'outil :** `TaskStop`

```typescript theme={null}
type TaskStopInput = {
  task_id?: string;
  shell_id?: string; // Déprécié : utilisez task_id
};
```

Arrête une tâche de fond en cours d'exécution ou un shell par ID. À partir de v2.1.198, `task_id` accepte également un coéquipier d'équipe d'agent ou un agent de fond nommé par ID d'agent ou nom.

<h3 id="notebookedit">
  NotebookEdit
</h3>

**Nom de l'outil :** `NotebookEdit`

```typescript theme={null}
type NotebookEditInput = {
  notebook_path: string;
  cell_id?: string;
  new_source: string;
  cell_type?: "code" | "markdown";
  edit_mode?: "replace" | "insert" | "delete";
};
```

Édite les cellules dans les fichiers de carnet Jupyter.

<h3 id="webfetch">
  WebFetch
</h3>

**Nom de l'outil :** `WebFetch`

```typescript theme={null}
type WebFetchInput = {
  url: string;
  prompt: string;
};
```

Récupère le contenu d'une URL et le traite avec un modèle IA.

<h3 id="websearch">
  WebSearch
</h3>

**Nom de l'outil :** `WebSearch`

```typescript theme={null}
type WebSearchInput = {
  query: string;
  allowed_domains?: string[];
  blocked_domains?: string[];
};
```

Recherche le web et retourne les résultats formatés.

<h3 id="workflow">
  Workflow
</h3>

**Nom de l'outil :** `Workflow`

```typescript theme={null}
type WorkflowInput = {
  script?: string;
  name?: string;
  scriptPath?: string;
  args?: unknown; // any JSON value; the published typings render this as an object map
  resumeFromRunId?: string;
  title?: string; // ignored; the script's meta block sets the title
  description?: string; // ignored; the script's meta block sets the description
};
```

Exécute un [flux de travail dynamique](/docs/fr/workflows) : un script qui orchestre de nombreux sous-agents en arrière-plan et retourne un résultat consolidé. L'outil `Workflow` est disponible dans Agent SDK v0.3.149 et versions ultérieures. Au moins l'un de `script`, `name` ou `scriptPath` est requis.

| Champ             | Type      | Description                                                                                                                                                                                                                                                                                                                                          |
| ----------------- | --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | Script de flux de travail en ligne. Doit commencer par `export const meta = { name, description }` comme littéral, suivi du corps du script utilisant `agent()`, `parallel()`, `pipeline()` et `phase()`. Un tableau `phases` optionnel dans `meta` regroupe les agents sous des étapes nommées dans la vue de progression                           |
| `name`            | `string`  | Nom d'un flux de travail intégré ou d'un flux de travail enregistré dans `.claude/workflows/`. Résolu en script                                                                                                                                                                                                                                      |
| `scriptPath`      | `string`  | Chemin vers un fichier de script de flux de travail sur le disque. Prend la priorité sur `script` et `name`. Claude Code persiste chaque invocation du script et retourne le chemin dans le résultat, afin que vous puissiez éditer ce fichier et réinvoquer avec le même `scriptPath` pour itérer                                                   |
| `args`            | `unknown` | Valeur d'entrée exposée au script en tant que `args` global, pour les flux de travail nommés paramétrés tels qu'une question de recherche ou une liste de chemins de fichiers. Passez les tableaux et les objets comme des valeurs JSON réelles, pas comme une chaîne codée en JSON                                                                  |
| `resumeFromRunId` | `string`  | ID d'exécution d'une invocation `Workflow` antérieure à reprendre. Les appels `agent()` complétés avec des entrées inchangées retournent généralement les résultats en cache ; le reste s'exécute en direct. [Reprendre après une pause](/docs/fr/workflows#resume-after-a-pause) couvre les appels complétés qui se réexécutent. Même session uniquement |
| `title`           | `string`  | Ignoré ; le bloc `meta` du script définit le titre                                                                                                                                                                                                                                                                                                   |
| `description`     | `string`  | Ignoré ; le bloc `meta` du script définit la description                                                                                                                                                                                                                                                                                             |

<h3 id="todowrite">
  TodoWrite
</h3>

**Nom de l'outil :** `TodoWrite`

```typescript theme={null}
type TodoWriteInput = {
  todos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

Crée et gère une liste de tâches structurée pour suivre la progression.

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  Voir [Disponibilité du modèle](/docs/fr/agent-sdk/todo-tracking#model-availability) pour vous inscrire.
</Note>

<h3 id="taskcreate">
  TaskCreate
</h3>

**Nom de l'outil :** `TaskCreate`

```typescript theme={null}
type TaskCreateInput = {
  subject: string;
  description: string;
  activeForm?: string;
  metadata?: Record<string, unknown>;
};
```

Crée une seule tâche et retourne son ID assigné.

<h3 id="taskupdate">
  TaskUpdate
</h3>

**Nom de l'outil :** `TaskUpdate`

```typescript theme={null}
type TaskUpdateInput = {
  taskId: string;
  status?: "pending" | "in_progress" | "completed" | "deleted";
  subject?: string;
  description?: string;
  activeForm?: string;
  addBlocks?: string[];
  addBlockedBy?: string[];
  owner?: string;
  metadata?: Record<string, unknown>;
};
```

Corrige une tâche par ID. Définissez `status` à `"deleted"` pour la supprimer.

<h3 id="taskget">
  TaskGet
</h3>

**Nom de l'outil :** `TaskGet`

```typescript theme={null}
type TaskGetInput = {
  taskId: string;
};
```

Retourne les détails complets d'une tâche, ou `null` lorsque l'ID n'est pas trouvé.

<h3 id="tasklist">
  TaskList
</h3>

**Nom de l'outil :** `TaskList`

```typescript theme={null}
type TaskListInput = {};
```

Retourne un instantané de toutes les tâches dans la liste actuelle.

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Nom de l'outil :** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeInput = {
  /** Déprécié : n'est plus utilisé. */
  allowedPrompts?: Array<{
    tool: "Bash";
    prompt: string;
  }>;
  [k: string]: unknown;
};
```

Quitte le mode de planification. Le champ `allowedPrompts` est déprécié et ignoré ; Claude Code l'accepte toujours pour que les appelants existants et les transcriptions se valident. Avant v2.1.205, il demandait des permissions Bash basées sur les invites pour implémenter le plan.

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Nom de l'outil :** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesInput = {
  server?: string;
};
```

Répertorie les ressources MCP disponibles à partir des serveurs connectés.

<h3 id="readmcpresource">
  ReadMcpResource
</h3>

**Nom de l'outil :** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceInput = {
  server: string;
  uri: string;
};
```

Lit une ressource MCP spécifique à partir d'un serveur.

<h3 id="enterworktree">
  EnterWorktree
</h3>

**Nom de l'outil :** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeInput = {
  name?: string;
  path?: string;
};
```

Crée et entre dans un worktree git temporaire pour un travail isolé. Passez `path` pour basculer dans un worktree existant au lieu d'en créer un nouveau. À la première entrée, la cible doit être un worktree enregistré du référentiel actuel ou, dans un espace de travail multi-référentiel, d'un référentiel imbriqué à l'intérieur ; depuis une session worktree, elle doit être sous `.claude/worktrees/` du référentiel de la session. `name` et `path` s'excluent mutuellement.

<h3 id="exitworktree">
  ExitWorktree
</h3>

**Nom de l'outil :** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeInput = {
  action: "keep" | "remove";
  discard_changes?: boolean;
};
```

Quitte le worktree git actuel et retourne au répertoire de travail d'origine. L'action `keep` laisse le worktree et la branche sur le disque, tandis que `remove` supprime les deux. `discard_changes` doit être `true` lors de la suppression d'un worktree qui a des fichiers non validés ou des commits non fusionnés.

<h3 id="enterplanmode">
  EnterPlanMode
</h3>

**Nom de l'outil :** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeInput = {};
```

Entre en mode de planification, où Claude recherche et présente un plan avant de faire des modifications.

<h3 id="croncreate">
  CronCreate
</h3>

**Nom de l'outil :** `CronCreate`

```typescript theme={null}
type CronCreateInput = {
  cron: string;
  prompt: string;
  recurring?: boolean;
  durable?: boolean;
};
```

Planifie une invite pour s'exécuter selon un calendrier cron à 5 champs en heure locale. Définissez `recurring` à `false` pour se déclencher une seule fois à la prochaine correspondance. Les tâches sont limitées à la session par défaut : démarrer une nouvelle conversation les efface, et reprendre avec `--resume` ou `--continue` restaure les tâches qui n'ont pas expiré. Voir [Tâches planifiées](/docs/fr/scheduled-tasks).

Définir `durable` à `true` demande la persistance vers `.claude/scheduled_tasks.json` pour que la tâche survive aux redémarrages. La planification durable n'est pas disponible dans chaque session : lorsqu'elle ne l'est pas, Claude Code accepte `durable: true` mais crée la tâche en session uniquement. Lisez le champ `durable` de la sortie pour voir si la tâche a persisté.

<h3 id="crondelete">
  CronDelete
</h3>

**Nom de l'outil :** `CronDelete`

```typescript theme={null}
type CronDeleteInput = {
  id: string;
};
```

Supprime une tâche cron planifiée par l'ID retourné par `CronCreate`.

<h3 id="cronlist">
  CronList
</h3>

**Nom de l'outil :** `CronList`

```typescript theme={null}
type CronListInput = {};
```

Répertorie les tâches cron planifiées : les tâches durables de `.claude/scheduled_tasks.json` et les tâches en session uniquement de la session actuelle.

<h3 id="schedulewakeup">
  ScheduleWakeup
</h3>

**Nom de l'outil :** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupInput = {
  delaySeconds?: number;
  reason?: string;
  prompt?: string;
  noop?: boolean;
  stop?: boolean;
};
```

Planifie un réveil unique qui déclenche l'invite donnée après un délai. Cet outil soutient la commande `/loop` à rythme personnel. Le runtime limite `delaySeconds` entre 60 et 3600 secondes. Les champs `delaySeconds`, `reason`, `prompt` et `noop` sont requis sauf si `stop` est true. `noop: true` signale un réveil où rien n'a changé. Définir `stop: true` annule le réveil en attente et termine le `/loop` à rythme personnel. Le champ `stop` nécessite Claude Code v2.1.202 ou version ultérieure. Voir la [ligne ScheduleWakeup dans la référence des outils](/docs/fr/tools-reference).

<h3 id="remotetrigger">
  RemoteTrigger
</h3>

**Nom de l'outil :** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerInput = {
  action:
    | "list"
    | "get"
    | "create"
    | "update"
    | "run"
    | "create_webhook_trigger"
    | "list_runs"
    | "get_run_log";
  trigger_id?: string;
  session_id?: string;
  cursor?: string;
  body?: {
    [k: string]: unknown;
  };
};
```

Gère les [Routines](/docs/fr/routines), les exécutions Claude Code planifiées et déclenchées hébergées dans le cloud. Cet outil soutient la commande `/schedule`. `trigger_id` est requis pour les actions `get`, `update`, `run` et `list_runs`. `body` est requis pour `create`, `update` et `create_webhook_trigger`, et optionnel pour `run`.

`create_webhook_trigger` attache une source d'événement à une routine existante, telle qu'un [événement GitHub](/docs/fr/routines#add-a-github-trigger) qui la déclenche. Le `body` nomme la source, les événements et la routine à déclencher. Nécessite Claude Code v2.1.225 ou version ultérieure.

`list_runs` répertorie les exécutions récentes d'une routine, et `get_run_log` lit le journal d'une exécution. `session_id` nomme l'exécution à lire, à partir d'un résultat `list_runs`, et `cursor` pagine à travers l'une ou l'autre action. Les deux actions nécessitent Claude Code v2.1.227 ou version ultérieure.

Cet outil n'est disponible que lorsque la session est authentifiée avec un compte claude.ai sur un plan avec Routines activées, et est absent lorsque la politique de votre organisation désactive [Claude Code sur le web](/docs/fr/claude-code-on-the-web). Sur Claude Code v2.1.227 ou version ultérieure, l'outil est également absent lorsqu'un propriétaire a [désactivé les routines pour l'organisation](/docs/fr/routines#routines-are-disabled-by-your-organizations-policy). Avant v2.1.227, une session avec seulement le basculement des routines désactivé affichait toujours l'outil, et le serveur refusait ses appels.

<h3 id="pushnotification">
  PushNotification
</h3>

**Nom de l'outil :** `PushNotification`

```typescript theme={null}
type PushNotificationInput = {
  message: string;
  status: "proactive";
};
```

Envoie une notification push proactive à l'utilisateur. Gardez `message` sous 200 caractères car les systèmes d'exploitation mobiles tronquent le texte plus long. Voir la [ligne PushNotification dans la référence des outils](/docs/fr/tools-reference) pour la disponibilité du fournisseur ; la livraison push s'effectue via l'infrastructure hébergée par Anthropic qui n'est pas accessible depuis Amazon Bedrock, Claude Platform sur AWS, Google Cloud's Agent Platform ou Microsoft Foundry.

<h3 id="repl">
  REPL
</h3>

Supprimé dans v2.1.275. Jusqu'à v2.1.274, un outil `REPL` expérimental pouvait être activé avec `CLAUDE_CODE_REPL=1` dans l'option [`env`](#options).

<h3 id="reportfindings">
  ReportFindings
</h3>

**Nom de l'outil :** `ReportFindings`

```typescript theme={null}
type ReportFindingsInput = {
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

Signale les résultats de révision de code sous forme de liste structurée pour que Claude Code puisse les rendre au lieu de les imprimer en tant que texte. `level` est le niveau d'effort auquel la révision s'est exécutée. Les résultats sont ordonnés du plus grave au moins grave, avec au maximum 32 par appel, et le tableau est vide lorsqu'aucun n'a survécu. Nécessite Claude Code v2.1.196 ou version ultérieure.

Chaque résultat porte ces champs :

* `file` : chemin relatif au référentiel où se trouve le résultat. Le `line` optionnel est la ligne indexée à 1 à laquelle il s'ancre.
* `summary` : déclaration d'une phrase du défaut. `failure_scenario` décrit les entrées concrètes et l'état qui mènent à la sortie incorrecte ou au crash.
* `short_summary` : étiquette compressée optionnelle d'au maximum 60 caractères pour l'affichage compact. Nécessite Claude Code v2.1.212 ou version ultérieure.
* `category` : slug kebab-case court optionnel du type de résultat, tel que `correctness` ou `test-coverage`. Nécessite Claude Code v2.1.199 ou version ultérieure.
* `verdict` : défini lorsqu'une passe de vérification s'est exécutée ; absent sur les révisions en ligne uniquement.
* `outcome` : défini uniquement lors de la signalisation après l'application des correctifs.

<h3 id="artifact">
  Artifact
</h3>

**Nom de l'outil :** `Artifact`

```typescript theme={null}
type ArtifactInput = {
  action?: "publish" | "list";
  file_path?: string;
  favicon?: string;
  icon?: string;
  limit?: number;
  scope?: "mine" | "shared" | "all";
  title?: string;
  description?: string;
  label?: string;
  url?: string;
  force?: boolean;
  capabilities?: Record<string, unknown>;
  contract?: "latest" | string;
};
```

Publie un fichier `.html` ou `.md` local en tant que page d'artefact hébergée, ou répertorie les artefacts publiés de l'utilisateur. Omettez `action` ou passez `"publish"` pour publier `file_path`, qui est requis pour l'action de publication. Chaque champ ci-dessous s'applique à une publication :

* `icon` : un mot générique court pour l'icône de l'onglet du navigateur de l'artefact, tel que `chart` ou `map`. Claude l'inclut à la première publication et l'omet à une mise à jour, ce qui conserve l'icône stockée de l'artefact.
* `favicon` : déprécié, et Claude l'omet.
* `title` : nomme la page publiée dans l'onglet du navigateur et la galerie lorsque le fichier HTML n'a pas de balise `<title>`.
* `url` : cible un artefact existant à mettre à jour sur place au lieu de créer un nouveau.

`force` est un dernier recours qui écrase une version plus récente qu'une autre session a publiée. En cas de conflit, la publication échouée retourne le contenu plus récent ; Claude fusionne ses modifications sur ce contenu, ou relit l'artefact, et publie à nouveau. Passez `force` uniquement lorsque l'utilisateur demande explicitement de rejeter cette version.

Passez `"list"` pour énumérer les artefacts publiés de l'utilisateur ; seuls `limit` et `scope` peuvent l'accompagner. `scope` par défaut à `"mine"`, qui répertorie les artefacts que l'utilisateur possède ; `"shared"` répertorie les artefacts que d'autres personnes ont partagés avec l'utilisateur, et `"all"` répertorie les deux.

* `capabilities` : les capacités d'exécution que la page publiée utilise, indexées par nom de capacité, telles que les [connecteurs que la page peut appeler](/docs/fr/artifacts#pull-live-data-with-mcp-connectors). Le service d'artefact valide la déclaration et rejette une publication qui nomme une capacité que le compte ne peut pas utiliser ou en donne une configuration invalide. Passez `{}` pour effacer une déclaration stockée, et omettez le champ lors d'un redéploiement pour la conserver. Nécessite Agent SDK v0.3.235 ou version ultérieure.
* `contract` : la version d'exécution contre laquelle la page publiée s'exécute. Omettez-la pour conserver la version actuelle de l'artefact, passez `"latest"` pour mettre à niveau, ou passez une version spécifique pour épingler ou revenir en arrière. Nécessite Agent SDK v0.3.235 ou version ultérieure.

Les types sont exportés, mais l'outil est désactivé par défaut dans les sessions Agent SDK. La publication nécessite également chaque condition du [tableau de disponibilité des artefacts](/docs/fr/artifacts#availability), que les sessions authentifiées avec une clé API ne satisfont pas.

<h3 id="projects">
  Projects
</h3>

**Nom de l'outil :** `Projects`

```typescript theme={null}
type ProjectsInput = {
  method:
    | "project_info"
    | "project_read"
    | "project_search"
    | "project_write"
    | "project_delete";
  path?: string;
  content?: string;
  local_path?: string;
  present_to_user?: boolean;
  query?: string;
  n?: number;
};
```

Lit et écrit le Project claude.ai attaché à la session. Distribue sur `method` :

* `project_info` : retourne les métadonnées du projet et la liste des documents.
* `project_read` : lit un document par `path`.
* `project_search` : interroge la base de connaissances du projet avec `query`. `n` limite les résultats et par défaut à 5.
* `project_write` : crée ou remplace un document à `path` à partir d'exactement l'un de `content`, qui porte du texte en ligne, ou `local_path`, qui nomme un fichier dans le répertoire de travail. `present_to_user: true` marque le document écrit comme le livrable que l'utilisateur doit voir.
* `project_delete` : supprime un document par `path`.

<h3 id="readmcpresourcedir">
  ReadMcpResourceDir
</h3>

**Nom de l'outil :** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirInput = {
  server: string;
  uri: string;
};
```

Répertorie les enfants directs d'une ressource de répertoire sur un serveur MCP. Utilisable uniquement contre un serveur qui a déclaré le support de la liste des répertoires ; la liste n'est pas récursive. La liste des répertoires n'est pas activée dans chaque session : lorsqu'elle est désactivée, l'appel retourne une liste `resources` vide et le champ `error` signale que la liste des répertoires n'est pas activée.

<h3 id="refreshmcptools">
  RefreshMcpTools
</h3>

**Nom de l'outil :** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsInput = {
  server?: string; // refresh only this server; omit to refresh all connected servers
};
```

Réinterroge la liste des outils des serveurs MCP connectés et applique les modifications. Les types sont exportés, mais Claude Code enregistre l'outil uniquement lorsque vous définissez `CLAUDE_CODE_ENABLE_REFRESH_MCP_TOOLS=1` dans l'option [`env`](#options), et uniquement dans les sessions avec au moins un serveur MCP. Nécessite Claude Code v2.1.211 ou version ultérieure.

<h3 id="showonboardingrolepicker">
  ShowOnboardingRolePicker
</h3>

**Nom de l'outil :** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerInput = {};
```

Rend une ligne de puces de sélecteur de rôle cliquable lors de l'intégration Cowork pour que l'utilisateur puisse choisir son rôle et obtenir un plugin correspondant installé. Ne prend aucun argument ; la liste des rôles est définie par le client. L'appel se bloque jusqu'à ce que l'utilisateur réponde.

<h3 id="mcpinput">
  McpInput
</h3>

**Nom de l'outil :** noms d'outils MCP dynamiques de la forme `mcp__<server>__<tool>`

```typescript theme={null}
type McpInput = {
  [k: string]: unknown;
};
```

Les arguments des outils MCP sont un objet ouvert : chaque serveur définit ses propres paramètres, donc le type ne place aucune contrainte sur les noms de champs ou les valeurs. Consultez le schéma d'outil du serveur pour les champs qu'un outil spécifique accepte.

<h2 id="tool-output-types">
  Types de sortie d'outil
</h2>

Documentation des schémas de sortie pour tous les outils Claude Code intégrés. Ces types sont exportés depuis `@anthropic-ai/claude-agent-sdk` et représentent les données de réponse réelles retournées par chaque outil.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

Union de types de sortie d'outil exportés depuis `@anthropic-ai/claude-agent-sdk` ; les membres incluent :

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | ArtifactOutput
  | AskUserQuestionOutput
  | BashOutput
  | CronCreateOutput
  | CronDeleteOutput
  | CronListOutput
  | EnterPlanModeOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | ExitWorktreeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | McpOutput
  | MonitorOutput
  | NotebookEditOutput
  | ProjectsOutput
  | PushNotificationOutput
  | ReadMcpResourceDirOutput
  | ReadMcpResourceOutput
  | RefreshMcpToolsOutput
  | RemoteTriggerOutput
  | ReportFindingsOutput
  | ScheduleWakeupOutput
  | ShowOnboardingRolePickerOutput
  | TaskCreateOutput
  | TaskGetOutput
  | TaskListOutput
  | TaskStopOutput
  | TaskUpdateOutput
  | TodoWriteOutput
  | WebFetchOutput
  | WebSearchOutput
  | WorkflowOutput;
```

<h3 id="agent-2">
  Agent
</h3>

**Nom de l'outil :** `Agent`. Le nom précédent `Task` est toujours accepté comme alias, et le tableau `tools` dans le message d'initialisation [`SDKSystemMessage`](#sdksystemmessage) liste actuellement cet outil comme `Task` pour la compatibilité rétroactive.

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
      modelsUsed?: string[];
      totalToolUseCount: number;
      totalDurationMs: number;
      totalTokens: number;
      usage: {
        input_tokens: number;
        output_tokens: number;
        cache_creation_input_tokens: number | null;
        cache_read_input_tokens: number | null;
        server_tool_use: {
          web_search_requests: number;
          web_fetch_requests: number;
        } | null;
        service_tier: string | null;
        cache_creation: {
          ephemeral_1h_input_tokens: number;
          ephemeral_5m_input_tokens: number;
        } | null;
        inference_geo?: string | null;
        speed?: string | null;
        iterations?: unknown;
        output_tokens_details?: {
          thinking_tokens?: number | null;
        } | null;
      };
      toolStats?: {
        readCount: number;
        searchCount: number;
        bashCount: number;
        editFileCount: number;
        linesAdded: number;
        linesRemoved: number;
        otherToolCount: number;
        frameCount?: number;
      };
      prompt: string;
      worktreePath?: string;
      worktreeBranch?: string;
    }
  | {
      status: "async_launched";
      isAsync?: true;
      agentId: string;
      description: string;
      resolvedModel?: string;
      modelsUsed?: string[];
      prompt: string;
      outputFile: string;
      canReadOutputFile?: boolean;
    }
  | {
      status: "remote_launched";
      taskId: string;
      sessionUrl: string;
      description: string;
      prompt: string;
      outputFile: string;
    };
```

Retourne le résultat du sous-agent. Discriminé sur le champ `status` : `"completed"` pour les tâches terminées, `"async_launched"` pour les tâches de fond, et `"remote_launched"` pour les tâches que Claude Code a envoyées à une session cloud distante, où `sessionUrl` renvoie à cette session et `taskId` l'identifie.

Sur la variante `completed`, `resolvedModel` nomme le modèle sur lequel le sous-agent a démarré, qui peut différer du modèle demandé en entrée `model` lorsque [`availableModels`](/docs/fr/model-config#restrict-model-selection) ou une autre substitution s'applique. Ce champ nécessite Claude Code v2.1.174 ou ultérieur. Sur `async_launched`, il nomme le modèle en cours d'utilisation lorsque la tâche est passée en arrière-plan.

`modelsUsed` énumère les modèles utilisés par le sous-agent, dans l'ordre. Le champ est présent uniquement lorsqu'un changement de modèle en cours d'exécution s'est produit, et un modèle apparaît à nouveau lorsque l'exécution a basculé vers lui. Sur `async_launched`, la liste couvre les modèles utilisés avant la mise en arrière-plan. À la fois `modelsUsed` et le comportement de mise en arrière-plan de `resolvedModel` nécessitent Claude Code v2.1.212 ou ultérieur.

Si Claude Code [a conservé le worktree isolé du sous-agent](/docs/fr/worktrees#isolate-subagents-with-worktrees), `worktreePath` sur le résultat `completed` est l'endroit où le trouver. `worktreeBranch` est sa branche, présente lorsque Claude Code a créé le worktree avec git.

Claude Code remplit `usage` et `totalTokens` à partir de la dernière demande API du sous-agent, pas de l'ensemble de l'exécution, donc `usage.service_tier` est la chaîne de niveau de service que l'API a signalée sur cette demande. Lorsqu'il est présent, `usage.output_tokens_details.thinking_tokens` est le nombre de jetons de sortie de cette demande qui étaient des jetons de réflexion. Le champ `output_tokens_details` nécessite TypeScript SDK v0.3.228 ou ultérieur, qui regroupe Claude Code v2.1.228.

`usage.output_tokens_details` correspond à [`Usage.output_tokens_details`](#usage) en signification, limité à cette dernière demande, mais chaque niveau est optionnel ici. Protégez à la fois l'objet et le champ, par exemple `usage.output_tokens_details?.thinking_tokens ?? 0`, plutôt que de le lire directement.

Avant v2.1.207, le type publié était plus étroit. Il omettait `worktreePath`, `worktreeBranch`, `citations`, `toolStats.frameCount`, et les champs d'utilisation `inference_geo`, `speed` et `iterations`, et il typait `service_tier` comme `"standard" | "priority" | "batch"`. Les champs que le type marque comme optionnels peuvent être absents sur les résultats enregistrés par les versions antérieures.

<h3 id="askuserquestion-2">
  AskUserQuestion
</h3>

**Nom de l'outil :** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionOutput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
  answers: Record<string, string>;
  response?: string;
  annotations?: Record<string, { preview?: string; notes?: string }>;
  afkTimeoutMs?: number;
};
```

Retourne les questions posées et les réponses de l'utilisateur. `response` est défini lorsque l'utilisateur a tapé une réponse libre au lieu de répondre aux questions structurées ; lorsqu'il est présent, Claude reçoit « L'utilisateur a répondu : … » au lieu de la liste de réponses par question.

<h3 id="bash-2">
  Bash
</h3>

**Nom de l'outil :** `Bash`

```typescript theme={null}
type BashOutput = {
  stdout: string;
  stderr: string;
  rawOutputPath?: string;
  interrupted: boolean;
  isImage?: boolean;
  backgroundTaskId?: string;
  backgroundedByUser?: boolean;
  timedOutAfterMs?: number;
  backgroundCwdHint?: string;
  backgroundEndsWithFinalResponse?: true;
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  noOutputExpected?: boolean;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
  staleReadFileStateHint?: string;
  ghRateLimitHint?: string;
  gitOperation?: {
    commit?: { sha: string; kind: "committed" | "amended" | "cherry-picked"; branch?: string };
    push?: { branch: string };
    branch?: { ref: string; action: "merged" | "rebased" };
    pr?: {
      number: number;
      url?: string;
      action: "created" | "edited" | "merged" | "commented" | "closed" | "reopened" | "ready" | "draft" | "auto-merge-enabled" | "auto-merge-disabled";
    };
  };
};
```

Les champs `stdout`, `stderr` et `backgroundTaskId` portent :

| Champ              | Ce qu'il porte                                                                                                                            |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `stdout`           | La sortie standard et la sortie d'erreur de la commande, fusionnées en un seul flux entrelacé                                             |
| `stderr`           | Les avis que l'outil lui-même ajoute, comme une réinitialisation du répertoire de travail du shell, pas la sortie d'erreur de la commande |
| `backgroundTaskId` | Présent pour les commandes de fond                                                                                                        |

`timedOutAfterMs` est le délai d'expiration en millisecondes, défini lorsque la commande a atteint son délai d'expiration et s'est déplacée en arrière-plan plutôt que de démarrer explicitement. `backgroundCwdHint` est défini lorsque la commande mise en arrière-plan contenait une fonction intégrée de changement de répertoire telle que `cd`, `pushd`, `popd` ou `chdir`, et note que le répertoire de travail de la session n'a pas changé. Les deux champs nécessitent Claude Code v2.1.210 ou ultérieur.

Lorsqu'un sous-agent s'exécutant au premier plan possède une commande mise en arrière-plan, Claude Code termine la commande lorsque ce sous-agent donne sa réponse finale. Claude Code définit `backgroundEndsWithFinalResponse` à `true` sur de telles commandes, et omet le champ lorsque la commande survit au tour, comme les commandes démarrées par la conversation principale ou par les sous-agents de fond. Le champ nécessite Claude Code v2.1.227 ou ultérieur.

Claude Code définit `gitOperation.commit.branch` à la branche nommée dans la ligne de résumé du commit git, et l'omet pour un commit effectué sur une HEAD détachée. Le champ nécessite Agent SDK v0.3.227 ou ultérieur. Claude Code signale une commande `gh pr reopen` comme l'action PR `reopened`, ce qui nécessite Agent SDK v0.3.234 ou ultérieur.

<h3 id="monitor-2">
  Monitor
</h3>

**Nom de l'outil :** `Monitor`

```typescript theme={null}
type MonitorOutput = {
  taskId: string;
  timeoutMs: number;
  persistent?: boolean;
};
```

Retourne l'ID de tâche de fond pour le moniteur en cours d'exécution. Utilisez cet ID avec `TaskStop` pour annuler la surveillance plus tôt.

<h3 id="edit-2">
  Edit
</h3>

**Nom de l'outil :** `Edit`

```typescript theme={null}
type FileEditOutput = {
  filePath: string;
  oldString: string;
  newString: string;
  originalFile: string | null;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  userModified: boolean;
  replaceAll: boolean;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
    repository?: string | null;
  };
};
```

Retourne le diff structuré de l'opération d'édition.

<h3 id="read-2">
  Read
</h3>

**Nom de l'outil :** `Read`

```typescript theme={null}
type FileReadOutput =
  | {
      type: "text";
      file: {
        filePath: string;
        content: string;
        numLines: number;
        startLine: number;
        totalLines: number;
        /** True when a whole-file read was auto-paginated because it exceeded the token cap (the content is a partial first page). */
        truncatedByTokenCap?: boolean;
      };
    }
  | {
      type: "image";
      file: {
        base64: string;
        type: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        originalSize: number;
        dimensions?: {
          originalWidth?: number;
          originalHeight?: number;
          displayWidth?: number;
          displayHeight?: number;
        };
      };
    }
  | {
      type: "notebook";
      file: {
        filePath: string;
        cells: unknown[];
      };
    }
  | {
      type: "pdf";
      file: {
        filePath: string;
        base64: string;
        originalSize: number;
      };
    }
  | {
      type: "parts";
      file: {
        filePath: string;
        originalSize: number;
        count: number;
        outputDir: string;
      };
      /** Document page number of the first extracted page; labels the page images in the tool_result content. */
      firstPage?: number;
      /** In-process only: the page-image bytes are delivered as image blocks in the tool_result content and aren't retained on the emitted tool_use_result, so this key is absent there. */
      pages?: {
        base64: string;
        mediaType: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        error?: string;
      }[];
    }
  | {
      type: "file_unchanged";
      file: {
        filePath: string;
      };
      /** Set when the dedup matched a startup-seeded entry (CLAUDE.md / nested memory) rather than a prior Read tool_result. */
      source?: "seeded";
    };
```

Retourne le contenu du fichier dans un format approprié au type de fichier. Discriminé sur le champ `type`.

<h3 id="write-2">
  Write
</h3>

**Nom de l'outil :** `Write`

```typescript theme={null}
type FileWriteOutput = {
  type: "create" | "update";
  filePath: string;
  content: string;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  originalFile: string | null;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
    repository?: string | null;
  };
  userModified?: boolean;
};
```

Retourne le résultat d'écriture avec les informations de diff structuré. Ce que `originalFile` et `structuredPatch` contiennent dépend de l'écriture :

* Pour un fichier nouvellement créé, `originalFile` est null et `structuredPatch` est vide
* Sur une réécriture, `originalFile` porte le contenu précédent, sauf lorsque ce contenu est plus grand qu'environ 10 Mo : Claude Code ignore alors le diff et retourne `originalFile` null et `structuredPatch` vide
* `structuredPatch` est également vide lorsque l'écriture n'a rien changé ou que le diff a expiré

<h3 id="glob-2">
  Glob
</h3>

**Nom de l'outil :** `Glob`

```typescript theme={null}
type GlobOutput = {
  durationMs: number;
  numFiles: number;
  filenames: string[];
  truncated: boolean;
  totalMatches?: number;
  countIsComplete?: boolean;
};
```

Retourne les chemins de fichiers correspondant au motif glob, triés par heure de modification.

`totalMatches` et `countIsComplete` nécessitent Claude Code v2.1.191 ou ultérieur. `totalMatches` signale le nombre de fichiers correspondants avant la troncature. Lorsque `countIsComplete` est false, `totalMatches` est une limite inférieure car la recherche sous-jacente a tronqué sa propre sortie.

<h3 id="grep-2">
  Grep
</h3>

**Nom de l'outil :** `Grep`

```typescript theme={null}
type GrepOutput = {
  mode?: "content" | "files_with_matches" | "count";
  numFiles: number;
  filenames: string[];
  content?: string;
  numLines?: number;
  numMatches?: number;
  totalFiles?: number;
  totalLines?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

Retourne les résultats de recherche. La forme varie selon `mode` : liste de fichiers, contenu avec correspondances ou comptages de correspondances. En mode `count`, `numFiles` et `numMatches` sont des totaux sur l'ensemble des résultats, pas la tranche paginée. Avant v2.1.208, une `head_limit` ou un `offset` qui tronquait les entrées énumérées tronquait également ces totaux.

`totalFiles` nécessite Claude Code v2.1.208 ou ultérieur et signale le nombre total de résultats avant la pagination `head_limit` et `offset` en mode `files_with_matches`. `totalLines` nécessite Claude Code v2.1.210 ou ultérieur et signale le nombre total de lignes avant la pagination en mode `content`.

<h3 id="taskstop-2">
  TaskStop
</h3>

**Nom de l'outil :** `TaskStop`

```typescript theme={null}
type TaskStopOutput = {
  message: string;
  task_id: string;
  task_type: string;
  command?: string;
};
```

Retourne la confirmation après l'arrêt de la tâche de fond.

<h3 id="notebookedit-2">
  NotebookEdit
</h3>

**Nom de l'outil :** `NotebookEdit`

```typescript theme={null}
type NotebookEditOutput = {
  new_source: string;
  old_source?: string;
  cell_id?: string;
  cell_type: "code" | "markdown";
  language: string;
  edit_mode: string;
  error?: string;
  notebook_path: string;
  original_file: string;
  updated_file: string;
};
```

Retourne le résultat de l'édition du carnet avec le contenu du fichier original et mis à jour.

<h3 id="webfetch-2">
  WebFetch
</h3>

**Nom de l'outil :** `WebFetch`

```typescript theme={null}
type WebFetchOutput = {
  bytes: number;
  code: number;
  codeText: string;
  result: string;
  durationMs: number;
  url: string;
  artifactRead?: {
    slug: string;
    ver?: string;
    seeded?: false;
  };
};
```

Retourne le contenu récupéré avec le statut HTTP et les métadonnées.

`artifactRead` est l'enregistrement propre de Claude Code d'une lecture d'artefact, présent uniquement lorsque Claude a récupéré un artefact que la session peut publier. Claude Code le relit lorsqu'une session reprend afin qu'une publication ultérieure s'appuie sur la bonne version ; votre code n'a pas besoin d'agir dessus. `slug` nomme l'artefact, `ver` est la version que la lecture a enregistrée et est absent lorsqu'elle n'en a enregistré aucune, et `seeded: false` marque une lecture dont la source complète n'a pas atteint Claude. Le champ `seeded` nécessite Agent SDK v0.3.239 ou ultérieur.

<h3 id="websearch-2">
  WebSearch
</h3>

**Nom de l'outil :** `WebSearch`

```typescript theme={null}
type WebSearchOutput = {
  query: string;
  results: Array<
    | {
        tool_use_id: string;
        content: Array<{ title: string; url: string }>;
      }
    | string
  >;
  durationSeconds: number;
  searchCount?: number;
};
```

Retourne les résultats de recherche du web.

<h3 id="workflow-2">
  Workflow
</h3>

**Nom de l'outil :** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched" | "remote_launched";
  taskId: string;
  taskType?: "local_workflow" | "remote_agent";
  workflowName?: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  sessionUrl?: string; // set when the workflow launched as a cloud session
  warning?: string;
  error?: string;
};
```

Retourne immédiatement après que l'outil accepte l'invocation. Le résultat final arrive plus tard en tant que complément de tâche. Vérifiez `error` avant de traiter l'exécution comme démarrée : un script qui échoue sa vérification de syntaxe retourne `status: "async_launched"` avec `error` défini, et ne s'exécute jamais.

| Champ           | Type                                    | Description                                                                                                                                                                                                            |
| --------------- | --------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`        | `"async_launched" \| "remote_launched"` | L'outil a accepté l'invocation. `"async_launched"` pour les exécutions en processus, `"remote_launched"` pour les exécutions envoyées à une session distante au lieu de s'exécuter en processus                        |
| `taskId`        | `string`                                | Identifiant de tâche de fond pour l'exécution                                                                                                                                                                          |
| `taskType`      | `"local_workflow" \| "remote_agent"`    | Type de tâche de la tâche de fond enregistrée, correspondant au bras `status`                                                                                                                                          |
| `workflowName`  | `string`                                | Le `meta.name` du script de workflow                                                                                                                                                                                   |
| `runId`         | `string`                                | Identifiant d'exécution de workflow à transmettre en tant que `resumeFromRunId` lors d'une invocation ultérieure. Absent pour les exécutions `remote_launched`, où l'URL de la session cloud est la poignée de reprise |
| `summary`       | `string`                                | Description d'une ligne de ce que fait le workflow                                                                                                                                                                     |
| `transcriptDir` | `string`                                | Répertoire où les transcriptions de sous-agent sont écrites pendant l'exécution                                                                                                                                        |
| `scriptPath`    | `string`                                | Chemin du script de workflow persisté pour cette exécution. Modifiez-le et transmettez-le en tant que `scriptPath` pour réexécuter sans renvoyer le script                                                             |
| `sessionUrl`    | `string`                                | URL de la session cloud, définie lorsque `status` est `"remote_launched"`                                                                                                                                              |
| `warning`       | `string`                                | Avertissement non bloquant, comme l'état git local divergeant de la branche poussée qu'une session cloud clonera                                                                                                       |
| `error`         | `string`                                | Défini lorsque le script échoue sa vérification de syntaxe. Lorsqu'il est présent, l'exécution n'a pas démarré malgré le statut lancé                                                                                  |

<h3 id="todowrite-2">
  TodoWrite
</h3>

**Nom de l'outil :** `TodoWrite`

```typescript theme={null}
type TodoWriteOutput = {
  oldTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
  newTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

Retourne les listes de tâches précédentes et mises à jour.

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  Consultez [Disponibilité du modèle](/docs/fr/agent-sdk/todo-tracking#model-availability) pour vous inscrire.
</Note>

<h3 id="taskcreate-2">
  TaskCreate
</h3>

**Nom de l'outil :** `TaskCreate`

```typescript theme={null}
type TaskCreateOutput = {
  task: {
    id: string;
    subject: string;
  };
};
```

Retourne la tâche créée avec son ID assigné.

<h3 id="taskupdate-2">
  TaskUpdate
</h3>

**Nom de l'outil :** `TaskUpdate`

```typescript theme={null}
type TaskUpdateOutput = {
  success: boolean;
  taskId: string;
  updatedFields: string[];
  error?: string;
  statusChange?: {
    from: string;
    to: string;
  };
};
```

Retourne le résultat de la mise à jour, y compris les champs qui ont changé.

<h3 id="taskget-2">
  TaskGet
</h3>

**Nom de l'outil :** `TaskGet`

```typescript theme={null}
type TaskGetOutput = {
  task: {
    id: string;
    subject: string;
    description: string;
    status: "pending" | "in_progress" | "completed";
    blocks: string[];
    blockedBy: string[];
  } | null;
};
```

Retourne l'enregistrement de tâche complet, ou `null` lorsque l'ID n'est pas trouvé.

<h3 id="tasklist-2">
  TaskList
</h3>

**Nom de l'outil :** `TaskList`

```typescript theme={null}
type TaskListOutput = {
  tasks: Array<{
    id: string;
    subject: string;
    status: "pending" | "in_progress" | "completed";
    owner?: string;
    blockedBy: string[];
  }>;
};
```

Retourne un instantané de toutes les tâches dans la liste actuelle.

<h3 id="exitplanmode-2">
  ExitPlanMode
</h3>

**Nom de l'outil :** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeOutput = {
  plan: string | null;
  isAgent: boolean;
  filePath?: string;
  hasTaskTool?: boolean;
  planWasEdited?: boolean;
  awaitingLeaderApproval?: boolean;
  requestId?: string;
};
```

Retourne l'état du plan après la sortie du mode de planification.

<h3 id="listmcpresources-2">
  ListMcpResources
</h3>

**Nom de l'outil :** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesOutput = Array<{
  uri: string;
  name: string;
  mimeType?: string;
  description?: string;
  server: string;
}>;
```

Retourne un tableau de ressources MCP disponibles.

<h3 id="readmcpresource-2">
  ReadMcpResource
</h3>

**Nom de l'outil :** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceOutput = {
  contents: Array<{
    uri: string;
    mimeType?: string;
    text?: string;
    blobSavedTo?: string;
  }>;
  error?: string;
};
```

Retourne le contenu de la ressource MCP demandée.

<h3 id="enterworktree-2">
  EnterWorktree
</h3>

**Nom de l'outil :** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeOutput = {
  worktreePath: string;
  worktreeBranch?: string;
  message: string;
};
```

Retourne les informations sur le worktree git.

<h3 id="exitworktree-2">
  ExitWorktree
</h3>

**Nom de l'outil :** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeOutput = {
  action: "keep" | "remove";
  originalCwd: string;
  worktreePath: string;
  worktreeBranch?: string;
  tmuxSessionName?: string;
  discardedFiles?: number;
  discardedCommits?: number;
  message: string;
};
```

Retourne l'action entreprise et les détails sur le worktree qui a été quitté.

<h3 id="enterplanmode-2">
  EnterPlanMode
</h3>

**Nom de l'outil :** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeOutput = {
  message: string;
};
```

Retourne une confirmation que le mode de planification a été activé.

<h3 id="croncreate-2">
  CronCreate
</h3>

**Nom de l'outil :** `CronCreate`

```typescript theme={null}
type CronCreateOutput = {
  id: string;
  humanSchedule: string;
  recurring: boolean;
  durable?: boolean; // true when persisted to .claude/scheduled_tasks.json; false when session-only
};
```

Retourne l'ID du travail et une description lisible par l'homme de la planification.

<h3 id="crondelete-2">
  CronDelete
</h3>

**Nom de l'outil :** `CronDelete`

```typescript theme={null}
type CronDeleteOutput = {
  id: string;
};
```

Retourne l'ID du travail supprimé.

<h3 id="cronlist-2">
  CronList
</h3>

**Nom de l'outil :** `CronList`

```typescript theme={null}
type CronListOutput = {
  jobs: {
    id: string;
    cron: string;
    humanSchedule: string;
    prompt: string;
    recurring?: boolean;
    durable?: boolean;
  }[];
};
```

Retourne les travaux cron planifiés : les travaux durables de `.claude/scheduled_tasks.json` et les travaux de session uniquement de la session actuelle. Un travail de session uniquement porte `durable: false` ; les travaux lus à partir du disque omettent le champ.

<h3 id="schedulewakeup-2">
  ScheduleWakeup
</h3>

**Nom de l'outil :** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupOutput = {
  scheduledFor: number;
  clampedDelaySeconds: number;
  wasClamped: boolean;
  stopped?: boolean;
  cancelledWakeups?: number;
};
```

Retourne quand le réveil se déclenchera en tant qu'horodatage d'époque en millisecondes, le délai réellement utilisé, et si le délai demandé a été limité. Le champ `stopped` est `true` lorsque l'appel a terminé la boucle avec `stop: true`. Il nécessite Claude Code v2.1.202 ou ultérieur. Le champ `cancelledWakeups` compte combien de réveils en attente un appel `stop: true` a annulés. Une valeur de 0 signifie que rien n'était en attente, et un cron `/loop` récurrent n'est pas annulé par `stop: true`. Il nécessite Claude Code v2.1.206 ou ultérieur.

<h3 id="remotetrigger-2">
  RemoteTrigger
</h3>

**Nom de l'outil :** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerOutput = {
  status: number;
  json: string;
  summary?: string;
};
```

Retourne le statut de réponse API et le corps pour l'opération de déclenchement.

<h3 id="pushnotification-2">
  PushNotification
</h3>

**Nom de l'outil :** `PushNotification`

```typescript theme={null}
type PushNotificationOutput = {
  message: string;
  pushSent?: boolean;
  localSent?: boolean;
  disabledReason?: "config_off" | "user_present" | "no_transport";
  sentAt?: string;
};
```

Retourne les détails de livraison, y compris si une notification push ou locale a été envoyée et pourquoi la livraison a été ignorée.

<h3 id="reportfindings-2">
  ReportFindings
</h3>

**Nom de l'outil :** `ReportFindings`

```typescript theme={null}
type ReportFindingsOutput = {
  count: number;
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

Retourne le nombre de résultats signalés, le niveau d'effort auquel l'examen s'est exécuté, et les résultats renvoyés pour le corps du résultat. Nécessite Claude Code v2.1.196 ou ultérieur. Le champ `short_summary` renvoyé nécessite Claude Code v2.1.212 ou ultérieur.

<h3 id="artifact-2">
  Artifact
</h3>

**Nom de l'outil :** `Artifact`

```typescript theme={null}
type ArtifactOutput =
  | {
      url: string;
      path: string;
      title?: string;
      version?: string;
      capabilities?: unknown;
      stored?: {
        contract: string;
        capabilities?: Record<string, unknown>;
      };
      warnings?: string[];
      contract?: string;
      updated?: boolean;
      liveSubscription?: string;
    }
  | {
      artifacts: Array<{
        title: string;
        url: string;
        updatedAt?: string;
        rel?: "mine" | "shared";
      }>;
      truncated?: boolean;
      scope?: "shared" | "all";
    };
```

Retourne l'`url` de la page publiée et le `path` local qui a été publié pour l'action de publication, avec `updated` défini à true lorsque la publication a redéployé un artefact existant, et `warnings` portant tous les avis au moment de la publication. L'action de liste retourne les lignes `artifacts` à la place, avec `truncated` défini lorsque plus d'artefacts existent que la limite demandée. Sur les listes dont la portée n'est pas `"mine"`, chaque ligne porte `rel` marquant si l'utilisateur possède l'artefact ou s'il lui a été partagé, et la `scope` de la sortie enregistre quelle portée non définie par défaut a produit la liste ; les deux sont absents sur les listes par défaut.

<h3 id="projects-2">
  Projects
</h3>

**Nom de l'outil :** `Projects`

```typescript theme={null}
type ProjectsOutput =
  | {
      method: "project_info";
      notice?: string;
      name: string;
      description: string;
      instructions: string;
      docs: Array<{ path: string; created_at: string | null }>;
      files?: Array<{
        path: string;
        file_kind: string;
        created_at: string | null;
      }>;
      sync_sources?: Array<{
        type: string | null;
        config: Record<string, unknown>;
      }>;
      knowledge: {
        knowledge_size: number;
        max_knowledge_size: number;
      };
    }
  | {
      method: "project_read";
      notice?: string;
      path: string;
      file_kind?: string;
      content?: string;
      local_file?: string;
      created_at: string | null;
    }
  | {
      method: "project_search";
      notice?: string;
      rag: boolean;
      hits?: Array<{ name?: string; doc_uuid?: string; text?: string }>;
      docs?: string[];
    }
  | {
      method: "project_write";
      notice?: string;
      path: string;
      doc_uuid: string;
      replaced: boolean;
      present_to_user?: boolean;
      local_path?: string;
    }
  | {
      method: "project_delete";
      notice?: string;
      path: string;
      deleted: boolean;
    };
```

Discriminé sur le champ `method`, reflétant l'entrée. `project_read` retourne les petits documents texte en ligne dans `content` et écrit les documents plus volumineux dans un chemin `local_file` à la place ; `project_search` retourne les `hits` RAG avec `rag: true` lorsque l'index du projet est disponible et revient à une liste de chemin `docs` sinon.

<h3 id="readmcpresourcedir-2">
  ReadMcpResourceDir
</h3>

**Nom de l'outil :** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirOutput = {
  resources: Array<{
    uri: string;
    name: string;
    mimeType?: string;
  }>;
  error?: string;
};
```

Retourne les enfants directs de la ressource de répertoire. Les sous-répertoires apparaissent avec mimeType `"inode/directory"` ; `error` porte un message lisible par l'homme lorsque le serveur n'a pas pu énumérer le répertoire.

<h3 id="refreshmcptools-2">
  RefreshMcpTools
</h3>

**Nom de l'outil :** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsOutput = Array<{
  server: string;
  status: "refreshed" | "error" | "not_connected";
  toolCount?: number; // tools now available from this server
  added?: string[]; // tool names this refresh added
  removed?: string[]; // tool names this refresh removed
  error?: string; // why the refresh failed or the server was unavailable
}>;
```

Retourne une entrée par serveur : `refreshed` signifie que la liste d'outils réinterrogée a été appliquée, `error` signifie que la réinterrogation a échoué et l'ensemble d'outils précédent a été conservé, et `not_connected` signifie que le serveur n'a pas de connexion active pour interroger.

<h3 id="showonboardingrolepicker-2">
  ShowOnboardingRolePicker
</h3>

**Nom de l'outil :** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerOutput = {
  role?: string;
  dismissed?: boolean;
};
```

Retourne la sélection de l'utilisateur : `role` lorsqu'il a choisi une puce de rôle ou en a tapé une, et `dismissed: true` lorsqu'il a fermé le sélecteur. Un objet vide signifie que l'utilisateur a approuvé l'appel sans choisir de rôle.

<h3 id="mcpoutput">
  McpOutput
</h3>

**Nom de l'outil :** noms d'outils MCP dynamiques de la forme `mcp__<server>__<tool>`

```typescript theme={null}
type McpOutput =
  | string
  | {
      type: string;
      [k: string]: unknown;
    }[]
  | {
      [k: string]: unknown;
    };
```

Les résultats des outils MCP sont retournés sous forme de chaîne ou de tableau de blocs de contenu, selon le serveur. La branche d'objet simple finale du type exporté est un artefact de génération de schéma : le SDK ne retourne pas un objet nu, car la sortie structurée d'un serveur est sérialisée en chaîne JSON avant d'être retournée. À l'exécution, la valeur peut également être `undefined`, bien que le type exporté ne modélise pas cela.

<h2 id="permission-types">
  Types de permission
</h2>

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

Opérations pour mettre à jour les permissions.

```typescript theme={null}
type PermissionUpdate =
  | {
      type: "addRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "replaceRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "setMode";
      mode: PermissionMode;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "addDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    };
```

<h3 id="permissionbehavior">
  `PermissionBehavior`
</h3>

```typescript theme={null}
type PermissionBehavior = "allow" | "deny" | "ask";
```

<h3 id="permissionupdatedestination">
  `PermissionUpdateDestination`
</h3>

```typescript theme={null}
type PermissionUpdateDestination =
  | "userSettings" // Paramètres utilisateur globaux
  | "projectSettings" // Paramètres de projet par répertoire
  | "localSettings" // Paramètres de projet locaux
  | "session" // Session actuelle uniquement
  | "cliArg"; // Argument CLI
```

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

```typescript theme={null}
type PermissionRuleValue = {
  toolName: string;
  ruleContent?: string;
};
```

<h2 id="other-types">
  Autres types
</h2>

<h3 id="apikeysource">
  `ApiKeySource`
</h3>

D'où provient la clé API pour les requêtes de la session, signalée comme `apiKeySource` sur le message d'initialisation [`SDKSystemMessage`](#sdksystemmessage).

```typescript theme={null}
type ApiKeySource =
  | "ANTHROPIC_API_KEY"
  | "apiKeyHelper"
  | "/login managed key"
  | "none"
  | "user"
  | "project"
  | "org"
  | "temporary"
  | "oauth";
```

Claude Code signale l'une de quatre valeurs :

| Valeur               | Clé en utilisation                                                                                                                                 |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_API_KEY`  | La clé dans la variable d'environnement `ANTHROPIC_API_KEY`                                                                                        |
| `apiKeyHelper`       | La clé renvoyée par votre commande [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper)                                                           |
| `/login managed key` | La clé que Claude Code a stockée lorsque vous vous êtes connecté avec un [compte Claude Console](/docs/fr/authentication#claude-console-authentication) |
| `none`               | Aucune clé API. La session s'authentifie d'une autre manière, par exemple une connexion claude.ai, un jeton porteur ou un fournisseur cloud        |

Agent SDK v0.3.234 et versions ultérieures listent ces quatre valeurs dans le type. Le type conserve également `user`, `project`, `org`, `temporary` et `oauth` afin que le code plus ancien soit toujours compilé, et Claude Code ne les signale pas.

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Les fonctionnalités bêta disponibles qui peuvent être activées via l'option `betas`. Consultez [En-têtes bêta](https://platform.claude.com/docs/en/api/beta-headers) pour plus d'informations.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  La bêta `context-1m-2025-08-07` est retirée à partir du 30 avril 2026. Passer cette valeur avec Claude Sonnet 4.5 ou Sonnet 4 n'a aucun effet, et les requêtes qui dépassent la fenêtre de contexte standard de 200 k jetons renvoient une erreur. Pour utiliser une fenêtre de contexte de 1 M de jetons, migrez vers [Claude Opus 5.5, Claude Opus 5, Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 ou Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), qui incluent 1 M de contexte au prix standard sans en-tête bêta requis.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

Informations sur une commande disponible.

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
  builtin?: boolean;
};
```

`builtin` est `true` sur une ligne lorsque la commande est celle de Claude Code et que taper `/name` l'exécute. Elle est absente pour une commande définie par un utilisateur, un projet, un plugin ou un serveur MCP, et pour une commande groupée que l'un de ceux-ci [remplace par nom](/docs/fr/skills#resolve-skills-that-share-a-name). Nécessite Agent SDK v0.3.277 ou version ultérieure.

<h3 id="modelinfo">
  `ModelInfo`
</h3>

Informations sur un modèle disponible.

```typescript theme={null}
type ModelInfo = {
  value: string;
  resolvedModel?: string;
  displayName: string;
  description: string;
  supportsEffort?: boolean;
  supportedEffortLevels?: ("low" | "medium" | "high" | "xhigh" | "max")[];
  supportsAdaptiveThinking?: boolean;
  supportsFastMode?: boolean;
  supportsAutoMode?: boolean;
};
```

| Champ                      | Type                                                               | Description                                                                                                                                                                                                                                                                                                                                               |
| :------------------------- | :----------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `value`                    | `string`                                                           | Identifiant du modèle à passer dans les appels API                                                                                                                                                                                                                                                                                                        |
| `resolvedModel`            | `string \| undefined`                                              | ID du modèle canonique sur le fil auquel la `value` de cette entrée se résout. Une entrée d'alias telle que `sonnet` se résout en un ID de modèle explicite tel que `claude-sonnet-5`, afin qu'un hôte puisse faire correspondre un ID de modèle explicite stocké à l'entrée d'alias qui le couvre. Nécessite Claude Code v2.1.197 ou version ultérieure. |
| `displayName`              | `string`                                                           | Nom d'affichage lisible par l'homme                                                                                                                                                                                                                                                                                                                       |
| `description`              | `string`                                                           | Description des capacités du modèle                                                                                                                                                                                                                                                                                                                       |
| `supportsEffort`           | `boolean \| undefined`                                             | Si ce modèle supporte les niveaux d'effort                                                                                                                                                                                                                                                                                                                |
| `supportedEffortLevels`    | `("low" \| "medium" \| "high" \| "xhigh" \| "max")[] \| undefined` | Niveaux d'effort que ce modèle accepte                                                                                                                                                                                                                                                                                                                    |
| `supportsAdaptiveThinking` | `boolean \| undefined`                                             | Si ce modèle supporte la réflexion adaptative, où Claude décide quand et combien réfléchir                                                                                                                                                                                                                                                                |
| `supportsFastMode`         | `boolean \| undefined`                                             | Si ce modèle supporte le mode rapide                                                                                                                                                                                                                                                                                                                      |
| `supportsAutoMode`         | `boolean \| undefined`                                             | Si ce modèle supporte le mode auto                                                                                                                                                                                                                                                                                                                        |

<h3 id="agentinfo">
  `AgentInfo`
</h3>

Informations sur un sous-agent disponible qui peut être invoqué via l'outil Agent.

```typescript theme={null}
type AgentInfo = {
  name: string;
  description: string;
  model?: string;
};
```

| Champ         | Type                  | Description                                                                                                                                                                                                                           |
| :------------ | :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`        | `string`              | Identifiant du type d'agent (par exemple, `"Explore"`, `"general-purpose"`)                                                                                                                                                           |
| `description` | `string`              | Description de quand utiliser cet agent                                                                                                                                                                                               |
| `model`       | `string \| undefined` | Modèle que cet agent utilise : un alias ou un ID de modèle, ou `'inherit'` pour le modèle du parent. Lorsqu'il est `undefined`, Claude Code choisit le modèle dans l'[ordre des modèles de sous-agent](/docs/fr/sub-agents#choose-a-model) |

<h3 id="mcpserverprovenance">
  `McpServerProvenance`
</h3>

Le serveur MCP qui sert un outil `mcp__*`, et d'où provient la définition de ce serveur. Les entrées de hook [`PreToolUse`](#pretoolusehookinput), `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` et `PermissionDenied` le portent comme `mcp_server`, et les options [`CanUseTool`](#canusetool) le portent comme `mcpServer`. Les deux l'omettent pour les outils qui ne proviennent pas d'un serveur MCP.

```typescript theme={null}
type McpServerProvenance = {
  name: string;
  source: string;
};
```

| Champ    | Type     | Description                                                                                                            |
| :------- | :------- | :--------------------------------------------------------------------------------------------------------------------- |
| `name`   | `string` | Le nom sous lequel le serveur est enregistré, la même valeur que [`mcpServerStatus()`](#query-object) signale pour lui |
| `source` | `string` | D'où provient la définition du serveur : `sdk`, `plugin` ou une portée de configuration                                |

`source` prend l'une des valeurs suivantes. L'ensemble est ouvert, donc traitez une valeur que vous ne reconnaissez pas comme une source configurée, jamais comme `sdk` :

* **`sdk`** : un serveur en processus que votre application a enregistré. Seule l'application hôte du SDK peut en enregistrer un, donc un serveur configuré ne signale jamais `sdk`, quel que soit son nom.
* **`plugin`** : un serveur qu'un [plugin](/docs/fr/agent-sdk/plugins) fournit. Son `name` est la forme `plugin:<plugin-name>:<server-name>` délimitée décrite sous [serveurs MCP fournis par plugin](/docs/fr/mcp#plugin-provided-mcp-servers).
* **Une portée de configuration** : `user`, `project`, `local`, `dynamic`, `managed`, `enterprise`, `claudeai` ou `agent`. Un serveur `.mcp.json` signale `project`, et [les portées d'installation MCP](/docs/fr/mcp#mcp-installation-scopes) définissent `local`, `project` et `user`. Les serveurs que votre application transmet dans l'option [`mcpServers`](#options), autres que les serveurs SDK en processus, signalent `dynamic`.

Basez les décisions de confiance sur `source`, pas sur `name` ou le préfixe du nom d'outil `mcp__<server>__`. Pour toute source autre que `sdk`, `name` est du texte non fiable : échappez-le avant l'affichage.

`McpServerProvenance` et les champs qui le portent nécessitent Agent SDK v0.3.274 ou version ultérieure.

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

État d'un serveur MCP connecté.

```typescript theme={null}
type McpServerStatus = {
  name: string;
  status: "connected" | "failed" | "needs-auth" | "pending" | "disabled";
  serverInfo?: {
    name: string;
    version: string;
  };
  error?: string;
  config?: McpServerStatusConfig;
  scope?: string;
  source?: string;
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
    _meta?: Record<string, unknown>;
  }[];
};
```

`source` indique d'où provient la définition du serveur, avec les mêmes valeurs et règle de confiance que le `source` de [`McpServerProvenance`](#mcpserverprovenance). Le champ nécessite Agent SDK v0.3.274 ou version ultérieure et est absent sur les versions antérieures.

`_meta` sur une entrée `tools` porte les membres MCP Apps de `_meta` de cet outil, afin que votre application puisse trouver la ressource `ui://` à rendre avec [`readMcpResource()`](#query-object). Claude Code transmet l'objet `ui` et la chaîne `ui/resourceUri` plate dépréciée, et retient toute autre clé. À l'intérieur de `ui`, `resourceUri` est une chaîne `ui://` et `visibility` un tableau de `"model"` et `"app"` lorsque le serveur les définit, et tout autre membre passe inchangé. Claude Code supprime l'une ou l'autre clé lorsque la valeur est malformée, et omet `_meta` d'un outil qui ne déclare ni l'une ni l'autre. Le champ est présent uniquement lorsque les [`capabilities`](#sdksystemmessage) du message d'initialisation incluent `mcp_tool_ui_meta_v1`, et nécessite TypeScript Agent SDK v0.3.280 ou version ultérieure.

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

La configuration d'un serveur MCP telle que signalée par `mcpServerStatus()`. C'est l'union de tous les types de transport de serveur MCP.

```typescript theme={null}
type McpServerStatusConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfig
  | McpClaudeAIProxyServerConfig;
```

Consultez [`McpServerConfig`](#mcpserverconfig) pour les détails sur chaque type de transport.

<h3 id="accountinfo">
  `AccountInfo`
</h3>

Informations de compte pour l'utilisateur authentifié.

```typescript theme={null}
type AccountInfo = {
  email?: string;
  organization?: string;
  subscriptionType?: string;
  tokenSource?: string;
  apiKeySource?: string;
};
```

<h3 id="modelusage">
  `ModelUsage`
</h3>

Statistiques d'utilisation par modèle renvoyées dans les messages de résultat. La valeur `costUSD` est une estimation côté client. Consultez [Suivre le coût et l'utilisation](/docs/fr/agent-sdk/cost-tracking) pour les avertissements de facturation.

```typescript theme={null}
type ModelUsage = {
  inputTokens: number;
  outputTokens: number;
  thinkingTokens?: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
  canonicalModel?: string;
  provider?: string;
  costBasis?: 'list' | 'managed' | 'unknown';
};
```

`thinkingTokens` compte les jetons de réflexion que ce modèle a générés. `outputTokens` les inclut déjà, donc n'additionnez pas les deux. Le champ est absent jusqu'à ce qu'un tour s'exécute sur une version de Claude Code qui l'enregistre, donc une session reprise qui a commencé sur une version antérieure signale un décompte partiel. `thinkingTokens` nécessite Agent SDK v0.3.257 ou version ultérieure.

Les champs `canonicalModel` et `provider` nécessitent Claude Code v2.1.218 ou version ultérieure. `canonicalModel` est l'ID de modèle canonique que la recherche de prix utilise ; il peut différer de la chaîne de modèle brute qui clé l'entrée, par exemple lorsque cette chaîne est un ID spécifique au fournisseur ou un alias.

`provider` nomme le backend API qui a servi le modèle, tel que `firstParty`, `bedrock`, `vertex`, `foundry`, `anthropicAws`, `mantle` ou `gateway`.

`costBasis` nomme la table de prix qui a tarifé la dernière requête du modèle : `list` pour le prix catalogue, `managed` pour une table [`modelPricing`](/docs/fr/settings-reference#modelpricing), ou `unknown` lorsqu'aucune ne correspondait à l'ID du modèle. Le champ nécessite Claude Code v2.1.246 ou version ultérieure.

<h3 id="configscope">
  `ConfigScope`
</h3>

```typescript theme={null}
type ConfigScope = "local" | "user" | "project";
```

<h3 id="nonnullableusage">
  `NonNullableUsage`
</h3>

Une version de [`Usage`](#usage) avec tous les champs nullables rendus non-nullables.

```typescript theme={null}
type NonNullableUsage = {
  [K in keyof Usage]: NonNullable<Usage[K]>;
};
```

<h3 id="usage">
  `Usage`
</h3>

Statistiques d'utilisation des jetons. C'est le type `BetaUsage` de `@anthropic-ai/sdk`.

```typescript theme={null}
type Usage = {
  input_tokens: number;
  output_tokens: number;
  cache_creation_input_tokens: number | null;
  cache_read_input_tokens: number | null;
  cache_creation: {
    ephemeral_5m_input_tokens: number;
    ephemeral_1h_input_tokens: number;
  } | null;
  server_tool_use: BetaServerToolUsage | null;
  service_tier: "standard" | "priority" | "batch" | null;
  speed: "standard" | "fast" | null;
  inference_geo: string | null;
  iterations: BetaIterationsUsage | null;
  output_tokens_details: BetaOutputTokensDetails | null;
};
```

`BetaServerToolUsage`, `BetaIterationsUsage` et `BetaOutputTokensDetails` sont définis dans `@anthropic-ai/sdk`.

`output_tokens_details` décompose la sortie facturée par catégorie. Il porte actuellement un champ, `thinking_tokens: number`, comptant les jetons de sortie que le modèle a générés comme raisonnement interne, y compris les délimiteurs de bloc de réflexion. Le champ `output_tokens_details` nécessite TypeScript SDK v0.3.228 ou version ultérieure, qui regroupe Claude Code v2.1.228.

* **Facturation** : lisez la décomposition pour l'observabilité, pas pour la facturation. `output_tokens` reste le total faisant autorité, et `output_tokens - thinking_tokens` approxime la sortie sans raisonnement.
* **Ce que le décompte couvre** : le raisonnement brut que le modèle a produit, qui peut être plus long que le texte de réflexion renvoyé dans le corps de la réponse. L'API le calcule en retokenisant ce texte brut, donc il peut différer du décompte de génération exact du modèle de quelques jetons.
* **Streaming** : sur les messages d'assistant diffusés, cette décomposition, comme `output_tokens`, est un espace réservé `message_start` et ne porte aucun décompte réel, donc lisez-la à partir du message de résultat `usage` comme [Lire les jetons de sortie du message de résultat](/docs/fr/agent-sdk/cost-tracking#read-output-tokens-from-the-result-message) le décrit. Sur le message de résultat, `thinking_tokens` lit `0` lorsque le modèle ou le fournisseur ne signale aucune décomposition.
* **Cas `null`** : `output_tokens_details` lui-même est `null` sur les messages d'assistant que Claude Code synthétise, tels que les messages d'erreur API.

<h3 id="calltoolresult">
  `CallToolResult`
</h3>

Type de résultat d'outil MCP (de `@modelcontextprotocol/sdk/types.js`). `structuredContent` est un objet JSON qui peut être renvoyé aux côtés de `content`, y compris les blocs d'image. Consultez [Retourner des données structurées](/docs/fr/agent-sdk/custom-tools#return-structured-data).

```typescript theme={null}
type CallToolResult = {
  content: Array<{
    type: "text" | "image" | "audio" | "resource" | "resource_link";
    // Additional fields vary by type
  }>;
  structuredContent?: Record<string, unknown>;
  isError?: boolean;
};
```

<h3 id="sdkmcpresourcelink">
  `SDKMcpResourceLink`
</h3>

Un fichier qu'un outil MCP a renvoyé par référence. Claude Code construit chaque entrée à partir d'un bloc `resource_link` dans le résultat de l'outil et livre la liste comme `resourceLinks` sur [`SDKUserMessage.tool_use_result`](#sdkusermessage), ou comme `resource_links` sur [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) lorsque l'appel s'est terminé en arrière-plan. Nécessite Agent SDK v0.3.257 ou version ultérieure.

```typescript theme={null}
type SDKMcpResourceLink = {
  uri: string;
  name: string;
  title?: string;
  description?: string;
  mimeType?: string;
  size?: number;
  annotations?: Record<string, unknown>;
};
```

Claude Code supprime un bloc dont `uri` ou `name` n'est pas une chaîne, et omet un champ optionnel dont la valeur n'est pas du type listé.

| Champ         | Type                                   | Description                                                          |
| :------------ | :------------------------------------- | :------------------------------------------------------------------- |
| `uri`         | `string`                               | URI de la ressource, telle que le serveur l'a renvoyée               |
| `name`        | `string`                               | Nom que le serveur a donné à la ressource                            |
| `title`       | `string \| undefined`                  | Titre d'affichage, lorsque le serveur en a défini un                 |
| `description` | `string \| undefined`                  | Description, lorsque le serveur en a défini une                      |
| `mimeType`    | `string \| undefined`                  | Type MIME, lorsque le serveur en a défini un                         |
| `size`        | `number \| undefined`                  | Taille en octets, lorsque le serveur en a défini une                 |
| `annotations` | `Record<string, unknown> \| undefined` | L'objet d'annotations MCP du bloc, lorsque le serveur en a défini un |

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Contrôle le comportement de réflexion/raisonnement de Claude. Prend la priorité sur le `maxThinkingTokens` déprécié.

```typescript theme={null}
type ThinkingDisplay = "summarized" | "omitted";

type ThinkingConfig =
  | { type: "adaptive"; display?: ThinkingDisplay } // The model determines when and how much to reason (Opus 4.6+)
  | { type: "enabled"; budgetTokens?: number; display?: ThinkingDisplay } // Fixed thinking token budget
  | { type: "disabled" }; // No extended thinking
```

Le champ optionnel `display` contrôle si le texte de réflexion est renvoyé `"summarized"` ou `"omitted"`. Sur Claude Opus 4.7 et versions ultérieures, la valeur par défaut de l'API est `"omitted"`, donc définissez `"summarized"` pour recevoir le contenu de réflexion dans les blocs `thinking`. Claude Code n'envoie pas `display` à Amazon Bedrock ou à la plateforme Agent de Google Cloud, donc sur ces fournisseurs Opus 4.7 et versions ultérieures renvoient des blocs `thinking` vides même lorsque vous définissez `display` sur `"summarized"`.

<h3 id="spawnedprocess">
  `SpawnedProcess`
</h3>

Interface pour la génération de processus personnalisée (utilisée avec l'option `spawnClaudeCodeProcess`). `ChildProcess` satisfait déjà cette interface.

```typescript theme={null}
interface SpawnedProcess {
  stdin: Writable;
  stdout: Readable;
  readonly killed: boolean;
  readonly exitCode: number | null;
  kill(signal: NodeJS.Signals): boolean;
  on(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  on(event: "error", listener: (error: Error) => void): void;
  once(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  once(event: "error", listener: (error: Error) => void): void;
  off(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  off(event: "error", listener: (error: Error) => void): void;
}
```

<h3 id="spawnoptions">
  `SpawnOptions`
</h3>

Options transmises à la fonction de génération personnalisée.

```typescript theme={null}
interface SpawnOptions {
  command: string;
  args: string[];
  cwd?: string;
  env: Record<string, string | undefined>;
  signal: AbortSignal;
}
```

<Note>
  Le champ `signal` indique à votre fonction de génération quand arrêter le processus. Transmettez-le comme l'option `signal` au `spawn()` de Node, ou transmettez-le à votre gestionnaire d'arrêt de VM ou de conteneur.

  Ce signal ne se déclenche pas à l'instant où [`Options.abortController`](#options) s'arrête. Le SDK ferme d'abord stdin du processus et attend environ deux secondes pour que l'interface de ligne de commande s'arrête proprement, puis arrête ce signal. Pour réagir au moment où l'appelant s'arrête, écoutez votre propre `Options.abortController.signal`, que votre fonction de génération peut référencer à partir de sa portée englobante.
</Note>

<h3 id="mcpsetserversresult">
  `McpSetServersResult`
</h3>

Résultat d'une opération `setMcpServers()`.

```typescript theme={null}
type McpSetServersResult = {
  added: string[];
  removed: string[];
  errors: Record<string, string>;
};
```

Lorsque vous appelez `setMcpServers()`, Claude Code applique ces règles :

* **Serveurs que l'appel ne nomme pas** : Claude Code maintient les serveurs fournis par plugin en cours d'exécution. Nécessite Agent SDK v0.3.210 ou version ultérieure.
* **Serveurs que l'appel nomme** : à l'exception des serveurs intégrés que l'interface de ligne de commande a démarrés au démarrage, Claude Code remplace un serveur en cours d'exécution uniquement lorsque sa configuration diffère de celle que vous avez transmise.
* **Serveurs intégrés que l'interface de ligne de commande a démarrés au démarrage** : si l'appel en nomme un, Claude Code supprime cette entrée et la signale dans `errors`.

La promesse se résout après que les serveurs stdio, HTTP et SSE nouvellement ajoutés se connectent ou échouent, donc les outils des serveurs qui se sont connectés sont disponibles au tour suivant.

`added` liste les serveurs que Claude Code a ajoutés ou remplacés, qu'ils se soient connectés ou non. Un serveur qui n'a pas pu se connecter apparaît à la fois dans `added` et `errors`, avec le texte d'échec sous `errors` et une ligne `failed` dans [`mcpServerStatus()`](#methods). Avant Claude Code v2.1.257, un serveur dont la tentative de connexion a levé une exception était signalé uniquement sous `errors`.

<h3 id="rewindfilesresult">
  `RewindFilesResult`
</h3>

Résultat d'une opération `rewindFiles()`.

```typescript theme={null}
type RewindFilesResult = {
  canRewind: boolean;
  error?: string;
  filesChanged?: string[];
  insertions?: number;
  deletions?: number;
  skippedLinks?: number;
};
```

`skippedLinks` compte les chemins suivis que la rembobinage a refusé de restaurer ou de supprimer pour la sécurité des liens : un lien symbolique, un lien physique ou un autre fichier non régulier au chemin suivi, un répertoire parent qui ne se résout plus à l'endroit où il pointait lorsque le point de contrôle a été pris, ou une sauvegarde qui n'a pas pu être lue en toute sécurité. Le champ nécessite Claude Code v2.1.216 ou version ultérieure. Un appel d'aperçu avec `rewindFiles(userMessageId, { dryRun: true })` ne le définit jamais.

<h3 id="sdkstatusmessage">
  `SDKStatusMessage`
</h3>

Message de mise à jour d'état (par exemple, compactage).

```typescript theme={null}
type SDKStatusMessage = {
  type: "system";
  subtype: "status";
  status: "compacting" | null;
  permissionMode?: PermissionMode;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktasknotificationmessage">
  `SDKTaskNotificationMessage`
</h3>

Notification lorsqu'une tâche en arrière-plan se termine, échoue ou est arrêtée. Les tâches en arrière-plan incluent les commandes Bash `run_in_background`, les montres [Monitor](#monitor) et les sous-agents en arrière-plan. Pour le champ `ambient`, consultez [`SDKTaskStartedMessage`](#sdktaskstartedmessage), qui le définit et son exigence de version.

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  ambient?: boolean;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  resource_links?: SDKMcpResourceLink[];
  uuid: UUID;
  session_id: string;
};
```

Lorsque Claude Code [déplace un long appel d'outil MCP en arrière-plan](/docs/fr/mcp#automatic-backgrounding-of-long-tool-calls), le bloc `tool_result` pour cet appel ne contient qu'un espace réservé et le résultat réel de l'appel arrive dans cette notification. Faites correspondre la notification à l'appel avec `tool_use_id`. Sur une notification `completed`, `resource_links` liste les fichiers que l'outil a renvoyés par référence comme entrées [`SDKMcpResourceLink`](#sdkmcpresourcelink), avec les mêmes limites de 50 liens et 64 KiB que [`tool_use_result.resourceLinks`](#sdkusermessage). Claude Code omet `resource_links` lorsque le résultat n'avait pas de liens et sur les notifications pour les tâches qui ne sont pas des appels d'outil MCP. `resource_links` nécessite Agent SDK v0.3.257 ou version ultérieure.

Claude Code ajoute un avis à chaque notification de tâche qu'il envoie au modèle, sauf les livraisons estampillées avec le sous-type [`scheduled-trigger`](#task-notification-subkinds), qui portent plutôt un cadrage de tâche assignée. L'avis indique qu'aucune entrée humaine n'a eu lieu, donc le modèle ne traite pas la notification comme une instruction ou une approbation de l'utilisateur.

Pour détecter un tour de notification de tâche, vérifiez `origin.kind === "task-notification"` sur [`SDKUserMessage`](#sdkusermessage) ou [`SDKResultMessage`](#sdkresultmessage) plutôt que de faire correspondre le texte de l'avis. Lisez `subkind` à partir du même champ si vous avez besoin de savoir ce qui l'a soulevé. Avant v2.1.205, Claude Code laissait l'avis hors des notifications qui arrivaient pendant que la session était inactive.

<h3 id="sdktoolusesummarymessage">
  `SDKToolUseSummaryMessage`
</h3>

Résumé de l'utilisation des outils dans une conversation.

```typescript theme={null}
type SDKToolUseSummaryMessage = {
  type: "tool_use_summary";
  summary: string;
  preceding_tool_use_ids: string[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookstartedmessage">
  `SDKHookStartedMessage`
</h3>

Émis lorsqu'un hook commence à s'exécuter.

Claude Code livre ce message, [`SDKHookProgressMessage`](#sdkhookprogressmessage) et [`SDKHookResponseMessage`](#sdkhookresponsemessage) au flux de messages immédiatement, y compris pendant qu'un hook `SessionStart` ou `Setup` s'exécute toujours au démarrage de la session. Claude Code v2.1.169 à v2.1.203 livrait ces messages en un lot après qu'un hook `SessionStart` ou `Setup` se soit terminé ; v2.1.204 a restauré la livraison en direct.

```typescript theme={null}
type SDKHookStartedMessage = {
  type: "system";
  subtype: "hook_started";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookprogressmessage">
  `SDKHookProgressMessage`
</h3>

Émis pendant qu'un hook s'exécute, avec la sortie stdout/stderr.

```typescript theme={null}
type SDKHookProgressMessage = {
  type: "system";
  subtype: "hook_progress";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  stdout: string;
  stderr: string;
  output: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookresponsemessage">
  `SDKHookResponseMessage`
</h3>

Émis lorsqu'un hook termine son exécution.

```typescript theme={null}
type SDKHookResponseMessage = {
  type: "system";
  subtype: "hook_response";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  output: string;
  stdout: string;
  stderr: string;
  exit_code?: number;
  outcome: "success" | "error" | "cancelled";
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktoolprogressmessage">
  `SDKToolProgressMessage`
</h3>

Émis périodiquement pendant qu'un outil s'exécute pour indiquer la progression.

```typescript theme={null}
type SDKToolProgressMessage = {
  type: "tool_progress";
  tool_use_id: string;
  tool_name: string;
  parent_tool_use_id: string | null;
  elapsed_time_seconds: number;
  task_id?: string;
  heartbeat?: boolean;
  subagent_type?: string;
  subagent_retry?: {
    agent_id: string;
    attempt: number;
    max_retries: number;
    retry_delay_ms: number;
    error_status: number | null;
    error_category: string;
  };
  uuid: UUID;
  session_id: string;
};
```

Pendant qu'un appel d'outil s'exécute dans la conversation principale, Claude Code émet un message `tool_progress` toutes les 30 secondes avec `heartbeat: true`. Chaque battement porte le nom de l'outil et les secondes écoulées, afin que vous puissiez distinguer un appel de longue durée d'une session bloquée. Claude Code n'émet pas de battements pour les appels d'outil à l'intérieur d'un sous-agent. Le champ `heartbeat` nécessite Agent SDK v0.3.214 ou version ultérieure. Avant v2.1.257, Claude Code n'émettait pas non plus de battements pour un appel d'outil Agent au premier plan.

Sur les messages `tool_progress` pour l'outil Agent autres que les battements, `subagent_type` nomme le type de sous-agent en cours d'exécution, tel que `general-purpose`. `subagent_retry` est présent pendant que ce sous-agent attend un backoff d'erreur API, tel qu'une limite de débit ou une surcharge, avec un message par tentative de nouvelle tentative. Les deux champs nécessitent Agent SDK v0.3.214 ou version ultérieure.

Pour rendre un indicateur de nouvelle tentative à partir de `subagent_retry` :

* Suivez l'indicateur par `parent_tool_use_id`, qui est unique par sous-agent. `tool_use_id` est partagé par les sous-agents parallèles d'un tour d'assistant, donc le suivi par celui-ci laisserait la mise à jour d'un sous-agent effacer l'indicateur d'un autre.
* Effacez l'indicateur lorsqu'un `tool_progress` ultérieur pour le même `parent_tool_use_id` arrive sans `subagent_retry` ni `heartbeat: true`, ou lorsque le message de résultat de l'outil arrive. Les cadres avec `heartbeat: true` ne signalent que la vivacité, donc conservez l'indicateur lorsqu'un arrive. `attempt` peut dépasser `max_retries` sous une nouvelle tentative persistante, donc ne dérivez pas l'effacement des compteurs.
* Traitez `error_category` comme un jeton pour choisir votre propre texte de message, pas comme du texte d'affichage. Les valeurs sont `rate_limit`, `overloaded`, `authentication_failed`, `server_error`, `cloud_credential_error` et `unknown`. Gérez une valeur que vous ne reconnaissez pas de la même manière que vous gérez `unknown`, car les versions ultérieures peuvent ajouter des valeurs.

<h3 id="sdkauthstatusmessage">
  `SDKAuthStatusMessage`
</h3>

Émis pendant les flux d'authentification.

```typescript theme={null}
type SDKAuthStatusMessage = {
  type: "auth_status";
  isAuthenticating: boolean;
  output: string[];
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskstartedmessage">
  `SDKTaskStartedMessage`
</h3>

Émis lorsqu'une tâche commence. Le champ `task_type` est `"local_bash"` pour les commandes Bash et les montres [Monitor](#monitor), `"local_agent"` pour les sous-agents, ou `"remote_agent"`.

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  is_backgrounded?: boolean;
  spawn_depth?: number;
  ambient?: boolean;
  uuid: UUID;
  session_id: string;
};
```

`ambient` est `true` pour les tâches qui ne font pas partie du travail de la session, telles que les tâches que Claude Code exécute pour son propre fonctionnement. Les montres de mise à jour en direct sont également ambiantes, y compris les montres que l'utilisateur a demandées. Excluez les tâches ambiantes des indicateurs d'activité. Le champ nécessite Agent SDK v0.3.247 ou version ultérieure.

`ambient` apparaît également sur [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) et sur les entrées [`SDKBackgroundTasksChangedMessage`](#sdkbackgroundtaskschangedmessage).

`is_backgrounded` et `spawn_depth` décrivent comment Claude Code a démarré la tâche. Les deux champs nécessitent Agent SDK v0.3.238 ou version ultérieure.

* `is_backgrounded` : Claude Code le définit sur les tâches `"local_agent"` et `"local_bash"`. `true` signifie que la tâche s'exécute en arrière-plan. `false` signifie que la tâche s'exécute au premier plan, et l'appel d'outil qui l'a démarrée reste bloqué jusqu'à ce que la tâche se termine ou se déplace en arrière-plan.
* `spawn_depth` : Claude Code le définit uniquement sur les tâches `"local_agent"`. Un sous-agent que le thread principal a généré a une profondeur `1`. Un sous-agent qu'un sous-agent de profondeur `1` a généré a une profondeur `2`, et ainsi de suite.

Un [sous-agent repris](/docs/fr/agent-sdk/subagents#resume-subagents) signale toujours `is_backgrounded: true`, car Claude Code exécute chaque sous-agent repris en arrière-plan. Lorsqu'une tâche au premier plan se déplace en arrière-plan plus tard, Claude Code signale la nouvelle valeur `is_backgrounded` dans un message [`task_updated`](#sdktaskupdatedmessage) plutôt que d'envoyer un second `task_started`.

<h3 id="sdktaskprogressmessage">
  `SDKTaskProgressMessage`
</h3>

Émis périodiquement pendant qu'un sous-agent ou une tâche en arrière-plan s'exécute. Le champ `summary` est rempli uniquement lorsque [`agentProgressSummaries`](#options) est activé.

```typescript theme={null}
type SDKTaskProgressMessage = {
  type: "system";
  subtype: "task_progress";
  task_id: string;
  tool_use_id?: string;
  description: string;
  subagent_type?: string;
  usage: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  last_tool_name?: string;
  summary?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskupdatedmessage">
  `SDKTaskUpdatedMessage`
</h3>

Émis lorsque l'état d'une tâche en arrière-plan change, par exemple lorsqu'elle passe de `running` à `completed`. Fusionnez `patch` dans votre carte de tâches locale indexée par `task_id`. Le champ `end_time` est un horodatage d'époque Unix en millisecondes, comparable avec `Date.now()`.

```typescript theme={null}
type SDKTaskUpdatedMessage = {
  type: "system";
  subtype: "task_updated";
  task_id: string;
  patch: {
    status?: "pending" | "running" | "completed" | "failed" | "killed";
    description?: string;
    end_time?: number;
    total_paused_ms?: number;
    error?: string;
    is_backgrounded?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkbackgroundtaskschangedmessage">
  `SDKBackgroundTasksChangedMessage`
</h3>

Émis chaque fois que l'ensemble des tâches en arrière-plan en direct change : une tâche démarre, se termine, est tuée, un agent au premier plan est mis en arrière-plan, ou le champ `description` ou `ambient` d'une tâche change.

Le tableau `tasks` est l'ensemble en direct complet. Remplacez tout ensemble mis en cache par chaque charge utile au lieu d'appairer les événements `task_started` et `task_notification`, afin que le prochain changement d'adhésion corrige tout événement que vous avez manqué.

L'ordre relatif à ces événements par tâche n'est pas spécifié, donc ne corrélez pas les deux flux.

Rien n'est émis au démarrage. Réinitialisez à un ensemble vide chaque fois que le processus CLI de la session démarre ou redémarre et laissez le prochain changement d'adhésion le repeupler.

Lorsque vous envoyez une demande de contrôle `initialize` répétée à une session en cours d'exécution, par exemple avec [`reinitialize()`](#query-object) après un écart de transport, Claude Code suit la réponse avec un instantané de l'ensemble en direct actuel, même lorsqu'il est vide. Un hôte qui se reconnecte apprend donc ce qui s'exécute sans attendre le prochain changement d'adhésion. Avant Agent SDK v0.3.239, Claude Code n'envoyait aucun instantané après un `initialize` répété.

Nécessite Claude Code v2.1.203 ou version ultérieure.

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
    ambient?: boolean;
  }[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkthinkingtokensmessage">
  `SDKThinkingTokensMessage`
</h3>

Émis pendant que Claude produit un bloc de réflexion, y compris un bloc édité. `estimated_tokens` est une estimation en cours des jetons de réflexion générés jusqu'à présent dans le bloc actuel, et `estimated_tokens_delta` est l'incrément porté par ce cadre. Utilisez ces estimations pour l'affichage de la progression.

Lorsque le modèle ou le fournisseur signale une décomposition, le décompte final pour la boucle d'agent de haut niveau est le [`usage.output_tokens_details.thinking_tokens`](#usage) du message de résultat, qui [n'inclut pas les jetons de sous-agent](/docs/fr/agent-sdk/cost-tracking#get-the-total-cost-of-a-query).

Nécessite Claude Code v2.1.153 ou version ultérieure.

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  user_message_uuid?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkfilespersistedevent">
  `SDKFilesPersistedEvent`
</h3>

Émis lorsque les points de contrôle de fichiers sont persistés sur le disque.

```typescript theme={null}
type SDKFilesPersistedEvent = {
  type: "system";
  subtype: "files_persisted";
  files: { filename: string; file_id: string }[];
  failed: { filename: string; error: string }[];
  processed_at: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkratelimitevent">
  `SDKRateLimitEvent`
</h3>

Émis lorsque la session rencontre une limite de débit.

```typescript theme={null}
type SDKRateLimitEvent = {
  type: "rate_limit_event";
  rate_limit_info: {
    status: "allowed" | "allowed_warning" | "rejected";
    resetsAt?: number;
    utilization?: number;
    errorCode?: "credits_required";
    canUserPurchaseCredits?: boolean;
    hasChargeableSavedPaymentMethod?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

Lorsque `errorCode` est `"credits_required"`, le rejet provient d'un abonnement claude.ai dont l'utilisation incluse est épuisée, et la session ne peut pas continuer jusqu'à ce que l'utilisateur achète des crédits d'utilisation. `canUserPurchaseCredits` indique si l'utilisateur authentifié peut acheter des crédits pour le compte, et `hasChargeableSavedPaymentMethod` indique si une méthode de paiement enregistrée est en dossier. Les trois champs sont absents sur les événements de limite de débit qui ne sont pas des rejets de crédits requis. Nécessite Claude Code v2.1.181 ou version ultérieure.

<h3 id="sdklocalcommandoutputmessage">
  `SDKLocalCommandOutputMessage`
</h3>

Claude Code n'émet pas ce type de message. Lorsque vous envoyez une commande telle que `/context` ou `/usage` comme invite, sa sortie arrive comme [`SDKAssistantMessage`](#sdkassistantmessage).

```typescript theme={null}
type SDKLocalCommandOutputMessage = {
  type: "system";
  subtype: "local_command_output";
  content: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkcommandschangedmessage">
  `SDKCommandsChangedMessage`
</h3>

Émis lorsque l'ensemble des commandes disponibles change en milieu de session, par exemple lorsque Claude Code découvre des compétences à mesure que l'agent entre dans un sous-répertoire. Le tableau `commands` est la liste complète mise à jour, donc remplacez tout cache de liste de commandes par cette charge utile. L'appel de [`supportedCommands()`](#query-object) après ce message renvoie la même liste mise à jour, car la méthode suit le dernier push ; cela nécessite Agent SDK v0.3.216 ou version ultérieure. Dans les versions antérieures du SDK, `supportedCommands()` renvoie l'instantané capturé à l'initialisation et ne reflète jamais les changements en milieu de session.

```typescript theme={null}
type SDKCommandsChangedMessage = {
  type: "system";
  subtype: "commands_changed";
  commands: SlashCommand[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpromptsuggestionmessage">
  `SDKPromptSuggestionMessage`
</h3>

Émis après un tour lorsque [`promptSuggestions`](#options) est activé et que Claude Code a généré une suggestion pour ce tour. Contient l'invite utilisateur suivante prédite. Pour les tours qui n'en reçoivent pas, consultez [Quand Claude Code ignore les suggestions](/docs/fr/interactive-mode#when-claude-code-skips-suggestions).

```typescript theme={null}
type SDKPromptSuggestionMessage = {
  type: "prompt_suggestion";
  suggestion: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkconversationresetmessage">
  `SDKConversationResetMessage`
</h3>

Émis lorsque la conversation de la session est remplacée sans terminer la session. Dans un appel `query()`, seul `/clear` et ses alias produisent ce message. Montez une transcription vide sous `new_conversation_id` et jetez tout titre de session mis en cache.

```typescript theme={null}
type SDKConversationResetMessage = {
  type: "conversation_reset";
  new_conversation_id: UUID;
  uuid: UUID;
  session_id: string;
};
```

Les typages publiés du SDK déclarent `SDKConversationResetMessage` dans Claude Code v2.1.203 et versions ultérieures. Avant v2.1.203, `SDKMessage` référençait le type sans le déclarer, donc le rétrécissement sur `type === "conversation_reset"` n'a pas pu être typé lorsque `skipLibCheck` était désactivé.

<h3 id="aborterror">
  `AbortError`
</h3>

Classe d'erreur personnalisée pour les opérations d'arrêt.

```typescript theme={null}
class AbortError extends Error {}
```

`AbortError` est la seule classe d'erreur dans l'API typée du SDK. Les autres défaillances, telles que la sortie ou l'échec du lancement du processus Claude Code, rejettent l'itération de message avec des erreurs qui ne portent aucune classe SDK pour correspondre. [Dépannage](/docs/fr/agent-sdk/troubleshooting) clé ces erreurs par message, avec la cause et la correction pour chacune.

<h2 id="sandbox-configuration">
  Configuration du sandbox
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Configuration du comportement du sandbox. Utilisez ceci pour activer le sandboxing des commandes et configurer les restrictions réseau par programmation.

```typescript theme={null}
type SandboxSettings = {
  enabled?: boolean;
  failIfUnavailable?: boolean;
  autoAllowBashIfSandboxed?: boolean;
  excludedCommands?: string[];
  allowUnsandboxedCommands?: boolean;
  network?: SandboxNetworkConfig;
  filesystem?: SandboxFilesystemConfig;
  ignoreViolations?: Record<string, string[]>;
  enableWeakerNestedSandbox?: boolean;
  ripgrep?: { command: string; args?: string[] };
};
```

| Property                    | Type                                                  | Default     | Description                                                                                                                                                                                                                                                                                   |
| :-------------------------- | :---------------------------------------------------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `boolean`                                             | `false`     | Activer le mode sandbox pour l'exécution des commandes                                                                                                                                                                                                                                        |
| `failIfUnavailable`         | `boolean`                                             | `true`      | S'arrêter au démarrage si `enabled` est `true` mais que le sandbox ne peut pas démarrer. Définissez `false` pour revenir à l'exécution non sandboxée avec un avertissement sur stderr                                                                                                         |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`      | Approuver automatiquement les commandes Bash lorsque le sandbox est activé                                                                                                                                                                                                                    |
| `excludedCommands`          | `string[]`                                            | `[]`        | Commandes qui contournent les restrictions du sandbox, telles que `['docker *']`. Celles-ci s'exécutent automatiquement sans sandbox et sans intervention du modèle ; [`sandbox.excludedCommands`](/docs/fr/settings-reference#sandbox-excludedcommands) couvre le moment où une entrée s'applique |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`      | Permettre au modèle de demander l'exécution de commandes en dehors du sandbox. Lorsque `true`, le modèle peut définir `dangerouslyDisableSandbox` dans l'entrée de l'outil, ce qui revient au [système de permissions](#permissions-fallback-for-unsandboxed-commands)                        |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined` | Configuration du sandbox spécifique au réseau                                                                                                                                                                                                                                                 |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | Configuration du sandbox spécifique au système de fichiers pour les restrictions de lecture/écriture                                                                                                                                                                                          |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined` | Mappage des sous-chaînes de commande, ou `*` pour chaque commande, aux sous-chaînes du texte de violation à ignorer, telles que `{ "*": ['/etc/hosts'] }` ; voir [`sandbox.ignoreViolations`](/docs/fr/settings-reference#sandbox-ignoreviolations)                                                |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`     | Activer un sandbox imbriqué plus faible pour la compatibilité                                                                                                                                                                                                                                 |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined` | Configuration du binaire ripgrep personnalisé pour les environnements sandbox                                                                                                                                                                                                                 |

<Note>
  Le sandbox dépend du support de la plateforme et, sur Linux, d'outils comme `bubblewrap` et `socat`. Lorsque `enabled` est `true` et que le sandbox ne peut pas démarrer, `query()` signale un message `result` avec `subtype: "error_during_execution"` et la raison dans `errors`. Pour un seul appel `query()`, le SDK lance une exception après avoir cédé ce résultat d'erreur, donc enveloppez la boucle dans un bloc try pour continuer au-delà. Voir [Gérer le résultat](/docs/fr/agent-sdk/agent-loop#handle-the-result) pour le contrat d'erreur.

  Pour s'exécuter sans sandbox à la place, définissez `failIfUnavailable: false`.
</Note>

<h4 id="example-usage">
  Exemple d'utilisation
</h4>

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({
    prompt: "Build and test my project",
    options: {
      sandbox: {
        enabled: true,
        autoAllowBashIfSandboxed: true,
        network: {
          allowLocalBinding: true
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result,
  // such as when the sandbox can't start (failIfUnavailable defaults to true).
  console.log(`Session ended with an error: ${error}`);
}
```

<Warning>
  **Sécurité des sockets Unix :** L'option `allowUnixSockets` peut accorder l'accès à des services système qui s'étendent en dehors du sandbox. Par exemple, autoriser `/var/run/docker.sock` accorde effectivement un accès complet au système hôte via l'API Docker, contournant l'isolation du sandbox. Autorisez uniquement les sockets Unix strictement nécessaires et comprenez les implications de sécurité de chacun.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Configuration spécifique au réseau pour le mode sandbox. Ces paramètres s'appliquent aux commandes Bash sandboxées lorsque `enabled` est `true` dans le parent [`SandboxSettings`](#sandboxsettings). Ils ne restreignent pas l'outil WebFetch, qui utilise à la place des [règles de permission](/docs/fr/permissions#webfetch).

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  strictAllowlist?: boolean;
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| Property                  | Type       | Default     | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :------------------------ | :--------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`        | Noms de domaine auxquels les processus sandboxés peuvent accéder                                                                                                                                                                                                                                                                                                                                                                                           |
| `deniedDomains`           | `string[]` | `[]`        | Noms de domaine auxquels les processus sandboxés ne peuvent pas accéder. Prend la priorité sur `allowedDomains`                                                                                                                                                                                                                                                                                                                                            |
| `strictAllowlist`         | `boolean`  | `false`     | Refuser aux commandes sandboxées l'accès aux hôtes en dehors de la [liste d'autorisation réseau](/docs/fr/sandboxing#network-isolation) au lieu de demander. Appliqué uniquement aux commandes sandboxées ; les outils en processus tels que WebFetch ne sont pas contrôlés par celui-ci. Honoré uniquement à partir des paramètres utilisateur, gérés ou CLI `--settings` ; les paramètres de projet sont ignorés. Nécessite Claude Code v2.1.219 ou ultérieur |
| `allowManagedDomainsOnly` | `boolean`  | `false`     | Paramètres gérés uniquement. Lorsqu'il est défini dans les [paramètres gérés](/docs/fr/managed-settings), seules les entrées `allowedDomains` et les règles d'autorisation `WebFetch(domain:...)` des paramètres gérés sont honorées, et les entrées d'autorisation des paramètres utilisateur, projet ou locaux sont ignorées. N'a aucun effet lorsqu'il est défini via les options SDK                                                                        |
| `allowLocalBinding`       | `boolean`  | `false`     | Permettre aux processus de se lier à des ports locaux (par exemple, pour les serveurs de développement)                                                                                                                                                                                                                                                                                                                                                    |
| `allowUnixSockets`        | `string[]` | `[]`        | Chemins de socket Unix auxquels les processus peuvent accéder (par exemple, socket Docker)                                                                                                                                                                                                                                                                                                                                                                 |
| `allowAllUnixSockets`     | `boolean`  | `false`     | Permettre l'accès à tous les sockets Unix                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `httpProxyPort`           | `number`   | `undefined` | Port du proxy HTTP pour les requêtes réseau                                                                                                                                                                                                                                                                                                                                                                                                                |
| `socksProxyPort`          | `number`   | `undefined` | Port du proxy SOCKS pour les requêtes réseau                                                                                                                                                                                                                                                                                                                                                                                                               |

<Note>
  Le proxy sandbox intégré applique `allowedDomains` en fonction du nom d'hôte demandé et ne termine ni n'inspecte le trafic TLS, donc des techniques telles que le [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) peuvent potentiellement le contourner. Voir [Limitations de sécurité du sandboxing](/docs/fr/sandboxing#security-limitations) pour les détails et [Déploiement sécurisé](/docs/fr/agent-sdk/secure-deployment#traffic-forwarding) pour configurer un proxy qui termine TLS.
</Note>

<h3 id="sandboxfilesystemconfig">
  `SandboxFilesystemConfig`
</h3>

Configuration spécifique au système de fichiers pour le mode sandbox.

```typescript theme={null}
type SandboxFilesystemConfig = {
  allowWrite?: string[];
  denyWrite?: string[];
  denyRead?: string[];
};
```

| Property     | Type       | Default | Description                                                              |
| :----------- | :--------- | :------ | :----------------------------------------------------------------------- |
| `allowWrite` | `string[]` | `[]`    | Modèles de chemin de fichier pour lesquels autoriser l'accès en écriture |
| `denyWrite`  | `string[]` | `[]`    | Modèles de chemin de fichier pour lesquels refuser l'accès en écriture   |
| `denyRead`   | `string[]` | `[]`    | Modèles de chemin de fichier pour lesquels refuser l'accès en lecture    |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Système de permissions de secours pour les commandes non sandboxées
</h3>

Lorsque `allowUnsandboxedCommands` est activé, le modèle peut demander l'exécution de commandes en dehors du sandbox en définissant `dangerouslyDisableSandbox: true` dans l'entrée de l'outil. Ces demandes reviennent au système de permissions existant, ce qui signifie que votre gestionnaire `canUseTool` est invoqué, vous permettant de mettre en œuvre une logique d'autorisation personnalisée.

Vos entrées `excludedCommands` prennent plutôt un appel en dehors du sandbox sans intervention du modèle ; [`sandbox.excludedCommands`](/docs/fr/settings-reference#sandbox-excludedcommands) couvre le moment où une entrée s'applique.

Dans l'exemple ci-dessous, `isCommandAuthorized` représente une vérification d'autorisation que vous définissez.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true // Model can request unsandboxed execution
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // Check if the model is requesting to bypass the sandbox
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // The model is requesting to run this command outside the sandbox
        console.log(`Unsandboxed command requested: ${input.command}`);

        if (isCommandAuthorized(input.command)) {
          return { behavior: "allow" as const, updatedInput: input };
        }
        return {
          behavior: "deny" as const,
          message: "Command not authorized for unsandboxed execution"
        };
      }
      return { behavior: "allow" as const, updatedInput: input };
    }
  }
})) {
  if ("result" in message) console.log(message.result);
}
```

<Warning>
  Les commandes s'exécutant avec `dangerouslyDisableSandbox: true` ont un accès complet au système. Assurez-vous que votre gestionnaire `canUseTool` valide ces demandes avec soin.

  Si `permissionMode` est défini sur `bypassPermissions` et `allowUnsandboxedCommands` est activé, le modèle peut exécuter de manière autonome des commandes en dehors du sandbox sans invites d'approbation, à l'exception des [actions qu'aucun mode n'approuve automatiquement](/docs/fr/permission-modes#actions-no-mode-auto-approves). Cette combinaison permet effectivement au modèle d'échapper à l'isolation du sandbox silencieusement.
</Warning>

<h2 id="see-also">
  Voir aussi
</h2>

* [Aperçu du SDK](/docs/fr/agent-sdk/overview) - Concepts généraux du SDK
* [Référence du SDK Python](/docs/fr/agent-sdk/python) - Documentation du SDK Python
* [Référence CLI](/docs/fr/cli-reference) - Interface de ligne de commande
* [Flux de travail courants](/docs/fr/common-workflows) - Guides étape par étape
