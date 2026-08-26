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
  Le SDK regroupe un binaire Claude Code natif pour votre plateforme en tant que dépendance optionnelle telle que `@anthropic-ai/claude-agent-sdk-darwin-arm64`. Vous n'avez pas besoin d'installer Claude Code séparément. Si votre gestionnaire de paquets ignore les dépendances optionnelles, le SDK lève `Native CLI binary for <platform> not found` ; définissez [`pathToClaudeCodeExecutable`](#options) sur un binaire `claude` installé séparément à la place.
</Note>

<h3 id="compile-to-a-single-executable">
  Compiler en un seul exécutable
</h3>

Lorsque vous compilez votre application en un exécutable à fichier unique avec `bun build --compile`, le SDK ne peut pas résoudre le binaire CLI fourni au moment de l'exécution. `require.resolve` ne fonctionne pas à l'intérieur du système de fichiers virtuel `$bunfs` de l'exécutable compilé, donc le SDK lève `Native CLI binary for <platform> not found`.

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
  extras?: { annotations?: ToolAnnotations }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-3">
  Paramètres
</h4>

| Paramètre     | Type                                                              | Description                                                                         |
| :------------ | :---------------------------------------------------------------- | :---------------------------------------------------------------------------------- |
| `name`        | `string`                                                          | Le nom de l'outil                                                                   |
| `description` | `string`                                                          | Une description de ce que fait l'outil                                              |
| `inputSchema` | `Schema extends AnyZodRawShape`                                   | Schéma Zod définissant les paramètres d'entrée de l'outil (supporte Zod 3 et Zod 4) |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>` | Fonction asynchrone qui exécute la logique de l'outil                               |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)` }`       | Annotations MCP optionnelles fournissant des indices comportementaux aux clients    |

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
  tools?: Array<SdkMcpToolDefinition<any>>;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  Paramètres
</h4>

| Paramètre         | Type                          | Description                                                   |
| :---------------- | :---------------------------- | :------------------------------------------------------------ |
| `options.name`    | `string`                      | Le nom du serveur MCP                                         |
| `options.version` | `string`                      | Chaîne de version optionnelle                                 |
| `options.tools`   | `Array<SdkMcpToolDefinition>` | Tableau de définitions d'outils créées avec [`tool()`](#tool) |

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

| Propriété            | Type                    | Description                                                                                                                                                                                                                                                                                                  |
| :------------------- | :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | Rôle du message                                                                                                                                                                                                                                                                                              |
| `uuid`               | `string`                | Identifiant de message unique                                                                                                                                                                                                                                                                                |
| `session_id`         | `string`                | Session à laquelle ce message appartient                                                                                                                                                                                                                                                                     |
| `message`            | `unknown`               | Charge utile de message brute de la transcription                                                                                                                                                                                                                                                            |
| `parent_tool_use_id` | `string \| null`        | Pour les messages de sous-agent, le `tool_use_id` de l'appel d'outil `Agent` qui l'a généré. `null` pour les messages de session principale et les sessions plus anciennes                                                                                                                                   |
| `parent_agent_id`    | `string \| null`        | Pour les messages d'un [sous-agent imbriqué](/docs/fr/sub-agents#spawn-nested-subagents), le `agentId` du sous-agent qui l'a généré. `null` pour les messages de session principale, les messages des sous-agents de niveau supérieur et les sessions plus anciennes. Nécessite Claude Code v2.1.202 ou ultérieur |

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
  Cette fonction est en version alpha et son API peut changer avant la stabilisation. Elle lit les sources MDM, y compris la liste de propriétés macOS et Windows HKLM/HKCU, pour la parité avec le démarrage de l'interface CLI, mais n'exécute pas le sous-processus `policyHelper` configuré par l'administrateur. Le champ `permissions.defaultMode` est retourné tel quel de tous les niveaux, y compris les paramètres de projet. Le filtre de confiance que l'interface CLI applique avant d'honorer les modes de permission croissants n'est pas appliqué.
</Note>

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  Paramètres
</h4>

`resolveSettings()` accepte un seul objet d'options. Tous les champs sont optionnels.

| Paramètre                       | Type                                  | Par défaut         | Description                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------------------ | :------------------------------------ | :----------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.cwd`                   | `string`                              | `process.cwd()`    | Répertoire pour résoudre les paramètres de projet et locaux par rapport à                                                                                                                                                                                                                                                                                                                                                        |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | Toutes les sources | Quelles sources du système de fichiers charger. Passez `[]` pour ignorer les paramètres utilisateur, projet et locaux. Les paramètres de politique gérée se chargent dans tous les cas. Les paramètres gérés par le serveur sont pris à partir de `serverManagedSettings` quand l'hôte les transmet, ou lus à partir du cache sur disque de l'interface CLI sinon ; l'instantané ne les récupère pas à partir du réseau          |
| `options.managedSettings`       | `Settings`                            | `undefined`        | Paramètres de politique restrictive fournis par l'hôte d'intégration. Supprimés par défaut quand un niveau géré déployé par l'administrateur est présent ; fusionnés sous ce niveau quand [`parentSettingsBehavior`](/docs/fr/settings#available-settings) est `"merge"`. Les clés non restrictives telles que `model` sont silencieusement supprimées pour que cette option puisse renforcer la politique gérée mais pas l'assouplir |
| `options.serverManagedSettings` | `Settings`                            | `undefined`        | Charge utile de paramètres gérés par le serveur depuis `/api/claude_code/settings`. Les clés non restrictives passent sans filtre                                                                                                                                                                                                                                                                                                |

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

L'exemple ci-dessous résout les paramètres pour un répertoire de projet et imprime la source qui contrôle la période de nettoyage.

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

