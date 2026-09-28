> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Controle o acesso ao servidor MCP para sua organização

> Restrinja quais servidores MCP os usuários podem adicionar ou conectar, ou forneça servidores para cada usuário, com arquivos de configuração gerenciados, configurações gerenciadas, listas de permissão e listas de bloqueio.

Por padrão, qualquer pessoa executando Claude Code pode conectar qualquer [servidor MCP](/docs/pt/mcp) que escolher. A Anthropic analisa conectores em relação aos seus [critérios de listagem](https://claude.com/docs/connectors/building/review-criteria) antes de adicioná-los ao [Diretório Anthropic](https://claude.ai/directory), mas não faz auditoria de segurança ou gerencia nenhum servidor MCP. Como administrador, você pode restringir quais servidores são executados em sua organização, desde a implantação de um conjunto fixo aprovado até a desabilitação completa do MCP, e você pode fornecer servidores para cada usuário.

Essas restrições cobrem os servidores que o Claude Code carrega por si só, incluindo os conectores que busca em claude.ai. Os conectores que o aplicativo desktop entrega para suas sessões locais e SSH chegam em processo e são governados pelas configurações da sua organização claude.ai; [Como conectores chegam ao Claude Code](/docs/pt/mcp#how-connectors-reach-claude-code) mostra quais controles se aplicam aos conectores em cada tipo de sessão, incluindo sessões em nuvem.

Esta página cobre como:

* [Escolher um padrão](#choose-a-pattern) que corresponda ao quanto de controle você precisa
* [Implantar um conjunto de servidor fixo com `managed-mcp.json`](#exclusive-control-with-managed-mcp-json), incluindo como [desabilitar MCP completamente](#disable-mcp-entirely)
* [Fornecer servidores através de configurações gerenciadas](#provide-servers-through-managed-settings) enquanto os usuários mantêm os seus próprios
* [Controlar servidores com listas de permissão e listas de bloqueio](#policy-based-control-with-allowlists-and-denylists)
* [Informar aos usuários o que esperar](#how-restrictions-appear-to-users) quando uma restrição bloqueia um servidor
* [Monitorar quais servidores sua organização realmente usa](#monitor-mcp-usage)

<Note>
  A página [Security](/docs/pt/security) cobre o modelo de ameaça MCP e como avaliar um servidor antes de aprová-lo. [Decide what to enforce](/docs/pt/admin-setup#decide-what-to-enforce) cobre restrições MCP junto com os outros controles administrativos.
</Note>

<h2 id="choose-a-pattern">
  Escolha um padrão
</h2>

Claude Code suporta uma variedade de níveis de restrição. Cada padrão usa um ou mais dos mecanismos cobertos abaixo: `managed-mcp.json` para implantar um conjunto fixo, a configuração gerenciada `managedMcpServers` para fornecer servidores junto com os que os usuários adicionam, e `allowedMcpServers`/`deniedMcpServers` para filtrar o que os usuários configuram.

| Padrão                          | O que faz                                                                                                                                                                                                                                                        | Configurar                                                                                                 |
| :------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| **Desabilitar MCP**             | Nenhum servidor é carregado, exceto [servidores em processo que o aplicativo que iniciou a sessão registra](#exclusive-control-with-managed-mcp-json) e qualquer um que você [forneça através de `managedMcpServers`](#provide-servers-through-managed-settings) | `managed-mcp.json` com um mapa de servidor vazio                                                           |
| **Implantação fixa**            | Cada usuário obtém os mesmos servidores e não pode adicionar outros                                                                                                                                                                                              | `managed-mcp.json` com os servidores que você deseja                                                       |
| **Servidores fornecidos**       | Cada usuário obtém os servidores remotos que você lista e mantém os seus próprios                                                                                                                                                                                | `managedMcpServers` nas configurações gerenciadas                                                          |
| **Catálogo aprovado**           | Publique uma lista de servidores aprovados; os usuários adicionam os que desejam, qualquer outra coisa é bloqueada                                                                                                                                               | `allowedMcpServers` + `allowManagedMcpServersOnly: true`                                                   |
| **Apenas servidores de plugin** | Os usuários não podem adicionar servidores através de `~/.claude.json` ou `.mcp.json`; os servidores de plugin ainda são carregados                                                                                                                              | [`strictPluginOnlyCustomization`](/docs/pt/settings-reference#strictpluginonlycustomization) com `mcp` na lista |
| **Lista de permissões suave**   | Aplique uma lista de permissões que os usuários podem ampliar em suas próprias configurações                                                                                                                                                                     | `allowedMcpServers` sem `allowManagedMcpServersOnly`                                                       |
| **Apenas lista de negação**     | Bloqueie servidores conhecidos como ruins, permita tudo o mais                                                                                                                                                                                                   | `deniedMcpServers`                                                                                         |
| **Sem restrições**              | Os usuários adicionam qualquer coisa                                                                                                                                                                                                                             | Não implante nenhuma configuração MCP gerenciada                                                           |

<Note>
  Claude Code não possui um registro de servidor MCP integrado que os usuários possam procurar e instalar. Para o padrão de catálogo aprovado, compartilhe a lista aprovada e seus comandos `claude mcp add` em algum lugar onde seus usuários a encontrem, como um wiki interno, ou distribua os servidores como plugins através de um [marketplace de plugin gerenciado](/docs/pt/plugins/org#restrict-what-users-can-install) para que os usuários possam procurar e instalá-los em `/plugin`.
</Note>

<h2 id="exclusive-control-with-managed-mcp-json">
  Controle exclusivo com managed-mcp.json
</h2>

Quando você implanta um arquivo `managed-mcp.json`, Claude Code carrega apenas estes servidores MCP:

* Os servidores que o arquivo define
* Servidores que você [fornece através de `managedMcpServers`](#provide-servers-through-managed-settings)
* Servidores em processo que o aplicativo que iniciou a sessão registra, como o servidor próprio da extensão VS Code ou os [conectores que o aplicativo desktop fornece](/docs/pt/mcp#how-connectors-reach-claude-code)

Os usuários não podem adicionar, modificar ou usar nenhum outro servidor MCP, incluindo servidores fornecidos por plugins e servidores passados com a [flag CLI `--mcp-config`](/docs/pt/cli-reference#cli-flags). O arquivo também suprime os conectores claude.ai que Claude Code busca por si mesmo, a menos que você [permita-os junto com o conjunto gerenciado](#allow-claude-ai-connectors-alongside-the-managed-set).

<h3 id="deploy-managed-mcp-json">
  Implantar managed-mcp.json
</h3>

`managed-mcp.json` é um arquivo independente, portanto não pode ser entregue através de [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings). Para entregar servidores através de configurações gerenciadas em vez disso, sem controle exclusivo, use [`managedMcpServers`](#provide-servers-through-managed-settings).

Qualquer processo que possa escrever em um caminho do sistema com privilégios de administrador pode implantar o arquivo. Em uma frota, isso geralmente é feito através de ferramentas de gerenciamento de dispositivos, como Jamf ou um perfil de configuração no macOS, Política de Grupo ou Intune no Windows, ou seu gerenciamento de frota de escolha no Linux. Claude Code procura o arquivo em um destes caminhos:

| Plataforma  | Caminho                                                    |
| :---------- | :--------------------------------------------------------- |
| macOS       | `/Library/Application Support/ClaudeCode/managed-mcp.json` |
| Linux e WSL | `/etc/claude-code/managed-mcp.json`                        |
| Windows     | `C:\Program Files\ClaudeCode\managed-mcp.json`             |

O arquivo usa o mesmo formato que um arquivo de projeto [`.mcp.json`](/docs/pt/mcp#project-scope):

```json theme={null}
{
  "mcpServers": {
    "github": {
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp/"
    },
    "sentry": {
      "type": "http",
      "url": "https://mcp.sentry.dev/mcp"
    },
    "company-internal": {
      "type": "stdio",
      "command": "/usr/local/bin/company-mcp-server",
      "args": ["--config", "/etc/company/mcp-config.json"],
      "env": {
        "COMPANY_API_URL": "https://internal.example.com"
      }
    }
  }
}
```

<h3 id="authenticate-with-per-user-credentials">
  Autenticar com credenciais por usuário
</h3>

Qualquer usuário na máquina pode ler este arquivo, portanto não armazene chaves de API ou outras credenciais em blocos `env`. Passe credenciais por usuário com uma destas opções:

* [Expansão `${VAR}`](/docs/pt/mcp#environment-variable-expansion-in-mcp-json) para ler segredos do ambiente de cada usuário.
* [OAuth ou cabeçalhos por usuário](/docs/pt/mcp#authenticate-with-remote-mcp-servers) para que cada usuário se autentique como si mesmo.
* [`headersHelper`](/docs/pt/mcp#use-dynamic-headers-for-custom-authentication) para gerar credenciais no momento da conexão.

<h3 id="servers-passed-with-mcp-config-or-strict-mcp-config">
  Servidores passados com `--mcp-config` ou `--strict-mcp-config`
</h3>

Quando uma sessão recebe servidores através de `--mcp-config` enquanto um `managed-mcp.json` que Claude Code pode ler e analisar está implantado, o que o usuário vê difere entre uma estação de trabalho e uma sessão na nuvem:

* Em uma estação de trabalho, Claude Code sai na inicialização com `You cannot dynamically configure MCP servers when an enterprise MCP config is present`.
* Em [sessões na nuvem](/docs/pt/claude-code-on-the-web) em um host onde o arquivo está implantado, como um [executor auto-hospedado](/docs/pt/self-hosted-environments-configuration#mcp-servers), Claude Code inicia apenas com os servidores gerenciados e ignora os conectores claude.ai e outros servidores que o host na nuvem fornece através de `--mcp-config`. Nada na sessão informa ao usuário quais servidores foram deixados de fora. Claude Code os nomeia em um aviso em seu stderr, que um executor auto-hospedado registra no nível de log `debug`.

O flag `--strict-mcp-config` pede para substituir o conjunto gerenciado. Se um usuário passar enquanto tal arquivo está implantado, Claude Code sai na inicialização em uma estação de trabalho e em uma sessão na nuvem igualmente.

<h3 id="how-allowlists-and-denylists-apply-to-the-managed-set">
  Como listas de permissão e listas de negação se aplicam ao conjunto gerenciado
</h3>

A lista de negação pode filtrar ainda mais os servidores em `managed-mcp.json`:

* `deniedMcpServers` também se aplica aos servidores gerenciados, portanto um servidor gerenciado que corresponda a uma entrada não será carregado.
* A própria `deniedMcpServers` de um usuário é mesclada a partir de suas configurações, portanto os usuários podem bloquear um servidor gerenciado para si mesmos.

`allowedMcpServers` não se aplica aos servidores em `managed-mcp.json`, com uma exceção: Claude Code ainda verifica um servidor cuja definição usa [expansão `${VAR}`](/docs/pt/mcp#environment-variable-expansion-in-mcp-json) contra a lista de permissão, porque a configuração efetiva desse servidor vem do ambiente de cada usuário em vez de apenas do arquivo. Antes da v2.1.259, cada servidor gerenciado tinha que passar pela lista de permissão sempre que uma era definida. Veja [Como um servidor é avaliado](#how-a-server-is-evaluated) para quais campos acionam a verificação `${VAR}` e a ordem completa de verificações.

Se você usou `allowedMcpServers` para impedir que alguns de seus próprios servidores `managed-mcp.json` fossem carregados, esses servidores começam a ser carregados no primeiro lançamento de cada usuário da v2.1.259 ou posterior, a menos que usem expansão `${VAR}`, sem aviso ou notificação: apenas `deniedMcpServers` ainda subtrai desses servidores. Adicione entradas de lista de negação para eles ou implante um `managed-mcp.json` separado por grupo antes que seus usuários façam upgrade.

<h3 id="validate-the-configuration">
  Validar a configuração
</h3>

Para confirmar que o arquivo está em vigor, execute duas verificações em uma máquina gerenciada:

1. `claude mcp list` mostra apenas os servidores em `managed-mcp.json`, além de qualquer um que você forneça através de `managedMcpServers`. Dois outros resultados significam que algo está errado:
   * Se os próprios servidores de um usuário ainda aparecerem, Claude Code não está lendo o arquivo, portanto verifique seu caminho e as permissões nos diretórios pai.
   * Se os servidores do arquivo não aparecerem e a seção `MCP config diagnostics` marca a configuração empresarial como falha ao analisar, Claude Code não consegue ler ou analisar o arquivo. Corrija o erro que essa seção nomeia e peça ao usuário para reiniciar Claude Code.
2. `claude mcp add --transport http test https://example.com/mcp` falha com `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`. A URL não precisa ser um servidor real, já que a verificação de política rejeita o comando antes de qualquer coisa ser contatada.

<h3 id="disable-mcp-entirely">
  Desabilitar MCP completamente
</h3>

Implante um `managed-mcp.json` contendo um mapa de servidor vazio para bloquear cada servidor MCP, exceto [servidores em processo que o aplicativo que iniciou a sessão registra](#exclusive-control-with-managed-mcp-json):

```json theme={null}
{
  "mcpServers": {}
}
```

`claude mcp add` falha com o erro de política empresarial acima. Os servidores que os usuários configuraram anteriormente param de ser carregados na próxima vez que iniciam uma sessão, sem aviso de que a política é o motivo. Os servidores que você fornece através de `managedMcpServers` ainda são carregados sob um mapa vazio, portanto deixe essa chave não definida também para desabilitar MCP completamente.

<h3 id="allow-claude-ai-connectors-alongside-the-managed-set">
  Permitir conectores claude.ai junto com o conjunto gerenciado
</h3>

Por padrão, implantar `managed-mcp.json` suprime os [conectores claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai) que Claude Code busca por si mesmo, incluindo conectores que um administrador configurou para a organização no console de administração claude.ai. Para carregar esses conectores junto com os servidores em `managed-mcp.json`, defina `"allowAllClaudeAiMcps": true` em uma [fonte de configurações gerenciadas](/docs/pt/admin-setup#decide-how-settings-reach-devices).

Com a configuração habilitada, Claude Code carrega os mesmos conectores claude.ai que carregaria se `managed-mcp.json` não estivesse implantado. [Listas de permissão e listas de negação](#policy-based-control-with-allowlists-and-denylists) ainda se aplicam a esses conectores, portanto você pode bloquear específicos com `deniedMcpServers`. A configuração afeta apenas os conectores claude.ai que Claude Code busca por si mesmo; servidores fornecidos por plugins permanecem suprimidos.

Sessões na nuvem e as sessões locais e SSH do aplicativo desktop recebem conectores de outra forma, descrita em [Como conectores chegam a Claude Code](/docs/pt/mcp#how-connectors-reach-claude-code). Um `managed-mcp.json` no host que executa uma sessão na nuvem, como um [host de executor auto-hospedado](/docs/pt/self-hosted-environments-configuration#mcp-servers), suprime os conectores dessa sessão independentemente de você definir `allowAllClaudeAiMcps`. Nenhum `managed-mcp.json` chega aos conectores que o aplicativo desktop fornece para suas sessões locais e SSH.

Claude Code lê `allowAllClaudeAiMcps` apenas de camadas de política controladas por administrador: configurações gerenciadas pelo servidor, uma chave de registro plist ou HKLM implantada por MDM, ou um arquivo `managed-settings.json` do sistema. Colocá-lo em configurações de usuário ou projeto não tem efeito, portanto os usuários não podem reabilitar conectores que o controle exclusivo suprimiu.

<h2 id="provide-servers-through-managed-settings">
  Fornecer servidores através de configurações gerenciadas
</h2>

Para fornecer a cada usuário um conjunto de servidores MCP remotos sem assumir controle exclusivo do MCP, liste-os em `managedMcpServers` em uma [fonte de configurações gerenciadas](/docs/pt/admin-setup#decide-how-settings-reach-devices): configurações gerenciadas pelo servidor, uma [política de gateway de aplicativos Claude](/docs/pt/claude-apps-gateway-config#what-goes-in-cli), um perfil MDM ou política de registro, ou `managed-settings.json`. Os usuários mantêm os servidores que adicionam a si mesmos e recebem os seus além deles. Requer Claude Code v2.1.259 ou posterior. Clientes anteriores ignoram a chave.

O valor é um objeto com chave pelo nome do servidor. Cada entrada tem a mesma forma que um servidor HTTP ou SSE em um arquivo de projeto [`.mcp.json`](/docs/pt/mcp#project-scope), incluindo os membros opcionais `headers` e `oauth` descritos em [Autenticar com servidores MCP remotos](/docs/pt/mcp#authenticate-with-remote-mcp-servers). Este exemplo fornece um servidor de busca no qual cada usuário se conecta com OAuth, e um servidor de registros que envia um cabeçalho que sua organização emite:

```json theme={null}
{
  "managedMcpServers": {
    "search": {
      "type": "http",
      "url": "https://search.example.com/mcp"
    },
    "records": {
      "type": "http",
      "url": "https://records.example.com/mcp",
      "headers": {
        "X-Records-Key": "key-issued-for-all-claude-code-users"
      }
    }
  }
}
```

Qualquer pessoa que possa ler as configurações gerenciadas em uma máquina, incluindo o usuário, pode ler um valor de cabeçalho que você define aqui. Use uma credencial emitida para esse público inteiro, ou deixe `headers` de fora e deixe cada usuário se conectar com OAuth.

<h3 id="what-an-entry-can-contain">
  O que uma entrada pode conter
</h3>

Claude Code carrega uma entrada apenas quando ela passa em todas as verificações abaixo. Ele descarta uma entrada que falha em uma, registra um aviso que você pode ler com `/status`, e ainda carrega as outras entradas:

* `type` é `http` ou `sse`. Como em `.mcp.json`, `streamable-http` é aceito como um alias para `http`.
* `url` é uma URL `https://`. Claude Code recusa uma URL simples `http://`, incluindo uma que aponta para `localhost`.
* A entrada não tem membro `command`, `args`, `env` ou `headersHelper`, portanto um documento de configurações gerenciadas nunca nomeia um programa para executar na máquina de um usuário.
* Nenhum valor contém uma referência `${VAR}`. Claude Code não expande variáveis de ambiente nessas entradas, portanto escreva valores literais.
* O nome do servidor contém apenas letras, números, hífens e sublinhados, e nenhuma chave ou valor contém caracteres de controle ou formatação invisível.

Claude Desktop tem uma configuração gerenciada com o mesmo nome cujo valor é uma matriz de uma forma de entrada diferente, portanto não copie uma na outra. Claude Code não aceita a forma de matriz e registra um aviso em vez de carregá-la.

Um gateway de aplicativos Claude executa as mesmas verificações quando inicializa; veja [Servidores MCP em uma política](/docs/pt/claude-apps-gateway-config#mcp-servers-in-a-policy).

<h3 id="how-provided-servers-load">
  Como os servidores fornecidos são carregados
</h3>

Essas regras decidem o que é carregado quando um servidor fornecido se sobrepõe a outra definição de servidor ou a outra configuração nesta página:

* Um servidor fornecido tem precedência sobre um servidor com o mesmo nome em escopo local, de projeto ou de usuário, e sobre um servidor de plugin ou conector claude.ai que aponta para a mesma URL.
* Se você também implantar `managed-mcp.json`, Claude Code carrega seus servidores e os servidores fornecidos juntos, e a entrada do arquivo tem precedência quando ambos definem um nome.
* Os servidores fornecidos continuam carregando quando [`strictPluginOnlyCustomization`](/docs/pt/settings-reference#strictpluginonlycustomization) bloqueia a superfície `mcp`.
* `deniedMcpServers` se aplica aos servidores fornecidos, incluindo entradas das próprias configurações de um usuário, portanto um usuário pode bloquear um para si mesmo. Os servidores fornecidos não precisam de uma entrada `allowedMcpServers`.

Quando você também não implantou `managed-mcp.json`, os sinalizadores por execução mantêm seu significado:

* Um servidor que um usuário passa com `--mcp-config` sob o mesmo nome substitui o fornecido para essa execução e é verificado contra `allowedMcpServers`.
* `--strict-mcp-config` deixa os servidores fornecidos de fora junto com todos os outros servidores configurados.

Com `managed-mcp.json` implantado, ambos os sinalizadores se comportam como [Controle exclusivo com managed-mcp.json](#exclusive-control-with-managed-mcp-json) descreve.

<h3 id="what-users-can-see-and-change">
  O que os usuários podem ver e alterar
</h3>

Os usuários não podem editar ou remover um servidor fornecido:

* `claude mcp remove` relata que o servidor é fornecido pela organização.
* Quando você também não implantou `managed-mcp.json`, uma entrada que um usuário adiciona sob o mesmo nome é salva mas não usada enquanto a sua está presente.
* Os usuários ainda podem desativar um servidor fornecido para si mesmos em [`/mcp`](/docs/pt/mcp#disable-a-server-without-removing-it), que lista servidores fornecidos em **Managed MCPs**.

`claude mcp get` e `/mcp` mostram a URL de um servidor fornecido apenas como seu host, por exemplo `https://mcp.example.com/…`, e `claude mcp get` mostra seus nomes de cabeçalho sem seus valores.

<h3 id="where-managedmcpservers-applies">
  Onde `managedMcpServers` se aplica
</h3>

Claude Code lê `managedMcpServers` da fonte gerenciada que seleciona em [Como Claude Code combina fontes gerenciadas](/docs/pt/managed-settings#how-claude-code-combines-managed-sources). Quando essa fonte define [`managedSourcesBehavior`](/docs/pt/settings-reference#managedsourcesbehavior) como `"merge"`, Claude Code fornece os servidores de todas as fontes de administrador, e quando duas fontes definem o mesmo nome, a entrada da fonte com classificação mais alta se aplica integralmente. Ele nunca lê a chave do registro HKCU gravável pelo usuário, de [configurações pai que um host de incorporação fornece](/docs/pt/managed-settings#parent-settings-from-embedding-hosts), ou de arquivos de configurações de usuário, projeto ou local, onde descarta a chave com um aviso.

Claude Code não lê a chave na guia Code do aplicativo Claude Desktop em uma implantação de terceiros ou nas sessões Cowork do aplicativo, porque Claude Desktop fornece e bloqueia os servidores MCP dessas sessões. `/status` e `claude doctor` dizem assim quando suas configurações gerenciadas carregam a chave lá.

<h3 id="when-provided-servers-connect">
  Quando os servidores fornecidos se conectam
</h3>

Quando `managedMcpServers` chega através de configurações gerenciadas pelo servidor, seu cronograma segue [Comportamento de busca e cache](/docs/pt/server-managed-settings#fetch-and-caching-behavior):

* Em uma máquina com configurações em cache, Claude Code retém a cópia em cache dessa chave até que o servidor confirme as configurações para a sessão, e aguarda essa confirmação antes de carregar os servidores MCP. Se a confirmação falhar, a sessão continua sem os servidores fornecidos e `/status` diz que eles estão retidos.
* No primeiro lançamento de uma máquina, sem nada em cache ainda, uma sessão interativa que começa antes das configurações chegarem conecta os servidores fornecidos assim que chegam, e uma execução `claude -p` que já começou pode terminar sem eles.

Com [entrada de gateway](/docs/pt/claude-apps-gateway-config#precedence-with-other-managed-sources), Claude Code carrega a política antes da sessão começar, portanto nenhum dos casos atrasa ou pula os servidores fornecidos.

As sessões interativas que já estão em execução aplicam suas edições à chave:

* **Adicionar um servidor**: Claude Code o conecta quando as configurações atualizadas chegam, sem uma reinicialização.
* **Alterar a entrada de um servidor**: essas sessões se reconectam a ele com a nova definição.
* **Remover um servidor**: uma sessão interativa em execução o desconecta assim que lê as configurações alteradas. Uma execução não interativa (`-p`) o mantém até terminar.

<h2 id="policy-based-control-with-allowlists-and-denylists">
  Controle baseado em políticas com listas de permissão e bloqueio
</h2>

Listas de permissão e bloqueio filtram quais servidores configurados podem ser carregados. Elas não são um registro: um servidor ainda precisa ser adicionado por um usuário, um plugin ou sua organização antes que qualquer uma das listas se aplique a ele.

Servidores que sua organização entrega através de `managedMcpServers` carregam sem uma entrada de lista de permissão, e [Como um servidor é avaliado](#how-a-server-is-evaluated) cobre servidores `managed-mcp.json`. A lista de bloqueio se aplica a todos os servidores independentemente de onde vieram, exceto entradas `type: "sdk"` em processo.

Para implantar servidores para usuários, use [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json) ou [`managedMcpServers`](#provide-servers-through-managed-settings). Ambas as listas também filtram servidores passados com a flag CLI [`--mcp-config`](/docs/pt/cli-reference#cli-flags), exceto entradas `type: "sdk"` em processo; `--strict-mcp-config` limita quais arquivos de configuração carregam e não contorna nenhuma das duas listas.

Para tornar a lista de permissão autoritária, defina `allowedMcpServers` e `allowManagedMcpServersOnly: true` juntos em uma [fonte de configurações gerenciadas](/docs/pt/admin-setup#decide-how-settings-reach-devices), como configurações gerenciadas por servidor ou um arquivo `managed-settings.json` implantado.

O bloqueio se aplica de todas as fontes gerenciadas controladas por administrador, então um bloqueio em um arquivo implantado ainda se aplica quando configurações gerenciadas por servidor que não mencionam MCP também estão em uso. Enquanto o bloqueio está ativado, a lista de permissão gerenciada vem da fonte de administrador com classificação mais alta que define uma. Ler o bloqueio e a lista de permissão entre fontes requer Claude Code v2.1.273 ou posterior.

[Restrinja a lista de permissão apenas às configurações gerenciadas](#restrict-the-allowlist-to-managed-settings-only) mostra a configuração.

Sem `allowManagedMcpServersOnly`, listas de permissão de todos os escopos de configurações se mesclam, incluindo o próprio `~/.claude/settings.json` do usuário, então um usuário pode ampliar o que sua lista de permissão permite. Listas de bloqueio se mesclam de todos os escopos independentemente.

<Note>
  `allowManagedMcpServersOnly` é separado de `allowManagedPermissionRulesOnly`, que bloqueia apenas [regras de permissão](/docs/pt/permissions#managed-settings). Definir esse sinalizador não impõe a lista de permissão MCP.
</Note>

<h3 id="match-servers-by-url-command-or-name">
  Corresponder servidores por URL, comando ou nome
</h3>

`allowedMcpServers` e `deniedMcpServers` são listas de entradas. Cada entrada é um objeto com uma única chave que identifica servidores por sua URL, seu comando ou seu nome:

| Chave           | Corresponde                                                                                 | Use para                               |
| :-------------- | :------------------------------------------------------------------------------------------ | :------------------------------------- |
| `serverUrl`     | Uma URL de servidor remoto, exata ou com wildcards `*`                                      | Servidores HTTP e SSE                  |
| `serverCommand` | O comando exato e argumentos que iniciam um servidor stdio                                  | Servidores stdio                       |
| `serverName`    | O rótulo atribuído pelo usuário. Correspondência exata apenas; wildcards não são expandidos | Qualquer tipo, mas veja o Aviso abaixo |

Deixar `allowedMcpServers` indefinido é diferente de defini-lo como um array vazio:

| Configuração        | Indefinido (padrão)            | Array vazio `[]`                                                                           | Preenchido                                                                                                    |
| :------------------ | :----------------------------- | :----------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------ |
| `allowedMcpServers` | Todos os servidores permitidos | Nenhum servidor permitido, exceto [os próprios da organização](#how-a-server-is-evaluated) | Apenas servidores correspondentes permitidos, exceto [os próprios da organização](#how-a-server-is-evaluated) |
| `deniedMcpServers`  | Nenhum servidor bloqueado      | Nenhum servidor bloqueado                                                                  | Servidores correspondentes bloqueados                                                                         |

Veja [Entradas inválidas em configurações gerenciadas](/docs/pt/managed-settings#invalid-entries-in-managed-settings) para o que acontece quando uma entrada falha na validação do esquema.

<Warning>
  Uma entrada `serverName`, em qualquer uma das listas, não é um controle de segurança. O nome é o rótulo que um usuário atribui ao executar `claude mcp add` ou editar um arquivo de configuração, não o servidor subjacente, então um usuário pode chamar qualquer servidor de `github`. Para conectores claude.ai, o nome é o nome de exibição retornado por claude.ai, que pode mudar. Para impor quais servidores realmente executam, adicione entradas `serverCommand` ou `serverUrl`.
</Warning>

A validação de `serverName` difere entre as duas listas:

* Em `deniedMcpServers`, `serverName` aceita qualquer string não vazia sem espaço em branco à esquerda ou à direita, então você pode bloquear [conectores claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai) pelo seu nome de exibição. Por exemplo, `{ "serverName": "claude.ai Slack" }` bloqueia o conector Slack. Prefira uma entrada `serverUrl` quando você precisar que a negação seja robusta a renomeações, ou quando um nome de conector colide e ganha um sufixo ` (N)`.
* Em `allowedMcpServers`, `serverName` é limitado a letras, números, hífens e sublinhados. Use `serverUrl` para colocar na lista de permissão um conector claude.ai que Claude Code busca a si mesmo; para conectores que um host na nuvem entrega para sessões auto-hospedadas, use as entradas listadas em [O tráfego do conector sai de sua rede](/docs/pt/self-hosted-environments-deploy#connector-traffic-leaves-your-network) em vez disso.

Para desativar todos os conectores claude.ai que Claude Code busca a si mesmo, veja [`disableClaudeAiConnectors`](/docs/pt/mcp#disable-claude-ai-connectors).

<h3 id="how-a-server-is-evaluated">
  Como um servidor é avaliado
</h3>

Antes de carregar um servidor, incluindo um de `managed-mcp.json`, Claude Code executa os três verificações abaixo em ordem. Ele as executa novamente quando um usuário reconecta um servidor ou ativa um desativado em `/mcp`. Servidores `type: "sdk"` em processo, que o [aplicativo que iniciou a sessão registra](/docs/pt/mcp#how-connectors-reach-claude-code), pulam todos os três.

1. **Mescle as listas.** Entradas de lista de permissão e bloqueio de todos os escopos de configurações se combinam em uma lista de permissão e uma lista de bloqueio. Quando `allowManagedMcpServersOnly` é `true`, apenas a lista de permissão gerenciada é mantida; a lista de bloqueio sempre se mescla de todos os escopos. Quando mais de uma fonte gerenciada está presente, [Chaves lidas de todas as fontes de administrador](/docs/pt/managed-settings#keys-read-from-every-admin-source) diz qual delas fornece as listas do escopo gerenciado.
2. **Verifique a lista de bloqueio.** Um servidor que corresponde a qualquer entrada de lista de bloqueio, por URL, comando ou nome, é bloqueado. Nada substitui uma correspondência de lista de bloqueio.
3. **Verifique a lista de permissão.** Se `allowedMcpServers` não estiver definido em nenhum lugar, todos os servidores que passaram na lista de bloqueio carregam. Se estiver definido, o que o servidor deve corresponder depende de seu tipo, mostrado na tabela abaixo.

   Os próprios servidores da organização pulam essa verificação: toda entrada `managedMcpServers`, e qualquer entrada `managed-mcp.json` cujos valores não usam expansão `${VAR}`. Servidores integrados também pulam, como Claude no Chrome, o servidor `ide` que Claude Code se conecta em um IDE VS Code ou JetBrains em execução, e servidores que a própria CLI configura.

   Um servidor `managed-mcp.json` que usa expansão `${VAR}` em seu comando, argumentos, `env`, URL ou cabeçalhos ainda é verificado, assim como todos os servidores que um usuário, um plugin, `--mcp-config` ou claude.ai adiciona.

| Tipo de servidor     | Permitido quando corresponde                                                                                                               |
| :------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| Remoto (HTTP ou SSE) | Uma entrada `serverUrl`. Uma correspondência `serverName` conta apenas quando a lista de permissão não contém entradas `serverUrl`         |
| Stdio                | Uma entrada `serverCommand`. Uma correspondência `serverName` conta apenas quando a lista de permissão não contém entradas `serverCommand` |

Três regras de correspondência se aplicam dentro dessas verificações:

* **Comandos correspondem exatamente.** Cada argumento, em ordem. `["npx", "-y", "server"]` não corresponde a `["npx", "server"]` ou `["npx", "-y", "server", "--flag"]`.
* **Valores `serverCommand` e `serverUrl` se expandem antes de corresponder.** Tanto a entrada de política quanto o valor configurado do servidor passam por expansão [`${VAR}` e `${VAR:-default}`](/docs/pt/mcp#environment-variable-expansion-in-mcp-json), então uma entrada escrita como `["${HOME}/bin/server"]` corresponde a uma configuração de servidor que usa a mesma referência ou o caminho expandido. No Windows, referencie uma variável de ambiente que está definida lá, como `${USERPROFILE}` em vez de `${HOME}`. Valores `serverName` correspondem literalmente e nunca se expandem. Os dois lados leem ambientes diferentes; [Como entradas de política se expandem](#how-policy-entries-expand) cobre qual, e como entradas de lista de permissão e bloqueio diferem.
* **URLs suportam wildcards `*`** em qualquer lugar do padrão, incluindo o esquema. A correspondência de nome de host não diferencia maiúsculas de minúsculas e ignora um ponto FQDN à direita, então `https://Mcp.Example.com/*` corresponde a `https://mcp.example.com/api`. Caminhos permanecem sensíveis a maiúsculas e minúsculas.

| Padrão                      | Permite                                                                                      |
| :-------------------------- | :------------------------------------------------------------------------------------------- |
| `https://mcp.example.com/*` | Todos os caminhos em um domínio específico                                                   |
| `https://mcp.example.com`   | Também todos os caminhos nesse domínio. Um padrão sem caminho corresponde a qualquer caminho |
| `https://*.example.com/*`   | Qualquer subdomínio de `example.com`                                                         |
| `http://localhost:*/*`      | Qualquer porta em localhost                                                                  |
| `*://mcp.example.com/*`     | Qualquer esquema para um domínio específico                                                  |

<h4 id="how-policy-entries-expand">
  Como entradas de política se expandem
</h4>

O valor configurado do servidor se expande a partir do ambiente do processo ativo, como o resto de `.mcp.json`. Uma entrada de política se expande a partir de um ambiente fixado em vez disso, então uma variável definida por um projeto ou arquivo de configurações do usuário não pode mudar o que uma entrada de lista de permissão significa. Como uma entrada de política ainda depende do valor do shell de inicialização para qualquer variável que ela referencia, use URLs e comandos literais para entradas que você depende para imposição.

| Lista de entradas   | Se expande de                                                                                                                                                                                                                         | Expansão que mudaria o esquema, host ou escopo de caminho de uma entrada de URL |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| `allowedMcpServers` | O ambiente com o qual Claude Code iniciou, mais valores `env` de configurações gerenciadas                                                                                                                                            | Claude Code ignora a entrada                                                    |
| `deniedMcpServers`  | O mesmo, e uma variável sem valor de inicialização e sem `:-default` preenche a partir de arquivos de configurações fora do repositório, como configurações de usuário ou gerenciadas, que apenas ampliam o que a entrada corresponde | A entrada ainda corresponde                                                     |

Requer Claude Code v2.1.219 ou posterior.

<h3 id="example-configuration">
  Configuração de exemplo
</h3>

A configuração abaixo configura uma lista de permissão rígida com uma lista de bloqueio. As linhas destacadas mudam como o resto da lista é avaliado, e os textos explicativos após o bloco explicam cada um:

```json {3,5,11} theme={null}
{
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://mcp.sentry.dev/*" },
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem", "."] },
    { "serverCommand": ["python", "/usr/local/bin/approved-server.py"] },
    { "serverUrl": "https://mcp.example.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ],
  "deniedMcpServers": [
    { "serverName": "dangerous-server" },
    { "serverCommand": ["npx", "-y", "unapproved-package"] },
    { "serverUrl": "https://*.untrusted.example.com/*" }
  ]
}
```

* **Linha 3**: a primeira entrada `serverUrl`. Uma vez que uma existe, todos os servidores remotos devem corresponder a um padrão de URL, então um usuário não pode obter um servidor remoto não listado dando a ele um nome permitido.
* **Linha 5**: a primeira entrada `serverCommand`. Mesmo efeito para servidores stdio, então todos os servidores locais devem corresponder a um comando listado exatamente.
* **Linha 11**: uma entrada `serverName` na lista de bloqueio. Entradas de lista de bloqueio sempre se aplicam, então qualquer servidor nomeado `dangerous-server` é bloqueado independentemente de sua URL ou comando.

Uma entrada `serverName` nesta lista de permissão nunca corresponderia a nada, já que ambos os tipos de transporte já têm entradas mais rigorosas.

Os acordeões abaixo percorrem como um servidor é avaliado contra outras combinações de lista de permissão e bloqueio.

<Accordion title="Lista de permissão apenas de URL">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://mcp.example.com/*" },
      { "serverUrl": "https://*.internal.example.com/*" }
    ]
  }
  ```

  | Servidor                                                | Resultado                                                    |
  | :------------------------------------------------------ | :----------------------------------------------------------- |
  | Servidor HTTP em `https://mcp.example.com/api`          | Permitido: corresponde ao padrão de URL                      |
  | Servidor HTTP em `https://api.internal.example.com/mcp` | Permitido: corresponde ao subdomínio com wildcard            |
  | Servidor HTTP em `https://external.example.com/mcp`     | Bloqueado: não corresponde a nenhum padrão de URL            |
  | Servidor stdio com qualquer comando                     | Bloqueado: sem entradas de nome ou comando para corresponder |
</Accordion>

<Accordion title="Lista de permissão apenas de comando">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | Servidor                                               | Resultado                                         |
  | :----------------------------------------------------- | :------------------------------------------------ |
  | Servidor stdio com `["npx", "-y", "approved-package"]` | Permitido: corresponde ao comando                 |
  | Servidor stdio com `["node", "server.js"]`             | Bloqueado: não corresponde ao comando             |
  | Servidor HTTP nomeado `my-api`                         | Bloqueado: sem entradas de nome para corresponder |
</Accordion>

<Accordion title="Lista de permissão mista de nome e comando">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | Servidor                                                                    | Resultado                                                                                    |
  | :-------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- |
  | Servidor stdio nomeado `local-tool` com `["npx", "-y", "approved-package"]` | Permitido: corresponde ao comando                                                            |
  | Servidor stdio nomeado `local-tool` com `["node", "server.js"]`             | Bloqueado: entradas de comando existem mas não corresponde                                   |
  | Servidor stdio nomeado `github` com `["node", "server.js"]`                 | Bloqueado: servidores stdio devem corresponder a comandos quando entradas de comando existem |
  | Servidor HTTP nomeado `github`                                              | Permitido: corresponde ao nome                                                               |
  | Servidor HTTP nomeado `other-api`                                           | Bloqueado: nome não corresponde                                                              |
</Accordion>

<Accordion title="Lista de permissão apenas de nome">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverName": "internal-tool" }
    ]
  }
  ```

  | Servidor                                                    | Resultado                            |
  | :---------------------------------------------------------- | :----------------------------------- |
  | Servidor stdio nomeado `github` com qualquer comando        | Permitido: sem restrições de comando |
  | Servidor stdio nomeado `internal-tool` com qualquer comando | Permitido: sem restrições de comando |
  | Servidor HTTP nomeado `github`                              | Permitido: corresponde ao nome       |
  | Qualquer servidor nomeado `other`                           | Bloqueado: nome não corresponde      |
</Accordion>

<Accordion title="Lista de permissão com substituição de lista de bloqueio">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://*.example.com/*" }
    ],
    "deniedMcpServers": [
      { "serverUrl": "https://staging.example.com/*" }
    ]
  }
  ```

  | Servidor                                           | Resultado                                                                                               |
  | :------------------------------------------------- | :------------------------------------------------------------------------------------------------------ |
  | Servidor HTTP em `https://mcp.example.com/api`     | Permitido: corresponde ao padrão de URL da lista de permissão, sem correspondência de lista de bloqueio |
  | Servidor HTTP em `https://staging.example.com/api` | Bloqueado: corresponde a ambos, mas a lista de bloqueio tem precedência                                 |
  | Servidor HTTP em `https://other.com/mcp`           | Bloqueado: não corresponde à lista de permissão                                                         |
</Accordion>

<h3 id="restrict-the-allowlist-to-managed-settings-only">
  Restrinja a lista de permissão apenas às configurações gerenciadas
</h3>

Para tornar a lista de permissão gerenciada a única que se aplica, defina `allowManagedMcpServersOnly` no arquivo de configurações gerenciadas:

```json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ]
}
```

Quando `allowManagedMcpServersOnly` é `true`, listas de permissão de configurações de usuário, projeto e local são ignoradas. A lista de bloqueio ainda se mescla de todos os escopos de configurações, então os usuários sempre podem bloquear servidores para si mesmos.

<h2 id="how-restrictions-appear-to-users">
  Como as restrições aparecem aos usuários
</h2>

Para ver o que os usuários veem na inicialização quando `managed-mcp.json` é implantado e a sessão também tem servidores `--mcp-config`, consulte [Controle exclusivo com managed-mcp.json](#exclusive-control-with-managed-mcp-json). Use esta tabela para reconhecer os outros relatórios e informar aos usuários o que esperar antes de implementar uma alteração:

| Restrição                                                                                                                          | O que o usuário vê                                                                                                           |
| :--------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json` está presente e o usuário executa `claude mcp add`                                                              | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`                   |
| O servidor está em uma lista de bloqueio e o usuário executa `claude mcp add`                                                      | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy`                                          |
| O servidor não está na lista de permissões e o usuário executa `claude mcp add`                                                    | `Cannot add MCP server "<name>": not allowed by enterprise policy`                                                           |
| O usuário executa `claude mcp remove` em um servidor de `managedMcpServers`                                                        | `MCP server "<name>" is provided by your organization (managed settings) and cannot be removed locally.`                     |
| Um servidor configurado anteriormente agora está bloqueado pela política                                                           | O servidor desaparece de `/mcp` e `claude mcp list`                                                                          |
| Um servidor fica bloqueado enquanto uma sessão está em execução e o usuário seleciona **Reconnect** ou o ativa novamente em `/mcp` | [`MCP server <name> is blocked by enterprise managed policy`](/docs/pt/errors#mcp-server-is-blocked-by-enterprise-managed-policy) |

Quando um servidor desaparece silenciosamente, o usuário não recebe nenhum sinal de que a política é o motivo, portanto, informe aos usuários afetados quais servidores estão bloqueados quando você implementar uma nova restrição.

<h2 id="monitor-mcp-usage">
  Monitorar o uso de MCP
</h2>

Quando [exportação OpenTelemetry](/docs/pt/monitoring-usage) está configurada, Claude Code pode registrar quais servidores MCP e ferramentas os usuários invocam. Defina `OTEL_LOG_TOOL_DETAILS=1` para incluir nomes de servidor MCP e ferramentas em eventos de ferramentas, depois agregue-os em seu coletor para ver quais servidores seus usuários realmente conectam. Consulte [Monitoramento](/docs/pt/monitoring-usage) para configurar o exportador e para o esquema de evento completo.

<h2 id="configuration-summary">
  Resumo de configuração
</h2>

Cada arquivo e configuração que esta página aborda, o que controla e como entregá-lo:

| Superfície                   | O que controla                                                                                                                                                                                                                                                   | Onde fica                                                                                                                                                                                                                                      | Como entregar                                                                                                                                                                                                       |
| :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `managed-mcp.json`           | Conjunto de servidor fixo, controle exclusivo                                                                                                                                                                                                                    | Caminho do sistema: `/Library/Application Support/ClaudeCode/`, `/etc/claude-code/`, ou `C:\Program Files\ClaudeCode\`                                                                                                                         | MDM, GPO, gerenciamento de frota ou qualquer processo com privilégios de administrador. Não pode ser definido através de configurações gerenciadas pelo servidor                                                    |
| `managedMcpServers`          | Servidores remotos fornecidos a cada usuário junto com os seus próprios                                                                                                                                                                                          | Apenas fontes de configurações gerenciadas; a configuração não tem efeito em outro lugar                                                                                                                                                       | Uma [fonte de configurações gerenciadas](/docs/pt/admin-setup#decide-how-settings-reach-devices): configurações gerenciadas pelo servidor, uma política de gateway, `managed-settings.json`, perfil MDM ou registro HKLM |
| `allowedMcpServers`          | Lista de permissão de servidores permitidos                                                                                                                                                                                                                      | Qualquer [escopo de configurações](/docs/pt/settings#where-settings-live); [Como um servidor é avaliado](#how-a-server-is-evaluated) diz como as listas de vários escopos e fontes gerenciadas se combinam                                          | Para aplicação, uma [fonte de configurações gerenciadas](/docs/pt/admin-setup#decide-how-settings-reach-devices): configurações gerenciadas pelo servidor, `managed-settings.json`, perfil MDM ou registro               |
| `deniedMcpServers`           | Lista de bloqueio de servidores bloqueados                                                                                                                                                                                                                       | Qualquer escopo de configurações; [Como um servidor é avaliado](#how-a-server-is-evaluated) diz como as listas de vários escopos e fontes gerenciadas se combinam                                                                              | Mesmo que `allowedMcpServers`                                                                                                                                                                                       |
| `allowManagedMcpServersOnly` | Bloqueia a lista de permissão apenas para fontes gerenciadas                                                                                                                                                                                                     | Apenas fontes de configurações gerenciadas; [Chaves lidas de cada fonte de administrador](/docs/pt/managed-settings#keys-read-from-every-admin-source) diz quais fontes gerenciadas podem ativá-la. A configuração não tem efeito em outros escopos | Mesmo que `allowedMcpServers`                                                                                                                                                                                       |
| `allowAllClaudeAiMcps`       | Carrega os conectores claude.ai que Claude Code busca por si mesmo junto com `managed-mcp.json`. [Um `managed-mcp.json` no host que executa uma sessão na nuvem ainda suprime os conectores dessa sessão](#allow-claude-ai-connectors-alongside-the-managed-set) | Apenas fontes de configurações gerenciadas; a configuração não tem efeito em outro lugar                                                                                                                                                       | Mesmo que `allowedMcpServers`                                                                                                                                                                                       |

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Decidir o que aplicar](/docs/pt/admin-setup#decide-what-to-enforce): restrições de MCP junto com regras de permissão, sandboxing e os outros controles de administrador
* [Conectar Claude Code a ferramentas via MCP](/docs/pt/mcp): a referência completa de MCP, incluindo transportes, escopos e autenticação
* [Configurações](/docs/pt/settings): a hierarquia de configurações e como as configurações gerenciadas têm precedência
* [Configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings): entregar `allowedMcpServers` e `deniedMcpServers` do console de administrador do Claude.ai
* [Segurança](/docs/pt/security): o modelo de ameaça que esses controles defendem
* [Guia do Administrador Empresarial Claude](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, gerenciamento de assentos e playbook de implementação
