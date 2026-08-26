> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Migrer vers Claude Agent SDK

> Guide pour migrer les SDK TypeScript et Python de Claude Code vers Claude Agent SDK

<h2 id="overview">
  Aperçu
</h2>

Le SDK Claude Code a été renommé en **Claude Agent SDK** et sa documentation a été réorganisée. Ce changement reflète les capacités plus larges du SDK pour construire des agents IA au-delà des simples tâches de codage.

<h2 id="what’s-changed">
  Qu'est-ce qui a changé
</h2>

| Aspect                              | Ancien                      | Nouveau                          |
| :---------------------------------- | :-------------------------- | :------------------------------- |
| **Nom du package (TS/JS)**          | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk` |
| **Package Python**                  | `claude-code-sdk`           | `claude-agent-sdk`               |
| **Emplacement de la documentation** | Documentation Claude Code   | Guide API → Section Agent SDK    |

<Note>
  **Modifications de la documentation :** La documentation d'Agent SDK a été déplacée de la documentation Claude Code vers le Guide API sous une section dédiée [Agent SDK](/docs/fr/agent-sdk/overview). La documentation Claude Code se concentre désormais sur l'outil CLI et les fonctionnalités d'automatisation.
</Note>

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

**4. Mettez à jour les dépendances package.json :**

Si vous avez le package listé dans votre `package.json`, mettez-le à jour :

Avant :

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-code": "^0.0.42"
  }
}
```

Après :

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-agent-sdk": "^0.2.0"
  }
}
```

**5. Consultez les [modifications incompatibles](#breaking-changes)**

Effectuez les modifications de code nécessaires pour terminer la migration.

<h3 id="for-python-projects">
  Pour les projets Python
</h3>

**1. Désinstallez l'ancien package :**

```bash theme={null}
pip uninstall claude-code-sdk
```

**2. Installez le nouveau package :**

```bash theme={null}
pip install claude-agent-sdk
```

**3. Mettez à jour vos imports :**

Modifiez tous les imports de `claude_code_sdk` vers `claude_agent_sdk` :

```python theme={null}
# Avant
from claude_code_sdk import query, ClaudeCodeOptions

# Après
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. Mettez à jour les noms de types :**

Modifiez `ClaudeCodeOptions` en `ClaudeAgentOptions` :

```python theme={null}
# Avant
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7")

# Après
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7")
```