| Propriété                         | Type                                                                                                     | Par défaut                                              | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------- | :------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abortController`                 | `AbortController`                                                                                        | `new AbortController()`                                 | Contrôleur pour annuler les opérations                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `additionalDirectories`           | `string[]`                                                                                               | `[]`                                                    | Répertoires supplémentaires auxquels Claude peut accéder                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `agent`                           | `string`                                                                                                 | `undefined`                                             | Nom de l'agent pour le thread principal. L'agent doit être défini dans l'option `agents` ou dans les paramètres                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                  | `undefined`                                             | Définir programmatiquement les sous-agents                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `agentProgressSummaries`          | `boolean`                                                                                                | `false`                                                 | Quand `true`, générer des résumés de progression d'une ligne pour les sous-agents et les transférer sur les événements [`task_progress`](#sdktaskprogressmessage) via le champ `summary`. S'applique aux sous-agents de premier plan et d'arrière-plan                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                | `false`                                                 | Activer le contournement des permissions. Requis lors de l'utilisation de `permissionMode: 'bypassPermissions'`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `allowedTools`                    | `string[]`                                                                                               | `[]`                                                    | Outils à approuver automatiquement sans demander. Cela ne restreint pas Claude à seulement ces outils ; les outils non répertoriés passent à `permissionMode` et `canUseTool`. Utilisez `disallowedTools` pour bloquer les outils. Voir [Permissions](/docs/fr/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                   |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                | `[]`                                                    | Activer les fonctionnalités bêta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                              | `undefined`                                             | Fonction de permission personnalisée, invoquée uniquement quand le [flux de permission](/docs/fr/agent-sdk/permissions#how-permissions-are-evaluated) se termine par une invite. Non invoquée pour les appels pré-approuvés par `allowedTools`, les règles d'autorisation, ou `permissionMode`. `AskUserQuestion`, les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), et les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) l'atteignent même si vous les avez autorisés ; en mode `dontAsk` ils sont refusés à la place. Voir [`CanUseTool`](#canusetool) pour les détails |
| `continue`                        | `boolean`                                                                                                | `false`                                                 | Continuer la conversation la plus récente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `cwd`                             | `string`                                                                                                 | `process.cwd()`                                         | Répertoire de travail actuel                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `debug`                           | `boolean`                                                                                                | `false`                                                 | Activer le mode débogage pour le processus Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `debugFile`                       | `string`                                                                                                 | `undefined`                                             | Écrire les journaux de débogage dans un chemin de fichier spécifique. Active implicitement le mode débogage                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `disallowedTools`                 | `string[]`                                                                                               | `[]`                                                    | Outils à refuser. Un nom simple tel que `"Bash"` supprime l'outil du contexte de Claude. Une règle délimitée telle que `"Bash(rm *)"` laisse l'outil disponible et refuse les appels correspondants dans chaque mode de permission, y compris `bypassPermissions`. Voir [Permissions](/docs/fr/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                   |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                        | Par défaut du modèle                                    | Contrôle l'effort que Claude met dans sa réponse. Fonctionne avec la réflexion adaptative pour guider la profondeur de réflexion. Voir [ajuster le niveau d'effort](/docs/fr/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `enableFileCheckpointing`         | `boolean`                                                                                                | `false`                                                 | Activer le suivi des modifications de fichiers pour le rembobinage. Voir [Sauvegarde de fichiers](/docs/fr/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `env`                             | `Record<string, string \| undefined>`                                                                    | `process.env`                                           | Variables d'environnement. Quand défini, cela remplace l'environnement du sous-processus au lieu de fusionner avec `process.env`, donc passez `{ ...process.env, YOUR_VAR: 'value' }` pour conserver les variables héritées comme `PATH`. Voir [Gérer les réponses API lentes ou bloquées](#handle-slow-or-stalled-api-responses) pour un exemple de ce modèle, et [Variables d'environnement](/docs/fr/env-vars) pour les variables que la CLI sous-jacente lit. Définissez `CLAUDE_AGENT_SDK_CLIENT_APP` pour identifier votre application dans l'en-tête User-Agent                                                                                                                       |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                              | Détection automatique                                   | Runtime JavaScript à utiliser                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `executableArgs`                  | `string[]`                                                                                               | `[]`                                                    | Arguments à passer à l'exécutable                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `extraArgs`                       | `Record<string, string \| null>`                                                                         | `{}`                                                    | Arguments supplémentaires                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `fallbackModel`                   | `string`                                                                                                 | `undefined`                                             | Modèle à utiliser si le principal échoue                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `forkSession`                     | `boolean`                                                                                                | `false`                                                 | Lors de la reprise avec `resume`, bifurquer vers un nouvel ID de session au lieu de continuer la session d'origine                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `forwardSubagentText`             | `boolean`                                                                                                | `false`                                                 | Transférer les blocs de texte et de réflexion des sous-agents en tant que messages assistant et utilisateur avec `parent_tool_use_id` défini, pour que les consommateurs puissent afficher une transcription imbriquée. Par défaut, seuls les blocs `tool_use` et `tool_result` des sous-agents sont émis                                                                                                                                                                                                                                                                                                                                                                               |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`        | `{}`                                                    | Rappels de hook pour les événements                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `includeHookEvents`               | `boolean`                                                                                                | `false`                                                 | Inclure les événements du cycle de vie du hook pour chaque événement de hook dans le flux de messages en tant que [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage), et [`SDKHookResponseMessage`](#sdkhookresponsemessage). Les événements du cycle de vie pour les hooks `SessionStart` et `Setup` sont toujours inclus et n'ont pas besoin de cette option                                                                                                                                                                                                                                                                      |
| `includePartialMessages`          | `boolean`                                                                                                | `false`                                                 | Inclure les événements de message partiel                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `loadTimeoutMs`                   | `number`                                                                                                 | `60000`                                                 | *Alpha.* Délai d'expiration en millisecondes pour chaque appel `sessionStore.load()` et `sessionStore.listSubkeys()` lors de la matérialisation de la reprise. Si l'adaptateur ne se règle pas dans cette fenêtre, la requête échoue au lieu de rester bloquée. Ignoré quand `sessionStore` n'est pas défini                                                                                                                                                                                                                                                                                                                                                                            |
| `managedSettings`                 | `Settings`                                                                                               | `undefined`                                             | Paramètres de niveau politique fournis par le processus parent qui génère. Supprimés quand un niveau de paramètres gérés contrôlé par l'informatique existe déjà sur la machine, sauf si cet administrateur accepte avec `parentSettingsBehavior: 'merge'`. Filtrés aux clés restrictives uniquement                                                                                                                                                                                                                                                                                                                                                                                    |
| `maxBudgetUsd`                    | `number`                                                                                                 | `undefined`                                             | Arrêter la requête quand l'estimation du coût côté client atteint cette valeur en USD. Comparé à la même estimation que `total_cost_usd` ; voir [Suivi des coûts et de l'utilisation](/docs/fr/agent-sdk/cost-tracking) pour les avertissements de précision                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `maxThinkingTokens`               | `number`                                                                                                 | `undefined`                                             | *Déprécié :* Utilisez `thinking` à la place. Tokens maximum pour le processus de réflexion                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `maxTurns`                        | `number`                                                                                                 | `undefined`                                             | Tours agentiques maximum (allers-retours d'utilisation d'outils)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                  | `{}`                                                    | Configurations de serveur MCP                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `model`                           | `string`                                                                                                 | Par défaut de CLI                                       | Alias de modèle Claude ou nom de modèle complet. Voir [valeurs acceptées et ID spécifiques au fournisseur](/docs/fr/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`          | `undefined`                                             | Rappel pour gérer les demandes d'élicitation MCP. Appelé quand un serveur MCP demande une entrée utilisateur et aucun hook ne la gère en premier. Quand non fourni, les demandes d'élicitation non gérées sont automatiquement refusées                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                            | `undefined`                                             | Définir le format de sortie pour les résultats de l'agent. Voir [Sorties structurées](/docs/fr/agent-sdk/structured-outputs) pour les détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `outputStyle`                     | `string`                                                                                                 | `undefined`                                             | Pas un champ `Options`. Définissez `outputStyle` dans l'objet [`settings`](/docs/fr/settings) en ligne ou un fichier de paramètres à la place. Voir [Activer un style de sortie](/docs/fr/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                 | Résolu automatiquement à partir du binaire natif groupé | Chemin vers l'exécutable Claude Code. Nécessaire uniquement si les dépendances optionnelles ont été ignorées lors de l'installation ou si votre plateforme ne figure pas dans l'ensemble pris en charge                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                      | `'default'`                                             | Mode de permission pour la session                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `permissionPromptToolName`        | `string`                                                                                                 | `undefined`                                             | Nom de l'outil MCP pour les invites de permission                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `persistSession`                  | `boolean`                                                                                                | `true`                                                  | Quand `false`, désactive la persistance de session sur disque. Les sessions ne peuvent pas être reprises plus tard                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `planModeInstructions`            | `string`                                                                                                 | `undefined`                                             | Instructions de flux de travail personnalisées pour le mode plan. Quand `permissionMode` est `'plan'`, cette chaîne remplace le corps du flux de travail du mode plan par défaut. La CLI l'enveloppe toujours avec le préambule d'application en lecture seule et le pied de page du protocole ExitPlanMode                                                                                                                                                                                                                                                                                                                                                                             |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                | `[]`                                                    | Charger les plugins personnalisés à partir de chemins locaux. Voir [Plugins](/docs/fr/agent-sdk/plugins) pour les détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `promptSuggestions`               | `boolean`                                                                                                | `false`                                                 | Activer les suggestions d'invite. Émet un message `prompt_suggestion` après chaque tour avec une invite utilisateur suivante prédite                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `resume`                          | `string`                                                                                                 | `undefined`                                             | ID de session à reprendre                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `resumeSessionAt`                 | `string`                                                                                                 | `undefined`                                             | Reprendre la session à un UUID de message spécifique                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                    | `undefined`                                             | Configurer le comportement du sandbox par programmation. Voir [Paramètres du sandbox](#sandboxsettings) pour les détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `sessionId`                       | `string`                                                                                                 | Généré automatiquement                                  | Utiliser un UUID spécifique pour la session au lieu d'en générer un automatiquement                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `sessionStore`                    | [`SessionStore`](/docs/fr/agent-sdk/session-storage#the-sessionstore-interface)                               | `undefined`                                             | Refléter les transcriptions de session vers un backend externe pour que n'importe quel hôte puisse les reprendre. Voir [Persister les sessions vers un stockage externe](/docs/fr/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                   | `'batched'`                                             | *Alpha.* Mode de vidage pour `sessionStore`. Ignoré quand `sessionStore` n'est pas défini                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `settings`                        | `string \| Settings`                                                                                     | `undefined`                                             | Objet [paramètres](/docs/fr/settings) en ligne ou chemin vers un fichier de paramètres. Remplit la couche de paramètres d'indicateur dans l'[ordre de précédence](/docs/fr/settings#settings-precedence). Modifiez à l'exécution avec [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                    | Paramètres par défaut de CLI (toutes les sources)       | Contrôler les paramètres du système de fichiers à charger. Passez `[]` pour désactiver les paramètres utilisateur, projet et locaux. Les paramètres de politique gérée se chargent indépendamment ; les paramètres gérés par le serveur sont récupérés quand la session s'authentifie avec une credential d'organisation sur une [configuration éligible](/docs/fr/server-managed-settings#platform-availability). Voir [Utiliser les fonctionnalités Claude Code](/docs/fr/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                                  |
| `skills`                          | `string[] \| 'all'`                                                                                      | `undefined`                                             | Compétences disponibles pour la session. Passez `'all'` pour activer chaque compétence découverte, ou une liste de noms de compétences. Quand défini, le SDK ajoute l'outil Skill à `allowedTools` automatiquement. Si vous passez également `tools`, incluez `'Skill'` dans cette liste. Voir [Compétences](/docs/fr/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                      |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                              | `undefined`                                             | Fonction personnalisée pour générer le processus Claude Code. Utilisez pour exécuter Claude Code dans des VM, des conteneurs ou des environnements distants                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `stderr`                          | `(data: string) => void`                                                                                 | `undefined`                                             | Rappel pour la sortie stderr                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `strictMcpConfig`                 | `boolean`                                                                                                | `false`                                                 | Utiliser uniquement les serveurs passés dans `mcpServers` et ignorer le projet `.mcp.json`, les paramètres utilisateur, les serveurs MCP fournis par les plugins, et les [connecteurs claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `systemPrompt`                    | `string \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean }` | `undefined` (invite minimale)                           | Configuration de l'invite système. Passez une chaîne pour une invite personnalisée, ou `{ type: 'preset', preset: 'claude_code' }` pour utiliser l'invite système de Claude Code. Lors de l'utilisation de la forme d'objet prédéfini, ajoutez `append` pour l'étendre avec des instructions supplémentaires, et définissez `excludeDynamicSections: true` pour déplacer le contexte par session dans le premier message utilisateur pour une [meilleure réutilisation du cache d'invite sur les machines](/docs/fr/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)                                                                                     |
| `taskBudget`                      | `{ total: number }`                                                                                      | `undefined`                                             | *Alpha.* Budget de tâche côté API en tokens. Quand défini, le modèle est informé de son budget de tokens restant pour qu'il puisse adapter l'utilisation des outils et terminer avant la limite                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                      | `{ type: 'adaptive' }` pour les modèles pris en charge  | Contrôle le comportement de réflexion/raisonnement de Claude. Voir [`ThinkingConfig`](#thinkingconfig) pour les options                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `title`                           | `string`                                                                                                 | `undefined`                                             | Titre d'affichage pour la session. Lors de la reprise via `resume` ou `continue`, le titre persistant de la session reprise a la priorité ; utilisez [`renameSession()`](#renamesession) pour renommer une session existante                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `toolAliases`                     | `Record<string, string>`                                                                                 | `undefined`                                             | Mapper les noms d'outils intégrés aux noms d'outils MCP pour que Claude appelle votre implémentation MCP à la place de l'intégrée. Par exemple, `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                              | `undefined`                                             | Configuration pour le comportement des outils intégrés. Voir [`ToolConfig`](#toolconfig) pour les détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                  | `undefined`                                             | Configuration des outils. Passez un tableau de noms d'outils ou utilisez le prédéfini pour obtenir les outils par défaut de Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |

<h4 id="handle-slow-or-stalled-api-responses">
  Gérer les réponses API lentes ou bloquées
</h4>

Le sous-processus CLI lit plusieurs variables d'environnement qui contrôlent les délais d'expiration de l'API et la détection de blocage. Transmettez-les via l'option `env` :

```typescript theme={null}
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
* `CLAUDE_CODE_MAX_RETRIES` : tentatives API maximales. Par défaut `10`, limité à `15`. Chaque tentative obtient sa propre fenêtre `API_TIMEOUT_MS`, donc le pire cas de temps mural est approximativement `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` plus le backoff. Pour les exécutions sans surveillance qui doivent attendre des pannes plus longues, définissez `CLAUDE_CODE_RETRY_WATCHDOG=1` : il réessaye les erreurs de capacité indéfiniment, et à partir de Claude Code v2.1.199 augmente la valeur par défaut pour les autres erreurs transitoires à `300` et supprime le plafond sur cette variable.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` : chien de garde de blocage pour les sous-agents lancés avec `run_in_background`. Par défaut `600000`. Réinitialise à chaque événement de flux ; en cas de blocage, il abandonne le sous-agent, marque la tâche comme échouée et expose l'erreur au parent avec tout résultat partiel. Ne s'applique pas aux sous-agents synchrones.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` avec `CLAUDE_STREAM_IDLE_TIMEOUT_MS` : abandonne la requête quand les en-têtes sont arrivés mais que le corps de la réponse cesse de diffuser. Le chien de garde est activé par défaut pour tous les fournisseurs ; définissez `CLAUDE_ENABLE_STREAM_WATCHDOG=0` pour le désactiver. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` par défaut à `300000` et est limité à ce minimum. La requête abandonnée passe par le chemin de tentative normal.

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
  applyFlagSettings(settings: { [K in keyof Settings]?: Settings[K] | null }): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  close(): void;
}
```

<h4 id="methods">
  Méthodes
</h4>

| Méthode                                | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | Interrompt la requête. Disponible uniquement en mode d'entrée en diffusion. Quand la CLI annonce la capacité `interrupt_receipt_v1` dans [`SDKSystemMessage.capabilities`](#sdksystemmessage), se résout avec une [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) listant les messages en attente qui survivent à l'interruption. Se résout à `undefined` sur les CLI antérieures à v2.1.205                                                                                                                                |
| `rewindFiles(userMessageId, options?)` | Restaure les fichiers à leur état au message utilisateur spécifié. Passez `{ dryRun: true }` pour prévisualiser les modifications. Nécessite `enableFileCheckpointing: true`. Voir [Sauvegarde de fichiers](/docs/fr/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                             |
| `setPermissionMode()`                  | Change le mode de permission (disponible uniquement en mode d'entrée en diffusion)                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `setModel()`                           | Change le modèle (disponible uniquement en mode d'entrée en diffusion)                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `setMaxThinkingTokens()`               | *Déprécié :* Utilisez l'option `thinking` à la place. Change les tokens de réflexion maximum. Passer `null` réinitialise la réflexion à la valeur par défaut de la session : un remplacement en milieu de session est effacé, et la réflexion reste désactivée pour les sessions qui l'ont désactivée                                                                                                                                                                                                                                     |
| `applyFlagSettings(settings)`          | Fusionne les paramètres dans la couche de paramètres d'indicateur de la session à l'exécution (disponible uniquement en mode d'entrée en diffusion). Voir [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                     |
| `initializationResult()`               | Retourne le résultat d'initialisation complet incluant les commandes prises en charge, les modèles, les informations de compte et la configuration du style de sortie                                                                                                                                                                                                                                                                                                                                                                     |
| `reinitialize()`                       | Renvoie la demande de contrôle `initialize` au CLI en cours d'exécution et retourne un résultat frais au lieu du résultat de première connexion mis en cache. Utilisez-le après une interruption de transport, comme se reconnecter à une session après une déconnexion, pour que les demandes de permission en attente atteignent à nouveau votre rappel `canUseTool`. Rendez le rappel idempotent par ID de requête, car une requête dont la réponse a été perdue est distribuée à nouveau. Nécessite Claude Code v2.1.195 ou ultérieur |
| `supportedCommands()`                  | Retourne les commandes slash disponibles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `supportedModels()`                    | Retourne les modèles disponibles avec les informations d'affichage                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `supportedAgents()`                    | Retourne les sous-agents disponibles en tant que [`AgentInfo`](#agentinfo)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `mcpServerStatus()`                    | Retourne l'état des serveurs MCP connectés                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `accountInfo()`                        | Retourne les informations de compte                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `reconnectMcpServer(serverName)`       | Reconnecter un serveur MCP par nom                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `toggleMcpServer(serverName, enabled)` | Activer ou désactiver un serveur MCP par nom                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `setMcpServers(servers)`               | Remplacer dynamiquement l'ensemble des serveurs MCP pour cette session. Retourne des informations sur les serveurs ajoutés, supprimés et les erreurs                                                                                                                                                                                                                                                                                                                                                                                      |
| `streamInput(stream)`                  | Diffuser les messages d'entrée vers la requête pour les conversations multi-tours                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `stopTask(taskId)`                     | Arrêter une tâche de fond en cours d'exécution par ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `close()`                              | Fermer la requête et terminer le processus sous-jacent. Termine de force la requête et nettoie toutes les ressources                                                                                                                                                                                                                                                                                                                                                                                                                      |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

Change n'importe quel [paramètre](/docs/fr/settings) sur une session en cours d'exécution sans redémarrer la requête. Utilisez-le quand un paramètre qui n'a pas de setter dédié doit changer en milieu de session, comme resserrer `permissions` après que l'agent ait lu une entrée non fiable. `setModel()` et `setPermissionMode()` sont des setters dédiés pour ces deux clés ; `applyFlagSettings()` est la forme générale qui accepte n'importe quel sous-ensemble des clés de paramètres, et passer `model` ici se comporte de la même manière que `setModel()`.

Seules certaines clés prennent effet en milieu de session :

* **Appliquées au tour suivant** : `model`, `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. Basculer `agent` applique également le remplacement de modèle, les hooks et l'invite système de cet agent au tour suivant.
* **Aucun effet en milieu de session** : les options d'invite système. Celles-ci sont résolues une fois au démarrage, donc la session en cours d'exécution conserve la valeur d'origine même si l'appel réussit. Pour les modifier, démarrez une nouvelle session.

