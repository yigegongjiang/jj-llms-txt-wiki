> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Modification des invites système

> Choisissez entre le préréglage `claude_code` et une invite système personnalisée, et personnalisez le comportement avec CLAUDE.md, les styles de sortie, append, ou une invite entièrement personnalisée.

Les invites système définissent le comportement, les capacités et le style de réponse de Claude. Commencez par le préréglage `claude_code` pour les outils de codage de type CLI ou IDE où un humain observe et dirige le travail. Écrivez votre propre invite pour les agents ayant une surface, une identité ou un modèle de permissions différents.

<h2 id="how-system-prompts-work">
  Fonctionnement des invites système
</h2>

Une invite système est l'ensemble initial d'instructions qui façonne le comportement de Claude tout au long d'une conversation. Le SDK Agent dispose de trois points de départ pour celle-ci :

* **Défaut minimal** : lorsque vous ne définissez pas `systemPrompt` en TypeScript ou `system_prompt` en Python, le SDK utilise une invite minimale qui couvre l'appel d'outils mais omet le reste du contenu du préréglage `claude_code`, y compris ses instructions de sécurité et de sûreté ainsi que son contexte concernant le répertoire de travail et l'environnement. Cela diffère de `claude -p`, qui utilise l'invite système Claude Code par défaut. Si vous migrez depuis la CLI et souhaitez un comportement correspondant, définissez le préréglage `claude_code`.
* **Préréglage `claude_code`** : l'invite système que la CLI Claude Code utilise, avec les instructions d'utilisation des outils, les instructions de sécurité et de sûreté, et le contexte concernant le répertoire de travail et l'environnement. Définissez `systemPrompt: { type: "preset", preset: "claude_code" }` en TypeScript ou `system_prompt={"type": "preset", "preset": "claude_code"}` en Python, éventuellement avec `append` pour ajouter vos propres instructions à la fin.
* **Chaîne personnalisée** : une invite que vous écrivez vous-même. Le SDK envoie uniquement ce que vous fournissez.

<h3 id="decide-on-a-starting-point">
  Décider d'un point de départ
</h3>

Le facteur décisif est la proximité de votre agent avec Claude Code : un agent de codage opérant dans un référentiel, avec un humain regardant la sortie en continu et dirigeant le travail. Plus votre produit s'éloigne de cela, plus vous voudrez écrire votre propre invite.

| Vous construisez                                                                                                                       | Utiliser                               | Ce que vous obtenez                                                                                                                                                |
| :------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Un outil de codage de type CLI ou IDE où un humain regarde et dirige, et les valeurs par défaut de Claude Code sont ce que vous voulez | Préréglage `claude_code`               | L'invite Claude Code, y compris la guidance des outils, les règles de sécurité et le contexte de l'environnement                                                   |
| Le même type d'outil, plus des règles spécifiques au produit comme les normes de codage, le format de sortie ou le contexte du domaine | Préréglage `claude_code` avec `append` | Tout ce qui précède, avec vos instructions ajoutées après le préréglage. Rien n'est supprimé, donc c'est la personnalisation à risque le plus faible               |
| Un agent avec une surface, une identité ou un modèle de permission différent, ou un agent non-codage                                   | Chaîne d'invite personnalisée          | Uniquement ce que vous écrivez. Vous êtes responsable du remplacement de la guidance des outils et des instructions de sécurité dont votre agent a toujours besoin |
| Une boucle d'appel d'outils mince sans persona d'agent, où vous fournissez tout le comportement dans l'invite utilisateur              | Pas d'option `systemPrompt`            | Le défaut minimal : support d'appel d'outils et rien d'autre                                                                                                       |

« Différent de Claude Code » signifie généralement l'un des éléments suivants :

