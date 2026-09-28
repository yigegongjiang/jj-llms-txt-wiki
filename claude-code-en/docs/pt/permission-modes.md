> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Escolha um modo de permissão

> Controle se Claude pede permissão antes de agir. Alterne modos de permissão com Shift+Tab na CLI, o indicador de modo no VS Code ou o seletor de modo no Desktop.

Um modo de permissão define quais ações Claude pode executar em uma sessão sem pedir sua permissão primeiro. No modo Manual, Claude Code para e pede sua permissão antes da maioria das ações que editam arquivos, executam comandos shell ou acessam a rede. No [modo automático](#eliminate-prompts-with-auto-mode), um segundo modelo, o classificador, revisa as ações em vez de você; [como o classificador avalia ações](#how-the-classifier-evaluates-actions) lista quais ações ele revisa e quais o ignoram.

Nos planos Pro, Max e Team, o modo de permissão inicial integrado é o modo automático. [Qual modo uma sessão inicia](#which-mode-a-session-starts-in) cobre as superfícies e configurações que alteram o modo de permissão inicial. Você também pode alterar o modo de permissão de uma sessão em execução a qualquer momento.

<h2 id="available-modes">
  Modos disponíveis
</h2>

Cada modo faz uma compensação diferente entre conveniência e supervisão. A tabela abaixo mostra o que Claude pode fazer sem um prompt de permissão em cada modo. O modo Manual aparece sob seu valor de configuração, `default`.

| Modo                                                                | O que é executado sem perguntar                                                                                                  | Melhor para                                      |
| :------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------- |
| `default`                                                           | Apenas leituras                                                                                                                  | Revisar cada ação você mesmo, trabalho sensível  |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | Leituras, edições de arquivo e comandos comuns do sistema de arquivos (`mkdir`, `touch`, `mv`, `cp`, etc.)                       | Iterando sobre código que você está revisando    |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | Leituras, mais comandos aprovados pelo classificador quando [modo automático](#eliminate-prompts-with-auto-mode) está disponível | Explorando uma base de código antes de alterá-la |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | Tudo, com verificações de segurança em segundo plano                                                                             | Tarefas longas, reduzindo fadiga de prompts      |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | Leituras e ferramentas pré-aprovadas; qualquer coisa que geraria um prompt é negada                                              | CI bloqueado e scripts                           |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | Tudo                                                                                                                             | Apenas contêineres isolados e VMs                |

O modo que revisa cada ação é nomeado **Manual** na CLI, em `claude --help`, nas extensões VS Code e JetBrains, e no aplicativo de desktop. Seu valor de configuração é `default`, que é o que hooks e integrações SDK usam. A CLI aceita `manual` como um alias em qualquer lugar onde você digita o valor, por exemplo `claude --permission-mode manual` ou `"defaultMode": "manual"`. O rótulo Manual e o alias `manual` requerem Claude Code v2.1.200 ou posterior. O rótulo do aplicativo de desktop não depende da sua versão da CLI.

As gravações em [caminhos protegidos](#protected-paths) nunca são auto-aprovadas, exceto no modo `bypassPermissions` e em sessões de modo plan onde permissões de bypass estão disponíveis, significando sessões iniciadas de uma forma que [coloca `bypassPermissions` no ciclo de modo](#switch-permission-modes).

Os modos definem a linha de base. Sobreponha [regras de permissão](/docs/pt/permissions#manage-permissions) no topo para pré-aprovar ou bloquear ferramentas específicas. Regras de negação bloqueiam em todos os modos, incluindo `bypassPermissions`. Regras de negação e solicitação não se aplicam a [`EndConversation`](/docs/pt/tools-reference#endconversation-tool-behavior) enquanto Claude ainda tiver pelo menos uma outra ferramenta que possa chamar. Regras de permissão não têm efeito em `bypassPermissions`.

<h3 id="actions-no-mode-auto-approves">
  Ações que nenhum modo auto-aprova
</h3>

Claude Code não auto-aprova o seguinte em nenhum modo, incluindo `bypassPermissions`. Cada item vincula à seção que diz o que acontece em vez disso em cada modo:

* Ferramentas correspondidas por uma [regra de solicitação](/docs/pt/permissions#manage-permissions) explícita
* Ferramentas de conector que sua organização [definiu como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools), em sessões onde essa configuração chega a Claude Code
* Ferramentas que requerem interação do usuário: a ferramenta integrada `AskUserQuestion` e ferramentas MCP marcadas [`requiresUserInteraction`](/docs/pt/mcp#require-approval-for-a-specific-tool)
* Remoções `rm` e `rmdir` direcionadas a um [caminho crítico](#critical-paths), que nenhuma regra de permissão ou hook `PreToolUse` `"allow"` aprova
* As [salvaguardas de mensagens entre sessões](#skip-all-checks-with-bypasspermissions-mode)
* Leituras fora dos diretórios de trabalho enquanto [`permissions.blockReadsOutsideWorkingDirectories`](/docs/pt/settings-reference#permissions-blockreadsoutsideworkingdirectories) está ativado: comandos Bash reconhecidos de leitura de arquivo e qualquer [retry não-sandboxizado](/docs/pt/sandboxing#the-unsandboxed-retry-escape-hatch) que precisa de aprovação para executar fora do sandbox, mesmo em modo automático e modo `bypassPermissions`. Requer Claude Code v2.1.257 ou posterior.

  Um comando que o analisador de shell não consegue rastrear, como um que muda de diretório mais de uma vez ou executa um subshell, solicita da mesma forma mesmo quando não nomeia nenhum caminho externo. Este prompt não se aplica quando o comando é executado no [sandbox](/docs/pt/sandboxing) e o sandbox impõe o bloqueio.

<h2 id="common-setups">
  Configurações comuns
</h2>

Os modos de permissão decidem se Claude pede antes de uma ação, e o [sandbox Bash](/docs/pt/sandboxing) e os [limites de isolamento](/docs/pt/sandbox-environments) externos decidem o que uma ação pode alcançar uma vez que é executada. Cada linha abaixo emparelha um objetivo com as flags ou configurações que o levam lá e o isolamento que precisa, como ponto de partida. [Modos disponíveis](#available-modes) lista o que é executado sem um prompt em cada modo.

| Você quer                                                 | Comece com                                                                                                                                                               | Isolamento necessário                                                                                                                                                                                 | Notas                                                                                                                                                                                                                                                                                    |
| :-------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Revisar cada ação você mesmo                              | Modo Manual: `claude --permission-mode default`                                                                                                                          | Nenhum                                                                                                                                                                                                | Trabalho sensível, código desconhecido                                                                                                                                                                                                                                                   |
| Iterar localmente com menos prompts, sem um classificador | Modo Manual mais o sandbox Bash em [modo auto-allow](/docs/pt/sandboxing#sandbox-modes): `claude --permission-mode default`, depois execute `/sandbox` e selecione auto-allow | O sandbox Bash integrado, em macOS, Linux e WSL2                                                                                                                                                      | Regras de negação ainda se aplicam, e regras de solicitação que nomeiam um comando, como `Bash(git push *)`, ainda solicitam. Para ativar o sandbox a partir de um arquivo de configurações em vez disso, defina [`sandbox.enabled`](/docs/pt/settings-reference#sandbox-enabled) como `true` |
| Explorar antes de alterar qualquer coisa                  | `claude --permission-mode plan`                                                                                                                                          | Nenhum                                                                                                                                                                                                | Claude Code bloqueia edições até que você [aprove um plano](#review-and-approve-a-plan)                                                                                                                                                                                                  |
| Trabalhar sem supervisão em modo automático               | `claude --permission-mode auto`, o [modo de permissão inicial integrado](#which-mode-a-session-starts-in) em Pro, Max e Team                                             | Nenhum; um sandbox ou contêiner adiciona defesa em profundidade                                                                                                                                       | Requer um [modelo suportado](#eliminate-prompts-with-auto-mode), e sua organização pode [desativar o modo automático](#eliminate-prompts-with-auto-mode)                                                                                                                                 |
| Executar em CI com uma lista de permissões exata          | `claude -p "run the test suite" --permission-mode dontAsk --allowedTools "Bash(npm test)" "Read"`                                                                        | Nenhum além do que seu executor de CI fornece                                                                                                                                                         | [Cloud sessions](/docs/pt/claude-code-on-the-web) ignora `dontAsk` de arquivos de configurações                                                                                                                                                                                               |
| Executar totalmente sem supervisão dentro de um contêiner | `claude -p "<prompt>" --dangerously-skip-permissions`                                                                                                                    | Obrigatório: um contêiner, VM ou o [runtime do sandbox](/docs/pt/sandbox-environments#sandbox-runtime); em Linux e macOS, execute como um [usuário não-root](#skip-all-checks-with-bypasspermissions-mode) | Cloud sessions ignora este modo de arquivos de configurações. Nesta execução `-p`, as [poucas chamadas que ainda solicitariam](#skip-all-checks-with-bypasspermissions-mode) são negadas em vez disso                                                                                    |

O sandbox Bash e o modo automático funcionam independentemente e se combinam, com as exceções listadas em [Sandbox modes](/docs/pt/sandboxing#sandbox-modes). Para a interação completa, veja [Como sandboxing se relaciona com permissões e modos de permissão](/docs/pt/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes) e [Como isolamento se relaciona com modos de permissão](/docs/pt/sandbox-environments#how-isolation-relates-to-permission-modes).

<h2 id="which-mode-a-session-starts-in">
  Qual modo uma sessão inicia
</h2>

Quando você inicia uma nova sessão em um terminal, Claude Code pega o modo de permissão do primeiro destes que se aplica:

1. A flag `--permission-mode` ou `--dangerously-skip-permissions`

2. `permissions.defaultMode` em um [arquivo de configurações](/docs/pt/settings#where-settings-live)

   Se você definir `"auto"` em `.claude/settings.json` ou `.claude/settings.local.json`, o valor não entra em vigor, e Claude Code então usa o padrão integrado em vez de um `defaultMode` de `~/.claude/settings.json`. Se você definir `"bypassPermissions"` nesses dois arquivos, também não entra em vigor, e a sessão inicia no modo Manual. Os outros valores se aplicam de qualquer arquivo de configurações.

3. O padrão integrado

Conversas que a extensão VS Code inicia seguem a lista própria da extensão em [Alternar modos de permissão](#switch-permission-modes). Para o modo de permissão que Claude Code inicia uma sessão retomada, veja [modo de permissão ao retomar](/docs/pt/sessions#permission-mode-on-resume).

O padrão integrado `auto` requer Claude Code v2.1.228 ou posterior em macOS, Linux e WSL, e v2.1.233 ou posterior no Windows nativo. Em versões anteriores, o padrão integrado é Manual.

O padrão integrado depende de como você executa Claude Code, do seu plano e se Claude Code conseguiu buscar seus sinalizadores de recurso. A primeira linha que corresponde à sua sessão se aplica. A tabela cobre sessões que você inicia em um terminal ou através da extensão VS Code; para o aplicativo de desktop e claude.ai, veja as abas Desktop e Web em [Alternar modos de permissão](#switch-permission-modes).

| Como você executa Claude Code                                                                                                                                                                                                                                  | Modo de permissão inicial integrado |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------- |
| Qualquer arquivo de configurações define `disableAutoMode` como `"disable"`                                                                                                                                                                                    | `default`                           |
| [Busca de sinalizador de recurso](/docs/pt/env-vars#features-that-need-feature-flag-fetching) está desativada                                                                                                                                                       | `default`                           |
| Sua [primeira sessão depois que você instala Claude Code ou faz upgrade](/docs/pt/env-vars#first-session-after-an-install-or-upgrade) para uma versão que adiciona este padrão, a menos que, após uma instalação limpa, Claude Code busque os sinalizadores a tempo | `default`                           |
| `claude -p` ou o [Agent SDK](/docs/pt/agent-sdk/permissions)                                                                                                                                                                                                        | `default`                           |
| Amazon Bedrock, Agent Platform do Google Cloud, Microsoft Foundry, [Claude Platform on AWS](/docs/pt/claude-platform-on-aws) ou uma sessão [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) conectada                                                       | `default`                           |
| Um plano Pro, Max ou Team, em um terminal ou através da [extensão VS Code](/docs/pt/vs-code)                                                                                                                                                                        | `auto`                              |
| Um plano Enterprise ou uma chave de API do Claude Console                                                                                                                                                                                                      | `default`                           |

Quando a busca de sinalizador de recurso está desativada, ou em uma [primeira sessão após uma instalação ou upgrade](/docs/pt/env-vars#first-session-after-an-install-or-upgrade) onde os sinalizadores ainda não chegaram, a extensão VS Code ignora todos os arquivos de configurações ao escolher o modo de permissão inicial.

Quando o sinalizador, um arquivo de configurações ou o padrão integrado seleciona `auto` mas o modo automático não está disponível para a sessão, Claude Code inicia a sessão no modo Manual em vez disso. O modo automático não está disponível quando a sessão não atende aos [requisitos de disponibilidade](#eliminate-prompts-with-auto-mode), como um arquivo de configurações desativando-o ou um modelo que não o suporta, ou quando Anthropic o desativou temporariamente no lado do servidor.

A primeira vez que o padrão integrado inicia uma de suas sessões em modo automático, Claude Code mostra um aviso que vincula a esta página:

* Em um terminal, uma vez, no topo da sessão
* Na extensão VS Code, como um cartão na tela de nova conversa que permanece até você descartá-lo

Nos planos Pro, Max e Team, se seu `~/.claude/settings.json` define um `defaultMode` diferente de `auto` e nenhum outro arquivo de configurações define um, suas sessões continuam iniciando nesse modo. Claude Code pergunta uma vez, no terminal ou na extensão VS Code, se você quer alterar a configuração para modo automático. Se você recusar, sua configuração permanece como está.

<h3 id="start-in-a-different-mode">
  Inicie em um modo de permissão diferente
</h3>

Você pode definir o modo de permissão inicial para uma sessão, ou como padrão para cada sessão em uma máquina, projeto ou organização. Quando mais de um arquivo de configurações define `permissions.defaultMode`, [precedência de configurações](/docs/pt/settings#settings-precedence) decide, então um valor de projeto ou gerenciado supera `~/.claude/settings.json`. Para alterar o modo de permissão de uma sessão já em execução, veja [Alternar modos de permissão](#switch-permission-modes).

| Para definir o modo de permissão inicial para         | Faça isto                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :---------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Uma sessão que você está prestes a iniciar            | Passe o modo de permissão como uma flag, por exemplo `claude --permission-mode default`                                                                                                                                                                                                                                                                                                                                      |
| Cada sessão de terminal que você inicia nesta máquina | Defina `permissions.defaultMode` em `~/.claude/settings.json`. Para o que a extensão VS Code lê, veja [Alternar modos de permissão](#switch-permission-modes)                                                                                                                                                                                                                                                                |
| Cada sessão de terminal que você inicia em um projeto | Defina `permissions.defaultMode` em `.claude/settings.json` do projeto. Sessões que você inicia em um terminal honram todos os valores exceto `auto` e `bypassPermissions`; sessões que a extensão VS Code inicia não leem configurações de projeto para o modo de permissão inicial                                                                                                                                         |
| Cada sessão de terminal em sua organização            | Defina `permissions.defaultMode` em [configurações gerenciadas](/docs/pt/managed-settings). Sessões de terminal iniciam nesse modo e as pessoas ainda podem alternar para modo automático; para o que a extensão VS Code lê, veja [Alternar modos de permissão](#switch-permission-modes). Para remover o modo automático para que ninguém possa selecioná-lo, defina `permissions.disableAutoMode` como `"disable"` em vez disso |

Este exemplo faz cada sessão de terminal em sua máquina iniciar no modo Manual, cujo valor de configuração é `default`. Salve em `~/.claude/settings.json`:

```json theme={null}
{
  "permissions": {
    "defaultMode": "default"
  }
}
```

A próxima sessão que você inicia mostra `⏸ manual mode on` na barra de status.

<h2 id="switch-permission-modes">
  Alternar modos de permissão
</h2>

Cada interface tem seu próprio controle para alternar modos de permissão durante uma sessão e sua própria forma de escolher o modo de permissão que novas sessões iniciam. Selecione sua interface para ver seus controles.

<Tabs>
  <Tab title="CLI">
    **Durante uma sessão**: pressione `Shift+Tab` para alternar modos de permissão. De `auto`, o primeiro pressionamento alterna para `default`, e o ciclo então executa `default` → `acceptEdits` → `plan` → de volta para `default`. Modos opcionais, descritos abaixo, se encaixam após `plan`. A barra de status mostra o modo ativo como um `⏸ manual mode on` cinza para `default`, ou como `⏵⏵ accept edits on`, `⏸ plan mode on`, `⏵⏵ auto mode on`, `⏵⏵ don't ask on` ou `⏵⏵ bypass permissions on`.

    Nem todo modo está no ciclo padrão:

    * `auto`: aparece quando [modo automático está disponível](#eliminate-prompts-with-auto-mode); alternar para ele alterna modos de permissão sem um prompt de confirmação
    * `bypassPermissions`: aparece depois que você inicia com `--permission-mode bypassPermissions`, `--dangerously-skip-permissions`, `--allow-dangerously-skip-permissions` ou `permissions.defaultMode: "bypassPermissions"` em [configurações de usuário, `--settings` ou gerenciadas](/docs/pt/settings-reference#permissions-defaultmode). A variante `--allow-` adiciona o modo de permissão ao ciclo sem ativá-lo
    * `dontAsk`: nunca aparece no ciclo; defina-o com `--permission-mode dontAsk`

    Modos opcionais habilitados se encaixam após `plan`, com `bypassPermissions` primeiro e `auto` por último. Se você tiver ambos habilitados, você alternará através de `bypassPermissions` a caminho de `auto`.

    **De um prompt de permissão Bash**: nos modos de permissão Manual e `acceptEdits`, quando [modo automático](#eliminate-prompts-with-auto-mode) está disponível, Claude Code adiciona **Sim, e alternar para modo automático** ao prompt de permissão de um comando Bash. Selecione-o para aprovar o comando e alternar a sessão para modo automático. Prompts da [ferramenta PowerShell](/docs/pt/tools-reference#powershell-tool) não oferecem a opção. Requer Claude Code v2.1.247 ou posterior.

    Claude Code não adiciona a opção a prompts forçados por uma de suas [regras `ask`](/docs/pt/permissions#manage-permissions) ou por um [hook](/docs/pt/hooks#pretooluse-decision-control), porque o modo automático ainda mostra esses prompts, então alternar não os removeria.

    **Na inicialização**: passe o modo de permissão como uma flag.

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **Como padrão**: defina `permissions.defaultMode` no escopo que você quer, conforme descrito em [Inicie em um modo de permissão diferente](#start-in-a-different-mode).

    A mesma flag `--permission-mode` funciona com `-p` para [execuções não-interativas](/docs/pt/headless).
  </Tab>

  <Tab title="VS Code">
    **Durante uma sessão**: clique no indicador de modo na parte inferior da caixa de prompt. Ele usa estes rótulos para os modos nesta página:

    | Rótulo da UI           | Modo                |
    | :--------------------- | :------------------ |
    | Manual                 | `default`           |
    | Editar automaticamente | `acceptEdits`       |
    | Plan                   | `plan`              |
    | Auto                   | `auto`              |
    | Bypass permissions     | `bypassPermissions` |

    **Como padrão**: para fixar o modo de permissão que conversas iniciam, defina `claudeCode.initialPermissionMode` nas configurações de usuário do VS Code como `default`, `manual`, `acceptEdits`, `plan` ou `bypassPermissions`. A configuração não aceita `auto`; para iniciar em Auto, deixe-a indefinida e escolha **Auto** no indicador de modo uma vez, como o item 2 abaixo descreve. A extensão inicia cada nova conversa no primeiro destes que se aplica:

    1. `claudeCode.initialPermissionMode`
    2. O modo que você escolheu por último no indicador de modo, se foi Manual, Editar automaticamente ou Auto. Escolher Plan ou Bypass permissions se aplica apenas a essa conversa
    3. `permissions.defaultMode` de [configurações gerenciadas](/docs/pt/managed-settings) ou `~/.claude/settings.json`, em planos Pro, Max e Team com [busca de sinalizador de recurso](#which-mode-a-session-starts-in) disponível
    4. O [padrão integrado](#which-mode-a-session-starts-in) para seu plano, provedor e configurações de organização

    A extensão nunca lê `.claude/settings.json` ou `.claude/settings.local.json` de um projeto para o modo de permissão inicial, e em conversas que não atendem às condições do item 3, ela não lê nenhum arquivo de configurações. Quando `claudeCode.claudeProcessWrapper` está definido, os itens 3 e 4 também não se aplicam: essas conversas iniciam em Manual a menos que o item 1 ou item 2 defina um modo de permissão.

    Auto aparece no indicador de modo quando [modo automático está disponível](#eliminate-prompts-with-auto-mode).

    Bypass permissions requer o toggle **Allow dangerously skip permissions** nas configurações da extensão. Sem ele, o modo de permissão não aparece no indicador, e um valor `bypassPermissions` do item 1 ou item 3 inicia a conversa em Manual em vez disso. Auto de qualquer item também inicia a conversa em Manual quando o modo automático não está disponível.

    Veja o [guia do VS Code](/docs/pt/vs-code) para detalhes específicos da extensão.
  </Tab>

  <Tab title="JetBrains">
    O plugin JetBrains executa Claude Code no terminal do IDE, então alternar modos de permissão funciona da mesma forma que na CLI: pressione `Shift+Tab` para alternar, ou passe `--permission-mode` ao iniciar.
  </Tab>

  <Tab title="Desktop">
    **Durante uma sessão**: na aba Code, use o seletor de modo ao lado do botão enviar. Nem todo modo aparece no seletor:

    * **Auto**: aparece quando [modo automático está disponível](#eliminate-prompts-with-auto-mode)
    * **Bypass permissions**: requer o toggle **Allow bypass permissions mode** nas configurações do Desktop em planos Pro e Max; em planos Team e Enterprise, a política da organização controla isso

    A aba Cowork não usa estes modos. Cowork tem seus próprios modos de permissão, habilitados separadamente, e a aba Cowork não mostra nenhum seletor de modo até que um modo além de seu padrão seja habilitado para sua conta. Veja a [documentação do Cowork](https://claude.com/docs/cowork/overview).

    Para detalhes específicos do desktop, veja [Escolha um modo de permissão](/docs/pt/desktop#choose-a-permission-mode) no guia do Desktop.

    **Como padrão**: defina `defaultMode` em [configurações](/docs/pt/settings#where-settings-live). O aplicativo de desktop lê os mesmos arquivos de configurações que a CLI e aplica o modo de permissão a novas sessões locais.

    Um modo que você escolhe no seletor de modo é lembrado por pasta e tem precedência sobre `defaultMode` para essa pasta. Plan é a exceção: escolhê-lo se aplica apenas à sessão atual.

    Para onde `defaultMode` vai em um arquivo de configurações, veja o exemplo em [Inicie em um modo de permissão diferente](#start-in-a-different-mode).
  </Tab>

  <Tab title="Web and mobile">
    Use o dropdown de modo ao lado da caixa de prompt em [claude.ai/code](https://claude.ai/code) ou no aplicativo móvel. Prompts de permissão aparecem no claude.ai para aprovação. Quais modos aparecem depende de onde a sessão é executada:

    * **[Sessões em nuvem](/docs/pt/claude-code-on-the-web)**: Accept edits, Plan e Auto. Accept edits corresponde ao modo `default`: sessões em nuvem pré-aprovam edições de arquivo independentemente do modo, então o dropdown mostra Accept edits em vez de Manual. Sessões em nuvem ainda honram `defaultMode: "acceptEdits"` de configurações. O modo Auto aparece apenas quando sua organização o permite e o modelo selecionado o suporta. Bypass permissions não está disponível.
    * **[Sessões de Remote Control](/docs/pt/remote-control)** em sua máquina local: Manual, Accept edits e Plan. Você não pode selecionar Auto ou Bypass permissions do aplicativo.
      * Exceto por Bypass permissions, o dropdown mostra o modo de permissão em que a sessão local está, incluindo um definido do terminal. Ele atualiza quando o modo de permissão muda no aplicativo ou no terminal. A sessão nunca relata Bypass permissions ao claude.ai, então alternar para ele do terminal não muda o que o dropdown mostra.
      * Sessões hospedadas pelo [aplicativo de desktop](/docs/pt/desktop) ou pela [extensão VS Code](/docs/pt/vs-code) relatam mudanças de modo de permissão ao claude.ai conforme acontecem, da mesma forma que sessões hospedadas em um terminal.
      * Antes de v2.1.202, sessões conectadas com `/remote-control` ou `claude --remote-control` não relatavam seu modo de permissão, então claude.ai e o aplicativo móvel poderiam mostrar um modo em que a sessão não estava. A incompatibilidade afetava apenas o rótulo. Claude Code gerou prompts de permissão a partir do modo de permissão real da sessão, e eles ainda apareciam no aplicativo para aprovação.

    Para Remote Control, a máquina local executando a sessão deve estar conectada com sua conta claude.ai; chaves de API não são suportadas. Você também pode definir o modo de permissão inicial ao iniciar essa sessão local:

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  Auto-aprovar edições de arquivo com modo acceptEdits
</h2>

O modo `acceptEdits` permite que Claude crie e edite arquivos em seu diretório de trabalho sem solicitar. A barra de status mostra `⏵⏵ accept edits on` enquanto este modo está ativo.

Além de edições de arquivo, o modo `acceptEdits` auto-aprova comandos Bash comuns do sistema de arquivos: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp` e `sed`. Esses comandos também são auto-aprovados quando prefixados com variáveis de ambiente seguras como `LANG=C` ou `NO_COLOR=1`, ou wrappers de processo como `timeout`, `nice` ou `nohup`. Como edições de arquivo, a auto-aprovação se aplica apenas a caminhos dentro de seu diretório de trabalho ou `additionalDirectories`. Caminhos fora desse escopo, gravações em [caminhos protegidos](#protected-paths), remoções `rm` e `rmdir` direcionadas a um [caminho crítico](#critical-paths) e todos os outros comandos Bash, exceto o [conjunto integrado somente leitura](/docs/pt/permissions#read-only-commands), ainda solicitam.

Quando a [ferramenta PowerShell](/docs/pt/tools-reference#powershell-tool) está habilitada, o modo `acceptEdits` também auto-aprova `Set-Content`, `Add-Content`, `Clear-Content` e `Remove-Item` em caminhos no escopo, junto com seus aliases comuns. As mesmas regras de escopo e caminho protegido se aplicam, e `Remove-Item` recebe [sua própria verificação](#remove-item-in-powershell). Um argumento posicional que contém um caractere de aspas, como o apóstrofo em `Set-Content .\notes.txt "It's done"`, ainda solicita mesmo em caminhos no escopo, porque Claude Code não consegue validar estaticamente um argumento cujas leituras entre aspas e sem aspas diferem. Passe o conteúdo através de um parâmetro nomeado como `-Value` para evitar o prompt.

Use `acceptEdits` quando você quer revisar mudanças em seu editor ou via `git diff` depois, em vez de aprovar cada edição inline.

Pressione `Shift+Tab` uma vez a partir do modo Manual para entrar nele, ou comece com ele diretamente:

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  Analise antes de editar com modo plan
</h2>

O modo plan diz a Claude para pesquisar e propor mudanças sem realizá-las. Claude lê arquivos, executa comandos shell para explorar e escreve um plano, mas não edita sua fonte. Exceto em sessões com [permissões de bypass disponíveis](#skip-all-checks-with-bypasspermissions-mode), edições permanecem bloqueadas até que você aprove o plano.

Quando [modo automático](/docs/pt/auto-mode-config) está disponível e a configuração `useAutoModeDuringPlan` está ativada, que é o padrão, o classificador revisa comandos shell durante o planejamento em vez de solicitar você. Comandos aprovados são executados, e os rejeitados são bloqueados. Caso contrário, comandos fora do [conjunto integrado somente leitura](/docs/pt/permissions#read-only-commands) solicitam aprovação, incluindo quando o [modo auto-allow](/docs/pt/sandboxing#sandbox-modes) do sandbox está habilitado. Em sessões com permissões de bypass disponíveis, nem o classificador nem um prompt se aplica a comandos de planejamento; [Ignorar todas as verificações com modo bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) cobre as poucas coisas que ainda solicitam lá. Em v2.1.212 até v2.1.217, sessões sem permissões de bypass solicitavam para cada comando fora do conjunto somente leitura, independentemente de o modo automático estar disponível.

Entre no modo plan pressionando `Shift+Tab` ou prefixando um único prompt com `/plan`. Você também pode iniciar no modo plan a partir da CLI:

```bash theme={null}
claude --permission-mode plan
```

Pressione `Shift+Tab` novamente para sair do modo plan sem aprovar um plano.

<h3 id="review-and-approve-a-plan">
  Revise e aprove um plano
</h3>

Quando o plano estiver pronto, Claude o apresenta e pergunta como proceder. A partir desse prompt você pode escolher:

* **Sim, e usar modo automático**: aprove e inicie em [modo automático](#eliminate-prompts-with-auto-mode). Se o modo automático não está [disponível para sua sessão](#eliminate-prompts-with-auto-mode), por exemplo porque sua organização o desativou, esta opção lê **Sim, auto-aceitar edições**. Se você iniciou a sessão com permissões de bypass habilitadas, a opção lê **Sim, e alternar para BYPASS PERMISSIONS (sem prompts adicionais) para esta sessão** em vez disso.
* **Sim, aprovar edições manualmente**: aprove e revise cada edição individualmente.
* **Não, continuar planejando**: permaneça no modo plan e diga a Claude o que alterar.

Aprovar um plano sai do modo plan e alterna a sessão para o modo de permissão que cada opção de aprovação descreve, então Claude começa a editar. Para planejar novamente, volte ao modo plan com `Shift+Tab`, ou prefixe seu próximo prompt com `/plan`.

Pressione `Ctrl+G` para abrir o plano proposto em seu editor de texto padrão e editá-lo diretamente antes de Claude prosseguir. Quando [`showClearContextOnPlanAccept`](/docs/pt/settings-reference#showclearcontextonplanaccept) está habilitado, a lista ganha uma primeira opção que aprova o plano e limpa o contexto de planejamento.

Aceitar um plano também dá à sessão um [título gerado](/docs/pt/sessions#name-your-sessions) baseado no plano, a menos que você já tenha nomeado a sessão.

<h3 id="set-plan-mode-as-the-default">
  Defina modo plan como padrão
</h3>

Para tornar o modo plan o padrão para sessões de terminal de um projeto, defina `defaultMode` como `plan` em `.claude/settings.json`, colocado conforme o exemplo em [Inicie em um modo de permissão diferente](#start-in-a-different-mode) mostra. Conversas que a [extensão VS Code](/docs/pt/vs-code) inicia não leem configurações de projeto para o modo de permissão inicial. Lá, defina `claudeCode.initialPermissionMode` como `plan` em suas configurações de usuário do VS Code em vez disso.

<h2 id="eliminate-prompts-with-auto-mode">
  Elimine prompts de permissão com modo automático
</h2>

O modo automático permite que Claude execute sem prompts de permissão rotineiros. Um modelo classificador separado revisa as ações antes de serem executadas, bloqueando qualquer coisa que ultrapasse sua solicitação, tenha como alvo infraestrutura não reconhecida ou pareça impulsionada por conteúdo hostil que Claude leu. As [regras de solicitação](/docs/pt/permissions#manage-permissions) explícitas ainda forçam um prompt.

Nos planos Pro, Max e Team, o modo automático é o [modo de permissão inicial integrado](#which-mode-a-session-starts-in).

O classificador também revisa cada mensagem que Claude envia para outro agente com [`SendMessage`](/docs/pt/tools-reference), seja texto simples ou uma mensagem estruturada de [equipe de agentes](/docs/pt/agent-teams), antes que Claude Code a entregue, tanto no modo automático quanto no [modo de plano enquanto o classificador revisa comandos](#analyze-before-you-edit-with-plan-mode); a revisão de envio requer Claude Code v2.1.222 ou posterior.

O classificador também revisa e aprova ou bloqueia remoções `rm` e `rmdir` direcionadas a um [caminho crítico](#critical-paths), como `rm -rf /` e `rm -rf ~`, inclusive quando a remoção está dentro de substituição de comando ou processo.

O modo automático também incentiva Claude a continuar trabalhando sem parar para fazer perguntas de esclarecimento, embora Claude ainda pergunte quando sua solicitação ou uma skill depende explicitamente disso. Para um comportamento mais autônomo em um modo que ainda o solicita, defina o [estilo de saída Proativo](/docs/pt/output-styles).

<Warning>
  O modo automático reduz prompts de permissão, mas não garante segurança. Use-o para tarefas em que você confia na direção geral, não como substituto para revisão em operações sensíveis.
</Warning>

O modo automático está disponível apenas quando sua conta atende a todos esses requisitos:

* **Plano**: Todos os planos.
* **Organização**: no Team e Enterprise, o modo automático está disponível por padrão. Os administradores podem desativá-lo para a organização definindo `permissions.disableAutoMode` como `"disable"` em [configurações gerenciadas](/docs/pt/managed-settings).
* **Modelo**: na API Anthropic e [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), Claude Opus 4.6 ou posterior, Sonnet 4.6 ou posterior, ou um [modelo Fable](/docs/pt/model-config#work-with-fable). No Amazon Bedrock, na Agent Platform do Google Cloud, no Microsoft Foundry e em sessões [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) conectadas, apenas Claude Sonnet 5, Opus 4.7 ou posterior e os modelos Fable. Modelos mais antigos, incluindo Sonnet 4.5, Opus 4.5, Haiku e modelos claude-3, não são suportados em nenhum provedor.
* **Provedor**: disponível por padrão na API Anthropic, Claude Platform on AWS, Amazon Bedrock, Agent Platform do Google Cloud, Microsoft Foundry e sessões gateway de aplicativos Claude conectadas.

Se Claude Code relatar o modo automático como indisponível, primeiro verifique esses requisitos e se algum arquivo de configurações define [`disableAutoMode`](/docs/pt/settings-reference#disableautomode). A Anthropic também pode ter desativado o modo automático no servidor, ou o servidor pode ter rejeitado o modo automático para sua conta. Uma sessão que recebeu qualquer uma das respostas mantém o modo automático desativado até o final da sessão, portanto, inicie uma nova sessão depois.

Uma mensagem separada que nomeia um modelo e diz que o modo automático "não pode determinar a segurança" de uma ação significa que uma solicitação do classificador falhou. Essa falha geralmente é transitória, mas no Amazon Bedrock pode se repetir até que sua conta possa invocar o modelo nomeado. Consulte a [referência de erros](/docs/pt/errors#auto-mode-cannot-determine-the-safety-of-an-action) para as causas e o que fazer.

Se você definir `defaultMode: "auto"` em [configurações](/docs/pt/settings-reference#all-settings) e uma sessão de terminal iniciar no modo Manual sem erro, a configuração provavelmente está em `.claude/settings.json` ou `.claude/settings.local.json`. `auto` não entra em vigor nesses arquivos. Mova-o para `~/.claude/settings.json`. Para uma conversa que a extensão VS Code iniciou, verifique a lista própria da extensão em [Alternar modos de permissão](#switch-permission-modes).

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Modo automático no Bedrock, Agent Platform ou Foundry
</h3>

No [Amazon Bedrock](/docs/pt/amazon-bedrock), [Agent Platform do Google Cloud](/docs/pt/google-vertex-ai), [Microsoft Foundry](/docs/pt/microsoft-foundry) e sessões [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) conectadas, o modo automático aparece no ciclo `Shift+Tab` por padrão. Aparecer no ciclo não altera o modo de permissão em que uma sessão inicia: nesses provedores, sessões de terminal iniciam em seu [`defaultMode`](/docs/pt/settings-reference#permissions-defaultmode), que é Manual a menos que você o altere, e conversas na [extensão VS Code](/docs/pt/vs-code) iniciam em Manual a menos que `claudeCode.initialPermissionMode` ou um modo que você escolheu na extensão defina um. Apenas Claude Sonnet 5, Opus 4.7 ou posterior e os modelos Fable são suportados nesses provedores.

Para tornar o modo automático o modo de permissão inicial padrão, defina `"permissions": {"defaultMode": "auto"}` em configurações de usuário ou gerenciadas. Em sessões que a extensão VS Code inicia, selecione **Auto** no indicador de modo. [Alternar modos de permissão](#switch-permission-modes) cobre o que supera essa escolha.

O checkup [`/doctor`](/docs/pt/commands#all-commands) propõe esse padrão de configurações de usuário nesses provedores da mesma forma que faz na API Anthropic.

Para impedir que desenvolvedores usem o modo automático, defina `disableAutoMode` como `"disable"` em [configurações gerenciadas](/docs/pt/managed-settings). Isso remove `auto` do ciclo `Shift+Tab`, e uma sessão iniciada com `--permission-mode auto` inicia em Manual. Uma sessão já em execução no modo automático o deixa quando a configuração chega a essa sessão de uma [fonte implantada por administrador](/docs/pt/managed-settings#which-managed-source-claude-code-uses) e mostra `auto mode disabled by settings`. Antes da v2.1.251, uma sessão em execução mantinha o modo automático até o final.

Na v2.1.158 até v2.1.206, o modo automático estava desativado nesses provedores até você definir `CLAUDE_CODE_ENABLE_AUTO_MODE=1`, e Claude Code ignorava `defaultMode: "auto"` nesses provedores a menos que a variável também fosse definida. A variável ainda é aceita para compatibilidade e não tem efeito a partir da v2.1.207.

<h3 id="server-side-classifier-review">
  Revisão do classificador no servidor
</h3>

Em modo automático, Claude Code pode pedir ao servidor para verificar as ações que [a ordem de decisão](#how-the-classifier-evaluates-actions) envia para revisão, como parte das solicitações de modelo da sessão, em vez de enviar suas próprias solicitações do classificador. Essas sessões pedem:

* **Uma conexão direta com a API Anthropic**: em uma sessão de terminal interativa, em todos os planos claude.ai e em contas que usam a API Claude, conforme Anthropic o implementa. Requer Claude Code v2.1.271 ou posterior nos planos Pro, Max e Team, e v2.1.278 ou posterior nos planos Enterprise e contas da API Claude. A partir da v2.1.282, uma sessão que [não busca sinalizadores de recurso](/docs/pt/env-vars#features-that-need-feature-flag-fetching), por exemplo porque você desativou a telemetria, pede ao servidor por padrão em qualquer tipo de sessão.
* **Um provedor de nuvem, ou um gateway LLM ou proxy**: no [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), Amazon Bedrock, Agent Platform do Google Cloud e Microsoft Foundry, e sempre que você aponta `ANTHROPIC_BASE_URL` para um [gateway LLM ou proxy](/docs/pt/llm-gateway), qualquer que seja seu plano. Pedir por padrão requer Claude Code v2.1.278 ou posterior.
* **Uma sessão [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) conectada**: requer Claude Code v2.1.280 ou posterior

Onde o servidor revisa as ações, seus vereditos decidem-nas. Dois outros resultados são possíveis:

* **O servidor não revisa a sessão**: uma resposta é concluída sem resultados de revisão, ou o servidor responde que não revisa essa sessão. As causas mais comuns são um gateway LLM ou proxy que descarta a solicitação de revisão ou os resultados, e uma plataforma, região ou credencial que ainda não tem verificações no servidor. Claude Code volta para suas próprias solicitações do classificador. Uma vez que esse fallback se mantém pelo resto da sessão, mostra um [aviso sobre cobranças de solicitação do classificador](/docs/pt/auto-mode-classifier-billing) em contas onde essas solicitações são cobradas.
* **O servidor não dá veredito para uma ação**: Claude Code nega a ação em vez de executá-la sem revisão. Em qualquer conexão, isso acontece quando a resposta termina antes dos resultados de revisão chegarem ou os resultados chegam em uma forma que Claude Code não consegue ler. Um gateway LLM ou proxy que corta respostas ou reescreve os resultados pode causar qualquer um. Em uma conexão direta com a API Anthropic, também acontece quando a verificação do servidor falha para a ação, por exemplo ao expirar. [O servidor não retornou veredito de segurança](/docs/pt/errors#the-server-returned-no-safety-verdict) cobre a mensagem de negação, o que acontece quando negações se repetem e o que fazer.

Para pular a solicitação ao servidor e sempre usar as próprias solicitações do classificador de Claude Code, defina [`CLAUDE_CODE_AUTO_MODE_SERVER=0`](/docs/pt/env-vars). Em uma conexão direta com a API Anthropic, a variável requer Claude Code v2.1.281 ou posterior. Defini-la como `1` lá ativa a revisão do servidor em uma sessão que não a tem ainda, como uma sessão `-p` ou Agent SDK, a menos que você também tenha definido `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`. Se você definir `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` e deixar `CLAUDE_CODE_AUTO_MODE_SERVER` indefinido, Claude Code também para de pedir ao servidor.

<h3 id="what-the-classifier-blocks-by-default">
  O que o classificador bloqueia por padrão
</h3>

O classificador confia em seu diretório de trabalho e nos remotos que foram configurados para ele quando a sessão iniciou. Um remoto adicionado ou redirecionado durante a sessão com `git remote add` ou `git remote set-url` não é confiável, e tudo o mais é tratado como externo até você [configurar infraestrutura confiável](/docs/pt/auto-mode-config). Antes da v2.1.200, remotos adicionados no meio da sessão também eram confiáveis.

**Bloqueado por padrão**:

* Baixar e executar código, como `curl | bash`
* Enviar dados sensíveis para endpoints externos
* Implantações e migrações de produção
* Exclusão em massa no armazenamento em nuvem
* Concessão de permissões IAM ou repositório
* Modificação de infraestrutura compartilhada
* Destruição irreversível de arquivos que existiam antes da sessão
* Force push
* Fazer commit ou fazer push de uma alteração que enviaria segredos ou dados sensíveis para fora do repositório quando executado, ou ampliar o que uma implantação expõe. Isso cobre um fluxo de trabalho CI ou configuração de implantação que passa um segredo para um destino que ainda não o recebe, um script ou etapa de configuração que lê um armazenamento de segredos e envia os dados para fora, e uma alteração de configuração que amplia o que uma implantação publica, como um registro, visibilidade, artefato ou configuração de sourcemap. A verificação se aplica em qualquer branch, se aplica mesmo quando o repositório é público e dispara quando a alteração é feita commit ou push, independentemente de esse commit ou push disparar o pipeline; limpá-la requer nomear o efeito de execução, não apenas o commit ou push. Antes da v2.1.211, essa verificação era limitada ao branch padrão: um push lá era bloqueado quando carregava conteúdo sensível, alterações encobertas ou mal descritas em relação ao que você pediu, conteúdo portado de fora do repositório ou roteado em torno de uma revisão que você pediu
* `git reset --hard`, `git checkout -- .`, `git restore .`, `git clean -fd`, `git stash drop` ou `git stash clear`, que o classificador presume descartaria alterações não confirmadas
* `git commit --amend` quando o commit no HEAD não foi criado nesta sessão
* A partir da v2.1.198, `git commit --amend` quando o commit no HEAD já foi feito push. Uma reword apenas de mensagem não é bloqueada: `--amend -m` sem nada recém-preparado, em um commit que Claude criou durante esta sessão
* `terraform destroy`, `pulumi destroy`, `cdk destroy` ou `terragrunt destroy`, e aplicar um plano que destrói recursos

Claude Code v2.1.195 e posterior bloqueiam mais categorias por padrão. Várias dependem de entradas de [ambiente](/docs/pt/auto-mode-config#define-trusted-infrastructure), como destinos remotos sensíveis e escopos IaC protegidos, que você pode restringir a nomes concretos.

* Escrever em um gerenciador de segredos, ou alterar registros DNS ou certificados TLS
* Mesclar uma solicitação de pull que nenhum humano aprovou, aprovar a própria solicitação de pull de Claude ou desabilitar verificações CI
* Postar um comentário que é em si um comando para automação, como `atlantis apply` ou `/deploy` ou `/merge` de um bot
* Alternar, ramificar ou excluir um sinalizador de recurso de produção
* Aplicar alterações de infraestrutura a um escopo IaC protegido, ou drenar e remover nós de cluster
* Gravações em um cluster de computação compartilhado que vão além do recurso que você nomeou, como um seletor de rótulo ou `--all` que captura trabalhos de outros usuários
* Criar recursos Kubernetes que executam em cada nó ou interceptam tráfego de cluster, como DaemonSets e webhooks de admissão
* Shells interativos ou port-forwards para um destino remoto sensível
* Abrir um túnel ou shell reverso que torna um serviço local acessível da internet pública
* Imprimir uma credencial ou token ao vivo na transcrição ou em um arquivo
* Acessar um local listado como local de dados sensíveis em seu [ambiente](/docs/pt/auto-mode-config#define-trusted-infrastructure), ou copiar dados de um. A partir da v2.1.198, isso também bloqueia enviar dados de um para um público que a entrada exclui
* Rotear uma instalação de pacote em torno de seu registro de pacotes interno para um registro público. A partir da v2.1.198, isso também se aplica quando você disse a Claude que um registro interno ou espelho existe na conversa, não apenas quando um está listado em seu ambiente
* Executar um comando com um sinalizador que desativa uma proteção de segurança, como `--insecure`
* Iniciar um loop de agente autônomo que executa sem aprovação humana ou sandbox, como um iniciado com `--dangerously-skip-permissions` ou `--no-sandbox`. A partir da v2.1.198, isso também cobre executar um agente de terceiros ou harness de avaliação com isolamento e aprovação por ação desabilitados, como um runner iniciado com `--yes-always`
* Ações do [Claude no Chrome](/docs/pt/chrome) que poderiam enviar conteúdo da página, cookies ou credenciais fora da origem

Claude Code v2.1.198 e posterior também bloqueiam estes por padrão:

* Excluir arquivos em `/tmp`, `$TMPDIR` ou outro diretório compartilhado de rascunho ou cache por wildcard, glob ou filtro de idade em vez de por um caminho nomeado específico
* Incluir detalhes sensíveis em conteúdo enviado, carregado, publicado ou escrito para outras pessoas ou sistemas compartilhados, quando sua própria mensagem não autorizou esses detalhes para esse destinatário. Corpos de PR e issue, mensagens de commit e comentários contam como esse tipo de conteúdo de saída quando o repositório está fora do limite de confiança ou é público, incluindo repositórios públicos de sua própria organização; caminhos de arquivo internos, nomes de código, dados de resposta de API ao vivo, como emails ou identificadores de conta, e identificadores de infraestrutura contam como detalhes sensíveis. O escopo de PR, issue e mensagem de commit requer Claude Code v2.1.200 ou posterior. Dados pessoais ao vivo de uma resposta de API em um corpo de PR ou issue, como um endereço de email, um identificador de conta ou organização, ou uma métrica de uso, requer que você nomeie esses detalhes e o destinatário independentemente da visibilidade ou limite de confiança do repositório. Essa verificação requer Claude Code v2.1.203 ou posterior
* Enviar pressionamentos de tecla para o próprio painel tmux de Claude Code para conduzir sua própria interface, que o classificador trata como Claude alterando suas próprias permissões ou supervisão

Claude Code v2.1.200 e posterior também bloqueiam estes por padrão:

* Comentar, excluir ou forçar a aprovação de um teste ou asserção que protege comportamento de segurança, como autenticação, controle de acesso, validação de entrada ou sandboxing
* Excluir ou desmontar um recurso com estado que Claude não criou na sessão, quando nenhuma regra de exclusão mais específica se aplica e você não nomeou esse recurso
* Redirecionar uma URL de base de API, endpoint de proxy, receptor de webhook ou espelho de registro para um host de terceiros que não se encaixa na tarefa, inclusive em arquivos de exemplo como `.env.example`
* Alterar para onde os pushes vão com `git remote set-url` ou `git remote add`, a menos que você tenha nomeado o novo remoto
* Fazer push de segredos ou dados pessoais ou confiados para um repositório conhecido como público, ou fazer push de material confidencial lá que não faz parte do próprio trabalho desse repositório. O próprio assunto de um repositório de dotfiles é a única exceção para dados pessoais ou confiados, e conteúdo de um repositório privado chegando a qualquer superfície pública é bloqueado da mesma forma; ambos os refinamentos requerem Claude Code v2.1.203 ou posterior. Antes da v2.1.203, dados pessoais eram agrupados com material confidencial e bloqueados apenas quando não faziam parte do próprio trabalho desse repositório. Quando a visibilidade de um repositório não é estabelecida, o classificador não bloqueia apenas nisso; ele julga o conteúdo contra as outras regras
* Abrir uma solicitação de pull contra um repositório ou organização diferente, fazer fork com `gh repo fork` ou fazer push para um repositório de terceiros, a menos que você tenha nomeado esse alvo externo

Claude Code v2.1.203 e posterior também bloqueiam estes por padrão:

* Conteúdo de um armazenamento local sensível, ou de um arquivo cujo nome, caminho ou tipo o marca como sensível, entrando em um commit, um push, texto de PR ou issue, um gist ou paste, ou uma publicação de pacote, a menos que você tenha nomeado tanto a origem quanto o destino. Transcrições de sessão e logs de conversa, pastas de ponto de credencial e configuração como chaves SSH, credenciais em nuvem, perfis de navegador e histórico de shell, e exportações de dados de usuário contam, e o repositório ser privado não o limpa

Claude Code v2.1.205 e posterior também bloqueiam estes por padrão:

* Escrever em transcrições de sessão de Claude Code, os arquivos de histórico `.jsonl` em `~/.claude/projects/` ou seu diretório de configuração configurado, seja diretamente ou através de um comando de shell. A regra também cobre as linhas de metadados que Claude Code acrescenta a cada entrada de transcrição para suas próprias verificações. Ler uma transcrição não é bloqueado
* Uma exclusão forçada recursiva como `rm -rf "$VAR"` ou `Remove-Item -Recurse -Force $dir` cujo alvo é uma variável de shell, ou um glob enraizado em uma, que não é atribuído em nenhum lugar na conversa que o classificador vê. O valor veio apenas da saída de comando anterior, que o classificador nunca recebe, portanto o classificador não pode verificar o alvo de exclusão contra as outras regras de exclusão. O bloqueio se limpa quando você nomeia o caminho exato sendo excluído, ou quando Claude re-executa a exclusão com o caminho literal resolvido escrito no comando. Exclusões cujo alvo o classificador pode resolver não são afetadas. Alvos `Remove-Item` que são um `*` simples ou terminam em `/*` ou `\*` nunca chegam ao classificador: Claude Code [nega-os imediatamente](#remove-item-in-powershell)

Claude Code v2.1.257 e posterior também bloqueiam estes por padrão:

* Solicitar credenciais do endpoint de metadados da instância em nuvem, como `169.254.169.254`, ou autenticar explicitamente uma chamada de nuvem, cluster ou registro com a identidade de conta de serviço ou nó da máquina
* Alcançar um host público por uma rota diferente de uma solicitação direta, como um túnel, um shell reverso, ou uma configuração de resolvedor ou proxy reescrita para apontar para fora
* Ler credenciais que pertencem ao host em vez de à sua tarefa, como certificados de nó ou auth de registro de contêiner do nó
* Conectar a ou escanear contêineres, pods ou VMs irmãos que Claude não iniciou, ou o nó sob o contêiner

Se Claude Code executar em algum lugar que se destine a permitir um desses, descreva essa configuração em uma entrada [Host containment](/docs/pt/auto-mode-config#define-trusted-infrastructure) em `autoMode.environment`.

Claude Code v2.1.261 e posterior também bloqueiam estes por padrão:

* Postar ou escrever um link para um serviço público de paste, diagrama ou compartilhamento de dados em uma mensagem, texto de PR ou issue, um documento, ou em qualquer outro lugar onde o link será aberto ou buscado, quando a própria URL carrega o conteúdo sendo compartilhado, a menos que você tenha nomeado esse serviço

**Permitido por padrão**:

* Operações de arquivo local em seu diretório de trabalho
* Instalação de dependências declaradas em seus arquivos de lock ou manifestos
* Leitura de `.env` e envio de credenciais para sua API correspondente
* Solicitações HTTP somente leitura
* Fazer push para qualquer branch do repositório em que você está trabalhando, incluindo o branch padrão. Um branch não padrão cujo nome o marca como alvo de implantação ou publicação, como `production` ou `gh-pages`, não é coberto: o classificador julga um push lá em seus próprios termos. O conteúdo do push ainda é verificado contra as outras regras, regras [`permissions.deny`](/docs/pt/permissions#manage-permissions) ainda podem bloquear comandos push [conforme escrito](/docs/pt/permissions#bash-rule-limits) em todos os modos, e a proteção de branch própria do remoto ainda se aplica. Antes da v2.1.211, apenas pushes para o branch em que você iniciou, branches que Claude criou e pushes rotineiros para o branch padrão eram permitidos por padrão, e antes da v2.1.203 qualquer push direto para o branch padrão era bloqueado

Claude Code v2.1.195 e posterior também permitem estes por padrão:

* Excluir os trabalhos exatos que Claude criou anteriormente na mesma sessão
* Ler, revisar ou escrever código relacionado à segurança, configs e modelos de ameaça como parte de sua tarefa
* Mensagens entre agentes trabalhando juntos na mesma sessão multi-agente
* Enviar dados para os domínios confiáveis, buckets e serviços que você lista em [`environment`](/docs/pt/auto-mode-config#define-trusted-infrastructure). Isso cobre apenas fluxo de dados, não operações destrutivas ou de credencial na mesma infraestrutura
* [Claude no Chrome](/docs/pt/chrome) navegação para um domínio interno confiável, localhost ou uma URL que você nomeou

Comandos em sandbox não obtêm acesso à rede por padrão. Claude nomeia os hosts que um comando precisa no próprio comando, o classificador os revisa com o comando, e uma lista aprovada abre esses hosts apenas para esse comando. [Domínios permitidos por comando](/docs/pt/sandboxing#per-command-allowed-domains-in-auto-mode) cobre o que uma lista pode e não pode abrir e o que acontece quando um comando alcança um host não listado.

Execute `claude auto-mode defaults` para imprimir as listas de regras completas como JSON. Se ações rotineiras forem bloqueadas, um administrador pode adicionar repositórios, buckets e serviços confiáveis via configuração `autoMode.environment`: consulte [Configurar modo automático](/docs/pt/auto-mode-config).

Fazer push para qualquer branch do repositório em que você está trabalhando e criar uma solicitação de pull que corresponda à sua solicitação executam sem um prompt, a menos que o push ou solicitação de pull se enquadre na [lista bloqueada](#what-the-classifier-blocks-by-default), como segredos ou dados sensíveis saindo do repositório, ou uma solicitação de pull que tenha como alvo um repositório ou organização diferente. Para exigir um checkpoint humano antes desses comandos enquanto permanece no modo automático, adicione regras `permissions.ask`, que correspondem ao comando [conforme escrito](/docs/pt/permissions#bash-rule-limits): consulte [Limites comuns](/docs/pt/auto-mode-config#common-boundaries).

<h3 id="first-read-outside-the-working-directories">
  A primeira leitura fora dos diretórios de trabalho
</h3>

Enquanto [`permissions.blockReadsOutsideWorkingDirectories`](/docs/pt/settings-reference#permissions-blockreadsoutsideworkingdirectories) está desativado, leituras de arquivo executam sem um prompt no modo automático, incluindo leituras fora dos [diretórios de trabalho](/docs/pt/permissions#working-directories). A primeira vez que Claude usa a ferramenta Read, Grep ou Glob em um caminho fora deles, Claude Code pergunta se você deseja continuar permitindo essas leituras.

O prompt não aparece em execuções `-p` não interativas ou sessões em segundo plano; leituras lá executam como antes.

Qualquer que seja sua resposta, Claude continua trabalhando:

* **Continuar permitindo**: a leitura é executada, leituras posteriores fora dos diretórios de trabalho executam como antes, e Claude Code registra sua resposta para que o prompt não apareça novamente
* **Bloquear a partir de agora**: a leitura é recusada, e Claude Code define [`permissions.blockReadsOutsideWorkingDirectories`](/docs/pt/settings-reference#permissions-blockreadsoutsideworkingdirectories) como `true` em suas configurações de usuário, o que faz as ferramentas de arquivo recusarem essas leituras em todas as sessões posteriores e em todos os modos de permissão. Para deixar Claude ler esse caminho depois, adicione seu diretório com `/add-dir` ou remova a configuração.
* **Perguntar novamente na próxima vez**: a leitura é recusada, e a próxima leitura fora dos diretórios de trabalho solicita novamente

<h3 id="boundaries-you-state-in-conversation">
  Limites que você declara na conversa
</h3>

O classificador trata limites que você declara na conversa como um sinal de bloqueio. Se você disser a Claude "não faça push" ou "aguarde até eu revisar antes de implantar", o classificador bloqueia ações correspondentes mesmo quando as regras padrão as permitiriam. Um limite permanece em vigor até você levantá-lo em uma mensagem posterior. O próprio julgamento de Claude de que uma condição foi atendida não o levanta.

Limites não são armazenados como regras. O classificador os relê da transcrição em cada verificação, portanto um limite pode ser perdido se [compactação de contexto](/docs/pt/costs#reduce-token-usage) remover a mensagem que o declarou. Para uma garantia firme, adicione uma [regra de negação](/docs/pt/permissions#permission-rule-syntax).

<h3 id="approvals-you-state-in-conversation">
  Aprovações que você declara na conversa
</h3>

Se você disser a Claude que uma ação bloqueada é permitida, o classificador lê isso como sua aprovação e pode limpar o bloqueio. Como você o expressou decide se a ação é executada e até onde a aprovação chega:

* **Nomeie a ação e seus detalhes**: sua mensagem tem que nomear a ação e a coisa específica que a torna perigosa, como o branch de um force push. Nomear apenas o verbo não limpa nada, portanto "você pode fazer force-push" deixa o bloqueio em vigor.
* **Espere que cubra uma ação**: uma aprovação cobre a ação destrutiva que você nomeou, portanto uma ação posterior é bloqueada novamente a menos que você tenha concedido a aprovação como permanente. Para parar de aprovar um padrão rotineiro uma ação por vez, adicione-o a [`autoMode.allow`](/docs/pt/auto-mode-config#override-the-block-and-allow-rules).
* **Alguns bloqueios permanecem em vigor**: [a ordem de precedência do classificador](/docs/pt/auto-mode-config#override-the-block-and-allow-rules) estabelece quais bloqueios sua aprovação pode alcançar. Para executar uma etapa que não limpará, [saia do modo automático](#switch-permission-modes) e responda ao prompt de permissão.

<h3 id="when-auto-mode-falls-back">
  Quando o modo automático volta
</h3>

Quando o modo automático não pode aprovar as ações de sua sessão, o que acontece depende do caso:

* **Uma ação bloqueada**: Claude Code mostra uma notificação e lista a ação em `/permissions` na aba **Recently denied**, onde você pode pressionar `r` para tentar novamente com uma aprovação manual. Quando o classificador produz [nenhum veredito sobre a ação](/docs/pt/errors#auto-mode-cannot-determine-the-safety-of-an-action), porque uma verificação de segurança separada do modo automático recusou a própria solicitação do classificador ou sua resposta não foi analisada, Claude Code nega a ação sem a notificação ou a entrada **Recently denied**.
* **Bloqueios repetidos**: se o classificador bloqueia uma ação 3 vezes seguidas ou 20 vezes no total, o modo automático pausa e Claude Code retoma a solicitação. Aprovar a ação solicitada retoma o modo automático. Esses limites não são configuráveis. Qualquer ação permitida redefine o contador consecutivo, enquanto o contador total persiste para a sessão e redefine apenas quando seu próprio limite dispara um fallback. Claude Code não conta uma negação para nenhum limite quando [uma verificação de segurança separada do modo automático recusa a solicitação do classificador](/docs/pt/errors#auto-mode-cannot-determine-the-safety-of-an-action); a entrada vinculada cobre como Claude Code lida com essas negações.
* **Sessões que não podem solicitar**: uma execução `-p` [não interativa](/docs/pt/headless) sem um [`--permission-prompt-tool`](/docs/pt/cli-reference#cli-flags) não tem prompt para voltar. Quando bloqueios repetidos atingem um limite, a ação não é executada e Claude continua trabalhando. O mesmo se aplica quando [uma verificação de segurança separada do modo automático recusa a solicitação do classificador](/docs/pt/errors#auto-mode-cannot-determine-the-safety-of-an-action). Claude Code não para a execução em nenhum dos casos.
* **Nenhum veredito do servidor**: sob [revisão do classificador no servidor](#server-side-classifier-review), Claude Code nega uma ação para a qual o servidor não dá veredito, e para a volta após dez respostas seguidas sem veredito. Consulte [O servidor não retornou veredito de segurança](/docs/pt/errors#the-server-returned-no-safety-verdict).
* **Uma mudança de modo durante uma verificação**: se você alternar modos de permissão enquanto uma verificação do classificador está pendente, Claude Code descarta um veredito que o novo modo não teria solicitado em vez de aplicá-lo: você é solicitado para aprovação, ou a ação é auto-negada no [modo `dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode).

Bloqueios repetidos geralmente significam que o classificador está perdendo contexto sobre sua infraestrutura. Use `/feedback` para relatar falsos positivos, ou peça a um administrador para [configurar infraestrutura confiável](/docs/pt/auto-mode-config).

<span id="how-the-classifier-evaluates-actions" />

<AccordionGroup>
  <Accordion title="Como o classificador avalia ações">
    Cada ação passa por uma ordem de decisão fixa. O primeiro passo correspondente vence:

    1. Ações que correspondem a suas [regras de permitir, solicitar ou negar](/docs/pt/permissions#manage-permissions) resolvem imediatamente, com essas exceções:
       * Gravações em [caminhos protegidos](#protected-paths) são roteadas para o classificador mesmo quando uma regra de permissão corresponde, e assim são remoções `rm` e `rmdir` direcionadas a um [caminho crítico](#critical-paths) em Claude Code v2.1.218 e posterior
       * Ferramentas MCP marcadas [`requiresUserInteraction`](/docs/pt/mcp#require-approval-for-a-specific-tool) o solicitam diretamente mesmo quando uma regra de permissão corresponde, e assim fazem ferramentas de conector [que sua organização definiu como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) em sessões onde essa configuração chega a Claude Code
       * Um comando de shell que carrega [domínios permitidos por comando](/docs/pt/sandboxing#per-command-allowed-domains-in-auto-mode) também é roteado para o classificador mesmo quando uma regra de permissão corresponde, porque uma regra aprova o comando, não seus hosts
       * Regras de solicitação que correspondem no conteúdo de um comando, como `Bash(git push *)`, voltam para um prompt de permissão
    2. Ações somente leitura e edições de arquivo em seu diretório de trabalho são auto-aprovadas, exceto gravações em [caminhos protegidos](#protected-paths) e [a primeira leitura fora dos diretórios de trabalho](#first-read-outside-the-working-directories), que o solicita
       * Em uma sessão com [revisão do classificador no servidor](#server-side-classifier-review), ações somente leitura e comandos de shell [em sandbox](/docs/pt/sandboxing#sandbox-modes) aguardam essa revisão e são bloqueados se ela os sinalizar
    3. Tudo o mais vai para o classificador. As ferramentas de conector e ferramentas MCP `requiresUserInteraction` que o solicitam diretamente na etapa 1 nunca chegam ao classificador, portanto nem uma aprovação exigida pela organização nem uma etapa de consentimento é auto-aprovada
    4. Se o classificador bloqueia, Claude recebe o motivo e tenta uma alternativa. Na maioria das sessões o motivo nomeia a regra que o classificador correspondeu, como `[Data Exfiltration]`, em vez de dar uma explicação escrita; consulte [Revisar negações](/docs/pt/auto-mode-config#review-denials)

    Ao entrar no modo automático, regras de permissão amplas que concedem execução de código arbitrária são descartadas:

    * Blanket `Bash(*)` ou `PowerShell(*)`
    * Intérpretes com wildcard como `Bash(python*)`
    * Comandos de execução do gerenciador de pacotes
    * Regras de permissão `Agent`
    * Regras de permissão [`Monitor`](/docs/pt/tools-reference#monitor-tool), porque Claude Code executa comandos Monitor através do shell

    Regras estreitas como `Bash(npm test)` permanecem em vigor. Claude Code restaura as regras descartadas quando você sai do modo automático. Antes da v2.1.236, Claude Code deixava regras de permissão `Monitor` em vigor no modo automático, portanto uma regra que correspondesse à ferramenta inteira aprovava comandos Monitor sem revisão do classificador.

    Claude Code também executa `git status` em si antes de um comando que descartaria trabalho não confirmado, como `git reset --hard` ou `rm -rf`, e mostra ao classificador se há trabalho preparado, modificado ou não rastreado presente. Claude Code relata arquivos não rastreados nessa verificação mesmo quando a configuração git do repositório define `status.showUntrackedFiles=no`.

    Nas solicitações do classificador enviadas pelo próprio Claude Code, o classificador vê mensagens de usuário, chamadas de ferramenta diferentes de buscas somente leitura como leituras de arquivo e buscas, e seu conteúdo CLAUDE.md. Resultados de ferramenta são removidos dessas solicitações, portanto conteúdo hostil em um arquivo ou página da web não pode manipular o classificador diretamente.

    Você pode anotar o resultado de uma chamada com um campo [`classifierContext`](/docs/pt/hooks#annotate-a-result-for-the-auto-mode-classifier) do hook PostToolUse, que o classificador lê como contexto fornecido pela aplicação. O campo requer Claude Code v2.1.236 ou posterior.

    Uma sonda separada no servidor escaneia resultados de ferramenta recebidos e sinaliza conteúdo suspeito antes de Claude lê-lo. Para mais sobre como essas camadas funcionam juntas, consulte o [anúncio do modo automático](https://claude.com/blog/auto-mode) e o [aprofundamento de engenharia](https://www.anthropic.com/engineering/claude-code-auto-mode).
  </Accordion>

  <Accordion title="Como o modo automático lida com subagentos">
    O classificador verifica o trabalho de [subagentos](/docs/pt/sub-agents) em três pontos:

    1. Antes de um subagentos iniciar, a descrição da tarefa delegada é avaliada, portanto uma tarefa com aparência perigosa é bloqueada no tempo de spawn.
    2. Enquanto o subagentos executa, cada uma de suas ações passa pelo classificador com as mesmas regras que a sessão pai, e qualquer `permissionMode` no frontmatter do subagentos é ignorado.
    3. Quando o subagentos termina, o classificador revisa seu trabalho e seu relatório final antes que o pai leia o relatório. Quando o classificador sinaliza o trabalho ou relatório do subagentos, ou uma verificação de segurança de API separada recusa a revisão, o relatório ainda é entregue, precedido por um aviso de segurança. Quando o classificador não está disponível para a revisão, o relatório chega com uma nota para verificar o trabalho do subagentos antes de agir com base nele.
  </Accordion>

  <Accordion title="Custo e latência">
    O classificador executa em Claude Sonnet 5 por padrão em vez de em sua seleção `/model`. Um modelo classificador que Anthropic configura no servidor tem precedência sobre esse padrão. Quando o modelo de sua sessão é Claude Sonnet 4.6, ou quando [`availableModels`](/docs/pt/model-config#restrict-model-selection) exclui Sonnet 5, o classificador executa no modelo de sua sessão, ou em um modelo Opus quando a sessão executa em um [modelo Fable](/docs/pt/model-config#work-with-fable); em provedores diferentes da API Anthropic, esse fallback Opus é o modelo Opus padrão do provedor.

    A primeira solicitação de modo automático da sessão valida o padrão Sonnet 5: se a solicitação for bem-sucedida, Sonnet 5 permanece o modelo classificador da sessão, e se falhar porque o modelo não está disponível, a sessão usa o fallback. Depois que essa validação se resolve, o modelo do classificador não muda para a sessão.

    Em planos Enterprise e em contas que usam a API Claude, [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), Amazon Bedrock, Agent Platform do Google Cloud ou Microsoft Foundry, chamadas do classificador contam para seu uso de token. Cada verificação envia uma porção da transcrição mais a ação pendente, adicionando uma volta antes da execução. Leituras e edições de diretório de trabalho fora de caminhos protegidos pulam o classificador, portanto a sobrecarga vem principalmente de comandos de shell e operações de rede. Onde o servidor revisa as ações como parte das solicitações de modelo da sessão, não há chamadas de classificador separadas para contar; consulte [Revisão do classificador no servidor](#server-side-classifier-review).

    O acesso à rede em sandbox não adiciona solicitações de classificador por conexão. O classificador julga [os hosts que um comando nomeia](/docs/pt/sandboxing#per-command-allowed-domains-in-auto-mode) junto com o comando em uma revisão, e Claude Code verifica cada conexão contra a lista aprovada sem chamar o classificador novamente.
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  Permitir apenas ferramentas pré-aprovadas com modo dontAsk
</h2>

Se você definir o modo `dontAsk`, Claude Code nega automaticamente toda chamada de ferramenta que de outra forma solicitaria você. Claude ainda executa ações que não precisam de aprovação no modo Manual, como leituras de arquivo dentro de seus diretórios de trabalho e [comandos Bash somente leitura](/docs/pt/permissions#read-only-commands), além de ações que correspondem às suas regras `permissions.allow` e chamadas aprovadas por um [hook PreToolUse](/docs/pt/permissions#extend-permissions-with-hooks). Use este modo para pipelines de CI ou ambientes restritos onde você pré-define o que Claude pode fazer; a sessão nunca aguarda entrada. A barra de status mostra `⏵⏵ don't ask on` enquanto este modo está ativo.

Claude Code nega chamadas que correspondem às suas [regras `ask`](/docs/pt/permissions#manage-permissions) explícitas em vez de solicitar. Também nega a ferramenta integrada `AskUserQuestion` mesmo que suas regras de permissão correspondam a ela, e faz o mesmo para ferramentas de conector [que sua organização definiu como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) em sessões onde essa configuração chega a Claude Code. Nega ferramentas MCP marcadas [`_meta["anthropic/requiresUserInteraction"]`](/docs/pt/mcp#require-approval-for-a-specific-tool) da mesma forma, porque seu cartão de aprovação precisa de uma resposta que este modo nunca coleta; isso requer Claude Code v2.1.199 ou posterior.

Remoções `rm` e `rmdir` direcionadas a um [caminho crítico](#critical-paths), como `rm -rf /` e `rm -rf ~`, são negadas mesmo quando uma regra de permissão corresponde a elas ou um hook `PreToolUse` as permite.

Sessões em nuvem em [Claude Code na web](/docs/pt/claude-code-on-the-web) ignoram `defaultMode: "dontAsk"`; veja [bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) para detalhes.

Defina-o na inicialização com a flag:

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  Ignorar todas as verificações com modo bypassPermissions
</h2>

O modo `bypassPermissions` desativa prompts de permissão e verificações de segurança para que chamadas de ferramentas sejam executadas imediatamente, incluindo gravações em [caminhos protegidos](#protected-paths).

As [ações que nenhum modo auto-aprova](#actions-no-mode-auto-approves) ainda solicitam neste modo.

Duas [salvaguardas de mensagens entre sessões](/docs/pt/cross-session-messaging) ainda se aplicam neste modo, e em sessões de modo plan interativas onde permissões de bypass estão disponíveis:

* O prompt de aprovação [`isolatePeerMachines`](/docs/pt/settings-reference#isolatepeermachines) para mensagens para suas sessões além desta máquina ainda aparece.
* Quando nenhum valor [`crossSessionInbound`](/docs/pt/cross-session-messaging#control-inbound-messages) se aplica, Claude Code mantém uma mensagem de entrada de outra de suas sessões para sua aprovação, e entrega sem perguntar apenas quando a sessão de envio se identifica como também ignorando prompts de permissão. Se você deixar o modo de permissão enquanto mensagens são mantidas, Claude Code re-aplica as regras de entrada e entrega qualquer mensagem mantida que elas agora aceitam.

Em sessões de terminal interativas com permissões de bypass disponíveis, Claude Code também não impõe os [bloqueios do modo plan](#analyze-before-you-edit-with-plan-mode). Claude ainda é instruído a planejar sem editar, mas uma edição de arquivo ou comando shell que ele tenta durante o planejamento é executado sem solicitar. [Regras de solicitação](/docs/pt/permissions#manage-permissions) explícitas e remoções `rm` e `rmdir` direcionadas a um [caminho crítico](#critical-paths) ainda solicitam.

O modo plan mantém seus bloqueios em qualquer lugar onde Claude Code é executado sem um terminal interativo, incluindo [execuções não-interativas](/docs/pt/headless) com `-p`, sessões do [Agent SDK](/docs/pt/agent-sdk/permissions#plan-mode-plan), e conversas no painel de chat da [extensão VS Code](/docs/pt/vs-code). Lá, `--allow-dangerously-skip-permissions` torna `bypassPermissions` selecionável depois.

<Warning>
  Use este modo apenas em ambientes isolados como contêineres, VMs ou dev containers sem acesso à internet, onde Claude Code não consegue danificar seu sistema host.
</Warning>

Você não consegue entrar em `bypassPermissions` a partir de uma sessão que você iniciou sem ele habilitado. Habilite-o na inicialização com [`permissions.defaultMode: "bypassPermissions"`](/docs/pt/settings-reference#permissions-defaultmode) ou com uma flag de habilitação:

```bash theme={null}
claude --permission-mode bypassPermissions
```

A flag `--dangerously-skip-permissions` é equivalente.

Claude Code recusa `bypassPermissions` em uma sessão que você inicia com [`--restricted`](/docs/pt/cli-reference#cli-flags). `--restricted` requer Claude Code v2.1.248 ou posterior.

A primeira vez que você inicia uma sessão interativa com este modo habilitado, Claude Code mostra um diálogo de aviso pedindo que você aceite responsabilidade pelas ações tomadas sem verificações de permissão. Claude Code salva sua aceitação em configurações de usuário, então o diálogo aparece apenas uma vez. Se você recusar, Claude Code sai. Em [modo não-interativo](/docs/pt/headless) nenhum diálogo é mostrado, e uma [sessão em segundo plano](/docs/pt/agent-view) iniciada com `--bg` é recusada até que você tenha aceitado o diálogo em uma sessão interativa.

Em Linux e macOS, Claude Code recusa iniciar neste modo quando executado como root ou sob `sudo`:

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

A verificação é ignorada automaticamente dentro de um sandbox reconhecido. Para executar autonomamente em um contêiner, use a configuração [dev container](/docs/pt/devcontainer), que executa Claude Code como um usuário não-root.

[Claude Code na web](/docs/pt/claude-code-on-the-web) não honra `defaultMode: "bypassPermissions"` ou `"dontAsk"` de seus arquivos de configurações, então as configurações verificadas de um repositório não conseguem iniciar uma sessão em nuvem no modo bypass-permissions. A configuração é ignorada silenciosamente e a sessão inicia no modo de permissão mostrado no dropdown de modo em vez disso. Veja [Alternar modos de permissão](#switch-permission-modes) para quais modos as sessões em nuvem oferecem.

<Warning>
  `bypassPermissions` não oferece proteção contra injeção de prompt ou ações não intencionais. Para verificações de segurança em segundo plano com muito menos prompts de permissão, use [modo automático](#eliminate-prompts-with-auto-mode) em vez disso. Administradores podem bloquear este modo definindo `permissions.disableBypassPermissionsMode` como `"disable"` em [configurações gerenciadas](/docs/pt/managed-settings).
</Warning>

<h2 id="protected-paths">
  Caminhos protegidos
</h2>

Gravações em um pequeno conjunto de caminhos nunca são auto-aprovadas, exceto no modo `bypassPermissions` e em sessões de terminal interativo em modo plan com [permissões de bypass](#skip-all-checks-with-bypasspermissions-mode) disponíveis. Isso evita corrupção acidental do estado do repositório e da configuração própria do Claude.

| Modo                     | Gravações em caminhos protegidos                                                                                                                                                                                                                                                                                |
| :----------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`, `acceptEdits` | Solicitado                                                                                                                                                                                                                                                                                                      |
| `plan`                   | Permitido em sessões de terminal interativo com [permissões de bypass](#skip-all-checks-with-bypasspermissions-mode) disponíveis. Caso contrário, roteado para o classificador quando [modo automático](#eliminate-prompts-with-auto-mode) está disponível durante o planejamento, e solicitado quando não está |
| `auto`                   | Roteado para o classificador                                                                                                                                                                                                                                                                                    |
| `dontAsk`                | Negado                                                                                                                                                                                                                                                                                                          |
| `bypassPermissions`      | Permitido                                                                                                                                                                                                                                                                                                       |

Em uma sessão iniciada com [`--restricted`](/docs/pt/cli-reference#cli-flags), que requer Claude Code v2.1.248 ou posterior, o classificador não consegue aprovar gravações em caminhos protegidos.

Regras [`permissions.allow`](/docs/pt/permissions#manage-permissions) em arquivos de configurações não pré-aprovam gravações em caminhos protegidos. A verificação de segurança é executada antes de Claude Code avaliar regras de permissão de arquivos de configurações, então uma entrada como `Edit(.claude/**)` em `~/.claude/settings.json` ou `.claude/settings.json` não altera o resultado por modo na tabela acima. Em modos que solicitam, o prompt para uma gravação em `.claude/` oferece **Sim, e permitir que Claude edite suas próprias configurações para esta sessão**, o que aprova gravações posteriores em `.claude/` nessa sessão sem solicitar novamente.

Diretórios protegidos:

* `.git`
* `.config/git`
* `.vscode`
* `.idea`
* `.husky`
* `.cargo`
* `.devcontainer`
* `.yarn`
* `.mvn`
* `.claude`, exceto por `.claude/worktrees` onde Claude armazena seus próprios git worktrees

Arquivos protegidos:

* `.gitconfig`, `.gitmodules`
* `.bashrc`, `.bash_profile`, `.bash_login`, `.bash_aliases`, `.bash_logout`, `.zshrc`, `.zprofile`, `.zshenv`, `.zlogin`, `.zlogout`, `.profile`, `.envrc`
* `.npmrc`, `.yarnrc`, `.yarnrc.yml`, `.pnp.cjs`, `.pnp.loader.mjs`, `.pnpmfile.cjs`, `bunfig.toml`, `.bunfig.toml`
* `.bazelrc`, `.bazelversion`, `.bazeliskrc`
* `.pre-commit-config.yaml`, `lefthook.yml`, `lefthook.yaml`, `.lefthook.yml`, `.lefthook.yaml`
* `gradle-wrapper.properties`, `maven-wrapper.properties`
* `.devcontainer.json`
* `.ripgreprc`, `pyrightconfig.json`
* `.mcp.json`, `.claude.json`

<h2 id="critical-paths">
  Caminhos críticos
</h2>

Claude Code nunca deixa uma regra [`permissions.allow`](/docs/pt/permissions#manage-permissions) ou um hook [`PreToolUse`](/docs/pt/permissions#extend-permissions-with-hooks) que retorna `"allow"` aprovar um comando `rm` ou `rmdir` que tenha como alvo um caminho crítico, mesmo em modos que pulam outros prompts. Este disjuntor protege contra erro do modelo. Uma regra de negação correspondente ainda bloqueia o comando completamente.

O que acontece em vez disso depende do seu modo de permissão:

| Modo                     | O que Claude Code faz com uma remoção de caminho crítico                                                                                                                                                     |
| :----------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`, `acceptEdits` | Pede que você o aprove                                                                                                                                                                                       |
| `plan`                   | Pede que você o aprove. Com [modo automático disponível durante o planejamento](#analyze-before-you-edit-with-plan-mode) e nenhuma permissão de bypass disponível, envia-o para o classificador em vez disso |
| `auto`                   | Envia-o para o [classificador](#eliminate-prompts-with-auto-mode)                                                                                                                                            |
| `dontAsk`                | Nega-o                                                                                                                                                                                                       |
| `bypassPermissions`      | Pede que você o aprove                                                                                                                                                                                       |

Se uma [regra de solicitação](/docs/pt/permissions#manage-permissions) explícita corresponder ao comando, Claude Code o solicita mesmo em modo `auto`. Em modos que solicitam, um hook [`PermissionRequest`](/docs/pt/hooks#permissionrequest) pode responder ao prompt da forma que responde a qualquer outro.

Claude Code trata um alvo `rm` ou `rmdir` como um caminho crítico quando é qualquer um dos seguintes:

* A raiz do sistema de arquivos
* Diretórios de nível superior, significando qualquer filho direto da raiz, como `/usr`, `/etc` ou `/data`
* Seu diretório home
* Raízes de unidade do Windows e seus diretórios de nível superior, como `C:\` e `C:\Windows`
* Seu diretório de trabalho e seus pais
* Seus diretórios de trabalho adicionais e seus pais, mas apenas quando a remoção é um glob sob um deles, como `rm -rf <dir>/*`. `rm -rf <dir>` no próprio diretório não dispara essa verificação

Claude Code também trata um glob ou barra à direita diretamente sob uma variável de shell, como `rm -rf "$DIR"/*`, como uma remoção de caminho crítico, porque o comando se torna uma remoção da raiz do sistema de arquivos quando a variável está vazia.

O prompt para este caso de variável nomeia o `rm` sinalizado e diz como reescrevê-lo para que a verificação passe:

* Para uma variável como `$DIR`, proteja cada expansão para que o shell pare com um erro quando a variável não estiver definida ou vazia, como em `rm -rf "${DIR:?}"/*`, ou use um caminho literal
* Para uma variável que normalmente está definida, como `$HOME`, use um caminho literal

Uma remoção cujas expansões estão todas protegidas dessa forma não é uma remoção de caminho crítico, então em modo `bypassPermissions` ela é executada sem um prompt.

Esconder a remoção dentro de uma subshell com `(...)`, um grupo de chaves com `{ ...; }`, substituição de comando com `$(...)` ou backticks, ou substituição de processo com `<(...)`, não pula a verificação. Claude Code encontra uma remoção de caminho crítico independentemente de estar dentro da forma aninhada, como em `(rm -rf ~)` ou `echo "$(rm -rf ~)"`, ou em outro lugar no mesmo comando.

<h3 id="remove-item-in-powershell">
  Remove-Item em PowerShell
</h3>

Quando você habilita a [ferramenta PowerShell](/docs/pt/tools-reference#powershell-tool), Claude Code dá a `Remove-Item` sua própria verificação, separada da lista de caminhos críticos `rm`. O resultado depende do alvo, e o primeiro caso correspondente se aplica:

* **Caminhos do sistema**: a raiz do sistema de arquivos e seus diretórios de nível superior, raízes de unidade e seus diretórios de nível superior, e seu diretório home. Claude Code nega o comando em todos os modos, sem o solicitar.
* **Wildcards**: um `*` nu, ou qualquer alvo terminando em `/*` ou `\*`, incluindo um glob sob uma variável de shell como `$dir/*`. Claude Code nega o comando em todos os modos, sem o solicitar, antes do [classificador](#eliminate-prompts-with-auto-mode) vê-lo.
* **Seu diretório de trabalho ou um de seus pais, com `-Recurse`**: Claude Code trata o comando como qualquer outro que precisa de aprovação em seu modo de permissão, então o solicita em modos que solicitam, envia-o para o classificador em modo `auto` e o nega em modo `dontAsk`. O modo `bypassPermissions` pula essa verificação.

<h2 id="see-also">
  Veja também
</h2>

* [Permissões](/docs/pt/permissions): regras de permissão, solicitação e negação; políticas gerenciadas
* [Configurar modo automático](/docs/pt/auto-mode-config): diga ao classificador qual infraestrutura sua organização confia
* [Hooks](/docs/pt/hooks): lógica de permissão personalizada via hooks `PreToolUse` e `PermissionRequest`
* [Segurança](/docs/pt/security): salvaguardas e melhores práticas
* [Sandboxing](/docs/pt/sandboxing): isolamento de sistema de arquivos e rede para comandos Bash
* [Modo não-interativo](/docs/pt/headless): execute Claude Code com a flag `-p`