`effortLevel` accepte un nom de [niveau d'effort](/docs/fr/model-config#adjust-effort-level). Il accepte également `"ultracode"`, qui exécute la session au niveau d'effort `xhigh` et active [ultracode](/docs/fr/workflows#let-claude-decide-with-ultracode). Le type `Settings` déclare `effortLevel` sans cette valeur, donc passez l'équivalent `{ ultracode: true }` en TypeScript. La valeur `ultracode` nécessite Claude Code v2.1.203 ou ultérieur et n'est acceptée que par `applyFlagSettings()`, pas par la clé `effortLevel` dans un fichier de paramètres.

Les valeurs sont écrites dans la couche de paramètres d'indicateur, la même couche que l'option `settings` en ligne de `query()` remplit au démarrage. Les paramètres d'indicateur se situent près du haut de l'[ordre de précédence des paramètres](/docs/fr/settings#settings-precedence) : ils remplacent les paramètres utilisateur, projet et locaux, et seuls les paramètres de politique gérée peuvent les remplacer. C'est le même niveau que la [section de précédence sur la page](#settings-precedence) appelle les options programmatiques.

Les appels successifs fusionnent superficiellement les clés de niveau supérieur. Un deuxième appel avec `{ permissions: {...} }` remplace l'objet `permissions` entier de l'appel précédent plutôt que de le fusionner profondément. Pour effacer une clé de la couche d'indicateur et revenir aux sources de précédence inférieure, passez `null` pour cette clé. Passer `undefined` n'a aucun effet car la sérialisation JSON le supprime.

Disponible uniquement en mode d'entrée en diffusion, la même contrainte que `setModel()` et `setPermissionMode()`.

L'exemple ci-dessous bascule le modèle actif en milieu de session, puis efface le remplacement pour que le modèle revienne à ce que les paramètres utilisateur ou projet spécifient.

```typescript theme={null}
const q = query({ prompt: messageStream });

// Remplacer le modèle pour le reste de la session
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Plus tard : effacer le remplacement et revenir aux paramètres de précédence inférieure
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` est TypeScript uniquement. Le SDK Python n'expose pas de méthode équivalente.
</Note>

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
};
```

Quand un client envoie `initialize` à une session qui est déjà en cours d'exécution, le wrapper de réponse de contrôle porte également un tableau `pending_permission_requests` optionnel. Le champ se trouve sur le wrapper de réponse lui-même, pas dans la charge utile `SDKControlInitializeResponse` ci-dessus. Chaque entrée est un message `control_request` complet avec la même forme `{ type: "control_request", request_id, request }` que la session diffuse pour les demandes de permission lors de l'exécution.

Ce sont des demandes qui ont été émises avant que le client se connecte et attendent toujours une réponse. Le SDK lit le tableau pour vous et distribue chaque entrée à votre rappel [`canUseTool`](#canusetool), la même redistribution que [`reinitialize()`](#query-object) déclenche après une interruption de transport. Gérez les ID de requête répétés de manière idempotente, car une entrée peut répéter une requête que le rappel a déjà reçue avant que la connexion ne soit interrompue.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

Le reçu d'interruption : la valeur que [`interrupt()`](#query-object) se résout avec sur une CLI qui annonce la capacité `interrupt_receipt_v1` dans [`SDKSystemMessage.capabilities`](#sdksystemmessage). Nécessite Claude Code v2.1.205 ou ultérieur. Les CLI antérieures répondent à l'interruption avec une charge utile de succès vide, donc `interrupt()` se résout à `undefined`.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
};
```

`still_queued` liste les UUID des messages utilisateur qui survivent à l'interruption : messages toujours en attente, plus tout lot déjà retiré de la file d'attente pour le tour suivant mais pas encore accessible par l'abandon. Chacun s'exécute comme son propre tour après l'interruption sauf si vous l'annulez d'abord. Utilisez le reçu pour décider si vous devez renvoyer quelque chose ; renvoyer un message qui est déjà listé produit un tour en double.

Interprétez la liste avec ces avertissements :

* Seuls les messages qui ont été mis en attente avec un UUID apparaissent. Un tableau vide ne signifie pas que rien d'autre ne s'exécutera.
* Seuls les messages du thread principal sont listés. Les messages adressés à un sous-agent sont hors de portée.
* La liste peut inclure des UUID que votre client n'a jamais envoyés, comme les déclencheurs de [tâche programmée](/docs/fr/scheduled-tasks). Ignorez les UUID que vous ne reconnaissez pas au lieu de les traiter comme une erreur.

Le reçu est un instantané pris au moment où l'interruption est traitée, et sur une interruption propre, il arrive avant le [`SDKResultMessage`](#sdkresultmessage) du tour interrompu. Lisez le reçu plutôt que d'inspecter la file d'attente après ce résultat : la boucle démarre immédiatement le tour en attente suivant, donc la file d'attente que vous inspectez après le résultat a déjà changé.

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
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| Champ                                 | Requis | Description                                                                                                                                                                                                                                                              |
| :------------------------------------ | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`                         | Oui    | Description en langage naturel de quand utiliser cet agent                                                                                                                                                                                                               |
| `tools`                               | Non    | Tableau de noms d'outils autorisés. S'il est omis, hérite tous les outils du parent. Pour précharger les compétences dans le contexte de l'agent, utilisez le champ `skills` plutôt que de lister `'Skill'` ici                                                          |
| `disallowedTools`                     | Non    | Tableau de noms d'outils à explicitement interdire pour cet agent. Les modèles au niveau du serveur MCP sont également acceptés : `mcp__server` ou `mcp__server__*` supprime chaque outil de ce serveur, et `mcp__*` supprime chaque outil MCP de n'importe quel serveur |
| `prompt`                              | Oui    | L'invite système de l'agent                                                                                                                                                                                                                                              |
| `model`                               | Non    | Remplacement de modèle pour cet agent. Accepte un alias tel que `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, ou un ID de modèle complet. S'il est omis ou `'inherit'`, utilise le modèle principal                                                           |
| `mcpServers`                          | Non    | Spécifications de serveur MCP pour cet agent                                                                                                                                                                                                                             |
| `skills`                              | Non    | Tableau de noms de compétences à précharger dans le contexte de l'agent                                                                                                                                                                                                  |
| `initialPrompt`                       | Non    | Auto-soumis comme le premier tour utilisateur quand cet agent s'exécute en tant qu'agent du thread principal                                                                                                                                                             |
| `maxTurns`                            | Non    | Nombre maximum de tours agentiques (allers-retours API) avant arrêt                                                                                                                                                                                                      |
| `background`                          | Non    | Exécuter cet agent en tant que tâche de fond non-bloquante quand invoqué                                                                                                                                                                                                 |
| `memory`                              | Non    | Source de mémoire pour cet agent : `'user'`, `'project'`, ou `'local'`                                                                                                                                                                                                   |
| `effort`                              | Non    | Niveau d'effort de raisonnement pour cet agent. Accepte un niveau nommé ou un entier                                                                                                                                                                                     |
| `permissionMode`                      | Non    | Mode de permission pour l'exécution des outils dans cet agent. Voir [`PermissionMode`](#permissionmode)                                                                                                                                                                  |
| `criticalSystemReminder_EXPERIMENTAL` | Non    | Expérimental : Rappel critique ajouté à l'invite système                                                                                                                                                                                                                 |

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

| Valeur      | Description                                         | Emplacement                   |
| :---------- | :-------------------------------------------------- | :---------------------------- |
| `'user'`    | Paramètres utilisateur globaux                      | `~/.claude/settings.json`     |
| `'project'` | Paramètres de projet partagés (contrôle de version) | `.claude/settings.json`       |
| `'local'`   | Paramètres de projet locaux (gitignorés)            | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Comportement par défaut
</h4>

Quand `settingSources` est omis ou `undefined`, `query()` charge les mêmes paramètres du système de fichiers que la CLI Claude Code : utilisateur, projet et local. Les paramètres de politique gérée sont chargés dans tous les cas ; les paramètres gérés par le serveur sont récupérés quand la session s'authentifie avec une credential d'organisation sur une [configuration éligible](/docs/fr/server-managed-settings#platform-availability). Voir [Ce que settingSources ne contrôle pas](/docs/fr/agent-sdk/claude-code-features#what-settingsources-does-not-control) pour les entrées qui sont lues indépendamment de cette option, et comment les désactiver.

<h4 id="why-use-settingsources">
  Pourquoi utiliser settingSources
</h4>

**Désactiver les paramètres du système de fichiers :**

```typescript theme={null}
// Ne pas charger les paramètres utilisateur, projet ou locaux à partir du disque
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**Charger tous les paramètres du système de fichiers explicitement :**

```typescript theme={null}
const result = query({
  prompt: "Analyze this code",
  options: {
    settingSources: ["user", "project", "local"] // Charger tous les paramètres
  }
});
```

**Charger uniquement des sources de paramètres spécifiques :**

```typescript theme={null}
// Charger uniquement les paramètres de projet, ignorer utilisateur et local
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Uniquement .claude/settings.json
  }
});
```

**Environnements de test et CI :**

```typescript theme={null}
// Assurer un comportement cohérent en CI en excluant les paramètres locaux
const result = query({
  prompt: "Run tests",
  options: {
    settingSources: ["project"], // Uniquement les paramètres partagés par l'équipe
    permissionMode: "bypassPermissions"
  }
});
```

**Applications SDK uniquement :**

```typescript theme={null}
// Définir tout par programmation.
// Passez [] pour refuser les sources de paramètres du système de fichiers.
const result = query({
  prompt: "Review this PR",
  options: {
    settingSources: [],
    agents: {
      /* ... */
    },
    mcpServers: {
      /* ... */
    },
    allowedTools: ["Read", "Grep", "Glob"]
  }
});
```

**Chargement des instructions de projet CLAUDE.md :**

```typescript theme={null}
// Charger les paramètres de projet pour inclure les fichiers CLAUDE.md
const result = query({
  prompt: "Add a new feature following project conventions",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code" // Utiliser l'invite système de Claude Code
    },
    settingSources: ["project"], // Charge CLAUDE.md du répertoire de projet
    allowedTools: ["Read", "Write", "Edit"]
  }
});
```

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
  | "auto"; // Utiliser un classificateur de modèle pour approuver ou refuser chaque appel d'outil
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Type de fonction de permission personnalisée pour contrôler l'utilisation des outils.

La fonction est le remplacement SDK pour l'invite de permission interactive : elle est invoquée uniquement quand le [flux d'évaluation de permission](/docs/fr/agent-sdk/permissions#how-permissions-are-evaluated) se termine par une invite. Les appels d'outils déjà approuvés par une entrée `allowedTools`, une règle d'autorisation de paramètres, ou le mode de permission, comme `acceptEdits` ou `bypassPermissions`, ne l'invoquent jamais. Pour contrôler chaque appel d'outil, utilisez un [hook `PreToolUse`](/docs/fr/agent-sdk/hooks) à la place.

`AskUserQuestion`, les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool), et les outils connecteur [que votre organisation a définis sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) l'atteignent même quand une règle d'autorisation correspond. En mode `dontAsk` ces appels sont refusés à la place, sans l'invoquer.

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
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

| Champ                           | Type                   | Description                                                                                                                                                                                                   |
| :------------------------------ | :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | Accepte le champ `preview` sur les options [`AskUserQuestion`](/docs/fr/agent-sdk/user-input#question-format) et définit son format de contenu. Lorsqu'il n'est pas défini, Claude n'émet pas de prévisualisations |

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

Message de réponse assistant.

```typescript theme={null}
type SDKAssistantMessage = {
  type: "assistant";
  uuid: UUID;
  session_id: string;
  message: BetaMessage; // Du SDK Anthropic
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
};
```

Le champ `message` est un [`BetaMessage`](https://platform.claude.com/docs/fr/api/messages/create) du SDK Anthropic. Il inclut des champs comme `id`, `content`, `model`, `stop_reason` et `usage`.

`SDKAssistantMessageError` est l'un de : `'authentication_failed'`, `'oauth_org_not_allowed'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'` ou `'unknown'`. `'model_not_found'` signifie que le modèle sélectionné n'existe pas ou n'est pas disponible pour votre compte ou déploiement. `'overloaded'` signifie que l'API a retourné un 529 parce que le serveur est à pleine capacité, par opposition à `'rate_limit'`, qui est un 429 contre votre quota.

