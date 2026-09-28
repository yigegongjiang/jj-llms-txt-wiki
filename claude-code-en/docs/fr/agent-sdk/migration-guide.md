> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Migrer vers Claude Agent SDK

> Guide pour migrer les SDK TypeScript et Python de Claude Code vers Claude Agent SDK

<h2 id="overview">
  Aperçu
</h2>

Le Claude Code SDK a été renommé en **Claude Agent SDK** et sa documentation a été réorganisée. Ce changement reflète les capacités plus larges du SDK pour construire des agents IA au-delà des simples tâches de codage.

Vous migrez depuis le SDK OpenAI Agents ? La [recette de migration du SDK OpenAI Agents](https://platform.claude.com/cookbook/claude-agent-sdk-04-migrating-from-openai-agents-sdk) mappe chaque primitive sur le Claude Agent SDK à travers un seul exemple travaillé.

<h2 id="what’s-changed">
  Ce qui a changé
</h2>

| Aspect                              | Ancien                      | Nouveau                                                                        |
| :---------------------------------- | :-------------------------- | :----------------------------------------------------------------------------- |
| **Nom du package (TS/JS)**          | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk`                                               |
| **Package Python**                  | `claude-code-sdk`           | `claude-agent-sdk`                                                             |
| **Emplacement de la documentation** | Documentation Claude Code   | Documentation Claude Code → section dédiée [Agent SDK](/docs/fr/agent-sdk/overview) |

<h2 id="migration-steps">
  Étapes de migration
</h2>

<h3 id="for-typescript/javascript-projects">
  Pour les projets TypeScript/JavaScript
</h3>

**1. Désinstallez l'ancien package :**

```bash theme={null}
npm uninstall @anthropic-ai/claude-code
```

**2. Installez le nouveau package :**

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

**3. Mettez à jour vos imports :**

Modifiez tous les imports de `@anthropic-ai/claude-code` vers `@anthropic-ai/claude-agent-sdk` :

```typescript theme={null}
// Avant
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-code";

// Après
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
```

**4. Mettez à jour package.json :**

Si `@anthropic-ai/claude-code` est toujours listé dans votre `package.json`, remplacez-le par `@anthropic-ai/claude-agent-sdk` et mettez à jour la plage de version également, par exemple de `"^0.0.42"` à `"^0.3.0"`.

**5. Consultez les [modifications incompatibles](#breaking-changes)**

Effectuez les modifications de code nécessaires pour terminer la migration.

<h3 id="for-python-projects">
  Pour les projets Python
</h3>

**1. Désinstallez l'ancien package :**

```bash theme={null}
pip uninstall -y claude-code-sdk
```

Si l'ancien package n'est pas installé, pip affiche `WARNING: Skipping claude-code-sdk as it is not installed.` C'est normal et vous pouvez passer à l'étape suivante.

**2. Installez le nouveau package :**

```bash theme={null}
pip install claude-agent-sdk
```

Si `claude-code-sdk` est listé dans votre `requirements.txt` ou `pyproject.toml`, remplacez-le par `claude-agent-sdk`.

**3. Mettez à jour vos imports :**

Modifiez tous les imports de `claude_code_sdk` vers `claude_agent_sdk` :

```python theme={null}
# Avant
from claude_code_sdk import query, ClaudeCodeOptions

# Après
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. Consultez les [modifications incompatibles](#breaking-changes)**

Effectuez les modifications de code nécessaires pour terminer la migration.

<h2 id="breaking-changes">
  Changements majeurs
</h2>

<Warning>
  Pour améliorer l'isolation et la configuration explicite, Claude Agent SDK v0.1.0 introduit des changements majeurs pour les utilisateurs migrant depuis Claude Code SDK.
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python : ClaudeCodeOptions renommé en ClaudeAgentOptions
</h3>

**Ce qui a changé :** Le type Python SDK `ClaudeCodeOptions` a été renommé en `ClaudeAgentOptions`.

**Migration :**

```python theme={null}
# AVANT (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# APRÈS (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

<h3 id="system-prompt-no-longer-default">
  Le système de prompt n'est plus défini par défaut
</h3>

**Ce qui a changé :** Le SDK n'utilise plus le système de prompt de Claude Code par défaut.

**Migration :**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // AVANT (v0.0.x) - Utilisait le système de prompt de Claude Code par défaut
  const before = query({ prompt: "Hello" });

  // APRÈS (v0.1.0) - Utilise un système de prompt minimal par défaut
  // Pour obtenir l'ancien comportement, demandez explicitement le préréglage de Claude Code :
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // Ou utilisez un système de prompt personnalisé :
  const customResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: "You are a helpful coding assistant"
    }
  });
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  import asyncio


  async def main():
      # AVANT (v0.0.x) - Utilisait le système de prompt de Claude Code par défaut
      async for message in query(prompt="Hello"):
          print(message)

      # APRÈS (v0.1.0) - Utilise un système de prompt minimal par défaut
      # Pour obtenir l'ancien comportement, demandez explicitement le préréglage de Claude Code :
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(
              system_prompt={"type": "preset", "preset": "claude_code"}  # Use the preset
          ),
      ):
          print(message)

      # Ou utilisez un système de prompt personnalisé :
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="settings-sources-default">
  Défaut des sources de paramètres
</h3>

Ce défaut a été brièvement modifié dans v0.1.0 pour ne charger aucun paramètre du système de fichiers, puis a été rétabli, donc aucune action de migration n'est nécessaire.

**Comportement actuel :** Omettre `settingSources` sur `query()` charge les paramètres utilisateur, projet et système de fichiers local, correspondant à la CLI. Cela inclut `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, les fichiers CLAUDE.md et les commandes personnalisées.

Pour fonctionner isolé des paramètres du système de fichiers, passez `settingSources: []`, ou `setting_sources=[]` en Python. Consultez [Contrôler les paramètres du système de fichiers avec settingSources](/docs/fr/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) pour savoir ce que charge chaque source.

L'isolation est particulièrement importante pour les pipelines CI/CD, les applications déployées, les environnements de test et les systèmes multi-locataires où les personnalisations locales ne doivent pas s'échapper.

<Note>
  Python SDK 0.1.59 et antérieures traitaient une liste vide de la même manière que l'omission de l'option, donc mettez à jour avant de vous fier à `setting_sources=[]`. Consultez [Ce que settingSources ne contrôle pas](/docs/fr/agent-sdk/claude-code-features#what-settingsources-does-not-control) pour les entrées qui sont lues même lorsque `settingSources` est `[]`.
</Note>

<h2 id="next-steps">
  Prochaines étapes
</h2>

* Explorez l'[Aperçu d'Agent SDK](/docs/fr/agent-sdk/overview) pour en savoir plus sur les fonctionnalités disponibles
* Consultez la [Référence SDK TypeScript](/docs/fr/agent-sdk/typescript) pour la documentation API détaillée
* Consultez la [Référence SDK Python](/docs/fr/agent-sdk/python) pour la documentation spécifique à Python
* En savoir plus sur les [Outils personnalisés](/docs/fr/agent-sdk/custom-tools) et l'[Intégration MCP](/docs/fr/agent-sdk/mcp)
