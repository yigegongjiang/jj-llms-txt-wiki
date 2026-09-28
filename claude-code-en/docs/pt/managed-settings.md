> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Implantar configurações gerenciadas

> Implante configurações gerenciadas na máquina de cada desenvolvedor: mecanismos de entrega por SO, como Claude Code combina fontes gerenciadas e como verificar a aplicação.

Configurações gerenciadas são as configurações que sua organização implanta na máquina de cada desenvolvedor. Claude Code as aplica acima de todos os outros níveis, portanto nenhum valor de usuário, projeto, local ou `--settings` as substitui, exceto por algumas [exceções sensíveis à segurança](/docs/pt/settings#exceptions-to-managed-settings-precedence) onde um valor mais restritivo de um nível inferior ainda conta.

Esta página é para o administrador que implanta configurações gerenciadas ou depura por que uma não está sendo aplicada. Para decidir o que impor, comece com a tabela [Decidir o que impor](/docs/pt/admin-setup#decide-what-to-enforce). Para o caminho do console claude.ai, consulte [Configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings). Para saber em qual arquivo os valores próprios do desenvolvedor vão, consulte [Configurações](/docs/pt/settings).

<h2 id="deploy-a-managed-settings-file">
  Implantar um arquivo de configurações gerenciadas
</h2>

Esta é a forma mais rápida de colocar uma política em cada máquina: um arquivo `managed-settings.json`. Se você ainda não escolheu como entregar configurações gerenciadas, ou seus dispositivos estão sob MDM ou os desenvolvedores executam sessões na nuvem, leia primeiro [Escolher um mecanismo de entrega](#choose-a-delivery-mechanism).

<Steps>
  <Step title="Escrever managed-settings.json">
    Escreva um `managed-settings.json` que contenha as chaves que você decidiu impor, na mesma forma JSON que `settings.json`. A tabela [Decidir o que impor](/docs/pt/admin-setup#decide-what-to-enforce) lista as chaves por trás de cada controle, e cada entrada na [referência de configurações](/docs/pt/settings-reference) diz se uma fonte gerenciada pode defini-la. Este arquivo bloqueia duas leituras de arquivo, desativa o modo de bypass e faz Claude Code ignorar regras de permissão de arquivos de usuário, projeto e local e de `--allowedTools`:

    ```json managed-settings.json theme={null}
    {
      "permissions": {
        "deny": [
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    Para um exemplo mais completo que mostra a forma de mais chaves gerenciadas, incluindo o método de login, modelos, servidores MCP e marketplaces, consulte [Configurações gerenciadas de uma organização](/docs/pt/settings-example#an-organizations-managed-settings).
  </Step>

  <Step title="Colocar o arquivo em cada máquina">
    Salve o arquivo como `managed-settings.json` no diretório do sistema para o sistema operacional, usando qualquer ferramenta que já coloque arquivos em sua frota:

    * **macOS**: `/Library/Application Support/ClaudeCode/managed-settings.json`
    * **Linux e WSL**: `/etc/claude-code/managed-settings.json`
    * **Windows**: `C:\Program Files\ClaudeCode\managed-settings.json`
  </Step>

  <Step title="Confirmar que a política foi aplicada">
    Em uma máquina, execute `/status` dentro de Claude Code. A linha `Setting sources` mostra `Enterprise managed settings (file)`. Implante no resto da frota depois disso; [Verificar que uma política está em vigor](#check-that-a-policy-is-in-force) cobre o que observar quando a linha está faltando.
  </Step>
</Steps>

<span id="managed-settings-delivery" />

<span id="delivery-mechanisms" />

<h2 id="choose-a-delivery-mechanism">
  Escolher um mecanismo de entrega
</h2>

O arquivo nas etapas acima é uma de quatro formas de colocar configurações gerenciadas em uma máquina. Cada mecanismo carrega as mesmas chaves de política que um arquivo `settings.json`, portanto a [referência de configurações](/docs/pt/settings-reference) se aplica a todos eles. Algumas chaves estão vinculadas a fontes particulares, e a linha Scope de cada entrada diz qual:

* **Controles de entrega**: [`policyHelper`](/docs/pt/settings-reference#policyhelper), [`wslInheritsWindowsSettings`](/docs/pt/settings-reference#wslinheritswindowssettings) e [`managedSourcesBehavior`](/docs/pt/settings-reference#managedsourcesbehavior)
* **Chaves de login do gateway**: [`forceLoginGatewayUrl`](/docs/pt/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/pt/settings-reference#gatewayinternalnetworks) e o valor `"gateway"` de [`forceLoginMethod`](/docs/pt/settings-reference#forceloginmethod)

Um arquivo de configurações gerenciadas, um perfil MDM ou o console claude.ai aplica uma política a todos que alcança. Para dar a um grupo de desenvolvedores uma política diferente, implante um arquivo ou perfil diferente para esse grupo; o console claude.ai [ainda não pode direcionar um grupo](/docs/pt/server-managed-settings#current-limitations), enquanto um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) auto-hospedado entrega configurações gerenciadas por grupo IdP.

Quando mais de um mecanismo entrega uma política para a mesma máquina, Claude Code por padrão usa um e ignora os outros. [Como Claude Code combina fontes gerenciadas](#how-claude-code-combines-managed-sources) fornece a ordem e o opt-in que aplica cada fonte.

As linhas MDM e arquivo são chamadas juntas de configurações gerenciadas por endpoint, porque a política é armazenada no dispositivo do desenvolvedor, em oposição à linha gerenciada pelo servidor, onde Claude Code a busca.

Escolha um mecanismo por como você já gerencia dispositivos, usando a tabela abaixo.

| Mecanismo                                                              | Como você o entrega                                                                                                                                                                                                                          | Quando Claude Code o lê                                                                                                                                                                                                                                                                 | Use quando                                                                                        |
| :--------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------ |
| [Configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) | No console de administração claude.ai, ou em um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) auto-hospedado                                                                                                                      | Buscado na inicialização e pesquisado a cada hora; consulte [alterações que precisam de aprovação](#where-and-when-a-policy-applies)                                                                                                                                                    | Você quer um lugar para alterar a política de uma organização claude.ai sem tocar em cada máquina |
| Política MDM ou nível do SO                                            | Como um perfil de configuração macOS ou um valor de registro `HKLM` do Windows, através de Jamf, Intune, Group Policy ou uma ferramenta similar; consulte [onde cada mecanismo armazena a política](#where-each-mechanism-stores-the-policy) | Lido na inicialização e verificado quanto a alterações a cada 30 minutos                                                                                                                                                                                                                | Você já gerencia dispositivos com MDM ou Group Policy                                             |
| Baseado em arquivo                                                     | Como `managed-settings.json` em um diretório do sistema em cada máquina; consulte [onde cada mecanismo armazena a política](#where-each-mechanism-stores-the-policy)                                                                         | Lido na inicialização e recarregado quando um arquivo muda                                                                                                                                                                                                                              | Máquinas sem MDM, hosts Linux ou imagens que você constrói você mesmo                             |
| Registro HKCU, Windows e WSL                                           | Como um valor de registro `HKCU` do Windows; consulte [onde cada mecanismo armazena a política](#where-each-mechanism-stores-the-policy)                                                                                                     | Lido na inicialização e verificado quanto a alterações a cada 30 minutos; Claude Code o usa apenas quando nenhuma outra fonte gerenciada entrega uma chave de política e nenhuma [configuração pai fornecida pelo host](#let-an-embedding-host-add-policy) fornece uma chave restritiva | Você não pode escrever a chave `HKLM` de nível de máquina                                         |

Modelos iniciais para Jamf, Iru, Intune e Group Policy estão no [repositório de exemplos MDM](https://github.com/anthropics/claude-code/tree/main/examples/mdm).

Para servidores MCP gerenciados, que você implanta junto com qualquer um destes através de `managed-mcp.json` ou fornece através da chave [`managedMcpServers`](/docs/pt/settings-reference#managedmcpservers), consulte [Configuração MCP gerenciada](/docs/pt/managed-mcp).

<h3 id="where-and-when-a-policy-applies">
  Onde e quando uma política se aplica
</h3>

Uma política implantada alcança as sessões do desenvolvedor da seguinte forma:

* **Superfícies**: na máquina do desenvolvedor, o terminal, as extensões VS Code e JetBrains, a aba Code do aplicativo desktop e as sessões [Agent SDK](/docs/pt/agent-sdk/typescript) leem todas essas fontes. As sessões Agent SDK carregam configurações gerenciadas mesmo quando `settingSources` exclui os arquivos de usuário, projeto e local.
* **Sessões na nuvem**: uma sessão em um ambiente hospedado pela Anthropic não lê um perfil MDM ou arquivo de dispositivo, portanto a política para ela deve vir de configurações gerenciadas pelo servidor. Uma sessão em um [ambiente auto-hospedado](/docs/pt/self-hosted-environments) também lê o arquivo de configurações gerenciadas em sua imagem de executor, por padrão apenas quando as configurações gerenciadas pelo servidor não entregam nenhuma chave de política, exceto pelas [chaves que Claude Code lê de cada fonte de administrador](#keys-read-from-every-admin-source). [Como Claude Code combina fontes gerenciadas](#how-claude-code-combines-managed-sources) cobre o opt-in que aplica ambas.
* **Sessões Cowork**: [Cowork](https://claude.com/docs/cowork/overview) no aplicativo Claude Desktop executa suas sessões em Claude Code. Em uma sessão Cowork, Claude Code nunca busca configurações gerenciadas pelo servidor do console de administração claude.ai, mesmo quando o usuário se conecta com uma conta Team ou Enterprise, portanto qual política se aplica depende de onde a sessão é executada:

  * **Na máquina do usuário**: por padrão, Claude Code em uma sessão Cowork lê a política MDM ou nível do SO e o arquivo de configurações gerenciadas nesse dispositivo, portanto implante a política lá.
  * **Em um sandbox de VM completa**: quando sua configuração gerenciada do Claude Desktop define [`requireCoworkFullVmSandbox`](https://claude.com/docs/third-party/claude-desktop/configuration#requirecoworkfullvmsandbox), Claude Code é executado dentro de uma máquina virtual onde a política MDM do dispositivo e o arquivo de configurações gerenciadas não estão presentes.
  * **Sessões Cowork remotas**: estas são executadas em VMs gerenciadas pela Anthropic, onde Claude Code não tem política de dispositivo para ler.

  Onde quer que a sessão seja executada, claude.ai aplica as listas [`strictKnownMarketplaces`](/docs/pt/settings-reference#strictknownmarketplaces) e [`blockedMarketplaces`](/docs/pt/settings-reference#blockedmarketplaces) do console de administração quando alguém adiciona um marketplace de um repositório git em claude.ai ou de **Customize** na aba Cowork. [Como as restrições funcionam](/docs/pt/plugins/org#restrict-what-users-can-install) descreve essa verificação. A tabela [cobertura de superfície](/docs/pt/model-config#surface-coverage) compara Cowork com as outras superfícies.
* **Sessões em execução**: a maioria das alterações alcança uma sessão em execução no cronograma na [tabela de mecanismo de entrega](#choose-a-delivery-mechanism), sem uma reinicialização.
  * Alterações em [`forceRemoteSettingsRefresh`](/docs/pt/settings-reference#forceremotesettingsrefresh), [`requiredMinimumVersion`](/docs/pt/settings-reference#requiredminimumversion) e [algumas chaves editáveis pelo usuário](/docs/pt/settings#when-edits-take-effect) entram em vigor na próxima inicialização de sessão.
  * Uma entrada [`policyHelper`](/docs/pt/settings-reference#policyhelper) nova ou alterada entra em vigor no próximo lançamento. Se configurações gerenciadas pelo servidor sombrearem o auxiliar nesse lançamento, o auxiliar é executado assim que uma busca relata que essas configurações foram removidas.
* **Alterações que precisam de aprovação**: além das [atualizações que aguardam o próximo lançamento](/docs/pt/server-managed-settings#fetch-and-caching-behavior), uma alteração gerenciada pelo servidor em uma configuração que [precisa de aprovação](/docs/pt/server-managed-settings#security-approval-dialogs), como um hook ou uma variável `env`, aguarda o desenvolvedor aceitar o diálogo em uma sessão interativa e se aplica para a execução atual em uma sessão que uma extensão IDE ou o Agent SDK hospeda. Outras alterações gerenciadas pelo servidor se aplicam na próxima pesquisa.
* **Sessões de longa duração**: uma sessão deixada aberta por semanas ainda pode ficar para trás em um lançamento. [`requiredMinimumVersion`](/docs/pt/settings-reference#requiredminimumversion) bloqueia um binário desatualizado de iniciar e não encerra uma sessão que já está em execução.

<span id="format-the-policy-for-each-platform" />

<h3 id="where-each-mechanism-stores-the-policy">
  Onde cada mecanismo armazena a política
</h3>

As chaves são as mesmas em todos os lugares, mas cada mecanismo as armazena em um lugar e forma diferentes:

* **Gerenciado pelo servidor**: os servidores da Anthropic, ou seu gateway, mantêm a política. Claude Code mantém um cache local que aplica na inicialização e [substitui em cada busca bem-sucedida](/docs/pt/server-managed-settings#security-considerations).
* **Perfil de configuração macOS**: o domínio de preferências gerenciadas `com.anthropic.claudecode`. Use as mesmas chaves de nível superior que `managed-settings.json`, com configurações aninhadas como dicionários e listas como arrays plist.
* **Registro HKLM do Windows**: o JSON como um valor `REG_SZ` ou `REG_EXPAND_SZ` nomeado `Settings` sob `HKLM\SOFTWARE\Policies\ClaudeCode`.
* **Baseado em arquivo**: `managed-settings.json`, um diretório opcional `managed-settings.d/` e `managed-mcp.json` no diretório do sistema: `/Library/Application Support/ClaudeCode/` no macOS, `/etc/claude-code/` no Linux e WSL, e `C:\Program Files\ClaudeCode\` no Windows. Claude Code não lê o caminho legado do Windows `C:\ProgramData\ClaudeCode\managed-settings.json`.
* **Registro HKCU do Windows**: o mesmo valor `Settings` sob `HKCU\SOFTWARE\Policies\ClaudeCode`.

<h3 id="split-a-file-based-policy-across-teams">
  Dividir uma política baseada em arquivo entre equipes
</h3>

Se várias equipes possuem partes de uma política, coloque cada parte em seu próprio arquivo em `managed-settings.d/`, ao lado de `managed-settings.json` no mesmo diretório do sistema, em vez de editar um arquivo compartilhado.

Claude Code mescla `managed-settings.json` primeiro, depois cada arquivo `*.json` no diretório em ordem alfabética. Nomeie os arquivos com prefixos numéricos para controlar a ordem, como `10-telemetry.json` e `20-security.json`. Claude Code ignora arquivos ocultos e arquivos que não terminam em `.json`.

Quando dois arquivos definem a mesma chave, Claude Code os combina por estas regras:

* **Valores únicos**, como `"model": "opus"` ou `"cleanupPeriodDays": 7`: o valor do arquivo posterior substitui o anterior
* **Listas**, como `permissions.deny` ou `sandbox.network.allowedDomains`: as duas listas se combinam, com duplicatas removidas
* **Blocos aninhados**, como `env` ou `sandbox`: os dois blocos se mesclam chave por chave, e cada chave dentro segue essas mesmas regras
* **`fallbackModel`**: a cadeia posterior substitui a anterior inteira
* **[`extraKnownMarketplaces`](/docs/pt/settings-reference#extraknownmarketplaces) e [`managedMcpServers`](/docs/pt/settings-reference#managedmcpservers)**: uma entrada posterior com o mesmo nome substitui a anterior inteira
* **[`modelPicker`](/docs/pt/settings-reference#modelpicker)**: o lineup posterior substitui o anterior inteiro

<span id="precedence-within-the-managed-tier" />

<span id="which-managed-source-claude-code-uses" />

<h2 id="how-claude-code-combines-managed-sources">
  Como Claude Code combina fontes gerenciadas
</h2>

Quando sua organização entrega mais de uma fonte gerenciada para a mesma máquina, a chave [`managedSourcesBehavior`](/docs/pt/settings-reference#managedsourcesbehavior) decide o que Claude Code faz com as outras:

* **`"first-wins"`, o padrão**: Claude Code usa a fonte de classificação mais alta que entrega pelo menos uma chave de política e ignora o resto em vez de mesclá-las, exceto pelas chaves em [Chaves lidas de cada fonte de administrador](#keys-read-from-every-admin-source). Claude Code não mostra aviso para as fontes que pula; `/status` [nomeia a fonte que usou e as que pulou](#read-the-source-in-/status).
* **`"merge"`**: Claude Code aplica cada fonte de administrador que entrega uma chave de política e as combina por tipo de chave: na maioria das chaves o valor da fonte de classificação mais alta se aplica, listas se unem e locks assumem o valor mais restritivo. [Compor cada fonte gerenciada](#compose-every-managed-source) diz onde definir a chave e como cada tipo de chave se combina. Requer Claude Code v2.1.242 ou posterior.

Ambas as configurações classificam as fontes da mesma forma. Dois termos recorrem nesta seção:

* **Chave de política**: qualquer chave de configurações que não seja as duas chaves de controle, [`wslInheritsWindowsSettings`](/docs/pt/settings-reference#wslinheritswindowssettings) e [`managedSourcesBehavior`](/docs/pt/settings-reference#managedsourcesbehavior). Um arquivo de configurações gerenciadas ou política MDM que contém apenas aquelas não conta, e Claude Code passa para a próxima fonte.
* **Fonte de administrador**: uma das três primeiras fontes abaixo. O registro HKCU gravável pelo usuário não é uma.

Claude Code verifica as fontes nesta ordem, prioridade mais alta primeiro:

1. Configurações remotas, entregues do claude.ai como [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) ou por um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway). Claude Code busca essa fonte apenas quando a sessão se autentica na API da Anthropic diretamente com um [login ou chave elegível](/docs/pt/server-managed-settings#platform-availability), ou se conecta a um gateway com `/login`. Em outros provedores, ou quando `ANTHROPIC_BASE_URL` aponta para algo diferente da API da Anthropic, começa na próxima fonte
2. Políticas MDM ou nível do SO: a plist macOS ou a chave de registro HKLM
3. Arquivos de configurações gerenciadas, `managed-settings.d/*.json` e `managed-settings.json` mesclados juntos
4. O registro HKCU, no Windows, e no WSL uma vez que a chave de registro HKLM ou o arquivo de configurações gerenciadas do Windows ativa [`wslInheritsWindowsSettings`](/docs/pt/settings-reference#wslinheritswindowssettings) e o valor HKCU também o define. Claude Code o lê apenas quando nenhuma fonte acima dele entrega uma chave de política e nenhuma [configuração pai fornecida pelo host](#let-an-embedding-host-add-policy) fornece uma chave restritiva

Este diagrama mostra a classificação, com exemplos das chaves entre fontes que Claude Code lê das três primeiras fontes sob qualquer configuração:

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=53f6be49f06eff48e01422c8ae1bc2e6" className="dark:hidden" alt="Diagrama mostrando as quatro fontes de configurações gerenciadas classificadas de configurações remotas no topo através de MDM, arquivos de configurações gerenciadas e o registro HKCU na parte inferior. Por padrão, a primeira fonte com uma chave de política fornece a política e o resto é pulado; com managedSourcesBehavior definido como merge, cada fonte de administrador com uma chave de política contribui, combinada por tipo de chave, e o registro HKCU fica de fora. Um painel lateral mostra que chaves entre fontes como os locks de sandbox, forceRemoteSettingsRefresh e o env por variável são lidos de cada fonte de administrador, que exclui o registro HKCU." width="680" height="330" data-path="images/managed-source-precedence.svg" />

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence-dark.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=ae407a9a08a3d680e80cf1a2af845d71" className="hidden dark:block" alt="Diagrama mostrando as quatro fontes de configurações gerenciadas classificadas de configurações remotas no topo através de MDM, arquivos de configurações gerenciadas e o registro HKCU na parte inferior. Por padrão, a primeira fonte com uma chave de política fornece a política e o resto é pulado; com managedSourcesBehavior definido como merge, cada fonte de administrador com uma chave de política contribui, combinada por tipo de chave, e o registro HKCU fica de fora. Um painel lateral mostra que chaves entre fontes como os locks de sandbox, forceRemoteSettingsRefresh e o env por variável são lidos de cada fonte de administrador, que exclui o registro HKCU." width="680" height="330" data-path="images/managed-source-precedence-dark.svg" />

<h3 id="keys-read-from-every-admin-source">
  Chaves lidas de cada fonte de administrador
</h3>

Sob a configuração padrão `"first-wins"`, Claude Code lê a maioria das chaves apenas da [fonte que selecionou](#how-claude-code-combines-managed-sources) e ignora um valor em uma fonte de classificação mais baixa mesmo quando a fonte selecionada deixa essa chave indefinida.

Algumas chaves funcionam diferentemente. Claude Code as lê de cada fonte de administrador, portanto uma política MDM de classificação mais baixa ou arquivo de configurações gerenciadas ainda pode defini-las quando a fonte selecionada não o faz. Claude Code deixa o registro HKCU gravável pelo usuário de fora dessa verificação; quando HKCU é a única fonte e nenhum host fornece configurações pai, HKCU se aplica como qualquer fonte selecionada.

As chaves entre fontes incluem:

* `sandbox.network.allowManagedDomainsOnly` e `sandbox.filesystem.allowManagedReadPathsOnly`: um `true` em qualquer fonte de administrador ativa o lock. Enquanto um lock está ativo, Claude Code une a lista de permissões que ele bloqueia, `sandbox.network.allowedDomains` junto com regras de permissão `WebFetch(domain:...)`, ou `sandbox.filesystem.allowRead`, em cada fonte de administrador. Sem o lock, Claude Code trata a lista de permissões como qualquer outra chave, portanto sob `"first-wins"` a lista de permissões de uma fonte de administrador não selecionada é ignorada
* `allowAllClaudeAiMcps`
* `allowManagedMcpServersOnly`: um `true` em qualquer fonte de administrador ativa o lock da lista de permissões MCP. Enquanto o lock está ativo, a lista `allowedMcpServers` gerenciada vem da fonte de administrador de classificação mais alta que define uma. Uma lista gerenciada pelo servidor substitui a lista de uma fonte inferior em vez de se combinar com ela.

  Se nenhuma fonte de administrador define uma lista, cada servidor que passa a lista de negação é carregado, a menos que [configurações pai](#let-an-embedding-host-add-policy) forneçam uma lista.

  Sem o lock, Claude Code lê `allowedMcpServers` da fonte gerenciada que aplica, portanto sob `"first-wins"` a lista de uma fonte de administrador não selecionada é ignorada. Requer Claude Code v2.1.273 ou posterior
* `deniedMcpServers` e [`disableClaudeAiConnectors`](/docs/pt/settings-reference#disableclaudeaiconnectors): uma entrada ou um `true` em qualquer fonte de administrador se aplica. Requer Claude Code v2.1.273 ou posterior
* Os caminhos binários de sandbox `sandbox.bwrapPath` e `sandbox.socatPath`
* O binário `ripgrep` de sandbox, [`sandbox.ripgrep`](/docs/pt/settings-reference#sandbox-ripgrep)
* `sandbox.filesystem.disabled` e `sandbox.network.strictAllowlist`
* [`useAutoModeDuringPlan`](/docs/pt/settings-reference#useautomodeduringplan), [`syncClaudeAiSkills`](/docs/pt/settings-reference#syncclaudeaiskills) e [`syncClaudeAiPlugins`](/docs/pt/settings-reference#syncclaudeaiplugins), onde um `false` de qualquer fonte de administrador desativa o comportamento. Um `false` nas configurações de usuário ou local do desenvolvedor também o desativa; cada chave só pode negar
* [`enableArtifact`](/docs/pt/settings-reference#enableartifact), onde um `false` de qualquer fonte de administrador desativa a [ferramenta Artifact](/docs/pt/artifacts). Um `false` nas configurações de usuário, projeto ou local do desenvolvedor também o desativa, e nenhuma fonte o ativa novamente; consulte [quais valores de nível inferior ainda contam](/docs/pt/settings#exceptions-to-managed-settings-precedence). Requer Claude Code v2.1.242 ou posterior
* [`maxEffortLevel`](/docs/pt/settings-reference#maxeffortlevel), onde o limite mais baixo em qualquer fonte de administrador se aplica. Se um desenvolvedor define um limite mais baixo em suas próprias configurações ou com `--settings`, Claude Code aplica aquele; nenhuma fonte pode aumentar o limite. Requer Claude Code v2.1.267 ou posterior
* Um opt-out de trailer de commit em `attribution`, ou no `includeCoAuthoredBy` descontinuado, de qualquer nível
* [`forceRemoteSettingsRefresh`](/docs/pt/server-managed-settings)
* `env`, mesclado por variável em fontes de administrador: cada variável vem da fonte de prioridade mais alta que a define, portanto fontes inferiores preenchem variáveis que as superiores deixam indefinidas. Algumas variáveis seguem suas próprias regras; [Exceções por chave em fontes gerenciadas](/docs/pt/server-managed-settings#per-key-exceptions-across-managed-sources) nomeia cada uma. Requer Claude Code v2.1.223 ou posterior. Antes de v2.1.223, Claude Code aplicava apenas o bloco `env` inteiro da fonte selecionada

As [chaves de login do gateway](#choose-a-delivery-mechanism) seguem uma regra separada. Claude Code nunca as lê de configurações gerenciadas pelo servidor, portanto enquanto as configurações gerenciadas pelo servidor são a fonte selecionada, a fonte de administrador de classificação mais alta na máquina que carrega uma chave de política ainda as fornece. Um valor em uma fonte de administrador classificada abaixo daquela, ou no registro HKCU, é ignorado.

Quando uma fonte de administrador define `allowManagedMcpServersOnly` ou uma lista `allowedMcpServers` e esse valor não é o que está em vigor, `/status` e `claude doctor` nomeiam essa fonte e chave.

<h3 id="compose-every-managed-source">
  Compor cada fonte gerenciada
</h3>

Para ter Claude Code aplicar cada fonte de administrador que sua organização entrega, defina [`managedSourcesBehavior`](/docs/pt/settings-reference#managedsourcesbehavior) como `"merge"` na fonte de classificação mais alta que você implanta. Claude Code lê a chave apenas da fonte de classificação mais alta que carrega a chave ou uma chave de política, portanto uma fonte inferior não pode se optar para mesclar com a fonte acima dela, e uma máquina que nunca recebe configurações gerenciadas pelo servidor precisa da chave em seu perfil MDM também. O registro HKCU gravável pelo usuário nunca se mescla com outra fonte. Requer Claude Code v2.1.242 ou posterior.

Sob `"merge"`, Claude Code adiciona entradas de lista de uma fonte inferior, como regras `permissions.allow` e hooks, à política, portanto ative-o apenas quando cada fonte classificada abaixo da sua mais alta estiver sob controle de um administrador.

Esta tabela mostra como Claude Code combina cada tipo de chave sob `"merge"`. A entrada [`managedSourcesBehavior`](/docs/pt/settings-reference#managedsourcesbehavior) nomeia cada chave em três das linhas: listas de permissões de restrição, valores tomados inteiros e chaves lidas apenas da fonte de classificação mais alta.

| Tipo de chave                                           | Como Claude Code a combina                                                                                                                                                | Exemplos                                                                                                                       |
| :------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------- |
| Listas                                                  | Combina as entradas de cada fonte                                                                                                                                         | `permissions.allow`, `hooks`, `sandbox.network.allowedDomains`, `deniedMcpServers`                                             |
| Locks                                                   | Aplica o valor mais restritivo que qualquer fonte define; um valor mais solto se aplica apenas da fonte de classificação mais alta                                        | `allowManagedHooksOnly`, `permissions.disableBypassPermissionsMode`, `crossSessionInbound`                                     |
| Listas de permissões de restrição                       | Toma a lista inteira da fonte de classificação mais alta que a define, sem adicionar entradas de fontes inferiores                                                        | `availableModels`, `allowedMcpServers`, `strictKnownMarketplaces`, `allowedChannelPlugins` e a cadeia `fallbackModel`          |
| Valores tomados inteiros                                | Toma o valor inteiro da fonte de classificação mais alta que o define, sem combinar entradas ou campos de fontes inferiores                                               | `sandbox.credentials.awsPairs`, `sandbox.ripgrep`                                                                              |
| Servidores MCP fornecidos                               | Combina os nomes de servidor de cada fonte; quando duas fontes definem o mesmo nome, aplica a entrada inteira da fonte de classificação mais alta                         | `managedMcpServers`                                                                                                            |
| Chaves lidas apenas da fonte de classificação mais alta | Ignora a chave em cada fonte inferior, mesmo quando a fonte de classificação mais alta a deixa indefinida                                                                 | Auxiliares de credencial como `apiKeyHelper`, pins de login como `forceLoginOrgUUID`, `modelPicker`, `permissions.defaultMode` |
| `env`                                                   | Mescla por variável em fontes de administrador sob qualquer configuração, como [Chaves lidas de cada fonte de administrador](#keys-read-from-every-admin-source) descreve |                                                                                                                                |
| Toda outra chave                                        | Toma o valor da fonte de classificação mais alta que o define                                                                                                             | `model`, `cleanupPeriodDays`                                                                                                   |

Para confirmar quais fontes se combinaram em uma máquina, [leia a linha `Setting sources` em `/status`](#read-the-source-in-/status); essa seção diz o que cada rótulo significa.

<h3 id="compute-the-policy-with-a-helper-program">
  Calcular a política com um programa auxiliar
</h3>

Um [`policyHelper`](/docs/pt/settings-reference#policyhelper) é um executável que sua política MDM ou arquivo de configurações gerenciadas nomeia, e Claude Code o executa para calcular configurações gerenciadas na inicialização. Quando a fonte selecionada configura um e o auxiliar emite um objeto `managedSettings`, essa saída muda o que Claude Code lê:

* **O objeto `managedSettings` emitido é a única configuração gerenciada para a sessão**, incluindo para as [chaves que de outra forma lê de cada fonte de administrador](#keys-read-from-every-admin-source), exceto por [`forceRemoteSettingsRefresh`, que tem sua própria regra de inicialização](/docs/pt/settings-reference#forceremotesettingsrefresh)

Para quais falhas de auxiliar, e o que Claude Code faz quando uma falha, consulte [Falhas de auxiliar](/docs/pt/settings-reference#helper-failures).

<span id="parent-settings-from-embedding-hosts" />

<span id="control-policy-from-an-embedding-host" />

<span id="merge-policy-from-an-embedding-host" />

<h3 id="let-an-embedding-host-add-policy">
  Deixar um host de incorporação adicionar política
</h3>

Quando outro aplicativo inicia Claude Code, como Claude Desktop, uma extensão IDE ou um aplicativo Agent SDK, esse host pode passar suas próprias configurações gerenciadas através da opção SDK `managedSettings`. Claude Code chama essas configurações pai.

Por padrão, Claude Code ignora configurações pai sempre que uma fonte de administrador está presente: configurações gerenciadas pelo servidor, uma política MDM ou nível do SO, ou um arquivo de configurações gerenciadas.

Para ter Claude Code mesclar configurações pai junto com uma fonte de administrador, defina [`parentSettingsBehavior`](/docs/pt/settings-reference#parentsettingsbehavior) como `"merge"` na fonte gerenciada de prioridade mais alta; Claude Code lê a chave apenas dessa fonte.

Claude Code então mantém apenas os valores do host que restringem o que Claude pode fazer, com uma lacuna a saber: a menos que você também defina os locks `allowManaged*Only`, as regras de permissão de permissão do host e as listas de permissões de sandbox ainda se aplicam. Consulte [Restringir configurações pai](/docs/pt/claude-apps-gateway#restrict-parent-settings) para os locks.

Um [`policyHelper`](/docs/pt/settings-reference#policyhelper) pode desativar a mesclagem pai independentemente dessa chave; sua entrada diz quando.

Claude Code também aplica essas verificações a valores fornecidos pelo pai por conta própria:

* Quando qualquer fonte de administrador define `allowManagedPermissionRulesOnly`, Claude Code descarta [regras de permissão de permissão fornecidas pelo pai](/docs/pt/claude-apps-gateway#restrict-parent-settings) e `additionalDirectories` conforme as lê, mesmo quando uma fonte de prioridade mais alta deixa a chave indefinida. O efeito da chave em suas próprias regras de permissão vem das configurações gerenciadas que Claude Code aplica, ou das configurações pai que você escolheu mesclar
* Claude Code aplica o valor `forceLoginOrgUUID` ou `allowedMcpServers` nas configurações gerenciadas que aplica e bloqueia um fornecido pelo pai. Fora do lock da lista de permissões MCP, um valor em uma fonte de administrador inferior que Claude Code não aplica nem se aplica nem bloqueia o do pai.

  No Claude Code v2.1.273 ou posterior, enquanto `allowManagedMcpServersOnly` está ativo, a lista `allowedMcpServers` da fonte de administrador de classificação mais alta que define uma se aplica e bloqueia a do pai, como uma [chave entre fontes](#keys-read-from-every-admin-source). A lista do pai se aplica apenas quando nenhuma fonte de administrador define uma. A entrada [`managedSourcesBehavior`](/docs/pt/settings-reference#managedsourcesbehavior) diz qual fonte fornece cada chave sob `"merge"`. Antes de v2.1.223, um valor em qualquer fonte de administrador bloqueava o do pai
* Para `availableModels`, Claude Code aplica o valor nas configurações gerenciadas que aplica e bloqueia uma lista fornecida pelo pai
* Para `strictKnownMarketplaces`, Claude Code igualmente aplica a lista nas configurações gerenciadas que aplica e bloqueia uma fornecida pelo pai. A lista do pai se aplica apenas quando nenhuma fonte gerenciada aplicada define uma. Requer Claude Code v2.1.282 ou posterior
* Um `blockedMarketplaces` fornecido pelo pai se aplica além de qualquer lista de bloqueio que uma fonte gerenciada define. Requer Claude Code v2.1.282 ou posterior

<h4 id="keep-cowork-folder-access-when-only-managed-rules-apply">
  Manter o acesso à pasta Cowork quando apenas regras gerenciadas se aplicam
</h4>

[Cowork](https://claude.com/docs/cowork/overview) no aplicativo Claude Desktop executa suas sessões em Claude Code e concede a cada sessão acesso a suas pastas de trabalho, como a pasta que o usuário conecta, através de regras de permissão que fornece quando inicia a sessão. Quando sua política gerenciada define [`allowManagedPermissionRulesOnly`](/docs/pt/settings-reference#allowmanagedpermissionrulesonly), Claude Code mantém apenas as regras de permissão na política gerenciada: descarta regras de permissão que um host fornece como configurações pai, como `--allowedTools` ou em um arquivo de configurações, portanto as gravações nessas pastas perdem sua pré-aprovação. Em uma sessão Cowork que pede antes de edições, Cowork não pode mostrar o prompt, e Claude relata cada gravação como bloqueada porque o caminho se resolve para um local protegido ou um caminho fora da pasta conectada.

Para restaurar as gravações, adicione regras de permissão para essas pastas à fonte gerenciada que Claude Code [seleciona](#precedence-within-the-managed-tier) nessas máquinas: em uma frota gerenciada por MDM, essa é a política MDM em vez de um arquivo de configurações gerenciadas separado. Este exemplo usa a forma de arquivo, e uma política MDM toma as mesmas chaves. Mantém `allowManagedPermissionRulesOnly` definido e permite edições sob uma pasta `CoworkProjects` no diretório inicial de cada usuário; substitua o caminho pelas pastas que seus usuários conectam:

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true,
  "permissions": {
    "allow": [
      "Edit(~/CoworkProjects/**)"
    ]
  }
}
```

Depois de implantar a política, Claude pode salvar arquivos sob essa pasta em uma nova sessão Cowork. [Regras Read e Edit](/docs/pt/permissions#read-and-edit) cobrem a sintaxe de caminho, incluindo a forma `//` para caminhos absolutos.

<h3 id="what-a-developer-can-change">
  O que um desenvolvedor pode alterar
</h3>

Os próprios arquivos de configurações de um desenvolvedor, valores `--settings` e arquivos de projeto nunca substituem um valor gerenciado; as [exceções](/docs/pt/settings#exceptions-to-managed-settings-precedence) apenas deixam um valor inferior mais restritivo contar. Estes casos ficam fora dessa regra:

* **O modelo para uma sessão**: um `model` gerenciado é um padrão, não um lock. `--model` e `ANTHROPIC_MODEL` ainda escolhem o modelo para essa sessão, portanto implante [`availableModels`](/docs/pt/settings-reference#availablemodels) para restringir a escolha.
* **Direitos de administrador local**: um desenvolvedor que é um administrador na máquina pode editar a própria fonte gerenciada, é por isso que a ferramenta MDM pode reimplantar o perfil ou arquivo em um cronograma e por que a chave de registro HKLM e o domínio de preferências gerenciadas macOS existem.
* **O cache gerenciado pelo servidor**: as configurações gerenciadas pelo servidor vêm dos servidores da Anthropic, e uma edição no cache local [dura apenas até a próxima busca bem-sucedida](/docs/pt/server-managed-settings#security-considerations).
* **Outras ferramentas**: as configurações gerenciadas vinculam apenas Claude Code. Um desenvolvedor que chama a API de outra ferramenta não está sob elas.

<span id="verify-enforcement" />

<span id="verify-that-a-policy-is-in-force" />

<h2 id="check-that-a-policy-is-in-force">
  Verificar que uma política está em vigor
</h2>

Um desenvolvedor relata que uma política não está sendo aplicada, ou você quer confirmar que um lançamento chegou antes de empurrá-lo para a frota. Dois comandos nessa máquina respondem: `/status` mostra qual fonte gerenciada Claude Code selecionou, e `claude doctor` lista o que descartou.

<h3 id="read-the-source-in-/status">
  Ler a fonte em /status
</h3>

Na máquina do desenvolvedor, execute `/status` dentro de Claude Code e leia a linha `Setting sources`. Quando uma fonte gerenciada está em vigor, a linha lista `Enterprise managed settings` com a fonte que Claude Code selecionou entre parênteses:

* `(remote)`: configurações gerenciadas pelo servidor do claude.ai ou um gateway
* `(plist)` ou `(HKLM)`: uma política MDM ou nível do SO
* `(file)`, `(drop-ins)` ou `(file + drop-ins)`: `managed-settings.json`, o diretório drop-in ou ambos
* `(remote + file, merged)` ou outra lista terminando em `, merged`: sua organização [compõe cada fonte gerenciada](#compose-every-managed-source) e Claude Code mesclou as fontes listadas na política. Uma fonte inferior ainda pode fornecer variáveis `env` sem aparecer na lista. Requer Claude Code v2.1.242 ou posterior
* `(HKCU)`: o fallback de registro gravável pelo usuário
* `(parent process)`: um [host de incorporação](#let-an-embedding-host-add-policy) forneceu configurações restritivas
* `(helper)`: um [`policyHelper`](/docs/pt/settings-reference#policyhelper) configurado pela fonte MDM ou arquivo selecionada

Quando Claude Code encontrou uma fonte gerenciada na máquina e não a selecionou, uma segunda linha, `Skipped sources`, nomeia cada tal fonte. Leia-a para distinguir uma política que nunca alcançou a máquina de uma que alcançou e que uma fonte de prioridade mais alta substituiu. Requer Claude Code v2.1.242 ou posterior.

Quando a política não está sendo aplicada, a linha `Setting sources` diz qual de dois problemas você tem:

* **A linha está faltando**: Claude Code não encontrou nenhuma fonte gerenciada que entregue uma chave de política.

  Se você implantou um arquivo de configurações gerenciadas, verifique se ele fica no caminho para o SO e se contém uma [chave de política](#how-claude-code-combines-managed-sources) em vez de apenas as chaves de controle. Um arquivo que não é JSON válido não produz este estado; Claude Code [recusa iniciar](#find-entries-claude-code-dropped) em vez disso.

  Quando você implantou através de configurações gerenciadas pelo servidor em vez disso, execute `claude doctor`, que relata o [resultado da busca](/docs/pt/server-managed-settings#verify-settings-delivery).
* **A linha nomeia uma fonte diferente da que você implantou**: uma fonte de prioridade mais alta está presente e Claude Code ignorou a sua, e `Skipped sources` a lista. [Como Claude Code combina fontes gerenciadas](#how-claude-code-combines-managed-sources) fornece a ordem.

<span id="invalid-entries-in-managed-settings" />

<h3 id="find-entries-claude-code-dropped">
  Encontrar entradas que Claude Code descartou
</h3>

Quando um arquivo de configurações gerenciadas, perfil MDM, valor de registro ou payload gerenciado pelo servidor falha na validação de esquema, Claude Code primeiro pula as entradas individuais que pode reparar, como uma regra de permissão inválida, com um aviso para cada uma, depois descarta qualquer chave de nível superior cujo valor ainda falha e continua aplicando cada chave válida restante.

Claude Code é mais rigoroso com o `managedSettings` que um [`policyHelper`](/docs/pt/settings-reference#policyhelper) emite: faz os mesmos reparos de entrada, mas qualquer violação de esquema que sobreviva falha a execução inteira do auxiliar, e na inicialização Claude Code recusa iniciar, o mesmo que para um auxiliar que sai com código diferente de zero.

Quando um arquivo de configurações gerenciadas, arquivo drop-in, plist MDM ou valor de registro HKLM está presente mas não pode ser analisado como um objeto JSON, Claude Code recusa iniciar e imprime [um erro nomeando a fonte](/docs/pt/errors#managed-settings-document-could-not-be-parsed), mesmo quando outra fonte de administrador entrega uma política válida. Cada fonte falha desta forma quando:

* **Arquivo de configurações gerenciadas ou arquivo drop-in**: o arquivo não é JSON válido, ou seu nível superior não é um objeto
* **Plist MDM**: o `plutil` do macOS relata o plist malformado, ou seu conteúdo convertido não é um objeto JSON
* **Valor de registro HKLM**: o valor `Settings` não é uma string, está vazio ou não contém um objeto JSON

Três estados de fonte não causam essa recusa:

* Um arquivo, perfil ou valor de registro ausente não é uma falha; Claude Code é executado sem essa fonte.
* Um arquivo de configurações gerenciadas vazio conta como `{}`.
* Um valor malformado na chave de registro HKCU gravável pelo usuário nunca bloqueia o lançamento. Claude Code o relata como um aviso em `/status` e `claude doctor` em vez disso.

Se um arquivo de configurações gerenciadas, arquivo drop-in ou diretório `managed-settings.d/` não puder ser lido e nenhuma fonte de administrador fornecer uma política, sessões conectadas com credenciais claude.ai ou Claude Console saem na inicialização com uma mensagem para contatar um administrador.

Para encontrar uma entrada descartada, procure em um de três lugares:

* Sessões interativas mostram um diálogo na inicialização listando as entradas inválidas.
* Execuções não interativas com `-p` imprimem um resumo para stderr.
* [`claude doctor`](/docs/pt/debug-your-config) lista cada entrada inválida com sua fonte e campo.

<h4 id="keys-that-fail-closed">
  Chaves que falham fechadas
</h4>

Algumas chaves de aplicação não são descartadas quando inválidas. Claude Code aplica um fallback mais restritivo até que o valor seja corrigido; a tabela mostra o que aplica para cada chave:

| Campo                         | Comportamento quando presente mas inválido                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :---------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedMcpServers`           | Aplicado como uma lista de permissões vazia até que o valor seja corrigido, portanto nenhum servidor MCP que os usuários adicionem é admitido. Servidores que sua organização entrega através de [`managedMcpServers`](/docs/pt/settings-reference#managedmcpservers) ainda carregam, e servidores `managed-mcp.json` carregam por [Como um servidor é avaliado](/docs/pt/managed-mcp#how-a-server-is-evaluated). Uma entrada individual inválida é removida e o subconjunto válido é aplicado.    |
| `allowedHttpHookUrls`         | Claude Code aplica uma [lista de permissões](/docs/pt/settings-reference#allowedhttphookurls) gerenciada vazia até que você corrija o valor, portanto um hook HTTP é executado apenas se outro arquivo de configurações listar sua URL. Se apenas uma entrada individual for inválida, Claude Code remove essa entrada e aplica o resto.                                                                                                                                                      |
| `httpHookAllowedEnvVars`      | Claude Code aplica uma [lista de permissões](/docs/pt/settings-reference#httphookallowedenvvars) gerenciada vazia até que você corrija o valor, portanto uma variável de cabeçalho é interpolada apenas se outro arquivo de configurações a nomear. Se apenas uma entrada individual for inválida, Claude Code remove essa entrada e aplica o resto.                                                                                                                                          |
| `allowedChannelPlugins`       | Claude Code aplica uma lista de permissões vazia até que você corrija o valor, portanto nenhum plugin de canal passado para `--channels` é admitido. Se apenas uma entrada individual for inválida, ele remove essa entrada e aplica o resto.                                                                                                                                                                                                                                            |
| `strictKnownMarketplaces`     | Aplicado como uma lista de permissões vazia até que o valor seja corrigido, portanto nenhuma [fonte de marketplace](/docs/pt/plugins/org#restrict-what-users-can-install) é admitida. Uma entrada individual que é inválida ou não pode ser aplicada, como um regex `hostPattern` que não compila, é removida e o subconjunto válido é aplicado.                                                                                                                                              |
| `allowManagedHooksOnly`       | Tratado como `true` até ser corrigido: as [restrições de hook](/docs/pt/settings-reference#allowmanagedhooksonly) se aplicam e, a menos que `disableCommandPluginSources` seja explicitamente `false`, plugins de origem de comando são desabilitados.                                                                                                                                                                                                                                        |
| `allowManagedMcpServersOnly`  | Tratado como `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `disableCommandPluginSources` | Tratado como `true`, portanto plugins de origem de comando permanecem desabilitados até que o valor seja corrigido.                                                                                                                                                                                                                                                                                                                                                                      |
| `disableSideloadFlags`        | Tratado como `true` até que o valor seja corrigido, com os efeitos listados para [`disableSideloadFlags`](/docs/pt/settings-reference#disablesideloadflags).                                                                                                                                                                                                                                                                                                                                  |
| `availableModels`             | Aplicado como uma lista de permissões vazia até ser corrigido, portanto apenas o modelo Padrão está disponível; uma entrada não-string é removida e o subconjunto válido é aplicado.                                                                                                                                                                                                                                                                                                     |
| `enforceAvailableModels`      | Tratado como `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `syncClaudeAiPlugins`         | Tratado como `false`, portanto a sincronização de [plugins claude.ai](/docs/pt/settings-reference#syncclaudeaiplugins) está desativada até que o valor seja corrigido.                                                                                                                                                                                                                                                                                                                        |
| `forceLoginOrgUUID`           | Nenhuma organização é permitida fazer login até que o valor seja corrigido.                                                                                                                                                                                                                                                                                                                                                                                                              |
| `gatewayInternalNetworks`     | Quando o valor inválido vem da fonte gerenciada mais alta na máquina, `/login` recusa cada novo [gateway de nuvem](/docs/pt/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) login na máquina até que o valor seja corrigido.                                                                                                                                                                                                                                             |
| `crossSessionInbound`         | Tratado como `refuse`, o valor mais restritivo, portanto [mensagens entre sessões](/docs/pt/cross-session-messaging#control-inbound-messages) de entrada são recusadas até que o valor seja corrigido. O desenvolvedor vê [um aviso](/docs/pt/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse).                                                                                                                                                                                       |
| `deniedMcpServers`            | Uma entrada individual inválida é removida e o subconjunto válido é aplicado. Um valor totalmente inválido é descartado com um aviso, já que negar cada servidor bloquearia servidores que a política nunca nomeou.                                                                                                                                                                                                                                                                      |
| `blockedMarketplaces`         | Uma entrada individual inválida é removida e o subconjunto válido é aplicado. Uma entrada que analisa mas nunca pode corresponder, como um regex `hostPattern` que não compila, é mantida com um aviso. Ela bloqueia nada até ser corrigida, mas [restrições de marketplace](/docs/pt/plugins/org#restrict-what-users-can-install) permanecem ativas. Um valor totalmente inválido é descartado com um aviso, já que bloquear cada marketplace bloquearia fontes que a política nunca nomeou. |
| `sandbox.credentials`         | Uma entrada inválida recuperável é degradada para `mode: "deny"` com um aviso; uma irrecuperável é removida; entradas válidas permanecem aplicadas. Consulte [entradas de credencial inválidas](/docs/pt/settings-reference#invalid-credential-entries-in-managed-settings)                                                                                                                                                                                                                   |

`allowedHttpHookUrls` e `httpHookAllowedEnvVars` mesclam entre arquivos de configurações, portanto entradas em suas configurações de usuário, projeto ou local ainda se aplicam enquanto a lista gerenciada está vazia.

Os fallbacks para essas duas chaves e para `allowedChannelPlugins` requerem Claude Code v2.1.267 ou posterior; versões anteriores descartam a chave inteira quando seu valor ou qualquer entrada é inválida. Os fallbacks para `strictKnownMarketplaces`, `blockedMarketplaces` e `disableSideloadFlags` requerem Claude Code v2.1.277 ou posterior; versões anteriores descartam a chave inteira quando seu valor ou qualquer entrada é inválida.

`requiredMinimumVersion` e `requiredMaximumVersion` falham abertos por design: um valor inválido é descartado em vez de ser aplicado.

Esta tolerância se aplica apenas a configurações gerenciadas. Arquivos de configurações de usuário, projeto e local permanecem rigorosos: um arquivo cuja JSON ou forma de nível superior falha na validação é rejeitado como um todo e relatado, e uma entrada individual que falha, como uma regra de permissão malformada, é pulada com um aviso enquanto o resto do arquivo se aplica.

<span id="managed-only-settings" />

<h2 id="keys-only-a-managed-source-can-set">
  Chaves que apenas uma fonte gerenciada pode definir
</h2>

Claude Code lê as seguintes chaves apenas de uma fonte gerenciada; colocá-las em arquivos de configurações de usuário ou projeto não tem efeito.

A maioria delas são bloqueios: o valor que um bloqueio governa, como regras de permissão ou `sandbox.network.allowedDomains`, é uma chave ordinária que qualquer nível pode definir, e o bloqueio diz ao Claude Code para honrar apenas o valor gerenciado.

A tabela cobre os controles de permissão, plugin e entrega. Para qualquer chave não listada aqui, a coluna Escopo da [referência de configurações](/docs/pt/settings-reference#all-settings) diz se é apenas gerenciada; as chaves apenas gerenciadas restantes lá incluem a URL de login do gateway, versão, navegador, simulador móvel, host SSH, sessão local do Desktop, caminho binário da sandbox, preço do modelo e controles CLAUDE.md.

| Configuração                                                                                                          | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :-------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`allowAllClaudeAiMcps`](/docs/pt/settings-reference#allowallclaudeaimcps)                                                 | Carregue os conectores claude.ai que Claude Code busca por si mesmo junto com um `managed-mcp.json` implantado em vez de suprimi-los                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [`allowedChannelPlugins`](/docs/pt/settings-reference#allowedchannelplugins)                                               | Lista de permissões de plugins de canal que podem enviar mensagens. Substitui a lista de permissões padrão da Anthropic quando definida. Requer `channelsEnabled: true`. Veja [Restringir quais plugins de canal podem ser executados](/docs/pt/channels#restrict-which-channel-plugins-can-run)                                                                                                                                                                                                                                                               |
| [`allowManagedHooksOnly`](/docs/pt/settings-reference#allowmanagedhooksonly)                                               | Quando `true`, restringe quais hooks são executados; veja [o que é executado sob `allowManagedHooksOnly`](/docs/pt/settings-reference#what-runs-under-allowmanagedhooksonly) para a lista completa de efeitos                                                                                                                                                                                                                                                                                                                                                  |
| [`allowManagedMcpServersOnly`](/docs/pt/settings-reference#allowmanagedmcpserversonly)                                     | Quando `true`, apenas `allowedMcpServers` das configurações gerenciadas são respeitados. `deniedMcpServers` ainda é mesclado de todas as fontes. Veja [Chaves lidas de todas as fontes de administrador](#keys-read-from-every-admin-source) para quais fontes gerenciadas podem defini-la, e [Configuração MCP gerenciada](/docs/pt/managed-mcp)                                                                                                                                                                                                              |
| [`allowManagedPermissionRulesOnly`](/docs/pt/settings-reference#allowmanagedpermissionrulesonly)                           | Torna as configurações gerenciadas a única fonte de configurações de regras de permissão. A entrada lista todas as fontes que ignora                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [`blockedMarketplaces`](/docs/pt/settings-reference#blockedmarketplaces)                                                   | Lista de bloqueio de fontes de marketplace. As fontes bloqueadas são verificadas antes do download, portanto nunca tocam o sistema de arquivos. Veja [restrições de marketplace gerenciadas](/docs/pt/plugins/org#restrict-what-users-can-install)                                                                                                                                                                                                                                                                                                             |
| [`channelsEnabled`](/docs/pt/settings-reference#channelsenabled)                                                           | Permitir [canais](/docs/pt/channels) para a organização. Veja [controles empresariais](/docs/pt/channels#enterprise-controls) para o padrão em cada plano                                                                                                                                                                                                                                                                                                                                                                                                           |
| [`disableCommandPluginSources`](/docs/pt/settings-reference#disablecommandpluginsources)                                   | Quando `true`, bloqueia [fontes de plugin `command`](/docs/pt/plugins/marketplace-reference#command-plugin-source) inteiramente, portanto o comando declarado no marketplace nunca é executado. Também bloqueia comandos [`headersHelper`](/docs/pt/plugins/host-marketplace#authenticate-archive-downloads) do marketplace, exceto para um marketplace que as próprias configurações gerenciadas declaram. Quando não definido, segue `allowManagedHooksOnly`. Requer Claude Code v2.1.229 ou posterior, e o bloqueio `headersHelper` requer v2.1.238 ou posterior |
| [`disableSideloadFlags`](/docs/pt/settings-reference#disablesideloadflags)                                                 | Rejeite os sinalizadores `--plugin-dir`, `--plugin-url`, `--agents` e `--mcp-config` na inicialização. Em sessões na nuvem, Claude Code descarta os servidores MCP que o servidor entregou através de `--mcp-config`, exceto entradas `type: "sdk"` em processo, e inicia a sessão. Requer Claude Code v2.1.193 ou posterior                                                                                                                                                                                                                              |
| [`forceRemoteSettingsRefresh`](/docs/pt/settings-reference#forceremotesettingsrefresh)                                     | Quando `true`, bloqueia a inicialização da CLI até que as configurações gerenciadas remotas sejam buscadas recentemente e sai se a busca falhar. Veja [aplicação de falha fechada](/docs/pt/server-managed-settings#enforce-fail-closed-startup)                                                                                                                                                                                                                                                                                                               |
| [`managedMcpServers`](/docs/pt/settings-reference#managedmcpservers)                                                       | Servidores MCP remotos fornecidos a cada usuário junto com os seus próprios. Fornece servidores em vez de bloquear qualquer coisa. Veja [Fornecer servidores através de configurações gerenciadas](/docs/pt/managed-mcp#provide-servers-through-managed-settings). Requer Claude Code v2.1.259 ou posterior                                                                                                                                                                                                                                                    |
| [`managedSourcesBehavior`](/docs/pt/settings-reference#managedsourcesbehavior)                                             | Se Claude Code aplica apenas a fonte gerenciada de prioridade mais alta ou [compõe cada uma delas](#compose-every-managed-source)                                                                                                                                                                                                                                                                                                                                                                                                                         |
| [`parentSettingsBehavior`](/docs/pt/settings-reference#parentsettingsbehavior)                                             | Se as configurações pai fornecidas pelo host são mescladas sob a política gerenciada                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [`pluginSuggestionMarketplaces`](/docs/pt/settings-reference#pluginsuggestionmarketplaces)                                 | Marketplaces cujos plugins Claude Code pode sugerir aos usuários                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| [`pluginTrustMessage`](/docs/pt/settings-reference#plugintrustmessage)                                                     | Mensagem personalizada anexada ao aviso de confiança de plugin mostrado antes da instalação                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| [`policyHelper`](/docs/pt/settings-reference#policyhelper)                                                                 | Executável que calcula configurações gerenciadas na inicialização; veja [Calcular configurações gerenciadas com um auxiliar de política](/docs/pt/settings-reference#policyhelper)                                                                                                                                                                                                                                                                                                                                                                             |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](/docs/pt/settings-reference#sandbox-filesystem-allowmanagedreadpathsonly) | Quando `true`, apenas caminhos `filesystem.allowRead` das configurações gerenciadas são respeitados. `denyRead` ainda é mesclado de todas as fontes                                                                                                                                                                                                                                                                                                                                                                                                       |
| [`sandbox.network.allowManagedDomainsOnly`](/docs/pt/settings-reference#sandbox-network-allowmanageddomainsonly)           | Honre apenas regras de permissão `allowedDomains` e `WebFetch(domain:...)` gerenciadas; bloqueie outros domínios sem solicitar                                                                                                                                                                                                                                                                                                                                                                                                                            |
| [`strictKnownMarketplaces`](/docs/pt/settings-reference#strictknownmarketplaces)                                           | Controla de quais fontes de marketplace de plugins os usuários podem adicionar e instalar plugins. Veja [restrições de marketplace gerenciadas](/docs/pt/plugins/org#restrict-what-users-can-install)                                                                                                                                                                                                                                                                                                                                                          |
| [`strictPluginOnlyCustomization`](/docs/pt/settings-reference#strictpluginonlycustomization)                               | Bloqueie skills, agentes, hooks e servidores MCP de fontes de usuário e projeto; `true` bloqueia todos os quatro, uma matriz nomeia qual                                                                                                                                                                                                                                                                                                                                                                                                                  |
| [`wslInheritsWindowsSettings`](/docs/pt/settings-reference#wslinheritswindowssettings)                                     | Quando definido no registro HKLM ou em um arquivo sob `C:\Program Files\ClaudeCode`, faça o WSL ler a cadeia de política do Windows e ler `/etc/claude-code` apenas quando nenhum arquivo de configurações gerenciadas ou drop-in sob esse diretório entregar uma [chave de política](#how-claude-code-combines-managed-sources); a entrada fornece a ordem                                                                                                                                                                                               |

<Note>
  Nos planos Team e Enterprise, um Proprietário ativa ou desativa [Controle Remoto](/docs/pt/remote-control) e [sessões web](/docs/pt/claude-code-on-the-web) em toda a organização nas [configurações de administrador do Claude Code](https://claude.ai/admin-settings/claude-code). O Controle Remoto pode ser desativado adicionalmente por dispositivo com a configuração [`disableRemoteControl`](/docs/pt/settings-reference#disableremotecontrol). As sessões web não têm chave de configurações gerenciadas por dispositivo.

  Para verificar se essas configurações de organização chegaram a uma determinada máquina, execute `claude doctor` lá e leia a linha `Organization policy`, que diz onde Claude Code carregou a política ou por que não carregou. Requer Claude Code v2.1.261 ou posterior. Em uma sessão em execução, `/status` mostra a mesma linha quando a política não foi carregada.
</Note>

<h2 id="turn-telemetry-off-for-your-organization">
  Desativar telemetria para sua organização
</h2>

Claude Code envia [telemetria](/docs/pt/data-usage#telemetry-services) operacional da Anthropic por padrão em sessões que usam a API da Anthropic, seja diretamente, através de um gateway LLM ou através de um `ANTHROPIC_BASE_URL` personalizado; [Comportamentos padrão por provedor de API](/docs/pt/data-usage#default-behaviors-by-api-provider) diz quais provedores a enviam. Para desativá-la para cada desenvolvedor sem depender da shell de cada pessoa, entregue `DISABLE_TELEMETRY` através do bloco `env` de suas configurações gerenciadas. Este exemplo define `DISABLE_TELEMETRY` para todos que a política alcança:

```json theme={null}
{
  "env": {
    "DISABLE_TELEMETRY": "1"
  }
}
```

Claude Code aplica um valor de `1` sem mostrar ao usuário o [diálogo de aprovação](/docs/pt/server-managed-settings#environment-variables-and-the-approval-dialog).

Se você desativar a telemetria, Claude Code para de enviar os dados de uso que alimentam o [painel de análise](/docs/pt/analytics) de sua organização para os desenvolvedores que a política alcança. A variável também desativa a busca de sinalizadores de recurso, o que torna Remote Control, modo automático padrão e os outros [recursos que precisam de busca de sinalizadores de recurso](/docs/pt/env-vars#features-that-need-feature-flag-fetching) indisponíveis para esses desenvolvedores.

[Onde e quando uma política se aplica](#where-and-when-a-policy-applies) diz qual mecanismo de entrega alcança cada superfície, e [Disponibilidade de plataforma](/docs/pt/server-managed-settings#platform-availability) diz quais sessões pulam a busca de configurações gerenciadas pelo servidor.

Se sua organização usa chaves de criptografia gerenciadas pelo cliente e roteia Claude Code através de um gateway, [Configurar proxies e gateways](/docs/pt/third-party-integrations#configure-proxies-and-gateways) diz por que essas sessões precisam dessa variável.

<h2 id="see-also">
  Veja também
</h2>

* [Configurar Claude Code para sua organização](/docs/pt/admin-setup): decidir o que impor e como
* [Configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings): entregar política do console claude.ai ou um gateway
* [Configuração MCP gerenciada](/docs/pt/managed-mcp): controlar quais servidores MCP os desenvolvedores podem usar
* [Todas as configurações](/docs/pt/settings-reference): cada chave, com se uma fonte gerenciada pode defini-la
* [Arquivos de configurações de exemplo](/docs/pt/settings-example#an-organizations-managed-settings): um `managed-settings.json` completo mostrando a forma das chaves gerenciadas