<h3 id="sdkusermessage">
  `SDKUserMessage`
</h3>

Message d'entrée utilisateur.

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // Du SDK Anthropic
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
};
```

Définissez `shouldQuery` sur `false` pour ajouter le message à la transcription sans déclencher un tour assistant. Le message est conservé et fusionné dans le prochain message utilisateur qui déclenche un tour. Utilisez ceci pour injecter du contexte, comme la sortie d'une commande que vous avez exécutée en dehors de la bande, sans dépenser un appel de modèle.

Sur un message qui porte un bloc `tool_result`, `tool_use_result` est l'objet de sortie structuré de l'outil plutôt que le texte envoyé au modèle. Sa forme dépend de l'outil nommé par le bloc `tool_use` correspondant, donc le champ est typé `unknown` ; les formes intégrées sont listées sous [Types de sortie d'outil](#tool-output-types).

Pour l'outil `Agent`, `tool_use_result` est [`AgentOutput`](#agent-2). Sur un résultat `completed`, `content` contient le rapport du sous-agent sans l'ID d'agent et la remorque d'utilisation que Claude Code ajoute au texte `tool_result`, donc rendez à partir de `tool_use_result` au lieu d'analyser ce texte.

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

Un tour utilisateur injecté de l'extérieur de la session, dont le [`origin`](#sdkmessageorigin) est `peer` ou `channel`, atteint le flux en tant que relecture, qu'il ait été livré pendant un tour actif ou ait démarré un nouveau tour alors que la session était inactive. Avant v2.1.207, un tour injecté livré alors que la session était inactive ne produisait aucun message sur le flux et n'apparaissait que lorsque vous relisiez la transcription.

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
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
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
      errors: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      origin?: SDKMessageOrigin;
    };
```

