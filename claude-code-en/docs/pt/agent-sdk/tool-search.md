> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dimensione para muitas ferramentas com busca de ferramentas

> Dimensione seu agente para milhares de ferramentas descobrindo e carregando apenas o que é necessário, sob demanda.

A busca de ferramentas permite que seu agente trabalhe com centenas ou milhares de ferramentas descobrindo e carregando-as dinamicamente sob demanda. Em vez de carregar todas as definições de ferramentas na janela de contexto antecipadamente, o agente pesquisa seu catálogo de ferramentas e carrega apenas as ferramentas de que precisa.

Esta abordagem resolve dois desafios conforme as bibliotecas de ferramentas se dimensionam:

* **Eficiência de contexto:** As definições de ferramentas podem consumir grandes porções da janela de contexto (50 ferramentas podem usar 10-20K tokens), deixando menos espaço para o trabalho real.
* **Precisão da seleção de ferramentas:** A precisão da seleção de ferramentas se degrada com mais de 30-50 ferramentas carregadas simultaneamente.

A busca de ferramentas está ativada por padrão.

<h2 id="how-tool-search-works">
  Como funciona a busca de ferramentas
</h2>

Quando a busca de ferramentas está ativa, as definições de ferramentas são retidas da janela de contexto. O agente recebe um resumo das ferramentas disponíveis e pesquisa as relevantes quando a tarefa requer uma capacidade não carregada. Até cinco das ferramentas mais relevantes são carregadas no contexto por padrão, onde permanecem disponíveis para turnos subsequentes. Se a conversa for longa o suficiente para que o SDK compacte mensagens anteriores para liberar espaço, as ferramentas descobertas anteriormente podem ser removidas e o agente pesquisa novamente conforme necessário.

A busca de ferramentas adiciona uma viagem extra de ida e volta na primeira vez que Claude descobre uma ferramenta (a etapa de pesquisa), mas para grandes conjuntos de ferramentas isso é compensado por um contexto menor a cada turno. Com menos de \~10 ferramentas, carregar tudo antecipadamente é geralmente mais rápido.

Para detalhes sobre o mecanismo de API subjacente, consulte [Busca de ferramentas na API](https://platform.claude.com/docs/pt/agents-and-tools/tool-use/tool-search-tool).

<Note>
  A busca de ferramentas é suportada em Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 e modelos posteriores; consulte [compatibilidade de modelos na documentação da API](https://platform.claude.com/docs/pt/agents-and-tools/tool-use/tool-search-tool#model-compatibility) para a lista atual. Na Agent Platform do Google Cloud, os modelos mínimos suportados são Claude Sonnet 4.5 e Claude Opus 4.5.
</Note>

<h2 id="configure-tool-search">
  Configurar busca de ferramentas
</h2>

A busca de ferramentas está ativada por padrão. Está desativada por padrão no Google Cloud's Agent Platform, onde é suportada para Claude Sonnet 4.5 e posterior e Claude Opus 4.5 e posterior. Também está desativada quando `ANTHROPIC_BASE_URL` aponta para um host não de primeira parte, já que a maioria dos proxies não encaminha blocos `tool_reference`. Você pode substituir qualquer padrão com a variável de ambiente `ENABLE_TOOL_SEARCH`:

| Valor          | Comportamento                                                                                                                                                                                                                                                                                      |
| :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (não definido) | A busca de ferramentas está ativada. As definições de ferramentas são adiadas e descobertas sob demanda. Volta para carregamento antecipado no Google Cloud's Agent Platform ou em um `ANTHROPIC_BASE_URL` não de primeira parte.                                                                  |
| `true`         | A busca de ferramentas está sempre ativada. O SDK envia o cabeçalho beta mesmo no Google Cloud's Agent Platform e através de proxies. As solicitações falham em modelos Google Cloud's Agent Platform anteriores a Sonnet 4.5 ou Opus 4.5, ou em proxies que não suportam blocos `tool_reference`. |
| `auto`         | Verifica a contagem de tokens combinada de todas as definições de ferramentas em relação à janela de contexto do modelo. Se excederem 10%, a busca de ferramentas é ativada. Se estiverem abaixo de 10%, todas as ferramentas são carregadas no contexto normalmente.                              |
| `auto:N`       | O mesmo que `auto` com uma porcentagem personalizada. `auto:5` ativa quando as definições de ferramentas excedem 5% da janela de contexto. Valores mais baixos ativam mais cedo.                                                                                                                   |
| `false`        | A busca de ferramentas está desativada. Todas as definições de ferramentas são carregadas no contexto a cada turno.                                                                                                                                                                                |

Definir [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/pt/env-vars) mantém a busca de ferramentas desativada, e `ENABLE_TOOL_SEARCH` não pode substituí-la. A variável remove o cabeçalho beta que as definições de ferramentas `defer_loading` e blocos de conteúdo `tool_reference` exigem.

A busca de ferramentas se aplica a todas as ferramentas registradas, sejam elas provenientes de servidores MCP remotos ou [servidores MCP SDK personalizados](/docs/pt/agent-sdk/custom-tools). Ao usar `auto`, o limite é baseado no tamanho combinado de todas as definições de ferramentas em todos os servidores.

Defina o valor na opção `env` em `query()`. Em TypeScript, `env` substitui o ambiente do subprocesso, portanto espalhe `...process.env` para manter as variáveis herdadas. Em Python, `env` é mesclado no topo do ambiente herdado. Este exemplo se conecta a um servidor MCP remoto que expõe muitas ferramentas, pré-aprova todas elas com um curinga e usa `auto:5` para que a busca de ferramentas seja ativada quando suas definições excedem 5% da janela de contexto:

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
          ENABLE_TOOL_SEARCH: "auto:5" // Activate tool search when tools exceed 5% of context
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
              "ENABLE_TOOL_SEARCH": "auto:5"  # Activate tool search when tools exceed 5% of context
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

Definir `ENABLE_TOOL_SEARCH` como `"false"` desativa a busca de ferramentas e carrega todas as definições de ferramentas no contexto a cada turno. Isso remove a viagem de pesquisa, que pode ser mais rápida quando o conjunto de ferramentas é pequeno (menos de \~10 ferramentas) e as definições cabem confortavelmente na janela de contexto.

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
* **Suporte de modelo:** Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 e modelos posteriores; consulte [compatibilidade de modelo na documentação da API](https://platform.claude.com/docs/pt/agents-and-tools/tool-use/tool-search-tool#model-compatibility) para a lista atual. Na Agent Platform do Google Cloud, Claude Sonnet 4.5 e posteriores e Claude Opus 4.5 e posteriores.

<h2 id="related-documentation">
  Documentação relacionada
</h2>

* [Busca de ferramentas na API](https://platform.claude.com/docs/pt/agents-and-tools/tool-use/tool-search-tool): Documentação completa da API para busca de ferramentas, incluindo implementações personalizadas
* [Conectar servidores MCP](/docs/pt/agent-sdk/mcp): Conecte-se a ferramentas externas via servidores MCP
* [Ferramentas personalizadas](/docs/pt/agent-sdk/custom-tools): Crie suas próprias ferramentas com servidores MCP SDK
* [Referência do SDK TypeScript](/docs/pt/agent-sdk/typescript): Referência completa da API
* [Referência do SDK Python](/docs/pt/agent-sdk/python): Referência completa da API
