> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Como Claude Code usa prompt caching

> Claude Code gerencia prompt caching automaticamente. Veja por que uma mudança de modelo dispara um turno lento sem cache, o que `/compact` custa, por que edições de CLAUDE.md não se aplicam no meio da sessão e como verificar sua taxa de acerto de cache.

Prompt caching torna Claude Code mais rápido e eficiente em termos de custo. Sem caching, a API reprocessaria seu histórico completo a cada turno. Com caching, ela reutiliza o que já processou, cobra a releitura na [taxa de token em cache](https://platform.claude.com/docs/en/about-claude/pricing) e processa completamente apenas o que mudou.

Claude Code gerencia prompt caching para você, a menos que você [desative-o](#disable-prompt-caching). Ainda é útil saber como o prompt caching funciona, porque algumas ações invalidam o cache e tornam a próxima resposta mais lenta e cara enquanto ele se reconstrói. Esta página cobre quais ações são essas, por que algumas configurações aguardam uma reinicialização para serem aplicadas e como verificar o desempenho do cache quando o uso parece alto.

<h2 id="how-the-cache-is-organized">
  Como o cache é organizado
</h2>

Cada vez que você envia uma mensagem no Claude Code, ele faz uma nova solicitação de API. O modelo não se lembra de nada entre solicitações, então Claude Code reenvia o contexto completo: o prompt do sistema, o contexto do seu projeto, todas as mensagens anteriores e resultados de ferramentas, e sua nova mensagem. O novo conteúdo é anexado no final, o que significa que a maior parte de cada solicitação é idêntica à anterior. O prompt caching é como a API evita reprocessar a parte que não mudou.

A API faz cache correspondendo o início de cada solicitação, chamado de prefixo, contra o conteúdo que processou recentemente. Em um turno normal, o prefixo é toda a solicitação anterior e apenas a troca mais recente é nova. A correspondência é exata, então uma mudança em qualquer lugar no prefixo recomputa tudo depois dela. Não há cache por arquivo ou por segmento. Veja [como o prompt caching funciona](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#how-prompt-caching-works) na referência da API para o mecanismo subjacente.

<img src="https://mintcdn.com/claude-code/VbDJw--l6T9a9Wvm/images/prompt-caching-prefix.svg?fit=max&auto=format&n=VbDJw--l6T9a9Wvm&q=85&s=f2e8f0b8298a50305fe428ca3f1d1594" className="dark:hidden" alt="Quatro turnos mostrados como barras horizontais crescentes. A solicitação de cada turno contém tudo do turno anterior mais a troca mais recente anexada no final. Nos turnos dois e três, o prefixo inalterado é lido do cache e apenas a nova troca é processada. No turno quatro, o prompt do sistema mudou, então o prefixo não corresponde mais e toda a solicitação é reprocessada e escrita." width="720" height="454" data-path="images/prompt-caching-prefix.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/prompt-caching-prefix-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=297dc1c639f0915cae858d0c4b6f3be5" className="hidden dark:block" alt="Quatro turnos mostrados como barras horizontais crescentes. A solicitação de cada turno contém tudo do turno anterior mais a troca mais recente anexada no final. Nos turnos dois e três, o prefixo inalterado é lido do cache e apenas a nova troca é processada. No turno quatro, o prompt do sistema mudou, então o prefixo não corresponde mais e toda a solicitação é reprocessada e escrita." width="720" height="454" data-path="images/prompt-caching-prefix-dark.svg" />

Para aproveitar ao máximo a correspondência de prefixo, Claude Code ordena cada solicitação para que o conteúdo que raramente muda entre turnos venha primeiro:

| Camada              | Conteúdo                                                       | Muda quando                                             |
| ------------------- | -------------------------------------------------------------- | ------------------------------------------------------- |
| Prompt do sistema   | Instruções principais, definições de ferramentas               | O conjunto de definições de ferramentas carregadas muda |
| Contexto do projeto | CLAUDE.md, memória automática, regras sem escopo               | A sessão começa, ou após `/clear` ou `/compact`         |
| Conversa            | Suas mensagens, respostas do Claude, resultados de ferramentas | A cada turno                                            |

Uma mudança na camada de conversa deixa o prompt do sistema e o contexto do projeto em cache. Uma mudança no prompt do sistema invalida tudo, porque todo o conteúdo posterior agora fica atrás de um prefixo diferente. A terceira coluna fornece gatilhos comuns em vez de uma lista exaustiva, e as seções abaixo cobrem o conjunto completo.

A regra de correspondência de prefixo explica a maioria dos comportamentos nesta página. [Plan mode](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode) e [skill loading](/docs/pt/skills), por exemplo, anexam suas instruções como mensagens de conversa, então o prefixo em cache permanece intacto.

Duas configurações não aparecem na tabela de camadas, mas ainda afetam o que permanece em cache:

* **Model**: cada modelo tem seu próprio cache. Trocar modelos recomputa toda a solicitação mesmo quando o conteúdo é idêntico. Veja [Switching models](#switching-models) abaixo.
* **Effort level**: na maioria dos modelos, cada nível de esforço tem seu próprio cache, então mudar o esforço no meio da sessão recomputa toda a solicitação. No Opus 5.5 e Fable 5.1 com uma chave de API ou uma assinatura Claude, o cache permanece intacto por padrão. Veja [Changing effort level](#changing-effort-level) abaixo.

<Tip>
  Escolha seu modelo e nível de esforço no início de uma sessão, depois salve `/compact` para pausas naturais entre tarefas. Quanto menos mudanças você fizer no meio da tarefa, maior será sua taxa de acerto de cache.
</Tip>

<h3 id="where-the-cache-lives">
  Onde o cache reside
</h3>

O caching acontece no lado do servidor, na infraestrutura que serve seu modelo. Onde isso fica depende de como você se autentica:

* **Chave de API, assinatura Claude, ou [Claude Platform on AWS](/docs/pt/claude-platform-on-aws)**: o cache reside na infraestrutura da Anthropic, acessado através da [Claude API](https://platform.claude.com/docs)
* **Amazon Bedrock ou Google Cloud's Agent Platform**: o cache reside na infraestrutura de serviço do seu provedor de nuvem
* **Microsoft Foundry**: depende da [hosting option](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) da implantação. Implantações hospedadas no Azure são servidas na infraestrutura do Azure; implantações hospedadas na Anthropic são servidas na infraestrutura da Anthropic
* **Custom `ANTHROPIC_BASE_URL` ou [LLM gateway](/docs/pt/llm-gateway)**: o cache reside onde suas solicitações são encaminhadas, e se o caching funciona depende do gateway

Claude Code também anexa contexto do sistema no meio da conversa, como notificações de mudança de arquivo, e marca esse bloco para cache em todos os provedores e conexões, a menos que você defina [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/pt/llm-gateway-protocol#disable-pre-release-capabilities), caso em que esse bloco é enviado sem cache.

No endpoint próprio do provedor, Amazon Bedrock e seu [Mantle endpoint](/docs/pt/amazon-bedrock#use-the-mantle-endpoint), Google Cloud's Agent Platform, e Microsoft Foundry fazem cache do bloco da mesma forma que a Claude API faz.

Quando suas solicitações passam por um [LLM gateway](/docs/pt/llm-gateway), um `ANTHROPIC_BASE_URL` customizado, ou uma substituição de URL base do provedor de nuvem como [`ANTHROPIC_BEDROCK_BASE_URL`](/docs/pt/env-vars), o que permanece em cache depende de como o gateway lida com os [marcadores `cache_control`](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#explicit-cache-breakpoints) que Claude Code envia:

* **Encaminha-os inalterados**: o bloco e sua conversa fazem cache da mesma forma que no endpoint próprio do provedor.
* **Rejeita a solicitação marcada com um erro `400` nomeando `cache_control`**: Claude Code reenvia a solicitação com o marcador movido do bloco para sua última mensagem de conversa, e o mantém lá pelo resto da conversa. O bloco é cobrado como entrada sem cache; sua conversa permanece em cache.
* **Remove os marcadores enquanto retorna sucesso**: todo o histórico de conversa é cobrado como entrada sem cache a cada turno. Um gateway que converte conteúdo de sistema em forma de bloco para uma string simples remove o marcador da mesma forma.

Para o que cada provedor armazena e processa, veja [data usage](/docs/pt/data-usage). Onde quer que o cache resida, as entradas expiram após um período de inatividade, e [Cache lifetime](#cache-lifetime) abaixo cobre o TTL e como estendê-lo.

<h2 id="actions-that-invalidate-the-cache">
  Ações que invalidam o cache
</h2>

Essas ações fazem com que a próxima solicitação perca parte ou todo o cache. Você vê um turno mais lento e mais caro uma única vez, após o qual o novo prefixo é armazenado em cache. A maioria delas é evitável durante a tarefa uma vez que você sabe que têm um custo. Uma mudança de modelo pode parecer gratuita até você notar o turno mais lento que se segue.

* [Switching models](#switching-models)
* [Changing effort level](#changing-effort-level)
* [Turning on fast mode](#turning-on-fast-mode)
* [Connecting or disconnecting an MCP server](#connecting-or-disconnecting-an-mcp-server)
* [Enabling or disabling a plugin](#enabling-or-disabling-a-plugin)
* [Denying an entire tool](#denying-an-entire-tool)
* [Compacting the conversation](#compacting-the-conversation)
* [Accumulating many images](#accumulating-many-images)
* [Upgrading Claude Code](#upgrading-claude-code)

<h3 id="switching-models">
  Switching models
</h3>

Cada modelo tem seu próprio cache. Alternar com [`/model`](/docs/pt/model-config#setting-your-model) significa que a próxima solicitação lê todo o histórico de conversa sem acertos de cache, mesmo que o conteúdo seja idêntico.

Quando você executa `/model` no terminal, Claude Code pede que você confirme a mudança apenas enquanto o cache ainda está quente e o novo modelo não é aquele que produziu a última resposta. O cache permanece quente por um [cache TTL](#cache-lifetime) após Claude Code ter enviado pela última vez uma solicitação nesta conversa ou Claude ter respondido. Depois que esse tempo passa, o cache expirou, então Claude Code muda sem perguntar.

Antes da v2.1.238, Claude Code não verificava o cache TTL e perguntava mesmo depois que o cache havia expirado.

Você também pode exigir essa confirmação ou ignorá-la com um [PreModelSwitch hook](/docs/pt/hooks#premodelswitch-decision-control).

A [`opusplan` model setting](/docs/pt/model-config#opusplan-model-setting) resolve para Opus durante o modo de plano e Sonnet durante a execução, então cada alternância de modo de plano é uma mudança de modelo e inicia um cache novo.

[Automatic model fallback](/docs/pt/model-config#automatic-model-fallback) em modelos Fable, Opus 5.5 e Opus 5 também é uma mudança de modelo. Quando um classificador de segurança sinaliza uma solicitação em uma categoria que tem um modelo de fallback, Claude Code executa novamente a solicitação nesse modelo e a sessão continua lá.

Quando a frontmatter de uma skill ou comando nomeia um [`model`](/docs/pt/skills#frontmatter-reference) diferente do modelo atual da sessão, esse turno também é uma mudança de modelo: a próxima solicitação lê todo o histórico de conversa sem acertos de cache. O modelo de sessão retoma no seu próximo prompt. Uma skill `context: fork` define o [modelo do subagente bifurcado](/docs/pt/skills#run-skills-in-a-subagent) em vez disso.

<h3 id="changing-effort-level">
  Changing effort level
</h3>

Na maioria dos modelos, alterar o [effort level](/docs/pt/model-config#adjust-effort-level) no meio da sessão significa que a próxima solicitação lê todo o histórico de conversa sem acertos de cache. Enquanto o cache ainda está quente, Claude Code pede que você confirme a mudança primeiro.

No Opus 5.5 e Fable 5.1 com uma chave de API ou uma assinatura Claude, alterar o esforço mantém o cache, e Claude Code aplica o novo nível sem perguntar. Isso não se aplica no Amazon Bedrock, na plataforma de agentes do Google Cloud, ou em um [Claude apps gateway](/docs/pt/claude-apps-gateway), ou quando você define [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/pt/llm-gateway-protocol#disable-pre-release-capabilities) ou sua organização tem uma configuração HIPAA.

Antes da v2.1.260, alterar o esforço no Fable 5.1 com uma chave de API ou uma assinatura Claude também invalidava o cache.

<h3 id="turning-on-fast-mode">
  Turning on fast mode
</h3>

Habilitar [fast mode](/docs/pt/fast-mode) adiciona um cabeçalho de solicitação que faz parte da chave de cache, então a primeira solicitação que Claude Code envia com fast mode ativado lê todo o histórico de conversa sem acertos de cache. Claude Code define esse cabeçalho uma vez quando um turno começa e o mantém durante todo o turno, então quando você ativa fast mode enquanto Claude está trabalhando, a falha de cache do cabeçalho acontece na primeira solicitação do seu próximo turno. Esses tokens de entrada não armazenados em cache são cobrados com [fast mode rates](/docs/pt/fast-mode#understand-the-cost-tradeoff), é por isso que ativar no início de uma sessão custa menos do que ativar profundamente em uma longa. Se seu modelo atual não suportar fast mode, habilitar fast mode também [muda seu modelo](#switching-models), e essa mudança inicia um cache novo por conta própria a partir da próxima solicitação no turno em execução.

O custo se aplica uma vez por conversa. Após o primeiro turno de fast mode, Claude Code continua enviando o cabeçalho e varia apenas a configuração de velocidade da solicitação, que não faz parte da chave de cache. Desativar fast mode, o [fallback automático para velocidade padrão](/docs/pt/fast-mode#handle-rate-limits) após um limite de taxa, e ativá-lo novamente mais tarde mantêm o cache. Se você [ficar sem créditos de uso](/docs/pt/fast-mode#handle-rate-limits) no meio da sessão, Claude Code tenta novamente cada solicitação de fast mode rejeitada em velocidade padrão da mesma forma, então esse fallback também mantém o cache. `/clear` e `/compact` redefinem isso, já que reconstruem o cache nesses pontos de qualquer forma.

<h3 id="connecting-or-disconnecting-an-mcp-server">
  Connecting or disconnecting an MCP server
</h3>

As definições de ferramentas ficam na camada de prompt do sistema, então o cache se invalida quando o conjunto de definições de ferramentas na solicitação muda entre turnos. Alternar a [advisor tool](/docs/pt/advisor) é uma exceção: sua definição fica após o ponto de interrupção do cache, então habilitar ou desabilitar `/advisor` mantém o prefixo armazenado em cache intacto. Se uma mudança de [MCP server](/docs/pt/mcp) faz isso depende se suas ferramentas são adiadas por [tool search](/docs/pt/mcp#scale-with-mcp-tool-search) ou carregadas no prefixo:

* **Deferred tools**, o padrão em modelos suportados: um servidor conectando, desconectando ou alterando sua lista de ferramentas apenas anexa novo conteúdo e não perturba nada já armazenado em cache.
* **Tools loaded into the prefix**: qualquer mudança nelas invalida o cache. Isso acontece quando [tool search está indisponível ou desabilitado](/docs/pt/mcp#configure-tool-search), como em modelos da plataforma de agentes do Google Cloud anteriores à geração Claude 4.5, com um gateway `ANTHROPIC_BASE_URL` personalizado, ou em uma implantação do Microsoft Foundry [hospedada no Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options) uma vez que Claude Code detecta que a implantação rejeita tool search. Também acontece para um servidor ou ferramenta marcada [`alwaysLoad`](/docs/pt/mcp#exempt-a-server-from-deferral), e para definições mantidas na frente por [threshold-based loading](/docs/pt/mcp#configure-tool-search).

Quando as ferramentas são carregadas no prefixo, a causa mais comum de uma invalidação é um servidor conectando ou desconectando no meio da sessão, o que pode acontecer sem nenhuma ação da sua parte: o processo de um servidor stdio sai, uma sessão HTTP expira, ou um servidor [reconecta automaticamente após uma falha transitória](/docs/pt/mcp#automatic-reconnection). Um servidor conectado também pode enviar uma [dynamic tool update](/docs/pt/mcp#dynamic-tool-updates) que altera sua lista de ferramentas.

Editar sua configuração MCP não muda o cache por si só. A nova configuração entra em vigor apenas após uma reinicialização, que é quando o servidor conecta ou desconecta.

<h3 id="enabling-or-disabling-a-plugin">
  Enabling or disabling a plugin
</h3>

Quando você habilita ou desabilita um [plugin](/docs/pt/plugins/overview), o que a mudança custa depende de quais tipos de componentes o plugin fornece. Os casos abaixo cobrem cada tipo de componente, quando Claude Code aplica a mudança e o que acontece quando você desabilita um plugin novamente na mesma sessão.

<h4 id="plugin-components-that-keep-the-cache">
  Plugin components that keep the cache
</h4>

Claude Code nunca invalida o cache para skills, commands, agents, hooks, monitors ou themes de um plugin. Ele anexa seu conteúdo após a conversa existente, então a próxima solicitação paga por esse conteúdo e ainda lê tudo antes dele do cache.

<h4 id="plugins-that-provide-mcp-servers">
  Plugins that provide MCP servers
</h4>

Quando você habilita ou desabilita um plugin que fornece [MCP servers](/docs/pt/plugins/components#mcp-servers), Claude Code segue as mesmas regras de quando você [conecta ou desconecta um MCP server](#connecting-or-disconnecting-an-mcp-server):

* Se Claude Code adia as ferramentas do servidor, ele mantém o cache.
* Se Claude Code as carrega no prefixo, a próxima solicitação relê toda a conversa.

<h4 id="code-intelligence-plugins">
  Code intelligence plugins
</h4>

Quando você habilita um [code intelligence plugin](/docs/pt/plugins/code-intelligence), Claude obtém a [LSP tool](/docs/pt/tools-reference#lsp-tool-behavior).

<h4 id="when-plugin-changes-apply">
  When plugin changes apply
</h4>

Uma mudança que você faz no menu `/plugin` passa por [`/reload-plugins`](/docs/pt/plugins/cli-reference#reload-plugins), que Claude Code executa para você quando você fecha o menu. Você paga o custo, seja anúncios anexados ou uma releitura completa, no primeiro turno após a mudança ser aplicada. Claude Code também pode aplicar uma mudança por conta própria:

* Para um plugin com uma fonte `command`, Claude Code [pode recarregar o plugin em si](/docs/pt/plugins/loading#when-a-command-source-re-runs).
* Quando você [instala um plugin da interface `/plugin`](/docs/pt/plugins/install#install-a-plugin), Claude Code pode ativá-lo durante a instalação. O resumo da instalação informa se fez isso.
* Quando você [move a sessão com `/cd`](/docs/pt/permissions#move-the-session-to-another-directory) na v2.1.246 ou posterior, Claude Code aplica os plugins que as configurações do novo diretório habilitam como parte da mudança, sem o aviso de releitura completa que mantém um `/reload-plugins`.
* Em sessões interativas, quando você adiciona ou remove um plugin em uma [folder of plugins](/docs/pt/plugins/create#load-a-directory-or-archive-for-one-session) que você passou com `--plugin-dir`, a mudança se aplica imediatamente. Se aplicá-la acionaria uma releitura completa, Claude Code retém a mudança e mostra um aviso para executar `/reload-plugins`. Requer Claude Code v2.1.265 ou posterior.

Quando `/reload-plugins` é executado e o recarregamento acionaria uma releitura completa, Claude Code mostra um aviso e não aplica o recarregamento. Execute `/reload-plugins --force` para aplicá-lo de qualquer forma.

`/reload-plugins` também é executado em sessões sem um terminal interativo, como o aplicativo de desktop, o Agent SDK e [non-interactive mode](/docs/pt/headless) com `-p`, quando você o digita diretamente na sessão. Requer Claude Code v2.1.260 ou posterior.

Nessas sessões, o recarregamento aplica tudo exceto mudanças de MCP server de plugin, que [entram em vigor em sua próxima sessão](/docs/pt/plugins/cli-reference#reload-plugins) e portanto nunca custam uma releitura completa no meio da sessão.

<h4 id="plugins-you-enable-and-then-disable-in-one-session">
  Plugins you enable and then disable in one session
</h4>

Quando você desabilita um plugin que habilitou anteriormente na sessão, Claude Code restaura a forma de solicitação anterior. Se esse prefixo ainda estiver dentro de seu [cache lifetime](#cache-lifetime), a próxima solicitação lê a entrada de cache mais antiga em vez de reconstruir.

<h3 id="denying-an-entire-tool">
  Denying an entire tool
</h3>

Se você adicionar um nome de ferramenta simples como `Bash` ou `WebFetch` como uma [deny rule](/docs/pt/permissions#manage-permissions), Claude não pode chamar essa ferramenta a partir de sua próxima solicitação em diante, seja você adicionar a regra através de `/permissions` ou [editando um arquivo de configurações diretamente](/docs/pt/settings#when-edits-take-effect). Isso inclui uma regra que você adiciona através de `/permissions` no meio de um turno.

Quando [tool search](/docs/pt/mcp#scale-with-mcp-tool-search) está ativo, que é o padrão em modelos suportados, as definições de ferramentas da solicitação não mudam e o prefixo armazenado em cache sobrevive. Quando tool search está indisponível ou desabilitado, Claude Code remove a definição da próxima solicitação, o que invalida o cache, e o mesmo acontece ao remover a regra depois.

Apenas uma deny rule que corresponde na posição do nome da ferramenta bloqueia uma ferramenta dessa forma: um nome de ferramenta simples, a forma equivalente `Bash(*)`, ou um [tool-name glob](/docs/pt/permissions#tool-name-wildcards) como `"*"`. Um glob que corresponde apenas a ferramentas MCP, como `"mcp__*"`, bloqueia essas ferramentas da mesma forma. Deny rules com escopo como `Bash(rm *)`, e todas as regras allow e ask, não mudam quais ferramentas Claude vê. Claude Code as verifica quando Claude tenta uma chamada, deixando o prefixo intacto.

<h3 id="compacting-the-conversation">
  Compacting the conversation
</h3>

[Compaction](/docs/pt/context-window#what-survives-compaction) substitui seu histórico de mensagens por um resumo. Por design, isso invalida a camada de conversa, já que a próxima solicitação tem um histórico novo e mais curto que não compartilha um prefixo com o antigo. Claude Code reutiliza a camada de prompt do sistema a menos que a conversa tenha sido [retomada mantendo um prompt do sistema que teria mudado de outra forma](#resuming-a-session); nesse caso, a primeira compactação muda para o prompt atual e essa camada é reconstruída uma vez. Ele recarrega o contexto do projeto do disco, que cache-hits apenas se CLAUDE.md e memory não tiverem mudado desde o início da sessão.

Para produzir o resumo, Claude Code envia uma solicitação separada com o mesmo prompt do sistema, ferramentas e histórico que sua conversa, mais uma instrução de sumarização anexada como uma mensagem de usuário final. Enquanto o cache está quente, essa solicitação lê seu prefixo do cache, então um `/compact` no meio da sessão custa uma fração do que o tamanho do contexto sugere e gasta a maior parte do tempo gerando o resumo.

Após uma pausa mais longa que o [cache lifetime](#cache-lifetime), não há cache deixado para ler, então a solicitação de sumarização reprocessa o histórico completo como entrada não armazenada em cache. É por isso que `/compact` custa mais quando você [retoma uma sessão antiga](/docs/pt/sessions#resume-from-a-summary). Em ambos os casos quente e frio, o turno após compactação reconstrói o cache de conversa apenas para o resumo muito mais curto, então esse turno não é a parte lenta.

<Tip>
  Compaction funciona a seu favor quando o contexto que você descarta é conteúdo que você não precisa mais. Para escolher quando sua sobrecarga acontece, execute `/compact` em uma pausa natural em seu trabalho, como entre tarefas, em vez de esperar que a auto-compactação seja acionada no meio da tarefa. Se você seguiu um caminho que deseja abandonar inteiramente, [`/rewind`](#rewinding-the-conversation) para um turno anterior em vez disso. Rewind trunca de volta para um prefixo que já está armazenado em cache, em vez de construir um novo como compaction faz.
</Tip>

<h3 id="accumulating-many-images">
  Accumulating many images
</h3>

A API limita quantas imagens e PDFs cada solicitação pode carregar. Para os números atuais, veja [Request limits](https://platform.claude.com/docs/en/build-with-claude/vision#request-limits) na documentação da API. Claude Code também limita o tamanho total das imagens e PDFs em uma solicitação, então capturas de tela grandes atingem o limite com menos imagens do que pequenas.

Quando a próxima solicitação passaria por qualquer limite, Claude Code remove um lote das imagens e PDFs mais antigas do que envia, o que deixa espaço para mais antes de precisar remover novamente. Claude não pode mais ver as imagens removidas. Se Claude precisar de uma delas novamente, compartilhe-a novamente.

Remover imagens muda as mensagens que as continham, então a próxima solicitação reprocessa a conversa a partir da mais antiga dessas mensagens em diante. Como Claude Code remove um lote por vez, você vê um turno mais lento por lote em vez de um com cada nova captura de tela.

<h3 id="upgrading-claude-code">
  Upgrading Claude Code
</h3>

Uma nova versão de Claude Code normalmente atualiza o prompt do sistema ou definições de ferramentas, então a primeira conversa que você inicia após uma atualização constrói seu cache do topo. [Auto-update](/docs/pt/setup#auto-updates) baixa novas versões em segundo plano, mas as aplica no próximo lançamento, nunca no meio da sessão, então você vê isso como um primeiro turno não armazenado em cache após reiniciar em vez de uma surpresa durante uma sessão. Defina `DISABLE_AUTOUPDATER=1` para controlar quando as atualizações se aplicam.

<Note>
  Para o que custa retomar uma conversa que você iniciou antes da atualização, veja [Resuming a session](#resuming-a-session).
</Note>

<h2 id="actions-that-keep-the-cache">
  Ações que mantêm o cache
</h2>

Essas ações ou anexam ao final da conversa ou não tocam na solicitação. Algumas delas, como editar CLAUDE.md, mantêm o cache pela mesma razão que a mudança não chega à sessão em execução até `/clear`, `/compact` ou uma reinicialização.

* [Editando arquivos em seu repositório](#editing-files-in-your-repository)
* [Editando CLAUDE.md durante a sessão](#editing-claude-md-mid-session)
* [Alterando modo de permissão](#changing-permission-mode)
* [Alterando estilo de saída](#changing-output-style)
* [Invocando skills e comandos](#invoking-skills-and-commands)
* [Executando `/recap`](#running-%2Frecap)
* [Revertendo a conversa](#rewinding-the-conversation)
* [Gerando um subagente](#subagents-and-the-cache)

<h3 id="editing-files-in-your-repository">
  Editando arquivos em seu repositório
</h3>

O conteúdo dos arquivos entra no contexto apenas quando Claude os lê, e as leituras se anexam à conversa. Editar um arquivo que Claude leu anteriormente não muda retroativamente a leitura anterior no histórico. Em vez disso, Claude Code anexa um `<system-reminder>` observando que o arquivo mudou, e Claude o relê se necessário.

<h3 id="editing-claude-md-mid-session">
  Editando CLAUDE.md durante a sessão
</h3>

Seus arquivos CLAUDE.md no nível do projeto-raiz e do usuário são lidos uma vez no início da sessão e mantidos na memória. Editá-los durante a sessão não invalida o cache, mas a edição também não se aplica. Claude continua trabalhando com a versão que foi carregada no início da sessão. O novo conteúdo é carregado na próxima `/clear`, `/compact` ou reinicialização.

[Arquivos CLAUDE.md aninhados em subdiretórios](/docs/pt/memory) e [regras com frontmatter `paths:`](/docs/pt/memory#path-specific-rules) são carregados depois, quando Claude lê um arquivo correspondente pela primeira vez. Editar um antes de ser carregado tem efeito. Depois de carregado, o conteúdo faz parte do histórico da conversa, então uma edição durante a sessão não muda retroativamente.

<h3 id="changing-permission-mode">
  Alterando modo de permissão
</h3>

Alternar entre [modos de permissão](/docs/pt/permission-modes), como de Manual para aceitar edições, não muda o prompt do sistema ou as definições de ferramentas, então as mudanças de modo são seguras para o cache. A exceção é o modo de plano com a configuração de modelo [`opusplan`](/docs/pt/model-config#opusplan-model-setting), que alterna o modelo entre Opus e Sonnet conforme você entra ou sai do modo de plano. Isso torna a alternância de modo uma [mudança de modelo](#switching-models).

<h3 id="changing-output-style">
  Alterando estilo de saída
</h3>

Quando você alterna [estilos de saída](/docs/pt/output-styles) durante a sessão com [`/output-style`](/docs/pt/output-styles#change-your-output-style), `/config` ou a configuração `outputStyle`, Claude usa o novo estilo a partir da sua próxima mensagem. Claude Code entrega as instruções do novo estilo como uma mensagem na conversa, então essa solicitação ainda lê o prompt do sistema e a conversa anterior do cache.

Antes da v2.1.251, uma mudança de estilo durante a sessão mantinha o cache mas não se aplicava até você executar `/clear` ou iniciar uma nova sessão.

<h3 id="invoking-skills-and-commands">
  Invocando skills e comandos
</h3>

[Skills](/docs/pt/skills) e [comandos](/docs/pt/commands) injetam suas instruções como mensagens do usuário no ponto de invocação. Nada anterior na conversa muda. Uma skill ou comando cujo frontmatter nomeia um `model` pode ser uma [mudança de modelo](#switching-models) para esse turno.

<h3 id="running-/recap">
  Executando `/recap`
</h3>

[`/recap`](/docs/pt/interactive-mode#session-recap) gera um resumo para exibição em seu terminal. Diferentemente de `/compact`, ele anexa o resumo como saída de comando em vez de substituir seu histórico de mensagens, então o prefixo em cache permanece intacto.

<h3 id="rewinding-the-conversation">
  Revertendo a conversa
</h3>

[`/rewind`](/docs/pt/checkpointing) trunca sua conversa de volta a um turno anterior. O histórico restante é o mesmo conteúdo do qual o cache foi construído naquele ponto, e o prompt do sistema e as camadas de contexto do projeto não mudam, então a próxima solicitação atinge a entrada de cache anterior. Cada turno desde então leu através desse prefixo, o que manteve a entrada ativa mesmo se o turno original foi há mais tempo do que o TTL.

Restaurar checkpoints de arquivo junto com a conversa não tem efeito separado no cache. O conteúdo dos arquivos entra no contexto apenas quando Claude os lê, o mesmo que [editar arquivos em seu repositório](#editing-files-in-your-repository).

<h2 id="resuming-a-session">
  Retomando uma sessão
</h2>

Quando você [retoma uma sessão](/docs/pt/sessions#resume-a-session), Claude Code envia toda a conversa novamente, e a solicitação lê do cache qualquer parte de seu prefixo que não tenha sido alterada e ainda esteja dentro do [tempo de vida do cache](#cache-lifetime). A tabela de camadas no topo desta página diz quais mudanças cada camada.

O prompt do sistema mudaria após uma [atualização do Claude Code](#upgrading-claude-code) ou com texto [`--append-system-prompt`](/docs/pt/cli-reference#system-prompt-flags) diferente na retomada. Por padrão, a conversa retomada mantém o prompt do sistema com o qual começou, portanto seu histórico ainda fica atrás do mesmo prompt, e a mudança entra em vigor assim que a conversa é compactada ou em uma nova conversa. [Sinalizadores de prompt do sistema em conversas retomadas](/docs/pt/cli-reference#system-prompt-flags-in-resumed-conversations) aborda os casos em que Claude Code reconstrói o prompt em cada solicitação.

<h2 id="cache-lifetime">
  Tempo de vida do cache
</h2>

Prefixos em cache expiram após um período de inatividade. Cada solicitação que acerta o cache redefine o temporizador, então o cache permanece aquecido enquanto você continua trabalhando. Após um intervalo longo o suficiente, a próxima solicitação recomputa a entrada completa e reestabelece o cache, o que é por que o primeiro turno de volta após se afastar pode ser notavelmente mais lento.

Em um plano Pro ou Max, quando você retoma uma sessão grande após um longo intervalo, Claude Code [oferece retomar de um resumo](/docs/pt/sessions#resume-from-a-summary) para que solicitações posteriores não carreguem o histórico completo.

O tempo de vida (TTL) controla quanto tempo um intervalo o cache sobrevive. A API oferece dois: um TTL de cinco minutos e um [TTL de uma hora](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#1-hour-cache-duration) que mantém o cache aquecido através de pausas mais longas, mas [cobra gravações de cache a uma taxa mais alta](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing). O TTL mais longo ajuda quando você deixa uma sessão ociosa e volta a ela, porque você pula o reprocessamento que um prefixo expirado custa. Custa mais em rajadas curtas de trabalho que nunca ficam ociosas por mais de cinco minutos, onde a taxa de gravação mais alta se aplica e o tempo de vida do cache mais longo não é utilizado.

<h3 id="which-ttl-each-request-gets">
  Qual TTL cada solicitação obtém
</h3>

Claude Code decide o TTL por solicitação, e cada solicitação se enquadra em um dos dois buckets fixos:

* **Conversa principal**: seus turnos interativos, execuções não-interativas `-p` e turnos do Agent SDK, mais os auxiliares que Claude Code executa inline com eles
* **Tudo mais**: as solicitações que Claude Code faz fora dessa conversa, como [subagentes](/docs/pt/sub-agents), [workflows](/docs/pt/workflows), [colegas](/docs/pt/agent-teams) em processo, forks, compactação e títulos de sessão

A menos que você escolha um TTL você mesmo, Claude Code solicita o TTL de uma hora apenas em uma assinatura Claude dentro do uso incluído do seu plano. Lá ele solicita a hora para a conversa principal, mais um pequeno conjunto de solicitações auxiliares que a Anthropic controla no lado do servidor. Esta tabela fornece o TTL padrão de cada bucket sob ambos os tipos de cobrança.

| Bucket de solicitação | Assinatura Claude, dentro do uso do plano                                                      | Créditos de uso, chave de API ou provedor de nuvem |
| --------------------- | ---------------------------------------------------------------------------------------------- | -------------------------------------------------- |
| Conversa principal    | Uma hora                                                                                       | Cinco minutos                                      |
| Tudo mais             | Cinco minutos, exceto as solicitações auxiliares controladas pelo servidor, que obtêm uma hora | Cinco minutos                                      |

Depois que você ultrapassa o limite de uso do seu plano e Claude Code usa [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), você é cobrado por esse uso, então Claude Code reduz a conversa principal para o TTL de cinco minutos mais barato. Para manter o TTL de uma hora lá, [escolha o TTL você mesmo](#choose-the-ttl-yourself).

<h3 id="choose-the-ttl-yourself">
  Escolha o TTL você mesmo
</h3>

Você pode definir um TTL para qualquer bucket. Cada controle leva `5m` ou `1h`, e Claude Code ignora qualquer outro valor.

* **Conversa principal**: a configuração [`promptCacheTtl`](/docs/pt/settings-reference#promptcachettl), ou a variável de ambiente [environment variable](/docs/pt/env-vars) `CLAUDE_CODE_PROMPT_CACHE_TTL`
* **Tudo mais**: a configuração [`subagentPromptCacheTtl`](/docs/pt/settings-reference#subagentpromptcachettl), ou a variável de ambiente `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`

Ambas as configurações e ambas as variáveis de ambiente exigem Claude Code v2.1.242 ou posterior. Se você se conectar com uma chave de API ou usar um provedor de nuvem, defina `promptCacheTtl` para `1h` para dar à conversa principal um cache de uma hora. As solicitações fora dela mantêm o padrão de cinco minutos até que você escolha um TTL para esse bucket também.

Quando mais de um controle se aplica, Claude Code leva a primeira correspondência nesta ordem:

1. `FORCE_PROMPT_CACHING_5M=1`, que força cinco minutos para ambos os buckets
2. A variável de ambiente do bucket
3. A configuração do bucket
4. Para as solicitações de um subagente, o valor `cacheTtl` no campo frontmatter [`experimental`](/docs/pt/sub-agents#supported-frontmatter-fields) do subagente, que exige Claude Code v2.1.248 ou posterior. Claude Code ignora um `1h` lá enquanto sua assinatura Claude está usando créditos de uso
5. `ENABLE_PROMPT_CACHING_1H=1`, que solicita uma hora para ambos os buckets
6. O [padrão para o bucket da solicitação](#which-ttl-each-request-gets)

Defina `FORCE_PROMPT_CACHING_5M=1` quando você está depurando o comportamento do cache, comparando os dois TTLs ou substituindo um TTL mais longo definido em [configurações gerenciadas](/docs/pt/managed-settings).

Para confirmar qual TTL as gravações de cache da sua conversa principal usaram, execute `claude -p "hello" --output-format json` e leia `usage.cache_creation` no resultado. Claude Code relata gravações de cache de uma hora em `ephemeral_1h_input_tokens` e gravações de cache de cinco minutos em `ephemeral_5m_input_tokens`.

Através de um gateway LLM que você define com `ANTHROPIC_BASE_URL`, parte da solicitação de uma hora viaja no cabeçalho `anthropic-beta`, então configure o gateway para [encaminhar esse cabeçalho inalterado](/docs/pt/llm-gateway-protocol#request-headers). O TTL de uma hora não está disponível através do [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway#availability-and-limitations). No Amazon Bedrock, suporte a prompt caching, comprimento mínimo de prefixo armazenável em cache e disponibilidade de TTL de uma hora variam por modelo. Se as contagens de tokens de cache permanecerem em zero, verifique [modelos, regiões e limites suportados](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) na documentação do Amazon Bedrock.

<h2 id="cache-scope">
  Escopo do cache
</h2>

Em Claude Code, o cache é efetivamente limitado a uma máquina e diretório. Cada conversa carrega o diretório de trabalho, plataforma, shell e versão do SO, e o prompt do sistema nomeia seus caminhos de memória automática, então duas sessões em diretórios diferentes constroem prefixos diferentes e perdem o cache uma da outra. Isso inclui worktrees do mesmo repositório, já que cada worktree tem seu próprio diretório de trabalho.

Sessões que você executa em paralelo no mesmo diretório constroem prefixos correspondentes e leem o cache uma da outra. Sessões sequenciais compartilham o prefixo apenas quando o snapshot de status git na inicialização corresponde, já que cada conversa também carrega a branch e commits recentes desse snapshot.

O cache de API subjacente é mais amplo. Os caches são isolados entre organizações e, em alguns provedores, [entre workspaces dentro de uma organização](https://platform.claude.com/docs/pt/build-with-claude/prompt-caching#cache-storage-and-sharing). Dentro desses limites, quaisquer duas solicitações com o mesmo modelo e prefixo leem o mesmo cache. Para chamadores do Agent SDK executando frotas de processos automatizados, veja [melhorar prompt caching entre usuários e máquinas](/docs/pt/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) para suprimir as seções por máquina do prompt do sistema e compartilhar o cache entre máquinas.

<h2 id="check-cache-performance">
  Verificar desempenho do cache
</h2>

O desempenho do cache aparece como duas contagens de tokens que a API relata em cada resposta. A forma mais direta de observá-los ao vivo é um [script de statusline](/docs/pt/statusline) que lê o objeto `current_usage`:

| Campo                         | Significado                                                                                                                                                                     |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cache_creation_input_tokens` | Tokens escritos no cache neste turno, cobrados à taxa de gravação de cache                                                                                                      |
| `cache_read_input_tokens`     | Tokens servidos do cache neste turno, cobrados à [taxa de token em cache](https://platform.claude.com/docs/en/about-claude/pricing) do modelo, abaixo da taxa de entrada padrão |

Uma alta proporção de leitura para criação significa que o caching está funcionando bem. Se a criação permanecer alta turno após turno, algo está mudando em seu prefixo. A seção [ações que invalidam o cache](#actions-that-invalidate-the-cache) lista as causas usuais.

Para um resumo por sessão, execute `/usage`. Após a primeira resposta da conversa principal, Claude Code adiciona uma [linha `Prompt cache (main)`](/docs/pt/costs#prompt-cache-statistics) ao bloco Session, mostrando a taxa de acerto da sessão, contagem de falhas e se o cache está aquecido neste momento. Um script de statusline pode ler os mesmos números do [objeto `prompt_cache`](/docs/pt/statusline#prompt-cache-fields). Ambos requerem Claude Code v2.1.251 ou posterior.

A linha `Prompt cache (main)` também nomeia a provável causa da última falha quando Claude Code consegue identificar uma, por exemplo `likely cause: tool definitions changed`. O texto de causa provável requer Claude Code v2.1.260 ou posterior.

Para visibilidade em toda uma organização, o exportador OpenTelemetry relata tokens de leitura e criação de cache por usuário e sessão. Veja [Monitorar uso](/docs/pt/monitoring-usage) para a referência de métrica e atributo de evento.

<h2 id="subagents-and-the-cache">
  Subagents e o cache
</h2>

Um [subagent](/docs/pt/sub-agents) inicia sua própria conversa com seu próprio prompt do sistema e conjunto de ferramentas, separado do pai. Sua primeira solicitação não lê o cache do pai, porque os dois prefixos diferem, e aquece um cache próprio ao longo de seus turnos. Subagents ficam fora do [bucket TTL](#which-ttl-each-request-gets) da conversa principal, então recebem cinco minutos mesmo em uma assinatura até você [escolher um mais longo](#choose-the-ttl-yourself).

O cache do pai não é afetado. Do lado do pai, a chamada e resultado do subagent se anexam à conversa, deixando o prefixo do pai intacto.

Um [fork](/docs/pt/sub-agents#fork-the-current-conversation), por contraste, herda o prompt do sistema, ferramentas e histórico de conversa do pai exatamente, então sua primeira solicitação lê o cache do pai.

Outras solicitações também podem ler um prefixo que uma solicitação anterior armazenou em cache:

* **Cópias de sessão**: uma sessão que você [copia com `/fork`](/docs/pt/agent-view#copy-the-session-with-%2Ffork) recebe sua instrução de isolamento como uma mensagem no final da conversa copiada, então o cache que a conversa original construiu permanece intacto.
* **Compactação**: a chamada de resumo descrita em [Compactando a conversa](#compacting-the-conversation) usa a mesma abordagem de compartilhamento de prefixo.
* **Subagents retomados**: quando Claude [retoma um subagent](/docs/pt/sub-agents#resume-subagents), a primeira solicitação da execução retomada pode ler o cache que a execução original aqueceu.
* **Workflow fan-outs**: em um [workflow fan-out](/docs/pt/workflows#prompt-caching-in-a-fan-out) de agentes com o mesmo prefixo, Claude Code mantém todos exceto o primeiro por até 5 segundos por padrão, então suas primeiras solicitações podem ler o prefixo que o primeiro agente armazenou em cache.

<h2 id="disable-prompt-caching">
  Desabilitar prompt caching
</h2>

Desabilitar caching é ocasionalmente útil ao depurar comportamento de caching com um modelo ou provedor específico. Para desativá-lo, defina uma dessas variáveis de ambiente como `1`:

| Variável                        | Efeito                            |
| ------------------------------- | --------------------------------- |
| `DISABLE_PROMPT_CACHING`        | Desabilitar para todos os modelos |
| `DISABLE_PROMPT_CACHING_HAIKU`  | Desabilitar para Haiku apenas     |
| `DISABLE_PROMPT_CACHING_SONNET` | Desabilitar para Sonnet apenas    |
| `DISABLE_PROMPT_CACHING_OPUS`   | Desabilitar para Opus apenas      |
| `DISABLE_PROMPT_CACHING_FABLE`  | Desabilitar para Fable apenas     |

Para definir a política de caching em toda uma organização, coloque qualquer uma dessas ou as [variáveis de TTL](#cache-lifetime) no bloco `env` de [configurações gerenciadas](/docs/pt/managed-settings). Para uso normal, deixe o caching habilitado.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Lições de construir Claude Code: Prompt caching é tudo](https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything): a lógica de design para modo de plano, carregamento de ferramentas adiado e compactação
* [Explorar a janela de contexto](/docs/pt/context-window): o que carrega em contexto e quando
* [Reduzir uso de tokens](/docs/pt/costs#reduce-token-usage): estratégias além de caching para gerenciar tamanho de contexto
* [Rastrear e reduzir custos](/docs/pt/agent-sdk/cost-tracking): rastreamento de tokens de cache e configuração de TTL para chamadores do Agent SDK
* [Prompt caching](https://platform.claude.com/docs/pt/build-with-claude/prompt-caching): o mecanismo de API subjacente, breakpoints e preços