Plusieurs champs du résultat portent des détails diagnostiques au-delà du `subtype` :

* `api_error_status` : le code de statut HTTP de l'erreur API qui a terminé la conversation. Absent ou `null` quand le tour s'est terminé sans erreur API.
* `ttft_ms` : temps jusqu'au premier jeton en millisecondes, mesuré quand le premier message assistant complet arrive. Présent uniquement sur le bras de succès.
* `ttft_stream_ms` : temps en millisecondes jusqu'au premier événement de flux `message_start`, quand le flux de réponse s'ouvre. Inférieur à `ttft_ms` ; l'écart entre les deux est le temps passé à diffuser le premier message. Présent uniquement sur le bras de succès.
* `terminal_reason` : pourquoi la boucle s'est terminée. L'un de `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"` ou `"turn_setup_failed"`.
* `fast_mode_state` : l'un de `"on"`, `"off"` ou `"cooldown"`.

Le champ `origin` transmet le [`SDKMessageOrigin`](#sdkmessageorigin) du message utilisateur qui a déclenché ce résultat. Quand une tâche de fond se termine et que le SDK injecte un tour de suivi synthétique, le `SDKResultMessage` résultant porte `origin: { kind: "task-notification" }`. Vérifiez ce champ pour distinguer les résultats qui répondent à votre invite des résultats émis pour les suivis de tâches de fond, afin que vous puissiez acheminer ou supprimer ces derniers. Le champ est absent pour les résultats émis avant tout tour utilisateur, comme les erreurs de démarrage.

Quand un hook `PreToolUse` retourne `permissionDecision: "defer"`, le résultat a `stop_reason: "tool_deferred"` et `deferred_tool_use` porte l'`id`, le `name` et l'`input` de l'outil en attente. Lisez ce champ pour afficher la demande dans votre propre interface utilisateur, puis reprenez avec le même `session_id` pour continuer. Consultez [Différer un appel d'outil pour plus tard](/docs/fr/hooks#defer-a-tool-call-for-later) pour le trajet complet.

<h3 id="sdksystemmessage">
  `SDKSystemMessage`
</h3>

Message d'initialisation système.

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
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  capabilities?: string[];
};
```

Le tableau `capabilities` nomme les comportements de protocole que ce CLI implémente, afin que vous puissiez détecter les fonctionnalités au lieu de comparer les chaînes `claude_code_version`. C'est un ensemble ouvert : ignorez les valeurs que vous ne reconnaissez pas, et vérifiez la capacité spécifique dont vous dépendez. Le champ nécessite Claude Code v2.1.205 ou ultérieur et est absent sur les CLI antérieurs.

| Capacité               | Signification                                                                                                                                                                            |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1` | [`interrupt()`](#query-object) se résout avec une réception [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) nommant les messages en attente qui survivent à l'interruption |

<h3 id="sdkpartialassistantmessage">
  `SDKPartialAssistantMessage`
</h3>

Message partiel en diffusion (uniquement quand `includePartialMessages` est true). Le champ `parent_tool_use_id` est toujours `null` : les événements de flux sont émis pour la session principale uniquement. Pour l'attribution de sous-agent, utilisez les messages complets, qui portent `parent_tool_use_id`, ou activez [`forwardSubagentText`](#options) pour recevoir le texte et la réflexion du sous-agent en tant que messages complets.

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // Du SDK Anthropic
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // Temps jusqu'au premier jeton en ms, présent uniquement sur les événements message_start
};
```

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

Bannière de texte générique émise par la boucle. Porte les lignes d'état non-erreur, les retours de hook comme la raison de blocage d'un hook `UserPromptSubmit`, et la sortie de commande. Rendez `content` en texte brut au `level` donné.

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

Émis lors de l'arrêt gracieux du worker afin que les clients distants puissent montrer pourquoi le worker a disparu au lieu d'attendre l'expiration du heartbeat. La `reason` est une courte chaîne snake\_case définie par le CLI hôte, comme `"host_exit"` ou `"remote_control_disabled"`. Agissez sur ceci uniquement lors de la diffusion en direct. Une session reprise rejoue les instances passées de ce message, donc ignorez-les dans ce cas.

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

Événement de progression d'installation de plugin. Émis quand [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/fr/env-vars) est défini, pour que votre application Agent SDK puisse suivre l'installation du plugin de marketplace avant le premier tour. Les statuts `started` et `completed` encadrent l'installation globale. Les statuts `installed` et `failed` rapportent les marchés individuels et incluent `name`.

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

Événement de flux émis quand le système de permissions refuse automatiquement un appel d'outil sans invite interactive. Utilisez-le pour afficher le refus dans votre interface utilisateur au fur et à mesure, plutôt que d'observer uniquement le résultat d'outil `is_error` qui suit. Le chemin de demande interactive atteint votre application séparément via le callback [`canUseTool`](#canusetool). Les refus émis par un hook `PreToolUse` ne sont pas signalés via cet événement.

Cet événement nécessite Claude Code v2.1.136 ou ultérieur.

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

| Champ                  | Type     | Description                                                                                                                       |
| ---------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `tool_name`            | `string` | Nom de l'outil qui a été refusé                                                                                                   |
| `tool_use_id`          | `string` | ID du bloc `tool_use` auquel ce refus répond                                                                                      |
| `agent_id`             | `string` | ID du sous-agent quand l'appel refusé provient d'un sous-agent. Reflète le champ sur `can_use_tool` pour l'acheminement côté hôte |
| `decision_reason_type` | `string` | Discriminateur du composant qui a décidé, tel que `"rule"`, `"mode"`, `"classifier"` ou `"asyncAgent"`                            |
| `decision_reason`      | `string` | Raison lisible par l'homme du composant décideur, quand disponible                                                                |
| `message`              | `string` | Message de rejet retourné au modèle dans le `tool_result`                                                                         |

<h3 id="sdkpermissiondenial">
  `SDKPermissionDenial`
</h3>

Informations sur une utilisation d'outil refusée.

```typescript theme={null}
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: Record<string, unknown>;
};
```

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

Provenance d'un message de rôle utilisateur. Ceci apparaît comme `origin` sur [`SDKUserMessage`](#sdkusermessage) et est transmis au [`SDKResultMessage`](#sdkresultmessage) correspondant afin que vous puissiez dire ce qui a déclenché un tour donné.

```typescript theme={null}
type SDKMessageOrigin =
  | { kind: "human" }
  | { kind: "channel"; server: string }
  | {
      kind: "peer";
      from: string;
      name?: string;
      senderTaskId?: string;
      body?: string;
    }
  | { kind: "task-notification" }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" };
```

| `kind`              | Signification                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | Entrée directe de l'utilisateur final. Sur les messages utilisateur, une `origin` absente signifie également une entrée humaine.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `channel`           | Message arrivant sur un [canal](/docs/fr/channels). `server` est le nom du serveur MCP source.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `peer`              | Message d'un autre agent. Pour un [coéquipier](/docs/fr/agent-teams) en processus envoyant à `main` via `SendMessage`, `from` est le nom du coéquipier et `senderTaskId` est son ID de tâche. Pour un pair entre sessions comme un autre processus Claude Code local, `from` est l'adresse de l'expéditeur et `senderTaskId` est absent. }`name` et `body` nécessitent Claude Code v2.1.205 ou ultérieur. `name` est le nom d'affichage de l'expéditeur, normalisé par Claude Code : il supprime les points de code de contrôle, de format, de substitut Unicode, et de séparateur de ligne ou de paragraphe, puis tronque le résultat et le limite à 64 points de code avec des points de suspension. `body` est le corps du message décodé avec l'enveloppe de pair supprimée, octet-exact avec ce que le modèle voit. Pour un message de coéquipier, `body` est toujours présent ; pour un pair entre sessions, il est présent uniquement quand le tour est exactement une enveloppe de pair formée par Claude Code. Rendez `name` et `body` au lieu de réanalyser le texte du message. |
| `task-notification` | Tour synthétique injecté après la fin d'une tâche de fond. Voir [`SDKTaskNotificationMessage`](#sdktasknotificationmessage).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `coordinator`       | Message d'un coordinateur d'équipe dans une [équipe d'agents](/docs/fr/agent-teams).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `auto-continuation` | Tour synthétique injecté quand la session continue sans nouvelle entrée utilisateur, comme un résultat de commande qui déclenche une invite de suivi.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |

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
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PermissionRequest"
  | "Setup"
  | "TeammateIdle"
  | "TaskCompleted"
  | "ConfigChange"
  | "WorktreeCreate"
  | "WorktreeRemove"
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
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCompletedHookInput
  | ConfigChangeHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
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
};
```

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
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact";
  agent_type?: string;
  model?: string;
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

<h4 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h4>

