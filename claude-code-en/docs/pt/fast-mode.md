> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Acelere respostas com modo rápido

> Obtenha respostas mais rápidas do Opus no Claude Code alternando o modo rápido.

<Note>
  O modo rápido está em [visualização de pesquisa](#research-preview). O recurso, preços e disponibilidade podem mudar com base no feedback.
</Note>

O modo rápido é uma configuração de alta velocidade para Claude Opus, tornando o modelo até 2,5x mais rápido a um custo maior por token. Ative-o com `/fast` quando você precisar de velocidade para trabalho interativo como iteração rápida ou depuração ao vivo, e desative-o quando o custo importa mais do que a latência.

O modo rápido não é um modelo diferente. Ele usa Claude Opus com uma configuração de API diferente que prioriza a velocidade sobre a eficiência de custo. Você obtém qualidade e capacidades idênticas com respostas mais rápidas. O modo rápido é suportado no Opus 5.5, Opus 5 e Opus 4.8. Não está disponível no Sonnet, Haiku ou outros modelos.

O Opus 4.7 não suporta modo rápido, portanto alternar para ele desativa o modo rápido. O modo rápido para Opus 4.7 foi descontinuado em 25 de junho de 2026 e removido em 24 de julho de 2026.

O que você precisa saber:

* Use `/fast` para alternar o modo rápido no CLI do Claude Code. A [extensão VS Code](/docs/pt/vs-code) oferece um comando **Toggle fast mode** quando o modelo selecionado suporta modo rápido. O Claude Code salva essa alternância na sua configuração [`fastMode`](#toggle-fast-mode).
* O preço do modo rápido por MTok de entrada/saída é \$8/\$40 no Opus 5.5 e \$10/\$50 no Opus 5 e Opus 4.8.
* Disponível para usuários do Claude Code em planos de assinatura (Pro/Max/Team/Enterprise) e no Claude Console. Organizações Team e Enterprise precisam que um Owner ative primeiro, e organizações Console precisam ter acesso provisionado primeiro, ambos descritos em [Requisitos](#requirements).
* Para usuários do Claude Code em planos de assinatura (Pro/Max/Team/Enterprise), o modo rápido está disponível apenas via créditos de uso e não está incluído nos limites de taxa de assinatura.

<h2 id="toggle-fast-mode">
  Alternar modo rápido
</h2>

Na CLI, alterne o modo rápido de uma destas formas:

* Execute `/fast`, pressione Space para alternar ativado ou desativado, depois pressione Enter para confirmar
* Defina `"fastMode": true` no seu [arquivo de configurações do usuário](/docs/pt/settings)

Por padrão, o modo rápido que você ativa em uma sessão interativa persiste entre sessões. Você pode configurar o modo rápido para ser redefinido a cada sessão. Consulte [require per-session opt-in](#require-per-session-opt-in) para obter detalhes.

Fora de uma [sessão em nuvem](#use-fast-mode-in-cloud-sessions), no [modo não interativo](/docs/pt/headless) com a flag `-p`, `/fast` funciona apenas em uma sessão iniciada com modo rápido em seu valor [`--settings`](/docs/pt/cli-reference#cli-flags), por exemplo `claude -p --settings '{"fastMode": true}'`; a alternância então se aplica apenas a essa sessão e não é salva como seu padrão. O formulário `-p` requer Claude Code v2.1.205 ou posterior. Em qualquer outro lugar no modo não interativo, o comando relata que o modo rápido não está disponível.

Você pode executar `/fast` enquanto Claude está trabalhando, e Claude Code alterna o modo rápido sem esperar que o turno termine. Claude Code conclui o turno em execução em sua velocidade original, portanto a mudança de velocidade entra em vigor a partir do seu próximo turno. Se seu modelo atual não suportar modo rápido, ativá-lo também alterna seu modelo, e Claude Code usa o novo modelo a partir de sua próxima solicitação naquele turno.

Para melhor eficiência de custo, ative o modo rápido no início de uma sessão em vez de alternar no meio da conversa. Consulte [understand the cost tradeoff](#understand-the-cost-tradeoff) para obter detalhes.

Quando você ativa o modo rápido:

* Se seu modelo atual não suportar modo rápido, Claude Code alterna para Opus
* Você verá uma mensagem de confirmação: "Fast mode ON"
* Um pequeno ícone `↯` aparece ao lado do prompt enquanto o modo rápido está ativo
* Execute `/fast` novamente a qualquer momento para verificar se o modo rápido está ativado ou desativado

Opus 5.5 é o padrão do modo rápido no Claude Code v2.1.280 e posterior. Antes da v2.1.280, o modo rápido usava como padrão Opus 5 a partir da v2.1.219, Opus 4.8 na v2.1.154 até v2.1.218, e Opus 4.7 na v2.1.142 até v2.1.153.

Quando você desativa o modo rápido com `/fast` novamente, você permanece no Opus. Para alternar para um modelo diferente, use `/model`.

<h3 id="switch-models-while-fast-mode-is-on">
  Alternar modelos enquanto o modo rápido está ativado
</h3>

O modo rápido segue suas alternâncias de modelo em ambas as direções:

* **Alternar para outro**: quando você alterna para um modelo que não suporta modo rápido, Claude Code desativa o modo rápido. Isso inclui Opus 4.7; antes da v2.1.221, o modo rápido permanecia ativado após uma alternância para Opus 4.7 e a API rejeitava as solicitações.
* **Alternar de volta**: alternar de volta para um modelo Opus suportado ativa o modo rápido novamente quando sua preferência de modo rápido salva está ativada, a mesma preferência que uma nova sessão inicia por padrão. Uma alternância de modelo nunca ativa o modo rápido para uma sessão cuja preferência salva está desativada, e com [per-session opt-in](#require-per-session-opt-in) configurado, alternar de volta não ativa o modo rápido novamente; execute `/fast` para reativá-lo.

Sempre que uma alternância de modelo ativa ou desativa o modo rápido, Claude Code mostra uma confirmação `Fast mode ON` ou `Fast mode OFF`, e o ícone `↯` aparece enquanto o modo rápido está ativado. Isso vale se você alternar com `/model`, com [`/config model=<model>`](/docs/pt/settings), ou de um dispositivo conectado através de [Remote Control](/docs/pt/remote-control).

Claude Code reenvia o status do modo rápido da sessão para dispositivos conectados através de Remote Control após uma alternância de modelo, uma reconexão, ou uma [verificação de disponibilidade](#use-fast-mode-behind-proxies-and-llm-gateways) falhada.

<h3 id="use-fast-mode-in-cloud-sessions">
  Usar modo rápido em sessões em nuvem
</h3>

O modo rápido funciona em [sessões em nuvem](/docs/pt/claude-code-on-the-web) quando está disponível em sua conta, seja a sessão executada em infraestrutura gerenciada pela Anthropic ou em um [executor auto-hospedado](/docs/pt/self-hosted-environments). Requer Claude Code v2.1.271 ou posterior no ambiente da sessão.

Digite `/fast on` na sessão para ativar o modo rápido. Ele permanece ativado apenas para essa sessão e não é salvo como seu padrão. Os [requisitos](#requirements) também se aplicam em sessões em nuvem.

<h2 id="understand-the-cost-tradeoff">
  Entender o tradeoff de custo
</h2>

O modo rápido tem preços por token mais altos do que o Opus padrão:

| Modelo   | Entrada (MTok) | Saída (MTok) |
| -------- | -------------- | ------------ |
| Opus 5.5 | \$8            | \$40         |
| Opus 5   | \$10           | \$50         |
| Opus 4.8 | \$10           | \$50         |

O preço do modo rápido é fixo em toda a janela de contexto de 1M token. Para a taxa padrão do Opus para comparar, consulte a [referência de preços do Claude](https://platform.claude.com/docs/pt/about-claude/pricing).

A primeira vez que você ativa o modo rápido em uma conversa, você paga o preço total do token de entrada não armazenado em cache do modo rápido para todo o contexto da conversa. Quanto mais profundo você estiver em uma conversa, mais isso custa, portanto ativar o modo rápido desde o início é mais barato. O custo se aplica uma vez por conversa, portanto desativar e ativar o modo rápido novamente mais tarde não o repete. Para o mecanismo, consulte [como o modo rápido interage com o cache de prompt](/docs/pt/prompt-caching#turning-on-fast-mode).

<h3 id="see-where-fast-mode-spend-appears">
  Veja onde o gasto do modo rápido aparece
</h3>

Você vê o gasto do modo rápido em um lugar diferente dependendo de como você se conectou, portanto primeiro execute [`/status`](/docs/pt/commands) para verificar. Se mostrar uma linha `Login method` como `Claude Max account`, você se conectou com uma assinatura Claude. Se mostrar uma linha `API key` em vez disso, suas solicitações são cobradas em uma organização do Claude Console.

* **Pro e Max**: você paga pelo modo rápido com seus créditos de uso. Vá para [**Settings > Usage**](https://claude.ai/settings/usage) em claude.ai, onde a seção **Usage credits** mostra quanto você gastou em créditos de uso este mês. Essa figura inclui o modo rápido, mas não o separa individualmente.
* **Team e Enterprise**: sua organização paga pelo seu uso do modo rápido com seus créditos de uso. Para ver seu próprio gasto de créditos de uso, execute [`/usage`](/docs/pt/costs#check-your-usage-credits-spend). Para ver onde sua organização vê esse gasto, consulte [Claude for Teams and Enterprise](/docs/pt/costs#claude-for-teams-and-enterprise).
* **Claude Console**: sua organização paga pelo modo rápido junto com o resto de seu uso de API. Nas páginas [Usage](https://platform.claude.com/usage) e [Cost](https://platform.claude.com/cost) do Console, selecione **Speed (Research Preview)** no menu **Group by** para separar o modo rápido do uso em velocidade padrão. Você vê essa opção apenas quando o intervalo de datas selecionado inclui uso do modo rápido.

<h2 id="decide-when-to-use-fast-mode">
  Decidir quando usar o modo rápido
</h2>

O modo rápido é melhor para trabalho interativo onde a latência de resposta importa mais do que o custo:

* Iteração rápida em mudanças de código
* Sessões de depuração ao vivo
* Trabalho sensível ao tempo com prazos apertados

O modo padrão é melhor para:

* Tarefas autônomas longas onde a velocidade importa menos
* Processamento em lote ou pipelines CI/CD
* Cargas de trabalho sensíveis ao custo

<h3 id="fast-mode-vs-effort-level">
  Modo rápido vs nível de esforço
</h3>

O modo rápido e o nível de esforço afetam a velocidade de resposta, mas de formas diferentes:

| Configuração                    | Efeito                                                                                                      |
| ------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| **Modo rápido**                 | Mesma qualidade de modelo, latência mais baixa, custo mais alto                                             |
| **Nível de esforço mais baixo** | Menos tempo de pensamento, respostas mais rápidas, qualidade potencialmente mais baixa em tarefas complexas |

Você pode combinar ambos: use o modo rápido com um [nível de esforço](/docs/pt/model-config#adjust-effort-level) mais baixo para máxima velocidade em tarefas diretas.

<h2 id="requirements">
  Requisitos
</h2>

O modo rápido requer todos os seguintes:

* **Apenas API Anthropic ou assinatura**: o modo rápido está disponível através da API do Anthropic Console e para planos de assinatura Claude usando créditos de uso. Não está disponível no Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry ou Claude Platform na AWS. As organizações do Console também devem ter [acesso ao modo rápido provisionado](#enable-fast-mode-for-your-organization).
* **Créditos de uso ativados para planos de assinatura**: em um plano Pro, Max, Team ou Enterprise, sua conta deve ter [créditos de uso](/docs/pt/costs#add-usage-credits-to-your-subscription) ativados, o que permite cobrança além do uso incluído no seu plano. Até que estejam ativados, `/fast` mostra "Fast mode requires usage credits". Como você os ativa depende do seu plano:
  * Em Pro e Max, ative-os na seção **Usage credits** de [**Settings > Usage**](https://claude.ai/settings/usage) em claude.ai, ou execute `/usage-credits` para abrir essa página.
  * Em Team e Enterprise, um membro com acesso de cobrança os ativa para a organização em [**Admin settings > Usage**](https://claude.ai/admin-settings/usage), e um membro sem acesso executa `/usage-credits` para enviar uma solicitação aos administradores da organização.

<Note>
  O uso do modo rápido é cobrado diretamente nos créditos de uso, mesmo que você tenha uso restante no seu plano.
</Note>

* **Organização Console paga**: contas do Claude Console não usam créditos de uso, e sua organização paga pelo modo rápido por token junto com o resto do seu uso de API. No plano Evaluation gratuito do Console, `/fast` mostra "Fast mode unavailable during evaluation. Please purchase credits." Para limpar isso, compre créditos nas suas [configurações de cobrança do Console](https://platform.claude.com/settings/billing).
* **Habilitação de proprietário para Team e Enterprise**: o modo rápido está desativado por padrão para organizações Team e Enterprise. Um proprietário deve explicitamente [ativar o modo rápido](#enable-fast-mode-for-your-organization) antes que os usuários possam acessá-lo.

<Note>
  Quatro configurações de organização podem bloquear a ativação do modo rápido com `/fast`:

  * **Modo rápido não ativado**: se o modo rápido não tiver sido ativado para sua organização, ativar o modo rápido com `/fast` mostra "Fast mode has been disabled by your organization."
  * **Modo rápido desativado por configurações gerenciadas**: se sua organização implanta [configurações gerenciadas](/docs/pt/managed-settings) que definem [`fastMode: false`](/docs/pt/settings-reference#fastmode), ativar o modo rápido com `/fast` mostra a mesma mensagem "Fast mode has been disabled by your organization".
  * **Opt-in obrigatório por sessão**: configurações gerenciadas que definem [`fastModePerSessionOptIn: true`](#require-per-session-opt-in) recusam `/fast on` com a mesma mensagem em todos os lugares, exceto em uma sessão de terminal interativa.
  * **Modelo de modo rápido não permitido**: se a lista de permissões [`availableModels`](/docs/pt/model-config#restrict-model-selection) da sua organização excluir o modelo Opus do modo rápido, ativá-lo é recusado com "is not in your organization's allowed models". Em uma sessão já em execução em um modelo Opus permitido que suporte modo rápido, `/fast` ativa o modo rápido no seu modelo atual sem alternar modelos.
</Note>

<h3 id="enable-fast-mode-for-your-organization">
  Ativar modo rápido para sua organização
</h3>

Onde você ativa o modo rápido depende de qual produto sua organização usa:

* **Console** (clientes de API): um administrador o ativa em [Preferências do Claude Code](https://platform.claude.com/claude-code/preferences). O modo rápido está em [visualização de pesquisa](#research-preview), portanto sua organização também deve ter acesso ao modo rápido provisionado antes que as solicitações de modo rápido sejam bem-sucedidas. Para obter acesso, entre em contato com seu gerente de conta ou junte-se à lista de espera, conforme descrito em [modo rápido na API Claude](https://platform.claude.com/docs/en/build-with-claude/fast-mode).

  Sem acesso provisionado, a API rejeita cada solicitação de modo rápido com um 429, e Claude Code trata cada rejeição como um [limite de taxa do modo rápido](#handle-rate-limits). Diferentemente do cooldown de um limite de taxa, as rejeições continuam até que o acesso seja provisionado.
* **Claude AI** (Team e Enterprise): um proprietário o ativa em [Admin Settings > Claude Code](https://claude.ai/admin-settings/claude-code)

Outra opção para desativar completamente o modo rápido é definir `CLAUDE_CODE_DISABLE_FAST_MODE=1`. Consulte [Variáveis de ambiente](/docs/pt/env-vars).

<h3 id="use-fast-mode-behind-proxies-and-llm-gateways">
  Usar modo rápido atrás de proxies e gateways LLM
</h3>

Antes de oferecer modo rápido, Claude Code verifica a disponibilidade de modo rápido da sua organização com uma solicitação diretamente para `api.anthropic.com`. A verificação não segue [`ANTHROPIC_BASE_URL`](/docs/pt/llm-gateway-connect#set-the-base-url-and-credential), portanto em uma rede que roteia o tráfego Claude através de um [gateway LLM](/docs/pt/llm-gateway) e bloqueia a saída direta para `api.anthropic.com`, a verificação falha mesmo que as solicitações de inferência funcionem. A verificação usa um [proxy HTTP](/docs/pt/network-config#proxy-configuration) configurado, portanto um bloqueio de rede falha na verificação apenas onde `api.anthropic.com` é inacessível mesmo através do proxy.

Quando a verificação falha, `/fast` relata "Fast mode unavailable due to network connectivity issues", e as solicitações são executadas em velocidade padrão, mesmo quando sua organização tem modo rápido ativado. Uma verificação que foi bem-sucedida no passado continua funcionando a partir de seu resultado em cache, portanto uma verificação bloqueada afeta principalmente novas instalações.

A mesma mensagem de conectividade aparece em uma rede aberta quando a verificação atinge `api.anthropic.com` mas apresenta uma credencial que Anthropic rejeita. Uma sessão cuja chave resolvida é uma credencial emitida por gateway, mantida em [`ANTHROPIC_API_KEY`](/docs/pt/llm-gateway-connect#set-the-base-url-and-credential) ou produzida por um [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper), envia a verificação com essa chave, e a solicitação rejeitada é relatada como uma falha de conectividade.

Para restaurar o modo rápido, coloque na lista de permissões a saída direta para `api.anthropic.com` onde um bloqueio de rede é a causa, ou defina qualquer variável que corresponda a como a verificação falha:

* `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS=1` trata uma verificação falhada como disponível e ainda honra uma resposta "disabled by your organization". Use-a quando sua rede recusa a conexão, ou quando Anthropic rejeita uma credencial de gateway; colocar na lista de permissões não ajuda no caso de credencial, já que nada é bloqueado.
* `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1` pula a verificação inteiramente. Use-a quando sua rede intercepta a solicitação em vez de recusá-la.

Duas configurações de gateway relatam "Fast mode has been disabled by your organization" em vez da mensagem de conectividade, mesmo quando sua organização tem modo rápido ativado:

* Uma sessão que se autentica com [`ANTHROPIC_AUTH_TOKEN`](/docs/pt/llm-gateway-connect#set-the-base-url-and-credential) apenas pula a verificação: sem um login claude.ai ou uma chave de API Anthropic, e sem uma verificação bem-sucedida em cache, Claude Code trata o modo rápido como desativado pela sua organização sem enviar a solicitação.
* Um proxy que intercepta a verificação e responde com sua própria página, por exemplo um proxy de inspeção TLS retornando uma página de bloqueio HTTP 200, é lido como uma resposta dizendo que sua organização tem modo rápido desativado.

Em ambos os casos, defina `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1` para restaurar o modo rápido. `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS` não se aplica a nenhum dos dois casos, já que apenas ignora verificações falhadas e ambos produzem uma resposta desativada. Colocar na lista de permissões a saída direta não ajuda no caso de token de portador, que nunca envia a solicitação.

As variáveis afetam apenas a verificação do lado do cliente. Quando sua organização tem modo rápido desativado, a API rejeita solicitações de modo rápido independentemente de estarem definidas ou não. Uma rejeição da API permanece mesmo com uma variável de pulo definida. Claude Code tenta novamente a solicitação rejeitada em velocidade padrão, desativa o modo rápido e `/fast` relata que sua organização desativou o modo rápido.

Definir `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` também suprime a verificação de disponibilidade. Sem uma verificação bem-sucedida em cache anterior, `/fast` relata "Fast mode is currently unavailable"; ambas as variáveis de pulo restauram o modo rápido nessa configuração também.

<h3 id="require-per-session-opt-in">
  Exigir opt-in por sessão
</h3>

Por padrão, o modo rápido que um usuário ativa em uma sessão interativa persiste entre sessões. Para alterar isso, defina `fastModePerSessionOptIn` como `true` em qualquer [arquivo de configurações](/docs/pt/settings#where-settings-live), o que faz com que cada sessão comece com o modo rápido desativado e exija que os usuários o ativem explicitamente com `/fast`. Os proprietários em planos [Team](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise) ou [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise) podem implantá-lo em toda a organização através de [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings).

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Isso é útil para controlar custos em organizações onde os usuários executam várias sessões simultâneas. A preferência de modo rápido do usuário ainda é salva, portanto remover essa configuração restaura o comportamento padrão persistente.

Quando configurações gerenciadas definem a chave, `/fast on` funciona apenas em uma sessão de terminal interativa. Em todos os outros lugares, incluindo [modo não interativo](/docs/pt/headless), a [extensão VS Code](/docs/pt/vs-code) e [sessões em nuvem](#use-fast-mode-in-cloud-sessions), é recusado com uma mensagem de que sua organização desativou o modo rápido.

<h2 id="handle-rate-limits">
  Lidar com limites de taxa
</h2>

O modo rápido tem limites de taxa separados do Opus padrão. Todos os modelos Opus suportados compartilham um pool de limite de taxa do modo rápido: o uso em qualquer um deles é extraído dos mesmos limites. Quando você atinge o limite de taxa do modo rápido:

1. O modo rápido automaticamente volta para velocidade padrão
2. O ícone `↯` fica cinza para indicar cooldown
3. Você continua trabalhando com velocidade e preços padrão
4. Quando o cooldown expira, o modo rápido é automaticamente reativado

Para desativar o modo rápido manualmente em vez de esperar pelo cooldown, execute `/fast` novamente.

Se você ficar sem créditos de uso no meio de uma sessão, Claude Code tenta novamente cada solicitação de modo rápido rejeitada com velocidade e preços padrão, para que você continue trabalhando, e não há cooldown. Como você vê a rejeição depende do tipo de sessão:

* Em uma sessão interativa, Claude Code mostra uma notificação "Fast mode disabled · usage credits exhausted" e desativa o modo rápido para o resto da sessão. Sua preferência de modo rápido salva não muda; execute `/fast` para ativar o modo rápido novamente.
* Em [modo não-interativo](/docs/pt/headless) com `--output-format stream-json`, e através do Agent SDK, Claude Code emite o mesmo texto no fluxo de mensagens como uma mensagem `system` com subtipo `notification`, uma vez por turno enquanto você estiver sem créditos de uso. O modo rápido permanece ativado. Requer Claude Code v2.1.221 ou posterior.

<h2 id="research-preview">
  Research preview
</h2>

O modo rápido é um recurso de visualização de pesquisa. Isso significa:

* O recurso pode mudar com base no feedback
* A disponibilidade e preços estão sujeitos a alterações
* A configuração de API subjacente pode evoluir

Relate problemas ou feedback através de seus canais de suporte Anthropic usuais.

<h2 id="see-also">
  Veja também
</h2>

* [Configuração de modelo](/docs/pt/model-config): alterne modelos e ajuste níveis de esforço
* [Gerenciar custos efetivamente](/docs/pt/costs): rastreie o uso de tokens e reduza custos
* [Configuração da linha de status](/docs/pt/statusline): exiba informações de modelo e contexto
