> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Adapter à de nombreux outils avec la recherche d'outils

> Adaptez votre agent à des milliers d'outils en découvrant et chargeant uniquement ce qui est nécessaire, à la demande.

La recherche d'outils permet à votre agent de travailler avec des centaines ou des milliers d'outils en les découvrant et en les chargeant dynamiquement à la demande. Au lieu de charger toutes les définitions d'outils dans la fenêtre de contexte dès le départ, l'agent recherche dans votre catalogue d'outils et charge uniquement les outils dont il a besoin.

Cette approche résout deux défis à mesure que les bibliothèques d'outils se développent :

* **Efficacité du contexte :** Les définitions d'outils peuvent consommer de grandes portions de la fenêtre de contexte (50 outils peuvent utiliser 10-20 K tokens), laissant moins de place pour le travail réel.
* **Précision de la sélection d'outils :** La précision de la sélection d'outils se dégrade avec plus de 30-50 outils chargés à la fois.

<h2 id="how-tool-search-works">
  Fonctionnement de la recherche d'outils
</h2>

La recherche d'outils est activée par défaut, à l'exception des cas énumérés dans [Configurer la recherche d'outils](#configure-tool-search).

Lorsqu'elle est active, les définitions d'outils sont retenues de la fenêtre de contexte. L'agent reçoit un résumé des outils disponibles et recherche les outils pertinents lorsque la tâche nécessite une capacité non déjà chargée. Jusqu'à cinq des outils les plus pertinents sont chargés dans le contexte par défaut, où ils restent disponibles pour les tours suivants jusqu'à ce que le SDK compacte les messages où l'agent les a découverts. Après cette compaction, l'agent recherche à nouveau ces outils lorsqu'il en a besoin.

La recherche d'outils ajoute un aller-retour supplémentaire chaque fois que Claude recherche des outils, mais pour les grands ensembles d'outils, cela est compensé par un contexte plus petit à chaque tour. Avec moins d'environ 10 outils dont les définitions tiennent confortablement dans la fenêtre de contexte, charger tout dès le départ est généralement plus rapide.

Pour plus de détails sur le mécanisme API sous-jacent, consultez [Recherche d'outils dans l'API](https://platform.claude.com/docs/fr/agents-and-tools/tool-use/tool-search-tool).

<Note>
  La recherche d'outils n'est pas prise en charge sur Microsoft Foundry [déploiements hébergés sur Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options), qui la rejettent côté serveur : le SDK détecte le rejet et charge les définitions d'outils dès le départ pour ce déploiement à la place. [`ENABLE_TOOL_SEARCH`](#configure-tool-search) ne peut pas remplacer cela, puisque le rejet provient du déploiement lui-même.
</Note>

<h2 id="configure-tool-search">
  Configurer la recherche d'outils
</h2>

La recherche d'outils est activée par défaut. Pour les modèles sur la liste des modèles non supportés du SDK, le SDK charge les définitions d'outils en amont à la place, et aucune valeur `ENABLE_TOOL_SEARCH` ne remplace cela. Sur Google Cloud's Agent Platform, le SDK décide par génération de modèle :

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5, et versions ultérieures** : la recherche d'outils est activée par défaut.
* **Modèles antérieurs d'Agent Platform** : le SDK charge les définitions d'outils en amont, car leurs piles de service rejettent l'en-tête bêta requis. `ENABLE_TOOL_SEARCH` ne peut pas remplacer cela.

Avant Claude Code v2.1.221, le SDK désactivait la recherche d'outils pour tous les modèles sur Google Cloud's Agent Platform sauf si vous définissiez `ENABLE_TOOL_SEARCH`.

Le SDK désactive également la recherche d'outils lorsque `ANTHROPIC_BASE_URL` pointe vers un hôte tiers, car la plupart des proxies ne transmettent pas les blocs `tool_reference`. Vous pouvez remplacer ce défaut par défaut avec la variable d'environnement `ENABLE_TOOL_SEARCH` :

| Valeur       | Comportement                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (non défini) | La recherche d'outils est activée. Les définitions d'outils sont différées et découvertes à la demande. Revient au chargement en amont sur les modèles Google Cloud's Agent Platform antérieurs à la génération Claude 4.5, un `ANTHROPIC_BASE_URL` tiers, ou un déploiement Microsoft Foundry hébergé sur Azure.                                                                                                                                                          |
| `true`       | La recherche d'outils est toujours activée, sauf sur un déploiement Microsoft Foundry hébergé sur Azure, où le rejet côté serveur force toujours le chargement en amont, et sur les modèles Google Cloud's Agent Platform antérieurs à la génération Claude 4.5, où le SDK continue de charger les définitions d'outils en amont. Le SDK envoie l'en-tête bêta via les proxies, et les requêtes échouent sur les proxies qui ne supportent pas les blocs `tool_reference`. |
| `auto`       | Compte les tokens dans les définitions d'outils que la recherche d'outils peut différer et compare le total par rapport à la fenêtre de contexte du modèle. Lorsque le total atteint 10 % de la fenêtre, la recherche d'outils s'active. En dessous de cela, le SDK charge chaque définition d'outil dans le contexte en amont.                                                                                                                                            |
| `auto:N`     | Identique à `auto` avec un pourcentage personnalisé. `auto:5` s'active lorsque ces définitions atteignent 5 % de la fenêtre de contexte. Les valeurs plus basses s'activent plus tôt.                                                                                                                                                                                                                                                                                      |
| `false`      | La recherche d'outils est désactivée. Toutes les définitions d'outils sont chargées dans le contexte à chaque tour.                                                                                                                                                                                                                                                                                                                                                        |

La définition de [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/fr/env-vars) maintient la recherche d'outils désactivée. Vous ne pouvez pas la remplacer en définissant `ENABLE_TOOL_SEARCH` vous-même. Votre organisation peut maintenir la recherche d'outils activée via les [paramètres gérés](/docs/fr/managed-settings), sur Claude Code v2.1.227 ou version ultérieure. [Désactiver les capacités de pré-version](/docs/fr/llm-gateway-protocol#disable-pre-release-capabilities) couvre où le remplacement s'applique et ce que la variable supprime.

La recherche d'outils s'applique à tous les outils enregistrés, qu'ils proviennent de serveurs MCP distants ou de [serveurs MCP SDK personnalisés](/docs/fr/agent-sdk/custom-tools). Lorsque vous utilisez `auto`, le SDK compte chaque définition que la recherche d'outils peut différer vers un seuil combiné : chaque outil MCP qui n'est pas marqué [`alwaysLoad`](/docs/fr/mcp#exempt-a-server-from-deferral), de n'importe quel serveur, plus les outils intégrés qui se chargent à la demande. Le SDK charge toujours les outils intégrés principaux tels que Bash, Read, et Edit en amont et ne les compte pas vers le seuil.

Définissez la valeur dans l'option `env` sur `query()`. En TypeScript, `env` remplace l'environnement du sous-processus, donc propagez `...process.env` pour conserver les variables héritées. En Python, `env` est fusionné au-dessus de l'environnement hérité. Cet exemple se connecte à un serveur MCP distant qui expose de nombreux outils, les pré-approuve tous avec un caractère générique, et utilise `auto:5` pour que la recherche d'outils s'active lorsque les définitions qu'elle peut différer atteignent 5 % de la fenêtre de contexte :

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Find and run the appropriate database query",
      options: {
        mcpServers: {
          "enterprise-tools": {
            // Connect to a remote MCP server
            type: "http",
            url: "https://tools.example.com/mcp"
          }
        },
        allowedTools: ["mcp__enterprise-tools__*"], // Wildcard pre-approves all tools from this server
        env: {
          ...process.env, // env replaces the subprocess environment, so keep inherited variables
          ENABLE_TOOL_SEARCH: "auto:5" // Activate tool search when deferrable definitions reach 5% of context
        }
      }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "enterprise-tools": {
                  "type": "http",
                  "url": "https://tools.example.com/mcp",
              }
          },
          allowed_tools=[
              "mcp__enterprise-tools__*"
          ],  # Wildcard pre-approves all tools from this server
          env={
              "ENABLE_TOOL_SEARCH": "auto:5"  # Activate tool search when deferrable definitions reach 5% of context
          },
      )

      try:
          async for message in query(
              prompt="Find and run the appropriate database query",
              options=options,
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

Pour exécuter cet exemple, remplacez `https://tools.example.com/mcp` par l'URL de votre propre serveur MCP. En cas de succès, le texte du résultat s'affiche sur la console.

Comme il s'agit d'un appel `query()` unique, le SDK lève une exception après avoir produit un résultat d'erreur, donc l'exemple enveloppe la boucle dans un bloc try. Pour voir pourquoi une exécution a échoué, vérifiez le `subtype` du message de résultat, tel que `error_during_execution`, à l'intérieur de la boucle. Pour plus d'informations sur les messages de résultat, consultez [Gérer le résultat](/docs/fr/agent-sdk/agent-loop#handle-the-result).

<h2 id="optimize-tool-discovery">
  Optimiser la découverte d'outils
</h2>

Le mécanisme de recherche fait correspondre les requêtes aux noms et descriptions des outils. Des noms comme `search_slack_messages` apparaissent pour une plus large gamme de requêtes que `query_slack`. Les descriptions avec des mots-clés spécifiques (« Rechercher les messages Slack par mot-clé, canal ou plage de dates ») correspondent à plus de requêtes que les descriptions génériques (« Interroger Slack »).

Vous pouvez également ajouter une section de message système listant les catégories d'outils disponibles. Cela donne à l'agent un contexte sur les types d'outils disponibles à rechercher. Transmettez le texte via l'option `systemPrompt` en TypeScript ou `system_prompt` en Python, en utilisant le preset `claude_code` avec `append`, qui ajoute votre texte au prompt du preset au lieu de le remplacer :

<CodeGroup>
  ```typescript TypeScript theme={null}
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: "You can search for tools to interact with Slack, GitHub, and Jira."
    }
  }
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      system_prompt={
          "type": "preset",
          "preset": "claude_code",
          "append": "You can search for tools to interact with Slack, GitHub, and Jira.",
      }
  )
  ```
</CodeGroup>

Pour l'ensemble complet des options de message système, consultez [Modification des messages système](/docs/fr/agent-sdk/modifying-system-prompts).

<h2 id="limits">
  Limites
</h2>

* **Outils maximum :** 10 000 outils dans votre catalogue
* **Résultats de recherche :** retourne jusqu'à cinq outils les plus pertinents par recherche par défaut
* **Support du modèle :** Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 et les modèles ultérieurs ; consultez la [compatibilité des modèles dans la documentation de l'API](https://platform.claude.com/docs/fr/agents-and-tools/tool-use/tool-search-tool#model-compatibility) pour la liste actuelle. Les mêmes minimums s'appliquent sur la plateforme Agent de Google Cloud.

<h2 id="related-documentation">
  Documentation connexe
</h2>

* [Recherche d'outils dans l'API](https://platform.claude.com/docs/fr/agents-and-tools/tool-use/tool-search-tool) : Documentation API complète pour la recherche d'outils, y compris les implémentations personnalisées
* [Connecter les serveurs MCP](/docs/fr/agent-sdk/mcp) : Connectez-vous à des outils externes via les serveurs MCP
* [Outils personnalisés](/docs/fr/agent-sdk/custom-tools) : Créez vos propres outils avec les serveurs MCP SDK
* [Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript) : Référence API complète
* [Référence du SDK Python](/docs/fr/agent-sdk/python) : Référence API complète
