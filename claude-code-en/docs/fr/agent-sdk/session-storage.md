> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Persister les sessions dans un stockage externe

> Miroir les transcriptions de session Agent SDK vers votre propre magasin d'objets, magasin clé-valeur ou base de données afin que d'autres hôtes puissent reprendre vos sessions.

Par défaut, le SDK écrit les transcriptions de session dans des fichiers JSONL sous `~/.claude/projects/` sur le système de fichiers local. Un adaptateur `SessionStore` vous permet de mettre en miroir ces transcriptions vers votre propre backend, tel qu'un magasin d'objets, un magasin clé-valeur ou une base de données, afin qu'une session créée sur un hôte puisse être reprise sur un autre hôte exécuté à partir d'un répertoire de travail correspondant.

Raisons courantes d'utiliser un magasin de sessions :

* **Déploiements multi-hôtes.** Les fonctions serverless, les workers autoscalés et les exécuteurs CI ne partagent pas de système de fichiers. Un magasin partagé permet aux réplicas de reprendre les sessions les unes des autres.
* **Durabilité.** Les conteneurs locaux sont éphémères. Un magasin externe survit aux redémarrages et aux redéploiements.
* **Conformité et audit.** Conservez les transcriptions dans un stockage que vous gouvernez déjà, avec vos propres règles de rétention, chiffrement et contrôles d'accès.

<h2 id="the-sessionstore-interface">
  L'interface `SessionStore`
</h2>

Un `SessionStore` est un objet avec deux méthodes requises, `append` et `load`, et quatre méthodes optionnelles. Le SDK appelle `append` pour écrire les entrées de transcription lors d'une requête et `load` pour les relire pour la reprise.

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Exported from @anthropic-ai/claude-agent-sdk as
  // SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

  type SessionKey = {
    projectKey: string;
    sessionId: string;
    subpath?: string;
  };

  type SessionStore = {
    // Required
    append(key: SessionKey, entries: SessionStoreEntry[]): Promise<void>;
    load(key: SessionKey): Promise<SessionStoreEntry[] | null>;

    // Optional
    listSessions?(
      projectKey: string,
    ): Promise<Array<{ sessionId: string; mtime: number }>>;
    listSessionSummaries?(projectKey: string): Promise<SessionSummaryEntry[]>;
    delete?(key: SessionKey): Promise<void>;
    listSubkeys?(key: {
      projectKey: string;
      sessionId: string;
    }): Promise<string[]>;
  };

  type SessionSummaryEntry = {
    sessionId: string;
    mtime: number;
    data: Record<string, unknown>;
  };
  ```

  ```python Python theme={null}
  # Exported from claude_agent_sdk as
  # SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

  class SessionKey(TypedDict):
      project_key: str
      session_id: str
      subpath: NotRequired[str]

  class SessionStore(Protocol):
      # Required
      async def append(
          self, key: SessionKey, entries: list[SessionStoreEntry]
      ) -> None: ...
      async def load(self, key: SessionKey) -> list[SessionStoreEntry] | None: ...

      # Optional — omit or raise NotImplementedError
      async def list_sessions(
          self, project_key: str
      ) -> list[SessionStoreListEntry]: ...
      async def list_session_summaries(
          self, project_key: str
      ) -> list[SessionSummaryEntry]: ...
      async def delete(self, key: SessionKey) -> None: ...
      async def list_subkeys(self, key: SessionListSubkeysKey) -> list[str]: ...

  class SessionSummaryEntry(TypedDict):
      session_id: str
      mtime: int
      data: dict[str, Any]
  ```
</CodeGroup>

`SessionKey` adresse une transcription. `projectKey` est un encodage stable et sûr pour le système de fichiers du répertoire de travail, `sessionId` est l'UUID de la session, et `subpath` est défini lorsque l'entrée appartient à une transcription de sous-agent ou à un fichier sidecar plutôt qu'à la conversation principale.

Parce que `projectKey` encode le répertoire de travail, reprenez ou continuez à partir du magasin à partir d'un répertoire de travail correspondant à l'exécution d'origine. En TypeScript, si vous définissez [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/fr/sessions#name-the-project-directory-yourself) à côté de `CLAUDE_CONFIG_DIR` dans l'option [`env`](/docs/fr/agent-sdk/typescript#options) d'une requête, le SDK indexe les entrées de cette requête, et ses recherches `resume` et `continue`, par ce nom à la place. Parce que les assistants autonomes tels que `listSessions` et `deleteSession` ne prennent pas `env` et lisent l'environnement du processus, définissez `CLAUDE_CONFIG_DIR` et le même nom dans l'environnement du processus hôte également. Nécessite Agent SDK v0.3.234 ou ultérieur.

Traitez `subpath` comme un suffixe de clé opaque ; il suit la disposition sur disque, par exemple `subagents/agent-<id>`. Lorsque `subpath` n'est pas défini, la clé fait référence à la transcription principale.

| Méthode                | Requise | Appelée quand                                                                                                                                                                                                                                                                                                                                                                           |
| :--------------------- | :------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`               | Oui     | Après chaque lot d'entrées de transcription écrites localement. Les entrées sont des objets sûrs pour JSON, une par ligne dans le JSONL local.                                                                                                                                                                                                                                          |
| `load`                 | Oui     | Avant le lancement du sous-processus lorsque `resume` est défini ou `continue: true` résout la session de magasin la plus récente, et une fois par session lors de l'énumération qui revient à `listSessionSummaries`. Retournez `null` si la session est inconnue.                                                                                                                     |
| `listSessions`         | Non     | Par `listSessions({ sessionStore })` et par `query()`/`startup()` avec `continue: true`. Si non défini, `continue: true` lève une exception, et `listSessions({ sessionStore })` lève une exception sauf si `listSessionSummaries` est implémenté.                                                                                                                                      |
| `listSessionSummaries` | Non     | Par `listSessions({ sessionStore })` pour lire les métadonnées de toutes les sessions en un seul appel. Maintenez les résumés à l'intérieur de `append`. Si non défini, l'énumération revient à `listSessions` plus un `load` par session.                                                                                                                                              |
| `delete`               | Non     | Par `deleteSession({ sessionStore })`. La suppression de la clé principale (pas de `subpath`) doit en cascade à toutes les sous-clés de cette session et supprimer également l'entrée de résumé de la session, de sorte qu'une session supprimée cesse d'apparaître dans `listSessionSummaries`. Si non défini, la suppression est une no-op, ce qui convient aux backends append-only. |
| `listSubkeys`          | Non     | Pendant la reprise, pour découvrir les transcriptions de sous-agents. Si non défini, seule la transcription principale est restaurée.                                                                                                                                                                                                                                                   |

