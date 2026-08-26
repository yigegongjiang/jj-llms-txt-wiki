> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Início Rápido

> Comece com o Agent SDK Python ou TypeScript para construir agentes de IA que funcionam autonomamente

Use o Agent SDK para construir um agente de IA que leia seu código, encontre bugs e os corrija, tudo sem intervenção manual.

**O que você fará:**

1. Configurar um projeto com o Agent SDK
2. Criar um arquivo com código com bugs
3. Executar um agente que encontra e corrige os bugs automaticamente

<h2 id="prerequisites">
  Pré-requisitos
</h2>

* **Node.js 18+** ou **Python 3.10+**
* Uma **conta Anthropic** ([inscreva-se aqui](https://platform.claude.com/))

<h2 id="setup">
  Configuração
</h2>

<Steps>
  <Step title="Criar uma pasta de projeto">
    Crie um novo diretório para este início rápido:

    ```bash theme={null}
    mkdir my-agent
    cd my-agent
    ```

    Para seus próprios projetos, você pode executar o SDK de qualquer pasta; ele terá acesso aos arquivos nesse diretório e seus subdiretórios por padrão.
  </Step>

  <Step title="Instalar o SDK">
    Instale o pacote Agent SDK para sua linguagem:

    <Tabs>
      <Tab title="TypeScript (novo projeto)">
        ```bash theme={null}
        npm init -y
        npm pkg set type=module
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        Definir `"type": "module"` em `package.json` permite que seu script de agente use `await` de nível superior, e [tsx](https://tsx.is) executa arquivos TypeScript diretamente.
      </Tab>

      <Tab title="TypeScript (projeto existente)">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        npm install --save-dev tsx
        ```

        [tsx](https://tsx.is) executa arquivos TypeScript diretamente. Se seu projeto usa CommonJS, nomeie seu script de agente como `agent.mts` em vez de `agent.ts`. A extensão `.mts` faz o tsx tratar o arquivo como um módulo ES, para que `await` de nível superior funcione sem converter todo o seu projeto para módulos ES. Use `agent.mts` no lugar de `agent.ts` nas etapas de criação e execução mais adiante neste início rápido.
      </Tab>

      <Tab title="Python (uv)">
        [uv](https://docs.astral.sh/uv/) é um gerenciador de pacotes Python rápido que lida com ambientes virtuais automaticamente:

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        Crie e ative um ambiente virtual, depois instale o pacote.

        No macOS ou Linux:

        ```bash theme={null}
        python3 -m venv .venv
        source .venv/bin/activate
        pip install claude-agent-sdk
        ```

        No Windows:

        ```powershell theme={null}
        py -m venv .venv
        .venv\Scripts\Activate.ps1
        pip install claude-agent-sdk
        ```

        Se o PowerShell bloquear `Activate.ps1` com um erro de política de execução, execute `Set-ExecutionPolicy -Scope Process RemoteSigned` primeiro.
      </Tab>
    </Tabs>

    <Note>
      O SDK TypeScript agrupa um binário nativo Claude Code para sua plataforma como uma dependência opcional, portanto você não precisa instalar Claude Code separadamente.
    </Note>
  </Step>

  <Step title="Defina sua chave de API">
    Obtenha uma chave de API no [Claude Console](https://platform.claude.com/), depois defina-a como uma variável de ambiente no shell onde você executará seu agente:

    <Tabs>
      <Tab title="macOS / Linux">
        ```bash theme={null}
        export ANTHROPIC_API_KEY=your-api-key
        ```
      </Tab>

      <Tab title="Windows (PowerShell)">
        ```powershell theme={null}
        $env:ANTHROPIC_API_KEY = "your-api-key"
        ```
      </Tab>
    </Tabs>

    O SDK lê a chave do ambiente do processo que executa seu agente; ele não carrega arquivos `.env` automaticamente. Se você mantiver a chave em um arquivo `.env`, carregue-a você mesmo, por exemplo com o pacote `dotenv`, antes de chamar o SDK.

    O SDK também suporta autenticação através de provedores de API de terceiros:

    * **Amazon Bedrock**: defina a variável de ambiente `CLAUDE_CODE_USE_BEDROCK=1` e configure as credenciais AWS
    * **Claude Platform on AWS**: defina `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` e `ANTHROPIC_AWS_WORKSPACE_ID`, depois configure as credenciais AWS
    * **Google Cloud's Agent Platform**: defina a variável de ambiente `CLAUDE_CODE_USE_VERTEX=1` e configure as credenciais Google Cloud
    * **Microsoft Azure**: defina a variável de ambiente `CLAUDE_CODE_USE_FOUNDRY=1` e configure as credenciais Azure

    Consulte os guias de configuração para [Amazon Bedrock](/docs/pt/amazon-bedrock), [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai), ou [Microsoft Foundry](/docs/pt/microsoft-foundry) para detalhes.

    <Note>
      A menos que previamente aprovado, a Anthropic não permite que desenvolvedores terceirizados ofereçam login claude.ai ou limites de taxa para seus produtos, incluindo agentes construídos no Agent SDK Claude. Use os métodos de autenticação de chave de API descritos neste documento.
    </Note>
  </Step>
</Steps>

<h2 id="create-a-buggy-file">
  Criar um arquivo com bugs
</h2>

Este início rápido o orienta na construção de um agente que pode encontrar e corrigir bugs no código. Primeiro, você precisa de um arquivo com alguns bugs intencionais para o agente corrigir. Crie `utils.py` no diretório `my-agent` e cole o seguinte código:

```python theme={null}
def calculate_average(numbers):
    total = 0
    for num in numbers:
        total += num
    return total / len(numbers)


def get_user_name(user):
    return user["name"].upper()
```

Este código tem dois bugs:

1. `calculate_average([])` falha com divisão por zero
2. `get_user_name(None)` falha com um TypeError

<h2 id="build-an-agent-that-finds-and-fixes-bugs">
  Construir um agente que encontra e corrige bugs
</h2>

Crie `agent.py` se estiver usando o SDK Python, ou `agent.ts` para TypeScript. Use `agent.mts` em vez disso se seu projeto existente usar CommonJS:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage


  async def main():
      # Agentic loop: streams messages as Claude works
      async for message in query(
          prompt="Review utils.py for bugs that would cause crashes. Fix any issues you find.",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Edit", "Glob"],  # Auto-approve these tools
              permission_mode="acceptEdits",  # Auto-approve file edits
          ),
      ):
          # Print human-readable output
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "text"):
                      print(block.text)  # Claude's reasoning
                  elif hasattr(block, "name"):
                      print(f"Tool: {block.name}")  # Tool being called
          elif isinstance(message, ResultMessage):
              print(f"Done: {message.subtype}")  # Final result


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Agentic loop: streams messages as Claude works
  for await (const message of query({
    prompt: "Review utils.py for bugs that would cause crashes. Fix any issues you find.",
    options: {
      allowedTools: ["Read", "Edit", "Glob"], // Auto-approve these tools
      permissionMode: "acceptEdits" // Auto-approve file edits
    }
  })) {
    // Print human-readable output
    if (message.type === "assistant" && message.message?.content) {
      for (const block of message.message.content) {
        if ("text" in block) {
          console.log(block.text); // Claude's reasoning
        } else if ("name" in block) {
          console.log(`Tool: ${block.name}`); // Tool being called
        }
      }
    } else if (message.type === "result") {
      console.log(`Done: ${message.subtype}`); // Final result
    }
  }
  ```
</CodeGroup>

Este código tem três partes principais:

1. **`query`**: o ponto de entrada principal que cria o loop agentic. Ele retorna um iterador assíncrono, então você usa `async for` para transmitir mensagens enquanto Claude trabalha. Veja a API completa na referência do SDK [Python](/docs/pt/agent-sdk/python#query) ou [TypeScript](/docs/pt/agent-sdk/typescript#query).

2. **`prompt`**: o que você quer que Claude faça. Claude descobre quais ferramentas usar com base na tarefa.

3. **`options`**: configuração para o agente. Este exemplo usa `allowedTools` para pré-aprovar `Read`, `Edit` e `Glob`, e `permissionMode: "acceptEdits"` para auto-aprovar alterações de arquivo. Outras opções incluem `systemPrompt`, `mcpServers` e muito mais. Veja todas as opções para [Python](/docs/pt/agent-sdk/python#claudeagentoptions) ou [TypeScript](/docs/pt/agent-sdk/typescript#options).

O loop `async for` continua executando enquanto Claude pensa, chama ferramentas, observa resultados e decide o que fazer a seguir. Cada iteração produz uma mensagem: o raciocínio de Claude, uma chamada de ferramenta, um resultado de ferramenta ou o resultado final. O SDK lida com a orquestração (execução de ferramentas, gerenciamento de contexto, tentativas) para que você apenas consuma o fluxo. O loop termina quando Claude conclui a tarefa ou encontra um erro.

O tratamento de mensagens dentro do loop filtra a saída legível por humanos. Sem filtragem, você veria objetos de mensagem brutos, incluindo inicialização do sistema e estado interno, o que é útil para depuração, mas barulhento caso contrário.

<Note>
  Este exemplo usa streaming para mostrar o progresso em tempo real. Se você não precisar de saída ao vivo (por exemplo, para trabalhos em segundo plano ou pipelines de CI), você pode coletar todas as mensagens de uma vez. Veja [Streaming vs. modo de turno único](/docs/pt/agent-sdk/streaming-vs-single-mode) para detalhes.
</Note>

<h3 id="run-your-agent">
  Execute seu agente
</h3>

Seu agente está pronto. Execute-o com o seguinte comando:

<Tabs>
  <Tab title="TypeScript">
    ```bash theme={null}
    npx tsx agent.ts
    ```

    Se você nomeou seu script como `agent.mts`, execute `npx tsx agent.mts` em vez disso.
  </Tab>

  <Tab title="Python (uv)">
    ```bash theme={null}
    uv run agent.py
    ```
  </Tab>

  <Tab title="Python (pip)">
    Com seu ambiente virtual ainda ativado:

    ```bash theme={null}
    python agent.py
    ```
  </Tab>
</Tabs>

Conforme funciona, o agente imprime seu raciocínio e cada ferramenta que chama, terminando com `Done: success`. Após executar, verifique `utils.py`. Você verá código defensivo tratando listas vazias e usuários nulos. Seu agente autonomamente:

1. **Leu** `utils.py` para entender o código
2. **Analisou** a lógica e identificou casos extremos que causariam falhas
3. **Editou** o arquivo para adicionar tratamento de erros apropriado

Isto é o que torna o Agent SDK diferente: Claude executa ferramentas diretamente em vez de pedir que você as implemente.

<Note>
  Se você vir "API key not found", certifique-se de que definiu a variável de ambiente `ANTHROPIC_API_KEY` no shell onde você executa seu agente. O SDK não carrega arquivos `.env` automaticamente. Veja o [guia completo de solução de problemas](/docs/pt/troubleshooting) para mais ajuda.
</Note>

<h3 id="try-other-prompts">
  Tente outros prompts
</h3>

Agora que seu agente está configurado, tente alguns prompts diferentes:

* `"Add docstrings to all functions in utils.py"`
* `"Add type hints to all functions in utils.py"`
* `"Create a README.md documenting the functions in utils.py"`

<h3 id="customize-your-agent">
  Personalize seu agente
</h3>

Você pode modificar o comportamento do seu agente alterando as opções. Aqui estão alguns exemplos:

**Adicionar capacidade de busca na web:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "WebSearch"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "WebSearch"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

**Dê a Claude um prompt de sistema personalizado:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob"],
      permission_mode="acceptEdits",
      system_prompt="You are a senior Python developer. Always follow PEP 8 style guidelines.",
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob"],
      permissionMode: "acceptEdits",
      systemPrompt: "You are a senior Python developer. Always follow PEP 8 style guidelines."
    }
  };
  ```
</CodeGroup>

**Execute comandos no terminal:**

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      allowed_tools=["Read", "Edit", "Glob", "Bash"], permission_mode="acceptEdits"
  )
  ```

  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      allowedTools: ["Read", "Edit", "Glob", "Bash"],
      permissionMode: "acceptEdits"
    }
  };
  ```
</CodeGroup>

Com `Bash` ativado, tente: `"Write unit tests for utils.py, run them, and fix any failures"`

<h2 id="key-concepts">
  Conceitos-chave
</h2>

**Ferramentas** controlam o que seu agente pode fazer:

| Ferramentas                            | O que o agente pode fazer   |
| -------------------------------------- | --------------------------- |
| `Read`, `Glob`, `Grep`                 | Análise somente leitura     |
| `Read`, `Edit`, `Glob`                 | Analisar e modificar código |
| `Read`, `Edit`, `Bash`, `Glob`, `Grep` | Automação completa          |

**Modos de permissão** controlam quanto de supervisão humana você deseja:

| Modo                | Comportamento                                                                                                                                                                                                                                                                                                                  | Caso de uso                                      |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------ |
| `acceptEdits`       | Auto-aprova edições de arquivo e comandos comuns do sistema de arquivos, pede outras ações                                                                                                                                                                                                                                     | Fluxos de trabalho de desenvolvimento confiáveis |
| `plan`              | Executa ferramentas somente leitura; edições de arquivo nunca são auto-aprovadas e chegam ao seu callback `canUseTool`                                                                                                                                                                                                         | Escopo de uma tarefa antes de aprovar a execução |
| `dontAsk`           | Nega qualquer coisa não em `allowedTools`; ferramentas de conector [sua organização definida como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) e ferramentas que exigem interação do usuário são negadas mesmo se você as listou                                                                                   | Agentes headless bloqueados                      |
| `auto`              | Um classificador de modelo aprova ou nega cada chamada de ferramenta                                                                                                                                                                                                                                                           | Agentes autônomos com proteções de segurança     |
| `bypassPermissions` | Executa cada ferramenta sem prompts, exceto ferramentas correspondidas por uma regra [`ask`](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) explícita, ferramentas de conector [sua organização definida como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) e ferramentas que exigem interação do usuário | CI em sandbox, ambientes totalmente confiáveis   |
| `default`           | Requer um callback `canUseTool` para lidar com aprovação                                                                                                                                                                                                                                                                       | Fluxos de aprovação personalizados               |

O exemplo acima usa o modo `acceptEdits`, que auto-aprova operações de arquivo para que o agente possa executar sem prompts interativos. Se você quiser solicitar aprovação dos usuários, use o modo `default` e forneça um callback [`canUseTool`](/docs/pt/agent-sdk/user-input) que coleta entrada do usuário. Para mais controle, veja [Permissões](/docs/pt/agent-sdk/permissions).

<h2 id="next-steps">
  Próximos passos
</h2>

Agora que você criou seu primeiro agente, aprenda como estender suas capacidades e adaptá-lo ao seu caso de uso:

* **[Permissões](/docs/pt/agent-sdk/permissions)**: controle o que seu agente pode fazer e quando precisa de aprovação
* **[Hooks](/docs/pt/agent-sdk/hooks)**: execute código personalizado antes ou depois de chamadas de ferramenta
* **[Sessões](/docs/pt/agent-sdk/sessions)**: construa agentes multi-turno que mantêm contexto
* **[Servidores MCP](/docs/pt/agent-sdk/mcp)**: conecte-se a bancos de dados, navegadores, APIs e outros sistemas externos
* **[Hospedagem](/docs/pt/agent-sdk/hosting)**: implante agentes no Docker, nuvem e CI/CD
* **[Agentes de exemplo](https://github.com/anthropics/claude-agent-sdk-demos)**: veja exemplos completos: assistente de email, agente de pesquisa e muito mais
