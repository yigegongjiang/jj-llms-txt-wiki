> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Escale decisões difíceis com a ferramenta advisor

> Combine seu modelo principal com um modelo advisor mais forte que Claude consulta em momentos-chave durante uma tarefa.

<Note>
  A ferramenta advisor é experimental e requer a API Anthropic. Não está disponível no Amazon Bedrock, Claude Platform on AWS, na plataforma de agentes do Google Cloud ou no Microsoft Foundry. O comportamento, preços e disponibilidade podem mudar.
</Note>

A ferramenta advisor permite que Claude consulte um segundo modelo, tipicamente mais forte, em momentos-chave durante uma tarefa, como antes de se comprometer com uma abordagem, quando preso em um erro recorrente, ou antes de declarar uma tarefa concluída. O advisor recebe a conversa completa, incluindo cada chamada de ferramenta e resultado, e retorna orientação que Claude aplica antes de continuar.

O advisor é executado no servidor na infraestrutura da Anthropic como uma [server tool](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool), disponível para contas de assinatura e faturadas por API. Você escolhe qual modelo atua como advisor, e Claude decide quando chamá-lo.

Esta página cobre como ativar o advisor, quais emparelhamentos de modelos são aceitos, o que Claude mostra durante uma consulta, e como o uso do advisor é faturado.

<h2 id="when-to-use-the-advisor">
  Quando usar o advisor
</h2>

O advisor é adequado para tarefas longas e com múltiplas etapas onde a maioria dos turnos é rotineira, mas a qualidade do plano determina o resultado. Exemplos incluem grandes refatorações, sessões de depuração onde um erro continua recorrendo, e tarefas que você deseja verificadas independentemente antes de Claude declarar que estão concluídas.