* **Surface différente** : la sortie n'est pas lue dans un terminal par la personne qui l'a déclenchée. Les interfaces de chat, les consommateurs de sortie structurée et l'automatisation non-codage ont chacun besoin d'une invite qui correspond à la façon dont leur sortie est rendue et examinée. L'automatisation de codage sans surveillance, comme un travail CI qui corrige les erreurs de lint ou examine les diffs, s'adapte toujours au préréglage car le travail lui-même est ce pour lequel le préréglage est écrit.
* **Identité différente** : l'agent ne devrait pas se présenter comme Claude Code. Un bot d'assistance, un assistant d'analyse de données ou tout agent spécifique à un domaine a besoin de son propre nom, portée et persona.
* **Modèle de permission différent** : l'agent s'exécute de manière autonome sans qu'un humain n'approuve chaque étape, ou opère sur un ensemble étroit de ressources. L'invite de Claude Code suppose qu'un humain est dans la boucle avec accès à un ensemble complet d'outils.
* **Tâches non-codage** : la plupart de l'invite de Claude Code est une guidance de codage. Pour les agents de recherche, de contenu ou d'opérations, cette guidance entre en concurrence avec les instructions dont vous avez réellement besoin.

Le [tableau de comparaison](#compare-the-four-approaches) montre ce que chaque méthode de personnalisation préserve.

<h2 id="customize-agent-behavior">
  Personnaliser le comportement de l'agent
</h2>

`append` et une chaîne de prompt personnalisée modifient chacun directement le prompt système, et un style de sortie change les instructions que Claude Code donne à Claude pour chaque réponse. CLAUDE.md emprunte un chemin différent : le SDK le lit et injecte son contenu dans la conversation en tant que contexte de projet, donc il façonne le comportement aux côtés de n'importe quel prompt système que vous choisissez. [Skills](/docs/fr/agent-sdk/skills), [hooks](/docs/fr/agent-sdk/hooks), et [permissions](/docs/fr/agent-sdk/permissions) façonnent également le comportement en dehors du prompt système et sont couverts sur leurs propres pages.

<h3 id="claude-md-files-for-project-level-instructions">
  Fichiers CLAUDE.md pour les instructions au niveau du projet
</h3>

Les fichiers CLAUDE.md donnent à Claude un contexte de projet persistant et des instructions. Le SDK injecte leur contenu dans la conversation et laisse le prompt système intact, donc ils fonctionnent avec n'importe quelle configuration de prompt système. Pour savoir quoi mettre dans CLAUDE.md, où le placer, et comment écrire des instructions efficaces, consultez [When to add to CLAUDE.md](/docs/fr/memory#when-to-add-to-claude-md) et le reste de [How Claude remembers your project](/docs/fr/memory). Cette section couvre ce qui est spécifique au SDK : comment CLAUDE.md se charge.

Le SDK lit CLAUDE.md quand la source de paramètre correspondante est activée : `'project'` charge `CLAUDE.md` ou `.claude/CLAUDE.md` du répertoire de travail, et `'user'` charge `~/.claude/CLAUDE.md`. Les options `query()` par défaut activent les deux sources, donc CLAUDE.md se charge automatiquement. Si vous définissez `settingSources` en TypeScript ou `setting_sources` en Python explicitement, incluez les sources dont vous avez besoin. Le chargement de CLAUDE.md est contrôlé par les sources de paramètres, pas par le préréglage `claude_code`.

<h4 id="load-claude-md-with-the-sdk">
  Charger CLAUDE.md avec le SDK
</h4>

Pour charger CLAUDE.md, définissez `settingSources` pour inclure le niveau où vous gardez votre CLAUDE.md. L'exemple ci-dessous charge un CLAUDE.md au niveau du projet aux côtés du préréglage `claude_code`, donc Claude a à la fois le prompt de l'agent de codage et les conventions de votre projet :

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Add a new React component for user profiles",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code" // Use Claude Code's system prompt
      },
      settingSources: ["project"] // Loads CLAUDE.md from project
    }
  })) {
    messages.push(message);
  }

  // Now Claude has access to your project guidelines from CLAUDE.md
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  messages = []


  async def main():
      async for message in query(
          prompt="Add a new React component for user profiles",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",  # Use Claude Code's system prompt
              },
              setting_sources=["project"],  # Loads CLAUDE.md from project
          ),
      ):
          messages.append(message)


  asyncio.run(main())

  # Now Claude has access to your project guidelines from CLAUDE.md
  ```
</CodeGroup>

Quand vous exécutez l'un ou l'autre exemple, le SDK diffuse les messages en continu tandis que Claude travaille : un message d'initialisation système, des messages d'assistant, des messages utilisateur portant les résultats des outils, et un message de résultat final avec le résultat de la session.

CLAUDE.md est persistant dans toutes les sessions d'un projet, partagé avec votre équipe via git, et découvert automatiquement sans modifications de code. Il n'est pas chargé si vous passez un tableau `settingSources` vide.

<h3 id="output-styles-for-persistent-configurations">
  Styles de sortie pour les configurations persistantes
</h3>

Les styles de sortie sont des configurations enregistrées d'instructions qui modifient le rôle, le ton et le format de sortie de Claude. Ils sont stockés sous forme de fichiers markdown et peuvent être réutilisés dans les sessions et les projets.

<h4 id="create-an-output-style">
  Créer un style de sortie
</h4>

Un style de sortie est un fichier markdown avec [frontmatter](/docs/fr/output-styles#frontmatter) pour les métadonnées, suivi du contenu du prompt. Enregistrez-le dans `~/.claude/output-styles/` pour un style au niveau utilisateur disponible dans chaque projet, ou `.claude/output-styles/` dans votre référentiel pour un style au niveau du projet que vous pouvez valider et partager avec votre équipe.

Un style de sortie personnalisé laisse les instructions d'ingénierie logicielle du préréglage `claude_code` de côté et utilise les vôtres. Pour les conserver et superposer vos instructions par-dessus, définissez `keep-coding-instructions: true` dans le frontmatter. Ces instructions ne sont que dans le prompt système complet de Claude Code, donc le paramètre n'a aucun effet dans une session sur le prompt système plus court, que vous activez ou désactivez avec [`CLAUDE_CODE_SIMPLE_SYSTEM_PROMPT`](/docs/fr/env-vars#variables). Conservez-les quand votre agent fait toujours du travail d'ingénierie logicielle. Laissez-les de côté quand vous remplacez entièrement le rôle.

L'exemple ci-dessous définit une persona d'examen de code qui conserve les instructions de codage, puisque l'examen du code bénéficie toujours des conseils de sécurité et de qualité du code de Claude Code. Enregistrez-le sous `~/.claude/output-styles/code-reviewer.md` pour le rendre disponible dans tous les projets :

```markdown ~/.claude/output-styles/code-reviewer.md theme={null}
---
name: Code Reviewer
description: Thorough code review assistant
keep-coding-instructions: true
---