```typescript theme={null}
type PermissionRequestHookInput = BaseHookInput & {
  hook_event_name: "PermissionRequest";
  tool_name: string;
  tool_input: unknown;
  permission_suggestions?: PermissionUpdate[];
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
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
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

Union de tous les types d'entrée d'outil, exportée depuis `@anthropic-ai/claude-agent-sdk`.

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | AskUserQuestionInput
  | BashInput
  | TaskOutputInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ReadMcpResourceInput
  | SubscribeMcpResourceInput
  | SubscribePollingInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | UnsubscribeMcpResourceInput
  | UnsubscribePollingInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**Nom de l'outil :** `Agent` (précédemment `Task`, qui est toujours accepté comme alias)

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan";
  isolation?: "worktree";
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

Exécute les commandes bash dans une session shell persistante avec délai d'expiration optionnel et exécution en arrière-plan.

<h3 id="monitor">
  Monitor
</h3>

**Nom de l'outil :** `Monitor`

```typescript theme={null}
type MonitorInput = {
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
  description: string;
  timeout_ms?: number;
  persistent?: boolean;
};
```

Exécute une source de fond et livre chaque événement à Claude pour qu'il puisse réagir sans interrogation : `command` exécute un script et émet un événement par ligne stdout, et `ws` ouvre une WebSocket et émet un événement par trame texte. Fournissez exactement l'un de `command` ou `ws`. La source `ws` nécessite Claude Code v2.1.195 ou version ultérieure.

Définissez `persistent: true` pour les montres de longueur de session telles que les queues de journal. Lorsque Monitor exécute une commande, il suit les mêmes règles de permission que Bash ; une montre WebSocket demande une approbation séparément. Voir la [référence de l'outil Monitor](/docs/fr/tools-reference#monitor-tool) pour le comportement et la disponibilité du fournisseur.

<h3 id="taskoutput">
  TaskOutput
</h3>

**Nom de l'outil :** `TaskOutput`

```typescript theme={null}
type TaskOutputInput = {
  task_id: string;
  block: boolean;
  timeout: number;
};
```

Récupère la sortie d'une tâche de fond en cours d'exécution ou terminée.

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
  args?: unknown;
  resumeFromRunId?: string;
};
```

Exécute un [flux de travail dynamique](/docs/fr/workflows) : un script qui orchestre de nombreux sous-agents en arrière-plan et retourne un résultat consolidé. L'outil `Workflow` est disponible dans Agent SDK v0.3.149 et versions ultérieures. Au moins l'un de `script`, `name` ou `scriptPath` est requis.

| Champ             | Type      | Description                                                                                                                                                                                                                                                                                                                |
| ----------------- | --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | Script de flux de travail en ligne. Doit commencer par `export const meta = { name, description }` comme littéral, suivi du corps du script utilisant `agent()`, `parallel()`, `pipeline()` et `phase()`. Un tableau `phases` optionnel dans `meta` regroupe les agents sous des étapes nommées dans la vue de progression |
| `name`            | `string`  | Nom d'un flux de travail intégré ou d'un flux de travail enregistré dans `.claude/workflows/`. Résolu en script                                                                                                                                                                                                            |
| `scriptPath`      | `string`  | Chemin vers un fichier de script de flux de travail sur le disque. Prend la priorité sur `script` et `name`. Chaque invocation persiste son script et retourne le chemin dans le résultat, afin que vous puissiez éditer ce fichier et réinvoquer avec le même `scriptPath` pour itérer                                    |
| `args`            | `unknown` | Valeur d'entrée exposée au script en tant que `args` global, pour les flux de travail nommés paramétrés tels qu'une question de recherche ou une liste de chemins de fichiers. Passez les tableaux et les objets comme des valeurs JSON réelles, pas comme une chaîne codée en JSON                                        |
| `resumeFromRunId` | `string`  | ID d'exécution d'une invocation `Workflow` antérieure à reprendre. Les appels `agent()` complétés avec des entrées inchangées retournent les résultats en cache ; seuls les appels modifiés ou nouveaux s'exécutent en direct. Même session uniquement                                                                     |

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
  À partir du TypeScript Agent SDK 0.3.142, `TodoWrite` est désactivé par défaut. Utilisez `TaskCreate`, `TaskGet`, `TaskUpdate` et `TaskList` à la place. Voir [Migrer vers les outils Task](/docs/fr/agent-sdk/todo-tracking#migrate-to-task-tools) pour mettre à jour votre code de surveillance, ou définissez `CLAUDE_CODE_ENABLE_TASKS=0` pour revenir à `TodoWrite`.
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

<h2 id="tool-output-types">
  Types de sortie d'outil
</h2>

Documentation des schémas de sortie pour tous les outils Claude Code intégrés. Ces types sont exportés depuis `@anthropic-ai/claude-agent-sdk` et représentent les données de réponse réelles retournées par chaque outil.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

Union de tous les types de sortie d'outil.

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | AskUserQuestionOutput
  | BashOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | MonitorOutput
  | NotebookEditOutput
  | ReadMcpResourceOutput
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

**Nom de l'outil :** `Agent` (précédemment `Task`, qui est toujours accepté comme alias)

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
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

Le champ `resolvedModel` sur les variantes `completed` et `async_launched` nomme le modèle sur lequel le sous-agent a réellement fonctionné, qui peut différer du modèle demandé en entrée `model` lorsque [`availableModels`](/docs/fr/model-config#restrict-model-selection) ou une autre substitution s'applique. Ce champ nécessite Claude Code v2.1.174 ou ultérieur.

Sur la variante `completed`, `worktreePath` est défini lorsque le sous-agent s'est exécuté dans un worktree git isolé, et `worktreeBranch` nomme la branche de ce worktree lorsque Claude Code l'a créée. `usage.service_tier` porte la chaîne de niveau de service que l'API a signalée pour les demandes du sous-agent.

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
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
};
```

Retourne la sortie de la commande avec stdout/stderr séparés. Les commandes de fond incluent un `backgroundTaskId`.

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
  originalFile: string;
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
  };
};
```

Retourne le résultat d'écriture avec les informations de diff structuré.

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
};
```

Retourne les chemins de fichiers correspondant au motif glob, triés par heure de modification.

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
  appliedLimit?: number;
  appliedOffset?: number;
};
```

Retourne les résultats de recherche. La forme varie selon `mode` : liste de fichiers, contenu avec correspondances ou comptages de correspondances.

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
};
```

Retourne le contenu récupéré avec le statut HTTP et les métadonnées.

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
};
```

Retourne les résultats de recherche du web.

<h3 id="workflow-2">
  Workflow
</h3>

**Nom de l'outil :** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched";
  taskId: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  error?: string;
};
```

Retourne immédiatement après que l'outil accepte l'invocation. Le résultat final arrive plus tard en tant que complément de tâche. Vérifiez `error` avant de traiter l'exécution comme démarrée : un script qui échoue sa vérification de syntaxe retourne `status: "async_launched"` avec `error` défini, et ne s'exécute jamais.

| Champ           | Type               | Description                                                                                                                                                |
| --------------- | ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`        | `"async_launched"` | L'outil a accepté l'invocation. C'est la seule valeur que le champ prend                                                                                   |
| `taskId`        | `string`           | Identifiant de tâche de fond pour l'exécution                                                                                                              |
| `runId`         | `string`           | Identifiant d'exécution de workflow à transmettre en tant que `resumeFromRunId` lors d'une invocation ultérieure                                           |
| `summary`       | `string`           | Description d'une ligne de ce que fait le workflow                                                                                                         |
| `transcriptDir` | `string`           | Répertoire où les transcriptions de sous-agent sont écrites pendant l'exécution                                                                            |
| `scriptPath`    | `string`           | Chemin du script de workflow persisté pour cette exécution. Modifiez-le et transmettez-le en tant que `scriptPath` pour réexécuter sans renvoyer le script |
| `error`         | `string`           | Défini lorsque le script échoue sa vérification de syntaxe. Lorsqu'il est présent, l'exécution n'a pas démarré malgré le statut `async_launched`           |

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
  À partir du TypeScript Agent SDK 0.3.142, `TodoWrite` est désactivé par défaut. Utilisez `TaskCreate`, `TaskGet`, `TaskUpdate` et `TaskList` à la place. Consultez [Migrer vers les outils Task](/docs/fr/agent-sdk/todo-tracking#migrate-to-task-tools) pour mettre à jour votre code de surveillance, ou définissez `CLAUDE_CODE_ENABLE_TASKS=0` pour revenir à `TodoWrite`.
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
  }>;
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

```typescript theme={null}
type ApiKeySource = "user" | "project" | "org" | "temporary" | "oauth";
```

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Fonctionnalités bêta disponibles qui peuvent être activées via l'option `betas`. Voir [En-têtes bêta](https://platform.claude.com/docs/fr/api/beta-headers) pour plus d'informations.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  La bêta `context-1m-2025-08-07` est retirée à partir du 30 avril 2026. Passer cette valeur avec Claude Sonnet 4.5 ou Sonnet 4 n'a aucun effet, et les demandes qui dépassent la fenêtre de contexte standard de 200 k tokens retournent une erreur. Pour utiliser une fenêtre de contexte de 1 M tokens, migrez vers [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 ou Claude Opus 4.8](https://platform.claude.com/docs/fr/about-claude/models/overview), qui incluent 1 M de contexte à prix standard sans en-tête bêta requis.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

