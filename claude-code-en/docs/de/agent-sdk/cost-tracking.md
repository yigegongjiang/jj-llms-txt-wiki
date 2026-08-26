> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kosten und Nutzung verfolgen

> Erfahren Sie, wie Sie die Token-Nutzung verfolgen, Kosten schätzen und Prompt Caching mit dem Claude Agent SDK konfigurieren.

Das Claude Agent SDK bietet detaillierte Token-Nutzungsinformationen für jede Interaktion mit Claude. Dieser Leitfaden erklärt, wie Sie die Nutzung ordnungsgemäß verfolgen und die Kostenberichterstattung verstehen, besonders bei parallelen Tool-Verwendungen und mehrstufigen Gesprächen.

Für die vollständige API-Dokumentation siehe die [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript) und [Python SDK-Referenz](/docs/de/agent-sdk/python).

<Warning>
  Die Felder `total_cost_usd` und `costUSD` sind clientseitige Schätzungen, keine verbindlichen Abrechnungsdaten. Das SDK berechnet sie lokal aus einer Preistabelle, die zur Build-Zeit gebündelt wird, daher können sie von dem abweichen, was Sie tatsächlich abgerechnet bekommen, wenn:

  * sich die Preise ändern
  * die installierte SDK-Version ein Modell nicht erkennt
  * Abrechnungsregeln gelten, die der Client nicht modellieren kann

  Verwenden Sie diese Felder für Entwicklungseinblicke und ungefähre Budgetierung. Für verbindliche Abrechnung verwenden Sie die [Usage and Cost API](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) oder die Seite „Nutzung" in der [Claude Console](https://platform.claude.com/usage). Berechnen Sie Endbenutzer nicht und treffen Sie keine finanziellen Entscheidungen basierend auf diesen Feldern.
</Warning>

<h2 id="understand-token-usage">
  Token-Nutzung verstehen
</h2>

Die TypeScript- und Python-SDKs stellen die gleichen Nutzungsdaten mit unterschiedlichen Feldnamen bereit:

* **TypeScript** bietet Token-Aufschlüsselungen pro Schritt auf jeder Assistenten-Nachricht (`message.message.id`, `message.message.usage`), Kosten pro Modell über `modelUsage` auf der Ergebnis-Nachricht und eine kumulative Summe auf der Ergebnis-Nachricht.
* **Python** bietet Token-Aufschlüsselungen pro Schritt auf jeder Assistenten-Nachricht (`message.usage`, `message.message_id`), Kosten pro Modell über `model_usage` auf der Ergebnis-Nachricht und die akkumulierte Summe auf der Ergebnis-Nachricht (`total_cost_usd` und `usage` dict).

Beide SDKs verwenden das gleiche zugrunde liegende Kostenmodell und stellen die gleiche Granularität bereit. Der Unterschied liegt in der Feldbennung und wo die Nutzung pro Schritt verschachtelt ist.

Die Kostenverfolgung hängt davon ab, zu verstehen, wie das SDK Nutzungsdaten umfasst:

