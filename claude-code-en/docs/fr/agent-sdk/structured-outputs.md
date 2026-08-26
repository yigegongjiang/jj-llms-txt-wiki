> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Obtenir une sortie structurée des agents

> Retourner du JSON validé à partir de workflows d'agents en utilisant JSON Schema, Zod ou Pydantic. Obtenir des données structurées et type-safe après une utilisation multi-tour d'outils.

Les sorties structurées vous permettent de définir la forme exacte des données que vous souhaitez récupérer d'un agent. L'agent peut utiliser tous les outils dont il a besoin pour accomplir la tâche, et vous obtenez toujours du JSON validé correspondant à votre schéma à la fin. Définissez un [JSON Schema](https://json-schema.org/understanding-json-schema/about) pour la structure dont vous avez besoin, et le SDK valide la sortie par rapport à celui-ci, en relançant des invites en cas de non-correspondance. Si la validation n'aboutit pas dans la limite de tentatives, le résultat est une erreur au lieu de données structurées ; voir [Gestion des erreurs](#error-handling).

Pour une sécurité de type complète, utilisez [Zod](#type-safe-schemas-with-zod-and-pydantic) (TypeScript) ou [Pydantic](#type-safe-schemas-with-zod-and-pydantic) (Python) pour définir votre schéma et obtenir des objets fortement typés en retour.

<h2 id="why-structured-outputs">
  Pourquoi les sorties structurées ?
</h2>

Les agents retournent du texte libre par défaut, ce qui fonctionne pour le chat mais pas quand vous avez besoin d'utiliser la sortie par programmation. Les sorties structurées vous donnent des données typées que vous pouvez passer directement à votre logique d'application, base de données ou composants UI.

Considérez une application de recettes où un agent recherche sur le web et ramène des recettes. Sans sorties structurées, vous obtenez du texte libre que vous devriez analyser vous-même. Avec les sorties structurées, vous définissez la forme que vous souhaitez et obtenez des données typées que vous pouvez utiliser directement dans votre application.

<AccordionGroup>
  <Accordion title="Sans sorties structurées">
    ```text theme={null}
    Voici une recette classique de biscuits aux pépites de chocolat !

    **Biscuits aux pépites de chocolat**
    Temps de préparation : 15 minutes | Temps de cuisson : 10 minutes

    Ingrédients :
    - 2 1/4 tasses de farine tout usage
    - 1 tasse de beurre, ramolli
    ...
    ```

    Pour utiliser ceci dans votre application, vous devriez analyser le titre, convertir ' 15 minutes ' en nombre, séparer les ingrédients des instructions et gérer le formatage incohérent entre les réponses.
  </Accordion>

  <Accordion title="Avec sorties structurées">
    ```json theme={null}
    {
      "name": "Biscuits aux pépites de chocolat",
      "prep_time_minutes": 15,
      "cook_time_minutes": 10,
      "ingredients": [
        { "item": "farine tout usage", "amount": 2.25, "unit": "tasses" },
        { "item": "beurre, ramolli", "amount": 1, "unit": "tasse" }
        // ...
      ],
      "steps": ["Préchauffer le four à 375°F", "Mélanger le beurre et le sucre" /* ... */]
    }
    ```

    Des données typées que vous pouvez utiliser directement dans votre UI.
  </Accordion>
</AccordionGroup>

<h2 id="quick-start">
  Démarrage rapide
</h2>

Pour utiliser les sorties structurées, définissez un [JSON Schema](https://json-schema.org/understanding-json-schema/about) décrivant la forme des données que vous souhaitez, puis passez-le à `query()` via l'option `outputFormat` (TypeScript) ou `output_format` (Python). Quand l'agent termine, le message de résultat inclut un champ `structured_output` avec des données validées correspondant à votre schéma.

L'exemple ci-dessous demande à l'agent de rechercher Anthropic et de retourner le nom de l'entreprise, l'année de fondation et le siège social en tant que sortie structurée.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Définir la forme des données que vous souhaitez récupérer
  const schema = {
    type: "object",
    properties: {
      company_name: { type: "string" },
      founded_year: { type: "number" },
      headquarters: { type: "string" }
    },
    required: ["company_name"]
  };

  for await (const message of query({
    prompt: "Research Anthropic and provide key company information",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    // Le message de résultat contient structured_output avec des données validées
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      console.log(message.structured_output);
      // { company_name: "Anthropic", founded_year: 2021, headquarters: "San Francisco, CA" }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  // Définir la forme des données que vous souhaitez récupérer
  schema = {
      "type": "object",
      "properties": {
          "company_name": {"type": "string"},
          "founded_year": {"type": "number"},
          "headquarters": {"type": "string"},
      },
      "required": ["company_name"],
  }


  async def main():
      async for message in query(
          prompt="Research Anthropic and provide key company information",
          options=ClaudeAgentOptions(
              output_format={"type": "json_schema", "schema": schema}
          ),
      ):
          // Le message de résultat contient structured_output avec des données validées
          if isinstance(message, ResultMessage) and message.structured_output:
              print(message.structured_output)
              // {'company_name': 'Anthropic', 'founded_year': 2021, 'headquarters': 'San Francisco, CA'}


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="type-safe-schemas-with-zod-and-pydantic">
  Schémas type-safe avec Zod et Pydantic
</h2>

Au lieu d'écrire JSON Schema à la main, vous pouvez utiliser [Zod](https://zod.dev/) (TypeScript) ou [Pydantic](https://docs.pydantic.dev/latest/) (Python) pour définir votre schéma. Ces bibliothèques génèrent le JSON Schema pour vous et vous permettent d'analyser la réponse en un objet entièrement typé que vous pouvez utiliser dans tout votre codebase avec l'autocomplétion et la vérification de type.

L'exemple ci-dessous définit un schéma pour un plan d'implémentation de fonctionnalité avec un résumé, une liste d'étapes (chacune avec un niveau de complexité) et les risques potentiels. L'agent planifie la fonctionnalité et retourne un objet `FeaturePlan` typé. Vous pouvez ensuite accéder à des propriétés comme `plan.summary` et itérer sur `plan.steps` avec une sécurité de type complète.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { z } from "zod";
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Définir le schéma avec Zod
  const FeaturePlan = z.object({
    feature_name: z.string(),
    summary: z.string(),
    steps: z.array(
      z.object({
        step_number: z.number(),
        description: z.string(),
        estimated_complexity: z.enum(["low", "medium", "high"])
      })
    ),
    risks: z.array(z.string())
  });

  type FeaturePlan = z.infer<typeof FeaturePlan>;

  // Convertir en JSON Schema
  const schema = z.toJSONSchema(FeaturePlan);

  // Utiliser dans la requête
  for await (const message of query({
    prompt:
      "Plan how to add dark mode support to a React app. Break it into implementation steps.",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      // Valider et obtenir un résultat entièrement typé
      const parsed = FeaturePlan.safeParse(message.structured_output);
      if (parsed.success) {
        const plan: FeaturePlan = parsed.data;
        console.log(`Feature: ${plan.feature_name}`);
        console.log(`Summary: ${plan.summary}`);
        plan.steps.forEach((step) => {
          console.log(`${step.step_number}. [${step.estimated_complexity}] ${step.description}`);
        });
      }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from pydantic import BaseModel
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  class Step(BaseModel):
      step_number: int
      description: str
      estimated_complexity: str  # 'low', 'medium', 'high'


  class FeaturePlan(BaseModel):
      feature_name: str
      summary: str
      steps: list[Step]
      risks: list[str]


  async def main():
      async for message in query(
          prompt="Plan how to add dark mode support to a React app. Break it into implementation steps.",
          options=ClaudeAgentOptions(
              output_format={
                  "type": "json_schema",
                  "schema": FeaturePlan.model_json_schema(),
              }
          ),
      ):
          if isinstance(message, ResultMessage) and message.structured_output:
              # Valider et obtenir un résultat entièrement typé
              plan = FeaturePlan.model_validate(message.structured_output)
              print(f"Feature: {plan.feature_name}")
              print(f"Summary: {plan.summary}")
              for step in plan.steps:
                  print(
                      f"{step.step_number}. [{step.estimated_complexity}] {step.description}"
                  )


  asyncio.run(main())
  ```
</CodeGroup>

**Avantages :**

* Inférence de type complète (TypeScript) et indications de type (Python)
* Validation à l'exécution avec `safeParse()` ou `model_validate()`
* Meilleurs messages d'erreur
* Schémas composables et réutilisables

<h2 id="output-format-configuration">
  Configuration du format de sortie
</h2>

L'option `outputFormat` (TypeScript) ou `output_format` (Python) accepte un objet avec :

* `type` : Défini sur `"json_schema"` pour les sorties structurées
* `schema` : Un objet [JSON Schema](https://json-schema.org/understanding-json-schema/about) définissant votre structure de sortie. Vous pouvez générer ceci à partir d'un schéma Zod avec `z.toJSONSchema()` ou d'un modèle Pydantic avec `.model_json_schema()`

Le SDK supporte les fonctionnalités JSON Schema standard incluant tous les types de base (object, array, string, number, boolean, null), `enum`, `const`, `required`, les objets imbriqués et les définitions `$ref`. Pour la liste complète des fonctionnalités supportées et des limitations, voir [Limitations JSON Schema](https://platform.claude.com/docs/fr/build-with-claude/structured-outputs#json-schema-limitations).

Un schéma qui n'est pas un JSON Schema valide échoue l'exécution au démarrage avec une erreur nommant le problème. Avant la v2.1.205, un schéma invalide était silencieusement ignoré et l'agent retournait du texte non structuré.

Le mot-clé `format`, tel que `"format": "email"`, est accepté comme annotation et n'est pas appliqué par le validateur du SDK. Avant la v2.1.205, tout schéma contenant `format` était traité comme invalide.

<h2 id="example-todo-tracking-agent">
  Exemple : agent de suivi des TODO
</h2>

Cet exemple démontre comment les sorties structurées fonctionnent avec l'utilisation multi-étapes d'outils. L'agent doit trouver des commentaires TODO dans la base de code, puis rechercher les informations de git blame pour chacun. Il décide de manière autonome quels outils utiliser (Grep pour rechercher, Bash pour exécuter les commandes git) et combine les résultats en une seule réponse structurée.

Le schéma inclut des champs optionnels (`author` et `date`) puisque les informations de git blame pourraient ne pas être disponibles pour tous les fichiers. L'agent remplit ce qu'il peut trouver et omet le reste.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Définir la structure pour l'extraction de TODO
  const todoSchema = {
    type: "object",
    properties: {
      todos: {
        type: "array",
        items: {
          type: "object",
          properties: {
            text: { type: "string" },
            file: { type: "string" },
            line: { type: "number" },
            author: { type: "string" },
            date: { type: "string" }
          },
          required: ["text", "file", "line"]
        }
      },
      total_count: { type: "number" }
    },
    required: ["todos", "total_count"]
  };

  // L'agent utilise Grep pour trouver les TODOs, Bash pour obtenir les informations de git blame
  for await (const message of query({
    prompt: "Find all TODO comments in this codebase and identify who added them",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: todoSchema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      const data = message.structured_output as { total_count: number; todos: Array<{ file: string; line: number; text: string; author?: string; date?: string }> };
      console.log(`Found ${data.total_count} TODOs`);
      data.todos.forEach((todo) => {
        console.log(`${todo.file}:${todo.line} - ${todo.text}`);
        if (todo.author) {
          console.log(`  Added by ${todo.author} on ${todo.date}`);
        }
      });
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Définir la structure pour l'extraction de TODO
  todo_schema = {
      "type": "object",
      "properties": {
          "todos": {
              "type": "array",
              "items": {
                  "type": "object",
                  "properties": {
                      "text": {"type": "string"},
                      "file": {"type": "string"},
                      "line": {"type": "number"},
                      "author": {"type": "string"},
                      "date": {"type": "string"},
                  },
                  "required": ["text", "file", "line"],
              },
          },
          "total_count": {"type": "number"},
      },
      "required": ["todos", "total_count"],
  }


  async def main():
      # L'agent utilise Grep pour trouver les TODOs, Bash pour obtenir les informations de git blame
      async for message in query(
          prompt="Find all TODO comments in this codebase and identify who added them",
          options=ClaudeAgentOptions(
              output_format={"type": "json_schema", "schema": todo_schema}
          ),
      ):
          if isinstance(message, ResultMessage) and message.structured_output:
              data = message.structured_output
              print(f"Found {data['total_count']} TODOs")
              for todo in data["todos"]:
                  print(f"{todo['file']}:{todo['line']} - {todo['text']}")
                  if "author" in todo:
                      print(f"  Added by {todo['author']} on {todo['date']}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="error-handling">
  Gestion des erreurs
</h2>

La génération de sortie structurée peut échouer quand l'agent ne peut pas produire du JSON valide correspondant à votre schéma. Cela se produit généralement quand le schéma est trop complexe pour la tâche, la tâche elle-même est ambiguë, ou l'agent atteint sa limite de tentatives en essayant de corriger les erreurs de validation. Cela peut aussi se produire sans aucune erreur de validation : un [repli de modèle](/docs/fr/model-config#automatic-model-fallback) peut rétracter une sortie déjà complétée en cours de flux, et si aucune nouvelle tentative ne la remplace, l'exécution se termine avec la même erreur. Vérifiez le champ `errors` du message de résultat pour distinguer les deux causes avant de déboguer votre schéma.

Quand une erreur se produit, le message de résultat a un `subtype` indiquant ce qui s'est mal passé :

| Subtype                               | Signification                                                                                                                                             |
| ------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `success`                             | La sortie a été générée et validée avec succès                                                                                                            |
| `error_max_structured_output_retries` | Aucune sortie valide n'a survécu après plusieurs tentatives (erreurs de validation, ou une rétraction de repli de modèle sans nouvelle tentative réussie) |

L'exemple ci-dessous vérifie le champ `subtype` pour déterminer si la sortie a été générée avec succès ou si vous devez gérer un échec :

<CodeGroup>
  ```typescript TypeScript theme={null}
  for await (const msg of query({
    prompt: "Extract contact info from the document",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: contactSchema
      }
    }
  })) {
    if (msg.type === "result") {
      if (msg.subtype === "success" && msg.structured_output) {
        // Utiliser la sortie validée
        console.log(msg.structured_output);
      } else if (msg.subtype === "error_max_structured_output_retries") {
        // Gérer l'échec - réessayer avec une invite plus simple, revenir à non-structuré, etc.
        console.error("Could not produce valid output");
      }
    }
  }
  ```

  ```python Python theme={null}
  async for message in query(
      prompt="Extract contact info from the document",
      options=ClaudeAgentOptions(
          output_format={"type": "json_schema", "schema": contact_schema}
      ),
  ):
      if isinstance(message, ResultMessage):
          if message.subtype == "success" and message.structured_output:
              # Utiliser la sortie validée
              print(message.structured_output)
          elif message.subtype == "error_max_structured_output_retries":
              # Gérer l'échec
              print("Could not produce valid output")
  ```
</CodeGroup>

**Conseils pour éviter les erreurs :**

* **Gardez les schémas ciblés.** Les schémas profondément imbriqués avec de nombreux champs requis sont plus difficiles à satisfaire. Commencez simple et ajoutez de la complexité au besoin.
* **Faites correspondre le schéma à la tâche.** Si la tâche pourrait ne pas avoir toutes les informations que votre schéma nécessite, rendez ces champs optionnels.
* **Utilisez des invites claires.** Les invites ambiguës rendent plus difficile pour l'agent de savoir quelle sortie produire.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Documentation JSON Schema](https://json-schema.org/) : apprenez la syntaxe JSON Schema pour définir des schémas complexes avec des objets imbriqués, des tableaux, des énumérations et des contraintes de validation
* [Sorties structurées de l'API](https://platform.claude.com/docs/en/build-with-claude/structured-outputs) : utilisez les sorties structurées avec l'API Claude directement pour les requêtes à un seul tour sans utilisation d'outils
* [Outils personnalisés](/docs/fr/agent-sdk/custom-tools) : donnez à votre agent des outils personnalisés à appeler pendant l'exécution avant de retourner une sortie structurée