You are an expert code reviewer.

For every code submission:
1. Check for bugs and security issues
2. Evaluate performance
3. Suggest improvements
4. Rate code quality (1-10)
```

<h4 id="activate-an-output-style">
  Activer un style de sortie
</h4>

Une fois créé, activez les styles de sortie via :

* **CLI** : exécutez `/output-style <style>`, par exemple `/output-style concise`, ou exécutez `/config` et sélectionnez un style. La commande `/output-style` nécessite Claude Code v2.1.269 ou ultérieur.
* **Paramètres** : définissez `outputStyle` dans `.claude/settings.local.json`
* **TypeScript SDK** : définissez `outputStyle` à l'intérieur de l'objet `settings` en ligne passé à `query()`, ou pointez `settings` vers un fichier de paramètres qui le définit. `outputStyle` n'est pas un champ `Options` de niveau supérieur :

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

Dans le SDK Python, définissez `outputStyle` via l'option `settings`, qui prend une chaîne JSON telle que `'{"outputStyle": "Explanatory"}'` ou un chemin vers un fichier de paramètres qui le définit.

**Remarque pour les utilisateurs du SDK :** Les styles de sortie sont chargés quand vous incluez `settingSources: ['user']` ou `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` ou `setting_sources=["project"]` (Python) dans vos options.

<h3 id="append-to-the-claude_code-preset">
  Ajouter au préréglage `claude_code`
</h3>

Vous pouvez utiliser le préréglage Claude Code avec une propriété `append` pour ajouter vos instructions personnalisées tout en préservant toutes les fonctionnalités intégrées.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Help me write a Python function to calculate fibonacci numbers",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "Always include detailed docstrings and type hints in Python code."
      }
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  messages = []


  async def main():
      async for message in query(
          prompt="Help me write a Python function to calculate fibonacci numbers",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",
                  "append": "Always include detailed docstrings and type hints in Python code.",
              }
          ),
      ):
          messages.append(message)
          if isinstance(message, AssistantMessage):
              print(message.content)


  asyncio.run(main())
  ```
