> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sitzungen in externem Speicher persistieren

> Spiegeln Sie Sitzungstranskripte zu S3, Redis oder Ihrem eigenen Backend, damit jeder Host sie fortsetzen kann.

Standardmäßig schreibt das SDK Sitzungstranskripte in JSONL-Dateien unter `~/.claude/projects/` im lokalen Dateisystem. Ein `SessionStore`-Adapter ermöglicht es Ihnen, diese Transkripte in Ihrem eigenen Backend zu spiegeln, z. B. in S3, Redis oder einer Datenbank, sodass eine auf einem Host erstellte Sitzung auf einem anderen Host fortgesetzt werden kann.

Häufige Gründe für die Verwendung eines Session Store:

* **Multi-Host-Bereitstellungen.** Serverlose Funktionen, automatisch skalierte Worker und CI-Runner teilen sich kein Dateisystem. Ein gemeinsamer Store ermöglicht es jedem Replikat, jede Sitzung fortzusetzen.
* **Dauerhaftigkeit.** Lokale Container sind kurzlebig. Ein Store, der von S3 oder einer Datenbank unterstützt wird, übersteht Neustarts und Neubereitstellungen.
* **Compliance und Audit.** Bewahren Sie Transkripte in Speicher auf, den Sie bereits kontrollieren, mit Ihren eigenen Aufbewahrungsrichtlinien, Verschlüsselung und Zugriffskontrolle.

<h2 id="the-sessionstore-interface">
  Die `SessionStore`-Schnittstelle
</h2>

Ein `SessionStore` ist ein Objekt mit zwei erforderlichen Methoden, `append` und `load`, und drei optionalen Methoden. Das SDK ruft `append` auf, um Transkripteinträge während einer Abfrage zu schreiben, und `load`, um sie zum Fortsetzen zurückzulesen.

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

`SessionKey` adressiert ein Transkript. `projectKey` ist eine stabile, dateisystemsichere Kodierung des Arbeitsverzeichnisses, `sessionId` ist die Sitzungs-UUID, und `subpath` wird gesetzt, wenn der Eintrag zu einem Subagent-Transkript oder einer Sidecar-Datei statt zur Hauptkonversation gehört. Behandeln Sie `subpath` als einen undurchsichtigen Schlüsselsuffix; er folgt dem On-Disk-Layout, z. B. `subagents/agent-<id>`. Wenn `subpath` nicht definiert ist, bezieht sich der Schlüssel auf das Haupttranskript.

| Methode        | Erforderlich | Aufgerufen wenn                                                                                                                                                                                                                                 |
| :------------- | :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`       | Ja           | Nach jedem Batch von Transkripteinträgen, die lokal geschrieben werden. Einträge sind JSON-sichere Objekte, eine pro Zeile in der lokalen JSONL.                                                                                                |
| `load`         | Ja           | Einmal vor dem Spawnen des Subprozesses, wenn `resume` gesetzt ist. Geben Sie `null` zurück, wenn die Sitzung unbekannt ist.                                                                                                                    |
| `listSessions` | Nein         | Von `listSessions({ sessionStore })` und von `query()`/`startup()` mit `continue: true`. Wenn nicht definiert, werfen diese Aufrufe einen Fehler.                                                                                               |
| `delete`       | Nein         | Von `deleteSession({ sessionStore })`. Das Löschen des Hauptschlüssels (kein `subpath`) muss auf alle Unterschlüssel für diese Sitzung kaskadieren. Wenn nicht definiert, ist das Löschen ein No-Op, was für Append-Only-Backends geeignet ist. |
| `listSubkeys`  | Nein         | Während des Wiederaufnehmens, um Subagent-Transkripte zu entdecken. Wenn nicht definiert, wird nur das Haupttranskript wiederhergestellt.                                                                                                       |

<h2 id="quick-start">
  Schnellstart
</h2>

Das SDK wird mit einem `InMemorySessionStore` für Entwicklung und Tests ausgeliefert. Das folgende Beispiel führt eine Abfrage mit dem angehängten Store aus, erfasst die Sitzungs-ID aus der Ergebnismeldung und setzt dann aus dem Store in einem zweiten `query()`-Aufruf fort. Der zweite Aufruf übergibt die gleiche Store-Instanz plus `resume`, sodass das SDK das Transkript aus dem Store statt aus dem lokalen Dateisystem lädt:

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

Die zweite Abfrage gibt eine Zusammenfassung der Dateien aus der ersten Abfrage aus, was zeigt, dass der Agent mit vollständigem Kontext aus dem Store fortgesetzt wurde.

<h2 id="write-your-own-adapter">
  Schreiben Sie Ihren eigenen Adapter
</h2>

Implementieren Sie `append` und `load` gegen Ihr Backend. Fügen Sie `listSessions`, `delete` und `listSubkeys` hinzu, wenn Sie möchten, dass `listSessions()`, `deleteSession()` und Subagent-Wiederaufnahme gegen den Store funktionieren.

Einträge, die an `append` übergeben werden, sind als `SessionStoreEntry` typisiert (ein `{ type: string; ... }`-Objekt). Behandeln Sie sie als undurchsichtige JSON-sichere Werte: persistieren Sie sie in Reihenfolge und geben Sie sie von `load` in der gleichen Reihenfolge zurück. `load` muss Einträge zurückgeben, die tiefengleich mit dem sind, was angehängt wurde; Byte-gleiche Serialisierung ist nicht erforderlich, daher sind Backends wie Postgres `jsonb`, die Objektschlüssel neu ordnen, in Ordnung.

<h2 id="reference-implementations">
  Referenzimplementierungen
</h2>

Das TypeScript SDK-Repository enthält ausführbare Referenz-Adapter für S3, Redis und Postgres unter [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores). Sie werden nicht auf npm veröffentlicht; kopieren Sie die `src/`-Datei, die Sie benötigen, in Ihr Projekt und installieren Sie den entsprechenden Backend-Client.

| Adapter                                                                                                                        | Backend-Client       | Speichermodell                                                                    |
| :----------------------------------------------------------------------------------------------------------------------------- | :------------------- | :-------------------------------------------------------------------------------- |
| [`S3SessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3)             | `@aws-sdk/client-s3` | Eine JSONL-Teildatei pro `append()`; `load()` listet auf, sortiert und verkettet. |
| [`RedisSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis)       | `ioredis`            | `RPUSH`/`LRANGE`-Liste pro Transkript, plus einen sortierten Satz-Sitzungsindex.  |
| [`PostgresSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres) | `pg`                 | Eine Zeile pro Eintrag in einer `jsonb`-Tabelle, geordnet nach `BIGSERIAL`.       |