Dans une `SessionSummaryEntry`, `mtime` est l'heure d'écriture du stockage du sidecar et doit partager une source d'horloge avec les valeurs `mtime` que `listSessions` retourne. `data` est un état opaque appartenant au SDK ; persistez-le verbatim sans l'interpréter.

Construisez les entrées en appelant l'assistant exporté `foldSessionSummary`, `fold_session_summary` en Python, sur chaque lot à l'intérieur de `append`. Ignorez les lots dont la clé a un `subpath` ; les transcriptions de sous-agents ne doivent pas contribuer au résumé de la session principale. Le fold ne définit jamais `mtime` : horodatez-le au moment de la persistance, via l'argument `options.mtime` en TypeScript ou en réécrivant le champ sur l'entrée retournée en Python. Les appels `append` concurrents pour la même session peuvent être en concurrence sur le sidecar, donc sérialisez la lecture-fold-écriture avec une transaction, un compare-and-swap, ou un verrou par session ; le fold lui-même est pur.

Pour ce que le SDK fait avec la transcription que `load` retourne, voir [Reprendre à partir du magasin](#resume-from-the-store).

<h2 id="quick-start">
  Démarrage rapide
</h2>

Le SDK expédie un `InMemorySessionStore` pour le développement et les tests. L'exemple ci-dessous exécute une requête avec le magasin attaché, capture l'ID de session du message de résultat, puis reprend à partir du magasin dans un deuxième appel `query()`. Le deuxième appel transmet la même instance de magasin plus `resume`, afin que le SDK charge la transcription à partir du magasin au lieu du système de fichiers local :

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, InMemorySessionStore } from "@anthropic-ai/claude-agent-sdk";

  const store = new InMemorySessionStore();

  let sessionId: string | undefined;
  try {
    for await (const message of query({
      prompt: "List the TypeScript files under src/",
      options: { sessionStore: store },
    })) {
      if (message.type === "result") {
        sessionId = message.session_id;
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, sessionId was already captured by the loop
    // above; connection or process failures yield no result message.
    console.error(`Session ended with an error: ${error}`);
  }

  // Resume from the store. The agent has full context from the first call.
  for await (const message of query({
    prompt: "Summarize what those files do",
    options: { sessionStore: store, resume: sessionId },
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      ClaudeAgentOptions,
      InMemorySessionStore,
      ResultMessage,
      query,
  )

  store = InMemorySessionStore()


  async def main():
      session_id = None
      try:
          async for message in query(
              prompt="List the Python files under src/",
              options=ClaudeAgentOptions(session_store=store),
          ):
              if isinstance(message, ResultMessage):
                  session_id = message.session_id
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, session_id was already captured by the
          # loop above; connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")

      # Resume from the store. The agent has full context from the first call.
      async for message in query(
          prompt="Summarize what those files do",
          options=ClaudeAgentOptions(session_store=store, resume=session_id),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

La deuxième requête affiche un résumé des fichiers de la première requête, ce qui montre que l'agent a repris avec le contexte complet du magasin.

<h2 id="write-your-own-adapter">
  Écrivez votre propre adaptateur
</h2>

Implémentez `append` et `load` par rapport à votre backend. Ajoutez `listSessions`, `listSessionSummaries`, `delete` et `listSubkeys` si vous voulez que `listSessions()`, les lectures de métadonnées en un seul appel, `deleteSession()` et la reprise de sous-agent fonctionnent par rapport au magasin.

Les entrées transmises à `append` sont typées comme `SessionStoreEntry` (un objet `{ type: string; ... }`). Traitez-les comme des valeurs JSON-safe opaques : persistez-les dans l'ordre et retournez-les de `load` dans le même ordre. `load` doit retourner des entrées qui sont deep-equal à ce qui a été ajouté ; la sérialisation byte-equal n'est pas requise, donc un backend qui réorganise les clés d'objet, comme un type de colonne JSON binaire, convient.

<h2 id="reference-implementations">
  Implémentations de référence
</h2>

Les deux référentiels SDK incluent des adaptateurs de référence exécutables sous [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores) en TypeScript et [`examples/session_stores/`](https://github.com/anthropics/claude-agent-sdk-python/tree/main/examples/session_stores) en Python. Il y a un adaptateur par type de stockage, et chacun montre comment `append` et `load` se mappent sur ce type de backend. Ils ne sont pas publiés en tant que packages ; copiez l'adaptateur du type le plus proche de votre backend dans votre projet, installez le client de votre backend, et adaptez-le.

| Type de stockage                                      | Modèle de stockage                                                                                          | Adaptateur d'exemple                                                                                                                                                                                                                                       |
| :---------------------------------------------------- | :---------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Magasin d'objets                                      | Un fichier de partie par `append()` ; `load()` liste les parties, les trie et les concatène.                | S3 ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/s3_session_store.py))                   |
| Magasin clé-valeur                                    | Une liste par transcription que `append()` pousse et `load()` lit en plage, plus un index trié de sessions. | Redis ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/redis_session_store.py))          |
| Base de données relationnelle ou magasin de documents | Une ligne ou un document par entrée, stocké en JSON et ordonné par une clé assignée à l'insertion.          | Postgres ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/postgres_session_store.py)) |

Chaque adaptateur prend une instance de client préconfigurée, afin que vous contrôliez les identifiants, TLS, la région et le pooling. L'exemple suivant câble l'adaptateur de magasin d'objets dans `query()` et reprend ensuite à partir de celui-ci sur un autre hôte :

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";
import { S3Client } from "@aws-sdk/client-s3";
import { S3SessionStore } from "./S3SessionStore"; // copied from examples/session-stores/s3

const store = new S3SessionStore({
  bucket: "my-claude-sessions",
  prefix: "transcripts",
  client: new S3Client({ region: "us-east-1" }),
});

for await (const message of query({
  prompt: "Hello!",
  options: { sessionStore: store },
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}

// Later, possibly on a different host:
for await (const message of query({
  prompt: "Continue where we left off",
  options: { sessionStore: store, resume: "previous-session-id" },
})) {
  // ...
}
```

<h3 id="validate-your-adapter">
  Validez votre adaptateur
</h3>

Les deux SDK expédient une suite de conformité qui affirme le contrat comportemental que `append`, `load` et les méthodes optionnelles doivent satisfaire. Les tests pour les méthodes optionnelles ignorent automatiquement lorsque ces méthodes ne sont pas implémentées.

En TypeScript, copiez [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) du répertoire d'exemples dans votre suite de tests. En Python, la suite est expédiée dans le package. Pour l'exécuter avec pytest, qui n'est pas une dépendance du SDK, installez d'abord pytest :

```bash theme={null}
pip install pytest
```

Ensuite, transmettez votre adaptateur à la suite dans un fichier de test en tant que fabrique sans argument, que `run_session_store_conformance` appelle une fois par contrat pour construire un nouveau magasin :

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.anyio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

Passer la classe `MyRedisStore` elle-même, comme cet exemple le fait, fonctionne lorsque le constructeur ne prend aucun argument. Pour un adaptateur qui prend un client préconfiguré, transmettez plutôt une lambda qui construit le magasin. Parce que les contrats réutilisent les mêmes clés de session, chaque magasin que la fabrique retourne doit commencer avec un stockage vide, donc faites en sorte que la lambda provisionne un stockage de sauvegarde isolé par appel, comme un faux en mémoire frais, un préfixe de clé unique, ou une nouvelle base de données de test.

<h2 id="behavior-notes">
  Notes de comportement
</h2>

<h3 id="dual-write-architecture">
  Architecture à double écriture
</h3>

Le sous-processus Claude Code écrit toujours d'abord chaque lot d'entrées de transcription sur le disque local, puis le SDK transfère le même lot à `append()` de votre magasin, de sorte que le magasin est un miroir de la transcription locale plutôt qu'un remplacement. Quelle copie survit à l'exécution dépend de la façon dont l'exécution a commencé :

* **Session nouvelle, ou une reprise quand le magasin n'a rien pour la session** : la transcription locale sous votre répertoire de configuration survit à l'exécution, et le magasin reçoit une copie.
* **Exécution [reprise à partir du magasin](#resume-from-the-store)** : la copie locale est supprimée à la fin de l'exécution, de sorte que le magasin détient la seule copie durable.

Si vous ne voulez pas qu'une session nouvelle laisse une transcription sur le disque local, définissez `CLAUDE_CONFIG_DIR` sur un répertoire temporaire dans `options.env`. Une exécution reprise à partir du magasin supprime déjà sa copie locale, elle n'a donc besoin d'aucun tel paramètre. En TypeScript, propagez également `process.env` dans `env`, puisque l'[option `env`](/docs/fr/agent-sdk/typescript#options) remplace l'environnement du sous-processus.

Si votre application se connecte via des fichiers dans le répertoire de configuration, tels que les identifiants OAuth ou un `apiKeyHelper` dans votre `settings.json` utilisateur, copiez d'abord ces fichiers dans le répertoire temporaire, ou définissez `ANTHROPIC_API_KEY` dans `env` à la place. Sinon, l'exécution échoue avec `Not logged in`.

Deux options entrent en conflit avec le miroir, et le SDK lève une exception au démarrage si vous combinez l'une ou l'autre avec un magasin :

* **`persistSession: false`** en TypeScript : désactive les écritures locales sur lesquelles le miroir est construit. Le SDK Python n'a pas d'option équivalente.
* **Sauvegarde de point de contrôle de fichier**, `enableFileCheckpointing` en TypeScript ou `enable_file_checkpointing` en Python : écrit ses sauvegardes de fichiers directement sur le disque local, et le SDK ne les met pas en miroir vers le magasin.

<h3 id="resume-from-the-store">
  Reprise à partir du magasin
</h3>

Quand vous passez `resume`, ou `continue: true` en TypeScript ou `continue_conversation=True` en Python, ensemble avec un magasin, le SDK demande au magasin une transcription avant de générer le sous-processus :

* **`resume`** : le SDK demande la session dont vous avez passé l'ID.
* **`continue: true`** ou **`continue_conversation=True`** : le SDK demande la session la plus récente du magasin.

Quand le magasin retourne la transcription, le SDK l'écrit dans un répertoire de configuration temporaire, exécute le sous-processus avec `CLAUDE_CONFIG_DIR` pointant là-bas, et supprime le répertoire quand l'exécution se termine. La transcription locale que cette exécution écrit est supprimée avec elle, c'est pourquoi le magasin détient la seule copie durable sur ce chemin.

Le SDK amorce également le répertoire temporaire avec des fichiers de votre répertoire de configuration réel. Ce qu'il copie diffère selon le langage :

* **TypeScript** : identifiants, `.claude.json`, et votre `settings.json` utilisateur. De `settings.json`, il supprime les clés qui se comportent mal sous un répertoire de configuration temporaire : `enabledPlugins`, `extraKnownMarketplaces`, son alias [`additionalMarketplaces`](/docs/fr/settings-reference#extraknownmarketplaces), et tout `CLAUDE_CONFIG_DIR` dans le bloc `env` du fichier. Avant Agent SDK v0.3.232, le SDK ne supprimait pas l'alias. L'authentification configurée dans les paramètres, telle que [`apiKeyHelper`](/docs/fr/settings-reference#apikeyhelper), fonctionne quand vous reprenez à partir du magasin. Avant Agent SDK v0.3.222, le SDK TypeScript copiait uniquement les identifiants et `.claude.json`.
* **Python** : identifiants et `.claude.json` uniquement, donc une application qui s'authentifie via `apiKeyHelper` dans votre `settings.json` utilisateur échoue avec `Not logged in` lors de la reprise à partir d'un magasin. Un `apiKeyHelper` dans les paramètres gérés ou de projet fonctionne toujours, car Claude Code lit ces fichiers à partir d'emplacements que `CLAUDE_CONFIG_DIR` n'affecte pas.

Quand le magasin n'a rien pour la session, le SDK s'exécute sous votre répertoire de configuration réel à la place, et le résultat dépend de l'option que vous avez passée :

* **`resume`** : les deux SDK passent l'ID au sous-processus, qui reprend la transcription locale exactement comme `resume` le fait sans magasin.
* **`continue: true`** en TypeScript : le SDK démarre une session nouvelle.
* **`continue_conversation=True`** en Python : le SDK continue à partir de la session locale la plus récente.

<h3 id="mirror-writes-are-best-effort">
  Les écritures en miroir sont au mieux
</h3>

Si `append()` rejette, le SDK réessaie le lot jusqu'à deux fois supplémentaires avec un délai court, pour un maximum de trois tentatives au total. Un appel qui expire n'est pas réessayé, car l'appel original peut toujours aboutir. Si le lot échoue toujours, le SDK enregistre l'erreur, émet un message `{ type: "system", subtype: "mirror_error" }` dans l'itérateur, supprime le lot, et continue la requête. Parce qu'un lot réessayé peut redélivrer des entrées qui ont déjà abouti, dédupliquez par `entry.uuid` dans votre implémentation de `append()`.

Une panne du magasin n'interrompt pas l'agent, puisque le sous-processus écrit localement d'abord. Surveillez `mirror_error` si vous devez détecter une perte de données du magasin. Sur une exécution [reprise à partir du magasin](#resume-from-the-store), un lot supprimé n'a pas de copie survivante une fois l'exécution terminée.

<h3 id="getsessionmessages-returns-the-post-compaction-chain">
  `getSessionMessages` retourne la chaîne post-compaction
</h3>

`getSessionMessages({ sessionStore })` retourne la chaîne de messages liée que l'agent verrait à la reprise. Après la compaction automatique, les tours antérieurs sont remplacés par un résumé, donc une session dont le magasin contient 503 entrées brutes peut retourner 18 messages de `getSessionMessages`. Pour l'historique brut complet, y compris les tours pré-compaction et les entrées de métadonnées, appelez `store.load(key)` directement.

<h3 id="forksession-is-not-a-byte-copy">
  `forkSession` n'est pas une copie byte
</h3>

`forkSession({ sessionStore })` lit les entrées source, réécrit chaque champ `sessionId` et remapte les UUID de message, puis ajoute les entrées transformées sous une nouvelle clé. Une copie au niveau de l'adaptateur ou un raccourci `CopyObject` produirait une transcription qui référence toujours l'ancien ID de session, donc le SDK n'en utilise pas.

<h3 id="subagent-transcripts">
  Transcriptions de sous-agents
</h3>

Les transcriptions de sous-agents sont mises en miroir sous `subpath: "subagents/agent-<id>"`. `listSubagents({ sessionStore })` nécessite que l'adaptateur implémente `listSubkeys` ; `getSubagentMessages({ sessionStore })` l'utilise quand disponible mais revient au subpath direct quand il n'est pas défini. La reprise appelle également `listSubkeys` pour restaurer les fichiers de sous-agents ; sans cela, seule la transcription principale est matérialisée.

<h3 id="retention">
  Rétention
</h3>

Le SDK ne supprime jamais de votre magasin de son propre chef. La rétention est la responsabilité de l'adaptateur : utilisez le mécanisme d'expiration ou de cycle de vie de votre backend, ou exécutez un nettoyage programmé, selon vos exigences de conformité.

Les transcriptions locales sous `CLAUDE_CONFIG_DIR` sont balayées indépendamment par le paramètre `cleanupPeriodDays`, en suivant les [règles de balayage de rétention](/docs/fr/claude-directory#cleaned-up-automatically). Une exécution [reprise à partir du magasin](#resume-from-the-store) ne laisse pas de transcription locale, donc pour ces exécutions la rétention de votre magasin est la seule rétention qu'il y a.

<h2 id="supported-on">
  Supporté sur
</h2>

Les fonctions SDK TypeScript suivantes acceptent une option `sessionStore` et opèrent par rapport au magasin au lieu du système de fichiers local quand elle est fournie :

* [`query()`](/docs/fr/agent-sdk/typescript#query)
* [`startup()`](/docs/fr/agent-sdk/typescript#startup)
* [`listSessions()`](/docs/fr/agent-sdk/typescript#listsessions)
* [`getSessionInfo()`](/docs/fr/agent-sdk/typescript#getsessioninfo)
* [`getSessionMessages()`](/docs/fr/agent-sdk/typescript#getsessionmessages)
* [`renameSession()`](/docs/fr/agent-sdk/typescript#renamesession)
* [`tagSession()`](/docs/fr/agent-sdk/typescript#tagsession)
* [`deleteSession()`](/docs/fr/agent-sdk/typescript)
* [`forkSession()`](/docs/fr/agent-sdk/typescript)
* [`listSubagents()`](/docs/fr/agent-sdk/typescript)
* [`getSubagentMessages()`](/docs/fr/agent-sdk/typescript)

Dans le SDK Python, définissez `session_store` dans [`ClaudeAgentOptions`](/docs/fr/agent-sdk/python#claudeagentoptions) pour exécuter `query()` par rapport à un magasin. Les opérations restantes ont chacune une fonction Python soutenue par un magasin qui prend le magasin comme argument : `list_sessions_from_store()`, `get_session_info_from_store()`, `get_session_messages_from_store()`, `list_subagents_from_store()`, `get_subagent_messages_from_store()`, `rename_session_via_store()`, `tag_session_via_store()`, `delete_session_via_store()`, et `fork_session_via_store()`. `startup()` n'a pas d'équivalent Python. Les fonctions autonomes documentées dans la [référence du SDK Python](/docs/fr/agent-sdk/python#functions), telles que `list_sessions()`, lisent les fichiers de session locaux.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Travailler avec les sessions](/docs/fr/agent-sdk/sessions) : Continuer, reprendre et forker sans magasin personnalisé
* [Héberger le SDK](/docs/fr/agent-sdk/hosting) : Modèles de déploiement pour les environnements multi-hôtes
* [TypeScript `Options`](/docs/fr/agent-sdk/typescript#options) : Référence complète des options
* [Implémentations de référence](#reference-implementations) : Adaptateurs d'exemple exécutables pour un magasin d'objets, un magasin clé-valeur et une base de données, dans les deux référentiels SDK
