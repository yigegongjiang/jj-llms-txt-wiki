> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Suivre les coûts et l'utilisation

> Découvrez comment suivre l'utilisation des tokens, estimer les coûts et configurer la mise en cache des invites avec le Claude Agent SDK.

Le Claude Agent SDK fournit des informations détaillées sur l'utilisation des tokens pour chaque interaction avec Claude. Ce guide explique comment suivre correctement l'utilisation et comprendre les rapports de coûts, en particulier lorsqu'il s'agit d'utiliser des outils en parallèle et de conversations multi-étapes.

Pour la documentation complète de l'API, consultez la [référence du SDK TypeScript](/docs/fr/agent-sdk/typescript) et la [référence du SDK Python](/docs/fr/agent-sdk/python).

<Warning>
  Les champs `total_cost_usd` et `costUSD` sont des estimations côté client, pas des données de facturation faisant autorité. Le SDK les calcule localement à partir d'une table de prix intégrée au moment de la compilation, ils peuvent donc diverger de ce que vous êtes réellement facturé lorsque :

  * les prix changent
  * la version du SDK installée ne reconnaît pas un modèle
  * des règles de facturation s'appliquent que le client ne peut pas modéliser

  Utilisez ces champs pour obtenir des informations de développement et un budget approximatif. Pour une facturation faisant autorité, utilisez l'[API d'utilisation et de coûts](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) ou la page Utilisation dans la [Console Claude](https://platform.claude.com/usage). Ne facturez pas les utilisateurs finaux et ne déclenchez pas de décisions financières à partir de ces champs.
</Warning>

<h2 id="understand-token-usage">
  Comprendre l'utilisation des tokens
</h2>

Les SDK TypeScript et Python exposent les mêmes données d'utilisation avec des noms de champs différents :

* **TypeScript** fournit des ventilations de tokens par étape sur chaque message d'assistant (`message.message.id`, `message.message.usage`), le coût par modèle via `modelUsage` sur le message de résultat, et un total cumulatif sur le message de résultat.
* **Python** fournit des ventilations de tokens par étape sur chaque message d'assistant (`message.usage`, `message.message_id`), le coût par modèle via `model_usage` sur le message de résultat, et le total accumulé sur le message de résultat (`total_cost_usd` et dictionnaire `usage`).

Les deux SDK utilisent le même modèle de coûts sous-jacent et exposent la même granularité. La différence réside dans la dénomination des champs et dans l'endroit où l'utilisation par étape est imbriquée.

Le suivi des coûts dépend de la compréhension de la façon dont le SDK délimite les données d'utilisation :

* **Appel `query()` :** une invocation de la fonction `query()` du SDK. Un seul appel peut impliquer plusieurs étapes (Claude répond, utilise des outils, obtient des résultats, répond à nouveau). Chaque appel produit un message [`result`](/docs/fr/agent-sdk/typescript#sdkresultmessage) à la fin.
* **Étape :** un seul cycle requête/réponse dans un appel `query()`. Chaque étape produit des messages d'assistant avec l'utilisation des tokens.
* **Session :** une série d'appels `query()` liés par un ID de session (en utilisant l'option `resume`). Chaque appel `query()` dans une session rapporte son propre coût indépendamment.

Le diagramme suivant montre le flux de messages d'un seul appel `query()`, avec l'utilisation des tokens rapportée à chaque étape et l'estimation cumulative à la fin :

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" alt="Diagramme montrant une requête produisant deux étapes de messages. L'étape 1 a quatre messages d'assistant partageant le même ID et l'utilisation (compter une fois), l'étape 2 a un message d'assistant avec un nouvel ID, et le message de résultat final affiche le total_cost_usd estimé." width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<Steps>
  <Step title="Chaque étape produit des messages d'assistant">
    Lorsque Claude répond, il envoie un ou plusieurs messages d'assistant. Dans TypeScript, chaque message d'assistant contient un `BetaMessage` imbriqué (accessible via `message.message`) avec un `id` et un objet [`usage`](https://platform.claude.com/docs/en/api/messages) avec des comptages de tokens (`input_tokens`, `output_tokens`). En Python, la classe de données `AssistantMessage` expose les mêmes données directement via `message.usage` et `message.message_id`. Lorsque Claude utilise plusieurs outils en un seul tour, tous les messages de ce tour partagent le même ID, donc dédupliquez par ID pour éviter le double comptage.
  </Step>

  <Step title="Le message de résultat fournit l'estimation cumulative">
    Lorsque l'appel `query()` se termine, le SDK émet un message de résultat avec `total_cost_usd` et `usage` cumulatif. Ceci est disponible à la fois dans TypeScript ([`SDKResultMessage`](/docs/fr/agent-sdk/typescript#sdkresultmessage)) et Python ([`ResultMessage`](/docs/fr/agent-sdk/python#resultmessage)). Si vous effectuez plusieurs appels `query()` (par exemple, dans une session multi-tours), chaque résultat ne reflète que le coût de cet appel individuel. Si vous avez seulement besoin du total estimé, vous pouvez ignorer l'utilisation par étape et lire cette valeur unique.
  </Step>
</Steps>

<h2 id="get-the-total-cost-of-a-query">
  Obtenir le coût total d'une requête
</h2>

Le message de résultat ([TypeScript](/docs/fr/agent-sdk/typescript#sdkresultmessage), [Python](/docs/fr/agent-sdk/python#resultmessage)) marque la fin de la boucle d'agent pour un appel `query()`. Il inclut `total_cost_usd`, le coût estimé cumulatif sur toutes les étapes de cet appel. Cela fonctionne à la fois pour les résultats de succès et d'erreur. Si vous utilisez des sessions pour effectuer plusieurs appels `query()`, chaque résultat ne reflète que le coût de cet appel individuel.

Les trois champs au niveau du résultat diffèrent dans ce qu'ils comptent lorsque l'agent génère des [sous-agents](/docs/fr/agent-sdk/subagents). Utilisez `modelUsage`, ou `model_usage` en Python, pour la comptabilité des jetons de l'arborescence complète ; le champ `usage` sous-compte dès que l'imbrication se produit.

| Champ                        | Activité des sous-agents                                                                                                                     |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `usage`                      | Exclue. Compte uniquement la boucle d'agent de niveau supérieur, donc les jetons consommés à l'intérieur des sous-agents ne sont pas ajoutés |
| `total_cost_usd`             | Incluse. Compte les demandes de sous-agents aux côtés de la boucle de niveau supérieur                                                       |
| `modelUsage` / `model_usage` | Incluse. Compte les demandes de sous-agents aux côtés de la boucle de niveau supérieur, ventilée par modèle                                  |

Les exemples suivants itèrent sur le flux de messages d'un appel `query()` et impriment le coût total lorsque le message `result` arrive :

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
  Suivre l'utilisation par étape et par modèle
</h2>

Les exemples de cette section utilisent les noms de champs TypeScript. En Python, les champs équivalents sont [`AssistantMessage.usage`](/docs/fr/agent-sdk/python#assistantmessage) et `AssistantMessage.message_id` pour l'utilisation par étape, et [`ResultMessage.model_usage`](/docs/fr/agent-sdk/python#resultmessage) pour les ventilations par modèle.

<h3 id="track-per-step-usage">
  Suivre l'utilisation par étape
</h3>

Chaque message d'assistant contient un `BetaMessage` imbriqué (accessible via `message.message`) avec un `id` et un objet `usage` avec des comptages de tokens. Lorsque Claude utilise des outils en parallèle, plusieurs messages partagent le même `id` avec des données d'utilisation identiques. Suivez les ID que vous avez déjà comptés et ignorez les doublons pour éviter des totaux gonflés.

<Warning>
  Les appels d'outils parallèles produisent plusieurs messages d'assistant dont le `BetaMessage` imbriqué partage le même `id` et l'utilisation identique. Dédupliquez toujours par ID pour obtenir des comptages de tokens précis par étape.
</Warning>

L'exemple suivant accumule les tokens d'entrée et de sortie sur toutes les étapes, en comptant chaque ID de message unique une seule fois :

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
  Ventiler l'utilisation par modèle
</h3>

Le message de résultat inclut [`modelUsage`](/docs/fr/agent-sdk/typescript#modelusage), une carte du nom du modèle aux comptages de tokens et coûts par modèle. Ceci est utile lorsque vous exécutez plusieurs modèles (par exemple, Haiku pour les sous-agents et Opus pour l'agent principal) et que vous souhaitez voir où vont les tokens.

L'exemple suivant exécute une requête et imprime le coût et la ventilation des tokens pour chaque modèle utilisé :

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

Chaque appel `query()` retourne son propre `total_cost_usd`. Le SDK ne fournit pas de total au niveau de la session, donc si votre application effectue plusieurs appels `query()` (par exemple, dans une session multi-tours ou entre différents utilisateurs), accumulez les totaux vous-même.

Les exemples suivants exécutent deux appels `query()` séquentiellement, ajoutent le `total_cost_usd` de chaque appel à un total cumulatif, et impriment à la fois le coût par appel et le coût combiné :

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
  Gérer les erreurs, la mise en cache et les divergences de tokens
</h2>

Pour un suivi précis des coûts, tenez compte des conversations échouées, de la tarification des tokens en cache et des incohérences occasionnelles de rapports.

<h3 id="resolve-output-token-discrepancies">
  Résoudre les divergences de tokens de sortie
</h3>

Dans de rares cas, vous pourriez observer différentes valeurs `output_tokens` pour les messages avec le même ID. Lorsque cela se produit :

1. **Utilisez la valeur la plus élevée :** le message final d'un groupe contient généralement le total exact.
2. **Préférez le message de résultat :** le `total_cost_usd` dans le message de résultat reflète l'estimation accumulée du SDK sur toutes les étapes, il est donc plus fiable que de sommer les valeurs par étape vous-même. C'est toujours une estimation et peut différer de votre facture réelle.
3. **Signalez les incohérences :** déposez des problèmes sur le [référentiel GitHub Claude Code](https://github.com/anthropics/claude-code/issues).

<h3 id="track-costs-on-failed-conversations">
  Suivre les coûts sur les conversations échouées
</h3>

Les messages de résultat de succès et d'erreur incluent `usage` et `total_cost_usd`. Si une conversation échoue à mi-chemin, vous avez toujours consommé des tokens jusqu'au point d'échec. Lisez toujours les données de coûts du message de résultat quel que soit son `subtype`.

<h3 id="track-cache-tokens">
  Suivre les tokens en cache
</h3>

Le Agent SDK utilise automatiquement la [mise en cache des invites](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) pour réduire les coûts sur le contenu répété. Vous n'avez pas besoin de configurer la mise en cache vous-même. L'objet d'utilisation inclut deux champs supplémentaires pour le suivi du cache :

* `cache_creation_input_tokens` : tokens utilisés pour créer de nouvelles entrées de cache (facturés à un taux plus élevé que les tokens d'entrée standard).
* `cache_read_input_tokens` : tokens lus à partir des entrées de cache existantes (facturés à un taux réduit).

Suivez-les séparément de `input_tokens` pour comprendre les économies de mise en cache. Dans TypeScript, ces champs sont typés sur l'objet [`Usage`](/docs/fr/agent-sdk/typescript#usage). En Python, ils apparaissent comme des clés dans le dictionnaire [`ResultMessage.usage`](/docs/fr/agent-sdk/python#resultmessage) (par exemple, `message.usage.get("cache_read_input_tokens", 0)`).

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  Prolonger le TTL du cache d'invite à une heure
</h3>

Les entrées de cache écrites par le SDK utilisent un TTL de 5 minutes par défaut lorsque vous vous authentifiez avec une clé API ou que vous exécutez sur Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry. Si votre charge de travail exécute de nombreuses sessions courtes contre le même système d'invite et le même contexte avec des écarts plus longs que 5 minutes entre elles, le cache expire entre les sessions et chaque nouvelle session paie le prix d'entrée complet.

Pour demander un TTL d'une heure sur les écritures de cache, définissez la variable d'environnement [`ENABLE_PROMPT_CACHING_1H`](/docs/fr/env-vars). Vous pouvez l'exporter dans votre environnement shell ou conteneur, ou la transmettre via `options.env`.

L'exemple suivant active le TTL d'une heure pour un agent s'exécutant sur Amazon Bedrock :

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

Les écritures de cache avec un TTL d'une heure sont facturées à un taux plus élevé que les écritures de 5 minutes, donc l'activation de ceci échange un coût d'écriture plus élevé pour plus de lectures de cache. Consultez la [tarification de la mise en cache des invites](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) pour plus de détails. Les utilisateurs d'abonnement Claude reçoivent déjà automatiquement un TTL d'une heure et n'ont pas besoin de définir cette variable.

<h2 id="related-documentation">
  Documentation connexe
</h2>

* [Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript) - Documentation complète de l'API
* [Aperçu du SDK](/docs/fr/agent-sdk/overview) - Prise en main du SDK
* [Permissions du SDK](/docs/fr/agent-sdk/permissions) - Gestion des permissions des outils
