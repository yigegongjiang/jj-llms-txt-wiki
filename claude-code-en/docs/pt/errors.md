> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referência de erros

> Procure mensagens de erro de tempo de execução do Claude Code com o que cada uma significa e como corrigi-la.

Esta página lista os erros de tempo de execução que o Claude Code exibe e como se recuperar de cada um, além do que verificar quando as respostas parecem incorretas sem um erro. Para erros de instalação como `command not found` ou falhas de TLS durante a configuração, consulte [Troubleshoot installation and login](/docs/pt/troubleshoot-install).

Esses erros e comandos de recuperação se aplicam em toda a CLI, no [aplicativo Desktop](/docs/pt/desktop) e no [Claude Code na web](/docs/pt/claude-code-on-the-web), já que todos os três envolvem a mesma CLI do Claude Code. Para problemas específicos da superfície, consulte a seção de solução de problemas na página dessa superfície.

<Note>
  O Claude Code chama a API Claude para respostas do modelo, portanto, a maioria dos erros de tempo de execução mapeia para um código de erro de API subjacente. Esta página cobre o que cada erro significa dentro do Claude Code e como se recuperar. Para as definições de código de status HTTP bruto, consulte a [referência de erro da plataforma Claude](https://platform.claude.com/docs/pt/api/errors).
</Note>

<h2 id="find-your-error">
  Encontre seu erro
</h2>

Corresponda a mensagem que você vê em seu terminal a uma seção abaixo.

| Mensagem                                                                                           | Seção                                                                                                                            |
| :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `API Error: 500 Internal server error`                                                             | [Erros de servidor](#api-error-500-internal-server-error)                                                                        |
| `API Error: Repeated 529 Overloaded errors`                                                        | [Erros de servidor](#api-error-repeated-529-overloaded-errors)                                                                   |
| `Request timed out`                                                                                | [Erros de servidor](#request-timed-out), ou [Rede](#unable-to-connect-to-api) se a mensagem mencionar sua conexão com a internet |
| `Server error mid-response. The response above may be incomplete.`                                 | [Erros de servidor](#the-response-above-may-be-incomplete)                                                                       |
| `Connection closed mid-response` / `Response stalled mid-stream`                                   | [Erros de servidor](#the-response-above-may-be-incomplete)                                                                       |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`               | [Erros de servidor](#auto-mode-cannot-determine-the-safety-of-an-action)                                                         |
| `Auto mode could not evaluate this action and is blocking it for safety`                           | [Erros de servidor](#auto-mode-cannot-determine-the-safety-of-an-action)                                                         |
| `Auto mode classifier transcript exceeded context window`                                          | [Erros de servidor](#auto-mode-cannot-determine-the-safety-of-an-action)                                                         |
| `Agent terminated early due to an API error`                                                       | [Erros de servidor](#agent-terminated-early-due-to-an-api-error)                                                                 |
| `You've hit your session limit` / `You've hit your weekly limit`                                   | [Limites de uso](#youve-hit-your-session-limit)                                                                                  |
| `Usage credits required for 1M context`                                                            | [Limites de uso](#usage-credits-required-for-1m-context)                                                                         |
| `Server is temporarily limiting requests`                                                          | [Limites de uso](#server-is-temporarily-limiting-requests)                                                                       |
| `Request rejected (429)`                                                                           | [Limites de uso](#request-rejected-429)                                                                                          |
| `Credit balance is too low`                                                                        | [Limites de uso](#credit-balance-is-too-low)                                                                                     |
| `Not logged in · Please run /login`                                                                | [Autenticação](#not-logged-in)                                                                                                   |
| `Could not resolve authentication method`                                                          | [Autenticação](#could-not-resolve-authentication-method)                                                                         |
| `Invalid API key`                                                                                  | [Autenticação](#invalid-api-key)                                                                                                 |
| `Your apiKeyHelper script is failing`                                                              | [Autenticação](#your-apikeyhelper-script-is-failing)                                                                             |
| `This organization has been disabled`                                                              | [Autenticação](#this-organization-has-been-disabled)                                                                             |
| `Your organization has disabled API key authentication`                                            | [Autenticação](#your-organization-has-disabled-api-key-authentication)                                                           |
| `Your organization has disabled Claude subscription access`                                        | [Autenticação](#your-organization-has-disabled-claude-subscription-access)                                                       |
| `Routines are disabled by your organization's policy`                                              | [Autenticação](#routines-are-disabled-by-your-organizations-policy)                                                              |
| `Remote Control is only available when using Claude via api.anthropic.com`                         | [Autenticação](#remote-control-requires-the-anthropic-api)                                                                       |
| `OAuth token revoked` / `OAuth token has expired`                                                  | [Autenticação](#oauth-token-revoked-or-expired)                                                                                  |
| `Login expired · Please run /login`                                                                | [Autenticação](#login-expired)                                                                                                   |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                         | [Autenticação](#login-expired)                                                                                                   |
| `does not meet scope requirement user:profile`                                                     | [Autenticação](#oauth-scope-requirement)                                                                                         |
| `AWS credentials expired or invalid`                                                               | [Autenticação](#aws-credentials-expired-or-invalid)                                                                              |
| `AWS authentication failed`                                                                        | [Autenticação](#aws-authentication-failed)                                                                                       |
| `AWS default-chain credential resolve timed out`                                                   | [Autenticação](#aws-default-chain-credential-resolve-timed-out)                                                                  |
| `Unable to connect to API`                                                                         | [Rede](#unable-to-connect-to-api)                                                                                                |
| `Waiting for API response · will retry in`                                                         | [Tentativas automáticas](#automatic-retries), ou [Rede](#unable-to-connect-to-api) se persistir                                  |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"` | [Rede](#bedrock-streaming-response-has-an-unexpected-content-type)                                                               |
| `SSL certificate verification failed`                                                              | [Rede](#ssl-certificate-errors)                                                                                                  |
| `SSL certificate error (...)` during login or startup                                              | [Rede](#ssl-certificate-errors)                                                                                                  |
| `403` with `x-deny-reason: host_not_allowed` in a cloud or routine session                         | [Rede](#host-not-allowed-in-a-cloud-session)                                                                                     |
| `Couldn't reconnect to your Remote Control session`                                                | [Rede](#couldnt-reconnect-to-your-remote-control-session)                                                                        |
| `Prompt is too long`                                                                               | [Erros de solicitação](#prompt-is-too-long)                                                                                      |
| `Error during compaction: Conversation too long`                                                   | [Erros de solicitação](#error-during-compaction-conversation-too-long)                                                           |
| `Request too large`                                                                                | [Erros de solicitação](#request-too-large)                                                                                       |
| `Image was too large`                                                                              | [Erros de solicitação](#image-was-too-large)                                                                                     |
| `Unable to resize image`                                                                           | [Erros de solicitação](#unable-to-resize-image)                                                                                  |
| `PDF too large` / `PDF is password protected`                                                      | [Erros de solicitação](#pdf-errors)                                                                                              |
| `Extra inputs are not permitted`                                                                   | [Erros de solicitação](#extra-inputs-are-not-permitted)                                                                          |
| `There's an issue with the selected model`                                                         | [Erros de solicitação](#theres-an-issue-with-the-selected-model)                                                                 |
| `Model ... is not a recognized model id`                                                           | [Erros de solicitação](#model-is-not-a-recognized-model-id)                                                                      |
| `Claude Opus is not available with the Claude Pro plan`                                            | [Erros de solicitação](#claude-opus-is-not-available-with-the-claude-pro-plan)                                                   |
| `Model ... is restricted by your organization's settings`                                          | [Erros de solicitação](#model-is-restricted-by-your-organizations-settings)                                                      |
| `thinking.type.enabled is not supported for this model`                                            | [Erros de solicitação](#thinking-type-enabled-is-not-supported-for-this-model)                                                   |
| `max_tokens must be greater than thinking.budget_tokens`                                           | [Erros de solicitação](#thinking-budget-exceeds-output-limit)                                                                    |
| `API Error: 400 due to tool use concurrency issues`                                                | [Erros de solicitação](#tool-use-or-thinking-block-mismatch)                                                                     |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`      | [Erros de solicitação](#usage-policy-refusal)                                                                                    |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                  | [Erros de solicitação](#safety-measures-flagged-a-cybersecurity-topic)                                                           |
| `Installation was killed before it could finish (exit code 137)`                                   | [Erros de instalação](#installation-was-killed-before-it-could-finish)                                                           |
| `The connection dropped while downloading the update`                                              | [Erros de instalação](#the-connection-dropped-while-downloading-the-update)                                                      |
| `Download timed out: exceeded the total deadline`                                                  | [Erros de instalação](#the-connection-dropped-while-downloading-the-update)                                                      |
| `--bg and --print conflict`                                                                        | [Erros de linha de comando](#command-line-errors)                                                                                |
| `Error: --json-schema is not a valid JSON Schema`                                                  | [Erros de linha de comando](#command-line-errors)                                                                                |
| `Could not import <server>: <reason>`                                                              | [Erros de linha de comando](#could-not-import-a-server-from-claude-desktop)                                                      |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                           | [Erros de linha de comando](#mcp-permission-prompt-tool-not-found)                                                               |
| `Marketplace "<name>" is registered from an untrusted source`                                      | [Erros de plugin](#marketplace-is-registered-from-an-untrusted-source)                                                           |
| `references ${user_config.*} in a shell-form command`                                              | [Erros de plugin](#plugin-command-references-user-config)                                                                        |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                 | [Erros de plugin](#plugin-command-references-user-config)                                                                        |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                | [Erros de plugin](#plugin-command-references-user-config)                                                                        |
| `would be spawned with zero tools — refusing`                                                      | [Erros de ferramenta](#agent-would-be-spawned-with-zero-tools)                                                                   |
| `File is covered by a Read deny rule in your permission settings`                                  | [Erros de ferramenta](#file-is-covered-by-a-read-deny-rule)                                                                      |
| `Can't open MCP settings in a background session`                                                  | [Erros de sessão em segundo plano](#commands-refused-in-a-background-session)                                                    |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                        | [Erros de sessão em segundo plano](#claude_code_process_wrapper-launcher-errors)                                                 |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                | [Avisos de configuração](#workspace-has-not-been-trusted)                                                                        |
| Respostas parecem de qualidade inferior ao usual                                                   | [Qualidade de resposta](#responses-seem-lower-quality-than-usual)                                                                |

<h2 id="automatic-retries">
  Tentativas automáticas
</h2>

O Claude Code tenta novamente falhas transitórias antes de mostrar um erro. Erros de servidor, respostas sobrecarregadas, tempos limite de solicitação, throttles 429 temporários e conexões perdidas são todos repetidos até 10 vezes com backoff exponencial. A partir da v2.1.198, isso cobre conexões que caem no meio de uma resposta antes de qualquer saída visível ter sido transmitida: Claude Code re-emite a solicitação com o mesmo backoff e o turno continua em vez de parar com um erro de conexão. A partir da v2.1.199, throttles 429 temporários que não carregam os cabeçalhos de cota do seu plano também são repetidos quando você está conectado com uma assinatura claude.ai; versões anteriores os repetiam apenas para autenticações de chave de API e Enterprise.

Algumas classes de falha não são repetidas, porque uma tentativa não pode ter sucesso:

* A partir da v2.1.199, uma falha de validação de certificado TLS, como um proxy que inspeciona TLS, um pacote `NODE_EXTRA_CA_CERTS` ausente ou um certificado expirado, falha na primeira tentativa para que a correção apareça imediatamente em vez de após o orçamento de tentativa completo. Consulte [Erros de certificado SSL](#ssl-certificate-errors). Condições TLS transitórias, como um tempo limite de handshake, ainda são repetidas.
* A partir da v2.1.199, um erro de servidor que chega depois que Claude já transmitiu saída visível mantém a resposta parcial e anexa um [aviso de resposta incompleta](#the-response-above-may-be-incomplete) em vez de tentar novamente, já que re-executar a solicitação poderia executar as mesmas chamadas de ferramentas duas vezes. Versões anteriores descartavam a saída parcial e relatavam o turno como um erro.
* Uma [resposta de streaming do Amazon Bedrock com um tipo de conteúdo inesperado](#bedrock-streaming-response-has-an-unexpected-content-type) falha na primeira tentativa, porque o gateway ou proxy reescrevendo a resposta reescreveria a tentativa da mesma forma. Requer Claude Code v2.1.208 ou posterior.

Enquanto tenta novamente, o spinner mostra uma contagem regressiva `Retrying in Ns · attempt x/y` após um rótulo de erro. O rótulo nomeia a razão específica da primeira tentativa para falhas em que você pode agir imediatamente: a rede está inativa, um handshake TLS falhou ou você atingiu um limite de taxa. Para outros erros, ele lê `API error` no início. A partir da v2.1.198, ele muda para a razão específica da terceira tentativa, ou na tentativa final quando `CLAUDE_CODE_MAX_RETRIES` permite menos de três; versões anteriores mudam apenas na tentativa final.

A partir da v2.1.198, a dica de spinner usual é suprimida durante tentativas. Uma vez que a razão do erro é revelada, se a falha for uma sobrecarga 529, a linha abaixo da contagem regressiva também nomeia onde verificar o status do serviço: `status.claude.com` na API Anthropic, ou o host do provedor ou gateway nomeado na mensagem em outras configurações.

Se nenhum dado chegar no fluxo de resposta por 20 segundos enquanto uma solicitação ainda está pendente, o spinner mostra `Waiting for API response · will retry in … · check your network` antes de qualquer tentativa ter começado. A solicitação ainda não falhou: a contagem regressiva é executada até o ponto em que Claude Code interrompe a conexão travada e tenta novamente, portanto o banner desaparece por conta própria assim que os dados retomam ou a tentativa é bem-sucedida. A partir da v2.1.185, o limite é de 20 segundos; versões anteriores mostram o banner após 10 segundos com uma redação diferente. Se reaparecer em cada tentativa, trate-o como um [problema de rede](#unable-to-connect-to-api).

Quando você vê um dos erros nesta página, essas tentativas já foram esgotadas, a menos que pertença a uma classe que não é repetida, como uma falha de validação de certificado. Você pode ajustar o comportamento com estas variáveis de ambiente:

| Variável                                     | Padrão       | Efeito                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :------------------------------------------- | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/pt/env-vars)    | 10           | Número de tentativas de repetição. Limitado a 15 a partir da v2.1.186; a partir da v2.1.199 `CLAUDE_CODE_RETRY_WATCHDOG` aumenta o padrão e remove o limite. Reduza-o para expor falhas mais rapidamente em scripts.                                                                                                                                                                                                                                                                                       |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/pt/env-vars) | não definido | Defina como `1` em sessões autônomas, como trabalhos de CI, para repetir erros de capacidade `429` e `529` indefinidamente em vez de falhar após `CLAUDE_CODE_MAX_RETRIES` tentativas. A partir da v2.1.199, também aumenta a contagem de tentativas padrão para outros erros transitórios, como erros de servidor, tempos limite e conexões perdidas, para 300, aproximadamente três horas de backoff, e remove o limite de 15 em `CLAUDE_CODE_MAX_RETRIES` se você definir essa variável explicitamente. |
| [`API_TIMEOUT_MS`](/docs/pt/env-vars)             | 600000       | Tempo limite por solicitação em milissegundos. Aumente-o para redes lentas ou proxies.                                                                                                                                                                                                                                                                                                                                                                                                                     |

<h2 id="server-errors">
  Erros do servidor
</h2>

Esses erros vêm do provedor de inferência em vez de sua conta ou solicitação. Na API Anthropic, isso significa infraestrutura Anthropic. No Amazon Bedrock, na Agent Platform do Google Cloud, no Microsoft Foundry ou em um gateway personalizado, significa a infraestrutura desse provedor.

<h3 id="api-error-500-internal-server-error">
  API Error: 500 Internal server error
</h3>

Claude Code mostra o código de status e a mensagem de erro da API para qualquer resposta 5xx. O exemplo abaixo mostra uma resposta 500 na API Anthropic:

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

A frase final nomeia onde verificar a saúde do serviço e varia por provedor. As configurações do Amazon Bedrock, da Agent Platform do Google Cloud e do Microsoft Foundry nomeiam a página de status desse provedor. Um `ANTHROPIC_BASE_URL` personalizado nomeia o host do gateway.

Isso indica uma falha inesperada dentro da API. Não é causado pelo seu prompt, configurações ou conta.

**O que fazer:**

* Verifique [status.claude.com](https://status.claude.com) ou a página de status do provedor nomeada na mensagem para incidentes ativos
* Aguarde um minuto e envie sua mensagem novamente. Sua mensagem original ainda está na conversa, então para um prompt longo você pode digitar `try again` em vez de colar tudo novamente.
* Se o erro persistir sem nenhum incidente postado, execute `/feedback` para que Anthropic possa investigar com os detalhes da sua solicitação. Consulte [Report an error](#report-an-error) se `/feedback` não estiver disponível no seu ambiente.

<h3 id="api-error-repeated-529-overloaded-errors">
  API Error: Repeated 529 Overloaded errors
</h3>

A API está temporariamente em capacidade máxima em todos os usuários. Claude Code já tentou novamente várias vezes antes de mostrar esta mensagem:

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

A frase final varia por provedor da mesma forma que o erro 500 acima.

Um 529 não é seu limite de uso e não conta contra sua cota.

**O que fazer:**

* Verifique [status.claude.com](https://status.claude.com) ou a página de status do provedor nomeada na mensagem para avisos de capacidade
* Tente novamente em alguns minutos
* Execute `/model` e mude para um modelo diferente para continuar trabalhando, já que a capacidade é rastreada por modelo. Claude Code o solicita fazer isso quando um modelo está sob carga particularmente alta, por exemplo `Opus is experiencing high load, please use /model to switch to Sonnet`.

<h3 id="request-timed-out">
  Request timed out
</h3>

A API não respondeu antes do prazo de conexão.

```text theme={null}
Request timed out
```

Isso pode acontecer durante períodos de alta carga ou quando o modelo está gerando uma resposta muito grande. O tempo limite de solicitação padrão é de 10 minutos.

**O que fazer:**

* Tente novamente a solicitação
* Para tarefas de longa duração, divida o trabalho em prompts menores
* Se uma rede lenta ou proxy for a causa, aumente `API_TIMEOUT_MS` conforme descrito em [Automatic retries](#automatic-retries)
* Se os tempos limite forem frequentes e sua rede estiver saudável, consulte [Network and connection errors](#network-and-connection-errors) abaixo

<h3 id="the-response-above-may-be-incomplete">
  The response above may be incomplete
</h3>

Uma resposta de streaming falhou depois que Claude já havia produzido saída visível. Reenviar a solicitação pode executar as mesmas chamadas de ferramenta duas vezes, então Claude Code mantém o que já foi transmitido e anexa este aviso em vez de descartar a vez. Qual variante você vê nomeia a causa:

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection closed mid-response. The response above may be incomplete.
API Error: Response stalled mid-stream. The response above may be incomplete.
```

* `Server error mid-response`: um erro de servidor sobrecarregado ou 5xx no meio do stream. Esta variante requer Claude Code v2.1.199 ou posterior; antes disso, esse caso descartava a saída parcial e relatava toda a vez como um erro.
* `Connection closed mid-response`: a conexão foi interrompida.
* `Response stalled mid-stream`: o stream parou de enviar dados.

**O que fazer:**

* Leia a resposta que foi transmitida. Nada foi perdido, mas as frases finais ou chamadas de ferramenta podem estar faltando.
* Responda com `continue` para que Claude continue de onde parou
* Se o mesmo erro aparecer antes de qualquer saída visível, Claude Code tenta novamente a solicitação em vez de finalizá-la. Consulte [Automatic retries](#automatic-retries).

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  Auto mode cannot determine the safety of an action
</h3>

O modelo que [auto mode](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) usa para classificar ações não conseguiu produzir uma decisão, então o auto mode não aprovou a ação automaticamente. A mensagem que você vê depende de por que o classificador falhou.

Leituras, buscas e edições dentro do seu diretório de trabalho ignoram o classificador, então elas continuam funcionando em todos esses casos.

Quando o modelo classificador está sobrecarregado:

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait briefly and then try this action again.
```

**O que fazer:**

* Tente novamente após alguns segundos; Claude vê a mesma mensagem e geralmente tenta novamente por conta própria
* Se as tentativas continuarem falhando, continue com tarefas somente leitura e volte à ação bloqueada mais tarde
* Isso é transitório e não relacionado à [auto mode eligibility](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode); você não precisa alterar as configurações

Quando o classificador retornou uma resposta não analisável:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**O que fazer:**

* Tente novamente a ação; isso geralmente funciona na próxima tentativa
* Execute `claude --debug` e repita a ação para ver a resposta do classificador subjacente no log de depuração

Quando uma verificação de segurança de API separada bloqueou a solicitação do classificador por causa do conteúdo anterior da conversa:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

**O que fazer:**

* Isso não é uma decisão sobre sua ação. O conteúdo já em sua conversa acionou um filtro de segurança na API quando o auto mode enviou a conversa para o classificador
* Tentar novamente não ajudará; o mesmo conteúdo da conversa acionará o filtro novamente
* Mude para um [permission mode](/docs/pt/permission-modes) diferente para que você possa aprovar a ação quando solicitado, ou inicie uma conversa nova sem o conteúdo que acionou

Quando a conversa cresceu além da janela de contexto do classificador:

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

Em uma sessão interativa, o auto mode volta a um prompt de permissão normal para essa ação para que você possa aprová-la ou negá-la manualmente. No [non-interactive mode](/docs/pt/headless) a execução é abortada porque a transcrição só cresce e tentar novamente não pode ter sucesso.

**O que fazer:**

* Aprove ou negue a ação no prompt que aparece
* Execute `/compact` para reduzir o tamanho da conversa para que as ações subsequentes se encaixem novamente na janela do classificador

<h3 id="agent-terminated-early-due-to-an-api-error">
  Agent terminated early due to an API error
</h3>

A solicitação de API de um [subagent](/docs/pt/sub-agents) falhou terminalmente, por exemplo porque um limite de uso foi atingido ou as tentativas de um erro de servidor se esgotaram, então o subagent parou antes de terminar sua tarefa. Esta mensagem requer Claude Code v2.1.199 ou posterior; antes disso, o texto de erro da API era retornado para Claude como se fosse o resultado do subagent.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**O que fazer:**

* Corresponda o detalhe do erro após os dois pontos à sua própria seção nesta página, como [Usage limits](#usage-limits) ou [Server errors](#server-errors), e siga as etapas dessa seção
* Depois que o erro subjacente for resolvido, peça a Claude para tentar novamente a tarefa ou [resume the subagent](/docs/pt/sub-agents#resume-subagents)

Quando uma limitação de taxa, sobrecarga ou erro de servidor interrompe um subagent em primeiro plano que já produziu saída de texto, Claude recebe essa saída parcial marcada como incompleta em vez deste erro. Um subagent cuja única saída foram chamadas de ferramenta também recebe este erro; na v2.1.199 isso retornou um resultado parcial vazio. Consulte [API errors in subagents](/docs/pt/sub-agents#api-errors-in-subagents).

<h2 id="usage-limits">
  Limites de uso
</h2>

Esses erros significam que uma cota vinculada à sua conta ou plano foi atingida. Eles são distintos de [erros de servidor](#server-errors), que afetam todos.

<h3 id="youve-hit-your-session-limit">
  Você atingiu seu limite de sessão
</h3>

Os planos de assinatura incluem uma permissão de uso contínua. Quando ela se esgota, você vê uma dessas mensagens:

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
```

Claude Code bloqueia solicitações adicionais até o horário de reset mostrado na mensagem. Os limites de sessão e semanais são compartilhados entre todos os modelos, portanto mudar de modelo não restaura o acesso. O limite de Opus se aplica apenas a solicitações de Opus, portanto mudar para outro modelo com `/model` mantém você trabalhando.

O uso é contabilizado contra as permissões de sessão e semanais ao mesmo tempo. Uma única rajada de atividade pesada, como um grande fanout de fluxo de trabalho, pode esgotar a permissão semanal antes que a janela de sessão seja resetada.

**O que fazer:**

* Aguarde o horário de reset mostrado no erro
* Para o limite de Opus, execute `/model` e mude para outro modelo para continuar trabalhando
* Execute `/usage` para ver seus limites de plano e quando eles são resetados
* Execute `/usage-credits` para comprar uso adicional em Pro e Max, ou para solicitá-lo ao seu administrador em Team e Enterprise. Consulte [usage credits for paid plans](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) para saber como isso é cobrado.
* Para atualizar seu plano para limites base mais altos, consulte [claude.com/pricing](https://claude.com/pricing)

Para monitorar sua permissão restante antes de atingir o limite, adicione os campos `rate_limits` a uma [custom status line](/docs/pt/statusline#rate-limit-usage), ou no aplicativo Desktop clique no [usage ring](/docs/pt/desktop#check-usage) ao lado do seletor de modelo.

<h3 id="usage-credits-required-for-1m-context">
  Créditos de uso necessários para contexto de 1M
</h3>

O modelo selecionado usa a janela de contexto estendida de 1M tokens, e seu plano inclui apenas através de créditos de uso.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on, or /model to switch to standard context
```

Esta é uma verificação de direito, não um esgotamento de cota. Ela é acionada mesmo quando suas permissões de sessão e semanais têm capacidade restante. Consulte [Extended context](/docs/pt/model-config#extended-context) para saber quais planos incluem contexto de 1M diretamente e quais exigem créditos de uso.

Quando esse erro aparece no meio de uma conversa porque o contexto cresceu além de 200K tokens, Claude Code compacta automaticamente a conversa de volta para o limite de contexto padrão e mantém a sessão nesse limite depois, portanto nenhuma ação é necessária. Em versões anteriores à v2.1.172, o erro se repetia em cada solicitação subsequente, incluindo `/compact`; execute `/clear` nessas versões para recuperar. As etapas abaixo se aplicam quando você selecionou explicitamente um modelo `[1m]`.

**O que fazer:**

* Execute `/model` e selecione a variante sem o sufixo `[1m]` para voltar à janela de contexto padrão
* Execute `/usage-credits` para ativar a cobrança medida para a variante 1M em Pro e Max, ou para solicitá-la ao seu administrador em Team e Enterprise
* Se o erro persistir após `/model`, uma ID de modelo 1M pode estar definida em outro lugar. Consulte [There's an issue with the selected model](#theres-an-issue-with-the-selected-model) para os locais de configuração a verificar em ordem de prioridade.
* Para remover variantes 1M do seletor de modelo completamente, defina [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/pt/env-vars)

<h3 id="server-is-temporarily-limiting-requests">
  O servidor está limitando temporariamente as solicitações
</h3>

A API aplicou um throttle de curta duração que não está relacionado à sua cota de plano.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code diferencia esses dos seus limites de plano pela ausência dos cabeçalhos de cota unificados que uma resposta de limite real carrega. A partir da v2.1.199 isso é [retentado automaticamente](#automatic-retries) com backoff antes de ser mostrado, independentemente de como você se autentica. Em versões anteriores, uma sessão conectada com uma assinatura claude.ai falhou a vez na primeira ocorrência; apenas autenticações de chave de API e Enterprise a retentaram.

**O que fazer:**

* Aguarde um pouco e tente novamente
* Verifique [status.claude.com](https://status.claude.com) se persistir

<h3 id="request-rejected-429">
  Solicitação rejeitada (429)
</h3>

Você atingiu o limite de taxa configurado para sua chave de API, projeto Amazon Bedrock ou projeto Google Cloud.

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

A frase final nomeia onde verificar a saúde do serviço e varia por provedor. Amazon Bedrock, Agent Platform do Google Cloud e configurações Microsoft Foundry nomeiam o status de serviço desse provedor em vez da página de status do Anthropic. Um `ANTHROPIC_BASE_URL` personalizado nomeia o host do gateway.

**O que fazer:**

* Execute `/status` e confirme que a credencial ativa é a que você espera. Um `ANTHROPIC_API_KEY` deslocado em seu ambiente pode rotear solicitações através de uma chave de nível baixo em vez de sua assinatura.
* Verifique seu console de provedor para os limites ativos e solicite um nível mais alto se necessário
* Para chaves de API do Anthropic, consulte a [rate limits reference](https://platform.claude.com/docs/en/api/rate-limits) para saber como os níveis funcionam e como definir limites por workspace
* Reduza a concorrência: diminua [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/pt/env-vars), evite executar muitos subagentes paralelos, ou mude para um modelo menor com `/model` para execuções de script de alto volume

<h3 id="credit-balance-is-too-low">
  Saldo de crédito muito baixo
</h3>

Sua organização Console ficou sem créditos pré-pagos.

```text theme={null}
Credit balance is too low
```

**O que fazer:**

* Adicione créditos em [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing), e considere ativar o auto-reload lá para que o saldo seja recarregado antes de chegar a zero
* Mude para autenticação de assinatura com `/login` se você tiver um plano Pro, Max, Team ou Enterprise
* Defina limites de gastos por workspace no Console para evitar que um único projeto drene o saldo da organização. Consulte [Manage costs effectively](/docs/pt/costs).

<h2 id="authentication-errors">
  Erros de autenticação
</h2>

Esses erros significam que Claude Code não consegue provar sua identidade para a API. Execute `/status` a qualquer momento para ver qual credencial está ativa no momento.

<h3 id="not-logged-in">
  Não conectado
</h3>

Nenhuma credencial válida está disponível para esta sessão.

```text theme={null}
Not logged in · Please run /login
```

**O que fazer:**

* Execute `/login` para autenticar com sua assinatura Claude ou conta Console
* Se você esperava que uma variável de ambiente o autenticasse, confirme que `ANTHROPIC_API_KEY` está definida e exportada no shell onde você iniciou `claude`
* Para CI ou automação onde login interativo não é possível, configure um script [`apiKeyHelper`](/docs/pt/settings#available-settings) que busque uma chave na inicialização
* Consulte [Precedência de autenticação](/docs/pt/authentication#authentication-precedence) para entender qual credencial Claude Code usa quando várias estão presentes

Se você for solicitado a fazer login repetidamente, consulte [Não conectado ou token expirado](/docs/pt/troubleshoot-install#not-logged-in-or-token-expired) para correções de relógio do sistema e Keychain do macOS.

<h3 id="could-not-resolve-authentication-method">
  Não foi possível resolver o método de autenticação
</h3>

A sessão chegou ao cliente da API sem nenhuma credencial. Isso aparece em [sessões em segundo plano](/docs/pt/agent-view), sessões em nuvem e contextos do Agent SDK onde a verificação de login interativo não é executada antes da primeira solicitação.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

Antes da v2.1.174, uma sessão em segundo plano ou em nuvem atribuída a um worker pré-inicializado ocioso poderia falhar dessa forma mesmo quando credenciais válidas foram configuradas. Atualize para recuperar. Nas versões atuais, o erro significa que nenhuma credencial estava disponível para o processo do worker.

**O que fazer:**

* Atualize para v2.1.174 ou posterior se isso aparecer em uma sessão em segundo plano ou em nuvem e suas credenciais já estiverem configuradas
* Confirme que `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` ou suas credenciais do provedor de nuvem estão definidas no ambiente que inicia o worker, não apenas no seu shell interativo
* Para o Agent SDK, consulte [configuração de autenticação](/docs/pt/agent-sdk/overview#get-started)
* Execute `/status` em uma sessão interativa no mesmo ambiente para confirmar qual fonte de credencial é resolvida

<h3 id="invalid-api-key">
  Chave de API inválida
</h3>

A variável de ambiente `ANTHROPIC_API_KEY` ou o script `apiKeyHelper` retornou uma chave que a API rejeitou.

```text theme={null}
Invalid API key · Fix external API key
```

**O que fazer:**

* Verifique se há erros de digitação e confirme que a chave não foi revogada no [Console](https://platform.claude.com/settings/keys)
* Execute `env | grep ANTHROPIC` no mesmo shell. Ferramentas como direnv, plugins de shell dotenv e terminais IDE podem carregar uma chave obsoleta de um arquivo `.env` em seu projeto sem você defini-la explicitamente.
* Desdefina `ANTHROPIC_API_KEY` e execute `/login` para usar autenticação de assinatura
* Se a chave vem de um script [`apiKeyHelper`](/docs/pt/settings#available-settings), execute o script diretamente para confirmar que ele imprime uma chave válida em stdout
* Execute `/status` para confirmar qual fonte de credencial Claude Code está realmente usando

<h3 id="your-apikeyhelper-script-is-failing">
  Seu script apiKeyHelper está falhando
</h3>

O comando configurado na configuração [`apiKeyHelper`](/docs/pt/settings#available-settings) saiu com um erro, expirou ou não imprimiu nada em stdout. Sem uma chave do script, a solicitação chega à API com uma credencial de espaço reservado, e a API a rejeita com `401`.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

Claude Code executa novamente o script e tenta novamente a solicitação até mais duas vezes antes de mostrar esta mensagem, portanto a falha aparece dentro de três tentativas. Antes da v2.1.208, Claude Code gastava o [orçamento de tentativas](#automatic-retries) completo reenviando a solicitação com a credencial de espaço reservado e depois relatava um erro de autenticação `401` genérico em vez da falha do script.

Executar `/login` não ajuda aqui: a saída do helper [tem precedência](/docs/pt/authentication#authentication-precedence) sobre um login salvo enquanto a configuração estiver presente.

**O que fazer:**

* Execute o comando configurado em `apiKeyHelper` diretamente no seu shell para reproduzir a falha
* Se o comando relatar uma sessão expirada, autentique-se novamente com seu provedor de credenciais, por exemplo, fazendo login novamente em seu SSO ou cofre de segredos
* Corrija o comando para que ele imprima a chave em stdout e saia com código 0. Consulte [girar credenciais com apiKeyHelper](/docs/pt/llm-gateway-connect#rotate-credentials-with-apikeyhelper) para uma configuração funcionando.
* Execute `/status` para confirmar que `apiKeyHelper` é a fonte de credencial ativa. Cada vez que o comando falha, seu código de saída e saída de erro aparecem em um painel `Cloud authentication` no terminal.

<h3 id="this-organization-has-been-disabled">
  Esta organização foi desabilitada
</h3>

Uma `ANTHROPIC_API_KEY` obsoleta de uma organização Console desabilitada está substituindo seu login de assinatura.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your other credentials
API Error: 400 ... This organization has been disabled.
```

Variáveis de ambiente têm precedência sobre `/login`, portanto uma chave exportada no seu perfil de shell ou carregada de um arquivo `.env` é usada mesmo quando você tem uma assinatura Pro ou Max funcionando. No modo não interativo (`-p`), a chave é sempre usada quando presente.

**O que fazer:**

* Desdefina `ANTHROPIC_API_KEY` no shell atual e remova-a do seu perfil de shell, depois reinicie `claude`
* Execute `/status` depois para confirmar que a credencial ativa é sua assinatura
* Se nenhuma variável de ambiente estiver definida e o erro persistir, a organização desabilitada é aquela vinculada ao seu `/login`. Entre em contato com o suporte ou faça login com uma conta diferente.

<h3 id="your-organization-has-disabled-api-key-authentication">
  Sua organização desabilitou a autenticação por chave de API
</h3>

Esta mensagem requer Claude Code v2.1.169 ou posterior. O administrador da organização Console desabilitou a autenticação por chave de API, portanto a API rejeita a chave que Claude Code está enviando. A dica de recuperação após o `·` varia dependendo de onde a chave veio:

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

Variáveis de ambiente e `apiKeyHelper` têm precedência sobre `/login`, portanto executar `/login` sozinho não ajuda enquanto qualquer um deles ainda estiver fornecendo uma chave. Consulte [Precedência de autenticação](/docs/pt/authentication#authentication-precedence).

**O que fazer:**

* Se a mensagem mencionar `ANTHROPIC_API_KEY`, desdefina-a no shell atual e remova-a do seu perfil de shell ou arquivo `.env`, depois reinicie `claude`
* Se a mensagem mencionar `apiKeyHelper`, remova a configuração [`apiKeyHelper`](/docs/pt/settings#available-settings) do seu `settings.json`
* Execute `/login` para fazer login com sua conta claude.ai
* Execute `/status` depois para confirmar que a credencial ativa é sua assinatura em vez de uma chave de API
* Se você precisar de autenticação por chave de API para automação, peça ao administrador da sua organização para reabilitá-la no Console

<h3 id="your-organization-has-disabled-claude-subscription-access">
  Sua organização desabilitou o acesso à assinatura Claude
</h3>

Sua organização Claude não permite fazer login no Claude Code com um login de assinatura. Executar `/login` novamente com a mesma conta retorna o mesmo erro.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

Esta é uma configuração de organização no lado do servidor, portanto não pode ser substituída por configurações locais, variáveis de ambiente ou sinalizadores CLI.

O Agent SDK e o modo não interativo `-p` apresentam isso como o código de erro `oauth_org_not_allowed`.

**O que fazer:**

* Peça ao seu administrador para habilitar o acesso ao Claude Code para sua organização
* Autentique-se com uma chave de API do Console em vez de sua assinatura. Consulte [Autenticação do Claude Console](/docs/pt/authentication#claude-console-authentication) para configuração.
* Se você for o administrador e não vir uma opção para habilitar o acesso, entre em contato com [suporte da Anthropic](https://support.claude.com)

<h3 id="routines-are-disabled-by-your-organizations-policy">
  Rotinas são desabilitadas pela política da sua organização
</h3>

Um Proprietário em sua organização Team ou Enterprise desabilitou rotinas no nível da organização. O erro aparece quando você tenta criar ou executar uma rotina, incluindo de `/schedule` e da interface de [Rotinas](/docs/pt/routines) em claude.ai/code.

```text theme={null}
Routines are disabled by your organization's policy.
```

Esta é uma configuração no lado do servidor, portanto não pode ser substituída por configurações locais, variáveis de ambiente ou sinalizadores CLI.

**O que fazer:**

* Peça a um Proprietário em sua organização para habilitar o botão **Routines** em [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Para trabalho agendado único que não requer rotinas no nível da organização, consulte [tarefas agendadas](/docs/pt/scheduled-tasks)

<h3 id="remote-control-requires-the-anthropic-api">
  Remote Control requer a API Anthropic
</h3>

A sessão não está se comunicando com a API Anthropic diretamente, portanto não há backend claude.ai para [Remote Control](/docs/pt/remote-control) emparelhar.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com.
```

Isso aparece em Amazon Bedrock, Agent Platform do Google Cloud e Microsoft Foundry. A partir da v2.1.196, também aparece quando [`ANTHROPIC_BASE_URL`](/docs/pt/env-vars) aponta para um host diferente de `api.anthropic.com`, como um [gateway LLM](/docs/pt/llm-gateway) ou proxy, mesmo quando você faz login com claude.ai.

**O que fazer:**

* Desdefina `ANTHROPIC_BASE_URL` e reinicie a sessão, ou inicie Remote Control de uma sessão que se comunique com a API Anthropic diretamente
* Para esta e as outras mensagens de inicialização do Remote Control, consulte [Solucionar problemas do Remote Control](/docs/pt/remote-control#troubleshooting)

<h3 id="oauth-token-revoked-or-expired">
  Token OAuth revogado ou expirado
</h3>

Seu login salvo não é mais válido. Um token revogado significa que você se desconectou em todos os lugares ou um administrador removeu o acesso; um token expirado significa que a atualização automática falhou no meio da sessão.

Ambas as mensagens relatam uma rejeição que a API retornou para uma solicitação que Claude Code enviou. Quando o login salvo já foi limpo após uma atualização falhada, você vê [Login expirado](#login-expired) em vez disso.

```text theme={null}
OAuth token revoked · Please run /login
OAuth token has expired · Please run /login
API Error: 401 ... authentication_error
```

**O que fazer:**

* Execute `/login` para fazer login novamente
* Se o erro retornar na mesma sessão após autenticar novamente, execute `/logout` primeiro para limpar completamente o token armazenado, depois `/login`
* Para prompts repetidos de login entre inicializações, consulte as verificações de relógio do sistema e Keychain do macOS em [Solução de problemas](/docs/pt/troubleshoot-install#not-logged-in-or-token-expired)
* Para outras falhas, incluindo `403 Forbidden` e problemas de navegador OAuth, consulte [Login e autenticação](/docs/pt/troubleshoot-install#login-and-authentication)

<h3 id="login-expired">
  Login expirado
</h3>

Claude Code tentou renovar seu login salvo claude.ai ou Claude Console e o serviço OAuth rejeitou o token de atualização armazenado, portanto Claude Code limpou as credenciais salvas. Depois disso, cada solicitação para localmente antes de chegar à API, porque apenas `/login` pode criar novas credenciais. Antes da v2.1.206, Claude Code enviava a solicitação mesmo assim com qualquer credencial que permanecesse no ambiente, e cada modelo então falhava com [Há um problema com o modelo selecionado](#theres-an-issue-with-the-selected-model) ou um 401 em vez de um prompt para fazer login.

```text theme={null}
Login expired · Please run /login
```

Em [modo não interativo](/docs/pt/headless) (`-p`) e no [Agent SDK](/docs/pt/agent-sdk/overview), a mensagem lê da seguinte forma, e o código de erro estruturado é `authentication_failed`:

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

Este não é o mesmo estado que [Token OAuth revogado ou expirado](#oauth-token-revoked-or-expired). Essas mensagens relatam um 401 que a API retornou. Claude Code em si produz `Login expired` para um login que já falhou em renovar, portanto não envia nenhuma solicitação.

Sessões autenticadas com uma chave de API, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/pt/env-vars) ou um provedor de terceiros não usam o login salvo e nunca veem esta mensagem.

**O que fazer:**

* Execute `/login` para fazer login novamente. Tentar novamente sem fazer login mostra a mesma mensagem em cada solicitação.
* Em modo não interativo, execute `claude` no mesmo ambiente, conclua `/login`, depois execute novamente seu comando. Para automação que não consegue fazer login interativamente, autentique-se com `ANTHROPIC_API_KEY` ou [gere um token de longa duração com `claude setup-token`](/docs/pt/authentication#generate-a-long-lived-token).
* Se fazer login continuar falhando, consulte [Login e autenticação](/docs/pt/troubleshoot-install#login-and-authentication)

<h3 id="oauth-scope-requirement">
  Requisito de escopo OAuth
</h3>

O token armazenado é anterior a um escopo de permissão que um recurso mais novo precisa. Você vê isso com mais frequência de `/usage` e do indicador de uso da linha de status:

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**O que fazer:**

* Execute `/login` para obter um novo token com os escopos atuais. Você não precisa fazer logout primeiro.

<h3 id="aws-credentials-expired-or-invalid">
  Credenciais AWS expiradas ou inválidas
</h3>

Esta mensagem requer Claude Code v2.1.198 ou posterior e só aparece quando [`awsAuthRefresh`](/docs/pt/amazon-bedrock#advanced-credential-configuration) está definido no seu arquivo de configurações. Seu token de sessão AWS expirou ou foi rejeitado, e a atualização automática que Claude Code já executou não produziu uma credencial que a API aceita. Aparece em um 401 de [Claude Platform on AWS](/docs/pt/claude-platform-on-aws) ou do [endpoint Mantle](/docs/pt/amazon-bedrock#use-the-mantle-endpoint), que é como esses provedores relatam um token de segurança expirado.

A dica de ação no meio nomeia o comando `awsAuthRefresh` do seu arquivo de configurações, portanto varia. A parte estável é o `AWS credentials expired or invalid` inicial:

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

Sem `awsAuthRefresh` configurado, o mesmo 401 mostra a mensagem genérica `Please run /login` em vez disso, que não consegue atualizar credenciais AWS.

**O que fazer:**

* Execute o comando `awsAuthRefresh` nomeado na mensagem, como `aws sso login --profile myprofile`, em outro terminal e conclua o login do navegador, depois tente novamente
* Em uma sessão interativa, execute `/login`, escolha **plataforma de terceiros**, depois selecione **Claude Platform on AWS · refresh credentials** em **Usando plataformas de terceiros** para executar o mesmo comando sem reiniciar Claude Code. Consulte [Configurar credenciais AWS](/docs/pt/claude-platform-on-aws#1-configure-aws-credentials)
* Se o erro se repetir após o comando de atualização ter sucesso, confirme que a identidade é válida fora do Claude Code com `aws sts get-caller-identity` no mesmo shell e perfil

<h3 id="aws-authentication-failed">
  Falha na autenticação AWS
</h3>

Esta mensagem requer Claude Code v2.1.198 ou posterior e só aparece quando [`awsAuthRefresh`](/docs/pt/amazon-bedrock#advanced-credential-configuration) está definido no seu arquivo de configurações. Seu provedor AWS retornou um 403, ou [Amazon Bedrock](/docs/pt/amazon-bedrock) retornou um 401.

Claude Code não consegue dizer qual causa você atingiu. Amazon Bedrock relata um token de segurança expirado como um 403, mas um 403 também é como ele relata uma negação de autorização, como um `AccessDeniedException` de uma permissão IAM ausente ou um modelo que não está habilitado para sua conta.

Um 401 do Amazon Bedrock também chega aqui em vez de em [Credenciais AWS expiradas ou inválidas](#aws-credentials-expired-or-invalid), porque Amazon Bedrock não relata um token expirado como um 401. Um 401 desse endpoint normalmente vem de algo mais no caminho da solicitação, como um proxy corporativo.

Uma atualização de credencial corrige um token expirado e não consegue corrigir as outras causas, portanto a mensagem oferece ambas:

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

A dica de ação no meio nomeia o comando `awsAuthRefresh` do seu arquivo de configurações, portanto varia. A parte estável é o `AWS authentication failed` inicial.

**O que fazer:**

* Execute o comando `awsAuthRefresh` nomeado na mensagem, ou `aws sso login`, caso uma credencial expirada seja a causa
* Se suas credenciais estão atuais, confirme que as permissões IAM em [Configuração IAM](/docs/pt/amazon-bedrock#iam-configuration) estão anexadas à identidade que você está usando e que o modelo selecionado está habilitado para sua conta e região
* Execute `aws sts get-caller-identity` para confirmar qual identidade suas solicitações usam; um `AWS_PROFILE` obsoleto ou perfil padrão é uma causa comum de incompatibilidade de permissão

<h3 id="aws-default-chain-credential-resolve-timed-out">
  Resolução de credencial da cadeia padrão AWS expirou
</h3>

O provedor de credencial padrão AWS não produziu credenciais dentro de 60 segundos, portanto Claude Code parou a resolução e falhou a solicitação. A falha é resolução de credencial local: a solicitação nunca chegou a [Amazon Bedrock](/docs/pt/amazon-bedrock), [Claude Platform on AWS](/docs/pt/claude-platform-on-aws) ou ao [endpoint Mantle](/docs/pt/amazon-bedrock#use-the-mantle-endpoint). Claude Code limpa seu [cache de credenciais](/docs/pt/amazon-bedrock#credential-caching-and-resolution-timeout) e tenta novamente antes desta mensagem de erro aparecer, portanto no momento em que você a vê a cadeia travou em tentativas repetidas.

```text theme={null}
API Error: AWS default-chain credential resolve timed out
```

As causas comuns são um comando `credential_process` no seu perfil AWS que aguarda entrada que não consegue receber, e um contêiner ou VM cujo serviço de metadados de instância (IMDS) nunca responde à sonda da cadeia. Antes da v2.1.207, uma cadeia travada deixava a solicitação aguardando indefinidamente em vez de falhar com esta mensagem.

**O que fazer:**

* Execute `aws sts get-caller-identity` no mesmo shell com o mesmo `AWS_PROFILE`. Se também travar, corrija o perfil; um comando `credential_process` que solicita interativamente é uma causa comum.
* Conclua a etapa de login antes de iniciar Claude Code, por exemplo `aws sso login --profile myprofile`, para que a cadeia seja resolvida do cache SSO local em vez de aguardar um fluxo de navegador
* Se sua cadeia executa um login interativo que legitimamente precisa de mais de 60 segundos, como SSO com MFA através de um wrapper como `aws-vault`, aumente o limite em milissegundos com [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/pt/env-vars)

<h2 id="network-and-connection-errors">
  Erros de rede e conexão
</h2>

Esses erros significam que uma solicitação de rede do Claude Code falhou ao atingir seu destino, ou algo entre Claude Code e a API alterou a resposta no caminho de volta. Geralmente originam-se em sua rede local, proxy ou firewall, ou na política de rede do ambiente em nuvem.

<h3 id="unable-to-connect-to-api">
  Não é possível conectar à API
</h3>

A conexão TCP com a API falhou ou nunca foi concluída.

```text theme={null}
Unable to connect to API. Check your internet connection
Unable to connect to API (ECONNREFUSED)
Unable to connect to API (ECONNRESET)
Unable to connect to API (ETIMEDOUT)
fetch failed
Request timed out. Check your internet connection and proxy settings
```

As causas comuns incluem falta de acesso à internet, uma VPN que bloqueia `api.anthropic.com`, ou um proxy corporativo necessário que não está configurado.

**O que fazer:**

* Confirme que você pode alcançar o host da API a partir do mesmo shell executando `curl -I https://api.anthropic.com`. No Windows PowerShell, use `curl.exe -I https://api.anthropic.com` para que o alias `Invoke-WebRequest` integrado não seja usado.
* Se você estiver atrás de um proxy corporativo, defina `HTTPS_PROXY` antes de iniciar Claude Code e consulte [Configuração de rede](/docs/pt/network-config)
* Se você rotear através de um gateway LLM ou relay, defina [`ANTHROPIC_BASE_URL`](/docs/pt/env-vars) para seu endereço. Consulte [Conectar Claude Code a um gateway LLM](/docs/pt/llm-gateway-connect) para configuração.
* Certifique-se de que seu firewall permite os hosts listados em [Requisitos de acesso à rede](/docs/pt/network-config#network-access-requirements)
* Falhas intermitentes são [retentadas automaticamente](#automatic-retries); falhas persistentes apontam para um problema de rede local

Se `curl` funcionar mas Claude Code ainda falhar, a causa geralmente é algo entre o runtime e a rede em vez da rede em si:

* No Linux e WSL, verifique `/etc/resolv.conf` para um nameserver inacessível. WSL em particular pode herdar um resolver quebrado do host.
* No macOS, um cliente VPN que foi desconectado ou desinstalado pode deixar uma interface de túnel ou regra de roteamento para trás. Verifique `ifconfig` para interfaces `utun` obsoletas e remova a extensão de rede da VPN em Configurações do Sistema.
* Docker Desktop e runtimes de contêiner similares podem interceptar tráfego de saída. Saia deles e tente novamente para descartar isso.

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  Resposta de streaming do Bedrock tem um content-type inesperado
</h3>

Um gateway ou proxy entre Claude Code e [Amazon Bedrock](/docs/pt/amazon-bedrock) está transformando o corpo da resposta de streaming ou seu cabeçalho `Content-Type`. Amazon Bedrock transmite respostas como `application/vnd.amazon.eventstream`, e Claude Code rejeita uma resposta de streaming bem-sucedida que relata um content-type diferente em vez de decodificar um corpo que não consegue ler. A solicitação não é retentada.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

Antes da v2.1.208, a mesma configuração incorreta aparecia como `API Error: Truncated event message received` após toda a resposta ter sido armazenada em buffer.

**O que fazer:**

* Configure o gateway para passar o corpo da resposta `InvokeModelWithResponseStream` e seu cabeçalho `Content-Type` sem modificações. Um intermediário que re-emite o stream como server-sent events é uma causa comum.
* Se o gateway reescrever apenas o cabeçalho e passar o corpo binário intacto, defina [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/pt/env-vars) para pular a verificação até que o gateway seja corrigido. Consulte [Erros de streaming atrás de um gateway ou proxy](/docs/pt/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

<h3 id="ssl-certificate-errors">
  Erros de certificado SSL
</h3>

Um proxy ou dispositivo de segurança em sua rede está interceptando tráfego TLS com seu próprio certificado, e Claude Code não confia nele.

```text theme={null}
Unable to connect to API: SSL certificate verification failed. Check your proxy or corporate SSL certificates
Unable to connect to API: Self-signed certificate detected
```

A partir da v2.1.199, uma falha de validação de certificado não é retentada, portanto esse erro aparece na primeira tentativa em vez de após o [orçamento de retry](#automatic-retries) completo. Versões anteriores gastavam alguns minutos retentando antes de mostrá-lo. Condições TLS transitórias, como um timeout de handshake, ainda são retentadas.

Durante `/login` e a verificação de conectividade de inicialização, a mesma falha é relatada com o código OpenSSL e a correção inline:

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

**O que fazer:**

* Exporte o pacote CA da sua organização e aponte Claude Code para ele com `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem`
* Consulte [Configuração de rede](/docs/pt/network-config#custom-ca-certificates) para instruções de configuração completa
* Não defina `NODE_TLS_REJECT_UNAUTHORIZED=0`, que desabilita a validação de certificado inteiramente

<h3 id="host-not-allowed-in-a-cloud-session">
  Host não permitido em uma sessão em nuvem
</h3>

Uma solicitação HTTP de saída de uma sessão em nuvem ou rotina foi bloqueada pela política de rede do ambiente.

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

Você também pode ver um certificado TLS que não corresponde ao certificado real do destino. O ambiente em nuvem roteia o tráfego de saída através de um proxy que aplica a política de rede, portanto um certificado incompatível significa que o proxy encerrou a conexão, não o destino.

Isso não é um problema de rede do lado do cliente. Sessões em nuvem e [rotinas](/docs/pt/routines) são executadas dentro de um ambiente em sandbox cuja tráfego de saída é filtrado para a lista de permissões do ambiente. O ambiente **Default** usa acesso **Trusted**, que permite a [lista de permissões padrão](/docs/pt/claude-code-on-the-web#default-allowed-domains) de registros de pacotes, APIs de provedores em nuvem, registros de contêiner e domínios de desenvolvimento comuns, mas bloqueia tudo o mais.

**O que fazer:**

* Abra a rotina para edição ou inicie uma sessão em nuvem. Selecione o ícone de nuvem mostrando o nome do seu ambiente, como **Default**, para abrir o seletor. Passe o mouse sobre seu ambiente e clique no ícone de configurações.
* Na caixa de diálogo **Update cloud environment**, altere **Network access** de **Trusted** para **Custom**, depois adicione o domínio bloqueado a **Allowed domains**. Digite um domínio por linha. Marque **Also include default list of common package managers** para manter a [lista de permissões padrão](/docs/pt/claude-code-on-the-web#default-allowed-domains) junto com seus domínios personalizados. Selecione **Full** em vez disso se quiser acesso irrestrito.
* Clique em **Save changes**. A próxima execução usa a lista de permissões atualizada.

Consulte [Network access](/docs/pt/claude-code-on-the-web#network-access) para níveis de acesso e a lista de permissões padrão. Sessões locais de CLI não são afetadas por essa política.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  Não foi possível reconectar à sua sessão de Remote Control
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

Retomar com `claude --resume` ou `claude --continue` reconecta à sessão [Remote Control](/docs/pt/remote-control) registrada nessa conversa. Esta mensagem significa que a reconexão falhou por um motivo que pode ser temporário, como uma interrupção de rede ou um erro de servidor, portanto Claude Code não pode confirmar se a sessão remota ainda existe. Sua sessão local continua funcionando sem Remote Control.

**O que fazer:**

* Execute `/remote-control` para tentar novamente a conexão
* Inicie Claude Code sem `--resume` para criar uma nova sessão de Remote Control
* Para outras mensagens de inicialização do Remote Control, consulte [Troubleshoot Remote Control](/docs/pt/remote-control#troubleshooting)

Você não verá esta mensagem quando o servidor confirmar que a sessão anterior não existe mais; Claude Code cria uma nova nesse caso. Antes da v2.1.200, qualquer falha de reconexão criava uma nova sessão de Remote Control, o que deixava sessões extras na lista de sessões em claude.ai/code.

<h2 id="request-errors">
  Erros de solicitação
</h2>

Esses erros estão relacionados ao conteúdo da sua solicitação. A maioria retorna da API após ela rejeitar a solicitação; alguns são produzidos localmente pelo Claude Code antes de qualquer solicitação ser enviada.

<h3 id="prompt-is-too-long">
  Prompt é muito longo
</h3>

A conversa mais os arquivos anexados excedem a janela de contexto do modelo.

```text theme={null}
Prompt is too long
```

**O que fazer:**

* Execute `/compact` para resumir turnos anteriores e liberar espaço, ou `/clear` para começar do zero
* Execute `/context` para ver um detalhamento do que está consumindo a janela: prompt do sistema, ferramentas, arquivos de memória e mensagens
* Desabilite servidores MCP que você não está usando com `/mcp disable <name>` para remover suas definições de ferramentas do contexto
* Reduza arquivos de memória `CLAUDE.md` grandes, ou mova instruções para [regras com escopo de caminho](/docs/pt/memory#path-specific-rules) que carregam apenas quando relevante
* Suagentes herdam todas as definições de ferramentas MCP da sessão pai, o que pode preencher sua janela de contexto antes do primeiro turno. Desabilite servidores MCP que você não está usando antes de gerar suagentes.
* Auto-compact está ativado por padrão e normalmente previne esse erro. Se você definiu [`DISABLE_AUTO_COMPACT`](/docs/pt/env-vars), reabilite-o ou execute `/compact` manualmente antes da janela ficar cheia.

Veja [Explore the context window](/docs/pt/context-window) para uma visualização interativa de como o contexto se preenche.

<h3 id="error-during-compaction-conversation-too-long">
  Erro durante compactação: Conversa muito longa
</h3>

`/compact` em si falhou porque não há contexto livre suficiente para manter o resumo que produz.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

Isso pode acontecer quando a janela já está cheia no momento em que auto-compact é acionado, ou quando você executa `/compact` após ver `Prompt is too long`.

**O que fazer:**

* Pressione Esc duas vezes para abrir a lista de mensagens e voltar vários turnos. Isso remove as mensagens mais recentes do contexto. Depois execute `/compact` novamente.
* Se voltar não liberar espaço suficiente, execute `/clear` para iniciar uma sessão nova. Sua conversa anterior é preservada e pode ser reabierta com `/resume`.

<h3 id="request-too-large">
  Solicitação muito grande
</h3>

O corpo da solicitação bruta excedeu o limite de bytes da API antes da tokenização, geralmente por causa de um arquivo grande colado ou anexado.

```text theme={null}
Request too large (max 30 MB). Double press esc to go back and remove or shrink the attached content.
```

Este é um limite de tamanho na solicitação HTTP, separado do [limite de janela de contexto](#prompt-is-too-long).

**O que fazer:**

* Pressione Esc duas vezes e volte passado o turno que adicionou o conteúdo superdimensionado
* Referencie arquivos grandes por caminho em vez de colar seu conteúdo, para que Claude possa lê-los em pedaços
* Para imagens, veja [Image was too large](#image-was-too-large) abaixo

<h3 id="image-was-too-large">
  Imagem era muito grande
</h3>

Uma imagem colada ou anexada excede os limites de tamanho ou dimensão da API.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code substitui a imagem não processável por um espaço reservado de texto e tenta novamente, para que as mensagens subsequentes tenham sucesso. Em versões anteriores a 2.1.142, uma imagem colada poderia permanecer na conversa e repetir o mesmo erro em cada mensagem subsequente. Para recuperar nessas versões, pressione Esc duas vezes e volte passado o turno onde a imagem foi adicionada.

**O que fazer:**

* Redimensione a imagem antes de colar. A API aceita imagens de até 8000 pixels na borda mais longa para uma única imagem, ou 2000 pixels quando muitas imagens estão em contexto.
* Faça uma captura de tela mais apertada da região relevante em vez da tela inteira

<h3 id="unable-to-resize-image">
  Não foi possível redimensionar a imagem
</h3>

Claude Code não conseguiu reduzir uma imagem anexada antes de enviá-la para a API.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
```

Claude Code normalmente redimensiona imagens grandes automaticamente. Esses erros significam que o processador de imagem nativo falhou ao carregar ou retornou um erro, então a imagem não pôde ser redimensionada para caber dentro dos limites da API.

**O que fazer:**

* Se a mensagem pedir para você converter a imagem, converta-a para PNG, JPEG, GIF ou WebP e anexe-a novamente. Claude Code pode verificar dimensões para esses formatos sem o processador de imagem.
* Se a mensagem relatar um limite de dimensão ou tamanho, redimensione ou recomprima a imagem abaixo desse limite antes de anexar.

<h3 id="pdf-errors">
  Erros de PDF
</h3>

O PDF que você anexou não pôde ser processado.

```text theme={null}
PDF too large (max 100 pages, 32 MB). Try splitting it or extracting text first.
PDF is password protected. Try removing protection or extracting text first.
The PDF file was not valid. Try converting to a different format first.
```

**O que fazer:**

* Para PDFs superdimensionados, peça ao Claude para ler um intervalo de páginas com a ferramenta Read em vez de anexar o arquivo inteiro, ou extraia texto com uma ferramenta como `pdftotext` e referencie o arquivo de saída por caminho
* Para PDFs protegidos ou inválidos, remova a senha ou re-exporte o arquivo de seu aplicativo de origem, depois tente novamente

<h3 id="extra-inputs-are-not-permitted">
  Entradas extras não são permitidas
</h3>

Um proxy ou gateway LLM entre Claude Code e a API removeu o cabeçalho de solicitação `anthropic-beta`, então a API rejeitou campos que dependem dele.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Extra inputs are not permitted ... tools.0.custom.input_examples
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code envia campos somente beta como `context_management`, `effort` e `input_examples` de ferramentas junto com um cabeçalho `anthropic-beta` que os habilita. Quando um gateway encaminha o corpo mas remove o cabeçalho, a API vê campos que não reconhece.

**O que fazer:**

* Configure seu gateway para encaminhar o cabeçalho `anthropic-beta`. Veja [feature pass-through](/docs/pt/llm-gateway-protocol#feature-pass-through) para o que os gateways devem encaminhar.
* Como alternativa, defina [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/pt/env-vars) antes de iniciar. Isso desabilita recursos que exigem o cabeçalho beta para que as solicitações tenham sucesso através de um gateway que não pode encaminhá-lo.

<h3 id="theres-an-issue-with-the-selected-model">
  Há um problema com o modelo selecionado
</h3>

O nome do modelo configurado não foi reconhecido ou sua conta não tem acesso a ele. A partir de v2.1.160, a dica à direita, mostrada aqui em sua forma interativa, varia por superfície.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**O que fazer:**

* **CLI interativo**: execute `/model` para escolher entre modelos disponíveis para sua conta.
* **Modo não interativo (`-p`)**: passe `--model` com um alias ou ID válido, ou defina [`ANTHROPIC_MODEL`](/docs/pt/env-vars). O texto de erro mostra `Run --model` nesta superfície.
* **Agent SDK**: o texto de erro omite a dica porque o modelo é definido programaticamente. Defina [`model` em `Options`](/docs/pt/agent-sdk/typescript#options) em TypeScript ou [`ClaudeAgentOptions(model=...)`](/docs/pt/agent-sdk/python#claudeagentoptions) em Python, e trate o erro estruturado `model_not_found` para exibir sua própria tentativa ou seletor de modelo.
* Use um alias como `sonnet` ou `opus` em vez de um ID versionado completo. Os aliases resolvem para um padrão mantido para que não fiquem obsoletos. Veja [Model configuration](/docs/pt/model-config).
* Se o modelo errado continuar voltando na CLI, um ID obsoleto está definido em algum lugar. Verifique em [ordem de prioridade](/docs/pt/model-config#setting-your-model): a flag `--model`, a variável de ambiente `ANTHROPIC_MODEL`, depois o campo `model` em `.claude/settings.local.json`, o `.claude/settings.json` do seu projeto e `~/.claude/settings.json`. Remova o valor obsoleto e Claude Code volta para o padrão da sua conta.
* Claude Code relata um login claude.ai expirado como [Login expired](#login-expired), não como este erro. Antes de v2.1.206, um login expirado que não podia mais ser atualizado falhava em cada modelo com este erro; execute `/login` se você vir isso em uma versão mais antiga.
* Para implantações do Agent Platform do Google Cloud, veja [Troubleshooting do Agent Platform do Google Cloud](/docs/pt/google-vertex-ai#troubleshooting).

<h3 id="model-is-not-a-recognized-model-id">
  Modelo não é um ID de modelo reconhecido
</h3>

A string de modelo que você passou para uma mudança de modelo não é um alias de modelo, um ID de modelo que esta versão do Claude Code conhece, ou um ID que começa com `claude-`. As causas usuais são um erro de digitação no ID, um nome de exibição como `Sonnet 5` onde o ID `claude-sonnet-5` é esperado, ou um alias que apenas versões mais recentes do Claude Code reconhecem. Claude Code rejeita a mudança imediatamente. Antes de v2.1.200, Claude Code salvava a string e falhava na próxima solicitação com [Há um problema com o modelo selecionado](#theres-an-issue-with-the-selected-model).

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

A dica à direita nomeia o alias ou ID de modelo mais próximo. Quando nada é próximo o suficiente, lê `Run /model to see available models.` em vez disso.

Claude Code produz esse erro localmente no momento em que a mudança é solicitada, antes de qualquer solicitação de API ser feita. Aplica-se quando um modelo é definido através do método [Agent SDK](/docs/pt/agent-sdk/typescript) `setModel()` ou por um aplicativo como o [Desktop app](/docs/pt/desktop) que executa o CLI do Claude Code para você.

**O que fazer:**

* Execute `/model` sem argumento para abrir o seletor e escolher entre os modelos disponíveis para sua conta, depois passe o alias ou ID mostrado lá
* Se você usou um alias que uma versão mais recente do Claude Code suporta, execute `claude update`. Um ID completo que começa com `claude-` passa nesta verificação mesmo quando o modelo é mais recente que sua versão do Claude Code, então atualizar não é necessário para esses.
* Um modelo salvo antes de v2.1.200 não é reparado por esta verificação. Se um valor obsoleto continuar voltando, remova-o dos locais listados em [Há um problema com o modelo selecionado](#theres-an-issue-with-the-selected-model).
* A verificação é executada apenas na API Anthropic. No Amazon Bedrock, Agent Platform do Google Cloud, Microsoft Foundry, [Claude Platform on AWS](/docs/pt/claude-platform-on-aws) e atrás de um [LLM gateway](/docs/pt/llm-gateway) ou um `ANTHROPIC_BASE_URL` customizado, seu provedor ou gateway define os nomes dos modelos, então Claude Code aceita qualquer string e a passa.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus não está disponível com o plano Claude Pro
</h3>

Seu plano de assinatura ativo não inclui o modelo que você selecionou.

```text theme={null}
Claude Opus is not available with the Claude Pro plan · Select a different model in /model
```

**O que fazer:**

* Execute `/model` e selecione um modelo que seu plano inclui
* Se você atualizou seu plano recentemente e ainda vê isso, execute `/logout` depois `/login`. O token armazenado reflete seu plano no momento em que você se conectou, então atualizar na web não entra em vigor em uma sessão existente até que você se autentique novamente.
* Veja [claude.com/pricing](https://claude.com/pricing) para quais modelos cada plano inclui

<h3 id="model-is-restricted-by-your-organizations-settings">
  Modelo é restringido pelas configurações da sua organização
</h3>

Seu administrador de organização desabilitou este modelo no console de administração claude.ai, ou ele é excluído por uma lista de permissões [`availableModels`](/docs/pt/model-config#restrict-model-selection) em configurações gerenciadas. Quando o modelo restringido foi definido com `--model`, `ANTHROPIC_MODEL` ou a configuração `model`, Claude Code substitui um modelo permitido e continua. Digitar `/model <name>` para um modelo restringido é rejeitado com `Run /model to choose a different model.` e a sessão mantém seu modelo atual.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Claude Code trata um alias de família de modelo, um de `opus`, `sonnet`, `haiku` ou `fable`, como uma solicitação para essa família em vez de sua versão mais recente. Na API Anthropic e em [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), um alias de família restringido resolve para a versão mais recente da família que sua organização e a lista de permissões `availableModels` permitem, e o aviso de substituição nomeia essa versão. Claude Code rejeita `/model <alias>` apenas quando cada versão da família é restringida. Antes de v2.1.205, um alias de família era substituído ou rejeitado com base em sua versão mais recente sozinha, mesmo quando uma versão mais antiga da mesma família era permitida.

**O que fazer:**

* Execute `/model` para escolher entre os modelos que sua organização permite. Modelos restritos estão ocultos do seletor.
* Se o modelo restringido foi definido em `--model`, `ANTHROPIC_MODEL` ou o campo `model` de um arquivo de configurações, remova ou atualize esse valor para que o aviso não recorra em cada inicialização
* Se você precisa de acesso ao modelo restringido, peça ao administrador da sua organização para habilitá-lo. Veja [Organization model restrictions](/docs/pt/model-config#organization-model-restrictions).

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled não é suportado para este modelo
</h3>

Sua versão do Claude Code é mais antiga que o mínimo para Sonnet 5, Opus 4.8 ou Opus 4.7. O CLI enviou uma configuração de pensamento que o modelo não aceita mais.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**O que fazer:**

* Execute `claude update` e reinicie Claude Code. Opus 4.7 precisa de v2.1.111 ou posterior. Opus 4.8 precisa de v2.1.154 ou posterior. Sonnet 5 precisa de v2.1.197 ou posterior
* Se você não conseguir atualizar, execute `/model` e selecione Opus 4.6 ou Sonnet 4.6 em vez disso
* Se você encontrar isso no [Agent SDK](/docs/pt/agent-sdk/overview), atualize o pacote SDK em vez disso. Opus 4.8 precisa do TypeScript SDK v0.3.154 ou posterior e do Python SDK v0.2.88 ou posterior. Sonnet 5 precisa do TypeScript SDK v0.3.197 ou posterior

<h3 id="thinking-budget-exceeds-output-limit">
  Orçamento de pensamento excede limite de saída
</h3>

O orçamento de pensamento estendido configurado excede o comprimento máximo de resposta, então não há espaço deixado para a resposta real.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code ajusta esses valores automaticamente na API Anthropic. Você normalmente vê esse erro no Amazon Bedrock ou Agent Platform do Google Cloud quando [`MAX_THINKING_TOKENS`](/docs/pt/env-vars) é definido mais alto que o limite de saída do provedor, ou quando o modo de plano aumenta o orçamento de pensamento.

**O que fazer:**

* Diminua `MAX_THINKING_TOKENS`, ou aumente [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/pt/env-vars) acima do orçamento de pensamento
* Veja [Extended thinking](/docs/pt/model-config#extended-thinking) para como o orçamento interage com o comprimento de saída

<h3 id="tool-use-or-thinking-block-mismatch">
  Incompatibilidade de bloco de uso de ferramenta ou pensamento
</h3>

O histórico de conversa chegou à API em um estado inconsistente, geralmente após uma chamada de ferramenta ser interrompida ou um turno ser editado no meio do fluxo.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

Todas as três variantes significam a mesma coisa: a sequência de blocos `tool_use`, `tool_result` e `thinking` no histórico não corresponde mais ao que a API espera.

**O que fazer:**

* Se você está usando Opus 4.7 ou Opus 4.8, execute `claude update` primeiro. Versões anteriores a v2.1.156 podem acionar esse erro durante o uso normal de ferramentas, e `/rewind` não o limpa.
* Execute `/rewind`, ou pressione Esc duas vezes, para voltar a um checkpoint antes do turno corrompido e continuar de lá. Veja [Checkpointing](/docs/pt/checkpointing) para como os checkpoints são criados e restaurados.

<h3 id="usage-policy-refusal">
  Recusa de Política de Uso
</h3>

A API recusou responder porque o conteúdo na conversa acionou uma verificação de [Política de Uso](https://www.anthropic.com/legal/aup). A mensagem inclui um ID de Solicitação que você pode citar para suporte se acreditar que a recusa está incorreta.

```text theme={null}
API Error: Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.
```

A verificação avalia a conversa completa, não apenas seu prompt mais recente, então enviar uma nova mensagem na mesma sessão geralmente re-aciona a mesma recusa. O mesmo se aplica após sair e reabrir a sessão com `--continue` ou `--resume`, já que a transcrição em disco ainda contém o conteúdo acionador. Em [Amazon Bedrock](/docs/pt/amazon-bedrock), [Agent Platform do Google Cloud](/docs/pt/google-vertex-ai) e [Microsoft Foundry](/docs/pt/microsoft-foundry), esta mensagem também cobre solicitações que as medidas de segurança do modelo sinalizaram como um tópico de cibersegurança. Veja [Safety measures flagged a cybersecurity topic](#safety-measures-flagged-a-cybersecurity-topic).

**O que fazer:**

* Pressione Esc duas vezes ou execute `/rewind` para voltar a um checkpoint antes do turno que acionou a recusa, depois reformule ou tome uma abordagem diferente. Veja [Checkpointing](/docs/pt/checkpointing).
* Se você não conseguir identificar qual turno causou, execute `/clear` para iniciar uma conversa nova no mesmo projeto. Sua conversa anterior é preservada em disco e permanece disponível em `/resume`.
* Em [modo não interativo](/docs/pt/headless) (`-p`), onde rewind não está disponível, tente novamente com um prompt reformulado em uma nova sessão sem `--continue`. As verificações de política variam por modelo, então mudar para um modelo diferente com `--model` também pode resolver a recusa em alguns casos.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  Medidas de segurança sinalizaram um tópico de cibersegurança
</h3>

As medidas de segurança do modelo sinalizaram conteúdo na conversa como um tópico de cibersegurança. A mensagem nomeia o modelo que sinalizou a solicitação:

```text theme={null}
API Error: Opus 4.8 has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude.

If you were not engaging in a cybersecurity topic, please send feedback via /feedback.
```

A mensagem vincula ao [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude), que concede acesso para trabalho legítimo de cibersegurança. A proteção em si é do lado do servidor e antecede v2.1.203; esta versão mudou apenas a redação da mensagem e a página para a qual ela vincula.

O que você vê depende do seu provedor e modo:

* Em [Amazon Bedrock](/docs/pt/amazon-bedrock), [Agent Platform do Google Cloud](/docs/pt/google-vertex-ai) e [Microsoft Foundry](/docs/pt/microsoft-foundry), uma sinalização de cibersegurança produz a mensagem de [Recusa de Política de Uso](#usage-policy-refusal) em vez disso.
* [Modo não interativo](/docs/pt/headless) omite a sentença `/feedback`.

Antes de v2.1.203, a mensagem lia `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` seguida por um link de formulário de isenção.

**O que fazer:**

* Se seu trabalho exigir este conteúdo, solicite acesso através do [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)
* Se sua solicitação não era sobre um tópico de cibersegurança, execute `/feedback` para relatar o falso positivo
* Para continuar trabalhando na mesma sessão, pressione Esc duas vezes ou execute `/rewind` para voltar a um checkpoint antes do turno que acionou a sinalização, depois tome uma abordagem diferente. Veja [Checkpointing](/docs/pt/checkpointing).

<h2 id="installation-errors">
  Erros de instalação
</h2>

Esses erros aparecem durante a instalação ou atualização do Claude Code, a partir do [script de instalação](/docs/pt/setup#install-claude-code), `claude install`, ou `claude update`. Para problemas de `command not found`, PATH, permissão e TLS durante a configuração, consulte [Solucionar problemas de instalação e login](/docs/pt/troubleshoot-install).

<h3 id="installation-was-killed-before-it-could-finish">
  A instalação foi interrompida antes de ser concluída
</h3>

O script de instalação relata quando a etapa `claude install` é encerrada por um sinal. No Linux, o código de saída 137 significa que o processo recebeu SIGKILL, e em um host com pouca memória, geralmente é o killer de falta de memória (OOM) do kernel. O script imprime esta explicação e sai com o código 137:

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Para qualquer outro sinal fatal, e para o código de saída 137 no macOS, o script imprime `Installation was killed before it could finish (exit code <N>)` com o código de saída real e omite a explicação de falta de memória. A mensagem vem do script de instalação que macOS e Linux usam, que também cobre instalações dentro do WSL; os scripts de instalação nativos do Windows nunca a imprimem. Antes da v2.1.200, o script saía apenas com a linha `Killed` nua do shell.

**O que fazer:**

* Interrompa outros processos para liberar memória e execute novamente o instalador
* Adicione espaço de swap ou mude para uma instância maior. Consulte [Instalação interrompida em servidores Linux com pouca memória](/docs/pt/troubleshoot-install#install-killed-on-low-memory-linux-servers) para os comandos de arquivo de swap.

<h3 id="the-connection-dropped-while-downloading-the-update">
  A conexão foi interrompida durante o download da atualização
</h3>

A conexão com o servidor de download foi fechada enquanto `claude install`, `claude update`, ou o [atualizador automático](/docs/pt/setup#auto-updates) estava buscando o binário do Claude Code, e as tentativas de repetição não se recuperaram. Claude Code tenta novamente o download quando a conexão cai, a transferência trava ou o arquivo baixado falha em sua soma de verificação, até três tentativas no total. Um erro HTTP concluído, como um 404, não é repetido porque o servidor já respondeu. Antes da v2.1.202, uma única conexão interrompida falhava no download imediatamente com o erro nú `aborted` em vez de tentar novamente.

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

O texto entre parênteses nomeia qual tentativa falhou e o erro de rede subjacente. `claude update` precede a mensagem com `Error: Failed to install native update` no stderr.

Um download que permanece conectado mas não é concluído em 10 minutos falha com `Download timed out: exceeded the total deadline` em vez disso. Claude Code não tenta novamente um download que expirou, porque uma conexão muito lenta para terminar dentro do prazo não terminará em uma tentativa imediata de repetição. As etapas abaixo se aplicam a ambas as mensagens. Antes da v2.1.205, o mesmo prazo de 10 minutos era relatado como o genérico `timeout of 600000ms exceeded` do cliente HTTP.

A causa usual é um proxy ou gateway que fecha uma transferência longa antes de ser concluída. O binário do Claude Code é um download grande, portanto um limite de conexão de proxy que nunca afeta o tráfego normal da API ainda pode interrompê-lo.

**O que fazer:**

* Execute `claude update` novamente. Em uma rede caso contrário saudável, o download geralmente é bem-sucedido na próxima execução. Para a mensagem de tempo limite, execute-a novamente de uma rede mais rápida ou menos limitada.
* Se sua rede exigir um proxy, defina `HTTPS_PROXY` antes de executar o instalador ou `claude update`. Consulte [Verificar conectividade de rede](/docs/pt/troubleshoot-install#check-network-connectivity).
* Se um proxy corporativo continuar fechando a transferência, peça à sua equipe de rede para permitir o download completo de `downloads.claude.ai`. Consulte [Requisitos de acesso à rede](/docs/pt/network-config#network-access-requirements).
* Execute `claude doctor` do seu shell para diagnósticos de instalação

<h2 id="command-line-errors">
  Erros de linha de comando
</h2>

Esses erros vêm do comando `claude` de linha de comando e seus subcomandos. Claude Code os imprime antes de executar seu prompt ou enviar qualquer solicitação de API.

<h3 id="conflict-between-bg-and-print">
  Conflito entre --bg e --print
</h3>

Esta mensagem requer Claude Code v2.1.198 ou posterior. Você combinou `--bg` com `-p` ou `--print` na mesma invocação de `claude`. `--bg` inicia uma [sessão em background](/docs/pt/agent-view#from-your-shell) que você depois anexa com `claude agents`, enquanto `--print` executa [não interativamente](/docs/pt/headless) e nunca inicia a sessão interativa que `claude agents` anexa. Antes da v2.1.198, essa combinação criava silenciosamente um job em background que nunca poderia ser anexado.

```text theme={null}
--bg and --print conflict: --print never starts the interactive session that `claude agents` attaches to, so the job would be unattachable. The prompt is the positional — drop --print: `claude --bg '<task>'`.
```

**O que fazer:**

* Remova `-p` ou `--print`. `--bg` recebe o prompt como seu argumento posicional, então `claude --bg "<task>"` é o comando completo. Veja [Dispatch new agents from your shell](/docs/pt/agent-view#from-your-shell).
* Para executar o prompt não interativamente e imprimir o resultado em vez de criar uma sessão em background, remova `--bg` e execute `claude -p "<task>"`

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  O valor de --json-schema não é um JSON Schema válido
</h3>

O schema que você passou para [`--json-schema`](/docs/pt/cli-reference#cli-flags) no [modo não interativo](/docs/pt/headless#get-structured-output) falhou na compilação do JSON Schema, então `claude` sai com código 1 em vez de executar o prompt. Antes da v2.1.205, um schema inválido produzia saída não estruturada sem erro, e qualquer schema que usasse a palavra-chave `format` era tratado como inválido.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

O texto após o segundo dois-pontos é o diagnóstico do validador e nomeia a palavra-chave ou localização que falhou. Schemas que usam a palavra-chave `format`, como `"format": "email"`, são válidos: Claude Code aceita `format` como uma anotação e não a impõe.

Claude Code executa duas verificações antes da compilação do schema: ele rejeita um valor que não é JSON analisável com `Error: --json-schema is not valid JSON`, e JSON válido que não é um objeto com `Error: --json-schema must be a JSON object`.

**O que fazer:**

* Corrija a parte do schema que o diagnóstico nomeia, depois execute o comando novamente
* Se o diagnóstico for `schema too large`, reduza o aninhamento do schema e a reutilização de `$ref`
* Veja [Get structured output](/docs/pt/headless#get-structured-output) para um schema e comando funcionando

<h3 id="could-not-import-a-server-from-claude-desktop">
  Não foi possível importar um servidor do Claude Desktop
</h3>

Claude Code não conseguiu adicionar um dos servidores que você selecionou em `claude mcp add-from-claude-desktop`. O comando ainda importa os outros servidores selecionados e imprime uma linha por servidor que não conseguiu adicionar. Antes da v2.1.205, o primeiro servidor que falhou parou a importação e nenhum dos servidores selecionados foi adicionado.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

O texto após o nome do servidor é o motivo. O mais comum é a verificação de nome: Claude Desktop permite caracteres em nomes de servidores, como espaços e pontos, que `claude mcp` restringe a letras, números, hífens e sublinhados. Outros motivos incluem uma configuração de servidor que falha na validação e um servidor bloqueado pela [política MCP](/docs/pt/managed-mcp) da sua organização.

**O que fazer:**

* Renomeie o servidor em `claude_desktop_config.json` para usar apenas letras, números, hífens e sublinhados, depois execute `claude mcp add-from-claude-desktop` novamente
* Adicione esse servidor diretamente com `claude mcp add` ou `claude mcp add-json` sob um nome válido. Veja [Import MCP servers from Claude Desktop](/docs/pt/mcp#import-mcp-servers-from-claude-desktop).

<h3 id="mcp-permission-prompt-tool-not-found">
  Ferramenta de prompt de permissão MCP não encontrada
</h3>

A ferramenta que você passou para [`--permission-prompt-tool`](/docs/pt/cli-reference#cli-flags) não estava entre as ferramentas MCP conectadas quando a execução primeiro precisou de uma decisão de permissão, seja porque seu servidor nunca se conectou ou porque nenhum servidor conectado expõe uma ferramenta com esse nome. Claude Code ainda envia seu prompt: a execução [não interativa](/docs/pt/headless) sai com esse erro, e código de saída 1, na primeira chamada de ferramenta que precisa de aprovação, então não produz resposta mesmo que a solicitação tenha sido feita. Antes do primeiro prompt, Claude Code aguarda até o tempo limite de conexão por servidor de 30 segundos definido por [`MCP_TIMEOUT`](/docs/pt/env-vars) para que esse servidor se conecte. Antes da v2.1.206, a inicialização não aguardava o servidor terminar de se conectar, então um servidor que iniciava lentamente mas estava saudável também produzia esse erro.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

A lista após `Available MCP tools:` nomeia as ferramentas MCP que estavam conectadas quando a espera terminou.

**O que fazer:**

* Verifique se o servidor inicia e permanece conectado: execute `claude mcp list` no mesmo diretório e confirme se o servidor está listado como conectado
* Confirme se o nome da ferramenta corresponde ao nome `mcp__<server>__<tool>` que o servidor expõe
* Se o servidor precisar de mais de 30 segundos para iniciar, aumente [`MCP_TIMEOUT`](/docs/pt/env-vars)

<h2 id="plugin-errors">
  Erros de plugin
</h2>

Esses erros vêm da configuração de [plugin](/docs/pt/plugins) e [marketplace](/docs/pt/plugin-marketplaces). Para problemas de plugin que não produzem uma das mensagens nesta página, como uma URL de marketplace que não carrega ou um plugin que é instalado mas não aparece, consulte [Solução de problemas de plugin](/docs/pt/discover-plugins#troubleshooting).

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  Marketplace registrado de uma fonte não confiável
</h3>

O marketplace é registrado sob um nome que é [reservado para marketplaces oficiais da Anthropic](/docs/pt/plugin-marketplaces#marketplace-schema), mas sua fonte registrada não é um repositório GitHub `anthropics`. Claude Code verifica novamente os nomes reservados toda vez que carrega ou atualiza um marketplace, portanto o marketplace e os plugins instalados a partir dele param de carregar. Antes da v2.1.205, o nome era verificado apenas quando o marketplace era adicionado, então uma entrada registrada antes de seu nome ficar reservado continuava carregando.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

**O que fazer:**

* Execute `claude plugin marketplace remove <name>`, depois adicione o marketplace novamente do repositório oficial `github.com/anthropics`
* Se você publicar um marketplace de terceiros que usou o nome antes de ele ficar reservado, renomeie-o e peça aos usuários para adicioná-lo novamente de sua fonte
* Consulte a lista de nomes reservados em [Marketplace schema](/docs/pt/plugin-marketplaces#marketplace-schema)

<h3 id="plugin-command-references-user-config">
  Plugin command references user\_config in a shell command
</h3>

Um hook de plugin, [monitor](/docs/pt/plugins-reference#monitors), ou comando MCP [`headersHelper`](/docs/pt/mcp#use-dynamic-headers-for-custom-authentication) referencia uma [opção de plugin](/docs/pt/plugins-reference#user-configuration) `${user_config.KEY}`, e a string substituída seria passada para um shell. Um valor configurado contendo `$(...)`, backticks ou `;` seria executado como código lá, então Claude Code recusa iniciar o componente em vez de substituir o valor. A verificação é executada no modelo de comando, então o erro aparece mesmo quando nenhum valor está configurado ainda. Antes da v2.1.207, o valor era substituído no comando shell.

A redação depende de qual superfície referenciou a opção. Um hook em forma de shell relata:

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

Um monitor relata:

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

Um MCP `headersHelper` relata:

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**O que fazer:**

* Para um hook, adicione um array `args` para que ele seja executado em [exec form](/docs/pt/hooks#exec-form-and-shell-form), onde cada `${user_config.KEY}` se torna um argumento sem shell no meio. Ou remova a referência e leia a variável de ambiente `$CLAUDE_PLUGIN_OPTION_<KEY>` dentro do script
* Para um monitor, remova a referência e faça o script do monitor ler o valor de um arquivo de configuração
* Para um `headersHelper`, mova `${user_config.KEY}` para o campo `headers` do servidor, que não é analisado por shell, ou leia o valor dentro do script helper

<h2 id="tool-errors">
  Erros de ferramenta
</h2>

Esses erros vêm das ferramentas integradas do Claude recusando uma entrada. Claude corrige a maioria dos erros de ferramenta por conta própria; os dois abaixo precisam de uma mudança sua, porque vêm de uma definição de subagenteou de uma regra de permissão que você controla.

<h3 id="agent-would-be-spawned-with-zero-tools">
  Agent seria gerado com zero ferramentas
</h3>

Nada na [lista de `tools` de um subagente](/docs/pt/sub-agents#supported-frontmatter-fields) foi resolvido para uma ferramenta, então Claude Code recusa iniciar o subagente em vez de iniciar um que não possa agir. A mensagem agrupa as entradas pelo motivo pelo qual não foram resolvidas: não é uma ferramenta reconhecida, uma ferramenta que não está disponível para subagentes, ou reconhecida mas não corresponde a nenhuma ferramenta na sessão atual. Omitir o campo `tools` nunca dispara essa recusa. Um padrão de servidor MCP como `mcp__github__*` não é isento: quando nenhuma ferramenta conectada vem desse servidor, o lançamento é recusado com o padrão no grupo de não correspondência. Antes da v2.1.208, o subagente era lançado sem ferramentas e retornava um resultado vazio ou confuso.

```text theme={null}
Agent 'code-reviewer' seria gerado com zero ferramentas — recusando. Sua lista de ferramentas foi resolvida para nada: não reconhecido [Grpe]. Corrija o frontmatter de ferramentas do agente ou passe um subagent_type diferente.
```

**O que fazer:**

* Corrija cada entrada que o erro nomeia contra as [ferramentas disponíveis para subagentes](/docs/pt/sub-agents#available-tools)
* Remova entradas para ferramentas que a sessão não possui, como ferramentas MCP de um servidor que não está conectado
* Para dar ao subagente todas as ferramentas que o pai tem, delete o campo `tools` em vez de listar ferramentas

<h3 id="file-is-covered-by-a-read-deny-rule">
  Arquivo é coberto por uma regra de negação Read
</h3>

A ferramenta Edit foi chamada em um caminho correspondido por uma [regra de negação `Read`](/docs/pt/permissions#read-and-edit), incluindo criar um novo arquivo nesse caminho. Editar reescreve conteúdo que Claude tem que ser capaz de ler novamente, então a chamada é recusada antes de qualquer acesso ao arquivo. A regra bloqueia apenas a ferramenta Edit: Write e NotebookEdit não são cobertos por regras de negação `Read`. Antes da v2.1.208, apenas uma regra de negação `Edit` bloqueava edições, e uma regra de negação `Read` sozinha não.

```text theme={null}
Arquivo é coberto por uma regra de negação Read em suas configurações de permissão e não pode ser editado.
```

**O que fazer:**

* Se Claude deve ser capaz de editar o arquivo, remova ou restrinja a regra de negação `Read` em `/permissions` ou em [configurações](/docs/pt/settings#permission-settings)
* Se o arquivo deve permanecer intocado, mantenha a regra e adicione uma regra de negação `Edit` para o mesmo caminho para que as ferramentas Write e NotebookEdit também sejam bloqueadas

<h2 id="background-session-errors">
  Erros de sessão em background
</h2>

[Sessões em background](/docs/pt/agent-view) são executadas sem um terminal interativo próprio, portanto comandos que precisam de um se comportam de forma diferente lá. Essas mensagens aparecem na transcrição de uma sessão em background, na visualização do agente ou após anexar.

<h3 id="commands-refused-in-a-background-session">
  Comandos recusados em uma sessão em background
</h3>

Comandos que abrem um diálogo interativo são recusados em uma sessão em background com uma mensagem nomeando um formulário que funciona lá ou dizendo para você executar o comando a partir de um terminal regular. `/install-github-app`, a lista de configurações `/mcp` e as ações de autenticação no menu do servidor MCP são todos recusados dessa forma. Antes da v2.1.208, eles abriam seu diálogo dentro da sessão em background.
Na v2.1.208 apenas, o seletor `/model` também foi recusado em uma sessão em background, e `/upgrade` imprimiu a URL de atualização em vez de abrir um navegador.

A redação nomeia o comando que foi recusado. A lista de configurações `/mcp` relata:

```text theme={null}
Can't open MCP settings in a background session — use `/mcp enable|disable|reconnect <server>` to steer, or run /mcp from an interactive terminal to authenticate.
```

**O que fazer:**

* Use o formulário que a mensagem nomeia, como `/mcp reconnect <server>`, `/mcp enable` ou `/mcp disable`
* Para fluxos de entrada e autorização, execute o comando a partir de uma sessão `claude` regular em um terminal

<h3 id="claude_code_process_wrapper-launcher-errors">
  Erros do launcher CLAUDE\_CODE\_PROCESS\_WRAPPER
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/pt/corporate-launcher) está definido e seu valor não pode ser usado, portanto Claude Code recusa iniciar o processo afetado em vez de executá-lo sem o launcher. Problemas de configuração são relatados com uma mensagem que começa com o nome da variável e declara o motivo, por exemplo:

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

Um launcher que inicia mas sai sem se substituir por Claude Code falha na sessão que estava iniciando, e a linha da sessão na visualização do agente relata que o launcher `must exec, not daemonize`, seguido por qualquer coisa que o launcher imprimiu. Uma sessão que não pode iniciar ou alcançar o serviço em background por causa do launcher relata o problema do launcher como o motivo dentro de `Couldn't reach the background service (...)`.

**O que fazer:**

* Defina a variável para o caminho absoluto de um executável que termina chamando `exec "$@"`. Veja [o contrato do launcher](/docs/pt/corporate-launcher#the-launcher-contract) para o contrato completo
* Verifique `/status`, que mostra o comando de inicialização resolvido em sua entrada Self-exec e avisa quando o serviço em background em execução não corresponde a ele, ou execute `claude daemon status` a partir de um shell
* Após corrigir o valor no bloco `env` de [settings](/docs/pt/corporate-launcher#set-up-the-launcher), reinicie o serviço em background com `claude daemon stop --any` para que o próximo dispatch inicie um envolvido

<h2 id="configuration-warnings">
  Avisos de configuração
</h2>

Claude Code escreve essas mensagens para stderr na inicialização em vez de mostrar um erro na conversa. Elas relatam configuração que foi lida mas não foi aplicada.

<h3 id="workspace-has-not-been-trusted">
  Workspace não foi confiável
</h3>

Claude Code encontrou regras `permissions.allow` ou entradas `permissions.additionalDirectories` no arquivo `.claude/settings.json` ou `.claude/settings.local.json` do projeto e não as aplicou, porque [as regras de permissão do projeto requerem confiança do workspace](/docs/pt/permissions#project-allow-rules-and-workspace-trust). A contagem, o nome da configuração e o arquivo nomeado na mensagem variam com sua configuração. As regras `deny` e `ask` não são afetadas.

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**O que fazer:**

* Execute `claude` no diretório e aceite o diálogo de confiança. O diálogo aparece mesmo quando um diretório pai já é confiável, lista as regras sendo retidas e permite que você recuse e continue trabalhando sem elas. Antes da v2.1.200, nenhum diálogo aparecia nessa situação, então essa etapa não podia ser concluída lá.
* No [modo não interativo](/docs/pt/headless) com `-p` nenhum diálogo é mostrado. Defina a entrada `hasTrustDialogAccepted` em `~/.claude.json` usando a chave `projects` exata que a mensagem imprime.
* Se a mensagem nomear `.claude/settings.local.json` e você iniciou Claude Code fora de um repositório git ou no seu diretório inicial, atualize para v2.1.200 ou posterior. As versões 2.1.196 a 2.1.199 trataram seu próprio `.claude/settings.local.json` como fornecido pelo repositório nesses workspaces. Na v2.1.207 e posterior, atualizar não é suficiente fora de um repositório git se você não confiou na pasta: determinar que uma pasta não está dentro de um repositório executa git, e Claude Code executa essa verificação apenas depois que você aceita o diálogo de confiança, então use a primeira etapa. Seu diretório inicial e qualquer outro [diretório inicial de configuração](/docs/pt/permissions#project-allow-rules-and-workspace-trust) estão isentos e não esperam pelo diálogo. Veja [Regras de permissão do projeto e confiança do workspace](/docs/pt/permissions#project-allow-rules-and-workspace-trust).

<h2 id="responses-seem-lower-quality-than-usual">
  As respostas parecem ter qualidade inferior ao usual
</h2>

Se as respostas do Claude parecerem menos capazes do que você espera, mas nenhum erro for exibido, a causa geralmente é o estado da conversa em vez do modelo em si. Claude Code não muda silenciosamente versões de modelo. Ele pode mudar para um modelo de fallback em três casos específicos:

* Um [`--fallback-model`](/docs/pt/cli-reference#cli-flags) configurado assume o controle após um erro de disponibilidade, apenas para esse turno, com um aviso na transcrição
* Uma verificação de inicialização do Amazon Bedrock ou da Agent Platform do Google Cloud encontra seu modelo padrão indisponível
* [Fallback automático de modelo](/docs/pt/model-config#automatic-model-fallback) no Fable 5 move a sessão para o modelo Opus padrão e mostra um aviso na transcrição

A verificação de seleção de modelo abaixo captura o segundo e terceiro casos; o primeiro aparece como um aviso de transcrição em vez de uma mudança de `/model`. [Configuração de modelo](/docs/pt/model-config) explica quando cada fallback se aplica.

Verifique estes primeiro:

* **Seleção de modelo**: execute `/model` para confirmar que você está no modelo que espera. Uma escolha anterior de `/model` ou uma variável de ambiente `ANTHROPIC_MODEL` pode colocá-lo em um modelo menor do que pretendia.
* **Nível de esforço**: execute `/effort` para verificar o nível de raciocínio atual e aumentá-lo para depuração difícil ou trabalho de design. Os padrões variam por modelo, então verifique antes de assumir que você está abaixo do máximo. Veja [Ajustar nível de esforço](/docs/pt/model-config#adjust-effort-level) para padrões por modelo e o atalho `ultrathink`.
* **Pressão de contexto**: execute `/context` para ver o quão cheio está a janela. Se estiver próximo da capacidade, execute `/compact` em um ponto natural ou `/clear` para começar do zero. Veja [Explorar a janela de contexto](/docs/pt/context-window) para como auto-compact afeta turnos anteriores.
* **Instruções obsoletas**: arquivos `CLAUDE.md` grandes ou desatualizados e definições de ferramentas MCP consomem contexto e podem orientar respostas. A verificação `/doctor` sinaliza arquivos de memória superdimensionados e extensões não utilizadas, e `/context` mostra o uso de tokens de ferramentas MCP. Antes da v2.1.205, `/doctor` abria uma tela de diagnósticos que sinalizava arquivos de memória superdimensionados e definições de subagente.

Quando uma resposta sai errada, retroceder geralmente funciona melhor do que responder com correções. Pressione Esc duas vezes ou execute `/rewind` para voltar antes do turno ruim, depois reformule o prompt com mais especificidades. Corrigir na thread mantém a tentativa errada no contexto, o que pode ancorar respostas posteriores a ela. Veja [Checkpointing](/docs/pt/checkpointing).

Se a qualidade ainda parecer inadequada após verificar o acima, execute `/feedback` e descreva o que você esperava versus o que obteve. O feedback enviado desta forma inclui a transcrição da conversa, que é a forma mais rápida para a Anthropic diagnosticar uma regressão real. Veja [Relatar um erro](#report-an-error) se `/feedback` não estiver disponível em seu ambiente.

Se Claude avisar sobre uma injeção de prompt suspeita, ou recusar uma solicitação por causa de uma injeção suspeita, e o texto que o aviso nomeia for contexto que Claude Code adiciona à conversa automaticamente em vez de conteúdo de arquivo ou web, execute `claude update` e tente novamente. Se o aviso se repetir após atualizar, [relate-o](#report-an-error) em vez de colar o conteúdo sinalizado de volta no prompt. Antes da v2.1.201, Sonnet 5 recusava algumas solicitações da mesma forma.

<h2 id="report-an-error">
  Relatar um erro
</h2>

Para erros de componentes que esta página não cobre, consulte o guia relevante:

* Servidor MCP falhou ao conectar ou autenticar: [MCP](/docs/pt/mcp)
* Script de hook falhou ou bloqueou uma ferramenta: [Debug hooks](/docs/pt/hooks#debug-hooks)
* Permissão negada ou erros do sistema de arquivos durante a instalação: [Solucionar problemas de instalação e login](/docs/pt/troubleshoot-install)

Se um erro não estiver listado aqui ou a correção sugerida não ajudar:

* Execute `/feedback` dentro do Claude Code para enviar a transcrição e uma descrição para a Anthropic. O comando também oferece abrir um problema do GitHub pré-preenchido. O envio para a Anthropic requer [autenticação](/docs/pt/authentication). No Amazon Bedrock, na plataforma de agentes do Google Cloud, no Microsoft Foundry e em outros provedores terceirizados, ou quando nenhuma credencial da Anthropic está configurada, `/feedback` salva um arquivo local que você pode enviar para seu representante de conta da Anthropic.
* Execute `claude doctor` do seu shell para um diagnóstico somente leitura da sua instalação, ou execute o checkup `/doctor` dentro do Claude Code para encontrar e corrigir problemas de configuração
* Verifique [status.claude.com](https://status.claude.com) para incidentes ativos
* Pesquise [problemas existentes](https://github.com/anthropics/claude-code/issues) no GitHub
