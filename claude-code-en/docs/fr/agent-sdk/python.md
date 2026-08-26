> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Référence du SDK Agent - Python

> Référence API complète du SDK Agent Python, incluant toutes les fonctions, types et classes.

<h2 id="installation">
  Installation
</h2>

Installez le package dans un environnement virtuel. Sur les installations récentes de Debian, Ubuntu et Homebrew Python, l'exécution de `pip install` contre le Python système échoue avec `error: externally-managed-environment`.

```bash theme={null}
python3 -m venv .venv
source .venv/bin/activate
pip install claude-agent-sdk
```

Pour uv, Windows PowerShell et la configuration de la clé API, consultez [Démarrer dans la vue d'ensemble du Agent SDK](/docs/fr/agent-sdk/overview#get-started).

<h2 id="choosing-between-query-and-claudesdkclient">
  Choisir entre `query()` et `ClaudeSDKClient`
</h2>

Le SDK Python offre deux façons d'interagir avec Claude Code :

<h3 id="quick-comparison">
  Comparaison rapide
</h3>

| Fonctionnalité                | `query()`                                      | `ClaudeSDKClient`                        |
| :---------------------------- | :--------------------------------------------- | :--------------------------------------- |
| **Session**                   | Crée une nouvelle session par défaut           | Réutilise la même session                |
| **Conversation**              | Échange unique                                 | Plusieurs échanges dans le même contexte |
| **Connexion**                 | Gérée automatiquement                          | Contrôle manuel                          |
| **Entrée en streaming**       | ✅ Supportée                                    | ✅ Supportée                              |
| **Interruptions**             | ❌ Non supportées                               | ✅ Supportées                             |
| **Hooks**                     | ✅ Supportés                                    | ✅ Supportés                              |
| **Outils personnalisés**      | ✅ Supportés                                    | ✅ Supportés                              |
| **Continuer la conversation** | Manuel via `continue_conversation` ou `resume` | ✅ Automatique                            |
| **Cas d'usage**               | Tâches ponctuelles                             | Conversations continues                  |

<h3 id="when-to-use-query-one-off-tasks">
  Quand utiliser `query()` (tâches ponctuelles)
</h3>

**Idéal pour :**

* Les questions ponctuelles où vous n'avez pas besoin d'historique de conversation
* Les tâches indépendantes qui ne nécessitent pas de contexte des échanges précédents
* Les scripts d'automatisation simples
* Quand vous voulez un nouveau départ à chaque fois

<h3 id="when-to-use-claudesdkclient-continuous-conversation">
  Quand utiliser `ClaudeSDKClient` (conversation continue)
</h3>

**Idéal pour :**

* **Continuer les conversations** - Quand vous avez besoin que Claude se souvienne du contexte
* **Questions de suivi** - S'appuyer sur les réponses précédentes
* **Applications interactives** - Interfaces de chat, REPLs
* **Logique basée sur les réponses** - Quand l'action suivante dépend de la réponse de Claude
* **Contrôle de session** - Gérer explicitement le cycle de vie de la conversation

<h2 id="functions">
  Fonctions
</h2>

<h3 id="query">
  `query()`
</h3>

Crée une nouvelle session pour chaque interaction avec Claude Code par défaut. Retourne un itérateur asynchrone qui produit les messages au fur et à mesure qu'ils arrivent. Chaque appel à `query()` recommence à zéro sans mémoire des interactions précédentes, sauf si vous passez `continue_conversation=True` ou `resume` dans [`ClaudeAgentOptions`](#claudeagentoptions). Voir [Sessions](/docs/fr/agent-sdk/sessions).

```python theme={null}
async def query(
    *,
    prompt: str | AsyncIterable[dict[str, Any]],
    options: ClaudeAgentOptions | None = None,
    transport: Transport | None = None
) -> AsyncIterator[Message]
```

<h4 id="parameters">
  Paramètres
</h4>

| Paramètre   | Type                         | Description                                                                             |
| :---------- | :--------------------------- | :-------------------------------------------------------------------------------------- |
| `prompt`    | `str \| AsyncIterable[dict]` | Le prompt d'entrée sous forme de chaîne ou d'itérable asynchrone pour le mode streaming |
| `options`   | `ClaudeAgentOptions \| None` | Objet de configuration optionnel (par défaut `ClaudeAgentOptions()` si None)            |
| `transport` | `Transport \| None`          | Transport personnalisé optionnel pour communiquer avec le processus CLI                 |

<h4 id="returns">
  Retours
</h4>

Retourne un `AsyncIterator[Message]` qui produit les messages de la conversation.

<h4 id="example-with-options">
  Exemple - Avec options
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions


async def main():
    options = ClaudeAgentOptions(
        system_prompt="You are an expert Python developer",
        permission_mode="acceptEdits",
        cwd="/home/user/project",
    )

    async for message in query(prompt="Create a Python web server", options=options):
        print(message)


asyncio.run(main())
```

<h3 id="tool">
  `tool()`
</h3>

Décorateur pour définir des outils MCP avec sécurité des types.

```python theme={null}
def tool(
    name: str,
    description: str,
    input_schema: type | dict[str, Any],
    annotations: ToolAnnotations | None = None
) -> Callable[[Callable[[Any], Awaitable[dict[str, Any]]]], SdkMcpTool[Any]]
```

<h4 id="parameters-2">
  Paramètres
</h4>

| Paramètre      | Type                                            | Description                                                                      |
| :------------- | :---------------------------------------------- | :------------------------------------------------------------------------------- |
| `name`         | `str`                                           | Identifiant unique pour l'outil                                                  |
| `description`  | `str`                                           | Description lisible de ce que fait l'outil                                       |
| `input_schema` | `type \| dict[str, Any]`                        | Schéma définissant les paramètres d'entrée de l'outil (voir ci-dessous)          |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | Annotations MCP optionnelles fournissant des indices comportementaux aux clients |

<h4 id="input-schema-options">
  Options de schéma d'entrée
</h4>

1. **Mappage de type simple** (recommandé) :

   ```python theme={null}
   {"text": str, "count": int, "enabled": bool}
   ```

2. **Format JSON Schema** (pour la validation complexe) :
   ```python theme={null}
   {
       "type": "object",
       "properties": {
           "text": {"type": "string"},
           "count": {"type": "integer", "minimum": 0},
       },
       "required": ["text"],
   }
   ```

<h4 id="returns-2">
  Retours
</h4>

Une fonction décorateur qui enveloppe l'implémentation de l'outil et retourne une instance `SdkMcpTool`.

<h4 id="example">
  Exemple
</h4>

```python theme={null}
from claude_agent_sdk import tool
from typing import Any


@tool("greet", "Greet a user", {"name": str})
async def greet(args: dict[str, Any]) -> dict[str, Any]:
    return {"content": [{"type": "text", "text": f"Hello, {args['name']}!"}]}
```

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

Réexportée depuis `mcp.types` (également disponible via `from claude_agent_sdk import ToolAnnotations`). Tous les champs sont des indices optionnels ; les clients ne doivent pas s'y fier pour les décisions de sécurité.

| Champ             | Type           | Par défaut | Description                                                                                                                                                         |
| :---------------- | :------------- | :--------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `title`           | `str \| None`  | `None`     | Titre lisible pour l'outil                                                                                                                                          |
| `readOnlyHint`    | `bool \| None` | `False`    | Si `True`, l'outil ne modifie pas son environnement                                                                                                                 |
| `destructiveHint` | `bool \| None` | `True`     | Si `True`, l'outil peut effectuer des mises à jour destructrices (uniquement significatif quand `readOnlyHint` est `False`)                                         |
| `idempotentHint`  | `bool \| None` | `False`    | Si `True`, les appels répétés avec les mêmes arguments n'ont pas d'effet supplémentaire (uniquement significatif quand `readOnlyHint` est `False`)                  |
| `openWorldHint`   | `bool \| None` | `True`     | Si `True`, l'outil interagit avec des entités externes (par exemple, recherche web). Si `False`, le domaine de l'outil est fermé (par exemple, un outil de mémoire) |

```python theme={null}
from claude_agent_sdk import tool, ToolAnnotations
from typing import Any


@tool(
    "search",
    "Search the web",
    {"query": str},
    annotations=ToolAnnotations(readOnlyHint=True, openWorldHint=True),
)
async def search(args: dict[str, Any]) -> dict[str, Any]:
    return {"content": [{"type": "text", "text": f"Results for: {args['query']}"}]}
```

<h3 id="create_sdk_mcp_server">
  `create_sdk_mcp_server()`
</h3>

Crée un serveur MCP en processus qui s'exécute dans votre application Python.

```python theme={null}
def create_sdk_mcp_server(
    name: str,
    version: str = "1.0.0",
    tools: list[SdkMcpTool[Any]] | None = None
) -> McpSdkServerConfig
```

<h4 id="parameters-3">
  Paramètres
</h4>

| Paramètre | Type                            | Par défaut | Description                                                   |
| :-------- | :------------------------------ | :--------- | :------------------------------------------------------------ |
| `name`    | `str`                           | -          | Identifiant unique pour le serveur                            |
| `version` | `str`                           | `"1.0.0"`  | Chaîne de version du serveur                                  |
| `tools`   | `list[SdkMcpTool[Any]] \| None` | `None`     | Liste des fonctions d'outil créées avec le décorateur `@tool` |

<h4 id="returns-3">
  Retours
</h4>

Retourne un objet `McpSdkServerConfig` qui peut être passé à `ClaudeAgentOptions.mcp_servers`.

<h4 id="example-2">
  Exemple
</h4>

```python theme={null}
from claude_agent_sdk import tool, create_sdk_mcp_server


@tool("add", "Add two numbers", {"a": float, "b": float})
async def add(args):
    return {"content": [{"type": "text", "text": f"Sum: {args['a'] + args['b']}"}]}


@tool("multiply", "Multiply two numbers", {"a": float, "b": float})
async def multiply(args):
    return {"content": [{"type": "text", "text": f"Product: {args['a'] * args['b']}"}]}


calculator = create_sdk_mcp_server(
    name="calculator",
    version="2.0.0",
    tools=[add, multiply],  # Pass decorated functions
)

# Use with Claude
options = ClaudeAgentOptions(
    mcp_servers={"calc": calculator},
    allowed_tools=["mcp__calc__add", "mcp__calc__multiply"],
)
```

<h3 id="list_sessions">
  `list_sessions()`
</h3>

Liste les sessions passées avec métadonnées. Filtrez par répertoire de projet ou listez les sessions dans tous les projets. Synchrone ; retourne immédiatement.

```python theme={null}
def list_sessions(
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0,
    include_worktrees: bool = True
) -> list[SDKSessionInfo]
```

<h4 id="parameters-4">
  Paramètres
</h4>

| Paramètre           | Type          | Par défaut | Description                                                                                                  |
| :------------------ | :------------ | :--------- | :----------------------------------------------------------------------------------------------------------- |
| `directory`         | `str \| None` | `None`     | Répertoire pour lequel lister les sessions. Quand omis, retourne les sessions dans tous les projets          |
| `limit`             | `int \| None` | `None`     | Nombre maximum de sessions à retourner                                                                       |
| `offset`            | `int`         | `0`        | Nombre de sessions à ignorer depuis le début des résultats triés. À utiliser avec `limit` pour la pagination |
| `include_worktrees` | `bool`        | `True`     | Quand `directory` est à l'intérieur d'un dépôt git, inclure les sessions de tous les chemins worktree        |

<h4 id="return-type-sdksessioninfo">
  Type de retour : `SDKSessionInfo`
</h4>

| Propriété       | Type          | Description                                                                              |
| :-------------- | :------------ | :--------------------------------------------------------------------------------------- |
| `session_id`    | `str`         | Identifiant de session unique                                                            |
| `summary`       | `str`         | Titre d'affichage : titre personnalisé, résumé généré automatiquement, ou premier prompt |
| `last_modified` | `int`         | Heure de dernière modification en millisecondes depuis l'époque                          |
| `file_size`     | `int \| None` | Taille du fichier de session en octets (`None` pour les backends de stockage distant)    |
| `custom_title`  | `str \| None` | Titre de session défini par l'utilisateur                                                |
| `first_prompt`  | `str \| None` | Premier prompt utilisateur significatif dans la session                                  |
| `git_branch`    | `str \| None` | Branche git à la fin de la session                                                       |
| `cwd`           | `str \| None` | Répertoire de travail pour la session                                                    |
| `tag`           | `str \| None` | Étiquette de session définie par l'utilisateur (voir [`tag_session()`](#tag_session))    |
| `created_at`    | `int \| None` | Heure de création de la session en millisecondes depuis l'époque                         |

<h4 id="example-3">
  Exemple
</h4>

Affiche les 10 sessions les plus récentes pour un projet. Les résultats sont triés par `last_modified` décroissant, donc le premier élément est le plus récent. Omettez `directory` pour rechercher dans tous les projets.

```python theme={null}
from claude_agent_sdk import list_sessions

for session in list_sessions(directory="/path/to/project", limit=10):
    print(f"{session.summary} ({session.session_id})")
```

<h3 id="get_session_messages">
  `get_session_messages()`
</h3>

Récupère les messages d'une session passée. Synchrone ; retourne immédiatement.

```python theme={null}
def get_session_messages(
    session_id: str,
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0
) -> list[SessionMessage]
```

<h4 id="parameters-5">
  Paramètres
</h4>

| Paramètre    | Type          | Par défaut | Description                                                                  |
| :----------- | :------------ | :--------- | :--------------------------------------------------------------------------- |
| `session_id` | `str`         | requis     | L'ID de session pour lequel récupérer les messages                           |
| `directory`  | `str \| None` | `None`     | Répertoire de projet à regarder. Quand omis, recherche dans tous les projets |
| `limit`      | `int \| None` | `None`     | Nombre maximum de messages à retourner                                       |
| `offset`     | `int`         | `0`        | Nombre de messages à ignorer depuis le début                                 |

<h4 id="return-type-sessionmessage">
  Type de retour : `SessionMessage`
</h4>

| Propriété            | Type                           | Description                   |
| :------------------- | :----------------------------- | :---------------------------- |
| `type`               | `Literal["user", "assistant"]` | Rôle du message               |
| `uuid`               | `str`                          | Identifiant de message unique |
| `session_id`         | `str`                          | Identifiant de session        |
| `message`            | `Any`                          | Contenu du message brut       |
| `parent_tool_use_id` | `None`                         | Réservé pour un usage futur   |

<h4 id="example-4">
  Exemple
</h4>

```python theme={null}
from claude_agent_sdk import list_sessions, get_session_messages

sessions = list_sessions(limit=1)
if sessions:
    messages = get_session_messages(sessions[0].session_id)
    for msg in messages:
        print(f"[{msg.type}] {msg.uuid}")
```

<h3 id="get_session_info">
  `get_session_info()`
</h3>

Lit les métadonnées d'une seule session par ID sans scanner le répertoire de projet complet. Synchrone ; retourne immédiatement.

```python theme={null}
def get_session_info(
    session_id: str,
    directory: str | None = None,
) -> SDKSessionInfo | None
```

<h4 id="parameters-6">
  Paramètres
</h4>

| Paramètre    | Type          | Par défaut | Description                                                                               |
| :----------- | :------------ | :--------- | :---------------------------------------------------------------------------------------- |
| `session_id` | `str`         | requis     | UUID de la session à rechercher                                                           |
| `directory`  | `str \| None` | `None`     | Chemin du répertoire de projet. Quand omis, recherche dans tous les répertoires de projet |

Retourne [`SDKSessionInfo`](#return-type-sdksessioninfo), ou `None` si la session n'est pas trouvée.

<h4 id="example-5">
  Exemple
</h4>

Recherchez les métadonnées d'une seule session sans scanner le répertoire de projet. Utile quand vous avez déjà un ID de session d'une exécution précédente.

```python theme={null}
from claude_agent_sdk import get_session_info

info = get_session_info("550e8400-e29b-41d4-a716-446655440000")
if info:
    print(f"{info.summary} (branch: {info.git_branch}, tag: {info.tag})")
```

<h3 id="rename_session">
  `rename_session()`
</h3>

Renomme une session en ajoutant une entrée de titre personnalisé. Les appels répétés sont sûrs ; le titre le plus récent gagne. Synchrone.

```python theme={null}
def rename_session(
    session_id: str,
    title: str,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-7">
  Paramètres
</h4>

| Paramètre    | Type          | Par défaut | Description                                                                               |
| :----------- | :------------ | :--------- | :---------------------------------------------------------------------------------------- |
| `session_id` | `str`         | requis     | UUID de la session à renommer                                                             |
| `title`      | `str`         | requis     | Nouveau titre. Doit être non-vide après suppression des espaces                           |
| `directory`  | `str \| None` | `None`     | Chemin du répertoire de projet. Quand omis, recherche dans tous les répertoires de projet |

Lève `ValueError` si `session_id` n'est pas un UUID valide ou si `title` est vide ; `FileNotFoundError` si la session ne peut pas être trouvée.

<h4 id="example-6">
  Exemple
</h4>

Renommez la session la plus récente pour qu'elle soit plus facile à trouver plus tard. Le nouveau titre apparaît dans [`SDKSessionInfo.custom_title`](#return-type-sdksessioninfo) lors des lectures ultérieures.

```python theme={null}
from claude_agent_sdk import list_sessions, rename_session

sessions = list_sessions(directory="/path/to/project", limit=1)
if sessions:
    rename_session(sessions[0].session_id, "Refactor auth module")
```

<h3 id="tag_session">
  `tag_session()`
</h3>

Étiquette une session. Passez `None` pour effacer l'étiquette. Les appels répétés sont sûrs ; l'étiquette la plus récente gagne. Synchrone.

```python theme={null}
def tag_session(
    session_id: str,
    tag: str | None,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-8">
  Paramètres
</h4>

| Paramètre    | Type          | Par défaut | Description                                                                               |
| :----------- | :------------ | :--------- | :---------------------------------------------------------------------------------------- |
| `session_id` | `str`         | requis     | UUID de la session à étiqueter                                                            |
| `tag`        | `str \| None` | requis     | Chaîne d'étiquette, ou `None` pour effacer. Nettoyée Unicode avant stockage               |
| `directory`  | `str \| None` | `None`     | Chemin du répertoire de projet. Quand omis, recherche dans tous les répertoires de projet |

Lève `ValueError` si `session_id` n'est pas un UUID valide ou si `tag` est vide après nettoyage ; `FileNotFoundError` si la session ne peut pas être trouvée.

<h4 id="example-7">
  Exemple
</h4>

Étiquetez une session, puis filtrez par cette étiquette lors d'une lecture ultérieure. Passez `None` pour effacer une étiquette existante.

```python theme={null}
from claude_agent_sdk import list_sessions, tag_session

# Tag a session
tag_session("550e8400-e29b-41d4-a716-446655440000", "needs-review")

# Later: find all sessions with that tag
for session in list_sessions(directory="/path/to/project"):
    if session.tag == "needs-review":
        print(session.summary)
```

<h2 id="classes">
  Classes
</h2>

<h3 id="claudesdkclient">
  `ClaudeSDKClient`
</h3>

**Maintient une session de conversation sur plusieurs échanges.** C'est l'équivalent Python de la façon dont la fonction `query()` du SDK TypeScript fonctionne en interne - elle crée un objet client qui peut continuer les conversations.

<h4 id="key-features">
  Fonctionnalités clés
</h4>

* **Continuité de session** : Maintient le contexte de conversation sur plusieurs appels `query()`
* **Même conversation** : La session conserve les messages précédents
* **Support des interruptions** : Peut arrêter l'exécution en cours de tâche
* **Cycle de vie explicite** : Vous contrôlez quand la session commence et se termine
* **Flux basé sur les réponses** : Peut réagir aux réponses et envoyer des suivis
* **Outils personnalisés et hooks** : Supporte les outils personnalisés (créés avec le décorateur `@tool`) et les hooks

```python theme={null}
class ClaudeSDKClient:
    def __init__(self, options: ClaudeAgentOptions | None = None, transport: Transport | None = None)
    async def connect(self, prompt: str | AsyncIterable[dict] | None = None) -> None
    async def query(self, prompt: str | AsyncIterable[dict], session_id: str = "default") -> None
    async def receive_messages(self) -> AsyncIterator[Message]
    async def receive_response(self) -> AsyncIterator[Message]
    async def interrupt(self) -> None
    async def set_permission_mode(self, mode: str) -> None
    async def set_model(self, model: str | None = None) -> None
    async def rewind_files(self, user_message_id: str) -> None
    async def get_mcp_status(self) -> McpStatusResponse
    async def reconnect_mcp_server(self, server_name: str) -> None
    async def toggle_mcp_server(self, server_name: str, enabled: bool) -> None
    async def stop_task(self, task_id: str) -> None
    async def get_server_info(self) -> dict[str, Any] | None
    async def disconnect(self) -> None
```

<h4 id="methods">
  Méthodes
</h4>

| Méthode                                   | Description                                                                                                                                                                |
| :---------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `__init__(options)`                       | Initialise le client avec une configuration optionnelle                                                                                                                    |
| `connect(prompt)`                         | Se connecte à Claude avec un prompt initial optionnel ou un flux de messages                                                                                               |
| `query(prompt, session_id)`               | Envoie une nouvelle requête en mode streaming                                                                                                                              |
| `receive_messages()`                      | Reçoit tous les messages de Claude comme un itérateur asynchrone                                                                                                           |
| `receive_response()`                      | Reçoit les messages jusqu'à et incluant un ResultMessage                                                                                                                   |
| `interrupt()`                             | Envoie un signal d'interruption (fonctionne uniquement en mode streaming)                                                                                                  |
| `set_permission_mode(mode)`               | Change le mode de permission pour la session actuelle                                                                                                                      |
| `set_model(model)`                        | Change le modèle pour la session actuelle. Passez `None` pour réinitialiser à la valeur par défaut                                                                         |
| `rewind_files(user_message_id)`           | Restaure les fichiers à leur état au message utilisateur spécifié. Nécessite `enable_file_checkpointing=True`. Voir [File checkpointing](/docs/fr/agent-sdk/file-checkpointing) |
| `get_mcp_status()`                        | Obtient le statut de tous les serveurs MCP configurés. Retourne [`McpStatusResponse`](#mcpstatusresponse)                                                                  |
| `reconnect_mcp_server(server_name)`       | Réessaye de se connecter à un serveur MCP qui a échoué ou a été déconnecté                                                                                                 |
| `toggle_mcp_server(server_name, enabled)` | Active ou désactive un serveur MCP en cours de session. La désactivation supprime ses outils                                                                               |
| `stop_task(task_id)`                      | Arrête une tâche de fond en cours d'exécution. Un [`TaskNotificationMessage`](#tasknotificationmessage) avec le statut `"stopped"` suit dans le flux de messages           |
| `get_server_info()`                       | Obtient les informations du serveur incluant l'ID de session et les capacités                                                                                              |
| `disconnect()`                            | Se déconnecte de Claude                                                                                                                                                    |

<h4 id="context-manager-support">
  Support du gestionnaire de contexte
</h4>

Le client peut être utilisé comme un gestionnaire de contexte asynchrone pour la gestion automatique de la connexion :

```python theme={null}
async with ClaudeSDKClient() as client:
    await client.query("Hello Claude")
    async for message in client.receive_response():
        print(message)
```

> **Important :** Lors de l'itération sur les messages, évitez d'utiliser `break` pour quitter tôt car cela peut causer des problèmes de nettoyage asyncio. À la place, laissez l'itération se terminer naturellement ou utilisez des drapeaux pour suivre quand vous avez trouvé ce que vous cherchiez.

<h4 id="example-continuing-a-conversation">
  Exemple - Continuer une conversation
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient, AssistantMessage, TextBlock, ResultMessage


async def main():
    async with ClaudeSDKClient() as client:
        # First question
        await client.query("What's the capital of France?")

        # Process response
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")

        # Follow-up question - the session retains the previous context
        await client.query("What's the population of that city?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")

        # Another follow-up - still in the same conversation
        await client.query("What are some famous landmarks there?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")


asyncio.run(main())
```

<h4 id="example-streaming-input-with-claudesdkclient">
  Exemple - Entrée en streaming avec ClaudeSDKClient
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient


async def message_stream():
    """Generate messages dynamically."""
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Analyze the following data:"},
    }
    await asyncio.sleep(0.5)
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Temperature: 25°C, Humidity: 60%"},
    }
    await asyncio.sleep(0.5)
    yield {
        "type": "user",
        "message": {"role": "user", "content": "What patterns do you see?"},
    }