**5. Consultez les [modifications incompatibles](#breaking-changes)**

Effectuez les modifications de code nécessaires pour terminer la migration.

<h2 id="breaking-changes">
  Modifications incompatibles
</h2>

<Warning>
  Pour améliorer l'isolation et la configuration explicite, Claude Agent SDK v0.1.0 introduit des modifications incompatibles pour les utilisateurs migrant depuis Claude Code SDK. Consultez attentivement cette section avant de migrer.
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python : ClaudeCodeOptions renommé en ClaudeAgentOptions
</h3>

**Qu'est-ce qui a changé :** Le type SDK Python `ClaudeCodeOptions` a été renommé en `ClaudeAgentOptions`.

**Migration :**

```python theme={null}
# AVANT (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# APRÈS (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

**Pourquoi ce changement :** Le nom du type correspond désormais à la marque « Claude Agent SDK » et assure la cohérence dans les conventions de nommage du SDK.

<h3 id="system-prompt-no-longer-default">
  Le système prompt n'est plus par défaut
</h3>

**Qu'est-ce qui a changé :** Le SDK n'utilise plus le système prompt de Claude Code par défaut.

**Migration :**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // AVANT (v0.0.x) - Utilisait le système prompt de Claude Code par défaut
  const before = query({ prompt: "Hello" });

  // APRÈS (v0.1.0) - Utilise un système prompt minimal par défaut
  // Pour obtenir l'ancien comportement, demandez explicitement le préréglage de Claude Code :
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // Ou utilisez un système prompt personnalisé :
  const customResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: "You are a helpful coding assistant"
    }
  });
  ```

  ```python Python theme={null}
  # AVANT (v0.0.x) - Utilisait le système prompt de Claude Code par défaut
  async for message in query(prompt="Hello"):
      print(message)

  # APRÈS (v0.1.0) - Utilise un système prompt minimal par défaut
  # Pour obtenir l'ancien comportement, demandez explicitement le préréglage de Claude Code :
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          system_prompt={"type": "preset", "preset": "claude_code"}  # Utiliser le préréglage
      ),
  ):
      print(message)

  # Ou utilisez un système prompt personnalisé :
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
  ):
      print(message)
  ```
</CodeGroup>

**Pourquoi ce changement :** Fournit un meilleur contrôle et une meilleure isolation pour les applications SDK. Vous pouvez désormais construire des agents avec un comportement personnalisé sans hériter des instructions axées sur le CLI de Claude Code.

<h3 id="settings-sources-default">
  Défaut des sources de paramètres
</h3>

Ce défaut a été brièvement modifié dans v0.1.0 puis annulé, donc aucune action de migration n'est nécessaire.

**Comportement actuel :** L'omission de `settingSources` sur `query()` charge les paramètres utilisateur, projet et système de fichiers local, correspondant au CLI. Cela inclut `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, les fichiers CLAUDE.md et les commandes personnalisées.

Pour s'exécuter isolé des paramètres du système de fichiers, passez un tableau vide :

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const isolatedResult = query({
    prompt: "Hello",
    options: {
      settingSources: [] // Aucun paramètre du système de fichiers chargé
    }
  });

  // Ou charger uniquement des sources spécifiques :
  const projectOnlyResult = query({
    prompt: "Hello",
    options: {
      settingSources: ["project"] // Uniquement les paramètres du projet
    }
  });
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(setting_sources=[]),  # Aucun paramètre du système de fichiers chargé
  ):
      print(message)

  # Ou charger uniquement des sources spécifiques :
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          setting_sources=["project"]  # Uniquement les paramètres du projet
      ),
  ):
      print(message)
  ```
</CodeGroup>

L'isolation est particulièrement importante pour les pipelines CI/CD, les applications déployées, les environnements de test et les systèmes multi-locataires où les personnalisations locales ne doivent pas s'infiltrer.

<Note>
  SDK v0.1.0 a brièvement défini par défaut aucun paramètre chargé ; cela a été annulé dans les versions ultérieures. Python SDK 0.1.59 et antérieures traitaient une liste vide de la même manière que l'omission de l'option, donc mettez à jour avant de compter sur `setting_sources=[]`. Consultez [Ce que settingSources ne contrôle pas](/docs/fr/agent-sdk/claude-code-features#what-settingsources-does-not-control) pour les entrées qui sont lues même lorsque `settingSources` est `[]`.
</Note>

<h2 id="why-the-rename">
  Pourquoi le changement de nom ?
</h2>

Le SDK Claude Code a été conçu à l'origine pour les tâches de codage, mais il a évolué en un cadre puissant pour construire tous les types d'agents IA. Le nouveau nom « Claude Agent SDK » reflète mieux ses capacités :

* Construire des agents commerciaux (assistants juridiques, conseillers financiers, support client)
* Créer des agents de codage spécialisés (bots SRE, examinateurs de sécurité, agents d'examen de code)
* Développer des agents personnalisés pour n'importe quel domaine avec utilisation d'outils, intégration MCP et bien plus

<h2 id="getting-help">
  Obtenir de l'aide
</h2>

Si vous rencontrez des problèmes lors de la migration :

**Pour TypeScript/JavaScript :**

1. Vérifiez que tous les imports sont mis à jour pour utiliser `@anthropic-ai/claude-agent-sdk`
2. Vérifiez que votre package.json a le nouveau nom de package
3. Exécutez `npm install` pour vous assurer que les dépendances sont mises à jour

**Pour Python :**

1. Vérifiez que tous les imports sont mis à jour pour utiliser `claude_agent_sdk`
2. Vérifiez que votre requirements.txt ou pyproject.toml a le nouveau nom de package
3. Exécutez `pip install claude-agent-sdk` pour vous assurer que le package est installé

<h2 id="next-steps">
  Prochaines étapes
</h2>

* Explorez l'[Aperçu d'Agent SDK](/docs/fr/agent-sdk/overview) pour en savoir plus sur les fonctionnalités disponibles
* Consultez la [Référence SDK TypeScript](/docs/fr/agent-sdk/typescript) pour la documentation API détaillée
* Consultez la [Référence SDK Python](/docs/fr/agent-sdk/python) pour la documentation spécifique à Python
* En savoir plus sur les [Outils personnalisés](/docs/fr/agent-sdk/custom-tools) et l'[Intégration MCP](/docs/fr/agent-sdk/mcp)
