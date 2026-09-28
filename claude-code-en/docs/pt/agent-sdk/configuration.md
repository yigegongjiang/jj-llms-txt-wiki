> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configure seu agente

> Configure sessões do Agent SDK: componha o objeto de opções, defina o modelo, ambiente e limites, e encontre a página de cada opção de recurso.

Uma sessão do Agent SDK lê a configuração de arquivos de configuração, variáveis de ambiente e do objeto `options` que você passa ao iniciá-la. Esta página mostra como compor o objeto `options` e quais arquivos de configuração e variáveis de ambiente o controlam.

Para cada tipo de opção e padrão, consulte as referências [`Options`](/docs/pt/agent-sdk/typescript#options) (TypeScript) e [`ClaudeAgentOptions`](/docs/pt/agent-sdk/python#claudeagentoptions) (Python).

<h2 id="pass-options-to-a-session">
  Passar opções para uma sessão
</h2>

Cada chamada `query()` aceita um objeto de opções: `Options` em TypeScript, `ClaudeAgentOptions` em Python. Cada campo é opcional, e uma sessão iniciada sem opções é executada com os padrões do SDK. O exemplo abaixo configura uma sessão somente leitura que resume os TODOs abertos de um projeto. Os pares são lidos como TypeScript / Python onde as grafias diferem:

* **`model`**: escolhe o modelo
* **`allowedTools` / `allowed_tools`**: pré-aprova uma lista de ferramentas somente leitura
* **`maxTurns` / `max_turns`**: limita a contagem de turnos
* **`cwd`**: define o diretório de trabalho

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Summarize the open TODOs in this repo",
    options: {
      model: "claude-sonnet-5",
      allowedTools: ["Read", "Glob", "Grep"],
      maxTurns: 8,
      cwd: "/path/to/repo",
    },
  })) {
    if (message.type === "result" && message.subtype === "success" && !message.is_error) {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query

  async def main():
      options = ClaudeAgentOptions(
          model="claude-sonnet-5",
          allowed_tools=["Read", "Glob", "Grep"],
          max_turns=8,
          cwd="/path/to/repo",
      )

      async for message in query(
          prompt="Summarize the open TODOs in this repo",
          options=options,
      ):
          if isinstance(message, ResultMessage) and not message.is_error:
              print(message.result)

  asyncio.run(main())
  ```
</CodeGroup>

Aponte `cwd` para um de seus próprios projetos e execute o exemplo. O resumo dos TODOs abertos desse projeto é impresso quando a mensagem de resultado chega.

`allowedTools` (TypeScript) ou `allowed_tools` (Python) pré-aprova as ferramentas listadas, portanto as chamadas para elas são executadas sem parar para aprovação. As ferramentas fora da lista permanecem disponíveis. Quando Claude chama uma ferramenta não listada, o modo de permissão decide se a chamada é executada. Para mais informações, consulte [Regras de permissão e negação](/docs/pt/agent-sdk/permissions#allow-and-deny-rules).

<h2 id="load-settings-files">
  Carregar arquivos de configuração
</h2>

Os arquivos de configuração fornecem configuração além do objeto de opções. Duas opções controlam como eles são carregados:

* **`settingSources` / `setting_sources`**: controla quais fontes do sistema de arquivos são carregadas: usuário, projeto e local. Os arquivos de configuração e arquivos CLAUDE.md chegam através dessas fontes.
* **`settings`**: carrega um caminho de arquivo de configuração ou uma string JSON embutida em qualquer idioma, e TypeScript também aceita um objeto de configuração. Qualquer forma que você passar substitui as configurações do sistema de arquivos do usuário, projeto e local; apenas as configurações de política gerenciada têm classificação mais alta. As referências documentam a ordem de precedência completa em [Precedência de configurações](/docs/pt/agent-sdk/typescript#settings-precedence) para TypeScript e [Precedência de configurações](/docs/pt/agent-sdk/python#settings-precedence) para Python.

Passe `[]` para desabilitar as configurações do usuário, projeto e local. Para mais informações, consulte [Usar recursos do Claude Code no SDK](/docs/pt/agent-sdk/claude-code-features).

<h2 id="choose-a-model">
  Escolher um modelo
</h2>

A menos que a opção `model`, suas configurações ou seu ambiente selecionem um modelo, uma nova sessão é iniciada no [modelo padrão do Claude Code](/docs/pt/model-config#default-model-setting). Para a ordem dessas fontes, consulte [Definir seu modelo](/docs/pt/model-config#setting-your-model). Defina `model` para fixar um modelo específico ou para escolher um menor para agentes mais rápidos e baratos. O valor aceita um alias de modelo ou um nome de modelo completo; os aliases e as versões que eles resolvem estão listados em [Aliases de modelo](/docs/pt/model-config#model-aliases).

Defina `fallbackModel` (TypeScript) ou `fallback_model` (Python) para nomear um modelo de backup. Quando o primário está sobrecarregado ou indisponível, a sessão muda para o backup. O primário é retentado no início de cada turno do usuário, portanto a sessão retorna a ele assim que a interrupção passa.

Em qualquer idioma, a opção aceita um único modelo ou uma lista separada por vírgulas de backups. Para a ordem e o limite da cadeia, consulte [Cadeias de modelo de fallback](/docs/pt/model-config#fallback-model-chains). Em TypeScript, um fallback igual a `model` lança um erro na inicialização.

Os exemplos abaixo mostram uma lista de fallback em TypeScript e um único fallback em Python:

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    model: "claude-fable-5",
    fallbackModel: "claude-opus-5,claude-sonnet-5",
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      model="claude-fable-5",
      fallback_model="claude-opus-5",
  )
  ```
</CodeGroup>

<span id="sampling-parameters" />

<Note>
  Os parâmetros de solicitação da [API de Mensagens](https://platform.claude.com/docs/en/api/messages) `temperature`, `top_p` e `max_tokens` não têm campos no objeto de opções em nenhum idioma. Defina o [nível de esforço](/docs/pt/agent-sdk/agent-loop#effort-level) ou um [limite de gastos](#limit-turns-and-spend) em vez disso, ou chame a API de Mensagens quando você precisar desses parâmetros diretamente.
</Note>

<h2 id="set-environment-variables">
  Definir variáveis de ambiente
</h2>

A opção `env` define variáveis de ambiente para o processo Claude Code que executa sua sessão. Se seus valores substituem o ambiente herdado ou se mesclam com ele difere por idioma:

* **TypeScript**: `env` substitui o ambiente do subprocesso
* **Python**: o SDK mescla seus valores sobre o ambiente herdado, e seus valores substituem os herdados

Em TypeScript, espalhe `process.env` em `env` para manter variáveis herdadas como `PATH`, `HOME` e `ANTHROPIC_API_KEY`. Quando você deixa `env` indefinido, o subprocesso herda seu ambiente em ambos os idiomas.

O exemplo roteia o tráfego de API através de um gateway definindo `ANTHROPIC_BASE_URL`.

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    env: { ...process.env, ANTHROPIC_BASE_URL: "https://gateway.example.com" },
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={"ANTHROPIC_BASE_URL": "https://gateway.example.com"},
  )
  ```
</CodeGroup>

As variáveis que você passa também podem configurar o próprio Claude Code. Para as variáveis que o processo Claude Code lê, consulte [Variáveis de ambiente](/docs/pt/env-vars). Para ajustar os tempos limite de API e detecção de travamento dessa forma, siga a seção Lidar com respostas de API lentas ou travadas na referência [TypeScript](/docs/pt/agent-sdk/typescript#handle-slow-or-stalled-api-responses) ou na referência [Python](/docs/pt/agent-sdk/python#handle-slow-or-stalled-api-responses).

<h2 id="set-the-working-directory">
  Definir o diretório de trabalho
</h2>

Defina `cwd` para executar a sessão em um diretório específico. Quando você deixa `cwd` indefinido, a sessão é executada no diretório de trabalho do seu processo. Nenhum SDK tem um setter para `cwd`. Para executar em um diretório diferente, inicie outra sessão com esse `cwd`.

Claude Code lê o diretório de trabalho para determinar:

* **Configurações e hooks do projeto**: qual [configuração e hooks do projeto são carregados](/docs/pt/agent-sdk/claude-code-features)
* **Skills**: onde [as skills da sessão são descobertas](/docs/pt/agent-sdk/skills)
* **Armazenamento de sessão**: qual projeto uma [sessão armazenada pertence](/docs/pt/agent-sdk/session-storage)

Para permitir que as ferramentas acessem arquivos fora do diretório de trabalho, adicione caminhos com `additionalDirectories` (TypeScript) ou `add_dirs` (Python). Para o escopo dessa concessão, consulte [Diretórios adicionais concedem acesso a arquivos, não configuração](/docs/pt/permissions#additional-directories-grant-file-access-not-configuration).

<h2 id="limit-turns-and-spend">
  Limitar turnos e gastos
</h2>

Limite turnos e gastos com `maxTurns` / `max_turns` e `maxBudgetUsd` / `max_budget_usd`. Ambos os limites estão desativados quando indefinidos. Quando uma sessão atinge um limite, a execução termina com uma mensagem de resultado cujo subtipo nomeia o limite, `error_max_turns` ou `error_max_budget_usd`. O que acontece a seguir difere por modo de entrada:

* **`query()` de disparo único**: o SDK produz o resultado do limite e depois lança, portanto envolva o loop em um bloco try para continuar além do erro
* **Entrada de streaming**: a sessão permanece viva além de um resultado de limite, e a contagem de turnos máximos recomeça para cada mensagem enfileirada. O total do orçamento se acumula entre mensagens, e uma vez que o gasto atinge o limite, mensagens posteriores na mesma conversa terminam com o mesmo resultado de orçamento. Um [`/clear`](/docs/pt/agent-sdk/cost-tracking) reinicia o orçamento

Os dois limites tratam `0` de forma diferente:

* **`maxTurns` / `max_turns`**: `0` executa a sessão sem um limite de turnos, o mesmo que deixar a opção indefinida
* **`maxBudgetUsd` / `max_budget_usd`**: a CLI rejeita `0` como um valor inválido na inicialização, e a sessão nunca é executada

Para mais informações sobre ambos os limites, incluindo gastos de subagentes, consulte [Turnos e orçamento](/docs/pt/agent-sdk/agent-loop#turns-and-budget).

<h2 id="change-configuration-mid-session">
  Alterar configuração no meio da sessão
</h2>

Quando você inicia uma sessão com [entrada de streaming](/docs/pt/agent-sdk/streaming-vs-single-mode), você pode alternar seu modelo e modo de permissão enquanto ela é executada. Onde você chama os setters difere por idioma:

* **TypeScript**: métodos no objeto que `query()` retorna
* **Python**: métodos em [`ClaudeSDKClient`](/docs/pt/agent-sdk/python#claudesdkclient), já que `query()` retorna um iterador simples sem métodos de controle

Ambos os idiomas têm os mesmos setters:

* **`setModel()` / `set_model()`**: alterna o modelo. Chame-o sem modelo para alternar para o [modelo padrão do Claude Code](/docs/pt/model-config#default-model-setting) em vez do `model` que você passou nas opções.
* **`setPermissionMode()` / `set_permission_mode()`**: alterna o modo de permissão

TypeScript também tem `applyFlagSettings()` e `updateSettings()`:

* **`applyFlagSettings()`**: aplica configurações em tempo de execução, como em `await session.applyFlagSettings({ effortLevel: "high" })`. O método aceita chaves de arquivo de configuração em vez de campos de opções, portanto verifique a referência [`applyFlagSettings()`](/docs/pt/agent-sdk/typescript#applyflagsettings) para o esquema e para quais chaves têm efeito no meio da sessão.
* **`updateSettings()`**: escreve uma chave na lista de permissões para um arquivo de configuração. A referência [`updateSettings()`](/docs/pt/agent-sdk/typescript#updatesettings) nomeia a chave que cada fonte aceita e o piso de versão.
  * Passe `"localSettings"` para escrever o arquivo de configurações locais do projeto, como em `await session.updateSettings("localSettings", { outputStyle: "Explanatory" })`. A chave escrita tem efeito na próxima solicitação da sessão e persiste para sessões posteriores que carregam configurações `local`.
  * Passe `"userSettings"` para escrever `effortLevel`, a única chave que a fonte aceita. Claude Code a salva como o nível de esforço padrão para o modelo atual da sessão, e o esforço da sessão em execução não muda.

O exemplo abaixo executa uma sessão de dois turnos, altera a configuração entre os turnos e imprime o modelo que respondeu cada turno. Em TypeScript, o fluxo de prompt mantém a segunda mensagem até que os setters tenham sido executados, e o segundo turno é executado no novo modelo.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SDKUserMessage } from "@anthropic-ai/claude-agent-sdk";

  function userMessage(text: string): SDKUserMessage {
    return { type: "user", message: { role: "user", content: text }, parent_tool_use_id: null };
  }

  // Hold the second prompt until the setters have run.
  let startSecondTurn!: () => void;
  const secondTurnReady = new Promise<void>((resolve) => {
    startSecondTurn = resolve;
  });

  async function* turnPrompts(): AsyncGenerator<SDKUserMessage, void> {
    yield userMessage("Reply with exactly: ready");
    await secondTurnReady;
    yield userMessage("Reply with exactly: done");
  }

  const session = query({
    prompt: turnPrompts(),
    options: {
      model: "claude-sonnet-5",
    },
  });

  let turnModel = "";
  let completedTurns = 0;

  for await (const message of session) {
    if (message.type === "assistant") {
      turnModel = message.message.model;
    } else if (message.type === "result") {
      completedTurns += 1;
      if (completedTurns === 1) {
        console.log(`First turn model: ${turnModel}`);
        await session.setModel("claude-opus-5");
        await session.setPermissionMode("acceptEdits");
        startSecondTurn();
      } else {
        console.log(`Second turn model: ${turnModel}`);
        break;
      }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import AssistantMessage, ClaudeAgentOptions, ClaudeSDKClient

  async def main():
      options = ClaudeAgentOptions(model="claude-sonnet-5")

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Reply with exactly: ready")
          first_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  first_model = message.model

          await client.set_model("claude-opus-5")
          await client.set_permission_mode("acceptEdits")

          await client.query("Reply with exactly: done")
          second_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  second_model = message.model

      print(f"First turn model: {first_model}")
      print(f"Second turn model: {second_model}")

  asyncio.run(main())
  ```
</CodeGroup>

Na API Claude, o programa imprime `First turn model: claude-sonnet-5`, depois `Second turn model: claude-opus-5` após a mudança.

<Note>
  Cada modelo tem seu próprio cache de prompt, portanto após uma mudança no meio da sessão a próxima solicitação recomputa a conversa completa sem cache nas taxas do novo modelo. Para mais informações, consulte [Alternando modelos](/docs/pt/prompt-caching#switching-models).
</Note>

<h2 id="configure-specific-features">
  Configurar recursos específicos
</h2>

A tabela abaixo mapeia cada opção para o recurso que ela configura. Para opções que esta página não cobre, consulte as referências [TypeScript](/docs/pt/agent-sdk/typescript#options) e [Python](/docs/pt/agent-sdk/python#claudeagentoptions). Se você conhece seu objetivo mas não qual opção o serve, comece em [Escolher o recurso certo](/docs/pt/agent-sdk/claude-code-features#choose-the-right-feature).

| TypeScript                | Python                      | Controla                                                  | Coberto em                                                                                                                                                                                                            |
| ------------------------- | --------------------------- | --------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionMode`          | `permission_mode`           | O que o agente pode fazer sem aprovação                   | [Configurar permissões](/docs/pt/agent-sdk/permissions)                                                                                                                                                                    |
| `allowedTools`            | `allowed_tools`             | Quais chamadas de ferramentas são pré-aprovadas           | [Configurar permissões](/docs/pt/agent-sdk/permissions)                                                                                                                                                                    |
| `canUseTool`              | `can_use_tool`              | Seu callback de aprovação para chamadas de ferramentas    | [Lidar com solicitações de aprovação de ferramentas](/docs/pt/agent-sdk/user-input#handle-tool-approval-requests)                                                                                                          |
| `systemPrompt`            | `system_prompt`             | As instruções do agente                                   | [Modificando prompts do sistema](/docs/pt/agent-sdk/modifying-system-prompts)                                                                                                                                              |
| `settingSources`          | `setting_sources`           | Quais configurações do sistema de arquivos são carregadas | [Usar recursos do Claude Code no SDK](/docs/pt/agent-sdk/claude-code-features)                                                                                                                                             |
| `mcpServers`              | `mcp_servers`               | Servidores de ferramentas externas                        | [Conectar a ferramentas externas com MCP](/docs/pt/agent-sdk/mcp)                                                                                                                                                          |
| `agents`                  | `agents`                    | Definições de subagentes                                  | [Subagentes](/docs/pt/agent-sdk/subagents)                                                                                                                                                                                 |
| `hooks`                   | `hooks`                     | Callbacks em pontos do ciclo de vida                      | [Hooks](/docs/pt/agent-sdk/hooks)                                                                                                                                                                                          |
| `skills`                  | `skills`                    | Quais skills são carregadas                               | [Estender agentes com skills](/docs/pt/agent-sdk/skills)                                                                                                                                                                   |
| `plugins`                 | `plugins`                   | Quais plugins são carregados                              | [Plugins](/docs/pt/agent-sdk/plugins)                                                                                                                                                                                      |
| `outputFormat`            | `output_format`             | Esquemas de saída estruturada                             | [Saídas estruturadas](/docs/pt/agent-sdk/structured-outputs)                                                                                                                                                               |
| `resume`                  | `resume`                    | Continuando uma sessão armazenada                         | [Sessões](/docs/pt/agent-sdk/sessions)                                                                                                                                                                                     |
| `forkSession`             | `fork_session`              | Ramificando uma sessão                                    | [Sessões](/docs/pt/agent-sdk/sessions)                                                                                                                                                                                     |
| `sessionStore`            | `session_store`             | Persistência de sessão externa                            | [Armazenamento de sessão](/docs/pt/agent-sdk/session-storage)                                                                                                                                                              |
| `enableFileCheckpointing` | `enable_file_checkpointing` | Edições de arquivo rebobináveis                           | [Checkpointing de arquivo](/docs/pt/agent-sdk/file-checkpointing)                                                                                                                                                          |
| `effort`                  | `effort`                    | Quanto trabalho Claude coloca nas respostas               | [Nível de esforço](/docs/pt/agent-sdk/agent-loop#effort-level)                                                                                                                                                             |
| `sandbox`                 | `sandbox`                   | Comportamento de sandbox para execução de ferramentas     | [TypeScript](/docs/pt/agent-sdk/typescript#sandbox-configuration) e referências [Python](/docs/pt/agent-sdk/python#sandbox-configuration), com contexto de implantação em [Implantação segura](/docs/pt/agent-sdk/secure-deployment) |

<h2 id="next-steps">
  Próximas etapas
</h2>

Para ver a configuração composta em agentes funcionais:

* **[Quickstart](/docs/pt/agent-sdk/quickstart)**: construa e execute um primeiro agente de ponta a ponta
* **[Exemplos](/docs/pt/agent-sdk/examples)**: encontre um projeto completo e executável ou uma receita guiada do Claude Cookbook que corresponda ao que você deseja construir
* **[Isolamento multi-tenant](/docs/pt/agent-sdk/hosting#multi-tenant-isolation)**: isole as configurações e memória de cada tenant com `settingSources` / `setting_sources`, `env` e `cwd`