Jeder Adapter nimmt eine vorkonfigurierte Client-Instanz an, sodass Sie Anmeldedaten, TLS, Region und Pooling kontrollieren. Zum Beispiel mit S3:

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
  Validieren Sie Ihren Adapter
</h3>

Beide SDKs werden mit einer Konformitätssuite ausgeliefert, die den Verhaltensvertrag durchsetzt, den `append`, `load` und die optionalen Methoden erfüllen müssen. Tests für optionale Methoden werden automatisch übersprungen, wenn diese Methoden nicht implementiert sind.

In TypeScript kopieren Sie [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) aus dem Beispielverzeichnis in Ihre Test-Suite. In Python wird die Suite im Paket ausgeliefert:

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.asyncio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

<h2 id="behavior-notes">
  Verhaltenshinweise
</h2>

<h3 id="dual-write-architecture">
  Dual-Write-Architektur
</h3>

Der Store ist ein Spiegel, kein Ersatz. Der Claude Code-Subprozess schreibt immer zuerst auf die lokale Festplatte; das SDK leitet dann jeden Batch an `append()` weiter. Wenn Sie möchten, dass die lokale Kopie kurzlebig ist, zeigen Sie `CLAUDE_CONFIG_DIR` auf ein temporäres Verzeichnis in `options.env`. Da der Spiegel von lokalen Schreibvorgängen abhängt, kann `sessionStore` nicht mit `persistSession: false` kombiniert werden; das SDK wirft einen Fehler, wenn Sie beide setzen. Es wirft auch einen Fehler, wenn es mit `enableFileCheckpointing` kombiniert wird, da Dateihistorie-Backup-Blobs direkt auf die lokale Festplatte geschrieben werden und nicht zum Store gespiegelt werden.

<h3 id="mirror-writes-are-best-effort">
  Spiegelschreibvorgänge sind Best-Effort
</h3>

Wenn `append()` ablehnt, versucht das SDK den Batch bis zu zwei weitere Male mit kurzer Backoff-Zeit erneut, insgesamt höchstens drei Versuche. Ein Aufruf, der eine Zeitüberschreitung aufweist, wird nicht erneut versucht, da der ursprüngliche Aufruf möglicherweise noch ankommt. Wenn der Batch immer noch fehlschlägt, wird der Fehler protokolliert, eine `{ type: "system", subtype: "mirror_error" }`-Meldung wird in den Iterator ausgegeben, der Batch wird verworfen, und die Abfrage wird fortgesetzt. Das lokale Transkript ist bereits auf der Festplatte dauerhaft, daher unterbricht ein Store-Ausfall den Agenten nicht und verliert keine Daten lokal. Überwachen Sie auf `mirror_error`, wenn Sie Datenverluste im Store erkennen müssen. Da ein erneut versuchter Batch Einträge, die bereits angekommen sind, erneut bereitstellen kann, deduplizieren Sie nach `entry.uuid` in Ihrer `append()`-Implementierung.

