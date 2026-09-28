> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar permissões

> Controle como seu agente usa ferramentas com modos de permissão, hooks e regras declarativas de permissão/negação.

O Claude Agent SDK fornece controles de permissão para gerenciar como Claude usa ferramentas. Use modos de permissão e regras para definir o que é permitido automaticamente, e o callback [`canUseTool`](/docs/pt/agent-sdk/user-input) para lidar com tudo mais em tempo de execução.

<h2 id="how-permissions-are-evaluated">
  Como as permissões são avaliadas
</h2>

Quando Claude solicita uma ferramenta, o SDK verifica as permissões nesta ordem:

<Steps>
  <Step title="Hooks">
    Execute [hooks](/docs/pt/agent-sdk/hooks) primeiro. Um hook pode negar a chamada completamente ou deixá-la passar. Um hook que retorna `allow` não ignora as regras de negação e pergunta abaixo; essas são avaliadas independentemente do resultado do hook. Um hook `PreToolUse` allow também não pode aprovar uma remoção `rm` ou `rmdir` direcionada a um [caminho crítico](/docs/pt/permission-modes#critical-paths).
  </Step>

  <Step title="Regras de negação">
    Verifique as regras `deny` (de `disallowed_tools` e [settings.json](/docs/pt/settings-reference#permission-settings)). Se uma regra de negação corresponder, a ferramenta é bloqueada, mesmo no modo `bypassPermissions`. Regras de negação com nome simples como `Bash` removem a ferramenta do contexto do Claude antes desta avaliação começar, portanto apenas regras com escopo como `Bash(rm *)` são verificadas nesta etapa.
  </Step>

  <Step title="Regras de pergunta">
    Verifique as regras `ask` de [settings.json](/docs/pt/settings-reference#permission-settings). Se uma regra de pergunta corresponder, a chamada passa para seu callback [`canUseTool`](/docs/pt/agent-sdk/user-input) para confirmação, mesmo no modo `bypassPermissions`.

    Ferramentas que requerem interação do usuário se comportam da mesma forma: `AskUserQuestion` e ferramentas MCP cujo servidor define [`_meta["anthropic/requiresUserInteraction"]`](/docs/pt/mcp#require-approval-for-a-specific-tool) sempre passam para o callback, mesmo quando uma regra de permissão corresponde. No modo `dontAsk` ambos os casos são negados, porque esse modo nunca solicita. A anotação MCP requer Claude Code v2.1.199 ou posterior.

    Ferramentas do conector [claude.ai](/docs/pt/mcp#organization-controls-on-connector-tools) que sua organização definiu como `ask` também saem do fluxo nesta etapa. Cada chamada passa para o callback, mesmo no modo `bypassPermissions` e mesmo quando uma regra de permissão corresponde. O callback recebe o motivo `Your organization requires approval for this tool`. No modo `dontAsk` a chamada é negada, porque esse modo nunca solicita.
  </Step>

  <Step title="Modo de permissão">
    Aplique o [modo de permissão](#permission-modes) ativo:

    * No modo `bypassPermissions`, Claude Code aprova tudo que chega a esta etapa, exceto remoções `rm` e `rmdir` direcionadas a um [caminho crítico](/docs/pt/permission-modes#critical-paths), que passam para a próxima etapa.
    * No modo `acceptEdits`, Claude Code aprova as operações de arquivo listadas em [Modo Accept edits](#accept-edits-mode-acceptedits).
    * No modo `plan`, Claude Code envia ferramentas de edição de arquivo e escrita de shell para seu callback `canUseTool` independentemente das regras de permissão, para que operações de escrita não possam ser aprovadas automaticamente durante o planejamento.
    * Em outros modos, a solicitação passa para a próxima etapa.
  </Step>

  <Step title="Regras de permissão">
    Verifique as regras `allow` (de `allowed_tools` e settings.json). Se uma regra corresponder, a ferramenta é aprovada. Uma chamada que a ferramenta aprova por conta própria é resolvida nesta etapa também, sem necessidade de regra: por exemplo uma leitura de arquivo dentro de seus diretórios de trabalho ou um [comando Bash somente leitura](/docs/pt/permissions#read-only-commands). Remoções `rm` e `rmdir` direcionadas a um [caminho crítico](/docs/pt/permission-modes#critical-paths) nunca são aprovadas por uma regra de permissão: elas chegam ao seu callback nos modos que solicitam, vão para o [classificador](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) no modo `auto` no Claude Code v2.1.218 ou posterior, e são negadas no modo `dontAsk`.
  </Step>

  <Step title="Callback canUseTool">
    Se não for resolvido por nenhum dos anteriores, chame seu callback [`canUseTool`](/docs/pt/agent-sdk/user-input) para uma decisão. No modo `dontAsk`, esta etapa é ignorada e a ferramenta é negada.

    No SDK TypeScript, se você definir [`permissionPrompts: 'none'`](/docs/pt/agent-sdk/typescript#options), seu callback não é chamado nesta etapa. Um hook [`PermissionRequest`](/docs/pt/hooks#permissionrequest) ainda tem a chance de decidir, e se não decidir, Claude Code nega a chamada. A opção requer Claude Code v2.1.259 ou posterior.
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" className="dark:hidden" alt="Diagrama do fluxo de avaliação de permissões em seis etapas correspondendo às etapas acima: uma solicitação de ferramenta passa por hooks, regras de negação, regras de pergunta, modo de permissão, regras de permissão e canUseTool. Hooks, regras de negação e canUseTool podem rotear para Bloqueado; bypass do modo de permissão, regras de permissão e canUseTool podem rotear para Executar; regras de pergunta rotear para canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/permissions-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=e53a91e9059cbf51852b7cedb4dd4251" className="hidden dark:block" alt="Diagrama do fluxo de avaliação de permissões em seis etapas correspondendo às etapas acima: uma solicitação de ferramenta passa por hooks, regras de negação, regras de pergunta, modo de permissão, regras de permissão e canUseTool. Hooks, regras de negação e canUseTool podem rotear para Bloqueado; bypass do modo de permissão, regras de permissão e canUseTool podem rotear para Executar; regras de pergunta rotear para canUseTool." width="1180" height="260" data-path="images/agent-sdk/permissions-flow-dark.svg" />

Se você passar um callback `canUseTool` em uma configuração onde o SDK TypeScript espera que a ordem de avaliação aprove automaticamente as chamadas antes do callback ser consultado, o SDK emite um aviso de processo Node.js uma vez quando a consulta é construída. O código do aviso é `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`. Duas configurações o acionam:

* `permissionMode: 'bypassPermissions'`, que aprova automaticamente cada chamada que chega à etapa do modo de permissão, exceto pelas [ações que nenhum modo aprova automaticamente](/docs/pt/permission-modes#actions-no-mode-auto-approves)
* Cada entrada `allowedTools` simples como `"Read"`, que aprova automaticamente essa ferramenta inteira antes do callback ser consultado, exceto pelas [ações que nenhum modo aprova automaticamente](/docs/pt/permission-modes#actions-no-mode-auto-approves)

Entradas com um especificador como `Bash(ls *)` e o modo `acceptEdits` não o acionam, e regras de permissão provenientes de arquivos de configuração não são visíveis para a verificação.

Ouça com `process.on('warning', ...)` e corresponda o código para registrá-lo ou suprimi-lo. Para controlar cada chamada de ferramenta independentemente do modo e das regras, use um [hook `PreToolUse`](/docs/pt/agent-sdk/hooks).

Esta página se concentra em **regras de permissão e negação** e **modos de permissão**. Para as outras etapas:

* **Hooks:** execute código personalizado para permitir, negar ou modificar solicitações de ferramenta. Veja [Controlar execução com hooks](/docs/pt/agent-sdk/hooks).
* **Callback canUseTool:** solicite aprovação dos usuários em tempo de execução, quando nenhuma etapa anterior resolver a chamada. Veja [Lidar com aprovações e entrada do usuário](/docs/pt/agent-sdk/user-input).

<h2 id="allow-and-deny-rules">
  Regras de permissão e negação
</h2>

`allowed_tools` e `disallowed_tools` (TypeScript: `allowedTools` / `disallowedTools`) adicionam entradas às listas de regras de permissão e negação no fluxo de avaliação acima. Se você nomear uma das [ferramentas de rastreamento de tarefas](/docs/pt/agent-sdk/todo-tracking#model-availability) em `allowed_tools`, Claude Code também ativa a sessão. Qualquer outra ferramenta não listada em `allowed_tools` ainda está disponível para Claude, e uma chamada a ela que precisa de aprovação passa para o modo de permissão. As regras de negação se comportam de forma diferente dependendo se nomeiam uma ferramenta ou definem um padrão dentro de uma.

| Opção                             | Efeito                                                                                                                                                                                                                                                                       |
| :-------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowed_tools=["Read", "Grep"]`  | `Read` e `Grep` são aprovadas automaticamente. Outras ferramentas não listadas aqui ainda existem, e chamadas a elas que precisam de aprovação passam para o modo de permissão e `canUseTool`.                                                                               |
| `disallowed_tools=["Bash"]`       | A definição da ferramenta `Bash` é removida da solicitação. Claude não vê a ferramenta e não pode tentar usá-la.                                                                                                                                                             |
| `disallowed_tools=["Bash(rm *)"]` | `Bash` permanece disponível. Chamadas correspondentes a `rm *` [conforme escrito](/docs/pt/permissions#bash-rule-limits) são negadas em todos os modos de permissão, incluindo `bypassPermissions`. Outras chamadas `Bash`, incluindo `/bin/rm`, passam para o modo de permissão. |
| `disallowed_tools=["*"]`          | Toda definição de ferramenta é removida da solicitação. Globs de nome de ferramenta são suportados em regras de negação: `"*"` corresponde a todas as ferramentas e `"mcp__*"` corresponde a todas as ferramentas MCP em todos os servidores.                                |

As regras de permissão aceitam globs de nome de ferramenta apenas após um prefixo literal `mcp__<server>__`. O segmento do servidor deve estar livre de glob para que a regra nomeie um servidor específico que você configurou: `mcp__puppeteer__*` corresponde a todas as ferramentas do servidor `puppeteer`, e `mcp__github__get_*` corresponde às suas ferramentas `get_`. Uma entrada não ancorada como `allowed_tools=["*"]` ou `allowed_tools=["mcp__*"]` é ignorada com um aviso de inicialização e não aprova automaticamente nada.

As regras com escopo para `Read` e `Edit` usam um padrão de caminho. As regras `Edit(path)` governam todas as ferramentas integradas que escrevem arquivos, incluindo `Write` e `NotebookEdit`; uma regra `Write(path)` nunca é correspondida pelas verificações de permissão de arquivo.

Use `//path` para um caminho absoluto do sistema de arquivos: uma regra de negação de `Edit(//secrets/**)` bloqueia escritas em qualquer lugar sob `/secrets` no disco. Com uma única barra inicial, `Edit(/secrets/**)` ancora na fonte da regra. Para regras passadas através de `allowed_tools` ou `disallowed_tools`, isso significa o diretório de trabalho da sessão, portanto a regra não bloqueia `/secrets` no disco. Veja [Regras Read e Edit](/docs/pt/permissions#read-and-edit) para as quatro formas de âncora e como as regras dos arquivos de configuração são resolvidas.

<Warning>
  **Ferramentas aprovadas automaticamente nunca chegam a `canUseTool`.** Uma chamada de ferramenta aprovada em qualquer etapa anterior, por `acceptEdits` ou `bypassPermissions`, ou por uma regra de permissão, ignora seu callback `canUseTool`, portanto as verificações de permissão que você coloca lá são silenciosamente ignoradas para essa ferramenta. `AskUserQuestion`, ferramentas MCP marcadas [`_meta["anthropic/requiresUserInteraction"]`](/docs/pt/mcp#require-approval-for-a-specific-tool), ferramentas de conector [que sua organização definiu como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools), e remoções `rm` e `rmdir` direcionadas a um [caminho crítico](/docs/pt/permission-modes#critical-paths) ainda chegam ao callback, mesmo quando uma regra de permissão corresponde. No modo `auto`, as remoções de caminho crítico vão para o [classificador](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) em vez do callback, enquanto as outras chamadas listadas aqui ainda chegam a ele; o roteamento do classificador requer Claude Code v2.1.218 ou posterior. No modo `dontAsk` essas chamadas são negadas, sem invocar o callback.

  A cobertura depende da forma da entrada: um nome simples como `Read` ou `mcp__github__get_issue` aprova automaticamente todas as chamadas a essa ferramenta, exceto as exceções acima, enquanto uma regra com escopo como `Bash(npm test *)` aprova automaticamente apenas chamadas correspondentes, e outras chamadas `Bash` que precisam de aprovação ainda passam para o callback. Para verificações que devem ser executadas em todas as chamadas de ferramenta, use um [hook `PreToolUse`](/docs/pt/agent-sdk/hooks): hooks são executados antes de qualquer outra etapa, e uma negação de hook se aplica mesmo no modo `bypassPermissions`.
</Warning>

Para um agente bloqueado, combine `allowedTools` com `permissionMode: "dontAsk"`:

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

As ferramentas listadas são aprovadas, exceto pelas [ações que nenhum modo aprova automaticamente](/docs/pt/permission-modes#actions-no-mode-auto-approves), e todas as outras chamadas que solicitariam aprovação são negadas. Chamadas que não precisam de aprovação no modo `default` são executadas independentemente de você listá-las, como [comandos Bash somente leitura](/docs/pt/permissions#read-only-commands), ferramentas como `Agent` que não solicitam antes de executar, e leituras de arquivo dentro de seus diretórios de trabalho. Para colocar uma ferramenta completamente fora do alcance de Claude, adicione seu nome simples a `disallowedTools`.

<Warning>
  **`allowed_tools` não restringe `bypassPermissions`.** `allowed_tools` aprova previamente as ferramentas que você lista. Outras ferramentas não listadas não são correspondidas por nenhuma regra de permissão e passam para o modo de permissão, onde `bypassPermissions` as aprova. Definir `allowed_tools=["Read"]` junto com `permission_mode="bypassPermissions"` ainda aprova todas as ferramentas, incluindo `Bash`, `Write` e `Edit`. Se você precisar de `bypassPermissions` mas quiser ferramentas específicas bloqueadas, use `disallowed_tools`.
</Warning>

Você também pode configurar regras de permissão, negação e solicitação de forma declarativa em `.claude/settings.json`. Essas regras são lidas quando a fonte de configuração `project` está habilitada, o que é o padrão para opções `query()`. Se você definir `setting_sources` (TypeScript: `settingSources`) explicitamente, inclua `"project"` para que se apliquem. Veja [Configurações de permissão](/docs/pt/settings-reference#permission-settings) para a sintaxe das regras.

<h2 id="permission-modes">
  Modos de permissão
</h2>

Os modos de permissão fornecem controle global sobre como Claude usa ferramentas. Você pode definir o modo de permissão ao chamar `query()` ou alterá-lo dinamicamente durante sessões de streaming.

<h3 id="available-modes">
  Modos disponíveis
</h3>

O SDK suporta estes modos de permissão:

| Modo                | Descrição                                  | Comportamento da ferramenta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :------------------ | :----------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Comportamento de permissão padrão          | Sem aprovações automáticas baseadas em modo; chamadas que precisam de aprovação e não correspondem a nenhuma regra de permissão acionam seu callback `canUseTool`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `dontAsk`           | Negar em vez de solicitar                  | Qualquer chamada que de outra forma solicitaria é negada. Chamadas aprovadas por `allowed_tools` ou regras são executadas, assim como chamadas que não precisam de aprovação no modo `default`, como leituras de arquivo dentro de seus diretórios de trabalho e chamadas para `Agent`. Ferramentas de conector [que sua organização definiu como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) e ferramentas que exigem interação do usuário são negadas mesmo que você as tenha pré-aprovado, assim como remoções `rm` e `rmdir` direcionadas a um [caminho crítico](/docs/pt/permission-modes#critical-paths). `canUseTool` nunca é chamado |
| `acceptEdits`       | Aceitar automaticamente edições de arquivo | Edições de arquivo e [operações do sistema de arquivos](#accept-edits-mode-acceptedits) (`mkdir`, `rm`, `mv`, etc.) são automaticamente aprovadas                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `bypassPermissions` | Contornar verificações de permissão        | As ferramentas são executadas sem prompts de permissão, exceto pelas [ações que nenhum modo aprova automaticamente](/docs/pt/permission-modes#actions-no-mode-auto-approves). Use com cuidado                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `plan`              | Modo de planejamento                       | Claude explora e planeja sem editar seus arquivos de origem; edições de arquivo nunca são aprovadas automaticamente e solicitam através de seu callback `canUseTool`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `auto`              | Aprovações classificadas pelo modelo       | Um classificador de modelo aprova ou nega prompts de permissão. Consulte [Modo Auto](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) para disponibilidade                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |

<Warning>
  **Herança de subagente:** Um subagente é executado no modo de permissão da sessão pai, a menos que você defina `permissionMode` em sua [`AgentDefinition`](/docs/pt/agent-sdk/typescript#agentdefinition) e a sessão pai esteja em modo `default`, `dontAsk` ou `plan`. Mesmo assim, Claude Code nunca aplica um valor `"bypassPermissions"`. Um subagente é executado em modo `bypassPermissions` apenas quando a sessão pai também está. A exceção `bypassPermissions` requer Claude Code v2.1.267 ou posterior.

  Subagentes podem ter prompts de sistema diferentes e comportamento menos restrito do que seu agente principal, portanto herdar `bypassPermissions` concede a eles acesso completo e autônomo ao sistema. As [ações que nenhum modo aprova automaticamente](/docs/pt/permission-modes#actions-no-mode-auto-approves) ainda se aplicam.
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
    Chame `set_permission_mode()` (Python) ou `setPermissionMode()` (TypeScript) para alterar o modo durante a sessão. O novo modo entra em vigor imediatamente para todas as solicitações de ferramenta subsequentes. Isso permite que você comece restritivo e afrouxe as permissões conforme a confiança aumenta, por exemplo, alternando para `acceptEdits` após revisar a abordagem inicial de Claude.

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

Aprova automaticamente operações de arquivo para que Claude possa editar código sem solicitar. Outras ferramentas (como comandos Bash que não são operações do sistema de arquivos) ainda exigem permissões normais.

**Operações aprovadas automaticamente:**

* Edições de arquivo (ferramentas Edit, Write)
* Comandos do sistema de arquivos: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`

Ambos se aplicam apenas a caminhos dentro do diretório de trabalho ou `additionalDirectories`. No modo `acceptEdits`, Claude Code não aprova automaticamente a solicitação quando Claude:

* Trabalha em um caminho fora desse escopo
* Escreve em um caminho protegido
* Remove um [caminho crítico](/docs/pt/permission-modes#critical-paths) com `rm` ou `rmdir`

**Use quando:** você confia nas edições de Claude e deseja iteração mais rápida, como durante prototipagem ou ao trabalhar em um diretório isolado.

<h4 id="don’t-ask-mode-dontask">
  Modo não perguntar (`dontAsk`)
</h4>

Converte qualquer prompt de permissão em uma negação, sem chamar `canUseTool`. Ferramentas pré-aprovadas por `allowed_tools`, regras de permissão em `settings.json` ou um hook são executadas normalmente, assim como chamadas que não precisam de aprovação no modo `default`, como leituras de arquivo dentro de seus diretórios de trabalho e chamadas para `Agent`. Ferramentas de conector [que sua organização definiu como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools), ferramentas que exigem interação do usuário e remoções `rm` e `rmdir` direcionadas a um [caminho crítico](/docs/pt/permission-modes#critical-paths) são negadas mesmo quando uma regra de permissão corresponde. Uma permissão de hook `PreToolUse` também não limpa uma remoção de caminho crítico.

**Use quando:** você deseja uma superfície de ferramenta fixa e explícita para um agente sem interface e prefere uma negação definitiva em vez de depender silenciosamente de `canUseTool` estar ausente.

<h4 id="bypass-permissions-mode-bypasspermissions">
  Modo contornar permissões (`bypassPermissions`)
</h4>

Aprova automaticamente usos de ferramentas sem solicitar, exceto os casos listados no aviso abaixo. Hooks ainda são executados e podem bloquear operações se necessário. No Linux e macOS, Claude Code recusa iniciar neste modo como root ou sob `sudo` fora de uma [sandbox reconhecida](/docs/pt/permission-modes#skip-all-checks-with-bypasspermissions-mode), e a consulta falha antes da primeira volta.

<Warning>
  Use com extrema cautela. Claude tem acesso completo ao sistema neste modo. Use apenas em ambientes controlados onde você confia em todas as operações possíveis.

  `allowed_tools` não restringe este modo. Cada ferramenta é aprovada, não apenas as que você listou. Estes controles ainda se aplicam:

  * Regras de negação, regras explícitas `ask` e hooks são avaliados antes da verificação de modo e ainda podem bloquear uma ferramenta.
  * Ferramentas de conector [que sua organização definiu como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools), ferramentas que exigem interação do usuário e remoções `rm` e `rmdir` direcionadas a um [caminho crítico](/docs/pt/permission-modes#critical-paths) ainda caem para seu callback `canUseTool`.
  * Os [salvaguardas de mensagens entre sessões](/docs/pt/permission-modes#skip-all-checks-with-bypasspermissions-mode) ainda se aplicam.
</Warning>

<h4 id="plan-mode-plan">
  Modo plano (`plan`)
</h4>

Claude explora a base de código e produz um plano sem editar seus arquivos de origem. Ferramentas somente leitura são executadas como fazem no modo de permissão `default`.

Edições de arquivo nunca são aprovadas automaticamente no modo plano, mesmo quando uma regra de permissão corresponde. Em vez disso, elas solicitam através de seu callback `canUseTool`. No Claude Code v2.1.212 ou posterior, comandos shell que modificam arquivos, como `touch` e `rm`, chegam ao seu callback `canUseTool` da mesma forma.

Se você definir `allowDangerouslySkipPermissions: true` junto com `permissionMode: 'plan'`, edições de arquivo e comandos shell que modificam arquivos ainda chegam ao seu callback `canUseTool`. A opção permite que você alterne para `bypassPermissions` mais tarde com `setPermissionMode()`.

Claude pode usar `AskUserQuestion` para esclarecer requisitos antes de finalizar o plano. Consulte [Lidar com aprovações e entrada do usuário](/docs/pt/agent-sdk/user-input#handle-clarifying-questions) para lidar com esses prompts.

**Use quando:** você deseja que Claude proponha alterações sem executá-las, como durante revisão de código ou quando você precisa aprovar alterações antes que sejam feitas.

<h2 id="related-resources">
  Recursos relacionados
</h2>

Para as outras etapas no fluxo de avaliação de permissões:

* [Lidar com aprovações e entrada do usuário](/docs/pt/agent-sdk/user-input): prompts de aprovação interativa e perguntas de esclarecimento
* [Guia de hooks](/docs/pt/agent-sdk/hooks): executar código personalizado em pontos-chave do ciclo de vida do agente
* [Regras de permissão](/docs/pt/settings-reference#permission-settings): regras declarativas de permissão/negação em `settings.json`
