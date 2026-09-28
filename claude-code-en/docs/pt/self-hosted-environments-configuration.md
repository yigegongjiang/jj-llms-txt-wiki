> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Personalizar sessões em ambientes auto-hospedados

> Personalize sessões de ambientes auto-hospedados com scripts wrapper para credenciais por sessão, hooks de ciclo de vida e geração de runners sob demanda.

<Note>
  Ambientes auto-hospedados estão em beta pública em planos Team e Enterprise; um [Owner](/docs/pt/cloud-environments#organization-shared-environments) os habilita ativando **Allow self-hosted environments** na [página de administração **Cloud environments**](https://claude.ai/admin-settings/cloud-environments). Esta página assume um runner funcionando; consulte o [guia de início rápido](/docs/pt/self-hosted-environments-quickstart) para configuração e [Deploy to production](/docs/pt/self-hosted-environments-deploy) para as receitas de frota.
</Note>

Um [ambiente auto-hospedado](/docs/pt/self-hosted-environments) executa [sessões na nuvem](/docs/pt/claude-code-on-the-web) do Claude Code em sua própria infraestrutura, executadas por um processo runner que você implanta. Sem configuração, esse runner clona o repositório da sessão, gera Claude Code e limpa. Esta página é para o engenheiro de plataforma operando os runners: ela cobre os pontos de extensão para quando esses padrões não se encaixam, desde provisionamento de credenciais por sessão até substituição completa do checkout. Wrappers e hooks são executados como arquivos executáveis no host do runner, que é Linux ou macOS, e os exemplos nesta página assumem um shell POSIX.

Algumas variáveis de ambiente de hook nesta página ainda usam `pool`, como `CLAUDE_RUNNER_POOL_ID`; os nomes de flag CLI e variável de ambiente usam `environment`, como `--environment-secret-file`.

<h2 id="wrapper-scripts">
  Wrapper scripts
</h2>

Use um script wrapper quando cada sessão precisar de configuração que o runner não consegue fazer por conta própria: provisionamento de credenciais de curta duração com escopo para o criador da sessão, exportação de segredos específicos do ambiente, preparação de cadeias de ferramentas de linguagem ou aplicação de limites de recursos ao redor do processo filho. O runner inicia seu wrapper no lugar do binário Claude Code, uma vez por sessão. Termine o wrapper com `exec` em `$CLAUDE_RUNNER_CLAUDE_BIN`, o binário próprio do runner, para que sinais e códigos de saída se propaguem corretamente.

Aponte `--exec-path`, ou `SELF_HOSTED_RUNNER_EXEC_PATH`, para o wrapper quando você inicia o runner:

```bash theme={null}
claude self-hosted-runner --environment-secret-file /etc/claude/environment-secret --exec-path /etc/claude/session-wrapper.sh
```

O runner define o seguinte no ambiente do wrapper:

| Variável                            | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :---------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN`  | O JWT da sessão, prefixado com `sk-ant-cc-`. Sua reivindicação `act` identifica o criador da sessão, com o email do criador e o assunto do provedor de identidade upstream quando a superfície criadora os registrou. O valor é o token no momento do spawn; atualizações chegam pela stdin do filho, então um wrapper vê apenas o valor inicial. Consulte [Verify session identity](/docs/pt/self-hosted-environments-identity).                                                                                                                                                                                                                                                                 |
| `CCR_SESSION_ACCOUNT_EMAIL`         | O email do criador da sessão, pré-extraído pelo runner da reivindicação `act.email` do token sem verificação de assinatura. Adequado para rotulagem, como trailers de commit. Quando o email controla a emissão de credenciais, verifique o token e leia a reivindicação dele em vez disso; consulte [Provision credentials scoped to the session creator](#provision-credentials-scoped-to-the-session-creator). Não definido quando o token não carrega email do criador. Trate como informação de identificação pessoal.                                                                                                                                                                  |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`     | A superfície do cliente que criou a sessão, como `web_claude_ai`, `desktop_app`, `ios`, `claude_code_cli` ou `scheduled_trigger`. Anthropic registra o valor uma vez na criação da sessão, então o wrapper e cada hook de ciclo de vida veem o mesmo valor. Use-o apenas para análise de adoção e rotulagem, não como sinal de autorização. Não definido quando a sessão não tem superfície registrada ou reconhecida, então referencie-o como `${CLAUDE_RUNNER_CLIENT_PLATFORM:-}` sob `set -u`. Requer Claude Code v2.1.229 ou posterior.                                                                                                                                                  |
| `CLAUDE_RUNNER_CLAUDE_BIN`          | Caminho absoluto para o binário Claude Code próprio do runner. Termine seu wrapper com `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"` para passar para o binário fixado sem codificar um caminho de instalação.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `CLAUDE_CODE_REMOTE_SESSION_ID`     | ID da sessão na forma marcada `cse_...`. Esta é a mesma sessão que os [lifecycle hooks](#lifecycle-hooks) veem como `CLAUDE_RUNNER_SESSION_ID` na forma `session_...`; as variáveis UUID correspondem em ambos, e substituir o prefixo `cse_` por `session_` produz o ID mostrado na URL da sessão.                                                                                                                                                                                                                                                                                                                                                                                          |
| `CLAUDE_CODE_REMOTE_SESSION_UUID`   | O mesmo ID da sessão na forma UUID canônica, para sistemas que usam UUIDs como chave.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `CLAUDE_SESSION_INGRESS_TOKEN_FILE` | Caminho absoluto para um arquivo por sessão contendo o JWT da sessão atual, mantido atualizado em atualizações de token. Subprocessos shell o leem para seu cabeçalho `Authorization` ao baixar anexos que o usuário adicionou à sessão. `exec` preserva a variável automaticamente; um wrapper que reconstrói o ambiente do filho deve levar a variável, ou downloads de anexos param silenciosamente de funcionar.                                                                                                                                                                                                                                                                         |
| `CLAUDE_CONFIG_DIR`                 | Diretório de configuração Claude por sessão, escrito no início da sessão a partir do snapshot da configuração do host do runner que o runner captura na inicialização; consulte [Permissions and tool approval](#permissions-and-tool-approval). Escritas aqui são isoladas para esta sessão. O diretório fica sob `<base-dir>/_sessions/` após o término da sessão, a menos que você inicie o runner com [`--remove-session-state`](/docs/pt/self-hosted-environments-reference#runner-cli-flags); consulte [Reuse a pre-warmed checkout](/docs/pt/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout).                                                                                      |
| `ANTHROPIC_BASE_URL`                | A URL base da API que o filho usará, entregue pelo plano de controle por sessão e normalmente `https://api.anthropic.com`. Não a substitua: a credencial de inferência da sessão é um token OAuth emitido pela Anthropic que outros provedores não aceitam, então a inferência em ambientes auto-hospedados não é roteável para outro lugar.                                                                                                                                                                                                                                                                                                                                                 |
| `CLAUDE_CODE_OAUTH_TOKEN`           | O token de acesso OAuth de curta duração que o filho usa para inferência de modelo, com escopo apenas para inferência de modelo e upload de arquivo, com uma vida útil de cerca de 30 minutos. O runner o re-emite antes da expiração e entrega a rotação pela stdin do filho, então um wrapper que não [mantém stdin anexado](#keep-stdin-and-file-descriptor-3-attached) vê apenas o valor inicial. Não confie na lista de permissões de IP da sua organização para limitar o uso deste token: trate-o como uma credencial de portador que permanece utilizável por aproximadamente 30 minutos se vazar, e não o registre, escreva em disco ou encaminhe para fora do contêiner da sessão. |

O wrapper também herda o resto do ambiente gerenciado do filho, incluindo quaisquer variáveis de ambiente fornecidas pelo servidor. `exec` propaga tudo automaticamente; se seu wrapper gera o filho de outra forma, encaminhe o ambiente completo.

<h3 id="keep-stdin-and-file-descriptor-3-attached">
  Keep stdin and file descriptor 3 attached
</h3>

A stdin do filho é o canal de controle do runner. Rotações de token e sinais de fim de sessão chegam nela. O runner também abre um pipe no descritor de arquivo 3 e lê sinais de atividade do filho dele para conduzir timeouts de inatividade e inicialização. Um simples `exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"` preserva ambos automaticamente.

Se seu wrapper coloca o filho em background com um simples `&`, ele sever a stdin do filho: a sessão parece saudável até a vida útil do token OAuth inicial de aproximadamente 30 minutos expirar, então cada chamada de API falha com `401 authentication_error`. Se seu wrapper deve colocar o filho em background, por exemplo para manter uma trap de teardown viva, salve stdin no descritor de arquivo 4 ou superior e re-anexe-a explicitamente:

```bash theme={null}
exec 4<&0
"$CLAUDE_RUNNER_CLAUDE_BIN" "$@" <&4 4<&- &
CHILD=$!
trap 'teardown' EXIT
wait "$CHILD"
```

Não feche ou reutilize o descritor de arquivo 3 no wrapper. Redirecionar stdout e stderr do filho é aceitável.

<h3 id="provision-credentials-scoped-to-the-session-creator">
  Provision credentials scoped to the session creator
</h3>

Use o subcomando `decode-token` para ler reivindicações do JWT da sessão. Ele lê o token de um argumento, de `CLAUDE_CODE_SESSION_ACCESS_TOKEN` ou de stdin, nessa ordem; consulte [Verify the token inside the session](/docs/pt/self-hosted-environments-identity#verify-the-token-inside-the-session) para o que ele verifica. O exemplo abaixo decodifica a identidade do criador, a troca por credenciais AWS de curta duração e faz exec em Claude Code:

```bash theme={null}
#!/bin/bash
# Key on the stable Anthropic user ID and require a human creator.
CREATOR_SUB=$("$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token \
  | jq -re '.act.sub // "" | select(startswith("user:"))') \
  || { echo "decode-token: verification failed or no human creator" >&2; exit 1; }

creds=$(your-sts-helper assume-role --subject "$CREATOR_SUB") \
  || { echo "credential exchange failed" >&2; exit 1; }
eval "$creds"

exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@"
```

Use `jq -re` em vez de `jq -r` quando a reivindicação extraída controla uma decisão de autenticação, para que uma reivindicação ausente saia com código diferente de zero em vez de passar a string literal `null` para downstream. Sessões criadas por uma identidade de serviço da organização, como sessões de bot e agente, carregam um assunto `agent:` em vez de `user:`, então este exemplo as recusa; se seu ambiente serve essas sessões, decida explicitamente se o wrapper volta para uma credencial padrão para elas em vez de sair. Quando sua troca de credenciais precisa do assunto SSO ou email em vez disso, leia `.act.attested_by.sub` ou `.act.email` e trate sua ausência: o token os carrega apenas quando a superfície criadora os registrou, e uma [sessão despachada por CLI](/docs/pt/self-hosted-environments-testing#run-the-test-loop) pode carecer de ambos. Para a referência de reivindicação completa e verificação de serviços fora do runner, consulte [Verify session identity](/docs/pt/self-hosted-environments-identity).

<h2 id="lifecycle-hooks">
  Lifecycle hooks
</h2>

Lifecycle hooks substituem estágios do pipeline por sessão do runner com seus próprios scripts. Aponte o runner para um diretório de hooks com `--hooks-dir <path>`, ou `SELF_HOSTED_RUNNER_HOOKS_DIR`. O runner procura por arquivos executáveis com nomes bem conhecidos; qualquer hook que não esteja presente cai para o comportamento integrado, então você só escreve os que precisa. Hooks são executados com os privilégios próprios do runner, e filhos de sessão compartilham esse UID, então monte o diretório de hooks como somente leitura, ou coloque-o na imagem, para que o código da sessão não possa modificá-lo; consulte a [seção de hardening](/docs/pt/self-hosted-environments-deploy#harden-your-deployment).

Esses hooks são distintos dos [Claude Code hooks](/docs/pt/hooks), que são executados dentro da sessão; lifecycle hooks são executados no runner, ao redor da sessão.

<h3 id="checkout">
  checkout
</h3>

Executado uma vez por repositório, no lugar do clone e fetch integrados do runner. Use o hook para clonar de um espelho de leitura, semear uma árvore de trabalho de um arquivo ou aplicar autenticação git por sessão. O runner define:

| Variável                           | Descrição                                                                                                                                                            |
| :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_REPO_URL`           | URL do repositório para clonar, após qualquer `--git-host-rewrite` e `--git-ssh-rewrite` terem sido aplicados                                                        |
| `CLAUDE_RUNNER_REPO_REF`           | Revisão para fazer checkout: branch, tag ou commit SHA conforme a sessão o solicitou. Vazio significa o branch padrão do repositório.                                |
| `CLAUDE_RUNNER_CHECKOUT_PATH`      | Caminho absoluto onde a árvore de trabalho deve ser deixada                                                                                                          |
| `CLAUDE_RUNNER_SESSION_ID`         | ID da sessão na forma marcada `session_...`, para logging e correlação                                                                                               |
| `CLAUDE_RUNNER_SESSION_UUID`       | O mesmo ID da sessão na forma UUID canônica                                                                                                                          |
| `CLAUDE_RUNNER_API_BASE_URL`       | URL base da API Anthropic para chamadas com escopo de sessão                                                                                                         |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`    | A superfície do cliente que criou a sessão, como `web_claude_ai`, `desktop_app` ou `ios`. Não definido quando a sessão não tem superfície registrada ou reconhecida. |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | O token de acesso da sessão, para chamadas de API com escopo de sessão                                                                                               |

O script deve deixar uma árvore de trabalho em `CLAUDE_RUNNER_CHECKOUT_PATH` com checkout na revisão solicitada. HEAD desanexado é aceitável; o runner cria o branch de trabalho da sessão em cima. O runner verifica se o caminho contém um `.git` depois; se seu hook materializa uma fonte não-git como Perforce ou um tarball desempacotado, defina `CLAUDE_RUNNER_SKIP_GIT_VERIFY=1` no ambiente do runner para pular essa verificação. Fluxos baseados em Git como criação de branch de trabalho e push de resultados requerem um checkout git, então exporte resultados de árvores não-git com um hook [`post-session`](#post-session).

O runner não passa uma credencial git para o hook. Em vez disso, emita uma credencial de clone por sessão a partir da identidade da sessão: verifique `CLAUDE_CODE_SESSION_ACCESS_TOKEN` com uma biblioteca JWT padrão contra o endpoint JWKS sob `CLAUDE_RUNNER_API_BASE_URL`, conforme descrito em [Verify the token from your service](/docs/pt/self-hosted-environments-identity#verify-the-token-from-your-service), então faça seu serviço de credencial emitir uma credencial de clone de curta duração para a identidade na reivindicação `act` do token. `CLAUDE_RUNNER_CLAUDE_BIN` não está definido no ambiente do checkout-hook, então o subcomando `decode-token` não está disponível aqui. Voltar para qualquer autenticação git que o host já tenha, como um agente SSH, credential helper ou `.netrc`, também é uma opção.

Quando o hook sai com código diferente de zero, ou sai com 0 sem deixar um checkout utilizável atrás, o que o runner faz depende do repositório:

* **Um repositório para o qual a sessão faz push de resultados**: o runner falha a sessão, e em uma saída diferente de zero exibe a cauda do stderr do script para o usuário.
* **Um repositório que a sessão apenas lê**, como um repositório adicionado a uma sessão em execução: o runner registra uma linha `[runner:warn]` com o detalhe da falha, publica um passo `Skipped` para a sessão, remove o que o hook deixou no caminho de checkout e continua com os repositórios restantes. Quando o runner não consegue remover o caminho imediatamente, ele tenta novamente a remoção no fim da sessão. Se pular deixa a sessão sem nenhum repositório, o runner falha a sessão mesmo assim.

Antes da v2.1.228, o runner falhava a sessão em uma falha de hook para qualquer repositório, então um repositório somente leitura que o hook não conseguia servir falhava a sessão novamente em cada novo runner fresco em que a sessão retomava.

O runner remove o caminho de checkout após a sessão terminar.

<h3 id="post-session">
  post-session
</h3>

Executado uma vez por sessão, após o filho Claude Code ter saído e antes do runner desmontar o workspace. Este hook é sua única chance de salvar trabalho não confirmado: em `--capacity` acima de um, o runner deleta worktrees por sessão logo após o hook retornar, e em `--capacity 1` o [clone canônico](/docs/pt/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout) reutilizado é hard-reset quando a próxima sessão começa, então mudanças rastreadas não confirmadas não sobrevivem em nenhum caminho. Usos típicos são fazer push de um branch de snapshot de mudanças não confirmadas, arquivar logs ou emitir um evento de fim de sessão para seus próprios sistemas.

O hook dispara em cada fim de sessão onde um processo filho foi gerado, qualquer que seja a causa; os valores `CLAUDE_RUNNER_EXIT_REASON` abaixo enumeram os casos. Não pode disparar quando o runner termina abruptamente, como uma preempção de VM ou perda de energia; se você precisa de garantias contra terminação abrupta, faça snapshot periodicamente de dentro da sessão com um hook Claude Code `PostToolUse` em vez disso. O runner define:

| Variável                           | Descrição                                                                                                                                                                                                      |
| :--------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_SESSION_ID`         | ID da sessão na forma marcada `session_...`                                                                                                                                                                    |
| `CLAUDE_RUNNER_SESSION_UUID`       | O mesmo ID da sessão na forma UUID canônica                                                                                                                                                                    |
| `CLAUDE_RUNNER_EXIT_REASON`        | Como a sessão terminou; consulte os valores abaixo da tabela                                                                                                                                                   |
| `CLAUDE_RUNNER_WORKSPACE_PATHS`    | Caminhos absolutos separados por dois-pontos das árvores de trabalho da sessão. Vazio para sessões sem repositório.                                                                                            |
| `CLAUDE_RUNNER_DEBUG_LOG_PATH`     | Caminho para o log de debug da sessão, ainda em disco enquanto o hook é executado                                                                                                                              |
| `CLAUDE_RUNNER_API_BASE_URL`       | URL base da API Anthropic para chamadas com escopo de sessão                                                                                                                                                   |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`    | A superfície do cliente que criou a sessão, como `web_claude_ai`, `desktop_app` ou `ios`. Não definido quando a sessão não tem superfície registrada ou reconhecida. Requer Claude Code v2.1.229 ou posterior. |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | O token de acesso da sessão, para chamadas de API com escopo de sessão                                                                                                                                         |

`CLAUDE_RUNNER_EXIT_REASON` toma um de quatro valores:

* `completed`: a sessão terminou de forma limpa. O processo Claude Code saiu normalmente, ou a sessão foi arquivada ou deletada enquanto ainda estava em execução.
* `failed`: o processo Claude Code travou, ou a configuração falhou após ele ter iniciado.
* `interrupted`: o runner parou a sessão. Ele liberou a sessão para liberar o slot, a sessão expirou na inicialização, o servidor moveu a sessão para fora deste runner, o runner estava drenando, ou a sessão ultrapassou seu limite [`--kill-session-after-min`](/docs/pt/self-hosted-environments-reference#runner-cli-flags).
* `abandoned`: reservado para uma sessão que outro runner reivindicou. O hook não dispara atualmente nesse caso.

Os [contadores de ciclo de vida da sessão](/docs/pt/self-hosted-environments-reference#session-lifecycle-counter-semantics) contam uma liberação, um timeout de inicialização e uma movimentação de servidor como `completed` em vez de `interrupted`, porque o runner devolveu o slot de forma limpa. Espere essa diferença se você comparar recibos de hook com os contadores.

O status de saída do hook nunca afeta o resultado da sessão; uma falha é registrada e ignorada. O runner aguarda até `--post-session-hook-timeout-sec`, 60 segundos por padrão, em cada fim de sessão incluindo shutdown do runner. Este exemplo salva trabalho não confirmado para um branch de resgate:

```bash theme={null}
#!/usr/bin/env bash
set -u
IFS=':'
# Pin config the session could have planted in the checkout's .git/config:
# -c overrides beat repo-local settings, blocking session-written fsmonitor,
# hook-path, and gpg-program config from executing code with the hook's
# privileges. Repo-local credential.helper, core.sshCommand, and pushurl
# still apply; if the hook holds credentials the session didn't, pin the
# push URL and helper too (see the note below the script).
g() { git -c core.fsmonitor=false -c core.hooksPath=/dev/null \
        -c commit.gpgsign=false "$@"; }
for ws in $CLAUDE_RUNNER_WORKSPACE_PATHS; do
  cd "$ws" 2>/dev/null || continue
  [ -z "$(g status --porcelain 2>/dev/null)" ] && continue
  g add -A
  g commit -q -m "runner snapshot: $CLAUDE_RUNNER_SESSION_ID ($CLAUDE_RUNNER_EXIT_REASON)" || continue
  g push -q origin "HEAD:refs/heads/rescue/$CLAUDE_RUNNER_SESSION_ID" || true
done
```

O hook faz push com quaisquer credenciais git disponíveis em seu próprio ambiente no host do runner. Sob a [postura de sem-credenciais-na-imagem](/docs/pt/self-hosted-environments-deploy#configure-git), incluindo quando o clone integrado passa pelo proxy git Anthropic, não há nenhuma, então emita uma credencial de push de curta duração dentro do hook antes de fazer push: troque o token de sessão que o hook recebe em `CLAUDE_CODE_SESSION_ACCESS_TOKEN` com seu próprio serviço de token, verificando-o conforme [Verify session identity](/docs/pt/self-hosted-environments-identity) descreve. Quando o hook mantém uma credencial que a sessão não tinha, também fixe para onde ele faz push: substitua `origin` por uma URL fornecida pelo operador e passe `-c credential.helper=` mais seu próprio helper, para que a configuração local do repo que a sessão escreveu não possa redirecionar o push credenciado.

<h4 id="hook-timing-when-the-runner-releases-a-session">
  Hook timing when the runner releases a session
</h4>

Uma sessão liberada pode retomar em outro runner. Em um runner na v2.1.236 ou posterior, o que a sessão estava fazendo na liberação decide se ela pode retomar antes deste hook terminar:

* **Inativo após uma volta, ou expirado na inicialização**: o runner para o filho e executa este hook até a conclusão. Apenas então ele libera a sessão. Uma mensagem do usuário enviada enquanto o hook é executado não pode retomar a sessão em outro runner antes do hook terminar.
* **Aguardando o usuário responder a um prompt, como um prompt de permissão**: o runner libera a sessão primeiro, então executa este hook. Uma mensagem do usuário enviada enquanto o hook é executado pode retomar a sessão em outro runner antes do hook terminar.

Isso se aplica sempre que o runner libera uma sessão: no timeout de inatividade, no tempo [`--retire-at`](/docs/pt/self-hosted-environments-reference#runner-cli-flags), e, em um runner na v2.1.260 ou posterior, no limite [`--kill-session-after-min`](/docs/pt/self-hosted-environments-reference#runner-cli-flags) de uma sessão. Uma sessão cuja volta terminou e que mantém apenas tarefas em background conta como inativa aqui. Antes da v2.1.236, o runner liberava a sessão primeiro e então executava este hook em ambos os casos.

Durante uma drenagem `SIGTERM`, o runner mantém a concessão da sessão até o hook terminar; consulte [Shutdown timing](/docs/pt/self-hosted-environments-deploy#shutdown-timing).

<h3 id="command">
  command
</h3>

Executado uma vez por sessão após checkout, no lugar do spawn do filho integrado. O hook recebe o mesmo ambiente que um [wrapper script](#wrapper-scripts) e deve fazer `exec` em `"$CLAUDE_RUNNER_CLAUDE_BIN"` da mesma forma. Use o hook `command` para manter toda a customização em um diretório de hooks; use `--exec-path` quando o wrapper vive em outro lugar. Se `--exec-path` também está definido, a flag tem precedência e o hook `command` é ignorado.

Sempre faça `exec` do binário próprio do runner em vez de um `claude` resolvido por PATH; caso contrário você derrota o [pinning de versão](/docs/pt/self-hosted-environments-deploy#pin-the-version).

<h2 id="on-demand-runners">
  On-demand runners
</h2>

Em vez de executar uma frota fixa, você pode inicializar um runner por sessão. O orquestrador é um subcomando separado e sem estado que faz polling na Anthropic para solicitações de spawn, uma por sessão que está enfileirada sem runner disponível, e executa seu hook `spawn-runner` para cada uma. Seu hook submete uma carga de trabalho para sua plataforma: um Kubernetes Job, uma instância EC2, um Nomad dispatch.

Runners sob demanda melhoram a higiene de credenciais. Em uma frota fixa, o segredo do ambiente vive em cada host do runner, que é o mesmo host que executa sessões do usuário. Com o orquestrador, o segredo do ambiente fica apenas no host do orquestrador, que nunca executa código do usuário; cada runner gerado recebe uma ordem de trabalho de uso único que registra exatamente um runner e depois expira.

Para iniciar o orquestrador, passe o segredo do ambiente e um diretório de hooks contendo um script `spawn-runner` executável:

```bash theme={null}
claude self-hosted-runner orchestrator \
  --environment-secret-file /etc/claude/environment-secret \
  --hooks-dir /etc/claude/hooks
```

O orquestrador não mantém estado entre polls, então você pode executar duas ou mais réplicas contra o mesmo ambiente para disponibilidade. Cada solicitação de spawn é reivindicada no lado do servidor por exatamente uma réplica. Todas as réplicas devem usar o mesmo valor `--expected-spawn-seconds`; consulte o [contrato do hook](#the-spawn-runner-hook).

<h3 id="the-spawn-runner-hook">
  The spawn-runner hook
</h3>

O orquestrador executa `${hooks-dir}/spawn-runner` uma vez por solicitação de spawn. O hook deve submeter trabalho de forma assíncrona, sem aguardar o boot do runner, e retornar dentro de `--hook-timeout`, 60 segundos por padrão. O hook recebe:

| Variável                              | Descrição                                                                                                                                                                                                                                                                                                                                                   |
| :------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_RUNNER_WORK_ORDER_FILE`       | Caminho para um arquivo temporário contendo o JWT da ordem de trabalho assinada que o novo runner se registra. Deletado após o hook sair. Não registre o conteúdo do arquivo.                                                                                                                                                                               |
| `CLAUDE_RUNNER_ORDER_ID`              | Chave de idempotência opaca, única por solicitação de spawn e segura para nomes de recursos Kubernetes. Use-a como sua chave de dedup do provisionador.                                                                                                                                                                                                     |
| `CLAUDE_RUNNER_SESSION_ID`            | A sessão para a qual esta solicitação é. Vazio para solicitações de pré-aquecimento, que inicializam um runner em standby antes de qualquer sessão específica quando [`--min-idle`](/docs/pt/self-hosted-environments-reference#orchestrator-cli-flags) está definido, então não assuma que a variável está definida.                                            |
| `CLAUDE_RUNNER_SESSION_UUID`          | O mesmo ID da sessão na forma UUID canônica. Vazio para solicitações de pré-aquecimento.                                                                                                                                                                                                                                                                    |
| `CLAUDE_RUNNER_ATTEMPT`               | Quantas solicitações de spawn esta sessão teve. `0` para solicitações de pré-aquecimento.                                                                                                                                                                                                                                                                   |
| `CLAUDE_RUNNER_ORDER_SERVER_TIME`     | Hora do servidor do cabeçalho HTTP `Date` da resposta de poll. Quando o hook verifica o `exp` do JWT da ordem de trabalho, compare contra este valor em vez do relógio local para tolerar skew. Vazio quando o gateway omitiu o cabeçalho.                                                                                                                  |
| `CLAUDE_RUNNER_POOL_ID`               | O ID do ambiente que o novo runner deve se juntar, na forma `ccpool_...`                                                                                                                                                                                                                                                                                    |
| `CLAUDE_RUNNER_ACCOUNT_ID`            | ID marcado da conta que enfileirou a sessão, para roteamento por conta, quota ou chargeback. Vazio quando indisponível, e sempre vazio para sessões do canal Claude Tag, que nenhuma conta enfileira.                                                                                                                                                       |
| `CLAUDE_RUNNER_ACCOUNT_EMAIL`         | Email da conta que enfileirou a sessão. Vazio quando indisponível. Trate o email como informação de identificação pessoal e não o registre.                                                                                                                                                                                                                 |
| `CLAUDE_RUNNER_PRIMARY_REPO_URL`      | URL da primeira fonte git da sessão, para roteamento para um runner com esse repositório pré-aquecido. Vazio quando a sessão não tem fontes git.                                                                                                                                                                                                            |
| `CLAUDE_RUNNER_PRIMARY_REPO_REVISION` | Revisão da primeira fonte git da sessão: branch, SHA ou tag. Vazio quando não especificado.                                                                                                                                                                                                                                                                 |
| `CLAUDE_RUNNER_REPO_SOURCES`          | Array JSON de `{url, revision}` para todas as fontes git da sessão, para hooks que roteiam em um repositório secundário. Vazio quando não há fontes.                                                                                                                                                                                                        |
| `CLAUDE_RUNNER_CORRELATION_ID`        | O ID de correlação fornecido na criação da sessão, ecoado de volta para que o hook possa mapear esta ordem de trabalho para a solicitação que criou a sessão. Vazio quando a sessão não tem nenhum.                                                                                                                                                         |
| `CLAUDE_RUNNER_CLIENT_PLATFORM`       | A superfície do cliente que criou a sessão, como `web_claude_ai`, `desktop_app`, `ios` ou `scheduled_trigger`, para análise de adoção. Não definido quando a sessão não tem superfície registrada ou reconhecida, e para solicitações de pré-aquecimento; verifique-o com `[ -n "${CLAUDE_RUNNER_CLIENT_PLATFORM:-}" ]`, que permanece seguro sob `set -u`. |

O runner gerado se registra com a ordem de trabalho no lugar do segredo do ambiente:

* **Inicie-o com a ordem de trabalho**: aponte [`--environment-secret-file`](/docs/pt/self-hosted-environments-reference#runner-cli-flags) para um arquivo contendo o JWT da ordem de trabalho, ou defina `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET` para o valor JWT.
* **Copie o JWT antes do hook sair**: o orquestrador deleta o arquivo da ordem de trabalho após o hook sair, então copie o JWT para a carga de trabalho que você submete, como um Kubernetes Secret no Job gerado, em vez de passar o caminho do arquivo.
* **Use `--capacity 1` em runners gerados**: uma ordem de trabalho vinculada a sessão registra exatamente um runner vinculado a essa sessão, então uma capacidade maior adiciona slots que nunca recebem trabalho, e o runner registra um aviso na inicialização.
* **Ordens de trabalho de pré-aquecimento registram desvinculadas**: o runner em standby não está vinculado a uma sessão e reclama trabalho enfileirado como um runner de frota fixa.

O contrato tem quatro regras agnósticas do provisionador:

1. **Seja idempotente em `CLAUDE_RUNNER_ORDER_ID`.** Reentrega da mesma solicitação deve gerar no máximo um runner. Derive um nome de recurso determinístico do ID e deixe sua plataforma rejeitar a duplicata.
2. **Não tente novamente a carga de trabalho.** Um ID de ordem significa no máximo uma carga de trabalho criada. Se o runner nunca se registra, Anthropic re-solicita com um ID de ordem fresco após `--expected-spawn-seconds`.
3. **Use o contrato de código de saída.** Saída 0 significa submetido. Saída 1 significa falha retentável; a sessão recua e é re-oferecida. Saída 2 ou superior significa não-retentável; a sessão é bloqueada de gerar novamente até um [Owner](/docs/pt/cloud-environments#organization-shared-environments) selecionar **Retry** nela na aba **Activity** do ambiente. Em saída diferente de zero, a cauda do stderr do hook aparece lá como o motivo da falha, então escreva o erro acionável para stderr e nunca segredos. Para uma solicitação de pré-aquecimento não há sessão para falhar: o orquestrador registra uma saída diferente de zero localmente apenas, e o servidor re-solicita o spawn após a concessão.
4. **Defina `--expected-spawn-seconds` para pelo menos seu tempo de boot p99.** Esta é a concessão no lado do servidor. Todas as réplicas do orquestrador devem usar o mesmo valor.

Tudo que o hook escreve para stdout ou stderr aparece no log do orquestrador com credenciais automaticamente redatadas. Se sessões ficarem enfileiradas, verifique o corpo `/healthz` do orquestrador para contagens de fila, então abra a aba **Activity** do seu ambiente na [página de administração **Cloud environments**](https://claude.ai/admin-settings/cloud-environments): expanda uma sessão falhada lá para seu erro de spawn e selecione **Retry** para re-solicitá-la.

<h2 id="mcp-servers">
  MCP servers
</h2>

Para disponibilizar [MCP servers](/docs/pt/mcp) em cada sessão, adicione-os no tempo de construção da imagem com o mesmo comando `claude mcp add` usado em uma instalação desktop. Se seu runner é um processo bare em vez de um contêiner, execute o mesmo comando como o usuário do runner no host, então reinicie o runner: ele lê configuração do host uma vez na inicialização. A flag `--scope user` é obrigatória; o escopo local padrão escreve sob uma chave por diretório que o runner não semeia em sessões. Por exemplo, em seu Dockerfile:

```dockerfile theme={null}
RUN claude mcp add --scope user sidecar -- /usr/local/bin/mcp-sidecar
RUN claude mcp add --scope user --transport http internal http://mcp-gateway.svc.cluster.local:8080
```

O runner faz um snapshot da configuração do host uma vez na inicialização. O snapshot captura a chave `mcpServers` do `.claude.json` do host, que vive ao lado em vez de dentro de `~/.claude/`, e o runner semeia apenas essa chave em cada configuração isolada da sessão; estado da conta e histórico de projeto são descartados. Para confirmar que os servidores chegaram às sessões, inicie uma sessão no ambiente e peça a Claude para listar suas ferramentas MCP; o runner também registra um aviso de inicialização para qualquer entrada capturada cujo `type` ele não reconhece e descarta a entrada, então você pode ver por que esse servidor está faltando nas sessões. Quando `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR` está definido, o runner lê `.claude.json` desse diretório em vez disso, então apontar a variável para um diretório vazio também desabilita a semeadura de MCP.

Claude Code também carrega MCP servers de outras fontes:

* O [arquivo MCP gerenciado](/docs/pt/managed-mcp) de escopo empresarial em seu caminho de sistema padrão: `/etc/claude-code/managed-mcp.json` em hosts do runner Linux, `/Library/Application Support/ClaudeCode/managed-mcp.json` em hosts macOS. Use-o para frotas bloqueadas onde apenas servidores listados pelo administrador podem carregar. Consulte [exclusive control with managed-mcp.json](/docs/pt/managed-mcp#exclusive-control-with-managed-mcp-json) para as regras de precedência. Quando este arquivo está no host do runner, Claude Code pula os MCP servers que o plano de controle da Anthropic entrega a uma sessão, incluindo conectores claude.ai, e os nomeia em um aviso no stderr do filho da sessão, que o runner registra no nível de log `debug`. Antes da v2.1.229, essas sessões saíam na inicialização com `You cannot dynamically configure MCP servers when an enterprise MCP config is present`.
* A chave [`managedMcpServers`](/docs/pt/settings-reference#managedmcpservers) em [managed settings](/docs/pt/managed-settings) no host do runner: fornece servidores HTTP e SSE sem tomar controle exclusivo, então servidores das outras fontes ainda carregam. Requer Claude Code v2.1.259 ou posterior.
* `<repo>/.mcp.json`: escopo de projeto. Confirme o arquivo no repositório; seus servidores são pré-aprovados em sessões na nuvem.

Quando a entrega de conectores está habilitada para sua organização, o plano de controle da Anthropic entrega os conectores que você configurou em claude.ai para sessões criadas interativamente através de configuração MCP fornecida pelo servidor, roteada através de `api.anthropic.com`. Sessões criadas programaticamente, como [CLI dispatches](/docs/pt/self-hosted-environments-testing#run-the-test-loop), não recebem entrega de conectores; dê-lhes MCP servers através de qualquer uma das outras fontes que esta seção lista em vez disso. O token OAuth do filho não carrega um escopo para buscar conectores diretamente, então o filho não tenta essa busca em si; a entrega é orientada pelo servidor.

`settings.json` não carrega definições de MCP server, e não há campo `mcpServers` de nível superior no esquema de configurações. Em managed settings, forneça servidores com a chave [`managedMcpServers`](/docs/pt/settings-reference#managedmcpservers) em vez disso.

Sessões herdam o ambiente do runner, então defina [`ENABLE_TOOL_SEARCH`](/docs/pt/mcp#scale-with-mcp-tool-search) lá para controlar a busca de ferramentas MCP para cada sessão que um runner gera; a página MCP cobre os valores.

<h2 id="prompt-sessions-to-push-their-work">
  Prompt sessions to push their work
</h2>

Sessões hospedadas pela Anthropic executam um hook [`Stop`](/docs/pt/hooks#stop), o hook Claude Code que é executado quando Claude termina de responder, que solicita a Claude fazer commit e push de seu trabalho. O runner não instala um. Sem ele, uma sessão que termina com mudanças não confirmadas deixa esse trabalho apenas no disco do runner, e o botão **Create PR** em claude.ai/code fica inativo até o branch existir no remoto.

A implementação de referência abaixo tem duas partes. Mescle o bloco de configurações em `~/.claude/settings.json` no host do runner, que o runner semeia em cada sessão, e salve o script como `~/.claude/hooks/stop-hook-nudge.sh` no host do runner e torne-o executável:

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "timeout": 10,
            "command": "\"$CLAUDE_CONFIG_DIR/hooks/stop-hook-nudge.sh\""
          }
        ]
      }
    ]
  }
}
```

```sh theme={null}
#!/bin/sh
# Stop-hook reference implementation for self-hosted runners.
#
# Nudges Claude once per turn if the project directory has uncommitted
# changes OR unpushed commits, so work isn't lost when an idle session
# is released and so the "Create PR" button on claude.ai/code lights up.
#
# Runner-level (no repo changes): drop this file at ~/.claude/hooks/ on
# the runner host and merge the accompanying Stop-hook settings block
# into ~/.claude/settings.json — the runner seeds both into every session.
# Repo-level alternative: commit to <repo>/.claude/hooks/ and change the
# settings.json command path to $CLAUDE_PROJECT_DIR/.claude/hooks/.
#
# stdin: hook JSON payload (see https://code.claude.com/docs/en/hooks)
# stdout: {"decision":"block","reason":"..."} to nudge, or nothing to allow stop.

# Re-entry guard: the harness sets stop_hook_active=true when re-invoking
# the Stop hook after a block. Bail so we only nudge once per turn. The
# harness emits compact JSON (no space after the colon), which this
# pattern relies on; use jq if you need a whitespace-tolerant check.
in=$(cat)
case "$in" in *'"stop_hook_active":true'*) exit 0 ;; esac

d="$CLAUDE_PROJECT_DIR"

# Not a git repo → nothing to nudge.
git -C "$d" rev-parse --git-dir >/dev/null 2>&1 || exit 0

# No remote → "push to the remote" is unsatisfiable; bail.
[ -z "$(git -C "$d" remote 2>/dev/null)" ] && exit 0

# Uncommitted changes (staged, unstaged, or untracked). Exclude .claude/
# entirely — operator-seeded settings and CLI-written runtime state
# (scheduler lock, worktrees, routine state) live there and neither is
# "uncommitted work" the model needs to push.
s=$(git -C "$d" status --porcelain -- . ':(exclude).claude/' 2>/dev/null)
if [ -n "$s" ]; then
  printf '{"decision":"block","reason":"There are uncommitted changes in the repository. Please commit and push these changes to the remote branch."}'
  exit 0
fi

# Unpushed commits. Count commits on HEAD not reachable from any
# remote-tracking ref or FETCH_HEAD. This works uniformly for:
#   - init+fetch checkouts (runner default: only FETCH_HEAD exists)
#   - clone-based checkouts (origin/* exist)
#   - the runner default: the child starts on the session's outcome
#     branch, which the runner creates after checkout
#   - detached HEAD, when a custom setup skips that branch creation
# With no reference point at all (never fetched), stay silent rather
# than false-positive on a read-only turn.
base=""
git -C "$d" rev-parse --verify -q FETCH_HEAD >/dev/null && base="FETCH_HEAD"
if [ -z "$base" ] && [ -z "$(git -C "$d" for-each-ref --count=1 refs/remotes/origin 2>/dev/null)" ]; then
  exit 0
fi
# shellcheck disable=SC2086  # $base is either "" or "FETCH_HEAD", intentional word-split
unpushed=$(git -C "$d" rev-list HEAD --not $base --remotes=origin --count 2>/dev/null) || unpushed=0
if [ "$unpushed" -gt 0 ]; then
  branch=$(git -C "$d" symbolic-ref --short -q HEAD)
  if [ -n "$branch" ]; then
    # $branch is attacker-influenced — git-check-ref-format(1) allows `"`
    # in ref names. `\` is forbidden (rule 10) but escaped anyway as cheap
    # defense-in-depth.
    # Escape JSON metacharacters before interpolating into the hand-built
    # payload so a branch like x","continue":false can't inject keys into
    # the hook-output JSON the harness parses. $unpushed is safe — the
    # -gt guard above rejects anything that isn't a plain integer.
    branch_esc=$(printf '%s' "$branch" | sed 's/\\/\\\\/g; s/"/\\"/g')
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on branch '\''%s'\''. Please push these changes to the remote repository."}' "$unpushed" "$branch_esc"
  else
    printf '{"decision":"block","reason":"There are %s unpushed commit(s) on a detached HEAD. Please create a branch and push it to the remote repository."}' "$unpushed"
  fi
  exit 0
fi

exit 0
```

O hook solicita a Claude fazer commit e push antes da sessão terminar, e fica silencioso quando o diretório não é um repositório git ou não tem remoto.

<h2 id="permissions-and-tool-approval">
  Permissions and tool approval
</h2>

Uma sessão auto-hospedada não tem terminal anexado, então um prompt de permissão não respondido paralisa a volta até o usuário responder na UI. O plano de controle da Anthropic envia a lista de ferramentas de cada sessão e regras de permissão com a carga de trabalho; a configuração padrão pré-aprova chamadas de ferramentas rotineiras, incluindo `Bash`, e sessões na nuvem [pré-aprovam edições de arquivo independentemente do modo](/docs/pt/permission-modes#switch-permission-modes). Uma chamada que nada pré-aprova solicita através da UI da sessão.

<Note>
  Apenas fixe auto mode em um ambiente cujos contêineres de sessão são executados com [default-deny network egress](/docs/pt/self-hosted-environments-deploy#default-deny-egress) e o resto da [seção de hardening](/docs/pt/self-hosted-environments-deploy#harden-your-deployment) em vigor. Chamadas de ferramentas rotineiras, incluindo solicitações de rede `Bash`, são executadas sem um humano no loop tanto no conjunto de ferramentas pré-aprovadas padrão quanto em auto mode, então o limite de rede é o que limita para onde essas chamadas podem alcançar.
</Note>

Para manter prompts ao mínimo independentemente do que o plano de controle envia, fixe [auto mode](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) de seu script wrapper ou hook [`command`](#command). Auto mode permite que sessões sejam executadas sem prompts de permissão rotineiros: um modelo classificador separado revisa ações antes de serem executadas e bloqueia as que rejeita, e regras de ask explícitas ainda forçam um prompt; a página de modos de permissão cobre o que o classificador verifica. O runner anexa flags computadas pelo servidor antes de invocar o wrapper, e para flags de valor único como `--permission-mode` o parser honra a última ocorrência, então uma flag que você anexa após `"$@"` substitui o valor enviado pelo servidor:

```bash theme={null}
#!/bin/bash
exec "$CLAUDE_RUNNER_CLAUDE_BIN" "$@" --permission-mode auto
```

Para pré-aprovar ferramentas específicas em vez disso, anexe `--allowed-tools` com suas regras, por exemplo `--allowed-tools "Bash(bazel *) Bash(yarn *) mcp__internal__*"`. Flags de lista como `--allowed-tools` e `--disallowed-tools` acumulam em ocorrências em vez de substituir, então suas regras se aplicam em cima de quaisquer regras que o plano de controle envia. Para estreitar, anexe `--disallowed-tools`, que nega ferramentas mesmo se outra regra as permite.

<h3 id="how-each-session’s-config-is-assembled">
  How each session's config is assembled
</h3>

O runner dá a cada sessão seu próprio diretório de configuração, semeado de um snapshot do `~/.claude/` do host que o runner captura uma vez na inicialização: `settings.json`, `CLAUDE.md`, hooks, agentes, comandos e skills em sua imagem do runner se aplicam a cada sessão como a linha de base de nível de usuário. Se você alterar a configuração em um host em execução, a alteração tem efeito apenas após reiniciar o runner.

Defina `SELF_HOSTED_RUNNER_HOST_CONFIG_DIR` para semear de um caminho diferente, ou aponte-o para um diretório vazio para desabilitar a semeadura.

`.claude/settings.json` confirmado no repositório se sobrepõe como configurações de projeto. Sessões também leem [`managed-settings.json`](/docs/pt/settings#where-settings-live) do caminho de sistema padrão em sua imagem do runner. Se suas chaves se aplicam ao lado de [server-managed settings](/docs/pt/server-managed-settings) segue [como Claude Code combina fontes gerenciadas](/docs/pt/managed-settings#how-claude-code-combines-managed-sources): por padrão, quando sua organização entrega quaisquer chaves gerenciadas pelo servidor, sessões ignoram o arquivo da imagem do runner além das [chaves que Claude Code lê de cada fonte de administrador](/docs/pt/managed-settings#keys-read-from-every-admin-source), como o bloco `env`, os locks de sandbox, os caminhos binários de sandbox e `forceRemoteSettingsRefresh`. Consulte [settings precedence](/docs/pt/settings#settings-precedence).

Quando o plano de controle da Anthropic fornece uma sessão com [Claude Code hooks](/docs/pt/hooks), o runner os instala ao lado, não sobre, sua própria configuração. Requer Claude Code v2.1.229 ou posterior.

* **Onde eles pousam**: o runner escreve cada script de hook fornecido para um subdiretório reservado `hooks/.ccr-launcher/` do diretório de configuração da sessão e registra os scripts em um arquivo de configurações separado que passa para a sessão com `--settings`, deixando o `settings.json` semeado e seus próprios scripts em `hooks/<name>` intocados. O runner recria o subdiretório reservado para cada sessão e não semeia conteúdo do host em `~/.claude/hooks/.ccr-launcher/` em sessões.
* **Quem os autora**: o plano de controle popula os scripts de constantes fixas em sua própria implantação, nunca de entrada por sessão ou de terceiros.
* **O que ainda os governa**: hooks entregues através de `--settings` entram na configuração de hook mesclada ordinária, não na camada gerenciada, então suas configurações gerenciadas ainda se aplicam. `disableAllHooks` os desabilita, e eles não estão entre as categorias que [`allowManagedHooksOnly`](/docs/pt/settings-reference#allowmanagedhooksonly) mantém carregadas.

<h3 id="repository-committed-permission-rules">
  Repository-committed permission rules
</h3>

Não coloque uma entrada `"Edit"`, `"Write"` ou `"NotebookEdit"` nua em um `permissions.allow` confirmado no repositório. Uma regra de ferramenta de arquivo nua corresponde à ferramenta independentemente do caminho, concedendo escritas em qualquer lugar no host em vez de apenas o workspace, então a guarda de confinamento de escopo de escrita do runner sinaliza a sessão; com [`--confine-repo-settings enforce`](/docs/pt/self-hosted-environments-reference#runner-cli-flags) ela recusa gerar a sessão em vez de registrar e continuar. Consulte a [seção de hardening](/docs/pt/self-hosted-environments-deploy#harden-your-deployment).

Um repositório não precisa de nenhuma regra de ferramenta de arquivo: sessões na nuvem [pré-aprovam edições de arquivo independentemente do modo](/docs/pt/permission-modes#switch-permission-modes). Se você fizer uma regra, escope-a para o workspace, como `"Edit(/**)"`; uma barra inicial única é relativa à raiz do projeto, que é o workspace da sessão. Regras de ferramenta de arquivo nuas são aceitáveis no `settings.json` de nível de host do operador, já que esse arquivo não é confirmado no repositório.

Um `defaultMode` de `auto` é apenas honrado do arquivo de configurações de nível de imagem ou de nível de usuário, então um repositório verificado não pode se conceder auto mode. Para quais modos sessões na nuvem aceitam e a sintaxe de regra completa, consulte [permission modes](/docs/pt/permission-modes).

<h2 id="what’s-next">
  What's next
</h2>

* [Reference](/docs/pt/self-hosted-environments-reference): cada flag CLI, variável de ambiente e métrica
* [Verify session identity](/docs/pt/self-hosted-environments-identity): valide o token de sessão de serviços fora do runner
