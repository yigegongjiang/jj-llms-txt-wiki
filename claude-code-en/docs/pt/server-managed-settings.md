> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar configurações gerenciadas pelo servidor

> Configure centralmente o Claude Code para sua organização através de configurações entregues pelo servidor, sem exigir infraestrutura de gerenciamento de dispositivos.

As configurações gerenciadas pelo servidor permitem que Proprietários da organização configurem centralmente o Claude Code a partir de [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code) no console claude.ai. Os clientes do Claude Code buscam essas configurações automaticamente quando os usuários se autenticam com uma credencial elegível em uma plataforma onde a entrega gerenciada pelo servidor é suportada. Consulte [Disponibilidade de plataforma](#platform-availability) para as credenciais e plataformas que se qualificam.

<Note>
  As configurações gerenciadas pelo servidor estão disponíveis para clientes do [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_teams#team-&-enterprise) e [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_enterprise).
</Note>

<h2 id="requirements">
  Requisitos
</h2>

Para usar configurações gerenciadas pelo servidor, você precisa de:

* Plano Claude for Teams ou Claude for Enterprise
* Função de Proprietário ou Proprietário Primário em sua organização Claude, para visualizar e editar a configuração
* Acesso de rede a `api.anthropic.com`

<h2 id="choose-between-server-managed-and-endpoint-managed-settings">
  Escolha entre configurações gerenciadas pelo servidor e gerenciadas pelo endpoint
</h2>

O Claude Code suporta duas abordagens para configuração centralizada. As configurações gerenciadas pelo servidor entregam a configuração dos servidores da Anthropic. As [configurações gerenciadas pelo endpoint](/docs/pt/managed-settings#delivery-mechanisms) são implantadas diretamente em dispositivos através de políticas nativas do SO (preferências gerenciadas do macOS, registro do Windows) ou arquivos de configurações gerenciadas.

| Abordagem                                                                               | Melhor para                                                       | Modelo de segurança                                                                                                                      |
| :-------------------------------------------------------------------------------------- | :---------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| **Configurações gerenciadas pelo servidor**                                             | Organizações sem MDM, ou usuários em dispositivos não gerenciados | Configurações que o Claude Code busca dos servidores da Anthropic na inicialização e atualiza a cada hora durante a sessão               |
| **[Configurações gerenciadas pelo endpoint](/docs/pt/managed-settings#delivery-mechanisms)** | Organizações com MDM ou gerenciamento de endpoint                 | Configurações implantadas em dispositivos via perfis de configuração MDM, políticas de registro ou arquivos de configurações gerenciadas |

Se seus dispositivos estão inscritos em uma solução MDM ou gerenciamento de endpoint, as configurações gerenciadas pelo endpoint fornecem garantias de segurança mais fortes porque o arquivo de configurações pode ser protegido contra modificação do usuário no nível do SO. As configurações gerenciadas pelo endpoint não chegam às [sessões na nuvem](/docs/pt/model-config#surface-coverage) em ambientes hospedados pela Anthropic, portanto as organizações que usam Claude Code na web devem configurar também as configurações gerenciadas pelo servidor. As sessões em um [ambiente auto-hospedado](/docs/pt/self-hosted-environments) também leem o arquivo de configurações gerenciadas na imagem do executor. A [precedência de configurações](#settings-precedence) abaixo diz quando esse arquivo se aplica.

<h2 id="configure-server-managed-settings">
  Configurar configurações gerenciadas pelo servidor
</h2>

<Steps>
  <Step title="Abrir o console de administração">
    No console claude.ai, vá para [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code).

    Se o link o redirecionar para uma página diferente de Admin Settings em vez da página Claude Code, sua conta não tem a função necessária. Funções de Admin e outras funções que não sejam Owner não podem visualizar ou editar configurações gerenciadas, portanto, peça a um Owner ou Primary Owner em sua organização para fazer a alteração. Veja [Controle de acesso](#access-control).
  </Step>

  <Step title="Definir suas configurações">
    Adicione sua configuração como JSON. Todas as [configurações disponíveis em `settings.json`](/docs/pt/settings-reference#all-settings) são suportadas, exceto aquelas restritas à entrega de política em nível do SO; veja [Limitações atuais](#current-limitations) para essa lista curta. Isso inclui [hooks](/docs/pt/hooks), [variáveis de ambiente](/docs/pt/env-vars) e [configurações apenas gerenciadas](/docs/pt/managed-settings#managed-only-settings) como `allowManagedPermissionRulesOnly`.

    Este exemplo impõe uma lista de negação de permissões, impede que os usuários ignorem as permissões e restringe as regras de permissão àquelas definidas nas configurações gerenciadas. A regra `Bash(curl *)` corresponde a `curl` [conforme Claude a escreve](/docs/pt/permissions#bash-rule-limits), não `/usr/bin/curl` ou `sh -c 'curl …'`; para imposição de rede que não dependa do texto do comando, adicione um [bloco `sandbox` com `allowManagedDomainsOnly`](/docs/pt/sandboxing#configure-the-sandbox-for-your-organization).

    ```json theme={null}
    {
      "permissions": {
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    Hooks usam o mesmo formato que em `settings.json`.

    Este exemplo executa um script de auditoria após cada edição de arquivo em toda a organização:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              { "type": "command", "command": "/usr/local/bin/audit-edit.sh" }
            ]
          }
        ]
      }
    }
    ```

    Como hooks executam comandos shell, os usuários em sessões interativas veem uma [caixa de diálogo de aprovação de segurança](#security-approval-dialogs) antes de Claude Code aplicá-los.

    Para configurar o classificador do [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) para que ele saiba quais repositórios, buckets e domínios sua organização confia, entregue um bloco `autoMode` da mesma forma; veja [Configurar modo automático](/docs/pt/auto-mode-config) para saber como as entradas `autoMode` afetam o que o classificador bloqueia e avisos importantes sobre os campos `environment`, `allow`, `soft_deny` e `hard_deny`.
  </Step>

  <Step title="Salvar e implantar">
    Salve suas alterações. Os clientes do Claude Code recebem as configurações atualizadas na próxima inicialização ou ciclo de polling por hora.
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  Verificar entrega de configurações
</h3>

Para confirmar que as configurações estão sendo aplicadas, peça a um usuário para reiniciar o Claude Code. Se a configuração incluir configurações que acionem a [caixa de diálogo de aprovação de segurança](#security-approval-dialogs), o usuário vê um prompt descrevendo as configurações gerenciadas na próxima vez que Claude Code as buscar: na próxima inicialização ou dentro de uma hora em uma sessão interativa em execução. Você também pode verificar que as regras de permissão gerenciadas estão ativas pedindo a um usuário para executar `/permissions` para visualizar suas regras de permissão efetivas.

Para verificar o resultado da busca em uma máquina específica, peça ao usuário para executar `claude doctor` e ler a linha `Managed settings (remote)`. Requer Claude Code v2.1.248 ou posterior. A linha relata um de quatro resultados:

* As configurações entregues foram carregadas
* Sua organização não tem configurações gerenciadas pelo servidor configuradas
* A busca falhou, com a causa e se uma política em cache ainda se aplica
* Claude Code pulou a busca, com o motivo. Veja [Disponibilidade da plataforma](#platform-availability) para os provedores e configurações que a pulam

Enquanto a busca ainda está em andamento, a linha relata isso em vez disso.

Em uma sessão em execução, `/status` mostra a mesma linha após uma busca com falha e, para algumas causas de busca pulada, como uma variável de provedor de terceiros ou um `ANTHROPIC_BASE_URL` personalizado exportado no shell do usuário.

<h3 id="access-control">
  Controle de acesso
</h3>

Os seguintes papéis podem gerenciar configurações gerenciadas pelo servidor:

* **Primary Owner**
* **Owner**

Restrinja o acesso a pessoal confiável, pois as alterações de configurações se aplicam a todos os usuários da organização.

<h3 id="managed-only-settings">
  Configurações apenas gerenciadas
</h3>

A maioria das [chaves de configurações](/docs/pt/settings-reference#all-settings) funciona em qualquer escopo. Um punhado de chaves são lidas apenas de configurações gerenciadas e não têm efeito quando colocadas em arquivos de configurações de usuário ou projeto. Veja [configurações apenas gerenciadas](/docs/pt/managed-settings#managed-only-settings) para os controles de permissão e plugin, ou leia a coluna Scope do índice [Todas as configurações](/docs/pt/settings-reference#all-settings) para o conjunto completo.

<h3 id="current-limitations">
  Limitações atuais
</h3>

As configurações gerenciadas pelo servidor têm as seguintes limitações:

* As configurações se aplicam uniformemente a todos os usuários da organização. Configurações por grupo ainda não são suportadas.
* Você não pode distribuir um arquivo [`managed-mcp.json`](/docs/pt/managed-mcp) através de configurações gerenciadas pelo servidor. Entregue as chaves de política `allowedMcpServers` e `deniedMcpServers` lá em vez disso. No Claude Code v2.1.259 ou posterior, você também pode fornecer servidores remotos com [`managedMcpServers`](/docs/pt/managed-mcp#provide-servers-through-managed-settings), que aceita apenas servidores `http` e `sse` e não assume controle exclusivo da forma que o arquivo faz.

  Claude Code lê um `managed-mcp.json` implantado em seu [caminho do sistema](/docs/pt/managed-mcp#exclusive-control-with-managed-mcp-json) separadamente da camada de configurações gerenciadas, portanto, o arquivo ainda se aplica quando as configurações gerenciadas pelo servidor estão em vigor.
* Configurações restritas a fontes de política em nível do SO, como `policyHelper` e `wslInheritsWindowsSettings`, não são honradas. Implante-as através de MDM ou um arquivo `managed-settings.json` do sistema em vez disso. Um `policyHelper` implantado dessa forma é executado apenas quando sua fonte é a selecionada em [precedência dentro da camada gerenciada](/docs/pt/managed-settings#precedence-within-the-managed-tier).

<h2 id="settings-delivery">
  Entrega de configurações
</h2>

<h3 id="settings-precedence">
  Precedência de configurações
</h3>

As configurações gerenciadas pelo servidor e as [configurações gerenciadas pelo endpoint](/docs/pt/managed-settings#delivery-mechanisms) ocupam o nível mais alto na [hierarquia de configurações](/docs/pt/settings#settings-precedence) do Claude Code. Nenhum outro nível de configurações pode substituí-las, incluindo argumentos de linha de comando, exceto pelas [exceções à precedência de configurações gerenciadas](/docs/pt/settings#exceptions-to-managed-settings-precedence).

Dentro do nível gerenciado, o Claude Code usa por padrão a primeira fonte que entrega pelo menos uma chave de política, verificando primeiro as configurações gerenciadas pelo servidor e depois as configurações gerenciadas pelo endpoint, exceto pelas [chaves de exceção cobertas a seguir](#per-key-exceptions-across-managed-sources). [Como o Claude Code combina fontes gerenciadas](/docs/pt/managed-settings#precedence-within-the-managed-tier) tem a classificação completa, a exclusão para as chaves de controle e o opt-in que se aplica a cada fonte.

Se a fonte selecionada for uma política MDM ou arquivo de configurações gerenciadas cujo [`policyHelper`](/docs/pt/settings-reference#policyhelper) fornece configurações gerenciadas, a saída do helper substitui essa fonte como a única configuração gerenciada para a execução. O Claude Code não consulta um `policyHelper` configurado em MDM ou configurações baseadas em arquivo enquanto as configurações gerenciadas pelo servidor entregam uma chave de política.

Se uma busca posterior encontrar as configurações gerenciadas pelo servidor removidas, o Claude Code executa esse helper imediatamente em vez de na próxima inicialização. A entrada [`policyHelper`](/docs/pt/settings-reference#policyhelper) cobre o que acontece quando essa execução falha.

Se você limpar sua configuração gerenciada pelo servidor no console de administração com a intenção de voltar a uma plist gerenciada pelo endpoint ou política de registro, esteja ciente de que [configurações em cache](#fetch-and-caching-behavior) persistem em máquinas cliente até a próxima busca bem-sucedida, e as chaves que [se aplicam apenas na próxima inicialização](#fetch-and-caching-behavior), como `model`, permanecem em vigor até que cada cliente reinicie. Execute `/status` para ver qual fonte gerenciada está ativa.

<h3 id="per-key-exceptions-across-managed-sources">
  Exceções por chave entre fontes gerenciadas
</h3>

Três tipos de chaves são exceções à regra de não mesclagem:

* **Chaves de bloqueio entre fontes**: um pequeno conjunto de chaves, como os bloqueios da lista de permissão de sandbox, [listadas na página de configurações gerenciadas](/docs/pt/managed-settings#precedence-within-the-managed-tier). O Claude Code as honra quando qualquer fonte gerenciada controlada por administrador as define; o nível de registro HKCU gravável pelo usuário é excluído.

  Quando um [`policyHelper`](/docs/pt/settings-reference#policyhelper) fornece configurações gerenciadas, sua saída é a única fonte que essas verificações leem, exceto por [`forceRemoteSettingsRefresh`](/docs/pt/settings-reference#forceremotesettingsrefresh), que o Claude Code lê das fontes de administrador diretamente na inicialização.
* **O bloco `env`**: além da unidade de telemetria e variáveis de roteamento emparelhadas com uma chave de credencial, ambas cobertas abaixo, ele se mescla por chave entre as fontes controladas por administrador. Para cada variável de ambiente, a fonte de prioridade mais alta que a define vence, e as fontes de administrador inferiores preenchem variáveis que as fontes superiores deixam não definidas. Uma entrada `env` gerenciada pelo endpoint, portanto, se aplica sempre que a configuração gerenciada pelo servidor deixa essa variável não definida, ou enquanto um valor de servidor em cache para ela é [retido pendente de confirmação do servidor](#fetch-and-caching-behavior). Requer Claude Code v2.1.223 ou posterior. Antes da v2.1.223, o Claude Code aplica apenas o bloco `env` da fonte selecionada.
  * **Unidade de telemetria**: as chaves do exportador `OTEL_EXPORTER_OTLP_*`, os toggles de captura de conteúdo `OTEL_LOG_*`, `OTEL_LOGS_EXPORTER` e as variáveis de rastreamento beta `ENABLE_BETA_TRACING_DETAILED` e `BETA_TRACING_ENDPOINT` seguem a fonte mais alta que define qualquer uma delas como uma unidade. Uma fonte que entrega a chave de credencial `otelHeadersHelper` também reclama a unidade, mas coloca essas variáveis apenas quando é a fonte selecionada: uma fonte que não é selecionada mas entrega a chave não contribui com nenhuma delas e ainda bloqueia fontes inferiores de preenchê-las. De qualquer forma, um endpoint do exportador de uma fonte nunca pode ser emparelhado com credenciais de outra.
  * **Roteamento emparelhado com credencial**: uma fonte que emparelha variáveis de roteamento com uma chave de credencial somente de fonte selecionada, como `apiKeyHelper` ou `otelHeadersHelper`, contribui com essas variáveis de roteamento apenas quando vence o slot.
* **Chaves de login do gateway**: o Claude Code nunca lê [`forceLoginGatewayUrl`](/docs/pt/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/pt/settings-reference#gatewayinternalnetworks) ou o valor `"gateway"` de [`forceLoginMethod`](/docs/pt/settings-reference#forceloginmethod) das configurações gerenciadas pelo servidor, portanto um valor lá não se aplica nem oculta um definido em uma política MDM ou arquivo de configurações gerenciadas. A entrada [`managedSourcesBehavior`](/docs/pt/settings-reference#managedsourcesbehavior) diz qual fonte de administrador na máquina as fornece.

<h3 id="fetch-and-caching-behavior">
  Comportamento de busca e cache
</h3>

O Claude Code busca configurações dos servidores da Anthropic na inicialização e faz polling para atualizações a cada hora durante sessões ativas.

Um cliente conectado através de um [gateway de aplicativos Claude](#platform-availability) busca suas configurações do gateway e aguarda essa busca antes da sessão iniciar, portanto a busca nas listas abaixo não se aplica a ele. [Impor inicialização com falha fechada](#enforce-fail-closed-startup) cobre o que acontece quando essa busca falha.

**Primeiro lançamento sem configurações em cache:**

* Quando um desenvolvedor se conecta na inicialização, como em uma primeira execução ou após `/logout`, o Claude Code aguarda até cinco segundos pela busca antes de abrir a sessão. Quando a política chega a tempo, o Claude Code a aplica desde a primeira tela e mostra seus [`companyAnnouncements`](/docs/pt/settings-reference#companyannouncements) nela. Quando o payload precisa de [aprovação de segurança](#security-approval-dialogs), o Claude Code encerra a espera e aplica o payload uma vez que o desenvolvedor aprova
* Em qualquer outra inicialização, e quando essa espera de cinco segundos se esgota, o Claude Code abre a sessão enquanto a busca continua, portanto uma breve janela passa antes das configurações carregarem e as restrições entrarem em vigor
* Se a busca falhar, o Claude Code continua sem configurações gerenciadas pelo servidor e avisa em sessões interativas que nenhuma política remota se aplica; as configurações gerenciadas pelo endpoint ainda se aplicam. Se uma fonte gerenciada define [`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup), o Claude Code sai em vez disso

**Lançamentos subsequentes com configurações em cache:**

* As configurações em cache se aplicam imediatamente na inicialização, exceto pelos valores em cache `modelPricing` e `managedMcpServers` e as variáveis de ambiente que o Claude Code retém até que o servidor confirme o payload
* Um [`modelPricing`](/docs/pt/settings-reference#modelpricing) em cache não se aplica até que a busca da sessão confirme o payload. Até então, os valores de custo que os desenvolvedores veem em `/usage` e na linha de status estão no preço de lista
* Um bloco [`managedMcpServers`](/docs/pt/settings-reference#managedmcpservers) em cache não se aplica até que a busca da sessão confirme o payload. O Claude Code aguarda até 30 segundos por essa busca antes de conectar servidores MCP. Se a busca falhar ou expirar, a sessão inicia sem os servidores da organização, `/status` diz isso, e eles se conectam uma vez que uma busca posterior os confirma. Veja [Quando servidores fornecidos se conectam](/docs/pt/managed-mcp#when-provided-servers-connect) para o comportamento completo, incluindo primeiro lançamento. Requer Claude Code v2.1.259 ou posterior
* O Claude Code busca configurações atualizadas em segundo plano
* As configurações em cache persistem através de falhas de rede. Se a busca de inicialização falhar, o Claude Code avisa em sessões interativas que a política em cache está em vigor
* Até que uma busca seja bem-sucedida, os valores retidos na inicialização permanecem retidos

O Claude Code retém várias categorias de variáveis no bloco `env` em cache até que o servidor confirme o payload para a sessão. Isso evita que um proxy em cache, autoridade de certificação, endpoint ou valor de credencial redirecione, intercepte ou reautentique a busca de configurações que confirma o payload. O endurecimento se aplica apenas ao cache de configurações buscado do servidor: [configurações gerenciadas pelo endpoint](/docs/pt/managed-settings#delivery-mechanisms) implantadas através de MDM ou `managed-settings.json` não são afetadas. A retenção requer Claude Code v2.1.198 ou posterior; antes da v2.1.198, o bloco `env` em cache inteiro se aplica na inicialização. As categorias retidas incluem:

* Configuração de proxy e TLS, como `HTTPS_PROXY`, `NODE_EXTRA_CA_CERTS` e as variáveis de certificado de cliente mTLS `CLAUDE_CODE_CLIENT_CERT` e `CLAUDE_CODE_CLIENT_KEY`
* Roteamento de API e seleção de provedor, incluindo `ANTHROPIC_BASE_URL`, as variáveis de seleção de provedor como `CLAUDE_CODE_USE_BEDROCK` e `CLAUDE_CODE_USE_VERTEX`, e as URLs de endpoint do provedor como `ANTHROPIC_BEDROCK_BASE_URL`
* Credenciais de autenticação, como `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` e `CLAUDE_CODE_OAUTH_TOKEN`
* O seletor de diretório de configuração `CLAUDE_CONFIG_DIR`
* Seletores de fonte de credencial e diretório de configuração, no Claude Code v2.1.223 ou posterior: as variáveis de Workload Identity Federation como `ANTHROPIC_FEDERATION_RULE_ID` e `ANTHROPIC_IDENTITY_TOKEN`, os seletores de perfil e diretório de configuração `ANTHROPIC_PROFILE` e `ANTHROPIC_CONFIG_DIR`, e as variáveis de diretório do sistema operacional `HOME`, `XDG_CONFIG_HOME`, `APPDATA` e `USERPROFILE`

O Claude Code lê as variáveis de Workload Identity Federation e os seletores `ANTHROPIC_PROFILE` e `ANTHROPIC_CONFIG_DIR` apenas na inicialização, portanto um valor entregue pelo servidor para eles não muda a fonte de credencial da sessão mesmo após a busca ser bem-sucedida. Para entregar esses seletores no Claude Code v2.1.223 ou posterior, use [configurações gerenciadas pelo endpoint](/docs/pt/managed-settings#delivery-mechanisms) como MDM ou `managed-settings.json`. Para `CLAUDE_CONFIG_DIR` e as variáveis de diretório do sistema operacional, a retenção em si é a proteção: o valor em cache fica fora do ambiente até que o servidor confirme o payload.

Todas as outras chaves no bloco `env` em cache se aplicam na inicialização. Uma vez que o servidor confirme o payload, e você o aprove se precisar de [aprovação de segurança](#security-approval-dialogs), as variáveis retidas se aplicam pelo resto da sessão.

Se sua organização precisa de um proxy para alcançar `api.anthropic.com`, a retenção afeta apenas o bloco `env` entregue pelo servidor em si: um proxy definido em um bloco `env` [gerenciado pelo endpoint](/docs/pt/managed-settings#delivery-mechanisms) através de MDM ou `managed-settings.json`, no ambiente do shell ou em [configurações do usuário](/docs/pt/settings#where-settings-live) alcança a busca de configurações. A fonte gerenciada pelo endpoint requer Claude Code v2.1.223 ou posterior: o valor de proxy gerenciado pelo servidor em cache é retido até que a busca o confirme, portanto o valor gerenciado pelo endpoint se preenche por chave e alcança a busca em si. Antes da v2.1.223, use o ambiente do shell ou configurações do usuário para que o proxy se aplique junto com um payload de servidor em cache. O primeiro lançamento não tem cache, portanto uma fonte gerenciada pelo endpoint, o ambiente do shell ou configurações do usuário ainda é necessário para a busca inicial.

O Claude Code aplica a maioria das atualizações de configurações a sessões em execução sem uma reinicialização. Algumas atualizações se aplicam apenas na próxima inicialização, incluindo configuração do exportador OpenTelemetry, a chave `model` e a remoção de uma variável do bloco `env`.

<h3 id="invalid-entries-in-delivered-settings">
  Entradas inválidas em configurações entregues
</h3>

Quando parte de um payload falha na validação do esquema, o Claude Code exibe um erro de validação e aplica todas as configurações válidas restantes; [Entradas inválidas em configurações gerenciadas](/docs/pt/managed-settings#invalid-entries-in-managed-settings) diz o que ele descarta e quais chaves voltam a um valor mais rigoroso. Requer Claude Code v2.1.169 ou posterior.

A entrega gerenciada pelo servidor adiciona esses comportamentos:

* O cache em `~/.claude/remote-settings.json` armazena o payload salvo com entradas inválidas removidas, exceto por valores inválidos de `cleanupPeriodDays` e `desktopSessionCleanupPeriodDays`, que permanecem na cópia em cache e nunca são aplicados.
* Quando nenhum campo no payload pode ser salvo e o payload não é apenas essas chaves de retenção, o Claude Code rejeita o payload, mantém as últimas configurações em cache aceitas e escreve `Remote settings: Settings validation failed - no fields could be salvaged` no log de depuração. Com `forceRemoteSettingsRefresh` definido, a CLI sai em vez disso.
* A [caixa de diálogo de aprovação de segurança](#security-approval-dialogs) avalia o payload salvo, portanto uma entrada inválida removida nunca é apresentada para aprovação e nunca é executada.

Para depurar problemas de entrega, execute `claude --debug-file <path>` e procure no log por `Remote settings`. Valide uma alteração de payload com `claude doctor` em uma máquina de teste antes de implantá-la na organização.

<h3 id="enforce-fail-closed-startup">
  Impor inicialização com falha fechada
</h3>

Por padrão, se a busca de configurações remotas falhar na inicialização, a CLI continua com as configurações em cache da última busca bem-sucedida, exceto pelos [valores que o Claude Code retém](#fetch-and-caching-behavior) até que uma busca seja bem-sucedida. Em uma máquina que nunca as buscou, a CLI continua sem configurações gerenciadas pelo servidor e ainda aplica qualquer [configuração gerenciada pelo endpoint](/docs/pt/managed-settings#delivery-mechanisms) no dispositivo.

Para impedir que clientes iniciem em configurações gerenciadas pelo servidor em cache ou ausentes, defina `forceRemoteSettingsRefresh: true` em suas configurações gerenciadas.

Clientes conectados através de um [gateway de aplicativos Claude](#platform-availability) aguardam a busca de inicialização independentemente de você definir isso, e lidam com uma busca falhada da seguinte forma:

* Se o gateway responde um lançamento interativo assistido com um `401` e essa configuração está desativada, o gateway encerrou esse login. O Claude Code imprime [`Cloud gateway session expired — run /login to reconnect.`](/docs/pt/errors#cloud-gateway-session-expired) e abre a sessão desconectada do gateway até que o usuário execute `/login`.
* Quando a busca falha de qualquer outra forma, ou em qualquer outro tipo de lançamento exceto um subcomando `claude auth`, o cliente sai com um erro.

Quando essa configuração está ativa em uma sessão que busca configurações gerenciadas pelo servidor, a CLI bloqueia na inicialização até que as configurações remotas sejam buscadas recentemente. Se a busca falhar, a CLI sai em vez de prosseguir sem a política. Essa configuração se auto-perpetua: uma vez entregue do servidor, ela também é armazenada em cache localmente para que as inicializações subsequentes imponham o mesmo comportamento mesmo antes da primeira busca bem-sucedida de uma nova sessão. Uma sessão que [não busca configurações gerenciadas pelo servidor](#platform-availability) inicia sem aguardar.

Para ativar isso, adicione a chave à sua configuração de configurações gerenciadas:

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Você também pode definir essa chave em um [perfil MDM](/docs/pt/managed-settings#delivery-mechanisms) gerenciado pelo endpoint ou arquivo `managed-settings.json` do sistema para impor comportamento de falha fechada no primeiro lançamento, antes de qualquer payload do servidor ter chegado. Esse sinalizador é uma exceção à [regra de precedência](#settings-precedence) acima: o Claude Code o honra quando qualquer fonte gerenciada controlada por administrador o define, mesmo se um payload gerenciado pelo servidor em cache também estiver presente, portanto ele não ignora um valor entregue por MDM quando configurações gerenciadas pelo servidor existem.

Quando um [`policyHelper`](/docs/pt/settings-reference#policyhelper) fornece configurações gerenciadas, sua saída substitui todas as outras fontes gerenciadas para as chaves que o Claude Code lê após a inicialização. Para as fontes que o Claude Code lê essa chave, veja [sua entrada de configurações](/docs/pt/settings-reference#forceremotesettingsrefresh). A entrada `policyHelper` diz quais fontes o Claude Code lê o helper e quando ele é executado.

A busca de configurações também envia um cabeçalho `Cache-Control: no-cache` para que proxies HTTP intermediários não sirvam uma resposta obsoleta.

Antes de ativar essa configuração, certifique-se de que suas políticas de rede permitem conectividade a `api.anthropic.com`. Se esse endpoint estiver inacessível, a CLI sai na inicialização e os usuários não podem iniciar o Claude Code.

Os subcomandos `claude auth` como `claude auth login` estão isentos dessa verificação e da saída de inicialização do gateway, para que os usuários possam se reautenticar quando credenciais expiradas forem o motivo da falha na busca de configurações.

<h3 id="security-approval-dialogs">
  Caixas de diálogo de aprovação de segurança
</h3>

Certas configurações que podem representar riscos de segurança exigem aprovação explícita do usuário antes de o Claude Code aplicá-las em uma sessão interativa:

* **Configurações de comando shell**: configurações que executam comandos shell, como `apiKeyHelper`, `statusLine` e `otelHeadersHelper`
* **Configurações de binário de sandbox**: `sandbox.bwrapPath`, `sandbox.socatPath` e `sandbox.ripgrep`. Cada uma dessas configurações aponta para um executável, e o Claude Code executa esse executável
* **Configurações de rede e isolamento de sandbox**: configurações de [sandbox](/docs/pt/sandboxing) que deixam o proxy de sandbox ler, redirecionar ou autenticar tráfego, ou que enfraquecem o isolamento do sandbox: `sandbox.network.tlsTerminate`, `sandbox.network.httpProxyPort`, `sandbox.network.socksProxyPort`, `sandbox.credentials`, `sandbox.allowAppleEvents`, `sandbox.enableWeakerNestedSandbox`, `sandbox.enableWeakerNetworkIsolation`, `sandbox.filesystem.disabled`, `sandbox.network.allowAllUnixSockets`, `sandbox.network.allowUnixSockets` e `sandbox.network.allowMachLookup`. Um bloco `sandbox.credentials` que contém apenas regras `deny` não precisa de aprovação, pois restringe o sandbox sem dar ao proxy uma credencial. Antes da v2.1.251, o Claude Code aplicava essas configurações sem aprovação
* **Variáveis de ambiente personalizadas**: variáveis `env` entregues que exigem aprovação do usuário, como variáveis de proxy e URL base; veja [Variáveis de ambiente e a caixa de diálogo de aprovação](#environment-variables-and-the-approval-dialog)
* **Configurações de hook**: qualquer definição de hook

Quando essas configurações estão presentes, os usuários veem uma caixa de diálogo de segurança explicando o que está sendo configurado. Os usuários devem aprovar para prosseguir. Se um usuário rejeitar as configurações, o Claude Code sai.

Um CLAUDE.md gerenciado entregue através da chave [`claudeMd`](/docs/pt/settings-reference#claudemd) não requer aprovação, porque é texto de instrução para Claude em vez de um comando que o Claude Code executa. O Claude Code ainda verifica [permissões](/docs/pt/permissions) para as ferramentas que Claude usa ao seguir essas instruções. Antes da v2.1.260, um valor `claudeMd` exigia aprovação também.

<h4 id="approval-memory">
  Memória de aprovação
</h4>

O Claude Code registra sua aprovação em seu diretório de configuração, `~/.claude` a menos que você defina [`CLAUDE_CONFIG_DIR`](/docs/pt/env-vars). O que ele registra depende da credencial que a busca de configurações usa:

* **Um login claude.ai salvo por `/login` ou `claude auth login`, ou o [login do Console sem chave](/docs/pt/authentication#sign-in-without-an-api-key)**: uma aprovação por organização, mantida pela conta que aprovou mais recentemente.
* **Um login de [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway)**: uma aprovação por gateway.

  Se você se desconectar e reconectar ao mesmo gateway, o Claude Code não mostra a caixa de diálogo novamente enquanto as configurações que exigem aprovação não forem alteradas. O Claude Code a mostra novamente quando essas configurações mudam, quando você se conecta a um gateway diferente e quando você aceita um novo certificado para o mesmo gateway.

  O Claude Code não salva aprovação para um gateway de desenvolvimento de loopback alcançado por HTTP simples, portanto a caixa de diálogo aparece novamente após cada login.
* **Qualquer outra credencial**, como uma chave de API ou `CLAUDE_CODE_OAUTH_TOKEN`: uma aprovação para as configurações entregues, mantida com a cópia em cache das configurações nesse diretório de configuração. O Claude Code mostra a caixa de diálogo novamente quando as configurações que exigem aprovação mudam, e após você executar `/logout` ou `claude auth logout`, qualquer um dos quais exclui a cópia em cache.

Uma aprovação para `sandbox.credentials` ou `sandbox.network.tlsTerminate` também cobre as entradas [`sandbox.network.allowedDomains`](/docs/pt/settings-reference#sandbox-network-alloweddomains) nessas mesmas configurações entregues, porque ambas as configurações atuam nessa lista de permissão. A caixa de diálogo aparece novamente quando seu administrador adiciona ou remove uma dessas entradas, mesmo que `sandbox.network.allowedDomains` não exija aprovação por si só.

Com um login claude.ai salvo:

* Se você se desconectar e reconectar, ou mudar para outra organização e depois retornar, o Claude Code não mostra a caixa de diálogo novamente enquanto essas configurações não forem alteradas, a menos que outra conta as tenha aprovado para essa organização no mesmo diretório de configuração no meio tempo.
* Se você se conectar à mesma organização com uma conta diferente, o Claude Code mostra a caixa de diálogo novamente mesmo quando as configurações não forem alteradas. A aprovação dessa conta substitui a anterior, portanto quando você muda de volta, o Claude Code mostra a caixa de diálogo mais uma vez.

O Claude Code nem sempre pode mostrar a caixa de diálogo. Cada caso abaixo diz quais configurações se aplicam quando ele não pode e quando você vê a caixa de diálogo novamente:

* **Uma sessão interativa que não pode mostrar a caixa de diálogo**: o Claude Code não aplica as configurações entregues e mantém as últimas configurações aprovadas. A caixa de diálogo aparece na próxima sessão que pode mostrá-la. Requer Claude Code v2.1.211 ou posterior.
* **`claude install` ou `claude update`**: o Claude Code não mostra a caixa de diálogo durante nenhum dos comandos. O comando é executado com as últimas configurações aprovadas, e a caixa de diálogo aparece em sua próxima sessão interativa. Se o Claude Code aguardar a busca de configurações na inicialização, como com [`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup) definido ou em uma implantação de [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway), ele mostra a caixa de diálogo durante o comando em vez disso, e uma execução de instalação de um pipe falha; veja [`Raw mode is not supported` durante install](/docs/pt/troubleshoot-install#raw-mode-is-not-supported-during-install). Antes da v2.1.246, o Claude Code tentava mostrar a caixa de diálogo durante esses comandos também.
* **Um erro fecha a caixa de diálogo antes de você responder**: o Claude Code não aplica as configurações entregues e mantém as últimas configurações aprovadas. Ele mostra a caixa de diálogo novamente na próxima sessão que pode mostrá-la.
* **Uma execução não interativa**, como `claude -p` ou uma sessão do Agent SDK: o Claude Code não pode mostrar a caixa de diálogo, portanto quando as configurações entregues exigiriam aprovação, ele as aplica apenas para essa execução. Ele não as registra como aprovadas ou as escreve no [cache local](#fetch-and-caching-behavior), e a próxima sessão interativa mostra a caixa de diálogo. Até que um usuário aprove em uma sessão interativa, cada execução não interativa busca as configurações novamente na inicialização. Antes da v2.1.207, uma execução não interativa salvava as configurações como aprovadas, portanto as sessões interativas posteriores nunca mostravam a caixa de diálogo para elas.

<h4 id="environment-variables-and-the-approval-dialog">
  Variáveis de ambiente e a caixa de diálogo de aprovação
</h4>

O Claude Code aplica algumas variáveis `env` entregues sem mostrar ao usuário a caixa de diálogo de aprovação, incluindo:

* Toggles de recurso e comando
* Seleção de modelo e configurações de comportamento, como `ANTHROPIC_MODEL`, `DISABLE_PROMPT_CACHING` e `CLAUDE_CODE_EFFORT_LEVEL`
* Contexto de janela e configurações de compactação, como `DISABLE_AUTO_COMPACT`
* Opções de UI de terminal e acessibilidade
* Limites numéricos, orçamentos e timeouts

Outras variáveis entregues podem exigir aprovação do usuário antes de entrarem em vigor; um valor de proxy, URL base ou `OTEL_EXPORTER_OTLP_ENDPOINT` não vazio sempre faz. Quando uma variável entregue precisa de aprovação, a caixa de diálogo a nomeia, portanto o usuário vê exatamente o que a política está pedindo para definir. Antes da v2.1.218, o Claude Code aplicava menos variáveis sem perguntar ao usuário, portanto configurações como `DISABLE_AUTO_COMPACT` acionavam a caixa de diálogo em qualquer valor não vazio.

O Claude Code decide se quatro toggles de privacidade precisam de aprovação pelo valor entregue em vez do nome da variável: `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, `DISABLE_ERROR_REPORTING`, `DISABLE_TELEMETRY` e `DO_NOT_TRACK`. Um valor verdadeiro como `1` ou `true` apenas desativa rastreamento, relatório ou outro tráfego não essencial, portanto o Claude Code o aplica sem perguntar ao usuário. Para qualquer outro valor não vazio, o Claude Code mostra a caixa de diálogo. Antes da v2.1.218, todos eles exceto `DO_NOT_TRACK` se aplicavam sem aprovação em qualquer valor, e `DO_NOT_TRACK` acionava a caixa de diálogo em qualquer valor não vazio.

O Claude Code também decide se [`API_FORCE_IDLE_TIMEOUT`](/docs/pt/env-vars) precisa de aprovação pelo valor entregue: um valor verdadeiro apenas ativa o [timeout de inatividade do corpo](/docs/pt/network-config#streaming-idle-watchdogs), portanto o Claude Code o aplica sem perguntar ao usuário. Para qualquer outro valor não vazio, o Claude Code mostra a caixa de diálogo. Antes da v2.1.248, qualquer valor não vazio acionava a caixa de diálogo.

Se [`ANTHROPIC_CUSTOM_HEADERS`](/docs/pt/env-vars#variables) precisa de aprovação também depende do valor entregue. Cabeçalhos que apenas marcam solicitações, como `Accept-Language`, se aplicam sem a caixa de diálogo. Uma linha que nomeia uma credencial, um seletor de org ou tenant, um roteamento ou substituição de host, ou um cabeçalho de comportamento de API, como `Authorization`, `X-Api-Key`, `Host`, `anthropic-beta` ou os cabeçalhos `X-Amzn-Bedrock-*`, requer aprovação. Também requer uma linha cujo nome não é um token de cabeçalho HTTP válido, ou cujo valor contém um caractere que um cabeçalho HTTP não pode carregar. A verificação corresponde a palavras dentro do nome do cabeçalho, portanto `X-Client-Version`, que contém `client` e `version`, requer aprovação também. Antes da v2.1.251, qualquer valor `ANTHROPIC_CUSTOM_HEADERS` se aplicava sem ela.

Um valor falso como `0` ou `false` para [`ENABLE_BETA_TRACING_DETAILED`](/docs/pt/env-vars#variables) ou [`OTEL_LOG_RAW_API_BODIES`](/docs/pt/env-vars#variables) se aplica sem a caixa de diálogo, porque apenas desativa rastreamento detalhado ou captura de corpo de API bruto. Qualquer outro valor não vazio para qualquer variável requer aprovação.

<h2 id="platform-availability">
  Disponibilidade de plataforma
</h2>

As configurações gerenciadas pelo servidor exigem uma conexão direta a `api.anthropic.com`. A entrega também exige que a sessão se autentique com uma destas credenciais:

* Um login OAuth de Equipe ou Empresa
* Um token OAuth fornecido através de `CLAUDE_CODE_OAUTH_TOKEN`
* Uma chave de API configurada diretamente
* Um [perfil Anthropic](/docs/pt/authentication#anthropic-profiles-and-federation-credentials) `user_oauth`, a menos que o perfil defina um `base_url` diferente da API Anthropic. Requer Claude Code v2.1.257 ou posterior.

Nem as chaves retornadas por um script [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper) nem as credenciais de [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) acionam a busca de configurações.

Em uma sessão de [Cowork](https://claude.com/docs/cowork/overview) no aplicativo Claude Desktop, Claude Code não busca configurações gerenciadas pelo servidor do console de administração claude.ai, mesmo quando o usuário se conecta com uma conta de Equipe ou Empresa. [Onde e quando uma política se aplica](/docs/pt/managed-settings#where-and-when-a-policy-applies) cobre qual política alcança sessões de Cowork na máquina do usuário e sessões de Cowork remotas. claude.ai ainda aplica suas listas [`strictKnownMarketplaces`](/docs/pt/settings-reference#strictknownmarketplaces) e [`blockedMarketplaces`](/docs/pt/settings-reference#blockedmarketplaces) quando um usuário de Cowork adiciona um marketplace de um repositório git em claude.ai ou de **Customize** na aba Cowork. [Como as restrições funcionam](/docs/pt/plugins/org#restrict-what-users-can-install) descreve essa verificação.

Se você exportar uma variável de provedor `CLAUDE_CODE_USE_*` ou um `ANTHROPIC_BASE_URL` não padrão em seu shell, Claude Code ignora a busca de configurações para suas sessões. [`claude doctor` e `/status` relatam a busca ignorada e sua causa](#verify-settings-delivery).

Você não pode limpar a exportação com um bloco `env` gerenciado pelo servidor, porque o bloco chega através da busca que a exportação impede. Um bloco `env` de [configurações gerenciadas pelo endpoint](/docs/pt/managed-settings#delivery-mechanisms) também não restaura a busca: Claude Code verifica a elegibilidade antes de aplicar blocos `env` gerenciados, portanto a alteração de valor gerenciado pelo endpoint altera a seleção de provedor da sessão, mas a busca permanece ignorada.

Para restaurar a entrega gerenciada pelo servidor, remova a exportação do seu shell ou defina a variável como `""` no bloco `env` de suas configurações de usuário, que se aplica antes da verificação de elegibilidade. Para impor política sem depender de usuários para alterar seus shells, entregue as configurações através do canal gerenciado pelo endpoint.

Para implantações do Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) auto-hospedado fornece a entrega equivalente de configurações gerenciadas remotamente: clientes assinados no gateway buscam configurações gerenciadas do gateway em vez de `api.anthropic.com`. A semântica de falha difere na inicialização: um cliente de gateway que não consegue alcançar o gateway sai com um erro em vez de fazer fallback para configurações em cache, enquanto a atualização de fundo por hora é fail-open em ambos os canais.

<h2 id="audit-logging">
  Auditoria de logs
</h2>

Os eventos de log de auditoria para alterações de configurações estão disponíveis através da API de conformidade ou exportação de log de auditoria. Entre em contato com sua equipe de conta da Anthropic para obter acesso.

Os eventos de auditoria incluem o tipo de ação executada, a conta e o dispositivo que executaram a ação, e referências aos valores anteriores e novos.

<h2 id="security-considerations">
  Considerações de segurança
</h2>

As configurações gerenciadas pelo servidor fornecem aplicação de política centralizada, mas funcionam como um controle do lado do cliente, não como um limite de segurança. Em dispositivos não gerenciados, um usuário não precisa de acesso de administrador ou sudo para contorná-las.

| Cenário                                                                        | Comportamento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :----------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Usuário edita o arquivo de configurações em cache                              | O arquivo adulterado se aplica na inicialização, exceto pelos [valores que Claude Code retém](#fetch-and-caching-behavior) até que o servidor confirme o payload. A próxima busca do servidor restaura as configurações corretas, exceto pelas [chaves que se aplicam apenas no próximo lançamento](#fetch-and-caching-behavior), como `model` ou uma variável adicionada ao bloco `env`, que permanecem em vigor até o relançamento                                                                                                                                                                                                                                                                                                     |
| Usuário deleta o arquivo de configurações em cache                             | [Comportamento de primeiro lançamento](#fetch-and-caching-behavior) ocorre                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Usuário executa um binário Claude Code modificado                              | Um usuário que pode executar um cliente modificado pode contornar qualquer controle do lado do cliente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Usuário executa uma versão anterior do Claude Code                             | Versões que antecedem as configurações gerenciadas pelo servidor não as buscam ou aplicam                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| API está indisponível                                                          | As configurações em cache se aplicam se disponíveis, exceto pelos [valores que Claude Code retém](#fetch-and-caching-behavior) até que uma busca seja bem-sucedida. Sem um cache, Claude Code não aplica nenhuma configuração gerenciada pelo servidor até a próxima busca bem-sucedida e ainda aplica qualquer [configuração gerenciada pelo endpoint](/docs/pt/managed-settings#delivery-mechanisms) no dispositivo. Com `forceRemoteSettingsRefresh: true`, a CLI sai em vez de continuar, exceto para [subcomandos `claude auth`](#enforce-fail-closed-startup). Clientes conectados através de um [gateway de aplicativos Claude](#platform-availability) saem na inicialização sem essa configuração, com a mesma exceção `claude auth` |
| Usuário se autentica com uma organização diferente                             | As configurações não são entregues para contas fora da organização gerenciada                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Usuário configura um [provedor de modelo de terceiros](#platform-availability) | As configurações gerenciadas pelo servidor são ignoradas. Isso inclui definir `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_MANTLE`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, `CLAUDE_CODE_USE_ANTHROPIC_AWS`, ou um `ANTHROPIC_BASE_URL` não padrão                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Tráfego de rede é interceptado ou redirecionado                                | Validação TLS desabilitada ou tráfego interceptado pode alterar as configurações que o cliente recebe                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |

Para registrar edições em arquivos de configurações locais, incluindo `managed-settings.json`, use [hooks `ConfigChange`](/docs/pt/hooks#configchange). Claude Code não os executa quando as configurações gerenciadas pelo servidor chegam ou são atualizadas, ou quando um perfil MDM ou política de registro muda, e um hook não pode bloquear uma alteração `policy_settings`.

Para restringir quais organizações seus usuários podem acessar com as credenciais que o cliente fornece, consulte [Enforce network-level access control with Tenant Restrictions](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions) no Claude Help Center. Para garantias de aplicação mais fortes, use [configurações gerenciadas pelo endpoint](/docs/pt/managed-settings#delivery-mechanisms) em dispositivos inscritos em uma solução MDM.

<h2 id="see-also">
  Veja também
</h2>

Páginas relacionadas para gerenciar a configuração do Claude Code:

* [Todas as configurações](/docs/pt/settings-reference): todas as chaves de configuração
* [Configurações gerenciadas pelo endpoint](/docs/pt/managed-settings#delivery-mechanisms): configurações gerenciadas implantadas em dispositivos por TI
* [Authentication](/docs/pt/authentication): configure o acesso do usuário ao Claude Code
* [Security](/docs/pt/security): salvaguardas de segurança e melhores práticas
