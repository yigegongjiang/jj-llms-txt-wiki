> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Persister les sessions dans un stockage externe

> Miroir les transcriptions de session vers S3, Redis ou votre propre backend pour que n'importe quel hôte puisse les reprendre.

Par défaut, le SDK écrit les transcriptions de session dans des fichiers JSONL sous `~/.claude/projects/` sur le système de fichiers local. Un adaptateur `SessionStore` vous permet de mettre en miroir ces transcriptions vers votre propre backend, tel que S3, Redis ou une base de données, afin qu'une session créée sur un hôte puisse être reprise sur un autre.

Raisons courantes d'utiliser un magasin de sessions :

* **Déploiements multi-hôtes.** Les fonctions serverless, les workers autoscalés et les exécuteurs CI ne partagent pas de système de fichiers. Un magasin partagé permet à n'importe quel réplica de reprendre n'importe quelle session.
* **Durabilité.** Les conteneurs locaux sont éphémères. Un magasin sauvegardé par S3 ou une base de données survit aux redémarrages et aux redéploiements.
* **Conformité et audit.** Conservez les transcriptions dans un stockage que vous gouvernez déjà, avec vos propres règles de rétention, chiffrement et contrôles d'accès.

<h2 id="the-sessionstore-interface">
  L'interface `SessionStore`
</h2>

Un `SessionStore` est un objet avec deux méthodes requises, `append` et `load`, et trois méthodes optionnelles. Le SDK appelle `append` pour écrire les entrées de transcription lors d'une requête et `load` pour les relire pour la reprise.

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Exported from @anthropic-ai/claude-agent-sdk as
  // SessionStore, SessionKey, SessionStoreEntry.

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
    delete?(key: SessionKey): Promise<void>;
    listSubkeys?(key: {
      projectKey: string;
      sessionId: string;
    }): Promise<string[]>;
  };
  ```

  ```python Python theme={null}
  # Exported from claude_agent_sdk as
  # SessionStore, SessionKey, SessionStoreEntry.

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
      async def delete(self, key: SessionKey) -> None: ...
      async def list_subkeys(self, key: SessionListSubkeysKey) -> list[str]: ...
  ```
</CodeGroup>

`SessionKey` adresse une transcription. `projectKey` est un encodage stable et sûr pour le système de fichiers du répertoire de travail, `sessionId` est l'UUID de la session, et `subpath` est défini lorsque l'entrée appartient à une transcription de sous-agent ou à un fichier sidecar plutôt qu'à la conversation principale. Traitez `subpath` comme un suffixe de clé opaque ; il suit la disposition sur disque, par exemple `subagents/agent-<id>`. Lorsque `subpath` n'est pas défini, la clé fait référence à la transcription principale.

| Méthode        | Requise | Appelée quand                                                                                                                                                                                                                                 |
| :------------- | :------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`       | Oui     | Après chaque lot d'entrées de transcription écrites localement. Les entrées sont des objets sûrs pour JSON, une par ligne dans le JSONL local.                                                                                                |
| `load`         | Oui     | Une fois avant le lancement du sous-processus, lorsque `resume` est défini. Retournez `null` si la session est inconnue.                                                                                                                      |
| `listSessions` | Non     | Par `listSessions({ sessionStore })` et par `query()`/`startup()` avec `continue: true`. Si non défini, ces appels lèvent une exception.                                                                                                      |
| `delete`       | Non     | Par `deleteSession({ sessionStore })`. La suppression de la clé principale (pas de `subpath`) doit en cascade à toutes les sous-clés de cette session. Si non défini, la suppression est une no-op, ce qui convient aux backends append-only. |
| `listSubkeys`  | Non     | Pendant la reprise, pour découvrir les transcriptions de sous-agents. Si non défini, seule la transcription principale est restaurée.                                                                                                         |

<h2 id="quick-start">
  Démarrage rapide
</h2>