async def main():
    async with ClaudeSDKClient() as client:
        # Stream input to Claude
        await client.query(message_stream())

        # Process response
        async for message in client.receive_response():
            print(message)

        # Follow-up in same session
        await client.query("Should we be concerned about these readings?")

        async for message in client.receive_response():
            print(message)


asyncio.run(main())
```

<h4 id="example-using-interrupts">
  Exemple - Utiliser les interruptions
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, ResultMessage


async def interruptible_task():
    options = ClaudeAgentOptions(allowed_tools=["Bash"], permission_mode="acceptEdits")

    async with ClaudeSDKClient(options=options) as client:
        # Start a long-running task
        await client.query("Count from 1 to 100 slowly, using the bash sleep command")

        # Let it run for a bit
        await asyncio.sleep(2)

        # Interrupt the task
        await client.interrupt()
        print("Task interrupted!")

        # Drain the interrupted task's messages (including its ResultMessage)
        async for message in client.receive_response():
            if isinstance(message, ResultMessage):
                print(f"Interrupted task finished with subtype={message.subtype!r}")
                # subtype is "error_during_execution" for interrupted tasks

        # Send a new command
        await client.query("Just say hello instead")

        # Now receive the new response
        async for message in client.receive_response():
            if isinstance(message, ResultMessage) and message.subtype == "success":
                print(f"New result: {message.result}")


asyncio.run(interruptible_task())
```

<Note>
  **Comportement du buffer après interruption :** `interrupt()` envoie un signal d'arrêt mais ne vide pas le buffer de messages. Les messages déjà produits par la tâche interrompue, incluant son `ResultMessage` (avec `subtype="error_during_execution"`), restent dans le flux. Vous devez les vider avec `receive_response()` avant de lire la réponse à une nouvelle requête. Si vous envoyez une nouvelle requête immédiatement après `interrupt()` et appelez `receive_response()` une seule fois, vous recevrez les messages de la tâche interrompue, pas la réponse de la nouvelle requête.
</Note>

<h4 id="example-advanced-permission-control">
  Exemple - Contrôle avancé des permissions
</h4>

```python theme={null}
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions
from claude_agent_sdk.types import (
    PermissionResultAllow,
    PermissionResultDeny,
    ToolPermissionContext,
)


async def custom_permission_handler(
    tool_name: str, input_data: dict, context: ToolPermissionContext
) -> PermissionResultAllow | PermissionResultDeny:
    """Custom logic for tool permissions."""

    # Block writes to system directories
    if tool_name == "Write" and input_data.get("file_path", "").startswith("/system/"):
        return PermissionResultDeny(
            message="System directory write not allowed", interrupt=True
        )

    # Redirect sensitive file operations
    if tool_name in ["Write", "Edit"] and "config" in input_data.get("file_path", ""):
        safe_path = f"./sandbox/{input_data['file_path']}"
        return PermissionResultAllow(
            updated_input={**input_data, "file_path": safe_path}
        )

    # Allow everything else
    return PermissionResultAllow(updated_input=input_data)


async def main():
    # Don't also list the gated tools in allowed_tools: allow rules approve calls before can_use_tool runs
    options = ClaudeAgentOptions(can_use_tool=custom_permission_handler)

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Update the system config file")

        async for message in client.receive_response():
            # Will use sandbox path instead
            print(message)


asyncio.run(main())
```

<h2 id="types">
  Types
</h2>

<Note>
  **`@dataclass` vs `TypedDict` :** Ce SDK utilise deux types de types. Les classes décorées avec `@dataclass` (telles que `ResultMessage`, `AgentDefinition`, `TextBlock`) sont des instances d'objet à l'exécution et supportent l'accès par attribut : `msg.result`. Les classes définies avec `TypedDict` (telles que `ThinkingConfigEnabled`, `McpStdioServerConfig`, `SyncHookJSONOutput`) sont **des dicts simples à l'exécution** et nécessitent l'accès par clé : `config["budget_tokens"]`, pas `config.budget_tokens`. La syntaxe d'appel `ClassName(field=value)` fonctionne pour les deux, mais seules les dataclasses produisent des objets avec des attributs.
</Note>

<h3 id="sdkmcptool">
  `SdkMcpTool`
</h3>

Définition pour un outil MCP SDK créé avec le décorateur `@tool`.

```python theme={null}
@dataclass
class SdkMcpTool(Generic[T]):
    name: str
    description: str
    input_schema: type[T] | dict[str, Any]
    handler: Callable[[T], Awaitable[dict[str, Any]]]
    annotations: ToolAnnotations | None = None
```

| Propriété      | Type                                       | Description                                                                                                    |
| :------------- | :----------------------------------------- | :------------------------------------------------------------------------------------------------------------- |
| `name`         | `str`                                      | Identifiant unique pour l'outil                                                                                |
| `description`  | `str`                                      | Description lisible                                                                                            |
| `input_schema` | `type[T] \| dict[str, Any]`                | Schéma pour la validation d'entrée                                                                             |
| `handler`      | `Callable[[T], Awaitable[dict[str, Any]]]` | Fonction asynchrone qui gère l'exécution de l'outil                                                            |
| `annotations`  | `ToolAnnotations \| None`                  | Annotations MCP optionnelles (par exemple, `readOnlyHint`, `destructiveHint`, `openWorldHint`). De `mcp.types` |

<h3 id="transport">
  `Transport`
</h3>