Informations sur une commande slash disponible.

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
};
```

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

| Champ                      | Type                                                               | Description                                                                                                                                                                                                                                                                                                                              |
| :------------------------- | :----------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `value`                    | `string`                                                           | Identifiant de modèle à passer dans les appels API                                                                                                                                                                                                                                                                                       |
| `resolvedModel`            | `string \| undefined`                                              | ID de modèle canonique sur le fil que la `value` de cette entrée résout. Une entrée d'alias telle que `sonnet` résout à un ID de modèle explicite tel que `claude-sonnet-5`, de sorte qu'un hôte peut faire correspondre un ID de modèle explicite stocké à l'entrée d'alias qui le couvre. Nécessite Claude Code v2.1.197 ou ultérieur. |
| `displayName`              | `string`                                                           | Nom d'affichage lisible par l'homme                                                                                                                                                                                                                                                                                                      |
| `description`              | `string`                                                           | Description des capacités du modèle                                                                                                                                                                                                                                                                                                      |
| `supportsEffort`           | `boolean \| undefined`                                             | Si ce modèle supporte les niveaux d'effort                                                                                                                                                                                                                                                                                               |
| `supportedEffortLevels`    | `("low" \| "medium" \| "high" \| "xhigh" \| "max")[] \| undefined` | Niveaux d'effort que ce modèle accepte                                                                                                                                                                                                                                                                                                   |
| `supportsAdaptiveThinking` | `boolean \| undefined`                                             | Si ce modèle supporte la réflexion adaptative, où Claude décide quand et combien réfléchir                                                                                                                                                                                                                                               |
| `supportsFastMode`         | `boolean \| undefined`                                             | Si ce modèle supporte le mode rapide                                                                                                                                                                                                                                                                                                     |
| `supportsAutoMode`         | `boolean \| undefined`                                             | Si ce modèle supporte le mode auto                                                                                                                                                                                                                                                                                                       |

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

| Champ         | Type                  | Description                                                                      |
| :------------ | :-------------------- | :------------------------------------------------------------------------------- |
| `name`        | `string`              | Identifiant de type d'agent (par exemple, `"Explore"`, `"general-purpose"`)      |
| `description` | `string`              | Description de quand utiliser cet agent                                          |
| `model`       | `string \| undefined` | Alias de modèle que cet agent utilise. S'il est omis, hérite le modèle du parent |

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

Statut d'un serveur MCP connecté.

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
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
  }[];
};
```

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

La configuration d'un serveur MCP telle que rapportée par `mcpServerStatus()`. C'est l'union de tous les types de transport de serveur MCP.

```typescript theme={null}
type McpServerStatusConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfig
  | McpClaudeAIProxyServerConfig;
```

Voir [`McpServerConfig`](#mcpserverconfig) pour les détails sur chaque type de transport.

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

Statistiques d'utilisation par modèle retournées dans les messages de résultat. La valeur `costUSD` est une estimation côté client. Voir [Suivi des coûts et de l'utilisation](/docs/fr/agent-sdk/cost-tracking) pour les avertissements de facturation.

```typescript theme={null}
type ModelUsage = {
  inputTokens: number;
  outputTokens: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
};
```

<h3 id="configscope">
  `ConfigScope`
</h3>

```typescript theme={null}
type ConfigScope = "local" | "user" | "project";
```

<h3 id="nonnullableusage">
  `NonNullableUsage`
</h3>

Une version de [`Usage`](#usage) avec tous les champs nullables rendus non nullables.

```typescript theme={null}
type NonNullableUsage = {
  [K in keyof Usage]: NonNullable<Usage[K]>;
};
```

<h3 id="usage">
  `Usage`
</h3>

Statistiques d'utilisation des tokens. C'est le type `BetaUsage` de `@anthropic-ai/sdk`.

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
};
```

`BetaServerToolUsage` et `BetaIterationsUsage` sont définis dans `@anthropic-ai/sdk`.

<h3 id="calltoolresult">
  `CallToolResult`
</h3>

Type de résultat d'outil MCP (depuis `@modelcontextprotocol/sdk/types.js`). `structuredContent` est un objet JSON qui peut être retourné aux côtés de `content`, incluant des blocs d'image. Voir [Retourner des données structurées](/docs/fr/agent-sdk/custom-tools#return-structured-data).

```typescript theme={null}
type CallToolResult = {
  content: Array<{
    type: "text" | "image" | "audio" | "resource" | "resource_link";
    // Les champs supplémentaires varient selon le type
  }>;
  structuredContent?: Record<string, unknown>;
  isError?: boolean;
};
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Contrôle le comportement de réflexion/raisonnement de Claude. Prend la priorité sur le `maxThinkingTokens` déprécié.

```typescript theme={null}
type ThinkingDisplay = "summarized" | "omitted";

type ThinkingConfig =
  | { type: "adaptive"; display?: ThinkingDisplay } // Le modèle détermine quand et combien raisonner (Opus 4.6+)
  | { type: "enabled"; budgetTokens?: number; display?: ThinkingDisplay } // Budget de token de réflexion fixe
  | { type: "disabled" }; // Pas de réflexion étendue
```

Le champ `display` optionnel contrôle si le texte de réflexion est retourné `"summarized"` ou `"omitted"`. Sur Claude Opus 4.7 et versions ultérieures, la valeur par défaut de l'API est `"omitted"`, donc définissez `"summarized"` pour recevoir le contenu de réflexion dans les blocs `thinking`.

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

Options passées à la fonction de génération personnalisée.

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
  Le champ `signal` indique à votre fonction de génération quand arrêter le processus. Passez-le comme option `signal` à `spawn()` de Node, ou passez-le à votre gestionnaire d'arrêt de VM ou de conteneur.

  Ce signal ne se déclenche pas au moment où [`Options.abortController`](#options) s'arrête. Le SDK ferme d'abord stdin du processus et attend environ deux secondes pour que l'interface de ligne de commande s'arrête proprement, puis arrête ce signal. Pour réagir au moment où l'appelant s'arrête, écoutez votre propre `Options.abortController.signal`, que votre fonction de génération peut référencer depuis sa portée englobante.
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
};
```

<h3 id="sdkstatusmessage">
  `SDKStatusMessage`
</h3>

Message de mise à jour de statut (par exemple, compaction).

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

Notification quand une tâche de fond se termine, échoue ou est arrêtée. Les tâches de fond incluent les commandes Bash `run_in_background`, les montres [Monitor](#monitor) et les sous-agents de fond.

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  uuid: UUID;
  session_id: string;
};
```

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

Émis quand un hook commence à s'exécuter.

