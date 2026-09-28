> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Sitzungen in externem Speicher persistieren

> Spiegeln Sie Agent SDK-Sitzungstranskripte in Ihren eigenen Objektspeicher, Key-Value-Store oder Ihre Datenbank, damit andere Hosts Ihre Sitzungen fortsetzen können.

Standardmäßig schreibt das SDK Sitzungstranskripte in JSONL-Dateien unter `~/.claude/projects/` im lokalen Dateisystem. Ein `SessionStore`-Adapter ermöglicht es Ihnen, diese Transkripte in Ihrem eigenen Backend zu spiegeln, z. B. in einem Objektspeicher, einem Key-Value-Store oder einer Datenbank, sodass eine auf einem Host erstellte Sitzung auf einem anderen Host mit einem übereinstimmenden Arbeitsverzeichnis fortgesetzt werden kann.

Häufige Gründe für die Verwendung eines Session Store:

* **Multi-Host-Bereitstellungen.** Serverlose Funktionen, automatisch skalierte Worker und CI-Runner teilen sich kein Dateisystem. Ein gemeinsamer Store ermöglicht es Replikaten, die Sitzungen voneinander fortzusetzen.
* **Dauerhaftigkeit.** Lokale Container sind kurzlebig. Ein externer Store übersteht Neustarts und Neubereitstellungen.
* **Compliance und Audit.** Bewahren Sie Transkripte in Speicher auf, den Sie bereits kontrollieren, mit Ihren eigenen Aufbewahrungsrichtlinien, Verschlüsselung und Zugriffskontrolle.

<h2 id="the-sessionstore-interface">
  Die `SessionStore`-Schnittstelle
</h2>

Ein `SessionStore` ist ein Objekt mit zwei erforderlichen Methoden, `append` und `load`, und vier optionalen Methoden. Das SDK ruft `append` auf, um Transkripteinträge während einer Abfrage zu schreiben, und `load`, um sie zum Fortsetzen zurückzulesen.

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

`SessionKey` adressiert ein Transkript. `projectKey` ist eine stabile, dateisystemsichere Kodierung des Arbeitsverzeichnisses, `sessionId` ist die Sitzungs-UUID, und `subpath` wird gesetzt, wenn der Eintrag zu einem Subagent-Transkript oder einer Sidecar-Datei statt zur Hauptkonversation gehört.