Adiciona menos valor em tarefas curtas onde há pouco a planejar, ou em trabalho onde cada turno precisa do modelo mais forte. Para esses casos, [mude o modelo principal](/docs/pt/model-config#setting-your-model) em vez disso, ou veja [como o advisor se compara com opusplan e subagents](#compare-with-related-features) para outras formas de obter uma segunda opinião.

<h2 id="enable-the-advisor">
  Ativar o advisor
</h2>

Você pode definir o modelo advisor de três formas:

* **Comando `/advisor`**: defina ou altere o advisor no meio da sessão e salve-o como seu padrão
* **Configuração `advisorModel`**: configure um padrão persistente em seu [arquivo de configurações](/docs/pt/settings)
* **Flag `--advisor`**: defina o advisor para uma única sessão no lançamento

Cada uma dessas opções ativa o advisor para sessões cujo modelo principal [o suporta](#choose-an-advisor-model). Após a sessão iniciar, Claude Code mostra uma notificação `Advisor Tool (experimental) is on and may use more tokens · /advisor`. Para parar de usar o advisor, veja [Desativar o advisor](#turn-the-advisor-off).

Em alguns planos, Fable como advisor também precisa de seu [consentimento único para cobrar o uso de Fable aos créditos de uso](/docs/pt/model-config#fable-and-usage-credits). Para saber o que acontece antes de você ter dado esse consentimento, veja [Advisor Fable e créditos de uso](#fable-advisor-and-usage-credits).

<h3 id="use-the-/advisor-command">
  Use o comando `/advisor`
</h3>

Execute `/advisor` sem argumentos para abrir um seletor listando os modelos advisor disponíveis, ou passe o modelo diretamente:

```
/advisor opus
```

O comando confirma com `Advisor set to` seguido pelo nome do modelo advisor. Sua seleção é salva em `advisorModel` nas configurações do usuário e persiste entre sessões, exceto nos casos que a entrada [`advisorModel`](/docs/pt/settings-reference#advisormodel) lista como aplicável apenas à sessão atual.

O comando também funciona onde não há um seletor de terminal: em [modo não interativo](/docs/pt/headless) com `-p`, no Agent SDK, no aplicativo desktop e sobre [Remote Control](/docs/pt/remote-control). Isso requer Claude Code v2.1.260 ou posterior. Nessas superfícies:

* Execute `/advisor` sem argumento para imprimir o modelo advisor atual e os aliases que ele aceita.
* Execute `/advisor` com um modelo, como `/advisor opus`, para defini-lo.
* Execute `/advisor off` para desativá-lo.

Claude Code não invoca um advisor salvo que a allowlist [`availableModels`](/docs/pt/model-config#restrict-model-selection) da sua organização exclua. Para usar o advisor, escolha um modelo permitido com `/advisor`.

Claude Code ainda salva um advisor que seu modelo principal atual não suporta. Esse advisor é ativado após você mudar para um [modelo principal compatível](#choose-an-advisor-model) com [`/model`](/docs/pt/model-config#setting-your-model). Se a API já recusou o advisor salvo na conversa atual, ele permanece desativado até `/clear` ou `/compact`, mesmo após você mudar de modelos.

Em alguns planos, Fable como advisor também precisa de seu [consentimento único para cobrar o uso de Fable aos créditos de uso](/docs/pt/model-config#fable-and-usage-credits). Para saber o que `/advisor fable` faz antes de você ter dado esse consentimento, veja [Advisor Fable e créditos de uso](#fable-advisor-and-usage-credits).

<h3 id="set-advisormodel-in-settings">
  Defina `advisorModel` nas configurações
</h3>

Para configurar o advisor como padrão sem abrir uma sessão, defina-o em seu arquivo de configurações:

```json theme={null}
{
  "advisorModel": "opus"
}
```

<h3 id="use-the-advisor-flag">
  Use a flag `--advisor`
</h3>

Para definir o advisor para uma única sessão sem alterar sua configuração salva, inicie com a flag:

```bash theme={null}
claude --advisor opus
```

Claude Code usa a flag em vez da configuração `advisorModel` para essa sessão. Ela não lista `--advisor` em `claude --help`. Claude Code sai com um erro no lançamento se:

* O modelo principal da sessão não suportar o advisor
* O modelo solicitado, como Haiku, não puder atuar como um advisor
* A allowlist [`availableModels`](/docs/pt/model-config#restrict-model-selection) da sua organização exclua o modelo solicitado
* Você solicitou Fable e sua conta ainda requer o [consentimento de créditos de uso](#fable-advisor-and-usage-credits)

Se você iniciar uma [sessão em background](/docs/pt/agent-view) com `--advisor` e uma dessas situações se aplicar, Claude Code inicia a sessão sem o advisor em vez de sair.

<h2 id="choose-an-advisor-model">
  Escolha um modelo advisor
</h2>

O advisor deve ser pelo menos tão capaz quanto o modelo principal. Os advisors aceitos para cada modelo principal são:

| Modelo principal     | Advisors aceitos                       | Notas                                                                                     |
| -------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------- |
| Haiku 4.5            | Fable, Opus, Sonnet                    | Haiku pode chamar o advisor, mas não pode atuar como um                                   |
| Sonnet 4.6           | Fable, Opus, Sonnet                    |                                                                                           |
| Sonnet 5             | Fable, Opus 4.7 ou posterior, Sonnet 5 | Um advisor Sonnet 4.6 é rejeitado, e a API recusa um advisor Opus 4.6                     |
| Opus 4.6             | Fable, Opus, Sonnet 5                  | Um advisor Sonnet 4.6 é rejeitado                                                         |
| Opus 4.7 ou Opus 4.8 | Fable, e Opus 4.7 ou posterior         | Um advisor Opus 4.6 ou Sonnet é rejeitado                                                 |
| Opus 5.5 ou Opus 5   | Fable, e Opus 5 ou posterior           | Um advisor Opus 4.6 ou Sonnet é rejeitado, e a API recusa um advisor Opus 4.7 ou Opus 4.8 |
| Fable 5              | Fable 5.1 ou Fable 5                   | Um advisor Opus ou Sonnet é rejeitado                                                     |
| Fable 5.1            | Fable 5.1                              | Um advisor Opus ou Sonnet é rejeitado, e a API recusa um advisor Fable 5                  |

Fable 5.1 requer Claude Code v2.1.257 ou posterior. Ambos os modelos Fable requerem [acesso a Fable](/docs/pt/model-config#work-with-fable).

Defina o advisor como `fable`, `opus`, ou `sonnet`. Esses aliases resolvem para a versão padrão integrada do Claude Code para cada família de modelos, que avança com novos lançamentos do Claude Code. Você também pode passar um ID de modelo completo como `claude-opus-5-5`.

Subagentes herdam o advisor configurado e aplicam a mesma verificação de emparelhamento contra seu próprio modelo.

Claude Code valida o emparelhamento antes de enviar uma solicitação, e a API valida novamente:

* Para um advisor que a tabela lista como rejeitado, Claude Code não o anexa às solicitações do modelo principal. A saída do comando `/advisor` e uma notificação mostram isso. Subagentes cujo próprio modelo satisfaz o emparelhamento ainda podem usar o advisor.
* Para um advisor que a tabela lista como recusado pela API, Claude Code o anexa e a API o recusa. Claude Code então reenvia essa solicitação sem o advisor, e o resto da conversa é executado sem um, então você não vê nenhum erro e não obtém nenhuma chamada de advisor. Escolha um advisor aceito com `/advisor`; a mudança entra em vigor após `/clear` ou `/compact` e em novas sessões.
* Se o modelo principal ou o advisor for um modelo que Claude Code não reconhece, o advisor não será anexado.

<h3 id="fable-advisor-and-usage-credits">
  Advisor Fable e créditos de uso
</h3>

Em alguns planos, o uso de Fable é cobrado em créditos de uso, e Fable como advisor é cobrado da mesma forma. Se sua conta exigir o [consentimento único para cobrar o uso de Fable em créditos de uso](/docs/pt/model-config#fable-and-usage-credits), Claude Code solicitará quando você selecionar um modelo Fable com `/model` e não aplicará Fable como advisor até que você tenha aceito esse consentimento.

Antes de você ter aceito, Claude Code não salva Fable como advisor quando você digita `/advisor fable` ou escolhe Fable no seletor `/advisor`. Em vez disso, ele o aponta para `/model fable`. Com `claude --advisor fable`, Claude Code sai no lançamento com uma mensagem que aponta para `/model fable`. Em uma [sessão em segundo plano](#use-the-advisor-flag), ele inicia a sessão sem o advisor em vez de sair. Com Fable já salvo como seu `advisorModel`, Claude Code envia solicitações sem o advisor. Em uma sessão interativa cujo modelo principal suporta o advisor, ele também mostra uma notificação que aponta para `/model fable`.

Para aceitar o consentimento, execute `/model fable` e escolha continuar em Fable. Claude Code registra o consentimento e [salva Fable como seu modelo selecionado](/docs/pt/model-config#default-model-setting). Em seguida, selecione Fable como o advisor.

<h3 id="common-model-pairings">
  Emparelhamentos de modelos comuns
</h3>

Qualquer emparelhamento aceito funciona. Essas combinações equilibram custo contra capacidade de diferentes formas:

| Emparelhamento                    | Quando usar                                                                                                                                                  |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Sonnet principal + advisor Opus   | Sonnet lida com trabalho rotineiro e escala planejamento, falhas ambíguas e verificações de conclusão para Opus                                              |
| Sonnet principal + advisor Fable  | Orientação Fable em pontos de decisão sem executar Fable em toda parte. Requer acesso a Fable                                                                |
| Haiku principal + advisor Opus    | Modelo principal de menor custo com planejamento forte. Espere custo mais alto que Haiku sozinho, mas menor que mudar o modelo principal para Sonnet ou Opus |
| Opus principal + advisor Opus     | Um segundo Opus revisa o primeiro. Útil para tarefas de alto risco onde uma verificação independente importa mais que o custo                                |
| Fable principal + advisor Fable   | Emparelhamento de maior capacidade quando Fable está disponível. Claude Code não aplica um advisor Opus ou Sonnet a um modelo principal Fable                |
| Sonnet principal + advisor Sonnet | Uma segunda opinião de menor custo para capturar oversights rotineiros                                                                                       |

<h2 id="when-claude-consults-the-advisor">
  Quando Claude consulta o advisor
</h2>

Claude decide quando chamar o advisor. Tende a consultar antes de se comprometer com uma abordagem, quando um erro continua recorrendo, e antes de declarar uma tarefa concluída, mas o tempo é orientado pelo modelo em vez de baseado em regras.

Você pode pedir uma consulta em seu prompt da mesma forma que solicitaria qualquer ferramenta, por exemplo `consult the advisor before you continue`. Não há configuração para limitar ou forçar chamadas do advisor; se você quiser que Claude consulte mais ou menos frequentemente durante uma tarefa, diga isso em suas instruções.

<h2 id="what-you-see-during-a-session">
  O que você vê durante uma sessão
</h2>

Quando Claude chama o advisor, a transcrição mostra uma linha `Advising` com o nome do modelo advisor enquanto a chamada está em andamento. Quando o resultado retorna, a linha relata se o advisor forneceu orientação:

* **Reviewed**: a linha confirma que o advisor revisou a conversa. Quando o advisor retornou orientação legível, pressione `Ctrl+O` para lê-la.
* **Declined**: a linha lê `Advisor declined to advise on this request`. Se o advisor forneceu um motivo, pressione `Ctrl+O` para lê-lo.

Claude geralmente segue a orientação do advisor, mas se adapta quando sua própria evidência contradiz uma afirmação específica: se uma etapa recomendada falha quando tentada, ou o conteúdo do arquivo contradiz o conselho, Claude expõe o conflito em vez de seguir a orientação incondicionalmente.

O advisor sempre recebe a conversa completa, e Claude controla o tempo. Para mais controle ou uma configuração diferente, veja [como o advisor se compara com subagents e opusplan](#compare-with-related-features).

<h2 id="cost">
  Custo
</h2>

Quando Claude chama o advisor, o modelo advisor lê a conversa, então cada chamada consome tokens nas taxas do modelo advisor além do uso do seu modelo principal. Como esses tokens do advisor são faturados depende de como você paga:

* **Faturamento por API**: você paga as taxas de entrada e saída do modelo advisor para tokens do advisor
* **Planos de assinatura**: o uso do advisor conta para os limites de uso do seu plano, exceto que um advisor Fable é cobrado em [créditos de uso](/docs/pt/model-config#fable-and-usage-credits) em planos onde o uso de Fable o faz

Se sua conta exigir o consentimento de créditos de uso, um advisor Fable não cobra nada antes de você concedê-lo, porque Claude Code [não aplica a seleção](#fable-advisor-and-usage-credits) até então.

Claude chama o advisor em pontos de decisão em vez de em cada turno, então emparelhar um modelo principal mais rápido com um advisor mais forte tipicamente custa menos que executar o modelo mais forte em toda parte. O uso do advisor conta para os totais da sessão mostrados por [`/usage`](/docs/pt/costs#track-your-costs).

Para como tokens do advisor são reportados em respostas da API, veja [Usage and billing](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool#usage-and-billing) na documentação da API Claude.

<h2 id="impact-on-prompt-caching">
  Impacto no prompt caching
</h2>

Ativar ou desativar o advisor no meio da sessão não invalida o [prompt cache](/docs/pt/prompt-caching) do seu modelo principal. Diferentemente de [mudar modelos](/docs/pt/prompt-caching#switching-models), alternar `/advisor` mantém o prefixo em cache intacto, e a orientação retornada pelo advisor é armazenada em cache como parte da transcrição em turnos posteriores.

A própria leitura do advisor da conversa não é armazenada em cache. Cada chamada do advisor processa a transcrição completa novamente, sem reutilização entre chamadas.

<h2 id="requirements">
  Requisitos
</h2>

A ferramenta advisor requer todos os seguintes:

* **Apenas API Anthropic**: o advisor é uma ferramenta executada no servidor. Não está disponível no Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform ou Microsoft Foundry. Através de um [LLM gateway](/docs/pt/llm-gateway) configurado com `ANTHROPIC_BASE_URL`, a disponibilidade depende se o gateway encaminha a solicitação intacta para a API Anthropic. Se o gateway ou seu upstream não reconhecer a ferramenta advisor, consulte [Retry automático e encaminhamento de erro](/docs/pt/llm-gateway-protocol#automatic-retry-and-error-forwarding) para saber como Claude Code responde.
* **Modelo principal suportado**: Fable, Opus 4.6 ou posterior, Sonnet 4.6 ou posterior, ou Haiku 4.5. Consulte [Escolher um modelo advisor](#choose-an-advisor-model) para saber quais advisors cada um aceita.
* **Busca de feature-flag**: Claude Code ativa o advisor através de um feature flag que busca da Anthropic. Em uma sessão onde uma variável que desativa a busca de flag está definida, como `DISABLE_TELEMETRY`, o advisor permanece desativado. Consulte [Recursos que precisam de busca de feature-flag](/docs/pt/env-vars#features-that-need-feature-flag-fetching).

<h2 id="turn-the-advisor-off">
  Desativar o advisor
</h2>

Para parar de usar o advisor, execute `/advisor off` ou escolha **No advisor** no seletor `/advisor`:

```
/advisor off
```

Para desativar a ferramenta advisor inteiramente, defina `CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`. O comando `/advisor` fica indisponível e qualquer `advisorModel` configurado é ignorado. A flag `--advisor` é aceita mas não tem efeito. Veja [Environment variables](/docs/pt/env-vars).

<h2 id="compare-with-related-features">
  Compare com recursos relacionados
</h2>

O advisor é uma das várias formas de combinar forças de modelos. Escolha com base em quando você quer um segundo modelo envolvido.

| Abordagem                                                       | Quando o modelo mais forte é executado                                                                                                       | Como começa                                 |
| --------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------- |
| Ferramenta Advisor                                              | Em pontos de decisão no meio da tarefa                                                                                                       | Claude a chama quando precisa de orientação |
| [`opusplan`](/docs/pt/model-config#opusplan-model-setting)           | Durante plan mode quando [permitido por `availableModels`](/docs/pt/model-config#restrict-model-selection), depois muda para Sonnet para execução | Você entra em plan mode                     |
| [Subagents](/docs/pt/sub-agents#choose-a-model) com `model` definido | Para toda a subtarefa delegada                                                                                                               | Claude delega, ou você invoca o subagent    |
| [`/model`](/docs/pt/model-config#setting-your-model)                 | Para todos os turnos subsequentes                                                                                                            | Você muda de modelos                        |

<h2 id="see-also">
  Veja também
</h2>

* [Model configuration](/docs/pt/model-config): mude modelos, defina níveis de esforço, e use `opusplan`
* [Manage costs effectively](/docs/pt/costs): rastreie o uso de tokens entre modelos
* [Advisor tool in the Claude API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool): entenda a ferramenta de servidor subjacente, ou use-a diretamente da Messages API
* [The advisor strategy](https://claude.com/blog/the-advisor-strategy): por que emparelhar um modelo principal rápido com um advisor mais forte funciona