Le SDK expédie un `InMemorySessionStore` pour le développement et les tests. L'exemple ci-dessous exécute une requête avec le magasin attaché, capture l'ID de session du message de résultat, puis reprend à partir du magasin dans un deuxième appel `query()`. Le deuxième appel transmet la même instance de magasin plus `resume`, afin que le SDK charge la transcription à partir du magasin au lieu du système de fichiers local :

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, InMemorySessionStore } from "@anthropic-ai/claude-agent-sdk";

  const store = new InMemorySessionStore();

  let sessionId: string | undefined;
  for await (const message of query({
    prompt: "List the TypeScript files under src/",
    options: { sessionStore: store },
  })) {
    if (message.type === "result") {
      sessionId = message.session_id;
    }
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
      async for message in query(
          prompt="List the Python files under src/",
          options=ClaudeAgentOptions(session_store=store),
      ):
          if isinstance(message, ResultMessage):
              session_id = message.session_id

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

Implémentez `append` et `load` par rapport à votre backend. Ajoutez `listSessions`, `delete` et `listSubkeys` si vous voulez que `listSessions()`, `deleteSession()` et la reprise de sous-agent fonctionnent par rapport au magasin.

Les entrées transmises à `append` sont typées comme `SessionStoreEntry` (un objet `{ type: string; ... }`). Traitez-les comme des valeurs JSON-safe opaques : persistez-les dans l'ordre et retournez-les de `load` dans le même ordre. `load` doit retourner des entrées qui sont deep-equal à ce qui a été ajouté ; la sérialisation byte-equal n'est pas requise, donc les backends comme Postgres `jsonb` qui réorganisent les clés d'objet vont bien.

<h2 id="reference-implementations">
  Implémentations de référence
</h2>

Le référentiel TypeScript SDK inclut des adaptateurs de référence exécutables pour S3, Redis et Postgres sous [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores). Ils ne sont pas publiés sur npm ; copiez le fichier `src/` dont vous avez besoin dans votre projet et installez le client backend correspondant.

| Adaptateur                                                                                                                     | Client backend       | Modèle de stockage                                                                |
| :----------------------------------------------------------------------------------------------------------------------------- | :------------------- | :-------------------------------------------------------------------------------- |
| [`S3SessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3)             | `@aws-sdk/client-s3` | Un fichier de partie JSONL par `append()` ; `load()` liste, trie et concatène.    |
| [`RedisSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis)       | `ioredis`            | Liste `RPUSH`/`LRANGE` par transcription, plus un index de sorted-set de session. |
| [`PostgresSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres) | `pg`                 | Une ligne par entrée dans une table `jsonb`, ordonnée par `BIGSERIAL`.            |

Chaque adaptateur prend une instance de client préconfigurée, afin que vous contrôliez les identifiants, TLS, la région et le pooling. Par exemple, avec S3 :

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

En TypeScript, copiez [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) du répertoire d'exemples dans votre suite de tests. En Python, la suite est expédiée dans le package :

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.asyncio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

<h2 id="behavior-notes">
  Notes de comportement
</h2>

<h3 id="dual-write-architecture">
  Architecture à double écriture
</h3>

Le magasin est un miroir, pas un remplacement. Le sous-processus Claude Code écrit toujours d'abord sur le disque local ; le SDK transfère ensuite chaque lot à `append()`. Si vous voulez que la copie locale soit éphémère, pointez `CLAUDE_CONFIG_DIR` vers un répertoire temporaire dans `options.env`. Parce que le miroir dépend des écritures locales, `sessionStore` ne peut pas être combiné avec `persistSession: false` ; le SDK lève une exception si vous définissez les deux. Il lève également une exception s'il est combiné avec `enableFileCheckpointing`, car les blobs de sauvegarde de l'historique des fichiers sont écrits directement sur le disque local et ne sont pas mis en miroir vers le magasin.

<h3 id="mirror-writes-are-best-effort">
  Les écritures en miroir sont au mieux
</h3>

Si `append()` rejette, le SDK réessaie le lot jusqu'à deux fois supplémentaires avec un délai court, pour un maximum de trois tentatives au total. Un appel qui expire n'est pas réessayé, car l'appel original peut toujours aboutir. Si le lot échoue toujours, l'erreur est enregistrée, un message `{ type: "system", subtype: "mirror_error" }` est émis dans l'itérateur, le lot est supprimé, et la requête continue. La transcription locale est déjà durable sur disque, donc une panne du magasin n'interrompt pas l'agent ou ne perd pas de données localement. Surveillez `mirror_error` si vous devez détecter une perte de données du magasin. Parce qu'un lot réessayé peut redélivrer des entrées qui ont déjà abouti, dédupliquez par `entry.uuid` dans votre implémentation de `append()`.

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

Le SDK ne supprime jamais de votre magasin de son propre chef. La rétention est la responsabilité de l'adaptateur : implémentez des TTL, des politiques de cycle de vie S3 ou un nettoyage programmé selon vos exigences de conformité. Les transcriptions locales sous `CLAUDE_CONFIG_DIR` sont balayées indépendamment par le paramètre `cleanupPeriodDays`.

<h2 id="supported-on">
  Supporté sur
</h2>

Les fonctions SDK suivantes acceptent une option `sessionStore` et opèrent par rapport au magasin au lieu du système de fichiers local quand elle est fournie :

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

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Travailler avec les sessions](/docs/fr/agent-sdk/sessions) : Continuer, reprendre et forker sans magasin personnalisé
* [Héberger le SDK](/docs/fr/agent-sdk/hosting) : Modèles de déploiement pour les environnements multi-hôtes
* [TypeScript `Options`](/docs/fr/agent-sdk/typescript#options) : Référence complète des options
* [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores) : Adaptateurs de référence S3, Redis et Postgres exécutables
