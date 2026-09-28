> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar a ferramenta Bash em sandbox

> Aprenda como a ferramenta Bash em sandbox do Claude Code fornece isolamento de sistema de arquivos e rede para execução de agentes mais segura e autônoma.

O sandbox Bash permite que Claude execute a maioria dos comandos shell sem parar para pedir permissão. Em vez de aprovar cada comando, você define quais arquivos e domínios de rede os comandos podem acessar, e o sistema operacional impõe esse limite para cada comando Bash, PowerShell ou Monitor e seus processos filhos.

<Note>
  Para comparar outras abordagens de isolamento, como dev containers, containers personalizados e máquinas virtuais, consulte [Sandbox environments](/docs/pt/sandbox-environments). Para reduzir prompts de permissão para ferramentas diferentes de Bash, consulte [permission modes](/docs/pt/permission-modes).
</Note>

<h2 id="get-started">
  Comece agora
</h2>

O sandbox é integrado ao Claude Code e é executado em macOS, Linux e WSL2. Windows nativo não é suportado. No Windows, execute Claude Code dentro de uma distribuição WSL2.

No macOS, não há nada para instalar: o sandboxing usa o framework Seatbelt integrado. No Linux e WSL2, o sandbox depende de dois pacotes, abordados em [Set up Linux and WSL2](#set-up-linux-and-wsl2). Mesmo que você ainda não os tenha instalado, você pode começar com `/sandbox`, porque seu painel mostra se algo está faltando.

<Steps>
  <Step title="Run /sandbox">
    Inicie uma sessão do Claude Code e execute o comando `/sandbox`:

    ```text theme={null}
    /sandbox
    ```

    Isso abre o painel de sandbox com três abas, mais uma aba Dependencies no Linux quando o filtro seccomp opcional está faltando:

    * **Mode**: escolha como os comandos em sandbox são aprovados, abordado na próxima etapa
    * **Overrides**: escolha se os comandos que falham sob o sandbox podem voltar a ser executados sem sandbox. Esta é a configuração [`allowUnsandboxedCommands`](/docs/pt/settings-reference#sandbox-allowunsandboxedcommands)
    * **Config**: visualize as configurações de sandbox resolvidas

    Se o painel mostrar apenas uma aba Dependencies, um pacote necessário está faltando. Instale-o conforme descrito em [Set up Linux and WSL2](#set-up-linux-and-wsl2), reinicie Claude Code e execute `/sandbox` novamente.
  </Step>

  <Step title="Choose a mode">
    Na aba Mode, selecione auto-allow ou regular permissions. Auto-allow executa comandos em sandbox sem avisar, e regular permissions mantém os prompts de permissão regulares mesmo quando os comandos estão em sandbox. Consulte [Sandbox modes](#sandbox-modes) para saber quais comandos ainda solicitam no modo auto-allow.
  </Step>

  <Step title="Run a Bash command">
    Peça ao Claude para executar um comando, como uma compilação ou um conjunto de testes. Por padrão, os comandos dentro do sandbox podem escrever no diretório de trabalho, no diretório temporário da sessão e em qualquer [diretório que você tenha adicionado](/docs/pt/permissions#additional-directories-grant-file-access-not-configuration) com `--add-dir`, `/add-dir` ou `permissions.additionalDirectories`.

    Na primeira vez que um comando precisa de um novo domínio de rede, Claude Code solicita aprovação; em [modo auto](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode), Claude em vez disso nomeia os hosts que um comando precisa [no próprio comando](#per-command-allowed-domains-in-auto-mode) para o classificador revisar com ele.

    Comandos que não podem ser executados em sandbox voltam ao fluxo de permissão regular. Claude Code intitula seu prompt de permissão como "Bash command (unsandboxed)" em vez de "Bash command", para que você possa saber quais comandos foram executados fora do sandbox. Para ampliar ou estreitar o que o sandbox permite, consulte [Configure sandboxing](#configure-sandboxing).

    Se comandos em sandbox falharem com `Operation not permitted` dentro de um container, consulte a entrada Bubblewrap em [Troubleshooting](#troubleshooting).
  </Step>
</Steps>

Quando você seleciona um modo no painel, Claude Code o salva nas configurações locais do seu projeto em `.claude/settings.local.json`, que se aplicam ao projeto atual. Claude Code adiciona esse arquivo ao seu gitignore global quando salva uma configuração lá. Para habilitar o sandbox em todos os seus projetos, defina [`sandbox.enabled`](/docs/pt/settings-reference#sandbox-enabled) como `true` em suas configurações de usuário em `~/.claude/settings.json`. Para impor sandboxing para cada desenvolvedor em uma organização, use [managed settings](#enforce-sandboxing-with-managed-settings).

Para alterar o sandbox para uma sessão sem escrever em um arquivo de configurações, inicie Claude Code com [`--settings`](/docs/pt/settings#change-a-setting-for-one-session). Por exemplo, este comando inicia uma sessão em sandbox na qual Claude não pode tentar novamente um comando bloqueado fora do sandbox:

```bash theme={null}
claude --settings '{"sandbox": {"enabled": true, "allowUnsandboxedCommands": false}}'
```

<Warning>
  Por padrão, se o sandbox não conseguir iniciar porque as dependências estão faltando ou a plataforma não é suportada, Claude Code mostra um aviso e executa comandos sem sandboxing. Para tornar isso uma falha difícil em vez disso, defina [`sandbox.failIfUnavailable`](/docs/pt/settings-reference#sandbox-failifunavailable) como `true`. Isso é destinado a implantações gerenciadas que exigem sandboxing como um portão de segurança.
</Warning>

<h3 id="set-up-linux-and-wsl2">
  Configure o Linux e o WSL2
</h3>

No Linux e WSL2, o sandbox depende de dois pacotes:

* [`bubblewrap`](https://github.com/containers/bubblewrap): a ferramenta de sandboxing sem privilégios que impõe isolamento de sistema de arquivos
* [`socat`](http://www.dest-unreach.org/socat/): o relay usado para rotear tráfego de rede através do proxy de sandbox

Instale-os com o gerenciador de pacotes da sua distribuição:

<Tabs>
  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt-get install bubblewrap socat
    ```
  </Tab>

  <Tab title="Fedora">
    ```bash theme={null}
    sudo dnf install bubblewrap socat
    ```
  </Tab>
</Tabs>

Quando uma dependência está faltando, a aba Dependencies em `/sandbox` lista qual de `ripgrep`, `bubblewrap`, `socat` e o filtro seccomp sua plataforma não possui. Se você não vir a aba após instalar e reiniciar Claude Code, todas as dependências estão presentes.

Ripgrep é incluído no binário nativo do Claude Code. O filtro seccomp é opcional e adiciona bloqueio de socket de domínio Unix. Instale-o com `npm install -g @anthropic-ai/sandbox-runtime` se estiver faltando.

Quando uma dependência necessária está faltando, a aba Dependencies é a única aba mostrada até que você a instale. Quando apenas o filtro seccomp opcional está faltando, a aba Dependencies aparece junto com as outras abas. A verificação de dependência é executada na inicialização, portanto reinicie Claude Code após instalar pacotes para que `/sandbox` os detecte.

<AccordionGroup>
  <Accordion title="Ubuntu 24.04 e posterior: permitir que bubblewrap crie namespaces de usuário">
    No Ubuntu 24.04 e posterior, a política padrão do AppArmor impede que bubblewrap crie os namespaces de usuário que precisa para isolamento.

    Para verificar se seu ambiente impõe essa restrição, incluindo dentro do WSL2, execute `sysctl kernel.apparmor_restrict_unprivileged_userns`. Se o comando retornar `0`, pule esta etapa. Se imprimir um erro `No such file or directory`, a chave não existe e você pode pular esta etapa. Se retornar `1`, adicione um perfil AppArmor que conceda a `bwrap` essa capacidade:

    ```bash theme={null}
    sudo tee /etc/apparmor.d/bwrap > /dev/null <<'EOF'
    abi <abi/4.0>,
    include <tunables/global>

    profile bwrap /usr/bin/bwrap flags=(unconfined) {
      userns,
      include if exists <local/bwrap>
    }
    EOF
    ```

    O perfil se aplica apenas a `bwrap` em si, não aos comandos executados dentro do sandbox. Recarregue AppArmor para aplicá-lo:

    ```bash theme={null}
    sudo systemctl reload apparmor
    ```
  </Accordion>

  <Accordion title="Notas do WSL2">
    Verifique sua versão do WSL com `wsl -l -v` do PowerShell. Se você vir `Sandboxing requires WSL2`, sua distribuição está executando WSL1. Atualize-a para WSL2 ou execute Claude Code sem sandboxing.

    No WSL2, WSL entrega um lançamento de um binário do Windows como `cmd.exe`, `powershell.exe` ou qualquer coisa em `/mnt/c/` para o host do Windows através de um socket Unix, portanto, se um comando em sandbox pode ou não lançar um deles depende das configurações de [Unix-socket](/docs/pt/settings-reference#sandbox-network-allowunixsockets) do sandbox: o filtro seccomp opcional tem que ser instalado para bloquear o socket em primeiro lugar. Para permitir esses lançamentos, defina `allowAllUnixSockets`; para mantê-los fora do sandbox completamente, adicione o comando a [`excludedCommands`](/docs/pt/settings-reference#sandbox-excludedcommands).
  </Accordion>
</AccordionGroup>

<h3 id="sandbox-modes">
  Modos de sandbox
</h3>

Claude Code oferece dois modos de sandbox. Em ambos, o sandbox impõe as mesmas restrições de sistema de arquivos e rede; a diferença é apenas se os comandos em sandbox são aprovados automaticamente ou requerem permissão explícita.

<h4 id="auto-allow-mode">
  Modo auto-allow
</h4>

Quando um comando pode ser colocado em sandbox, Claude Code o executa dentro do sandbox e o aprova automaticamente, sem pedir sua permissão. Comandos que não podem ser colocados em sandbox, como aqueles que precisam de acesso à rede para hosts não permitidos, voltam ao fluxo de permissão regular, onde Claude Code verifica suas [permission rules](/docs/pt/permissions) e bloqueia qualquer comando que essas regras não permitam, com um prompt no modo Manual.

Mesmo no modo auto-allow, o seguinte ainda se aplica:

* [Deny rules](/docs/pt/permissions) explícitas são sempre respeitadas
* Comandos `rm` ou `rmdir` que visam um [critical path](/docs/pt/permission-modes#critical-paths) ainda passam pelo fluxo de permissão regular
* [Ask rules](/docs/pt/permissions) com escopo de conteúdo como `Bash(git push *)` ainda forçam um prompt mesmo para comandos em sandbox
* Uma regra ask `Bash` simples, ou o formulário equivalente `Bash(*)`, é ignorada para comandos executados em sandbox; ainda se aplica a comandos que voltam ao fluxo de permissão regular. Em [plan mode](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode), a regra não é ignorada: ela solicita comandos em sandbox também, incluindo os somente leitura. Antes da v2.1.212, a omissão se aplicava no modo plan também

<Info>
  O modo auto-allow funciona independentemente de sua configuração de permission mode, com três exceções: [plan mode](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode), um comando auto mode que carrega [per-command allowed domains](#per-command-allowed-domains-in-auto-mode), e [server-side classifier review](/docs/pt/permission-modes#how-the-classifier-evaluates-actions) de comandos em sandbox no modo auto. Mesmo que você não esteja no modo "accept edits", comandos Bash em sandbox são executados automaticamente quando auto-allow está habilitado. Isso significa que comandos Bash que modificam arquivos dentro dos limites do sandbox são executados sem avisar, mesmo no modo Manual, onde as ferramentas de edição de arquivo solicitariam.

  No modo plan, auto-allow não amplia aprovações; consulte [plan mode](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode) para saber como Claude Code bloqueia comandos enquanto você planeja. Antes da v2.1.212, auto-allow executava comandos em sandbox sem um prompt no modo plan também.
</Info>

<h4 id="regular-permissions-mode">
  Modo de permissões regular
</h4>

Todos os comandos Bash passam pelo fluxo de permissão regular, mesmo quando em sandbox. Isso fornece mais controle, mas requer mais aprovações.

<h4 id="the-unsandboxed-retry-escape-hatch">
  A válvula de escape da nova tentativa fora do sandbox
</h4>

Alguns comandos não podem ser executados dentro do sandbox, como ferramentas que são incompatíveis com ele ou que precisam de um host que você não permitiu. Claude Code relata violações de sandbox no resultado do comando bloqueado, nomeando o caminho ou host que o sandbox negou, para que Claude veja o que o sandbox bloqueou. Em vez de falhar na tarefa ou exigir que você desative o sandboxing, Claude Code inclui um escape hatch: Claude analisa a violação e pode tentar novamente o comando com o parâmetro `dangerouslyDisableSandbox`.

O comando retentado é executado fora do sandbox, portanto passa pelo fluxo de permissão regular. No modo Manual você recebe um prompt de confirmação. Em [auto mode](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode), o classificador avalia o comando subjacente. Enquanto [`permissions.blockReadsOutsideWorkingDirectories`](/docs/pt/settings-reference#permissions-blockreadsoutsideworkingdirectories) está ativado, uma retentativa que precisa de aprovação para ser executada fora do sandbox o solicita em vez disso. Para ser solicitado em cada retentativa sem sandbox mesmo no modo auto, adicione uma [ask rule](/docs/pt/permissions#match-by-input-parameter) para `Bash(dangerouslyDisableSandbox:true)`.

Você pode desabilitar esse escape hatch definindo `"allowUnsandboxedCommands": false` em suas [sandbox settings](/docs/pt/settings-reference#sandbox-settings). Com o escape hatch desabilitado, Claude Code ignora o parâmetro `dangerouslyDisableSandbox`, e cada comando que Claude executa deve ser executado em sandbox a menos que você o tenha listado em `excludedCommands`. A aba **Overrides** do `/sandbox` mostra essa configuração como **Strict sandbox mode**.

O modo strict sandbox se aplica aos comandos que Claude executa. Comandos que você digita você mesmo no prompt [shell-mode com prefixo `!`](/docs/pt/interactive-mode#shell-mode-with-prefix) são executados fora do sandbox a menos que a sessão seja uma destas:

* **Uma [background session](/docs/pt/agent-view)**: o modo strict sandbox cobre comandos shell-mode também
* **Uma sessão Linux com [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/pt/env-vars#variables) definido**: cada comando é executado em sandbox, comandos shell-mode incluídos

Antes da v2.1.260, o modo strict sandbox colocava em sandbox comandos shell-mode em cada sessão.

<h4 id="temporary-directories">
  Diretórios temporários
</h4>

O diretório temporário da sessão é gravável dentro do sandbox por padrão, junto com o diretório de trabalho. A menos que você [desabilite isolamento de sistema de arquivos](#disable-filesystem-isolation), Claude Code define `$TMPDIR` para este diretório para comandos em sandbox, portanto ferramentas que escrevem arquivos temporários funcionam sem configuração extra.

Comandos não em sandbox herdam o `$TMPDIR` do seu shell quando está definido, portanto enquanto isolamento de sistema de arquivos está ativado, comandos em sandbox e não em sandbox resolvem `$TMPDIR` para diretórios diferentes. Se seu shell deixar `$TMPDIR` indefinido ou vazio, um comando não em sandbox que referencia `$TMPDIR` recebe sua substituição [`CLAUDE_CODE_TMPDIR`](/docs/pt/env-vars) ou o diretório temporário do sistema operacional quando você não definiu uma ou a substituição é um caminho longo, portanto a variável não se expande para uma string vazia. Para passar arquivos temporários entre os dois, escreva-os no diretório de trabalho em vez disso.

<h2 id="configure-sandboxing">
  Configure o sandboxing
</h2>

Personalize o comportamento do sandbox através de seu arquivo `settings.json`. Consulte [Settings](/docs/pt/settings-reference#sandbox-settings) para a referência de configuração completa.

Por padrão, comandos em sandbox podem escrever no diretório de trabalho atual, no diretório temporário da sessão e em qualquer [diretório que você tenha adicionado](/docs/pt/permissions#additional-directories-grant-file-access-not-configuration) com `--add-dir`, `/add-dir` ou `permissions.additionalDirectories`. Se comandos de subprocesso como `kubectl`, `terraform` ou `npm` precisarem escrever fora desses diretórios, use `sandbox.filesystem.allowWrite` para conceder acesso a caminhos específicos:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "allowWrite": ["~/.kube", "/tmp/build"]
    }
  }
}
```

Esses caminhos são impostos no nível do SO, portanto todos os comandos executados dentro do sandbox, incluindo seus processos filhos, os respeitam. Esta é a abordagem recomendada quando uma ferramenta precisa de acesso de escrita a um local específico, em vez de excluir a ferramenta do sandbox inteiramente com `excludedCommands`.

Quando você define o mesmo array de sistema de arquivos em múltiplos [settings scopes](/docs/pt/settings#settings-precedence), Claude Code mescla-os, combinando caminhos de cada escopo em vez de substituir o array de um escopo pelo de outro.

Se você excluir uma fonte com [`--setting-sources`](/docs/pt/cli-reference) na CLI ou [`settingSources`](/docs/pt/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) no Agent SDK, Claude Code ignora suas entradas `sandbox.filesystem`, suas regras de permissão `Edit` e suas regras de negação `Read` ao construir a configuração do sandbox. Requer Claude Code v2.1.246 ou posterior.

Quando você edita essas listas de sistema de arquivos durante uma sessão, Claude Code [aplica a mudança à sessão em execução](/docs/pt/settings#when-edits-take-effect), portanto o próximo comando em sandbox é executado sob os novos caminhos.

Prefixos de caminho controlam como os caminhos são resolvidos:

| Prefixo             | Significado                                                                                              | Exemplo                                                                    |
| :------------------ | :------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------- |
| `/`                 | Caminho absoluto da raiz do sistema de arquivos                                                          | `/tmp/build` permanece `/tmp/build`                                        |
| `~/`                | Relativo ao diretório home                                                                               | `~/.kube` torna-se `$HOME/.kube`                                           |
| `./` ou sem prefixo | Relativo à raiz do projeto para configurações de projeto, ou a `~/.claude` para configurações de usuário | `./output` em `.claude/settings.json` resolve para `<project-root>/output` |

Esta sintaxe difere das [Read and Edit permission rules](/docs/pt/permissions#read-and-edit), que usam `//path` para absoluto e `/path` para relativo ao projeto. Os caminhos do sistema de arquivos do sandbox usam convenções padrão: `/tmp/build` é absoluto. Para como Claude Code trata uma barra à direita ou um curinga nesses caminhos, consulte [Sandbox path prefixes](/docs/pt/settings-reference#sandbox-path-prefixes).

Você também pode negar acesso de escrita ou leitura usando `sandbox.filesystem.denyWrite` e `sandbox.filesystem.denyRead`, e permitir novamente caminhos específicos dentro de uma região negada usando `sandbox.filesystem.allowRead`. Quando as regras de leitura se sobrepõem, o caminho mais específico vence:

| Regras de exemplo                                      | Resultado                                                                                                                                                                                                              |
| :----------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"denyRead": ["~/"]` com `"allowRead": ["~/projects"]` | `~/projects` é legível e o resto do diretório home permanece bloqueado. O allow mais estreito reabre essa parte da região negada                                                                                       |
| `"allowRead": ["~/"]` com `"denyRead": ["~/.env"]`     | `~/.env` permanece bloqueado e o resto do diretório home é legível. O deny se mantém dentro de um allow mais amplo, portanto um allow amplo não pode reexpor silenciosamente um segredo                                |
| `"allowRead": ["~/"]` com `"denyRead": ["~/**/.env"]`  | Cada `.env` sob o diretório home permanece bloqueado e o resto é legível. Um [wildcard deny](/docs/pt/settings-reference#sandbox-path-prefixes) se mantém dentro de um allow mais amplo da mesma forma que um caminho exato |

O exemplo abaixo bloqueia a leitura de todo o diretório home enquanto ainda permite leituras do projeto atual. Coloque-o no `.claude/settings.json` do seu projeto, porque o caminho relativo `.` resolve para a raiz do projeto apenas quando a configuração reside em configurações de projeto:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["."]
    }
  }
}
```

Se você colocasse a mesma configuração em `~/.claude/settings.json`, `.` resolveria para `~/.claude` em vez disso, e arquivos do projeto permaneceriam bloqueados pela regra `denyRead`.

Para negar aos comandos em sandbox acesso de leitura a diretórios home e volumes montados enquanto mantém os diretórios de trabalho legíveis, defina [`permissions.blockReadsOutsideWorkingDirectories`](/docs/pt/settings-reference#permissions-blockreadsoutsideworkingdirectories) em vez de escrever regras de caminho.

<h3 id="disable-filesystem-isolation">
  Desative o isolamento do sistema de arquivos
</h3>

Defina `sandbox.filesystem.disabled` como `true` para pular o isolamento do sistema de arquivos enquanto mantém o isolamento de rede. O exemplo abaixo desativa o isolamento do sistema de arquivos enquanto mantém uma lista de permissão de domínios de rede:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "disabled": true
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

O sandbox tem duas camadas independentes: [filesystem isolation](#filesystem-isolation) controla quais caminhos os comandos em sandbox podem ler e escrever, e [network isolation](#network-isolation) controla quais domínios eles podem alcançar. Com a camada de sistema de arquivos desativada, comandos em sandbox obtêm acesso irrestrito de leitura e escrita ao sistema de arquivos do host, enquanto sua saída de rede permanece confinada aos seus domínios permitidos. Desative a camada quando você fizer sandbox para controlar onde os comandos se conectam em vez do que eles escrevem.

A configuração está desativada por padrão e se aplica nas plataformas onde o sandbox é executado: macOS, Linux e WSL2. Requer Claude Code v2.1.216 ou posterior.

<Warning>
  Com o isolamento do sistema de arquivos desativado e comandos auto-permitidos, um comando em sandbox pode escrever arquivos que comandos posteriores executam ou leem, como arquivos de inicialização do shell, executáveis em `$PATH` ou `~/.claude/settings.json`, e usá-los para ampliar seu próprio acesso na próxima execução. Defina `filesystem.disabled` como `true` apenas para cargas de trabalho que você confia não escalarem seu próprio acesso. Bloquear domínios de rede com [`allowManagedDomainsOnly`](#keep-developers-from-widening-the-policy) reduz o risco, mas não o remove, já que esse bloqueio se aplica apenas a comandos executados dentro do sandbox.
</Warning>

<h4 id="which-settings-can-disable-it">
  Quais configurações podem desativá-lo
</h4>

Como desativar o isolamento do sistema de arquivos amplia o que os comandos em sandbox podem fazer, Claude Code honra `filesystem.disabled` apenas dessas fontes de configuração:

* Configurações de usuário, configurações gerenciadas e o sinalizador CLI `--settings` podem defini-lo. Configurações de projeto em `.claude/settings.json` e `.claude/settings.local.json` não podem, portanto um projeto verificado não pode desativar o isolamento do sistema de arquivos.
* Quando as configurações gerenciadas configuram `sandbox.filesystem` de qualquer forma, ou listam qualquer entrada `sandbox.credentials.files` com `"mode": "deny"`, apenas as configurações gerenciadas podem definir a chave. Isso mantém as restrições de sistema de arquivos implantadas pelo administrador em vigor; para relaxar tal implantação, defina `"disabled": true` nas configurações gerenciadas.
* Quando [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/pt/env-vars) está definido, Claude Code ignora `filesystem.disabled` de cada fonte, incluindo configurações gerenciadas, e mantém o isolamento do sistema de arquivos ativado.

Se uma entrada `credentials.files` gerenciada fixa `filesystem.disabled`, bloqueando a chave para configurações gerenciadas para que os desenvolvedores não possam desativar o isolamento do sistema de arquivos, depende do `mode` da entrada e do que acontece com a entrada quando o sandbox inicia:

| Entrada gerenciada                                                                                                 | Fixa `filesystem.disabled`    | O que protege o arquivo quando o isolamento está desativado                                                                                  |
| ------------------------------------------------------------------------------------------------------------------ | ----------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `"mode": "deny"`                                                                                                   | Sim                           | Nada: o bloqueio de leitura faz parte da camada do sistema de arquivos                                                                       |
| `"mode": "mask"`, aplicado como uma máscara                                                                        | Não                           | A própria mascaragem: a [cópia sentinela e proxy](#mask-credential-files) no Linux e WSL2, as próprias regras de leitura do sandbox no macOS |
| `"mode": "mask"`, [recuado para `deny`](#mask-credential-files) na configuração                                    | Não                           | Nada, igual a `deny`. Liste um caminho que não pode ser mascarado, como um diretório, como uma entrada `deny` explícita, que fixa a chave    |
| `"mode": "mask"`, [degradado para `deny` pela validação](/docs/pt/managed-settings#invalid-entries-in-managed-settings) | Sim, como um `deny` explícito | Nada, igual a `deny`                                                                                                                         |

Um recuo acontece quando o sandbox inicia, depois que Claude Code já leu as configurações em que a verificação de pino é executada, portanto uma entrada recuada nunca fixa. A validação reescreve uma entrada inválida para `deny` enquanto as configurações carregam, portanto uma entrada degradada fixa como uma que você escreveu como `deny`.

<h4 id="what-changes-when-filesystem-isolation-is-off">
  O que muda quando o isolamento do sistema de arquivos está desativado
</h4>

Definir `filesystem.disabled` remove as proteções que a camada do sistema de arquivos em si aplica. Proteções que outras camadas aplicam continuam se aplicando:

| Proteção                                                                                     | Com isolamento do sistema de arquivos desativado                                                                                                                                  |
| -------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `filesystem.denyRead` e [`credentials.files`](#protect-credentials) blocos de leitura `deny` | Não aplicado. A camada do sistema de arquivos aplica ambos                                                                                                                        |
| `credentials.envVars` entradas `deny` e `mask`                                               | Aplicado. A limpeza de variáveis de ambiente é independente da camada do sistema de arquivos                                                                                      |
| [`credentials.files` entradas `mask`](#mask-credential-files) aplicadas como máscaras        | Aplicado: mascaramento é independente da camada do sistema de arquivos. Uma entrada que [recuou para `deny`](#mask-credential-files) não é aplicada, como qualquer entrada `deny` |

Duas outras coisas mudam:

* Comandos em sandbox herdam `$TMPDIR` do seu shell em vez do diretório temporário da sessão, porque cada diretório temporário é gravável e Claude Code não redireciona mais comandos para o da sessão.

  No Linux a variável geralmente não está definida no shell pai. A orientação de ferramenta Bash diz a Claude para criar diretórios de rascunho com `mktemp -d` em vez de confiar em `$TMPDIR`.
* [`autoAllowBashIfSandboxed`](/docs/pt/settings-reference#sandbox-autoallowbashifsandboxed) ainda padrão para `true`, portanto comandos em sandbox continuam executando sem prompts. Defina-o como `false` para solicitar comandos em sandbox.

<h3 id="protect-credentials">
  Proteja credenciais
</h3>

A configuração `sandbox.credentials` declara arquivos de credenciais e variáveis de ambiente a proteger de comandos em sandbox. Cada entrada nomeia um caminho de arquivo ou uma variável de ambiente e um `mode`. O bloco `credentials` dedicado mantém as regras de credenciais agrupadas e separadas das regras gerais do sistema de arquivos.

Para entradas com `"mode": "deny"`, caminhos de arquivo são negados para leituras dentro do sandbox, a mesma restrição que `filesystem.denyRead` aplica, e variáveis de ambiente são removidas antes de cada comando em sandbox ser executado. A proteção de arquivo faz parte da camada do sistema de arquivos, portanto não se aplica se você [desativar o isolamento do sistema de arquivos](#disable-filesystem-isolation); a proteção de variável de ambiente ainda se aplica.

O exemplo abaixo bloqueia leituras do arquivo de credenciais AWS e do diretório SSH e remove `GITHUB_TOKEN` e `NPM_TOKEN` do ambiente de comandos em sandbox:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "credentials": {
      "files": [
        { "path": "~/.aws/credentials", "mode": "deny" },
        { "path": "~/.ssh", "mode": "deny" }
      ],
      "envVars": [
        { "name": "GITHUB_TOKEN", "mode": "deny" },
        { "name": "NPM_TOKEN", "mode": "deny" }
      ]
    }
  }
}
```

Entradas de variáveis de ambiente e entradas de arquivo também aceitam `"mode": "mask"`, descrito em [Mask credentials](#mask-credentials).

Os caminhos de arquivo seguem as mesmas [regras de prefixo](/docs/pt/settings-reference#sandbox-path-prefixes) que as configurações `sandbox.filesystem.*`.

Claude Code mescla as entradas `deny` de cada [settings scope](/docs/pt/settings#settings-precedence) que a sessão carrega. Uma entrada `deny` apenas restringe o acesso, portanto qualquer escopo pode adicionar uma, mas nenhum escopo pode remover uma que outro escopo adicionou.

Quando você [exclui uma fonte de configuração](#configure-sandboxing):

* **Configurações de projeto ou local**: Claude Code não aplica nenhuma de suas entradas `credentials`. Requer Claude Code v2.1.246 ou posterior.
* **Configurações de usuário**: Claude Code ainda aplica as entradas `deny` em `~/.claude/settings.json` e mantém suas entradas `mask` de [arquivo](#mask-credential-files) como restrições, mas descarta suas entradas `mask` de [variável de ambiente](#mask-environment-variables).

Não há uma lista de negação de credenciais integrada, portanto apenas os arquivos e variáveis que você listar são restritos.

`sandbox.credentials` afeta apenas comandos Bash em sandbox. Para remover credenciais de todos os subprocessos independentemente do sandboxing, defina [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/pt/env-vars).

<h3 id="mask-credentials">
  Mascare credenciais
</h3>

Mascaramento vai além de uma entrada `deny` em [Protect credentials](#protect-credentials). Em vez de bloquear uma credencial, Claude Code mostra aos comandos em sandbox um espaço reservado, o sentinela, e o [sandbox proxy](#network-isolation) troca o valor real em solicitações de saída para hosts que você permite. Para arquivos, a substituição é comportamento do Linux e WSL2; [macOS bloqueia o arquivo em vez disso](#mask-credential-files).

<h4 id="mask-environment-variables">
  Mascare variáveis de ambiente
</h4>

`"mode": "mask"` protege uma credencial mantendo as ferramentas que se autenticam com ela funcionando. `deny` remove a variável inteiramente, o que também quebra ferramentas que precisam dela, como `gh` ou `npm`. Requer Claude Code v2.1.199 ou posterior.

Com `mask`, o comando em sandbox vê um valor sentinela por sessão em vez do real. Cada entrada `mask` pode listar `injectHosts`, os hosts aos quais o valor real é permitido alcançar. Quando uma solicitação sai do sandbox para um deles, o [sandbox proxy](#network-isolation) substitui o sentinela pelo valor real. O comando e tudo que ele registra nunca mantêm a credencial real, mas suas solicitações ainda se autenticam.

O proxy substitui a credencial dentro do conteúdo da solicitação, portanto tem que vê-lo. Defina [`network.tlsTerminate`](/docs/pt/settings-reference#sandbox-network-tlsterminate) para que o proxy termine TLS em si.

Sem isso, o mascaramento falha sem expor nada: o comando ainda vê apenas o sentinela, mas o sentinela chega ao servidor inalterado e a autenticação falha. Claude Code relata essa configuração incorreta na inicialização.

A substituição cobre cabeçalhos e corpos de solicitação. Solicitações que se autenticam com uma assinatura derivada da credencial, em vez da credencial em si, precisam ser re-assinadas no proxy; [Re-sign AWS requests](#re-sign-aws-requests) cobre como isso funciona para AWS.

O proxy injeta apenas em conexões que a [lista de permissão de domínio](#network-isolation) admite, portanto cada destino `injectHosts` também deve ser alcançável através de `network.allowedDomains`.

O exemplo abaixo mascara dois tokens. `GH_TOKEN` é substituído apenas em solicitações para `api.github.com`, enquanto `NPM_TOKEN` não tem `injectHosts` e é substituído em solicitações para cada host em `network.allowedDomains`.

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com", "registry.npmjs.org"]
    },
    "credentials": {
      "envVars": [
        { "name": "GH_TOKEN", "mode": "mask", "injectHosts": ["api.github.com"] },
        { "name": "NPM_TOKEN", "mode": "mask" }
      ]
    }
  }
}
```

<span id="ipv6-destinations-in-injecthosts" />Soletra um destino IPv6 de forma diferente nas duas listas, porque cada lista tem seu próprio matcher:

* **`network.allowedDomains`**: a [forma entre colchetes que listas de domínio usam](#ipv6-addresses-in-domain-lists), como `"[::1]"`. O proxy verifica esta lista para admitir a conexão.
* **`injectHosts`**: o endereço nu em sua forma canônica comprimida, como `"::1"` ou `"2001:db8::1"`. O proxy corresponde cada entrada ao endereço de destino nu da conexão, ignorando portas, portanto uma soletração entre colchetes, com ID de zona ou comprimida de forma diferente nunca corresponde e o proxy nunca injeta a credencial lá.

`claude doctor` sinaliza entradas `injectHosts` que nunca podem corresponder com o aviso `Sandbox credential injectHosts entries can never match their destination`. Esta verificação requer Claude Code v2.1.229 ou posterior.

Diferentemente de `deny`, o mascaramento autoriza o proxy a enviar sua credencial real para os hosts listados, portanto Claude Code o honra apenas de configurações que você ou seu administrador controlam: configurações de usuário, configurações gerenciadas e o sinalizador CLI `--settings`. Claude Code ignora entradas `mask` no `.claude/settings.json` ou `.claude/settings.local.json` de um repositório. Nesses arquivos também ignora `network.tlsTerminate` e [`credentials.allowPlaintextInject`](/docs/pt/settings-reference#sandbox-credentials-allowplaintextinject), a configuração que permite ao proxy injetar credenciais em solicitações não criptografadas. Se você [excluir configurações de usuário](#configure-sandboxing), Claude Code descarta as entradas `mask` de variável de ambiente em `~/.claude/settings.json` também.

Quando seu administrador entrega entradas `mask`, `network.tlsTerminate` ou `credentials.allowPlaintextInject` através de configurações gerenciadas pelo servidor, elas contam como [configurações que precisam de aprovação](/docs/pt/server-managed-settings#security-approval-dialogs).

Quando a mesma variável é listada com `deny` em qualquer escopo, `deny` tem precedência.

Mascaramento substitui o valor inteiro da variável por padrão, o que se adequa a um token nu. Campos de entrada opcionais, que requerem Claude Code v2.1.224 ou posterior, lidam com valores com estrutura:

* `extract`: uma expressão regular que Claude Code aplica através do valor, substituindo apenas o texto capturado pelo grupo 1 de cada correspondência, portanto uma ferramenta que analisa o valor, como uma string de conexão `DATABASE_URL`, ainda funciona dentro do sandbox. O padrão deve conter pelo menos um grupo de captura.
* `onExtractNoMatch` controla o que acontece quando o padrão não corresponde a nada:
  * `warn`, o padrão, avisa e passa a variável através desmascarada
  * `deny` desativa a variável dentro do sandbox
  * `error` interrompe a inicialização do sandbox até você corrigir a configuração
* `decode: "jwt"`: para uma variável contendo um JSON Web Token (JWT). Claude Code verifica se o valor é um JWT e o substitui por um token falso estruturalmente válido, portanto o código dentro do sandbox que decodifica o token continua funcionando. Adicione `maskClaims` para listar reivindicações de carga útil de nível superior para mascarar individualmente em vez de substituir o token inteiro; as outras reivindicações permanecem legíveis. Quando o valor não se verifica como um JWT, ou nenhuma reivindicação listada corresponde, Claude Code passa a variável através desmascarada com um aviso. `decode` não pode ser combinado com `extract`.

Consulte as [linhas `credentials.envVars[]` na referência de configurações](/docs/pt/settings-reference#sandbox-settings) para a lista de campos completa.

<h4 id="re-sign-aws-requests">
  Reassine solicitações da AWS
</h4>

Solicitações AWS carregam assinaturas SigV4 sobre o conteúdo da solicitação, portanto mascara `AWS_ACCESS_KEY_ID` e `AWS_SECRET_ACCESS_KEY` juntos. O proxy detecta uma solicitação SigV4 pela sentinela da chave de acesso e a re-assina depois de substituir os valores reais. Mascarar apenas o segredo deixa solicitações assinadas com o espaço reservado, que o proxy não pode detectar, portanto falham na AWS; Claude Code avisa sobre este caso na inicialização, mas não quando apenas a ID da chave de acesso é mascarada. Uma solicitação detectada que o proxy não pode re-assinar, como uma faltando seu cabeçalho `x-amz-date`, falha com um erro de proxy em vez de alcançar o servidor com uma assinatura quebrada.

Claude Code vincula as variáveis convencionais `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` e `AWS_SESSION_TOKEN` em uma credencial automaticamente quando você mascara seus valores inteiros. Se sua credencial AWS reside em variáveis com outros nomes, agrupe-as você mesmo com [`credentials.awsPairs`](/docs/pt/settings-reference#sandbox-credentials-awspairs), que requer Claude Code v2.1.224 ou posterior. Este exemplo adiciona o emparelhamento a uma configuração que já mascara `MY_KEY_ID`, `MY_SECRET_KEY` e `MY_SESSION_TOKEN` inteiros, como na [configuração de mascaramento acima](#mask-environment-variables):

```json theme={null}
{
  "sandbox": {
    "credentials": {
      "awsPairs": [
        {
          "accessKeyIdVar": "MY_KEY_ID",
          "secretAccessKeyVar": "MY_SECRET_KEY",
          "sessionTokenVar": "MY_SESSION_TOKEN"
        }
      ]
    }
  }
}
```

Cada entrada segue estas regras:

* `accessKeyIdVar` e `secretAccessKeyVar` nomeiam as entradas `envVars` mascaradas contendo a ID da chave de acesso e a chave secreta. O `sessionTokenVar` opcional nomeia a entrada contendo o token de sessão para credenciais temporárias; quando definido, o proxy envia o token real como `x-amz-security-token` em solicitações re-assinadas.
* Cada variável nomeada deve ser uma entrada `mask` que mascara seu valor inteiro, sem `extract` ou `decode`.
* O proxy re-assina solicitações nos hosts listados na entrada `injectHosts` da ID da chave de acesso.
* Nomear qualquer uma das variáveis convencionais em um par substitui o emparelhamento automático.

Como entradas `mask`, `awsPairs` é honrado apenas de configurações de usuário, configurações gerenciadas e o sinalizador CLI `--settings`.

Três formas de solicitação AWS carregam assinaturas que o proxy não pode recomputar. Quando tal solicitação é assinada com um espaço reservado de um par mascarado, o proxy falha em vez de encaminhar uma assinatura quebrada; solicitações assinadas com credenciais desmascaradas nunca são afetadas. A configuração [`credentials.sigv4`](/docs/pt/settings-reference#sandbox-credentials-sigv4), que requer Claude Code v2.1.224 ou posterior, relaxa isto por forma: definir a chave de uma forma como `passthrough` encaminha a solicitação com sua assinatura derivada de espaço reservado, portanto a ferramenta chamadora recebe a própria rejeição da AWS em vez de um erro de proxy. Como `awsPairs`, `sigv4` é honrado apenas de configurações de usuário, configurações gerenciadas e o sinalizador CLI `--settings`.

| Forma de solicitação             | Chave `sigv4` | Por que o proxy não pode re-assinar                                                                               |
| :------------------------------- | :------------ | :---------------------------------------------------------------------------------------------------------------- |
| uploads de streaming aws-chunked | `streaming`   | Assinaturas por chunk se encadeiam fora da assinatura de semente, portanto re-assinar exigiria reescrever o corpo |
| URLs pré-assinadas               | `presigned`   | A assinatura reside na URL em si, sem cabeçalho `Authorization`                                                   |
| Assinaturas assimétricas SigV4A  | `sigv4a`      | Não há HMAC de chave compartilhada para recomputar                                                                |

<h4 id="mask-credential-files">
  Mascare arquivos de credenciais
</h4>

Entradas de arquivo também aceitam `"mode": "mask"`, que requer Claude Code v2.1.221 ou posterior. O que um comando em sandbox vê depende da plataforma:

* **Linux e WSL2**: comandos em sandbox leem uma cópia sentinela do arquivo, um substituto cujo segredo é substituído por um valor de espaço reservado, e o [sandbox proxy](#network-isolation) substitui o valor real na saída.
* **macOS**: comandos em sandbox não podem ler o arquivo listado. Claude Code não constrói nenhuma cópia sentinela e não substitui nada na saída, portanto ferramentas que se autenticam com o arquivo não funcionam dentro do sandbox, o mesmo efeito que `deny`. Diferentemente de uma entrada `deny`, o bloqueio de leitura se mantém mesmo quando você [desativa o isolamento do sistema de arquivos](#disable-filesystem-isolation).

Em cada plataforma, Claude Code aplica o requisito [`network.tlsTerminate`](/docs/pt/settings-reference#sandbox-network-tlsterminate) e `injectHosts` da mesma forma que para [variáveis de ambiente mascaradas](#mask-environment-variables), e ignora configurações de repositório da mesma forma. Se você [excluir configurações de usuário](#configure-sandboxing), Claude Code mantém as entradas `mask` de arquivo em `~/.claude/settings.json` como restrições, mas as entradas não autorizam mais o proxy a substituir o valor real.

O exemplo abaixo mascara um token GitHub armazenado em `~/.config/gh/hosts.yml`; o padrão `extract`, coberto abaixo, diz a Claude Code qual parte do arquivo é o segredo. No Linux e WSL2, comandos em sandbox que leem o arquivo obtêm um sentinela no lugar do token, e o proxy substitui o token real em solicitações para `api.github.com`:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com"]
    },
    "credentials": {
      "files": [
        {
          "path": "~/.config/gh/hosts.yml",
          "mode": "mask",
          "extract": "oauth_token:\\s*(\\S+)",
          "injectHosts": ["api.github.com"]
        }
      ]
    }
  }
}
```

Para confirmar que a máscara está ativa, peça a Claude para executar `cat ~/.config/gh/hosts.yml` em um comando em sandbox: no Linux e WSL2 a saída mostra um valor sentinela no lugar do token, e no macOS a leitura falha em vez disso.

No Linux e WSL2, o padrão `extract` é o que mantém o resto de `hosts.yml` legível. Claude Code aplica a expressão regular através de todo o arquivo e substitui apenas o texto capturado pelo grupo 1 de cada correspondência, portanto `gh` ainda analisa sua configuração e apenas o token é um espaço reservado. Use `extract` para qualquer arquivo estruturado que ferramentas analisem, como `.netrc`, JSON ou YAML; o padrão deve conter pelo menos um grupo de captura. Sem `extract`, Claude Code substitui todo o conteúdo do arquivo por um valor sentinela, o que se adequa a um arquivo que contém um único segredo nu e nada mais.

Para um arquivo que contém um JSON Web Token (JWT), defina `decode: "jwt"` em vez de, ou junto com, `extract`. `decode` requer Claude Code v2.1.224 ou posterior. Claude Code encontra candidatos JWT com um padrão integrado, ou com seu padrão `extract` quando definido, verifica se cada candidato é um JWT e o substitui por um token falso estruturalmente válido, portanto o código que decodifica o token dentro do sandbox continua funcionando. Adicione `maskClaims` para mascarar apenas as reivindicações de carga útil de nível superior nomeadas dentro de cada token verificado e deixar as outras reivindicações legíveis. Quando nenhum candidato se verifica, ou nenhuma reivindicação nomeada corresponde, o campo `onExtractNoMatch` abaixo governa o resultado, como faz para um padrão que não corresponde a nada.

Dois campos opcionais refinam como a correspondência se comporta. Ambos se aplicam apenas quando `mode` é `mask` e `extract` ou `decode` está definido. No macOS, Claude Code aplica entradas `mask` como `deny` antes do padrão ser executado sempre que o isolamento do sistema de arquivos está ativado, portanto esses campos e os resultados de não correspondência abaixo têm efeito lá apenas quando [o isolamento do sistema de arquivos está desativado](#disable-filesystem-isolation):

* `onExtractNoMatch` controla o que acontece quando a correspondência não encontra nada para mascarar no arquivo:

  * `warn`, o padrão, avisa e pula a entrada, portanto comandos em sandbox podem ler o arquivo real desmascarado. O padrão se adequa a credenciais que podem estar legitimamente ausentes; se o segredo pode estar presente mas o padrão pode perdê-lo, use `deny`
  * `deny` torna o arquivo ilegível em vez disso
  * `error` interrompe a inicialização do sandbox até você corrigir a configuração

  Claude Code trata `deny` como `error` sempre que o bloqueio de leitura não seria aplicado: quando você [desativa o isolamento do sistema de arquivos](#disable-filesystem-isolation), e quando uma entrada `filesystem.allowRead` de qualquer fonte de configuração reabre o caminho do arquivo.
* `maskDuplicates` também substitui cópias verbatim de cada valor de credencial mascarado, uma captura `extract` ou um token verificado por `decode`, encontrado fora dos intervalos correspondidos, para um segredo repetido onde a correspondência não alcança. Ele corresponde substrings brutas, portanto um valor curto ou comum seria substituído em todos os lugares que aparece; reserve-o para segredos longos e de alta entropia. Padrão: false.

`mask` se aplica a um único arquivo, portanto liste cada arquivo de credencial individualmente. Claude Code recua para `deny` para uma entrada `mask` que não pode mascarar com segurança: um caminho de diretório, um padrão glob, um arquivo maior que 8 MiB ou um arquivo que não é texto UTF-8. Escreva diretórios como entradas `deny` explícitas em vez disso; a tabela em [Which settings can disable it](#which-settings-can-disable-it) cobre se cada forma fixa `filesystem.disabled` e como se comporta com o isolamento do sistema de arquivos desativado.

<h2 id="how-sandboxing-works">
  Como o sandboxing funciona
</h2>

<h3 id="filesystem-isolation">
  Isolamento do sistema de arquivos
</h3>

A ferramenta Bash em sandbox restringe o acesso ao sistema de arquivos a diretórios específicos:

* **Comportamento padrão de escrita**: acesso de leitura e escrita ao diretório de trabalho atual e seus subdiretórios, quaisquer diretórios que você adicionou com `--add-dir`, `/add-dir`, ou [`permissions.additionalDirectories`](/docs/pt/settings-reference#permissions-additionaldirectories), além do diretório temporário da sessão para o qual `$TMPDIR` aponta
* **Comportamento padrão de leitura**: acesso de leitura a todo o computador, exceto certos diretórios negados. Observe que esse padrão ainda permite ler arquivos de credenciais como `~/.aws/credentials` e `~/.ssh/`. Use [`sandbox.credentials`](#protect-credentials) para bloquear leituras desses arquivos e desconfigurar variáveis de ambiente secretas, ou adicione os caminhos a `denyRead`.
* **Acesso bloqueado**: não é possível modificar arquivos fora do diretório de trabalho, diretórios adicionados e diretório temporário da sessão sem permissão explícita, incluindo arquivos de configuração de shell como `~/.bashrc` e binários do sistema em `/bin/`
* **Git worktrees**: quando o diretório de trabalho é um [git worktree vinculado](/docs/pt/worktrees), o sandbox também permite escritas no diretório `.git` compartilhado do repositório principal para que comandos como `git commit` possam atualizar refs e o índice. As escritas em `hooks/` e `config` dentro desse diretório permanecem negadas.
* **Configurável**: defina caminhos permitidos e negados personalizados através de configurações

Para ignorar o isolamento do sistema de arquivos inteiramente mantendo o isolamento de rede, defina [`sandbox.filesystem.disabled`](#disable-filesystem-isolation).

<h3 id="protected-paths">
  Caminhos protegidos
</h3>

Dentro dos diretórios que comandos em sandbox podem escrever, o sandbox ainda nega escritas nos arquivos dos quais Claude Code carrega configuração e código. Um comando que pudesse editar esses arquivos poderia conceder a si mesmo permissões, ou adicionar um hook ou servidor MCP que Claude Code executa fora do sandbox. O sistema de permissões tem seus próprios [caminhos protegidos](/docs/pt/permission-modes#protected-paths), que controlam o que Claude Code aprova antes de uma ferramenta ser executada; a lista do sandbox se aplica a um comando que já está em execução. Ela cobre quatro grupos de caminhos:

* **No seu diretório de trabalho e nos diretórios acima dele**: os arquivos de configurações `.claude`, os diretórios `.claude/skills`, `.claude/agents`, `.claude/commands` e `.claude/hooks`, `.mcp.json`, e os arquivos que Claude Code executa por conta própria, como `.claude/workflows` e `.claude/scheduled_tasks.json`
* **Apenas no seu diretório de trabalho**: arquivos de inicialização de shell como `.bashrc` e `.zshrc`, `.gitconfig`, os diretórios `.vscode` e `.idea`, e `hooks` e `config` dentro de `.git`
* **Arquivos que transformariam seu diretório de trabalho em um repositório git bare**: `HEAD`, `objects` e `refs` no nível superior, além de `config` e `hooks` lá quando um `HEAD` fica ao lado deles. Um arquivo nomeado `config` é negado mesmo sem `HEAD`. No Linux e WSL2, o sandbox exclui um arquivo `HEAD` de nível superior ou diretório `objects` ou `refs` que apareça enquanto um comando em sandbox está em execução
* **Em `~/.claude`, ou no diretório para o qual `CLAUDE_CONFIG_DIR` aponta**: a maioria de seu conteúdo, além de `~/.claude.json` e o armazenamento de credenciais `.credentials.json`

Se um symlink aparecer no caminho de um arquivo de configurações protegidas durante a sessão, o sandbox também nega escritas no arquivo para o qual ele aponta, começando com o próximo comando.

Não há forma de isentar um desses caminhos: uma entrada `allowWrite` ou uma regra de permissão Edit que cubra o caminho não remove a proteção. A única forma de desativar a proteção é [`filesystem.disabled`](#disable-filesystem-isolation), que desativa o isolamento do sistema de arquivos para cada caminho. Para ver a maioria desses caminhos resolvidos para sua máquina, execute `/sandbox` e abra a aba **Config**, que os lista sob **Denied within allowed**, misturados com suas próprias entradas `denyWrite`.

Se `git merge` ou `git checkout` falhar com `unable to unlink old` em um desses caminhos, consulte [Troubleshooting](#troubleshooting).

<h3 id="network-isolation">
  Isolamento de rede
</h3>

O acesso à rede é controlado através de um servidor proxy executado fora do sandbox:

* **Restrições de domínio**: Claude Code não pré-permite nenhum domínio por padrão. Na primeira vez que um comando precisa de um novo domínio, Claude Code solicita aprovação; em [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode), Claude nomeia os hosts que um comando precisa no próprio comando, por [Domínios permitidos por comando](#per-command-allowed-domains-in-auto-mode).
* **Opções de aprovação**: se você escolher Sim quando solicitado, Claude Code permite o host para o resto da sessão atual e não solicita novamente para conexões posteriores ao mesmo host. Se você escolher "Sim, e não pergunte novamente", Claude Code salva uma regra de permissão `WebFetch(domain:...)` em suas [configurações locais](/docs/pt/permissions#permission-system), para que o host permaneça permitido em sessões futuras.
* **Domínios pré-permitidos**: pré-permita domínios com [`allowedDomains`](/docs/pt/settings-reference#sandbox-network-alloweddomains) para evitar o prompt inteiramente. Claude Code também pré-permite domínios de regras de permissão `WebFetch(domain:...)`, conforme descrito em [Regras de permissão](#permission-rules).
* **Allowlist rigorosa**: se você definir [`strictAllowlist`](/docs/pt/settings-reference#sandbox-network-strictallowlist) como `true` em configurações de usuário, gerenciadas ou CLI `--settings`, Claude Code nega aos comandos em sandbox acesso a qualquer host fora da allowlist em vez de solicitar. A allowlist é a mesma contra a qual o sandbox solicita de outra forma: `allowedDomains` mais domínios de regras de permissão `WebFetch(domain:...)`, ou apenas as entradas de configurações gerenciadas quando `allowManagedDomainsOnly` está definido. Claude Code impõe isso apenas para comandos em sandbox; ferramentas em processo como `WebFetch` ainda seguem suas [regras de permissão](#permission-rules). Defini-lo no `.claude/settings.json` ou `.claude/settings.local.json` de um repositório não tem efeito. Requer Claude Code v2.1.219 ou posterior.
* **Bloqueio gerenciado**: se [`allowManagedDomainsOnly`](/docs/pt/settings-reference#sandbox-network-allowmanageddomainsonly) estiver definido em configurações gerenciadas, domínios não permitidos são bloqueados automaticamente em vez de solicitar, e apenas `allowedDomains` e regras de permissão `WebFetch(domain:...)` de configurações gerenciadas são honrados.
* **Proxy corporativo**: quando sua rede requer que o tráfego de saída passe por um proxy corporativo, defina `HTTPS_PROXY`, `HTTP_PROXY` e `NO_PROXY` conforme [configuração de proxy](/docs/pt/network-config#proxy-configuration) descreve, no bloco `env` de suas configurações para que [agentes de fundo](/docs/pt/network-config#set-network-variables-in-settings-not-the-shell) também os obtenham, ou no ambiente a partir do qual você inicia Claude Code. Claude Code impõe a allowlist de domínio e então encaminha conexões permitidas através desse proxy upstream.
* **Suporte a proxy personalizado**: usuários avançados podem implementar regras personalizadas no tráfego de saída
* **Cobertura abrangente**: as restrições se aplicam a todos os scripts, programas e subprocessos gerados por comandos

Em uma regra `WebFetch(domain:...)`, o sandbox honra duas formas de wildcard: um `*.` inicial, como `*.example.com`, e um `*` simples. A forma `*` simples requer Claude Code v2.1.186 ou posterior. Um wildcard em qualquer outra posição, como `WebFetch(domain:example.*)`, ainda corresponde a buscas mas não tem efeito em comandos em sandbox.

<Note>
  O proxy integrado impõe a allowlist com base no nome de host solicitado e, por padrão, não termina ou inspeciona tráfego TLS. A configuração experimental [`network.tlsTerminate`](/docs/pt/settings-reference#sandbox-network-tlsterminate), disponível no Claude Code v2.1.199 e posterior, faz com que o proxy integrado termine TLS em si mesmo, o que as entradas de credenciais [`mask`](#mask-credentials) exigem. Consulte [Limitações de segurança](#security-limitations) para as implicações do padrão, e [Configuração de proxy personalizado](#custom-proxy-configuration) se seu modelo de ameaça exigir inspeção TLS.
</Note>

<h4 id="per-command-allowed-domains-in-auto-mode">
  Domínios permitidos por comando em modo automático
</h4>

Em [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) com sandboxing ativado, Claude nomeia os hosts que um comando precisa no próprio comando em vez de disparar uma aprovação de rede para cada conexão. Cada comando Bash, PowerShell ou [Monitor](/docs/pt/tools-reference#monitor-tool) que é executado no sandbox pode carregar uma lista de hosts além da allowlist do sandbox: um domínio como `registry.npmjs.org`, um wildcard como `*.pythonhosted.org`, ou um endereço IP, cada um com uma porta opcional `:port`. O classificador revisa os hosts junto com o comando. Requer Claude Code v2.1.271 ou posterior.

Uma lista aprovada abre esses hosts apenas para esse comando, enquanto ele é executado. Nada é adicionado aos hosts permitidos da sua sessão ou às suas configurações; o próximo comando nomeia seus próprios hosts.

Um comando que carrega hosts vai para o classificador em vez de ser aprovado por uma regra de permissão ou pelo [modo de aprovação automática](#sandbox-modes) do sandbox. Se uma [regra ask](/docs/pt/permissions#manage-permissions) força um prompt para o comando, o diálogo de permissão em seu terminal lista os hosts ao lado dele, e aprovar lá cobre ambos.

Uma lista por comando amplia apenas o que o sandbox nega por padrão. As entradas [`deniedDomains`](/docs/pt/settings-reference#sandbox-network-denieddomains) ainda bloqueiam. Quando [`strictAllowlist`](/docs/pt/settings-reference#sandbox-network-strictallowlist) ou [`allowManagedDomainsOnly`](/docs/pt/settings-reference#sandbox-network-allowmanageddomainsonly) bloqueia a allowlist, Claude Code recusa listas por comando.

Enquanto listas por comando se aplicam, Claude Code recusa uma conexão a um host que nenhum comando aprovado listou, sem um prompt ou uma verificação do classificador. A recusa nomeia o host no resultado do comando, e Claude executa novamente o comando com o host adicionado.

<h4 id="ipv6-addresses-in-domain-lists">
  Endereços IPv6 em listas de domínios
</h4>

As listas de domínios do sandbox são `allowedDomains`, `deniedDomains` e as regras `WebFetch(domain:...)` que as alimentam. Para corresponder a um endereço IPv6 em qualquer uma delas, escreva o literal entre colchetes: `"[::1]"` corresponde a esse endereço em cada porta, e `"[::1]:443"` corresponde a ele apenas na porta 443. Escreva a porta como um número de 1 a 65535 sem zeros à esquerda. A forma entre colchetes requer Claude Code v2.1.229 ou posterior. Antes da v2.1.229, quando o texto após o último dois-pontos de uma entrada sem colchetes era um número de porta, Claude Code o lia como um, então `::1:443` nomeava o endereço `::1` na porta 443.

Quando você escolhe "Sim, e não pergunte novamente" no prompt de aprovação de rede para um endereço IPv6, Claude Code salva a regra `WebFetch(domain:...)` com o endereço entre colchetes, para que a regra continue correspondendo ao endereço em sessões futuras.

Uma entrada sem colchetes com dois ou mais dois-pontos é ambígua: `::1:443` é tanto um endereço IPv6 completo quanto um endereço seguido por uma porta. Claude Code impõe ortografias ambíguas conservadoramente em vez de adivinhar qual leitura você pretendia:

* **Listas de negação**: Claude Code nega cada leitura que a entrada analisa como, então qualquer leitura que você pretendia é bloqueada. Para uma entrada sem leitura analisável, Claude Code não bloqueia nada.
* **Listas de permissão**: Claude Code nunca permite mais do que você escreveu. Ele reescreve uma entrada ambígua para sua leitura de host-e-porta quando essa leitura analisa de forma limpa, e pode descartar a entrada inteiramente em vez de ampliar a allowlist.

Execute `claude doctor` em seu terminal para encontrar as entradas afetadas: o aviso `Sandbox network domain entries have unreliable spellings` nomeia até três delas e conta o resto. Reescreva cada uma na forma entre colchetes para limpar o aviso. O aviso também nomeia entradas cuja ortografia é não confiável por outras razões, como `@`, caracteres de caminho ou consulta, ou wildcards dentro de colchetes.

<h3 id="os-level-enforcement">
  Imposição no nível do SO
</h3>

A ferramenta Bash em sandbox usa primitivos de segurança do sistema operacional:

* **macOS**: usa Seatbelt para imposição de sandbox
* **Linux**: usa [bubblewrap](https://github.com/containers/bubblewrap) para isolamento
* **WSL2**: usa bubblewrap, igual ao Linux

WSL1 não é suportado porque bubblewrap requer recursos de kernel disponíveis apenas no WSL2.

Esses mesmos primitivos estão disponíveis como o pacote autônomo [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime), que a página [Sandbox environments](/docs/pt/sandbox-environments#sandbox-runtime) aborda como uma abordagem separada para envolver todo o processo do Claude Code.

<h2 id="how-sandboxing-relates-to-permissions-and-permission-modes">
  Como o sandboxing se relaciona com permissões e modos de permissão
</h2>

Sandboxing, [regras de permissão](/docs/pt/permissions), e [modos de permissão](/docs/pt/permission-modes) são camadas complementares. As seções abaixo cobrem como o sandbox interage com cada uma.

<h3 id="permission-rules">
  Regras de permissão
</h3>

Regras de permissão e sandboxing controlam coisas diferentes:

* **Regras de permissão** controlam quais ferramentas o Claude Code pode usar e são avaliadas antes de qualquer ferramenta ser executada. Elas se aplicam a todas as ferramentas: Bash, Read, Edit, WebFetch, MCP e outras, exceto que uma regra de negação ou pergunta não pode bloquear [`EndConversation`](/docs/pt/tools-reference#endconversation-tool-behavior) enquanto qualquer outra ferramenta permanecer.
* **Sandboxing** fornece aplicação em nível de SO que restringe o que os comandos shell podem acessar no nível do sistema de arquivos e rede. Aplica-se apenas aos comandos Bash, PowerShell e [Monitor](/docs/pt/tools-reference#monitor-tool) e seus processos filhos.

As duas camadas também diferem em como são aplicadas. O Claude Code avalia decisões de permissão antes de um comando ser executado, com base na string do comando e, em modo automático, no julgamento de um classificador separado sobre se o comando é seguro. O sistema operacional aplica o limite do sandbox no processo em execução, portanto ele se mantém independentemente do que o modelo escolheu executar e mesmo que um comando permitido faça mais do que seu nome sugere.

Restrições de sistema de arquivos e rede são configuradas através de ambas as configurações de sandbox e regras de permissão:

| Configuração ou regra                                          | O que faz                                                                                                    |
| :------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------- |
| `sandbox.filesystem.allowWrite`                                | Concede acesso de escrita ao subprocesso para caminhos fora do diretório de trabalho                         |
| `sandbox.filesystem.denyWrite` e `sandbox.filesystem.denyRead` | Bloqueiam acesso do subprocesso a caminhos específicos                                                       |
| `sandbox.filesystem.allowRead`                                 | Permite novamente a leitura de caminhos específicos dentro de uma região `denyRead`                          |
| [`sandbox.filesystem.disabled`](#disable-filesystem-isolation) | Desativa a camada de sistema de arquivos inteiramente enquanto mantém isolamento de rede                     |
| Regras de permissão `Edit`                                     | Concedem acesso de escrita a caminhos específicos, da mesma forma que `sandbox.filesystem.allowWrite` faz    |
| Regras de negação `Read` e `Edit`                              | Bloqueiam acesso a arquivos ou diretórios específicos                                                        |
| Regras de permissão `WebFetch(domain:...)`                     | Controlam acesso a domínios                                                                                  |
| `allowedDomains` do Sandbox                                    | Controla quais domínios os comandos Bash podem alcançar                                                      |
| `deniedDomains` do Sandbox                                     | Bloqueia domínios específicos mesmo quando um wildcard `allowedDomains` mais amplo permitiria de outra forma |

Caminhos e domínios das configurações de sandbox e regras de permissão são mesclados na configuração final do sandbox.

O [diretório de exemplos do repositório claude-code](https://github.com/anthropics/claude-code/tree/main/examples/settings) inclui configurações de configurações iniciais para cenários de implantação comuns, incluindo exemplos específicos de sandbox. Use-os como pontos de partida e ajuste-os para suas necessidades.

<h3 id="permission-modes">
  Modos de permissão
</h3>

`/sandbox` não é um [modo de permissão](/docs/pt/permission-modes). Modos de permissão decidem se uma chamada de ferramenta é executada e se você é solicitado primeiro, enquanto o sandbox restringe o que um comando Bash pode acessar uma vez que é executado. Eles diferem no que controlam e o que substitui o prompt por ação:

|                                                                          | O que controla                                             | O que substitui o prompt                                                                                                                                                                                          |
| :----------------------------------------------------------------------- | :--------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/sandbox`                                                               | O que um comando Bash pode acessar uma vez que é executado | O limite do sandbox em si, em [modo auto-allow](#sandbox-modes)                                                                                                                                                   |
| [Modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) | Se cada chamada de ferramenta é executada                  | Um classificador que revisa ações                                                                                                                                                                                 |
| `--dangerously-skip-permissions`                                         | Se cada chamada de ferramenta é executada                  | Nada. Verificações de [caminho protegido](/docs/pt/permission-modes#protected-paths) também são ignoradas; as [ações que nenhum modo auto-aprova](/docs/pt/permission-modes#actions-no-mode-auto-approves) ainda se aplicam |

O [modo auto-allow](#sandbox-modes) do sandbox é separado do [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode): auto-allow aprova comandos Bash porque o limite do sandbox os contém, enquanto modo automático usa um classificador para revisar ações. Os dois funcionam independentemente e podem ser combinados, com as exceções listadas em [Modos de sandbox](#sandbox-modes). Para escolher um limite de isolamento para execuções autônomas, consulte [Ambientes de sandbox](/docs/pt/sandbox-environments#how-isolation-relates-to-permission-modes). Para uma tabela de emparelhamentos comuns de modo de permissão e sandbox com os sinalizadores que iniciam cada um, consulte [Configurações comuns](/docs/pt/permission-modes#common-setups).

<h2 id="configure-the-sandbox-for-your-organization">
  Configure o sandbox para sua organização
</h2>

Administradores podem exigir sandboxing para cada usuário, impedir que desenvolvedores ampliem a política e rotear tráfego de sandbox através de um proxy corporativo.

<h3 id="enforce-sandboxing-with-managed-settings">
  Imponha o sandboxing com configurações gerenciadas
</h3>

Para exigir o sandbox para cada desenvolvedor, entregue as chaves `sandbox` através de [managed settings](/docs/pt/managed-settings#delivery-mechanisms), seja como um arquivo gerenciado pelo seu MDM ou através de [server-managed settings](/docs/pt/server-managed-settings) no claude.ai.

A seguinte configuração de managed settings habilita o sandbox, recusa iniciar Claude Code se o sandbox não conseguir inicializar e impede que o modelo tente novamente comandos fora do sandbox:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true,
    "allowUnsandboxedCommands": false
  }
}
```

As duas chaves além de `enabled` controlam o que acontece quando o sandbox não consegue executar um comando:

* **`failIfUnavailable`**: uma dependência faltante como bubblewrap no Linux bloqueia Claude Code de iniciar em vez de mostrar um aviso e voltar a execução sem sandbox
* **`allowUnsandboxedCommands: false`**: Claude Code ignora o escape hatch `dangerouslyDisableSandbox`, portanto quando um comando falha sob o sandbox, Claude não consegue retentá-lo sem sandbox

Duas adições valem a pena considerar junto com elas. Adicione `excludedCommands` para qualquer ferramenta aprovada pela organização que deve ser executada sem isolamento. Adicione entradas [`sandbox.credentials`](#protect-credentials) para diretórios de credenciais como `~/.aws` e `~/.ssh` e para variáveis de ambiente secretas, já que a política de leitura padrão ainda permite.

Esta configuração coloca em sandbox os comandos que Claude executa. Um desenvolvedor ainda pode digitar um comando no [prompt de shell-mode com `!`](/docs/pt/interactive-mode#shell-mode-with-prefix) e executá-lo fora do sandbox, com o mesmo acesso que já possui em qualquer terminal fora do Claude Code. Veja [The unsandboxed retry escape hatch](#the-unsandboxed-retry-escape-hatch) para as sessões onde comandos digitados são executados em sandbox.

O sandbox não é executado no Windows nativo, portanto se sua frota inclui hosts Windows, escope esta configuração para macOS e Linux ou tenha esses usuários executarem Claude Code dentro do WSL2 ou um container.

<h3 id="keep-developers-from-widening-the-policy">
  Impeça que desenvolvedores ampliem a política
</h3>

Para chaves booleanas como `enabled` e `failIfUnavailable`, Claude Code usa o valor gerenciado e ignora qualquer coisa que um desenvolvedor defina localmente. Para chaves de array como `excludedCommands` e `allowRead`, Claude Code mescla entradas de cada escopo que a sessão carrega, portanto um desenvolvedor pode anexar entradas que ampliem a política.

Defina `allowManagedReadPathsOnly` como `true` em managed settings para que apenas entradas `allowRead` de managed settings sejam honradas. Isso impede que desenvolvedores ampliem o acesso de leitura além dos caminhos aprovados pela organização. Para bloquear domínios de rede para os valores gerenciados da mesma forma, defina [`allowManagedDomainsOnly`](/docs/pt/settings-reference#sandbox-network-allowmanageddomainsonly).

Quando managed settings configuram `sandbox.filesystem` ou listam qualquer entrada `sandbox.credentials.files` com `"mode": "deny"`, apenas managed settings podem definir [`filesystem.disabled`](#disable-filesystem-isolation), portanto desenvolvedores não conseguem desativar restrições de filesystem implantadas pelo administrador. Se uma entrada `mask` fixa a chave depende de como ela se resolve; a tabela sob [Which settings can disable it](#which-settings-can-disable-it) cobre os quatro casos.

`excludedCommands` não tem um equivalente de lockdown apenas gerenciado, portanto um desenvolvedor sempre pode anexar entradas que executem comandos adicionais fora do sandbox. Mantenha a lista gerenciada estreita.

<h3 id="custom-proxy-configuration">
  Configuração de proxy personalizado
</h3>

Para organizações que exigem segurança de rede avançada, você pode implementar um proxy personalizado para:

* Descriptografar e inspecionar tráfego HTTPS
* Aplicar regras de filtragem personalizadas
* Registrar todas as solicitações de rede
* Integrar com infraestrutura de segurança existente

Para apontar Claude Code para seu proxy, defina as portas de proxy em [sandbox settings](/docs/pt/settings-reference#sandbox-settings):

```json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080,
      "socksProxyPort": 8081
    }
  }
}
```

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Alguns comandos falham dentro do sandbox mesmo que funcionem fora dele. As correções abaixo abrangem os casos mais comuns.

* **Comandos falham com um erro host-not-allowed**: muitas ferramentas CLI precisam alcançar hosts específicos. Conceder permissão quando solicitado adiciona o host à sua lista de permitidos para que a ferramenta seja executada dentro do sandbox no futuro.
* **`jest` trava ou falha**: `watchman` é incompatível com o sandbox. Execute `jest --no-watchman` em vez disso.
* **CLIs baseadas em Go falham na verificação TLS no macOS**: ferramentas como `gh`, `gcloud` e `terraform` podem falhar na verificação TLS sob Seatbelt. Liste essas ferramentas em [`excludedCommands`](/docs/pt/settings-reference#sandbox-excludedcommands). Se você estiver usando `httpProxyPort` com um proxy MITM e CA personalizado, defina [`enableWeakerNetworkIsolation`](/docs/pt/settings-reference#sandbox-enableweakernetworkisolation) como `true` em vez disso.
* **`open`, `osascript`, ou fluxos de autenticação baseados em navegador falham com erro `-600` no macOS**: o sandbox bloqueia Apple Events por padrão. Defina [`allowAppleEvents`](/docs/pt/settings-reference#sandbox-allowappleevents) como `true` em suas configurações de usuário, gerenciadas ou CLI para permitir. As configurações do projeto são ignoradas para esta chave. Habilitá-lo remove o isolamento de execução de código, pois comandos em sandbox podem então iniciar outras aplicações sem sandbox sem prompt do usuário e enviar comandos AppleScript para aplicações em execução, sujeito ao prompt de consentimento de automação do macOS (TCC). Alternativamente, adicione o comando a [`excludedCommands`](/docs/pt/settings-reference#sandbox-excludedcommands).
* **Comandos `docker` falham**: `docker` é incompatível com o sandbox. Adicione `docker *` a [`excludedCommands`](/docs/pt/settings-reference#sandbox-excludedcommands).
* **`pbcopy`, `xclip`, ou `wl-copy` não atualiza a área de transferência**: esses utilitários de área de transferência podem falhar ao alcançar a área de transferência do sistema de dentro do sandbox, caso em que o texto canalizado para eles não chega.

  Para colocar a saída do Claude em sua área de transferência, peça ao Claude para imprimi-la em sua resposta e execute [`/copy`](/docs/pt/commands). `/copy` escreve na área de transferência do processo Claude Code em vez de um comando em sandbox.

  Quando Claude canaliza texto para uma dessas ferramentas, adicionar a ferramenta a [`excludedCommands`](/docs/pt/settings-reference#sandbox-excludedcommands) não tira essa chamada do sandbox por si só.
* **Um comando git falha com `unable to unlink old`**: `git merge`, `git checkout` e comandos similares falham dessa forma quando precisam substituir um arquivo que o sandbox nega gravações, seja esse arquivo sob um [caminho protegido](#protected-paths) como `.claude/skills`, sob uma de suas entradas `denyWrite`, ou fora dos diretórios que o sandbox permite que comandos gravem. No Linux e WSL2 o erro termina com `Read-only file system`.

  Após a falha, Claude pode [oferecer executar novamente o comando fora do sandbox](#the-unsandboxed-retry-escape-hatch); aprove essa nova tentativa ou execute o comando git você mesmo em outro terminal. Se você definiu `allowUnsandboxedCommands` como `false`, Claude não pode oferecer a nova tentativa, então execute o comando você mesmo. Se o mesmo comando git falhar frequentemente, adicione-o a [`excludedCommands`](/docs/pt/settings-reference#sandbox-excludedcommands).
* **Bubblewrap falha ao iniciar dentro de um container**: em um container sem privilégios, bubblewrap não consegue montar um sistema de arquivos `/proc` fresco, então comandos em sandbox falham com um erro `bwrap` como `Can't mount proc on /newroot/proc: Operation not permitted`. Defina [`enableWeakerNestedSandbox`](/docs/pt/settings-reference#sandbox-enableweakernestedsandbox) como `true` para que o sandbox interno faça bind-mount do `/proc` existente do container em vez disso. Use esta configuração apenas quando o container externo já fornece o limite de isolamento que você precisa, pois expõe informações de processo a comandos em sandbox que uma montagem `/proc` fresca ocultaria.
* **Arquivos de somente leitura de 0 bytes aparecem em caminhos de configurações `.claude`, e "Sim, e não pergunte novamente" não salva**: no Linux e WSL2, o sandbox mantém uma negação de gravação em um arquivo que ainda não existe criando um espaço reservado de somente leitura de 0 bytes lá enquanto um comando em sandbox é executado. O sandbox remove o espaço reservado depois. Se uma sessão for encerrada antes dessa limpeza ser executada, por exemplo por SIGKILL, os espaços reservados permanecem. Sessões posteriores os vinculam como somente leitura novamente a cada início, então uma gravação de configurações como salvar uma escolha de permissão falha onde um está.

  Execute `claude doctor` para listar os arquivos de espaço reservado restantes. O aviso [`Stale sandbox mask files left by a killed session`](/docs/pt/errors#stale-sandbox-mask-files-left-by-a-killed-session) nomeia até três deles e conta o resto. Delete cada arquivo com `rm` enquanto nenhuma outra sessão Claude Code está sendo executada nesse projeto. Antes da v2.1.257, Claude Code deixava os mesmos espaços reservados para trás sem sinalizá-los.
* **`--dangerously-skip-permissions` falha como root**: este sinalizador é bloqueado ao executar como root ou via sudo no Linux e macOS, porque acesso root combinado com nenhum prompt de permissão pode modificar qualquer arquivo ou serviço no sistema. A verificação é ignorada automaticamente dentro de um sandbox reconhecido. Para executar autonomamente em um container, use a configuração [dev container](/docs/pt/devcontainer), que executa Claude Code como um usuário não-root.

<h2 id="limitations">
  Limitações
</h2>

Sandboxing reduz risco, mas não é um limite de isolamento completo. Revise as limitações abaixo antes de confiar nele como um controle de segurança difícil.

<h3 id="security-limitations">
  Limitações de segurança
</h3>

* **Filtragem de rede**: o sandbox restringe quais domínios os processos podem se conectar. Por padrão, o proxy integrado não termina ou inspeciona TLS no tráfego de saída, portanto o conteúdo de conexões criptografadas não é examinado. A configuração experimental [`network.tlsTerminate`](/docs/pt/settings-reference#sandbox-network-tlsterminate) termina TLS no proxy para [substituição de credenciais `mask`](#mask-credentials), mas não adiciona filtragem de conteúdo. Você é responsável por garantir que apenas domínios confiáveis sejam permitidos em sua política.

<Warning>
  Permitir domínios amplos como `github.com` pode criar caminhos para exfiltração de dados. Como o proxy toma sua decisão de permissão do nome de host fornecido pelo cliente sem inspecionar TLS, código executado dentro do sandbox pode potencialmente usar [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) ou técnicas similares para alcançar hosts fora da allowlist. Se seu modelo de ameaça exigir garantias mais fortes, configure um [custom proxy](#custom-proxy-configuration) que termine TLS e inspecione tráfego, e instale seu certificado CA dentro do sandbox. Isolamento de rede mais forte e consciente de TLS é uma área ativa de desenvolvimento.
</Warning>

* **Escalação de privilégio via Unix sockets**: a configuração `allowUnixSockets` pode inadvertidamente conceder acesso a serviços do sistema que poderiam levar a bypasses de sandbox. Por exemplo, permitir acesso a `/var/run/docker.sock` efetivamente concede acesso ao sistema host através do socket Docker. Considere cuidadosamente quaisquer Unix sockets que você permita através do sandbox.
* **Escalação de permissão de sistema de arquivos**: permissões de escrita de sistema de arquivos excessivamente amplas podem habilitar ataques de escalação de privilégio. Permitir escritas em diretórios contendo executáveis em `$PATH`, diretórios de configuração do sistema ou arquivos de configuração de shell do usuário como `.bashrc` ou `.zshrc` pode levar a execução de código em diferentes contextos de segurança quando outros usuários ou processos do sistema acessam esses arquivos.
* **Força do sandbox Linux**: a implementação Linux fornece isolamento forte de sistema de arquivos e rede, mas inclui um modo `enableWeakerNestedSandbox` que o habilita a funcionar dentro de ambientes Docker sem namespaces privilegiados, ou em hosts Linux onde namespaces de usuário sem privilégios são desabilitados por sysctl. Esta opção enfraquece consideravelmente a segurança e deve ser usada apenas quando isolamento adicional é de outra forma imposto.
* **Apple Events no macOS**: o sandbox macOS bloqueia Apple Events por padrão. A configuração `allowAppleEvents` remove essa restrição para que ferramentas como `open` e `osascript` funcionem, mas remove isolamento de execução de código: comandos em sandbox podem iniciar outras aplicações sem sandbox sem nenhum prompt do usuário, e podem enviar comandos AppleScript para aplicações em execução, sujeito ao prompt de consentimento de automação macOS por aplicativo (TCC). Isso é apenas honrado a partir de configurações de usuário, gerenciadas ou CLI. Configurações de projeto não podem habilitá-lo.

<h3 id="platform-and-tool-compatibility">
  Compatibilidade de plataforma e ferramentas
</h3>

* **Suporte de plataforma**: suporta macOS, Linux e WSL2. WSL1 e Windows nativo não são suportados.
* **Overhead de desempenho**: mínimo, mas algumas operações de sistema de arquivos podem ser ligeiramente mais lentas.
* **Compatibilidade de ferramentas**: algumas ferramentas que exigem padrões de acesso específicos do sistema podem precisar de ajustes de configuração, ou podem precisar ser executadas fora do sandbox.

<h3 id="scope">
  Escopo
</h3>

O sandbox isola subprocessos Bash. Outras ferramentas operam sob limites diferentes:

* **Ferramentas de arquivo integradas**: Read, Edit e Write usam o sistema de permissão diretamente em vez de serem executadas através do sandbox. Consulte [permissions](/docs/pt/permissions).
* **Computer use**: quando Claude abre aplicativos e controla sua tela, ele é executado em seu desktop real em vez de em um ambiente isolado. Prompts de permissão por aplicativo controlam cada aplicativo. Consulte [computer use in the CLI](/docs/pt/computer-use) ou [computer use in Desktop](/docs/pt/desktop#let-claude-use-your-computer).
* **Variáveis de ambiente**: comandos Bash em sandbox herdam o ambiente do processo pai por padrão, incluindo quaisquer credenciais definidas lá. Use [`sandbox.credentials`](#protect-credentials) para remover ou mascarar variáveis específicas para comandos em sandbox, ou defina [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/pt/env-vars) para remover credenciais de todos os subprocessos.
* **Subagents**: [subagents](/docs/pt/sub-agents) são executados no mesmo processo que a sessão pai e usam a mesma configuração de sandbox. Comandos Bash dentro de um subagent são colocados em sandbox quando sandboxing está habilitado na sessão pai.

<Warning>
  Sandboxing eficaz requer isolamento tanto de sistema de arquivos quanto de rede. Sem isolamento de rede, um agente comprometido poderia exfiltrar arquivos sensíveis como chaves SSH. Sem isolamento de sistema de arquivos, seja de uma política permissiva ou de [desabilitar a camada de sistema de arquivos](#disable-filesystem-isolation), um agente comprometido poderia fazer backdoor de recursos do sistema para obter acesso à rede. Quando você amplia os padrões, verifique que um caminho `allowWrite`, uma entrada `allowedDomains` ampla ou uma exceção `excludedCommands` não desfaz uma restrição no outro lado.
</Warning>

<h2 id="see-also">
  Veja também
</h2>

* [Sandbox environments](/docs/pt/sandbox-environments): compare o sandbox integrado com dev containers, containers e VMs
* [Security](/docs/pt/security): recursos de segurança abrangentes e melhores práticas
* [Permissions](/docs/pt/permissions): configuração de permissão e controle de acesso
* [All settings](/docs/pt/settings-reference): todas as chaves de configuração
* [CLI reference](/docs/pt/cli-reference): opções de linha de comando
