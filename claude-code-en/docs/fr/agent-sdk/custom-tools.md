> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Donner à Claude des outils personnalisés

> Définissez des outils personnalisés avec le serveur MCP en processus du SDK Agent pour que Claude puisse appeler vos fonctions, accéder à vos API et effectuer des opérations spécifiques au domaine.

Les outils personnalisés étendent le SDK Agent en vous permettant de définir vos propres fonctions que Claude peut appeler lors d'une conversation. En utilisant le serveur MCP en processus du SDK, vous pouvez donner à Claude accès aux bases de données, aux API externes, à la logique spécifique au domaine ou à toute autre capacité dont votre application a besoin.

Ce guide couvre comment définir des outils avec des schémas d'entrée et des gestionnaires, les regrouper dans un serveur MCP, les transmettre à `query`, et contrôler les outils auxquels Claude peut accéder. Il couvre également la gestion des erreurs, les annotations d'outils et le retour de contenu non textuel comme les images.

<h2 id="quick-reference">
  Référence rapide
</h2>

| Si vous voulez...                                | Faites ceci                                                                                                                                                                                                                             |
| :----------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Définir un outil                                 | Utilisez [`@tool`](/docs/fr/agent-sdk/python#tool) (Python) ou [`tool()`](/docs/fr/agent-sdk/typescript#tool) (TypeScript) avec un nom, une description, un schéma et un gestionnaire. Voir [Créer un outil personnalisé](#create-a-custom-tool). |
| Enregistrer un outil auprès de Claude            | Enveloppez dans `create_sdk_mcp_server` / `createSdkMcpServer` et transmettez à `mcpServers` dans `query()`. Voir [Appeler un outil personnalisé](#call-a-custom-tool).                                                                 |
| Pré-approuver un outil                           | Ajoutez à vos outils autorisés. Voir [Configurer les outils autorisés](#configure-allowed-tools).                                                                                                                                       |
| Supprimer un outil intégré du contexte de Claude | Transmettez un tableau `tools` listant uniquement les outils intégrés que vous souhaitez. Voir [Configurer les outils autorisés](#configure-allowed-tools).                                                                             |
| Laisser Claude appeler les outils en parallèle   | Définissez `readOnlyHint: true` sur les outils sans effets secondaires. Voir [Ajouter des annotations d'outils](#add-tool-annotations).                                                                                                 |
| Contrôler le message d'erreur que Claude lit     | Retournez `isError: true` pour composer le message au lieu de faire remonter l'exception brute. Voir [Gérer les erreurs](#handle-errors).                                                                                               |
| Retourner des images ou des fichiers             | Utilisez des blocs `image` ou `resource` dans le tableau de contenu. Voir [Retourner des images et des ressources](#return-images-and-resources).                                                                                       |
| Retourner un résultat JSON lisible par machine   | Définissez `structuredContent` sur le résultat. Voir [Retourner des données structurées](#return-structured-data).                                                                                                                      |
| Adapter à de nombreux outils                     | Utilisez [la recherche d'outils](/docs/fr/agent-sdk/tool-search) pour charger les outils à la demande.                                                                                                                                       |

<h2 id="create-a-custom-tool">
  Créer un outil personnalisé
</h2>

Un outil est défini par quatre parties, transmises comme arguments à l'assistant [`tool()`](/docs/fr/agent-sdk/typescript#tool) en TypeScript ou au décorateur [`@tool`](/docs/fr/agent-sdk/python#tool) en Python :

* **Nom :** un identifiant unique que Claude utilise pour appeler l'outil.
* **Description :** ce que fait l'outil. Claude lit ceci pour décider quand l'appeler.
* **Schéma d'entrée :** les arguments que Claude doit fournir. En TypeScript, c'est toujours un [schéma Zod](https://zod.dev/), et les `args` du gestionnaire sont typés automatiquement à partir de celui-ci. En Python, c'est un dictionnaire mappant les noms aux types, comme `{"latitude": float}`, que le SDK convertit en JSON Schema pour vous. Le décorateur Python accepte également un dictionnaire [JSON Schema](https://json-schema.org/understanding-json-schema/about) complet directement quand vous avez besoin d'énumérations, de plages, de champs optionnels ou d'objets imbriqués.
* **Gestionnaire :** la fonction asynchrone qui s'exécute quand Claude appelle l'outil. Elle reçoit les arguments validés et doit retourner un objet avec :
  * `content` (obligatoire) : un tableau de blocs de résultats, chacun avec un `type` de `"text"`, `"image"`, `"audio"`, `"resource"` ou `"resource_link"`. Voir [Retourner des images et des ressources](#return-images-and-resources) pour les blocs non textuels.
  * `structuredContent` (optionnel) : un objet JSON contenant le résultat sous forme de données lisibles par machine, retourné aux côtés de `content`. Voir [Retourner des données structurées](#return-structured-data).
  * `isError` (optionnel) : définissez sur `true` pour signaler un échec d'outil afin que Claude puisse y réagir. Voir [Gérer les erreurs](#handle-errors).

Après avoir défini un outil, enveloppez-le dans un serveur avec [`createSdkMcpServer`](/docs/fr/agent-sdk/typescript#createsdkmcpserver) (TypeScript) ou [`create_sdk_mcp_server`](/docs/fr/agent-sdk/python#create_sdk_mcp_server) (Python). Le serveur s'exécute en processus à l'intérieur de votre application, pas comme un processus séparé.

<h3 id="weather-tool-example">
  Exemple d'outil météo
</h3>

Cet exemple définit un outil `get_temperature` et l'enveloppe dans un serveur MCP. Il configure uniquement l'outil ; pour le transmettre à `query` et l'exécuter, voir [Appeler un outil personnalisé](#call-a-custom-tool) ci-dessous.

<CodeGroup>
  ```python Python theme={null}
  from typing import Any
  import httpx
  from claude_agent_sdk import tool, create_sdk_mcp_server


  # Define a tool: name, description, input schema, handler
  @tool(
      "get_temperature",
      "Get the current temperature at a location",
      {"latitude": float, "longitude": float},
  )
  async def get_temperature(args: dict[str, Any]) -> dict[str, Any]:
      async with httpx.AsyncClient() as client:
          response = await client.get(
              "https://api.open-meteo.com/v1/forecast",
              params={
                  "latitude": args["latitude"],
                  "longitude": args["longitude"],
                  "current": "temperature_2m",
                  "temperature_unit": "fahrenheit",
              },
          )
          data = response.json()

      # Return a content array - Claude sees this as the tool result
      return {
          "content": [
              {
                  "type": "text",
                  "text": f"Temperature: {data['current']['temperature_2m']}°F",
              }
          ]
      }


  # Wrap the tool in an in-process MCP server
  weather_server = create_sdk_mcp_server(
      name="weather",
      version="1.0.0",
      tools=[get_temperature],
  )
  ```

  ```typescript TypeScript theme={null}
  import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  // Define a tool: name, description, input schema, handler
  const getTemperature = tool(
    "get_temperature",
    "Get the current temperature at a location",
    {
      latitude: z.number().describe("Latitude coordinate"), // .describe() adds a field description Claude sees
      longitude: z.number().describe("Longitude coordinate")
    },
    async (args) => {
      // args is typed from the schema: { latitude: number; longitude: number }
      const response = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&current=temperature_2m&temperature_unit=fahrenheit`
      );
      const data: any = await response.json();

      // Return a content array - Claude sees this as the tool result
      return {
        content: [{ type: "text", text: `Temperature: ${data.current.temperature_2m}°F` }]
      };
    }
  );

  // Wrap the tool in an in-process MCP server
  const weatherServer = createSdkMcpServer({
    name: "weather",
    version: "1.0.0",
    tools: [getTemperature]
  });
  ```
</CodeGroup>

Voir la référence TypeScript [`tool()`](/docs/fr/agent-sdk/typescript#tool) ou la référence Python [`@tool`](/docs/fr/agent-sdk/python#tool) pour les détails complets des paramètres, y compris les formats de schéma JSON et la structure des valeurs de retour.

<Tip>
  Pour rendre un paramètre optionnel : en TypeScript, ajoutez `.default()` au champ Zod. En Python, le schéma dict traite chaque clé comme obligatoire, donc omettez le paramètre du schéma, mentionnez-le dans la chaîne de description, et lisez-le avec `args.get()` dans le gestionnaire. L'outil [`get_precipitation_chance` ci-dessous](#add-more-tools) montre les deux modèles.
</Tip>

<h3 id="call-a-custom-tool">
  Appeler un outil personnalisé
</h3>

Transmettez le serveur MCP que vous avez créé à `query` via l'option `mcpServers`. La clé dans `mcpServers` devient le segment `{server_name}` dans le nom complètement qualifié de chaque outil : `mcp__{server_name}__{tool_name}`. Listez ce nom dans `allowedTools` afin que l'outil s'exécute sans invite de permission.

Ces extraits réutilisent le `weatherServer` de l'[exemple ci-dessus](#weather-tool-example) pour demander à Claude quel est le temps dans un endroit spécifique.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={"weather": weather_server},
          allowed_tools=["mcp__weather__get_temperature"],
      )

      async for message in query(
          prompt="What's the temperature in San Francisco?",
          options=options,
      ):
          # ResultMessage is the final message after all tool calls complete
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "What's the temperature in San Francisco?",
    options: {
      mcpServers: { weather: weatherServer },
      allowedTools: ["mcp__weather__get_temperature"]
    }
  })) {
    // "result" is the final message after all tool calls complete
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h3 id="add-more-tools">
  Ajouter plus d'outils
</h3>

Un serveur contient autant d'outils que vous en listez dans son tableau `tools`. Avec plus d'un outil sur un serveur, vous pouvez lister chacun dans `allowedTools` individuellement ou utiliser le caractère générique `mcp__weather__*` pour couvrir tous les outils que le serveur expose.

L'exemple ci-dessous ajoute un deuxième outil, `get_precipitation_chance`, au `weatherServer` de l'[exemple d'outil météo](#weather-tool-example) et le reconstruit avec les deux outils dans le tableau.

<CodeGroup>
  ```python Python theme={null}
  # Define a second tool for the same server
  @tool(
      "get_precipitation_chance",
      "Get the hourly precipitation probability for a location. "
      "Optionally pass 'hours' (1-24) to control how many hours to return.",
      {"latitude": float, "longitude": float},
  )
  async def get_precipitation_chance(args: dict[str, Any]) -> dict[str, Any]:
      # 'hours' isn't in the schema - read it with .get() to make it optional
      hours = args.get("hours", 12)
      async with httpx.AsyncClient() as client:
          response = await client.get(
              "https://api.open-meteo.com/v1/forecast",
              params={
                  "latitude": args["latitude"],
                  "longitude": args["longitude"],
                  "hourly": "precipitation_probability",
                  "forecast_days": 1,
              },
          )
          data = response.json()
      chances = data["hourly"]["precipitation_probability"][:hours]

      return {
          "content": [
              {
                  "type": "text",
                  "text": f"Next {hours} hours: {'%, '.join(map(str, chances))}%",
              }
          ]
      }


  # Rebuild the server with both tools in the array
  weather_server = create_sdk_mcp_server(
      name="weather",
      version="1.0.0",
      tools=[get_temperature, get_precipitation_chance],
  )
  ```

  ```typescript TypeScript theme={null}
  // Define a second tool for the same server
  const getPrecipitationChance = tool(
    "get_precipitation_chance",
    "Get the hourly precipitation probability for a location",
    {
      latitude: z.number(),
      longitude: z.number(),
      hours: z
        .number()
        .int()
        .min(1)
        .max(24)
        .default(12) // .default() makes the parameter optional
        .describe("How many hours of forecast to return")
    },
    async (args) => {
      const response = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&hourly=precipitation_probability&forecast_days=1`
      );
      const data: any = await response.json();
      const chances = data.hourly.precipitation_probability.slice(0, args.hours);

      return {
        content: [{ type: "text", text: `Next ${args.hours} hours: ${chances.join("%, ")}%` }]
      };
    }
  );

  // Rebuild the server with both tools in the array
  const weatherServer = createSdkMcpServer({
    name: "weather",
    version: "1.0.0",
    tools: [getTemperature, getPrecipitationChance]
  });
  ```
</CodeGroup>

Chaque outil dans ce tableau consomme de l'espace de fenêtre de contexte à chaque tour. Si vous définissez des dizaines d'outils, voir [la recherche d'outils](/docs/fr/agent-sdk/tool-search) pour les charger à la demande à la place.

<h3 id="add-tool-annotations">
  Ajouter des annotations d'outils
</h3>

Les [annotations d'outils](https://modelcontextprotocol.io/docs/concepts/tools#tool-annotations) sont des métadonnées optionnelles décrivant le comportement d'un outil. Transmettez-les comme cinquième argument à l'assistant `tool()` en TypeScript ou via l'argument de mot-clé `annotations` pour le décorateur `@tool` en Python. Tous les champs d'indice sont des booléens.

| Champ             | Par défaut | Signification                                                                                                                      |
| :---------------- | :--------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| `readOnlyHint`    | `false`    | L'outil ne modifie pas son environnement. Contrôle si l'outil peut être appelé en parallèle avec d'autres outils en lecture seule. |
| `destructiveHint` | `true`     | L'outil peut effectuer des mises à jour destructrices. Informatif uniquement.                                                      |
| `idempotentHint`  | `false`    | Les appels répétés avec les mêmes arguments n'ont aucun effet supplémentaire. Informatif uniquement.                               |
| `openWorldHint`   | `true`     | L'outil atteint les systèmes en dehors de votre processus. Informatif uniquement.                                                  |

Les annotations sont des métadonnées, pas une application. Un outil marqué `readOnlyHint: true` peut toujours écrire sur le disque si c'est ce que fait le gestionnaire. Gardez l'annotation exacte par rapport au gestionnaire.

Cet exemple ajoute `readOnlyHint` à l'outil `get_temperature` de l'[exemple d'outil météo](#weather-tool-example).

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import tool, ToolAnnotations


  @tool(
      "get_temperature",
      "Get the current temperature at a location",
      {"latitude": float, "longitude": float},
      annotations=ToolAnnotations(
          readOnlyHint=True
      ),  # Lets Claude batch this with other read-only calls
  )
  async def get_temperature(args):
      return {"content": [{"type": "text", "text": "..."}]}
  ```

  ```typescript TypeScript theme={null}
  tool(
    "get_temperature",
    "Get the current temperature at a location",
    { latitude: z.number(), longitude: z.number() },
    async (args) => ({ content: [{ type: "text", text: `...` }] }),
    { annotations: { readOnlyHint: true } } // Lets Claude batch this with other read-only calls
  );
  ```
</CodeGroup>

Voir `ToolAnnotations` dans la référence [TypeScript](/docs/fr/agent-sdk/typescript#toolannotations) ou [Python](/docs/fr/agent-sdk/python#toolannotations).

<h2 id="control-tool-access">
  Contrôler l'accès aux outils
</h2>

L'[exemple d'outil météo](#weather-tool-example) a enregistré un serveur et listé les outils dans `allowedTools`. Cette section couvre comment les noms d'outils sont construits et comment limiter l'accès quand vous avez plusieurs outils ou que vous souhaitez restreindre les outils intégrés.

<h3 id="tool-name-format">
  Format du nom d'outil
</h3>

Quand les outils MCP sont exposés à Claude, leurs noms suivent un format spécifique :

* Modèle : `mcp__{server_name}__{tool_name}`
* Exemple : Un outil nommé `get_temperature` dans le serveur `weather` devient `mcp__weather__get_temperature`

<h3 id="configure-allowed-tools">
  Configurer les outils autorisés
</h3>

L'option `tools` et les listes autorisées/interdites affectent deux couches : la disponibilité, qui contrôle si un outil apparaît dans le contexte de Claude, et la permission, qui contrôle si un appel est approuvé une fois que Claude tente de l'utiliser. `tools` et les entrées `disallowedTools` sans portée changent la disponibilité. `allowedTools` et les règles `disallowedTools` avec portée changent la permission uniquement.

| Option                    | Couche        | Effet                                                                                                                                                                                                                                      |
| :------------------------ | :------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools: ["Read", "Grep"]` | Disponibilité | Seuls les outils intégrés listés sont dans le contexte de Claude. Les outils intégrés non listés sont supprimés. Les outils MCP ne sont pas affectés.                                                                                      |
| `tools: []`               | Disponibilité | Tous les outils intégrés sont supprimés. Claude ne peut utiliser que vos outils MCP.                                                                                                                                                       |
| outils autorisés          | Permission    | Les outils listés s'exécutent sans invite de permission. Les outils non listés restent disponibles ; les appels passent par le [flux de permission](/docs/fr/agent-sdk/permissions).                                                            |
| outils interdits          | Les deux      | Un nom d'outil simple tel que `"Bash"` supprime l'outil du contexte de Claude, comme l'omission de `tools`. Une règle avec portée telle que `"Bash(rm *)"` laisse l'outil dans le contexte et refuse uniquement les appels correspondants. |

Pour supprimer complètement un outil intégrés, omettez-le de `tools` ou listez son nom simple dans `disallowedTools` (Python : `disallowed_tools`) ; les deux gardent l'outil hors du contexte afin que Claude ne le tente jamais. Une règle `disallowedTools` avec portée bloque les appels correspondants mais laisse l'outil visible, donc Claude peut gaspiller un tour en le tentant. Voir [Configurer les permissions](/docs/fr/agent-sdk/permissions) pour l'ordre d'évaluation complet.

<h2 id="handle-errors">
  Gérer les erreurs
</h2>

Une erreur de gestionnaire n'arrête pas la boucle d'agent. Le serveur MCP en processus du SDK capture les exceptions non capturées et les retourne comme des résultats d'erreur, donc la façon dont vous signalez une erreur détermine ce que Claude lit, non pas si la requête échoue :

| Ce qui se passe                                                                                 | Résultat                                                                                                                                                                    |
| :---------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Le gestionnaire lève une exception non capturée                                                 | Le serveur MCP la convertit en résultat d'erreur contenant le message d'exception brut. Claude voit ce message, et la boucle d'agent continue.                              |
| Le gestionnaire capture l'erreur et retourne `isError: true` (TS) / `"is_error": True` (Python) | Claude voit le message que vous composez. Vous pouvez ajouter du contexte que l'exception brute n'a pas, comme quelle requête a échoué ou ce qu'il faut essayer à la place. |

Dans les deux cas, Claude peut réessayer, essayer un outil différent ou expliquer l'échec. Capturez les erreurs vous-même quand le message d'exception brut n'est pas suffisant pour que Claude agisse.

L'exemple ci-dessous capture deux types d'échecs à l'intérieur du gestionnaire et compose le message d'erreur que Claude lit. Un statut HTTP non-200 est capturé de la réponse et retourné comme un résultat d'erreur. Une erreur réseau ou un JSON invalide est capturé par le `try/except` (Python) ou `try/catch` (TypeScript) environnant et est également retourné comme un résultat d'erreur. Dans les deux cas, Claude reçoit un message qui décrit l'échec au lieu d'une chaîne d'exception brute.

<CodeGroup>
  ```python Python theme={null}
  import json
  import httpx
  from typing import Any


  @tool(
      "fetch_data",
      "Fetch data from an API",
      {"endpoint": str},  # Simple schema
  )
  async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
      try:
          async with httpx.AsyncClient() as client:
              response = await client.get(args["endpoint"])
              if response.status_code != 200:
                  # Return the failure as a tool result so Claude can react to it.
                  # is_error marks this as a failed call rather than odd-looking data.
                  return {
                      "content": [
                          {
                              "type": "text",
                              "text": f"API error: {response.status_code} {response.reason_phrase}",
                          }
                      ],
                      "is_error": True,
                  }

              data = response.json()
              return {"content": [{"type": "text", "text": json.dumps(data, indent=2)}]}
      except Exception as e:
          # Composes the message Claude reads. An uncaught exception would
          # reach Claude as the raw str(e) with no context.
          return {
              "content": [{"type": "text", "text": f"Failed to fetch data: {str(e)}"}],
              "is_error": True,
          }
  ```

  ```typescript TypeScript theme={null}
  tool(
    "fetch_data",
    "Fetch data from an API",
    {
      endpoint: z.string().url().describe("API endpoint URL")
    },
    async (args) => {
      try {
        const response = await fetch(args.endpoint);

        if (!response.ok) {
          // Return the failure as a tool result so Claude can react to it.
          // isError marks this as a failed call rather than odd-looking data.
          return {
            content: [
              {
                type: "text",
                text: `API error: ${response.status} ${response.statusText}`
              }
            ],
            isError: true
          };
        }

        const data = await response.json();
        return {
          content: [
            {
              type: "text",
              text: JSON.stringify(data, null, 2)
            }
          ]
        };
      } catch (error) {
        // Composes the message Claude reads. An uncaught throw would
        // reach Claude as the raw error message with no context.
        return {
          content: [
            {
              type: "text",
              text: `Failed to fetch data: ${error instanceof Error ? error.message : String(error)}`
            }
          ],
          isError: true
        };
      }
    }
  );
  ```
</CodeGroup>

<h2 id="return-images-and-resources">
  Retourner des images et des ressources
</h2>

Le tableau `content` dans un résultat d'outil accepte les blocs `text`, `image`, `audio`, `resource` et `resource_link`. Vous pouvez les mélanger dans la même réponse. En TypeScript, les blocs audio sont enregistrés sur le disque et Claude reçoit un bloc texte avec le chemin du fichier enregistré ; en Python, le SDK supprime les blocs audio du résultat de l'outil et enregistre un avertissement. Les blocs de lien de ressource sont convertis en un bloc texte contenant le nom du lien, l'URI et la description.

<h3 id="images">
  Images
</h3>

Un bloc image porte les octets d'image en ligne, codés en base64. Il n'y a pas de champ URL. Pour retourner une image qui se trouve à une URL, récupérez-la dans le gestionnaire, lisez les octets de réponse et encodez-les en base64 avant de les retourner. Le résultat est traité comme une entrée visuelle.

| Champ      | Type      | Notes                                                                                   |
| :--------- | :-------- | :-------------------------------------------------------------------------------------- |
| `type`     | `"image"` |                                                                                         |
| `data`     | `string`  | Octets codés en base64. Base64 brut uniquement, pas de préfixe `data:image/...;base64,` |
| `mimeType` | `string`  | Obligatoire. Par exemple `image/png`, `image/jpeg`, `image/webp`, `image/gif`           |

<CodeGroup>
  ```python Python theme={null}
  import base64
  import httpx


  # Define a tool that fetches an image from a URL and returns it to Claude
  @tool("fetch_image", "Fetch an image from a URL and return it to Claude", {"url": str})
  async def fetch_image(args):
      async with httpx.AsyncClient() as client:  # Fetch the image bytes
          response = await client.get(args["url"])

      return {
          "content": [
              {
                  "type": "image",
                  "data": base64.b64encode(response.content).decode(
                      "ascii"
                  ),  # Base64-encode the raw bytes
                  "mimeType": response.headers.get(
                      "content-type", "image/png"
                  ),  # Read MIME type from the response
              }
          ]
      }
  ```

  ```typescript TypeScript theme={null}
  tool(
    "fetch_image",
    "Fetch an image from a URL and return it to Claude",
    {
      url: z.string().url()
    },
    async (args) => {
      const response = await fetch(args.url); // Fetch the image bytes
      const buffer = Buffer.from(await response.arrayBuffer()); // Read into a Buffer for base64 encoding
      const mimeType = response.headers.get("content-type") ?? "image/png";

      return {
        content: [
          {
            type: "image",
            data: buffer.toString("base64"), // Base64-encode the raw bytes
            mimeType
          }
        ]
      };
    }
  );
  ```
</CodeGroup>

<h3 id="resources">
  Ressources
</h3>

Un bloc ressource intègre un morceau de contenu identifié par un URI. L'URI est un libellé pour que Claude le référence ; le contenu réel se trouve dans le champ `text` ou `blob` du bloc. Utilisez ceci quand votre outil produit quelque chose qui a du sens à adresser par nom plus tard, comme un fichier généré ou un enregistrement d'un système externe.

| Champ               | Type         | Notes                                                                                                                                                                     |
| :------------------ | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`              | `"resource"` |                                                                                                                                                                           |
| `resource.uri`      | `string`     | Identifiant du contenu. N'importe quel schéma URI                                                                                                                         |
| `resource.text`     | `string`     | Le contenu, s'il est textuel. Fournissez ceci ou `blob`, pas les deux                                                                                                     |
| `resource.blob`     | `string`     | Le contenu codé en base64, s'il est binaire. TypeScript uniquement : le SDK Python supprime les ressources binaires du résultat de l'outil et enregistre un avertissement |
| `resource.mimeType` | `string`     | Optionnel                                                                                                                                                                 |

Cet exemple montre un bloc ressource retourné de l'intérieur d'un gestionnaire d'outil. L'URI `file:///tmp/report.md` est un libellé que Claude peut référencer plus tard ; le SDK ne lit pas à partir de ce chemin.

<CodeGroup>
  ```typescript TypeScript theme={null}
  return {
    content: [
      {
        type: "resource",
        resource: {
          uri: "file:///tmp/report.md", // Label for Claude to reference, not a path the SDK reads
          mimeType: "text/markdown",
          text: "# Report\n..." // The actual content, inline
        }
      }
    ]
  };
  ```

  ```python Python theme={null}
  return {
      "content": [
          {
              "type": "resource",
              "resource": {
                  "uri": "file:///tmp/report.md",  # Label for Claude to reference, not a path the SDK reads
                  "mimeType": "text/markdown",
                  "text": "# Report\n...",  # The actual content, inline
              },
          }
      ]
  }
  ```
</CodeGroup>

Ces formes de bloc proviennent du type MCP `CallToolResult`. Voir la [spécification MCP](https://modelcontextprotocol.io/specification/2025-06-18/server/tools#tool-result) pour la définition complète.

<h2 id="return-structured-data">
  Retourner des données structurées
</h2>

`structuredContent` est un objet JSON optionnel sur le résultat, séparé du tableau `content`. Utilisez-le pour retourner des valeurs brutes que Claude peut lire comme des champs exacts au lieu de les analyser à partir d'une chaîne de texte ou d'une image.

Quand `structuredContent` est défini, Claude reçoit le JSON plus tous les blocs image ou ressource de `content`. Les blocs de texte dans `content` ne sont pas transmis, car on suppose qu'ils dupliquent les données structurées. L'exemple ci-dessous rend un graphique sous forme de bloc image et retourne les points de données derrière lui dans `structuredContent` du même gestionnaire.

```typescript TypeScript theme={null}
return {
  content: [
    {
      type: "image",
      data: chartPngBuffer.toString("base64"),
      mimeType: "image/png"
    }
  ],
  structuredContent: {
    series: "temperature_2m",
    unit: "fahrenheit",
    points: [62.1, 63.4, 65.0, 64.2]
  }
};
```

<Note>
  Le décorateur Python `@tool` transmet uniquement `content` et `is_error` du dictionnaire de retour du gestionnaire. Pour retourner `structuredContent` de Python, exécutez un [serveur MCP autonome](/docs/fr/agent-sdk/mcp) à la place d'un serveur SDK en processus.
</Note>

<h2 id="example-unit-converter">
  Exemple : convertisseur d'unités
</h2>

Cet outil convertit les valeurs entre les unités de longueur, température et poids. Un utilisateur peut demander « convertir 100 kilomètres en miles » ou « qu'est-ce que 72°F en Celsius », et Claude choisit le bon type d'unité et les unités de la demande.

Il démontre deux modèles :

* **Schémas d'énumération :** `unit_type` est limité à un ensemble fixe de valeurs. En TypeScript, utilisez `z.enum()`. En Python, le schéma dict ne supporte pas les énumérations, donc le dictionnaire JSON Schema complet est requis.
* **Gestion des entrées non supportées :** quand une paire de conversion n'est pas trouvée, le gestionnaire retourne `isError: true` afin que Claude puisse dire à l'utilisateur ce qui s'est mal passé au lieu de traiter un échec comme un résultat normal.

<CodeGroup>
  ```python Python theme={null}
  from typing import Any
  from claude_agent_sdk import tool, create_sdk_mcp_server


  # z.enum() in TypeScript becomes an "enum" constraint in JSON Schema.
  # The dict schema has no equivalent, so full JSON Schema is required.
  @tool(
      "convert_units",
      "Convert a value from one unit to another",
      {
          "type": "object",
          "properties": {
              "unit_type": {
                  "type": "string",
                  "enum": ["length", "temperature", "weight"],
                  "description": "Category of unit",
              },
              "from_unit": {
                  "type": "string",
                  "description": "Unit to convert from, e.g. kilometers, fahrenheit, pounds",
              },
              "to_unit": {"type": "string", "description": "Unit to convert to"},
              "value": {"type": "number", "description": "Value to convert"},
          },
          "required": ["unit_type", "from_unit", "to_unit", "value"],
      },
  )
  async def convert_units(args: dict[str, Any]) -> dict[str, Any]:
      conversions = {
          "length": {
              "kilometers_to_miles": lambda v: v * 0.621371,
              "miles_to_kilometers": lambda v: v * 1.60934,
              "meters_to_feet": lambda v: v * 3.28084,
              "feet_to_meters": lambda v: v * 0.3048,
          },
          "temperature": {
              "celsius_to_fahrenheit": lambda v: (v * 9) / 5 + 32,
              "fahrenheit_to_celsius": lambda v: (v - 32) * 5 / 9,
              "celsius_to_kelvin": lambda v: v + 273.15,
              "kelvin_to_celsius": lambda v: v - 273.15,
          },
          "weight": {
              "kilograms_to_pounds": lambda v: v * 2.20462,
              "pounds_to_kilograms": lambda v: v * 0.453592,
              "grams_to_ounces": lambda v: v * 0.035274,
              "ounces_to_grams": lambda v: v * 28.3495,
          },
      }

      key = f"{args['from_unit']}_to_{args['to_unit']}"
      fn = conversions.get(args["unit_type"], {}).get(key)

      if not fn:
          return {
              "content": [
                  {
                      "type": "text",
                      "text": f"Unsupported conversion: {args['from_unit']} to {args['to_unit']}",
                  }
              ],
              "is_error": True,
          }

      result = fn(args["value"])
      return {
          "content": [
              {
                  "type": "text",
                  "text": f"{args['value']} {args['from_unit']} = {result:.4f} {args['to_unit']}",
              }
          ]
      }


  converter_server = create_sdk_mcp_server(
      name="converter",
      version="1.0.0",
      tools=[convert_units],
  )
  ```

  ```typescript TypeScript theme={null}
  import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  const convert = tool(
    "convert_units",
    "Convert a value from one unit to another",
    {
      unit_type: z.enum(["length", "temperature", "weight"]).describe("Category of unit"),
      from_unit: z
        .string()
        .describe("Unit to convert from, e.g. kilometers, fahrenheit, pounds"),
      to_unit: z.string().describe("Unit to convert to"),
      value: z.number().describe("Value to convert")
    },
    async (args) => {
      type Conversions = Record<string, Record<string, (v: number) => number>>;

      const conversions: Conversions = {
        length: {
          kilometers_to_miles: (v) => v * 0.621371,
          miles_to_kilometers: (v) => v * 1.60934,
          meters_to_feet: (v) => v * 3.28084,
          feet_to_meters: (v) => v * 0.3048
        },
        temperature: {
          celsius_to_fahrenheit: (v) => (v * 9) / 5 + 32,
          fahrenheit_to_celsius: (v) => ((v - 32) * 5) / 9,
          celsius_to_kelvin: (v) => v + 273.15,
          kelvin_to_celsius: (v) => v - 273.15
        },
        weight: {
          kilograms_to_pounds: (v) => v * 2.20462,
          pounds_to_kilograms: (v) => v * 0.453592,
          grams_to_ounces: (v) => v * 0.035274,
          ounces_to_grams: (v) => v * 28.3495
        }
      };

      const key = `${args.from_unit}_to_${args.to_unit}`;
      const fn = conversions[args.unit_type]?.[key];

      if (!fn) {
        return {
          content: [
            {
              type: "text",
              text: `Unsupported conversion: ${args.from_unit} to ${args.to_unit}`
            }
          ],
          isError: true
        };
      }

      const result = fn(args.value);
      return {
        content: [
          {
            type: "text",
            text: `${args.value} ${args.from_unit} = ${result.toFixed(4)} ${args.to_unit}`
          }
        ]
      };
    }
  );

  const converterServer = createSdkMcpServer({
    name: "converter",
    version: "1.0.0",
    tools: [convert]
  });
  ```
</CodeGroup>

Une fois le serveur défini, transmettez-le à `query` de la même manière que l'exemple météo. Cet exemple envoie trois invites différentes dans une boucle pour montrer le même outil gérant différents types d'unités. Pour chaque réponse, il inspecte les objets `AssistantMessage` (qui contiennent les appels d'outils que Claude a effectués pendant ce tour) et imprime chaque `ToolUseBlock` avant d'imprimer le texte final de `ResultMessage`. Cela vous permet de voir quand Claude utilise l'outil par rapport à répondre à partir de ses propres connaissances.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      ResultMessage,
      AssistantMessage,
      ToolUseBlock,
  )


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={"converter": converter_server},
          allowed_tools=["mcp__converter__convert_units"],
      )

      prompts = [
          "Convert 100 kilometers to miles.",
          "What is 72°F in Celsius?",
          "How many pounds is 5 kilograms?",
      ]

      for prompt in prompts:
          async for message in query(prompt=prompt, options=options):
              if isinstance(message, AssistantMessage):
                  for block in message.content:
                      if isinstance(block, ToolUseBlock):
                          print(f"[tool call] {block.name}({block.input})")
              elif isinstance(message, ResultMessage) and message.subtype == "success":
                  print(f"Q: {prompt}\nA: {message.result}\n")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const prompts = [
    "Convert 100 kilometers to miles.",
    "What is 72°F in Celsius?",
    "How many pounds is 5 kilograms?"
  ];

  for (const prompt of prompts) {
    for await (const message of query({
      prompt,
      options: {
        mcpServers: { converter: converterServer },
        allowedTools: ["mcp__converter__convert_units"]
      }
    })) {
      if (message.type === "assistant") {
        for (const block of message.message.content) {
          if (block.type === "tool_use") {
            console.log(`[tool call] ${block.name}`, block.input);
          }
        }
      } else if (message.type === "result" && message.subtype === "success") {
        console.log(`Q: ${prompt}\nA: ${message.result}\n`);
      }
    }
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  Étapes suivantes
</h2>

Les outils personnalisés enveloppent les fonctions asynchrones dans une interface standard. Vous pouvez mélanger les modèles de cette page dans le même serveur : un seul serveur peut contenir un outil de base de données, un outil de passerelle API et un moteur de rendu d'image côte à côte.

À partir d'ici :

* Si votre serveur grandit à des dizaines d'outils, voir [la recherche d'outils](/docs/fr/agent-sdk/tool-search) pour différer le chargement jusqu'à ce que Claude en ait besoin.
* Pour vous connecter à des serveurs MCP externes (système de fichiers, GitHub, Slack) au lieu de construire les vôtres, voir [Connecter les serveurs MCP](/docs/fr/agent-sdk/mcp).
* Pour contrôler quels outils s'exécutent automatiquement par rapport à ceux nécessitant une approbation, voir [Configurer les permissions](/docs/fr/agent-sdk/permissions).

<h2 id="related-documentation">
  Documentation connexe
</h2>

* [Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript)
* [Référence du SDK Python](/docs/fr/agent-sdk/python)
* [Documentation MCP](https://modelcontextprotocol.io)
* [Aperçu du SDK](/docs/fr/agent-sdk/overview)