Da `projectKey` das Arbeitsverzeichnis kodiert, müssen Sie zum Fortsetzen oder Fortfahren aus dem Speicher aus einem Arbeitsverzeichnis arbeiten, das dem ursprünglichen Lauf entspricht. In TypeScript können Sie, wenn Sie [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/de/sessions#name-the-project-directory-yourself) neben `CLAUDE_CONFIG_DIR` in der [`env`-Option](/docs/de/agent-sdk/typescript#options) einer Abfrage setzen, das SDK diese Abfrage und ihre `resume`- und `continue`-Lookups nach diesem Namen statt nach dem Arbeitsverzeichnis schlüsseln. Da eigenständige Hilfsfunktionen wie `listSessions` und `deleteSession` kein `env` annehmen und die Prozessumgebung lesen, setzen Sie `CLAUDE_CONFIG_DIR` und denselben Namen auch in der Host-Prozessumgebung. Erfordert Agent SDK v0.3.234 oder später.

Behandeln Sie `subpath` als einen undurchsichtigen Schlüsselsuffix; er folgt dem On-Disk-Layout, z. B. `subagents/agent-<id>`. Wenn `subpath` nicht definiert ist, bezieht sich der Schlüssel auf das Haupttranskript.

| Methode                | Erforderlich | Aufgerufen wenn                                                                                                                                                                                                                                                                                                                                                                               |
| :--------------------- | :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`               | Ja           | Nach jedem Batch von Transkripteinträgen, die lokal geschrieben werden. Einträge sind JSON-sichere Objekte, eine pro Zeile in der lokalen JSONL.                                                                                                                                                                                                                                              |
| `load`                 | Ja           | Vor dem Spawnen des Subprozesses, wenn `resume` gesetzt ist oder `continue: true` die neueste Speichersitzung auflöst, und einmal pro Sitzung beim Auflisten, wenn auf `listSessionSummaries` zurückgegriffen wird. Geben Sie `null` zurück, wenn die Sitzung unbekannt ist.                                                                                                                  |
| `listSessions`         | Nein         | Von `listSessions({ sessionStore })` und von `query()`/`startup()` mit `continue: true`. Wenn nicht definiert, wirft `continue: true` einen Fehler, und `listSessions({ sessionStore })` wirft einen Fehler, es sei denn, `listSessionSummaries` ist implementiert.                                                                                                                           |
| `listSessionSummaries` | Nein         | Von `listSessions({ sessionStore })`, um Metadaten für alle Sitzungen in einem Aufruf zu lesen. Verwalten Sie die Zusammenfassungen in `append`. Wenn nicht definiert, wird auf `listSessions` plus ein Pro-Sitzungs-`load` zurückgegriffen.                                                                                                                                                  |
| `delete`               | Nein         | Von `deleteSession({ sessionStore })`. Das Löschen des Hauptschlüssels (kein `subpath`) muss auf alle Unterschlüssel für diese Sitzung kaskadieren und auch den Zusammenfassungseintrag der Sitzung entfernen, sodass eine gelöschte Sitzung nicht mehr in `listSessionSummaries` angezeigt wird. Wenn nicht definiert, ist das Löschen ein No-Op, was für Append-Only-Backends geeignet ist. |
| `listSubkeys`          | Nein         | Während des Wiederaufnehmens, um Subagent-Transkripte zu entdecken. Wenn nicht definiert, wird nur das Haupttranskript wiederhergestellt.                                                                                                                                                                                                                                                     |

In einem `SessionSummaryEntry` ist `mtime` die Speicherschreibzeit des Sidecars und muss eine Zeitquelle mit den `mtime`-Werten teilen, die `listSessions` zurückgibt. `data` ist ein undurchsichtiger, vom SDK verwalteter Zustand; speichern Sie ihn wörtlich, ohne ihn zu interpretieren.

Erstellen Sie die Einträge, indem Sie die exportierte `foldSessionSummary`-Hilfsfunktion, `fold_session_summary` in Python, auf jeden Batch in `append` aufrufen. Überspringen Sie Batches, deren Schlüssel einen `subpath` hat; Subagent-Transkripte dürfen nicht zur Zusammenfassung der Hauptsitzung beitragen. Die Fold setzt `mtime` nie: Stempeln Sie es zur Persistierungszeit durch das `options.mtime`-Argument in TypeScript oder durch Überschreiben des Feldes auf dem zurückgegebenen Eintrag in Python. Gleichzeitige `append`-Aufrufe für dieselbe Sitzung können beim Sidecar konkurrieren, daher serialisieren Sie das Lesen-Fold-Schreiben mit einer Transaktion, einem Compare-and-Swap oder einem Pro-Sitzungs-Lock; die Fold selbst ist rein.

Für das, was das SDK mit dem Transkript macht, das `load` zurückgibt, siehe [Aus dem Speicher fortsetzen](#resume-from-the-store).

<h2 id="quick-start">
  Schnellstart
</h2>

Das SDK wird mit einem `InMemorySessionStore` für Entwicklung und Tests ausgeliefert. Das folgende Beispiel führt eine Abfrage mit dem angehängten Store aus, erfasst die Sitzungs-ID aus der Ergebnismeldung und setzt dann aus dem Store in einem zweiten `query()`-Aufruf fort. Der zweite Aufruf übergibt die gleiche Store-Instanz plus `resume`, sodass das SDK das Transkript aus dem Store statt aus dem lokalen Dateisystem lädt:

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

Die zweite Abfrage gibt eine Zusammenfassung der Dateien aus der ersten Abfrage aus, was zeigt, dass der Agent mit vollständigem Kontext aus dem Store fortgesetzt wurde.

<h2 id="write-your-own-adapter">
  Schreiben Sie Ihren eigenen Adapter
</h2>

Implementieren Sie `append` und `load` gegen Ihr Backend. Fügen Sie `listSessions`, `listSessionSummaries`, `delete` und `listSubkeys` hinzu, wenn Sie möchten, dass `listSessions()`, Metadaten-Lesevorgänge mit einem Aufruf, `deleteSession()` und Subagent-Wiederaufnahme gegen den Store funktionieren.

Einträge, die an `append` übergeben werden, sind als `SessionStoreEntry` typisiert (ein `{ type: string; ... }`-Objekt). Behandeln Sie sie als undurchsichtige JSON-sichere Werte: persistieren Sie sie in Reihenfolge und geben Sie sie von `load` in der gleichen Reihenfolge zurück. `load` muss Einträge zurückgeben, die tiefengleich mit dem sind, was angehängt wurde; Byte-gleiche Serialisierung ist nicht erforderlich, daher sind Backends, die Objektschlüssel neu ordnen, wie eine binäre JSON-Spaltentyp, in Ordnung.

<h2 id="reference-implementations">
  Referenzimplementierungen
</h2>

Beide SDK-Repositories enthalten ausführbare Referenz-Adapter unter [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores) in TypeScript und [`examples/session_stores/`](https://github.com/anthropics/claude-agent-sdk-python/tree/main/examples/session_stores) in Python. Es gibt einen Adapter pro Speichertyp, und jeder zeigt, wie `append` und `load` auf diese Art von Backend abgebildet werden. Sie werden nicht als Pakete veröffentlicht; kopieren Sie den Adapter für den Typ, der Ihrem Backend am nächsten kommt, in Ihr Projekt, installieren Sie den Client Ihres Backends, und passen Sie ihn an.

| Speichertyp                                 | Speichermodell                                                                                                                       | Beispiel-Adapter                                                                                                                                                                                                                                           |
| :------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Objektspeicher                              | Eine Teildatei pro `append()`; `load()` listet die Teile auf, sortiert sie und verkettet sie.                                        | S3 ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/s3_session_store.py))                   |
| Schlüssel-Wert-Speicher                     | Eine Liste pro Transkript, die `append()` hinzufügt und `load()` im Bereich liest, plus ein sortierter Index von Sitzungen.          | Redis ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/redis_session_store.py))          |
| Relationale Datenbank oder Dokumentspeicher | Eine Zeile oder ein Dokument pro Eintrag, gespeichert als JSON und geordnet nach einem Schlüssel, der beim Einfügen zugewiesen wird. | Postgres ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/postgres_session_store.py)) |

Jeder Adapter nimmt eine vorkonfigurierte Client-Instanz an, sodass Sie Anmeldedaten, TLS, Region und Pooling kontrollieren. Das folgende Beispiel verbindet den Objektspeicher-Adapter mit `query()` und setzt dann auf einem anderen Host fort:

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

In TypeScript kopieren Sie [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) aus dem Beispielverzeichnis in Ihre Test-Suite. In Python wird die Suite im Paket ausgeliefert. Um sie mit pytest auszuführen, das keine SDK-Abhängigkeit ist, installieren Sie zuerst pytest:

```bash theme={null}
pip install pytest
```

Übergeben Sie dann Ihren Adapter an die Suite in einer Testdatei als eine Fabrik ohne Argumente, die `run_session_store_conformance` einmal pro Vertrag aufruft, um einen neuen Store zu erstellen:

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.anyio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

Die Übergabe der `MyRedisStore`-Klasse selbst, wie in diesem Beispiel, funktioniert, wenn der Konstruktor keine Argumente benötigt. Für einen Adapter, der einen vorkonfigurierten Client benötigt, übergeben Sie stattdessen ein Lambda, das den Store konstruiert. Da die Verträge dieselben Sitzungsschlüssel wiederverwenden, muss jeder Store, den die Fabrik zurückgibt, mit leerem Speicher beginnen. Lassen Sie das Lambda daher isolierten Speicher pro Aufruf bereitstellen, z. B. einen neuen speicherinternen Fake, ein eindeutiges Schlüsselpräfix oder eine neue Test-Datenbank.

<h2 id="behavior-notes">
  Verhaltenshinweise
</h2>

<h3 id="dual-write-architecture">
  Dual-Write-Architektur
</h3>

Der Claude Code-Subprozess schreibt immer zuerst jeden Batch von Transkripteinträgen auf die lokale Festplatte, und das SDK leitet dann denselben Batch an die `append()`-Funktion Ihres Stores weiter, sodass der Store ein Spiegel des lokalen Transkripts ist und nicht dessen Ersatz. Welche Kopie den Durchlauf überlebt, hängt davon ab, wie der Durchlauf gestartet wurde:

* **Neue Sitzung oder Wiederaufnahme, wenn der Store nichts für die Sitzung hat**: Das lokale Transkript in Ihrem Konfigurationsverzeichnis überlebt den Durchlauf, und der Store erhält eine Kopie.
* **Durchlauf [vom Store wiederaufgenommen](#resume-from-the-store)**: Die lokale Kopie wird am Ende des Durchlaufs gelöscht, sodass der Store die einzige dauerhafte Kopie enthält.

Wenn Sie nicht möchten, dass eine neue Sitzung ein Transkript auf der lokalen Festplatte hinterlässt, setzen Sie `CLAUDE_CONFIG_DIR` in `options.env` auf ein temporäres Verzeichnis. Ein Durchlauf, der vom Store wiederaufgenommen wird, löscht bereits seine lokale Kopie, benötigt also keine solche Einstellung. In TypeScript müssen Sie auch `process.env` in `env` verteilen, da die [`env`-Option](/docs/de/agent-sdk/typescript#options) die Subprozessumgebung ersetzt.

Wenn sich Ihre App über Dateien im Konfigurationsverzeichnis anmeldet, z. B. OAuth-Anmeldedaten oder ein `apiKeyHelper` in Ihrer Benutzer-`settings.json`, kopieren Sie diese Dateien zuerst in das temporäre Verzeichnis, oder setzen Sie stattdessen `ANTHROPIC_API_KEY` in `env`. Andernfalls schlägt der Durchlauf mit `Not logged in` fehl.

Zwei Optionen stehen in Konflikt mit dem Spiegel, und das SDK wirft beim Start einen Fehler, wenn Sie eine davon mit einem Store kombinieren:

* **`persistSession: false`** in TypeScript: Deaktiviert die lokalen Schreibvorgänge, auf denen der Spiegel aufgebaut ist. Das Python SDK hat keine entsprechende Option.
* **Datei-Checkpointing**, `enableFileCheckpointing` in TypeScript oder `enable_file_checkpointing` in Python: Schreibt seine Datei-Backups direkt auf die lokale Festplatte, und das SDK spiegelt sie nicht zum Store.

<h3 id="resume-from-the-store">
  Vom Store wiederaufnehmen
</h3>

Wenn Sie `resume` oder `continue: true` in TypeScript oder `continue_conversation=True` in Python zusammen mit einem Store übergeben, fragt das SDK den Store nach einem Transkript, bevor es den Subprozess startet:

* **`resume`**: Das SDK fragt nach der Sitzung, deren ID Sie übergeben haben.
* **`continue: true`** oder **`continue_conversation=True`**: Das SDK fragt nach der neuesten Sitzung des Stores.

Wenn der Store das Transkript zurückgibt, schreibt das SDK es in ein temporäres Konfigurationsverzeichnis, führt den Subprozess mit `CLAUDE_CONFIG_DIR` aus, das dort zeigt, und löscht das Verzeichnis, wenn der Durchlauf endet. Das lokale Transkript, das dieser Durchlauf schreibt, wird mit ihm gelöscht, weshalb der Store auf diesem Pfad die einzige dauerhafte Kopie enthält.

Das SDK startet auch das temporäre Verzeichnis mit Dateien aus Ihrem echten Konfigurationsverzeichnis. Was kopiert wird, unterscheidet sich je nach Sprache:

* **TypeScript**: Anmeldedaten, `.claude.json` und Ihre Benutzer-`settings.json`. Aus `settings.json` entfernt es die Schlüssel, die sich unter einem temporären Konfigurationsverzeichnis nicht richtig verhalten: `enabledPlugins`, `extraKnownMarketplaces`, sein [`additionalMarketplaces`](/docs/de/settings-reference#extraknownmarketplaces)-Alias und alle `CLAUDE_CONFIG_DIR` im `env`-Block der Datei. Vor Agent SDK v0.3.232 entfernte das SDK den Alias nicht. Die in den Einstellungen konfigurierte Authentifizierung, z. B. [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper), funktioniert, wenn Sie vom Store wiederaufnehmen. Vor Agent SDK v0.3.222 kopierte das TypeScript SDK nur Anmeldedaten und `.claude.json`.
* **Python**: Nur Anmeldedaten und `.claude.json`, daher schlägt eine App, die sich über `apiKeyHelper` in Ihrer Benutzer-`settings.json` authentifiziert, mit `Not logged in` fehl, wenn sie vom Store wiederaufgenommen wird. Ein `apiKeyHelper` in verwalteten oder Projekteinstellungen funktioniert immer noch, da Claude Code diese Dateien von Speicherorten liest, die `CLAUDE_CONFIG_DIR` nicht beeinflusst.

Wenn der Store nichts für die Sitzung hat, führt das SDK unter Ihrem echten Konfigurationsverzeichnis aus, und das Ergebnis hängt davon ab, welche Option Sie übergeben haben:

* **`resume`**: Beide SDKs übergeben die ID an den Subprozess, der das lokale Transkript genau wie `resume` ohne Store wiederaufnimmt.
* **`continue: true`** in TypeScript: Das SDK startet eine neue Sitzung.
* **`continue_conversation=True`** in Python: Das SDK setzt die neueste lokale Sitzung fort.

<h3 id="mirror-writes-are-best-effort">
  Spiegelschreibvorgänge sind Best-Effort
</h3>

Wenn `append()` ablehnt, versucht das SDK den Batch bis zu zwei weitere Male mit kurzer Backoff-Zeit erneut, insgesamt höchstens drei Versuche. Ein Aufruf, der eine Zeitüberschreitung aufweist, wird nicht erneut versucht, da der ursprüngliche Aufruf möglicherweise noch ankommt. Wenn der Batch immer noch fehlschlägt, protokolliert das SDK den Fehler, gibt eine `{ type: "system", subtype: "mirror_error" }`-Nachricht in den Iterator aus, verwirft den Batch und setzt die Abfrage fort. Da ein erneut versuchter Batch Einträge, die bereits angekommen sind, erneut bereitstellen kann, deduplizieren Sie nach `entry.uuid` in Ihrer `append()`-Implementierung.

Ein Store-Ausfall unterbricht den Agenten nicht, da der Subprozess zuerst lokal schreibt. Überwachen Sie auf `mirror_error`, wenn Sie Datenverluste im Store erkennen müssen. Bei einem Durchlauf, der [vom Store wiederaufgenommen wird](#resume-from-the-store), hat ein verworfener Batch keine überlebende Kopie, sobald der Durchlauf endet.

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

Das SDK löscht niemals von selbst aus Ihrem Store. Die Aufbewahrung ist die Verantwortung des Adapters: verwenden Sie die Ablauf- oder Lebenszyklusmechanismen Ihres Backends oder führen Sie geplante Bereinigung gemäß Ihren Compliance-Anforderungen durch.

Lokale Transkripte unter `CLAUDE_CONFIG_DIR` werden unabhängig durch die `cleanupPeriodDays`-Einstellung bereinigt, nach den [Aufbewahrungssweep-Regeln](/docs/de/claude-directory#cleaned-up-automatically). Ein Durchlauf, der [vom Store wiederaufgenommen wird](#resume-from-the-store), hinterlässt kein lokales Transkript, daher ist für diese Durchläufe die Aufbewahrung Ihres Stores die einzige Aufbewahrung, die es gibt.

<h2 id="supported-on">
  Unterstützt auf
</h2>

Die folgenden TypeScript SDK-Funktionen akzeptieren eine `sessionStore`-Option und arbeiten gegen den Store statt gegen das lokale Dateisystem, wenn es bereitgestellt wird:

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

Im Python SDK setzen Sie `session_store` in [`ClaudeAgentOptions`](/docs/de/agent-sdk/python#claudeagentoptions), um `query()` gegen einen Store auszuführen. Die verbleibenden Operationen haben jeweils eine Store-gestützte Python-Funktion, die den Store als Argument akzeptiert: `list_sessions_from_store()`, `get_session_info_from_store()`, `get_session_messages_from_store()`, `list_subagents_from_store()`, `get_subagent_messages_from_store()`, `rename_session_via_store()`, `tag_session_via_store()`, `delete_session_via_store()` und `fork_session_via_store()`. `startup()` hat kein Python-Äquivalent. Die eigenständigen Funktionen, die in der [Python SDK-Referenz](/docs/de/agent-sdk/python#functions) dokumentiert sind, wie `list_sessions()`, lesen lokale Sitzungsdateien.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Mit Sitzungen arbeiten](/docs/de/agent-sdk/sessions): Fortsetzen, Wiederaufnehmen und Forken ohne einen benutzerdefinierten Store
* [Das SDK hosten](/docs/de/agent-sdk/hosting): Bereitstellungsmuster für Multi-Host-Umgebungen
* [TypeScript `Options`](/docs/de/agent-sdk/typescript#options): Vollständige Optionsreferenz
* [Referenzimplementierungen](#reference-implementations): Ausführbare Beispieladapter für einen Objektspeicher, einen Schlüssel-Wert-Speicher und eine Datenbank in beiden SDK-Repositories
