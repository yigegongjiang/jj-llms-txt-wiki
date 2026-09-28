> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referência de hooks

> Referência para eventos de hooks do Claude Code, esquema de configuração, formatos de entrada/saída JSON, códigos de saída, hooks assíncronos, hooks HTTP, hooks de prompt e hooks de ferramentas MCP.

<Tip>
  Para um guia de início rápido com exemplos, consulte [Automatizar ações com hooks](/docs/pt/hooks-guide).
</Tip>

Hooks são comandos shell definidos pelo usuário, endpoints HTTP, chamadas de ferramentas MCP, prompts LLM ou subagentos que executam automaticamente em pontos específicos do ciclo de vida do Claude Code. O Claude Code dispara os mesmos eventos de hook onde quer que seja executado: sessões no terminal, extensões de IDE, o [aplicativo Desktop](/docs/pt/desktop-quickstart) e [Claude Code na web](/docs/pt/claude-code-on-the-web). Use esta referência para consultar esquemas de eventos, opções de configuração, formatos de entrada/saída JSON e recursos avançados como hooks assíncronos, hooks HTTP e hooks de ferramentas MCP.

<h2 id="hook-lifecycle">
  Ciclo de vida do hook
</h2>

Claude Code executa hooks em pontos específicos durante uma sessão. Quando um evento dispara e um matcher corresponde, Claude Code passa contexto JSON sobre o evento para seu manipulador de hook. Para hooks de comando, a entrada chega em stdin. Para hooks HTTP, chega como corpo da solicitação POST. Seu manipulador pode então inspecionar a entrada, tomar ação e opcionalmente retornar uma decisão.

Os eventos caem em três cadências:

