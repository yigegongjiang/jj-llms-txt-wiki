> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Obter saída estruturada de agentes

> Retorne JSON validado de fluxos de trabalho de agentes usando JSON Schema, Zod ou Pydantic. Obtenha dados estruturados e type-safe após o uso de múltiplas ferramentas.

Saídas estruturadas permitem que você defina a forma exata dos dados que deseja receber de um agente. O agente pode usar qualquer ferramenta necessária para completar a tarefa, e você ainda obtém JSON validado correspondente ao seu schema no final. Defina um [JSON Schema](https://json-schema.org/understanding-json-schema/about) para a estrutura que você precisa, e o SDK valida a saída contra ele, re-solicitando em caso de incompatibilidade. Se a validação não for bem-sucedida dentro do limite de tentativas, o resultado é um erro em vez de dados estruturados; veja [Tratamento de erros](#error-handling).

Para segurança de tipo completa, use [Zod](#type-safe-schemas-with-zod-and-pydantic) (TypeScript) ou [Pydantic](#type-safe-schemas-with-zod-and-pydantic) (Python) para definir seu schema e obter objetos fortemente tipados.

<h2 id="why-structured-outputs">
  Por que saídas estruturadas?
</h2>

Agentes retornam texto livre por padrão, o que funciona para chat, mas não quando você precisa usar a saída programaticamente. Saídas estruturadas fornecem dados tipados que você pode passar diretamente para sua lógica de aplicação, banco de dados ou componentes de UI.

Considere um aplicativo de receitas onde um agente pesquisa a web e traz receitas. Sem saídas estruturadas, você obtém texto livre que precisaria analisar você mesmo. Com saídas estruturadas, você define a forma que deseja e obtém dados tipados que pode usar diretamente em seu aplicativo.

<AccordionGroup>
  <Accordion title="Sem saídas estruturadas">
    ```text theme={null}
    Aqui está uma receita clássica de biscoito com gotas de chocolate!

    **Biscoitos com Gotas de Chocolate**
    Tempo de preparo: 15 minutos | Tempo de cozimento: 10 minutos

    Ingredientes:
    - 2 1/4 xícaras de farinha de trigo
    - 1 xícara de manteiga, amolecida
    ...
    ```

    Para usar isso em seu aplicativo, você precisaria analisar o título, converter "15 minutos" em um número, separar ingredientes de instruções e lidar com formatação inconsistente entre respostas.
  </Accordion>

  <Accordion title="Com saídas estruturadas">
    ```json theme={null}
    {
      "name": "Biscoitos com Gotas de Chocolate",
      "prep_time_minutes": 15,
      "cook_time_minutes": 10,
      "ingredients": [
        { "item": "farinha de trigo", "amount": 2.25, "unit": "xícaras" },
        { "item": "manteiga, amolecida", "amount": 1, "unit": "xícara" }
        // ...
      ],
      "steps": ["Preaqueça o forno a 375°F", "Misture manteiga e açúcar" /* ... */]
    }
    ```

    Dados tipados que você pode usar diretamente em sua UI.
  </Accordion>
</AccordionGroup>

<h2 id="quick-start">
  Início rápido
</h2>

Para usar saídas estruturadas, defina um [JSON Schema](https://json-schema.org/understanding-json-schema/about) descrevendo a forma dos dados que você deseja, depois passe-o para `query()` via a opção `outputFormat` (TypeScript) ou `output_format` (Python). Quando o agente terminar, a mensagem de resultado inclui um campo `structured_output` com dados validados correspondendo ao seu schema.

O exemplo abaixo pede ao agente para pesquisar Anthropic e retornar o nome da empresa, ano de fundação e sede como saída estruturada.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Define a forma dos dados que você deseja receber
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
    // A mensagem de resultado contém structured_output com dados validados
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      console.log(message.structured_output);
      // { company_name: "Anthropic", founded_year: 2021, headquarters: "San Francisco, CA" }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Define a forma dos dados que você deseja receber
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
          # A mensagem de resultado contém structured_output com dados validados
          if isinstance(message, ResultMessage) and message.structured_output:
              print(message.structured_output)
              # {'company_name': 'Anthropic', 'founded_year': 2021, 'headquarters': 'San Francisco, CA'}


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="type-safe-schemas-with-zod-and-pydantic">
  Schemas type-safe com Zod e Pydantic
</h2>

Em vez de escrever JSON Schema manualmente, você pode usar [Zod](https://zod.dev/) (TypeScript) ou [Pydantic](https://docs.pydantic.dev/latest/) (Python) para definir seu schema. Essas bibliotecas geram o JSON Schema para você e permitem que você analise a resposta em um objeto totalmente tipado que você pode usar em todo seu codebase com autocomplete e verificação de tipo.

O exemplo abaixo define um schema para um plano de implementação de recurso com um resumo, lista de etapas (cada uma com nível de complexidade) e riscos potenciais. O agente planeja o recurso e retorna um objeto `FeaturePlan` tipado. Você pode então acessar propriedades como `plan.summary` e iterar sobre `plan.steps` com segurança de tipo completa.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { z } from "zod";
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Define schema com Zod
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

  // Converta para JSON Schema
  const schema = z.toJSONSchema(FeaturePlan);

  // Use em query
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
      // Valide e obtenha resultado totalmente tipado
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
              # Valide e obtenha resultado totalmente tipado
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

**Benefícios:**

* Inferência de tipo completa (TypeScript) e dicas de tipo (Python)
* Validação em tempo de execução com `safeParse()` ou `model_validate()`
* Mensagens de erro melhores
* Schemas compostos e reutilizáveis

<h2 id="output-format-configuration">
  Configuração do formato de saída
</h2>

A opção `outputFormat` (TypeScript) ou `output_format` (Python) aceita um objeto com:

* `type`: Defina como `"json_schema"` para saídas estruturadas
* `schema`: Um objeto [JSON Schema](https://json-schema.org/understanding-json-schema/about) definindo sua estrutura de saída. Você pode gerar isso a partir de um schema Zod com `z.toJSONSchema()` ou um modelo Pydantic com `.model_json_schema()`

O SDK suporta recursos padrão de JSON Schema, incluindo todos os tipos básicos (object, array, string, number, boolean, null), `enum`, `const`, `required`, objetos aninhados e definições `$ref`. Para a lista completa de recursos suportados e limitações, veja [Limitações de JSON Schema](https://platform.claude.com/docs/pt/build-with-claude/structured-outputs#json-schema-limitations).

Um schema que não é um JSON Schema válido falha na execução na inicialização com um erro nomeando o problema. Antes da v2.1.205, um schema inválido era silenciosamente ignorado e o agente retornava texto não estruturado.

A palavra-chave `format`, como `"format": "email"`, é aceita como uma anotação e não é aplicada pelo validador do SDK. Antes da v2.1.205, qualquer schema contendo `format` era tratado como inválido.

<h2 id="example-todo-tracking-agent">
  Exemplo: agente de rastreamento de TODO
</h2>

Este exemplo demonstra como saídas estruturadas funcionam com uso de ferramentas em múltiplas etapas. O agente precisa encontrar comentários TODO no codebase, depois procurar informações de git blame para cada um. Ele decide autonomamente quais ferramentas usar (Grep para pesquisar, Bash para executar comandos git) e combina os resultados em uma única resposta estruturada.

O schema inclui campos opcionais (`author` e `date`) já que informações de git blame podem não estar disponíveis para todos os arquivos. O agente preenche o que consegue encontrar e omite o resto.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Define estrutura para extração de TODO
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

  // Agente usa Grep para encontrar TODOs, Bash para obter informações de git blame
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

  # Define estrutura para extração de TODO
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
      # Agente usa Grep para encontrar TODOs, Bash para obter informações de git blame
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
  Tratamento de erros
</h2>

A geração de saída estruturada pode falhar quando o agente não consegue produzir JSON válido correspondendo ao seu schema. Isso normalmente acontece quando o schema é muito complexo para a tarefa, a tarefa em si é ambígua, ou o agente atinge seu limite de tentativas tentando corrigir erros de validação. Também pode acontecer sem nenhuma falha de validação: um [fallback de modelo](/docs/pt/model-config#automatic-model-fallback) pode retratar uma saída já concluída no meio do fluxo, e se nenhuma tentativa bem-sucedida a substituir, a execução termina com o mesmo erro. Verifique o campo `errors` na mensagem de resultado para distinguir as duas causas antes de depurar seu schema.

Quando um erro ocorre, a mensagem de resultado tem um `subtype` indicando o que deu errado:

| Subtype                               | Significado                                                                                                                                       |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `success`                             | Saída foi gerada e validada com sucesso                                                                                                           |
| `error_max_structured_output_retries` | Nenhuma saída válida sobreviveu após múltiplas tentativas (falhas de validação, ou uma retração de fallback de modelo sem tentativa bem-sucedida) |

O exemplo abaixo verifica o campo `subtype` para determinar se a saída foi gerada com sucesso ou se você precisa lidar com uma falha:

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
        // Use a saída validada
        console.log(msg.structured_output);
      } else if (msg.subtype === "error_max_structured_output_retries") {
        // Trate a falha - tente novamente com prompt mais simples, volte para não estruturado, etc.
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
              # Use a saída validada
              print(message.structured_output)
          elif message.subtype == "error_max_structured_output_retries":
              # Trate a falha
              print("Could not produce valid output")
  ```
</CodeGroup>

**Dicas para evitar erros:**

* **Mantenha schemas focados.** Schemas profundamente aninhados com muitos campos obrigatórios são mais difíceis de satisfazer. Comece simples e adicione complexidade conforme necessário.
* **Corresponda schema à tarefa.** Se a tarefa pode não ter todas as informações que seu schema requer, torne esses campos opcionais.
* **Use prompts claros.** Prompts ambíguos dificultam para o agente saber qual saída produzir.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Documentação de JSON Schema](https://json-schema.org/): aprenda sintaxe de JSON Schema para definir schemas complexos com objetos aninhados, arrays, enums e restrições de validação
* [Saídas Estruturadas da API](https://platform.claude.com/docs/pt/build-with-claude/structured-outputs): use saídas estruturadas com a API Claude diretamente para solicitações de turno único sem uso de ferramentas
* [Ferramentas personalizadas](/docs/pt/agent-sdk/custom-tools): dê ao seu agente ferramentas personalizadas para chamar durante a execução antes de retornar saída estruturada