</CodeGroup>

<h4 id="improve-prompt-caching-across-users-and-machines">
  Améliorer la mise en cache des prompts entre les utilisateurs et les machines
</h4>

Par défaut, deux sessions qui utilisent le même préréglage `claude_code` et le même texte `append` ne peuvent toujours pas partager une entrée de cache de prompt si elles s'exécutent à partir de répertoires de travail différents. C'est parce que le préréglage intègre le contexte par session dans le prompt système avant votre texte `append` : le répertoire de travail, s'il s'agit d'un référentiel git, la plateforme, le shell actif, la version du système d'exploitation, et les chemins de mémoire automatique. Toute différence dans ce contexte produit un prompt système différent et un échec du cache. Le contenu de CLAUDE.md n'affecte pas le cache du prompt système parce que le SDK l'injecte dans la conversation, pas dans le prompt système.

Pour rendre le prompt système identique dans les sessions, définissez `excludeDynamicSections: true` en TypeScript ou `"exclude_dynamic_sections": True` en Python. Le contexte par session se déplace dans le premier message utilisateur, laissant seulement le préréglage statique et votre texte `append` dans le prompt système afin que les configurations identiques partagent une entrée de cache dans les utilisateurs et les machines.

<Note>
  `excludeDynamicSections` nécessite `@anthropic-ai/claude-agent-sdk` v0.2.98 ou ultérieur, ou `claude-agent-sdk` v0.1.58 ou ultérieur pour Python. Définissez-le sur la forme d'objet préréglé uniquement. Le SDK l'ignore quand vous passez un prompt personnalisé au lieu du préréglage ; pour garder les instructions d'un prompt personnalisé en cache dans le SDK TypeScript, consultez [Cache the static part of a custom prompt](#cache-the-static-part-of-a-custom-prompt).
</Note>

L'exemple suivant associe un bloc `append` partagé avec `excludeDynamicSections` afin qu'une flotte d'agents s'exécutant à partir de répertoires différents puisse réutiliser le même prompt système en cache :

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Triage the open issues in this repo",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "You operate Acme's internal triage workflow. Label issues by component and severity.",
        excludeDynamicSections: true
      }
    }
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      async for message in query(
          prompt="Triage the open issues in this repo",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",
                  "append": "You operate Acme's internal triage workflow. Label issues by component and severity.",
                  "exclude_dynamic_sections": True,
              },
          ),
      ):
          ...


  asyncio.run(main())
  ```
</CodeGroup>

**Compromis :** le répertoire de travail, l'indicateur de référentiel git, la plateforme, le shell actif, la version du système d'exploitation, et les chemins de mémoire automatique atteignent toujours Claude, mais comme faisant partie du premier message utilisateur plutôt que du prompt système. Les instructions dans le message utilisateur ont un poids légèrement inférieur au même texte dans le prompt système, donc Claude peut s'y fier moins fortement quand il raisonne sur le répertoire courant ou les chemins de mémoire automatique. Activez cette option quand la réutilisation du cache entre sessions est plus importante que le contexte d'environnement maximalement autoritaire.

Pour l'indicateur équivalent en mode CLI non interactif, consultez [`--exclude-dynamic-system-prompt-sections`](/docs/fr/cli-reference).

<h3 id="custom-system-prompts">
  Prompts système personnalisés
</h3>

Vous pouvez fournir une chaîne personnalisée en tant que `systemPrompt` pour remplacer entièrement la valeur par défaut par vos propres instructions.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const customPrompt = `You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices`;

  const messages = [];

  for await (const message of query({
    prompt: "Create a data processing pipeline",
    options: {
      systemPrompt: customPrompt
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  custom_prompt = """You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices"""

  messages = []


  async def main():
      async for message in query(
          prompt="Create a data processing pipeline",
          options=ClaudeAgentOptions(system_prompt=custom_prompt),
      ):
          messages.append(message)
          if isinstance(message, AssistantMessage):
              print(message.content)


  asyncio.run(main())
  ```
</CodeGroup>