* **`query()` Aufruf:** eine Invokation der `query()` Funktion des SDK. Ein einzelner Aufruf kann mehrere Schritte beinhalten (Claude antwortet, verwendet Tools, erhält Ergebnisse, antwortet erneut). Jeder Aufruf erzeugt am Ende eine [`result`](/docs/de/agent-sdk/typescript#sdkresultmessage) Nachricht.
* **Schritt:** ein einzelner Request/Response-Zyklus innerhalb eines `query()` Aufrufs. Jeder Schritt erzeugt Assistenten-Nachrichten mit Token-Nutzung.
* **Sitzung:** eine Serie von `query()` Aufrufen, die durch eine Sitzungs-ID verknüpft sind (mit der `resume` Option). Jeder `query()` Aufruf innerhalb einer Sitzung meldet seine eigenen Kosten unabhängig.

Das folgende Diagramm zeigt den Nachrichtenstrom aus einem einzelnen `query()` Aufruf, mit Token-Nutzung, die bei jedem Schritt gemeldet wird, und der kumulativen Schätzung am Ende:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" alt="Diagramm, das eine Abfrage zeigt, die zwei Schritte von Nachrichten erzeugt. Schritt 1 hat vier Assistenten-Nachrichten, die die gleiche ID und Nutzung teilen (einmal zählen), Schritt 2 hat eine Assistenten-Nachricht mit einer neuen ID, und die endgültige Ergebnis-Nachricht zeigt die geschätzte total_cost_usd." width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<Steps>
  <Step title="Jeder Schritt erzeugt Assistenten-Nachrichten">
    Wenn Claude antwortet, sendet es eine oder mehrere Assistenten-Nachrichten. In TypeScript enthält jede Assistenten-Nachricht eine verschachtelte `BetaMessage` (zugänglich über `message.message`) mit einer `id` und einem [`usage`](https://platform.claude.com/docs/en/api/messages) Objekt mit Token-Zählungen (`input_tokens`, `output_tokens`). In Python stellt die `AssistantMessage` Dataclass die gleichen Daten direkt über `message.usage` und `message.message_id` bereit. Wenn Claude mehrere Tools in einer Runde verwendet, teilen alle Nachrichten in dieser Runde die gleiche ID, daher deduplizieren Sie nach ID, um Doppelzählungen zu vermeiden.
  </Step>

  <Step title="Die Ergebnis-Nachricht bietet die kumulative Schätzung">
    Wenn der `query()` Aufruf abgeschlossen ist, gibt das SDK eine Ergebnis-Nachricht mit `total_cost_usd` und kumulativer `usage` aus. Dies ist in TypeScript ([`SDKResultMessage`](/docs/de/agent-sdk/typescript#sdkresultmessage)) und Python ([`ResultMessage`](/docs/de/agent-sdk/python#resultmessage)) verfügbar. Wenn Sie mehrere `query()` Aufrufe tätigen (zum Beispiel in einer mehrstufigen Sitzung), spiegelt jedes Ergebnis nur die Kosten dieses einzelnen Aufrufs wider. Wenn Sie nur die geschätzte Summe benötigen, können Sie die Nutzung pro Schritt ignorieren und diesen einzelnen Wert lesen.
  </Step>
</Steps>

<h2 id="get-the-total-cost-of-a-query">
  Gesamtkosten einer Abfrage abrufen
</h2>

Die Ergebnis-Nachricht ([TypeScript](/docs/de/agent-sdk/typescript#sdkresultmessage), [Python](/docs/de/agent-sdk/python#resultmessage)) markiert das Ende der Agent-Schleife für einen `query()`-Aufruf. Sie enthält `total_cost_usd`, die geschätzte kumulative Kosten über alle Schritte in diesem Aufruf. Dies funktioniert sowohl für erfolgreiche als auch für Fehler-Ergebnisse. Wenn Sie Sitzungen verwenden, um mehrere `query()`-Aufrufe zu tätigen, spiegelt jedes Ergebnis nur die Kosten dieses einzelnen Aufrufs wider.

Die drei Felder auf Ergebnis-Ebene unterscheiden sich darin, was sie zählen, wenn der Agent [Subagenten](/docs/de/agent-sdk/subagents) erzeugt. Verwenden Sie `modelUsage` oder `model_usage` in Python für die Gesamtbaum-Token-Abrechnung; das Feld `usage` unterschätzt, sobald Verschachtelung auftritt.

| Feld                         | Subagenten-Aktivität                                                                                                                        |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `usage`                      | Ausgeschlossen. Zählt nur die Agent-Schleife auf oberster Ebene, daher werden Token, die in Subagenten verbraucht werden, nicht hinzugefügt |
| `total_cost_usd`             | Eingeschlossen. Zählt Subagenten-Anfragen zusammen mit der Schleife auf oberster Ebene                                                      |
| `modelUsage` / `model_usage` | Eingeschlossen. Zählt Subagenten-Anfragen zusammen mit der Schleife auf oberster Ebene, aufgeschlüsselt nach Modell                         |

Die folgenden Beispiele durchlaufen den Nachrichtenstrom aus einem `query()`-Aufruf und geben die Gesamtkosten aus, wenn die `result`-Nachricht ankommt:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({ prompt: "Summarize this project" })) {
      if (message.type === "result") {
        console.log(`Total cost: $${message.total_cost_usd}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, it still carried total_cost_usd and the
    // branch above has already run; connection or process failures yield
    // no result message.
    console.error(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ResultMessage
  import asyncio


  async def main():
      try:
          async for message in query(prompt="Summarize this project"):
              if isinstance(message, ResultMessage):
                  print(f"Total cost: ${message.total_cost_usd or 0}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, it still carried total_cost_usd and the
          # branch above has already run; connection or process failures yield
          # no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="track-per-step-and-per-model-usage">
  Nutzung pro Schritt und pro Modell verfolgen
</h2>

Die Beispiele in diesem Abschnitt verwenden TypeScript-Feldnamen. In Python sind die entsprechenden Felder [`AssistantMessage.usage`](/docs/de/agent-sdk/python#assistantmessage) und `AssistantMessage.message_id` für die Nutzung pro Schritt, und [`ResultMessage.model_usage`](/docs/de/agent-sdk/python#resultmessage) für Aufschlüsselungen pro Modell.

<h3 id="track-per-step-usage">
  Nutzung pro Schritt verfolgen
</h3>

Jede Assistenten-Nachricht enthält eine verschachtelte `BetaMessage` (zugänglich über `message.message`) mit einer `id` und einem `usage` Objekt mit Token-Zählungen. Wenn Claude Tools parallel verwendet, teilen mehrere Nachrichten die gleiche `id` mit identischen Nutzungsdaten. Verfolgen Sie, welche IDs Sie bereits gezählt haben, und überspringen Sie Duplikate, um aufgeblähte Summen zu vermeiden.

<Warning>
  Parallele Tool-Aufrufe erzeugen mehrere Assistenten-Nachrichten, deren verschachtelte `BetaMessage` die gleiche `id` und identische Nutzung teilt. Deduplizieren Sie immer nach ID, um genaue Token-Zählungen pro Schritt zu erhalten.
</Warning>

Das folgende Beispiel akkumuliert Input- und Output-Tokens über alle Schritte, zählt jede eindeutige Nachrichten-ID nur einmal:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const seenIds = new Set<string>();
let totalInputTokens = 0;
let totalOutputTokens = 0;

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type === "assistant") {
      const msgId = message.message.id;

      // Parallel tool calls share the same ID, only count once
      if (!seenIds.has(msgId)) {
        seenIds.add(msgId);
        totalInputTokens += message.message.usage.input_tokens;
        totalOutputTokens += message.message.usage.output_tokens;
      }
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result, so the
  // totals below still reflect the steps that ran before the failure.
  console.error(`Session ended with an error: ${error}`);
}

console.log(`Steps: ${seenIds.size}`);
console.log(`Input tokens: ${totalInputTokens}`);
console.log(`Output tokens: ${totalOutputTokens}`);
```

<h3 id="break-down-usage-per-model">
  Nutzung pro Modell aufschlüsseln
</h3>

Die Ergebnis-Nachricht enthält [`modelUsage`](/docs/de/agent-sdk/typescript#modelusage), eine Zuordnung von Modellname zu Token-Zählungen und Kosten pro Modell. Dies ist nützlich, wenn Sie mehrere Modelle ausführen (zum Beispiel Haiku für Sub-Agenten und Opus für den Haupt-Agenten) und sehen möchten, wohin die Tokens gehen.

Das folgende Beispiel führt eine Abfrage aus und gibt die Kosten und Token-Aufschlüsselung für jedes verwendete Modell aus:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type !== "result") continue;

    for (const [modelName, usage] of Object.entries(message.modelUsage)) {
      console.log(`${modelName}: $${usage.costUSD.toFixed(4)}`);
      console.log(`  Input tokens: ${usage.inputTokens}`);
      console.log(`  Output tokens: ${usage.outputTokens}`);
      console.log(`  Cache read: ${usage.cacheReadInputTokens}`);
      console.log(`  Cache creation: ${usage.cacheCreationInputTokens}`);
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result. If the
  // failure was an error result, the per-model breakdown above has already
  // printed; connection or process failures yield no result message.
  console.error(`Session ended with an error: ${error}`);
}
```

<h2 id="accumulate-costs-across-multiple-calls">
  Kosten über mehrere Aufrufe akkumulieren
</h2>

Jeder `query()` Aufruf gibt seinen eigenen `total_cost_usd` zurück. Das SDK bietet keine Sitzungs-Ebenen-Summe, daher müssen Sie, wenn Ihre Anwendung mehrere `query()` Aufrufe tätigt (zum Beispiel in einer mehrstufigen Sitzung oder über verschiedene Benutzer), die Summen selbst akkumulieren.

Die folgenden Beispiele führen zwei `query()` Aufrufe nacheinander aus, addieren den `total_cost_usd` jedes Aufrufs zu einer laufenden Summe und geben sowohl die Kosten pro Aufruf als auch die kombinierten Kosten aus:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Track cumulative cost across multiple query() calls
  let totalSpend = 0;

  const prompts = [
    "Read the files in src/ and summarize the architecture",
    "List all exported functions in src/auth.ts"
  ];

  for (const prompt of prompts) {
    try {
      for await (const message of query({ prompt })) {
        if (message.type === "result") {
          totalSpend += message.total_cost_usd;
          console.log(`This call: $${message.total_cost_usd}`);
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. If the
      // failure was an error result, this call's cost was already counted;
      // connection or process failures yield no result message. Continue
      // with the next prompt.
      console.error(`Call failed: ${error}`);
    }
  }

  console.log(`Total spend: $${totalSpend.toFixed(4)}`);
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ResultMessage
  import asyncio


  async def main():
      # Track cumulative cost across multiple query() calls
      total_spend = 0.0

      prompts = [
          "Read the files in src/ and summarize the architecture",
          "List all exported functions in src/auth.ts",
      ]

      for prompt in prompts:
          try:
              async for message in query(prompt=prompt):
                  if isinstance(message, ResultMessage):
                      cost = message.total_cost_usd or 0
                      total_spend += cost
                      print(f"This call: ${cost}")
          except Exception as error:
              # A single-shot query() raises after yielding an error result. If
              # the failure was an error result, this call's cost was already
              # counted; connection or process failures yield no result message.
              # Continue with the next prompt.
              print(f"Call failed: {error}")

      print(f"Total spend: ${total_spend:.4f}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="handle-errors-caching-and-token-discrepancies">
  Fehler, Caching und Token-Diskrepanzen behandeln
</h2>

Für genaue Kostenverfolgung müssen Sie fehlgeschlagene Gespräche, Cache-Token-Preisgestaltung und gelegentliche Berichterstattungsinkonsistenzen berücksichtigen.

<h3 id="resolve-output-token-discrepancies">
  Output-Token-Diskrepanzen auflösen
</h3>

In seltenen Fällen können Sie unterschiedliche `output_tokens` Werte für Nachrichten mit der gleichen ID beobachten. Wenn dies auftritt:

1. **Verwenden Sie den höchsten Wert:** die letzte Nachricht in einer Gruppe enthält typischerweise die genaue Summe.
2. **Bevorzugen Sie die Ergebnis-Nachricht:** die `total_cost_usd` in der Ergebnis-Nachricht spiegelt die akkumulierte Schätzung des SDK über alle Schritte wider, daher ist sie zuverlässiger als die Summe der Werte pro Schritt selbst. Es ist immer noch eine Schätzung und kann von Ihrer tatsächlichen Rechnung abweichen.
3. **Melden Sie Inkonsistenzen:** reichen Sie Probleme im [Claude Code GitHub Repository](https://github.com/anthropics/claude-code/issues) ein.

<h3 id="track-costs-on-failed-conversations">
  Kosten bei fehlgeschlagenen Gesprächen verfolgen
</h3>

Sowohl erfolgreiche als auch Fehler-Ergebnis-Nachrichten enthalten `usage` und `total_cost_usd`. Wenn ein Gespräch in der Mitte fehlschlägt, haben Sie immer noch Tokens bis zum Punkt des Fehlers verbraucht. Lesen Sie Kostendaten immer aus der Ergebnis-Nachricht, unabhängig von ihrem `subtype`.

<h3 id="track-cache-tokens">
  Cache-Tokens verfolgen
</h3>

Das Agent SDK verwendet automatisch [prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching), um Kosten bei wiederholtem Inhalt zu reduzieren. Sie müssen Caching nicht selbst konfigurieren. Das Nutzungs-Objekt enthält zwei zusätzliche Felder für Cache-Verfolgung:

* `cache_creation_input_tokens`: Tokens, die zum Erstellen neuer Cache-Einträge verwendet werden (berechnet zu einem höheren Satz als Standard-Input-Tokens).
* `cache_read_input_tokens`: Tokens, die aus vorhandenen Cache-Einträgen gelesen werden (berechnet zu einem reduzierten Satz).

Verfolgen Sie diese separat von `input_tokens`, um Cache-Einsparungen zu verstehen. In TypeScript sind diese Felder auf dem [`Usage`](/docs/de/agent-sdk/typescript#usage) Objekt typisiert. In Python erscheinen sie als Schlüssel im [`ResultMessage.usage`](/docs/de/agent-sdk/python#resultmessage) dict (zum Beispiel, `message.usage.get("cache_read_input_tokens", 0)`).

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  Prompt-Cache-TTL auf eine Stunde verlängern
</h3>

Cache-Einträge, die vom SDK geschrieben werden, verwenden standardmäßig eine 5-Minuten-TTL, wenn Sie sich mit einem API-Schlüssel authentifizieren oder auf Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry ausführen. Wenn Ihre Workload viele kurze Sitzungen gegen das gleiche System-Prompt und den gleichen Kontext mit Lücken länger als 5 Minuten zwischen ihnen ausführt, läuft der Cache zwischen Sitzungen ab und jede neue Sitzung zahlt den vollen Input-Preis.

Um eine 1-Stunden-TTL bei Cache-Schreibvorgängen anzufordern, setzen Sie die [`ENABLE_PROMPT_CACHING_1H`](/docs/de/env-vars) Umgebungsvariable. Sie können sie in Ihrer Shell- oder Container-Umgebung exportieren oder sie durch `options.env` übergeben.

Das folgende Beispiel aktiviert 1-Stunden-TTL für einen Agenten, der auf Amazon Bedrock ausgeführt wird:

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import ClaudeAgentOptions, query
  import asyncio


  async def main():
      options = ClaudeAgentOptions(
          env={
              "CLAUDE_CODE_USE_BEDROCK": "1",
              "ENABLE_PROMPT_CACHING_1H": "1",
          },
      )

      async for message in query(prompt="Summarize this project", options=options):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const options = {
    env: {
      ...process.env,
      CLAUDE_CODE_USE_BEDROCK: "1",
      ENABLE_PROMPT_CACHING_1H: "1",
    },
  };

  for await (const message of query({ prompt: "Summarize this project", options })) {
    console.log(message);
  }
  ```
</CodeGroup>

Cache-Schreibvorgänge mit einer 1-Stunden-TTL werden zu einem höheren Satz als 5-Minuten-Schreibvorgänge berechnet, daher aktiviert dies einen Austausch von höheren Schreibkosten für mehr Cache-Lesevorgänge. Siehe [prompt caching Preisgestaltung](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) für Details. Claude-Abonnement-Benutzer erhalten bereits automatisch 1-Stunden-TTL und müssen diese Variable nicht setzen.

<h2 id="related-documentation">
  Verwandte Dokumentation
</h2>

* [TypeScript SDK Referenz](/docs/de/agent-sdk/typescript) - Vollständige API-Dokumentation
* [SDK Übersicht](/docs/de/agent-sdk/overview) - Erste Schritte mit dem SDK
* [SDK Berechtigungen](/docs/de/agent-sdk/permissions) - Verwaltung von Tool-Berechtigungen
