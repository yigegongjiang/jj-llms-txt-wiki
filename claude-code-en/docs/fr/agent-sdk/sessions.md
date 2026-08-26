> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Travailler avec les sessions

> Comment les sessions conservent l'historique des conversations de l'agent, et quand utiliser continue, resume et fork pour revenir à une exécution antérieure.

Une session est l'historique des conversations que le SDK accumule pendant que votre agent travaille. Elle contient votre prompt, chaque appel d'outil que l'agent a effectué, chaque résultat d'outil et chaque réponse. Le SDK l'écrit automatiquement sur le disque pour que vous puissiez y revenir plus tard.

Revenir à une session signifie que l'agent a le contexte complet d'avant : les fichiers qu'il a déjà lus, l'analyse qu'il a déjà effectuée, les décisions qu'il a déjà prises. Vous pouvez poser une question de suivi, récupérer après une interruption ou vous brancher pour essayer une approche différente.

<Note>
  Les sessions conservent la **conversation**, pas le système de fichiers. Pour créer un instantané et annuler les modifications de fichiers que l'agent a apportées, utilisez [file checkpointing](/docs/fr/agent-sdk/file-checkpointing).
</Note>

Ce guide couvre comment choisir la bonne approche pour votre application, les interfaces du SDK qui suivent automatiquement les sessions, comment capturer les ID de session et utiliser `resume` et `fork` manuellement, et ce qu'il faut savoir sur la reprise des sessions sur plusieurs hôtes.

<h2 id="choose-an-approach">
  Choisir une approche
</h2>