En Python, chargez un grand prompt personnalisé à partir d'un fichier avec `system_prompt={"type": "file", "path": "..."}` au lieu de le passer en tant que chaîne. Le SDK Python passe un prompt de chaîne en tant qu'un argument de ligne de commande au sous-processus CLI, donc un prompt qui dépasse la limite de longueur d'argument du système d'exploitation échoue au lancement du processus avant toute demande d'API. Sur Linux, l'erreur est `Argument list too long`. Consultez [`SystemPromptFile`](/docs/fr/agent-sdk/python#systempromptfile) pour les seuils de plateforme et le comportement de Windows.

<h4 id="cache-the-static-part-of-a-custom-prompt">
  Mettre en cache la partie statique d'un prompt personnalisé
</h4>

Dans le SDK TypeScript, vous pouvez passer un prompt personnalisé en tant que tableau de chaînes au lieu d'une chaîne, avec le marqueur `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` entre la partie statique et le reste. Utilisez ceci quand votre prompt combine des instructions qui sont identiques à chaque demande avec un contexte qui change par demande, comme le client ou le ticket que l'agent traite. Quand vous passez les deux parties en tant qu'une chaîne, une modification de la partie par demande change le prompt système entier, donc les instructions statiques manquent également le cache. Cette forme n'est pas disponible dans le SDK Python ; [`ClaudeAgentOptions`](/docs/fr/agent-sdk/python#claudeagentoptions) énumère les formes que `system_prompt` accepte.

<Note>
  Claude Code divise le prompt seulement quand il appelle l'API Claude directement ou s'exécute sur [Claude Platform on AWS](/docs/fr/claude-platform-on-aws). Dans chaque autre configuration, comme Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, ou une [passerelle LLM](/docs/fr/llm-gateway-connect), il envoie le prompt entier en tant qu'un bloc, identique à passer une chaîne. La même chose se produit chaque fois que vous définissez [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/fr/llm-gateway-protocol#disable-pre-release-capabilities).
</Note>

Pour diviser le prompt, importez `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` depuis `@anthropic-ai/claude-agent-sdk` et passez-le en tant qu'élément de tableau propre entre les deux parties. Le SDK envoie les chaînes avant le marqueur en tant qu'un bloc de texte et les chaînes après en tant qu'un deuxième bloc, chacun avec son propre point de rupture de cache. Dans l'exemple ci-dessous, un agent d'assistance charge ses instructions de triage à partir d'un fichier et reçoit les détails d'un ticket à chaque demande, donc les instructions restent en cache tandis que les détails du ticket changent :

```typescript TypeScript theme={null}
import { readFile } from "node:fs/promises";
import { query, SYSTEM_PROMPT_DYNAMIC_BOUNDARY } from "@anthropic-ai/claude-agent-sdk";

// Identical on every request
const instructions = await readFile("triage-instructions.md", "utf8");
// Different on every request
const ticketContext = "Customer plan: Enterprise. Other open tickets from this customer: 3.";

for await (const message of query({
  prompt: "Triage ticket 4821",
  options: {
    systemPrompt: [instructions, SYSTEM_PROMPT_DYNAMIC_BOUNDARY, ticketContext]
  }
})) {
  // ...
}
```

[Track cache tokens](/docs/fr/agent-sdk/cost-tracking#track-cache-tokens) décrit les champs `cache_creation_input_tokens` et `cache_read_input_tokens` sur chaque message de résultat.

Le SDK assemble les blocs du tableau comme suit :

* Le SDK joint les chaînes de chaque côté du marqueur avec une ligne vierge entre elles et supprime le marqueur lui-même, donc le texte du marqueur n'atteint pas Claude.
* Si vous incluez le marqueur plus d'une fois, le premier est la division et le SDK supprime les autres.
* Si vous laissez le marqueur de côté, le SDK joint toutes les chaînes en un bloc, identique à passer une chaîne.

Avec les drapeaux [`--system-prompt` ou `--system-prompt-file`](/docs/fr/cli-reference#system-prompt-flags) de la CLI, le prompt est une chaîne, donc il n'y a pas de tableau pour porter le marqueur. Incluez une ligne contenant seulement `__SYSTEM_PROMPT_DYNAMIC_BOUNDARY__` entre les parties statiques et par demande à la place. Claude Code divise le prompt à la première telle ligne en deux blocs identiques et supprime cette ligne. Nécessite Claude Code v2.1.275 ou ultérieur.

Dans le SDK, préférez la forme de tableau, qui porte la limite sans ligne de marqueur.

<h3 id="change-the-prompt-of-an-existing-session">
  Modifier le prompt d'une session existante
</h3>

Par défaut, si vous passez un `append` ou un prompt personnalisé différent quand vous revenez à une session avec `resume` ou `continue`, Claude ne le voit pas au tour suivant. Claude Code enregistre le prompt système à la première demande d'une session et réutilise cet enregistrement jusqu'à ce que la session soit compactée. Le nouveau texte prend effet après ce compactage, ou dans une nouvelle session.

<h4 id="update-claude’s-instructions-mid-session">
  Mettre à jour les instructions de Claude en cours de session
</h4>

Si les instructions que vous mettez dans le prompt système doivent changer pendant qu'une session s'exécute, par exemple parce que votre utilisateur a basculé l'agent en mode lecture seule ou a modifié sa configuration dans votre application, envoyez les nouvelles instructions dans la conversation au lieu de modifier `systemPrompt` :

* **Dans votre message suivant** : incluez les nouvelles instructions dans le prochain message utilisateur que vous envoyez.
* **À partir d'un hook** : retournez [`additionalContext`](/docs/fr/hooks#add-context-for-claude) à partir d'un callback de hook `UserPromptSubmit` ou `PostToolUse` [hook callback](/docs/fr/agent-sdk/hooks#outputs), écrit comme une déclaration factuelle telle que « L'espace de travail est maintenant en lecture seule ». Le SDK insère le texte dans la conversation au point où le hook s'est déclenché, donc le prompt enregistré reste inchangé.

<h4 id="turn-recording-off-while-you-iterate-on-wording">
  Désactiver l'enregistrement pendant que vous itérez sur la formulation
</h4>

Pendant que vous itérez sur la formulation du prompt et que vous voulez que chaque modification atteigne une session que vous reprenez, définissez `snapshot` à false sur la forme d'objet du prompt système. Claude Code reconstruit alors le prompt à chaque demande. Le champ est disponible sur les formes préréglée et personnalisée de [`systemPrompt`](/docs/fr/agent-sdk/typescript#options) en TypeScript et de [`system_prompt`](/docs/fr/agent-sdk/python#systempromptpreset) en Python, et nécessite `@anthropic-ai/claude-agent-sdk` v0.3.257 ou ultérieur, ou `claude-agent-sdk` v0.2.153 ou ultérieur.

Gardez l'enregistrement activé en production. Avec l'enregistrement désactivé, un `append` ou un prompt personnalisé différent sur une session reprise atteint Claude au tour suivant, et cette demande ne peut pas réutiliser le [cache de prompt](/docs/fr/prompt-caching#how-the-cache-is-organized) de la session. Là où l'API applique la [pensée préservée](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking), Claude perd également sa pensée des tours antérieurs.

En dehors des [sessions cloud](/docs/fr/cloud-environments), si vous démarrez Claude Code en [mode bare](/docs/fr/headless#start-faster-with-bare-mode) en passant `--bare` via `extraArgs` ou en définissant `CLAUDE_CODE_SIMPLE=1`, l'enregistrement reste désactivé sauf si vous définissez `snapshot: true`.

L'enregistrement d'un `append` ou d'un prompt personnalisé par défaut nécessite Claude Code v2.1.265 ou ultérieur, que le SDK Agent TypeScript regroupe à partir de v0.3.265 et le SDK Agent Python à partir de v0.2.153. Avant Claude Code v2.1.268, les sessions qui ne [récupèrent pas les drapeaux de fonctionnalité](/docs/fr/env-vars#features-that-need-feature-flag-fetching), y compris les sessions sur Amazon Bedrock, Google Cloud's Agent Platform, et Microsoft Foundry, reconstruisaient le prompt à chaque demande et `snapshot` n'avait aucun effet.

<h2 id="compare-the-four-approaches">
  Comparaison des quatre approches
</h2>

Les quatre méthodes de personnalisation diffèrent par leur emplacement, la façon dont elles sont partagées et ce qu'elles préservent de la présélection `claude_code`.

| Fonctionnalité                 | CLAUDE.md                  | Styles de sortie                            | `systemPrompt` avec append | `systemPrompt` personnalisé     |
| ------------------------------ | -------------------------- | ------------------------------------------- | -------------------------- | ------------------------------- |
| **Persistance**                | Fichier par projet         | Enregistré sous forme de fichiers           | Session uniquement         | Session uniquement              |
| **Réutilisabilité**            | Par projet                 | Entre les projets                           | Duplication de code        | Duplication de code             |
| **Gestion**                    | Sur le système de fichiers | CLI + fichiers                              | Dans le code               | Dans le code                    |
| **Outils par défaut**          | Préservés                  | Préservés                                   | Préservés                  | Perdus (sauf s'ils sont inclus) |
| **Sécurité intégrée**          | Maintenue                  | Maintenue                                   | Maintenue                  | Doit être ajoutée               |
| **Contexte d'environnement**   | Automatique                | Automatique                                 | Automatique                | Doit être fourni                |
| **Niveau de personnalisation** | Ajouts uniquement          | Remplacer la valeur par défaut ou l'étendre | Ajouts uniquement          | Contrôle complet                |
| **Contrôle de version**        | Avec le projet             | Oui                                         | Avec le code               | Avec le code                    |
| **Portée**                     | Spécifique au projet       | Utilisateur ou projet                       | Session de code            | Session de code                 |

« Avec append » signifie utiliser `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` en TypeScript ou `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}` en Python. CLAUDE.md ne modifie pas le message système lui-même : le SDK injecte son contenu dans la conversation en tant que contexte du projet.

<h2 id="combine-approaches">
  Combiner les approches
</h2>

Les approches se composent. Un style de sortie persistant ou CLAUDE.md définit le comportement à long terme, et `append` superpose les instructions spécifiques à la session sans modifier la configuration enregistrée.

<h3 id="combine-an-output-style-with-session-specific-additions">
  Combiner un style de sortie avec des ajouts spécifiques à la session
</h3>

L'exemple ci-dessous suppose qu'un style de sortie Code Reviewer est déjà actif. Le bloc `append` superpose les domaines de focus spécifiques à la session sur la persona, de sorte qu'une seule session de révision peut prioriser OAuth et le stockage des tokens sans modifier le style de sortie enregistré :

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Assuming "Code Reviewer" output style is active (via /config or settings)
  // Add session-specific focus areas
  const messages = [];

  for await (const message of query({
    prompt: "Review this authentication module",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: `
          For this review, prioritize:
          - OAuth 2.0 compliance
          - Token storage security
          - Session management
        `
      }
    }
  })) {
    messages.push(message);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  # Assuming "Code Reviewer" output style is active (via /config or settings)
  # Add session-specific focus areas
  messages = []


  async def main():
      async for message in query(
          prompt="Review this authentication module",
          options=ClaudeAgentOptions(
              system_prompt={
                  "type": "preset",
                  "preset": "claude_code",
                  "append": """
                  For this review, prioritize:
                  - OAuth 2.0 compliance
                  - Token storage security
                  - Session management
                  """,
              }
          ),
      ):
          messages.append(message)


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="see-also">
  Voir aussi
</h2>

* [Styles de sortie](/docs/fr/output-styles) : créer, gérer et partager les styles de sortie pour la CLI, y compris le format de fichier et les emplacements de stockage
* [Comment Claude se souvient de votre projet](/docs/fr/memory) : ce qu'il faut mettre dans CLAUDE.md, où le placer et comment rédiger des instructions de projet efficaces
* [Référence du SDK TypeScript](/docs/fr/agent-sdk/typescript) : le type `Options` complet, y compris `systemPrompt`, `settingSources` et `settings`
* [Référence du SDK Python](/docs/fr/agent-sdk/python) : le type `ClaudeAgentOptions` complet, y compris `system_prompt` et `setting_sources`
* [Paramètres](/docs/fr/settings) : la référence `settings.json`, y compris où les styles de sortie et autres configurations sont stockés
