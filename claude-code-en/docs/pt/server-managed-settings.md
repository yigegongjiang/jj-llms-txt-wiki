> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar configurações gerenciadas pelo servidor

> Configure centralmente o Claude Code para sua organização através de configurações entregues pelo servidor, sem exigir infraestrutura de gerenciamento de dispositivos.

As configurações gerenciadas pelo servidor permitem que Proprietários da organização configurem centralmente o Claude Code a partir de [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code) no console claude.ai. Os clientes do Claude Code buscam essas configurações automaticamente quando os usuários se autenticam com um login OAuth organizacional ou uma chave de API configurada diretamente, em plataformas onde a entrega gerenciada pelo servidor é suportada. Consulte [Disponibilidade de plataforma](#platform-availability).

Essa abordagem foi projetada para organizações que não possuem infraestrutura de gerenciamento de dispositivos ou que precisam gerenciar configurações para usuários em dispositivos não gerenciados.

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

O Claude Code suporta duas abordagens para configuração centralizada. As configurações gerenciadas pelo servidor entregam a configuração dos servidores da Anthropic. As [configurações gerenciadas pelo endpoint](/docs/pt/settings#settings-files) são implantadas diretamente em dispositivos através de políticas nativas do SO (preferências gerenciadas do macOS, registro do Windows) ou arquivos de configurações gerenciadas.

| Abordagem                                                                  | Melhor para                                                       | Modelo de segurança                                                                                                                      |
| :------------------------------------------------------------------------- | :---------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| **Configurações gerenciadas pelo servidor**                                | Organizações sem MDM, ou usuários em dispositivos não gerenciados | Configurações entregues dos servidores da Anthropic no momento da autenticação                                                           |
| **[Configurações gerenciadas pelo endpoint](/docs/pt/settings#settings-files)** | Organizações com MDM ou gerenciamento de endpoint                 | Configurações implantadas em dispositivos via perfis de configuração MDM, políticas de registro ou arquivos de configurações gerenciadas |

Se seus dispositivos estão inscritos em uma solução MDM ou gerenciamento de endpoint, as configurações gerenciadas pelo endpoint fornecem garantias de segurança mais fortes porque o arquivo de configurações pode ser protegido contra modificação do usuário no nível do SO. As configurações gerenciadas pelo endpoint não chegam às [sessões na nuvem](/docs/pt/model-config#surface-coverage), portanto as organizações que usam Claude Code na web devem configurar também as configurações gerenciadas pelo servidor.

<h2 id="configure-server-managed-settings">
  Configurar configurações gerenciadas pelo servidor
</h2>

<Steps>
  <Step title="Abrir o console de administração">
    No console claude.ai, vá para [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code).

    Se o link o redirecionar para uma página diferente de Admin Settings em vez da página Claude Code, sua conta não tem a função necessária. Funções de Admin e outras funções que não sejam Owner não podem visualizar ou editar configurações gerenciadas, portanto, peça a um Owner ou Primary Owner em sua organização para fazer a alteração. Veja [Controle de acesso](#access-control).
  </Step>

  <Step title="Definir suas configurações">
    Adicione sua configuração como JSON. Todas as [configurações disponíveis em `settings.json`](/docs/pt/settings#available-settings) são suportadas, exceto aquelas restritas à entrega de política em nível do SO; veja [Limitações atuais](#current-limitations) para essa lista curta. Isso inclui [hooks](/docs/pt/hooks), [variáveis de ambiente](/docs/pt/env-vars) e [configurações apenas gerenciadas](/docs/pt/permissions#managed-only-settings) como `allowManagedPermissionRulesOnly`.

    Este exemplo impõe uma lista de negação de permissões, impede que os usuários ignorem as permissões e restringe as regras de permissão àquelas definidas nas configurações gerenciadas:

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

    Para configurar o classificador do [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) para que ele saiba quais repositórios, buckets e domínios sua organização confia:

    ```json theme={null}
    {
      "autoMode": {
        "environment": [
          "Source control: github.example.com/acme-corp and all repos under it",
          "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
          "Trusted internal domains: *.corp.example.com"
        ]
      }
    }
    ```

    Como hooks executam comandos shell, os usuários veem uma [caixa de diálogo de aprovação de segurança](#security-approval-dialogs) antes de serem aplicados. Veja [Configurar o modo automático](/docs/pt/auto-mode-config) para saber como as entradas `autoMode` afetam o que o classificador bloqueia e avisos importantes sobre os campos `environment`, `allow`, `soft_deny` e `hard_deny`.
  </Step>

  <Step title="Salvar e implantar">
    Salve suas alterações. Os clientes do Claude Code recebem as configurações atualizadas na próxima inicialização ou ciclo de polling por hora.
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  Verificar entrega de configurações
</h3>

Para confirmar que as configurações estão sendo aplicadas, peça a um usuário para reiniciar o Claude Code. Se a configuração incluir configurações que acionem a [caixa de diálogo de aprovação de segurança](#security-approval-dialogs), o usuário vê um prompt descrevendo as configurações gerenciadas na inicialização. Você também pode verificar que as regras de permissão gerenciadas estão ativas pedindo a um usuário para executar `/permissions` para visualizar suas regras de permissão efetivas.

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

A maioria das [chaves de configurações](/docs/pt/settings#available-settings) funciona em qualquer escopo. Um punhado de chaves são lidas apenas de configurações gerenciadas e não têm efeito quando colocadas em arquivos de configurações de usuário ou projeto. Veja [configurações apenas gerenciadas](/docs/pt/permissions#managed-only-settings) para a lista completa. Qualquer configuração não nessa lista ainda pode ser colocada em configurações gerenciadas e tem a precedência mais alta.

<h3 id="current-limitations">
  Limitações atuais
</h3>

As configurações gerenciadas pelo servidor têm as seguintes limitações:

* As configurações se aplicam uniformemente a todos os usuários da organização. Configurações por grupo ainda não são suportadas.
* Um arquivo [`managed-mcp.json`](/docs/pt/managed-mcp) não pode ser distribuído através de configurações gerenciadas pelo servidor. Entregue as chaves de política `allowedMcpServers` e `deniedMcpServers` lá em vez disso.
* Configurações restritas a fontes de política em nível do SO, como `policyHelper` e `wslInheritsWindowsSettings`, não são honradas. Implante-as através de MDM ou um arquivo `managed-settings.json` do sistema.

<h2 id="settings-delivery">
  Entrega de configurações
</h2>

<h3 id="settings-precedence">
  Precedência de configurações
</h3>

As configurações gerenciadas pelo servidor e as [configurações gerenciadas pelo endpoint](/docs/pt/settings#settings-files) ocupam o nível mais alto na [hierarquia de configurações](/docs/pt/settings#settings-precedence) do Claude Code. Nenhum outro nível de configurações pode substituí-las, incluindo argumentos de linha de comando.

Dentro do nível gerenciado, um [`policyHelper`](/docs/pt/settings#compute-managed-settings-with-a-policy-helper) configurado preempta todas as outras fontes gerenciadas, incluindo configurações gerenciadas pelo servidor: sua saída se torna a única configuração gerenciada para a execução.

Caso contrário, o Claude Code usa a primeira fonte que entrega uma configuração não vazia. As configurações gerenciadas pelo servidor são verificadas primeiro, depois as configurações gerenciadas pelo endpoint. As fontes não se mesclam: se as configurações gerenciadas pelo servidor entregarem qualquer chave, as configurações gerenciadas pelo endpoint são ignoradas. Se as configurações gerenciadas pelo servidor não entregarem nada, as configurações gerenciadas pelo endpoint se aplicam.

Uma exceção se aplica: um pequeno conjunto de [chaves de bloqueio entre fontes](/docs/pt/settings#settings-precedence), como os bloqueios da lista de permissão de sandbox, é honrado quando qualquer fonte gerenciada controlada por administrador os define; o nível de registro HKCU gravável pelo usuário é excluído.

Se você limpar sua configuração gerenciada pelo servidor no console de administração com a intenção de voltar a uma plist gerenciada pelo endpoint ou política de registro, esteja ciente de que [configurações em cache](#fetch-and-caching-behavior) persistem em máquinas cliente até a próxima busca bem-sucedida. Execute `/status` para ver qual fonte gerenciada está ativa.

<h3 id="fetch-and-caching-behavior">
  Comportamento de busca e cache
</h3>

O Claude Code busca configurações dos servidores da Anthropic na inicialização e faz polling para atualizações a cada hora durante sessões ativas.

**Primeiro lançamento sem configurações em cache:**

* O Claude Code busca configurações de forma assíncrona
* Se a busca falhar, o Claude Code continua sem configurações gerenciadas
* Há uma breve janela antes das configurações carregarem onde as restrições ainda não são aplicadas

**Lançamentos subsequentes com configurações em cache:**

* As configurações em cache se aplicam imediatamente na inicialização, exceto pelas variáveis de ambiente de transporte, roteamento e autenticação descritas abaixo
* O Claude Code busca configurações atualizadas em segundo plano
* As configurações em cache persistem através de falhas de rede. As variáveis retidas permanecem retidas até que uma busca seja bem-sucedida

A partir da v2.1.198, o Claude Code retém três categorias de variáveis no bloco `env` em cache até que o servidor confirme o payload para a sessão. Isso evita que um valor de proxy em cache, autoridade de certificação, endpoint ou credencial redirecione, intercepte ou reautentique a busca de configurações que confirma o payload. O endurecimento se aplica apenas ao cache de configurações buscado do servidor: [configurações gerenciadas pelo endpoint](/docs/pt/settings#settings-files) implantadas através de MDM ou `managed-settings.json` não são afetadas. As categorias retidas são:

* Configuração de proxy e TLS, como `HTTPS_PROXY`, `NODE_EXTRA_CA_CERTS` e as variáveis de certificado de cliente mTLS `CLAUDE_CODE_CLIENT_CERT` e `CLAUDE_CODE_CLIENT_KEY`
* Roteamento de API e seleção de provedor, incluindo `ANTHROPIC_BASE_URL`, as variáveis de seleção de provedor como `CLAUDE_CODE_USE_BEDROCK` e `CLAUDE_CODE_USE_VERTEX`, e as URLs de endpoint do provedor como `ANTHROPIC_BEDROCK_BASE_URL`
* Credenciais de autenticação, como `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` e `CLAUDE_CODE_OAUTH_TOKEN`

Todas as outras chaves no bloco `env` em cache, como telemetria e configuração OpenTelemetry, se aplicam na inicialização como antes. Uma vez que a busca seja bem-sucedida, as variáveis retidas se aplicam pelo resto da sessão.

Se sua organização precisa de um proxy para alcançar `api.anthropic.com`, defina-o no ambiente do shell ou em [configurações do usuário](/docs/pt/settings#settings-files) em vez de apenas no bloco `env` gerenciado. O primeiro lançamento não tem cache, portanto essas fontes já eram necessárias para a busca inicial.

O Claude Code aplica atualizações de configurações automaticamente sem reinicialização, exceto para configurações avançadas como configuração OpenTelemetry, que exigem uma reinicialização completa para entrar em vigor.

<h3 id="invalid-entries-in-delivered-settings">
  Entradas inválidas em configurações entregues
</h3>

Os payloads entregues são analisados com tolerância usando as mesmas regras que as outras fontes gerenciadas. Quando um payload contém uma entrada que falha na validação do esquema, o Claude Code remove essa entrada, exibe um erro de validação e aplica todas as configurações válidas restantes. Consulte [Entradas inválidas em configurações gerenciadas](/docs/pt/settings#invalid-entries-in-managed-settings) para o comportamento em nível de campo, incluindo como os campos de aplicação de segurança são tratados. Requer Claude Code v2.1.169 ou posterior.

A entrega gerenciada pelo servidor adiciona esses comportamentos:

* O cache em `~/.claude/remote-settings.json` armazena o payload salvo com entradas inválidas removidas. O payload inválido bruto nunca é persistido.
* Quando nenhum campo no payload pode ser salvo, o Claude Code mantém as últimas configurações em cache aceitas e registra um erro fatal.
* A [caixa de diálogo de aprovação de segurança](#security-approval-dialogs) avalia o payload salvo, portanto uma entrada inválida removida nunca é apresentada para aprovação e nunca é executada.

Para depurar problemas de entrega, execute `claude --debug-file <path>` e procure no log por `Remote settings`. Valide uma alteração de payload com `claude doctor` em uma máquina de teste antes de implantá-la na organização.

<h3 id="enforce-fail-closed-startup">
  Impor inicialização com falha fechada
</h3>

Por padrão, se a busca de configurações remotas falhar na inicialização, a CLI continua sem configurações gerenciadas. Para ambientes onde essa breve janela não aplicada é inaceitável, defina `forceRemoteSettingsRefresh: true` em suas configurações gerenciadas.

Quando essa configuração está ativa, a CLI bloqueia na inicialização até que as configurações remotas sejam buscadas recentemente. Se a busca falhar, a CLI sai em vez de prosseguir sem a política. Essa configuração se auto-perpetua: uma vez entregue do servidor, ela também é armazenada em cache localmente para que as inicializações subsequentes imponham o mesmo comportamento mesmo antes da primeira busca bem-sucedida de uma nova sessão.

Para ativar isso, adicione a chave à sua configuração de configurações gerenciadas:

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Você também pode definir essa chave em um [perfil MDM gerenciado pelo endpoint](/docs/pt/settings#settings-files) ou arquivo `managed-settings.json` do sistema para impor comportamento de falha fechada no primeiro lançamento, antes de qualquer payload do servidor ter sido entregue. A partir da v2.1.191, esse sinalizador é uma exceção à [regra de precedência](#settings-precedence) acima: ele é honrado quando definido em qualquer fonte gerenciada mesmo se um payload gerenciado pelo servidor em cache também estiver presente, portanto um valor entregue por MDM não é ignorado quando configurações gerenciadas pelo servidor existem.

A busca de configurações também envia um cabeçalho `Cache-Control: no-cache` para que proxies HTTP intermediários não sirvam uma resposta obsoleta.

Antes de ativar essa configuração, certifique-se de que suas políticas de rede permitem conectividade a `api.anthropic.com`. Se esse endpoint estiver inacessível, a CLI sai na inicialização e os usuários não podem iniciar o Claude Code.

A partir da v2.1.139, os subcomandos `claude auth` como `claude auth login` estão isentos dessa verificação, para que os usuários possam se autenticar novamente quando credenciais expiradas forem o motivo da falha na busca de configurações.

<h3 id="security-approval-dialogs">
  Caixas de diálogo de aprovação de segurança
</h3>

Certas configurações que podem representar riscos de segurança exigem aprovação explícita do usuário antes de o Claude Code aplicá-las:

* **Configurações de comando shell**: configurações que executam comandos shell
* **Variáveis de ambiente personalizadas**: variáveis não na lista de permissão segura conhecida
* **Configurações de hooks**: qualquer definição de hook
* **Conteúdo CLAUDE.md gerenciado**: um valor `claudeMd` entregue através de configurações gerenciadas

Quando essas configurações estão presentes, os usuários veem uma caixa de diálogo de segurança explicando o que está sendo configurado. Os usuários devem aprovar para prosseguir. Se um usuário rejeitar as configurações, o Claude Code sai.

<Note>
  Uma execução não interativa, como `claude -p` ou uma sessão do Agent SDK, não pode mostrar a caixa de diálogo. Quando as configurações entregues exigiriam aprovação, o Claude Code as aplica apenas para essa execução: ele não as registra como aprovadas ou as escreve no [cache local](#fetch-and-caching-behavior), e a próxima sessão interativa mostra a caixa de diálogo. Até que um usuário aprove em uma sessão interativa, cada execução não interativa busca as configurações novamente na inicialização. Antes da v2.1.207, uma execução não interativa salvava as configurações como aprovadas, portanto as sessões interativas posteriores nunca mostravam a caixa de diálogo para elas.
</Note>

<h2 id="platform-availability">
  Disponibilidade de plataforma
</h2>

As configurações gerenciadas pelo servidor exigem uma conexão direta a `api.anthropic.com`, e a entrega requer que a sessão se autentique com um login OAuth da organização ou uma chave de API configurada diretamente. As chaves retornadas por um script [`apiKeyHelper`](/docs/pt/settings#available-settings) não acionam a busca de configurações.

As configurações gerenciadas pelo servidor não estão disponíveis ao usar provedores de modelo de terceiros:

* Amazon Bedrock
* Google Cloud's Agent Platform
* Microsoft Foundry
* [Claude Platform on AWS](/docs/pt/claude-platform-on-aws)
* Endpoints de API personalizados via `ANTHROPIC_BASE_URL` ou [gateways LLM](/docs/pt/llm-gateway)

Se você exportar uma variável de provedor `CLAUDE_CODE_USE_*` ou um `ANTHROPIC_BASE_URL` não padrão em seu shell, Claude Code ignora a busca de configurações para suas sessões. Você não pode limpar a exportação com um bloco `env` gerenciado pelo servidor, porque o bloco chega através da busca que a exportação impede. Um bloco `env` de [configurações gerenciadas pelo endpoint](/docs/pt/settings#settings-files) também não restaura a busca: Claude Code verifica a elegibilidade antes de aplicar blocos `env` gerenciados, portanto a substituição altera a seleção de provedor da sessão, mas a busca permanece ignorada.

Para restaurar a entrega gerenciada pelo servidor, remova a exportação do seu shell ou defina a variável como `""` no bloco `env` de suas configurações de usuário, que se aplica antes da verificação de elegibilidade. Para impor política sem depender de usuários para alterar seus shells, entregue as configurações através do canal gerenciado pelo endpoint.

Para implantações do Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) auto-hospedado fornece a entrega equivalente de configurações gerenciadas remotamente: clientes assinados no gateway buscam configurações gerenciadas do gateway em vez de `api.anthropic.com`. A semântica de falha difere na inicialização: um cliente de gateway que não consegue alcançar o gateway sai com um erro em vez de fazer fallback para configurações em cache, enquanto a atualização de fundo por hora é fail-open em ambos os canais.

<h2 id="audit-logging">
  Auditoria de logs
</h2>

Os eventos de log de auditoria para alterações de configurações estão disponíveis através da API de conformidade ou exportação de log de auditoria. Entre em contato com sua equipe de conta da Anthropic para obter acesso.

Os eventos de auditoria incluem o tipo de ação executada, a conta e o dispositivo que executaram a ação, e referências aos valores anteriores e novos.

<h2 id="security-considerations">
  Considerações de segurança
</h2>

As configurações gerenciadas pelo servidor fornecem aplicação de política centralizada, mas funcionam como um controle do lado do cliente, não como um limite de segurança. Em dispositivos não gerenciados, um usuário não precisa de acesso de administrador ou sudo para contorná-las.

| Cenário                                                                        | Comportamento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :----------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Usuário edita o arquivo de configurações em cache                              | O arquivo adulterado se aplica na inicialização, mas as configurações corretas são restauradas na próxima busca do servidor. A partir da v2.1.198, as variáveis de ambiente de transporte, roteamento de API e autenticação no bloco `env` são [retidas até que o servidor confirme o payload](#fetch-and-caching-behavior)                                                                                                                                                                                                       |
| Usuário deleta o arquivo de configurações em cache                             | Comportamento de primeiro lançamento ocorre: configurações são buscadas de forma assíncrona com uma breve janela não aplicada                                                                                                                                                                                                                                                                                                                                                                                                     |
| Usuário executa um binário Claude Code modificado                              | Um usuário que pode executar um cliente modificado pode contornar qualquer controle do lado do cliente                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Usuário executa uma versão anterior do Claude Code                             | Versões que antecedem as configurações gerenciadas pelo servidor não as buscam ou aplicam                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| API está indisponível                                                          | As configurações em cache se aplicam se disponíveis, caso contrário, as configurações gerenciadas não são aplicadas até a próxima busca bem-sucedida. A partir da v2.1.198, as variáveis de ambiente de transporte, roteamento de API e autenticação no bloco `env` em cache são [retidas em caso de falha de busca](#fetch-and-caching-behavior); o resto do cache ainda se aplica. Com `forceRemoteSettingsRefresh: true`, a CLI sai em vez de continuar, exceto para [subcomandos `claude auth`](#enforce-fail-closed-startup) |
| Usuário se autentica com uma organização diferente                             | As configurações não são entregues para contas fora da organização gerenciada                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Usuário configura um [provedor de modelo de terceiros](#platform-availability) | As configurações gerenciadas pelo servidor são ignoradas. Isso inclui definir `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_MANTLE`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, `CLAUDE_CODE_USE_ANTHROPIC_AWS`, ou um `ANTHROPIC_BASE_URL` não padrão                                                                                                                                                                                                                                                                    |
| Tráfego de rede é interceptado ou redirecionado                                | Validação TLS desabilitada ou tráfego interceptado pode alterar as configurações que o cliente recebe                                                                                                                                                                                                                                                                                                                                                                                                                             |

Para detectar alterações de configuração em tempo de execução, use [hooks `ConfigChange`](/docs/pt/hooks#configchange) para registrar modificações ou bloquear alterações não autorizadas antes que entrem em vigor.

Para restringir quais organizações seus usuários podem acessar com as credenciais que o cliente fornece, consulte [Enforce network-level access control with Tenant Restrictions](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions) no Claude Help Center. Para garantias de aplicação mais fortes, use [configurações gerenciadas pelo endpoint](/docs/pt/settings#settings-files) em dispositivos inscritos em uma solução MDM.

<h2 id="see-also">
  Veja também
</h2>

Páginas relacionadas para gerenciar a configuração do Claude Code:

* [Settings](/docs/pt/settings): referência de configuração completa incluindo todas as configurações disponíveis
* [Configurações gerenciadas pelo endpoint](/docs/pt/settings#settings-files): configurações gerenciadas implantadas em dispositivos por TI
* [Authentication](/docs/pt/authentication): configure o acesso do usuário ao Claude Code
* [Security](/docs/pt/security): salvaguardas de segurança e melhores práticas