La quantité de gestion de session dont vous avez besoin dépend de la forme de votre application. La gestion des sessions entre en jeu lorsque vous envoyez plusieurs prompts qui doivent partager le contexte. Dans un seul appel `query()`, l'agent prend déjà autant de tours qu'il en a besoin, et les prompts de permission et `AskUserQuestion` sont [gérés en boucle](/docs/fr/agent-sdk/user-input) (ils ne terminent pas l'appel).

| Ce que vous construisez                                                      | Ce qu'il faut utiliser                                                                                                                                                               |
| :--------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tâche unique : prompt unique, pas de suivi                                   | Rien d'extra. Un seul appel `query()` le gère.                                                                                                                                       |
| Chat multi-tours dans un seul processus                                      | [`ClaudeSDKClient` (Python) ou `continue: true` (TypeScript)](#automatic-session-management). Le SDK suit la session pour vous sans gestion d'ID.                                    |
| Reprendre là où vous vous êtes arrêté après un redémarrage de processus      | `continue_conversation=True` (Python) / `continue: true` (TypeScript). Reprend la session la plus récente du répertoire, aucun ID nécessaire.                                        |
| Reprendre une session passée spécifique (pas la plus récente)                | Capturez l'ID de session et passez-le à `resume`.                                                                                                                                    |
| Essayer une approche alternative sans perdre l'original                      | Bifurquez la session.                                                                                                                                                                |
| Tâche sans état, ne voulez rien écrire sur le disque (TypeScript uniquement) | Définissez [`persistSession: false`](/docs/fr/agent-sdk/typescript#options). La session existe uniquement en mémoire pendant la durée de l'appel. Python persiste toujours sur le disque. |

<h3 id="continue-resume-and-fork">
  Continue, resume et fork
</h3>

Continue, resume et fork sont des champs d'options que vous définissez sur `query()` ([`ClaudeAgentOptions`](/docs/fr/agent-sdk/python#claudeagentoptions) en Python, [`Options`](/docs/fr/agent-sdk/typescript#options) en TypeScript).

**Continue** et **resume** reprennent tous les deux une session existante et l'ajoutent. La différence est la façon dont ils trouvent cette session :

* **Continue** trouve la session la plus récente dans le répertoire courant. Vous ne suivez rien. Fonctionne bien lorsque votre application exécute une conversation à la fois.
* **Resume** prend un ID de session spécifique. Vous suivez l'ID. Requis lorsque vous avez plusieurs sessions (par exemple, une par utilisateur dans une application multi-utilisateurs) ou que vous voulez revenir à une qui n'est pas la plus récente.

**Fork** est différent : il crée une nouvelle session qui commence par une copie de l'historique de l'original. L'original reste inchangé. Utilisez fork pour essayer une direction différente tout en gardant la possibilité de revenir en arrière.

<h2 id="automatic-session-management">
  Gestion automatique des sessions
</h2>

Les deux SDK offrent une interface qui suit l'état de la session pour vous entre les appels, vous n'avez donc pas besoin de passer les ID manuellement. Utilisez-les pour les conversations multi-tours dans un seul processus.

<h3 id="python-claudesdkclient">
  Python : `ClaudeSDKClient`
</h3>

[`ClaudeSDKClient`](/docs/fr/agent-sdk/python#claudesdkclient) gère les ID de session en interne. Chaque appel à `client.query()` continue automatiquement la même session. Appelez [`client.receive_response()`](/docs/fr/agent-sdk/python#claudesdkclient) pour itérer sur les messages de la requête actuelle. Utilisez le client comme gestionnaire de contexte asynchrone afin que la configuration et l'arrêt de la connexion soient gérés pour vous, ou appelez `connect()` et `disconnect()` manuellement.

Cet exemple exécute deux requêtes contre le même `client`. La première demande à l'agent d'analyser un module ; la seconde lui demande de refactoriser ce module. Parce que les deux appels passent par la même instance de client, la deuxième requête a le contexte complet de la première sans aucun `resume` ou ID de session explicite :

```python Python theme={null}
import asyncio
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    ResultMessage,
    TextBlock,
)


def print_response(message):
    """Print only the human-readable parts of a message."""
    if isinstance(message, AssistantMessage):
        for block in message.content:
            if isinstance(block, TextBlock):
                print(block.text)
    elif isinstance(message, ResultMessage):
        cost = (
            f"${message.total_cost_usd:.4f}"
            if message.total_cost_usd is not None
            else "N/A"
        )
        print(f"[done: {message.subtype}, cost: {cost}]")


async def main():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Edit", "Glob", "Grep"],
    )

    async with ClaudeSDKClient(options=options) as client:
        # First query: client captures the session ID internally
        await client.query("Analyze the auth module")
        async for message in client.receive_response():
            print_response(message)

        # Second query: automatically continues the same session
        await client.query("Now refactor it to use JWT")
        async for message in client.receive_response():
            print_response(message)


asyncio.run(main())
```

Consultez la [référence du SDK Python](/docs/fr/agent-sdk/python#choosing-between-query-and-claudesdkclient) pour plus de détails sur quand utiliser `ClaudeSDKClient` par rapport à la fonction `query()` autonome.

<h3 id="typescript-continue-true">
  TypeScript : `continue: true`
</h3>

Le SDK TypeScript n'a pas d'objet client tenant une session comme le `ClaudeSDKClient` de Python. À la place, passez `continue: true` sur chaque appel `query()` suivant et le SDK reprend la session la plus récente dans le répertoire courant. Aucun suivi d'ID requis.

Cet exemple effectue deux appels `query()` séparés. Le premier crée une session nouvelle ; le second définit `continue: true`, ce qui indique au SDK de trouver et reprendre la session la plus récente sur le disque. L'agent a le contexte complet du premier appel :

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// First query: creates a new session
for await (const message of query({
  prompt: "Analyze the auth module",
  options: { allowedTools: ["Read", "Glob", "Grep"] }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}

// Second query: continue: true resumes the most recent session
for await (const message of query({
  prompt: "Now refactor it to use JWT",
  options: {
    continue: true,
    allowedTools: ["Read", "Edit", "Write", "Glob", "Grep"]
  }
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}
```

<Note>
  L'API de session V2 expérimentale [](/docs/fr/agent-sdk/typescript-v2-preview), qui fournissait `createSession()` avec un modèle `send` / `stream`, a été supprimée dans TypeScript Agent SDK 0.3.142. Utilisez la fonction `query()` et les options de session décrites sur cette page à la place.
</Note>

<h2 id="use-session-options-with-query">
  Utiliser les options de session avec `query()`
</h2>

<h3 id="capture-the-session-id">
  Capturer l'ID de session
</h3>

Resume et fork nécessitent un ID de session. Lisez-le à partir du champ `session_id` sur le message de résultat ([`ResultMessage`](/docs/fr/agent-sdk/python#resultmessage) en Python, [`SDKResultMessage`](/docs/fr/agent-sdk/typescript#sdkresultmessage) en TypeScript), qui est présent sur chaque résultat indépendamment du succès ou de l'erreur. En TypeScript, l'ID est également disponible plus tôt en tant que champ direct sur le `SystemMessage` d'initialisation ; en Python, il est imbriqué dans `SystemMessage.data`.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      session_id = None

      async for message in query(
          prompt="Analyze the auth module and suggest improvements",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep"],
          ),
      ):
          if isinstance(message, ResultMessage):
              session_id = message.session_id
              if message.subtype == "success":
                  print(message.result)

      print(f"Session ID: {session_id}")
      return session_id


  session_id = asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  let sessionId: string | undefined;

  for await (const message of query({
    prompt: "Analyze the auth module and suggest improvements",
    options: { allowedTools: ["Read", "Glob", "Grep"] }
  })) {
    if (message.type === "result") {
      sessionId = message.session_id;
      if (message.subtype === "success") {
        console.log(message.result);
      }
    }
  }

  console.log(`Session ID: ${sessionId}`);
  ```
</CodeGroup>

<h3 id="resume-by-id">
  Reprendre par ID
</h3>

Passez un ID de session à `resume` pour revenir à cette session spécifique. L'agent reprend avec le contexte complet d'où la session s'est arrêtée. Les raisons courantes de reprendre :

* **Faire un suivi sur une tâche terminée.** L'agent a déjà analysé quelque chose ; maintenant vous voulez qu'il agisse sur cette analyse sans relire les fichiers.
* **Récupérer d'une limite.** La première exécution s'est terminée avec `error_max_turns` ou `error_max_budget_usd` (voir [Gérer le résultat](/docs/fr/agent-sdk/agent-loop#handle-the-result)) ; reprenez avec une limite plus élevée.
* **Redémarrer votre processus.** Vous avez capturé l'ID avant l'arrêt et voulez restaurer la conversation.

Cet exemple reprend la session de [Capturer l'ID de session](#capture-the-session-id) avec un prompt de suivi. Parce que vous reprenez, l'agent a déjà l'analyse antérieure en contexte :

<CodeGroup>
  ```python Python theme={null}
  # Earlier session analyzed the code; now build on that analysis
  async for message in query(
      prompt="Now implement the refactoring you suggested",
      options=ClaudeAgentOptions(
          resume=session_id,
          allowed_tools=["Read", "Edit", "Write", "Glob", "Grep"],
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const sessionId = "..."; // The ID you captured in the previous example

  // Earlier session analyzed the code; now build on that analysis
  for await (const message of query({
    prompt: "Now implement the refactoring you suggested",
    options: {
      resume: sessionId,
      allowedTools: ["Read", "Edit", "Write", "Glob", "Grep"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

Vous devriez voir une réponse qui s'appuie sur l'analyse antérieure au lieu de recommencer à zéro. Cela confirme que l'agent a repris la session avec son contexte antérieur intact.

<Tip>
  Si un appel `resume` retourne une session nouvelle au lieu de l'historique attendu, la cause la plus courante est un `cwd` non correspondant. Les sessions sont stockées sous `~/.claude/projects/<encoded-cwd>/*.jsonl`, ou sous `$CLAUDE_CONFIG_DIR/projects/<encoded-cwd>/*.jsonl` si vous définissez la variable d'environnement `CLAUDE_CONFIG_DIR`, où `<encoded-cwd>` est le répertoire de travail absolu avec chaque caractère non alphanumérique remplacé par `-` (donc `/Users/me/proj` devient `-Users-me-proj`). Si votre appel resume s'exécute à partir d'un répertoire différent, le SDK cherche au mauvais endroit. Le fichier de session doit également exister sur la machine actuelle.
</Tip>

Pour reprendre les sessions sur plusieurs machines ou dans des environnements sans serveur, mettez en miroir les transcriptions vers un stockage partagé avec un adaptateur [`SessionStore`](/docs/fr/agent-sdk/session-storage).

<h3 id="fork-to-explore-alternatives">
  Bifurquer pour explorer les alternatives
</h3>

La bifurcation crée une nouvelle session qui commence par une copie de l'historique de l'original mais diverge à partir de ce point. La bifurcation obtient son propre ID de session ; l'ID et l'historique de l'original restent inchangés. Vous vous retrouvez avec deux sessions indépendantes que vous pouvez reprendre séparément.

<Note>
  La bifurcation branche l'historique de la conversation, pas le système de fichiers. Si un agent bifurqué modifie des fichiers, ces modifications sont réelles et visibles pour toute session travaillant dans le même répertoire. Pour brancher et annuler les modifications de fichiers, utilisez [file checkpointing](/docs/fr/agent-sdk/file-checkpointing).
</Note>

Cet exemple s'appuie sur [Capturer l'ID de session](#capture-the-session-id) : vous avez déjà analysé un module d'authentification dans `session_id` et voulez explorer OAuth2 sans perdre le fil axé sur JWT. Le premier bloc bifurque la session et capture l'ID de la bifurcation (`forked_id`) ; le deuxième bloc reprend le `session_id` original pour continuer sur le chemin JWT. Vous avez maintenant deux ID de session pointant vers deux historiques séparés :

<CodeGroup>
  ```python Python theme={null}
  # Fork: branch from session_id into a new session
  forked_id = None
  async for message in query(
      prompt="Instead of JWT, outline how OAuth2 would work for the auth module",
      options=ClaudeAgentOptions(
          resume=session_id,
          fork_session=True,
          max_turns=5,
      ),
  ):
      if isinstance(message, ResultMessage):
          forked_id = message.session_id  # The fork's ID, distinct from session_id
          if message.subtype == "success":
              print(message.result)

  print(f"Forked session: {forked_id}")

  # Original session is untouched; resuming it continues the JWT thread
  async for message in query(
      prompt="Continue with the JWT approach",
      options=ClaudeAgentOptions(resume=session_id),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const sessionId = "..."; // The ID you captured in the previous example

  // Fork: branch from sessionId into a new session
  let forkedId: string | undefined;

  for await (const message of query({
    prompt: "Instead of JWT, outline how OAuth2 would work for the auth module",
    options: {
      resume: sessionId,
      forkSession: true,
      maxTurns: 5
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      forkedId = message.session_id; // The fork's ID, distinct from sessionId
    }
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }

  console.log(`Forked session: ${forkedId}`);

  // Original session is untouched; resuming it continues the JWT thread
  for await (const message of query({
    prompt: "Continue with the JWT approach",
    options: { resume: sessionId }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

Vous devriez voir que `forkedId` diffère de l'ID de session original. Reprendre la session originale continue toujours le fil JWT, ce qui confirme que la bifurcation n'a pas modifié l'historique original.

<h2 id="resume-across-hosts">
  Reprendre sur plusieurs hôtes
</h2>

Les fichiers de session sont locaux à la machine qui les a créés. Pour reprendre une session sur un hôte différent (travailleurs CI, conteneurs éphémères, sans serveur), vous avez deux options :

* **Déplacer le fichier de session.** Persistez `~/.claude/projects/<encoded-cwd>/<session-id>.jsonl` de la première exécution et restaurez-le au même chemin sur le nouvel hôte avant d'appeler `resume`. Le `cwd` doit correspondre.
* **Ne pas compter sur la reprise de session.** Capturez les résultats dont vous avez besoin (sortie d'analyse, décisions, diffs de fichiers) en tant qu'état d'application et passez-les dans le prompt d'une session nouvelle. C'est souvent plus robuste que d'expédier des fichiers de transcription.

Les deux SDK exposent des fonctions pour énumérer les sessions sur le disque et lire leurs messages : [`listSessions()`](/docs/fr/agent-sdk/typescript#listsessions) et [`getSessionMessages()`](/docs/fr/agent-sdk/typescript#getsessionmessages) en TypeScript, [`list_sessions()`](/docs/fr/agent-sdk/python#list_sessions) et [`get_session_messages()`](/docs/fr/agent-sdk/python#get_session_messages) en Python. Utilisez-les pour construire des sélecteurs de session personnalisés, une logique de nettoyage ou des visionneuses de transcription.

Les deux SDK exposent également des fonctions pour rechercher et muter des sessions individuelles : [`get_session_info()`](/docs/fr/agent-sdk/python#get_session_info), [`rename_session()`](/docs/fr/agent-sdk/python#rename_session) et [`tag_session()`](/docs/fr/agent-sdk/python#tag_session) en Python, et [`getSessionInfo()`](/docs/fr/agent-sdk/typescript#getsessioninfo), [`renameSession()`](/docs/fr/agent-sdk/typescript#renamesession) et [`tagSession()`](/docs/fr/agent-sdk/typescript#tagsession) en TypeScript. Utilisez-les pour organiser les sessions par tag ou leur donner des titres lisibles par l'homme.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [How the agent loop works](/docs/fr/agent-sdk/agent-loop) : Comprendre les tours, les messages et l'accumulation de contexte dans une session
* [File checkpointing](/docs/fr/agent-sdk/file-checkpointing) : Snapshot et annuler les modifications de fichiers que l'agent a effectuées au cours d'une session
* [Python `ClaudeAgentOptions`](/docs/fr/agent-sdk/python#claudeagentoptions) : Référence complète des options de session pour Python
* [TypeScript `Options`](/docs/fr/agent-sdk/typescript#options) : Référence complète des options de session pour TypeScript
