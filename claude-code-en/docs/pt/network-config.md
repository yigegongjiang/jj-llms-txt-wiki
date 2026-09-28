> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configuração de rede empresarial

> Configure Claude Code para ambientes empresariais com servidores proxy, Autoridades de Certificação (CA) personalizadas e autenticação mútua de Transport Layer Security (mTLS).

Claude Code suporta várias configurações de rede e segurança empresariais através de variáveis de ambiente. Isso inclui rotear o tráfego através de servidores proxy corporativos, confiar em Autoridades de Certificação (CA) personalizadas e autenticar com certificados de Transport Layer Security (mTLS) mútuo para segurança aprimorada.

Defina essas variáveis de ambiente antes de iniciar Claude Code. As variáveis exportadas no seu shell são lidas uma vez na inicialização, portanto uma sessão em execução não detecta alterações posteriores no seu ambiente de shell.

<Note>
  Todas as variáveis de ambiente mostradas nesta página também podem ser configuradas em [`settings.json`](/docs/pt/settings).
</Note>

<h2 id="proxy-configuration">
  Configuração de proxy
</h2>

<h3 id="environment-variables">
  Variáveis de ambiente
</h3>

Claude Code respeita variáveis de ambiente de proxy padrão. Em sessões do Claude Desktop onde o aplicativo gerencia a conexão do provedor, Claude Code as lê apenas de configurações gerenciadas e `~/.claude/settings.json`; consulte [autenticação mTLS](#mtls-authentication) para as regras de escopo.

```bash theme={null}
# Proxy HTTPS (recomendado)
export HTTPS_PROXY=https://proxy.example.com:8080

# Proxy HTTP (se HTTPS não estiver disponível)
export HTTP_PROXY=http://proxy.example.com:8080

# Ignorar proxy para solicitações específicas - formato separado por espaço
export NO_PROXY="localhost 192.168.1.1 example.com .example.com"
# Ignorar proxy para solicitações específicas - formato separado por vírgula
export NO_PROXY="localhost,192.168.1.1,example.com,.example.com"
# Ignorar proxy para todas as solicitações
export NO_PROXY="*"
```

Variantes em minúsculas também funcionam, e Claude Code usa a primeira que está definida na ordem `https_proxy`, `HTTPS_PROXY`, `http_proxy`, `HTTP_PROXY`.

Claude Code nunca envia suas conexões WebSocket para `localhost`, `::1` ou `127.0.0.0/8` através do proxy, portanto você não precisa de uma entrada de loopback em `NO_PROXY` para elas.

<Note>
  Claude Code não suporta proxies SOCKS.
</Note>

<h3 id="basic-authentication">
  Autenticação básica
</h3>

Se seu proxy exigir autenticação básica, inclua credenciais na URL do proxy:

```bash theme={null}
export HTTPS_PROXY=http://username:password@proxy.example.com:8080
```

<Warning>
  Evite codificar senhas em scripts. Use variáveis de ambiente ou armazenamento seguro de credenciais.
</Warning>

<Tip>
  Para proxies que exigem autenticação avançada (NTLM, Kerberos, etc.), considere usar um serviço LLM Gateway que suporte seu método de autenticação.
</Tip>

<h2 id="ca-certificate-store">
  Armazenamento de certificados CA
</h2>

Por padrão, Claude Code confia tanto em seus certificados CA Mozilla agrupados quanto no armazenamento de certificados do seu sistema operacional. Ler o armazenamento do SO requer um runtime com `tls.getCACertificates`: o instalador nativo sempre possui, e instalações npm precisam do Node 22.15 ou posterior. Em versões mais antigas do Node, apenas o conjunto agrupado e `NODE_EXTRA_CA_CERTS` se aplicam. Proxies de inspeção TLS empresariais funcionam sem configuração adicional quando seu certificado raiz é instalado no armazenamento de confiança do SO e o runtime pode lê-lo.

`CLAUDE_CODE_CERT_STORE` aceita uma lista separada por vírgulas de fontes. Os valores reconhecidos são `bundled` para o conjunto de CA Mozilla enviado com Claude Code e `system` para o armazenamento de confiança do sistema operacional. O padrão é `bundled,system`.

Para confiar apenas no conjunto de CA Mozilla agrupado:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=bundled
```

Para confiar apenas no armazenamento de certificados do SO:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=system
```

<Note>
  `CLAUDE_CODE_CERT_STORE` não possui uma chave de esquema dedicada em `settings.json`. Defina-a através do bloco `env` em `~/.claude/settings.json` ou diretamente no ambiente do processo.
</Note>

<h2 id="custom-ca-certificates">
  Certificados CA personalizados
</h2>

Se seu ambiente empresarial usa uma CA personalizada, configure Claude Code para confiar nela diretamente:

```bash theme={null}
export NODE_EXTRA_CA_CERTS=/path/to/ca-cert.pem
```

<h2 id="mtls-authentication">
  Autenticação mTLS
</h2>

Para ambientes corporativos que exigem autenticação por certificado de cliente:

```bash theme={null}
# Certificado de cliente para autenticação
export CLAUDE_CODE_CLIENT_CERT=/path/to/client-cert.pem

# Chave privada do cliente
export CLAUDE_CODE_CLIENT_KEY=/path/to/client-key.pem

# Opcional: Frase de acesso para chave privada criptografada
export CLAUDE_CODE_CLIENT_KEY_PASSPHRASE="your-passphrase"
```

Claude Code lê os arquivos de certificado e chave na inicialização e os relê cada vez que aplica configurações, como quando sua organização altera o bloco `env` em [configurações gerenciadas](/docs/pt/server-managed-settings) no meio da sessão.

Para rotacionar o certificado e a chave, substitua os arquivos nos mesmos caminhos. Claude Code detecta a substituição em uma sessão em execução sem necessidade de reinicialização. Quando uma solicitação de API falha com um erro no nível de conexão, como uma redefinição de conexão ou um erro de handshake TLS, ele relê ambos os arquivos e tenta novamente a solicitação com o novo par. Antes da v2.1.232, Claude Code não relinha em erros de conexão, portanto mantinha o par que já havia carregado até aplicar configurações novamente ou você reiniciar.

Claude Code relê os arquivos em resposta a solicitações com falha, não observando-os para detectar alterações:

* **Tempo**: Claude Code não faz nada no momento em que você substitui os arquivos. Ele apresenta o novo par na tentativa após uma falha qualificada, ou na próxima solicitação após aplicar configurações, o que vier primeiro.
* **Rejeições de gateway**: Claude Code relê quando seu gateway redefine a conexão ou rejeita o handshake TLS depois de parar de aceitar o par antigo. Ele não relê quando o gateway conclui o handshake e responde com um erro HTTP. Nesse caso, Claude Code carrega o novo par quando aplica configurações novamente ou quando você o reinicia.
* **Rotações parcialmente gravadas**: quando Claude Code relê enquanto sua rotação está no meio da gravação, como ler um certificado e chave que não correspondem um ao outro, ele mantém o par anterior e relê na próxima falha.
* **Exportadores de telemetria OTLP**: Claude Code mantém o certificado que os [exportadores](/docs/pt/monitoring-usage#mtls-authentication) carregaram no primeiro uso, portanto reinicie Claude Code para que um certificado rotacionado alcance seu coletor de telemetria.
* **Desativar o recarregamento**: defina [`CLAUDE_CODE_DISABLE_MTLS_RELOAD_ON_STALE_CONNECTION=1`](/docs/pt/env-vars#variables) para desativar a releitura de erro de conexão. Claude Code então detecta arquivos rotacionados apenas quando aplica configurações novamente ou na próxima inicialização.

Para confirmar que Claude Code detectou uma rotação, [inicie a sessão com registro de depuração](#verify-your-configuration) e procure por `Stale connection — reloaded rotated mTLS client material` no log. Claude Code não registra essa linha quando detecta a rotação ao aplicar configurações, portanto uma linha ausente sozinha não significa que a rotação falhou.

Substitua os arquivos antes do par atual expirar para que Claude Code não carregue um par já expirado na próxima inicialização.

Em [sessões na nuvem](/docs/pt/claude-code-on-the-web), o ambiente de hospedagem gerencia a conexão com a API, portanto Claude Code ignora as seguintes variáveis quando vêm de um bloco `env` do arquivo de configurações:

* `CLAUDE_CODE_CLIENT_CERT`
* `CLAUDE_CODE_CLIENT_KEY`
* `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`
* `NODE_EXTRA_CA_CERTS`
* `NODE_TLS_REJECT_UNAUTHORIZED`
* `CLAUDE_CODE_OAUTH_SCOPES`

Claude Code anota cada chave ignorada no log de depuração da sessão.

Em sessões do [Claude Desktop](/docs/pt/desktop) onde o aplicativo gerencia a conexão do provedor, como a aba Code em um [provedor de terceiros](/docs/pt/third-party-integrations) e sessões Cowork, Claude Code lê essas variáveis e as variáveis de proxy `HTTP_PROXY`, `HTTPS_PROXY` e `NO_PROXY` apenas de [configurações gerenciadas](/docs/pt/managed-settings) e `~/.claude/settings.json`: ele as ignora nos arquivos de configurações próprios de um repositório, portanto um repositório verificado não pode redirecionar o caminho TLS ou proxy de uma sessão cujas credenciais vêm do aplicativo. Em uma sessão local, SSH ou WSL Code tab conectada através de claude.ai, o aplicativo não gerencia a conexão, e Claude Code lê essas variáveis de cada escopo de configurações, como qualquer sessão de terminal; [sessões na nuvem](/docs/pt/claude-code-on-the-web) seguem as regras de sessão na nuvem acima onde quer que você as inicie. Antes da v2.1.217, Claude Code ignorava essas variáveis em todos os arquivos de configurações quando o aplicativo gerenciava a conexão.

<h2 id="verify-your-configuration">
  Verificar sua configuração
</h2>

Você geralmente descobre um endereço de proxy incorreto ou um caminho de certificado inválido a partir de um [erro de conexão ou certificado](/docs/pt/errors#network-and-connection-errors) em uma solicitação posterior, já que Claude Code não valida a maioria dessas configurações quando as lê. A única configuração que verifica na inicialização é a URL do proxy: quando não consegue analisar o valor, como um que está faltando o esquema `http://`, Claude Code interrompe o lançamento com um erro nomeando a variável a ser corrigida.

Para confirmar que sua configuração foi carregada antes de enviar uma solicitação, inicie Claude Code com registro de depuração:

```bash theme={null}
claude --debug
```

A saída de depuração vai para `~/.claude/debug/<session-id>.txt` em vez do terminal, ou para um caminho que você define com `--debug-file <path>`. No log, procure pelas linhas que confirmam que cada arquivo foi carregado:

```text theme={null}
CA certs: Appended extra certificates from NODE_EXTRA_CA_CERTS (/etc/ssl/certs/corp-ca.pem)
mTLS: Loaded client certificate from CLAUDE_CODE_CLIENT_CERT
mTLS: Loaded client key from CLAUDE_CODE_CLIENT_KEY
```

Se Claude Code não conseguir ler um desses arquivos, o log mostra uma linha `Failed to read` ou `Failed to load` com o motivo em vez disso.

Você também pode executar `/status` em uma sessão interativa e verificar estas linhas:

* **Proxy**: mostra a URL do proxy ativo e marca um valor que não consegue analisar como inválido e ignorado.
* **mTLS client cert** e **mTLS client key**: aparecem apenas quando os arquivos foram carregados, portanto uma linha ausente significa que o carregamento falhou e o log de depuração tem o motivo.
* **Additional CA cert(s)**: mostra o caminho `NODE_EXTRA_CA_CERTS` sem verificar se o arquivo foi carregado, portanto confirme este no log de depuração.

<h2 id="apply-network-settings-to-background-agents">
  Aplicar configurações de rede a agentes em segundo plano
</h2>

[Agentes em segundo plano](/docs/pt/agent-view) não são executados dentro do terminal que os despacharam. Um processo supervisor por usuário inicia sob demanda, sobrevive ao seu shell e hospeda cada sessão `claude agents`, `--bg` e `/background`. Veja [Como as sessões em segundo plano são hospedadas](/docs/pt/agent-view#how-background-sessions-are-hosted). Isso muda como a configuração nesta página chega a essas sessões.

<h3 id="set-network-variables-in-settings-not-the-shell">
  Defina variáveis de rede em configurações, não no shell
</h3>

O supervisor é um processo compartilhado por cada terminal. Ele herda o ambiente de qualquer shell que o inicie primeiro, e um supervisor instalado pelo SO não recebe nenhum ambiente de shell. Se você exportar um proxy, caminho de CA ou variável mTLS apenas no seu shell, ele chega aos agentes em segundo plano quando esse shell aconteceu de iniciar a frio o supervisor, e silenciosamente não chega quando um shell diferente fez isso.

Coloque as mesmas variáveis no bloco `env` de `~/.claude/settings.json` ou [configurações gerenciadas](/docs/pt/settings). Cada variável nesta página pode ser definida lá, e as configurações são a única configuração que chega a cada sessão em segundo plano em cada máquina.

<h3 id="configure-a-corporate-launcher-as-a-setting">
  Configure um inicializador corporativo como uma configuração
</h3>

Algumas organizações exigem que cada processo Claude Code inicie através de um inicializador corporativo que aplica sandboxing, controles de rede ou injeção de credenciais. O supervisor e seus workers iniciam Claude Code de um caminho fixo em vez de procurar `claude` em `PATH`, então cada agente em segundo plano ignora um wrapper que você coloca anteriormente em `PATH`.

Defina a configuração [`processWrapper`](/docs/pt/settings-reference#processwrapper) para prefixar o supervisor, seus workers e os outros processos em segundo plano listados em [O que o inicializador cobre](/docs/pt/corporate-launcher#what-the-launcher-covers) com seu inicializador. A variável de ambiente equivalente [`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/pt/env-vars) tem precedência quando ambas são definidas, e está sujeita à mesma regra: entregue-a através de configurações gerenciadas ou `~/.claude/settings.json`, não uma exportação de shell. [Execute Claude Code atrás de um inicializador corporativo](/docs/pt/corporate-launcher) cobre o contrato que o inicializador deve satisfazer, o que faz e não faz, e como implementá-lo.

<Note>
  Um supervisor já em execução mantém a configuração de inicialização com a qual foi iniciado. Após implantar a configuração do inicializador, execute [`claude daemon stop --any`](/docs/pt/agent-view#the-supervisor-process) para que o próximo `claude agents` ou `--bg` inicie um supervisor que a honre. Um serviço instalado leva `claude daemon stop` sem `--any`.
</Note>

<h2 id="streaming-idle-watchdogs">
  Watchdogs de inatividade de streaming
</h2>

Claude Code executa quatro temporizadores independentes que abortam uma resposta de modelo de streaming quando fica silenciosa, para que uma conexão morta falhe e tente novamente em vez de ficar pendurada. O prazo de primeiro byte cobre a espera pelos cabeçalhos de resposta, antes de qualquer parte da resposta ter chegado. Cada um dos outros três monitora uma resposta ativa para um sinal diferente.

| Timer                                | Aborta quando                                                                                                                                                                                                                      | Executa em                                                                                                                                                                                                                                                                                                                                                                     | Tempo limite padrão                                                                                                 |
| :----------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------ |
| Prazo de primeiro byte               | Nenhum cabeçalho de resposta chega após Claude Code enviar a solicitação                                                                                                                                                           | API Anthropic direta e [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), incluindo através de um proxy HTTPS, mas não quando `ANTHROPIC_BASE_URL` ou `ANTHROPIC_AWS_BASE_URL` as roteia através de um [gateway](/docs/pt/gateways). Opt-in no Amazon Bedrock com `CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK=1`; não executa no Agent Platform do Google Cloud ou no Microsoft Foundry | 180 segundos na API Anthropic direta, 300 segundos em outro lugar, mais um segundo por 32KB de corpo da solicitação |
| Watchdog de nível de evento          | Nenhum evento de resposta é analisado. Em conexões onde o watchdog de nível de byte executa, bytes chegando, incluindo pings de keep-alive, também redefinem este watchdog, por até cerca de cinco minutos sem um evento analisado | Cada provedor                                                                                                                                                                                                                                                                                                                                                                  | 300 segundos                                                                                                        |
| Watchdog de nível de byte            | Nenhum byte chega no fio, incluindo pings de keep-alive SSE                                                                                                                                                                        | API Anthropic direta, [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), e [gateway](/docs/pt/gateways) conexões, incluindo um `ANTHROPIC_BASE_URL` customizado. Opt-in no Amazon Bedrock `vnd.amazon.eventstream` respostas com `CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK=1`; não executa no Agent Platform do Google Cloud ou no Microsoft Foundry                                  | 180 segundos na API Anthropic direta, 300 segundos em outro lugar                                                   |
| Tempo limite de inatividade do corpo | Nenhum byte chega por 5 minutos                                                                                                                                                                                                    | Provedores diferentes da API Anthropic direta e Claude Platform on AWS, a menos que [`API_FORCE_IDLE_TIMEOUT`](/docs/pt/env-vars) mude isso                                                                                                                                                                                                                                         | 5 minutos                                                                                                           |

Configure os temporizadores com estas variáveis, cada uma detalhada na [referência de variáveis de ambiente](/docs/pt/env-vars):

* `CLAUDE_ENABLE_STREAM_WATCHDOG` e `CLAUDE_ENABLE_BYTE_WATCHDOG` forçam o watchdog correspondente ligado com `1` ou desligado com `0`, dentro das conexões que a tabela lista; nenhuma variável estende um watchdog para um tipo de conexão que não cobre. `CLAUDE_ENABLE_BYTE_WATCHDOG` definido como `0` também desativa o prazo de primeiro byte.
* `CLAUDE_STREAM_IDLE_TIMEOUT_MS` define o tempo limite de ambos os watchdogs. Claude Code aumenta valores abaixo de 5 minutos para 5 minutos, e limita o valor a 30 minutos para o watchdog de nível de byte.
* `CLAUDE_BYTE_STREAM_IDLE_TIMEOUT_MS` define o tempo limite do watchdog de nível de byte sem alterar o do watchdog de nível de evento, limitado entre 10 segundos e 30 minutos, e tem precedência sobre `CLAUDE_STREAM_IDLE_TIMEOUT_MS` para esse watchdog.
* `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS` define o prazo de primeiro byte diretamente. Deixe-o não definido e Claude Code usa o tempo limite do watchdog de nível de byte, então `CLAUDE_STREAM_IDLE_TIMEOUT_MS` e `CLAUDE_BYTE_STREAM_IDLE_TIMEOUT_MS` também alteram o prazo. Para os limites, a permissão de upload, o limite `API_TIMEOUT_MS`, e quanto tempo a tentativa aguarda após um aborto sem resposta, veja [Nenhuma resposta da API](/docs/pt/errors#no-response-from-api).
* `API_FORCE_IDLE_TIMEOUT` definido como `0` desativa o tempo limite de inatividade do corpo, e definido como `1` o ativa para cada provedor. Os watchdogs executam independentemente dele, então para permitir que um stream pause mais tempo do que seus limites, também aumente ou desative-os.

Quando um watchdog aborta um stream travado, Claude Code trata o aborto como uma falha no meio do stream, e o que você vê depende de quão longe a resposta tinha chegado. Claude Code tenta novamente a solicitação ou encerra a rodada com um erro, mantém a saída concluída e mostra um [aviso de resposta incompleta](/docs/pt/errors#the-response-above-may-be-incomplete), ou encerra a rodada normalmente. [Tentativas automáticas](/docs/pt/errors#automatic-retries) diz onde cada resultado se aplica.

Em uma [sessão não interativa](/docs/pt/headless), e para a resposta de um subagente em qualquer sessão, Claude Code pode primeiro solicitar ao Claude que continue a resposta cortada; [a entrada desse aviso](/docs/pt/errors#the-response-above-may-be-incomplete) diz quando faz isso e quando você ainda vê o aviso.

Quando o prazo de primeiro byte dispara, nenhuma resposta começou, então não há saída parcial para manter. Para como Claude Code reenvia a solicitação e quando a rodada termina em vez disso, veja [Nenhuma resposta da API](/docs/pt/errors#no-response-from-api).

<h2 id="network-access-requirements">
  Requisitos de acesso à rede
</h2>

Claude Code requer acesso aos seguintes URLs. Coloque-os na lista de permissões em sua configuração de proxy e regras de firewall, especialmente em ambientes de rede containerizados ou restritos. A verificação de conectividade de configuração de primeira execução aponta para aqui quando não consegue alcançar `api.anthropic.com` ou `platform.claude.com`; consulte [Não é possível conectar aos serviços Anthropic](/docs/pt/errors#unable-to-connect-to-anthropic-services) para as mensagens da verificação e etapas de recuperação.

| URL                                  | Necessário para                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`                  | Solicitações da API Claude, incluindo a verificação de [segurança de domínio](/docs/pt/data-usage#webfetch-domain-safety-check) do WebFetch, buscas de sinalizadores de recursos e registro de eventos de telemetria                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `claude.ai`                          | Autenticação de conta claude.ai                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `claude.com`                         | A entrada de conta claude.ai abre uma página `claude.com` no navegador, que redireciona para `claude.ai`; as buscas de documentação WebFetch pré-aprovadas também alcançam este host a partir da CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `platform.claude.com`                | Autenticação de conta do Anthropic Console. A troca, atualização e revogação de tokens OAuth também vão para este host para contas claude.ai, portanto, tanto as entradas do Console quanto do claude.ai o exigem                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `mcp-proxy.anthropic.com`            | [Conectores MCP do claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai), incluindo conectores que um administrador da organização configura. O tráfego do conector é roteado através deste proxy; os conectores são ativados por padrão para usuários autenticados no claude.ai. Para impedir que Claude Code os busque, defina [`ENABLE_CLAUDEAI_MCP_SERVERS=false`](/docs/pt/env-vars) ou a configuração [`disableClaudeAiConnectors`](/docs/pt/settings-reference#disableclaudeaiconnectors)                                                                                                                                                                     |
| `downloads.claude.ai`                | Downloads de executáveis de plugins; instalador nativo, atualizador automático nativo e verificações de versão de atualização                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `storage.googleapis.com`             | Contagens de instalação de plugins e metadados mostrados em `/plugin`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `storage.googleapis.com`             | Instalador nativo e atualizador automático nativo em versões anteriores a 2.1.116                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `registry.npmjs.org`                 | Instalações de plugins (buscando pacotes de plugins de origem npm e instalando dependências de pacotes Node.js de plugins), servidores MCP iniciados com `npx` e o registro de pacotes para instalações npm e bun do próprio Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                |
| `bridge.claudeusercontent.com`       | Ponte WebSocket da [extensão Claude no Chrome](/docs/pt/chrome)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `*.frame.claudeusercontent.com`      | Leituras de conteúdo de [Artifact](/docs/pt/artifacts). A CLI busca os arquivos de um artifact deste host quando Claude abre um, e apenas quando a ferramenta Artifact está [disponível](/docs/pt/artifacts#availability) para sua conta. Para desativar a ferramenta e remover este requisito, defina [`"enableArtifact": false`](/docs/pt/settings-reference#enableartifact) ou [`CLAUDE_CODE_DISABLE_ARTIFACT=1`](/docs/pt/env-vars); Claude Code também honra a configuração [`disableArtifact`](/docs/pt/settings-reference#disableartifact) descontinuada. Consulte [Desabilitar artifacts](/docs/pt/artifacts#disable-artifacts) para saber como essas configurações interagem |
| `github.com`                         | Clonagem de [marketplaces de plugins](/docs/pt/plugins/overview) e plugins hospedados no GitHub, incluindo o marketplace oficial da Anthropic, via HTTPS ou SSH. Para clonar fontes `owner/repo` do GitHub apenas via HTTPS, defina [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/pt/env-vars)                                                                                                                                                                                                                                                                                                                                                                      |
| `raw.githubusercontent.com`          | Feed de changelog para [`/release-notes`](/docs/pt/commands). Em sessões interativas, Claude Code também o busca em segundo plano na inicialização quando seu changelog em cache ainda não cobre a versão em execução, como na primeira inicialização após uma atualização; sessões não interativas e em nuvem nunca o buscam                                                                                                                                                                                                                                                                                                                                |
| `*-review.googlesource.com`          | Pesquisa de alteração Gerrit em checkouts `googlesource.com`. Quando uma sessão de guia Claude Desktop Code inicia ou retoma em um checkout [confiável](/docs/pt/permissions#project-allow-rules-and-workspace-trust) cujo `origin` é um host `googlesource.com`, Claude Code pergunta anonimamente ao servidor `-review` desse host pela alteração aberta correspondente ao `Change-Id` do HEAD, uma vez por inicialização ou retomada. Outros tipos de sessão pulam a pesquisa, e nenhum outro host Gerrit é contatado. Opcional: desabilite com [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/pt/env-vars)                                                |
| `http-intake.logs.us5.datadoghq.com` | Eventos de telemetria operacional, enviados apenas quando a CLI usa a API Anthropic diretamente, nunca para Amazon Bedrock, Agent Platform do Google Cloud ou Microsoft Foundry. Opcional: desabilite com [`DISABLE_TELEMETRY`](/docs/pt/data-usage#telemetry-services) ou `DO_NOT_TRACK`                                                                                                                                                                                                                                                                                                                                                                    |
| `browser-intake-us5-datadoghq.com`   | Relatórios de erros operacionais, enviados quando a CLI usa a API Anthropic diretamente e um portão de lançamento do lado do servidor os habilita. Opcional: desabilite com `DISABLE_ERROR_REPORTING` ou `DISABLE_TELEMETRY`; consulte [Serviços de telemetria](/docs/pt/data-usage#telemetry-services)                                                                                                                                                                                                                                                                                                                                                      |
| `formulae.brew.sh`                   | Verificações de versão de atualização em instalações do Homebrew. Outros métodos de instalação não contatam este host                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `code.claude.com`                    | Buscas de documentação do Claude Code pelo agente claude-code-guide integrado e solicitações WebFetch pré-aprovadas. Bloquear este host afeta apenas buscas de documentação                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

Se você instalar Claude Code através de npm ou gerenciar sua própria distribuição binária, os usuários finais não precisam dos usos do instalador nativo e atualizador automático de `downloads.claude.ai`, mas instalações npm e bun precisam de seu registro de pacotes, `registry.npmjs.org`, a menos que sua organização o espelhe. Os outros usos na tabela se aplicam independentemente do método de instalação.

Os dois hosts de ingestão do Datadog carregam apenas telemetria operacional opcional, e definir [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/pt/env-vars) desabilita ambos. Sessões em provedores de terceiros nunca enviam para esses hosts, mesmo quando uma plataforma define [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/pt/env-vars) e as métricas de telemetria são ativadas por padrão. Consulte [Serviços de telemetria](/docs/pt/data-usage#telemetry-services) para tudo que Claude Code envia e como desabilitá-lo antes de finalizar sua lista de permissões.

Ao usar [Amazon Bedrock](/docs/pt/amazon-bedrock), [Agent Platform do Google Cloud](/docs/pt/google-vertex-ai), [Microsoft Foundry](/docs/pt/microsoft-foundry) ou uma sessão de [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) conectada, o tráfego de modelo e autenticação vão para seu provedor ou gateway em vez de `api.anthropic.com`, `claude.ai` ou `platform.claude.com`. A ferramenta WebFetch ainda chama `api.anthropic.com` para sua [verificação de segurança de domínio](/docs/pt/data-usage#webfetch-domain-safety-check) a menos que você defina `skipWebFetchPreflight: true` em [configurações](/docs/pt/settings).

Ao rotear através de um [gateway LLM](/docs/pt/llm-gateway) com [`ANTHROPIC_BASE_URL`](/docs/pt/llm-gateway-connect#set-the-base-url-and-credential), a verificação de disponibilidade do [modo rápido](/docs/pt/fast-mode) ainda chama `api.anthropic.com` em vez da URL base do gateway. A verificação honra um proxy HTTP configurado, portanto, quando um bloqueio de rede é a causa, uma entrada de lista de permissões para `api.anthropic.com` no proxy é a solução. Um bloqueio de rede falha na verificação apenas quando o host é inacessível mesmo através do proxy, e o modo rápido então relata um erro de conectividade. O mesmo erro de conectividade aparece quando a verificação apresenta uma credencial emitida pelo gateway que Anthropic rejeita; a lista de permissões não ajuda lá, já que nada está bloqueado. Consulte [usar modo rápido atrás de proxies e gateways LLM](/docs/pt/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) para as variáveis que o restauram.

<h3 id="organization-ip-allowlists-and-proxy-egress">
  Listas de permissões de IP da organização e saída de proxy
</h3>

Se sua organização tiver [lista de permissões de IP](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) ativada para Claude, roteia `bridge.claudeusercontent.com` através da mesma saída de proxy que `claude.ai` e `api.anthropic.com`, por exemplo, colocando-o no mesmo segmento de aplicativo Zscaler ou política de direcionamento Netskope. Se você não conseguir roteá-lo dessa forma, adicione o endereço de saída que seu proxy usa para esse host à lista de permissões de IP da sua organização, mas apenas quando esse endereço for dedicado à sua organização: um intervalo de saída de proxy compartilhado também admite outros clientes do fornecedor de proxy.

Anthropic verifica conexões com `bridge.claudeusercontent.com` contra a lista de permissões de IP da sua organização usando o endereço de onde chegam. Se seu proxy enviar tráfego para esse host através de um endereço que não esteja nessa lista de permissões, Claude Code não conseguirá se conectar à [extensão Claude no Chrome](/docs/pt/chrome) mesmo que o resto do Claude Code funcione.

<h3 id="github-allow-lists-and-firewalls">
  Listas de permissões e firewalls do GitHub
</h3>

[Claude Code na web](/docs/pt/claude-code-on-the-web) em ambientes hospedados pela Anthropic e [Code Review](/docs/pt/code-review) se conectam aos seus repositórios a partir da infraestrutura gerenciada pela Anthropic; sessões em um [ambiente auto-hospedado](/docs/pt/self-hosted-environments) se conectam de dentro de sua rede, a menos que o executor opte pelo [proxy git Anthropic](/docs/pt/self-hosted-environments-deploy#use-the-anthropic-git-proxy), que busca do lado da Anthropic.

Se sua organização GitHub Enterprise Cloud restringe o acesso por endereço IP, ative [herança de lista de permissões de IP para GitHub Apps instalados](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#allowing-access-by-github-apps) e também [adicione uma entrada de lista de permissões](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#adding-an-allowed-ip-address) para os [endereços IP de saída](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) da Anthropic. A herança cobre apenas as solicitações que o GitHub App Claude faz como uma instalação, não as solicitações que faz em nome de seus usuários. Para outros firewalls, consulte os [endereços IP da API Anthropic](https://platform.claude.com/docs/en/api/ip-addresses).

Para instâncias [GitHub Enterprise Server](/docs/pt/github-enterprise-server) auto-hospedadas atrás de um firewall, coloque na lista de permissões os [endereços IP de saída](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses) da Anthropic para que a infraestrutura Anthropic possa alcançar seu host GHES para clonar repositórios e postar comentários de revisão. Sessões em um [ambiente auto-hospedado](/docs/pt/self-hosted-environments-deploy#configure-git) alcançam seu host GHES de dentro de sua rede, portanto essa exposição se aplica apenas a sessões hospedadas pela Anthropic, a fluxos pré-sessão hospedados, como o seletor de repositório, e a executores auto-hospedados que optam pelo [proxy git Anthropic](/docs/pt/self-hosted-environments-deploy#use-the-anthropic-git-proxy), que busca do lado da Anthropic. Para um host GHES que é roteável apenas dentro de sua rede, o [conector SCM](/docs/pt/self-hosted-environments-reference#scm-connector-flags) carrega os fluxos pré-sessão hospedados sobre uma conexão de saída, portanto a lista de permissões não é necessária para eles.

<h3 id="desktop-and-claude-ai">
  Desktop e claude.ai
</h3>

A tabela anterior cobre a CLI autônoma. O aplicativo Claude Desktop e claude.ai em um navegador carregam seu código de aplicativo e conteúdo do usuário de hosts CDN Anthropic adicionais, incluindo `assets-proxy.anthropic.com` e as outras origens `*.claudeusercontent.com` que servem [artifacts](/docs/pt/artifacts) nesses aplicativos. Permitir `claude.ai` enquanto bloqueia esses hosts produz uma página em branco em vez de um erro. Consulte [requisitos de acesso à rede](/docs/pt/desktop#network-access-requirements) na página Desktop.

Um [artifact](/docs/pt/artifacts) que carrega uma fonte tipográfica do [Google Fonts](/docs/pt/artifacts#improve-the-visual-design) também solicita `fonts.googleapis.com` e `fonts.gstatic.com`. Ambos os hosts são opcionais. Se você bloqueá-los, os artifacts são renderizados em fontes tipográficas de fallback. Bloqueie com uma rejeição rápida em vez de uma queda silenciosa para que a solicitação de fonte falhe imediatamente em vez de atrasar a primeira renderização da página.

Os artifacts também podem carregar bibliotecas JavaScript, como React ou um pacote de gráficos, de `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com` e `unpkg.com`, e de nenhum outro host externo. Se você bloquear esses hosts, as partes de um artifact que dependem de uma biblioteca não funcionam, e diferentemente de uma fonte bloqueada, uma biblioteca bloqueada não tem fallback. Bloqueie com uma rejeição rápida aqui também, para que uma solicitação de biblioteca bloqueada falhe imediatamente em vez de ficar pendurada até expirar.

<h2 id="additional-resources">
  Recursos adicionais
</h2>

* [Arquivos de configuração e precedência](/docs/pt/settings)
* [Referência de variáveis de ambiente](/docs/pt/env-vars)
* [Guia de solução de problemas](/docs/pt/troubleshooting)