Claude Code livre ce message, [`SDKHookProgressMessage`](#sdkhookprogressmessage) et [`SDKHookResponseMessage`](#sdkhookresponsemessage) au flux de messages immédiatement, y compris pendant qu'un hook `SessionStart` ou `Setup` s'exécute encore lors du démarrage de la session. Claude Code v2.1.169 à v2.1.203 a livré ces messages en un seul lot après qu'un hook `SessionStart` ou `Setup` se soit terminé ; v2.1.204 a restauré la livraison en direct.

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

Émis quand un hook termine l'exécution.

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
  uuid: UUID;
  session_id: string;
};
```

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

Émis quand une tâche de fond commence. Le champ `task_type` est `"local_bash"` pour les commandes Bash de fond et les montres [Monitor](#monitor), `"local_agent"` pour les sous-agents ou `"remote_agent"`.

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskprogressmessage">
  `SDKTaskProgressMessage`
</h3>

Émis périodiquement pendant qu'un sous-agent ou une tâche de fond s'exécute. Le champ `summary` est rempli uniquement quand [`agentProgressSummaries`](#options) est activé.

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

Émis quand l'état d'une tâche de fond change, par exemple quand elle passe de `running` à `completed`. Fusionnez `patch` dans votre carte de tâches locale indexée par `task_id`. Le champ `end_time` est un timestamp Unix epoch en millisecondes, comparable avec `Date.now()`.

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

Émis chaque fois que l'ensemble des tâches de fond en direct change : une tâche démarre, se termine, est tuée, ou un agent de premier plan est mis en arrière-plan. Le tableau `tasks` est l'ensemble complet en direct. Remplacez tout ensemble en cache par chaque charge utile au lieu d'associer les événements `task_started` et `task_notification`, de sorte que le prochain changement d'adhésion corrige tout événement que vous avez manqué.

L'ordre par rapport à ces événements par tâche n'est pas spécifié, donc ne mettez pas en corrélation les deux flux.

Rien n'est émis au démarrage. Réinitialisez à un ensemble vide chaque fois que le processus CLI de la session démarre ou redémarre et laissez le prochain changement d'adhésion le repeupler.

Nécessite Claude Code v2.1.203 ou ultérieur.

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
  }[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkthinkingtokensmessage">
  `SDKThinkingTokensMessage`
</h3>

Émis pendant que Claude produit un bloc de réflexion, y compris un bloc masqué, portant une estimation en cours des tokens de réflexion générés jusqu'à présent. `estimated_tokens` est le total en cours pour le bloc de réflexion actuel et `estimated_tokens_delta` est l'incrément porté par cette trame. Utilisez-le pour l'affichage de la progression. Le décompte final pour la boucle d'agent de haut niveau est le `usage.output_tokens` du message de résultat, qui [n'inclut pas les tokens des sous-agents](/docs/fr/agent-sdk/cost-tracking#get-the-total-cost-of-a-query) ; utilisez [`modelUsage`](#modelusage) pour la comptabilité de l'arborescence complète.

Nécessite Claude Code v2.1.153 ou ultérieur.

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkfilespersistedevent">
  `SDKFilesPersistedEvent`
</h3>

Émis quand les points de contrôle de fichiers sont persistés sur disque.

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

Émis quand la session rencontre une limite de débit.

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

Quand `errorCode` est `"credits_required"`, le rejet provient d'un abonnement claude.ai dont l'utilisation incluse est épuisée, et la session ne peut pas continuer jusqu'à ce que l'utilisateur achète des crédits d'utilisation. `canUserPurchaseCredits` indique si l'utilisateur authentifié peut acheter des crédits pour le compte, et `hasChargeableSavedPaymentMethod` indique si une méthode de paiement enregistrée est disponible. Ces trois champs sont absents sur les événements de limite de débit qui ne sont pas des rejets de crédits requis. Nécessite Claude Code v2.1.181 ou ultérieur.

<h3 id="sdklocalcommandoutputmessage">
  `SDKLocalCommandOutputMessage`
</h3>

Sortie d'une commande slash locale (par exemple, `/voice` ou `/usage`). Affichée comme du texte de style assistant dans la transcription.

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

Émis quand l'ensemble des commandes disponibles change en cours de session, par exemple quand des compétences sont découvertes alors que l'agent entre dans un sous-répertoire. Le tableau `commands` est la liste complète mise à jour, donc remplacez toute liste de commandes en cache par cette charge utile. Appeler `supportedCommands()` à nouveau n'est pas équivalent : cette méthode retourne l'instantané capturé à l'initialisation et ne reflète pas les changements en cours de session.

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

Émis après chaque tour quand `promptSuggestions` est activé. Contient une invite utilisateur suivante prédite.

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

Émis quand la conversation de la session est remplacée sans terminer la session, par exemple après `/clear`, à la sortie du mode plan, ou quand une nouvelle conversation démarre. Montez une transcription vide sous `new_conversation_id` et abandonnez tout titre de session en cache.

```typescript theme={null}
type SDKConversationResetMessage = {
  type: "conversation_reset";
  new_conversation_id: UUID;
  uuid: UUID;
  session_id: string;
};
```

Les typages publiés du SDK déclarent `SDKConversationResetMessage` dans Claude Code v2.1.203 et ultérieur. Avant v2.1.203, `SDKMessage` référençait le type sans le déclarer, donc le rétrécissement sur `type === "conversation_reset"` n'a pas pu être typé quand `skipLibCheck` était désactivé.

<h3 id="aborterror">
  `AbortError`
</h3>

Classe d'erreur personnalisée pour les opérations d'abandon.

```typescript theme={null}
class AbortError extends Error {}
```

<h2 id="sandbox-configuration">
  Configuration du sandbox
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Configuration pour le comportement du sandbox. Utilisez ceci pour activer le sandboxing des commandes et configurer les restrictions réseau par programmation.

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

| Propriété                   | Type                                                  | Par défaut  | Description                                                                                                                                                                                                                                                       |
| :-------------------------- | :---------------------------------------------------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `boolean`                                             | `false`     | Activer le mode sandbox pour l'exécution des commandes                                                                                                                                                                                                            |
| `failIfUnavailable`         | `boolean`                                             | `true`      | S'arrêter au démarrage si `enabled` est `true` mais que le sandbox ne peut pas démarrer. Définissez `false` pour revenir à l'exécution non sandboxée avec un avertissement sur stderr                                                                             |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`      | Approuver automatiquement les commandes bash quand le sandbox est activé                                                                                                                                                                                          |
| `excludedCommands`          | `string[]`                                            | `[]`        | Commandes qui contournent toujours les restrictions du sandbox (par exemple, `['docker']`). Celles-ci s'exécutent sans sandbox automatiquement sans implication du modèle                                                                                         |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`      | Permettre au modèle de demander l'exécution de commandes en dehors du sandbox. Quand `true`, le modèle peut définir `dangerouslyDisableSandbox` dans l'entrée de l'outil, qui revient au [système de permissions](#permissions-fallback-for-unsandboxed-commands) |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined` | Configuration du sandbox spécifique au réseau                                                                                                                                                                                                                     |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | Configuration du sandbox spécifique au système de fichiers pour les restrictions de lecture/écriture                                                                                                                                                              |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined` | Carte des catégories de violation aux motifs à ignorer (par exemple, `{ file: ['/tmp/*'], network: ['localhost'] }`)                                                                                                                                              |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`     | Activer un sandbox imbriqué plus faible pour la compatibilité                                                                                                                                                                                                     |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined` | Configuration de binaire ripgrep personnalisée pour les environnements sandbox                                                                                                                                                                                    |

<Note>
  Le sandbox dépend du support de la plateforme et, sur Linux, d'outils comme `bubblewrap` et `socat`. Quand `enabled` est `true` et que le sandbox ne peut pas démarrer, `query()` signale un message `result` avec `subtype: "error_during_execution"` et la raison dans `errors`. Pour un appel `query()` à message unique, le SDK lève une exception après avoir produit ce résultat d'erreur, donc enveloppez la boucle dans un bloc try pour continuer au-delà. Voir [Gérer le résultat](/docs/fr/agent-sdk/agent-loop#handle-the-result) pour le contrat d'erreur.

  Pour exécuter sans sandbox à la place, définissez `failIfUnavailable: false`.
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
  // Un appel query() à message unique lève une exception après avoir produit un résultat d'erreur,
  // par exemple quand le sandbox ne peut pas démarrer (failIfUnavailable est par défaut true).
  console.log(`Session ended with an error: ${error}`);
}
```

<Warning>
  **Sécurité des sockets Unix :** L'option `allowUnixSockets` peut accorder l'accès à des services système puissants. Par exemple, permettre `/var/run/docker.sock` accorde effectivement un accès complet au système hôte via l'API Docker, contournant l'isolation du sandbox. Autorisez uniquement les sockets Unix strictement nécessaires et comprenez les implications de sécurité de chacun.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Configuration spécifique au réseau pour le mode sandbox. Ces paramètres s'appliquent aux commandes Bash sandboxées quand `enabled` est `true` dans le parent [`SandboxSettings`](#sandboxsettings). Ils ne restreignent pas l'outil WebFetch, qui utilise à la place les [règles de permissions](/docs/fr/permissions#webfetch).

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| Propriété                 | Type       | Par défaut  | Description                                                                                                                                                                                                                                                                                                 |
| :------------------------ | :--------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`        | Noms de domaine auxquels les processus sandboxés peuvent accéder                                                                                                                                                                                                                                            |
| `deniedDomains`           | `string[]` | `[]`        | Noms de domaine auxquels les processus sandboxés ne peuvent pas accéder. Prend la priorité sur `allowedDomains`                                                                                                                                                                                             |
| `allowManagedDomainsOnly` | `boolean`  | `false`     | Paramètres gérés uniquement. Quand défini dans les [paramètres gérés](/docs/fr/permissions#managed-settings), seules les entrées `allowedDomains` des paramètres gérés sont honorées et les entrées des paramètres utilisateur, projet ou locaux sont ignorées. N'a aucun effet quand défini via les options SDK |
| `allowLocalBinding`       | `boolean`  | `false`     | Permettre aux processus de se lier aux ports locaux (par exemple, pour les serveurs de développement)                                                                                                                                                                                                       |
| `allowUnixSockets`        | `string[]` | `[]`        | Chemins de socket Unix auxquels les processus peuvent accéder (par exemple, socket Docker)                                                                                                                                                                                                                  |
| `allowAllUnixSockets`     | `boolean`  | `false`     | Permettre l'accès à tous les sockets Unix                                                                                                                                                                                                                                                                   |
| `httpProxyPort`           | `number`   | `undefined` | Port du proxy HTTP pour les demandes réseau                                                                                                                                                                                                                                                                 |
| `socksProxyPort`          | `number`   | `undefined` | Port du proxy SOCKS pour les demandes réseau                                                                                                                                                                                                                                                                |

<Note>
  Le proxy sandbox intégré applique `allowedDomains` en fonction du nom d'hôte demandé et ne termine pas ou n'inspecte pas le trafic TLS, donc des techniques telles que le [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) peuvent potentiellement le contourner. Voir [Limitations de sécurité du sandboxing](/docs/fr/sandboxing#security-limitations) pour les détails et [Déploiement sécurisé](/docs/fr/agent-sdk/secure-deployment#traffic-forwarding) pour configurer un proxy qui termine TLS.
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

| Propriété    | Type       | Par défaut | Description                                                    |
| :----------- | :--------- | :--------- | :------------------------------------------------------------- |
| `allowWrite` | `string[]` | `[]`       | Motifs de chemin de fichier pour permettre l'accès en écriture |
| `denyWrite`  | `string[]` | `[]`       | Motifs de chemin de fichier pour refuser l'accès en écriture   |
| `denyRead`   | `string[]` | `[]`       | Motifs de chemin de fichier pour refuser l'accès en lecture    |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Repli des permissions pour les commandes non sandboxées
</h3>

Quand `allowUnsandboxedCommands` est activé, le modèle peut demander l'exécution de commandes en dehors du sandbox en définissant `dangerouslyDisableSandbox: true` dans l'entrée de l'outil. Ces demandes reviennent au système de permissions existant, ce qui signifie que votre gestionnaire `canUseTool` est invoqué, vous permettant d'implémenter une logique d'autorisation personnalisée. Dans l'exemple ci-dessous, `isCommandAuthorized` représente une vérification d'autorisation que vous définissez.

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands` :**

  * `excludedCommands` : Une liste statique de commandes qui contournent toujours le sandbox automatiquement (par exemple, `['docker']`). Le modèle n'a aucun contrôle sur ceci.
  * `allowUnsandboxedCommands` : Permet au modèle de décider à l'exécution s'il faut demander l'exécution non sandboxée en définissant `dangerouslyDisableSandbox: true` dans l'entrée de l'outil.
</Note>

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true // Le modèle peut demander l'exécution non sandboxée
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // Vérifier si le modèle demande de contourner le sandbox
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // Le modèle demande d'exécuter cette commande en dehors du sandbox
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

Ce modèle vous permet de :

* **Auditer les demandes du modèle :** Enregistrer quand le modèle demande l'exécution non sandboxée
* **Implémenter des listes blanches :** Permettre uniquement à des commandes spécifiques de s'exécuter sans sandbox
* **Ajouter des flux d'approbation :** Exiger une autorisation explicite pour les opérations privilégiées

<Warning>
  Les commandes s'exécutant avec `dangerouslyDisableSandbox: true` ont un accès complet au système. Assurez-vous que votre gestionnaire `canUseTool` valide ces demandes avec soin.

  Si `permissionMode` est défini sur `bypassPermissions` et `allowUnsandboxedCommands` est activé, le modèle peut exécuter de manière autonome des commandes en dehors du sandbox sans aucune invite d'approbation (une [règle `ask`](/docs/fr/agent-sdk/permissions#how-permissions-are-evaluated) explicite en force toujours une). Cette combinaison permet effectivement au modèle d'échapper à l'isolation du sandbox silencieusement.
</Warning>

<h2 id="see-also">
  Voir aussi
</h2>

* [Aperçu du SDK](/docs/fr/agent-sdk/overview) - Concepts généraux du SDK
* [Référence du SDK Python](/docs/fr/agent-sdk/python) - Documentation du SDK Python
* [Référence CLI](/docs/fr/cli-reference) - Interface de ligne de commande
* [Flux de travail courants](/docs/fr/common-workflows) - Guides étape par étape
