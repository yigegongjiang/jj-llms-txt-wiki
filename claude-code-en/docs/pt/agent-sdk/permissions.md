> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar permissões

> Controle como seu agente usa ferramentas com modos de permissão, hooks e regras declarativas de permitir/negar.

O Claude Agent SDK fornece controles de permissão para gerenciar como Claude usa ferramentas. Use modos de permissão e regras para definir o que é permitido automaticamente, e o callback [`canUseTool`](/docs/pt/agent-sdk/user-input) para lidar com tudo mais em tempo de execução.

<Note>
  Esta página cobre modos de permissão e regras. Para construir fluxos de aprovação interativos onde os usuários aprovam ou negam solicitações de ferramentas em tempo de execução, consulte [Lidar com aprovações e entrada do usuário](/docs/pt/agent-sdk/user-input).
</Note>

<h2 id="how-permissions-are-evaluated">
  Como as permissões são avaliadas
</h2>

Quando Claude solicita uma ferramenta, o SDK verifica as permissões nesta ordem:

<Steps>
  <Step title="Hooks">
    Execute [hooks](/docs/pt/agent-sdk/hooks) primeiro. Um hook pode negar a chamada completamente ou passá-la adiante. Um hook que retorna `allow` não ignora as regras de negar e perguntar abaixo; essas são avaliadas independentemente do resultado do hook.
  </Step>

  <Step title="Regras de negação">
    Verifique as regras `deny` (de `disallowed_tools` e [settings.json](/docs/pt/settings#permission-settings)). Se uma regra de negação corresponder, a ferramenta é bloqueada, mesmo no modo `bypassPermissions`. Regras com nome simples como `Bash` removem a ferramenta do contexto do Claude antes desta avaliação começar, portanto apenas regras com escopo como `Bash(rm *)` são verificadas neste passo.
  </Step>

  <Step title="Regras de pergunta">
    Verifique as regras `ask` de [settings.json](/docs/pt/settings#permission-settings). Se uma regra de pergunta corresponder, a chamada passa para seu callback [`canUseTool`](/docs/pt/agent-sdk/user-input) para confirmação, mesmo no modo `bypassPermissions`.

    Ferramentas que requerem interação do usuário se comportam da mesma forma: `AskUserQuestion` e ferramentas MCP cujo servidor define [`_meta["anthropic/requiresUserInteraction"]`](/docs/pt/mcp#require-approval-for-a-specific-tool) sempre passam para o callback, mesmo quando uma regra de permitir corresponde. No modo `dontAsk` ambos os casos são negados, porque esse modo nunca solicita. A anotação MCP requer Claude Code v2.1.199 ou posterior.

    Ferramentas do conector [claude.ai](/docs/pt/mcp#organization-controls-on-connector-tools) que sua organização definiu como `ask` também saem do fluxo neste passo. Cada chamada passa para o callback, mesmo no modo `bypassPermissions` e mesmo quando uma regra de permitir corresponde. O callback recebe o motivo `Sua organização requer aprovação para esta ferramenta`. No modo `dontAsk` a chamada é negada, porque esse modo nunca solicita.
  </Step>

  <Step title="Modo de permissão">
    Aplique o [modo de permissão](#permission-modes) ativo. `bypassPermissions` aprova tudo que chega a este passo. `acceptEdits` aprova operações de arquivo. `plan` roteia ferramentas de edição de arquivo e escrita de shell para seu callback `canUseTool` independentemente das regras de permitir, portanto operações de escrita não podem ser aprovadas automaticamente durante o planejamento. Outros modos passam adiante.
  </Step>

  <Step title="Regras de permitir">
    Verifique as regras `allow` (de `allowed_tools` e settings.json). Se uma regra corresponder, a ferramenta é aprovada.
  </Step>

  <Step title="Callback canUseTool">
    Se não for resolvido por nenhum dos anteriores, chame seu callback [`canUseTool`](/docs/pt/agent-sdk/user-input) para uma decisão. No modo `dontAsk`, este passo é ignorado e a ferramenta é negada.
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" alt="Diagrama do fluxo de avaliação de permissões em seis etapas correspondendo aos passos acima: uma solicitação de ferramenta passa por hooks, regras de negação, regras de pergunta, modo de permissão, regras de permissão e canUseTool. Hooks, regras de negação e canUseTool podem rotear para Bloqueado; bypass de modo de permissão, regras de permissão e canUseTool podem rotear para Executar; regras de pergunta rotear para canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

A partir da v2.1.198, se você passar um callback `canUseTool` que esta ordem de avaliação nunca pode alcançar, o SDK TypeScript emite um aviso de processo Node.js uma vez quando a consulta é construída. O código do aviso é `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`. Duas configurações o acionam:

* `permissionMode: 'bypassPermissions'`, que aprova automaticamente cada chamada que chega ao passo do modo de permissão
* Cada entrada `allowedTools` simples como `"Read"`, que aprova automaticamente essa ferramenta inteira antes do callback ser consultado

Entradas com um especificador como `Bash(ls *)` e o modo `acceptEdits` não o acionam, e regras de permitir provenientes de arquivos de configuração não são visíveis para a verificação.

Ouça com `process.on('warning', ...)` e corresponda o código para registrá-lo ou suprimi-lo. Para controlar cada chamada de ferramenta independentemente do modo e das regras, use um [hook `PreToolUse`](/docs/pt/agent-sdk/hooks) em vez disso.

Esta página se concentra em **regras de permitir e negar** e **modos de permissão**. Para os outros passos:

* **Hooks:** execute código personalizado para permitir, negar ou modificar solicitações de ferramentas. Consulte [Controlar execução com hooks](/docs/pt/agent-sdk/hooks).
* **Callback canUseTool:** solicite aprovação dos usuários em tempo de execução, quando nenhum passo anterior resolver a chamada. Consulte [Lidar com aprovações e entrada do usuário](/docs/pt/agent-sdk/user-input).

<h2 id="allow-and-deny-rules">
  Regras de permitir e negar
</h2>

`allowed_tools` e `disallowed_tools` (TypeScript: `allowedTools` / `disallowedTools`) adicionam entradas às listas de regras de permitir e negar no fluxo de avaliação acima. Regras de permitir afetam apenas a aprovação: uma ferramenta não listada em `allowed_tools` ainda está disponível para Claude e passa para o modo de permissão. Regras de negar se comportam de forma diferente dependendo se nomeiam uma ferramenta ou definem um padrão dentro de uma.

| Opção                             | Efeito                                                                                                                                                                                                                                      |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `allowed_tools=["Read", "Grep"]`  | `Read` e `Grep` são auto-aprovadas. Ferramentas não listadas aqui ainda existem e passam para o modo de permissão e `canUseTool`.                                                                                                           |
| `disallowed_tools=["Bash"]`       | A definição da ferramenta `Bash` é removida da solicitação. Claude não vê a ferramenta e não pode tentar usá-la.                                                                                                                            |
| `disallowed_tools=["Bash(rm *)"]` | `Bash` permanece disponível. Chamadas correspondentes a `rm *` são negadas em todos os modos de permissão, incluindo `bypassPermissions`. Outras chamadas de `Bash` passam para o modo de permissão.                                        |
| `disallowed_tools=["*"]`          | Toda definição de ferramenta é removida da solicitação. Globs de nome de ferramenta são suportados em regras de negar: `"*"` corresponde a todas as ferramentas e `"mcp__*"` corresponde a todas as ferramentas MCP em todos os servidores. |

Regras de permitir aceitam globs de nome de ferramenta apenas após um prefixo literal `mcp__<server>__`. O segmento do servidor deve estar livre de glob para que a regra nomeie um servidor específico que você configurou: `mcp__puppeteer__*` corresponde a todas as ferramentas do servidor `puppeteer`, e `mcp__github__get_*` corresponde às suas ferramentas `get_`. Uma entrada não ancorada como `allowed_tools=["*"]` ou `allowed_tools=["mcp__*"]` é ignorada com um aviso de inicialização e não auto-aprova nada.

Regras com escopo para `Read` e `Edit` usam um padrão de caminho. Regras `Edit(path)` governam todas as ferramentas integradas que escrevem arquivos, incluindo `Write` e `NotebookEdit`; uma regra `Write(path)` nunca é correspondida pelas verificações de permissão de arquivo.

Use `//path` para um caminho absoluto do sistema de arquivos: uma regra de negar de `Edit(//secrets/**)` bloqueia escritas em qualquer lugar sob `/secrets` no disco. Com uma única barra inicial, `Edit(/secrets/**)` ancora na fonte da regra em vez disso. Para regras passadas através de `allowed_tools` ou `disallowed_tools`, isso significa o diretório de trabalho da sessão, portanto a regra não bloqueia `/secrets` no disco. Consulte [Regras de Read e Edit](/docs/pt/permissions#read-and-edit) para as quatro formas de âncora e como regras de arquivos de configuração são resolvidas.

<Warning>
  **Ferramentas auto-aprovadas nunca chegam a `canUseTool`.** Uma chamada de ferramenta aprovada em qualquer etapa anterior, por `acceptEdits` ou `bypassPermissions`, ou por uma regra de permitir, ignora seu callback `canUseTool`, portanto verificações de permissão que você coloca lá são silenciosamente contornadas para essa ferramenta. `AskUserQuestion`, ferramentas MCP marcadas [`_meta["anthropic/requiresUserInteraction"]`](/docs/pt/mcp#require-approval-for-a-specific-tool), e ferramentas de conector [que sua organização definiu como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) ainda chegam ao callback, mesmo quando uma regra de permitir corresponde.

  A cobertura depende da forma da entrada: um nome simples como `Read` ou `mcp__github__get_issue` auto-aprova todas as chamadas para essa ferramenta, enquanto uma regra com escopo como `Bash(ls *)` auto-aprova apenas chamadas correspondentes e outras chamadas de `Bash` ainda passam para o callback. Para verificações que devem ser executadas em todas as chamadas de ferramenta, use um hook [`PreToolUse`](/docs/pt/agent-sdk/hooks): hooks são executados antes de qualquer outra etapa, e uma negação de hook se aplica mesmo no modo `bypassPermissions`.
</Warning>

Para um agente bloqueado, combine `allowedTools` com `permissionMode: "dontAsk"`. Ferramentas listadas são aprovadas, além das ferramentas que sempre solicitam no Aviso acima; qualquer outra coisa é negada completamente em vez de solicitar:

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

<Warning>
  **`allowed_tools` não restringe `bypassPermissions`.** `allowed_tools` apenas pré-aprova as ferramentas que você lista. Ferramentas não listadas não são correspondidas por nenhuma regra de permitir e passam para o modo de permissão, onde `bypassPermissions` as aprova. Definir `allowed_tools=["Read"]` junto com `permission_mode="bypassPermissions"` ainda aprova todas as ferramentas, incluindo `Bash`, `Write` e `Edit`. Se você precisar de `bypassPermissions` mas quiser que ferramentas específicas sejam bloqueadas, use `disallowed_tools`.
</Warning>

Você também pode configurar regras de permitir, negar e perguntar declarativamente em `.claude/settings.json`. Essas regras são lidas quando a fonte de configuração `project` está habilitada, o que é o padrão para opções `query()`. Se você definir `setting_sources` (TypeScript: `settingSources`) explicitamente, inclua `"project"` para que se apliquem. Consulte [Configurações de permissão](/docs/pt/settings#permission-settings) para a sintaxe das regras.

<h2 id="permission-modes">
  Modos de permissão
</h2>

Os modos de permissão fornecem controle global sobre como Claude usa ferramentas. Você pode definir o modo de permissão ao chamar `query()` ou alterá-lo dinamicamente durante sessões de streaming.

<h3 id="available-modes">
  Modos disponíveis
</h3>

O SDK suporta estes modos de permissão:

| Modo                | Descrição                           | Comportamento da ferramenta                                                                                                                                                                                                                                                                                                                       |
| :------------------ | :---------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `default`           | Comportamento de permissão padrão   | Sem auto-aprovações; ferramentas não correspondidas acionam seu callback `canUseTool`                                                                                                                                                                                                                                                             |
| `dontAsk`           | Negar em vez de solicitar           | Qualquer coisa não pré-aprovada por `allowed_tools` ou regras é negada; ferramentas de conector [sua organização definida como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) e ferramentas que requerem interação do usuário são negadas mesmo se você as pré-aprovou. `canUseTool` nunca é chamado                                    |
| `acceptEdits`       | Auto-aceitar edições de arquivo     | Edições de arquivo e [operações de sistema de arquivos](#accept-edits-mode-acceptedits) (`mkdir`, `rm`, `mv`, etc.) são automaticamente aprovadas                                                                                                                                                                                                 |
| `bypassPermissions` | Ignorar verificações de permissão   | As ferramentas são executadas sem solicitações de permissão, exceto ferramentas correspondidas por uma regra [`ask`](#how-permissions-are-evaluated) explícita, ferramentas de conector [sua organização definida como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) e ferramentas que requerem interação do usuário (use com cuidado) |
| `plan`              | Modo de planejamento                | Claude explora e planeja sem editar seus arquivos de origem; edições de arquivo nunca são auto-aprovadas e solicitam através de seu callback `canUseTool`                                                                                                                                                                                         |
| `auto`              | Aprovações classificadas por modelo | Um classificador de modelo aprova ou nega cada chamada de ferramenta. Consulte [Auto mode](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) para disponibilidade                                                                                                                                                                            |

<Warning>
  **Herança de subagentos:** Quando o pai usa `bypassPermissions`, `acceptEdits` ou `auto`, todos os subagentos herdam esse modo e ele não pode ser substituído por subagentos. Subagentos podem ter prompts de sistema diferentes e comportamento menos restrito do que seu agente principal, portanto herdar `bypassPermissions` concede a eles acesso completo e autônomo ao sistema. Uma regra [`ask`](#how-permissions-are-evaluated) explícita, ferramentas de conector [sua organização definida como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) e ferramentas que requerem interação do usuário ainda forçam uma solicitação.
</Warning>

<h3 id="set-permission-mode">
  Definir modo de permissão
</h3>

Você pode definir o modo de permissão uma vez ao iniciar uma consulta, ou alterá-lo dinamicamente enquanto a sessão está ativa.

<Tabs>
  <Tab title="No momento da consulta">
    Passe `permission_mode` (Python) ou `permissionMode` (TypeScript) ao criar uma consulta. Este modo se aplica para toda a sessão, a menos que seja alterado dinamicamente.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Help me refactor this code",
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Set the mode here
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        for await (const message of query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Set the mode here
          }
        })) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Durante streaming">
    Chame `set_permission_mode()` (Python) ou `setPermissionMode()` (TypeScript) para alterar o modo no meio da sessão. O novo modo entra em vigor imediatamente para todas as solicitações de ferramentas subsequentes. Isso permite que você comece restritivo e afrouxe as permissões conforme a confiança aumenta, por exemplo, alternando para `acceptEdits` após revisar a abordagem inicial de Claude.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions


      async def main():
          async with ClaudeSDKClient(
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Start in default mode
              )
          ) as client:
              await client.query("Help me refactor this code")

              # Change mode dynamically mid-session
              await client.set_permission_mode("acceptEdits")

              # Process messages with the new permission mode
              async for message in client.receive_response():
                  if hasattr(message, "result"):
                      print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        const q = query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Start in default mode
          }
        });

        // Change mode dynamically mid-session
        await q.setPermissionMode("acceptEdits");

        // Process messages with the new permission mode
        for await (const message of q) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>
</Tabs>

<h3 id="mode-details">
  Detalhes do modo
</h3>

<h4 id="accept-edits-mode-acceptedits">
  Modo aceitar edições (`acceptEdits`)
</h4>

Auto-aprova operações de arquivo para que Claude possa editar código sem solicitar. Outras ferramentas (como comandos Bash que não são operações de sistema de arquivos) ainda requerem permissões normais.

**Operações auto-aprovadas:**

* Edições de arquivo (ferramentas Edit, Write)
* Comandos de sistema de arquivos: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`

Ambos se aplicam apenas a caminhos dentro do diretório de trabalho ou `additionalDirectories`. Caminhos fora desse escopo e gravações em caminhos protegidos ainda solicitam.

**Use quando:** você confia nas edições de Claude e quer iteração mais rápida, como durante prototipagem ou ao trabalhar em um diretório isolado.

<h4 id="don’t-ask-mode-dontask">
  Modo não perguntar (`dontAsk`)
</h4>

Converte qualquer solicitação de permissão em uma negação. Ferramentas pré-aprovadas por `allowed_tools`, regras de permitir em `settings.json` ou um hook são executadas normalmente. Ferramentas de conector [sua organização definida como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) e ferramentas que requerem interação do usuário são negadas mesmo quando uma regra de permitir corresponde. Tudo mais é negado sem chamar `canUseTool`.

**Use quando:** você quer uma superfície de ferramenta fixa e explícita para um agente sem cabeça e prefere uma negação dura sobre confiança silenciosa em `canUseTool` estar ausente.

<h4 id="bypass-permissions-mode-bypasspermissions">
  Modo ignorar permissões (`bypassPermissions`)
</h4>

Auto-aprova todos os usos de ferramentas sem solicitações. Hooks ainda são executados e podem bloquear operações se necessário.

<Warning>
  Use com extrema cautela. Claude tem acesso completo ao sistema neste modo. Use apenas em ambientes controlados onde você confia em todas as operações possíveis.

  `allowed_tools` não restringe este modo. Todas as ferramentas são aprovadas, não apenas as que você listou. Regras de negação (`disallowed_tools`), regras explícitas de `ask` e hooks são avaliados antes da verificação do modo e ainda podem bloquear uma ferramenta. Ferramentas de conector [sua organização definida como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) e ferramentas que requerem interação do usuário ainda caem através de seu callback `canUseTool`.
</Warning>

<h4 id="plan-mode-plan">
  Modo plano (`plan`)
</h4>

Claude explora a base de código e produz um plano sem editar seus arquivos de origem. Ferramentas somente leitura são executadas como no modo padrão. Edições de arquivo nunca são auto-aprovadas no modo plano, mesmo quando uma regra de permitir corresponde. Elas solicitam através de seu callback `canUseTool` em vez disso. Claude pode usar `AskUserQuestion` para esclarecer requisitos antes de finalizar o plano. Consulte [Lidar com aprovações e entrada do usuário](/docs/pt/agent-sdk/user-input#handle-clarifying-questions) para lidar com essas solicitações.

**Use quando:** você quer que Claude proponha mudanças sem executá-las, como durante revisão de código ou quando você precisa aprovar mudanças antes que sejam feitas.

<h2 id="related-resources">
  Recursos relacionados
</h2>

Para os outros passos no fluxo de avaliação de permissões:

* [Lidar com aprovações e entrada do usuário](/docs/pt/agent-sdk/user-input): solicitações de aprovação interativa e perguntas de esclarecimento
* [Guia de hooks](/docs/pt/agent-sdk/hooks): execute código personalizado em pontos-chave do ciclo de vida do agente
* [Regras de permissão](/docs/pt/settings#permission-settings): regras declarativas de permitir/negar em `settings.json`