Classe de base abstraite pour les implémentations de transport personnalisées. Utilisez ceci pour communiquer avec le processus Claude sur un canal personnalisé (par exemple, une connexion distante au lieu d'un sous-processus local).

<Warning>
  C'est une API interne de bas niveau. L'interface peut changer dans les versions futures. Les implémentations personnalisées doivent être mises à jour pour correspondre à tout changement d'interface.
</Warning>

```python theme={null}
from abc import ABC, abstractmethod
from collections.abc import AsyncIterator
from typing import Any


class Transport(ABC):
    @abstractmethod
    async def connect(self) -> None: ...

    @abstractmethod
    async def write(self, data: str) -> None: ...

    @abstractmethod
    def read_messages(self) -> AsyncIterator[dict[str, Any]]: ...

    @abstractmethod
    async def close(self) -> None: ...

    @abstractmethod
    def is_ready(self) -> bool: ...

    @abstractmethod
    async def end_input(self) -> None: ...
```

| Méthode           | Description                                                                              |
| :---------------- | :--------------------------------------------------------------------------------------- |
| `connect()`       | Connecte le transport et prépare la communication                                        |
| `write(data)`     | Écrit les données brutes (JSON + nouvelle ligne) dans le transport                       |
| `read_messages()` | Itérateur asynchrone qui produit les messages JSON analysés                              |
| `close()`         | Ferme la connexion et nettoie les ressources                                             |
| `is_ready()`      | Retourne `True` si le transport peut envoyer et recevoir                                 |
| `end_input()`     | Ferme le flux d'entrée (par exemple, fermer stdin pour les transports de sous-processus) |

Importation : `from claude_agent_sdk import Transport`

<h3 id="claudeagentoptions">
  `ClaudeAgentOptions`
</h3>

Dataclass de configuration pour les requêtes Claude Code.

```python theme={null}
@dataclass
class ClaudeAgentOptions:
    tools: list[str] | ToolsPreset | None = None
    allowed_tools: list[str] = field(default_factory=list)
    system_prompt: str | SystemPromptPreset | SystemPromptFile | None = None
    mcp_servers: dict[str, McpServerConfig] | str | Path = field(default_factory=dict)
    strict_mcp_config: bool = False
    permission_mode: PermissionMode | None = None
    continue_conversation: bool = False
    resume: str | None = None
    max_turns: int | None = None
    max_budget_usd: float | None = None
    disallowed_tools: list[str] = field(default_factory=list)
    model: str | None = None
    fallback_model: str | None = None
    betas: list[SdkBeta] = field(default_factory=list)
    output_format: dict[str, Any] | None = None
    permission_prompt_tool_name: str | None = None
    cwd: str | Path | None = None
    cli_path: str | Path | None = None
    settings: str | None = None
    add_dirs: list[str | Path] = field(default_factory=list)
    env: dict[str, str] = field(default_factory=dict)
    extra_args: dict[str, str | None] = field(default_factory=dict)
    max_buffer_size: int | None = None
    debug_stderr: Any = sys.stderr  # Deprecated
    stderr: Callable[[str], None] | None = None
    can_use_tool: CanUseTool | None = None
    hooks: dict[HookEvent, list[HookMatcher]] | None = None
    user: str | None = None
    include_partial_messages: bool = False
    include_hook_events: bool = False
    fork_session: bool = False
    agents: dict[str, AgentDefinition] | None = None
    setting_sources: list[SettingSource] | None = None
    skills: list[str] | Literal["all"] | None = None
    sandbox: SandboxSettings | None = None
    plugins: list[SdkPluginConfig] = field(default_factory=list)
    max_thinking_tokens: int | None = None  # Deprecated: use thinking instead
    thinking: ThinkingConfig | None = None
    effort: EffortLevel | None = None
    enable_file_checkpointing: bool = False
    session_store: SessionStore | None = None
    session_store_flush: SessionStoreFlushMode = "batched"
```

| Propriété                     | Type                                                                                  | Par défaut                         | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :---------------------------- | :------------------------------------------------------------------------------------ | :--------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools`                       | `list[str] \| ToolsPreset \| None`                                                    | `None`                             | Configuration des outils. Utilisez `{"type": "preset", "preset": "claude_code"}` pour les outils par défaut de Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `allowed_tools`               | `list[str]`                                                                           | `[]`                               | Outils à approuver automatiquement sans demander. Ceci ne restreint pas Claude à seulement ces outils ; les outils non listés passent par `permission_mode` et `can_use_tool`. Utilisez `disallowed_tools` pour bloquer les outils. Voir [Permissions](/docs/fr/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                          |
| `system_prompt`               | `str \| SystemPromptPreset \| SystemPromptFile \| None`                               | `None`                             | Configuration du prompt système. Passez une chaîne pour un prompt personnalisé, `{"type": "preset", "preset": "claude_code"}` pour le prompt système de Claude Code avec `"append"` optionnel, ou `{"type": "file", "path": "..."}` pour charger un grand prompt depuis le disque. Voir [`SystemPromptPreset`](#systempromptpreset) et [`SystemPromptFile`](#systempromptfile)                                                                                                                                                                                                                                                                                                                  |
| `mcp_servers`                 | `dict[str, McpServerConfig] \| str \| Path`                                           | `{}`                               | Configurations de serveur MCP ou chemin vers le fichier de configuration                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `strict_mcp_config`           | `bool`                                                                                | `False`                            | Quand `True`, utilisez uniquement les serveurs passés dans `mcp_servers` et ignorez le projet `.mcp.json`, les paramètres utilisateur, les serveurs MCP fournis par les plugins, et les [connecteurs claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai). Correspond à l'indicateur CLI `--strict-mcp-config`                                                                                                                                                                                                                                                                                                                                                                                    |
| `permission_mode`             | `PermissionMode \| None`                                                              | `None`                             | Mode de permission pour l'utilisation des outils                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `continue_conversation`       | `bool`                                                                                | `False`                            | Continuer la conversation la plus récente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `resume`                      | `str \| None`                                                                         | `None`                             | ID de session à reprendre                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `max_turns`                   | `int \| None`                                                                         | `None`                             | Nombre maximum de tours agentiques (allers-retours d'utilisation d'outils)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `max_budget_usd`              | `float \| None`                                                                       | `None`                             | Arrêtez la requête quand l'estimation du coût côté client atteint cette valeur USD. Comparé à la même estimation que `total_cost_usd` ; voir [Suivi du coût et de l'utilisation](/docs/fr/agent-sdk/cost-tracking) pour les avertissements de précision                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `disallowed_tools`            | `list[str]`                                                                           | `[]`                               | Outils à refuser. Un nom simple tel que `"Bash"` supprime l'outil du contexte de Claude. Une règle délimitée telle que `"Bash(rm *)"` laisse l'outil disponible et refuse les appels correspondants dans chaque mode de permission, y compris `bypassPermissions`. Voir [Permissions](/docs/fr/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                           |
| `enable_file_checkpointing`   | `bool`                                                                                | `False`                            | Activez le suivi des modifications de fichiers pour le rembobinage. Voir [Sauvegarde de points de contrôle de fichiers](/docs/fr/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `model`                       | `str \| None`                                                                         | `None`                             | Alias de modèle Claude ou nom de modèle complet. Voir [valeurs acceptées et identifiants spécifiques au fournisseur](/docs/fr/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `fallback_model`              | `str \| None`                                                                         | `None`                             | Modèle de secours à utiliser si le modèle principal échoue                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `betas`                       | `list[SdkBeta]`                                                                       | `[]`                               | Fonctionnalités bêta à activer. Voir [`SdkBeta`](#sdkbeta) pour les options disponibles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `output_format`               | `dict[str, Any] \| None`                                                              | `None`                             | Format de sortie pour les réponses structurées (par exemple, `{"type": "json_schema", "schema": {...}}`). Voir [Sorties structurées](/docs/fr/agent-sdk/structured-outputs) pour les détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `permission_prompt_tool_name` | `str \| None`                                                                         | `None`                             | Nom de l'outil MCP pour les prompts de permission                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `cwd`                         | `str \| Path \| None`                                                                 | `None`                             | Répertoire de travail actuel                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `cli_path`                    | `str \| Path \| None`                                                                 | `None`                             | Chemin personnalisé vers l'exécutable CLI de Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `settings`                    | `str \| None`                                                                         | `None`                             | Chemin vers le fichier de paramètres                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `add_dirs`                    | `list[str \| Path]`                                                                   | `[]`                               | Répertoires supplémentaires auxquels Claude peut accéder                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `env`                         | `dict[str, str]`                                                                      | `{}`                               | Variables d'environnement fusionnées au-dessus de l'environnement de processus hérité. Voir [Variables d'environnement](/docs/fr/env-vars) pour les variables que le CLI sous-jacent lit, et [Gérer les réponses API lentes ou bloquées](#handle-slow-or-stalled-api-responses) pour les variables liées aux délais d'expiration                                                                                                                                                                                                                                                                                                                                                                     |
| `extra_args`                  | `dict[str, str \| None]`                                                              | `{}`                               | Arguments CLI supplémentaires à passer directement au CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `max_buffer_size`             | `int \| None`                                                                         | `None`                             | Octets maximum lors de la mise en buffer de la sortie standard du CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `debug_stderr`                | `Any`                                                                                 | `sys.stderr`                       | *Déprécié* - Objet de type fichier pour la sortie de débogage. Utilisez plutôt le callback `stderr`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `stderr`                      | `Callable[[str], None] \| None`                                                       | `None`                             | Fonction de callback pour la sortie stderr du CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `can_use_tool`                | [`CanUseTool`](#canusetool) ` \| None`                                                | `None`                             | Fonction de callback de permission d'outil, invoquée uniquement quand le [flux de permission](/docs/fr/agent-sdk/permissions#how-permissions-are-evaluated) aboutit à un prompt. Non invoquée pour les appels pré-approuvés par `allowed_tools`, les règles d'autorisation, ou `permission_mode`. `AskUserQuestion`, les outils connecteur [que votre organisation a défini sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools), et les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool) l'atteignent même si vous les avez autorisés ; en mode `dontAsk` ces appels sont refusés à la place. Voir [`CanUseTool`](#canusetool) pour les détails |
| `hooks`                       | `dict[HookEvent, list[HookMatcher]] \| None`                                          | `None`                             | Configurations de hook pour intercepter les événements                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `user`                        | `str \| None`                                                                         | `None`                             | Identifiant utilisateur                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `include_partial_messages`    | `bool`                                                                                | `False`                            | Inclure les événements de streaming de messages partiels. Quand activé, les messages [`StreamEvent`](#streamevent) sont produits                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `include_hook_events`         | `bool`                                                                                | `False`                            | Inclure les événements du cycle de vie des hooks dans le flux de messages en tant qu'objets `HookEventMessage`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `fork_session`                | `bool`                                                                                | `False`                            | Quand reprendre avec `resume`, bifurquer vers un nouvel ID de session au lieu de continuer la session originale                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `agents`                      | `dict[str, AgentDefinition] \| None`                                                  | `None`                             | Sous-agents définis programmatiquement                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `plugins`                     | `list[SdkPluginConfig]`                                                               | `[]`                               | Charger les plugins personnalisés à partir de chemins locaux. Voir [Plugins](/docs/fr/agent-sdk/plugins) pour les détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `sandbox`                     | [`SandboxSettings`](#sandboxsettings) ` \| None`                                      | `None`                             | Configurez le comportement du sandbox programmatiquement. Voir [Paramètres du sandbox](#sandboxsettings) pour les détails                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `setting_sources`             | `list[SettingSource] \| None`                                                         | `None` (CLI defaults: all sources) | Contrôlez quels paramètres du système de fichiers charger. Passez `[]` pour désactiver les paramètres utilisateur, projet et locaux. Les paramètres de politique gérée se chargent indépendamment ; les paramètres gérés par le serveur sont récupérés quand la session s'authentifie avec une credential d'organisation sur une [configuration éligible](/docs/fr/server-managed-settings#platform-availability). Voir [Utiliser les fonctionnalités de Claude Code](/docs/fr/agent-sdk/claude-code-features#what-settingsources-does-not-control) pour les entrées qui sont lues indépendamment de cette option, et comment les désactiver                                                              |
| `skills`                      | `list[str] \| Literal["all"] \| None`                                                 | `None`                             | Compétences disponibles pour la session. Passez `"all"` pour activer chaque compétence découverte, ou une liste de noms de compétences. Quand défini, le SDK ajoute l'outil Skill à `allowed_tools` automatiquement. Si vous passez aussi `tools`, incluez `"Skill"` dans cette liste. Voir [Compétences](/docs/fr/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                                 |
| `max_thinking_tokens`         | `int \| None`                                                                         | `None`                             | *Déprécié* - Tokens maximum pour les blocs de réflexion. Utilisez `thinking` à la place                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `thinking`                    | [`ThinkingConfig`](#thinkingconfig) ` \| None`                                        | `None`                             | Contrôle le comportement de la réflexion étendue. Prend la priorité sur `max_thinking_tokens`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `effort`                      | [`EffortLevel`](#effortlevel) ` \| None`                                              | `None`                             | Niveau d'effort pour la profondeur de réflexion. Voir [ajuster le niveau d'effort](/docs/fr/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `session_store`               | [`SessionStore`](/docs/fr/agent-sdk/session-storage#the-sessionstore-interface) ` \| None` | `None`                             | Miroir les transcriptions de session vers un backend externe pour que n'importe quel hôte puisse les reprendre. Voir [Persister les sessions vers un stockage externe](/docs/fr/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `session_store_flush`         | `Literal["batched", "eager"]`                                                         | `"batched"`                        | Quand vider les entrées de transcription en miroir vers `session_store`. `"batched"` vide une fois par tour ou quand le buffer se remplit ; `"eager"` déclenche un vidage en arrière-plan après chaque frame. Ignoré quand `session_store` est `None`                                                                                                                                                                                                                                                                                                                                                                                                                                           |

<h4 id="handle-slow-or-stalled-api-responses">
  Gérer les réponses API lentes ou bloquées
</h4>

Le sous-processus CLI lit plusieurs variables d'environnement qui contrôlent les délais d'expiration de l'API et la détection de blocage. Passez-les via `ClaudeAgentOptions.env` :

```python theme={null}
options = ClaudeAgentOptions(
    env={
        "API_TIMEOUT_MS": "120000",
        "CLAUDE_CODE_MAX_RETRIES": "2",
        "CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS": "120000",
    },
)
```

* `API_TIMEOUT_MS` : délai d'expiration par requête sur le client Anthropic, en millisecondes. Par défaut `600000`. S'applique à la boucle principale et à tous les sous-agents.
* `CLAUDE_CODE_MAX_RETRIES` : nombre maximum de tentatives API. Par défaut `10`, limité à `15`. Chaque tentative obtient sa propre fenêtre `API_TIMEOUT_MS`, donc le pire temps mural est approximativement `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` plus le backoff. Pour les exécutions sans surveillance qui doivent attendre des pannes plus longues, définissez `CLAUDE_CODE_RETRY_WATCHDOG=1` : il réessaye les erreurs de capacité indéfiniment, et à partir de Claude Code v2.1.199 augmente la valeur par défaut pour les autres erreurs transitoires à `300` et supprime le plafond sur cette variable.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` : chien de garde de blocage pour les sous-agents lancés avec `run_in_background`. Par défaut `600000`. Réinitialise à chaque événement de flux ; en cas de blocage, il abandonne le sous-agent, marque la tâche comme échouée et expose l'erreur au parent avec tout résultat partiel. Ne s'applique pas aux sous-agents synchrones.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` avec `CLAUDE_STREAM_IDLE_TIMEOUT_MS` : abandonne la requête quand les en-têtes sont arrivés mais le corps de la réponse cesse de faire du streaming. Le chien de garde est activé par défaut pour tous les fournisseurs ; définissez `CLAUDE_ENABLE_STREAM_WATCHDOG=0` pour le désactiver. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` par défaut à `300000` et est limité à ce minimum. La requête abandonnée passe par le chemin de tentative normal.

<h3 id="outputformat">
  `OutputFormat`
</h3>

Configuration pour la validation de sortie structurée. Passez ceci comme un `dict` au champ `output_format` sur `ClaudeAgentOptions` :

```python theme={null}
# Expected dict shape for output_format
{
    "type": "json_schema",
    "schema": {...},  # Your JSON Schema definition
}
```

| Champ    | Requis | Description                                              |
| :------- | :----- | :------------------------------------------------------- |
| `type`   | Oui    | Doit être `"json_schema"` pour la validation JSON Schema |
| `schema` | Oui    | Définition JSON Schema pour la validation de sortie      |

<h3 id="systempromptpreset">
  `SystemPromptPreset`
</h3>

Configuration pour utiliser le preset de prompt système de Claude Code avec des ajouts optionnels.

```python theme={null}
class SystemPromptPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
    append: NotRequired[str]
    exclude_dynamic_sections: NotRequired[bool]
```

| Champ                      | Requis | Description                                                                                                                                                                                                                                                                                                                                                                       |
| :------------------------- | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`                     | Oui    | Doit être `"preset"` pour utiliser un preset de prompt système                                                                                                                                                                                                                                                                                                                    |
| `preset`                   | Oui    | Doit être `"claude_code"` pour utiliser le prompt système de Claude Code                                                                                                                                                                                                                                                                                                          |
| `append`                   | Non    | Instructions supplémentaires à ajouter au preset de prompt système                                                                                                                                                                                                                                                                                                                |
| `exclude_dynamic_sections` | Non    | Déplacez le contexte par session comme le répertoire de travail, le statut git et les chemins de mémoire du prompt système vers le premier message utilisateur. Améliore la réutilisation du cache de prompt entre les utilisateurs et les machines. Voir [Modifier les prompts système](/docs/fr/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) |

<h3 id="systempromptfile">
  `SystemPromptFile`
</h3>

Configuration pour charger un prompt système personnalisé à partir d'un fichier au lieu de le passer en tant que chaîne. Le SDK mappe ceci à l'indicateur CLI [`--system-prompt-file`](/docs/fr/cli-reference#system-prompt-flags). Utilisez la forme fichier quand le prompt est volumineux : le SDK passe un `system_prompt` chaîne sur l'argv du sous-processus CLI, qui est soumis aux limites de longueur de ligne de commande du système d'exploitation avant que le SDK n'envoie une requête API. Sur Linux, un seul argument plus long que environ 128 KB échoue au spawn du processus avec `Argument list too long`. Sur Windows, la ligne de commande entière est limitée à environ 32 KB, donc la forme chaîne échoue à un seuil inférieur.

```python theme={null}
class SystemPromptFile(TypedDict):
    type: Literal["file"]
    path: str
```

| Champ  | Requis | Description                                                |
| :----- | :----- | :--------------------------------------------------------- |
| `type` | Oui    | Doit être `"file"` pour charger le prompt depuis le disque |
| `path` | Oui    | Chemin vers un fichier contenant le prompt système         |

<h3 id="settingsource">
  `SettingSource`
</h3>

Contrôle quelles sources de configuration basées sur le système de fichiers le SDK charge les paramètres à partir de.

```python theme={null}
SettingSource = Literal["user", "project", "local"]
```

| Valeur      | Description                                             | Emplacement                   |
| :---------- | :------------------------------------------------------ | :---------------------------- |
| `"user"`    | Paramètres utilisateur globaux                          | `~/.claude/settings.json`     |
| `"project"` | Paramètres de projet partagés (contrôle de version)     | `.claude/settings.json`       |
| `"local"`   | Paramètres de projet locaux (non contrôlés par version) | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Comportement par défaut
</h4>

Quand `setting_sources` est omis ou `None`, `query()` charge les mêmes paramètres du système de fichiers que le CLI Claude Code : utilisateur, projet et local. Les paramètres de politique gérée sont chargés dans tous les cas ; les paramètres gérés par le serveur sont récupérés quand la session s'authentifie avec une credential d'organisation sur une [configuration éligible](/docs/fr/server-managed-settings#platform-availability). Voir [Ce que settingSources ne contrôle pas](/docs/fr/agent-sdk/claude-code-features#what-settingsources-does-not-control) pour les entrées qui sont lues indépendamment de cette option, et comment les désactiver.

<h4 id="why-use-setting_sources">
  Pourquoi utiliser setting\_sources
</h4>

**Désactiver les paramètres du système de fichiers :**

```python theme={null}
# Do not load user, project, or local settings from disk
from claude_agent_sdk import query, ClaudeAgentOptions

async for message in query(
    prompt="Analyze this code",
    options=ClaudeAgentOptions(
        setting_sources=[]
    ),
):
    print(message)
```

<Note>
  Dans le SDK Python 0.1.59 et antérieur, une liste vide était traitée de la même manière que l'omission de l'option, donc `setting_sources=[]` n'a pas désactivé les paramètres du système de fichiers. Mettez à niveau vers une version plus récente si vous avez besoin qu'une liste vide prenne effet. Le SDK TypeScript n'est pas affecté.
</Note>

**Charger tous les paramètres du système de fichiers explicitement :**

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions

async for message in query(
    prompt="Analyze this code",
    options=ClaudeAgentOptions(
        setting_sources=["user", "project", "local"]
    ),
):
    print(message)
```

**Charger uniquement des sources de paramètres spécifiques :**

```python theme={null}
# Load only project settings, ignore user and local
async for message in query(
    prompt="Run CI checks",
    options=ClaudeAgentOptions(
        setting_sources=["project"]  # Only .claude/settings.json
    ),
):
    print(message)
```

**Environnements de test et CI :**

```python theme={null}
# Ensure consistent behavior in CI by excluding local settings
async for message in query(
    prompt="Run tests",
    options=ClaudeAgentOptions(
        setting_sources=["project"],  # Only team-shared settings
        permission_mode="bypassPermissions",
    ),
):
    print(message)
```

**Applications SDK uniquement :**

```python theme={null}
# Define everything programmatically.
# Pass [] to opt out of filesystem setting sources.
async for message in query(
    prompt="Review this PR",
    options=ClaudeAgentOptions(
        setting_sources=[],
        agents={...},
        mcp_servers={...},
        allowed_tools=["Read", "Grep", "Glob"],
    ),
):
    print(message)
```

**Chargement des instructions de projet CLAUDE.md :**

```python theme={null}
# Load project settings to include CLAUDE.md files
async for message in query(
    prompt="Add a new feature following project conventions",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",  # Use Claude Code's system prompt
        },
        setting_sources=["project"],  # Loads CLAUDE.md from project
        allowed_tools=["Read", "Write", "Edit"],
    ),
):
    print(message)
```

<h4 id="settings-precedence">
  Précédence des paramètres
</h4>

Quand plusieurs sources sont chargées, les paramètres sont fusionnés avec cette précédence (la plus haute à la plus basse) :

1. Paramètres locaux (`.claude/settings.local.json`)
2. Paramètres de projet (`.claude/settings.json`)
3. Paramètres utilisateur (`~/.claude/settings.json`)

Les options programmatiques telles que `agents` et `allowed_tools` remplacent les paramètres du système de fichiers utilisateur, projet et local. Les paramètres de politique gérée prennent la priorité sur les options programmatiques.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Configuration pour un sous-agent défini programmatiquement.

```python theme={null}
@dataclass
class AgentDefinition:
    description: str
    prompt: str
    tools: list[str] | None = None
    disallowedTools: list[str] | None = None
    model: str | None = None
    skills: list[str] | None = None
    memory: Literal["user", "project", "local"] | None = None
    mcpServers: list[str | dict[str, Any]] | None = None
    initialPrompt: str | None = None
    maxTurns: int | None = None
    background: bool | None = None
    effort: EffortLevel | int | None = None
    permissionMode: PermissionMode | None = None
```

| Champ             | Requis | Description                                                                                                                                                                                                                                                                   |
| :---------------- | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | Oui    | Description en langage naturel de quand utiliser cet agent                                                                                                                                                                                                                    |
| `prompt`          | Oui    | Le prompt système de l'agent                                                                                                                                                                                                                                                  |
| `tools`           | Non    | Tableau des noms d'outils autorisés. Si omis, hérite de tous les outils                                                                                                                                                                                                       |
| `disallowedTools` | Non    | Tableau des noms d'outils à supprimer de l'ensemble d'outils de l'agent. Les motifs au niveau du serveur MCP sont également acceptés : `mcp__server` ou `mcp__server__*` supprime chaque outil de ce serveur, et `mcp__*` supprime chaque outil MCP de n'importe quel serveur |
| `model`           | Non    | Remplacement de modèle pour cet agent. Accepte un alias tel que `"sonnet"`, `"opus"`, `"haiku"`, ou `"inherit"`, ou un ID de modèle complet. Si omis, utilise le modèle principal                                                                                             |
| `skills`          | Non    | Liste des noms de compétences à précharger dans le contexte de l'agent au démarrage. Les compétences non listées restent invocables via l'outil Skill                                                                                                                         |
| `memory`          | Non    | Source de mémoire pour cet agent : `"user"`, `"project"`, ou `"local"`                                                                                                                                                                                                        |
| `mcpServers`      | Non    | Serveurs MCP disponibles pour cet agent. Chaque entrée est un nom de serveur ou un dict `{name: config}` en ligne                                                                                                                                                             |
| `initialPrompt`   | Non    | Auto-soumis comme le premier tour utilisateur quand cet agent s'exécute comme l'agent du fil principal                                                                                                                                                                        |
| `maxTurns`        | Non    | Nombre maximum de tours agentiques avant que l'agent s'arrête                                                                                                                                                                                                                 |
| `background`      | Non    | Exécutez cet agent comme une tâche de fond non-bloquante quand invoqué                                                                                                                                                                                                        |
| `effort`          | Non    | Niveau d'effort de raisonnement pour cet agent. Accepte un niveau nommé ou un entier. Voir [`EffortLevel`](#effortlevel)                                                                                                                                                      |
| `permissionMode`  | Non    | Mode de permission pour l'exécution des outils dans cet agent. Voir [`PermissionMode`](#permissionmode)                                                                                                                                                                       |

<Note>
  Les noms de champs `AgentDefinition` utilisent camelCase, tels que `disallowedTools`, `permissionMode`, et `maxTurns`. Ces noms correspondent directement au format de fil partagé avec le SDK TypeScript. Ceci diffère de `ClaudeAgentOptions`, qui utilise Python snake\_case pour les champs de niveau supérieur équivalents tels que `disallowed_tools` et `permission_mode`. Parce que `AgentDefinition` est une dataclass, passer un mot-clé snake\_case lève une `TypeError` au moment de la construction.
</Note>

<h3 id="permissionmode">
  `PermissionMode`
</h3>

Modes de permission pour contrôler l'exécution des outils.

```python theme={null}
PermissionMode = Literal[
    "default",  # Standard permission behavior
    "acceptEdits",  # Auto-accept file edits
    "plan",  # Planning mode - explore without editing
    "dontAsk",  # Deny anything not pre-approved instead of prompting
    "bypassPermissions",  # Bypass permission checks; explicit ask rules still prompt (use with caution)
    "auto",  # A model classifier approves or denies each tool call
]
```

<h3 id="effortlevel">
  `EffortLevel`
</h3>

Niveaux d'effort pour guider la profondeur de réflexion.

```python theme={null}
EffortLevel = Literal[
    "low",  # Minimal thinking, fastest responses
    "medium",  # Moderate thinking
    "high",  # Deep reasoning
    "xhigh",  # Extended reasoning; falls back to "high" on models that don't support it
    "max",  # Maximum effort
]
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Alias de type pour les fonctions de callback de permission d'outil.

```python theme={null}
CanUseTool = Callable[
    [str, dict[str, Any], ToolPermissionContext], Awaitable[PermissionResult]
]
```

Le callback reçoit :

* `tool_name` : Nom de l'outil en cours d'appel
* `input_data` : Les paramètres d'entrée de l'outil
* `context` : Un `ToolPermissionContext` avec des informations supplémentaires

Retourne un `PermissionResult` (soit `PermissionResultAllow` soit `PermissionResultDeny`).

Le callback est le remplacement SDK pour le prompt de permission interactif : il est invoqué uniquement quand le [flux d'évaluation de permission](/docs/fr/agent-sdk/permissions#how-permissions-are-evaluated) aboutit à un prompt. Les appels d'outil déjà approuvés par une entrée `allowed_tools`, une règle d'autorisation des paramètres, ou le mode de permission, tel que `acceptEdits` ou `bypassPermissions`, ne l'invoquent jamais. Pour contrôler chaque appel d'outil, utilisez un [hook `PreToolUse`](/docs/fr/agent-sdk/hooks) à la place.

`AskUserQuestion`, les outils MCP marqués [`requiresUserInteraction`](/docs/fr/mcp#require-approval-for-a-specific-tool), et les outils connecteur [que votre organisation a défini sur `ask`](/docs/fr/mcp#organization-controls-on-connector-tools) l'atteignent même quand une règle d'autorisation correspond. En mode `dontAsk` ces appels sont refusés à la place, sans invoquer le callback.

<h3 id="toolpermissioncontext">
  `ToolPermissionContext`
</h3>

Informations de contexte passées aux callbacks de permission d'outil.

```python theme={null}
@dataclass
class ToolPermissionContext:
    signal: Any | None = None  # Future: abort signal support
    suggestions: list[PermissionUpdate] = field(default_factory=list)
    blocked_path: str | None = None
    decision_reason: str | None = None
    title: str | None = None
    display_name: str | None = None
    description: str | None = None
```

| Champ             | Type                     | Description                                                                                                                                                                                                                                                   |
| :---------------- | :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `signal`          | `Any \| None`            | Réservé pour le support futur du signal d'abandon                                                                                                                                                                                                             |
| `suggestions`     | `list[PermissionUpdate]` | Suggestions de mise à jour de permission du CLI. Les prompts Bash incluent une suggestion avec la destination `localSettings`, donc la retourner dans `updated_permissions` écrit la règle dans `.claude/settings.local.json` et persiste entre les sessions. |
| `blocked_path`    | `str \| None`            | Chemin de fichier qui a déclenché la demande de permission, le cas échéant. Par exemple, quand une commande Bash essaie d'accéder à un chemin en dehors des répertoires autorisés                                                                             |
| `decision_reason` | `str \| None`            | Raison pour laquelle cette demande de permission a été déclenchée. Transférée depuis le `permissionDecisionReason` d'un hook PreToolUse quand le hook a retourné `"ask"`                                                                                      |
| `title`           | `str \| None`            | Phrase complète du prompt de permission, telle que `Claude wants to read foo.txt`. Utilisez comme texte du prompt principal quand présent                                                                                                                     |
| `display_name`    | `str \| None`            | Phrase nominale courte pour l'action de l'outil, telle que `Read file`, appropriée pour les étiquettes de bouton                                                                                                                                              |
| `description`     | `str \| None`            | Sous-titre lisible pour l'interface utilisateur de permission                                                                                                                                                                                                 |

<h3 id="permissionresult">
  `PermissionResult`
</h3>

Type union pour les résultats de callback de permission.

```python theme={null}
PermissionResult = PermissionResultAllow | PermissionResultDeny
```

<h3 id="permissionresultallow">
  `PermissionResultAllow`
</h3>

Résultat indiquant que l'appel d'outil doit être autorisé.

```python theme={null}
@dataclass
class PermissionResultAllow:
    behavior: Literal["allow"] = "allow"
    updated_input: dict[str, Any] | None = None
    updated_permissions: list[PermissionUpdate] | None = None
```

| Champ                 | Type                             | Par défaut | Description                                         |
| :-------------------- | :------------------------------- | :--------- | :-------------------------------------------------- |
| `behavior`            | `Literal["allow"]`               | `"allow"`  | Doit être « allow »                                 |
| `updated_input`       | `dict[str, Any] \| None`         | `None`     | Entrée modifiée à utiliser à la place de l'original |
| `updated_permissions` | `list[PermissionUpdate] \| None` | `None`     | Mises à jour de permission à appliquer              |

<h3 id="permissionresultdeny">
  `PermissionResultDeny`
</h3>

Résultat indiquant que l'appel d'outil doit être refusé.

```python theme={null}
@dataclass
class PermissionResultDeny:
    behavior: Literal["deny"] = "deny"
    message: str = ""
    interrupt: bool = False
```

| Champ       | Type              | Par défaut | Description                                      |
| :---------- | :---------------- | :--------- | :----------------------------------------------- |
| `behavior`  | `Literal["deny"]` | `"deny"`   | Doit être « deny »                               |
| `message`   | `str`             | `""`       | Message expliquant pourquoi l'outil a été refusé |
| `interrupt` | `bool`            | `False`    | S'il faut interrompre l'exécution actuelle       |

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

Configuration pour mettre à jour les permissions programmatiquement.

```python theme={null}
@dataclass
class PermissionUpdate:
    type: Literal[
        "addRules",
        "replaceRules",
        "removeRules",
        "setMode",
        "addDirectories",
        "removeDirectories",
    ]
    rules: list[PermissionRuleValue] | None = None
    behavior: Literal["allow", "deny", "ask"] | None = None
    mode: PermissionMode | None = None
    directories: list[str] | None = None
    destination: (
        Literal["userSettings", "projectSettings", "localSettings", "session"] | None
    ) = None
```

| Champ         | Type                                      | Description                                                       |
| :------------ | :---------------------------------------- | :---------------------------------------------------------------- |
| `type`        | `Literal[...]`                            | Le type d'opération de mise à jour de permission                  |
| `rules`       | `list[PermissionRuleValue] \| None`       | Règles pour les opérations d'ajout/remplacement/suppression       |
| `behavior`    | `Literal["allow", "deny", "ask"] \| None` | Comportement pour les opérations basées sur les règles            |
| `mode`        | `PermissionMode \| None`                  | Mode pour l'opération setMode                                     |
| `directories` | `list[str] \| None`                       | Répertoires pour les opérations d'ajout/suppression de répertoire |
| `destination` | `Literal[...] \| None`                    | Où appliquer la mise à jour de permission                         |

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

Une règle à ajouter, remplacer ou supprimer dans une mise à jour de permission.

```python theme={null}
@dataclass
class PermissionRuleValue:
    tool_name: str
    rule_content: str | None = None
```

<h3 id="toolspreset">
  `ToolsPreset`
</h3>

Configuration des outils preset pour utiliser l'ensemble d'outils par défaut de Claude Code.

```python theme={null}
class ToolsPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Contrôle le comportement de la réflexion étendue. Une union de trois configurations :

```python theme={null}
ThinkingDisplay = Literal["summarized", "omitted"]


class ThinkingConfigAdaptive(TypedDict):
    type: Literal["adaptive"]
    display: NotRequired[ThinkingDisplay]


class ThinkingConfigEnabled(TypedDict):
    type: Literal["enabled"]
    budget_tokens: int
    display: NotRequired[ThinkingDisplay]


class ThinkingConfigDisabled(TypedDict):
    type: Literal["disabled"]


ThinkingConfig = ThinkingConfigAdaptive | ThinkingConfigEnabled | ThinkingConfigDisabled
```

| Variante   | Champs                             | Description                                              |
| :--------- | :--------------------------------- | :------------------------------------------------------- |
| `adaptive` | `type`, `display`                  | Claude décide de manière adaptative quand réfléchir      |
| `enabled`  | `type`, `budget_tokens`, `display` | Activez la réflexion avec un budget de tokens spécifique |
| `disabled` | `type`                             | Désactivez la réflexion                                  |

Le champ optionnel `display` contrôle si le texte de réflexion est retourné `"summarized"` ou `"omitted"`. Sur Claude Opus 4.7 et ultérieur, la valeur par défaut de l'API est `"omitted"`, donc définissez `"summarized"` pour recevoir le contenu de réflexion dans les sorties [`ThinkingBlock`](#thinkingblock).

Parce que ce sont des classes `TypedDict`, ce sont des dicts simples à l'exécution. Construisez-les soit comme des littéraux dict soit appelez la classe comme un constructeur ; les deux produisent un `dict`. Accédez aux champs avec `config["budget_tokens"]`, pas `config.budget_tokens` :

```python theme={null}
from claude_agent_sdk import ClaudeAgentOptions, ThinkingConfigEnabled

# Option 1: dict literal (recommended, no import needed)
options = ClaudeAgentOptions(thinking={"type": "enabled", "budget_tokens": 20000})

# Option 2: constructor-style (returns a plain dict)
config = ThinkingConfigEnabled(type="enabled", budget_tokens=20000)
print(config["budget_tokens"])  # 20000
# config.budget_tokens would raise AttributeError
```

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Type littéral pour les fonctionnalités bêta du SDK.

```python theme={null}
SdkBeta = Literal["context-1m-2025-08-07"]
```

Utilisez avec le champ `betas` dans `ClaudeAgentOptions` pour activer les fonctionnalités bêta.

<Warning>
  La bêta `context-1m-2025-08-07` est retirée à partir du 30 avril 2026. Passer cet en-tête avec Claude Sonnet 4.5 ou Sonnet 4 n'a aucun effet, et les requêtes qui dépassent la fenêtre de contexte standard de 200 000 tokens retournent une erreur. Pour utiliser une fenêtre de contexte de 1 million de tokens, migrez vers [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7, ou Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), qui incluent 1 million de contexte à prix standard sans en-tête bêta requis.
</Warning>

<h3 id="mcpsdkserverconfig">
  `McpSdkServerConfig`
</h3>

Configuration pour les serveurs MCP SDK créés avec `create_sdk_mcp_server()`.

```python theme={null}
class McpSdkServerConfig(TypedDict):
    type: Literal["sdk"]
    name: str
    instance: Any  # MCP Server instance
```

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

Type union pour les configurations de serveur MCP.

```python theme={null}
McpServerConfig = (
    McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig
)
```

<h4 id="mcpstdioserverconfig">
  `McpStdioServerConfig`
</h4>

```python theme={null}
class McpStdioServerConfig(TypedDict):
    type: NotRequired[Literal["stdio"]]  # Optional for backwards compatibility
    command: str
    args: NotRequired[list[str]]
    env: NotRequired[dict[str, str]]
```

<h4 id="mcpsseserverconfig">
  `McpSSEServerConfig`
</h4>

```python theme={null}
class McpSSEServerConfig(TypedDict):
    type: Literal["sse"]
    url: str
    headers: NotRequired[dict[str, str]]
```

<h4 id="mcphttpserverconfig">
  `McpHttpServerConfig`
</h4>

```python theme={null}
class McpHttpServerConfig(TypedDict):
    type: Literal["http"]
    url: str
    headers: NotRequired[dict[str, str]]
```

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

La configuration d'un serveur MCP telle que rapportée par [`get_mcp_status()`](#methods). C'est l'union de toutes les variantes de transport [`McpServerConfig`](#mcpserverconfig) plus une variante de sortie uniquement `claudeai-proxy` pour les serveurs proxifiés via claude.ai.

```python theme={null}
McpServerStatusConfig = (
    McpStdioServerConfig
    | McpSSEServerConfig
    | McpHttpServerConfig
    | McpSdkServerConfigStatus
    | McpClaudeAIProxyServerConfig
)
```

`McpSdkServerConfigStatus` est la forme sérialisable de [`McpSdkServerConfig`](#mcpsdkserverconfig) avec seulement les champs `type` (`"sdk"`) et `name` (`str`) ; l'`instance` en processus est omise. `McpClaudeAIProxyServerConfig` a les champs `type` (`"claudeai-proxy"`), `url` (`str`), et `id` (`str`).

<h3 id="mcpstatusresponse">
  `McpStatusResponse`
</h3>

Réponse de [`ClaudeSDKClient.get_mcp_status()`](#methods). Enveloppe la liste des statuts de serveur sous la clé `mcpServers`.

```python theme={null}
class McpStatusResponse(TypedDict):
    mcpServers: list[McpServerStatus]
```

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

Statut d'un serveur MCP connecté, contenu dans [`McpStatusResponse`](#mcpstatusresponse).

```python theme={null}
class McpServerStatus(TypedDict):
    name: str
    status: McpServerConnectionStatus  # "connected" | "failed" | "needs-auth" | "pending" | "disabled"
    serverInfo: NotRequired[McpServerInfo]
    error: NotRequired[str]
    config: NotRequired[McpServerStatusConfig]
    scope: NotRequired[str]
    tools: NotRequired[list[McpToolInfo]]
```

| Champ        | Type                                                          | Description                                                                                                                                                                             |
| :----------- | :------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`       | `str`                                                         | Nom du serveur                                                                                                                                                                          |
| `status`     | `str`                                                         | L'un de `"connected"`, `"failed"`, `"needs-auth"`, `"pending"`, ou `"disabled"`                                                                                                         |
| `serverInfo` | `dict` (optionnel)                                            | Nom et version du serveur (`{"name": str, "version": str}`)                                                                                                                             |
| `error`      | `str` (optionnel)                                             | Message d'erreur si le serveur n'a pas pu se connecter                                                                                                                                  |
| `config`     | [`McpServerStatusConfig`](#mcpserverstatusconfig) (optionnel) | Configuration du serveur. Même forme que [`McpServerConfig`](#mcpserverconfig) (stdio, SSE, HTTP, ou SDK), plus une variante `claudeai-proxy` pour les serveurs connectés via claude.ai |
| `scope`      | `str` (optionnel)                                             | Portée de la configuration                                                                                                                                                              |
| `tools`      | `list` (optionnel)                                            | Outils fournis par ce serveur, chacun avec les champs `name`, `description`, et `annotations`                                                                                           |

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

Configuration pour charger les plugins dans le SDK.

```python theme={null}
class SdkPluginConfig(TypedDict):
    type: Literal["local"]
    path: str
```

| Champ  | Type               | Description                                                                |
| :----- | :----------------- | :------------------------------------------------------------------------- |
| `type` | `Literal["local"]` | Doit être `"local"` (seuls les plugins locaux sont actuellement supportés) |
| `path` | `str`              | Chemin absolu ou relatif vers le répertoire du plugin                      |

**Exemple :**

```python theme={null}
plugins = [
    {"type": "local", "path": "./my-plugin"},
    {"type": "local", "path": "/absolute/path/to/plugin"},
]
```

Pour des informations complètes sur la création et l'utilisation de plugins, voir [Plugins](/docs/fr/agent-sdk/plugins).

<h2 id="message-types">
  Types de messages
</h2>

<h3 id="message">
  `Message`
</h3>

Type union de tous les messages possibles.

```python theme={null}
Message = (
    UserMessage
    | AssistantMessage
    | SystemMessage
    | ResultMessage
    | StreamEvent
    | RateLimitEvent
)
```

<h3 id="usermessage">
  `UserMessage`
</h3>

Message d'entrée utilisateur.

```python theme={null}
@dataclass
class UserMessage:
    content: str | list[ContentBlock]
    uuid: str | None = None
    parent_tool_use_id: str | None = None
    tool_use_result: dict[str, Any] | None = None
```

| Champ                | Type                        | Description                                                                |
| :------------------- | :-------------------------- | :------------------------------------------------------------------------- |
| `content`            | `str \| list[ContentBlock]` | Contenu du message sous forme de texte ou de blocs de contenu              |
| `uuid`               | `str \| None`               | Identifiant de message unique                                              |
| `parent_tool_use_id` | `str \| None`               | ID d'utilisation d'outil si ce message est une réponse de résultat d'outil |
| `tool_use_result`    | `dict[str, Any] \| None`    | Données de résultat d'outil si applicable                                  |

<h3 id="assistantmessage">
  `AssistantMessage`
</h3>

Message de réponse d'assistant avec blocs de contenu.

```python theme={null}
@dataclass
class AssistantMessage:
    content: list[ContentBlock]
    model: str
    parent_tool_use_id: str | None = None
    error: AssistantMessageError | None = None
    usage: dict[str, Any] | None = None
    message_id: str | None = None
```

| Champ                | Type                                                         | Description                                                                                |
| :------------------- | :----------------------------------------------------------- | :----------------------------------------------------------------------------------------- |
| `content`            | `list[ContentBlock]`                                         | Liste des blocs de contenu dans la réponse                                                 |
| `model`              | `str`                                                        | Modèle qui a généré la réponse                                                             |
| `parent_tool_use_id` | `str \| None`                                                | ID d'utilisation d'outil si c'est une réponse imbriquée                                    |
| `error`              | [`AssistantMessageError`](#assistantmessageerror) ` \| None` | Type d'erreur si la réponse a rencontré une erreur                                         |
| `usage`              | `dict[str, Any] \| None`                                     | Utilisation de tokens par message (mêmes clés que [`ResultMessage.usage`](#resultmessage)) |
| `message_id`         | `str \| None`                                                | ID de message API. Plusieurs messages d'un tour partagent le même ID                       |

<h3 id="assistantmessageerror">
  `AssistantMessageError`
</h3>

Types d'erreur possibles pour les messages d'assistant.

```python theme={null}
AssistantMessageError = Literal[
    "authentication_failed",
    "billing_error",
    "rate_limit",
    "invalid_request",
    "server_error",
    "max_output_tokens",
    "unknown",
]
```

<h3 id="systemmessage">
  `SystemMessage`
</h3>

Message système avec métadonnées.

```python theme={null}
@dataclass
class SystemMessage:
    subtype: str
    data: dict[str, Any]
```

<h3 id="resultmessage">
  `ResultMessage`
</h3>

Message de résultat final avec informations de coût et d'utilisation.

```python theme={null}
@dataclass
class ResultMessage:
    subtype: str
    duration_ms: int
    duration_api_ms: int
    is_error: bool
    num_turns: int
    session_id: str
    stop_reason: str | None = None
    total_cost_usd: float | None = None
    usage: dict[str, Any] | None = None
    result: str | None = None
    structured_output: Any = None
    model_usage: dict[str, Any] | None = None
    permission_denials: list[Any] | None = None
    deferred_tool_use: DeferredToolUse | None = None
    errors: list[str] | None = None
    api_error_status: int | None = None
    uuid: str | None = None
```

Le champ `subtype` détermine quels autres champs sont remplis. C'est l'un de `"success"`, `"error_during_execution"`, `"error_max_turns"`, `"error_max_budget_usd"`, ou `"error_max_structured_output_retries"`. La dataclass Python aplatit toutes les variantes en une seule forme, donc les champs qui ne s'appliquent pas au sous-type retourné sont `None`.

Plusieurs champs portent des détails de diagnostic quand la conversation se termine sur une erreur :

* `is_error` : `True` quand la conversation s'est terminée dans un état d'erreur. Toujours `True` sur les sous-types `error_*`. Sur `subtype="success"` c'est `True` quand la dernière demande de modèle a échoué, ce qui signifie que la boucle d'agent s'est terminée mais le dernier appel API a retourné une erreur.
* `api_error_status` : le code de statut HTTP de l'erreur API terminale. `None` quand le tour s'est terminé sans une. Rempli uniquement sur `subtype="success"`.
* `result` : texte du message d'assistant final sur `subtype="success"`, ou `None` sur les sous-types `error_*`. Quand `subtype="success"` et `is_error=True`, ceci contient la chaîne d'erreur API si une est disponible mais peut être vide, donc vérifiez `api_error_status` et le contenu `AssistantMessage` précédent pour plus de détails.
* `errors` : chaînes d'erreur au niveau de la boucle telles que le message max-turns. Rempli uniquement sur les sous-types `error_*`.

Le dict `usage` contient les clés suivantes quand présentes :

| Clé                           | Type  | Description                                                                                                                                                                                                                                        |
| ----------------------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `input_tokens`                | `int` | Tokens d'entrée consommés par la boucle d'agent de haut niveau. [Les tokens de sous-agent ne sont pas inclus](/docs/fr/agent-sdk/cost-tracking#get-the-total-cost-of-a-query) ; utilisez `model_usage` pour la comptabilité de l'arborescence complète. |
| `output_tokens`               | `int` | Tokens de sortie générés par la boucle d'agent de haut niveau. Les tokens de sous-agent ne sont pas inclus.                                                                                                                                        |
| `cache_creation_input_tokens` | `int` | Tokens utilisés pour créer de nouvelles entrées de cache.                                                                                                                                                                                          |
| `cache_read_input_tokens`     | `int` | Tokens lus à partir des entrées de cache existantes.                                                                                                                                                                                               |

Le dict `model_usage` mappe les noms de modèles à l'utilisation par modèle. Les clés du dict interne utilisent camelCase parce que la valeur est passée inchangée du processus CLI sous-jacent, correspondant au type TypeScript [`ModelUsage`](/docs/fr/agent-sdk/typescript#modelusage) :

| Clé                        | Type    | Description                                                                                                                                                         |
| -------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `inputTokens`              | `int`   | Tokens d'entrée pour ce modèle.                                                                                                                                     |
| `outputTokens`             | `int`   | Tokens de sortie pour ce modèle.                                                                                                                                    |
| `cacheReadInputTokens`     | `int`   | Tokens de lecture de cache pour ce modèle.                                                                                                                          |
| `cacheCreationInputTokens` | `int`   | Tokens de création de cache pour ce modèle.                                                                                                                         |
| `webSearchRequests`        | `int`   | Requêtes de recherche web effectuées par ce modèle.                                                                                                                 |
| `costUSD`                  | `float` | Coût estimé en USD pour ce modèle, calculé côté client. Voir [Suivre le coût et l'utilisation](/docs/fr/agent-sdk/cost-tracking) pour les avertissements de facturation. |
| `contextWindow`            | `int`   | Taille de la fenêtre de contexte pour ce modèle.                                                                                                                    |
| `maxOutputTokens`          | `int`   | Limite de tokens de sortie maximum pour ce modèle.                                                                                                                  |

<h3 id="streamevent">
  `StreamEvent`
</h3>

Événement de flux pour les mises à jour de messages partiels pendant le streaming. Reçu uniquement quand `include_partial_messages=True` dans `ClaudeAgentOptions`. Importez via `from claude_agent_sdk.types import StreamEvent`.

```python theme={null}
@dataclass
class StreamEvent:
    uuid: str
    session_id: str
    event: dict[str, Any]  # The raw Claude API stream event
    parent_tool_use_id: str | None = None
```

| Champ                | Type             | Description                                                                                                                                                                                                |
| :------------------- | :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `uuid`               | `str`            | Identifiant unique pour cet événement                                                                                                                                                                      |
| `session_id`         | `str`            | Identifiant de session                                                                                                                                                                                     |
| `event`              | `dict[str, Any]` | Les données d'événement de flux Claude API brutes                                                                                                                                                          |
| `parent_tool_use_id` | `str \| None`    | Toujours `None`. Les événements de flux sont émis pour la session principale uniquement. Pour l'attribution de sous-agent, utilisez des messages complets tels que [`AssistantMessage`](#assistantmessage) |

<h3 id="ratelimitevent">
  `RateLimitEvent`
</h3>

Émis quand le statut de limite de débit change (par exemple, de `"allowed"` à `"allowed_warning"`). Utilisez ceci pour avertir les utilisateurs avant qu'ils ne frappent une limite dure, ou pour reculer quand le statut est `"rejected"`.

```python theme={null}
@dataclass
class RateLimitEvent:
    rate_limit_info: RateLimitInfo
    uuid: str
    session_id: str
```

| Champ             | Type                              | Description                       |
| :---------------- | :-------------------------------- | :-------------------------------- |
| `rate_limit_info` | [`RateLimitInfo`](#ratelimitinfo) | État actuel de la limite de débit |
| `uuid`            | `str`                             | Identifiant d'événement unique    |
| `session_id`      | `str`                             | Identifiant de session            |

<h3 id="ratelimitinfo">
  `RateLimitInfo`
</h3>

État de limite de débit porté par [`RateLimitEvent`](#ratelimitevent).

```python theme={null}
RateLimitStatus = Literal["allowed", "allowed_warning", "rejected"]
RateLimitType = Literal[
    "five_hour", "seven_day", "seven_day_opus", "seven_day_sonnet", "overage"
]


@dataclass
class RateLimitInfo:
    status: RateLimitStatus
    resets_at: int | None = None
    rate_limit_type: RateLimitType | None = None
    utilization: float | None = None
    overage_status: RateLimitStatus | None = None
    overage_resets_at: int | None = None
    overage_disabled_reason: str | None = None
    raw: dict[str, Any] = field(default_factory=dict)
```

| Champ                     | Type                      | Description                                                                                                          |
| :------------------------ | :------------------------ | :------------------------------------------------------------------------------------------------------------------- |
| `status`                  | `RateLimitStatus`         | Statut actuel. `"allowed_warning"` signifie approcher la limite ; `"rejected"` signifie que la limite a été atteinte |
| `resets_at`               | `int \| None`             | Timestamp Unix quand la fenêtre de limite de débit se réinitialise                                                   |
| `rate_limit_type`         | `RateLimitType \| None`   | Quelle fenêtre de limite de débit s'applique                                                                         |
| `utilization`             | `float \| None`           | Fraction de la limite de débit consommée (0,0 à 1,0)                                                                 |
| `overage_status`          | `RateLimitStatus \| None` | Statut de l'utilisation de dépassement à l'usage, si applicable                                                      |
| `overage_resets_at`       | `int \| None`             | Timestamp Unix quand la fenêtre de dépassement se réinitialise                                                       |
| `overage_disabled_reason` | `str \| None`             | Pourquoi le dépassement est indisponible, si le statut est `"rejected"`                                              |
| `raw`                     | `dict[str, Any]`          | Dict brut complet du CLI, incluant les champs non modélisés ci-dessus                                                |

<h3 id="taskstartedmessage">
  `TaskStartedMessage`
</h3>

Émis quand une tâche de fond démarre. Une tâche de fond est tout ce qui est suivi en dehors du tour principal : une commande Bash en arrière-plan, une montre [Monitor](#monitor), un sous-agent généré via l'outil Agent, ou un agent distant. Le champ `task_type` vous dit lequel. Ce nommage n'est pas lié au renommage de l'outil `Task`-à-`Agent`.

```python theme={null}
@dataclass
class TaskStartedMessage(SystemMessage):
    task_id: str
    description: str
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    task_type: str | None = None
```

| Champ         | Type          | Description                                                                                                                        |
| :------------ | :------------ | :--------------------------------------------------------------------------------------------------------------------------------- |
| `task_id`     | `str`         | Identifiant unique pour la tâche                                                                                                   |
| `description` | `str`         | Description de la tâche                                                                                                            |
| `uuid`        | `str`         | Identifiant de message unique                                                                                                      |
| `session_id`  | `str`         | Identifiant de session                                                                                                             |
| `tool_use_id` | `str \| None` | ID d'utilisation d'outil associé                                                                                                   |
| `task_type`   | `str \| None` | Quel type de tâche de fond : `"local_bash"` pour Bash en arrière-plan et les montres Monitor, `"local_agent"`, ou `"remote_agent"` |

<h3 id="taskusage">
  `TaskUsage`
</h3>

Données de tokens et de timing pour une tâche de fond.

```python theme={null}
class TaskUsage(TypedDict):
    total_tokens: int
    tool_uses: int
    duration_ms: int
```

<h3 id="taskprogressmessage">
  `TaskProgressMessage`
</h3>

Émis périodiquement avec les mises à jour de progression pour une tâche de fond en cours d'exécution.

```python theme={null}
@dataclass
class TaskProgressMessage(SystemMessage):
    task_id: str
    description: str
    usage: TaskUsage
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    last_tool_name: str | None = None
```

| Champ            | Type          | Description                                            |
| :--------------- | :------------ | :----------------------------------------------------- |
| `task_id`        | `str`         | Identifiant unique pour la tâche                       |
| `description`    | `str`         | Description du statut actuel                           |
| `usage`          | `TaskUsage`   | Utilisation de tokens pour cette tâche jusqu'à présent |
| `uuid`           | `str`         | Identifiant de message unique                          |
| `session_id`     | `str`         | Identifiant de session                                 |
| `tool_use_id`    | `str \| None` | ID d'utilisation d'outil associé                       |
| `last_tool_name` | `str \| None` | Nom du dernier outil utilisé par la tâche              |

<h3 id="tasknotificationmessage">
  `TaskNotificationMessage`
</h3>

Émis quand une tâche de fond se termine, échoue, ou est arrêtée. Les tâches de fond incluent les commandes Bash `run_in_background`, les montres Monitor, et les sous-agents en arrière-plan.

```python theme={null}
@dataclass
class TaskNotificationMessage(SystemMessage):
    task_id: str
    status: TaskNotificationStatus  # "completed" | "failed" | "stopped"
    output_file: str
    summary: str
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    usage: TaskUsage | None = None
```

| Champ         | Type                     | Description                                       |
| :------------ | :----------------------- | :------------------------------------------------ |
| `task_id`     | `str`                    | Identifiant unique pour la tâche                  |
| `status`      | `TaskNotificationStatus` | L'un de `"completed"`, `"failed"`, ou `"stopped"` |
| `output_file` | `str`                    | Chemin vers le fichier de sortie de la tâche      |
| `summary`     | `str`                    | Résumé du résultat de la tâche                    |
| `uuid`        | `str`                    | Identifiant de message unique                     |
| `session_id`  | `str`                    | Identifiant de session                            |
| `tool_use_id` | `str \| None`            | ID d'utilisation d'outil associé                  |
| `usage`       | `TaskUsage \| None`      | Utilisation de tokens finale pour la tâche        |

<h2 id="content-block-types">
  Types de blocs de contenu
</h2>

<h3 id="contentblock">
  `ContentBlock`
</h3>

Type union de tous les blocs de contenu.

```python theme={null}
ContentBlock = TextBlock | ThinkingBlock | ToolUseBlock | ToolResultBlock
```

<h3 id="textblock">
  `TextBlock`
</h3>

Bloc de contenu texte.

```python theme={null}
@dataclass
class TextBlock:
    text: str
```

<h3 id="thinkingblock">
  `ThinkingBlock`
</h3>

Bloc de contenu de réflexion (pour les modèles avec capacité de réflexion).

```python theme={null}
@dataclass
class ThinkingBlock:
    thinking: str
    signature: str
```

<h3 id="tooluseblock">
  `ToolUseBlock`
</h3>

Bloc de requête d'utilisation d'outil.

```python theme={null}
@dataclass
class ToolUseBlock:
    id: str
    name: str
    input: dict[str, Any]
```

<h3 id="toolresultblock">
  `ToolResultBlock`
</h3>

Bloc de résultat d'exécution d'outil.

```python theme={null}
@dataclass
class ToolResultBlock:
    tool_use_id: str
    content: str | list[dict[str, Any]] | None = None
    is_error: bool | None = None
```

<h2 id="error-types">
  Types d'erreur
</h2>

<h3 id="claudesdkerror">
  `ClaudeSDKError`
</h3>

Classe d'exception de base pour toutes les erreurs du SDK.

```python theme={null}
class ClaudeSDKError(Exception):
    """Base error for Claude SDK."""
```

<h3 id="clinotfounderror">
  `CLINotFoundError`
</h3>

Levée quand Claude Code CLI n'est pas installé ou introuvable.

```python theme={null}
class CLINotFoundError(CLIConnectionError):
    def __init__(
        self, message: str = "Claude Code not found", cli_path: str | None = None
    ):
        """
        Args:
            message: Error message (default: "Claude Code not found")
            cli_path: Optional path to the CLI that was not found
        """
```

<h3 id="cliconnectionerror">
  `CLIConnectionError`
</h3>

Levée quand la connexion à Claude Code échoue.

```python theme={null}
class CLIConnectionError(ClaudeSDKError):
    """Failed to connect to Claude Code."""
```

<h3 id="processerror">
  `ProcessError`
</h3>

Levée quand le processus Claude Code échoue.

```python theme={null}
class ProcessError(ClaudeSDKError):
    def __init__(
        self, message: str, exit_code: int | None = None, stderr: str | None = None
    ):
        self.exit_code = exit_code
        self.stderr = stderr
```

<h3 id="clijsondecodeerror">
  `CLIJSONDecodeError`
</h3>

Levée quand l'analyse JSON échoue.

```python theme={null}
class CLIJSONDecodeError(ClaudeSDKError):
    def __init__(self, line: str, original_error: Exception):
        """
        Args:
            line: The line that failed to parse
            original_error: The original JSON decode exception
        """
        self.line = line
        self.original_error = original_error
```

<h2 id="hook-types">
  Types de hooks
</h2>

Pour un guide complet sur l'utilisation des hooks avec des exemples et des modèles courants, voir le [guide Hooks](/docs/fr/agent-sdk/hooks).

<h3 id="hookevent">
  `HookEvent`
</h3>

Types d'événements de hook supportés.

```python theme={null}
HookEvent = Literal[
    "PreToolUse",  # Called before tool execution
    "PostToolUse",  # Called after tool execution
    "PostToolUseFailure",  # Called when a tool execution fails
    "UserPromptSubmit",  # Called when user submits a prompt
    "Stop",  # Called when stopping execution
    "SubagentStop",  # Called when a subagent stops
    "PreCompact",  # Called before message compaction
    "Notification",  # Called for notification events
    "SubagentStart",  # Called when a subagent starts
    "PermissionRequest",  # Called when a permission decision is needed
]
```

<Note>
  Le SDK TypeScript supporte des événements de hook supplémentaires non encore disponibles en Python : `SessionStart`, `SessionEnd`, `Setup`, `TeammateIdle`, `TaskCompleted`, `ConfigChange`, `WorktreeCreate`, `WorktreeRemove`, `PostToolBatch` et `MessageDisplay`.
</Note>

<h3 id="hookcallback">
  `HookCallback`
</h3>

Définition de type pour les fonctions de callback de hook.

```python theme={null}
HookCallback = Callable[[HookInput, str | None, HookContext], Awaitable[HookJSONOutput]]
```

Paramètres :

* `input` : Entrée de hook fortement typée avec unions discriminées basées sur `hook_event_name` (voir [`HookInput`](#hookinput))
* `tool_use_id` : Identifiant d'utilisation d'outil optionnel (pour les hooks liés aux outils)
* `context` : Contexte de hook avec des informations supplémentaires

Retourne un [`HookJSONOutput`](#hookjsonoutput) qui peut contenir :

* `decision` : `"block"` pour bloquer l'action
* `systemMessage` : Message d'avertissement affiché à l'utilisateur
* `hookSpecificOutput` : Données de sortie spécifiques au hook

<h3 id="hookcontext">
  `HookContext`
</h3>

Informations de contexte passées aux callbacks de hook.

```python theme={null}
class HookContext(TypedDict):
    signal: Any | None  # Future: abort signal support
```

<h3 id="hookmatcher">
  `HookMatcher`
</h3>

Configuration pour faire correspondre les hooks à des événements ou des outils spécifiques.

```python theme={null}
@dataclass
class HookMatcher:
    matcher: str | None = (
        None  # Tool name or pattern to match (e.g., "Bash", "Write|Edit")
    )
    hooks: list[HookCallback] = field(
        default_factory=list
    )  # List of callbacks to execute
    timeout: float | None = (
        None  # Timeout in seconds for all hooks in this matcher (default: 60)
    )
```

<h3 id="hookinput">
  `HookInput`
</h3>

Type union de tous les types d'entrée de hook. Le type réel dépend du champ `hook_event_name`.

```python theme={null}
HookInput = (
    PreToolUseHookInput
    | PostToolUseHookInput
    | PostToolUseFailureHookInput
    | UserPromptSubmitHookInput
    | StopHookInput
    | SubagentStopHookInput
    | PreCompactHookInput
    | NotificationHookInput
    | SubagentStartHookInput
    | PermissionRequestHookInput
)
```

<h3 id="basehookinput">
  `BaseHookInput`
</h3>

Champs de base présents dans tous les types d'entrée de hook.

```python theme={null}
class BaseHookInput(TypedDict):
    session_id: str
    transcript_path: str
    cwd: str
    permission_mode: NotRequired[str]
```

| Champ             | Type              | Description                                        |
| :---------------- | :---------------- | :------------------------------------------------- |
| `session_id`      | `str`             | Identifiant de session actuel                      |
| `transcript_path` | `str`             | Chemin vers le fichier de transcription de session |
| `cwd`             | `str`             | Répertoire de travail actuel                       |
| `permission_mode` | `str` (optionnel) | Mode de permission actuel                          |

<h3 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h3>

Données d'entrée pour les événements de hook `PreToolUse`.

```python theme={null}
class PreToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PreToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Champ             | Type                    | Description                                                                                 |
| :---------------- | :---------------------- | :------------------------------------------------------------------------------------------ |
| `hook_event_name` | `Literal["PreToolUse"]` | Toujours « PreToolUse »                                                                     |
| `tool_name`       | `str`                   | Nom de l'outil sur le point d'être exécuté                                                  |
| `tool_input`      | `dict[str, Any]`        | Paramètres d'entrée pour l'outil                                                            |
| `tool_use_id`     | `str`                   | Identifiant unique pour cette utilisation d'outil                                           |
| `agent_id`        | `str` (optionnel)       | Identifiant de sous-agent, présent quand le hook se déclenche à l'intérieur d'un sous-agent |
| `agent_type`      | `str` (optionnel)       | Type de sous-agent, présent quand le hook se déclenche à l'intérieur d'un sous-agent        |

<h3 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h3>

Données d'entrée pour les événements de hook `PostToolUse`.

```python theme={null}
class PostToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PostToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_response: Any
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Champ             | Type                     | Description                                                                                 |
| :---------------- | :----------------------- | :------------------------------------------------------------------------------------------ |
| `hook_event_name` | `Literal["PostToolUse"]` | Toujours « PostToolUse »                                                                    |
| `tool_name`       | `str`                    | Nom de l'outil qui a été exécuté                                                            |
| `tool_input`      | `dict[str, Any]`         | Paramètres d'entrée qui ont été utilisés                                                    |
| `tool_response`   | `Any`                    | Réponse de l'exécution de l'outil                                                           |
| `tool_use_id`     | `str`                    | Identifiant unique pour cette utilisation d'outil                                           |
| `agent_id`        | `str` (optionnel)        | Identifiant de sous-agent, présent quand le hook se déclenche à l'intérieur d'un sous-agent |
| `agent_type`      | `str` (optionnel)        | Type de sous-agent, présent quand le hook se déclenche à l'intérieur d'un sous-agent        |

<h3 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h3>

Données d'entrée pour les événements de hook `PostToolUseFailure`. Appelé quand l'exécution d'un outil échoue.

```python theme={null}
class PostToolUseFailureHookInput(BaseHookInput):
    hook_event_name: Literal["PostToolUseFailure"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    error: str
    is_interrupt: NotRequired[bool]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Champ             | Type                            | Description                                                                                 |
| :---------------- | :------------------------------ | :------------------------------------------------------------------------------------------ |
| `hook_event_name` | `Literal["PostToolUseFailure"]` | Toujours « PostToolUseFailure »                                                             |
| `tool_name`       | `str`                           | Nom de l'outil qui a échoué                                                                 |
| `tool_input`      | `dict[str, Any]`                | Paramètres d'entrée qui ont été utilisés                                                    |
| `tool_use_id`     | `str`                           | Identifiant unique pour cette utilisation d'outil                                           |
| `error`           | `str`                           | Message d'erreur de l'exécution échouée                                                     |
| `is_interrupt`    | `bool` (optionnel)              | Si l'échec a été causé par une interruption                                                 |
| `agent_id`        | `str` (optionnel)               | Identifiant de sous-agent, présent quand le hook se déclenche à l'intérieur d'un sous-agent |
| `agent_type`      | `str` (optionnel)               | Type de sous-agent, présent quand le hook se déclenche à l'intérieur d'un sous-agent        |

<h3 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h3>

Données d'entrée pour les événements de hook `UserPromptSubmit`.

```python theme={null}
class UserPromptSubmitHookInput(BaseHookInput):
    hook_event_name: Literal["UserPromptSubmit"]
    prompt: str
```

| Champ             | Type                          | Description                        |
| :---------------- | :---------------------------- | :--------------------------------- |
| `hook_event_name` | `Literal["UserPromptSubmit"]` | Toujours « UserPromptSubmit »      |
| `prompt`          | `str`                         | Le prompt soumis par l'utilisateur |

<h3 id="stophookinput">
  `StopHookInput`
</h3>

Données d'entrée pour les événements de hook `Stop`.

```python theme={null}
class StopHookInput(BaseHookInput):
    hook_event_name: Literal["Stop"]
    stop_hook_active: bool
```

| Champ              | Type              | Description                  |
| :----------------- | :---------------- | :--------------------------- |
| `hook_event_name`  | `Literal["Stop"]` | Toujours « Stop »            |
| `stop_hook_active` | `bool`            | Si le hook d'arrêt est actif |

<h3 id="subagentstophookinput">
  `SubagentStopHookInput`
</h3>

Données d'entrée pour les événements de hook `SubagentStop`.

```python theme={null}
class SubagentStopHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStop"]
    stop_hook_active: bool
    agent_id: str
    agent_transcript_path: str
    agent_type: str
```

| Champ                   | Type                      | Description                                           |
| :---------------------- | :------------------------ | :---------------------------------------------------- |
| `hook_event_name`       | `Literal["SubagentStop"]` | Toujours « SubagentStop »                             |
| `stop_hook_active`      | `bool`                    | Si le hook d'arrêt est actif                          |
| `agent_id`              | `str`                     | Identifiant unique pour le sous-agent                 |
| `agent_transcript_path` | `str`                     | Chemin vers le fichier de transcription du sous-agent |
| `agent_type`            | `str`                     | Type du sous-agent                                    |

<h3 id="precompacthookinput">
  `PreCompactHookInput`
</h3>

Données d'entrée pour les événements de hook `PreCompact`.

```python theme={null}
class PreCompactHookInput(BaseHookInput):
    hook_event_name: Literal["PreCompact"]
    trigger: Literal["manual", "auto"]
    custom_instructions: str | None
```

| Champ                 | Type                        | Description                                    |
| :-------------------- | :-------------------------- | :--------------------------------------------- |
| `hook_event_name`     | `Literal["PreCompact"]`     | Toujours « PreCompact »                        |
| `trigger`             | `Literal["manual", "auto"]` | Ce qui a déclenché la compaction               |
| `custom_instructions` | `str \| None`               | Instructions personnalisées pour la compaction |

<h3 id="notificationhookinput">
  `NotificationHookInput`
</h3>

Données d'entrée pour les événements de hook `Notification`.

```python theme={null}
class NotificationHookInput(BaseHookInput):
    hook_event_name: Literal["Notification"]
    message: str
    title: NotRequired[str]
    notification_type: str
```

| Champ               | Type                      | Description                        |
| :------------------ | :------------------------ | :--------------------------------- |
| `hook_event_name`   | `Literal["Notification"]` | Toujours « Notification »          |
| `message`           | `str`                     | Contenu du message de notification |
| `title`             | `str` (optionnel)         | Titre de la notification           |
| `notification_type` | `str`                     | Type de notification               |

<h3 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h3>

Données d'entrée pour les événements de hook `SubagentStart`.

```python theme={null}
class SubagentStartHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStart"]
    agent_id: str
    agent_type: str
```

| Champ             | Type                       | Description                           |
| :---------------- | :------------------------- | :------------------------------------ |
| `hook_event_name` | `Literal["SubagentStart"]` | Toujours « SubagentStart »            |
| `agent_id`        | `str`                      | Identifiant unique pour le sous-agent |
| `agent_type`      | `str`                      | Type du sous-agent                    |

<h3 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h3>

Données d'entrée pour les événements de hook `PermissionRequest`. Permet aux hooks de gérer les décisions de permission programmatiquement.

```python theme={null}
class PermissionRequestHookInput(BaseHookInput):
    hook_event_name: Literal["PermissionRequest"]
    tool_name: str
    tool_input: dict[str, Any]
    permission_suggestions: NotRequired[list[Any]]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Champ                    | Type                           | Description                                                                                 |
| :----------------------- | :----------------------------- | :------------------------------------------------------------------------------------------ |
| `hook_event_name`        | `Literal["PermissionRequest"]` | Toujours « PermissionRequest »                                                              |
| `tool_name`              | `str`                          | Nom de l'outil demandant la permission                                                      |
| `tool_input`             | `dict[str, Any]`               | Paramètres d'entrée pour l'outil                                                            |
| `permission_suggestions` | `list[Any]` (optionnel)        | Mises à jour de permission suggérées du CLI                                                 |
| `agent_id`               | `str` (optionnel)              | Identifiant de sous-agent, présent quand le hook se déclenche à l'intérieur d'un sous-agent |
| `agent_type`             | `str` (optionnel)              | Type de sous-agent, présent quand le hook se déclenche à l'intérieur d'un sous-agent        |

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

Type union pour les valeurs de retour de callback de hook.

```python theme={null}
HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

Sortie de hook synchrone avec champs de contrôle et de décision.

```python theme={null}
class SyncHookJSONOutput(TypedDict):
    # Control fields
    continue_: NotRequired[bool]  # Whether to proceed (default: True)
    suppressOutput: NotRequired[bool]  # Hide stdout from transcript
    stopReason: NotRequired[str]  # Message when continue is False

    # Decision fields
    decision: NotRequired[Literal["block"]]
    systemMessage: NotRequired[str]  # Warning message for user
    reason: NotRequired[str]  # Feedback for Claude

    # Hook-specific output
    hookSpecificOutput: NotRequired[HookSpecificOutput]
```

<Note>
  Utilisez `continue_` (avec trait de soulignement) dans le code Python. Il est automatiquement converti en `continue` quand envoyé au CLI.
</Note>

<h4 id="hookspecificoutput">
  `HookSpecificOutput`
</h4>

Un `TypedDict` contenant le nom d'événement de hook et les champs spécifiques à l'événement. La forme dépend de la valeur `hookEventName`. Pour les détails complets sur les champs disponibles par événement de hook, voir [Contrôler l'exécution avec les hooks](/docs/fr/agent-sdk/hooks#outputs).

Une union discriminée de types de sortie spécifiques à l'événement. Le champ `hookEventName` détermine quels champs sont valides.

```python theme={null}
class PreToolUseHookSpecificOutput(TypedDict):
    hookEventName: Literal["PreToolUse"]
    permissionDecision: NotRequired[Literal["allow", "deny", "ask", "defer"]]
    permissionDecisionReason: NotRequired[str]
    updatedInput: NotRequired[dict[str, Any]]
    additionalContext: NotRequired[str]


class PostToolUseHookSpecificOutput(TypedDict):
    hookEventName: Literal["PostToolUse"]
    additionalContext: NotRequired[str]
    updatedToolOutput: NotRequired[Any]
    updatedMCPToolOutput: NotRequired[Any]  # Deprecated: use updatedToolOutput, which works for all tools


class PostToolUseFailureHookSpecificOutput(TypedDict):
    hookEventName: Literal["PostToolUseFailure"]
    additionalContext: NotRequired[str]


class UserPromptSubmitHookSpecificOutput(TypedDict):
    hookEventName: Literal["UserPromptSubmit"]
    additionalContext: NotRequired[str]


class NotificationHookSpecificOutput(TypedDict):
    hookEventName: Literal["Notification"]
    additionalContext: NotRequired[str]


class SubagentStartHookSpecificOutput(TypedDict):
    hookEventName: Literal["SubagentStart"]
    additionalContext: NotRequired[str]


class PermissionRequestHookSpecificOutput(TypedDict):
    hookEventName: Literal["PermissionRequest"]
    decision: dict[str, Any]


HookSpecificOutput = (
    PreToolUseHookSpecificOutput
    | PostToolUseHookSpecificOutput
    | PostToolUseFailureHookSpecificOutput
    | UserPromptSubmitHookSpecificOutput
    | NotificationHookSpecificOutput
    | SubagentStartHookSpecificOutput
    | PermissionRequestHookSpecificOutput
)
```

<h4 id="asynchookjsonoutput">
  `AsyncHookJSONOutput`
</h4>

Sortie de hook asynchrone qui diffère l'exécution du hook.

```python theme={null}
class AsyncHookJSONOutput(TypedDict):
    async_: Literal[True]  # Set to True to defer execution
    asyncTimeout: NotRequired[int]  # Timeout in milliseconds
```

<Note>
  Utilisez `async_` (avec trait de soulignement) dans le code Python. Il est automatiquement converti en `async` quand envoyé au CLI.
</Note>

<h3 id="hook-usage-example">
  Exemple d'utilisation de hook
</h3>

Cet exemple enregistre deux hooks : l'un qui bloque les commandes bash dangereuses comme `rm -rf /`, et un autre qui enregistre toute l'utilisation d'outils pour l'audit. Le hook de sécurité s'exécute uniquement sur les commandes Bash (via le `matcher`), tandis que le hook de journalisation s'exécute sur tous les outils.

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher, HookContext
from typing import Any


async def validate_bash_command(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Validate and potentially block dangerous bash commands."""
    if input_data["tool_name"] == "Bash":
        command = input_data["tool_input"].get("command", "")
        if "rm -rf /" in command:
            return {
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": "Dangerous command blocked",
                }
            }
    return {}


async def log_tool_use(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log all tool usage for auditing."""
    print(f"Tool used: {input_data.get('tool_name')}")
    return {}


options = ClaudeAgentOptions(
    hooks={
        "PreToolUse": [
            HookMatcher(
                matcher="Bash", hooks=[validate_bash_command], timeout=120
            ),  # 2 min for validation
            HookMatcher(
                hooks=[log_tool_use]
            ),  # Applies to all tools (default 60s timeout)
        ],
        "PostToolUse": [HookMatcher(hooks=[log_tool_use])],
    }
)

async for message in query(prompt="Analyze this codebase", options=options):
    print(message)
```

<h2 id="tool-input/output-types">
  Types d'entrée/sortie d'outil
</h2>

Documentation des schémas d'entrée/sortie pour tous les outils Claude Code intégrés. Bien que le SDK Python n'exporte pas ceux-ci en tant que types, ils représentent la structure des entrées et sorties d'outils dans les messages.

<h3 id="agent">
  Agent
</h3>

**Nom de l'outil :** `Agent` (précédemment `Task`, qui est toujours accepté comme alias)

**Entrée :**

```python theme={null}
{
    "description": str,  # A short (3-5 word) description of the task
    "prompt": str,  # The task for the agent to perform
    "subagent_type": str,  # The type of specialized agent to use
}
```

**Sortie :**

```python theme={null}
{
    "result": str,  # Final result from the subagent
    "usage": dict | None,  # Token usage statistics
    "total_cost_usd": float | None,  # Estimated total cost in USD
    "duration_ms": int | None,  # Execution duration in milliseconds
}
```

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**Nom de l'outil :** `AskUserQuestion`

Pose des questions de clarification à l'utilisateur pendant l'exécution. Voir [Gérer les approbations et les entrées utilisateur](/docs/fr/agent-sdk/user-input#handle-clarifying-questions) pour les détails d'utilisation.

**Entrée :**

```python theme={null}
{
    "questions": [  # Questions to ask the user (1-4 questions)
        {
            "question": str,  # The complete question to ask the user
            "header": str,  # Very short label displayed as a chip/tag (max 12 chars)
            "options": [  # The available choices (2-4 options)
                {
                    "label": str,  # Display text for this option (1-5 words)
                    "description": str,  # Explanation of what this option means
                }
            ],
            "multiSelect": bool,  # Set to true to allow multiple selections
        }
    ],
    "answers": dict[str, str | list[str]] | None,
    # User answers populated by the permission system. Multi-select
    # answers may be a list of labels or a comma-joined string
}
```

**Sortie :**

```python theme={null}
{
    "questions": [  # The questions that were asked
        {
            "question": str,
            "header": str,
            "options": [{"label": str, "description": str}],
            "multiSelect": bool,
        }
    ],
    "answers": dict[str, str],  # Maps question text to answer string
    # Multi-select answers are comma-separated
}
```

<h3 id="bash">
  Bash
</h3>

**Nom de l'outil :** `Bash`

**Entrée :**

```python theme={null}
{
    "command": str,  # The command to execute
    "timeout": int | None,  # Optional timeout in milliseconds (max 600000; higher values are clamped to the max)
    "description": str | None,  # Clear, concise description (5-10 words)
    "run_in_background": bool | None,  # Set to true to run in background
}
```

**Sortie :**

```python theme={null}
{
    "output": str,  # Combined stdout and stderr output
    "exitCode": int,  # Exit code of the command
    "killed": bool | None,  # Whether command was killed due to timeout
    "shellId": str | None,  # Shell ID for background processes
}
```

<h3 id="monitor">
  Monitor
</h3>

**Nom de l'outil :** `Monitor`

Exécute une source de fond et livre chaque événement à Claude pour qu'il puisse réagir sans interrogation : `command` exécute un script et émet un événement par ligne stdout, et `ws` ouvre une WebSocket et émet un événement par trame texte. Fournissez exactement l'un de `command` ou `ws`.

Lorsque Monitor exécute une commande, il suit les mêmes règles de permission que Bash ; une surveillance WebSocket demande une approbation séparément. La source `ws` nécessite Claude Code v2.1.195 ou ultérieur. Voir la [référence de l'outil Monitor](/docs/fr/tools-reference#monitor-tool) pour le comportement et la disponibilité du fournisseur.

**Entrée :**

```python theme={null}
{
    "command": str | None,  # Shell script; each stdout line is an event, exit ends the watch
    "ws": dict | None,  # WebSocket source: {"url": str, "protocols": list[str] | None}; each text frame is an event
    "description": str,  # Short description shown in notifications
    "timeout_ms": int | None,  # Kill after this deadline (default 300000, max 3600000)
    "persistent": bool | None,  # Run for the lifetime of the session; stop with TaskStop
}
```

**Sortie :**

```python theme={null}
{
    "taskId": str,  # ID of the background monitor task
    "timeoutMs": int,  # Timeout deadline in milliseconds (0 when persistent)
    "persistent": bool | None,  # True when running until TaskStop or session end
}
```

<h3 id="edit">
  Edit
</h3>

**Nom de l'outil :** `Edit`

**Entrée :**

```python theme={null}
{
    "file_path": str,  # The absolute path to the file to modify
    "old_string": str,  # The text to replace
    "new_string": str,  # The text to replace it with
    "replace_all": bool | None,  # Replace all occurrences (default False)
}
```

**Sortie :**

```python theme={null}
{
    "message": str,  # Confirmation message
    "replacements": int,  # Number of replacements made
    "file_path": str,  # File path that was edited
}
```

<h3 id="read">
  Read
</h3>

**Nom de l'outil :** `Read`

**Entrée :**

```python theme={null}
{
    "file_path": str,  # The absolute path to the file to read
    "offset": int | None,  # The line number to start reading from
    "limit": int | None,  # The number of lines to read
}
```

**Sortie (fichiers texte) :**

```python theme={null}
{
    "content": str,  # File contents with line numbers
    "total_lines": int,  # Total number of lines in file
    "lines_returned": int,  # Lines actually returned
}
```

**Sortie (images) :**

```python theme={null}
{
    "image": str,  # Base64 encoded image data
    "mime_type": str,  # Image MIME type
    "file_size": int,  # File size in bytes
}
```

<h3 id="write">
  Write
</h3>

**Nom de l'outil :** `Write`

**Entrée :**

```python theme={null}
{
    "file_path": str,  # The absolute path to the file to write
    "content": str,  # The content to write to the file
}
```

**Sortie :**

```python theme={null}
{
    "message": str,  # Success message
    "bytes_written": int,  # Number of bytes written
    "file_path": str,  # File path that was written
}
```

<h3 id="glob">
  Glob
</h3>

**Nom de l'outil :** `Glob`

**Entrée :**

```python theme={null}
{
    "pattern": str,  # The glob pattern to match files against
    "path": str | None,  # The directory to search in (defaults to cwd)
}
```

**Sortie :**

```python theme={null}
{
    "matches": list[str],  # Array of matching file paths
    "count": int,  # Number of matches found
    "search_path": str,  # Search directory used
}
```

<h3 id="grep">
  Grep
</h3>

**Nom de l'outil :** `Grep`

**Entrée :**

```python theme={null}
{
    "pattern": str,  # The regular expression pattern
    "path": str | None,  # File or directory to search in
    "glob": str | None,  # Glob pattern to filter files
    "type": str | None,  # File type to search
    "output_mode": str | None,  # "content", "files_with_matches", or "count"
    "-i": bool | None,  # Case insensitive search
    "-n": bool | None,  # Show line numbers
    "-B": int | None,  # Lines to show before each match
    "-A": int | None,  # Lines to show after each match
    "-C": int | None,  # Lines to show before and after
    "head_limit": int | None,  # Limit output to first N lines/entries
    "multiline": bool | None,  # Enable multiline mode
}
```

**Sortie (mode contenu) :**

```python theme={null}
{
    "matches": [
        {
            "file": str,
            "line_number": int | None,
            "line": str,
            "before_context": list[str] | None,
            "after_context": list[str] | None,
        }
    ],
    "total_matches": int,
}
```

**Sortie (mode fichiers\_avec\_correspondances) :**

```python theme={null}
{
    "files": list[str],  # Files containing matches
    "count": int,  # Number of files with matches
}
```

<h3 id="notebookedit">
  NotebookEdit
</h3>

**Nom de l'outil :** `NotebookEdit`

**Entrée :**

```python theme={null}
{
    "notebook_path": str,  # Absolute path to the Jupyter notebook
    "cell_id": str | None,  # The ID of the cell to edit
    "new_source": str,  # The new source for the cell
    "cell_type": "code" | "markdown" | None,  # The type of the cell
    "edit_mode": "replace" | "insert" | "delete" | None,  # Edit operation type
}
```

**Sortie :**

```python theme={null}
{
    "message": str,  # Success message
    "edit_type": "replaced" | "inserted" | "deleted",  # Type of edit performed
    "cell_id": str | None,  # Cell ID that was affected
    "total_cells": int,  # Total cells in notebook after edit
}
```

<h3 id="webfetch">
  WebFetch
</h3>

**Nom de l'outil :** `WebFetch`

**Entrée :**

```python theme={null}
{
    "url": str,  # The URL to fetch content from
    "prompt": str,  # The prompt to run on the fetched content
}
```

**Sortie :**

```python theme={null}
{
    "bytes": int,  # Size of the fetched content in bytes
    "code": int,  # HTTP response code
    "codeText": str,  # HTTP response code text
    "result": str,  # Processed result from applying the prompt to the content
    "durationMs": int,  # Time to fetch and process the content, in milliseconds
    "url": str,  # URL that was fetched
}
```

<h3 id="websearch">
  WebSearch
</h3>

**Nom de l'outil :** `WebSearch`

**Entrée :**

```python theme={null}
{
    "query": str,  # The search query to use
    "allowed_domains": list[str] | None,  # Only include results from these domains
    "blocked_domains": list[str] | None,  # Never include results from these domains
}
```

**Sortie :**

```python theme={null}
{
    "query": str,  # The search query
    "results": list[str | {"tool_use_id": str, "content": list[{"title": str, "url": str}]}],
    "durationSeconds": float,  # Search duration in seconds
}
```

<h3 id="todowrite">
  TodoWrite
</h3>

**Nom de l'outil :** `TodoWrite`

<Note>
  À partir de Claude Code v2.1.142, `TodoWrite` est désactivé par défaut. Utilisez `TaskCreate`, `TaskGet`, `TaskUpdate` et `TaskList` à la place. Voir [Migrer vers les outils Task](/docs/fr/agent-sdk/todo-tracking#migrate-to-task-tools) pour mettre à jour votre code de surveillance, ou définissez `CLAUDE_CODE_ENABLE_TASKS=0` pour revenir à `TodoWrite`.
</Note>

**Entrée :**

```python theme={null}
{
    "todos": [
        {
            "content": str,  # The task description
            "status": "pending" | "in_progress" | "completed",  # Task status
            "activeForm": str,  # Active form of the description
        }
    ]
}
```

**Sortie :**

```python theme={null}
{
    "message": str,  # Success message
    "stats": {"total": int, "pending": int, "in_progress": int, "completed": int},
}
```

<h3 id="taskcreate">
  TaskCreate
</h3>

**Nom de l'outil :** `TaskCreate`

**Entrée :**

```python theme={null}
{
    "subject": str,  # Short task title
    "description": str,  # Detailed task body
    "activeForm": str | None,  # Present-tense label shown while in progress
    "metadata": dict | None,  # Arbitrary caller metadata
}
```

**Sortie :**

```python theme={null}
{
    "task": {"id": str, "subject": str},  # Created task with assigned ID
}
```

<h3 id="taskupdate">
  TaskUpdate
</h3>

**Nom de l'outil :** `TaskUpdate`

**Entrée :**

```python theme={null}
{
    "taskId": str,  # ID of the task to patch
    "status": Literal["pending", "in_progress", "completed", "deleted"] | None,
    "subject": str | None,
    "description": str | None,
    "activeForm": str | None,
    "addBlocks": list[str] | None,  # Task IDs this task now blocks
    "addBlockedBy": list[str] | None,  # Task IDs that now block this task
    "owner": str | None,
    "metadata": dict | None,
}
```

**Sortie :**

```python theme={null}
{
    "success": bool,
    "taskId": str,
    "updatedFields": list[str],  # Names of fields that changed
    "error": str | None,
    "statusChange": {"from": str, "to": str} | None,
}
```

<h3 id="taskget">
  TaskGet
</h3>

**Nom de l'outil :** `TaskGet`

**Entrée :**

```python theme={null}
{
    "taskId": str,  # ID of the task to read
}
```

**Sortie :**

```python theme={null}
{
    "task": {
        "id": str,
        "subject": str,
        "description": str,
        "status": Literal["pending", "in_progress", "completed"],
        "blocks": list[str],
        "blockedBy": list[str],
    } | None,  # None when the ID is not found
}
```

<h3 id="tasklist">
  TaskList
</h3>

**Nom de l'outil :** `TaskList`

**Entrée :**

```python theme={null}
{}
```

**Sortie :**

```python theme={null}
{
    "tasks": [
        {
            "id": str,
            "subject": str,
            "status": Literal["pending", "in_progress", "completed"],
            "owner": str | None,
            "blockedBy": list[str],
        }
    ],
}
```

<h3 id="bashoutput">
  BashOutput
</h3>

**Nom de l'outil :** `BashOutput`

**Entrée :**

```python theme={null}
{
    "bash_id": str,  # The ID of the background shell
    "filter": str | None,  # Optional regex to filter output lines
}
```

**Sortie :**

```python theme={null}
{
    "output": str,  # New output since last check
    "status": "running" | "completed" | "failed",  # Current shell status
    "exitCode": int | None,  # Exit code when completed
}
```

<h3 id="killbash">
  KillBash
</h3>

**Nom de l'outil :** `KillBash`

**Entrée :**

```python theme={null}
{
    "shell_id": str  # The ID of the background shell to kill
}
```

**Sortie :**

```python theme={null}
{
    "message": str,  # Success message
    "shell_id": str,  # ID of the killed shell
}
```

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Nom de l'outil :** `ExitPlanMode`

**Entrée :**

```python theme={null}
{
    "plan": str  # The plan to run by the user for approval
}
```

**Sortie :**

```python theme={null}
{
    "message": str,  # Confirmation message
    "approved": bool | None,  # Whether user approved the plan
}
```

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Nom de l'outil :** `ListMcpResourcesTool`

**Entrée :**

```python theme={null}
{
    "server": str | None  # Optional server name to filter resources by
}
```

**Sortie :**

```python theme={null}
{
    "resources": [
        {
            "uri": str,
            "name": str,
            "description": str | None,
            "mimeType": str | None,
            "server": str,
        }
    ],
    "total": int,
}
```

<h3 id="readmcpresource">
  ReadMcpResource
</h3>

**Nom de l'outil :** `ReadMcpResourceTool`

**Entrée :**

```python theme={null}
{
    "server": str,  # The MCP server name
    "uri": str,  # The resource URI to read
}
```

**Sortie :**

```python theme={null}
{
    "contents": [
        {"uri": str, "mimeType": str | None, "text": str | None, "blob": str | None}
    ],
    "server": str,
}
```

<h2 id="advanced-features-with-claudesdkclient">
  Fonctionnalités avancées avec ClaudeSDKClient
</h2>

<h3 id="building-a-continuous-conversation-interface">
  Construire une interface de conversation continue
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    TextBlock,
)
import asyncio


class ConversationSession:
    """Maintains a single conversation session with Claude."""

    def __init__(self, options: ClaudeAgentOptions | None = None):
        self.client = ClaudeSDKClient(options)
        self.turn_count = 0

    async def start(self):
        await self.client.connect()
        print("Starting conversation session. Claude will remember context.")
        print(
            "Commands: 'exit' to quit, 'interrupt' to stop current task, 'new' for new session"
        )

        while True:
            user_input = input(f"\n[Turn {self.turn_count + 1}] You: ")

            if user_input.lower() == "exit":
                break
            elif user_input.lower() == "interrupt":
                await self.client.interrupt()
                print("Task interrupted!")
                continue
            elif user_input.lower() == "new":
                # Disconnect and reconnect for a fresh session
                await self.client.disconnect()
                await self.client.connect()
                self.turn_count = 0
                print("Started new conversation session (previous context cleared)")
                continue

            # Send message - the session retains all previous messages
            await self.client.query(user_input)
            self.turn_count += 1

            # Process response
            print(f"[Turn {self.turn_count}] Claude: ", end="")
            async for message in self.client.receive_response():
                if isinstance(message, AssistantMessage):
                    for block in message.content:
                        if isinstance(block, TextBlock):
                            print(block.text, end="")
            print()  # New line after response

        await self.client.disconnect()
        print(f"Conversation ended after {self.turn_count} turns.")


async def main():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Bash"], permission_mode="acceptEdits"
    )
    session = ConversationSession(options)
    await session.start()


# Example conversation:
# Turn 1 - You: "Create a file called hello.py"
# Turn 1 - Claude: "I'll create a hello.py file for you..."
# Turn 2 - You: "What's in that file?"
# Turn 2 - Claude: "The hello.py file I just created contains..." (remembers!)
# Turn 3 - You: "Add a main function to it"
# Turn 3 - Claude: "I'll add a main function to hello.py..." (knows which file!)

asyncio.run(main())
```

<h3 id="using-hooks-for-behavior-modification">
  Utiliser les hooks pour la modification du comportement
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    HookMatcher,
    HookContext,
)
import asyncio
from typing import Any


async def pre_tool_logger(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log all tool usage before execution."""
    tool_name = input_data.get("tool_name", "unknown")
    print(f"[PRE-TOOL] About to use: {tool_name}")

    # You can modify or block the tool execution here
    if tool_name == "Bash" and "rm -rf" in str(input_data.get("tool_input", {})):
        return {
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "deny",
                "permissionDecisionReason": "Dangerous command blocked",
            }
        }
    return {}


async def post_tool_logger(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log results after tool execution."""
    tool_name = input_data.get("tool_name", "unknown")
    print(f"[POST-TOOL] Completed: {tool_name}")
    return {}


async def user_prompt_modifier(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Add context to user prompts."""
    original_prompt = input_data.get("prompt", "")

    # Add a timestamp as additional context for Claude to see
    from datetime import datetime

    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    return {
        "hookSpecificOutput": {
            "hookEventName": "UserPromptSubmit",
            "additionalContext": f"[Submitted at {timestamp}] Original prompt: {original_prompt}",
        }
    }


async def main():
    options = ClaudeAgentOptions(
        hooks={
            "PreToolUse": [
                HookMatcher(hooks=[pre_tool_logger]),
                HookMatcher(matcher="Bash", hooks=[pre_tool_logger]),
            ],
            "PostToolUse": [HookMatcher(hooks=[post_tool_logger])],
            "UserPromptSubmit": [HookMatcher(hooks=[user_prompt_modifier])],
        },
        allowed_tools=["Read", "Write", "Bash"],
    )

    async with ClaudeSDKClient(options=options) as client:
        await client.query("List files in current directory")

        async for message in client.receive_response():
            # Hooks will automatically log tool usage
            pass


asyncio.run(main())
```

<h3 id="real-time-progress-monitoring">
  Surveillance de la progression en temps réel
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    ToolUseBlock,
    ToolResultBlock,
    TextBlock,
)
import asyncio


async def monitor_progress():
    options = ClaudeAgentOptions(
        allowed_tools=["Write", "Bash"], permission_mode="acceptEdits"
    )

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Create 5 Python files with different sorting algorithms")

        # Monitor progress in real-time
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, ToolUseBlock):
                        if block.name == "Write":
                            file_path = block.input.get("file_path", "")
                            print(f"Creating: {file_path}")
                    elif isinstance(block, ToolResultBlock):
                        print("Completed tool execution")
                    elif isinstance(block, TextBlock):
                        print(f"Claude says: {block.text[:100]}...")

        print("Task completed!")


asyncio.run(monitor_progress())
```

<h2 id="example-usage">
  Exemple d'utilisation
</h2>

<h3 id="basic-file-operations-using-query">
  Opérations de fichiers de base (utilisant query)
</h3>

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock
import asyncio


async def create_project():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Bash"],
        permission_mode="acceptEdits",
        cwd="/home/user/project",
    )

    async for message in query(
        prompt="Create a Python project structure with setup.py", options=options
    ):
        if isinstance(message, AssistantMessage):
            for block in message.content:
                if isinstance(block, ToolUseBlock):
                    print(f"Using tool: {block.name}")


asyncio.run(create_project())
```

<h3 id="error-handling">
  Gestion des erreurs
</h3>

```python theme={null}
from claude_agent_sdk import query, CLINotFoundError, ProcessError, CLIJSONDecodeError

try:
    async for message in query(prompt="Hello"):
        print(message)
except CLINotFoundError:
    print(
        "Claude Code CLI not found. Try reinstalling: pip install --force-reinstall claude-agent-sdk"
    )
except ProcessError as e:
    print(f"Process failed with exit code: {e.exit_code}")
except CLIJSONDecodeError as e:
    print(f"Failed to parse response: {e}")
```

<h3 id="streaming-mode-with-client">
  Mode streaming avec client
</h3>

```python theme={null}
from claude_agent_sdk import ClaudeSDKClient
import asyncio


async def interactive_session():
    async with ClaudeSDKClient() as client:
        # Send initial message
        await client.query("What's the weather like?")

        # Process responses
        async for msg in client.receive_response():
            print(msg)

        # Send follow-up
        await client.query("Tell me more about that")

        # Process follow-up response
        async for msg in client.receive_response():
            print(msg)


asyncio.run(interactive_session())
```

<h3 id="using-custom-tools-with-claudesdkclient">
  Utiliser les outils personnalisés avec ClaudeSDKClient
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    tool,
    create_sdk_mcp_server,
    AssistantMessage,
    TextBlock,
)
import asyncio
from typing import Any


# Define custom tools with @tool decorator
@tool("calculate", "Perform mathematical calculations", {"expression": str})
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    try:
        result = eval(args["expression"], {"__builtins__": {}})
        return {"content": [{"type": "text", "text": f"Result: {result}"}]}
    except Exception as e:
        return {
            "content": [{"type": "text", "text": f"Error: {str(e)}"}],
            "is_error": True,
        }


@tool("get_time", "Get current time", {})
async def get_time(args: dict[str, Any]) -> dict[str, Any]:
    from datetime import datetime

    current_time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    return {"content": [{"type": "text", "text": f"Current time: {current_time}"}]}


async def main():
    # Create SDK MCP server with custom tools
    my_server = create_sdk_mcp_server(
        name="utilities", version="1.0.0", tools=[calculate, get_time]
    )

    # Configure options with the server
    options = ClaudeAgentOptions(
        mcp_servers={"utils": my_server},
        allowed_tools=["mcp__utils__calculate", "mcp__utils__get_time"],
    )

    # Use ClaudeSDKClient for interactive tool usage
    async with ClaudeSDKClient(options=options) as client:
        await client.query("What's 123 * 456?")

        # Process calculation response
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Calculation: {block.text}")

        # Follow up with time query
        await client.query("What time is it now?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Time: {block.text}")


asyncio.run(main())
```

<h2 id="sandbox-configuration">
  Configuration du sandbox
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Configuration pour le comportement du sandbox. Utilisez ceci pour activer le sandboxing des commandes et configurer les restrictions réseau programmatiquement.

```python theme={null}
class SandboxSettings(TypedDict, total=False):
    enabled: bool
    autoAllowBashIfSandboxed: bool
    excludedCommands: list[str]
    allowUnsandboxedCommands: bool
    network: SandboxNetworkConfig
    ignoreViolations: SandboxIgnoreViolations
    enableWeakerNestedSandbox: bool
```

| Propriété                   | Type                                                  | Par défaut | Description                                                                                                                                                                                                                                                    |
| :-------------------------- | :---------------------------------------------------- | :--------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `bool`                                                | `False`    | Activez le mode sandbox pour l'exécution des commandes                                                                                                                                                                                                         |
| `autoAllowBashIfSandboxed`  | `bool`                                                | `True`     | Approuvez automatiquement les commandes bash quand le sandbox est activé                                                                                                                                                                                       |
| `excludedCommands`          | `list[str]`                                           | `[]`       | Commandes qui contournent toujours les restrictions du sandbox (par exemple, `["docker"]`). Celles-ci s'exécutent sans sandbox automatiquement sans implication du modèle                                                                                      |
| `allowUnsandboxedCommands`  | `bool`                                                | `True`     | Permettez au modèle de demander l'exécution de commandes en dehors du sandbox. Quand `True`, le modèle peut définir `dangerouslyDisableSandbox` dans l'entrée d'outil, qui revient au [système de permissions](#permissions-fallback-for-unsandboxed-commands) |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `None`     | Configuration de sandbox spécifique au réseau                                                                                                                                                                                                                  |
| `ignoreViolations`          | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `None`     | Configurez quelles violations de sandbox ignorer                                                                                                                                                                                                               |
| `enableWeakerNestedSandbox` | `bool`                                                | `False`    | Activez un sandbox imbriqué plus faible pour la compatibilité                                                                                                                                                                                                  |

<Note>
  Le sandbox dépend du support de la plateforme et, sur Linux, d'outils comme `bubblewrap` et `socat`. Par défaut, quand `enabled` est `True` mais que le sandbox ne peut pas démarrer, les commandes s'exécutent sans sandbox avec un avertissement sur stderr. Ce comportement par défaut diffère du SDK TypeScript, où `failIfUnavailable` est par défaut `true`.

  Définissez `"failIfUnavailable": True` dans vos paramètres de sandbox pour arrêter à la place. La clé n'est pas encore déclarée sur `SandboxSettings`, mais le SDK la transmet à Claude Code, qui la respecte. `query()` rapporte alors un `ResultMessage` avec `subtype="error_during_execution"` et la raison dans `errors`. Surveillez ce sous-type plutôt que de vous attendre à ce que `query()` lève une exception avant de produire des messages.
</Note>

<h4 id="example-usage-2">
  Exemple d'utilisation
</h4>

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, SandboxSettings

sandbox_settings: SandboxSettings = {
    "enabled": True,
    "autoAllowBashIfSandboxed": True,
    "network": {"allowLocalBinding": True},
}

async for message in query(
    prompt="Build and test my project",
    options=ClaudeAgentOptions(sandbox=sandbox_settings),
):
    print(message)
```

<Warning>
  **Sécurité des sockets Unix** : L'option `allowUnixSockets` peut accorder l'accès à des services système puissants. Par exemple, permettre `/var/run/docker.sock` accorde effectivement un accès complet au système hôte via l'API Docker, contournant l'isolation du sandbox. Autorisez uniquement les sockets Unix strictement nécessaires et comprenez les implications de sécurité de chacun.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Configuration spécifique au réseau pour le mode sandbox. Ces paramètres s'appliquent aux commandes Bash en sandbox quand `enabled` est `True` dans le parent [`SandboxSettings`](#sandboxsettings). Ils ne restreignent pas l'outil WebFetch, qui utilise à la place des [règles de permission](/docs/fr/permissions#webfetch).

```python theme={null}
class SandboxNetworkConfig(TypedDict, total=False):
    allowedDomains: list[str]
    deniedDomains: list[str]
    allowManagedDomainsOnly: bool
    allowUnixSockets: list[str]
    allowAllUnixSockets: bool
    allowLocalBinding: bool
    allowMachLookup: list[str]
    httpProxyPort: int
    socksProxyPort: int
```

| Propriété                 | Type        | Par défaut | Description                                                                                                                                                                           |
| :------------------------ | :---------- | :--------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `allowedDomains`          | `list[str]` | `[]`       | Noms de domaine auxquels les processus en sandbox peuvent accéder                                                                                                                     |
| `deniedDomains`           | `list[str]` | `[]`       | Noms de domaine auxquels les processus en sandbox ne peuvent pas accéder. Prend la priorité sur `allowedDomains`                                                                      |
| `allowManagedDomainsOnly` | `bool`      | `False`    | Paramètres gérés uniquement : quand défini dans les paramètres gérés, ignorez `allowedDomains` des sources de paramètres non gérées. N'a aucun effet quand défini via les options SDK |
| `allowUnixSockets`        | `list[str]` | `[]`       | Chemins de socket Unix auxquels les processus peuvent accéder (par exemple, socket Docker)                                                                                            |
| `allowAllUnixSockets`     | `bool`      | `False`    | Permettez l'accès à tous les sockets Unix                                                                                                                                             |
| `allowLocalBinding`       | `bool`      | `False`    | Permettez aux processus de se lier aux ports locaux (par exemple, pour les serveurs de développement)                                                                                 |
| `allowMachLookup`         | `list[str]` | `[]`       | macOS uniquement : noms de services XPC/Mach à autoriser. Supporte un caractère générique à la fin                                                                                    |
| `httpProxyPort`           | `int`       | `None`     | Port du proxy HTTP pour les requêtes réseau                                                                                                                                           |
| `socksProxyPort`          | `int`       | `None`     | Port du proxy SOCKS pour les requêtes réseau                                                                                                                                          |

<Note>
  Le proxy sandbox intégré applique la liste blanche réseau en fonction du nom d'hôte demandé et ne termine pas ou n'inspecte pas le trafic TLS, donc des techniques telles que le [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) peuvent potentiellement le contourner. Voir [Limitations de sécurité du sandboxing](/docs/fr/sandboxing#security-limitations) pour les détails et [Déploiement sécurisé](/docs/fr/agent-sdk/secure-deployment#traffic-forwarding) pour configurer un proxy qui termine TLS.
</Note>

<h3 id="sandboxignoreviolations">
  `SandboxIgnoreViolations`
</h3>

Configuration pour ignorer les violations de sandbox spécifiques.

```python theme={null}
class SandboxIgnoreViolations(TypedDict, total=False):
    file: list[str]
    network: list[str]
```

| Propriété | Type        | Par défaut | Description                                              |
| :-------- | :---------- | :--------- | :------------------------------------------------------- |
| `file`    | `list[str]` | `[]`       | Modèles de chemin de fichier pour ignorer les violations |
| `network` | `list[str]` | `[]`       | Modèles réseau pour ignorer les violations               |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Fallback de permissions pour les commandes sans sandbox
</h3>

Quand `allowUnsandboxedCommands` est activé, le modèle peut demander l'exécution de commandes en dehors du sandbox en définissant `dangerouslyDisableSandbox: True` dans l'entrée d'outil. Ces requêtes reviennent au système de permissions existant, ce qui signifie que votre gestionnaire `can_use_tool` sera invoqué, vous permettant d'implémenter une logique d'autorisation personnalisée.

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands` :**

  * `excludedCommands` : Une liste statique de commandes qui contournent toujours le sandbox automatiquement (par exemple, `["docker"]`). Le modèle n'a aucun contrôle sur ceci.
  * `allowUnsandboxedCommands` : Permet au modèle de décider à l'exécution s'il faut demander l'exécution sans sandbox en définissant `dangerouslyDisableSandbox: True` dans l'entrée d'outil.
</Note>

```python theme={null}
from claude_agent_sdk import (
    query,
    ClaudeAgentOptions,
    HookMatcher,
    PermissionResultAllow,
    PermissionResultDeny,
    ToolPermissionContext,
)


async def can_use_tool(
    tool: str, input: dict, context: ToolPermissionContext
) -> PermissionResultAllow | PermissionResultDeny:
    # Check if the model is requesting to bypass the sandbox
    if tool == "Bash" and input.get("dangerouslyDisableSandbox"):
        # The model is requesting to run this command outside the sandbox
        print(f"Unsandboxed command requested: {input.get('command')}")

        if is_command_authorized(input.get("command")):
            return PermissionResultAllow()
        return PermissionResultDeny(
            message="Command not authorized for unsandboxed execution"
        )
    return PermissionResultAllow()


# Required: dummy hook keeps the stream open for can_use_tool
async def dummy_hook(input_data, tool_use_id, context):
    return {"continue_": True}


async def prompt_stream():
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Deploy my application"},
    }


async def main():
    async for message in query(
        prompt=prompt_stream(),
        options=ClaudeAgentOptions(
            sandbox={
                "enabled": True,
                "allowUnsandboxedCommands": True,  # Model can request unsandboxed execution
            },
            permission_mode="default",
            can_use_tool=can_use_tool,
            hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
        ),
    ):
        print(message)
```

Ce modèle vous permet de :

* **Auditer les requêtes du modèle** : Enregistrez quand le modèle demande l'exécution sans sandbox
* **Implémenter des listes blanches** : Autorisez uniquement des commandes spécifiques à s'exécuter sans sandbox
* **Ajouter des flux d'approbation** : Nécessitez une autorisation explicite pour les opérations privilégiées

<Warning>
  Les commandes s'exécutant avec `dangerouslyDisableSandbox: True` ont un accès complet au système. Assurez-vous que votre gestionnaire `can_use_tool` valide ces requêtes avec soin.

  Si `permission_mode` est défini sur `bypassPermissions` et `allow_unsandboxed_commands` est activé, le modèle peut exécuter de manière autonome des commandes en dehors du sandbox sans aucun prompt d'approbation. Cette combinaison permet effectivement au modèle d'échapper à l'isolation du sandbox silencieusement.
</Warning>

<h2 id="see-also">
  Voir aussi
</h2>

* [Vue d'ensemble du SDK](/docs/fr/agent-sdk/overview) - Concepts généraux du SDK
* [Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript) - Documentation du SDK TypeScript
* [Référence CLI](/docs/fr/cli-reference) - Interface de ligne de commande
* [Flux de travail courants](/docs/fr/common-workflows) - Guides étape par étape
