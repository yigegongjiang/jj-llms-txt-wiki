> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dê a Claude ferramentas personalizadas

> Defina ferramentas personalizadas com o servidor MCP em processo do Agent SDK do Claude para que Claude possa chamar suas funções, acessar suas APIs e executar operações específicas do domínio.

Ferramentas personalizadas estendem o Agent SDK permitindo que você defina suas próprias funções que Claude pode chamar durante uma conversa. Usando o servidor MCP em processo do SDK, você pode dar a Claude acesso a bancos de dados, APIs externas, lógica específica do domínio ou qualquer outra capacidade que sua aplicação necessite.

Este guia cobre como definir ferramentas com esquemas de entrada e manipuladores, agrupá-las em um servidor MCP, passá-las para `query` e controlar quais ferramentas Claude pode acessar. Também cobre tratamento de erros, anotações de ferramentas e retorno de conteúdo não-texto como imagens.

<h2 id="quick-reference">
  Referência rápida
</h2>

| Se você quer...                                        | Faça isto                                                                                                                                                                                                                    |
| :----------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Definir uma ferramenta                                 | Use [`@tool`](/docs/pt/agent-sdk/python#tool) (Python) ou [`tool()`](/docs/pt/agent-sdk/typescript#tool) (TypeScript) com um nome, descrição, esquema e manipulador. Veja [Criar uma ferramenta personalizada](#create-a-custom-tool). |
| Registrar uma ferramenta com Claude                    | Envolva em `create_sdk_mcp_server` / `createSdkMcpServer` e passe para `mcpServers` em `query()`. Veja [Chamar uma ferramenta personalizada](#call-a-custom-tool).                                                           |
| Pré-aprovar uma ferramenta                             | Adicione às suas ferramentas permitidas. Veja [Configurar ferramentas permitidas](#configure-allowed-tools).                                                                                                                 |
| Remover uma ferramenta integrada do contexto de Claude | Passe um array `tools` listando apenas os integrados que você quer. Veja [Configurar ferramentas permitidas](#configure-allowed-tools).                                                                                      |
| Deixar Claude chamar ferramentas em paralelo           | Defina `readOnlyHint: true` em ferramentas sem efeitos colaterais. Veja [Adicionar anotações de ferramentas](#add-tool-annotations).                                                                                         |
| Controlar a mensagem de erro que Claude lê             | Retorne `isError: true` para compor a mensagem em vez de expor a exceção bruta. Veja [Tratar erros](#handle-errors).                                                                                                         |
| Retornar imagens ou arquivos                           | Use blocos `image` ou `resource` no array de conteúdo. Veja [Retornar imagens e recursos](#return-images-and-resources).                                                                                                     |
| Retornar um resultado JSON legível por máquina         | Defina `structuredContent` no resultado. Veja [Retornar dados estruturados](#return-structured-data).                                                                                                                        |
| Escalar para muitas ferramentas                        | Use [tool search](/docs/pt/agent-sdk/tool-search) para carregar ferramentas sob demanda.                                                                                                                                          |

<h2 id="create-a-custom-tool">
  Criar uma ferramenta personalizada
</h2>

Uma ferramenta é definida por quatro partes, passadas como argumentos para o auxiliar [`tool()`](/docs/pt/agent-sdk/typescript#tool) em TypeScript ou o decorador [`@tool`](/docs/pt/agent-sdk/python#tool) em Python:

* **Nome:** um identificador único que Claude usa para chamar a ferramenta.
* **Descrição:** o que a ferramenta faz. Claude lê isto para decidir quando chamá-la.
* **Esquema de entrada:** os argumentos que Claude deve fornecer. Em TypeScript isto é sempre um [esquema Zod](https://zod.dev/), e os `args` do manipulador são tipados automaticamente a partir dele. Em Python isto é um dict mapeando nomes para tipos, como `{"latitude": float}`, que o SDK converte para JSON Schema para você. O decorador Python também aceita um dict completo de [JSON Schema](https://json-schema.org/understanding-json-schema/about) diretamente quando você precisa de enums, intervalos, campos opcionais ou objetos aninhados.
* **Manipulador:** a função assíncrona que executa quando Claude chama a ferramenta. Ela recebe os argumentos validados e deve retornar um objeto com:
  * `content` (obrigatório): um array de blocos de resultado, cada um com um `type` de `"text"`, `"image"`, `"audio"`, `"resource"` ou `"resource_link"`. Veja [Retornar imagens e recursos](#return-images-and-resources) para blocos não-texto.
  * `structuredContent` (opcional): um objeto JSON contendo o resultado como dados legíveis por máquina, retornado junto com `content`. Veja [Retornar dados estruturados](#return-structured-data).
  * `isError` (opcional): defina como `true` para sinalizar uma falha de ferramenta para que Claude possa reagir a ela. Veja [Tratar erros](#handle-errors).

Depois de definir uma ferramenta, envolva-a em um servidor com [`createSdkMcpServer`](/docs/pt/agent-sdk/typescript#createsdkmcpserver) (TypeScript) ou [`create_sdk_mcp_server`](/docs/pt/agent-sdk/python#create_sdk_mcp_server) (Python). O servidor executa em processo dentro de sua aplicação, não como um processo separado.

<h3 id="weather-tool-example">
  Exemplo de ferramenta de clima
</h3>

Este exemplo define uma ferramenta `get_temperature` e a envolve em um servidor MCP. Ele apenas configura a ferramenta; para passá-la para `query` e executá-la, veja [Chamar uma ferramenta personalizada](#call-a-custom-tool) abaixo.

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

Veja a referência TypeScript [`tool()`](/docs/pt/agent-sdk/typescript#tool) ou a referência Python [`@tool`](/docs/pt/agent-sdk/python#tool) para detalhes completos de parâmetros, incluindo formatos de entrada JSON Schema e estrutura de valor de retorno.

<Tip>
  Para tornar um parâmetro opcional: em TypeScript, adicione `.default()` ao campo Zod. Em Python, o esquema dict trata cada chave como obrigatória, então deixe o parâmetro fora do esquema, mencione-o na string de descrição e leia-o com `args.get()` no manipulador. A ferramenta [`get_precipitation_chance` abaixo](#add-more-tools) mostra ambos os padrões.
</Tip>

<h3 id="call-a-custom-tool">
  Chamar uma ferramenta personalizada
</h3>

Passe o servidor MCP que você criou para `query` via a opção `mcpServers`. A chave em `mcpServers` torna-se o segmento `{server_name}` no nome totalmente qualificado de cada ferramenta: `mcp__{server_name}__{tool_name}`. Liste esse nome em `allowedTools` para que a ferramenta execute sem um prompt de permissão.

Estes trechos reutilizam o `weatherServer` do [exemplo acima](#weather-tool-example) para perguntar a Claude qual é o clima em um local específico.

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
  Adicionar mais ferramentas
</h3>

Um servidor contém quantas ferramentas você listar em seu array `tools`. Com mais de uma ferramenta em um servidor, você pode listar cada uma em `allowedTools` individualmente ou usar o curinga `mcp__weather__*` para cobrir cada ferramenta que o servidor expõe.

O exemplo abaixo adiciona uma segunda ferramenta, `get_precipitation_chance`, ao `weatherServer` do [exemplo de ferramenta de clima](#weather-tool-example) e o reconstrói com ambas as ferramentas no array.

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

Cada ferramenta neste array consome espaço de janela de contexto a cada turno. Se você está definindo dezenas de ferramentas, veja [tool search](/docs/pt/agent-sdk/tool-search) para carregá-las sob demanda em vez disso.

<h3 id="add-tool-annotations">
  Adicionar anotações de ferramentas
</h3>

[Anotações de ferramentas](https://modelcontextprotocol.io/docs/concepts/tools#tool-annotations) são metadados opcionais descrevendo como uma ferramenta se comporta. Passe-as como o quinto argumento para o auxiliar `tool()` em TypeScript ou via o argumento de palavra-chave `annotations` para o decorador `@tool` em Python. Todos os campos de dica são Booleanos.

| Campo             | Padrão  | Significado                                                                                                                           |
| :---------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------ |
| `readOnlyHint`    | `false` | A ferramenta não modifica seu ambiente. Controla se a ferramenta pode ser chamada em paralelo com outras ferramentas somente leitura. |
| `destructiveHint` | `true`  | A ferramenta pode executar atualizações destrutivas. Apenas informativo.                                                              |
| `idempotentHint`  | `false` | Chamadas repetidas com os mesmos argumentos não têm efeito adicional. Apenas informativo.                                             |
| `openWorldHint`   | `true`  | A ferramenta alcança sistemas fora de seu processo. Apenas informativo.                                                               |

Anotações são metadados, não imposição. Uma ferramenta marcada com `readOnlyHint: true` ainda pode escrever em disco se é isso que o manipulador faz. Mantenha a anotação precisa em relação ao manipulador.

Este exemplo adiciona `readOnlyHint` à ferramenta `get_temperature` do [exemplo de ferramenta de clima](#weather-tool-example).

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

Veja `ToolAnnotations` na referência [TypeScript](/docs/pt/agent-sdk/typescript#toolannotations) ou [Python](/docs/pt/agent-sdk/python#toolannotations).

<h2 id="control-tool-access">
  Controlar acesso a ferramentas
</h2>

O [exemplo de ferramenta de clima](#weather-tool-example) registrou um servidor e listou ferramentas em `allowedTools`. Esta seção cobre como nomes de ferramentas são construídos e como escopar acesso quando você tem múltiplas ferramentas ou quer restringir integrados.

<h3 id="tool-name-format">
  Formato de nome de ferramenta
</h3>

Quando ferramentas MCP são expostas a Claude, seus nomes seguem um formato específico:

* Padrão: `mcp__{server_name}__{tool_name}`
* Exemplo: Uma ferramenta nomeada `get_temperature` no servidor `weather` torna-se `mcp__weather__get_temperature`

<h3 id="configure-allowed-tools">
  Configurar ferramentas permitidas
</h3>

A opção `tools` e as listas de permitidas/não permitidas afetam duas camadas: disponibilidade, que controla se uma ferramenta aparece no contexto de Claude, e permissão, que controla se uma chamada é aprovada uma vez que Claude tenta. `tools` e entradas de `disallowedTools` com nome simples alteram a disponibilidade. `allowedTools` e regras de `disallowedTools` com escopo alteram apenas a permissão.

| Opção                      | Camada          | Efeito                                                                                                                                                                                                                                  |
| :------------------------- | :-------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools: ["Read", "Grep"]`  | Disponibilidade | Apenas os integrados listados estão no contexto de Claude. Integrados não listados são removidos. Ferramentas MCP não são afetadas.                                                                                                     |
| `tools: []`                | Disponibilidade | Todos os integrados são removidos. Claude pode usar apenas suas ferramentas MCP.                                                                                                                                                        |
| ferramentas permitidas     | Permissão       | Ferramentas listadas executam sem um prompt de permissão. Ferramentas não listadas permanecem disponíveis; chamadas passam pelo [fluxo de permissão](/docs/pt/agent-sdk/permissions).                                                        |
| ferramentas não permitidas | Ambas           | Um nome de ferramenta simples como `"Bash"` remove a ferramenta do contexto de Claude, o mesmo que omiti-la de `tools`. Uma regra com escopo como `"Bash(rm *)"` deixa a ferramenta no contexto e nega apenas chamadas correspondentes. |

Para remover um integrado completamente, omita-o de `tools` ou liste seu nome simples em `disallowedTools` (Python: `disallowed_tools`); ambos mantêm a ferramenta fora do contexto para que Claude nunca tente. Uma regra `disallowedTools` com escopo bloqueia chamadas correspondentes mas deixa a ferramenta visível, então Claude pode desperdiçar um turno tentando. Veja [Configurar permissões](/docs/pt/agent-sdk/permissions) para a ordem de avaliação completa.

<h2 id="handle-errors">
  Tratar erros
</h2>

Um erro de manipulador não interrompe o loop do agente. O servidor MCP em processo do SDK captura exceções não capturadas e as retorna como resultados de erro, portanto como você relata um erro determina o que Claude lê, não se a consulta falha:

| O que acontece                                                                          | Resultado                                                                                                                                                     |
| :-------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Manipulador lança uma exceção não capturada                                             | O servidor MCP a converte em um resultado de erro contendo a mensagem de exceção bruta. Claude vê essa mensagem, e o loop do agente continua.                 |
| Manipulador captura o erro e retorna `isError: true` (TS) / `"is_error": True` (Python) | Claude vê a mensagem que você compõe. Você pode adicionar contexto que a exceção bruta não possui, como qual solicitação falhou ou o que tentar em vez disso. |

Em ambos os casos Claude pode tentar novamente, tentar uma ferramenta diferente ou explicar a falha. Capture erros você mesmo quando a mensagem de exceção bruta não for suficiente para Claude agir.

O exemplo abaixo captura dois tipos de falhas dentro do manipulador e compõe a mensagem de erro que Claude lê. Um status HTTP não-200 é capturado da resposta e retornado como um resultado de erro. Um erro de rede ou JSON inválido é capturado pelo `try/except` (Python) ou `try/catch` (TypeScript) circundante e também retornado como um resultado de erro. Em ambos os casos Claude recebe uma mensagem que descreve a falha em vez de uma string de exceção bruta.

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
  Retornar imagens e recursos
</h2>

O array `content` em um resultado de ferramenta aceita blocos `text`, `image`, `audio`, `resource` e `resource_link`. Você pode misturá-los na mesma resposta. Em TypeScript, blocos de áudio são salvos em disco e Claude recebe um bloco de texto com o caminho do arquivo salvo; em Python, o SDK remove blocos de áudio do resultado da ferramenta e registra um aviso. Blocos de link de recurso são convertidos em um bloco de texto contendo o nome do link, URI e descrição.

<h3 id="images">
  Imagens
</h3>

Um bloco de imagem carrega os bytes da imagem inline, codificados como base64. Não há campo de URL. Para retornar uma imagem que vive em uma URL, busque-a no manipulador, leia os bytes da resposta e codifique-os em base64 antes de retornar. O resultado é processado como entrada visual.

| Campo      | Tipo      | Notas                                                                                  |
| :--------- | :-------- | :------------------------------------------------------------------------------------- |
| `type`     | `"image"` |                                                                                        |
| `data`     | `string`  | Bytes codificados em base64. Apenas base64 bruto, sem prefixo `data:image/...;base64,` |
| `mimeType` | `string`  | Obrigatório. Por exemplo `image/png`, `image/jpeg`, `image/webp`, `image/gif`          |

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
  Recursos
</h3>

Um bloco de recurso incorpora um pedaço de conteúdo identificado por uma URI. A URI é um rótulo para Claude referenciar; o conteúdo real fica no campo `text` ou `blob` do bloco. Use isto quando sua ferramenta produz algo que faz sentido endereçar por nome depois, como um arquivo gerado ou um registro de um sistema externo.

| Campo               | Tipo         | Notas                                                                                                                                                    |
| :------------------ | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`              | `"resource"` |                                                                                                                                                          |
| `resource.uri`      | `string`     | Identificador para o conteúdo. Qualquer esquema de URI                                                                                                   |
| `resource.text`     | `string`     | O conteúdo, se for texto. Forneça isto ou `blob`, não ambos                                                                                              |
| `resource.blob`     | `string`     | O conteúdo codificado em base64, se for binário. Apenas TypeScript: o SDK Python remove recursos binários do resultado da ferramenta e registra um aviso |
| `resource.mimeType` | `string`     | Opcional                                                                                                                                                 |

Este exemplo mostra um bloco de recurso retornado de dentro de um manipulador de ferramenta. A URI `file:///tmp/report.md` é um rótulo que Claude pode referenciar depois; o SDK não lê desse caminho.

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

Estas formas de bloco vêm do tipo MCP `CallToolResult`. Veja a [especificação MCP](https://modelcontextprotocol.io/specification/2025-06-18/server/tools#tool-result) para a definição completa.

<h2 id="return-structured-data">
  Retornar dados estruturados
</h2>

`structuredContent` é um objeto JSON opcional no resultado, separado do array `content`. Use-o para retornar valores brutos que Claude pode ler como campos exatos em vez de analisá-los de uma string de texto ou imagem.

Quando `structuredContent` é definido, Claude recebe o JSON mais quaisquer blocos de imagem ou recurso de `content`. Blocos de texto em `content` não são encaminhados, já que são assumidos duplicar os dados estruturados. O exemplo abaixo renderiza um gráfico como um bloco de imagem e retorna os pontos de dados por trás dele em `structuredContent` do mesmo manipulador.

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
  O decorador Python `@tool` encaminha apenas `content` e `is_error` do dict de retorno do manipulador. Para retornar `structuredContent` de Python, execute um [servidor MCP autônomo](/docs/pt/agent-sdk/mcp) em vez de um servidor SDK em processo.
</Note>

<h2 id="example-unit-converter">
  Exemplo: conversor de unidades
</h2>

Esta ferramenta converte valores entre unidades de comprimento, temperatura e peso. Um usuário pode perguntar "converter 100 quilômetros para milhas" ou "qual é 72°F em Celsius," e Claude escolhe o tipo de unidade certo e unidades da solicitação.

Demonstra dois padrões:

* **Esquemas de enum:** `unit_type` é restrito a um conjunto fixo de valores. Em TypeScript, use `z.enum()`. Em Python, o esquema dict não suporta enums, então o dict completo de JSON Schema é necessário.
* **Tratamento de entrada não suportada:** quando um par de conversão não é encontrado, o manipulador retorna `isError: true` para que Claude possa dizer ao usuário o que deu errado em vez de tratar uma falha como um resultado normal.

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

Uma vez que o servidor é definido, passe-o para `query` da mesma forma que o exemplo de clima. Este exemplo envia três prompts diferentes em um loop para mostrar a mesma ferramenta tratando diferentes tipos de unidades. Para cada resposta, ele inspeciona objetos `AssistantMessage` (que contêm as chamadas de ferramenta que Claude fez durante esse turno) e imprime cada `ToolUseBlock` antes de imprimir o texto final de `ResultMessage`. Isto permite que você veja quando Claude está usando a ferramenta versus respondendo de seu próprio conhecimento.

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
  Próximos passos
</h2>

Ferramentas personalizadas envolvem funções assíncronas em uma interface padrão. Você pode misturar os padrões nesta página no mesmo servidor: um único servidor pode conter uma ferramenta de banco de dados, uma ferramenta de gateway de API e um renderizador de imagem lado a lado.

A partir daqui:

* Se seu servidor crescer para dezenas de ferramentas, veja [tool search](/docs/pt/agent-sdk/tool-search) para adiar o carregamento delas até Claude precisar delas.
* Para conectar a servidores MCP externos (sistema de arquivos, GitHub, Slack) em vez de construir os seus próprios, veja [Conectar servidores MCP](/docs/pt/agent-sdk/mcp).
* Para controlar quais ferramentas executam automaticamente versus exigindo aprovação, veja [Configurar permissões](/docs/pt/agent-sdk/permissions).

<h2 id="related-documentation">
  Documentação relacionada
</h2>

* [Referência do SDK TypeScript](/docs/pt/agent-sdk/typescript)
* [Referência do SDK Python](/docs/pt/agent-sdk/python)
* [Documentação MCP](https://modelcontextprotocol.io)
* [Visão geral do SDK](/docs/pt/agent-sdk/overview)
