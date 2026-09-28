> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Suivre les coûts et l'utilisation

> Découvrez comment suivre l'utilisation des tokens, estimer les coûts et configurer la mise en cache des invites avec le Claude Agent SDK.

Le Claude Agent SDK fournit des informations détaillées sur l'utilisation des tokens pour chaque interaction avec Claude. Ce guide explique comment suivre correctement l'utilisation et comprendre les rapports de coûts, en particulier lorsqu'il s'agit d'utilisations d'outils parallèles et de conversations multi-étapes.

Pour la documentation complète de l'API, consultez la [référence du SDK TypeScript](/docs/fr/agent-sdk/typescript) et la [référence du SDK Python](/docs/fr/agent-sdk/python).

<Warning>
  Les champs `total_cost_usd` et `costUSD` sont des estimations côté client, pas des données de facturation faisant autorité. Le SDK les calcule localement à partir d'une table de prix fournie au moment de la compilation, sauf si une table [`modelPricing`](/docs/fr/settings-reference#modelpricing) est en vigueur. Ils peuvent diverger de ce que vous êtes réellement facturé lorsque :

  * les prix changent
  * la version du SDK installée ne reconnaît pas un modèle
  * des règles de facturation s'appliquent que le client ne peut pas modéliser

  Une règle de facturation que le SDK modélise est la [tarification de la résidence des données](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing). Lorsque la `usage` d'une réponse rapporte `inference_geo: "us"`, le SDK multiplie le prix catalogue des tokens de cette réponse par 1,1. Les frais par requête tels que la recherche web ne sont pas multipliés. Nécessite le SDK Agent TypeScript v0.3.239 ou ultérieur, ou le SDK Agent Python v0.2.144 ou ultérieur.

  Utilisez ces champs pour obtenir des informations de développement et un budget approximatif. Pour la facturation faisant autorité, utilisez l'[API d'utilisation et de coûts](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) ou la page Utilisation dans la [Console Claude](https://platform.claude.com/usage). Ne facturez pas les utilisateurs finaux et ne déclenchez pas de décisions financières à partir de ces champs.
</Warning>

<h2 id="understand-token-usage">
  Comprendre l'utilisation des tokens
</h2>

Les SDKs TypeScript et Python exposent les mêmes données d'utilisation avec des noms de champs différents :

* **TypeScript** fournit des ventilations de tokens par étape sur chaque message d'assistant (`message.message.id`, `message.message.usage`), le coût par modèle via `modelUsage` sur le message de résultat, et un total cumulatif sur le message de résultat.
* **Python** fournit des ventilations de tokens par étape sur chaque message d'assistant en tant que `message.usage` et `message.message_id`, le coût par modèle via `model_usage` sur le message de résultat, et le total cumulatif sur le message de résultat en tant que `total_cost_usd`.

Les deux SDKs utilisent le même modèle de coût sous-jacent et exposent la même granularité. La différence réside dans la dénomination des champs et dans le lieu où l'utilisation par étape est imbriquée.

Le suivi des coûts dépend de la compréhension de la façon dont le SDK délimite les données d'utilisation :

* **Appel `query()` :** une invocation de la fonction `query()` du SDK. Un seul appel peut impliquer plusieurs étapes : Claude répond, utilise des outils, obtient des résultats et répond à nouveau. Chaque appel produit un message [`result`](/docs/fr/agent-sdk/typescript#sdkresultmessage) à la fin, sauf en [mode d'entrée en streaming](/docs/fr/agent-sdk/streaming-vs-single-mode), où un appel `query()` porte plusieurs tours d'utilisateur et chaque tour émet son propre message `result`.
* **Étape :** un seul cycle requête/réponse au sein d'un appel `query()`. Chaque étape produit des messages d'assistant avec l'utilisation des tokens.
* **Session :** une série d'appels `query()` liés par un ID de session via l'option `resume`. Les résultats d'un appel repris rapportent la dépense totale de la session, pas seulement celle de cet appel. Consultez [Accumuler les coûts sur plusieurs appels](#accumulate-costs-across-multiple-calls) pour voir comment les totaux se reportent.

Le diagramme suivant montre le flux de messages d'un seul appel `query()`, avec l'utilisation des tokens rapportée à chaque étape et l'estimation cumulative à la fin :

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" className="dark:hidden" alt="Diagram showing a query producing two steps of messages. Step 1 has four assistant messages sharing the same ID and usage (count once), Step 2 has one assistant message with a new ID, and the final result message shows the estimated total_cost_usd." width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/message-usage-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=8ea95085abc0a6b7f55ecef498bd4d14" className="hidden dark:block" alt="Diagram showing a query producing two steps of messages. Step 1 has four assistant messages sharing the same ID and usage (count once), Step 2 has one assistant message with a new ID, and the final result message shows the estimated total_cost_usd." width="760" height="520" data-path="images/agent-sdk/message-usage-flow-dark.svg" />

<Steps>
  <Step title="Chaque étape produit des messages d'assistant">
    Quand Claude répond, il envoie un ou plusieurs messages d'assistant. En TypeScript, chaque message d'assistant contient un `BetaMessage` imbriqué (accessible via `message.message`) avec un `id` et un objet [`usage`](https://platform.claude.com/docs/en/api/messages) avec les comptages de tokens (`input_tokens`, `output_tokens`). En Python, la classe de données `AssistantMessage` expose les mêmes données directement via `message.usage` et `message.message_id`. Quand Claude utilise plusieurs outils en un seul tour, tous les messages de ce tour partagent le même ID, donc dédupliquez par ID pour éviter le double comptage.
  </Step>

  <Step title="Le message de résultat fournit l'estimation cumulative">
    Quand l'appel `query()` se termine, le SDK émet un message de résultat avec `total_cost_usd` et l'utilisation cumulative `usage`, typé en tant que [`SDKResultMessage`](/docs/fr/agent-sdk/typescript#sdkresultmessage) en TypeScript et [`ResultMessage`](/docs/fr/agent-sdk/python#resultmessage) en Python. Si vous avez seulement besoin du total estimé, vous pouvez ignorer l'utilisation par étape et lire cette valeur unique.

    Si vous effectuez plusieurs appels `query()` indépendants, chaque résultat reflète uniquement le coût de cet appel individuel. Un appel qui reprend une session compte également la dépense antérieure de la session.

    En mode d'entrée en streaming, chaque tour émet son propre message de résultat. Consultez [Suivre les coûts en mode d'entrée en streaming](#track-costs-in-streaming-input-mode) pour savoir comment lire les totaux d'appels dans ce mode.
  </Step>
</Steps>

<h2 id="track-costs-in-streaming-input-mode">
  Suivre les coûts en mode d'entrée en continu
</h2>

En [mode d'entrée en continu](/docs/fr/agent-sdk/streaming-vs-single-mode), un appel `query()` porte plusieurs tours d'utilisateur et chaque tour émet son propre message de résultat. Les champs de résultat diffèrent en portée :

* **`usage`** : couvre uniquement ce tour, et au sein de celui-ci uniquement la boucle principale de l'agent, pas les sous-agents qu'il a exécutés.
* **`total_cost_usd` et `modelUsage`, ou `model_usage` en Python** : portent le total cumulé pour l'ensemble de l'appel jusqu'à présent, plus toute dépense restaurée lorsque l'appel a repris une session.

Dans un appel où votre application n'envoie jamais `/clear`, `/reset`, ou `/new`, lisez le dernier résultat pour les totaux d'appel plutôt que de les additionner entre les résultats.

Les totaux cumulés recommencent à zéro chaque fois que votre application envoie l'une de ces trois commandes, et à l'intérieur d'un appel `query()` rien d'autre ne les réinitialise. Trois résultats importent pour votre comptabilité :

* **Le propre résultat du tour `/clear`** : couvre uniquement ce qui s'est exécuté depuis la réinitialisation, et porte un nouveau `session_id`.
* **Chaque résultat ultérieur** : continue à compter à partir de cette réinitialisation.
* **Le dernier résultat avant chaque `/clear`** : contient le total pour les tours depuis la réinitialisation précédente.

Pour totaliser l'ensemble de l'appel, ajoutez le dernier résultat avant chaque `/clear` au résultat final de l'appel. Tous les autres résultats, y compris celui du tour `/clear`, sont remplacés par un résultat ultérieur.

En TypeScript, le SDK émet également un [`SDKConversationResetMessage`](/docs/fr/agent-sdk/typescript#sdkconversationresetmessage) à chaque réinitialisation, vous permettant de détecter les réinitialisations à partir du flux. En Python, le SDK émet de même un `ConversationResetMessage`. Avant la version 0.2.137 du SDK Python, l'itérateur Python supprimait ce message, donc sur ces versions comptez les réinitialisations vous-même à partir des tours `/clear` que votre application envoie.

`maxBudgetUsd` (TypeScript) ou `max_budget_usd` (Python) compte uniquement la dépense propre de l'appel : les totaux restaurés à partir d'une session reprise ne comptent pas contre lui, et un `/clear` recommence le budget.

<h2 id="get-the-total-cost-of-a-query">
  Obtenir le coût total d'une requête
</h2>

Le message de résultat, typé comme [`SDKResultMessage`](/docs/fr/agent-sdk/typescript#sdkresultmessage) en TypeScript et [`ResultMessage`](/docs/fr/agent-sdk/python#resultmessage) en Python, marque la fin de la boucle d'agent pour un appel `query()`. Il inclut `total_cost_usd`, le coût estimé cumulatif sur toutes les étapes de cet appel. Un appel qui reprend une session compte également les dépenses antérieures de la session. Deux mises en garde s'appliquent lorsque vous lisez la valeur :

* En Python, le champ est typé comme optionnel, donc vérifiez qu'il n'est pas `None` avant de le lire.
* Les résultats de succès et d'erreur le portent tous les deux, bien que le résultat final d'un [plantage de session](#recover-totals-after-a-session-crash) peut le porter à zéro.

En mode d'entrée en streaming, lisez les totaux d'appels comme décrit dans [Suivre les coûts en mode d'entrée en streaming](#track-costs-in-streaming-input-mode).

Les trois champs au niveau du résultat diffèrent dans ce qu'ils comptent lorsque l'agent génère des [sous-agents](/docs/fr/agent-sdk/subagents). Utilisez `modelUsage`, ou `model_usage` en Python, pour la comptabilité des jetons de l'arborescence complète ; le champ `usage` sous-compte dès que l'imbrication se produit.

| Champ                        | Activité des sous-agents                                                                                                                     |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `usage`                      | Exclue. Compte uniquement la boucle d'agent de niveau supérieur, donc les jetons consommés à l'intérieur des sous-agents ne sont pas ajoutés |
| `total_cost_usd`             | Incluse. Compte les demandes de sous-agents aux côtés de la boucle de niveau supérieur                                                       |
| `modelUsage` / `model_usage` | Incluse. Compte les demandes de sous-agents aux côtés de la boucle de niveau supérieur, ventilées par modèle                                 |

En [mode d'entrée de message unique](/docs/fr/agent-sdk/streaming-vs-single-mode#single-message-input), lorsque les sous-agents d'arrière-plan s'exécutent toujours à la fin du dernier tour, Claude Code les attend, jusqu'à la limite décrite dans [tâches d'arrière-plan à la sortie](/docs/fr/headless#background-tasks-at-exit), avant d'émettre le résultat. Le `total_cost_usd`, `duration_api_ms` et `modelUsage` du résultat, ou `model_usage` en Python, incluent le travail effectué pendant cette attente.

Les exemples suivants itèrent sur le flux de messages d'un appel `query()` et impriment le coût total à l'arrivée du message `result` :

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
          # failure was an error result, the branch above has already run;
          # connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

Pour limiter le montant que les sous-agents peuvent ajouter à `total_cost_usd`, définissez les [limites de profondeur, de concurrence et de dépenses](/docs/fr/agent-sdk/subagents#cap-subagent-depth-concurrency-and-spend) sur la requête.

<h2 id="track-per-step-and-per-model-usage">
  Suivre l'utilisation par étape et par modèle
</h2>

Les exemples de cette section utilisent les noms de champs TypeScript. En Python, les champs équivalents sont [`AssistantMessage.usage`](/docs/fr/agent-sdk/python#assistantmessage) et `AssistantMessage.message_id` pour l'utilisation par étape, et [`ResultMessage.model_usage`](/docs/fr/agent-sdk/python#resultmessage) pour les ventilations par modèle.

<h3 id="track-per-step-usage">
  Suivre l'utilisation par étape
</h3>

Chaque message d'assistant contient un `BetaMessage` imbriqué (accessible via `message.message`) avec un `id` et un objet `usage` contenant les comptages de jetons. Lorsque Claude utilise des outils en parallèle, plusieurs messages partagent le même `id` avec des données d'utilisation identiques. Suivez les ID que vous avez déjà comptabilisés et ignorez les doublons pour éviter des totaux gonflés.

<Warning>
  Les valeurs par étape dédupliquées sont exactes pour les jetons d'entrée et de cache. Le `output_tokens` par étape est un espace réservé, donc [lisez les jetons de sortie du message de résultat](#read-output-tokens-from-the-result-message).
</Warning>

L'exemple suivant accumule les jetons d'entrée sur toutes les étapes, en comptant chaque ID de message de boucle principale unique une seule fois et en ignorant les messages des sous-agents, et lit le total de sortie du message de résultat, qui couvre la boucle principale :

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const seenIds = new Set<string>();
let totalInputTokens = 0;
let resultOutputTokens = 0;

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type === "assistant" && !message.parent_tool_use_id) {
      const msgId = message.message.id;

      // Parallel tool calls share the same ID, only count once
      if (!seenIds.has(msgId)) {
        seenIds.add(msgId);
        totalInputTokens += message.message.usage.input_tokens;
      }
    }
    if (message.type === "result") {
      // Per-step output_tokens is a placeholder; the result message
      // carries the accumulated output total.
      resultOutputTokens = message.usage.output_tokens;
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result, so the
  // input total below still reflects the steps that ran before the failure.
  console.error(`Session ended with an error: ${error}`);
}

console.log(`Steps: ${seenIds.size}`);
console.log(`Input tokens: ${totalInputTokens}`);
console.log(`Output tokens: ${resultOutputTokens}`);
```

<h3 id="break-down-usage-per-model">
  Ventiler l'utilisation par modèle
</h3>

Le message de résultat inclut [`modelUsage`](/docs/fr/agent-sdk/typescript#modelusage), une carte du nom du modèle aux comptages de jetons par modèle et au coût. Ceci est utile lorsque vous exécutez plusieurs modèles (par exemple, Haiku pour les sous-agents et Opus pour l'agent principal) et que vous souhaitez voir où vont les jetons.

Le `costBasis` de chaque entrée indique quelle table de prix a tarifé la dernière demande de ce modèle : `list` pour le prix catalogue, `managed` pour une table [`modelPricing`](/docs/fr/settings-reference#modelpricing), ou `unknown` lorsqu'aucune ne correspondait à l'ID du modèle. Le champ nécessite Claude Code v2.1.246 ou version ultérieure.

L'exemple suivant exécute une requête et affiche la ventilation des coûts et des jetons pour chaque modèle utilisé :

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
  Accumuler les coûts sur plusieurs appels
</h2>

Chaque appel `query()` retourne `total_cost_usd` sur ses résultats. La façon dont vous combinez les valeurs dépend de si les appels partagent une session :

* **Appels indépendants, sans option `resume` ou `continue`** : chaque résultat couvre uniquement son propre appel, donc additionnez les totaux vous-même, comme le font les exemples ci-dessous.
* **Appels qui reprennent la même session** : Claude Code enregistre les totaux de la session dans sa [transcription](/docs/fr/sessions#where-transcripts-are-stored) lorsque le processus se termine normalement et les restaure lorsqu'un appel ultérieur reprend ou crée une branche de la session. Chaque résultat inclut déjà les dépenses antérieures de la session. Lisez le dernier résultat pour le total de la session ; additionner les résultats double-compte les dépenses restaurées. Avant v2.1.277, une session que vous aviez reprise via le SDK ou `claude -p` commençait ses totaux à zéro, donc chaque appel couvrait uniquement cet appel.

En mode d'entrée en streaming, lisez le total de chaque appel comme décrit dans [Suivre les coûts en mode d'entrée en streaming](#track-costs-in-streaming-input-mode). Pour un appel qui s'est terminé par un crash, voir [Récupérer les totaux après un crash de session](#recover-totals-after-a-session-crash).

Les exemples suivants exécutent deux appels `query()` séquentiellement, ajoutent le `total_cost_usd` de chaque appel à un total cumulatif, et affichent à la fois le coût par appel et le coût combiné :

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

<h2 id="handle-errors-caching-and-output-token-counts">
  Gérer les erreurs, la mise en cache et les comptages de jetons de sortie
</h2>

Pour un suivi précis des coûts, tenez compte du comptage de sortie fictif sur les messages d'assistant, des jetons qu'une conversation échouée a consommés et de la tarification des jetons en cache.

<h3 id="read-output-tokens-from-the-result-message">
  Lire les jetons de sortie du message de résultat
</h3>

Claude Code construit chaque message d'assistant à partir de l'utilisation que l'API a signalée au début de la réponse, de sorte que le `output_tokens` du message n'est que le comptage que l'API avait signalé à `message_start`, avant que la réponse ne soit générée. Une réponse API peut produire plusieurs messages d'assistant, et chacun d'eux porte ce même placeholder.

L'API signale le comptage de sortie réel à la fin de la réponse, et Claude Code l'ajoute au message de résultat. Lisez les jetons de sortie à partir de `usage` du résultat, ou à partir de `modelUsage` pour une ventilation par modèle.

Pour regarder le comptage de sortie d'une réponse augmenter pendant qu'elle est diffusée en continu, définissez `includePartialMessages`, ou `include_partial_messages` en Python, et lisez `usage` à partir de chaque événement de flux `message_delta`, typé comme [`SDKPartialAssistantMessage`](/docs/fr/agent-sdk/typescript#sdkpartialassistantmessage) en TypeScript et [`StreamEvent`](/docs/fr/agent-sdk/python#streamevent) en Python.

<h3 id="track-costs-on-failed-conversations">
  Suivre les coûts sur les conversations échouées
</h3>

Les messages de résultat de succès et d'erreur incluent tous deux `usage` et `total_cost_usd` ; en Python, les deux champs sont typés comme optionnels, donc vérifiez qu'ils ne sont pas `None` avant de les lire.

Si une conversation échoue à mi-chemin, vous avez toujours consommé des jetons jusqu'au point de défaillance. Lisez les données de coût à partir de chaque message de résultat, que son `subtype` soit `success` ou l'un des sous-types d'erreur. Sur certains résultats d'erreur, `usage` signale moins que ce que l'appel a dépensé :

* **`error_during_execution` après un [plantage de session](#recover-totals-after-a-session-crash)** : chaque champ de coût peut être mis à zéro.
* **`error_max_budget_usd`** : `usage` omet la réponse qui a dépassé le budget, tandis que `total_cost_usd` et `modelUsage` l'incluent.

Lorsque vous avez le choix, comptabilisez à partir de `total_cost_usd` ou `modelUsage` plutôt que de `usage`.

<h3 id="recover-totals-after-a-session-crash">
  Récupérer les totaux après un plantage de session
</h3>

Lorsque le processus Claude Code plante, il émet un résultat `error_during_execution` final et se termine, en mode d'entrée unique et en mode d'entrée en continu. Ce résultat peut porter des `usage`, `total_cost_usd` et `modelUsage` mis à zéro, donc récupérez les totaux de l'appel à partir de ce qui est arrivé avant. L'étape 1 récupère les totaux complets chaque fois qu'un résultat antérieur existe ; le secours à l'étape 2 récupère uniquement les jetons d'entrée et de cache de la boucle principale.

1. Utilisez le résultat du tour avant le plantage. En mode d'entrée en continu, il contient le total cumulé décrit dans [Suivre les coûts en mode d'entrée en continu](#track-costs-in-streaming-input-mode). Allez à l'étape 2 à la place lorsque ce résultat ne peut pas vous aider :
   * L'appel était unique, donc aucun résultat antérieur n'existe.
   * Le plantage s'est produit au premier tour.
   * Le tour avant le plantage était le `/clear` lui-même, donc son résultat couvre uniquement la réinitialisation.
2. Additionnez plutôt le `usage` sur les messages d'assistant, en comptant chaque réponse API une fois, comme l'exemple [Track per-step usage](#track-per-step-usage) le fait. En mode unique, additionnez-les tous ; en mode d'entrée en continu, additionnez ceux qui sont arrivés après le dernier résultat. Cela vous donne les jetons d'entrée et de cache de la boucle principale. L'utilisation des sous-agents n'est pas récupérable de cette façon, pas plus que les jetons de sortie ou le coût en USD, car [le `output_tokens` par étape est un placeholder](#read-output-tokens-from-the-result-message).

<h3 id="track-cache-tokens">
  Suivre les jetons en cache
</h3>

Le SDK Agent utilise automatiquement la [mise en cache des invites](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) pour réduire les coûts sur le contenu répété. Vous n'avez pas besoin de configurer la mise en cache vous-même. L'objet d'utilisation inclut deux champs supplémentaires pour le suivi du cache :

* `cache_creation_input_tokens` : jetons utilisés pour créer de nouvelles entrées de cache (facturés à un taux plus élevé que les jetons d'entrée standard).
* `cache_read_input_tokens` : jetons lus à partir des entrées de cache existantes (facturés à un taux réduit).

Suivez-les séparément de `input_tokens` pour comprendre les économies de mise en cache. En TypeScript, ces champs sont typés sur l'objet [`Usage`](/docs/fr/agent-sdk/typescript#usage). En Python, ils apparaissent comme des clés dans le dictionnaire [`ResultMessage.usage`](/docs/fr/agent-sdk/python#resultmessage) (par exemple, `message.usage.get("cache_read_input_tokens", 0)`).

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  Étendre le TTL du cache d'invite à une heure
</h3>

Vos propres tours se situent dans le [bucket TTL de conversation principal](/docs/fr/prompt-caching#which-ttl-each-request-gets), ainsi que les assistants que Claude Code exécute en ligne avec eux. Les demandes que Claude Code effectue en dehors de cette conversation, telles que les [sous-agents](/docs/fr/agent-sdk/subagents), ont un [contrôle TTL séparé](/docs/fr/prompt-caching#choose-the-ttl-yourself).

Les entrées de cache pour vos propres tours utilisent un TTL de 5 minutes par défaut lorsque vous vous authentifiez avec une clé API ou que vous exécutez sur Amazon Bedrock, la plateforme d'agent de Google Cloud, Microsoft Foundry, ou [Claude Platform on AWS](/docs/fr/claude-platform-on-aws). Si votre charge de travail exécute de nombreuses sessions courtes contre le même invite système et contexte avec des écarts plus longs que 5 minutes entre elles, le cache expire entre les sessions et chaque nouvelle session paie le prix d'entrée complet.

Pour demander un TTL d'1 heure sur les écritures de cache, définissez la variable d'environnement [`ENABLE_PROMPT_CACHING_1H`](/docs/fr/env-vars). Vous pouvez l'exporter dans votre environnement shell ou conteneur, ou la transmettre via `options.env`.

L'exemple suivant active le TTL d'1 heure pour un agent s'exécutant sur Amazon Bedrock. Parce qu'il définit `CLAUDE_CODE_USE_BEDROCK`, il nécessite des identifiants AWS fonctionnels pour [Amazon Bedrock](/docs/fr/amazon-bedrock) ; sans eux, la requête échoue.

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

Les écritures de cache avec un TTL d'1 heure sont facturées à un taux plus élevé que les écritures de 5 minutes, donc l'activation de ceci échange un coût d'écriture plus élevé pour plus de lectures de cache. Consultez la [tarification de la mise en cache des invites](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) pour plus de détails. Sur un abonnement Claude dans l'utilisation incluse de votre plan, vous obtenez le TTL d'1 heure sur vos propres tours, et sur certaines des demandes d'assistance que Claude Code effectue à côté d'eux, sans définir cette variable, et Claude Code réduit ces tours au TTL de 5 minutes une fois que vous tirez sur les [crédits d'utilisation](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans).

`ENABLE_PROMPT_CACHING_1H` demande le TTL d'1 heure sur chaque demande dans les deux buckets. Pour choisir un TTL pour chaque bucket séparément, utilisez plutôt ces contrôles. Chacun prend `5m` ou `1h` et a la priorité sur `ENABLE_PROMPT_CACHING_1H` :

* Conversation principale : la variable d'environnement [`CLAUDE_CODE_PROMPT_CACHE_TTL`](/docs/fr/env-vars), ou le paramètre [`promptCacheTtl`](/docs/fr/settings-reference#promptcachettl)
* Tout le reste : la variable d'environnement `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`, ou le paramètre [`subagentPromptCacheTtl`](/docs/fr/settings-reference#subagentpromptcachettl)

La définition de `promptCacheTtl` à `1h` maintient le cache d'1 heure sur la conversation principale tandis que vous tirez sur les crédits d'utilisation. Pour l'ordre de précédence complet, consultez [choisir le TTL vous-même](/docs/fr/prompt-caching#choose-the-ttl-yourself).

<h2 id="related-documentation">
  Documentation connexe
</h2>

* [Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript) - Documentation complète de l'API
* [Aperçu du SDK](/docs/fr/agent-sdk/overview) - Prise en main du SDK
* [Permissions du SDK](/docs/fr/agent-sdk/permissions) - Gestion des permissions des outils