<h3 id="getsessionmessages-returns-the-post-compaction-chain">
  `getSessionMessages` gibt die Post-Komprimierungs-Kette zurück
</h3>

`getSessionMessages({ sessionStore })` gibt die verknüpfte Nachrichtenkette zurück, die der Agent beim Wiederaufnehmen sehen würde. Nach der automatischen Komprimierung werden frühere Umdrehungen durch eine Zusammenfassung ersetzt, daher kann eine Sitzung, deren Store 503 rohe Einträge enthält, 18 Nachrichten von `getSessionMessages` zurückgeben. Für die vollständige Rohhistorie, einschließlich Pre-Komprimierungs-Umdrehungen und Metadateneinträge, rufen Sie `store.load(key)` direkt auf.

<h3 id="forksession-is-not-a-byte-copy">
  `forkSession` ist keine Byte-Kopie
</h3>

`forkSession({ sessionStore })` liest die Quelleinträge, schreibt jedes `sessionId`-Feld um und ordnet Nachrichten-UUIDs neu zu, dann hängt die transformierten Einträge unter einem neuen Schlüssel an. Eine Adapter-Ebenen-Kopie oder `CopyObject`-Verknüpfung würde ein Transkript erzeugen, das immer noch auf die alte Sitzungs-ID verweist, daher verwendet das SDK keine.

<h3 id="subagent-transcripts">
  Subagent-Transkripte
</h3>

Subagent-Transkripte werden unter `subpath: "subagents/agent-<id>"` gespiegelt. `listSubagents({ sessionStore })` erfordert, dass der Adapter `listSubkeys` implementiert; `getSubagentMessages({ sessionStore })` verwendet es, wenn verfügbar, fällt aber auf den direkten Subpath zurück, wenn er nicht definiert ist. Das Wiederaufnehmen ruft auch `listSubkeys` auf, um Subagent-Dateien wiederherzustellen; ohne es wird nur das Haupttranskript materialisiert.

<h3 id="retention">
  Aufbewahrung
</h3>

Das SDK löscht niemals von selbst aus Ihrem Store. Die Aufbewahrung ist die Verantwortung des Adapters: implementieren Sie TTLs, S3-Lebenszyklusrichtlinien oder geplante Bereinigung gemäß Ihren Compliance-Anforderungen. Lokale Transkripte unter `CLAUDE_CONFIG_DIR` werden unabhängig durch die `cleanupPeriodDays`-Einstellung bereinigt.

<h2 id="supported-on">
  Unterstützt auf
</h2>

Die folgenden SDK-Funktionen akzeptieren eine `sessionStore`-Option und arbeiten gegen den Store statt gegen das lokale Dateisystem, wenn es bereitgestellt wird:

* [`query()`](/docs/de/agent-sdk/typescript#query)
* [`startup()`](/docs/de/agent-sdk/typescript#startup)
* [`listSessions()`](/docs/de/agent-sdk/typescript#listsessions)
* [`getSessionInfo()`](/docs/de/agent-sdk/typescript#getsessioninfo)
* [`getSessionMessages()`](/docs/de/agent-sdk/typescript#getsessionmessages)
* [`renameSession()`](/docs/de/agent-sdk/typescript#renamesession)
* [`tagSession()`](/docs/de/agent-sdk/typescript#tagsession)
* [`deleteSession()`](/docs/de/agent-sdk/typescript)
* [`forkSession()`](/docs/de/agent-sdk/typescript)
* [`listSubagents()`](/docs/de/agent-sdk/typescript)
* [`getSubagentMessages()`](/docs/de/agent-sdk/typescript)

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Mit Sitzungen arbeiten](/docs/de/agent-sdk/sessions): Fortsetzen, Wiederaufnehmen und Forken ohne einen benutzerdefinierten Store
* [Das SDK hosten](/docs/de/agent-sdk/hosting): Bereitstellungsmuster für Multi-Host-Umgebungen
* [TypeScript `Options`](/docs/de/agent-sdk/typescript#options): Vollständige Optionsreferenz
* [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores): Ausführbare S3-, Redis- und Postgres-Referenz-Adapter
