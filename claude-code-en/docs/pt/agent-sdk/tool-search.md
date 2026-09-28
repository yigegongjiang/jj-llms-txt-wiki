> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dimensione para muitas ferramentas com busca de ferramentas

> Dimensione seu agente para milhares de ferramentas descobrindo e carregando apenas o que é necessário, sob demanda.

A busca de ferramentas permite que seu agente trabalhe com centenas ou milhares de ferramentas descobrindo e carregando-as dinamicamente sob demanda. Em vez de carregar todas as definições de ferramentas na janela de contexto antecipadamente, o agente pesquisa seu catálogo de ferramentas e carrega apenas as ferramentas de que precisa.

Esta abordagem resolve dois desafios conforme as bibliotecas de ferramentas se dimensionam:

* **Eficiência de contexto:** As definições de ferramentas podem consumir grandes porções da janela de contexto (50 ferramentas podem usar 10-20K tokens), deixando menos espaço para o trabalho real.
* **Precisão da seleção de ferramentas:** A precisão da seleção de ferramentas se degrada com mais de 30-50 ferramentas carregadas simultaneamente.

<h2 id="how-tool-search-works">
  Como funciona a busca de ferramentas
</h2>

A busca de ferramentas está ativada por padrão, com as exceções listadas em [Configurar busca de ferramentas](#configure-tool-search).

Quando está ativa, as definições de ferramentas são retidas da janela de contexto. O agente recebe um resumo das ferramentas disponíveis e pesquisa as relevantes quando a tarefa requer uma capacidade não carregada. Até cinco das ferramentas mais relevantes são carregadas no contexto por padrão, onde permanecem disponíveis para turnos subsequentes até que o SDK compacte as mensagens onde o agente as descobriu. Após essa compactação, o agente pesquisa essas ferramentas novamente quando precisar delas.

A busca de ferramentas adiciona uma viagem extra de ida e volta cada vez que Claude pesquisa ferramentas, mas para grandes conjuntos de ferramentas isso é compensado por um contexto menor a cada turno. Com menos de \~10 ferramentas cujas definições se encaixam confortavelmente na janela de contexto, carregar tudo antecipadamente é geralmente mais rápido.

Para detalhes sobre o mecanismo de API subjacente, consulte [Busca de ferramentas na API](https://platform.claude.com/docs/pt/agents-and-tools/tool-use/tool-search-tool).

<Note>
  A busca de ferramentas não é suportada em implantações do Microsoft Foundry [hospedadas no Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options), que a rejeitam no lado do servidor: o SDK detecta a rejeição e carrega as definições de ferramentas antecipadamente para essa implantação. [`ENABLE_TOOL_SEARCH`](#configure-tool-search) não pode substituir isso, pois a rejeição vem da implantação em si.
</Note>

<h2 id="configure-tool-search">
  Configurar busca de ferramentas
</h2>

A busca de ferramentas está ativada por padrão. Para modelos na lista de modelos não suportados do SDK, o SDK carrega as definições de ferramentas antecipadamente, e nenhum valor `ENABLE_TOOL_SEARCH` substitui isso. Na Agent Platform do Google Cloud, o SDK decide por geração de modelo:

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5 e posterior**: a busca de ferramentas está ativada por padrão.
* **Modelos anteriores da Agent Platform**: o SDK carrega as definições de ferramentas antecipadamente, porque suas pilhas de serviço rejeitam o cabeçalho beta necessário. `ENABLE_TOOL_SEARCH` não pode substituir isso.

Antes do Claude Code v2.1.221, o SDK desativava a busca de ferramentas para todos os modelos na Agent Platform do Google Cloud, a menos que você definisse `ENABLE_TOOL_SEARCH`.

O SDK também desativa a busca de ferramentas quando `ANTHROPIC_BASE_URL` aponta para um host não de primeira parte, já que a maioria dos proxies não encaminha blocos `tool_reference`. Você pode substituir esse padrão com a variável de ambiente `ENABLE_TOOL_SEARCH`:

| Valor          | Comportamento                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (não definido) | A busca de ferramentas está ativada. As definições de ferramentas são adiadas e descobertas sob demanda. Volta para carregamento antecipado nos modelos da Agent Platform do Google Cloud anteriores à geração Claude 4.5, um `ANTHROPIC_BASE_URL` não de primeira parte ou uma implantação do Microsoft Foundry hospedada no Azure.                                                                                                                                                   |
| `true`         | A busca de ferramentas está sempre ativada, exceto em uma implantação do Microsoft Foundry hospedada no Azure, onde a rejeição do lado do servidor ainda força o carregamento antecipado, e nos modelos da Agent Platform do Google Cloud anteriores à geração Claude 4.5, onde o SDK continua carregando as definições de ferramentas antecipadamente. O SDK envia o cabeçalho beta através de proxies, e as solicitações falham em proxies que não suportam blocos `tool_reference`. |
| `auto`         | Conta os tokens nas definições de ferramentas que a busca de ferramentas pode adiar e compara o total com a janela de contexto do modelo. Quando o total atinge 10% da janela, a busca de ferramentas é ativada. Abaixo disso, o SDK carrega todas as definições de ferramentas no contexto antecipadamente.                                                                                                                                                                           |
| `auto:N`       | O mesmo que `auto` com uma porcentagem personalizada. `auto:5` ativa quando essas definições atingem 5% da janela de contexto. Valores mais baixos ativam mais cedo.                                                                                                                                                                                                                                                                                                                   |
| `false`        | A busca de ferramentas está desativada. Todas as definições de ferramentas são carregadas no contexto a cada turno.                                                                                                                                                                                                                                                                                                                                                                    |

Definir [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/pt/env-vars) mantém a busca de ferramentas desativada. Você não pode substituí-la definindo `ENABLE_TOOL_SEARCH` você mesmo. Sua organização pode manter a busca de ferramentas ativada através de [configurações gerenciadas](/docs/pt/managed-settings), no Claude Code v2.1.227 ou posterior. [Desabilitar capacidades de pré-lançamento](/docs/pt/llm-gateway-protocol#disable-pre-release-capabilities) cobre onde a substituição se aplica e o que a variável remove.

A busca de ferramentas se aplica a todas as ferramentas registradas, sejam elas provenientes de servidores MCP remotos ou [servidores MCP SDK personalizados](/docs/pt/agent-sdk/custom-tools). Quando você usa `auto`, o SDK conta todas as definições que a busca de ferramentas pode adiar em relação a um limite combinado: cada ferramenta MCP que não está marcada como [`alwaysLoad`](/docs/pt/mcp#exempt-a-server-from-deferral), de qualquer servidor, mais as ferramentas integradas que carregam sob demanda. O SDK sempre carrega ferramentas integradas principais, como Bash, Read e Edit antecipadamente e não as conta em relação ao limite.

Defina o valor na opção `env` em `query()`. Em TypeScript, `env` substitui o ambiente do subprocesso, portanto espalhe `...process.env` para manter as variáveis herdadas. Em Python, `env` é mesclado no topo do ambiente herdado. Este exemplo se conecta a um servidor MCP remoto que expõe muitas ferramentas, pré-aprova todas elas com um curinga e usa `auto:5` para que a busca de ferramentas seja ativada quando as definições que ela pode adiar atingem 5% da janela de contexto:

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

Para executar este exemplo, substitua `https://tools.example.com/mcp` pela URL do seu próprio servidor MCP. Em caso de sucesso, o texto do resultado é impresso no console.

Como esta é uma chamada `query()` de uma única vez, o SDK lança uma exceção após gerar um resultado de erro, portanto o exemplo envolve o loop em um bloco try. Para ver por que uma execução falhou, verifique o `subtype` da mensagem de resultado, como `error_during_execution`, dentro do loop. Para mais informações sobre mensagens de resultado, consulte [Lidar com o resultado](/docs/pt/agent-sdk/agent-loop#handle-the-result).

<h2 id="optimize-tool-discovery">
  Otimizar descoberta de ferramentas
</h2>

O mecanismo de pesquisa corresponde consultas com nomes e descrições de ferramentas. Nomes como `search_slack_messages` aparecem para uma gama mais ampla de solicitações do que `query_slack`. Descrições com palavras-chave específicas ("Pesquisar mensagens do Slack por palavra-chave, canal ou intervalo de datas") correspondem a mais consultas do que genéricas ("Consultar Slack").

Você também pode adicionar uma seção de prompt do sistema listando categorias de ferramentas disponíveis. Isso dá ao agente contexto sobre que tipos de ferramentas estão disponíveis para pesquisar. Passe o texto através da opção `systemPrompt` em TypeScript ou `system_prompt` em Python, usando a predefinição `claude_code` com `append`, que adiciona seu texto ao prompt da predefinição em vez de substituí-lo:

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

Para o conjunto completo de opções de prompt do sistema, consulte [Modificando prompts do sistema](/docs/pt/agent-sdk/modifying-system-prompts).

<h2 id="limits">
  Limites
</h2>

* **Ferramentas máximas:** 10.000 ferramentas em seu catálogo
* **Resultados de pesquisa:** retorna até cinco ferramentas mais relevantes por pesquisa por padrão
* **Suporte de modelo:** Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 e modelos posteriores; consulte [compatibilidade de modelo na documentação da API](https://platform.claude.com/docs/pt/agents-and-tools/tool-use/tool-search-tool#model-compatibility) para a lista atual. O mesmo se aplica na Agent Platform do Google Cloud.

<h2 id="related-documentation">
  Documentação relacionada
</h2>

* [Busca de ferramentas na API](https://platform.claude.com/docs/pt/agents-and-tools/tool-use/tool-search-tool): Documentação completa da API para busca de ferramentas, incluindo implementações personalizadas
* [Conectar servidores MCP](/docs/pt/agent-sdk/mcp): Conecte-se a ferramentas externas via servidores MCP
* [Ferramentas personalizadas](/docs/pt/agent-sdk/custom-tools): Crie suas próprias ferramentas com servidores MCP SDK
* [Referência do SDK TypeScript](/docs/pt/agent-sdk/typescript): Referência completa da API
* [Referência do SDK Python](/docs/pt/agent-sdk/python): Referência completa da API
