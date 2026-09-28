> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configuração de modelo

> Configure qual modelo Claude Code usa, níveis de esforço, contexto estendido e a janela de auto-compactação

<h2 id="available-models">
  Modelos disponíveis
</h2>

Para a configuração `model` no Claude Code, você pode configurar:

* Um **alias de modelo**
* Um **nome de modelo**
  * API Anthropic: um **[nome de modelo](https://platform.claude.com/docs/en/about-claude/models/overview)** completo
  * Amazon Bedrock: um ARN de perfil de inferência
  * Microsoft Foundry: um nome de implantação
  * Agent Platform do Google Cloud: um nome de versão

Para orientação sobre qual modelo e nível de esforço se adequam a diferentes tipos de trabalho, consulte [Escolhendo um modelo Claude e nível de esforço no Claude Code](https://claude.com/blog/claude-model-and-effort-level-in-claude-code) no blog.

<Note>
  `ANTHROPIC_BASE_URL` muda para onde as solicitações são enviadas, não qual modelo as responde. Para rotear Claude através de um gateway LLM, consulte [Gateways LLM](/docs/pt/llm-gateway).
</Note>

<h3 id="model-aliases">
  Aliases de modelo
</h3>

Use um alias de modelo para selecionar configurações de modelo sem precisar lembrar dos números exatos da versão:

| Alias de modelo  | Comportamento                                                                                                                                                                                                                                                                                                                                                   |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`default`**    | Valor especial que limpa qualquer substituição de modelo e reverte para o [padrão de tempo de execução para sua conta](#default-model-setting). Não é em si um alias de modelo                                                                                                                                                                                  |
| **`best`**       | Usa o modelo para o qual o alias [`fable` é resolvido](#fable-alias-resolution) onde Fable está disponível para você, caso contrário, o mesmo modelo que `opus`                                                                                                                                                                                                 |
| **`fable`**      | Usa o [modelo Fable para seu provedor](#fable-alias-resolution) para suas tarefas mais difíceis e de execução mais longa                                                                                                                                                                                                                                        |
| **`sonnet`**     | Usa o modelo Sonnet mais recente para tarefas de codificação diária                                                                                                                                                                                                                                                                                             |
| **`opus`**       | Usa o modelo Opus mais recente para tarefas de raciocínio complexo                                                                                                                                                                                                                                                                                              |
| **`haiku`**      | Usa o modelo Haiku rápido e eficiente para tarefas simples                                                                                                                                                                                                                                                                                                      |
| **`sonnet[1m]`** | Usa Sonnet com uma [janela de contexto de 1 milhão de tokens](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) para sessões longas. Sem efeito quando `sonnet` já é resolvido para Sonnet 5 com sua janela nativa de 1M; atrás de um [gateway LLM](/docs/pt/llm-gateway), seleciona a janela de 1M para Sonnet 5 |
| **`opus[1m]`**   | Usa Opus com uma [janela de contexto de 1 milhão de tokens](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) para sessões longas                                                                                                                                                                            |
| **`opusplan`**   | Modo especial que usa `opus` durante o Plan Mode, depois muda para `sonnet` para execução                                                                                                                                                                                                                                                                       |

A versão para a qual os aliases `opus` e `sonnet` são resolvidos depende do provedor:

| Provedor                                             | `opus`   | `sonnet`   |
| :--------------------------------------------------- | :------- | :--------- |
| API Anthropic                                        | Opus 5.5 | Sonnet 5   |
| [Claude Platform on AWS](/docs/pt/claude-platform-on-aws) | Opus 5.5 | Sonnet 4.6 |
| Amazon Bedrock, Agent Platform do Google Cloud       | Opus 5.5 | Sonnet 4.5 |
| Microsoft Foundry                                    | Opus 4.6 | Sonnet 4.5 |

<span id="fable-alias-resolution" />

A menos que você defina `ANTHROPIC_DEFAULT_FABLE_MODEL`, o alias `fable` é resolvido para Fable 5.1, exceto em sessões do [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway), onde `fable` e `best` são resolvidos para Fable 5. Antes da v2.1.257, `fable` era resolvido para Fable 5 em todos os provedores.

Um gateway que não está configurado para servir `claude-fable-5-1` rejeita solicitações para esse modelo. Para usar Fable 5.1 através de um gateway que o serve, selecione-o com `/model claude-fable-5-1`.

Onde um alias é resolvido para um modelo mais antigo, modelos mais novos estão disponíveis selecionando o nome completo do modelo explicitamente ou definindo `ANTHROPIC_DEFAULT_OPUS_MODEL` ou `ANTHROPIC_DEFAULT_SONNET_MODEL`.

Antes da v2.1.280, `opus` era resolvido para Opus 5 na API Anthropic, Claude Platform on AWS, Amazon Bedrock e Agent Platform do Google Cloud a partir da v2.1.219. Antes da v2.1.219, `opus` era resolvido para Opus 4.8 na API Anthropic a partir da v2.1.154, e no Claude Platform on AWS, Amazon Bedrock e Agent Platform do Google Cloud a partir da v2.1.207. Antes da v2.1.207, `opus` era resolvido para Opus 4.7 no Claude Platform on AWS e para Opus 4.6 no Amazon Bedrock e Agent Platform do Google Cloud.

Os aliases apontam para a versão recomendada para seu provedor e são atualizados ao longo do tempo. Para fixar uma versão específica, use o nome completo do modelo, por exemplo `claude-opus-5-5`, ou defina a variável de ambiente correspondente como `ANTHROPIC_DEFAULT_OPUS_MODEL`.

<Note>
  Opus 5.5 requer Claude Code v2.1.280 ou posterior. Opus 5 requer v2.1.219 ou posterior. Sonnet 5 requer v2.1.197 ou posterior. Execute `claude update` para atualizar.
</Note>

<h3 id="work-with-fable">
  Trabalhar com Fable
</h3>

[Claude Fable 5.1](https://platform.claude.com/docs/en/about-claude/models/overview) e Claude Fable 5 são os modelos mais capazes no Claude Code, adequados para tarefas maiores que uma única sessão. Eles sustentam sessões autônomas longas, investigam antes de agir e verificam seu trabalho com mais frequência do que modelos menores. Fable 5.1 é o lançamento mais recente.

Nenhum modelo Fable é o padrão do tipo de conta em nenhum plano ou provedor. Selecione um explicitamente:

* **Fable 5.1**: execute `/model fable`, ou inicie com `claude --model fable`. Em sessões do [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway), onde o alias é resolvido para Fable 5, execute `/model claude-fable-5-1` em vez disso.
* **Fable 5**: selecione-o por ID de modelo. Na API Anthropic, execute `/model claude-fable-5` ou inicie com `claude --model claude-fable-5`. Em outros provedores, use o ID do modelo Fable 5 do seu provedor ou [fixe-o](#pin-models-for-third-party-deployments) com `ANTHROPIC_DEFAULT_FABLE_MODEL`.

Se você se conectar à API Anthropic diretamente e suas configurações de usuário contiverem `claude-fable-5` ou `claude-fable-5[1m]` como o modelo, por exemplo porque você selecionou Fable no seletor `/model` antes da v2.1.257, Claude Code muda esse valor salvo para o alias `fable` ou `fable[1m]` na primeira vez que você executa v2.1.257 ou posterior. A linha do modelo de inicialização mostra `(auto-updated)` uma vez. Um valor `claude-fable-5` nas configurações de projeto, local ou gerenciado permanece como está.

Solicitações que os classificadores de segurança de um modelo Fable sinalizam, mais frequentemente em domínios de cibersegurança e biologia, acionam [fallback automático de modelo](#automatic-model-fallback).

Para aproveitar ao máximo o Fable:

* **Descreva o resultado, não as etapas**: entregue-lhe o resultado que você deseja e deixe-o planejar o caminho. Para mantê-lo trabalhando em direção a esse resultado, [defina uma meta](/docs/pt/goal).
* **Entregue-lhe problemas ambíguos**: investigações de causa raiz, depuração de interrupção e decisões de arquitetura são onde a investigação e verificação extras compensam.
* **Pule os lembretes de verificação**: ele verifica seu próprio trabalho com menos solicitação, então lembretes para testar ou verificar geralmente são desnecessários.
* **Dimensione tarefas maiores**: entregue-lhe trabalho que você normalmente dividiria em pedaços. Ele mantém sessões longas sem perder o fio.

<Note>
  Fable 5.1 requer Claude Code v2.1.257 ou posterior. Se uma solicitação para ele de uma versão mais antiga falhar, consulte [Claude Code não suporta este modelo](/docs/pt/errors#claude-code-does-not-support-this-model). Execute `claude update` para atualizar. Para disponibilidade sob retenção zero de dados, consulte [Disponibilidade de modelo sob ZDR](/docs/pt/zero-data-retention#model-availability-under-zdr).
</Note>

Na API Anthropic, um modelo Fable aparece no seletor `/model` a menos que [`availableModels`](#restrict-model-selection) ou [restrições de modelo de organização](#organization-model-restrictions) o excluam. Quando sua organização não consegue usar Fable em absoluto, por exemplo sob [retenção zero de dados](/docs/pt/zero-data-retention#model-availability-under-zdr), a linha permanece no seletor acinzentada, com uma nota sobre o motivo.

<h4 id="fable-and-usage-credits">
  Fable e créditos de uso
</h4>

Dependendo do seu plano e nível de assento, o uso de Fable pode ser cobrado em [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) em vez de usar os limites incluídos do seu plano. Quando isso acontece, o seletor `/model` mostra "Requer créditos de uso" na linha Fable. Para gerenciar créditos de uso, consulte [Adicionar créditos de uso à sua assinatura](/docs/pt/costs#add-usage-credits-to-your-subscription).

Em sessões interativas, Claude Code mostra um prompt de consentimento antes de uma solicitação Fable cobrar créditos de uso. Membros de planos Enterprise com faturamento de organização não veem o prompt. Você pode continuar no Fable usando créditos de uso ou mudar para seu modelo padrão. Você também pode descartar o prompt:

* No seletor `/model`, você mantém seu modelo atual.
* No meio da sessão, Claude Code continua a vez no seu modelo padrão.

Depois que você escolhe continuar no Fable usando créditos de uso, Claude Code não mostra o prompt novamente.

Em uma sessão com [Remote Control](/docs/pt/remote-control) conectado, uma [sessão em segundo plano](/docs/pt/agent-view), ou uma sessão de colega de [equipe de agentes](/docs/pt/agent-teams), ninguém pode estar no terminal, então Claude Code mantém o prompt de consentimento no meio da sessão para o prazo [`dialogExpiry`](/docs/pt/settings-reference#dialogexpiry), cinco minutos por padrão. Se ninguém tiver respondido até o prazo, Claude Code encerra a vez sem enviar a solicitação e adiciona um aviso à transcrição, que o cliente Remote Control também mostra. Sua seleção de modelo não é alterada, e Claude Code pede consentimento novamente na sua próxima mensagem.

O que você pode fazer enquanto o prompt está aguardando depende da sessão:

* Com Remote Control conectado ou em uma sessão de colega, pressione qualquer tecla no terminal para cancelar o prazo, e Claude Code aguarda sua resposta.
* Em uma sessão em segundo plano, responda antes do prazo.
* Se você enviar uma nova mensagem do cliente remoto antes de alguém digitar no terminal, Claude Code encerra a vez da mesma forma, e sua nova mensagem inicia a próxima vez. Depois que alguém digita no terminal, Claude Code continua aguardando a resposta e coloca sua nova mensagem na fila atrás dela.

No [modo não interativo](/docs/pt/headless) com a flag `-p` e através do Agent SDK, Claude Code nunca mostra o prompt de consentimento. Quando uma solicitação Fable lá cobraria créditos de uso, Claude Code a cobra sem perguntar.

<h3 id="setting-your-model">
  Definindo seu modelo
</h3>

Você pode configurar seu modelo de várias maneiras, listadas em ordem de prioridade:

1. **Durante a sessão**: use `/model <alias|name>` para mudar imediatamente, ou execute `/model` sem argumento para abrir o seletor. Consulte [quando Claude Code pede que você confirme a mudança](/docs/pt/prompt-caching#switching-models)
2. **Na inicialização**: inicie com `claude --model <alias|name>`
3. **Variável de ambiente**: defina `ANTHROPIC_MODEL=<alias|name>`
4. **Configurações**: configure permanentemente em seu arquivo de configurações usando o campo `model`
5. **[Padrão para novas sessões](#set-a-default-model-for-new-sessions)**: defina `ANTHROPIC_DEFAULT_MODEL=<alias|name>`

`/model` salva sua escolha como o padrão para novas sessões escrevendo o campo `model` em suas configurações de usuário. No seletor:

* `Enter`: muda o modelo e salva como seu padrão
* `s`: muda o modelo apenas para esta sessão e deixa seu padrão inalterado. Para usar uma chave diferente, rebinde [`modelPicker:thisSessionOnly`](/docs/pt/keybindings#model-picker-actions)

Digitar `/model <name>` diretamente se comporta como `Enter`. Para mudar apenas para esta sessão, abra o seletor com `/model` e pressione `s` na linha do modelo.

Se você mudar modelos com `/model`, a mudança também alcança [subagentos que herdam o modelo da conversa principal](/docs/pt/sub-agents#choose-a-model), porque Claude Code resolve seu modelo a partir daquele que sua sessão está usando quando Claude os inicia. Mude para Opus antes de Claude delegar pesquisa ou execuções de teste para um deles, e esse trabalho é executado no Opus também. Para manter um subagentos personalizado em um modelo menor, defina `model` em sua definição.

Se você definir um modelo com `/model` no [modo não interativo](/docs/pt/headless), com a flag `-p`, sua escolha se aplica apenas à sessão atual e não é salva como seu padrão; `/model` nesse modo requer Claude Code v2.1.205 ou posterior. As configurações de projeto e gerenciadas ainda têm precedência e se reaplicam no próximo lançamento. Um [modelo padrão de organização](#organization-default-model) que seu administrador configurou para substituir a seleção do usuário também se reaplica no próximo lançamento.

Na v2.1.144 até v2.1.152, `/model` se aplicava apenas à sessão atual e `d` no seletor salvava um padrão.

A flag `--model` e a variável de ambiente `ANTHROPIC_MODEL` se aplicam apenas à sessão que você inicia com elas. Para executar modelos diferentes em terminais diferentes ao mesmo tempo, inicie cada um com sua própria flag `--model` em vez de mudar com `/model`.

Os preços no seletor `/model` aparecem quando Claude Code fala com a API Anthropic, diretamente ou através de um [gateway LLM](/docs/pt/llm-gateway) que a proxeia, e o preço em uma linha é o preço do modelo que essa linha seleciona. Em [provedores de terceiros](/docs/pt/third-party-integrations) como Amazon Bedrock e no [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway), seu provedor ou gateway determina o que você paga, então as linhas do seletor não mostram preço. O preço é apenas um rótulo de exibição; não afeta qual modelo uma linha seleciona ou o que seu provedor cobra. Antes da v2.1.206, [Claude Platform on AWS](/docs/pt/claude-platform-on-aws) e sessões de gateway mostravam preços de lista Anthropic, e uma linha poderia mostrar o preço de um modelo diferente do que selecionava.

As sessões retomadas iniciadas com `claude --resume`, `--continue`, ou o seletor `/resume` mantêm o modelo que estavam usando quando a transcrição foi salva, independentemente da configuração `model` atual. Se o modelo restaurado foi descontinuado ou é excluído por [`availableModels`](#restrict-model-selection), a sessão cai para a ordem de precedência normal. Isso evita que a escolha `/model` de outra sessão mude o modelo ao retomar. Em provedores que usam IDs de implantação específicos do provedor em vez de IDs de modelo Anthropic, como Amazon Bedrock, Agent Platform do Google Cloud e Microsoft Foundry, o modelo de transcrição não é restaurado e a sessão resolve seu modelo através da ordem de precedência normal.

Um modelo que você escolhe para o novo lançamento com `--model` ou `ANTHROPIC_MODEL` ainda tem precedência sobre o modelo restaurado. A partir da v2.1.195, também uma variável da família [`ANTHROPIC_DEFAULT_OPUS_MODEL`](#environment-variables). [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions) também pode, sob as condições listadas em sua seção.

Quando o modelo ativo na inicialização vem das configurações de projeto ou gerenciadas em vez de sua própria seleção, o cabeçalho de inicialização mostra qual arquivo de configurações o definiu. Execute `/model` para substituir; a configuração de projeto ou gerenciada se reaplica no próximo lançamento. Em plataformas que incorporam Claude Code e definem [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/pt/env-vars), a configuração de modelo do host tem precedência sobre as configurações de modelo gerenciadas, enquanto uma lista de permissões `availableModels` gerenciada permanece em vigor a menos que o host forneça a sua própria; [Exceções à precedência de configurações gerenciadas](/docs/pt/settings#exceptions-to-managed-settings-precedence) diz quais chaves e variáveis o host substitui.

Se você ou sua organização configurarem [hooks PreModelSwitch](/docs/pt/hooks#premodelswitch), eles são executados antes de uma mudança solicitada ser aplicada e podem bloqueá-la ou pedir que você confirme.

Quando Claude Code não consegue dizer quais hooks PreModelSwitch seus [plugins gerenciados](/docs/pt/settings-reference#enabledplugins) da organização entregam, por exemplo porque um plugin gerenciado falhou ao carregar, ele recusa a mudança em vez de aplicá-la sem verificação, e verifica novamente em cada nova tentativa. Consulte [A mudança de modelo foi bloqueada por um hook PreModelSwitch](/docs/pt/errors#model-switch-was-blocked-by-a-premodelswitch-hook) para a mensagem e recuperação.

Quando você muda modelos através do método `setModel()` do [Agent SDK](/docs/pt/agent-sdk/overview) ou de um dispositivo conectado através de [Remote Control](/docs/pt/remote-control), ou um aplicativo como o [Desktop app](/docs/pt/desktop) que executa o CLI do Claude Code muda para você, Claude Code verifica se a string é uma que ele reconhece antes de salvá-la. Esta verificação requer Claude Code v2.1.200 ou posterior. Verificar uma escolha Remote Control requer Claude Code v2.1.260 ou posterior em sua máquina. Na API Anthropic, Claude Code reconhece:

* um alias de modelo
* uma entrada do seletor `/model`
* qualquer nome que comece com `claude-`
* um valor que você configurou como uma [opção de modelo personalizado](#add-a-custom-model-option) ou em [`modelOverrides`](#override-model-ids-per-version)

Claude Code rejeita uma string não reconhecida com `Model "<name>" is not a recognized model id.` e a sessão mantém seu modelo atual, em vez de salvar a string e falhar na próxima solicitação. Consulte [a referência de erro](/docs/pt/errors#model-is-not-a-recognized-model-id) para etapas de recuperação.

A verificação é executada apenas na API Anthropic. No Amazon Bedrock, Agent Platform do Google Cloud, Microsoft Foundry, [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), e atrás de um [gateway LLM](/docs/pt/llm-gateway) ou um `ANTHROPIC_BASE_URL` personalizado, seu provedor ou gateway define os nomes de modelo, então Claude Code passa qualquer string sem verificá-la. A verificação também não cobre a flag `--model`, a variável de ambiente `ANTHROPIC_MODEL`, ou a configuração `model`; um valor digitado incorretamente lá produz [Há um problema com o modelo selecionado](/docs/pt/errors#theres-an-issue-with-the-selected-model) na primeira solicitação. Claude Code ainda pode escrever a [linha de diagnóstico de modelo não reconhecido](/docs/pt/errors#unrecognized-model-id-on-a-request) no tempo de solicitação, em todos os provedores.

Quando o modelo solicitado tem uma data de aposentadoria programada ou é automaticamente remapeado para uma versão mais recente, Claude Code mostra um aviso que nomeia o modelo solicitado. Sessões interativas o mostram como um aviso de inicialização. A partir da v2.1.182, o mesmo aviso é escrito em stderr no [modo não interativo](/docs/pt/headless) ao usar o formato de saída de texto padrão. A verificação também cobre um `model` definido no [frontmatter de subagentos](/docs/pt/sub-agents). O aviso stderr é suprimido para `--output-format json` e `stream-json`; leia o modelo real do campo `modelUsage` da [mensagem de resultado](/docs/pt/headless#get-structured-output).

Por exemplo, inicie uma sessão no Opus:

```bash theme={null}
claude --model opus
```

Depois mude modelos dentro da sessão:

```text theme={null}
/model sonnet
```

Arquivo de configurações de exemplo:

```json theme={null}
{
    "permissions": {
        "allow": ["Bash(npm run lint)"]
    },
    "model": "opus"
}
```

<h4 id="set-a-default-model-for-new-sessions">
  Defina um modelo padrão para novas sessões
</h4>

Defina `ANTHROPIC_DEFAULT_MODEL=<alias|name>` para escolher o modelo em que suas sessões iniciam por padrão. Requer Claude Code v2.1.236 ou posterior.

Claude Code inicia uma nova sessão no modelo da variável apenas quando nenhum destes seleciona um modelo:

* A flag `--model`
* `ANTHROPIC_MODEL`
* Um valor `model` em qualquer arquivo de configurações, incluindo a escolha que você salva com `/model`
* Um [modelo padrão de organização](#organization-default-model)

Uma escolha que você salva com `/model` tem precedência sobre a variável em lançamentos posteriores também. Com `ANTHROPIC_MODEL` definido em vez disso, Claude Code retorna ao modelo da variável no próximo lançamento, seja qual for o que você salvou com `/model`.

Claude Code também resolve a opção Padrão para o modelo da variável, a menos que um modelo padrão de organização se aplique. Quando a opção Padrão é resolvida para o modelo da variável, a linha Padrão no seletor `/model` mostra o rótulo Definido por ANTHROPIC\_DEFAULT\_MODEL.

Claude Code ignora a variável nestes casos, e a opção Padrão é resolvida como se você não a tivesse definido:

* Você a definiu como `default`, `inherit`, `opusplan`, ou `haiku`
* [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) está ativado
* [`availableModels`](#restrict-model-selection) ou [restrições de modelo de organização](#organization-model-restrictions) excluem o modelo
* O modelo não está disponível para sua conta

Quando uma nova sessão iniciaria no modelo da variável, uma sessão que você retoma com `claude --resume`, `--continue`, ou o seletor `/resume` também inicia nele. Claude Code não restaura o modelo salvo na transcrição dessa sessão. Caso contrário, Claude Code não usa a variável quando você [retoma uma sessão](#setting-your-model).

<h4 id="a-new-session-starts-on-a-different-model-than-you-picked">
  Uma nova sessão inicia em um modelo diferente do que você escolheu
</h4>

Quando você escolhe um modelo com `/model` e sua próxima sessão inicia em algo diferente, estas são as causas usuais:

* **Você o escolheu para uma sessão.** Pressionar `s` no seletor, iniciar com `--model`, e executar `/model` no modo não interativo se aplicam apenas à sessão atual e deixam seu padrão salvo intacto.
* **Algo com prioridade mais alta define o modelo.** Um valor `model` nas configurações de projeto ou gerenciadas, `ANTHROPIC_MODEL` em seu shell, ou um [padrão de organização](#organization-default-model) que seu administrador definiu para substituir as escolhas do usuário se aplica novamente em cada lançamento. Sua escolha `/model` ainda está salva; está sendo superada. Quando as configurações de projeto ou gerenciadas definem o modelo, o cabeçalho de inicialização nomeia o arquivo.
* **Claude Code não conseguiu salvar sua escolha.** `/model` escreve `model` em `~/.claude/settings.json`. Se você não conseguir escrever nesse arquivo, por exemplo porque outra ferramenta o gera ou o vincula a uma cópia somente leitura, o modelo que você escolheu dura pela sessão e o próximo lançamento lê o valor antigo. Defina `model` na ferramenta que gera o arquivo, ou torne o arquivo gravável. Consulte [Uma mudança que você fez no Claude Code é perdida em novas sessões](/docs/pt/settings#a-change-you-made-in-claude-code-is-lost-in-new-sessions).
* **Você retomou uma sessão.** Uma sessão que você retoma com `claude --resume` ou `--continue` geralmente [mantém o modelo que estava usando](#setting-your-model) em vez de seu padrão atual.

<h2 id="restrict-model-selection">
  Restringir seleção de modelo
</h2>

Administradores corporativos podem usar `availableModels` em [configurações gerenciadas ou de política](/docs/pt/managed-settings) para restringir quais modelos os usuários podem selecionar. As entradas correspondem a uma família de modelos como `sonnet`, um prefixo de versão como `claude-sonnet-4-5`, ou um ID de modelo completo como `claude-sonnet-4-5-20250929`. Um prefixo de versão também corresponde a IDs de modelo posteriores que o estendem com outro segmento, portanto `claude-fable-5` permite tanto Fable 5 quanto Fable 5.1, enquanto `claude-fable-5-1` permite apenas Fable 5.1.

Em plataformas que incorporam Claude Code e definem [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/pt/env-vars), a configuração de modelo do host tem precedência sobre as configurações de modelo gerenciadas, enquanto uma lista de permissões `availableModels` gerenciada permanece em vigor a menos que o host forneça a sua própria; [Exceções à precedência de configurações gerenciadas](/docs/pt/settings#exceptions-to-managed-settings-precedence) diz quais chaves e variáveis o host substitui.

Quando `availableModels` é definido, a lista de permissões se aplica em todos os lugares onde um usuário pode especificar um modelo:

* **Modelo de sessão principal**: `/model`, a flag `--model`, a variável de ambiente `ANTHROPIC_MODEL`, a configuração `model`, [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), e o modelo restaurado ao [retomar uma sessão](#setting-your-model)
* **Resolução de alias**: as variáveis de ambiente `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL` e `ANTHROPIC_DEFAULT_FABLE_MODEL` não podem redirecionar um alias permitido para um modelo fora da lista
* **Modo rápido**: `/fast` recusa alternar quando isso implicaria mudar implicitamente para um modelo Opus fora da lista, com a mensagem "is not in your organization's allowed models"
* **Modelos de subagente e colega**: o campo `model` em [subagente](/docs/pt/sub-agents#choose-a-model) frontmatter, o parâmetro `model` da ferramenta Agent, modelos de colega de [equipe de agente](/docs/pt/agent-teams#specify-teammates-and-models), `CLAUDE_CODE_SUBAGENT_MODEL`, e, na v2.1.197 e anteriores, o seletor de modelo no assistente `/agents`&#x20;
* **Modelos de skill e comando**: o frontmatter `model` em [skills e comandos](/docs/pt/skills)
* **Modelo de advisor**: a configuração [`advisorModel`](/docs/pt/advisor) configurada e a flag `--advisor`
* **Modelo de agente de fundo**: o modelo selecionado no [seletor de dispatch](/docs/pt/agent-view)

Na API Anthropic e [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), um alias de família de modelo, `opus`, `sonnet`, `haiku` ou `fable`, resolve para seu modelo usual quando a lista de permissões permite esse modelo. Quando a lista de permissões bloqueia esse modelo, Claude Code substitui a versão mais recente da família que a lista de permissões permite e mostra um aviso nomeando os modelos solicitado e substituído. Com `["sonnet", "claude-opus-4-6"]`, por exemplo, tanto `/model opus` quanto `--model opus` selecionam Claude Opus 4.6, o Opus mais recente permitido. Antes da v2.1.205, um alias cuja versão mais recente lançada estava fora da lista era rejeitado ou substituído como qualquer outra seleção bloqueada, mesmo quando a lista permitia uma versão mais antiga.

A substituição precisa de uma versão permitida para pousar: quando a lista de permissões não permite nenhuma versão da família do alias, o alias segue o comportamento de rejeição e substituição abaixo como qualquer outro valor bloqueado.

Claude Code lida com qualquer outra seleção bloqueada de acordo com onde o modelo foi definido:

* **`/model`**: Claude Code rejeita a mudança com um erro
* **Flag `--model`, `ANTHROPIC_MODEL` ou a configuração `model`**: Claude Code substitui o valor na inicialização com um aviso nomeando os modelos solicitado e substituído, e a sessão inicia no modelo padrão
* **[`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions)**: Claude Code ignora a variável
* **Substituição de subagente ou colega**: Claude Code executa o subagente ou colega em um modelo de fallback em vez de falhar na solicitação. Veja [Escolher um modelo](/docs/pt/sub-agents#choose-a-model) para o fallback do subagente e [Especificar colegas e modelos](/docs/pt/agent-teams#specify-teammates-and-models) para o fallback do colega.

  Em sessões interativas, Claude Code avisa quando substitui o modelo de um subagente, por este fallback ou pela substituição de versão mais recente permitida acima, nomeando os modelos solicitado e substituído; não relata um fallback de colega.

  Onde a substituição de versão mais recente permitida acima opera, um alias de família bloqueado a segue. Antes da v2.1.222, um alias caía de volta como qualquer outro valor bloqueado em cada provedor
* **Substituição de skill ou comando**: Claude Code ignora a substituição, incluindo um alias de família bloqueado, e o skill ou comando é executado no modelo de sessão. Um skill ou comando que [é executado em um subagente](/docs/pt/skills#run-skills-in-a-subagent) segue o comportamento do subagente acima
* **Configuração `advisorModel`**: o advisor é desabilitado para a sessão
* **Flag `--advisor`**: Claude Code sai com um erro na inicialização. Em uma [sessão de fundo](/docs/pt/agent-view), inicia a sessão sem o advisor em vez de sair

Claude Code oculta modelos excluídos do seletor `/model`. Um ID de modelo completo na lista que não tem linha de seletor integrada, como uma versão mais antiga que a lista fixa, aparece no seletor `/model` como sua própria linha rotulada, a menos que Claude Code substitua as opções integradas por um alinhamento [`modelPicker`](/docs/pt/settings-reference#modelpicker). Antes da v2.1.199, tal ID era selecionável apenas digitando `/model <id>`.

Mudanças de modelo que Claude Code faz em seu nome são verificadas da mesma forma:

* **[Cadeias de modelo de fallback](#fallback-model-chains)**: entradas fora da lista de permissões são descartadas
* **Atualizações de modo de plano**: na API Anthropic e Claude Platform on AWS, uma atualização como [`opusplan`](#opusplan-model-setting) para um modelo excluído usa a versão mais recente permitida da família de atualização. Em provedores com IDs de modelo específicos do provedor, e quando nenhuma versão é permitida, a atualização é ignorada e o planejamento continua no modelo da sessão
* **[Fallback automático de modelo](#automatic-model-fallback)**: um fallback cujo alvo é excluído não é executado, portanto a solicitação sinalizada termina com uma recusa
* **[Classificador de modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode)**: o padrão Claude Sonnet 5 do classificador se aplica apenas quando a lista de permissões permite Sonnet 5. Quando é excluído, o classificador é executado no modelo da sessão, que a lista de permissões já governa, ou em um modelo Opus quando a sessão é executada em um [modelo Fable](#work-with-fable). Em provedores diferentes da API Anthropic, esse fallback Opus é executado no modelo Opus padrão do provedor sem consultar a lista de permissões. Requer Claude Code v2.1.210 ou posterior
* **[Modo rápido](/docs/pt/fast-mode)**: ativar o modo rápido é recusado quando o modelo em que a sessão seria executada depois está fora da lista de permissões

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

<h3 id="surface-coverage">
  Cobertura de superfície
</h3>

Cada superfície aplica a lista de permissões que recebe. Qual mecanismo de entrega alcança cada superfície difere:

| Mecanismo de entrega                                                                               | CLI e IDE | Sessões locais de desktop | Sessões web, móvel e na nuvem                                                                                                                                                                                                                                      | Agent SDK e não interativo | Cowork                   |
| :------------------------------------------------------------------------------------------------- | :-------- | :------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------- | :----------------------- |
| [Configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) do console de administração | Aplicado  | Aplicado                  | Aplicado                                                                                                                                                                                                                                                           | Aplicado                   | Não entregue             |
| [MDM ou arquivos de configurações gerenciadas](/docs/pt/managed-settings#delivery-mechanisms)           | Aplicado  | Aplicado                  | Não entregue em ambientes hospedados pela Anthropic; em [ambientes auto-hospedados](/docs/pt/self-hosted-environments), aplicado da imagem do executor por [como Claude Code combina fontes gerenciadas](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) | Aplicado                   | Aplicado onde implantado |

* Sessões na nuvem, em [Claude Code na web](/docs/pt/claude-code-on-the-web) ou no aplicativo Desktop, são executadas em VMs gerenciadas pela Anthropic por padrão: as configurações implantadas em seu dispositivo não as alcançam, portanto entregue a lista de permissões através de configurações gerenciadas pelo servidor. Sessões que sua organização roteia para um [ambiente auto-hospedado](/docs/pt/self-hosted-environments) são executadas em sua própria computação e também leem o arquivo de configurações gerenciadas na imagem do executor. [Como Claude Code combina fontes gerenciadas](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) diz quando esse arquivo se aplica. Uma mudança de modelo no meio da sessão em uma sessão na nuvem é rejeitada quando o modelo solicitado é excluído pela lista de permissões. Quando a `availableModels` lista em suas configurações gerenciadas pelo servidor é não vazia, o servidor rejeita a solicitação de um usuário para iniciar uma sessão na nuvem em um modelo que a lista exclui.
* Cowork, a aba de trabalho agentic no aplicativo Claude Desktop, executa suas sessões em Claude Code, mas, por design, não recebe configurações gerenciadas pelo servidor do console de administração claude.ai. Um arquivo de configurações gerenciadas se aplica a sessões Cowork quando está presente onde a sessão é executada; sessões Cowork remotas são executadas em VMs gerenciadas pela Anthropic, onde um arquivo implantado no dispositivo não está presente.
* Sessões em [provedores de terceiros](/docs/pt/server-managed-settings#platform-availability) como Amazon Bedrock, Agent Platform do Google Cloud, Microsoft Foundry e [Claude Platform on AWS](/docs/pt/claude-platform-on-aws) não recebem configurações gerenciadas pelo servidor, portanto entregue a lista de permissões através de MDM ou arquivos de configurações gerenciadas lá.
* A entrega gerenciada pelo servidor também requer que a sessão se autentique com um [login ou chave elegível](/docs/pt/server-managed-settings#platform-availability). Frotas que geram chaves apenas através de um script [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper) devem entregar a lista de permissões através de MDM ou arquivos de configurações gerenciadas.
* A aba Desktop Code também hospeda [sessões SSH](/docs/pt/desktop#ssh-sessions), que leem o arquivo de configurações gerenciadas do host remoto em que são executadas. Veja [Configurações gerenciadas de desktop](/docs/pt/desktop#managed-settings).
* Os seletores de modelo em claude.ai e no aplicativo Desktop ocultam ou desabilitam modelos excluídos pela lista de permissões de sua organização. O estado do seletor é uma conveniência para usuários; não é onde a aplicação acontece.

<h3 id="default-model-behavior">
  Comportamento do modelo padrão
</h3>

Por si só, `availableModels` deixa a opção Padrão no [padrão de tempo de execução](#default-model-setting) do sistema para a conta até que você também defina [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model). Se esse padrão for um modelo que você pretende restringir, defina `enforceAvailableModels` também.

Uma matriz `availableModels` vazia nunca ativa a aplicação do modelo Padrão: com `availableModels: []`, as seleções de modelo nomeadas são bloqueadas, mas o modelo Padrão para o tipo de conta permanece utilizável independentemente de `enforceAvailableModels`.

<h3 id="enforce-the-allowlist-for-the-default-model">
  Aplicar a lista de permissões para o modelo Padrão
</h3>

Defina `enforceAvailableModels: true` junto com um `availableModels` não vazio em configurações gerenciadas para estender a lista de permissões à opção Padrão. Isso requer Claude Code v2.1.175 ou posterior.

```json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

A opção Padrão resolve para o padrão do tipo de conta, ou para o [modelo padrão da organização](#organization-default-model) quando um administrador definiu um. Quando esse modelo não está na lista de permissões, a opção Padrão em vez disso resolve para a primeira entrada `availableModels` que nomeia um modelo permitido e disponível, e a linha Padrão do seletor `/model` mostra esse modelo. Isso se aplica em todos os lugares onde o padrão é alcançado: inicialização da sessão, seleção de Padrão em `/model`, a palavra-chave `"default"` em [cadeias de modelo de fallback](#fallback-model-chains), e o fallback usado quando uma seleção excluída é descartada.

`enforceAvailableModels` remapeia a opção Padrão apenas quando `availableModels` é não vazio. Com `availableModels: []`, o modelo Padrão para o tipo de conta permanece utilizável, portanto a configuração não pode bloquear usuários de cada modelo. Quando `availableModels` é não vazio, mas nenhuma entrada resolve para um modelo permitido e disponível, a aplicação é ignorada e Padrão resolve para o padrão do tipo de conta, com um aviso visível apenas em `--debug`. Mantenha pelo menos uma entrada garantida como disponível na lista para evitar isso.

Implante ambas as chaves juntas na fonte gerenciada de classificação mais alta que você entrega. Por padrão, Claude Code lê apenas essa fonte, portanto um par colocado em um arquivo de configurações gerenciadas é ignorado quando o console de administração entrega qualquer configuração; sob a mesclagem de aceitação em [como Claude Code combina fontes gerenciadas](/docs/pt/managed-settings#how-claude-code-combines-managed-sources), Claude Code ainda ignora um mapa `modelOverrides` de uma fonte classificada abaixo daquela que define `availableModels`.

<h3 id="control-the-model-users-run-on">
  Controlar o modelo em que os usuários são executados
</h3>

A configuração `model` é uma seleção inicial, não aplicação. Define qual modelo está ativo quando uma sessão inicia, mas os usuários ainda podem abrir `/model` e escolher Padrão, que resolve para o [padrão de tempo de execução](#default-model-setting) do sistema independentemente do que `model` está definido, a menos que [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) o redirecione.

Para controlar totalmente a experiência do modelo, combine estas configurações:

* **`availableModels`**: restringe quais modelos nomeados os usuários podem alternar
* **`enforceAvailableModels`**: estende a lista de permissões `availableModels` à opção Padrão, portanto Padrão não pode resolver para um modelo fora da lista
* **`model`**: define a seleção de modelo inicial quando uma sessão inicia
* **`ANTHROPIC_DEFAULT_SONNET_MODEL`** / **`ANTHROPIC_DEFAULT_OPUS_MODEL`** / **`ANTHROPIC_DEFAULT_HAIKU_MODEL`** / **`ANTHROPIC_DEFAULT_FABLE_MODEL`**: controlam para o que os aliases `sonnet`, `opus`, `haiku` e `fable` resolvem, e qual versão o [padrão do tipo de conta](#default-model-setting) usa

Este exemplo inicia usuários em Sonnet 4.5, limita o seletor a Sonnet e Haiku, e garante que Padrão resolve para um modelo na lista de permissões em vez do padrão de nível:

```json theme={null}
{
  "model": "claude-sonnet-4-5",
  "availableModels": ["claude-sonnet-4-5", "haiku"],
  "enforceAvailableModels": true,
  "env": {
    "ANTHROPIC_DEFAULT_SONNET_MODEL": "claude-sonnet-4-5"
  }
}
```

Sem `enforceAvailableModels` ou o bloco `env`, um usuário que seleciona Padrão no seletor obtém o [padrão de tempo de execução](#default-model-setting) em vez da versão fixada em `model`. As duas configurações cobrem escopos diferentes: `enforceAvailableModels` faz Padrão obedecer à lista de permissões, enquanto o bloco `env` fixa qual versão um alias permitido como `sonnet` resolve. Use `enforceAvailableModels` sozinho quando restringir famílias de modelos é suficiente; adicione o bloco `env` quando você também precisa fixar uma versão específica.

<h3 id="merge-behavior">
  Comportamento de mesclagem
</h3>

Quando as configurações gerenciadas que Claude Code aplica definem `availableModels`, essa lista sozinha se aplica, além de uma [plataforma host que fornece a sua própria](/docs/pt/settings#exceptions-to-managed-settings-precedence): entradas em configurações de usuário, projeto ou local não podem estendê-la, e Claude Code nunca mescla `availableModels` entre fontes gerenciadas; [como Claude Code combina fontes gerenciadas](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) diz qual lista de fonte se aplica. Caso contrário, listas de configurações de usuário, projeto e local são [concatenadas e desduplicadas](/docs/pt/settings#settings-precedence) como outras configurações de matriz. Antes de Claude Code v2.1.175, entradas de escopos de precedência mais baixa se mesclavam na lista gerenciada em vez de serem substituídas por ela.

Dentro da lista efetiva, uma entrada nomeando um modelo específico em uma família, seja um prefixo de versão ou um ID de modelo completo, desabilita a entrada de curinga da família: `["sonnet", "claude-sonnet-4-5"]` permite apenas versões Sonnet 4.5, não cada modelo Sonnet.

<h3 id="mantle-model-ids">
  IDs de modelo Mantle
</h3>

Quando o [endpoint Amazon Bedrock Mantle](/docs/pt/amazon-bedrock#use-the-mantle-endpoint) está habilitado, entradas em `availableModels` que começam com `anthropic.` são adicionadas ao seletor `/model` como opções personalizadas e roteadas para o endpoint Mantle. Esta é uma exceção à correspondência de alias descrita em [Fixar modelos para implantações de terceiros](#pin-models-for-third-party-deployments). A configuração ainda restringe o seletor a entradas listadas, e um ID Mantle incorpora um nome de família, portanto conta como uma entrada específica e desabilita o curinga da família: junto com qualquer ID Mantle, liste os prefixos de versão ou IDs completos que você quer manter selecionáveis. Veja [Comportamento de mesclagem](#merge-behavior).

<h3 id="organization-model-restrictions">
  Restrições de modelo da organização
</h3>

Administradores de organização em planos Claude Enterprise restringem quais modelos os membros podem executar desabilitando modelos individuais no console de administração claude.ai. Esta restrição é entregue com os direitos da conta quando Claude Code se autentica, separada de qualquer lista `availableModels` em configurações, e o servidor aplica a mesma restrição independentemente quando uma sessão é criada. Requer Claude Code v2.1.187 ou posterior.

A restrição se aplica quando um membro faz login ou usa sua própria chave de API. Credenciais com escopo de organização, como chaves de serviço da organização, não estão vinculadas a um usuário, portanto a restrição não se aplica a elas.

O Claude Console não tem controle de restrição de modelo. Organizações sem um plano Claude Enterprise, incluindo aquelas cujos membros se autenticam através da API Anthropic, restringem modelos com [`availableModels`](#restrict-model-selection) em [configurações gerenciadas](/docs/pt/managed-settings), adicionando [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) para cobrir a opção Padrão. [Cobertura de superfície](#surface-coverage) diz como cada superfície recebe e aplica estas configurações.

Um modelo restrito é ocultado do seletor `/model`. Selecioná-lo pelo nome com `--model`, a variável de ambiente `ANTHROPIC_MODEL` ou a configuração `model` mostra o aviso `Model "<name>" is restricted by your organization's settings. Using <model> instead.` e a sessão inicia em um modelo permitido. Digitar `/model <name>` para um modelo restrito é rejeitado com `Model '<name>' is restricted by your organization's settings. Run /model to choose a different model.` e a sessão mantém seu modelo atual.

Um [alias de família de modelo](#restrict-model-selection) como `opus` resolve para seu modelo usual quando a organização o permite. Quando a organização restringe esse modelo, Claude Code substitui a versão mais recente da família que a organização permite, com o mesmo aviso de substituição. `/model <alias>` é rejeitado apenas quando cada versão de sua família é restrita; um alias definido com `--model`, `ANTHROPIC_MODEL` ou a configuração `model` ainda é substituído na inicialização nesse caso. Antes da v2.1.205, um alias de família era substituído ou rejeitado com base apenas em sua versão mais recente lançada, mesmo quando uma versão mais antiga era permitida.

Restrições se aplicam em toda a organização ou por função:

* Desabilitar um modelo no nível da organização o remove para cada membro.
* O acesso em nível de função concede diferentes modelos a diferentes funções personalizadas, e um membro que possui várias funções pode usar qualquer modelo que uma de suas funções concede.
* Modelos Haiku estão sempre disponíveis e não podem ser desabilitados, portanto cada membro mantém pelo menos um modelo utilizável.
* Uma mudança de acesso entra em vigor em novas solicitações dentro de cerca de um minuto; o seletor `/model` reflete isso na próxima vez que uma sessão inicia.

Ambas as restrições se aplicam juntas: um modelo é selecionável apenas quando é permitido por `availableModels` e não é restrito pela organização. Restrições de organização alcançam sessões na API Anthropic e implantações [gateway LLM](/docs/pt/llm-gateway) apenas; em qualquer outro provedor, use `availableModels`.

<h2 id="organization-default-model">
  Modelo padrão da organização
</h2>

Os administradores da organização em planos Claude Enterprise podem definir um modelo padrão para membros do Claude Code a partir do console de administração claude.ai, para toda a organização ou por função personalizada. Quando um é definido, a opção Padrão é resolvida para esse modelo. Requer Claude Code v2.1.196 ou posterior.

A linha Padrão no seletor `/model` mostra o nome do padrão da organização com o rótulo Padrão da org. O rótulo lê Padrão da org se o administrador definiu o padrão para toda a organização ou para sua função. Um padrão de função cobre membros dessa função personalizada e tem precedência sobre o padrão em toda a organização; quando várias de suas funções definem padrões diferentes, o modelo mais capaz se aplica.

O padrão da organização é um ponto de partida, não uma restrição. Essas seleções têm precedência sobre ele:

* a flag `--model` e a variável de ambiente `ANTHROPIC_MODEL`
* um valor `model` em [configurações gerenciadas](/docs/pt/managed-settings) ou fornecido através de `--settings`
* um valor `model` em suas configurações de usuário, projeto ou local, incluindo um modelo que você salva com `/model`

Os administradores também podem configurar o padrão da organização para substituir a seleção do usuário. Com a substituição ativada, ela tem precedência sobre o valor `model` em configurações de usuário, projeto e local, portanto um modelo que você salva com `/model` se aplica para a sessão atual e o padrão da organização retorna no próximo lançamento. Quando sua seleção difere, `/model` mostra `O padrão da sua organização (<model>) se aplica ao reiniciar`. A flag `--model`, `ANTHROPIC_MODEL`, configurações gerenciadas e `--settings` ainda têm precedência mesmo com a substituição ativada. A substituição está disponível para um conjunto limitado de organizações; pergunte ao seu gerente de conta da Anthropic sobre disponibilidade.

Para limitar quais modelos os membros podem selecionar, use [restrições de modelo da organização](#organization-model-restrictions) ou [`availableModels`](#restrict-model-selection) em vez disso.

Claude Code lê o padrão da organização uma vez na inicialização, portanto um padrão que o administrador altera durante a sessão entra em vigor no próximo lançamento.

Quando o padrão da organização não substitui a seleção do usuário, o primeiro lançamento interativo após o administrador alterá-lo limpa a chave `model` de suas configurações de usuário uma vez, para que o novo padrão se aplique. Ele não altera nada mais no arquivo, e um modelo que você salva com `/model` após esse lançamento é mantido.

O padrão da organização passa por essas verificações de restrição antes de ser adotado:

* [`availableModels`](#restrict-model-selection) por si só não se aplica ao padrão da organização, portanto um padrão da organização fora da lista de permissões ainda se aplica. Quando [`enforceAvailableModels`](#enforce-the-allowlist-for-the-default-model) também está definido, um padrão da organização fora da lista de permissões é remapeado para a primeira entrada da lista de permissões, como qualquer outro Padrão
* um padrão da organização que [restrições de modelo da organização](#organization-model-restrictions) negam para sua conta é substituído pelo modelo mais recente permitido em sua família, ou uma família de menor custo quando cada versão dela é restrita
* um padrão da organização que não está disponível para sua conta é ignorado, e a opção Padrão é resolvida como seria [sem um padrão da organização](#default-model-setting)

A partir da v2.1.199, quando o padrão da organização é uma família de modelo diferente do padrão usual do tipo de conta, o seletor `/model` mantém uma linha separada para essa família usual, para que você ainda possa alternar para ela em uma sessão. Na v2.1.196 até v2.1.198 essa linha está faltando no seletor.

O padrão da organização alcança apenas sessões autenticadas com a API Anthropic. Para definir um padrão em qualquer outro lugar, incluindo implantações de [gateway LLM](/docs/pt/llm-gateway), use a chave `model` em [configurações gerenciadas](/docs/pt/managed-settings) em vez disso.

<h2 id="organization-effort-limits">
  Limites de esforço da organização
</h2>

Sua organização pode limitar o [nível de esforço](#adjust-effort-level) de duas maneiras. Em um plano Claude Enterprise, os administradores da organização definem limites de esforço por função, descritos abaixo. Em qualquer plano e qualquer provedor, incluindo Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, a configuração gerenciada [`maxEffortLevel`](/docs/pt/settings-reference#maxeffortlevel) limita o esforço no cliente. Quando ambos se aplicam a um modelo, o limite inferior se aplica.

Os administradores da organização em planos Claude Enterprise podem definir um [nível de esforço](#adjust-effort-level) máximo por modelo para cada função personalizada, juntamente com [restrições de modelo da organização](#organization-model-restrictions) no nível da função. Os níveis acima do limite não são oferecidos no seletor `/effort`, e nomear um nível superior com `--effort` ou `/effort` é executado no limite. Em sessões interativas e execuções simples de texto `--print`, um aviso nomeia os níveis solicitados e aplicados; com saída `json` ou `stream-json` ou em agentes em segundo plano, o limite é aplicado silenciosamente. Os limites são por modelo, portanto, alternar modelos pode mudar quais níveis estão disponíveis. Quando várias de suas funções concedem o mesmo modelo, o limite menos restritivo se aplica. Requer Claude Code v2.1.195 ou posterior.

Os limites de esforço são entregues juntamente com [restrições de modelo da organização](#organization-model-restrictions) e chegam às mesmas sessões.

<h2 id="special-model-behavior">
  Comportamento especial do modelo
</h2>

<h3 id="default-model-setting">
  Configuração do modelo `default`
</h3>

O comportamento de `default` depende do tipo de sua conta:

* **Pro, Max, Team, Enterprise e Anthropic API**: padrão para Opus 5.5
* **Claude Platform on AWS, Amazon Bedrock e Google Cloud's Agent Platform**: padrão para Opus 5.5
* **Microsoft Foundry**: padrão para Sonnet 4.5

Antes da v2.1.280, `default` era resolvido para Sonnet 5 em Pro e Team Standard, e para Opus 5 em Max, Team Premium, Enterprise, Anthropic API, Claude Platform on AWS, Amazon Bedrock e Google Cloud's Agent Platform a partir de v2.1.219. Antes da v2.1.219, `default` era resolvido para Opus 4.8 na Anthropic API, Max, Team Premium e Enterprise com pagamento conforme o uso a partir de v2.1.154, e na Claude Platform on AWS, Amazon Bedrock e Google Cloud's Agent Platform a partir de v2.1.207. Antes da v2.1.207, `default` era resolvido para Opus 4.7 na Claude Platform on AWS e para Sonnet 4.5 no Amazon Bedrock e Google Cloud's Agent Platform.

Quando um administrador definiu um [modelo padrão da organização](#organization-default-model), `default` é resolvido para esse modelo em vez do padrão do tipo de conta acima. Requer Claude Code v2.1.196 ou posterior. `default` também pode ser resolvido para o modelo que você definiu com [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), sob as condições listadas em sua seção.

Quando as configurações gerenciadas [aplicam a lista de permissões para o modelo Default](#enforce-the-allowlist-for-the-default-model) e o padrão do tipo de conta não está em `availableModels`, `default` é resolvido para o Default aplicado em vez do padrão do tipo de conta acima. Quando ambos se aplicam, o padrão da organização substitui o padrão do tipo de conta primeiro e a aplicação é feita em seguida: um padrão da organização na lista de permissões é mantido, enquanto um fora da lista é resolvido para o Default aplicado.

Os modelos Fable não são o padrão do tipo de conta em nenhum plano ou provedor. Escolher um com `/model` o salva como o modelo selecionado em suas configurações de usuário, para que as sessões posteriores iniciem nele. Para a alteração única que Claude Code faz em uma seleção Fable 5 salva na v2.1.257, consulte [Trabalhar com Fable](#work-with-fable).

<h3 id="opusplan-model-setting">
  Configuração do modelo `opusplan`
</h3>

O alias do modelo `opusplan` fornece uma abordagem híbrida automatizada:

* **No modo de plano**: usa `opus` para raciocínio complexo e decisões de arquitetura
* **No modo de execução**: alterna automaticamente para `sonnet` para geração de código e implementação

Isso combina o raciocínio do Opus para planejamento com a eficiência do Sonnet para execução.

A fase Opus do modo de plano usa a mesma janela de contexto que a configuração do modelo `opus`, e a fase de execução usa a mesma janela que `sonnet`. Quando `opus` e `sonnet` são resolvidos para modelos que são executados com a [janela de contexto de 1M](#extended-context) por padrão, como os modelos atuais fazem na Anthropic API, ambas as fases são executadas com ela. Para solicitar contexto de 1M para ambas as fases onde não o fazem, [defina o modelo](#setting-your-model) para `opusplan[1m]`, por exemplo com `/model opusplan[1m]`. Defini-lo com `/model` requer Claude Code v2.1.265 ou posterior; em versões anteriores, use o sinalizador `--model` ou a configuração `model` em vez disso.

Quando [`availableModels`](#restrict-model-selection) exclui o Opus mais recente mas permite uma versão mais antiga, por exemplo `["sonnet", "claude-opus-4-6"]`, `opusplan` usa o Opus mais recente permitido para planejamento e permanece apenas em Sonnet quando todo Opus é excluído. Uma sessão Haiku que normalmente seria atualizada para Sonnet no modo de plano também usa o Sonnet mais recente permitido, e permanece apenas em Haiku quando todo Sonnet é excluído. Antes da v2.1.205, o modo de plano permanecia no modelo da sessão sempre que a versão mais recente da família de atualização era excluída, mesmo quando a lista de permissões permitia uma mais antiga.

A substituição de uma versão mais antiga permitida se aplica na Anthropic API e [Claude Platform on AWS](/docs/pt/claude-platform-on-aws). No Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e Mantle, cujas implantações usam IDs de modelo específicos do provedor, o modo de plano permanece no modelo da sessão sempre que o modelo de atualização é excluído.

Para uma abordagem híbrida onde Claude decide no meio da tarefa quando consultar um segundo modelo em vez de alternar no limite do plano, consulte a [ferramenta advisor](/docs/pt/advisor).

<h3 id="fallback-model-chains">
  Cadeias de modelo de fallback
</h3>

Quando o modelo primário está sobrecarregado, indisponível ou retorna outro erro de servidor não repetível, Claude Code pode alternar para um modelo de fallback em vez de falhar na solicitação. Autenticação, faturamento, limite de taxa, tamanho de solicitação e erros de transporte, e uma [negação pela verificação de política da sua organização](/docs/pt/errors#automatic-retries), nunca acionam uma alternância; esses seguem sua manipulação normal de repetição e erro.

Configure um ou mais modelos de fallback e Claude Code os tenta em ordem, mostrando um aviso quando alterna. A alternância dura apenas para o turno atual, portanto sua próxima mensagem tenta o modelo primário primeiro novamente. Claude Code limita cadeias a três modelos após remoção de duplicatas e ignora entradas extras.

Defina uma cadeia para uma sessão com o sinalizador `--fallback-model`, que aceita uma lista separada por vírgulas:

```bash theme={null}
claude --fallback-model sonnet,haiku
```

Para persistir uma cadeia entre sessões, defina `fallbackModel` em [settings](/docs/pt/settings) como uma matriz:

```json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

O sinalizador `--fallback-model` tem precedência sobre a configuração `fallbackModel`. Cada entrada aceita um nome de modelo ou alias, e `"default"` se expande para o modelo padrão.

Claude Code não confirma a cadeia na inicialização e `/status` não a exibe. O aviso mostrado quando uma alternância acontece é o primeiro sinal visível de que um fallback está configurado.

Quando uma solicitação falha, Claude Code tenta cada entrada em ordem até que uma a aceite. Uma entrada que também não pode ser alcançada, como um modelo descontinuado fixado em configurações, falha para a próxima da mesma forma. Claude Code remove dois tipos de entrada antes dessa caminhada começar:

* **Fora da lista de permissões**: Claude Code descarta qualquer entrada não permitida por [`availableModels`](#restrict-model-selection) quando lê a cadeia.
* **Janela de contexto menor durante compactação**: a cadeia também cobre [compactação](/docs/pt/context-window#what-survives-compaction), mas Claude Code não fará fallback para um modelo com uma janela de contexto menor que a do primário, pois resumir lá cortaria parte da conversa primeiro. Se todo fallback for menor, a compactação mostra o erro original e você pode tentar novamente.

Claude Code também aplica a cadeia a [subagentes](/docs/pt/sub-agents). Quando a solicitação de um subagente falha, Claude Code tenta seus modelos de fallback configurados em ordem, e o subagente continua no modelo que aceita a solicitação. O modelo da sua sessão permanece inalterado. Antes da v2.1.247, uma falha que a cadeia cobria terminava o subagente.

<h3 id="automatic-model-fallback">
  Fallback automático de modelo
</h3>

Esta seção cobre fallback baseado em conteúdo de modelos Fable, Opus 5.5 e Opus 5. Para fallback baseado em disponibilidade quando um modelo está sobrecarregado ou indisponível, consulte [Cadeias de modelo de fallback](#fallback-model-chains).

Os modelos Fable, Opus 5.5 e Opus 5 são executados com classificadores de segurança, que na maioria das vezes sinalizam conteúdo de cibersegurança e biologia. Quando um classificador sinaliza uma solicitação e a categoria sinalizada tem um modelo de fallback, Claude Code executa novamente a solicitação nesse modelo e mostra um aviso na transcrição. Para essas duas categorias, o modelo de fallback depende de qual modelo recusou:

* **Fable 5.1, Fable 5 e Opus 5.5**: solicitações sinalizadas por biologia são executadas novamente em Opus 5, e solicitações sinalizadas por cibersegurança são executadas novamente em Opus 4.8.
* **Opus 5**: solicitações sinalizadas por cibersegurança são executadas novamente em Opus 4.8. Solicitações sinalizadas por biologia terminam com uma recusa, porque Opus 5 executa seus próprios classificadores de biologia sem modelo de fallback.

No Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, Claude Code resolve esses destinos através de sua implantação, e se você definir `ANTHROPIC_DEFAULT_OPUS_MODEL`, categorias que têm um fallback são executadas novamente no modelo fixado; consulte [Ativar fallback no Bedrock, Agent Platform e Foundry](#enable-fallback-on-bedrock-agent-platform-and-foundry).

Após um fallback, a sessão continua no modelo de fallback. Para retornar ao seu modelo original, execute [`/model`](#setting-your-model).

O fallback baseado em categoria requer Claude Code v2.1.219 ou posterior. Antes da v2.1.219, toda solicitação Fable 5 sinalizada era executada novamente no modelo Opus padrão do seu provedor, e Opus 5 não era uma fonte de fallback.

O modelo de fallback é verificado contra [`availableModels`](#restrict-model-selection). Quando é bloqueado, nenhum fallback ocorre. A recusa é mostrada como um erro normal e o modelo da sessão permanece inalterado.

<h4 id="check-what-triggered-fallback">
  Verificar o que acionou o fallback
</h4>

O fallback pode ser acionado na primeira solicitação de uma sessão, antes de você enviar algo incomum, porque a primeira solicitação carrega contexto do espaço de trabalho, como seu conteúdo CLAUDE.md e status do git. Um repositório que contém material de segurança ou biologia pode acionar o classificador apenas nesse contexto.

Para verificar se as personalizações são o gatilho, inicie uma sessão com `claude --safe-mode`, que desativa personalizações como CLAUDE.md, skills, servidores MCP e hooks. O status do git e nomes de diretórios não são personalizações e ainda estão inclusos.

<h4 id="ask-before-switching">
  Perguntar antes de alternar
</h4>

Para decidir o que acontece cada vez que uma solicitação é sinalizada, em vez de alternar automaticamente, execute `/config` e desative **Switch models when a message is flagged**, ou defina [`switchModelsOnFlag`](/docs/pt/settings-reference#switchmodelsonflag) como `false` em seu arquivo de configurações. Uma solicitação sinalizada pausa a sessão com duas opções: alternar para o modelo de fallback ou editar o prompt e tentar novamente no modelo atual.

Alguns casos se comportam diferentemente:

* Quando a categoria sinalizada não tem modelo de fallback, como um sinalizador de biologia em Opus 5, Claude Code não mostra o prompt e a solicitação termina com a recusa.
* Se ambos os modelos sinalizarem a mesma solicitação, você pode editar o prompt e tentar novamente ou iniciar uma nova sessão.
* Em sessões [Claude Code na web](/docs/pt/claude-code-on-the-web) no aplicativo móvel, edição e nova tentativa não são suportadas. Alterne modelos ou continue a sessão de um navegador de desktop ou do aplicativo de desktop.
* Em [modo não interativo](/docs/pt/cli-reference#cli-flags) e integrações SDK que não podem mostrar o prompt, uma solicitação sinalizada termina o turno com uma recusa.
* Quando o destino de fallback é bloqueado por [`availableModels`](#restrict-model-selection), Claude Code não mostra o prompt. A solicitação sinalizada termina com a recusa, o mesmo que fallback automático quando o destino é bloqueado.

<h4 id="enable-fallback-on-bedrock-agent-platform-and-foundry">
  Ativar fallback no Bedrock, Agent Platform e Foundry
</h4>

No [Amazon Bedrock](/docs/pt/amazon-bedrock), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai) e [Microsoft Foundry](/docs/pt/microsoft-foundry), IDs de modelo são específicos do provedor, portanto o fallback automático opera apenas quando Claude Code pode identificar ambos os modelos envolvidos:

* Claude Code deve reconhecer o modelo atual como uma fonte de fallback. Fable 5.1 e Fable 5 são reconhecidos quando o ID do modelo contém `claude-fable-5`, corresponde ao valor de `ANTHROPIC_DEFAULT_FABLE_MODEL` ou é mapeado com [`modelOverrides`](#override-model-ids-per-version). Opus 5.5 e Opus 5 são reconhecidos por seu ID de modelo do provedor ou um mapeamento [`modelOverrides`](#override-model-ids-per-version).
* O modelo de fallback deve ser resolvido em sua implantação. Se você definir `ANTHROPIC_DEFAULT_OPUS_MODEL`, solicitações sinalizadas são executadas novamente nesse modelo para cada categoria que tem um fallback; um sinalizador de biologia em Opus 5 ainda termina com uma recusa. Se você não o definir, solicitações sinalizadas por cibersegurança são executadas novamente em uma entrada Opus 4.8 na lista de modelos do provedor, e solicitações sinalizadas por biologia de um modelo Fable ou Opus 5.5 em uma entrada Opus 5.

Se nenhum dos modelos puder ser identificado, Claude Code não alterna automaticamente. A solicitação sinalizada termina com uma mensagem de recusa, e você pode alternar modelos com [`/model`](#setting-your-model) e tentar novamente. Definir `ANTHROPIC_DEFAULT_FABLE_MODEL` para seu ID de modelo Fable ativa o reconhecimento de Fable. Definir `ANTHROPIC_DEFAULT_OPUS_MODEL` para um ID de modelo Opus fornece às categorias sinalizadas um destino de fallback, a menos que o pino nomeie um modelo fora da família Opus ou o modelo que recusou; então Claude Code não alterna e a recusa permanece.

<h4 id="security-research-and-biology-workloads">
  Pesquisa de segurança e cargas de trabalho de biologia
</h4>

Cargas de trabalho em segurança ofensiva ou biologia, incluindo testes de penetração, exercícios Capture the Flag (CTF) e bases de código adjacentes à biologia, acionam fallback frequentemente, geralmente na primeira solicitação. Para trabalho substantivo de biologia em Fable 5.1, Fable 5 ou Opus 5.5, Claude Code move a sessão para Opus 5 na primeira solicitação sinalizada, e solicitações posteriores sinalizadas por biologia terminam em recusas lá, porque Opus 5 não tem fallback de biologia. Em Opus 5, você recebe essas recusas da primeira solicitação sinalizada.

Este é o roteamento esperado para esses domínios, não um sinalizador de conta. Se sua organização precisar de capacidade de classe Fable para este trabalho, peça ao seu time de contas da Anthropic sobre programas de acesso confiável.

<h3 id="adjust-effort-level">
  Ajustar nível de esforço
</h3>

[Níveis de esforço](https://platform.claude.com/docs/en/build-with-claude/effort) controlam raciocínio adaptativo, que permite ao modelo decidir se e quanto pensar em cada etapa com base na complexidade da tarefa. Esforço menor é mais rápido e mais barato para tarefas diretas, enquanto esforço maior fornece raciocínio mais profundo para problemas complexos.

Os níveis de esforço disponíveis dependem do modelo. Modelos não listados aqui não suportam esforço:

| Modelo                                          | Níveis                                  |
| :---------------------------------------------- | :-------------------------------------- |
| Fable 5.1 e Fable 5                             | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 5.5, Opus 5, Sonnet 5, Opus 4.8 e Opus 4.7 | `low`, `medium`, `high`, `xhigh`, `max` |
| Opus 4.6 e Sonnet 4.6                           | `low`, `medium`, `high`, `max`          |

Se você definir um nível que o modelo ativo não suporta, Claude Code volta para o nível mais alto suportado no ou abaixo do que você definiu. Por exemplo, `xhigh` é executado como `high` em Opus 4.6. Sua organização ou suas próprias configurações também podem limitar os níveis que um modelo oferece; consulte [Limites de esforço da organização](#organization-effort-limits).

Com a configuração [`ultracode`](/docs/pt/settings-reference#ultracode) desativada, Claude Code resolve o nível de esforço da sessão nesta ordem, tomando o primeiro que se aplica:

1. Uma escolha explícita: a variável de ambiente [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/pt/env-vars#variables), lançamento com `--effort`, ou `/effort` na sessão ([um `/effort` não interativo tem efeito mais estreito](#non-interactive-effort))
2. Suas configurações: o nível que você salvou para o modelo ou uma chave [`effortLevel`](/docs/pt/settings-reference#effortlevel), com a precedência entre eles e entre arquivos de configurações declarada em [`modelSettings`](/docs/pt/settings-reference#modelsettings)
3. O esforço padrão do modelo: `high` em cada modelo que suporta esforço, exceto que Opus 5.5 padrão para `medium`, Opus 4.7 padrão para `xhigh` e, quando sua organização define um nível de esforço padrão para seu [modelo padrão da organização](#organization-default-model), esse nível é o padrão quando você executa esse modelo

Opus 5.5 começa em `medium` a menos que uma das fontes acima defina um nível para ele, e um `effortLevel` de nível superior em seu arquivo de configurações de usuário não conta para Opus 5.5. Essa chave é a forma mais antiga que `/effort` escreveu antes de Claude Code salvar níveis por modelo: continua se aplicando onde se aplicava antes, em Opus 5, Fable 5.1 e modelos anteriores, enquanto Opus 5.5 e modelos lançados após ele começam em seu próprio padrão até você escolher um nível para eles com `/effort` ou o seletor `/model`. Um `effortLevel` de nível superior em configurações de projeto, local ou gerenciadas, ou um passado com `--settings`, se aplica a cada modelo.

Quando você define `low`, `medium`, `high` ou `xhigh` em uma sessão interativa em sua máquina, você escolhe quanto tempo dura confirmando-o:

* `Enter` no controle deslizante `/effort` ou no seletor `/model`, ou um nível digitado após `/effort`: salve o nível como seu padrão e aplique-o em sessões posteriores
* `s` no controle deslizante `/effort` ou no seletor `/model`: aplique o nível apenas a esta sessão. Requer Claude Code v2.1.257 ou posterior

Claude Code salva o nível por modelo, sob a chave [`modelSettings`](/docs/pt/settings-reference#modelsettings) em suas configurações de usuário, portanto cada modelo mantém seu próprio nível salvo.

`max` é o nível de raciocínio mais profundo. A menos que você o defina através da variável de ambiente `CLAUDE_CODE_EFFORT_LEVEL`, Claude Code aplica `max` apenas à sessão atual.

<Note>
  Um nível que você escolhe no controle de esforço em um telefone ou navegador conectado através de [Remote Control](/docs/pt/remote-control#what-connected-devices-see) se aplica apenas a essa sessão.
</Note>

<span id="non-interactive-effort" />

Quando você define um nível com `/effort` em uma execução [`-p`](/docs/pt/headless), Claude Code o aplica apenas a essa sessão e não o salva como seu padrão.

O menu `/effort` também oferece `ultracode`. Ultracode é uma configuração de Claude Code em vez de um nível de esforço do modelo: envia `xhigh` para o modelo e adicionalmente tem Claude orquestrar [fluxos de trabalho dinâmicos](/docs/pt/workflows) para tarefas substantivas. Para onde pode ser definido persistentemente, consulte a configuração [`ultracode`](/docs/pt/settings-reference#ultracode).

Você pode ativar ultracode através de qualquer um dos seguintes:

* **`/effort`**: execute `/effort ultracode`, ou selecione-o no menu
* **Sinalizador `--effort`**: lance com `claude --effort ultracode`, que inicia a sessão em esforço `xhigh` com ultracode ativado
* **Configuração `ultracode`**: defina [`"ultracode": true`](/docs/pt/settings-reference#ultracode) em um arquivo de configurações, com `--settings`, ou em uma solicitação de controle Agent SDK. Uma solicitação [`applyFlagSettings()`](/docs/pt/agent-sdk/typescript#applyflagsettings) também aceita `effortLevel: "ultracode"`
* **Seletor `/model`**: mova o controle deslizante de esforço para `ultracode` com as teclas de seta enquanto escolhe um modelo. Claude Code o ativa para a sessão atual, mesmo quando você salva esse modelo como seu padrão

Passar `ultracode` para o sinalizador `--effort` ou o valor Agent SDK `effortLevel` requer Claude Code v2.1.203 ou posterior. Antes da v2.1.203, `--effort ultracode` imprimia `Unknown --effort value 'ultracode'` e a sessão iniciava no esforço padrão.

A configuração `effortLevel` persistida e a variável de ambiente `CLAUDE_CODE_EFFORT_LEVEL` não aceitam `ultracode`. Quando `CLAUDE_CODE_EFFORT_LEVEL` é definido para um nível diferente de `xhigh`, solicitações são executadas nesse nível e a orquestração de fluxo de trabalho do ultracode permanece inativa. Selecionar ultracode então mostra um aviso de que a variável de ambiente substitui o esforço pela sessão.

<span id="when-ultracode-is-available" />

Ultracode não está disponível quando:

* [Fluxos de trabalho estão desativados](/docs/pt/workflows#turn-workflows-off)
* O modelo não suporta esforço `xhigh`
* Um [limite de esforço](#organization-effort-limits) abaixo de `xhigh` se aplica ao modelo

Nesses casos `--effort ultracode` inicia a sessão com ultracode desativado, no nível de esforço mais alto que o modelo e qualquer limite permitem, até `xhigh`.

<h4 id="choose-an-effort-level">
  Escolher um nível de esforço
</h4>

Cada nível negocia gasto de tokens contra capacidade. O padrão é adequado para a maioria das tarefas de codificação; ajuste quando quiser um equilíbrio diferente.

| Nível       | Quando usá-lo                                                                                                                                                  |
| :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `low`       | Reserve para tarefas curtas, escopo definido, sensíveis à latência que não são sensíveis à inteligência                                                        |
| `medium`    | Reduz o uso de tokens para trabalho sensível a custos que pode fazer concessões em inteligência. O padrão em Opus 5.5                                          |
| `high`      | Equilibra o uso de tokens e inteligência. O padrão em cada modelo exceto Opus 5.5 e Opus 4.7                                                                   |
| `xhigh`     | Raciocínio mais profundo com gasto de tokens mais alto. O padrão em Opus 4.7                                                                                   |
| `max`       | Pode melhorar o desempenho em tarefas exigentes, mas pode mostrar retornos decrescentes e é propenso a excesso de pensamento. Teste antes de adotar amplamente |
| `ultracode` | Uma configuração de Claude Code que planeja um [fluxo de trabalho dinâmico](/docs/pt/workflows) para cada tarefa substantiva com raciocínio `xhigh` por mensagem    |

A escala de esforço é calibrada por modelo, portanto o mesmo nome de nível não representa o mesmo valor subjacente entre modelos.

<h4 id="use-ultrathink-for-one-off-deep-reasoning">
  Usar ultrathink para raciocínio profundo único
</h4>

Inclua `ultrathink` em qualquer lugar em seu prompt para solicitar raciocínio mais profundo nesse turno sem alterar sua configuração de esforço de sessão. Claude Code reconhece a palavra-chave e adiciona uma instrução no contexto. O nível de esforço enviado para a API permanece inalterado. Claude Code passa outras frases como "think", "think hard" e "think more" como texto de prompt ordinário e não as reconhece como palavras-chave.

<h4 id="set-the-effort-level">
  Definir o nível de esforço
</h4>

Você pode alterar o esforço através de qualquer um dos seguintes:

* **`/effort`**: execute `/effort` sem argumentos para abrir um controle deslizante interativo, `/effort` seguido por um nome de nível para defini-lo diretamente, ou `/effort auto` para limpar seu nível salvo para o modelo ativo. Você pode executá-lo enquanto Claude está trabalhando, e uma vez que você confirme o [aviso de cache](/docs/pt/prompt-caching#changing-effort-level), se Claude Code mostrar um, Claude Code aplica o novo nível à próxima solicitação no turno
* **Em `/model`**: use as teclas de seta esquerda/direita para ajustar o controle deslizante de esforço ao selecionar um modelo
* **Sinalizador `--effort`**: passe um nome de nível para defini-lo para uma única sessão ao lançar Claude Code
* **Variável de ambiente**: defina `CLAUDE_CODE_EFFORT_LEVEL` para um nome de nível ou `auto`
* **Configurações**: defina um nível por modelo em [`modelSettings`](/docs/pt/settings-reference#modelsettings), ou defina [`effortLevel`](/docs/pt/settings-reference#effortlevel) para `low`, `medium`, `high` ou `xhigh` como o padrão para modelos sem um. `max` não é aceito como um nível em nenhuma chave, e `ultracode` tem sua própria chave [`ultracode`](/docs/pt/settings-reference#ultracode)
* **De um dispositivo conectado**: em uma sessão [Remote Control](/docs/pt/remote-control#what-connected-devices-see), escolha um nível no controle de esforço em seu telefone ou em seu navegador. O nível se aplica apenas à sessão atual. Requer Claude Code v2.1.234 ou posterior
* **Frontmatter de skill e subagente**: defina `effort` em um arquivo markdown [skill](/docs/pt/skills#frontmatter-reference) ou [subagente](/docs/pt/sub-agents#supported-frontmatter-fields) para substituir o nível de esforço quando esse skill ou subagente é executado

O esforço de frontmatter se aplica quando esse skill ou subagente está ativo, substituindo o nível de sessão, mas não a variável de ambiente. Um [`maxEffortLevel`](/docs/pt/settings-reference#maxeffortlevel) ou [limite de esforço da organização](#organization-effort-limits) ainda limita o nível em que o skill ou subagente é executado.

Se você definir `effortLevel` em [configurações gerenciadas](/docs/pt/managed-settings), Claude Code o aplica na etapa de configurações da [ordem de resolução de esforço](#adjust-effort-level), e os usuários ainda podem alterar o nível com `/effort` ou `--effort`. Para manter os usuários em ou abaixo de um nível, defina [`maxEffortLevel`](/docs/pt/settings-reference#maxeffortlevel).

O controle deslizante de esforço aparece em `/model` quando um modelo suportado é selecionado. O nível de esforço atual também é mostrado no cabeçalho da sessão ao lado do nome do modelo, por exemplo "with low effort", para que você possa confirmar qual configuração está ativa sem abrir `/model`. O rodapé também mostra brevemente o nível de esforço na inicialização e quando muda.

<h4 id="adaptive-reasoning-and-fixed-thinking-budgets">
  Raciocínio adaptativo e orçamentos de pensamento fixos
</h4>

O raciocínio adaptativo torna o pensamento opcional em cada etapa, portanto Claude pode responder mais rápido a prompts rotineiros e reservar pensamento mais profundo para etapas que se beneficiam dele. Se você quiser que Claude pense mais ou menos frequentemente do que o nível atual produz, você pode dizer isso diretamente em seu prompt ou em `CLAUDE.md`; o modelo responde a essa orientação dentro de sua configuração de esforço.

Os modelos Fable, Sonnet 5 e Opus 4.7 e posterior sempre usam raciocínio adaptativo. O modo de orçamento de pensamento fixo e `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` não se aplicam a eles.

Em Opus 4.6 e Sonnet 4.6, você pode definir `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` para reverter para o orçamento de pensamento fixo anterior controlado por `MAX_THINKING_TOKENS`. Consulte [variáveis de ambiente](/docs/pt/env-vars).

<h3 id="extended-thinking">
  Pensamento estendido
</h3>

Pensamento estendido é o raciocínio que Claude emite antes de responder. Em modelos que suportam [raciocínio adaptativo](#adjust-effort-level), o nível de esforço é o controle primário para quanto pensamento acontece; as configurações abaixo ativam ou desativam o pensamento e controlam como ele é exibido. Com o pensamento desativado na Anthropic API, Claude Code envia esforço `high` em vez de um nível mais alto para modelos que sabe [não aceitam essa combinação](/docs/pt/errors#effort-isnt-available-with-thinking-turned-off), como Opus 5.

| Controle                                      | Como defini-lo                                                                                                                                                                                                                                                                                                                                                                                                     |
| :-------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Alternar para a sessão atual                  | Pressione `Option+T` em macOS ou `Alt+T` em Windows e Linux                                                                                                                                                                                                                                                                                                                                                        |
| Definir o padrão global                       | Execute `/config` e alterne o modo de pensamento. Salvo como `alwaysThinkingEnabled` em `~/.claude/settings.json`                                                                                                                                                                                                                                                                                                  |
| Desativar através de uma variável de ambiente | Defina [`MAX_THINKING_TOKENS=0`](/docs/pt/env-vars), que desativa o pensamento na Anthropic API exceto em Opus 5.5 e modelos Fable. Em [provedores de terceiros](/docs/pt/third-party-integrations), Claude Code omite o parâmetro `thinking`, e modelos de raciocínio adaptativo ainda podem pensar. Outros valores se aplicam apenas com um [orçamento de pensamento fixo](#adaptive-reasoning-and-fixed-thinking-budgets) |

Você não pode desativar o pensamento em Opus 5.5 ou nos modelos Fable. O alternador de sessão, `alwaysThinkingEnabled` e `MAX_THINKING_TOKENS=0` não têm efeito lá, e o modelo decide por etapa quanto pensar com base no nível de esforço.

Claude Code recolhe a saída de pensamento por padrão. Pressione `Ctrl+O` para alternar o modo detalhado e ver o raciocínio como texto itálico cinzento. Sessões interativas na Anthropic API recebem blocos de pensamento redigidos por padrão, portanto defina `showThinkingSummaries: true` em [configurações](/docs/pt/settings) se quiser os resumos completos disponíveis quando expandir. Você é cobrado por todos os tokens de pensamento gerados, mesmo quando recolhidos ou redigidos.

<h3 id="extended-context">
  Contexto estendido
</h3>

Fable 5.1, Fable 5, Sonnet 5, Opus 4.6 e posterior, e Sonnet 4.6 suportam uma [janela de contexto de 1 milhão de tokens](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) para sessões longas com bases de código grandes.

Na Anthropic API, Fable 5.1, Fable 5, Sonnet 5 e Opus 4.7 e posterior são executados com a janela de 1M em cada plano, incluindo Pro. Você não seleciona uma variante `[1m]` ou ativa créditos de uso para a janela de 1M nesses modelos. O uso de Fable em si pode ser faturado para créditos de uso em alguns planos; consulte [Fable e créditos de uso](#fable-and-usage-credits).

Opus 4.6 e Sonnet 4.6 alcançam 1M apenas através de sua variante `[1m]`, e o acesso a essa variante depende do seu plano. Nos planos Max, Team e Enterprise, incluindo assentos Team Standard e Team Premium, Opus 4.6 com contexto de 1M está incluído em sua assinatura. Sonnet 4.6 com contexto de 1M requer [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) em cada plano de assinatura, incluindo Max.

| Plano                          | Opus 4.6 com contexto de 1M                                                                                 | Sonnet 4.6 com contexto de 1M                                                                               |
| ------------------------------ | ----------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| Max, Team e Enterprise         | Incluído na assinatura                                                                                      | Requer [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| Pro                            | Requer [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) | Requer [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) |
| API e pagamento conforme o uso | Acesso completo                                                                                             | Acesso completo                                                                                             |

Claude Code verifica esses requisitos de plano apenas quando se conecta diretamente à Anthropic API. Se você apontar `ANTHROPIC_BASE_URL` para um [gateway LLM](/docs/pt/llm-gateway#subscriptions-and-gateways) e seu login claude.ai salvo permanecer a credencial ativa, Claude Code não verifica seus créditos de uso do plano. As opções `[1m]` permanecem disponíveis em `/model`, e o gateway decide se a solicitação é bem-sucedida. Antes da v2.1.229, Claude Code rejeitava `/model sonnet[1m]` nessa configuração quando não conseguia confirmar créditos de uso na conta.

Para desativar contexto de 1M, defina `CLAUDE_CODE_DISABLE_1M_CONTEXT=1`. Claude Code remove variantes de modelo de 1M do seletor de modelo. Em modelos com uma janela nativa de 1M, como Sonnet 5 e os modelos Fable, também trata o modelo como tendo uma janela de contexto de 200K:

* Com compactação automática ativada, sessões compactam no limite de 200K através de [compactação automática](#set-the-auto-compact-window). Definir a janela de compactação automática acima de 200K não levanta a retenção, porque Claude Code limita essa janela à janela de contexto do modelo.
* Com compactação automática desativada, sessões param no limite de 200K com o [erro de limite de contexto](/docs/pt/errors#prompt-is-too-long) em vez de compactar.

Antes da v2.1.223, Claude Code mantinha apenas sessões Sonnet 5, Opus 4.8 e Opus 5 em 200K. Consulte [variáveis de ambiente](/docs/pt/env-vars).

A janela de contexto de 1M usa preços de modelo padrão sem prêmio para tokens além de 200K. Para planos onde contexto estendido está incluído em sua assinatura, o uso permanece coberto por sua assinatura. Para planos que acessam contexto estendido através de créditos de uso, tokens são faturados para créditos de uso.

Se sua conta suporta contexto de 1M, a opção aparece no seletor `/model` nas versões mais recentes de Claude Code. Se você não a vê, tente reiniciar sua sessão.

Você também pode usar o sufixo `[1m]` com aliases de modelo ou nomes de modelo completos:

```text theme={null}
# Use o alias opus[1m] ou sonnet[1m]
/model opus[1m]
/model sonnet[1m]

# Ou anexe [1m] a um nome de modelo completo
/model claude-opus-4-8[1m]
```

<h4 id="sonnet-5-context-window">
  Janela de contexto Sonnet 5
</h4>

Na Anthropic API, Sonnet 5 sempre é executado com a janela de contexto de 1M. Não há variante de 200K, nenhum sufixo `[1m]` para selecionar e nenhum crédito de uso necessário em nenhum plano. Sessões compactam automaticamente antes da janela preencher, em cerca de 967K tokens por padrão; defina [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/pt/env-vars) para escolher um limite diferente.

Duas configurações orçam a janela em 200K:

* **Gateway LLM**: quando `ANTHROPIC_BASE_URL` aponta para um [gateway](/docs/pt/llm-gateway), Claude Code não pode verificar suporte a 1M. Para usar a janela completa, selecione Sonnet 5 (1M context) no seletor de modelo, que mapeia para `sonnet[1m]`.
* **`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`**: mantém sessões em cada modelo com uma janela nativa de 1M em uma janela de 200K; consulte [Contexto estendido](#extended-context) para como a retenção é aplicada. Útil para implantações que precisam limitar contexto.

<h2 id="context-window-and-auto-compaction">
  Janela de contexto e auto-compactação
</h2>

A janela de auto-compactação é o quão cheia a janela de contexto pode ficar antes de Claude Code compactar a conversa. Para saber o que a compactação mantém e descarta por mecanismo, consulte [O que sobrevive à compactação](/docs/pt/context-window#what-survives-compaction).

<h3 id="set-the-auto-compact-window">
  Definir a janela de auto-compactação
</h3>

Você pode definir a janela de auto-compactação em três lugares:

* **Para esta sessão e posteriores**: execute `/autocompact` com um valor, como `/autocompact 500k`. Claude Code o salva em suas configurações de usuário como [`autoCompactWindow`](/docs/pt/settings-reference#autocompactwindow) e o aplica à sessão atual; se um [escopo de configurações](/docs/pt/settings#settings-precedence) de prioridade mais alta, como configurações gerenciadas, definir a chave, o comando salva seu valor, mas a sessão mantém a janela desse escopo, e o comando informa isso. Execute `/autocompact auto` para retornar à janela ajustada para seu modelo.
* **Para um lançamento**: passe [`--autocompact`](/docs/pt/cli-reference#cli-flags) ao iniciar Claude Code. O sinalizador substitui sua configuração salva para esse lançamento sem alterá-la, e `claude --autocompact auto` executa a sessão na janela ajustada mesmo se sua configuração salva tiver um valor. Diferentemente de `/autocompact`, o sinalizador não é preemptado por um escopo de configurações de prioridade mais alta, como configurações gerenciadas.
* **Em scripts e ambientes em nuvem**: defina [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/pt/env-vars). Enquanto estiver definido, ele tem precedência sobre o comando, o sinalizador e a configuração, e `/autocompact` relata a substituição em vez de alterar a janela.

O comando e o sinalizador aceitam um tamanho de janela de 100K a 1M de tokens, em qualquer uma destas formas:

* Uma contagem de token simples, como `200000`
* Um sufixo `k` ou `M`, como `500k` ou `1M`
* Um número simples de 100 a 1000, significando milhares, então `200` define 200.000

A variável de ambiente aceita apenas a contagem de token simples. Claude Code limita a janela à janela de contexto do modelo.

<h3 id="default-auto-compact-thresholds">
  Limites padrão de auto-compactação
</h3>

Se você não definir uma janela de auto-compactação, Claude Code compacta quando a conversa atinge o limite de contexto do modelo, exceto nestas sessões:

* [Sessões em nuvem](/docs/pt/claude-code-on-the-web) compactam conforme a conversa se aproxima do limite do modelo
* Sonnet 4.6 e Opus 4.6 sem [contexto estendido](#extended-context) compactam no limite de 200K, e assim fazem Opus 4.8 e posteriores quando executam com uma janela de contexto de 200K, como no Amazon Bedrock, na Plataforma de Agentes do Google Cloud e no Microsoft Foundry
* Quando você define [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/pt/env-vars), modelos com uma janela nativa de 1M, como Sonnet 5 e os modelos Fable, compactam no limite de 200K
* Modelos executando com uma janela nativa de 1M, como Sonnet 5, os modelos Fable e Opus 4.7 e posteriores na API Anthropic, compactam antes da janela se encher, em aproximadamente 967K tokens por padrão. No Amazon Bedrock, na Plataforma de Agentes do Google Cloud e no Microsoft Foundry, [Fixar modelos para implantações de terceiros](#pin-models-for-third-party-deployments) diz quais modelos executam com essa janela; para as configurações que orçam Sonnet 5 em 200K em vez disso, consulte [Janela de contexto Sonnet 5](#sonnet-5-context-window)
* Sessões em um ID de modelo que Claude Code não reconhece, como um alias de [gateway LLM](/docs/pt/llm-gateway), compactam na janela de contexto que Claude Code assume para o ID; consulte [Corrigir a janela para um gateway ou ID de modelo personalizado](#correct-the-window-for-a-gateway-or-custom-model-id)

<h3 id="correct-the-window-for-a-gateway-or-custom-model-id">
  Corrigir a janela para um gateway ou ID de modelo personalizado
</h3>

Em um [gateway LLM](/docs/pt/llm-gateway) ou outra implantação personalizada, Claude Code pode assumir uma janela de contexto para o ID do modelo que difere da janela real do modelo, independentemente de resolver ou não o ID para um modelo Claude. Defina [`CLAUDE_CODE_MAX_CONTEXT_TOKENS`](/docs/pt/env-vars) para a janela que Claude Code deve assumir em vez disso.

Como a variável se aplica depende do ID. Claude Code trata um ID como um provedor ou ortografia personalizada quando não começa com `claude-`, em qualquer capitalização, ou quando carrega um sufixo que Claude Code remove ao ler o ID, como a data `@YYYYMMDD` usada na Plataforma de Agentes do Google Cloud. Antes da v2.1.259, Claude Code não contava um sufixo removido, então um ID `claude-` não reconhecido com um sufixo de data era tratado como um nome `claude-` simples.

Um provedor não reconhecido ou ortografia personalizada, a mesma ortografia com `[1m]` e todos os outros IDs são três casos separados:

* Se Claude Code não conseguir resolver um provedor ou ortografia personalizada para um modelo que reconhece e o ID não contiver `[1m]`, a variável se aplica diretamente e a compactação proativa continua na janela declarada.
* Se Claude Code não conseguir resolver um provedor ou ortografia personalizada para um modelo que reconhece e o ID contiver `[1m]`, em qualquer capitalização, Claude Code assume uma janela de 1M para ele e a variável não se aplica por conta própria. Para corrigir a janela mantendo a compactação proativa, também defina [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/pt/env-vars). Com essa variável definida, Claude Code dimensiona o ID como a mesma ortografia sem `[1m]`, então `CLAUDE_CODE_MAX_CONTEXT_TOKENS` se aplica quando se aplicaria a essa ortografia sem tag.

  Com uma janela declarada acima de 200K, Claude Code então mostra um [aviso de inicialização](/docs/pt/errors#the-200k-limit-isnt-enforced) que o limite de 200K não é aplicado. O aviso é esperado nesta configuração.
* Se o ID resolver para um modelo que Claude Code reconhece, ou o ID for um nome `claude-` simples sem sufixo para Claude Code remover, em qualquer capitalização, a variável entra em vigor apenas quando você também define [`DISABLE_COMPACT`](/docs/pt/env-vars), que desabilita toda compactação.

  Por exemplo, um ID que contém um nome de modelo Claude que Claude Code conhece, como `anthropic/claude-opus-4-8`, `us.anthropic.claude-…-v1:0`, ou o datado `claude-sonnet-4-5@20250929`, resolve para esse modelo. Isso inclui IDs que também contêm `[1m]`: Claude Code resolve `claude-opus-4-8[1m]` para Opus 4.8 mesmo com `CLAUDE_CODE_DISABLE_1M_CONTEXT` definido.

Para um ID de modelo que Claude Code não reconhece, defina [`CLAUDE_CODE_DISABLE_UNKNOWN_MODEL_WINDOW_ENFORCEMENT=1`](/docs/pt/env-vars) para que Claude Code compacte apenas depois que a API rejeitar a conversa com um [erro muito longo que Claude Code reconhece](/docs/pt/errors#prompt-is-too-long). Claude Code não executa essa recuperação quando um gateway [reescreve o erro](/docs/pt/llm-gateway-connect#troubleshoot-gateway-errors) para uma redação que Claude Code não reconhece.

<h2 id="checking-your-current-model">
  Verificando seu modelo atual
</h2>

Você pode ver qual modelo está usando atualmente em dois lugares:

* Na [linha de status](/docs/pt/statusline), se você tiver uma configurada
* Em `/status`, que também exibe as informações de sua conta

<h2 id="add-a-custom-model-option">
  Adicionar uma opção de modelo personalizado
</h2>

Use `ANTHROPIC_CUSTOM_MODEL_OPTION` para adicionar uma única entrada personalizada ao seletor `/model` sem substituir os aliases integrados. Isso é útil para testar IDs de modelo que Claude Code não lista por padrão. Para implantações de gateway LLM, Claude Code pode preencher o seletor a partir do endpoint `/v1/models` do gateway quando `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1` está definido, portanto essa variável é necessária apenas quando a descoberta está desabilitada ou não retorna o modelo que você deseja. Consulte [descoberta de modelo de gateway](/docs/pt/llm-gateway-protocol#model-discovery).

Para listar vários modelos em vez disso, em sua própria ordem e sob rótulos que você escolher, defina [`modelPicker`](/docs/pt/settings-reference#modelpicker). Sua entrada diz quais linhas o seletor mantém quando esse alinhamento substitui o integrado.

Este exemplo define todas as três variáveis para tornar uma implantação Opus roteada por gateway selecionável. Claude Code lê variáveis de ambiente na inicialização, portanto execute as exportações antes de iniciar `claude`, ou reinicie uma sessão existente para aplicá-las:

```bash theme={null}
export ANTHROPIC_CUSTOM_MODEL_OPTION="my-gateway/claude-opus-5-5"
export ANTHROPIC_CUSTOM_MODEL_OPTION_NAME="Opus via Gateway"
export ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION="Custom deployment routed through the internal LLM gateway"
```

`ANTHROPIC_CUSTOM_MODEL_OPTION_NAME` e `ANTHROPIC_CUSTOM_MODEL_OPTION_DESCRIPTION` são opcionais:

* Se você omitir o nome, a entrada mostra o nome do modelo quando Claude Code [reconhece o ID](#customize-pinned-model-display-and-capabilities), e o ID do modelo caso contrário.
* Se você omitir a descrição, Claude Code usa `Custom model (<model-id>)`.

Claude Code lista a entrada personalizada após as entradas integradas, e qualquer linha [`modelPicker`](/docs/pt/settings-reference#modelpicker) que você acrescente vem depois dela.

Claude Code ignora a validação para o ID do modelo definido em `ANTHROPIC_CUSTOM_MODEL_OPTION`, portanto você pode usar qualquer string que seu endpoint de API aceite.

Quando [`availableModels`](#restrict-model-selection) está definido, inclua o ID do modelo personalizado na lista de permissões também. Caso contrário, Claude Code filtra a entrada personalizada do seletor e rejeita uma seleção `--model` dela como qualquer outro modelo excluído.

Um ID personalizado que incorpora um nome de família, como `my-gateway/claude-opus-5-5`, conta como uma entrada específica para essa família e desabilita seu curinga, portanto também liste as versões que você pretende manter selecionáveis. Consulte [Comportamento de mesclagem](#merge-behavior).

<h2 id="environment-variables">
  Variáveis de ambiente
</h2>

Use as seguintes variáveis de ambiente para controlar os nomes de modelo para os quais os aliases mapeiam. Cada valor deve ser um nome de modelo completo, ou o identificador equivalente para seu provedor de API. Para escolher o modelo em que suas sessões começam, defina [`ANTHROPIC_DEFAULT_MODEL`](#set-a-default-model-for-new-sessions), que esta tabela omite.

| Variável de ambiente             | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| -------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_FABLE_MODEL`  | O modelo a usar para `fable`, e o ID de modelo que Claude Code reconhece como modelo Fable para [fallback automático de modelo](#automatic-model-fallback) em provedores de terceiros                                                                                                                                                                                                                                                                                                                             |
| `ANTHROPIC_DEFAULT_OPUS_MODEL`   | O modelo a usar para `opus`, ou para `opusplan` quando Plan Mode está ativo.                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `ANTHROPIC_DEFAULT_SONNET_MODEL` | O modelo a usar para `sonnet`, ou para `opusplan` quando Plan Mode não está ativo.                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `ANTHROPIC_DEFAULT_HAIKU_MODEL`  | O modelo a usar para `haiku`, ou [funcionalidade de fundo](/docs/pt/costs#background-token-usage)                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `CLAUDE_CODE_SUBAGENT_MODEL`     | O modelo padrão para [subagents](/docs/pt/sub-agents#choose-a-model), [agent team](/docs/pt/agent-teams#specify-teammates-and-models) companheiros, e agentes de [workflow](/docs/pt/workflows) que não são atribuídos a um modelo de outra forma. Aceita um alias como `haiku` ou um nome de modelo completo. Um modelo por invocação ou o campo `model` de uma definição, incluindo `inherit`, tem precedência. Para alterar isso, defina [`CLAUDE_CODE_SUBAGENT_MODEL_FORCE`](/docs/pt/sub-agents#run-every-subagent-on-one-model) |

Nota: `ANTHROPIC_SMALL_FAST_MODEL` está descontinuado em favor de
`ANTHROPIC_DEFAULT_HAIKU_MODEL`.

<h3 id="pin-models-for-third-party-deployments">
  Fixar modelos para implantações de terceiros
</h3>

Ao implantar Claude Code através de [Amazon Bedrock](/docs/pt/amazon-bedrock), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai), [Microsoft Foundry](/docs/pt/microsoft-foundry), ou [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), fixe versões de modelo antes de lançar para usuários.

Sem fixação, Claude Code usa aliases de modelo como `fable`, `opus`, `sonnet` e `haiku` que resolvem para um ID de modelo padrão integrado para cada provedor. Esse padrão pode ficar atrás da versão mais recente do Anthropic, e o modelo para o qual aponta pode ainda não estar habilitado na conta de um usuário. Quando o padrão não está disponível, os usuários de Amazon Bedrock e Google Cloud's Agent Platform veem um aviso e a sessão volta para uma versão anterior do modelo padrão, ou para o modelo Sonnet padrão quando o padrão é um modelo Opus e nenhuma versão Opus está disponível. Os usuários de Microsoft Foundry veem erros em vez disso, porque Microsoft Foundry não tem verificação de inicialização equivalente.

No Amazon Bedrock e Google Cloud's Agent Platform, um usuário que inicia a sessão em uma versão específica de Sonnet ou Opus, por exemplo com `--model`, `ANTHROPIC_MODEL`, ou a configuração `model`, fixa essa versão como o padrão da sessão para o alias correspondente: a verificação de inicialização pula o padrão integrado que substitui e não mostra aviso de fallback. Antes da v2.1.211, a verificação era executada e poderia mostrar um aviso mesmo quando um modelo de sessão era explicitamente configurado.

<Warning>
  Defina as variáveis de ambiente de modelo para IDs de versão específicos como parte de sua configuração inicial. Fixar permite que você controle quando seus usuários se movem para um novo modelo.
</Warning>

Use as seguintes variáveis de ambiente com IDs de modelo específicos de versão para seu provedor:

| Provedor                      | Exemplo                                                              |
| :---------------------------- | :------------------------------------------------------------------- |
| Amazon Bedrock                | `export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'` |
| Google Cloud's Agent Platform | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'`              |
| Microsoft Foundry             | `export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'`              |

Aplique o mesmo padrão para `ANTHROPIC_DEFAULT_FABLE_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL` e `ANTHROPIC_DEFAULT_HAIKU_MODEL`. Para IDs de modelo atuais e legados em todos os provedores, veja [Visão geral de modelos](https://platform.claude.com/docs/en/about-claude/models/overview). Para atualizar usuários para uma nova versão de modelo, atualize essas variáveis de ambiente e reimplante.

Para habilitar [contexto estendido](#extended-context) para um modelo fixado, anexe `[1m]` ao ID do modelo em `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, ou `ANTHROPIC_DEFAULT_FABLE_MODEL`:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8[1m]'
```

Com o sufixo `[1m]`, a janela de contexto 1M se aplica a todo o uso do alias fixado, incluindo a fase Opus do modo de plano de [`opusplan`](#opusplan-model-setting) e [subagents](/docs/pt/sub-agents#choose-a-model) cujo frontmatter `model` nomeia o alias.

* Claude Code remove o sufixo antes de enviar o ID do modelo para seu provedor.
* Apenas anexe `[1m]` quando o modelo subjacente [suportar contexto 1M](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model).
* O sufixo é lido por variável, não por modelo. No Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, um ID de modelo sem `[1m]` em uma variável usa contexto 200K mesmo se outra variável define o mesmo modelo com o sufixo. Sonnet 5 sempre é executado com a janela 1M nesses provedores e nunca precisa do sufixo.

<Note>
  Uma lista de permissões `availableModels` entregue através de [MDM ou um arquivo de configurações gerenciado](/docs/pt/managed-settings#delivery-mechanisms) ainda se aplica ao usar provedores de terceiros; [configurações gerenciadas pelo servidor não são entregues lá](/docs/pt/server-managed-settings#platform-availability).

  A filtragem corresponde a um alias de modelo como `opus`, um prefixo de versão como `claude-opus-4-8`, ou o ID de modelo completo em forma de provedor. Prefixos específicos do provedor como `us.anthropic.` não são removidos, então para permitir um modelo específico, liste seu ID completo em forma de provedor, ou mapeie através de [`modelOverrides`](#override-model-ids-per-version). Qualquer sufixo `[1m]` é removido tanto da entrada da lista de permissões quanto do modelo solicitado antes da correspondência.
</Note>

<h3 id="customize-pinned-model-display-and-capabilities">
  Personalizar exibição e capacidades do modelo fixado
</h3>

Quando você fixa um modelo em um provedor de terceiros, sua linha no seletor `/model` mostra o nome do modelo por padrão se Claude Code reconhecer o ID fixado, e o ID bruto caso contrário:

* **Reconhecido**: o ID exato de um modelo que Claude Code conhece, como seu ID da API Anthropic ou a forma do seu provedor ou gateway, com ou sem o sufixo `[1m]`. Fixe `us.anthropic.claude-sonnet-4-5-20250929-v1:0` e a linha lê `Sonnet 4.5`.
* **Não reconhecido**: qualquer outro ID, como um ARN de perfil de inferência de aplicação ou uma versão de modelo que Claude Code não conhece, a menos que uma entrada [`modelOverrides`](#override-model-ids-per-version) mapeie um modelo para essa string exata. No Microsoft Foundry, nomes de implantação são definidos pelo usuário, então Claude Code nunca reconhece um ID fixado lá, mapeado ou não, e a linha mostra o nome da implantação por padrão.

Quando uma linha mostra o nome do modelo, sua descrição padrão inclui o ID fixado para que você ainda possa ver qual ID está fixado.

Claude Code também pode não reconhecer quais recursos um modelo fixado suporta. Você pode definir o nome de exibição e a descrição você mesmo e declarar capacidades com variáveis de ambiente complementares para cada modelo fixado.

Essas variáveis têm efeito em provedores de terceiros como Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry. As variáveis `_NAME` e `_DESCRIPTION` também têm efeito quando `ANTHROPIC_BASE_URL` aponta para um [gateway LLM](/docs/pt/llm-gateway). Elas não têm efeito ao conectar diretamente a `api.anthropic.com`.

| Variável de ambiente                                  | Descrição                                                                                                                                                                                |
| ----------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_NAME`                   | Nome de exibição para o modelo Opus fixado no seletor `/model`. Quando não definido, a linha mostra o nome do modelo se Claude Code reconhecer o ID fixado, e o ID fixado caso contrário |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION`            | Descrição de exibição para o modelo Opus fixado no seletor `/model`. Quando não definido, a linha mostra uma descrição padrão que começa com `Custom Opus model`                         |
| `ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES` | Lista separada por vírgulas de capacidades que o modelo Opus fixado suporta                                                                                                              |

Os mesmos sufixos `_NAME`, `_DESCRIPTION` e `_SUPPORTED_CAPABILITIES` estão disponíveis para `ANTHROPIC_DEFAULT_SONNET_MODEL`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`, `ANTHROPIC_DEFAULT_FABLE_MODEL` e `ANTHROPIC_CUSTOM_MODEL_OPTION`.

Claude Code habilita recursos como [níveis de esforço](#adjust-effort-level) e [pensamento estendido](#extended-thinking) correspondendo o ID do modelo contra padrões conhecidos. IDs específicos do provedor, como ARNs Amazon Bedrock ou nomes de implantação personalizados, geralmente não correspondem a esses padrões, deixando recursos suportados desabilitados. Defina `_SUPPORTED_CAPABILITIES` para informar ao Claude Code quais recursos o modelo realmente suporta:

| Valor de capacidade    | Habilita                                                                                      |
| ---------------------- | --------------------------------------------------------------------------------------------- |
| `effort`               | [Níveis de esforço](#adjust-effort-level) e o comando `/effort`                               |
| `xhigh_effort`         | O nível de esforço `xhigh`                                                                    |
| `max_effort`           | O nível de esforço `max`                                                                      |
| `thinking`             | [Pensamento estendido](#extended-thinking)                                                    |
| `adaptive_thinking`    | Raciocínio adaptativo que aloca dinamicamente o pensamento com base na complexidade da tarefa |
| `interleaved_thinking` | Pensamento entre chamadas de ferramenta                                                       |

Quando `_SUPPORTED_CAPABILITIES` é definido, Claude Code habilita as capacidades listadas e desabilita as não listadas para o modelo fixado correspondente. Quando a variável não está definida, Claude Code volta para detecção integrada baseada no ID do modelo.

Este exemplo fixa Opus para um ARN de modelo personalizado Amazon Bedrock, define um nome amigável e declara suas capacidades:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='arn:aws:bedrock:us-east-1:123456789012:custom-model/abc'
export ANTHROPIC_DEFAULT_OPUS_MODEL_NAME='Opus via Bedrock'
export ANTHROPIC_DEFAULT_OPUS_MODEL_DESCRIPTION='Opus 4.7 routed through a Bedrock custom endpoint'
export ANTHROPIC_DEFAULT_OPUS_MODEL_SUPPORTED_CAPABILITIES='effort,xhigh_effort,max_effort,thinking,adaptive_thinking,interleaved_thinking'
```

<h3 id="override-model-ids-per-version">
  Substituir IDs de modelo por versão
</h3>

Em plataformas que incorporam Claude Code e definem [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/pt/env-vars), a configuração de modelo do host tem precedência sobre configurações de modelo gerenciadas, enquanto uma lista de permissões `availableModels` gerenciada permanece em vigor a menos que o host forneça a sua própria; [Exceções à precedência de configurações gerenciadas](/docs/pt/settings#exceptions-to-managed-settings-precedence) diz quais chaves e variáveis o host substitui.

As variáveis de ambiente no nível de família acima configuram um ID de modelo por alias de família. Se você precisar mapear várias versões dentro da mesma família para IDs de provedor distintos, use a configuração `modelOverrides` em vez disso.

`modelOverrides` mapeia IDs de modelo Anthropic individuais para as strings específicas do provedor que Claude Code envia para a API do seu provedor. Quando um usuário seleciona um modelo mapeado no seletor `/model`, Claude Code usa seu valor configurado em vez do padrão integrado.

Isso permite que administradores corporativos roteiem cada versão de modelo para um ARN de perfil de inferência Amazon Bedrock específico, nome de versão Google Cloud's Agent Platform ou nome de implantação Microsoft Foundry para governança, alocação de custos ou roteamento regional.

Defina `modelOverrides` em seu [arquivo de configurações](/docs/pt/settings#where-settings-live):

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-sonnet-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/sonnet-prod"
  }
}
```

As chaves devem ser IDs de modelo Anthropic conforme listado na [Visão geral de modelos](https://platform.claude.com/docs/en/about-claude/models/overview). Para IDs de modelo datados, inclua o sufixo de data exatamente como aparece lá. Chaves desconhecidas são ignoradas.

Para parar a linha de [diagnóstico](/docs/pt/errors#unrecognized-model-id-on-a-request) `[claude-code:unrecognized_model]` para um ID como um alias de gateway, adicione uma entrada com esse ID como seu valor.

As substituições substituem os IDs de modelo integrados que suportam cada entrada no seletor `/model`. No Amazon Bedrock, as entradas `modelOverrides` têm precedência sobre qualquer perfil de inferência que Claude Code descobre automaticamente na inicialização. Claude Code passa valores que já são específicos do provedor, como ARNs de perfil de inferência Amazon Bedrock ou nomes de implantação Microsoft Foundry, para o provedor como estão.

As substituições também se aplicam quando você passa um ID de modelo Anthropic diretamente através de `--model`, a variável de ambiente `ANTHROPIC_MODEL`, ou uma variável de ambiente `ANTHROPIC_DEFAULT_*_MODEL`. No Amazon Bedrock, Google Cloud's Agent Platform e [Mantle](/docs/pt/amazon-bedrock#use-the-mantle-endpoint), um ID de modelo Anthropic sem entrada `modelOverrides` resolve para o mesmo ID específico do provedor que a linha do seletor `/model` para essa versão, quando o provedor suporta essa versão. Mantle suporta um subconjunto de versões. Para um ID de modelo Anthropic fora desse subconjunto, Claude Code envia o ID bruto para Mantle sem mapeá-lo, a menos que uma entrada `modelOverrides` o cubra. Antes da v2.1.200, `--model` e os valores de variável de ambiente chegavam ao provedor como estavam sem passar pelo mapa de substituição.

`modelOverrides` funciona junto com `availableModels`. A lista de permissões é avaliada contra o ID de modelo Anthropic, não o valor de substituição, então uma entrada como `"opus"` em `availableModels` continua a corresponder mesmo quando versões do Opus são mapeadas para ARNs. Quando `enforceAvailableModels` é definido em configurações gerenciadas, o Padrão imposto é resolvido através de `modelOverrides` de [configurações gerenciadas](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) apenas. O mapeamento de um administrador, como uma versão fixada para um ARN de perfil de inferência, é honrado no Padrão imposto. Substituições de configurações de usuário ou projeto não o afetam.

Quando `availableModels` é definido em [configurações gerenciadas](/docs/pt/managed-settings), apenas `modelOverrides` de configurações gerenciadas se aplicam a um ID de modelo Anthropic passado diretamente através de `--model` ou das variáveis de ambiente acima. Claude Code ignora substituições em configurações de usuário ou projeto para esses IDs, e nunca resolve um ID que a lista gerenciada exclui através de `modelOverrides` de qualquer fonte de configurações. Essa restrição de fonte gerenciada requer Claude Code v2.1.200 ou posterior. Veja [Restringir seleção de modelo](#restrict-model-selection) para como IDs bloqueados são tratados.

<h3 id="prompt-caching-configuration">
  Configuração de prompt caching
</h3>

Claude Code usa automaticamente [prompt caching](/docs/pt/prompt-caching) para otimizar o desempenho e reduzir custos. Você pode desabilitar prompt caching globalmente ou para níveis de modelo específicos:

| Variável de ambiente            | Descrição                                                                                                                |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `DISABLE_PROMPT_CACHING`        | Defina como `1` para desabilitar prompt caching para todos os modelos. Tem precedência sobre as configurações por modelo |
| `DISABLE_PROMPT_CACHING_HAIKU`  | Defina como `1` para desabilitar prompt caching apenas para modelos Haiku                                                |
| `DISABLE_PROMPT_CACHING_SONNET` | Defina como `1` para desabilitar prompt caching apenas para modelos Sonnet                                               |
| `DISABLE_PROMPT_CACHING_OPUS`   | Defina como `1` para desabilitar prompt caching apenas para modelos Opus                                                 |
| `DISABLE_PROMPT_CACHING_FABLE`  | Defina como `1` para desabilitar prompt caching apenas para modelos Fable                                                |

Para escolher o TTL do cache para a conversa principal e para subagents separadamente, veja [escolha o TTL você mesmo](/docs/pt/prompt-caching#choose-the-ttl-yourself). Para o que dispara uma falha de cache, veja [Como Claude Code usa prompt caching](/docs/pt/prompt-caching).