* por sessão: `SessionStart` e `SessionEnd`
* por turno: `UserPromptSubmit`, `Stop` e `StopFailure`
* em cada chamada de ferramenta dentro do loop agentic: `PreToolUse` e `PostToolUse`, exceto chamadas [`EndConversation`](/docs/pt/tools-reference#endconversation-tool-behavior), que pulam ambas

<div style={{maxWidth: "500px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/x7pO8l4XcvAXCoVc/images/hooks-lifecycle.svg?fit=max&auto=format&n=x7pO8l4XcvAXCoVc&q=85&s=81b9256c1bbe8832553485f5d9e9c746" className="dark:hidden" alt="Diagrama do ciclo de vida do hook mostrando Setup opcional alimentando SessionStart, depois um loop por turno contendo UserPromptSubmit, UserPromptExpansion para slash commands, o loop agentic aninhado (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted), e Stop ou StopFailure, seguido por TeammateIdle, PreCompact, PostCompact e SessionEnd, com Elicitation e ElicitationResult aninhados dentro da execução de ferramenta MCP, PermissionDenied como um ramo lateral de PermissionRequest para negações em modo automático, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged, FileChanged e DirectoryAdded como eventos assíncronos independentes, PreModelSwitch como um evento sequencial independente que é executado antes de uma mudança de modelo solicitada, PostModelSwitch como um evento assíncrono independente que é executado após as mudanças de modelo da sessão, e MessageDisplay como um evento somente de exibição que é executado enquanto o texto da mensagem do assistente é transmitido" width="520" height="1336" data-path="images/hooks-lifecycle.svg" />

    <img src="https://mintcdn.com/claude-code/x7pO8l4XcvAXCoVc/images/hooks-lifecycle-dark.svg?fit=max&auto=format&n=x7pO8l4XcvAXCoVc&q=85&s=c9b3d88487335f58cce0b52e2f9e7531" className="hidden dark:block" alt="Diagrama do ciclo de vida do hook mostrando Setup opcional alimentando SessionStart, depois um loop por turno contendo UserPromptSubmit, UserPromptExpansion para slash commands, o loop agentic aninhado (PreToolUse, PermissionRequest, PostToolUse, PostToolUseFailure, PostToolBatch, SubagentStart/Stop, TaskCreated, TaskCompleted), e Stop ou StopFailure, seguido por TeammateIdle, PreCompact, PostCompact e SessionEnd, com Elicitation e ElicitationResult aninhados dentro da execução de ferramenta MCP, PermissionDenied como um ramo lateral de PermissionRequest para negações em modo automático, WorktreeCreate, WorktreeRemove, Notification, ConfigChange, InstructionsLoaded, CwdChanged, FileChanged e DirectoryAdded como eventos assíncronos independentes, PreModelSwitch como um evento sequencial independente que é executado antes de uma mudança de modelo solicitada, PostModelSwitch como um evento assíncrono independente que é executado após as mudanças de modelo da sessão, e MessageDisplay como um evento somente de exibição que é executado enquanto o texto da mensagem do assistente é transmitido" width="520" height="1336" data-path="images/hooks-lifecycle-dark.svg" />
  </Frame>
</div>

A tabela abaixo resume quando cada evento dispara. A seção [Eventos de hook](#hook-events) documenta o esquema de entrada completo e as opções de controle de decisão para cada um.

| Evento                | Quando dispara                                                                                                                                                                                                                                                                                                          |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | Quando uma sessão começa ou é retomada                                                                                                                                                                                                                                                                                  |
| `Setup`               | Quando você inicia Claude Code com `--init-only`, ou com `--init` ou `--maintenance` no modo `-p`. Para preparação única em CI ou scripts                                                                                                                                                                               |
| `UserPromptSubmit`    | Quando você envia um prompt, antes de Claude processá-lo                                                                                                                                                                                                                                                                |
| `UserPromptExpansion` | Quando um comando digitado pelo usuário se expande em um prompt, antes de chegar a Claude. Pode bloquear a expansão                                                                                                                                                                                                     |
| `PreToolUse`          | Antes de uma chamada de ferramenta ser executada. Pode bloqueá-la                                                                                                                                                                                                                                                       |
| `PermissionRequest`   | Quando uma chamada de ferramenta precisa de uma decisão de permissão                                                                                                                                                                                                                                                    |
| `PermissionDenied`    | Quando o modo automático nega uma chamada de ferramenta, incluindo negações sem um veredicto do classificador. Use JSON `hookSpecificOutput.retry: true` para informar ao modelo que ele pode tentar novamente a chamada de ferramenta negada. Claude Code ignora `retry` quando o classificador não produziu veredicto |
| `PostToolUse`         | Depois que uma chamada de ferramenta é bem-sucedida                                                                                                                                                                                                                                                                     |
| `PostToolUseFailure`  | Depois que uma chamada de ferramenta falha                                                                                                                                                                                                                                                                              |
| `PostToolBatch`       | Depois que um lote completo de chamadas de ferramenta paralelas é resolvido, antes da próxima chamada do modelo                                                                                                                                                                                                         |
| `Notification`        | Quando Claude Code envia uma notificação                                                                                                                                                                                                                                                                                |
| `MessageDisplay`      | Enquanto o texto da mensagem do assistente está sendo exibido                                                                                                                                                                                                                                                           |
| `SubagentStart`       | Quando um subagente é criado                                                                                                                                                                                                                                                                                            |
| `SubagentStop`        | Quando um subagente termina                                                                                                                                                                                                                                                                                             |
| `TaskCreated`         | Quando uma tarefa está sendo criada via `TaskCreate`                                                                                                                                                                                                                                                                    |
| `TaskCompleted`       | Quando uma tarefa está sendo marcada como concluída                                                                                                                                                                                                                                                                     |
| `Stop`                | Quando Claude termina de responder                                                                                                                                                                                                                                                                                      |
| `StopFailure`         | Quando a rodada termina devido a um erro de API                                                                                                                                                                                                                                                                         |
| `TeammateIdle`        | Quando um colega de [equipe de agentes](/docs/pt/agent-teams) está prestes a ficar ocioso                                                                                                                                                                                                                                    |
| `InstructionsLoaded`  | Quando um arquivo CLAUDE.md ou `.claude/rules/*.md` é carregado no contexto. Dispara no início da sessão e quando os arquivos são carregados lentamente durante uma sessão                                                                                                                                              |
| `ConfigChange`        | Quando um arquivo de configuração muda durante uma sessão                                                                                                                                                                                                                                                               |
| `CwdChanged`          | Quando o diretório de trabalho muda, por exemplo quando Claude executa um comando `cd`. Útil para gerenciamento reativo do ambiente com ferramentas como direnv                                                                                                                                                         |
| `DirectoryAdded`      | Quando um diretório de trabalho é adicionado no meio da sessão via `/add-dir` ou a solicitação de controle SDK `register_repo_root`                                                                                                                                                                                     |
| `FileChanged`         | Quando um arquivo observado muda no disco. O campo `matcher` especifica quais nomes de arquivo observar                                                                                                                                                                                                                 |
| `WorktreeCreate`      | Quando um worktree está sendo criado via `--worktree`, `isolation: "worktree"`, ou para uma sessão em segundo plano. Substitui o comportamento padrão do git                                                                                                                                                            |
| `WorktreeRemove`      | Quando um worktree está sendo removido na saída da sessão, quando um subagente termina, ou quando você exclui uma sessão em segundo plano                                                                                                                                                                               |
| `PreCompact`          | Antes da compactação de contexto                                                                                                                                                                                                                                                                                        |
| `PostCompact`         | Depois que a compactação de contexto é concluída                                                                                                                                                                                                                                                                        |
| `PreModelSwitch`      | Antes de Claude Code aplicar uma mudança de modelo que você ou um cliente solicitou. Pode bloquear a mudança                                                                                                                                                                                                            |
| `PostModelSwitch`     | Depois que o modelo da sessão muda, incluindo mudanças que Claude Code faz por conta própria, como restaurar o modelo quando você retoma uma sessão                                                                                                                                                                     |
| `Elicitation`         | Quando um servidor MCP solicita entrada do usuário durante uma chamada de ferramenta                                                                                                                                                                                                                                    |
| `ElicitationResult`   | Depois que um usuário responde a uma elicitação MCP, antes da resposta ser enviada de volta ao servidor                                                                                                                                                                                                                 |
| `SessionEnd`          | Quando uma sessão é encerrada                                                                                                                                                                                                                                                                                           |

<h3 id="how-a-hook-resolves">
  Como um hook é resolvido
</h3>

Para ver como o evento, o matcher e o manipulador se encaixam, considere este hook `PreToolUse` que bloqueia comandos shell destrutivos.

<Tabs>
  <Tab title="macOS/Linux">
    O `matcher` se restringe a chamadas de ferramenta Bash e a condição `if` se restringe ainda mais a subcomandos Bash correspondendo a `rm *`, então `block-rm.sh` apenas é gerado quando ambos os filtros correspondem:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "if": "Bash(rm *)",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```

    O script lê a entrada JSON de stdin, extrai o comando e retorna uma `permissionDecision` de `"deny"` se contiver `rm -rf`. Salve-o em `.claude/hooks/block-rm.sh` em seu projeto e torne-o executável com `chmod +x .claude/hooks/block-rm.sh` para que Claude Code possa executá-lo:

    ```bash theme={null}
    #!/bin/bash
    # .claude/hooks/block-rm.sh
    COMMAND=$(jq -r '.tool_input.command')

    if echo "$COMMAND" | grep -q 'rm -rf'; then
      jq -n '{
        hookSpecificOutput: {
          hookEventName: "PreToolUse",
          permissionDecision: "deny",
          permissionDecisionReason: "Destructive command blocked by hook"
        }
      }'
    else
      exit 0  # no decision; normal permission flow applies
    fi
    ```

    Este script, como os outros exemplos Bash nesta página que analisam entrada JSON, usa `jq`, então instale `jq` e certifique-se de que está em seu `PATH` antes de tentar executá-los.
  </Tab>

  <Tab title="Windows (PowerShell)">
    O matcher `Bash|PowerShell` cobre a [ferramenta PowerShell](#powershell) bem como Bash. Uma única regra `if` corresponde apenas às chamadas de uma ferramenta, então cada ferramenta obtém seu próprio manipulador: o primeiro se restringe a subcomandos Bash correspondendo a `rm *`, o segundo a comandos PowerShell correspondendo a `Remove-Item *`. Ambos executam o mesmo script através de `powershell.exe`:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash|PowerShell",
            "hooks": [
              {
                "type": "command",
                "if": "Bash(rm *)",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.ps1"
                ]
              },
              {
                "type": "command",
                "if": "PowerShell(Remove-Item *)",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    A flag `-NoProfile` pula o carregamento de seu perfil PowerShell para que o hook inicie rapidamente, e `-ExecutionPolicy Bypass` permite que PowerShell execute o arquivo de script local.

    O script lê a entrada JSON de stdin, extrai o comando e retorna uma `permissionDecision` de `"deny"` se contiver `rm -rf` ou `Remove-Item` seguido por `-Recurse`. Salve-o em `.claude/hooks/block-rm.ps1` em seu projeto:

    ```powershell theme={null}
    # .claude/hooks/block-rm.ps1
    $callInput = [Console]::In.ReadToEnd() | ConvertFrom-Json
    $command = $callInput.tool_input.command

    if ($command -match 'rm -rf|Remove-Item.*-Recurse') {
      @{
        hookSpecificOutput = @{
          hookEventName = "PreToolUse"
          permissionDecision = "deny"
          permissionDecisionReason = "Destructive command blocked by hook"
        }
      } | ConvertTo-Json
    } else {
      exit 0  # no decision; normal permission flow applies
    }
    ```
  </Tab>
</Tabs>

Agora suponha que Claude Code decida executar `Bash "rm -rf /tmp/build"` contra a configuração macOS/Linux. Aqui está o que acontece:

<Frame>
  <img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/hook-resolution.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=be0bf3053550c26de5f54cd64674c197" className="dark:hidden" alt="Diagrama de resolução de hook: PreToolUse dispara, o matcher verifica correspondência de Bash, então a condição if verifica correspondência de Bash(rm *). Se ambos corresponderem, o comando do hook é executado e retorna permissionDecision deny, então a chamada da ferramenta é bloqueada e Claude Code continua. Se qualquer verificação falhar em corresponder, o hook é ignorado e a chamada da ferramenta é permitida prosseguir." width="930" height="270" data-path="images/hook-resolution.svg" />

  <img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/hook-resolution-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=e80af91f8507cee6bd51ac3c2dd92f63" className="hidden dark:block" alt="Diagrama de resolução de hook: PreToolUse dispara, o matcher verifica correspondência de Bash, então a condição if verifica correspondência de Bash(rm *). Se ambos corresponderem, o comando do hook é executado e retorna permissionDecision deny, então a chamada da ferramenta é bloqueada e Claude Code continua. Se qualquer verificação falhar em corresponder, o hook é ignorado e a chamada da ferramenta é permitida prosseguir." width="930" height="270" data-path="images/hook-resolution-dark.svg" />
</Frame>

<Steps>
  <Step title="Evento dispara">
    O evento `PreToolUse` dispara. Claude Code envia a entrada da ferramenta como JSON em stdin para o hook:

    ```json theme={null}
    { "tool_name": "Bash", "tool_input": { "command": "rm -rf /tmp/build" }, ... }
    ```
  </Step>

  <Step title="Matcher verifica">
    O matcher `"Bash"` corresponde ao nome da ferramenta, então este grupo de hook é ativado. Se você omitir o matcher ou usar `"*"`, o grupo é ativado em cada ocorrência do evento.
  </Step>

  <Step title="Condição if verifica">
    A condição `if` `"Bash(rm *)"` corresponde porque `rm -rf /tmp/build` é um subcomando correspondendo a `rm *`, então este manipulador é gerado. Se o comando tivesse sido `npm test`, a verificação `if` falharia e `block-rm.sh` nunca seria executado, evitando a sobrecarga de geração de processo. O campo `if` é opcional; sem ele, cada manipulador no grupo correspondido é executado.
  </Step>

  <Step title="Manipulador de hook executa">
    O script inspeciona o comando completo e encontra `rm -rf`, então imprime uma decisão em stdout:

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Destructive command blocked by hook"
      }
    }
    ```

    Se o comando tivesse sido uma variante mais segura de `rm` como `rm file.txt`, o script teria atingido `exit 0` em vez disso. Código de saída 0 sem saída significa que o hook não tem decisão a relatar, então a chamada da ferramenta continua através do [fluxo de permissão](/docs/pt/permissions) normal. O hook pode negar a chamada, mas ficar em silêncio não a aprova.
  </Step>

  <Step title="Claude Code age sobre o resultado">
    Claude Code lê a decisão JSON, bloqueia a chamada da ferramenta e mostra a razão ao Claude.
  </Step>
</Steps>

A seção [Configuração](#configuration) abaixo documenta o esquema completo, e cada seção [evento de hook](#hook-events) documenta qual entrada seu comando recebe e qual saída pode retornar.

<h2 id="configuration">
  Configuração
</h2>

Hooks são definidos em arquivos de configurações JSON. A configuração tem três níveis de aninhamento:

1. Escolha um [evento de hook](#hook-events) para responder, como `PreToolUse` ou `Stop`
2. Adicione um [grupo de matcher](#matcher-patterns) para filtrar quando dispara, como "apenas para a ferramenta Bash"
3. Defina um ou mais [manipuladores de hook](#hook-handler-fields) para executar quando correspondido

Consulte [Como um hook é resolvido](#how-a-hook-resolves) acima para um passo a passo completo com um exemplo anotado.

<Note>
  Esta página usa termos específicos para cada nível: **evento de hook** para o ponto do ciclo de vida, **grupo de matcher** para o filtro e **manipulador de hook** para o comando shell, endpoint HTTP, ferramenta MCP, prompt ou agente que executa. "Hook" por si só refere-se ao recurso geral.
</Note>

<h3 id="hook-locations">
  Locais de hooks
</h3>

Onde você define um hook determina seu escopo:

| Local                                             | Escopo                                                                                                              | Compartilhável                                                 |
| :------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------- |
| `~/.claude/settings.json`                         | Todos os seus projetos                                                                                              | Não, local para sua máquina                                    |
| `.claude/settings.json`                           | Projeto único                                                                                                       | Sim, pode ser confirmado no repositório                        |
| `.claude/settings.local.json`                     | Projeto único                                                                                                       | Não, gitignored quando Claude Code salva uma configuração nele |
| Configurações de política gerenciada              | Organização inteira                                                                                                 | Sim, controlado por administrador                              |
| [Plugin](/docs/pt/plugins/overview) `hooks/hooks.json` | Quando o plugin está ativado                                                                                        | Sim, agrupado com o plugin                                     |
| Frontmatter de [Skill](/docs/pt/skills)                | O resto da sessão uma vez que a skill é invocada. Consulte [Hooks em skills e agentes](#hooks-in-skills-and-agents) | Sim, definido no arquivo da skill                              |
| Frontmatter de [Subagent](/docs/pt/sub-agents)         | Enquanto esse subagente está em execução                                                                            | Sim, definido no arquivo do subagente                          |

Sessões em nuvem em [Claude Code na web](/docs/pt/claude-code-on-the-web) não leem seu `~/.claude/settings.json` local. Em um [ambiente auto-hospedado](/docs/pt/self-hosted-environments-configuration#permissions-and-tool-approval), Claude Code também executa os hooks que o operador propagou do `~/.claude/` do host do runner, e executa os hooks no arquivo de configurações gerenciadas da imagem do runner quando esse arquivo está entre as [fontes gerenciadas que Claude Code aplica](/docs/pt/managed-settings#how-claude-code-combines-managed-sources), o que por padrão significa apenas quando nem configurações gerenciadas pelo servidor nem uma política Claude Code entregue por MDM fornece o nível gerenciado. Consulte [o que é transferido da sua configuração](/docs/pt/cloud-environments#what-carries-over-from-your-setup) para saber quais arquivos de configurações e plugins, e portanto quais hooks, chegam a uma sessão em nuvem.

Para detalhes sobre resolução de arquivo de configurações, consulte [settings](/docs/pt/settings).

Hooks de arquivos de configurações, configurações de política gerenciada e plugins também executam dentro de [subagentes](/docs/pt/sub-agents). Quando um subagente chama uma ferramenta, eventos de ferramenta como `PreToolUse` e `PostToolUse` disparam os mesmos hooks configurados que na conversa principal, e a entrada carrega os campos de entrada comuns `agent_id` e `agent_type` [](#common-input-fields) que identificam o subagente.

Administradores corporativos podem usar `allowManagedHooksOnly` para restringir quais hooks executam:

* Seus hooks de usuário, projeto, local e plugin são bloqueados. Hooks de plugins forçadamente ativados em configurações gerenciadas `enabledPlugins` são isentos
* Claude Code também restringe suas configurações [`statusLine`](/docs/pt/statusline), [`fileSuggestion`](/docs/pt/settings-reference#filesuggestion) e [`subagentStatusLine`](/docs/pt/statusline#subagent-status-lines) às configurações gerenciadas
* Claude Code também desabilita plugins com uma [fonte `command`](/docs/pt/plugins/marketplace-reference#command-plugin-source), incluindo plugins forçadamente ativados em configurações gerenciadas `enabledPlugins`, a menos que [`disableCommandPluginSources`](/docs/pt/settings-reference#disablecommandpluginsources) seja explicitamente definido como `false`. Fontes `command` requerem Claude Code v2.1.229 ou posterior
* Claude Code também bloqueia [comandos `headersHelper`](/docs/pt/plugins/host-marketplace#authenticate-archive-downloads) do marketplace a menos que [`disableCommandPluginSources`](/docs/pt/settings-reference#disablecommandpluginsources) seja explicitamente definido como `false`, exceto para um marketplace que as próprias configurações gerenciadas declarem

Consulte [o que executa sob `allowManagedHooksOnly`](/docs/pt/settings-reference#what-runs-under-allowmanagedhooksonly).

Entradas de hook se mesclam entre níveis de configurações em vez de se substituírem: configurações de usuário, projeto e local adicionam seus próprios hooks sem remover os gerenciados, e a configuração [`disableAllHooks`](#disable-or-remove-hooks) não pode desabilitar hooks gerenciados de fora das configurações gerenciadas.

As [listas de permissões de hook HTTP](/docs/pt/settings-reference#hook-and-skill-settings) se aplicam a hooks de todas as fontes, incluindo configurações de política gerenciada:

* `allowedHttpHookUrls`: quando definido em qualquer nível de configurações, Claude Code executa um manipulador de hook HTTP apenas se sua URL corresponder à lista de permissões mesclada
* `httpHookAllowedEnvVars`: quando definido, Claude Code interpola apenas as variáveis de ambiente nessa lista em cabeçalhos de hook

<h3 id="matcher-patterns">
  Padrões de matcher
</h3>

O campo `matcher` filtra quando hooks disparam. Como um matcher é avaliado depende dos caracteres que contém:

| Valor do matcher                                      | Avaliado como                                                                                            | Exemplo                                                                                                                                                                                   |
| :---------------------------------------------------- | :------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"*"`, `""` ou omitido                                | Corresponder a todos                                                                                     | dispara em cada ocorrência do evento                                                                                                                                                      |
| Apenas letras, dígitos, `_`, `-`, espaços, `,` e `\|` | String exata ou lista de strings exatas separadas por `\|` ou `,` com espaço em branco opcional ao redor | `Bash` corresponde apenas à ferramenta Bash; `Edit\|Write` e `Edit, Write` cada um corresponde a qualquer ferramenta exatamente; `code-reviewer` corresponde apenas a esse tipo de agente |
| Contém qualquer outro caractere                       | Expressão regular JavaScript, não ancorada                                                               | `^Notebook` corresponde a qualquer ferramenta cujo nome começa com `Notebook`; `mcp__memory__.*` corresponde a cada ferramenta do servidor `memory`                                       |

Um matcher no caminho de expressão regular é testado com `RegExp.prototype.test` do JavaScript, que sucede em uma correspondência em qualquer lugar no valor. `Edit.*` corresponde tanto a `Edit` quanto a `NotebookEdit`; envolva o padrão em `^` e `$`, como em `^Edit$`, quando você precisa de uma correspondência de string inteira.

Hífens no conjunto de correspondência exata requerem Claude Code v2.1.195 ou posterior. Em versões anteriores, um nome com hífen como `code-reviewer` é avaliado como uma expressão regular não ancorada, então também dispara para `senior-code-reviewer`; ancorá-lo como `^code-reviewer$` nessas versões para corresponder apenas a esse nome.

`FileChanged` e `StopFailure` usam um conjunto de correspondência exata mais estreito de apenas letras, dígitos, `_` e `|`. Um hífen, espaço ou vírgula em um matcher para esses dois eventos o mantém no caminho de expressão regular, e apenas `|` separa alternativas. Todos os outros eventos com suporte a matcher na tabela a seguir aceitam `|` ou `,`.

O evento `FileChanged` não segue essas regras ao construir sua lista de monitoramento. Consulte [FileChanged](#filechanged).

Cada tipo de evento corresponde em um campo diferente:

| Evento                                                                                                                                            | O que o matcher filtra                                                                                    | Valores de matcher de exemplo                                                                                                                                                                                                                                                  |
| :------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`                                                        | nome da ferramenta                                                                                        | `Bash`, `Edit\|Write`, `mcp__.*`                                                                                                                                                                                                                                               |
| `SessionStart`                                                                                                                                    | como a sessão começou                                                                                     | `startup`, `resume`, `clear`, `compact`, `fork`                                                                                                                                                                                                                                |
| `Setup`                                                                                                                                           | qual sinalizador CLI acionou a configuração                                                               | `init`, `maintenance`                                                                                                                                                                                                                                                          |
| `SessionEnd`                                                                                                                                      | por que a sessão terminou                                                                                 | `clear`, `resume`, `logout`, `prompt_input_exit`, `other`                                                                                                                                                                                                                      |
| `Notification`                                                                                                                                    | tipo de notificação                                                                                       | `permission_prompt`, `idle_prompt`, `auth_success`, `elicitation_dialog`, `elicitation_url_dialog`, `elicitation_complete`, `elicitation_response`, `agent_needs_input`, `agent_completed`, `quota_auto_resume_fired`, `quota_auto_resume_stale`, `quota_auto_resume_disabled` |
| `SubagentStart`                                                                                                                                   | tipo de agente                                                                                            | `general-purpose`, `Explore`, `Plan`, nomes de agentes personalizados ou nomes com escopo de plugin como `^my-plugin:reviewer$`                                                                                                                                                |
| `PreCompact`, `PostCompact`                                                                                                                       | o que acionou a compactação                                                                               | `manual`, `auto`                                                                                                                                                                                                                                                               |
| `PreModelSwitch`, `PostModelSwitch`                                                                                                               | nome canônico do modelo para o qual a sessão muda, conforme descrito em [PreModelSwitch](#premodelswitch) | `claude-opus-5`, `claude-opus-4-6\|claude-opus-5`, `.*opus.*`                                                                                                                                                                                                                  |
| `SubagentStop`                                                                                                                                    | tipo de agente                                                                                            | mesmos valores que `SubagentStart`                                                                                                                                                                                                                                             |
| `ConfigChange`                                                                                                                                    | fonte de configuração                                                                                     | `user_settings`, `project_settings`, `local_settings`, `policy_settings`, `skills`                                                                                                                                                                                             |
| `CwdChanged`                                                                                                                                      | sem suporte a matcher                                                                                     | sempre dispara em cada ocorrência                                                                                                                                                                                                                                              |
| `DirectoryAdded`                                                                                                                                  | como o diretório foi adicionado                                                                           | `slash_command`, `register_repo_root`                                                                                                                                                                                                                                          |
| `FileChanged`                                                                                                                                     | nomes de arquivo literais para monitorar (consulte [FileChanged](#filechanged))                           | `.envrc\|.env`                                                                                                                                                                                                                                                                 |
| `StopFailure`                                                                                                                                     | tipo de erro                                                                                              | `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, `unknown`                                               |
| `InstructionsLoaded`                                                                                                                              | razão de carregamento                                                                                     | `session_start`, `nested_traversal`, `path_glob_match`, `include`, `compact`                                                                                                                                                                                                   |
| `UserPromptExpansion`                                                                                                                             | nome do comando                                                                                           | seus nomes de skill ou comando                                                                                                                                                                                                                                                 |
| `Elicitation`                                                                                                                                     | nome do servidor MCP                                                                                      | seus nomes de servidor MCP configurados                                                                                                                                                                                                                                        |
| `ElicitationResult`                                                                                                                               | nome do servidor MCP                                                                                      | mesmos valores que `Elicitation`                                                                                                                                                                                                                                               |
| `UserPromptSubmit`, `PostToolBatch`, `Stop`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `WorktreeCreate`, `WorktreeRemove`, `MessageDisplay` | sem suporte a matcher                                                                                     | sempre dispara em cada ocorrência                                                                                                                                                                                                                                              |

Corresponder `StopFailure` em `cloud_credential_error` requer Claude Code v2.1.267 ou posterior, a primeira versão que relata falhas de carregamento de credenciais sob esse valor em vez de `server_error` ou `unknown`.

Para a maioria dos eventos, Claude Code avalia o matcher contra um campo da [entrada JSON](#hook-input-and-output) que envia para seu hook em stdin. Para eventos de ferramenta, esse campo é `tool_name`. Para `PreModelSwitch` e `PostModelSwitch`, Claude Code avalia o matcher contra o nome canônico que deriva de `to_model`, conforme descrito em [PreModelSwitch](#premodelswitch). Cada seção [evento de hook](#hook-events) lista o conjunto completo de valores de matcher e o esquema de entrada para esse evento.

Este exemplo executa um script de linting apenas quando Claude escreve ou edita um arquivo:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/lint-check.sh"
          }
        ]
      }
    ]
  }
}
```

Se você adicionar um campo `matcher` a um evento sem suporte a matcher, ele é silenciosamente ignorado.

Para eventos de ferramenta, você pode filtrar mais estreitamente definindo o campo [`if`](#common-fields) em manipuladores de hook individuais. `if` usa [sintaxe de regra de permissão](/docs/pt/permissions) para corresponder contra o nome da ferramenta e argumentos juntos, então `"Bash(git *)"` executa quando qualquer subcomando da entrada Bash corresponde a `git *` e `"Edit(*.ts)"` executa apenas para arquivos TypeScript.

<h4 id="match-mcp-tools">
  Corresponder ferramentas MCP
</h4>

Ferramentas de servidor [MCP](/docs/pt/mcp) aparecem como ferramentas regulares em eventos de ferramenta (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, `PermissionDenied`), então você pode corresponder a elas da mesma forma que corresponde a qualquer outro nome de ferramenta.

Ferramentas MCP seguem o padrão de nomenclatura `mcp__<server>__<tool>`, por exemplo:

* `mcp__memory__create_entities`: ferramenta create entities do servidor Memory
* `mcp__filesystem__read_file`: ferramenta read file do servidor Filesystem
* `mcp__github__search_repositories`: ferramenta search do servidor GitHub

Para corresponder a cada ferramenta de um servidor, anexe `.*` ao prefixo do servidor. O `.*` é obrigatório: um matcher como `mcp__memory` ou `mcp__brave-search` contém apenas caracteres de correspondência exata, então é comparado como uma string exata e não corresponde a nenhuma ferramenta.

* `mcp__memory__.*` corresponde a todas as ferramentas do servidor `memory`
* `mcp__brave-search__.*` corresponde a todas as ferramentas de um servidor cujo nome contém um hífen
* `mcp__.*__write.*` corresponde a qualquer ferramenta cujo nome começa com `write` de qualquer servidor

Hífens no conjunto de correspondência exata requerem Claude Code v2.1.195 ou posterior. Em versões anteriores, um prefixo com hífen simples como `mcp__brave-search` é avaliado como uma expressão regular não ancorada e corresponde a cada ferramenta daquele servidor. A forma `mcp__brave-search__.*` funciona em todas as versões.

Ferramentas de um [servidor MCP fornecido por plugin](/docs/pt/mcp#plugin-provided-mcp-servers) usam um segmento de servidor com escopo que inclui o nome do plugin: `mcp__plugin_<plugin-name>_<server-name>__<tool>`. Um matcher escrito contra a chave do servidor simples nunca dispara para essas ferramentas. Para um plugin nomeado `my-plugin` que agrupa um servidor sob a chave `db`, uma ferramenta `query` aparece como `mcp__plugin_my-plugin_db__query`, então o matcher para cada ferramenta daquele servidor é `mcp__plugin_my-plugin_db__.*`. Use o mesmo nome de ferramenta com escopo no campo [`if`](#common-fields) de um manipulador. Consulte [Servidores MCP fornecidos por plugin](/docs/pt/mcp#plugin-provided-mcp-servers) para saber como o nome com escopo é construído.

Este exemplo registra todas as operações do servidor memory e valida operações de escrita de qualquer servidor MCP:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "mcp__memory__.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Memory operation initiated' >> ~/mcp-operations.log"
          }
        ]
      },
      {
        "matcher": "mcp__.*__write.*",
        "hooks": [
          {
            "type": "command",
            "command": "/home/user/scripts/validate-mcp-write.py"
          }
        ]
      }
    ]
  }
}
```

<h3 id="hook-handler-fields">
  Campos do manipulador de hook
</h3>

Cada objeto no array `hooks` interno é um manipulador de hook: o comando shell, endpoint HTTP, ferramenta MCP, prompt LLM ou agente que executa quando o matcher corresponde. Existem cinco tipos:

* **[Hooks de comando](#command-hook-fields)** (`type: "command"`): executam um comando shell. Seu script recebe a [entrada JSON](#hook-input-and-output) do evento em stdin e comunica resultados através de códigos de saída e stdout.
* **[Hooks HTTP](#http-hook-fields)** (`type: "http"`): enviam a entrada JSON do evento como uma solicitação HTTP POST para uma URL. O endpoint comunica resultados através do corpo da resposta usando o mesmo [formato de saída JSON](#json-output) que hooks de comando.
* **[Hooks de ferramenta MCP](#mcp-tool-hook-fields)** (`type: "mcp_tool"`): chamam uma ferramenta em um servidor [MCP](/docs/pt/mcp) já conectado. A saída de texto da ferramenta é tratada como stdout de hook de comando.
* **[Hooks de prompt](#prompt-and-agent-hook-fields)** (`type: "prompt"`): enviam um prompt para um modelo Claude para avaliação de turno único. O modelo retorna sua decisão como JSON. Consulte [Hooks baseados em prompt](#prompt-based-hooks).
* **[Hooks de agente](#prompt-and-agent-hook-fields)** (`type: "agent"`): geram um subagente que pode usar ferramentas como Read, Grep e Glob para verificar condições antes de retornar uma decisão. Hooks de agente são experimentais e podem mudar. Consulte [Hooks baseados em agente](#agent-based-hooks).

Todos os hooks correspondentes executam em paralelo. Se você definir o mesmo manipulador em mais de um arquivo de configurações, ele executa uma vez. Uma cópia do mesmo manipulador de um plugin ou skill permanece separada.

Manipuladores executam no diretório atual com o ambiente do Claude Code. Se o diretório atual não existir mais, por exemplo uma worktree ou diretório temporário que outro shell deletou no meio da sessão, Claude Code executa hooks de comando a partir do primeiro destes que ainda existe: o diretório em que a sessão começou, a raiz do projeto, seu diretório home ou o diretório temporário do sistema. Claude Code registra um aviso nomeando o diretório de fallback no [log de debug](#debug-hooks).

A variável de ambiente `$CLAUDE_CODE_REMOTE` é `"true"` em ambientes web remotos e não é definida na CLI local. Claude Code v2.1.199 e posterior define [`$CLAUDE_CODE_BRIDGE_SESSION_ID`](/docs/pt/env-vars) para o ID de sessão [Remote Control](/docs/pt/remote-control) enquanto a sessão local tem uma conexão Remote Control ativa.

<h4 id="common-fields">
  Campos comuns
</h4>

Esses campos se aplicam a todos os tipos de hook:

| Campo           | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :-------------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`          | sim         | `"command"`, `"http"`, `"mcp_tool"`, `"prompt"` ou `"agent"`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `if`            | não         | Sintaxe de regra de permissão para filtrar quando este hook executa, como `"Bash(git *)"` ou `"Edit(*.ts)"`. O comando do hook apenas é executado se a chamada de ferramenta corresponde ao padrão. Consulte a [tabela de correspondência Bash](#bash-if-matching) abaixo para saber como padrões Bash são avaliados contra subcomandos, `$()` e backticks. Apenas avaliado em eventos de ferramenta: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` e `PermissionDenied`. Em outros eventos, um hook com `if` definido nunca executa. Usa a mesma sintaxe que [regras de permissão](/docs/pt/permissions)                                                                          |
| `timeout`       | não         | Segundos antes de cancelar. Claude Code não o impõe em um hook de comando que você executa com [`async: true`](#run-hooks-in-the-background). Padrões: 600 para `command`, `http` e `mcp_tool`; 30 para `prompt`; 60 para `agent`. Claude Code reduz o padrão de `command`, `http` e `mcp_tool` para 30 em [`UserPromptSubmit`](#userpromptsubmit), [`PreModelSwitch`](#premodelswitch) e [`PostModelSwitch`](#postmodelswitch), e para 10 em [`MessageDisplay`](#messagedisplay). Hooks de [`SessionEnd`](#sessionend) compartilham um orçamento de 1,5 segundo; se suas configurações definirem um `timeout` por hook mais longo, Claude Code aumenta o orçamento para corresponder, até 60 segundos |
| `statusMessage` | não         | Mensagem de spinner personalizada exibida enquanto o hook executa                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `once`          | não         | Se `true`, Claude Code remove o hook após sua primeira execução bem-sucedida. Uma execução que falha, bloqueia com código de saída 2 ou expira deixa o hook em vigor, então ele executa novamente no próximo evento correspondente. Apenas honrado para hooks declarados em [frontmatter de skill](#hooks-in-skills-and-agents); ignorado em arquivos de configurações e frontmatter de agente                                                                                                                                                                                                                                                                                                         |

O campo `if` contém exatamente uma regra de permissão. Não há sintaxe `&&`, `||` ou lista para combinar regras; para aplicar múltiplas condições, defina um manipulador de hook separado para cada.

Em uma condição `if` para uma ferramenta de arquivo, um padrão de diretório de segmento único como `"Edit(src/**)"` corresponde apenas ao diretório `src` no diretório de trabalho e aos arquivos sob ele. Para corresponder a um diretório nomeado `src` em qualquer profundidade, escreva `"Edit(**/src/**)"`. Antes de v2.1.214, `"Edit(src/**)"` correspondia a um diretório nomeado `src` em qualquer profundidade sob o diretório de trabalho.

<span id="bash-if-matching" />Para padrões Bash, se seu comando de hook executa depende da forma do padrão e do comando Bash que Claude está invocando. Atribuições `VAR=value` iniciais são removidas antes da correspondência.

| padrão `if`        | Comando Bash                | Hook executa? | Por quê                                                                                                                                             |
| :----------------- | :-------------------------- | :------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Bash(git *)`      | `FOO=bar git push`          | sim           | atribuições iniciais são removidas; `git push` corresponde                                                                                          |
| `Bash(git *)`      | `npm test && git push`      | sim           | cada subcomando é verificado; `git push` corresponde                                                                                                |
| `Bash(rm *)`       | `echo $(rm -rf /)`          | sim           | comandos dentro de `$()` e backticks são verificados; `rm -rf /` corresponde                                                                        |
| `Bash(rm *)`       | `echo $(date)`              | não           | nenhum subcomando corresponde a `rm *`                                                                                                              |
| `Bash(cat *)`      | `echo before $(date) after` | não           | uma substituição pode estar em qualquer posição de argumento, então o comando completo e `date` são ambos verificados; nenhum corresponde a `cat *` |
| `Bash(git *)`      | `$TOOL git push`            | sim           | Claude Code não pode dizer para o que o nome do comando se expande, então executa o hook                                                            |
| `Bash(git push *)` | `echo $(date)`              | sim           | padrões que especificam mais do que o nome do comando executam o hook mesmo assim em `$()`, backticks ou `$VAR`                                     |

Quando Claude Code não pode determinar quais comandos a entrada Bash executa, ele executa seu hook independentemente do padrão. Como o filtro `if` é melhor esforço, use o [sistema de permissão](/docs/pt/permissions) em vez de um hook para impor um allow ou deny duro.

<h4 id="command-hook-fields">
  Campos de hook de comando
</h4>

Além dos [campos comuns](#common-fields), hooks de comando aceitam esses campos:

| Campo         | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                                         |
| :------------ | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `command`     | sim         | Comando shell a executar. Com `args`, o executável a gerar diretamente. Consulte [Forma exec e forma shell](#exec-form-and-shell-form)                                                                                                                                                                                                            |
| `args`        | não         | Lista de argumentos. Quando presente, `command` é resolvido como um executável e gerado diretamente com `args` como o vetor de argumentos, sem shell envolvido. Consulte [Forma exec e forma shell](#exec-form-and-shell-form)                                                                                                                    |
| `async`       | não         | Se `true`, executa em background sem bloquear. Consulte [Executar hooks em background](#run-hooks-in-the-background)                                                                                                                                                                                                                              |
| `asyncRewake` | não         | Se `true`, executa em background e acorda Claude na saída do código 2. O stderr do hook, ou stdout se stderr estiver vazio, é mostrado ao Claude como um lembrete do sistema para que possa reagir a uma falha de background de longa duração                                                                                                     |
| `shell`       | não         | Shell a usar para este hook. Aceita `"bash"` ou `"powershell"`. Padrão é `"bash"`, ou `"powershell"` no Windows quando Git Bash não está instalado. Definir `"powershell"` executa o comando via PowerShell no Windows. Não requer `CLAUDE_CODE_USE_POWERSHELL_TOOL` já que hooks geram PowerShell diretamente. Ignorado quando `args` é definido |

<a id="exec-form-and-shell-form" />

<h5 id="exec-form-and-shell-form">
  Forma exec e forma shell
</h5>

Um hook de comando executa como forma exec quando `args` é definido, e forma shell quando `args` é omitido. Defina `args` sempre que o hook referenciar um [placeholder de caminho](#reference-scripts-by-path), já que cada elemento é passado como um argumento sem aspas. Omita `args` quando você precisar de recursos de shell como pipes ou `&&`, ou quando nenhuma preocupação se aplica.

**Forma exec** executa quando `args` está presente. Claude Code resolve `command` como um executável em `PATH` e o gera diretamente com `args` como o vetor de argumentos. Não há shell, então cada elemento `args` é um argumento exatamente como escrito, e placeholders de caminho como `${CLAUDE_PLUGIN_ROOT}` são substituídos em `command` e em cada elemento `args` como strings simples. Caracteres especiais como apóstrofos, `$` e backticks passam verbatim porque não há shell para interpretá-los. Nenhuma tokenização de shell acontece em nenhuma plataforma.

**Forma shell** executa quando `args` está ausente. A string `command` é passada para um shell: `sh -c` em macOS e Linux, Git Bash no Windows, ou PowerShell quando Git Bash não está instalado. Defina o campo `shell` para escolher explicitamente. O shell tokeniza a string, expande variáveis e interpreta pipes, `&&`, redirecionamentos e globs.

<Note>
  No Windows, a forma exec requer que `command` seja resolvido para um executável real como `.exe`. Os shims `.cmd` e `.bat` que npm, npx, eslint e outras ferramentas instalam em `node_modules/.bin` não são executáveis e não podem ser gerados sem um shell. Para executá-los em forma exec, invoque o script subjacente com `node` diretamente, por exemplo `"command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/node_modules/eslint/bin/eslint.js"]`. O padrão `node` mais caminho-de-script funciona em todas as plataformas porque `node.exe` é um binário real. Para executar um shim `.cmd` ou `.bat` por nome, use forma shell.
</Note>

Este exemplo executa um script Node agrupado com um plugin. A forma exec passa o caminho do script resolvido como um argumento sem aspas:

```json theme={null}
{
  "type": "command",
  "command": "node",
  "args": ["${CLAUDE_PLUGIN_ROOT}/scripts/format.js", "--fix"]
}
```

A forma shell equivalente precisa de aspas para lidar com caminhos com espaços ou caracteres especiais:

```json theme={null}
{
  "type": "command",
  "command": "node \"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.js --fix"
}
```

Ambas as formas suportam os mesmos [placeholders de caminho](#reference-scripts-by-path), e ambas os exportam como as variáveis de ambiente `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT` e `CLAUDE_PLUGIN_DATA` no processo gerado, então um script pode ler `process.env.CLAUDE_PLUGIN_ROOT` independentemente de como foi lançado.

Hooks de plugin adicionalmente substituem valores [`${user_config.*}`](/docs/pt/plugins/manifest-reference#user-configuration), apenas em forma exec: o valor é substituído em `command` e em cada elemento `args` como uma string simples, então nenhum shell o re-analisa.

Um hook de plugin em forma shell cujo `command` referencia `${user_config.*}` falha com um [erro](/docs/pt/errors#plugin-command-references-user-config) em vez de executar. Para usar um valor de opção de um hook em forma shell, leia a variável de ambiente `$CLAUDE_PLUGIN_OPTION_<KEY>`, como `$CLAUDE_PLUGIN_OPTION_WEBHOOK_URL` para uma opção `webhook_url`, ou defina `args` para mudar o hook para forma exec. Antes de v2.1.207, comandos de hook de plugin em forma shell também substituíam `${user_config.*}`.

<Note>
  Em forma exec, `command` é apenas o nome ou caminho do executável. Se `command` é um nome simples sem separador de caminho e contém espaço em branco junto com `args`, Claude Code registra um aviso porque o spawn falhará: não há executável nomeado `node script.js`. Mova os tokens extras para `args`. Caminhos absolutos com espaços, como `C:\Program Files\nodejs\node.exe`, são um executável válido único e não disparam o aviso.
</Note>

<h4 id="http-hook-fields">
  Campos de hook HTTP
</h4>

Além dos [campos comuns](#common-fields), hooks HTTP aceitam esses campos:

| Campo            | Obrigatório | Descrição                                                                                                                                                                                                                                      |
| :--------------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`            | sim         | URL para enviar a solicitação POST                                                                                                                                                                                                             |
| `headers`        | não         | Cabeçalhos HTTP adicionais como pares chave-valor. Valores suportam interpolação de variável de ambiente usando sintaxe `$VAR_NAME` ou `${VAR_NAME}`. Apenas variáveis listadas em `allowedEnvVars` são resolvidas                             |
| `allowedEnvVars` | não         | Lista de nomes de variáveis de ambiente que podem ser interpoladas em valores de cabeçalho. Referências a variáveis não listadas são substituídas por strings vazias. Obrigatório para qualquer interpolação de variável de ambiente funcionar |

Claude Code envia a [entrada JSON](#hook-input-and-output) do hook como corpo da solicitação POST com `Content-Type: application/json`. O corpo da resposta usa o mesmo [formato de saída JSON](#json-output) que hooks de comando.

O tratamento de erros difere dos hooks de comando; consulte [Tratamento de resposta HTTP](#http-response-handling).

Este exemplo envia eventos `PreToolUse` para um serviço de validação local, autenticando com um token da variável de ambiente `MY_TOKEN`:

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/pre-tool-use",
            "timeout": 30,
            "headers": {
              "Authorization": "Bearer $MY_TOKEN"
            },
            "allowedEnvVars": ["MY_TOKEN"]
          }
        ]
      }
    ]
  }
}
```

<h4 id="mcp-tool-hook-fields">
  Campos de hook de ferramenta MCP
</h4>

Além dos [campos comuns](#common-fields), hooks de ferramenta MCP aceitam esses campos:

| Campo    | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                            |
| :------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `server` | sim         | Nome de um servidor MCP configurado. Para um [servidor fornecido por plugin](/docs/pt/mcp#plugin-provided-mcp-servers), este é o nome com escopo `plugin:<plugin-name>:<server-name>`, como `plugin:my-plugin:db`, não a chave do servidor simples. O servidor já deve estar conectado; o hook nunca dispara um fluxo OAuth ou de conexão |
| `tool`   | sim         | Nome da ferramenta a chamar naquele servidor                                                                                                                                                                                                                                                                                         |
| `input`  | não         | Argumentos passados para a ferramenta. Valores de string suportam substituição `${path}` da [entrada JSON](#hook-input-and-output) do hook, como `"${tool_input.file_path}"`                                                                                                                                                         |

Claude Code lê o conteúdo de texto da ferramenta da mesma forma que lê stdout de hook de comando, seguindo a [regra de análise sob código de saída 0](#exit-code-0). Se o servidor nomeado não estiver conectado, ou a ferramenta retornar `isError: true`, o hook produz um erro não-bloqueador e a execução continua.

Este exemplo chama a ferramenta `security_scan` no servidor MCP `my_server` após cada `Write` ou `Edit`, passando o caminho do arquivo editado:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "mcp_tool",
            "server": "my_server",
            "tool": "security_scan",
            "input": { "file_path": "${tool_input.file_path}" }
          }
        ]
      }
    ]
  }
}
```

Um hook `mcp_tool` pode executar apenas uma vez que Claude Code tenha disponibilizado os servidores MCP da sessão para hooks. `SessionStart` e `Setup` podem disparar antes desse ponto:

* **No lançamento**: `SessionStart` dispara antes dos servidores estarem disponíveis, incluindo quando você lança com `--continue` ou `--resume`. Claude Code pula os hooks `mcp_tool` do evento sem chamar suas ferramentas, e o [log de debug](#debug-hooks) registra `mcp_tool hooks are not available for the 'SessionStart' hook event (no MCP client context)`.
* **Mais tarde em uma sessão em execução**: após `/clear` ou uma compactação, `SessionStart` dispara novamente com os servidores já disponíveis, e seus hooks `mcp_tool` executam.
* **Em `Setup`**: `Setup` sempre dispara antes dos servidores estarem disponíveis, então Claude Code pula seus hooks `mcp_tool` toda vez e registra a mesma mensagem nomeando `Setup`.

Por exemplo, esta configuração chama a ferramenta `load_context` no servidor MCP `my_server` de um hook `SessionStart` sem matcher, então se aplica a cada fonte `SessionStart`:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "mcp_tool",
            "server": "my_server",
            "tool": "load_context"
          }
        ]
      }
    ]
  }
}
```

Quando você executa `claude`, Claude Code pula este hook, nunca chama `load_context` e escreve a mensagem `no MCP client context` no log de debug. Execute `/clear` nessa mesma sessão e o hook executa e chama `load_context`. Um hook `type: "command"` em `SessionStart` executa no lançamento, então use um para qualquer coisa que a sessão precise de seu primeiro turno.

<h4 id="prompt-and-agent-hook-fields">
  Campos de hook de prompt e agente
</h4>

Além dos [campos comuns](#common-fields), hooks de prompt e agente aceitam esses campos:

| Campo    | Obrigatório | Descrição                                                                                                                                                                                         |
| :------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `prompt` | sim         | Texto do prompt a enviar para o modelo. Use `$ARGUMENTS` como placeholder para a entrada JSON do hook. Escape com uma barra invertida para incluir texto literal: `\$1.00` renderiza como `$1.00` |
| `model`  | não         | Modelo a usar para avaliação. Padrão para um modelo rápido                                                                                                                                        |

<h3 id="reference-scripts-by-path">
  Referenciar scripts por caminho
</h3>

Use esses placeholders para referenciar scripts de hook relativos à raiz do projeto ou plugin, independentemente do diretório de trabalho quando o hook executa:

* `${CLAUDE_PROJECT_DIR}`: a raiz do projeto onde a sessão começou. Claude Code também define essa variável no ambiente de [servidores MCP stdio](/docs/pt/mcp#option-3-add-a-local-stdio-server) e servidores LSP de plugin.
* `${CLAUDE_PLUGIN_ROOT}`: o diretório de instalação do plugin, para scripts agrupados com um [plugin](/docs/pt/plugins/overview). Consulte [variáveis de ambiente de plugin](/docs/pt/plugins/manifest-reference#environment-variables) para saber como o caminho se comporta entre atualizações.
* `${CLAUDE_PLUGIN_DATA}`: o [diretório de dados persistentes](/docs/pt/plugins/components#path-variables-and-persistent-data) do plugin, para dependências e estado que devem sobreviver a atualizações de plugin.

<Note>
  **Worktrees são diferentes.** Se Claude entra em uma [worktree](/docs/pt/worktrees) durante a sessão, Claude Code mantém `${CLAUDE_PROJECT_DIR}` onde estava e passa o caminho da worktree para seus hooks de uma forma diferente:

  * **`${CLAUDE_PROJECT_DIR}` fica no lugar**: ainda aponta para a raiz do projeto onde a sessão começou, então um comando como `${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh` ainda executa o script no checkout principal.
  * **`cwd` segue Claude**: o campo `cwd` na [entrada JSON](#common-input-fields) do hook é a raiz da worktree após Claude entrar em uma worktree, e o novo diretório após Claude executar `cd`. Leia-o quando um hook precisa saber em qual diretório Claude está trabalhando.
</Note>

Prefira [forma exec](#exec-form-and-shell-form) para qualquer hook que referencie um placeholder de caminho. Em forma shell, envolva cada placeholder em aspas duplas.

<Tabs>
  <Tab title="Scripts de projeto">
    Este exemplo usa `${CLAUDE_PROJECT_DIR}` para executar um verificador de estilo do diretório `.claude/hooks/` do projeto após qualquer chamada de ferramenta `Write` ou `Edit`:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/check-style.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Scripts de plugin">
    Defina hooks de plugin em `hooks/hooks.json` com um campo `description` opcional de nível superior. Quando um plugin está ativado, seus hooks se mesclam com seus hooks de usuário e projeto.

    Este exemplo executa um script de formatação agrupado com o plugin:

    ```json theme={null}
    {
      "description": "Automatic code formatting",
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PLUGIN_ROOT}/scripts/format.sh",
                "args": [],
                "timeout": 30
              }
            ]
          }
        ]
      }
    }
    ```

    Consulte a [referência de componentes de plugin](/docs/pt/plugins/components#hooks) para detalhes sobre como criar hooks de plugin.
  </Tab>
</Tabs>

<h3 id="hooks-in-skills-and-agents">
  Hooks em skills e agentes
</h3>

Além de arquivos de configurações e plugins, hooks podem ser definidos diretamente em [skills](/docs/pt/skills) e [subagentes](/docs/pt/sub-agents) usando frontmatter, no mesmo formato de configuração que hooks baseados em configurações. Por quanto tempo Claude Code os mantém registrados depende do componente:

* **Hooks de subagente**: Claude Code os executa apenas enquanto esse subagente está em execução e os remove quando termina. Claude Code converte um hook `Stop` aqui para `SubagentStop`, o evento que dispara quando um subagente completa.
* **Hooks de skill**: Claude Code os registra quando você ou Claude invoca a skill e continua executando-os pelo resto da sessão, em turnos após o próprio turno da skill também. Para fazer Claude Code remover um hook após sua primeira execução bem-sucedida em vez disso, defina [`once: true`](#common-fields) nele.

Esta skill define um hook `PreToolUse` que executa um script de validação de segurança antes de cada comando `Bash`:

```yaml theme={null}
---
name: secure-operations
description: Perform operations with security checks
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/security-check.sh"
---
```

Subagentes usam o mesmo formato em seu frontmatter YAML.

Hooks de frontmatter em uma skill de projeto seguem a mesma [regra de confiança de workspace que hooks em arquivos de configurações](#workspace-trust). Claude Code os registra quando você ou Claude invoca a skill, incluindo em uma execução `-p` em uma pasta que você não confiou.

Hooks de frontmatter em um subagente de projeto executam apenas após você aceitar o [diálogo de confiança de workspace](/docs/pt/permissions#project-allow-rules-and-workspace-trust) para a pasta de onde o arquivo do agente veio. Uma sessão `-p` não conta como aceitá-lo. [O que executa antes de você confiar em uma pasta](/docs/pt/permissions#what-runs-before-you-trust-a-folder) compara isso com a regra de arquivo de configurações, e a página de subagentes lista [quais escopos estão isentos](/docs/pt/sub-agents#hooks-in-subagent-frontmatter). Antes de v2.1.218, esses hooks podiam executar de pastas que você não confiava.

<h3 id="the-/hooks-menu">
  O menu `/hooks`
</h3>

Digite `/hooks` no Claude Code para abrir um navegador somente leitura para seus hooks configurados. O menu mostra cada evento de hook com uma contagem de hooks configurados, permite que você detalhe em matchers e mostra os detalhes completos de cada manipulador de hook. Use-o para verificar configuração, verificar qual arquivo de configurações um hook veio, ou inspecionar comando, prompt ou URL de um hook.

O menu exibe todos os cinco tipos de hook: `command`, `prompt`, `agent`, `http` e `mcp_tool`. Cada hook é rotulado com um prefixo `[type]` e uma fonte indicando onde foi definido:

* `User Settings`: de `~/.claude/settings.json`
* `Project Settings`: de `.claude/settings.json`
* `Local Settings`: de `.claude/settings.local.json`
* `Plugin Hooks`: de `hooks/hooks.json` de um plugin
* `Session Hooks`: registrado em memória para a sessão atual

Selecionar um hook abre uma visualização de detalhes mostrando seu evento, matcher, tipo, arquivo de origem e o comando, prompt ou URL completo. O menu é somente leitura: para adicionar, modificar ou remover hooks, edite o JSON de configurações diretamente ou peça ao Claude para fazer a mudança.

<h3 id="disable-or-remove-hooks">
  Desabilitar ou remover hooks
</h3>

Para remover um hook, delete sua entrada do arquivo de configurações JSON.

Para desabilitar temporariamente todos os hooks sem removê-los, defina `"disableAllHooks": true` em seu arquivo de configurações. Claude Code lê o valor deixado após [precedência de configurações](/docs/pt/settings#settings-precedence) se aplicar, então um `"disableAllHooks": false` no `.claude/settings.json` de um projeto substitui um `true` em suas configurações de usuário. Para desabilitar hooks para uma execução qualquer que as configurações do projeto digam, passe `--settings '{"disableAllHooks": true}'`, que tem precedência sobre configurações de projeto e local. Não há forma de desabilitar um hook individual mantendo-o na configuração.

A configuração `disableAllHooks` respeita a hierarquia de configurações gerenciadas. Se um administrador configurou hooks através de configurações de política gerenciada, `disableAllHooks` definido em configurações de usuário, projeto ou local não pode desabilitar esses hooks gerenciados. Apenas `disableAllHooks` definido no nível de configurações gerenciadas pode desabilitar hooks gerenciados. Para o alcance completo de cada nível, consulte [`disableAllHooks`](/docs/pt/settings-reference#disableallhooks).

Edições diretas a hooks em arquivos de configurações são normalmente capturadas automaticamente pelo observador de arquivo.

<h2 id="hook-input-and-output">
  Entrada e saída de hook
</h2>

Hooks de comando recebem dados JSON via stdin e comunicam resultados através de códigos de saída, stdout e stderr. Hooks HTTP recebem o mesmo JSON como corpo da solicitação POST e comunicam resultados através do corpo da resposta HTTP. Esta seção cobre campos e comportamento comuns a todos os eventos. Cada seção de evento sob [Eventos de hook](#hook-events) inclui seu esquema de entrada específico e opções de controle de decisão.

No macOS e Linux, hooks de comando executam em sua própria sessão sem um terminal controlador. O processo de hook e qualquer processo filho não podem abrir `/dev/tty` ou enviar sequências de escape diretamente para a interface do Claude Code. Windows não tem `/dev/tty`.

Para exibir uma mensagem ao usuário em qualquer plataforma, retorne [`systemMessage`](#json-output) na saída JSON. Alguns eventos descartam isso ou o entregam em outro lugar, e cada [seção de evento](#hook-events) diz assim. Para disparar uma notificação de desktop, definir um título de janela ou tocar o sino, retorne [`terminalSequence`](#emit-terminal-notifications) em vez disso.

<h3 id="common-input-fields">
  Campos de entrada comuns
</h3>

Eventos de hook recebem esses campos como JSON, além de campos específicos do evento documentados em cada seção [evento de hook](#hook-events). Para hooks de comando, este JSON chega via stdin. Para hooks HTTP, chega como corpo da solicitação POST.

| Campo             | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :---------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `session_id`      | Identificador de sessão atual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `prompt_id`       | UUID identificando o prompt do usuário sendo processado atualmente. Corresponde ao [atributo `prompt.id` em eventos OpenTelemetry](/docs/pt/monitoring-usage#event-correlation-attributes), para que você possa correlacionar saída de hook com telemetria para um único prompt. Ausente até a primeira entrada do usuário. Requer Claude Code v2.1.196 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `transcript_path` | Caminho para JSON de conversa. O arquivo de transcrição é escrito de forma assíncrona e pode ficar atrás da conversa na memória, portanto pode não incluir ainda as mensagens mais recentes da rodada atual quando um hook dispara. Hooks que precisam do texto final do assistente da rodada atual devem usar `last_assistant_message` em [Stop](#stop) e [SubagentStop](#subagentstop) em vez de ler a transcrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `cwd`             | Diretório de trabalho atual quando o hook é invocado                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `scratchpad_dir`  | Caminho para o diretório scratchpad da sessão, onde Claude mantém arquivos de trabalho temporários. Ausente quando a sessão não tem scratchpad ou o diretório temporário não está disponível. Requer Claude Code v2.1.257 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `permission_mode` | [Modo de permissão](/docs/pt/permissions#permission-modes) atual: `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, `"dontAsk"` ou `"bypassPermissions"`. O modo rotulado **Manual** chega como `"default"`, nunca como `"manual"`, portanto scripts que correspondem a `"default"` continuam funcionando. Nem todos os eventos recebem este campo. Verifique o exemplo JSON em cada seção [evento de hook](#hook-events)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `effort`          | Objeto com um campo `level` contendo o [nível de esforço](/docs/pt/model-config#adjust-effort-level) em vigor quando o hook é executado: `"low"`, `"medium"`, `"high"`, `"xhigh"` ou `"max"`. Se você definir um nível que o modelo ativo não suporta, `level` relata o nível que Claude Code executou em vez disso; [Ajustar nível de esforço](/docs/pt/model-config#adjust-effort-level) diz como ele escolhe esse nível. Ultracode não é um nível distinto e é relatado como `"xhigh"`. O objeto corresponde ao campo `effort` da [linha de status](/docs/pt/statusline#available-data). Presente para eventos que disparam dentro de um contexto de uso de ferramenta, como `PreToolUse`, `PostToolUse`, `Stop` e `SubagentStop`, quando o modelo atual suporta o parâmetro de esforço. O nível também está disponível para comandos de hook e a ferramenta Bash como a variável de ambiente `$CLAUDE_EFFORT`. |
| `hook_event_name` | Nome do evento que disparou                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |

Ao executar com `--agent` ou dentro de um subagente, dois campos adicionais são incluídos:

| Campo        | Descrição                                                                                                                                                                                                                                                                                                                                                                                                               |
| :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `agent_id`   | Identificador único para o subagente. Presente apenas quando o hook dispara dentro de uma chamada de subagente. Use isso para distinguir chamadas de hook de subagente de chamadas de thread principal.                                                                                                                                                                                                                 |
| `agent_type` | Nome do agente (por exemplo, `"Explore"` ou `"security-reviewer"`). Presente quando a sessão usa `--agent` ou o hook dispara dentro de um subagente. Para subagentes, o tipo do subagente tem precedência sobre o valor `--agent` da sessão. Consulte [SubagentStart](#subagentstart) para os valores que subagentes personalizados e de plugin relatam e como escrever um matcher contra um nome com escopo de plugin. |

Apenas hooks [`SessionStart`](#sessionstart) podem receber um campo `model`, e Claude Code nem sempre o inclui. Hooks [`PreModelSwitch`](#premodelswitch) e [`PostModelSwitch`](#postmodelswitch) recebem `from_model` e `to_model` em vez disso, portanto use um hook PostModelSwitch para acompanhar o modelo conforme ele muda durante uma sessão.

Não há variável de ambiente `$CLAUDE_MODEL`. O hook pode ler `$ANTHROPIC_MODEL` se você defini-lo em seu shell, mas esse valor não muda quando você alterna modelos com `/model` durante uma sessão.

Um processo de hook herda o ambiente pai, além das variáveis exportadoras `OTEL_*` que Claude Code [remove de cada subprocesso que spawna](/docs/pt/monitoring-usage#administrator-configuration) e, quando [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/pt/env-vars#variables) é definido como `1`, as variáveis que ele remove.

Por exemplo, um hook `PreToolUse` para um comando Bash recebe isso em stdin:

```json theme={null}
{
  "session_id": "abc123",
  "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
  "transcript_path": "/home/user/.claude/projects/.../transcript.jsonl",
  "cwd": "/home/user/my-project",
  "scratchpad_dir": "/tmp/claude-1000/-home-user-my-project/abc123/scratchpad",
  "permission_mode": "default",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test",
    "description": "Run test suite",
    "timeout": 120000,
    "run_in_background": false
  },
  "tool_use_id": "toolu_01ABC123..."
}
```

Os campos `tool_name`, `tool_input` e `tool_use_id` são específicos do evento. Cada seção [evento de hook](#hook-events) documenta os campos adicionais para esse evento.

<h3 id="exit-code-output">
  Saída de código de saída
</h3>

O código de saída do seu comando de hook diz ao Claude Code se a ação deve prosseguir, ser bloqueada ou ser ignorada. O código de saída não atua sozinho. Claude Code lê [campos de saída JSON](#json-output) de stdout em cada código de saída, não apenas 0, e para eventos que usam o modelo de decisão padrão, um objeto analisado que passa na validação de esquema entra em vigor ao lado do código. O bloqueio da saída 2 é o único resultado que JSON não pode substituir.

Duas tabelas possuem as exceções por evento: [Comportamento de código de saída 2 por evento](#exit-code-2-behavior-per-event) diz o que códigos de saída fazem para cada evento, e [Controle de decisão](#decision-control) diz quais campos de decisão cada evento honra. Campos universais como `systemMessage` funcionam em muitos eventos e são listados na tabela [Saída JSON](#json-output).

<h4 id="exit-code-0">
  Código de saída 0
</h4>

Saída 0 significa sucesso, e é o código de saída pretendido quando você imprime JSON para controle estruturado.

Para a maioria dos eventos, Claude Code escreve stdout no log de debug e não o mostra na transcrição. As exceções são `UserPromptSubmit`, `UserPromptExpansion`, `SessionStart` e `PostModelSwitch`, onde Claude Code adiciona stdout em texto simples como contexto que Claude pode ver e agir.

Se Claude Code lê seu stdout como [saída JSON](#json-output) ou como texto simples depende de como ele começa e termina, ignorando espaço em branco ao redor:

* **Começa com `{` e termina com `}`**: Claude Code o analisa como JSON. Quando a saída é duas ou mais linhas que cada uma analisa como JSON por conta própria, e nenhuma linha é um objeto [saída JSON](#json-output) que define um campo, Claude Code trata toda a saída como texto simples. Quando uma dessas linhas define um campo, toda a saída é uma falha de análise, descrita abaixo.
* **Começa com `{` mas não termina com `}`**: Claude Code o trata como texto simples.
* **Começa com qualquer outra coisa**: Claude Code o trata como texto simples, um array JSON ou uma string JSON entre aspas incluída.

Para eventos que usam o modelo de decisão padrão, saída 0 com um objeto analisado que falha na validação de esquema é um erro não-bloqueador: a ação prossegue, e a transcrição mostra um aviso `<hook name> hook error` com a mensagem de validação. O mesmo acontece em qualquer código de saída diferente de 2, enquanto [saída 2 ainda bloqueia](#exit-code-2).

Para eventos que usam o modelo de decisão padrão, quando Claude Code tenta analisar seu stdout como JSON e não consegue, ele relata um erro não-bloqueador em cada código de saída diferente de 2. A transcrição mostra um aviso `<hook name> hook error` com a mensagem de análise. Nos eventos que adicionam stdout em texto simples como contexto, Claude Code não adiciona o texto. Antes de v2.1.248, Claude Code tratava esse stdout como texto simples.

Stderr de um hook que sai 0 vai apenas para o log de debug, nunca para a transcrição, e Claude nunca vê. Para lê-lo você mesmo, ative [debug logging](#debug-hooks). Para exibir um aviso para Claude de um hook `PostToolUse` ou `PostToolUseFailure`, saia 2 em vez disso para que [Claude veja o stderr](#exit-code-2-behavior-per-event) mesmo que a ferramenta já tenha executado.

<h4 id="exit-code-2">
  Código de saída 2
</h4>

Saída 2 significa um erro bloqueador. Em [eventos que podem bloquear](#exit-code-2-behavior-per-event), saída 2 bloqueia se você imprime JSON ou não: até mesmo um JSON `permissionDecision` de `"allow"` não pode substituir. Claude Code ainda lê qualquer [saída JSON](#json-output) válida em stdout. Em `Elicitation` e `ElicitationResult`, o `hookSpecificOutput` de um hook exit-2 é ignorado.

A mensagem de bloqueio é a razão da decisão de bloqueio do seu JSON quando faz uma, e seu texto stderr caso contrário. O que o bloqueio faz varia por evento: `PreToolUse` bloqueia a chamada da ferramenta, `UserPromptSubmit` rejeita o prompt, e assim por diante. [Comportamento de código de saída 2 por evento](#exit-code-2-behavior-per-event) lista o efeito para cada evento, e cada seção de evento diz onde a mensagem vai.

Um hook que sai 2 enquanto imprime JSON que falha na validação de esquema [saída JSON](#json-output) ainda bloqueia: Claude Code usa stderr como a razão de bloqueio e registra a falha de validação no log de debug. Antes de v2.1.214, Claude Code tratava essa combinação como um erro não-bloqueador e a ação prosseguia.

Este script bloqueia comandos `rm` saindo 2 e deixa cada outro comando para o fluxo de permissão normal:

```bash theme={null}
#!/bin/bash
# Lê entrada JSON de stdin, verifica o comando
input=$(cat)
command=$(jq -r '.tool_input.command' <<<"$input")

if [[ "$command" == rm* ]]; then
  echo "Blocked: rm commands are not allowed" >&2
  exit 2  # Erro bloqueador: chamada de ferramenta é prevenida
fi

exit 0  # Sem decisão: o fluxo de permissão normal se aplica
```

<h4 id="other-exit-codes">
  Outros códigos de saída
</h4>

Qualquer outro código de saída não bloqueia por conta própria para a maioria dos eventos de hook. O que acontece depende de seu stdout:

* Com um objeto analisado que passa na validação de esquema, para eventos que usam o modelo de decisão padrão, Claude Code ignora o código de saída e apenas o JSON decide o resultado:
  * Cada campo que o evento suporta é honrado, incluindo `permissionDecision`, `additionalContext`, `updatedInput` e `systemMessage`, e o hook não é relatado como um erro.
  * [Controle de decisão](#decision-control) lista os campos de decisão por evento; campos universais como `systemMessage` seguem a tabela [Saída JSON](#json-output).
* Com um objeto analisado que falha na validação de esquema, para eventos que usam o modelo de decisão padrão, é o mesmo erro não-bloqueador que [na saída 0](#exit-code-0): a ação prossegue, e o aviso `<hook name> hook error` carrega a mensagem de validação.
* Com stdout que Claude Code [tenta analisar como JSON](#exit-code-0) e não consegue, Claude Code relata o mesmo erro não-bloqueador que na saída 0 para eventos que usam o modelo de decisão padrão. A ação prossegue, e o aviso carrega a mensagem de análise.
* Com stdout que Claude Code [trata como texto simples](#exit-code-0), ou com stdout vazio, é um erro não-bloqueador para a maioria dos eventos de hook: a ação prossegue, e a transcrição mostra um aviso `<hook name> hook error` seguido pela primeira linha de stderr, prefixado com `Failed with non-blocking status code:`. Para capturar o stderr completo, ative [debug logging](#debug-hooks).

Eventos fora do modelo de decisão padrão mantêm suas próprias linhas na [tabela por evento](#exit-code-2-behavior-per-event): `WorktreeCreate` falha na criação em qualquer saída não-zero não importa o que seu JSON diz, e eventos que descartam saída de hook inteiramente, como `StopFailure`, ignoram seu JSON em cada código de saída, além de campos de efeito colateral como `terminalSequence`, que ainda disparam.

Um hook que não consegue iniciar cai no mesmo balde não-bloqueador. Quando o caminho do script não existe ou não é executável, o shell sai com um código como 127 e você vê o mesmo aviso com a mensagem do interpretador, por exemplo `Failed with non-blocking status code: /bin/sh: /path/to/hook.sh: No such file or directory`. Para a maioria dos eventos de hook, a ação prossegue. Quando você configura um hook de política, observe este aviso em sua primeira execução: um caminho digitado incorretamente em `settings.json` deixa o portão silenciosamente desabilitado.

<Warning>
  Para a maioria dos eventos de hook, código de saída 2 é o único código de saída que bloqueia apenas através do código. Sem JSON válido em stdout, Claude Code trata código de saída 1 como um erro não-bloqueador e prossegue com a ação, mesmo que 1 seja o código de falha Unix convencional. Se seu hook se destina a impor uma política, use `exit 2`. Os eventos de worktree diferem: qualquer código de saída não-zero de `WorktreeCreate` aborta a criação de worktree, e qualquer código de saída não-zero de `WorktreeRemove` faz a remoção de worktree falhar se o diretório ainda existir depois.
</Warning>

<h4 id="timeouts">
  Timeouts
</h4>

Além de um hook de comando que você executa com [`async: true`](#run-hooks-in-the-background), Claude Code cancela um hook `command`, `http` ou `mcp_tool` que atinge seu [`timeout`](#common-fields), descartando a saída do hook, portanto na maioria dos eventos um hook expirado não renderiza decisão.

Em [`PreModelSwitch`](#premodelswitch), um hook cancelado em seu timeout bloqueia a mudança de modelo. Em `PreToolUse`, as duas famílias de hook diferem:

* Um hook `command`, `http` ou `mcp_tool` expirado não bloqueia a chamada da ferramenta. A chamada continua através do [fluxo de permissão](/docs/pt/permissions) normal, portanto não conte com um hook travado para agir como um portão.
* Um hook de callback [Agent SDK](/docs/pt/agent-sdk/hooks) que excede seu timeout [bloqueia a chamada da ferramenta](#pretooluse).

<h4 id="exit-code-2-behavior-per-event">
  Comportamento de código de saída 2 por evento
</h4>

Código de saída 2 é a forma de um hook sinalizar "pare, não faça isso". O efeito depende do evento, porque alguns eventos representam ações que podem ser bloqueadas (como uma chamada de ferramenta que ainda não aconteceu) e outros representam coisas que já aconteceram ou não podem ser prevenidas.

| Evento de hook        | Pode bloquear? | O que acontece na saída 2                                                                                                                                                                                                                                        |
| :-------------------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`          | Sim            | Bloqueia a chamada da ferramenta                                                                                                                                                                                                                                 |
| `PermissionRequest`   | Não            | Código de saída 2 não é honrado para este evento e o fluxo de permissão prossegue inalterado. Negue através do objeto [`decision`](#permissionrequest-decision-control) em vez disso                                                                             |
| `UserPromptSubmit`    | Sim            | Bloqueia o processamento de prompt e apaga o prompt                                                                                                                                                                                                              |
| `UserPromptExpansion` | Sim            | Bloqueia a expansão                                                                                                                                                                                                                                              |
| `Stop`                | Sim            | Previne Claude de parar, continua a conversa                                                                                                                                                                                                                     |
| `SubagentStop`        | Sim            | Previne o subagente de parar                                                                                                                                                                                                                                     |
| `TeammateIdle`        | Sim            | Previne o colega de ficar ocioso, para que continue trabalhando                                                                                                                                                                                                  |
| `TaskCreated`         | Sim            | Reverte a criação de tarefa                                                                                                                                                                                                                                      |
| `TaskCompleted`       | Sim            | Previne a tarefa de ser marcada como concluída                                                                                                                                                                                                                   |
| `ConfigChange`        | Sim            | Bloqueia a mudança de configuração de entrar em efeito (exceto `policy_settings`)                                                                                                                                                                                |
| `StopFailure`         | Não            | Saída e código de saída são ignorados, exceto `terminalSequence`                                                                                                                                                                                                 |
| `PostToolUse`         | Não            | Mostra stderr ao Claude; a ferramenta já executou                                                                                                                                                                                                                |
| `PostToolUseFailure`  | Não            | Mostra stderr ao Claude; a ferramenta já falhou                                                                                                                                                                                                                  |
| `PostToolBatch`       | Sim            | Para o loop agentic antes da próxima chamada de modelo                                                                                                                                                                                                           |
| `PermissionDenied`    | Não            | Código de saída e stderr são ignorados porque a negação já ocorreu. Use JSON `hookSpecificOutput.retry: true` para dizer ao modelo que pode tentar novamente; Claude Code ignora `retry: true` para [negações sem veredicto](#permissiondenied-decision-control) |
| `Notification`        | Não            | Código de saída e stderr são ignorados                                                                                                                                                                                                                           |
| `SubagentStart`       | Não            | Mostra stderr apenas ao usuário                                                                                                                                                                                                                                  |
| `SessionStart`        | Não            | Mostra stderr apenas ao usuário                                                                                                                                                                                                                                  |
| `Setup`               | Não            | Código de saída e stderr são ignorados                                                                                                                                                                                                                           |
| `SessionEnd`          | Não            | Mostra stderr apenas ao usuário                                                                                                                                                                                                                                  |
| `CwdChanged`          | Não            | Mostra stderr apenas ao usuário                                                                                                                                                                                                                                  |
| `DirectoryAdded`      | Não            | Stderr vai para o log de debug; o diretório já foi adicionado                                                                                                                                                                                                    |
| `FileChanged`         | Não            | Mostra stderr apenas ao usuário                                                                                                                                                                                                                                  |
| `PreCompact`          | Sim            | Bloqueia compactação                                                                                                                                                                                                                                             |
| `PostCompact`         | Não            | Mostra stderr apenas ao usuário                                                                                                                                                                                                                                  |
| `PreModelSwitch`      | Sim            | Bloqueia a mudança de modelo e mostra stderr ao usuário                                                                                                                                                                                                          |
| `PostModelSwitch`     | Não            | Mostra stderr apenas ao usuário; o modelo já mudou                                                                                                                                                                                                               |
| `Elicitation`         | Sim            | Nega a elicitação                                                                                                                                                                                                                                                |
| `ElicitationResult`   | Sim            | Bloqueia a resposta (ação se torna decline)                                                                                                                                                                                                                      |
| `WorktreeCreate`      | Sim            | Qualquer código de saída não-zero causa falha na criação de worktree                                                                                                                                                                                             |
| `WorktreeRemove`      | Sim            | Qualquer código de saída não-zero causa falha na remoção de worktree se o diretório ainda existir depois. Consulte [WorktreeRemove](#worktreeremove) para o que acontece com o diretório                                                                         |
| `InstructionsLoaded`  | Não            | Código de saída é ignorado                                                                                                                                                                                                                                       |
| `MessageDisplay`      | Não            | O texto original é exibido                                                                                                                                                                                                                                       |

Para `SessionStart`, `SubagentStart` e `PostModelSwitch`, Claude Code renderiza o stderr de código de saída 2 na transcrição como um aviso `<hook name> hook error`, da mesma forma que renderiza um [erro não-bloqueador](#exit-code-output). Claude não vê, e a sessão ou subagente prossegue. Para `SubagentStart`, o aviso aparece na própria transcrição do subagente, não na conversa pai.

<h3 id="http-response-handling">
  Tratamento de resposta HTTP
</h3>

Hooks HTTP usam códigos de status HTTP e corpos de resposta em vez de códigos de saída e stdout. Os resultados abaixo se aplicam à maioria dos eventos; um evento com seu próprio contrato de falha na [tabela por evento](#exit-code-2-behavior-per-event), como `WorktreeCreate`, aplica esse contrato a um hook HTTP falhado também:

* **2xx com corpo vazio**: sucesso, equivalente a código de saída 0 sem saída
* **2xx com corpo de objeto JSON**: analisado usando o mesmo esquema [saída JSON](#json-output) que hooks de comando. Um corpo que falha na validação de esquema é um erro não-bloqueador
* **2xx com qualquer outro corpo, como texto simples**: erro não-bloqueador, tratado da mesma forma que um status não-2xx. Claude Code não adiciona o texto ao contexto de Claude
* **Status não-2xx**: erro não-bloqueador, execução continua
* **Falha de conexão**: erro não-bloqueador, execução continua
* **Timeout**: o hook é cancelado, conforme descrito em [Timeouts](#timeouts)

Diferentemente de hooks de comando, hooks HTTP não podem sinalizar um erro bloqueador apenas através de códigos de status. Para bloquear uma chamada de ferramenta ou negar uma permissão, retorne uma resposta 2xx com um corpo JSON contendo os campos de decisão apropriados.

<h3 id="json-output">
  Saída JSON
</h3>

Códigos de saída permitem você bloquear ou ficar em silêncio, mas saída JSON oferece controle mais granular. Em vez de sair com código 2 para bloquear, saia 0 e imprima um objeto JSON em stdout. Claude Code lê campos específicos desse JSON para controlar comportamento, incluindo [controle de decisão](#decision-control) para bloquear, permitir ou escalar para o usuário.

<Note>
  Escolha uma abordagem por hook: ou use códigos de saída sozinhos para sinalizar, ou saia 0 e imprima JSON para controle estruturado. Se você misturar, saída 2 mantém seu [efeito de bloqueio](#exit-code-2-behavior-per-event), e Claude Code ainda lê os campos JSON, com a exceção de elicitação única anotada em [Código de saída 2](#exit-code-2).
</Note>

O stdout do seu hook deve conter apenas o objeto JSON. Se seu perfil shell imprime texto na inicialização, pode interferir com análise JSON. Consulte [Hook JSON não tem efeito](/docs/pt/hooks-guide#hook-json-has-no-effect) no guia de troubleshooting.

As strings de saída de hook `additionalContext`, `systemMessage` e `initialUserMessage`, e seu stdout simples, são limitadas a 10.000 caracteres:

* **Escopo**: Claude Code mede cada string por conta própria, mesmo quando vários hooks executam para o mesmo evento. Para saída JSON, cada campo é medido separadamente; stdout simples é medido como um todo.
* **Acima do limite**: Claude Code salva a saída em um arquivo no diretório de sessão e a substitui pelo caminho do arquivo e uma visualização de até os primeiros 2.000 caracteres. Um resultado Bash grande válido é tratado da mesma forma, descrito em [Limites de saída](/docs/pt/tools-reference#output-limits). Diferentemente desse teto Bash, este limite não tem configuração ou variável de ambiente para aumentá-lo.
* **Lendo o arquivo**: Claude Code não pede a Claude para ler o arquivo, portanto mantenha qualquer coisa que Claude sempre deva ver dentro do limite.

O objeto JSON suporta três tipos de campos:

* **Campos universais** como `continue` são listados na tabela abaixo. Cada evento os aceita, mas alguns eventos os descartam ou entregam `systemMessage` em outro lugar que não a transcrição. Cada seção de evento diz assim. `terminalSequence` funciona nesses eventos também, com as exceções listadas em [Emitir notificações de terminal](#emit-terminal-notifications).
* **`decision` e `reason` de nível superior** são usados por alguns eventos para bloquear ou fornecer feedback.
* **`hookSpecificOutput`** é um objeto aninhado para eventos que precisam de controle mais rico. Requer um campo `hookEventName` definido para o nome do evento.

| Campo              | Padrão  | Descrição                                                                                                                                                                                                                                                                                                                                      |
| :----------------- | :------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `continue`         | `true`  | Se `false`, Claude para de processar inteiramente após o hook executar. Tem precedência sobre qualquer campo de decisão específico do evento                                                                                                                                                                                                   |
| `stopReason`       | nenhum  | Mensagem mostrada ao usuário quando `continue` é `false`. Fica na conversa, portanto Claude a vê se a conversa continuar                                                                                                                                                                                                                       |
| `suppressOutput`   | `false` | Não tem efeito: Claude Code aceita o campo mas não age sobre ele. O stdout de um hook bem-sucedido nunca é mostrado na transcrição e é registrado no log de debug                                                                                                                                                                              |
| `systemMessage`    | nenhum  | Mensagem de aviso mostrada ao usuário. Em [Agent SDK](/docs/pt/agent-sdk/overview) e saída [`--output-format stream-json`](/docs/pt/headless), pode chegar como um [`SDKInformationalMessage`](/docs/pt/agent-sdk/typescript#sdkinformationalmessage)                                                                                                         |
| `terminalSequence` | nenhum  | Uma sequência de escape de terminal para Claude Code emitir em seu nome, como uma notificação de desktop, título de janela ou sino. Restrito a OSC `0`/`1`/`2`/`9`/`99`/`777` e BEL. Se o valor contiver algo fora da lista de permissões, o campo é ignorado. Use isso em vez de escrever para `/dev/tty`, que não está disponível para hooks |

Para parar Claude inteiramente:

```json theme={null}
{ "continue": false, "stopReason": "Build failed, fix errors before continuing" }
```

Para hooks `PreToolUse` e `PostToolUse`, a parada se aplica mesmo quando a chamada da ferramenta falha ou é concluída enquanto Claude ainda está transmitindo uma resposta.

<h4 id="emit-terminal-notifications">
  Emitir notificações de terminal
</h4>

Hooks executam sem um terminal controlador, portanto escrever sequências de escape diretamente para `/dev/tty` falha. Em vez disso, retorne a sequência de escape no campo `terminalSequence` e Claude Code a emite para você através de seu próprio caminho de escrita de terminal. Isso é livre de corrida, funciona dentro de tmux e GNU screen, e funciona no Windows onde não há `/dev/tty`.

O campo aceita uma string de uma ou mais sequências de escape na lista de permissões:

* OSC `0`, `1`, `2`: títulos de janela e ícone
* OSC `9`: notificações iTerm2, ConEmu, Windows Terminal e WezTerm, incluindo progresso de barra de tarefas `9;4`
* OSC `99`: notificações Kitty
* OSC `777`: notificações urxvt, Ghostty e Warp
* BEL simples

Sequências podem ser terminadas com BEL ou com ST. Qualquer coisa fora da lista de permissões, incluindo sequências de cursor e cor CSI, sequências de paleta OSC, hiperlinks OSC 8, escritas de área de transferência OSC 52 e OSC 1337, é rejeitada e o campo é ignorado.

Claude Code escreve a sequência em si quando processa a saída do seu hook, portanto o campo funciona em eventos que descartam `systemMessage` e `continue`, como `Notification` e `StopFailure`. Tem dois limites:

* Claude Code escreve a sequência apenas em uma sessão interativa, e apenas enquanto sua interface está na tela. Em modo não-interativo com a flag `-p` e no Agent SDK, ignora o campo.
* Um hook de comando `WorktreeCreate` não pode retornar JSON, porque Claude Code lê seu stdout como o caminho de worktree. Um hook HTTP `WorktreeCreate` retorna JSON e pode incluir o campo.

O exemplo abaixo dispara uma notificação de desktop de um hook `Notification`. A sequência de escape é construída com `printf` escapes octais para que os bytes de controle nunca apareçam na linha de comando do shell, e `jq -n --arg` constrói a saída JSON para que aspas, barras invertidas e quebras de linha na mensagem de notificação sejam escapadas corretamente:

```bash theme={null}
#!/bin/bash
# Hook de notificação: ping no desktop quando Claude Code precisa de atenção.
input=$(cat)
title="Claude Code"
body=$(jq -r '.message // "Needs your attention"' <<<"$input")
seq=$(printf '\033]777;notify;%s;%s\007' "$title" "$body")
jq -nc --arg seq "$seq" '{terminalSequence: $seq}'
```

A forma `{ "terminalSequence": "..." }` é a mesma de qualquer shell ou linguagem.

<h4 id="add-context-for-claude">
  Adicionar contexto para Claude
</h4>

O campo `additionalContext` passa uma string do seu hook para a janela de contexto do Claude. Claude Code envolve a string em um lembrete do sistema e a insere na conversa no ponto onde o hook disparou. Claude lê o lembrete na próxima solicitação de modelo, mas não aparece como uma mensagem de chat na interface.

Retorne `additionalContext` dentro de `hookSpecificOutput` ao lado do nome do evento:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "This file is generated. Edit src/schema.ts and run `bun generate` instead."
  }
}
```

Onde o lembrete aparece depende do evento:

* [SessionStart](#sessionstart) e [SubagentStart](#subagentstart): no início da conversa, antes do primeiro prompt
* [UserPromptSubmit](#userpromptsubmit) e [UserPromptExpansion](#userpromptexpansion): ao lado do prompt enviado
* [PreToolUse](#pretooluse), [PostToolUse](#posttooluse), [PostToolUseFailure](#posttoolusefailure) e [PostToolBatch](#posttoolbatch): ao lado do resultado da ferramenta
* [Stop](#stop) e [SubagentStop](#subagentstop): no final da rodada. A conversa continua para que Claude possa agir sobre o feedback. Consulte [Controle de decisão Stop](#stop-decision-control)
* [PostModelSwitch](#postmodelswitch): com a próxima solicitação após a mudança. Consulte [Controle de decisão PostModelSwitch](#postmodelswitch-decision-control) para timing

Quando vários hooks retornam `additionalContext` para o mesmo evento, Claude recebe todos os valores.

Se um valor exceder 10.000 caracteres, Claude Code escreve o texto em um arquivo no diretório de sessão e passa Claude o caminho do arquivo com uma visualização de até os primeiros 2.000 caracteres em vez disso. Claude pode ler o arquivo, mas Claude Code não pede a Claude para.

Use `additionalContext` para informações que Claude deve saber sobre o estado atual do seu ambiente ou a operação que acabou de executar:

* **Estado do ambiente**: o branch atual, alvo de implantação ou sinalizadores de recurso ativos
* **Regras de projeto condicional**: qual comando de teste se aplica ao arquivo que acabou de ser editado, quais diretórios são somente leitura nesta worktree
* **Dados externos**: problemas abertos atribuídos a você, resultados recentes de CI, conteúdo obtido de um serviço interno

Para instruções que nunca mudam, prefira [CLAUDE.md](/docs/pt/memory). Ele carrega sem executar um script e é o lugar padrão para convenções de projeto estáticas.

Escreva o texto como declarações factuais em vez de instruções de sistema imperativas. Frases como "O alvo de implantação é produção" ou "Este repositório usa `bun test`" lê como informação de projeto. Texto enquadrado como comandos de sistema fora de banda pode disparar as defesas de injeção de prompt do Claude, o que faz com que Claude superficialize o texto para você em vez de tratá-lo como contexto.

Claude Code salva o texto injetado na transcrição de sessão. Para eventos de mid-sessão como `PostToolUse` ou `UserPromptSubmit`, quando você retoma com `--continue` ou `--resume`, Claude Code reproduz o texto salvo em vez de re-executar o hook para turnos anteriores, portanto valores como timestamps ou SHAs de commit ficam obsoletos. Hooks `SessionStart` executam novamente na retomada com `source` definido como `"resume"`, ou `"fork"` se você adicionou `--fork-session`, para que possam atualizar seu contexto.

<h4 id="decision-control">
  Controle de decisão
</h4>

Nem todo evento suporta bloqueio ou controle de comportamento através de JSON. Os eventos que fazem cada um usam um conjunto diferente de campos para expressar essa decisão. Use esta tabela como referência rápida antes de escrever um hook:

| Eventos                                                                                                                             | Padrão de decisão                                    | Campos-chave                                                                                                                                                                                                                                                                                            |
| :---------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| UserPromptSubmit, UserPromptExpansion, PostToolUse, PostToolUseFailure, PostToolBatch, Stop, SubagentStop, ConfigChange, PreCompact | `decision` de nível superior                         | `decision: "block"`, `reason`. Stop e SubagentStop também aceitam `hookSpecificOutput.additionalContext` para [feedback não-erro que continua a conversa](#stop-decision-control)                                                                                                                       |
| TeammateIdle, TaskCompleted                                                                                                         | Código de saída ou `continue: false`                 | Código de saída 2 bloqueia a ação com feedback de stderr. JSON `{"continue": false, "stopReason": "..."}` também para o colega inteiramente, correspondendo ao comportamento do hook `Stop`; [TaskCompleted ignora quando a ferramenta `TaskUpdate` disparou o evento](#taskcompleted-decision-control) |
| TaskCreated                                                                                                                         | Código de saída ou `decision` de nível superior      | Código de saída 2 ou `decision: "block"` [cancela a tarefa](#taskcreated-decision-control) e retorna a mensagem para Claude. `continue: false` é ignorado                                                                                                                                               |
| PreToolUse                                                                                                                          | `hookSpecificOutput`                                 | `permissionDecision` (allow/deny/ask/defer), `permissionDecisionReason`                                                                                                                                                                                                                                 |
| PreModelSwitch                                                                                                                      | `hookSpecificOutput` ou `decision` de nível superior | `permissionDecision` (allow/deny/ask), `permissionDecisionReason`. `decision: "block"` também [cancela a mudança](#premodelswitch-decision-control)                                                                                                                                                     |
| PermissionRequest                                                                                                                   | `hookSpecificOutput`                                 | `decision.behavior` (allow/deny)                                                                                                                                                                                                                                                                        |
| PermissionDenied                                                                                                                    | `hookSpecificOutput`                                 | `retry: true` diz ao modelo que pode tentar novamente a chamada de ferramenta negada; Claude Code ignora para [negações sem veredicto](#permissiondenied-decision-control)                                                                                                                              |
| WorktreeCreate                                                                                                                      | retorno de caminho                                   | Hook de comando imprime caminho em stdout; hook HTTP retorna `hookSpecificOutput.worktreePath`. Falha de hook ou caminho ausente falha na criação                                                                                                                                                       |
| WorktreeRemove                                                                                                                      | Código de saída                                      | Qualquer código de saída não-zero faz a remoção falhar se o diretório ainda existir depois. Saída JSON é descartada                                                                                                                                                                                     |
| Elicitation                                                                                                                         | `hookSpecificOutput`                                 | `action` (accept/decline/cancel), `content` (valores de campo de formulário para accept)                                                                                                                                                                                                                |
| ElicitationResult                                                                                                                   | `hookSpecificOutput`                                 | `action` (accept/decline/cancel), `content` (valores de campo de formulário override)                                                                                                                                                                                                                   |
| MessageDisplay                                                                                                                      | `hookSpecificOutput`                                 | `displayContent` substitui o texto exibido na tela. Apenas exibição: a transcrição e o que Claude vê mantêm o original                                                                                                                                                                                  |
| SessionStart, SubagentStart, PostModelSwitch                                                                                        | Apenas contexto                                      | `hookSpecificOutput.additionalContext` adiciona contexto para Claude. SessionStart também aceita [`initialUserMessage`, `watchPaths`, `sessionTitle` e `reloadSkills`](#sessionstart-decision-control). Sem bloqueio ou controle de decisão                                                             |
| Setup, Notification, SessionEnd, PostCompact, InstructionsLoaded, StopFailure, CwdChanged, DirectoryAdded, FileChanged              | Nenhum                                               | Sem controle de decisão. Usado para efeitos colaterais como logging ou limpeza                                                                                                                                                                                                                          |

Alguns eventos também podem reescrever conteúdo em vez de apenas permitir ou bloquear:

* `PreToolUse`: `updatedInput` diretamente sob `hookSpecificOutput` substitui os argumentos de uma ferramenta antes de executar. Consulte [Controle de decisão PreToolUse](#pretooluse-decision-control)
* `PermissionRequest`: `updatedInput` dentro do objeto `decision`. Consulte [Controle de decisão PermissionRequest](#permissionrequest-decision-control)
* `PostToolUse`: `updatedToolOutput` substitui o resultado da ferramenta. Consulte [Controle de decisão PostToolUse](#posttooluse-decision-control)
* `UserPromptSubmit`: não pode substituir o prompt; apenas injeta `additionalContext` ao lado dele

Para casos de uso de redação ou transformação, intercepte em `PreToolUse` para entradas de ferramenta de saída e `PostToolUse` para resultados de ferramenta de entrada.

Aqui estão exemplos de cada padrão em ação:

<Tabs>
  <Tab title="Decisão de nível superior">
    O único valor para `decision` é `"block"`. Para permitir que a ação prossiga, omita `decision` do seu JSON, ou saia 0 sem qualquer JSON:

    ```json theme={null}
    {
      "decision": "block",
      "reason": "Test suite must pass before proceeding"
    }
    ```
  </Tab>

  <Tab title="PreToolUse">
    Usa `hookSpecificOutput` para controle mais rico: permitir, negar ou escalar para o usuário. Você também pode modificar a entrada da ferramenta antes de executar ou injetar contexto adicional para Claude. Consulte [Controle de decisão PreToolUse](#pretooluse-decision-control) para o conjunto completo de opções.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "permissionDecision": "deny",
        "permissionDecisionReason": "Database writes are not allowed"
      }
    }
    ```
  </Tab>

  <Tab title="PermissionRequest">
    Usa `hookSpecificOutput` para permitir ou negar uma solicitação de permissão em nome do usuário. Ao permitir, você também pode modificar a entrada da ferramenta ou aplicar regras de permissão para que o usuário não seja solicitado novamente. Consulte [Controle de decisão PermissionRequest](#permissionrequest-decision-control) para o conjunto completo de opções.

    ```json theme={null}
    {
      "hookSpecificOutput": {
        "hookEventName": "PermissionRequest",
        "decision": {
          "behavior": "allow",
          "updatedInput": {
            "command": "npm run lint"
          }
        }
      }
    }
    ```
  </Tab>
</Tabs>

Para exemplos estendidos incluindo validação de comando Bash, filtragem de prompt e scripts de aprovação automática, consulte [O que você pode automatizar](/docs/pt/hooks-guide#what-you-can-automate) no guia e a [implementação de referência do validador de comando Bash](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py).

<h2 id="hook-events">
  Eventos de hook
</h2>

Cada evento corresponde a um ponto no ciclo de vida do Claude Code onde os hooks podem ser executados. As seções abaixo estão ordenadas para corresponder ao ciclo de vida: desde a configuração da sessão através do loop agentic até o final da sessão. Cada seção descreve quando o evento é disparado, quais matchers ele suporta, a entrada JSON que recebe e como controlar o comportamento através da saída.

<h3 id="sessionstart">
  SessionStart
</h3>

Executado quando Claude Code inicia uma nova sessão ou retoma uma sessão existente. Útil para carregar contexto de desenvolvimento como problemas existentes ou mudanças recentes no seu código, ou configurar variáveis de ambiente. Para contexto estático que não requer um script, use [CLAUDE.md](/docs/pt/memory) em vez disso.

SessionStart é executado em cada sessão, portanto mantenha esses hooks rápidos. Apenas hooks `type: "command"` e `type: "mcp_tool"` são suportados. Veja [campos de hook de ferramenta MCP](#mcp-tool-hook-fields) para quando hooks `mcp_tool` são executados.

O valor do matcher corresponde a como a sessão foi iniciada:

| Matcher   | Quando é disparado                                                                                                                  |
| :-------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| `startup` | Nova sessão                                                                                                                         |
| `resume`  | `--resume`, `--continue`, ou `/resume`                                                                                              |
| `clear`   | `/clear`                                                                                                                            |
| `compact` | Compactação automática ou manual                                                                                                    |
| `fork`    | Uma nova sessão bifurcada de uma existente: `--fork-session` com `--resume` ou `--continue`, a cópia de fundo `/fork`, ou `/branch` |

Antes da v2.1.214, sessões bifurcadas relatavam fonte `"resume"`.

Quando você inicia uma sessão interativa, retoma uma conversa no lançamento com `--continue` ou `--resume`, ou executa `/clear`, os hooks SessionStart são executados em segundo plano. Você pode digitar imediatamente, e uma conversa que você retomou aparece sem esperar pelos hooks. A primeira resposta do Claude ainda espera os hooks terminarem, portanto seu contexto chega ao Claude.

Quando você muda de conversas com `/resume` dentro de uma sessão, a mudança espera os hooks terminarem. Se você executar `/clear` ou mudar para outra conversa enquanto os hooks de fundo ainda estão em execução, nada que eles retornem se aplica à sessão.

A mesma espera se aplica no lançamento, incluindo uma sessão retomada: um prompt que você envia enquanto os hooks SessionStart ainda estão em execução não chega ao Claude até que terminem.

Durante qualquer espera, pressione `Esc` para levar o prompt de volta para a entrada sem enviá-lo. Os hooks continuam em execução.

<h4 id="sessionstart-input">
  Entrada SessionStart
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks SessionStart recebem `source` e opcionalmente `model`, `agent_type` e `session_title`:

| Campo           | Descrição                                                                                                                                                                                                                                   |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `source`        | Como a sessão começou: `"startup"` para novas sessões, `"resume"` para sessões retomadas, `"clear"` após `/clear`, `"compact"` após compactação, ou `"fork"` para uma nova sessão bifurcada de uma existente                                |
| `model`         | O identificador do modelo ativo. Pode ser omitido, por exemplo após `/clear` ou quando uma sessão é restaurada através da recuperação de conversa, portanto verifique o campo antes de lê-lo                                                |
| `agent_type`    | O nome do agente, presente quando você inicia Claude Code com `claude --agent <name>`                                                                                                                                                       |
| `session_title` | O título da sessão atual se um já estiver definido, por exemplo via `--name` ou `/rename`. Um hook que emite `sessionTitle` pode verificar `session_title` primeiro para evitar sobrescrever um título que o usuário definiu explicitamente |

Quando `source` é `"resume"` ou `"fork"` e a transcrição contém pelo menos uma resposta do Claude, os hooks SessionStart também recebem os quatro campos abaixo. Seu hook pode usá-los para relatar qual é o custo de retomar uma conversa obsoleta antes da primeira solicitação, por exemplo em uma [`systemMessage`](#json-output). Esses campos requerem Claude Code v2.1.251 ou posterior.

| Campo                         | Descrição                                                                                                                                                                                       |
| :---------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `seconds_since_last_response` | Segundos de relógio de parede desde a última resposta na transcrição retomada                                                                                                                   |
| `context_tokens`              | Tokens que a primeira solicitação da sessão retomada reenvia como seu prompt                                                                                                                    |
| `prompt_cache_likely_expired` | `true` quando a última resposta é mais antiga que o [tempo de vida do cache de prompt](/docs/pt/prompt-caching#cache-lifetime) da sessão ou uma compactação posterior substituiu a conversa em cache |
| `estimated_cache_write_usd`   | Custo estimado em dólares americanos de escrever `context_tokens` no cache de prompt no modelo da sessão, excluindo a resposta                                                                  |

Este exemplo mostra a entrada para uma sessão retomada 90 minutos após sua última resposta:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionStart",
  "source": "resume",
  "model": "claude-opus-5",
  "seconds_since_last_response": 5400,
  "context_tokens": 182340,
  "prompt_cache_likely_expired": true,
  "estimated_cache_write_usd": 1.1396
}
```

<h4 id="sessionstart-decision-control">
  Controle de decisão SessionStart
</h4>

Claude Code adiciona stdout que [trata como texto simples](#exit-code-0) ao contexto do Claude. Além dos [campos de saída JSON](#json-output) disponíveis para todos os hooks, você pode retornar esses campos específicos do evento:

| Campo                | Descrição                                                                                                                                                                                                                                                                                                                                                  |
| :------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext`  | String adicionada ao contexto do Claude no início da conversa, antes do primeiro prompt. Veja [Adicionar contexto para Claude](#add-context-for-claude) para como o texto é entregue e o que colocar nele                                                                                                                                                  |
| `initialUserMessage` | String usada como a primeira mensagem do usuário da sessão. Aplica-se em [modo não interativo](/docs/pt/headless) com a flag `-p`, onde se torna o primeiro turno mesmo que nenhum prompt seja fornecido. Se um prompt for fornecido, ele segue como o próximo turno. Ao contrário de `additionalContext`, que se anexa a um turno existente, isso cria o turno |
| `sessionTitle`       | Define o título da sessão, com o mesmo efeito que `/rename`. Use para nomear sessões automaticamente a partir da pasta de lançamento, ramo git ou nome de worktree. Aplica-se quando `source` é `"startup"`, `"resume"` ou `"fork"`; ignorado em `"clear"` e `"compact"`                                                                                   |
| `watchPaths`         | Array de caminhos absolutos para observar eventos [FileChanged](#filechanged) durante esta sessão                                                                                                                                                                                                                                                          |
| `reloadSkills`       | Booleano. Quando `true`, Claude Code verifica novamente os diretórios de [skill](/docs/pt/skills) e comando após os hooks SessionStart serem concluídos, portanto skills que o hook instalou estão disponíveis na mesma sessão, começando com o primeiro prompt                                                                                                 |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "Current branch: feat/auth-refactor\nUncommitted changes: src/auth.ts, src/login.tsx\nActive issue: #4211 Migrate to OAuth2",
    "sessionTitle": "auth-refactor"
  }
}
```

Como stdout simples já chega ao Claude para este evento, um hook que apenas carrega contexto pode imprimir para stdout diretamente sem construir JSON. Use a forma JSON quando você precisa combinar contexto com outros campos como `sessionTitle`.

Use `reloadSkills` quando um hook SessionStart instala ou atualiza skills. A descoberta de skill normalmente é executada antes dos hooks SessionStart terminarem, portanto arquivos que o hook escreve em `~/.claude/skills/` ou `.claude/skills/` de outra forma só apareceriam na próxima sessão. Este exemplo sincroniza um repositório de skills compartilhado e solicita a nova verificação:

```bash theme={null}
#!/bin/bash

git -C ~/.claude/skills/team-skills pull --quiet 2>/dev/null || \
  git clone --quiet https://git.example.com/your-org/team-skills.git ~/.claude/skills/team-skills

echo '{"hookSpecificOutput": {"hookEventName": "SessionStart", "reloadSkills": true}}'
```

A URL do repositório é um espaço reservado; substitua-a pelo seu próprio repositório de skills. Com o espaço reservado, o clone falha e imprime uma mensagem `fatal:` para stderr. Stderr de um hook SessionStart que sai com 0 é apenas informativo, portanto a solicitação `reloadSkills` ainda se aplica.

<h4 id="persist-environment-variables">
  Persistir variáveis de ambiente
</h4>

Os hooks SessionStart têm acesso à variável de ambiente `CLAUDE_ENV_FILE`, que fornece um caminho de arquivo onde você pode persistir variáveis de ambiente para comandos Bash subsequentes.

Para definir variáveis de ambiente individuais, escreva instruções `export` para `CLAUDE_ENV_FILE`. Use append (`>>`) para preservar variáveis definidas por outros hooks:

```bash theme={null}
#!/bin/bash

if [ -n "$CLAUDE_ENV_FILE" ]; then
  echo 'export NODE_ENV=production' >> "$CLAUDE_ENV_FILE"
  echo 'export DEBUG_LOG=true' >> "$CLAUDE_ENV_FILE"
  echo 'export PATH="$PATH:./node_modules/.bin"' >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

Para capturar todas as mudanças de ambiente de comandos de configuração, compare as variáveis exportadas antes e depois:

```bash theme={null}
#!/bin/bash

ENV_BEFORE=$(export -p | sort)

# Run your setup commands that modify the environment
source ~/.nvm/nvm.sh
nvm use 20

if [ -n "$CLAUDE_ENV_FILE" ]; then
  ENV_AFTER=$(export -p | sort)
  comm -13 <(echo "$ENV_BEFORE") <(echo "$ENV_AFTER") >> "$CLAUDE_ENV_FILE"
fi

exit 0
```

<Note>
  `CLAUDE_ENV_FILE` está disponível para hooks SessionStart, [Setup](#setup), [CwdChanged](#cwdchanged) e [FileChanged](#filechanged). Outros tipos de hook não têm acesso a esta variável.
</Note>

<h3 id="setup">
  Setup
</h3>

Disparado apenas quando você inicia Claude Code com `--init-only`, ou com `--init` ou `--maintenance` em [modo não interativo](/docs/pt/headless) com a flag `-p`. Não é disparado no startup normal. Use-o para instalação de dependência única ou limpeza agendada que você dispara explicitamente de CI ou scripts, separado do startup normal da sessão. Para inicialização por sessão, use [SessionStart](#sessionstart) em vez disso.

O valor do matcher corresponde à flag CLI que disparou o hook:

| Matcher       | Quando é disparado                         |
| :------------ | :----------------------------------------- |
| `init`        | `claude --init-only` ou `claude -p --init` |
| `maintenance` | `claude -p --maintenance`                  |

Quando você executa `claude --init-only`, Claude Code executa hooks Setup e hooks `SessionStart` com o matcher `startup`, depois sai sem iniciar uma conversa.

Quando você inicia ou continua uma conversa com `-p`, você também precisa fornecer um prompt, como um argumento ou canalizado em stdin. Você pode pular o prompt quando um hook `SessionStart` fornece [`initialUserMessage`](#sessionstart-decision-control) ou quando você retoma uma sessão com uma [chamada de ferramenta adiada](#defer-a-tool-call-for-later).

No sucesso, `--init-only` não imprime nada no terminal. Para confirmar que os hooks foram executados, comece com `claude --debug-file <path> --init-only`, substituindo `<path>` por um local de arquivo de log, e verifique o log para as entradas de hook Setup e SessionStart.

Como Setup não é disparado a cada lançamento, um plugin que precisa de uma dependência instalada não pode contar apenas com Setup. O padrão prático é verificar a dependência no primeiro uso e instalar se ausente, por exemplo um hook ou skill que testa `${CLAUDE_PLUGIN_DATA}/node_modules` e executa `npm install` se ausente. Veja o [diretório de dados persistentes](/docs/pt/plugins/components#path-variables-and-persistent-data) para onde armazenar dependências instaladas. Se você distribuir seu plugin através de um marketplace, você pode não precisar deste padrão: Claude Code [instala automaticamente dependências de pacote Node.js elegíveis](/docs/pt/plugins/loading#node-js-package-dependencies) quando armazena em cache o plugin.

<h4 id="setup-input">
  Entrada Setup
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks Setup recebem um campo `trigger` definido como `"init"` ou `"maintenance"`:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Setup",
  "trigger": "init"
}
```

<h4 id="setup-decision-control">
  Controle de decisão Setup
</h4>

Os hooks Setup não podem bloquear; a execução continua em qualquer código de saída. Em cada código de saída, Claude Code descarta os [campos de saída JSON](#json-output) de um hook Setup, como `systemMessage`, `continue` e `hookSpecificOutput.additionalContext`. Com `-p`, stdout, stderr e código de saída de um hook Setup aparecem na saída da execução apenas como [eventos `hook_response`](/docs/pt/headless#read-session-metadata) quando você inicia com `--output-format stream-json --verbose`.

Os hooks Setup têm acesso a `CLAUDE_ENV_FILE`. Variáveis escritas nesse arquivo persistem em comandos Bash subsequentes para a sessão, assim como em [hooks SessionStart](#persist-environment-variables). Apenas hooks `type: "command"` são executados em `Setup`. Um hook `type: "mcp_tool"` em `Setup` é sempre pulado, conforme descrito em [campos de hook de ferramenta MCP](#mcp-tool-hook-fields).

<h3 id="instructionsloaded">
  InstructionsLoaded
</h3>

Disparado quando um arquivo `CLAUDE.md` ou `.claude/rules/*.md` é carregado no contexto. Este evento é disparado no início da sessão para arquivos carregados com entusiasmo e novamente mais tarde quando arquivos são carregados preguiçosamente, por exemplo quando Claude acessa um subdiretório que contém um `CLAUDE.md` aninhado ou quando regras condicionais com frontmatter `paths:` correspondem. O hook não suporta bloqueio ou controle de decisão. Ele é executado de forma assíncrona para fins de observabilidade.

Este evento não é disparado quando Claude [lê `AGENTS.md` diretamente](/docs/pt/memory#agents-md) através da configuração **Project instructions**. Ele é disparado quando um `CLAUDE.md` importa seu `AGENTS.md`, com `load_reason` definido como `include` como para qualquer outro arquivo importado, e quando `CLAUDE.md` é um symlink para ele, como um carregamento normal de `CLAUDE.md`.

O matcher é executado contra `load_reason`. Por exemplo, use `"matcher": "session_start"` para disparar apenas para arquivos carregados no início da sessão, ou `"matcher": "path_glob_match|nested_traversal"` para disparar apenas para carregamentos preguiçosos.

<h4 id="instructionsloaded-input">
  Entrada InstructionsLoaded
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks InstructionsLoaded recebem estes campos:

| Campo               | Descrição                                                                                                                                                                                                                              |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `file_path`         | Caminho absoluto para o arquivo de instrução que foi carregado                                                                                                                                                                         |
| `memory_type`       | Escopo do arquivo: `"User"`, `"Project"`, `"Local"` ou `"Managed"`                                                                                                                                                                     |
| `load_reason`       | Por que o arquivo foi carregado: `"session_start"`, `"nested_traversal"`, `"path_glob_match"`, `"include"` ou `"compact"`. O valor `"compact"` é disparado quando arquivos de instrução são recarregados após um evento de compactação |
| `globs`             | Padrões de glob de caminho do frontmatter `paths:` do arquivo, se houver. Presente apenas para carregamentos `path_glob_match`                                                                                                         |
| `trigger_file_path` | Caminho para o arquivo cujo acesso disparou este carregamento, para carregamentos preguiçosos                                                                                                                                          |
| `parent_file_path`  | Caminho para o arquivo de instrução pai que incluiu este, para carregamentos `include`                                                                                                                                                 |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "InstructionsLoaded",
  "file_path": "/Users/my-project/CLAUDE.md",
  "memory_type": "Project",
  "load_reason": "session_start"
}
```

<h4 id="instructionsloaded-decision-control">
  Controle de decisão InstructionsLoaded
</h4>

Os hooks InstructionsLoaded não têm controle de decisão. Eles não podem bloquear ou modificar o carregamento de instruções. Claude Code descarta seus [campos de saída JSON](#json-output), como `systemMessage` e `continue`. Use este evento para auditoria de log, rastreamento de conformidade ou observabilidade.

<h3 id="userpromptsubmit">
  UserPromptSubmit
</h3>

Executado quando o usuário envia um prompt, antes de Claude processá-lo. Isso permite que você adicione contexto adicional com base no prompt/conversa, valide prompts ou bloqueie certos tipos de prompts.

Os hooks `UserPromptSubmit` têm um tempo limite padrão de 30 segundos para tipos `command`, `http` e `mcp_tool`, mais curto que o padrão de 600 segundos para esses tipos na maioria dos outros eventos. Como este hook é executado antes de cada prompt e bloqueia o processamento do modelo até ser concluído, um hook travado paralisa a sessão. Se seu hook precisar de mais tempo, defina o campo `timeout` na entrada do hook.

Além de um hook de comando que você executa com [`async: true`](#run-hooks-in-the-background), um hook `UserPromptSubmit` command, HTTP ou MCP tool que atinge seu tempo limite é cancelado e sua saída, incluindo qualquer `additionalContext`, é descartada. O prompt ainda chega ao Claude sem esse contexto. A transcrição mostra um aviso nomeando o hook, o tempo limite que foi disparado e que a saída foi descartada.

Um [hook de callback do Agent SDK](/docs/pt/agent-sdk/hooks) em `UserPromptSubmit` que atinge seu tempo limite bloqueia o prompt com uma mensagem nomeando o hook e o tempo limite, porque um callback lá pode estar agindo como um portão de política que não deve falhar aberto. A sessão continua. Antes da v2.1.208, um tempo limite de callback nesse evento terminava o turno com um erro de execução.

<h4 id="userpromptsubmit-input">
  Entrada UserPromptSubmit
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks UserPromptSubmit recebem o campo `prompt` contendo o texto que o usuário enviou. Conteúdo colado que colapsou para um espaço reservado `[Pasted text #N]` chega expandido no lugar. Em sessões onde Claude Code [marca texto colado para Claude](/docs/pt/terminal-config#how-claude-treats-pasted-text), esse conteúdo expandido fica entre uma linha `<pasted_content id="…">` e uma linha `</pasted_content id="…">`, portanto leve em conta essas linhas se seu hook analisa o prompt.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptSubmit",
  "prompt": "Write a function to calculate the factorial of a number"
}
```

<h4 id="userpromptsubmit-decision-control">
  Controle de decisão UserPromptSubmit
</h4>

Os hooks `UserPromptSubmit` podem controlar se um prompt do usuário é processado e adicionar contexto. Todos os [campos de saída JSON](#json-output) estão disponíveis.

Existem duas maneiras de adicionar contexto à conversa no código de saída 0:

* **Stdout de texto simples**: Claude Code adiciona stdout que [trata como texto simples](#exit-code-0) ao contexto do Claude
* **JSON com `additionalContext`**: use o formato JSON abaixo para mais controle. O campo `additionalContext` é adicionado como contexto

Nenhum canal produz uma entrada de transcrição visível. Stdout simples e o valor `additionalContext` são cada um injetados como um lembrete do sistema que começa com o nome do hook; Claude lê ambos. Para confirmar a entrega, verifique o [log de depuração](#debug-hooks).

Para bloquear um prompt, retorne um objeto JSON com `decision` definido como `"block"`:

| Campo                    | Descrição                                                                                                                         |
| :----------------------- | :-------------------------------------------------------------------------------------------------------------------------------- |
| `decision`               | `"block"` impede que o prompt seja processado e o apaga do contexto. Omita para permitir que o prompt prossiga                    |
| `reason`                 | Mostrado ao usuário quando `decision` é `"block"`. Não adicionado ao contexto                                                     |
| `additionalContext`      | String adicionada ao contexto do Claude ao lado do prompt enviado. Veja [Adicionar contexto para Claude](#add-context-for-claude) |
| `sessionTitle`           | Define o título da sessão. Use para nomear sessões automaticamente com base no conteúdo do prompt                                 |
| `suppressOriginalPrompt` | Se `true` quando `decision` é `"block"`, omite o texto do prompt original da mensagem de bloqueio mostrada ao usuário             |

Um hook que bloqueia ao sair com 2 roteia da mesma forma que `reason`: a mensagem de bloqueio mostra o texto stderr ao usuário e não é adicionada ao contexto.

```json theme={null}
{
  "decision": "block",
  "reason": "Explanation for decision",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "My additional context here",
    "sessionTitle": "My session title"
  }
}
```

<h3 id="userpromptexpansion">
  UserPromptExpansion
</h3>

Executado quando um comando digitado pelo usuário se expande em um prompt antes de chegar ao Claude. Use isso para bloquear comandos específicos de invocação direta, injetar contexto para uma skill particular ou registrar quais comandos os usuários invocam. Por exemplo, um hook correspondente a `deploy` pode bloquear `/deploy` a menos que um arquivo de aprovação esteja presente, ou um hook correspondente a uma skill de revisão pode anexar a lista de verificação de revisão da equipe como `additionalContext`.

Este evento cobre o caminho que `PreToolUse` não cobre: um hook `PreToolUse` correspondente à ferramenta `Skill` é disparado apenas quando Claude chama a ferramenta, mas digitar `/skillname` diretamente ignora `PreToolUse`. `UserPromptExpansion` é disparado nesse caminho direto.

Corresponde a `command_name`. Deixe o matcher vazio para disparar em cada comando do tipo prompt.

<h4 id="userpromptexpansion-input">
  Entrada UserPromptExpansion
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks UserPromptExpansion recebem `expansion_type`, `command_name`, `command_args`, `command_source` e a string `prompt` original. O campo `expansion_type` é `slash_command` para skills e comandos personalizados, ou `mcp_prompt` para prompts do servidor MCP.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../00893aaf.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "UserPromptExpansion",
  "expansion_type": "slash_command",
  "command_name": "example-skill",
  "command_args": "arg1 arg2",
  "command_source": "plugin",
  "prompt": "/example-skill arg1 arg2"
}
```

<h4 id="userpromptexpansion-decision-control">
  Controle de decisão UserPromptExpansion
</h4>

Os hooks `UserPromptExpansion` podem bloquear a expansão ou adicionar contexto. Todos os [campos de saída JSON](#json-output) estão disponíveis.

| Campo               | Descrição                                                                                                                           |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------------------- |
| `decision`          | `"block"` impede que o comando se expanda. Omita para permitir que prossiga                                                         |
| `reason`            | Mostrado ao usuário quando `decision` é `"block"`                                                                                   |
| `additionalContext` | String adicionada ao contexto do Claude ao lado do prompt expandido. Veja [Adicionar contexto para Claude](#add-context-for-claude) |

Um hook que bloqueia ao sair com 2 roteia da mesma forma que `reason`: a mensagem de bloqueio mostra o texto stderr ao usuário.

```json theme={null}
{
  "decision": "block",
  "reason": "This slash command is not available",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptExpansion",
    "additionalContext": "Additional context for this expansion"
  }
}
```

<h3 id="messagedisplay">
  MessageDisplay
</h3>

Executado enquanto uma mensagem do assistente é transmitida para a tela. Claude Code exibe a mensagem em incrementos: cada vez que um lote de linhas recém-concluídas está pronto para renderizar, o hook é executado uma vez com essas linhas e Claude Code renderiza o texto de substituição do hook em seu lugar. Uma mensagem longa produz várias chamadas; uma mensagem curta pode produzir apenas uma.

Use MessageDisplay para:

* remover markdown para uma exibição mínima
* transformar o texto que um aplicativo Agent SDK mostra aos seus usuários
* redactar chaves de API ou nomes de host internos das respostas do Claude

Claude Code mantém cada lote até que seu hook retorne, portanto mantenha o hook rápido. Se o hook falhar ou atingir o tempo limite, Claude Code exibe o texto original. O tempo limite padrão para este evento é 10 segundos; se seu hook precisar de mais tempo, defina o campo `timeout` na entrada do hook.

MessageDisplay é apenas para exibição: o texto de substituição altera apenas o que é renderizado na tela. A transcrição e o que Claude vê mantêm o texto original, portanto Claude nunca vê a substituição, e o modo detalhado mostra o original. O hook recebe apenas texto de mensagem do assistente, portanto resultados de ferramentas e o texto que você digita são renderizados inalterados.

MessageDisplay não suporta matchers e é disparado para cada mensagem do assistente que transmite texto; mensagens sem texto, como respostas apenas de chamada de ferramenta, não o disparam.

Em execuções não interativas, incluindo consultas do Agent SDK e `claude -p`, MessageDisplay é executado uma vez por mensagem do assistente em vez de uma vez por lote de linhas. A chamada única chega após a mensagem ser concluída e carrega o texto completo da mensagem: `index` é `0`, `final` é `true` e `delta` contém a mensagem inteira. Um hook que coleta o texto `delta` para cada mensagem recebe o mesmo texto total em ambos os modos.

<h4 id="messagedisplay-input">
  Entrada MessageDisplay
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks MessageDisplay recebem identificadores para o turno e mensagem, a posição desta chamada dentro da mensagem e o novo texto em `delta`. Os limites de lote dependem de como o texto é transmitido, portanto use `index` e `final` para rastrear o progresso através de uma mensagem em vez de esperar que as linhas sejam agrupadas de uma forma particular.

| Campo        | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `turn_id`    | UUID do turno atual                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `message_id` | UUID da mensagem do assistente sendo exibida. Estável em cada lote da mesma mensagem. Este não é o `msg_…` id da API, portanto não pode ser correlacionado com ids de mensagem de transcrição                                                                                                                                                                                                                                                              |
| `index`      | Índice baseado em zero deste lote dentro da mensagem                                                                                                                                                                                                                                                                                                                                                                                                       |
| `final`      | `true` no último lote da mensagem. Cada mensagem tem exatamente um lote final                                                                                                                                                                                                                                                                                                                                                                              |
| `delta`      | As linhas recém-concluídas desde o lote anterior, incluindo quebras de linha finais. Sempre linhas inteiras, exceto o lote final que pode terminar no meio de uma linha. Em execuções interativas, o delta do lote final está vazio quando a mensagem termina em uma quebra de linha, portanto trate `final`, não um delta não vazio, como o sinal de fim de mensagem. Em execuções do Agent SDK e `claude -p`, a chamada única carrega a mensagem inteira |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "MessageDisplay",
  "turn_id": "0c9e6a2f-7d41-4f4e-9a15-3f4f7c2b8d10",
  "message_id": "5b2a9c8e-1f63-4d8a-b7c4-9e0d2a6f1c3b",
  "index": 0,
  "final": false,
  "delta": "Here is the plan:\n"
}
```

<h4 id="messagedisplay-output">
  Saída MessageDisplay
</h4>

Além dos [campos de saída JSON](#json-output) disponíveis para todos os hooks, os hooks MessageDisplay podem retornar `displayContent` para substituir o delta na tela:

| Campo            | Descrição                                                     |
| :--------------- | :------------------------------------------------------------ |
| `displayContent` | Texto exibido no lugar do delta. Omita para exibir o original |

Os hooks MessageDisplay não têm controle de decisão. Eles não podem bloquear a mensagem ou alterar o que é armazenado na transcrição ou enviado ao Claude. Claude Code atua em `displayContent` de sua saída JSON e descarta `systemMessage` e `continue`.

Este exemplo remove formatação markdown das respostas do Claude para uma exibição de texto simples. O script lê cada lote de stdin, remove marcadores em negrito e backticks de código inline de `delta` e retorna o resultado como `displayContent`.

<Tabs>
  <Tab title="macOS/Linux">
    Registre um hook de comando para o evento em seu arquivo de configurações:

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.sh",
                "args": []
              }
            ]
          }
        ]
      }
    }
    ```

    Salve este script em `.claude/hooks/plain-display.sh` em seu projeto e torne-o executável com `chmod +x`:

    ```bash theme={null}
    #!/bin/bash
    jq '{hookSpecificOutput: {hookEventName: "MessageDisplay", displayContent: (.delta | gsub("\\*\\*"; "") | gsub("`"; ""))}}'
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    Registre um hook de comando que executa o script através do PowerShell:

    ```json theme={null}
    {
      "hooks": {
        "MessageDisplay": [
          {
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/plain-display.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    A flag `-NoProfile` pula o carregamento de seu perfil do PowerShell para que o hook comece rápido, e `-ExecutionPolicy Bypass` permite que o PowerShell execute o arquivo de script local.

    Salve este script em `.claude/hooks/plain-display.ps1` em seu projeto:

    ```powershell theme={null}
    $batch = [Console]::In.ReadToEnd() | ConvertFrom-Json
    $text = $batch.delta -replace '\*\*', '' -replace '`', ''
    @{
      hookSpecificOutput = @{
        hookEventName = "MessageDisplay"
        displayContent = $text
      }
    } | ConvertTo-Json
    ```
  </Tab>
</Tabs>

Lotes sem markdown passam inalterados. Se o script falhar, por exemplo porque `jq` está faltando, Claude Code exibe o texto original e nota a falha apenas em [saída de depuração](#debug-hooks), não na sessão.

<h3 id="pretooluse">
  PreToolUse
</h3>

Executado após Claude criar parâmetros de ferramenta e antes de processar a chamada de ferramenta. Corresponde a qualquer nome de ferramenta exceto `EndConversation`: ferramentas integradas como `Bash`, `PowerShell`, `Edit`, `Write`, `Read`, `Glob`, `Grep`, `Agent`, `Workflow`, `WebFetch`, `WebSearch`, `AskUserQuestion` e `ExitPlanMode`, e qualquer [nome de ferramenta MCP](#match-mcp-tools).

Para executar um hook quando um arquivo específico muda no disco, seja qual for o que o escreveu, use [FileChanged](#filechanged) em vez de corresponder a ferramentas de edição de arquivo por nome. Ao contrário de PreToolUse, Claude Code executa hooks FileChanged após a mudança, e eles não têm controle de decisão, portanto não podem bloquear a escrita.

<Warning>
  PreToolUse é executado apenas quando Claude chama uma ferramenta. Arquivos que você [referencia com `@` em seu prompt](/docs/pt/common-workflows#reference-files-and-directories) são adicionados sem nenhuma chamada de ferramenta: Claude Code insere seu conteúdo ao construir o prompt, portanto nenhum hook PreToolUse é disparado para eles, incluindo hooks correspondentes a `Read`. Para bloquear caminhos específicos de referências `@`, use uma [regra de negação `Read`](/docs/pt/permissions#read-and-edit) em vez disso.

  PreToolUse também não é disparado para [`EndConversation`](/docs/pt/tools-reference#endconversation-tool-behavior).
</Warning>

Use [controle de decisão PreToolUse](#pretooluse-decision-control) para permitir, negar, perguntar ou adiar a chamada de ferramenta.

Um [hook de callback do Agent SDK](/docs/pt/agent-sdk/hooks) em `PreToolUse` que excede seu tempo limite bloqueia a chamada de ferramenta, e Claude recebe um resultado de erro nomeando o tempo limite. Uma negação explícita retornada por outro hook ainda tem precedência.

<h4 id="pretooluse-input">
  Entrada PreToolUse
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks PreToolUse recebem `tool_name`, `tool_input` e `tool_use_id`.

Para uma [ferramenta MCP](#match-mcp-tools), a entrada também carrega `mcp_server`, um objeto com o `name` do servidor e uma `source` que diz de onde veio a definição do servidor. Os valores `source` incluem `plugin`, `sdk` e escopos de configuração como `user` e `project`. [`McpServerProvenance`](/docs/pt/agent-sdk/typescript#mcpserverprovenance) na referência do Agent SDK lista todos eles e diz como tratar um que você não reconhece. Baseie decisões de confiança em `source` em vez de em `name` ou no prefixo de nome de ferramenta `mcp__<server>__`. O campo `mcp_server` requer Claude Code v2.1.274 ou posterior.

Para as ferramentas de arquivo `Write`, `Edit` e `Read`, `tool_input.file_path` é sempre absoluto:

* Claude Code expande `~` e caminhos relativos antes dos hooks serem executados, portanto um hook que corresponde a caminhos não pode ser contornado via `~` ou uma ortografia relativa do mesmo caminho
* No Windows, o caminho chega com separadores de barra invertida, mesmo quando seu hook é executado sob Git Bash onde `$PWD` parece `/c/project`
* Uma comparação escrita com barras para frente, como uma verificação `/src/`, nunca corresponde a um caminho de barra invertida, e a chamada de ferramenta prossegue como se o hook não tivesse nada a bloquear
* Normalize separadores antes de comparar: `FILE_PATH="${FILE_PATH//\\//}"` em Bash, ou `file_path.replace("\\", "/")` em Python, depois corresponda a um segmento de caminho como `/src/` em vez de ancorar com `^`, já que o caminho é absoluto

Uma chamada `Write` no Windows entrega:

```json theme={null}
{
  "hook_event_name": "PreToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "C:\\project\\src\\index.ts",
    "content": "..."
  },
  ...
}
```

Os campos `tool_input` dependem da ferramenta:

<a id="bash" />

<h5 id="bash">
  Bash
</h5>

Executa comandos de shell.

| Campo               | Tipo    | Exemplo            | Descrição                                                                                                                                              |
| :------------------ | :------ | :----------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | string  | `"npm test"`       | O comando de shell a executar                                                                                                                          |
| `description`       | string  | `"Run test suite"` | Descrição opcional do que o comando faz                                                                                                                |
| `timeout`           | number  | `120000`           | Tempo limite opcional em milissegundos. Valores acima do [máximo](/docs/pt/tools-reference#bash-tool-behavior) são reduzidos ao máximo em vez de rejeitados |
| `run_in_background` | boolean | `false`            | Se o comando deve ser executado em segundo plano                                                                                                       |

Quando um comando Bash muda arquivos em um repositório Git, Claude Code pode registrar o que mudou. Ele registra as mudanças em cada modo de permissão quando a configuração [`bashEditDiffEnabled`](/docs/pt/settings-reference#basheditdiffenabled) ativa o registro; a entrada dessa configuração diz quais arquivos podem defini-la. Caso contrário, ele as registra apenas em modo automático e modo `bypassPermissions`, e apenas quando Claude Code direciona Claude a editar arquivos através de Bash. Defina `bashEditDiffEnabled` como `false` para desativar o registro. Comandos de fundo e comandos somente leitura não carregam diff.

Seu [hook PostToolUse](#posttooluse) então recebe os arquivos alterados em `tool_response.bashEditDiff`. A lista cobre o que mudou sob o repositório enquanto o comando era executado. Arquivos que Git ignora e arquivos em submódulos não são listados. Requer Claude Code v2.1.269 ou posterior.

<Note>
  A lista é melhor esforço e em beta público. Claude Code pode perder uma mudança, incluir um arquivo que outro processo mudou ao mesmo tempo, ou parar em seus limites de tamanho. A forma do campo pode mudar. Use a lista para encontrar o que revisar, não para impor uma política.
</Note>

`changedFiles` e `files` listam o que o comando mudou; os campos restantes dizem como completo e confiável essa lista é.

| Campo          | Tipo    | Exemplo                                                 | Descrição                                                                                                                                                                               |
| :------------- | :------ | :------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `changedFiles` | array   | `["/path/to/src/app.ts"]`                               | Caminhos absolutos dos arquivos que o comando mudou, no máximo 200. Presente sempre que `files` contém um diff ou `moreFiles` está acima de zero                                        |
| `files`        | array   | `[{"filePath": "/path/to/src/app.ts", "hunks": [...]}]` | Diffs de até 5 arquivos alterados, para exibição. `created` ou `deleted` é `true` para um arquivo que o comando adicionou ou removeu                                                    |
| `moreFiles`    | number  | `2`                                                     | Contagem de arquivos alterados sem diff em `files`                                                                                                                                      |
| `unavailable`  | boolean | `true`                                                  | Definido quando o diff está incompleto ou não pôde ser obtido                                                                                                                           |
| `skipped`      | boolean | `true`                                                  | Definido para um comando Git que move a árvore de trabalho, como `git checkout` ou `git stash`, portanto Claude Code não obtém diff                                                     |
| `shared`       | boolean | `true`                                                  | Definido quando outra chamada de ferramenta Bash, como a de um subagente, foi executada no mesmo repositório ao mesmo tempo, portanto algumas mudanças listadas podem ser desse comando |

<a id="powershell" />

<h5 id="powershell">
  PowerShell
</h5>

Executa comandos do PowerShell. Veja a [ferramenta PowerShell](/docs/pt/tools-reference#powershell-tool) para disponibilidade por plataforma.

Os campos correspondem à ferramenta Bash, com a string de comando em `command`:

| Campo               | Tipo    | Exemplo                    | Descrição                                        |
| :------------------ | :------ | :------------------------- | :----------------------------------------------- |
| `command`           | string  | `"Get-ChildItem -Recurse"` | O comando do PowerShell a executar               |
| `description`       | string  | `"List files recursively"` | Descrição opcional do que o comando faz          |
| `timeout`           | number  | `120000`                   | Tempo limite opcional em milissegundos           |
| `run_in_background` | boolean | `false`                    | Se o comando deve ser executado em segundo plano |

Corresponda a `Bash|PowerShell` em hooks que inspecionam comandos de shell, para que cubram ambas as ferramentas:

* No Windows, onde quer que a ferramenta PowerShell esteja habilitada, Claude trata o PowerShell como o shell primário e roteia comandos de shell através dele.
* No Windows sem Git Bash, a ferramenta é habilitada automaticamente e Claude Code não registra a ferramenta Bash.
* Um hook que corresponde apenas a `Bash` nunca é disparado lá.

<h5 id="write">
  Write
</h5>

Cria ou sobrescreve um arquivo.

| Campo       | Tipo   | Exemplo               | Descrição                                  |
| :---------- | :----- | :-------------------- | :----------------------------------------- |
| `file_path` | string | `"/path/to/file.txt"` | Caminho absoluto para o arquivo a escrever |
| `content`   | string | `"file content"`      | Conteúdo a escrever no arquivo             |

<h5 id="edit">
  Edit
</h5>

Substitui uma string em um arquivo existente.

| Campo         | Tipo    | Exemplo               | Descrição                                |
| :------------ | :------ | :-------------------- | :--------------------------------------- |
| `file_path`   | string  | `"/path/to/file.txt"` | Caminho absoluto para o arquivo a editar |
| `old_string`  | string  | `"original text"`     | Texto a encontrar e substituir           |
| `new_string`  | string  | `"replacement text"`  | Texto de substituição                    |
| `replace_all` | boolean | `false`               | Se deve substituir todas as ocorrências  |

<h5 id="read">
  Read
</h5>

Lê conteúdo de arquivo.

| Campo       | Tipo   | Exemplo               | Descrição                                   |
| :---------- | :----- | :-------------------- | :------------------------------------------ |
| `file_path` | string | `"/path/to/file.txt"` | Caminho absoluto para o arquivo a ler       |
| `offset`    | number | `10`                  | Número de linha opcional para começar a ler |
| `limit`     | number | `50`                  | Número opcional de linhas a ler             |

<h5 id="glob">
  Glob
</h5>

Encontra arquivos correspondentes a um padrão glob.

| Campo     | Tipo   | Exemplo          | Descrição                                                               |
| :-------- | :----- | :--------------- | :---------------------------------------------------------------------- |
| `pattern` | string | `"**/*.ts"`      | Padrão glob para corresponder arquivos                                  |
| `path`    | string | `"/path/to/dir"` | Diretório opcional para pesquisar. Padrão é diretório de trabalho atual |

<h5 id="grep">
  Grep
</h5>

Pesquisa conteúdo de arquivo com expressões regulares.

| Campo         | Tipo    | Exemplo          | Descrição                                                                         |
| :------------ | :------ | :--------------- | :-------------------------------------------------------------------------------- |
| `pattern`     | string  | `"TODO.*fix"`    | Padrão de expressão regular para pesquisar                                        |
| `path`        | string  | `"/path/to/dir"` | Arquivo ou diretório opcional para pesquisar                                      |
| `glob`        | string  | `"*.ts"`         | Padrão glob opcional para filtrar arquivos                                        |
| `output_mode` | string  | `"content"`      | `"content"`, `"files_with_matches"` ou `"count"`. Padrão é `"files_with_matches"` |
| `-i`          | boolean | `true`           | Pesquisa insensível a maiúsculas e minúsculas                                     |
| `multiline`   | boolean | `false`          | Habilitar correspondência multilinha                                              |

<h5 id="webfetch">
  WebFetch
</h5>

Busca e processa conteúdo da web.

| Campo    | Tipo   | Exemplo                       | Descrição                                |
| :------- | :----- | :---------------------------- | :--------------------------------------- |
| `url`    | string | `"https://example.com/api"`   | URL para buscar conteúdo                 |
| `prompt` | string | `"Extract the API endpoints"` | Prompt para executar no conteúdo buscado |

<h5 id="websearch">
  WebSearch
</h5>

Pesquisa a web.

| Campo             | Tipo   | Exemplo                        | Descrição                                           |
| :---------------- | :----- | :----------------------------- | :-------------------------------------------------- |
| `query`           | string | `"react hooks best practices"` | Consulta de pesquisa                                |
| `allowed_domains` | array  | `["docs.example.com"]`         | Opcional: incluir apenas resultados desses domínios |
| `blocked_domains` | array  | `["spam.example.com"]`         | Opcional: excluir resultados desses domínios        |

<h5 id="agent">
  Agent
</h5>

Gera um [subagente](/docs/pt/sub-agents).

| Campo           | Tipo   | Exemplo                    | Descrição                                           |
| :-------------- | :----- | :------------------------- | :-------------------------------------------------- |
| `prompt`        | string | `"Find all API endpoints"` | A tarefa para o agente executar                     |
| `description`   | string | `"Find API endpoints"`     | Descrição curta da tarefa                           |
| `subagent_type` | string | `"Explore"`                | Tipo de agente especializado a usar                 |
| `model`         | string | `"sonnet"`                 | Alias de modelo opcional para sobrescrever o padrão |

Quando uma chamada Agent em primeiro plano é concluída, seu [hook PostToolUse](#posttooluse) recebe o resultado do subagente e telemetria de execução em `tool_response`. Leia esses campos para inspecionar a execução; para rollups de token e custo entre subagentes, use os [contadores de token e custo](/docs/pt/monitoring-usage#token-counter) filtrados para `query_source` `"subagent"`, já que `totalTokens` e `usage` cobrem apenas a solicitação final:

| Campo               | Tipo   | Exemplo                                               | Descrição                                                                                                                                                                                                                                                   |
| :------------------ | :----- | :---------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`            | string | `"completed"`                                         | `"completed"` para subagentes em primeiro plano, `"async_launched"` para subagentes em segundo plano. A partir da v2.1.198, subagentes são executados em segundo plano por padrão, portanto um `run_in_background` omitido também produz `"async_launched"` |
| `agentId`           | string | `"a4d2c8f1e0b3a297"`                                  | Identificador para a execução do subagente                                                                                                                                                                                                                  |
| `content`           | array  | `[{"type": "text", "text": "Found 12 endpoints..."}]` | Os blocos de texto final do subagente, ou, para um subagente cujo relatório passa por `SubagentHandback`, uma nota breve sobre esse hand-back em seu lugar                                                                                                  |
| `resolvedModel`     | string | `"claude-sonnet-4-5"`                                 | Modelo em que o subagente começou, que pode diferir do modelo solicitado                                                                                                                                                                                    |
| `modelsUsed`        | array  | `["claude-sonnet-4-5", "claude-haiku-4-5"]`           | Modelos usados em ordem, com repetições consecutivas colapsadas; definido apenas quando o modelo foi trocado durante a execução. Requer Claude Code v2.1.212 ou posterior                                                                                   |
| `totalTokens`       | number | `12450`                                               | Contagem de tokens da solicitação final da API do subagente: tokens de entrada, saída e cache combinados. Isso não é um total em toda a execução                                                                                                            |
| `totalDurationMs`   | number | `48211`                                               | Duração de relógio de parede da execução do subagente                                                                                                                                                                                                       |
| `totalToolUseCount` | number | `7`                                                   | Contagem de chamadas de ferramenta que o subagente fez                                                                                                                                                                                                      |
| `usage`             | object | `{"input_tokens": 8320, ...}`                         | Divisão de tokens por tipo da solicitação final da API: `input_tokens`, `output_tokens`, `cache_creation_input_tokens`, `cache_read_input_tokens`                                                                                                           |

No Claude Code v2.1.271 ou posterior, um subagente que é executado com a ferramenta [`SubagentHandback`](/docs/pt/tools-reference), que Claude Code fornece em [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode), entrega seu relatório através dessa ferramenta em vez de retorná-lo como texto. O campo `content` de seu resultado `completed` então carrega uma nota breve sobre esse hand-back em vez do relatório em si. Para ler o relatório, corresponda a um hook `PreToolUse` ou `PostToolUse` em `SubagentHandback` e leia `tool_input.message`.

Para subagentes em segundo plano, a ferramenta retorna quando a tarefa se move para o fundo, portanto `tool_response` não carrega campos de uso: um lançamento em segundo plano retorna imediatamente, e uma tarefa em primeiro plano que Claude Code coloca em segundo plano durante a execução retorna nessa transição. Ele tem `status: "async_launched"`, `agentId`, `description`, `prompt`, `outputFile` e `resolvedModel`.

Em uma resposta `completed`, `resolvedModel` nomeia o modelo em que o subagente começou, que pode diferir do valor `model` em `tool_input`, como quando `availableModels` ou outra substituição se aplica. Em uma resposta `async_launched`, `resolvedModel` nomeia o modelo em uso quando o agente se moveu para o fundo, portanto uma troca que aconteceu antes de colocar em segundo plano é refletida lá. `modelsUsed` e o comportamento `resolvedModel` no tempo de colocação em segundo plano requerem Claude Code v2.1.212 ou posterior.

<a id="askuserquestion" />

<h5 id="askuserquestion">
  AskUserQuestion
</h5>

Faz ao usuário uma a quatro perguntas de múltipla escolha.

| Campo       | Tipo   | Exemplo                                                                                                            | Descrição                                                                                                                                                                                                                 |
| :---------- | :----- | :----------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `questions` | array  | `[{"question": "Which framework?", "header": "Framework", "options": [{"label": "React"}], "multiSelect": false}]` | Perguntas a apresentar, cada uma com uma string `question`, `header` curto, array `options` e flag `multiSelect` opcional                                                                                                 |
| `answers`   | object | `{"Which framework?": "React"}`                                                                                    | Opcional. Mapeia texto de pergunta para rótulo de opção selecionada. Respostas de seleção múltipla unem rótulos com vírgulas. Claude não define este campo; forneça-o via `updatedInput` para responder programaticamente |

<h5 id="exitplanmode">
  ExitPlanMode
</h5>

Apresenta um plano e pede ao usuário para aprová-lo antes de Claude sair do [modo de plano](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode). Claude escreve o plano em um arquivo no disco antes de chamar a ferramenta, portanto o `tool_input` literal do modelo é tipicamente vazio. Claude Code injeta o conteúdo do plano e o caminho do arquivo antes de passar a entrada para hooks.

| Campo            | Tipo   | Exemplo                                     | Descrição                                                                                                                                                            |
| :--------------- | :----- | :------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plan`           | string | `"## Refactor auth\n1. Extract..."`         | Conteúdo do plano em Markdown. Injetado do arquivo de plano no disco                                                                                                 |
| `planFilePath`   | string | `"/Users/.../plans/refactor-auth.md"`       | Caminho para o arquivo de plano. Injetado                                                                                                                            |
| `allowedPrompts` | array  | `[{"tool": "Bash", "prompt": "run tests"}]` | Descontinuado. Claude Code aceita o campo mas o ignora. Antes da v2.1.205, ele carregava permissões baseadas em prompt que Claude solicitou para implementar o plano |

Em `PostToolUse`, `tool_response` é um objeto com campos `plan` e `filePath` contendo o plano aprovado, mais flags de status interno. Leia `tool_response.plan` para o conteúdo do plano em vez de reler o arquivo do disco.

<h4 id="pretooluse-decision-control">
  Controle de decisão PreToolUse
</h4>

Os hooks `PreToolUse` podem controlar se uma chamada de ferramenta prossegue. Ao contrário de outros hooks que usam um campo `decision` de nível superior, PreToolUse retorna sua decisão dentro de um objeto `hookSpecificOutput`. Isso lhe dá controle mais rico: quatro resultados (permitir, negar, perguntar ou adiar) mais a capacidade de modificar a entrada da ferramenta antes da execução.

| Campo                      | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionDecision`       | `"allow"` pula o prompt de permissão, exceto para as [ações que nenhum modo auto-aprova](/docs/pt/permission-modes#actions-no-mode-auto-approves) e para `AskUserQuestion` e `ExitPlanMode`, que precisam de [`updatedInput` emparelhado com ele](#allow-with-updatedinput). `"deny"` impede a chamada de ferramenta. `"ask"` solicita ao usuário para confirmar. `"defer"` sai graciosamente para que a ferramenta possa ser retomada mais tarde. [Regras de negação e pergunta](/docs/pt/permissions#manage-permissions) ainda são avaliadas independentemente do que o hook retorna |
| `permissionDecisionReason` | Para `"allow"` e `"ask"`, mostrado ao usuário mas não ao Claude. Para `"deny"`, mostrado ao Claude. Para `"defer"`, ignorado                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `updatedInput`             | Modifica os parâmetros de entrada da ferramenta antes da execução. Substitui o objeto de entrada inteiro, portanto inclua campos inalterados ao lado dos modificados. Claude Code avalia regras de permissão e a elegibilidade de [auto-fundo](/docs/pt/tools-reference#background-commands) de um comando Bash contra a entrada que seu hook retorna, não a entrada que Claude enviou. Combine com `"allow"` para auto-aprovar, ou `"ask"` para mostrar a entrada modificada ao usuário. Para `"defer"`, ignorado                                                                |
| `additionalContext`        | String adicionada ao contexto do Claude ao lado do resultado da ferramenta. Ignorado quando `permissionDecision` é `"defer"`. Veja [Adicionar contexto para Claude](#add-context-for-claude)                                                                                                                                                                                                                                                                                                                                                                                 |

Quando vários hooks PreToolUse retornam decisões diferentes, a precedência é `deny` > `defer` > `ask` > `allow`.

Um hook que bloqueia ao sair com 2 roteia da mesma forma que `"deny"`: Claude vê a mensagem stderr como o motivo da negação.

Quando um hook retorna `"ask"`, o prompt de permissão exibido ao usuário inclui um rótulo identificando de onde o hook veio: `[settings]` para um hook de qualquer arquivo de configurações ou de frontmatter de agente, `[plugin:<name>]` para um hook de plugin, ou `[skill]` para um hook de frontmatter de skill. Isso ajuda os usuários a entender qual fonte de configuração está solicitando confirmação.

Um `"ask"` de um hook também força um prompt de permissão em [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode): o classificador ainda pode negar a chamada de ferramenta, mas não pode aprovar a chamada silenciosamente. Antes da v2.1.211, o classificador poderia aprovar um comando Bash executado fora do [sandbox](/docs/pt/sandboxing) sem mostrar o prompt que o hook solicitou; o classificador ainda aplicava suas próprias regras de segurança a esse comando, e uma negação de hook `"deny"` era sempre honrada.

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow",
    "permissionDecisionReason": "My reason here",
    "updatedInput": {
      "field_to_modify": "new value"
    },
    "additionalContext": "Current environment: production. Proceed with caution."
  }
}
```

<span id="allow-with-updatedinput" />

Em [modo não interativo](/docs/pt/headless) com a flag `-p`, Claude Code oferece `AskUserQuestion` e `ExitPlanMode` apenas quando a execução tem um [host de permissão](/docs/pt/headless#turn-off-permission-prompts-in-unattended-runs) para receber o prompt, como um callback `canUseTool` do Agent SDK. Essas ferramentas requerem interação do usuário. Retornar `permissionDecision: "allow"` junto com `updatedInput` satisfaz esse requisito: o hook lê a entrada da ferramenta de stdin, coleta a resposta através de sua própria UI e a retorna em `updatedInput` para que a ferramenta seja executada sem solicitar. Retornar `"allow"` sozinho não é suficiente para essas ferramentas. Para `AskUserQuestion`, repita o array `questions` original e adicione um objeto [`answers`](#askuserquestion) mapeando o texto de cada pergunta para a resposta escolhida.

A partir da v2.1.199, uma ferramenta MCP cujo servidor a marca com [`_meta["anthropic/requiresUserInteraction"]`](/docs/pt/mcp#require-approval-for-a-specific-tool) é mais rigorosa: um hook não pode pular seu prompt de aprovação com `"allow"`, com ou sem `updatedInput`, porque Claude Code não pode confirmar que o hook coletou a interação que a ferramenta precisa.

<Note>
  PreToolUse anteriormente usava campos `decision` e `reason` de nível superior, mas estes estão descontinuados para este evento. Use `hookSpecificOutput.permissionDecision` e `hookSpecificOutput.permissionDecisionReason` em vez disso. Os valores descontinuados `"approve"` e `"block"` mapeiam para `"allow"` e `"deny"` respectivamente. Outros eventos como PostToolUse e Stop continuam usando `decision` e `reason` de nível superior como seu formato atual.
</Note>

<h4 id="defer-a-tool-call-for-later">
  Adiar uma chamada de ferramenta para mais tarde
</h4>

`"defer"` é para integrações que executam `claude -p` como um subprocesso e leem sua saída JSON, como um aplicativo Agent SDK ou uma UI personalizada construída em cima de Claude Code. Permite que esse processo de chamada pause Claude em uma chamada de ferramenta, colete entrada através de sua própria interface e retome onde parou. Claude Code honra este valor apenas em [modo não interativo](/docs/pt/headless) com a flag `-p`. Em sessões interativas, ele registra um aviso e ignora o resultado do hook.

A ferramenta `AskUserQuestion` é o caso típico: Claude quer fazer uma pergunta ao usuário, mas não há terminal para responder. Uma execução `-p` oferece `AskUserQuestion` apenas quando tem um [host de permissão](/docs/pt/headless#turn-off-permission-prompts-in-unattended-runs), como uma ferramenta MCP que você passa com `--permission-prompt-tool`, portanto comece a execução com uma. A viagem de ida e volta funciona assim:

1. Claude chama `AskUserQuestion`. O hook `PreToolUse` é disparado.
2. O hook retorna `permissionDecision: "defer"`. A ferramenta não é executada. O processo sai com `stop_reason: "tool_deferred"` e a chamada de ferramenta pendente preservada na transcrição.
3. O processo de chamada lê `deferred_tool_use` do resultado do SDK, exibe a pergunta em sua própria UI e espera por uma resposta.
4. O processo de chamada executa `claude -p --resume <session-id>` com o mesmo host de permissão. A mesma chamada de ferramenta dispara `PreToolUse` novamente.
5. O hook retorna `permissionDecision: "allow"` com a resposta em `updatedInput`. A ferramenta é executada e Claude continua.

O campo `deferred_tool_use` carrega o `id`, `name` e `input` da ferramenta. O `input` são os parâmetros que Claude gerou para a chamada de ferramenta, capturados antes da execução:

```json theme={null}
{
  "type": "result",
  "subtype": "success",
  "stop_reason": "tool_deferred",
  "session_id": "abc123",
  "deferred_tool_use": {
    "id": "toolu_01abc",
    "name": "AskUserQuestion",
    "input": { "questions": [{ "question": "Which framework?", "header": "Framework", "options": [{"label": "React"}, {"label": "Vue"}], "multiSelect": false }] }
  }
}
```

Não há tempo limite ou limite de tentativas. A sessão permanece no disco até que você a retome, sujeita à varredura de retenção [`cleanupPeriodDays`](/docs/pt/settings-reference#cleanupperioddays), que exclui arquivos de sessão após 30 dias por padrão, seguindo as [regras de varredura de retenção](/docs/pt/claude-directory#cleaned-up-automatically). Se a resposta não estiver pronta quando você retomar, o hook pode retornar `"defer"` novamente e o processo sai da mesma forma. O processo de chamada controla quando quebrar o loop eventualmente retornando `"allow"` ou `"deny"` do hook.

`"defer"` funciona apenas quando Claude faz uma única chamada de ferramenta no turno. Se Claude faz várias chamadas de ferramenta de uma vez, `"defer"` é ignorado com um aviso e a ferramenta prossegue através do fluxo de permissão normal. A restrição existe porque retomar pode apenas re-executar uma ferramenta: não há maneira de adiar uma chamada de um lote sem deixar as outras não resolvidas.

Se a ferramenta adiada não estiver mais disponível quando você retomar, o processo sai com `stop_reason: "tool_deferred_unavailable"` e `is_error: true` antes do hook ser disparado. Isso acontece quando um servidor MCP que forneceu a ferramenta não está conectado para a sessão retomada. O payload `deferred_tool_use` ainda é incluído para que você possa identificar qual ferramenta desapareceu.

<Note>
  Para retomar uma sessão adiada em modo de plano, passe [`--permission-prompt-tool`](/docs/pt/cli-reference#cli-flags) junto com `--resume` para que Claude Code possa apresentar o plano para aprovação. Sem ele, Claude Code não restaura o modo de plano. Requer Claude Code v2.1.246 ou posterior.

  Quando você retoma com `-p`, Claude Code não restaura nenhum outro modo de permissão armazenado. Ele inicia a execução no modo de permissão que uma nova execução `claude -p` iniciaria, portanto passe `--permission-mode` ou `--dangerously-skip-permissions` novamente se a sessão adiada usou uma. Quando você retoma com `claude --resume <session-id>` sem `-p`, Claude Code restaura o modo de permissão armazenado, com as exceções listadas em [modo de permissão ao retomar](/docs/pt/sessions#permission-mode-on-resume).
</Note>

<h3 id="permissionrequest">
  PermissionRequest
</h3>

Executado quando Claude Code está prestes a pedir permissão para usar uma ferramenta. Em sessões que não podem mostrar um prompt, como subagentes em segundo plano em [modo não interativo](/docs/pt/headless), Claude Code ainda executa esses hooks, e se nenhum hook retornar uma decisão, ele nega a chamada de ferramenta.
Use [controle de decisão PermissionRequest](#permissionrequest-decision-control) para permitir ou negar em nome do usuário.

Use este evento quando você precisa de um sinal no momento em que Claude pede permissão para usar uma ferramenta. Claude Code executa um hook [Notification](#notification) com o tipo `permission_prompt` apenas após o prompt ter esperado cerca de seis segundos.

Claude Code não executa hooks PermissionRequest para a [solicitação de rede](/docs/pt/sandboxing#network-isolation) de um comando em sandbox. Para obter um sinal para esse prompt, use o tipo de notificação `permission_prompt`.

Corresponde ao nome da ferramenta, mesmos valores que PreToolUse.

<h4 id="permissionrequest-input">
  Entrada PermissionRequest
</h4>

Os hooks PermissionRequest recebem campos `tool_name` e `tool_input` como hooks PreToolUse, mas sem `tool_use_id`. Para uma ferramenta MCP, eles também recebem o objeto [`mcp_server`](#pretooluse-input). Um array `permission_suggestions` opcional contém as [atualizações de permissão](#permission-update-entries) que Claude Code sugere para esta solicitação, como adicionar uma regra de permissão ou alterar o modo de permissão.

O array `permission_suggestions` não é uma lista exata das opções que você vê, porque cada diálogo de permissão constrói suas próprias opções. Alguns diálogos, como o para edições de arquivo, não leem o array e derivam suas opções da solicitação em si. Um diálogo que o faz pode ainda reter uma opção cuja sugestão permanece no array, por exemplo quando [`allowManagedPermissionRulesOnly`](/docs/pt/settings-reference#allowmanagedpermissionrulesonly) oculta opções de salvamento de regra. Ele também pode oferecer opções que não têm entrada de sugestão, como [**Yes, and switch to auto mode**](/docs/pt/permission-modes#switch-permission-modes), que altera o modo de permissão diretamente em vez de através de uma atualização de permissão.

Os hooks PreToolUse são executados antes de cada chamada de ferramenta, independentemente de precisar de permissão. Os hooks PermissionRequest são executados apenas quando Claude Code está prestes a pedir permissão, ou quando de outra forma auto-negaria uma chamada que não pode solicitar. Nenhum evento é disparado para [`EndConversation`](/docs/pt/tools-reference#endconversation-tool-behavior).

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PermissionRequest",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf node_modules",
    "description": "Remove node_modules directory"
  },
  "permission_suggestions": [
    {
      "type": "addRules",
      "rules": [{ "toolName": "Bash", "ruleContent": "rm -rf node_modules" }],
      "behavior": "allow",
      "destination": "localSettings"
    }
  ]
}
```

<h4 id="permissionrequest-decision-control">
  Controle de decisão PermissionRequest
</h4>

Os hooks `PermissionRequest` podem permitir ou negar solicitações de permissão. Além dos [campos de saída JSON](#json-output) disponíveis para todos os hooks, seu script de hook pode retornar um objeto `decision` com esses campos específicos do evento:

| Campo                | Descrição                                                                                                                                                                                                                                                           |
| :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `behavior`           | `"allow"` concede a permissão, `"deny"` a nega. [Regras de negação e pergunta](/docs/pt/permissions#manage-permissions) ainda são avaliadas, portanto um hook retornando `"allow"` não sobrescreve uma regra de negação correspondente                                   |
| `updatedInput`       | Para `"allow"` apenas: modifica os parâmetros de entrada da ferramenta antes da execução. Substitui o objeto de entrada inteiro, portanto inclua campos inalterados ao lado dos modificados. A entrada modificada é re-avaliada contra regras de negação e pergunta |
| `updatedPermissions` | Para `"allow"` apenas: array de [entradas de atualização de permissão](#permission-update-entries) a aplicar, como adicionar uma regra de permissão ou alterar o modo de permissão da sessão                                                                        |
| `message`            | Para `"deny"` apenas: diz ao Claude por que a permissão foi negada                                                                                                                                                                                                  |
| `interrupt`          | Para `"deny"` apenas: se `true`, para Claude                                                                                                                                                                                                                        |

Um hook que sai com 2 sem um objeto `decision` deixa o fluxo de permissão inalterado, e seu stderr é descartado. Apenas o objeto `decision` pode conceder ou negar a solicitação.

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedInput": {
        "command": "npm run lint"
      }
    }
  }
}
```

<h4 id="permission-update-entries">
  Entradas de atualização de permissão
</h4>

O campo de saída `updatedPermissions` e o campo de entrada [`permission_suggestions`](#permissionrequest-input) ambos usam o mesmo array de objetos de entrada. Cada entrada tem um `type` que determina seus outros campos, e um `destination` que controla onde a mudança é escrita.

| `type`              | Campos                             | Efeito                                                                                                                                                                                                                    |
| :------------------ | :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `addRules`          | `rules`, `behavior`, `destination` | Adiciona regras de permissão. `rules` é um array de objetos `{toolName, ruleContent?}`. Omita `ruleContent` para corresponder a toda a ferramenta. `behavior` é `"allow"`, `"deny"` ou `"ask"`                            |
| `replaceRules`      | `rules`, `behavior`, `destination` | Substitui todas as regras do `behavior` dado no `destination` pelas `rules` fornecidas                                                                                                                                    |
| `removeRules`       | `rules`, `behavior`, `destination` | Remove regras correspondentes do `behavior` dado                                                                                                                                                                          |
| `setMode`           | `mode`, `destination`              | Altera o modo de permissão. Modos válidos são `default`, `auto`, `acceptEdits`, `dontAsk`, `bypassPermissions`, `plan` e `manual` como um alias para `default`. O alias `manual` requer Claude Code v2.1.200 ou posterior |
| `addDirectories`    | `directories`, `destination`       | Adiciona diretórios de trabalho. `directories` é um array de strings de caminho                                                                                                                                           |
| `removeDirectories` | `directories`, `destination`       | Remove diretórios de trabalho                                                                                                                                                                                             |

<Note>
  `setMode` com `bypassPermissions` só tem efeito se você iniciou a sessão com modo bypass já disponível: `--dangerously-skip-permissions`, `--permission-mode bypassPermissions`, `--allow-dangerously-skip-permissions` ou `permissions.defaultMode: "bypassPermissions"` em [configurações de usuário, `--settings` ou gerenciadas](/docs/pt/settings-reference#permissions-defaultmode). Caso contrário, a atualização é uma não-operação. A atualização também é uma não-operação quando [`permissions.disableBypassPermissionsMode`](/docs/pt/permissions#managed-settings) desabilita o modo, ou quando a sessão começa em [modo restrito](/docs/pt/cli-reference#cli-flags).

  `bypassPermissions` nunca é persistido como `defaultMode` independentemente de `destination`.
</Note>

O campo `destination` em cada entrada determina se a mudança permanece na memória ou persiste em um arquivo de configurações.

| `destination`     | Escreve para                                          |
| :---------------- | :---------------------------------------------------- |
| `session`         | apenas na memória, descartado quando a sessão termina |
| `localSettings`   | `.claude/settings.local.json`                         |
| `projectSettings` | `.claude/settings.json`                               |
| `userSettings`    | `~/.claude/settings.json`                             |

Um hook pode ecoar uma das `permission_suggestions` que recebeu como sua própria saída `updatedPermissions`.

<h3 id="posttooluse">
  PostToolUse
</h3>

Executado imediatamente após uma ferramenta ser concluída com sucesso.

Corresponde ao nome da ferramenta, mesmos valores que PreToolUse.

Corresponda mais amplamente quando o nome da ferramenta não é o filtro certo:

* Para executar um hook após qualquer ferramenta ser concluída com sucesso, omita o `matcher` ou defina-o como `"*"`. Seu hook pode então descobrir o que mudou por si mesmo, por exemplo executando `git status --porcelain`, que também lista arquivos não rastreados que `git diff` perde. Para chamadas de ferramenta que falham, adicione o mesmo hook em [PostToolUseFailure](#posttoolusefailure).
* Para executar um hook quando um arquivo específico muda no disco, seja qual for o que o escreveu, use [FileChanged](#filechanged). Claude Code não executa um hook `PostToolUse` correspondente a `Edit|Write` quando um comando `Bash` ou um processo fora de Claude Code reescreve o mesmo arquivo.

<h4 id="posttooluse-input">
  Entrada PostToolUse
</h4>

Os hooks `PostToolUse` são disparados após uma ferramenta já ter sido executada com sucesso. A entrada inclui tanto `tool_input`, os argumentos enviados para a ferramenta, quanto `tool_response`, o resultado que ela retornou. O esquema exato para ambos depende da ferramenta. Os caminhos `tool_input` de ferramenta de arquivo chegam no mesmo formato que para [PreToolUse](#pretooluse-input): sempre absoluto, com os separadores nativos da plataforma, portanto barras invertidas no Windows. Para uma ferramenta MCP, a entrada também carrega o objeto [`mcp_server`](#pretooluse-input).

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "/path/to/file.txt",
    "content": "file content"
  },
  "tool_response": {
    "filePath": "/path/to/file.txt",
    "type": "create"
  },
  "tool_use_id": "toolu_01ABC123...",
  "duration_ms": 12
}
```

| Campo         | Descrição                                                                                                                 |
| :------------ | :------------------------------------------------------------------------------------------------------------------------ |
| `duration_ms` | Opcional. Tempo de execução da ferramenta em milissegundos. Exclui tempo gasto em prompts de permissão e hooks PreToolUse |

<h4 id="posttooluse-decision-control">
  Controle de decisão PostToolUse
</h4>

Os hooks `PostToolUse` podem fornecer feedback ao Claude após a execução da ferramenta. Além dos [campos de saída JSON](#json-output) disponíveis para todos os hooks, seu script de hook pode retornar esses campos específicos do evento:

| Campo                  | Descrição                                                                                                                                                                                                                                                                                                                        |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`             | `"block"` adiciona o `reason` ao lado do resultado da ferramenta. Claude ainda vê a saída original; para substituí-la, use `updatedToolOutput`                                                                                                                                                                                   |
| `reason`               | Explicação mostrada ao Claude quando `decision` é `"block"`                                                                                                                                                                                                                                                                      |
| `additionalContext`    | String adicionada ao contexto do Claude ao lado do resultado da ferramenta. Veja [Adicionar contexto para Claude](#add-context-for-claude)                                                                                                                                                                                       |
| `classifierContext`    | Nota breve sobre o resultado desta chamada para o classificador de [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) em vez de para Claude. Veja [Anotar um resultado para o classificador de modo automático](#annotate-a-result-for-the-auto-mode-classifier). Requer Claude Code v2.1.236 ou posterior |
| `updatedToolOutput`    | Substitui a saída da ferramenta pelo valor fornecido antes de ser enviado ao Claude. O valor deve corresponder à forma de saída da ferramenta                                                                                                                                                                                    |
| `updatedMCPToolOutput` | Substitui a saída para [ferramentas MCP](#match-mcp-tools) apenas. Prefira `updatedToolOutput`, que funciona para todas as ferramentas                                                                                                                                                                                           |

O exemplo abaixo substitui a saída de uma chamada `Bash`. O valor de substituição corresponde à forma de saída da ferramenta `Bash`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "Additional information for Claude",
    "updatedToolOutput": {
      "stdout": "[redacted]",
      "stderr": "",
      "interrupted": false,
      "isImage": false
    }
  }
}
```

<Warning>
  `updatedToolOutput` apenas altera o que Claude vê. A ferramenta já foi executada no momento em que o hook é disparado, portanto qualquer arquivo escrito, comando executado ou solicitação de rede enviada já teve efeito. Telemetria como spans de ferramenta OpenTelemetry e eventos de análise também capturam a saída original antes do hook ser executado. Para impedir ou modificar uma chamada de ferramenta antes de ser executada, use um hook [PreToolUse](#pretooluse) em vez disso.

  O valor de substituição deve corresponder à forma de saída da ferramenta. Ferramentas integradas retornam objetos estruturados em vez de strings simples. Por exemplo, `Bash` retorna um objeto com campos `stdout`, `stderr`, `interrupted` e `isImage`. Para ferramentas integradas, um valor que não corresponde ao esquema de saída da ferramenta é ignorado e a saída original é usada. A saída de ferramenta MCP é passada sem validação de esquema. Remover detalhes de erro que Claude precisa pode fazer com que ele prossiga em uma suposição falsa.
</Warning>

<h4 id="annotate-a-result-for-the-auto-mode-classifier">
  Anotar um resultado para o classificador de modo automático
</h4>

Retorne `classifierContext` para enviar uma nota breve sobre o resultado da chamada de ferramenta para o classificador de [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) em vez de para Claude. O classificador [nunca recebe resultados de ferramentas em si](/docs/pt/permission-modes#how-the-classifier-evaluates-actions), portanto este campo é a forma suportada de contar algo sobre o que uma chamada retornou antes de revisar ações posteriores. O campo requer Claude Code v2.1.236 ou posterior.

O exemplo abaixo diz ao classificador de onde a saída de uma consulta veio:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "classifierContext": "This query ran against the staging database, not production."
  }
}
```

Quanto peso o classificador dá à nota depende de onde você configurou o hook:

* **Hooks configurados em Claude Code**: para hooks de arquivos de configurações, plugins, skills e frontmatter de agente, o classificador trata a nota como contexto não verificado fornecido pela aplicação. A nota nunca estabelece intenção do usuário, e se ela afirmar que você aprovou ou solicitou algo, o classificador verifica essa afirmação contra suas próprias mensagens na conversa
* **Callbacks do Agent SDK em processo**: quando um aplicativo que incorpora Claude Code registra o hook como um [callback do SDK TypeScript](/docs/pt/agent-sdk/hooks) e retorna a nota durante a sessão ao vivo, o classificador pode pesar uma declaração do usuário retransmitida na nota como intenção do usuário. Tal declaração pode satisfazer um requisito de consentimento que o classificador aceitaria de uma mensagem que você envia, mas nunca levanta um bloqueio que sua própria mensagem não pudesse levantar também. Após uma sessão retomar, Claude Code trata notas restauradas como contexto não verificado. Quando hooks de ambos os grupos anotam a mesma chamada, o classificador trata a nota combinada como não verificada

Claude Code aplica esses limites ao entregar a nota:

* **Comprimento**: Claude Code limita as notas para uma chamada de ferramenta a 2.000 caracteres e trunca o resto. O limite é compartilhado entre cada hook que responde a essa chamada
* **Apenas respostas síncronas**: Claude Code ignora o campo na resposta de um hook que [é executado em segundo plano](#run-hooks-in-the-background), porque essa resposta chega após Claude Code registrar o resultado da ferramenta
* **Chamadas que o classificador não registra**: a transcrição do classificador omite pesquisas somente leitura como leituras de arquivo e pesquisas. Claude Code descarta uma nota anexada a uma dessas chamadas
* **Interação com reescritas**: quando a nota descreve saída que você está substituindo com `updatedToolOutput`, retorne ambos os campos na mesma resposta do hook. Claude Code descarta a nota se essa reescrita for rejeitada ou outra reescrita de hook a substituir. Claude Code entrega uma nota que você retorna sem uma reescrita mesmo quando outro hook reescreve a saída

<Warning>
  O classificador lê conteúdo que você coloca em `classifierContext` como informação do aplicativo hospedando a sessão, portanto não copie saída de ferramenta não confiável ou texto de terceiros nele. Mantenha a nota para uma breve afirmação sobre esta chamada, como um fato sobre sua origem ou uma declaração do usuário sobre ela; não use o campo para entregar mensagens não relacionadas ou um fluxo de eventos.
</Warning>

<h3 id="posttoolusefailure">
  PostToolUseFailure
</h3>

Executado quando uma ferramenta que começou a executar falha: a ferramenta lançou um erro, ou uma ferramenta MCP retornou um resultado de erro. Use isso para registrar falhas, enviar alertas ou fornecer feedback corretivo ao Claude.

Corresponde ao nome da ferramenta, mesmos valores que PreToolUse.

<Note>
  Este evento não é disparado para chamadas de ferramenta rejeitadas antes da execução: um nome de ferramenta desconhecido, entrada que falha na validação de esquema ou específica da ferramenta, ou uma negação de permissão. Rejeições de validação são retornadas como resultados `tool_use_error` e acontecem antes dos hooks serem executados, portanto não disparam nem `PreToolUse` nem este evento. Negações de permissão disparam `PreToolUse` mas não este evento; veja [PermissionDenied](#permissiondenied).
</Note>

<h4 id="posttoolusefailure-input">
  Entrada PostToolUseFailure
</h4>

Os hooks PostToolUseFailure recebem os mesmos campos `tool_name` e `tool_input` que PostToolUse, junto com informações de erro como campos de nível superior. Para uma ferramenta MCP, eles também recebem o objeto [`mcp_server`](#pretooluse-input). Por exemplo, um comando `npm test` falhado pode entregar:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolUseFailure",
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test",
    "description": "Run test suite"
  },
  "tool_use_id": "toolu_01ABC123...",
  "error": "Exit code 1\nError: Cannot find module 'express'",
  "is_interrupt": false,
  "duration_ms": 4187
}
```

| Campo          | Descrição                                                                                                                                                                                                                                                       |
| :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `error`        | String descrevendo o que deu errado. O formato depende da ferramenta que falhou                                                                                                                                                                                 |
| `is_interrupt` | Booleano opcional. True quando a falha chegou a Claude Code como um aborto em vez de como um erro que a ferramenta relatou. Cancelar uma ferramenta em execução não dispara este hook; o resultado da ferramenta carrega a mensagem de interrupção em vez disso |
| `duration_ms`  | Opcional. Tempo de execução da ferramenta em milissegundos. Exclui tempo gasto em prompts de permissão e hooks PreToolUse                                                                                                                                       |

A string `error` é geralmente o mesmo texto que Claude recebe como resultado da ferramenta falhada. Seu formato varia por ferramenta e falha. Chave seu hook em `tool_name`, `is_interrupt` e a primeira linha `Exit code N`; trate o resto da string como texto de exibição, não um formato estável.

* Para Bash e PowerShell, um comando que foi executado e saiu produz uma primeira linha `Exit code N`, depois qualquer saída que o comando produziu como um bloco com stdout e stderr intercalados
* Um payload também pode carregar uma mensagem de falha nua sem linha de código de saída, quando Claude Code não pôde iniciar o próprio processo de shell
* Claude Code trunca no meio strings longas em torno de um marcador `... [N characters truncated] ...` e pode inserir linhas suas, como `Command timed out after 2m 0s`

<h4 id="posttoolusefailure-decision-control">
  Controle de decisão PostToolUseFailure
</h4>

Os hooks `PostToolUseFailure` podem fornecer contexto ao Claude após uma falha de ferramenta. Além dos [campos de saída JSON](#json-output) disponíveis para todos os hooks, seu script de hook pode retornar esses campos específicos do evento:

| Campo               | Descrição                                                                                                               |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | String adicionada ao contexto do Claude ao lado do erro. Veja [Adicionar contexto para Claude](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUseFailure",
    "additionalContext": "Additional information about the failure for Claude"
  }
}
```

<h3 id="posttoolbatch">
  PostToolBatch
</h3>

Executado uma vez após cada chamada de ferramenta em um lote ter sido resolvida, antes de Claude Code enviar a próxima solicitação para o modelo. `PostToolUse` é disparado uma vez por ferramenta, o que significa que é disparado concorrentemente quando Claude faz chamadas de ferramenta paralelas. `PostToolBatch` é disparado exatamente uma vez com o lote completo, portanto é o lugar certo para injetar contexto que depende do conjunto de ferramentas que foram executadas em vez de em qualquer ferramenta única. Não há matcher para este evento.

<h4 id="posttoolbatch-input">
  Entrada PostToolBatch
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks PostToolBatch recebem `tool_calls`, um array descrevendo cada chamada de ferramenta no lote:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "PostToolBatch",
  "tool_calls": [
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/accounts.py"},
      "tool_use_id": "toolu_01...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    },
    {
      "tool_name": "Read",
      "tool_input": {"file_path": "/.../ledger/transactions.py"},
      "tool_use_id": "toolu_02...",
      "tool_response": "     1\tfrom __future__ import annotations\n     2\t..."
    }
  ]
}
```

`tool_response` contém o mesmo conteúdo que o modelo recebe no bloco `tool_result` correspondente. O valor é uma string serializada ou array de bloco de conteúdo, exatamente como a ferramenta o emitiu. Para `Read`, isso significa texto com prefixo de número de linha em vez de conteúdo de arquivo bruto. As respostas podem ser grandes, portanto analise apenas os campos que você precisa.

<Note>
  A forma `tool_response` difere da de `PostToolUse`. `PostToolUse` passa o objeto `Output` estruturado da ferramenta, como `{filePath: "...", type: "create"}` para `Write`; `PostToolBatch` passa o conteúdo `tool_result` serializado que o modelo vê.
</Note>

<h4 id="posttoolbatch-decision-control">
  Controle de decisão PostToolBatch
</h4>

Os hooks `PostToolBatch` podem injetar contexto para Claude. Além dos [campos de saída JSON](#json-output) disponíveis para todos os hooks, seu script de hook pode retornar esses campos específicos do evento:

| Campo               | Descrição                                                                                                                                                                                                                               |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | String de contexto injetada uma vez antes da próxima chamada do modelo. Veja [Adicionar contexto para Claude](#add-context-for-claude) para detalhes de entrega, o que colocar nela e como sessões retomadas lidam com valores passados |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolBatch",
    "additionalContext": "These files are part of the ledger module. Run pytest before marking the task complete."
  }
}
```

Retornar `decision: "block"` ou `continue: false` para o loop agentic antes da próxima chamada do modelo. A mensagem de bloqueio vem do JSON `reason` ou `stopReason`, ou de stderr ao sair com 2. Você a vê como um aviso na transcrição, e ela permanece na conversa, portanto Claude a vê quando a conversa continua.

<h3 id="permissiondenied">
  PermissionDenied
</h3>

Executado quando [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) nega uma chamada de ferramenta, incluindo quando nega sem um veredicto do classificador porque [uma verificação de segurança separada do modo automático recusou a própria solicitação do classificador](/docs/pt/errors#auto-mode-cannot-determine-the-safety-of-an-action) ou sua resposta não foi analisada. Este hook é disparado apenas em modo automático: não é executado quando você nega manualmente um diálogo de permissão, quando um hook `PreToolUse` bloqueia uma chamada, ou quando uma regra `deny` corresponde. Use-o para registrar negações, ajustar configuração ou dizer ao modelo que pode tentar novamente a chamada de ferramenta.

Corresponde ao nome da ferramenta, mesmos valores que PreToolUse.

<h4 id="permissiondenied-input">
  Entrada PermissionDenied
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks PermissionDenied recebem `tool_name`, `tool_input`, `tool_use_id` e `reason`. Para uma ferramenta MCP, eles também recebem o objeto [`mcp_server`](#pretooluse-input).

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "auto",
  "hook_event_name": "PermissionDenied",
  "tool_name": "Bash",
  "tool_input": {
    "command": "rm -rf /tmp/build",
    "description": "Clean build directory"
  },
  "tool_use_id": "toolu_01ABC123...",
  "reason": "[Irreversible Local Destruction]"
}
```

| Campo    | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `reason` | O motivo da negação. Para um veredicto do classificador, na maioria das sessões ele nomeia a regra correspondente entre colchetes, como `[Data Exfiltration]`; veja [Revisar negações](/docs/pt/auto-mode-config#review-denials) para as outras formas. Para uma [negação sem veredicto](#permissiondenied-decision-control), começa com `Auto mode could not evaluate this action and is blocking it for safety`. Para uma negação porque o modelo do classificador não estava disponível, é o texto fixo `Classifier unavailable` |

<h4 id="permissiondenied-decision-control">
  Controle de decisão PermissionDenied
</h4>

Os hooks PermissionDenied podem dizer ao modelo que pode tentar novamente a chamada de ferramenta negada. Retorne um objeto JSON com `hookSpecificOutput.retry` definido como `true`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionDenied",
    "retry": true
  }
}
```

Quando `retry` é `true`, Claude Code adiciona uma mensagem à conversa dizendo ao modelo que pode tentar novamente a chamada de ferramenta. Claude Code não reverte a negação em si. Se seu hook não retornar JSON, ou retornar `retry: false`, a negação permanece e o modelo recebe a mensagem de rejeição original.

Claude Code ignora `retry: true` quando o classificador produziu [nenhum veredicto sobre a ação](/docs/pt/errors#auto-mode-cannot-determine-the-safety-of-an-action): sua resposta não foi analisada, ou uma verificação de segurança separada do modo automático recusou a própria solicitação do classificador. Para essas negações, Claude Code já diz ao modelo na mensagem de rejeição se deve tentar novamente mais tarde ou prosseguir.

<h3 id="notification">
  Notification
</h3>

Executado quando Claude Code envia notificações. Corresponde ao tipo de notificação. Omita o matcher para executar hooks para todos os tipos de notificação.

Você recebe esses eventos de hook mesmo com notificações de desktop desativadas: a configuração `preferredNotifChannel`, incluindo `notifications_disabled`, altera apenas como você é alertado, não se seu hook é executado.

| Matcher                      | Quando é disparado                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :--------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permission_prompt`          | Claude precisa de sua permissão para usar uma ferramenta ou a [solicitação de rede](/docs/pt/sandboxing#network-isolation) de um comando em sandbox, e o prompt esperou cerca de seis segundos                                                                                                                                                                                                                                                                                                                                   |
| `idle_prompt`                | Claude terminou de responder cerca de 60 segundos atrás e você não digitou desde então                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `auth_success`               | A autenticação é concluída                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `elicitation_dialog`         | Um servidor MCP abre um formulário de elicitação e você não digitou por cerca de seis segundos                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `elicitation_url_dialog`     | Um servidor MCP pede que você abra uma URL do navegador e você não digitou por cerca de seis segundos                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `elicitation_complete`       | Um servidor MCP relata que uma [elicitação de modo URL](#elicitation-input) está completa                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `elicitation_response`       | Uma resposta de elicitação MCP é enviada de volta para o servidor                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `agent_needs_input`          | Uma sessão em segundo plano começa a esperar sua entrada enquanto [agent view](/docs/pt/agent-view) está aberta em um terminal, ou a sessão atual pede uma pergunta de configuração de terminal de um [colega de equipe de agente](/docs/pt/agent-teams#choose-a-display-mode) e você não digitou por cerca de seis segundos                                                                                                                                                                                                          |
| `agent_completed`            | Uma sessão em segundo plano termina ou falha. Disparado apenas enquanto [agent view](/docs/pt/agent-view) está aberta em um terminal                                                                                                                                                                                                                                                                                                                                                                                             |
| `quota_auto_resume_fired`    | Claude Code continua sua tarefa após um limite de uso de claude.ai pausá-la: na redefinição, ou mais cedo quando algo que você faz em Claude Code durante a espera, como adicionar créditos de uso, atualizar seu plano ou trocar modelos, torna o uso disponível novamente, com a [exceção de configuração de modelo](/docs/pt/interactive-mode#wait-for-a-usage-limit-to-reset)                                                                                                                                                |
| `quota_auto_resume_stale`    | Um limite de uso de claude.ai foi redefinido enquanto seu computador dormia por mais de cerca de 30 minutos. Claude Code espera você pressionar `Enter` em vez de continuar. Após um sono mais curto, ele continua e dispara `quota_auto_resume_fired` em vez disso                                                                                                                                                                                                                                                         |
| `quota_auto_resume_disabled` | Claude Code termina sua espera por um limite de uso de claude.ai sem continuar sua tarefa: [`autoContinueAtUsageLimit`](/docs/pt/settings-reference#autocontinueatusagelimit) foi desativado ou a redefinição se moveu mais de 24 horas no futuro durante uma espera que Claude Code iniciou por conta própria, a tarefa continuada continuou atingindo o limite, ou a continuação foi bloqueada antes de chegar ao modelo. Não é disparado quando você pressiona `Esc` ou `Ctrl+C`, ou escolhe **Don't continue automatically** |

Os tipos `agent_needs_input` e `agent_completed` requerem Claude Code v2.1.198 ou posterior.

Os tipos `quota_auto_resume_fired`, `quota_auto_resume_stale` e `quota_auto_resume_disabled` requerem Claude Code v2.1.234 ou posterior.

Em sessões de terminal, `permission_prompt` para a solicitação de rede de um comando em sandbox requer Claude Code v2.1.246 ou posterior.

`agent_needs_input` para uma pergunta de configuração de terminal de colega requer Claude Code v2.1.248 ou posterior.

<Note>
  Os tipos `permission_prompt`, `idle_prompt`, `elicitation_dialog` e `elicitation_url_dialog` compartilham seu tempo com notificações de desktop, portanto em sessões de terminal você só os vê quando parece que você está longe do terminal:

  * Espere `permission_prompt` uma vez que você não digitou por cerca de seis segundos. O temporizador começa quando o prompt de permissão aparece, e cada pressionamento de tecla o adia. Para executar um hook imediatamente quando Claude pede permissão para usar uma ferramenta, use [PermissionRequest](#permissionrequest) em vez disso.
  * Espere `idle_prompt` cerca de 60 segundos após Claude terminar de responder, e apenas se você não digitou desde então. Claude Code não envia `idle_prompt` enquanto espera um limite de uso de claude.ai ser redefinido. Quando a espera termina por conta própria, um dos tipos `quota_auto_resume_*` é disparado em vez disso.
  * Espere `elicitation_dialog` para um formulário de elicitação, ou `elicitation_url_dialog` para uma solicitação de URL do navegador, uma vez que você não digitou por cerca de seis segundos. Ambos compartilham o mesmo portão de seis segundos que `permission_prompt`: o temporizador começa quando o diálogo aparece, e cada pressionamento de tecla o adia.

  Uma solicitação de permissão ou elicitação que chega enquanto outro diálogo está na tela mantém o mesmo portão de seis segundos, cronometrado a partir de quando a solicitação chega. Sua notificação pode alcançá-lo enquanto a solicitação ainda espera atrás do diálogo aberto.
</Note>

Claude Code cronometra `permission_prompt` diferentemente em sessões onde envia solicitações de permissão para o callback [`canUseTool`](/docs/pt/agent-sdk/user-input) do Agent SDK, que é como Claude Desktop e a extensão VS Code hospedam Claude Code:

* Espere `permission_prompt` cerca de seis segundos após Claude pedir permissão. Claude Code não o adia enquanto você digita.
* Se você ou um hook [PermissionRequest](#permissionrequest) responder mais cedo, Claude Code não executa `permission_prompt`.
* Defina [`CLAUDE_CODE_DISABLE_PERMISSION_PROMPT_NOTIFY_HOOKS`](/docs/pt/env-vars) como `1` para desativar `permission_prompt` nessas sessões.

Antes da v2.1.233, `permission_prompt` não era disparado nessas sessões.

Use matchers separados para executar diferentes manipuladores dependendo do tipo de notificação. Esta configuração dispara um script de alerta específico de permissão quando Claude precisa de aprovação de permissão e uma notificação diferente quando Claude está inativo:

```json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "matcher": "permission_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/permission-alert.sh"
          }
        ]
      },
      {
        "matcher": "idle_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/idle-notification.sh"
          }
        ]
      }
    ]
  }
}
```

<h4 id="notification-input">
  Entrada Notification
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks Notification recebem `message` com o texto de notificação, um `title` opcional e `notification_type` indicando qual tipo foi disparado.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Notification",
  "message": "Claude needs your permission",
  "title": "Permission needed",
  "notification_type": "permission_prompt"
}
```

Os hooks Notification não podem bloquear ou modificar notificações. Claude Code descarta seus campos `systemMessage` e `continue` mas ainda emite [`terminalSequence`](#emit-terminal-notifications), que é no que o exemplo de notificação de desktop se baseia. Os hooks Notification são destinados a efeitos colaterais como encaminhar a notificação para um serviço externo.

<h3 id="subagentstart">
  SubagentStart
</h3>

Executado quando Claude gera um subagente com a ferramenta Agent, quando Claude [retoma um subagente](/docs/pt/sub-agents#resume-subagents) e cada vez que um [colega de equipe de agente](/docs/pt/agent-teams) em processo manipula uma nova mensagem. Suporta matchers para filtrar por nome de tipo de agente. Para agentes integrados, este é o nome do agente como `general-purpose`, `Explore` ou `Plan`. Para [subagentes personalizados](/docs/pt/sub-agents), este é o campo `name` do frontmatter do agente, não o nome do arquivo.

Para subagentes enviados por um [plugin](/docs/pt/plugins/overview), o tipo de agente é o identificador com escopo de plugin como `my-plugin:reviewer`, não o nome de frontmatter nú. O dois-pontos coloca um nome com escopo de plugin no caminho de expressão regular, portanto ancor o matcher com `^` e `$` para uma correspondência exata: `^my-plugin:reviewer$`.

<h4 id="subagentstart-input">
  Entrada SubagentStart
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks SubagentStart recebem `agent_id` com o identificador único para o subagente e `agent_type` com o nome do agente que o matcher filtra.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SubagentStart",
  "agent_id": "agent-abc123",
  "agent_type": "Explore"
}
```

Os hooks SubagentStart não podem bloquear a criação de subagente, mas podem injetar contexto no subagente. Além dos [campos de saída JSON](#json-output) disponíveis para todos os hooks, você pode retornar:

| Campo               | Descrição                                                                                                                                                          |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | String adicionada ao contexto do subagente no início de sua conversa, antes de seu primeiro prompt. Veja [Adicionar contexto para Claude](#add-context-for-claude) |

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "SubagentStart",
    "additionalContext": "Follow security guidelines for this task"
  }
}
```

Quando o hook é executado novamente para o mesmo subagente, Claude Code injeta o contexto retornado apenas quando o contexto do subagente não já contém a cópia de uma execução anterior. A cópia injetada no lançamento permanece no lugar, deixando o [cache de prompt](/docs/pt/prompt-caching#subagents-and-the-cache) do subagente intacto. Após [compactação automática](/docs/pt/sub-agents#auto-compaction) descartar essa cópia, Claude Code injeta o contexto da próxima execução novamente.

<h3 id="subagentstop">
  SubagentStop
</h3>

Executado quando um subagente Claude Code terminou de responder. Corresponde ao tipo de agente, mesmos valores que SubagentStart.

<h4 id="subagentstop-input">
  Entrada SubagentStop
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks SubagentStop recebem `stop_hook_active`, `agent_id`, `agent_type`, `agent_transcript_path` e `last_assistant_message`. O campo `agent_type` é o valor usado para filtragem de matcher. O `transcript_path` é a transcrição da sessão principal, enquanto `agent_transcript_path` é a própria transcrição do subagente armazenada em uma pasta `subagents/` aninhada. O campo `last_assistant_message` contém o conteúdo de texto da resposta final do subagente, portanto hooks podem acessá-lo sem analisar o arquivo de transcrição.

Não há eventos de hook de subagente Claude Code que não sejam de subagente. Claude Code também executa agentes internos para alguns de seus próprios recursos, como [sugestões de prompt](/docs/pt/interactive-mode#prompt-suggestions) e [perguntas laterais `/btw`](/docs/pt/interactive-mode#side-questions-with-%2Fbtw), e SubagentStop é disparado quando um desses termina também. Para esses eventos, `agent_type` é o nome do agente que a sessão em si executa, como um definido com [`--agent`](/docs/pt/cli-reference#cli-flags) ou a configuração [`agent`](/docs/pt/settings-reference#agent), e uma string vazia quando a sessão é executada sem um.

Um `matcher` que nomeia tipos de agente não corresponde a um `agent_type` vazio. Um hook cujo matcher é omitido, `""`, ou `"*"`, ou é uma expressão regular que corresponde a uma string vazia, é executado para eventos com um `agent_type` vazio também.

No Claude Code v2.1.271 ou posterior, um subagente que é executado com a ferramenta [`SubagentHandback`](/docs/pt/tools-reference) entrega seu relatório através dessa ferramenta antes de parar. O campo `last_assistant_message` então contém o texto de fechamento do subagente, se houver, que não é o relatório entregue. O relatório é a entrada `message` dessa chamada, que um hook `PreToolUse` ou `PostToolUse` correspondente a `SubagentHandback` recebe como `tool_input.message`.

Os hooks SubagentStop também recebem os arrays `background_tasks` e `session_crons` descritos em [entrada Stop](#stop-input). Ambos os arrays estão no escopo da sessão pai, não do subagente.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../abc123.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "SubagentStop",
  "stop_hook_active": false,
  "agent_id": "def456",
  "agent_type": "Explore",
  "agent_transcript_path": "~/.claude/projects/.../abc123/subagents/agent-def456.jsonl",
  "last_assistant_message": "Analysis complete. Found 3 potential issues...",
  "background_tasks": [],
  "session_crons": []
}
```

Os hooks SubagentStop usam o mesmo formato de controle de decisão que [hooks Stop](#stop-decision-control), incluindo `hookSpecificOutput.additionalContext` com `hookEventName` definido como `"SubagentStop"`, para feedback sem erro que mantém o subagente em execução. Retornar `decision: "block"` com um `reason` mantém o subagente em execução e entrega `reason` ao subagente como sua próxima instrução. Um hook que bloqueia ao sair com 2 entrega sua mensagem stderr da mesma forma. Para injetar contexto na sessão pai após um subagente retornar, use um hook [`PostToolUse`](#posttooluse) na ferramenta `Agent` em vez disso.

<h3 id="taskcreated">
  TaskCreated
</h3>

Executado quando uma tarefa está sendo criada via ferramenta `TaskCreate`. Use isso para impor convenções de nomenclatura, exigir descrições de tarefa ou impedir que certas tarefas sejam criadas. Em uma [sessão sem as ferramentas Task](/docs/pt/tools-reference#task-tool-availability), este evento não é disparado.

Os hooks TaskCreated não suportam matchers e são disparados em cada ocorrência.

<h4 id="taskcreated-input">
  Entrada TaskCreated
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks TaskCreated recebem `task_id`, `task_subject` e opcionalmente `task_description`, `teammate_name` e `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "TaskCreated",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Campo              | Descrição                                                                            |
| :----------------- | :----------------------------------------------------------------------------------- |
| `task_id`          | Identificador da tarefa sendo criada                                                 |
| `task_subject`     | Título da tarefa                                                                     |
| `task_description` | Descrição detalhada da tarefa. Pode estar ausente                                    |
| `teammate_name`    | Nome do colega criando a tarefa. Pode estar ausente                                  |
| `team_name`        | Descontinuado. Nome de equipe derivado de sessão; será removido em uma versão futura |

<h4 id="taskcreated-decision-control">
  Controle de decisão TaskCreated
</h4>

Um hook TaskCreated pode bloquear a criação de duas maneiras. De qualquer forma, Claude Code exclui a tarefa e retorna sua mensagem ao Claude como o erro da ferramenta. Claude Code ignora `continue: false` deste evento e Claude continua trabalhando.

* **Código de saída 2**: Claude Code retorna o texto stderr como a mensagem.
* **JSON `{"decision": "block", "reason": "..."}`**: Claude Code retorna `reason` como a mensagem.

Este exemplo bloqueia tarefas cujos assuntos não seguem o formato necessário:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

if [[ ! "$TASK_SUBJECT" =~ ^\[TICKET-[0-9]+\] ]]; then
  echo "Task subject must start with a ticket number, e.g. '[TICKET-123] Add feature'" >&2
  exit 2
fi

exit 0
```

<h3 id="taskcompleted">
  TaskCompleted
</h3>

Executado quando uma tarefa está sendo marcada como concluída. Isso é disparado em duas situações: quando qualquer agente marca explicitamente uma tarefa como concluída através da ferramenta TaskUpdate, ou quando um [colega de equipe de agente](/docs/pt/agent-teams) termina seu turno com tarefas em andamento. Use isso para impor critérios de conclusão como testes aprovados ou verificações de lint antes de uma tarefa poder fechar.

Os hooks TaskCompleted não suportam matchers e são disparados em cada ocorrência.

<h4 id="taskcompleted-input">
  Entrada TaskCompleted
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks TaskCompleted recebem `task_id`, `task_subject` e opcionalmente `task_description`, `teammate_name` e `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TaskCompleted",
  "task_id": "task-001",
  "task_subject": "Implement user authentication",
  "task_description": "Add login and signup endpoints",
  "teammate_name": "implementer",
  "team_name": "session-a1b2c3d4"
}
```

| Campo              | Descrição                                                                            |
| :----------------- | :----------------------------------------------------------------------------------- |
| `task_id`          | Identificador da tarefa sendo concluída                                              |
| `task_subject`     | Título da tarefa                                                                     |
| `task_description` | Descrição detalhada da tarefa. Pode estar ausente                                    |
| `teammate_name`    | Nome do colega concluindo a tarefa. Pode estar ausente                               |
| `team_name`        | Descontinuado. Nome de equipe derivado de sessão; será removido em uma versão futura |

<h4 id="taskcompleted-decision-control">
  Controle de decisão TaskCompleted
</h4>

Os hooks TaskCompleted suportam duas maneiras de controlar a conclusão da tarefa:

* **Código de saída 2**: a tarefa não é marcada como concluída e a mensagem stderr é retornada ao modelo como feedback.
* **JSON `{"continue": false, "stopReason": "..."}`**: quando um colega terminando seu turno disparou o evento, para o colega inteiramente, correspondendo ao comportamento do hook `Stop`. O `stopReason` é mostrado ao usuário. Quando a ferramenta `TaskUpdate` disparou o evento, Claude Code ignora `continue: false`; o código de saída 2 ainda bloqueia a conclusão.

Este exemplo executa testes e bloqueia a conclusão da tarefa se falharem:

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
TASK_SUBJECT=$(echo "$INPUT" | jq -r '.task_subject')

# Run the test suite
if ! npm test 2>&1; then
  echo "Tests not passing. Fix failing tests before completing: $TASK_SUBJECT" >&2
  exit 2
fi

exit 0
```

<h3 id="stop">
  Stop
</h3>

Executado quando o agente Claude Code principal terminou de responder. Não é executado se a parada ocorreu devido a uma interrupção do usuário. Erros de API disparam [StopFailure](#stopfailure) em vez disso.

<Tip>
  O comando [`/goal`](/docs/pt/goal) é um atalho integrado para um hook Stop baseado em prompt com escopo de sessão. Use-o quando você quer que Claude continue trabalhando em direção a uma condição sem escrever configuração de hook.
</Tip>

<h4 id="stop-input">
  Entrada Stop
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks Stop recebem `stop_hook_active`, `last_assistant_message`, `background_tasks` e `session_crons`. O campo `stop_hook_active` é `true` quando Claude Code já está continuando como resultado de um hook stop. Verifique este valor ou processe a transcrição para evitar bloquear em uma condição que nunca será resolvida. Claude Code sobrescreve o hook e termina o turno após 8 bloqueios consecutivos.

O campo `last_assistant_message` contém o conteúdo de texto da resposta final do Claude, portanto hooks podem acessá-lo sem analisar o arquivo de transcrição. Para hooks que atuam no turno recém-concluído, como hooks de leitura em voz alta ou notificação, use este campo em vez de ler `transcript_path`: o arquivo de transcrição não é garantido incluir a mensagem final no tempo de Stop em todas as versões.

Os arrays `background_tasks` e `session_crons` permitem que hooks distingam "sessão está feita" de "sessão está pausada esperando que trabalho de fundo a acorde novamente". Ambos os arrays estão presentes quando o registro de tarefas é alcançável e estão vazios quando nada está em voo ou agendado.

Cada entrada em `background_tasks` descreve uma tarefa em voo e usa estes campos:

| Campo         | Descrição                                                                                                                                                                                                                                                  |
| :------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`          | Identificador de tarefa                                                                                                                                                                                                                                    |
| `type`        | Rótulo de tipo de tarefa amigável como `shell`, `subagent`, `monitor`, `workflow`, `teammate`, `cloud session` ou `MCP task`. Cada rótulo identifica qual recurso Claude Code criou a tarefa. Volta para o discriminante bruto para tipos não reconhecidos |
| `status`      | Status atual da tarefa                                                                                                                                                                                                                                     |
| `description` | Descrição de texto livre, limitada a 1000 caracteres com um marcador `… [+N chars]` em string quando cortado                                                                                                                                               |
| `command`     | Linha de comando de shell, limitada a 1000 caracteres. Presente apenas para tarefas `shell`                                                                                                                                                                |
| `agent_type`  | Nome de tipo de subagente. Presente apenas para tarefas `subagent`                                                                                                                                                                                         |
| `server`      | Nome do servidor MCP. Presente apenas para tarefas `monitor` e `MCP task`                                                                                                                                                                                  |
| `tool`        | Nome da ferramenta MCP. Presente apenas para tarefas `monitor` e `MCP task`                                                                                                                                                                                |
| `name`        | Nome do workflow. Presente apenas para tarefas `workflow`                                                                                                                                                                                                  |

Cada entrada em `session_crons` descreve um despertar agendado com escopo de sessão, originário de `CronCreate`, `ScheduleWakeup` e `/loop`:

| Campo       | Descrição                                                                                                                                               |
| :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `id`        | Identificador de tarefa Cron                                                                                                                            |
| `schedule`  | Expressão Cron, por exemplo `0 9 * * 1-5`                                                                                                               |
| `recurring` | `false` para despertares únicos cuja programação codifica um tempo de disparo único, `true` para tarefas que disparam novamente em cada correspondência |
| `prompt`    | Prompt enviado quando o cron dispara, limitado a 1000 caracteres com o mesmo marcador `… [+N chars]`                                                    |

Este exemplo mostra uma entrada Stop com uma tarefa de shell em voo e um cron recorrente:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "Stop",
  "stop_hook_active": true,
  "last_assistant_message": "I've completed the refactoring. Here's a summary...",
  "background_tasks": [
    {
      "id": "task-001",
      "type": "shell",
      "status": "running",
      "description": "tail logs",
      "command": "tail -f /var/log/syslog"
    }
  ],
  "session_crons": [
    {
      "id": "cron-001",
      "schedule": "0 9 * * 1-5",
      "recurring": true,
      "prompt": "check the build"
    }
  ]
}
```

<h4 id="stop-decision-control">
  Controle de decisão Stop
</h4>

Os hooks `Stop` e `SubagentStop` podem controlar se Claude continua. Além dos [campos de saída JSON](#json-output) disponíveis para todos os hooks, seu script de hook pode retornar esses campos específicos do evento:

| Campo                                  | Descrição                                                                                                                                                                                                   |
| :------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `decision`                             | `"block"` impede que Claude pare. Omita para permitir que Claude pare                                                                                                                                       |
| `reason`                               | Necessário quando `decision` é `"block"`. Diz ao Claude por que deve continuar                                                                                                                              |
| `hookSpecificOutput.additionalContext` | Feedback sem erro para Claude. A conversa continua para que Claude possa agir sobre isso, mas ao contrário de `decision: "block"` é mostrado na transcrição como feedback de hook em vez de um erro de hook |

Um hook que bloqueia ao sair com 2 roteia da mesma forma que `reason`: Claude recebe a mensagem stderr como a explicação de por que deve continuar.

```json theme={null}
{
  "decision": "block",
  "reason": "Must be provided when Claude is blocked from stopping"
}
```

Use `additionalContext` quando o hook está funcionando conforme projetado e dando orientação ao Claude, como "execute a suite de testes antes de terminar". Mantém a conversa passando através das mesmas proteções de loop que `decision: "block"`, a saber a entrada `stop_hook_active` e o limite de 8 continuações consecutivas, mas a transcrição a rotula como `Stop hook feedback` e nenhuma notificação de erro de hook é mostrada:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Stop",
    "additionalContext": "Please run the test suite before finishing"
  }
}
```

<h3 id="stopfailure">
  StopFailure
</h3>

Executado em vez de [Stop](#stop) quando o turno termina devido a um erro de API. Claude Code ignora a saída e código de saída do hook, além de [`terminalSequence`](#emit-terminal-notifications). Use isso para registrar falhas, enviar alertas ou tomar ações de recuperação quando Claude não pode completar uma resposta devido a limites de taxa, problemas de autenticação ou outros erros de API.

<h4 id="stopfailure-input">
  Entrada StopFailure
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks StopFailure recebem `error`, `error_details` opcional e `last_assistant_message` opcional. O campo `error` identifica o tipo de erro e é usado para filtragem de matcher.

| Campo                    | Descrição                                                                                                                                                                                                                                               |
| :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `error`                  | Tipo de erro: `rate_limit`, `overloaded`, `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error` ou `unknown`        |
| `error_details`          | Detalhes adicionais sobre o erro, quando disponível                                                                                                                                                                                                     |
| `last_assistant_message` | O texto de erro renderizado mostrado na conversa. Ao contrário de `Stop` e `SubagentStop`, onde este campo contém a saída conversacional do Claude, para `StopFailure` ele contém a string de erro da API em si, como `"API Error: Rate limit reached"` |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "StopFailure",
  "error": "rate_limit",
  "error_details": "429 Too Many Requests",
  "last_assistant_message": "API Error: Rate limit reached"
}
```

Os hooks StopFailure não têm controle de decisão. Eles são executados apenas para fins de notificação e registro.

<h3 id="teammateidle">
  TeammateIdle
</h3>

Executado quando um [colega de equipe de agente](/docs/pt/agent-teams) está prestes a ficar inativo após terminar seu turno. Use isso para impor portões de qualidade antes de um colega parar de trabalhar, como exigir verificações de lint aprovadas ou verificar que arquivos de saída existem.

Os hooks TeammateIdle não suportam matchers e são disparados em cada ocorrência.

<h4 id="teammateidle-input">
  Entrada TeammateIdle
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks TeammateIdle recebem `teammate_name` e `team_name`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "permission_mode": "default",
  "hook_event_name": "TeammateIdle",
  "teammate_name": "researcher",
  "team_name": "session-a1b2c3d4"
}
```

| Campo           | Descrição                                                                            |
| :-------------- | :----------------------------------------------------------------------------------- |
| `teammate_name` | Nome do colega que está prestes a ficar inativo                                      |
| `team_name`     | Descontinuado. Nome de equipe derivado de sessão; será removido em uma versão futura |

<h4 id="teammateidle-decision-control">
  Controle de decisão TeammateIdle
</h4>

Os hooks TeammateIdle suportam duas maneiras de controlar o comportamento do colega:

* **Código de saída 2**: o colega recebe a mensagem stderr como feedback e continua trabalhando em vez de ficar inativo.
* **JSON `{"continue": false, "stopReason": "..."}`**: para o colega inteiramente, correspondendo ao comportamento do hook `Stop`. O `stopReason` é mostrado ao usuário.

Este exemplo verifica se um artefato de compilação existe antes de permitir que um colega fique inativo:

```bash theme={null}
#!/bin/bash

if [ ! -f "./dist/output.js" ]; then
  echo "Build artifact missing. Run the build before stopping." >&2
  exit 2
fi

exit 0
```

<h3 id="configchange">
  ConfigChange
</h3>

Executado quando um arquivo de configuração muda durante uma sessão. Use isso para auditar mudanças de configurações, impor políticas de segurança ou bloquear modificações não autorizadas em arquivos de configuração.

Claude Code executa hooks ConfigChange quando um arquivo de configurações, um arquivo de política gerenciada ou um arquivo de skill muda. Para política gerenciada, ele os executa apenas quando `managed-settings.json` ou um arquivo em `managed-settings.d/` muda. Ele aplica [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) e mudanças em preferências gerenciadas macOS ou política de registro Windows sem executá-los. Em WSL com [`wslInheritsWindowsSettings`](/docs/pt/settings-reference#wslinheritswindowssettings), ele também aplica um arquivo de configurações gerenciadas do lado Windows alterado em sua pesquisa de política sem executá-los.

O matcher filtra na fonte de configuração:

| Matcher            | Quando é disparado                                                  |
| :----------------- | :------------------------------------------------------------------ |
| `user_settings`    | `~/.claude/settings.json` muda                                      |
| `project_settings` | `.claude/settings.json` muda                                        |
| `local_settings`   | `.claude/settings.local.json` muda                                  |
| `policy_settings`  | `managed-settings.json` ou um arquivo em `managed-settings.d/` muda |
| `skills`           | Um arquivo de skill em `.claude/skills/` muda                       |

Este exemplo registra todas as mudanças de configuração para auditoria de segurança:

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/audit-config-change.sh",
            "args": []
          }
        ]
      }
    ]
  }
}
```

<h4 id="configchange-input">
  Entrada ConfigChange
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks ConfigChange recebem `source` e opcionalmente `file_path`. O campo `source` indica qual tipo de configuração mudou, e `file_path` fornece o caminho para o arquivo específico que foi modificado.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "ConfigChange",
  "source": "project_settings",
  "file_path": "/Users/.../my-project/.claude/settings.json"
}
```

<h4 id="configchange-decision-control">
  Controle de decisão ConfigChange
</h4>

Os hooks ConfigChange podem bloquear mudanças de configuração de serem aplicadas. Use código de saída 2 ou um JSON `decision` para impedir a mudança. Quando bloqueado, as novas configurações não são aplicadas à sessão em execução.

| Campo      | Descrição                                                                                   |
| :--------- | :------------------------------------------------------------------------------------------ |
| `decision` | `"block"` impede que a mudança de configuração seja aplicada. Omita para permitir a mudança |
| `reason`   | Aceito mas nunca mostrado                                                                   |

```json theme={null}
{
  "decision": "block",
  "reason": "Configuration changes to project settings require admin approval"
}
```

As mudanças `policy_settings` não podem ser bloqueadas. Os hooks ainda são disparados para fontes `policy_settings` quando um arquivo de configurações gerenciadas na máquina muda, portanto você pode usá-los para registrar essas edições, mas qualquer decisão de bloqueio é ignorada. Isso garante que as configurações gerenciadas pela empresa sempre tenham efeito. Claude Code não executa hooks `ConfigChange` quando [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) chegam ou são atualizadas.

Claude Code atua na decisão de bloqueio da saída JSON de um hook ConfigChange e descarta `systemMessage` e `continue`. Uma mudança bloqueada não exibe nenhuma mensagem para você ou para Claude, independentemente de você bloquear com `reason` ou com stderr ao sair com 2. Claude Code apenas escreve uma linha no log de depuração.

<h3 id="cwdchanged">
  CwdChanged
</h3>

Executado quando um comando de shell na conversa principal muda o diretório de trabalho, por exemplo quando Claude executa um comando `cd`. Use isso para reagir a mudanças de diretório: recarregar variáveis de ambiente, ativar toolchains específicas do projeto ou executar scripts de configuração automaticamente. Emparelha com [FileChanged](#filechanged) para ferramentas como [direnv](https://direnv.net/) que gerenciam ambiente por diretório.

Os hooks CwdChanged têm acesso a [`CLAUDE_ENV_FILE`](#persist-environment-variables). Variáveis escritas nesse arquivo persistem em comandos Bash subsequentes até o próximo evento CwdChanged, quando Claude Code as limpa.

CwdChanged não suporta matchers e é disparado em cada ocorrência.

<h4 id="cwdchanged-input">
  Entrada CwdChanged
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks CwdChanged recebem `old_cwd` e `new_cwd`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project/src",
  "hook_event_name": "CwdChanged",
  "old_cwd": "/Users/my-project",
  "new_cwd": "/Users/my-project/src"
}
```

<h4 id="cwdchanged-output">
  Saída CwdChanged
</h4>

Além dos [campos de saída JSON](#json-output) disponíveis para todos os hooks, os hooks CwdChanged podem retornar `watchPaths` para definir dinamicamente quais caminhos de arquivo [FileChanged](#filechanged) observa:

| Campo        | Descrição                                                                                                                                                                                                                              |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Array de caminhos absolutos. Substitui a lista de observação dinâmica atual. Caminhos de sua configuração `matcher` são sempre observados. Retornar um array vazio limpa a lista dinâmica, que é típico ao entrar em um novo diretório |

Os hooks CwdChanged não têm controle de decisão. Eles não podem bloquear a mudança de diretório.

Claude Code lê `watchPaths` e `systemMessage` de sua saída JSON e descarta `continue`. Em sessões interativas, mostra o `systemMessage` como uma breve notificação de terminal. A mensagem não chega ao fluxo de mensagens do SDK.

<h3 id="directoryadded">
  DirectoryAdded
</h3>

Executado após você adicionar um diretório de trabalho no meio da sessão com o comando `/add-dir`, ou após um cliente SDK adicionar um com a solicitação de controle `register_repo_root`. Use isso para preparar um repositório recém-adicionado, por exemplo instalando suas dependências.

Claude Code não dispara este evento quando:

* Você passa um diretório com a flag de startup `--add-dir`; [SessionStart](#sessionstart) cobre esses diretórios
* Você adiciona um diretório na aba `/permissions` Workspace
* Você adiciona um diretório que já é um diretório de trabalho ou está dentro de um

Claude Code dispara DirectoryAdded após atualizar estado de sandbox e permissão, portanto ferramentas em sandbox já veem o novo diretório quando seu hook é executado. Comandos de hook em si são executados sem sandbox.

Claude Code não espera pelo hook: a adição é concluída imediatamente, e o hook é executado em segundo plano com o tempo limite padrão de 600 segundos.

O matcher filtra em como o diretório foi adicionado:

| Matcher              | Quando é disparado                                                                      |
| :------------------- | :-------------------------------------------------------------------------------------- |
| `slash_command`      | Você adiciona um diretório com `/add-dir`                                               |
| `register_repo_root` | Um cliente SDK adiciona um diretório com a solicitação de controle `register_repo_root` |

<h4 id="directoryadded-input">
  Entrada DirectoryAdded
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks DirectoryAdded recebem `directory` e `source`.

| Campo       | Descrição                                                                                                                          |
| :---------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| `directory` | Caminho absoluto do diretório que foi adicionado                                                                                   |
| `source`    | Como o diretório foi adicionado, `"slash_command"` para `/add-dir` ou `"register_repo_root"` para a solicitação de controle do SDK |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "DirectoryAdded",
  "directory": "/Users/my-other-repo",
  "source": "slash_command"
}
```

Os hooks DirectoryAdded não têm controle de decisão. Eles não podem bloquear a adição, que já foi concluída quando o hook é executado. Claude Code descarta o campo `continue` de sua saída JSON e exibe o resto diferentemente por fonte:

* `slash_command`: Claude Code entrega o `systemMessage` do hook ao Claude como contexto no próximo turno de conversa, em vez de mostrar a você. Uma contagem de hooks falhados aparece na transcrição. A saída de falha completa vai para o log de depuração
* `register_repo_root`: Claude Code escreve saída `systemMessage` e saída de falha apenas no log de depuração

<h3 id="filechanged">
  FileChanged
</h3>

Executado quando um arquivo observado muda no disco. Claude Code detecta mudanças com um observador de sistema de arquivos, não inspecionando chamadas de ferramenta, portanto executa o hook não importa o que mudou o arquivo: uma chamada de ferramenta `Edit` ou `Write`, um script que Claude executa com `Bash` ou um processo fora de Claude Code inteiramente. Um uso comum é recarregar variáveis de ambiente quando arquivos de configuração do projeto mudam.

O `matcher` para este evento serve dois papéis:

* **Construir a lista de observação**: o valor é dividido em `|` e cada segmento é registrado como um nome de arquivo literal no diretório de trabalho, portanto `".envrc|.env"` observa exatamente esses dois arquivos. Padrões regex não são úteis aqui: um valor como `^\.env` observaria um arquivo literalmente nomeado `^\.env`.
* **Filtrar quais hooks são executados**: quando um arquivo observado muda, o mesmo valor filtra quais grupos de hook são executados usando as [regras de matcher](#matcher-patterns) padrão contra o nome base do arquivo alterado.

Este exemplo normaliza terminações de linha em `data.csv` após qualquer mudança, incluindo um comando `Bash` ou um script externo reescrevendo o arquivo:

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": "data.csv",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/normalize-line-endings.sh"
          }
        ]
      }
    ]
  }
}
```

O hook lê o caminho absoluto do arquivo alterado do campo `file_path` da [entrada JSON](#filechanged-input) em stdin. Sua guarda `grep` testa a mesma coisa que `perl` remove, um CR no final de uma linha, portanto a execução após uma normalização sai sem tocar no arquivo. Uma guarda mais solta faz um loop para sempre, porque `perl -i` reescreve o arquivo mesmo quando substitui nada e Claude Code executa o hook novamente após cada reescrita. Salve este script em `/path/to/normalize-line-endings.sh` e torne-o executável:

```bash theme={null}
#!/bin/bash
FILE=$(jq -r .file_path)
if grep -q $'\r$' "$FILE"; then
  perl -pi -e 's/\r$//' "$FILE"
fi
```

Para confirmar que o hook funciona, peça ao Claude para anexar uma linha CRLF a `data.csv` com um comando `Bash`. Claude Code executa o hook e o arquivo termina com terminações LF.

Para observar arquivos que você não pode nomear antecipadamente, retorne [`watchPaths`](#filechanged-output) de um hook para atualizar a lista de observação dinamicamente. Claude Code inicia o observador apenas quando algo nomeia um arquivo para observar, portanto semeie a lista com um grupo FileChanged cujo matcher nomeia pelo menos um arquivo, ou com um hook [SessionStart](#sessionstart-decision-control) ou [CwdChanged](#cwdchanged) que retorna `watchPaths`. O matcher ainda filtra quais grupos de hook são executados quando um arquivo observado muda, portanto dê ao grupo que manipula caminhos dinâmicos um matcher omitido, que corresponde a cada arquivo observado e não adiciona nada à lista de observação. Um matcher `"*"` também corresponde a cada arquivo, mas Claude Code o registra na lista de observação como qualquer outro valor, como um arquivo literal nomeado `*`.

Os hooks FileChanged têm acesso a [`CLAUDE_ENV_FILE`](#persist-environment-variables). Variáveis escritas nesse arquivo persistem em comandos Bash subsequentes até o próximo evento [CwdChanged](#cwdchanged), quando Claude Code as limpa.

<h4 id="filechanged-input">
  Entrada FileChanged
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks FileChanged recebem `file_path` e `event`.

| Campo       | Descrição                                                                                                                     |
| :---------- | :---------------------------------------------------------------------------------------------------------------------------- |
| `file_path` | Caminho absoluto para o arquivo que mudou                                                                                     |
| `event`     | O que aconteceu: `"change"` para um arquivo modificado, `"add"` para um arquivo criado ou `"unlink"` para um arquivo excluído |

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/Users/my-project",
  "hook_event_name": "FileChanged",
  "file_path": "/Users/my-project/.envrc",
  "event": "change"
}
```

<h4 id="filechanged-output">
  Saída FileChanged
</h4>

Além dos [campos de saída JSON](#json-output) disponíveis para todos os hooks, os hooks FileChanged podem retornar `watchPaths` para atualizar dinamicamente quais caminhos de arquivo são observados:

| Campo        | Descrição                                                                                                                                                                                                                                             |
| :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `watchPaths` | Array de caminhos absolutos. Substitui a lista de observação dinâmica atual. Caminhos de sua configuração `matcher` são sempre observados. Use isso quando seu script de hook descobre arquivos adicionais para observar com base no arquivo alterado |

Os hooks FileChanged não têm controle de decisão. Eles não podem bloquear a mudança de arquivo de ocorrer.

Claude Code lê `watchPaths` e `systemMessage` de sua saída JSON e descarta `continue`. Em sessões interativas, mostra o `systemMessage` como uma breve notificação de terminal. A mensagem não chega ao fluxo de mensagens do SDK.

<h3 id="worktreecreate">
  WorktreeCreate
</h3>

Executado quando uma worktree está sendo criada, seja de `claude --worktree`, de um [subagente usando `isolation: "worktree"`](/docs/pt/sub-agents#choose-the-subagent-scope) ou para uma [sessão em segundo plano](/docs/pt/agent-view#how-file-edits-are-isolated) que Claude Code isola em sua própria worktree. Por padrão, Claude Code cria a cópia de trabalho isolada com `git worktree`. Configurar um hook WorktreeCreate substitui esse comportamento git padrão, permitindo que você use um sistema de controle de versão diferente como SVN, Perforce ou Mercurial.

Como o hook substitui o comportamento padrão inteiramente, [`.worktreeinclude`](/docs/pt/worktrees#copy-gitignored-files-into-worktrees) não é processado. Se você precisar copiar arquivos de configuração local como `.env` para a nova worktree, faça isso dentro de seu script de hook.

O hook deve retornar o caminho para o diretório de worktree criado. Claude Code usa este caminho como o diretório de trabalho para a sessão isolada. Veja [saída WorktreeCreate](#worktreecreate-output) para como cada tipo de hook retorna o caminho.

Claude Code atua no sucesso do hook e no caminho retornado, e descarta `systemMessage` e `continue`.

Este exemplo cria uma cópia de trabalho SVN e imprime o caminho para Claude Code usar. Substitua a URL do repositório pela sua própria:

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

O hook lê o `name` da worktree da entrada JSON em stdin, faz checkout de uma cópia fresca em um novo diretório e imprime o caminho do diretório. O `echo` na última linha é o que Claude Code lê como o caminho da worktree. Redirecione qualquer outra saída para stderr para que não interfira com o caminho.

<h4 id="worktreecreate-input">
  Entrada WorktreeCreate
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks WorktreeCreate recebem o campo `name`. Este é um identificador slug para a nova worktree, especificado pelo usuário ou auto-gerado, por exemplo `bold-oak-a3f2`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeCreate",
  "name": "feature-auth"
}
```

<h4 id="worktreecreate-output">
  Saída WorktreeCreate
</h4>

Os hooks WorktreeCreate não usam o modelo de decisão permitir/bloquear padrão. Em vez disso, o sucesso ou falha do hook determina o resultado. O hook deve retornar o caminho para o diretório de worktree criado:

* **Hooks de comando** (`type: "command"`): imprima o caminho como a última linha não vazia de stdout. Claude Code remove códigos de escape ANSI antes de ler essa linha, portanto banners de inicialização de shell impressos antes de seu `echo` são ignorados. Redirecione qualquer outra saída de hook para stderr.
* **Hooks HTTP** (`type: "http"`): retorne `{ "hookSpecificOutput": { "hookEventName": "WorktreeCreate", "worktreePath": "/absolute/path" } }` no corpo da resposta.

Se o hook falhar ou não produzir um caminho, a criação de worktree falha com um erro.

Claude Code resolve um caminho relativo contra o diretório em que o hook foi executado, colapsando qualquer segmento `.` ou `..` nele. Se o caminho resultante não for um diretório que Claude Code possa entrar, a sessão imprime um erro nomeando o caminho e sai com código 1.

Claude Code recusa um caminho absoluto que contém segmentos `.` ou `..`, e qualquer caminho que passa através de um symlink abaixo da raiz do repositório, porque um symlink comprometido no repositório poderia redirecionar a worktree para fora dele. O erro nomeia o componente rejeitado. Retorne um caminho normalizado que não passa através de um symlink dentro do repositório. Antes da v2.1.216, a criação de worktree seguia o caminho do hook sem essa triagem.

<h3 id="worktreeremove">
  WorktreeRemove
</h3>

Executado quando uma worktree está sendo removida. Este é o equivalente de limpeza para [WorktreeCreate](#worktreecreate). O evento é disparado quando:

* você sai de uma sessão `--worktree` e escolhe removê-la
* um subagente com `isolation: "worktree"` termina
* você exclui uma [sessão em segundo plano](/docs/pt/agent-view#what-deleting-a-session-removes) cuja worktree o hook criou

Para worktrees baseadas em git, Claude Code manipula a limpeza automaticamente com `git worktree remove`. Se você configurou um hook WorktreeCreate para um sistema de controle de versão não-git, emparelhe-o com um hook WorktreeRemove para manipular a limpeza. Sem um, o diretório de worktree é deixado no disco.

Claude Code descarta os [campos de saída JSON](#json-output) de um hook WorktreeRemove, como `systemMessage` e `continue`.

Para uma exclusão de sessão em segundo plano, Claude Code verifica o caminho de worktree armazenado antes de executar o hook e recusa um caminho que é um symlink ou passa através de um abaixo da raiz do repositório. O hook é executado para uma worktree que ainda contém arquivos apenas quando você confirma a exclusão em [agent view](/docs/pt/agent-view#what-deleting-a-session-removes); para tal worktree, [`claude rm`](/docs/pt/agent-view#manage-sessions-from-the-shell) mantém a sessão e worktree em vez disso. Antes da v2.1.216, o hook era executado no caminho armazenado sem essas verificações.

Claude Code passa o caminho retornado por WorktreeCreate como `worktree_path` na entrada do hook. Este exemplo lê esse caminho e remove o diretório:

```json theme={null}
{
  "hooks": {
    "WorktreeRemove": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'jq -r .worktree_path | xargs rm -rf'"
          }
        ]
      }
    ]
  }
}
```

<h4 id="worktreeremove-input">
  Entrada WorktreeRemove
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks WorktreeRemove recebem o campo `worktree_path`, que é o caminho absoluto para a worktree sendo removida.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "WorktreeRemove",
  "worktree_path": "/Users/.../my-project/.claude/worktrees/feature-auth"
}
```

O código de saída de um hook WorktreeRemove decide o resultado. Quando um hook sai com código não-zero e o diretório em `worktree_path` ainda existe depois, a remoção falha:

* A worktree permanece no disco, e o comando do hook e stderr vão para o [log de depuração](#debug-hooks).
* Se você estava excluindo uma sessão em segundo plano, a sessão também permanece. A mensagem de recusa em [agent view](/docs/pt/agent-view#what-deleting-a-session-removes) relata como o hook terminou, como `exited 1`, cita o início de seu stderr e diz se excluir a sessão novamente remove o diretório de qualquer forma.

<h3 id="precompact">
  PreCompact
</h3>

Executado antes de Claude Code estar prestes a executar uma operação de compactação.

O valor do matcher indica se a compactação foi disparada manualmente ou automaticamente:

| Matcher  | Quando é disparado                                                                                                                 |
| :------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| `manual` | `/compact`                                                                                                                         |
| `auto`   | Compactação automática quando a conversa atinge a [janela de compactação automática](/docs/pt/model-config#set-the-auto-compact-window) |

Saia com código 2 para bloquear a compactação. Para um `/compact` manual, a mensagem stderr é mostrada ao usuário. Você também pode bloquear retornando JSON com `"decision": "block"`.

Bloquear compactação automática tem efeitos diferentes dependendo de quando é disparado. Se a compactação foi disparada proativamente antes do limite de contexto, Claude Code a pula e a conversa continua sem compactação. Se a compactação foi disparada para recuperar de um erro de limite de contexto já retornado pela API, o erro subjacente aparece e a solicitação atual falha.

Claude Code descarta os campos `systemMessage` e `continue` de um hook PreCompact.

<h4 id="precompact-input">
  Entrada PreCompact
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks PreCompact recebem `trigger` e `custom_instructions`. Para `manual`, `custom_instructions` contém o que o usuário passa para `/compact` e é `null` quando ele não passa nada. Para `auto`, `custom_instructions` é `null`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreCompact",
  "trigger": "manual",
  "custom_instructions": null
}
```

<h3 id="postcompact">
  PostCompact
</h3>

Executado após Claude Code completar uma operação de compactação. Use este evento para reagir ao novo estado compactado, por exemplo para registrar o resumo gerado ou atualizar estado externo. Claude Code descarta os campos `systemMessage` e `continue` de um hook PostCompact.

Os mesmos valores de matcher se aplicam como para `PreCompact`:

| Matcher  | Quando é disparado                                                                                                                      |
| :------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| `manual` | Após `/compact`                                                                                                                         |
| `auto`   | Após compactação automática quando a conversa atinge a [janela de compactação automática](/docs/pt/model-config#set-the-auto-compact-window) |

<h4 id="postcompact-input">
  Entrada PostCompact
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks PostCompact recebem `trigger` e `compact_summary`. O campo `compact_summary` contém o resumo de conversa gerado pela operação de compactação.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PostCompact",
  "trigger": "manual",
  "compact_summary": "Summary of the compacted conversation..."
}
```

Os hooks PostCompact não têm controle de decisão. Eles não podem afetar o resultado da compactação mas podem executar tarefas de acompanhamento.

<h3 id="premodelswitch">
  PreModelSwitch
</h3>

Executado antes de Claude Code aplicar uma mudança de modelo que você ou um cliente solicitou. Use-o para bloquear uma mudança, exigir confirmação ou mostrar qual será o custo da mudança antes de acontecer.

PreModelSwitch requer Claude Code v2.1.251 ou posterior. Claude Code o executa para essas solicitações:

* `/model <name>` e o seletor `/model`
* O seletor de modelo `Option+P` ou `Alt+P`
* A configuração Model em `/config`
* Ativar [modo rápido](/docs/pt/fast-mode) quando isso muda o modelo da sessão
* Uma solicitação `set_model`, ou uma mudança de modelo em uma solicitação `apply_flag_settings`, de um host [Agent SDK](/docs/pt/agent-sdk/typescript#query-object) ou [Remote Control](/docs/pt/remote-control)

Claude Code não executa hooks PreModelSwitch para mudanças que faz por conta própria, como um [fallback de modelo automático](/docs/pt/model-config#automatic-model-fallback) ou restaurar o modelo quando você retoma uma sessão. Essas mudanças chegam apenas a [PostModelSwitch](#postmodelswitch).

Claude Code compara o matcher contra o nome canônico do modelo para o qual a sessão está mudando, ignorando qualquer sufixo `[1m]`. Um alias como `opus`, um ID de modelo datado e um ID específico do provedor como um ID de modelo Amazon Bedrock todos correspondem ao um nome canônico que resolvem, portanto `claude-opus-5` cobre cada ortografia de Opus 5.

Quando Claude Code não pode determinar um nome canônico para o alvo, por exemplo um ID de modelo personalizado que apenas seu [gateway LLM](/docs/pt/llm-gateway) conhece, ele executa cada hook PreModelSwitch independentemente do matcher. Um hook que bloqueia deve portanto verificar `to_model` de sua entrada em vez de confiar apenas no matcher.

Escreva o matcher como um nome exato, uma lista separada por `|` como `claude-opus-4-6|claude-opus-5` ou uma expressão regular como `.*opus.*`. Este exemplo usa um matcher de nome exato e também verifica `to_model` da entrada do hook, portanto recusa uma mudança para Opus 4.6 ao sair com código 2 e deixa qualquer outro alvo passar:

<Tabs>
  <Tab title="macOS/Linux">
    O comando verifica `to_model` com `jq`:

    ```json theme={null}
    {
      "hooks": {
        "PreModelSwitch": [
          {
            "matcher": "claude-opus-4-6",
            "hooks": [
              {
                "type": "command",
                "command": "jq -e '.to_model | test(\"opus-4-6\")' > /dev/null && { echo 'Opus 4.6 is retired for this project. Use a newer model.' >&2; exit 2; }; exit 0"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    Registre um hook de comando que executa um script através do PowerShell:

    ```json theme={null}
    {
      "hooks": {
        "PreModelSwitch": [
          {
            "matcher": "claude-opus-4-6",
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe",
                "args": [
                  "-NoProfile",
                  "-ExecutionPolicy",
                  "Bypass",
                  "-File",
                  "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-opus-46.ps1"
                ]
              }
            ]
          }
        ]
      }
    }
    ```

    Salve este script em `.claude/hooks/block-opus-46.ps1` em seu projeto:

    ```powershell theme={null}
    $hookInput = [Console]::In.ReadToEnd() | ConvertFrom-Json
    if ($hookInput.to_model -match 'opus-4-6') {
      [Console]::Error.WriteLine('Opus 4.6 is retired for this project. Use a newer model.')
      exit 2
    }
    exit 0
    ```
  </Tab>
</Tabs>

Para confirmar que o hook funciona, execute `/model claude-opus-4-6` de uma sessão executando um modelo diferente. Claude Code mantém o modelo atual e relata que um hook PreModelSwitch bloqueou a mudança, com sua mensagem como o motivo.

<h4 id="premodelswitch-input">
  Entrada PreModelSwitch
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks PreModelSwitch recebem os campos nesta tabela. Os últimos cinco descrevem qual é o custo de reenviar a conversa para o novo modelo, portanto um hook pode mostrar essa figura antes da mudança acontecer.

| Campo                       | Tipo             | Descrição                                                                                                                                                                                                                                                                                                        |
| :-------------------------- | :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `from_model`                | string           | ID de modelo da mudança de                                                                                                                                                                                                                                                                                       |
| `to_model`                  | string           | ID de modelo da mudança para. O matcher compara contra o nome canônico deste modelo                                                                                                                                                                                                                              |
| `requested_model`           | string ou `null` | O modelo que a solicitação nomeou: um alias como `opus`, um ID de modelo completo, ou `null` quando a solicitação foi para o modelo padrão                                                                                                                                                                       |
| `source`                    | string           | De onde a solicitação veio: `"command"` para `/model <name>`, a configuração Model em `/config` ou ativar modo rápido; `"picker"` para um seletor de modelo; `"sdk"` para uma solicitação `set_model`, ou uma mudança de modelo em uma solicitação `apply_flag_settings`, de um host Agent SDK ou Remote Control |
| `context_tokens`            | number           | Tokens que a próxima solicitação reenvia como seu prompt: os tokens de entrada, leitura de cache, criação de cache e saída da última resposta na conversa principal, combinados. `0` antes da primeira resposta                                                                                                  |
| `prompt_cache_warm`         | boolean          | Se o cache de prompt do modelo atual provavelmente ainda está quente, significando que a mudança o perde                                                                                                                                                                                                         |
| `cache_ttl`                 | string           | [Tempo de vida do cache de prompt](/docs/pt/prompt-caching#cache-lifetime) que Claude Code solicita para esta sessão: `"5m"` ou `"1h"`                                                                                                                                                                                |
| `estimated_cache_write_usd` | number           | Custo estimado em dólares americanos de escrever `context_tokens` no cache de prompt em `to_model` na taxa `cache_ttl`, excluindo a próxima resposta. O servidor pode não precisar re-cachear todo o contexto, portanto trate como uma estimativa                                                                |
| `pricing`                   | string           | Como Claude Code precificou `estimated_cache_write_usd`: `"configured"` em suas próprias taxas quando sua organização as configurou, `"catalog"` ao preço de lista, ou `"default"` quando `to_model` não tem preço conhecido e Claude Code assumiu uma taxa padrão                                               |

Este exemplo mostra a entrada para `/model opus` em uma sessão executando Sonnet 5:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreModelSwitch",
  "from_model": "claude-sonnet-5",
  "to_model": "claude-opus-5",
  "requested_model": "opus",
  "source": "command",
  "context_tokens": 182340,
  "prompt_cache_warm": true,
  "cache_ttl": "5m",
  "estimated_cache_write_usd": 1.1396,
  "pricing": "catalog"
}
```

<h4 id="premodelswitch-decision-control">
  Controle de decisão PreModelSwitch
</h4>

Os hooks `PreModelSwitch` podem cancelar a mudança, pedir ao usuário para confirmá-la ou deixá-la prosseguir. Código de saída 2 ou um `decision: "block"` de nível superior cancela a mudança.

Para controle mais fino, retorne `permissionDecision` e `permissionDecisionReason` em um objeto `hookSpecificOutput`, como em [PreToolUse](#pretooluse-decision-control). `PreModelSwitch` aceita `"allow"`, `"deny"` e `"ask"`. Não aceita `"defer"`, `updatedInput` ou `additionalContext`. A tabela abaixo descreve ambos os campos:

| Campo                      | Descrição                                                                                                                                                                                                               |
| :------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionDecision`       | `"allow"` prossegue e pula a [confirmação que Claude Code mostra enquanto o cache de prompt está quente](/docs/pt/prompt-caching#switching-models). `"deny"` cancela a mudança. `"ask"` solicita ao usuário para confirmá-la |
| `permissionDecisionReason` | Para `"deny"`, mostrado ao usuário como o motivo pelo qual a mudança foi bloqueada, ou retornado como o erro para uma solicitação `set_model`. Para `"ask"`, mostrado no prompt de confirmação. Ignorado para `"allow"` |

Apenas `/model` em uma sessão interativa pode mostrar o prompt `"ask"`. Em todas as outras superfícies, incluindo modo não interativo com a flag `-p`, `/config` e solicitações `set_model`, Claude Code trata `"ask"` como uma recusa.

Este exemplo pede ao usuário para confirmar e cita a contagem de tokens de `context_tokens`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreModelSwitch",
    "permissionDecision": "ask",
    "permissionDecisionReason": "Switching now re-sends about 180k tokens to the new model. Continue?"
  }
}
```

Quando vários hooks PreModelSwitch retornam decisões diferentes, a precedência é `deny` > `ask` > `allow`.

Claude Code mostra ao usuário qualquer `systemMessage` que seu hook retorna independentemente da decisão, portanto um hook de relatório de custo pode retornar `{"systemMessage": "..."}` e sair com 0.

Um hook PreModelSwitch que não responde antes de seu tempo limite bloqueia a mudança. Em [PreToolUse](#timeouts), por contraste, um hook de comando que atingiu o tempo limite deixa a chamada de ferramenta continuar. O tempo limite padrão para este evento é 30 segundos. `PreModelSwitch` executa apenas hooks `command`, `http` e `mcp_tool`, portanto os padrões `prompt` e `agent` não se aplicam.

Um hook que sai com um código diferente de 0 ou 2 e não imprime nenhuma decisão JSON não bloqueia: Claude Code mostra seu stderr e aplica a mudança, conforme descrito em [Outros códigos de saída](#other-exit-codes).

<h3 id="postmodelswitch">
  PostModelSwitch
</h3>

Executado após o modelo da sessão mudar. Use-o para dar orientação específica do modelo ao Claude sem editar cada CLAUDE.md, por exemplo uma instrução em toda a organização que se aplica em certos modelos.

PostModelSwitch requer Claude Code v2.1.251 ou posterior. Não pode bloquear, porque o modelo já mudou. Claude Code executa hooks PostModelSwitch após qualquer uma dessas mudanças:

* Uma mudança que você ou um cliente solicitou
* Um [fallback de modelo automático](/docs/pt/model-config#automatic-model-fallback), que muda o modelo da sessão
* Uma configuração como [`opusplan`](/docs/pt/model-config#opusplan-model-setting) entrando ou saindo do modo de plano
* Claude Code restaurando o modelo quando você retoma uma sessão

Claude Code não executa hooks PostModelSwitch quando um modelo de uma [cadeia de modelo fallback](/docs/pt/model-config#fallback-model-chains) serve um turno, porque essa substituição dura um turno e deixa o modelo da sessão inalterado.

O matcher segue as mesmas regras que [PreModelSwitch](#premodelswitch): Claude Code compara contra o nome canônico do modelo para o qual a sessão mudou.

Este exemplo adiciona orientação sempre que o modelo da sessão muda para qualquer modelo Opus:

```json theme={null}
{
  "hooks": {
    "PostModelSwitch": [
      {
        "matcher": ".*opus.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'On Opus, delegate implementation work to subagents and keep this conversation for planning and review.'"
          }
        ]
      }
    ]
  }
}
```

Para confirmar que o hook funciona, mude para um modelo Opus de uma sessão executando um modelo diferente, por exemplo execute `/model opus` de uma sessão Sonnet, depois pergunte ao Claude qual orientação ele tem sobre o modelo atual.

<h4 id="postmodelswitch-input">
  Entrada PostModelSwitch
</h4>

Os hooks PostModelSwitch recebem os mesmos campos que [PreModelSwitch](#premodelswitch-input), com `hook_event_name` definido como `"PostModelSwitch"` e dois valores `source` mais: `"auto"` para um fallback automático ou outra mudança que Claude Code fez por conta própria, e `"resume"` para o modelo restaurado quando você retoma uma sessão.

`requested_model` é `null` quando `source` é `"auto"`. Quando `source` é `"resume"`, é a configuração de modelo salva que Claude Code restaurou.

<h4 id="postmodelswitch-decision-control">
  Controle de decisão PostModelSwitch
</h4>

Claude Code pega seu stdout de [texto simples](#exit-code-0) do hook ao sair com 0, ou `additionalContext` de saída JSON, e o entrega ao Claude com a próxima solicitação após a mudança. Além dos [campos de saída JSON](#json-output) disponíveis para todos os hooks, você pode retornar:

| Campo               | Descrição                                                                                                                         |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------------------- |
| `additionalContext` | String adicionada ao contexto do Claude com a próxima solicitação. Veja [Adicionar contexto para Claude](#add-context-for-claude) |

Se o hook não terminar dentro de cinco segundos após você enviar a próxima solicitação, Claude Code envia essa solicitação sem a saída e a anexa à solicitação seguinte em vez disso. Se o modelo mudar várias vezes antes da próxima solicitação, Claude Code entrega apenas a saída para a mudança de alvo do último modelo.

<h3 id="sessionend">
  SessionEnd
</h3>

Executado quando uma sessão Claude Code termina. Útil para tarefas de limpeza, registrar estatísticas de sessão ou salvar estado de sessão. Suporta matchers para filtrar por motivo de saída.

O campo `reason` na entrada do hook indica por que a sessão terminou:

| Motivo                        | Descrição                                                                             |
| :---------------------------- | :------------------------------------------------------------------------------------ |
| `clear`                       | Sessão limpa com comando `/clear`                                                     |
| `resume`                      | Sessão mudada via `/resume` interativo                                                |
| `logout`                      | Usuário fez logout                                                                    |
| `prompt_input_exit`           | Usuário saiu enquanto entrada de prompt estava visível                                |
| `other`                       | Outros motivos de saída                                                               |
| `bypass_permissions_disabled` | Removido na v2.1.234; Claude Code não o envia. Remova-o de seus matchers `SessionEnd` |

<h4 id="sessionend-input">
  Entrada SessionEnd
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks SessionEnd recebem um campo `reason` indicando por que a sessão terminou. Veja a [tabela de motivos](#sessionend) acima para todos os valores.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionEnd",
  "reason": "other"
}
```

Os hooks SessionEnd não têm controle de decisão. Eles não podem bloquear o término da sessão mas podem executar tarefas de limpeza. Claude Code descarta seus [campos de saída JSON](#json-output), como `systemMessage`.

Os hooks SessionEnd têm um tempo limite padrão de 1,5 segundos. Aplica-se quando você sai, executa `/clear` ou muda de sessões com `/resume` interativo. Você pode dar a um hook mais tempo de duas maneiras:

* **`timeout` por hook**: defina `timeout` na configuração desse hook. O orçamento geral sobe automaticamente para corresponder ao `timeout` por hook mais alto em seus arquivos de configurações, até 60 segundos. Se você aumentar o orçamento dessa forma, um hook sem seu próprio `timeout` ainda mantém o padrão. Tempos limite definidos em hooks fornecidos por plugin não aumentam o orçamento.
* **`CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS`**: defina esta variável de ambiente em milissegundos para sobrescrever o orçamento explicitamente. O valor que você define também se torna o tempo limite para cada hook sem seu próprio `timeout`.

Este exemplo define o orçamento para 5 segundos:

```bash theme={null}
CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS=5000 claude
```

Antes da v2.1.268, `CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS` aumentava apenas o orçamento geral, e um hook sem seu próprio `timeout` ainda era cancelado após 1,5 segundos.

<h3 id="elicitation">
  Elicitation
</h3>

Executado quando um servidor MCP solicita entrada do usuário no meio de uma tarefa. Por padrão, Claude Code mostra um diálogo interativo para o usuário responder. Os hooks podem interceptar essa solicitação e responder programaticamente, pulando o diálogo inteiramente.

O campo matcher corresponde ao nome do servidor MCP.

<h4 id="elicitation-input">
  Entrada Elicitation
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks Elicitation recebem `mcp_server_name`, `message` e campos opcionais `mode`, `url`, `elicitation_id` e `requested_schema`.

Para elicitação de modo de formulário, o caso mais comum:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please provide your credentials",
  "mode": "form",
  "requested_schema": {
    "type": "object",
    "properties": {
      "username": { "type": "string", "title": "Username" }
    }
  }
}
```

Para elicitação de modo URL, usada para autenticação baseada em navegador:

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Elicitation",
  "mcp_server_name": "my-mcp-server",
  "message": "Please authenticate",
  "mode": "url",
  "url": "https://auth.example.com/login"
}
```

<h4 id="elicitation-output">
  Saída Elicitation
</h4>

Para responder programaticamente sem mostrar o diálogo, retorne um objeto JSON com `hookSpecificOutput`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "Elicitation",
    "action": "accept",
    "content": {
      "username": "alice"
    }
  }
}
```

| Campo     | Valores                       | Descrição                                                                        |
| :-------- | :---------------------------- | :------------------------------------------------------------------------------- |
| `action`  | `accept`, `decline`, `cancel` | Se deve aceitar, recusar ou cancelar a solicitação                               |
| `content` | object                        | Valores de campo de formulário a enviar. Usado apenas quando `action` é `accept` |

Código de saída 2 nega a elicitação. Claude Code não mostra sua mensagem stderr em lugar nenhum.

Claude Code atua em `hookSpecificOutput` da saída JSON de um hook Elicitation e descarta `systemMessage` e `continue`.

<h3 id="elicitationresult">
  ElicitationResult
</h3>

Executado após um usuário responder a uma elicitação MCP. Os hooks podem observar, modificar ou bloquear a resposta antes de ser enviada de volta para o servidor MCP.

O campo matcher corresponde ao nome do servidor MCP.

<h4 id="elicitationresult-input">
  Entrada ElicitationResult
</h4>

Além dos [campos de entrada comuns](#common-input-fields), os hooks ElicitationResult recebem `mcp_server_name`, `action` e campos opcionais `mode`, `elicitation_id` e `content`.

```json theme={null}
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "ElicitationResult",
  "mcp_server_name": "my-mcp-server",
  "action": "accept",
  "content": { "username": "alice" },
  "mode": "form",
  "elicitation_id": "elicit-123"
}
```

<h4 id="elicitationresult-output">
  Saída ElicitationResult
</h4>

Para sobrescrever a resposta do usuário, retorne um objeto JSON com `hookSpecificOutput`:

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "ElicitationResult",
    "action": "decline",
    "content": {}
  }
}
```

| Campo     | Valores                       | Descrição                                                                                   |
| :-------- | :---------------------------- | :------------------------------------------------------------------------------------------ |
| `action`  | `accept`, `decline`, `cancel` | Sobrescreve a ação do usuário                                                               |
| `content` | object                        | Sobrescreve valores de campo de formulário. Significativo apenas quando `action` é `accept` |

Código de saída 2 bloqueia a resposta, alterando a ação efetiva para `decline`. Claude Code não mostra sua mensagem stderr em lugar nenhum.

Claude Code atua em `hookSpecificOutput` da saída JSON de um hook ElicitationResult e descarta `systemMessage` e `continue`.

<h2 id="prompt-based-hooks">
  Hooks baseados em prompt
</h2>

Além de hooks de comando, HTTP e MCP tool, Claude Code suporta hooks baseados em prompt (`type: "prompt"`) que usam um LLM para avaliar se deve permitir ou bloquear uma ação, e hooks de agente (`type: "agent"`) que geram um verificador agentic com acesso a ferramentas. Nem todos os eventos suportam cada tipo de hook.

Eventos que suportam todos os cinco tipos de hook (`command`, `http`, `mcp_tool`, `prompt` e `agent`):

* `PermissionDenied`
* `PostToolBatch`
* `PostToolUse`
* `PostToolUseFailure`
* `PreToolUse`
* `Stop`
* `SubagentStop`
* `TaskCompleted`
* `TaskCreated`
* `TeammateIdle`
* `UserPromptExpansion`
* `UserPromptSubmit`

`PermissionRequest` suporta hooks `command`, `http`, `mcp_tool` e `prompt` mas não hooks `agent`. Se você configurar um hook de agente neste evento, Claude Code o ignora e o fluxo de permissão prossegue inalterado. Para permitir ou negar de um hook, retorne o [objeto de decisão](#permissionrequest-decision-control) de um hook de comando ou HTTP.

Eventos que suportam hooks `command`, `http` e `mcp_tool` mas não `prompt` ou `agent`:

* `ConfigChange`
* `CwdChanged`
* `DirectoryAdded`
* `Elicitation`
* `ElicitationResult`
* `FileChanged`
* `InstructionsLoaded`
* `MessageDisplay`
* `Notification`
* `PostCompact`
* `PostModelSwitch`
* `PreCompact`
* `PreModelSwitch`
* `SessionEnd`
* `StopFailure`
* `SubagentStart`
* `WorktreeCreate`
* `WorktreeRemove`

`SessionStart` e `Setup` suportam hooks `command` e `mcp_tool`, e [MCP tool hook fields](#mcp-tool-hook-fields) descreve quando seus hooks `mcp_tool` são executados. Eles não suportam hooks `http`, `prompt` ou `agent`.

<h3 id="how-prompt-based-hooks-work">
  Como hooks baseados em prompt funcionam
</h3>

Em vez de executar um comando Bash, hooks baseados em prompt:

1. Enviam a entrada do hook e seu prompt para um modelo Claude, Haiku por padrão
2. O LLM responde com JSON estruturado contendo uma decisão
3. Claude Code processa a decisão automaticamente

<h3 id="prompt-hook-configuration">
  Configuração de hook de prompt
</h3>

Defina `type` para `"prompt"` e forneça uma string `prompt` em vez de um `command`. Use o placeholder `$ARGUMENTS` para injetar dados de entrada do hook em seu texto de prompt.

Este hook `Stop` pede ao LLM para avaliar se todas as tarefas estão completas antes de permitir que Claude termine:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Evaluate if Claude should stop: $ARGUMENTS. Check if all tasks are complete."
          }
        ]
      }
    ]
  }
}
```

| Campo             | Obrigatório | Descrição                                                                                                                                                                                                                      |
| :---------------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`            | sim         | Deve ser `"prompt"`                                                                                                                                                                                                            |
| `prompt`          | sim         | O texto do prompt a enviar para o LLM. Use `$ARGUMENTS` como placeholder para a entrada JSON do hook. Se `$ARGUMENTS` não estiver presente, entrada JSON é anexada ao prompt                                                   |
| `model`           | não         | Modelo a usar para avaliação. Padrão para um modelo rápido                                                                                                                                                                     |
| `timeout`         | não         | Timeout em segundos. Padrão: 30                                                                                                                                                                                                |
| `continueOnBlock` | não         | Nos eventos aos quais se aplica, `true` alimenta uma razão `ok: false` de volta para Claude e continua em vez de terminar o turno. Padrão: `false`. Veja [Esquema de resposta](#response-schema) para comportamento por evento |

<h3 id="response-schema">
  Esquema de resposta
</h3>

O LLM deve responder com JSON contendo:

```json theme={null}
{
  "ok": true | false,
  "reason": "Explanation for the decision",
  "impossible": true | false
}
```

| Campo        | Descrição                                                                                                                                                                                                                                                      |
| :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ok`         | `true` para permitir. Para `false`, veja o comportamento por evento abaixo                                                                                                                                                                                     |
| `reason`     | Obrigatório quando `ok` é `false`                                                                                                                                                                                                                              |
| `impossible` | Opcional. O modelo o retorna com `ok: false` quando julga que a condição nunca pode ser satisfeita. Em `Stop` e `SubagentStop`, Claude Code então permite que o turno termine em vez de alimentar a razão de volta. Hooks de agente e outros eventos o ignoram |

O que acontece em `ok: false` depende do evento:

* `Stop` e `SubagentStop`: a razão é alimentada de volta para Claude como sua próxima instrução e o turno continua, a menos que a resposta também defina `impossible: true`, caso em que Claude Code permite a parada e o turno termina
* `PreToolUse`: a chamada de ferramenta é negada; por padrão o turno termina e a razão de negação aparece no chat como uma linha de aviso. Defina `continueOnBlock: true` para em vez disso retornar a razão para Claude como o erro da ferramenta para que possa se ajustar e continuar, equivalente a um hook de comando com `permissionDecision: "deny"`. Antes da v2.1.210, a razão de negação era retornada para Claude como o erro da ferramenta e o turno continuava
* `PostToolUse`: por padrão o turno termina e a razão aparece no chat como uma linha de aviso. Defina `continueOnBlock: true` para alimentar a razão de volta para Claude e continuar o turno em vez disso
* `PostToolBatch`, `UserPromptSubmit` e `UserPromptExpansion`: o turno termina e a razão aparece como uma linha de aviso. Esses eventos terminam o turno em `decision: "block"` independentemente de `continue`
* `PostToolUseFailure` e `TaskCreated`: a razão é retornada para Claude como um erro de ferramenta e o turno continua, independentemente de `continueOnBlock`
* `TaskCompleted`: quando dispara porque uma tarefa é marcada como concluída durante um turno, a razão é retornada para Claude como um erro de ferramenta e o turno continua, independentemente de `continueOnBlock`. Quando dispara porque um colega para, se comporta como `TeammateIdle` e interrompe o colega por padrão
* `TeammateIdle`: por padrão o colega para e a razão aparece como uma linha de aviso. Defina `continueOnBlock: true` para alimentar a razão de volta para o colega e mantê-lo trabalhando em vez disso
* `PermissionRequest`: `ok: false` não tem efeito. Para negar uma aprovação de um hook, use um [hook de comando](#command-hook-fields) retornando `hookSpecificOutput.decision.behavior: "deny"`
* `PermissionDenied`: `ok: false` não tem efeito porque a negação já aconteceu. A única saída que este evento lê é `hookSpecificOutput.retry`, que hooks de prompt e agente não podem definir. Eles são executados neste evento, mas sua saída é descartada. Use um [hook de comando](#command-hook-fields) para retornar `retry`

Se você precisar de controle mais fino em qualquer evento, use um [hook de comando](#command-hook-fields) com os campos por evento descritos em [Controle de decisão](#decision-control).

<h3 id="check-multiple-conditions-before-stopping">
  Verificar múltiplas condições antes de parar
</h3>

Este hook `Stop` usa um prompt detalhado para verificar três condições antes de permitir que Claude pare. Hooks `SubagentStop` usam o mesmo formato para avaliar se um [subagente](/docs/pt/sub-agents) deve parar. Se o modelo retornar `"ok": false` porque a condição ainda não foi atendida, Claude continua trabalhando com a razão fornecida como sua próxima instrução:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "You are evaluating whether Claude should stop working. Context: $ARGUMENTS\n\nAnalyze the conversation and determine if:\n1. All user-requested tasks are complete\n2. Any errors need to be addressed\n3. Follow-up work is needed\n\nRespond with JSON: {\"ok\": true} to allow stopping, or {\"ok\": false, \"reason\": \"your explanation\"} to continue working.",
            "timeout": 30
          }
        ]
      }
    ]
  }
}
```

<h2 id="agent-based-hooks">
  Hooks baseados em agente
</h2>

<Warning>
  Hooks de agente são experimentais. O comportamento e a configuração podem mudar em versões futuras. Para fluxos de trabalho em produção, prefira [command hooks](#command-hook-fields).
</Warning>

Hooks baseados em agente (`type: "agent"`) são como hooks baseados em prompt mas com acesso a ferramentas de múltiplos turnos. Em vez de uma única chamada LLM, um hook de agente gera um subagente que pode ler arquivos, pesquisar código e inspecionar o codebase para verificar condições. Hooks de agente suportam os mesmos eventos que [hooks baseados em prompt](#prompt-based-hooks), exceto `PermissionRequest`.

<h3 id="how-agent-hooks-work">
  Como hooks de agente funcionam
</h3>

Quando um hook de agente dispara:

1. Claude Code gera um subagente com seu prompt e a entrada JSON do hook
2. O subagente pode usar ferramentas como Read, Grep e Glob para investigar
3. Após até 50 turnos, o subagente retorna uma decisão estruturada `{ "ok": true/false }`
4. Claude Code permite a ação se `ok` for `true`. Se `ok` for `false`, Claude Code trata o bloqueio da mesma forma que um hook de prompt com `continueOnBlock: true` naquele evento, conforme listado em [Response schema](#response-schema)

Hooks de agente são úteis quando a verificação requer inspecionar arquivos reais ou saída de teste, não apenas avaliar dados de entrada do hook sozinhos.

<h3 id="agent-hook-configuration">
  Configuração de hook de agente
</h3>

Defina `type` para `"agent"` e forneça uma string `prompt`, usando `$ARGUMENTS` como placeholder para a entrada JSON do hook. Os campos de configuração são os mesmos que [prompt hooks](#prompt-hook-configuration), exceto que hooks de agente têm um timeout padrão mais longo de 60 segundos e nenhum campo `continueOnBlock`.

O esquema de resposta é `{ "ok": true }` para permitir ou `{ "ok": false, "reason": "..." }` para bloquear. Em `ok: false`, Claude Code trata um hook de agente da forma que trata um [prompt hook com `continueOnBlock: true`](#response-schema) no mesmo evento; hooks de agente não têm campo `continueOnBlock` e não suportam o campo `impossible` do hook de prompt.

Este hook `Stop` verifica que todos os testes unitários passam antes de permitir que Claude termine:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "agent",
            "prompt": "Verify that all unit tests pass. Run the test suite and check the results. $ARGUMENTS",
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

<h2 id="run-hooks-in-the-background">
  Executar hooks em background
</h2>

Por padrão, hooks bloqueiam a execução de Claude até que completem. Para tarefas de longa duração como deployments, suites de teste ou chamadas de API externas, defina `"async": true` para executar o hook em background enquanto Claude continua trabalhando. Hooks assíncronos não podem bloquear ou controlar comportamento de Claude: campos de resposta como `decision`, `permissionDecision` e `continue` não têm efeito, porque a ação que controlariam já completou.

<h3 id="configure-an-async-hook">
  Configurar um hook assíncrono
</h3>

Adicione `"async": true` à configuração de um hook de comando para executá-lo em background sem bloquear Claude. Este campo está apenas disponível em hooks `type: "command"`.

Este hook executa um script de teste após cada chamada de ferramenta `Write`. Claude continua trabalhando imediatamente enquanto `run-tests.sh` executa. Quando o script termina, sua saída é entregue no próximo turno de conversa:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/run-tests.sh",
            "async": true
          }
        ]
      }
    ]
  }
}
```

Uma vez que um hook assíncrono está executando em background, Claude Code não impõe `timeout` nele. Claude Code ainda impõe `timeout` em um hook que você executa com `asyncRewake`.

Claude Code entrega resultados de um hook assíncrono apenas enquanto a sessão está em execução:

* Em [modo não-interativo](/docs/pt/headless) com a flag `-p`, Claude Code mata qualquer hook assíncrono ainda em execução no teardown e o finaliza com resultado `cancelled`
* Se o trabalho do seu hook deve sobreviver a uma sessão `claude -p`, inicie um processo totalmente desacoplado a partir dele

<h3 id="how-async-hooks-execute">
  Como hooks assíncronos executam
</h3>

Quando um hook assíncrono dispara, Claude Code inicia o processo do hook e imediatamente continua sem esperar que termine. O hook recebe a mesma entrada JSON via stdin que um hook síncrono.

Após o processo em background sair, Claude Code entrega os campos `additionalContext` e `systemMessage` da resposta JSON do hook ao Claude no próximo turno de conversa. Diferentemente de um `systemMessage` de hook síncrono, nenhum dos dois campos é mostrado para você.

Claude Code valida que a resposta JSON contra o mesmo [esquema de saída](#json-output) que hooks síncronos, e descarta qualquer campo cujo valor tenha o tipo errado, como um `systemMessage` que não seja uma string, em vez de entregá-lo. Execute com `--debug` para ver um aviso nomeando cada campo descartado. Antes da v2.1.202, saída JSON malformada de um hook assíncrono poderia travar a sessão, e a falha recorria cada vez que a sessão era retomada.

Notificações de conclusão de hook assíncrono são suprimidas por padrão. Para vê-las, ative modo verbose com `Ctrl+O` ou inicie Claude Code com `--verbose`.

<h3 id="run-tests-after-file-changes">
  Executar testes após mudanças de arquivo
</h3>

Este hook inicia uma suite de testes em background sempre que Claude escreve um arquivo, então relata os resultados de volta ao Claude quando os testes terminam. Salve este script em `.claude/hooks/run-tests-async.sh` em seu projeto e torne-o executável com `chmod +x`:

```bash theme={null}
#!/bin/bash
# run-tests-async.sh

# Leia entrada de hook de stdin
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Apenas execute testes para arquivos de origem
if [[ "$FILE_PATH" != *.ts && "$FILE_PATH" != *.js ]]; then
  exit 0
fi

# Execute testes e relate resultados ao Claude via additionalContext
RESULT=$(npm test 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  MSG="Tests passed after editing $FILE_PATH"
else
  MSG="Tests failed after editing $FILE_PATH: $RESULT"
fi
jq -nc --arg msg "$MSG" '{hookSpecificOutput: {hookEventName: "PostToolUse", additionalContext: $msg}}'
```

Então adicione esta configuração a `.claude/settings.json` na raiz do seu projeto. A flag `async: true` permite que Claude continue trabalhando enquanto testes executam:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/run-tests-async.sh",
            "args": [],
            "async": true
          }
        ]
      }
    ]
  }
}
```

<h3 id="limitations">
  Limitações
</h3>

Hooks assíncronos têm restrições adicionais comparados a hooks síncronos:

* Saída de hook é entregue no próximo turno de conversa. Se a sessão está ociosa, a resposta espera até a próxima interação do usuário. Exceção: um hook `asyncRewake` que sai com código 2 acorda Claude imediatamente mesmo quando a sessão está ociosa.
* Cada execução cria um processo em background separado. Não há desduplicação através de múltiplos disparos do mesmo hook assíncrono.

<h2 id="security-considerations">
  Considerações de segurança
</h2>

<h3 id="disclaimer">
  Aviso
</h3>

<Warning>
  Hooks de comando executam comandos shell com suas permissões completas de usuário. Eles podem modificar, deletar ou acessar qualquer arquivo que sua conta de usuário pode acessar. Revise e teste todos os comandos de hook antes de adicioná-los à sua configuração.
</Warning>

<h3 id="workspace-trust">
  Confiança do workspace
</h3>

Claude Code verifica a confiança do workspace antes de executar qualquer hook de um arquivo de configurações. O que conta como confiável depende do tipo de sessão:

* **Sessão interativa**: Claude Code retém hooks de todos os arquivos de configurações, incluindo seu próprio `~/.claude/settings.json`, até que você aceite o [diálogo de confiança do workspace](/docs/pt/permissions#project-allow-rules-and-workspace-trust) para a pasta, ou para um diretório pai cuja confiança se estende a ela
* **Sessão `-p` ou SDK**: Claude Code nunca mostra o diálogo e trata a pasta como confiável, então hooks confirmados no `.claude/settings.json` de um repositório são executados em uma pasta que você nunca confiou

Antes de executar `claude -p` em um repositório que você não escreveu, revise seus arquivos de configurações `.claude/`, comece com [`--bare`](/docs/pt/headless#start-faster-with-bare-mode), ou [desative hooks para essa execução](#disable-or-remove-hooks) com `--settings '{"disableAllHooks": true}'`. Hooks de frontmatter em um subagente de projeto seguem uma regra mais rigorosa do que hooks de arquivo de configurações. [O que é executado antes de você confiar em uma pasta](/docs/pt/permissions#what-runs-before-you-trust-a-folder) lista cada tipo de conteúdo de repositório por tipo de sessão.

<h3 id="security-best-practices">
  Melhores práticas de segurança
</h3>

Mantenha essas práticas em mente ao escrever hooks:

* **Valide e sanitize entradas**: nunca confie em dados de entrada cegamente
* **Sempre cite variáveis shell**: use `"$VAR"` não `$VAR`
* **Bloqueie traversal de caminho**: verifique `..` em caminhos de arquivo
* **Use caminhos absolutos**: especifique caminhos completos para scripts. Na forma exec, use `${CLAUDE_PROJECT_DIR}` e o caminho não precisa de aspas. Na forma shell, envolva-o em aspas duplas
* **Pule arquivos sensíveis**: evite `.env`, `.git/`, chaves, etc.

<h2 id="windows-powershell-tool">
  Ferramenta Windows PowerShell
</h2>

No Windows, você pode executar hooks individuais em PowerShell definindo `"shell": "powershell"` em um hook de comando. Claude Code auto-detecta `pwsh.exe`, o executável do PowerShell 7 e posterior, e volta para `powershell.exe` para Windows PowerShell 5.1.

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write",
        "hooks": [
          {
            "type": "command",
            "shell": "powershell",
            "command": "Write-Host 'File written'"
          }
        ]
      }
    ]
  }
}
```

Para referenciar o diretório raiz do projeto a partir de um comando em forma de shell do PowerShell, escreva `${CLAUDE_PROJECT_DIR}` ou `$env:CLAUDE_PROJECT_DIR`. A partir da v2.1.198, Claude Code reescreve os espaços reservados `${CLAUDE_PROJECT_DIR}`, `${CLAUDE_PLUGIN_ROOT}` e `${CLAUDE_PLUGIN_DATA}` em um comando em forma de shell do PowerShell para a forma `${env:NAME}` do PowerShell, independentemente de o hook estar definido em `settings.json`, um plugin ou uma skill. PowerShell então resolve o valor do ambiente exportado após análise, então o espaço reservado funciona dentro de strings entre aspas duplas, mas não dentro de strings entre aspas simples, onde PowerShell nunca expande variáveis.

Antes da v2.1.198, essa reescrita se aplicava apenas a hooks de plugin. Em versões anteriores, um hook `settings.json` precisa da forma `$env:` ou [forma exec](#exec-form-and-shell-form), onde `${CLAUDE_PROJECT_DIR}` é substituído em cada elemento `args` independentemente de onde o hook está definido.

Não escreva a forma nua `$CLAUDE_PROJECT_DIR` em um hook do PowerShell. PowerShell a analisa como uma variável local indefinida e a resolve para `$null`, o que deixa o caminho do script sem seu prefixo de diretório raiz do projeto. Claude Code não reescreve essa forma; em vez disso, registra um aviso no [log de depuração](#debug-hooks).

O exemplo abaixo mostra um hook `settings.json` que executa um script de projeto com a forma `$env:`, que funciona em todas as versões:

```json theme={null}
{
  "type": "command",
  "shell": "powershell",
  "command": "& \"$env:CLAUDE_PROJECT_DIR\\.claude\\hooks\\check.ps1\""
}
```

<h2 id="debug-hooks">
  Debug de hooks
</h2>

Detalhes de execução de hook são escritos no arquivo de log de debug. Inicie Claude Code com `claude --debug-file <path>` para escrever o log em um local conhecido, ou execute `claude --debug` e leia o log em `~/.claude/debug/<session-id>.txt`. A flag `--debug` não imprime no terminal.

Por exemplo, um hook `PostToolUse` em `Write` cujo comando imprime `hook-ran` produz entradas como:

```text theme={null}
2026-07-19T02:03:24.382Z [DEBUG] Hook output does not start with {, treating as plain text
2026-07-19T02:03:24.382Z [DEBUG] "Hook PostToolUse:Write (PostToolUse) success:\nhook-ran"
```

Para detalhes de correspondência de hook mais granulares, defina `CLAUDE_CODE_DEBUG_LOG_LEVEL=verbose` para ver linhas de log adicionais como contagens de matcher de hook e correspondência de consulta.

Para troubleshooting de problemas comuns como hooks não disparando, Stop hooks que continuam bloqueando, ou erros de configuração, consulte [Limitações e troubleshooting](/docs/pt/hooks-guide#limitations-and-troubleshooting) no guia. Para um passo a passo de diagnóstico mais amplo cobrindo `/context`, `/doctor` e precedência de configurações, consulte [Debug your config](/docs/pt/debug-your-config).
