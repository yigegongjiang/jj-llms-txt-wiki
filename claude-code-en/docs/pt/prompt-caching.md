> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Como Claude Code usa prompt caching

> Claude Code gerencia prompt caching automaticamente. Veja por que uma mudança de modelo dispara um turno lento sem cache, o que `/compact` custa, por que edições de CLAUDE.md não se aplicam no meio da sessão e como verificar sua taxa de acerto de cache.

Prompt caching torna Claude Code mais rápido e eficiente em termos de custo. Sem caching, a API reprocessaria seu histórico completo a cada turno. Com caching, ela reutiliza o que já processou e faz apenas o novo trabalho para o que mudou.

Claude Code gerencia prompt caching para você, a menos que você [desative-o](#disable-prompt-caching). Ainda é útil saber como o prompt caching funciona, porque algumas ações invalidam o cache e tornam a próxima resposta mais lenta e cara enquanto ele se reconstrói. Esta página cobre quais ações são essas, por que algumas configurações aguardam uma reinicialização para serem aplicadas e como verificar o desempenho do cache quando o uso parece alto.

<h2 id="how-the-cache-is-organized">
  Como o cache é organizado
</h2>

Cada vez que você envia uma mensagem em Claude Code, ele faz uma nova solicitação de API. O modelo não se lembra de nada entre solicitações, então Claude Code reenvia o contexto completo: o prompt do sistema, seu contexto de projeto, cada mensagem anterior e resultado de ferramenta, e sua nova mensagem. Novo conteúdo é anexado ao final, o que significa que a maior parte de cada solicitação é idêntica à anterior. Prompt caching é como a API evita reprocessar a parte que não mudou.

A API faz cache correspondendo ao início de cada solicitação, chamado de prefixo, contra conteúdo que processou recentemente. Em um turno normal, o prefixo é a solicitação anterior inteira e apenas a troca mais recente é nova. A correspondência é exata, então uma mudança em qualquer lugar no prefixo recomputa tudo depois dela. Não há caching por arquivo ou por segmento. Veja [como o prompt caching funciona](https://platform.claude.com/docs/pt/build-with-claude/prompt-caching#how-prompt-caching-works) na referência da API para o mecanismo subjacente.

<img src="https://mintcdn.com/claude-code/VbDJw--l6T9a9Wvm/images/prompt-caching-prefix.svg?fit=max&auto=format&n=VbDJw--l6T9a9Wvm&q=85&s=f2e8f0b8298a50305fe428ca3f1d1594" className="dark:hidden" alt="Quatro turnos mostrados como barras horizontais crescentes. A solicitação de cada turno contém tudo do turno anterior mais a troca mais recente anexada ao final. Nos turnos dois e três, o prefixo inalterado é lido do cache e apenas a nova troca é processada. No turno quatro, o prompt do sistema mudou, então o prefixo não corresponde mais e toda a solicitação é reprocessada e escrita." width="720" height="454" data-path="images/prompt-caching-prefix.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/prompt-caching-prefix-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=297dc1c639f0915cae858d0c4b6f3be5" className="hidden dark:block" alt="Quatro turnos mostrados como barras horizontais crescentes. A solicitação de cada turno contém tudo do turno anterior mais a troca mais recente anexada ao final. Nos turnos dois e três, o prefixo inalterado é lido do cache e apenas a nova troca é processada. No turno quatro, o prompt do sistema mudou, então o prefixo não corresponde mais e toda a solicitação é reprocessada e escrita." width="720" height="454" data-path="images/prompt-caching-prefix-dark.svg" />

Para aproveitar ao máximo a correspondência de prefixo, Claude Code ordena cada solicitação para que o conteúdo que raramente muda entre turnos venha primeiro:

| Camada              | Conteúdo                                                          | Muda quando                                                                          |
| ------------------- | ----------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| Prompt do sistema   | Instruções principais, definições de ferramentas, estilo de saída | O conjunto de definições de ferramentas carregadas muda, ou Claude Code é atualizado |
| Contexto do projeto | CLAUDE.md, memória automática, regras sem escopo                  | A sessão começa, ou após `/clear` ou `/compact`                                      |
| Conversa            | Suas mensagens, respostas de Claude, resultados de ferramentas    | A cada turno                                                                         |

Uma mudança na camada de conversa deixa o prompt do sistema e o contexto do projeto em cache. Uma mudança no prompt do sistema invalida tudo, porque todo o conteúdo posterior agora fica atrás de um prefixo diferente. A terceira coluna fornece gatilhos comuns em vez de uma lista exaustiva, e as seções abaixo cobrem o conjunto completo, incluindo conteúdo como estilo de saída que é fixado no início da sessão.

A regra de correspondência de prefixo explica a maioria dos comportamentos nesta página. [Plan Mode](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode) e [carregamento de skills](/docs/pt/skills), por exemplo, anexam suas instruções como mensagens de conversa, então o prefixo em cache permanece intacto.

Duas configurações não fazem parte do texto do prompt, então não aparecem na tabela de camadas, mas ambas fazem parte da chave de cache:

* **Model**: cada modelo tem seu próprio cache. Trocar modelos recomputa toda a solicitação mesmo quando o conteúdo é idêntico. Veja [Trocar modelos](#switching-models) abaixo.
* **Effort level**: cada nível de esforço tem seu próprio cache para o mesmo modelo. Alterá-lo no meio da sessão recomputa toda a solicitação, e Claude Code pede que você confirme antes de aplicar a mudança. Veja [Alterando nível de esforço](#changing-effort-level) abaixo.

<Tip>
  Escolha seu modelo e nível de esforço no início de uma sessão, depois salve `/compact` para pausas naturais entre tarefas. Quanto menos mudanças você fizer no meio da tarefa, maior será sua taxa de acerto de cache.
</Tip>

<h3 id="where-the-cache-lives">
  Onde o cache reside
</h3>

O caching acontece no lado do servidor, na infraestrutura que serve seu modelo. Onde fica depende de como você se autentica:

* **Chave de API, assinatura Claude ou [Claude Platform on AWS](/docs/pt/claude-platform-on-aws)**: o cache reside na infraestrutura da Anthropic, acessado através da [Claude API](https://platform.claude.com/docs)
* **Amazon Bedrock ou Google Cloud's Agent Platform**: o cache reside na infraestrutura de serviço do seu provedor de nuvem
* **Microsoft Foundry**: as solicitações são roteadas para a infraestrutura da Anthropic
* **`ANTHROPIC_BASE_URL` personalizado ou [LLM gateway](/docs/pt/llm-gateway)**: o cache reside onde suas solicitações são encaminhadas, e se o caching funciona depende do gateway

Para o que cada provedor armazena e processa, veja [uso de dados](/docs/pt/data-usage). Onde quer que o cache resida, as entradas expiram após um período de inatividade, e [Cache lifetime](#cache-lifetime) abaixo cobre o TTL e como estendê-lo.

<h2 id="actions-that-invalidate-the-cache">
  Ações que invalidam o cache
</h2>

Essas ações fazem com que a próxima solicitação perca parte ou todo o cache. Você vê um turno mais lento e mais caro uma única vez, após o qual o novo prefixo é armazenado em cache. A maioria delas é evitável no meio da tarefa uma vez que você sabe que têm um custo. Uma mudança de modelo pode parecer gratuita até você notar o turno mais lento que se segue.

* [Trocar modelos](#switching-models)
* [Alterar nível de esforço](#changing-effort-level)
* [Ativar modo rápido](#turning-on-fast-mode)
* [Conectar ou desconectar um servidor MCP](#connecting-or-disconnecting-an-mcp-server)
* [Ativar ou desativar um plugin](#enabling-or-disabling-a-plugin)
* [Negar uma ferramenta inteira](#denying-an-entire-tool)
* [Compactar a conversa](#compacting-the-conversation)
* [Atualizar Claude Code](#upgrading-claude-code)

<h3 id="switching-models">
  Trocar modelos
</h3>

Cada modelo tem seu próprio cache. Trocar com [`/model`](/docs/pt/model-config#setting-your-model) significa que a próxima solicitação lê todo o histórico de conversa sem acertos de cache, mesmo que o conteúdo seja idêntico.

A [configuração de modelo `opusplan`](/docs/pt/model-config#opusplan-model-setting) resolve para Opus durante o modo de plano e Sonnet durante a execução, então cada alternância de modo de plano é uma mudança de modelo e inicia um cache novo.

[Fallback automático de modelo](/docs/pt/model-config#automatic-model-fallback) no Fable 5 também é uma mudança de modelo. Quando um classificador de segurança sinaliza uma solicitação, Claude Code a executa novamente no modelo Opus padrão e a sessão continua lá.

<h3 id="changing-effort-level">
  Alterar nível de esforço
</h3>

O cache é codificado por [nível de esforço](/docs/pt/model-config#adjust-effort-level) bem como modelo, então trocar com `/effort` significa que a próxima solicitação lê todo o histórico de conversa sem acertos de cache. Uma vez que uma conversa começou, Claude Code mostra um diálogo de confirmação antes de aplicar uma mudança de esforço que invalidaria o cache. Uma mudança que resolve para o mesmo nível já em vigor, como definir o padrão do modelo explicitamente, pula o diálogo e mantém o cache.

<h3 id="turning-on-fast-mode">
  Ativar modo rápido
</h3>

Ativar [modo rápido](/docs/pt/fast-mode) adiciona um cabeçalho de solicitação que faz parte da chave de cache, então a próxima solicitação lê todo o histórico de conversa sem acertos de cache. Esses tokens de entrada sem cache são cobrados com [taxas de modo rápido](/docs/pt/fast-mode#understand-the-cost-tradeoff), e é por isso que ativá-lo no início de uma sessão custa menos do que ativá-lo profundamente em uma longa. Ativar modo rápido a partir de um modelo que não é Opus também [muda seu modelo](#switching-models), o que inicia um cache novo por conta própria.

O custo se aplica uma vez por conversa. Após o primeiro turno de modo rápido, Claude Code continua enviando o cabeçalho e varia apenas a configuração de velocidade da solicitação, que não faz parte da chave de cache. Desativar modo rápido, o [fallback automático para velocidade padrão](/docs/pt/fast-mode#handle-rate-limits) após um limite de taxa e ativá-lo novamente mais tarde mantêm o cache. `/clear` e `/compact` redefinem isso, já que reconstruem o cache nesses pontos de qualquer forma.

<h3 id="connecting-or-disconnecting-an-mcp-server">
  Conectar ou desconectar um servidor MCP
</h3>

As definições de ferramentas ficam na camada de prompt do sistema, então o cache se invalida quando o conjunto de definições de ferramentas na solicitação muda entre turnos. Alternar a [ferramenta advisor](/docs/pt/advisor) é uma exceção: sua definição fica após o ponto de quebra do cache, então ativar ou desativar `/advisor` mantém o prefixo em cache intacto. Se uma mudança de [servidor MCP](/docs/pt/mcp) faz isso depende se suas ferramentas são adiadas por [busca de ferramentas](/docs/pt/mcp#scale-with-mcp-tool-search) ou carregadas no prefixo:

* **Ferramentas adiadas**, o padrão em modelos suportados: um servidor se conectando, desconectando ou alterando sua lista de ferramentas apenas anexa novo conteúdo e não perturba nada já armazenado em cache.
* **Ferramentas carregadas no prefixo**: qualquer mudança nelas invalida o cache. Isso acontece quando [a busca de ferramentas não está disponível ou está desativada](/docs/pt/mcp#configure-tool-search), como em modelos Haiku, no Vertex AI ou com um gateway `ANTHROPIC_BASE_URL` customizado. Também acontece para um servidor ou ferramenta marcada [`alwaysLoad`](/docs/pt/mcp#exempt-a-server-from-deferral) e para definições mantidas na frente por [carregamento baseado em limite](/docs/pt/mcp#configure-tool-search).

Quando as ferramentas carregam no prefixo, a causa mais comum de uma invalidação é um servidor se conectando ou desconectando no meio da sessão, o que pode acontecer sem nenhuma ação da sua parte: o processo de um servidor stdio sai, uma sessão HTTP expira ou um servidor [se reconecta automaticamente após uma falha transitória](/docs/pt/mcp#automatic-reconnection). Um servidor conectado também pode enviar uma [atualização de ferramenta dinâmica](/docs/pt/mcp#dynamic-tool-updates) que muda sua lista de ferramentas.

Editar sua configuração de MCP não muda o cache por si só. A nova configuração entra em vigor apenas após uma reinicialização, que é quando o servidor se conecta ou desconecta.

<h3 id="enabling-or-disabling-a-plugin">
  Ativar ou desativar um plugin
</h3>

[Plugins](/docs/pt/plugins) agrupam vários tipos de componentes, e o custo de uma mudança depende de quais componentes o plugin fornece. Skills, comandos, agentes, hooks, servidores LSP, monitores e temas nunca invalidam o cache: qualquer coisa que eles adicionem à solicitação é anexada após a conversa existente, então a próxima solicitação paga pelo novo conteúdo mas ainda lê tudo antes dele do cache.

A exceção é um plugin que fornece [servidores MCP](/docs/pt/plugins-reference#mcp-servers). Ativar ou desativar um segue as mesmas regras que [conectar ou desconectar um servidor MCP](#connecting-or-disconnecting-an-mcp-server): o cache sobrevive quando as ferramentas do servidor são adiadas, e a próxima solicitação relê toda a conversa quando elas carregam no prefixo.

As mudanças de plugin se aplicam quando você executa [`/reload-plugins`](/docs/pt/discover-plugins#apply-plugin-changes-without-restarting) ou inicia uma nova sessão. O custo, seja anúncios anexados ou uma releitura completa, aparece no primeiro turno após o recarregamento, não quando você executa `/plugin install`, `/plugin enable` ou `/plugin disable`. A partir da v2.1.163, quando um recarregamento acionaria a releitura completa, `/reload-plugins` mostra um aviso e não aplica o recarregamento. Passe `--force` para aplicar mesmo assim.

Desativar um plugin que você ativou anteriormente na sessão restaura a forma de solicitação anterior. Se esse prefixo ainda estiver dentro de seu [tempo de vida do cache](#cache-lifetime), a próxima solicitação lê a entrada de cache mais antiga em vez de reconstruir.

<h3 id="denying-an-entire-tool">
  Negar uma ferramenta inteira
</h3>

Adicionar um nome de ferramenta simples como `Bash` ou `WebFetch` como uma [regra de negação](/docs/pt/permissions#manage-permissions) remove essa ferramenta do contexto de Claude completamente. As definições de ferramentas integradas carregam na camada de prompt do sistema, então adicionar ou remover uma dessas regras no meio da sessão invalida o cache. A mudança entra em vigor no próximo turno, quer você a adicione através de `/permissions` ou [editando um arquivo de configurações diretamente](/docs/pt/settings#when-edits-take-effect).

Apenas uma regra de negação que corresponde na posição do nome da ferramenta tem esse efeito: um nome de ferramenta simples, a forma equivalente `Bash(*)`, ou um [glob de nome de ferramenta](/docs/pt/permissions#tool-name-wildcards) como `"*"`. Um glob que corresponde apenas a ferramentas MCP, como `"mcp__*"`, remove essas ferramentas da mesma forma mas deixa o cache intacto quando as ferramentas correspondidas são [adiadas](#connecting-or-disconnecting-an-mcp-server), o padrão, já que definições adiadas nunca estiveram no prefixo em cache. Regras de negação com escopo como `Bash(rm *)` e todas as regras de permissão e pergunta não mudam quais ferramentas Claude vê. Claude Code as verifica quando Claude tenta fazer uma chamada, deixando o prefixo intacto.

<h3 id="compacting-the-conversation">
  Compactar a conversa
</h3>

[Compactação](/docs/pt/context-window#what-survives-compaction) substitui seu histórico de mensagens por um resumo. Por design, isso invalida a camada de conversa, já que a próxima solicitação tem um histórico novo e mais curto que não compartilha um prefixo com o antigo. Claude Code reutiliza a camada de prompt do sistema e recarrega o contexto do projeto do disco, que acerta o cache apenas se CLAUDE.md e memória não mudaram desde o início da sessão.

Para produzir o resumo, Claude Code envia uma solicitação única com o mesmo prompt do sistema, ferramentas e histórico que sua conversa, mais uma instrução de resumo anexada como uma mensagem de usuário final. Como compartilha seu prefixo, essa solicitação lê o cache existente em vez de reprocessar o histórico completo. A maior parte do tempo de compactação vai para gerar o resumo, não para uma perda de cache. O turno que se segue reconstrói o cache de conversa apenas para o resumo muito mais curto, então o turno pós-compactação não é a parte lenta.

<Tip>
  A compactação funciona a seu favor quando o contexto que você descarta é conteúdo que não precisa mais. Para escolher quando sua sobrecarga acontece, execute `/compact` em uma pausa natural em seu trabalho, como entre tarefas, em vez de esperar que a compactação automática seja acionada no meio da tarefa. Se você seguiu um caminho que deseja abandonar completamente, use [`/rewind`](#rewinding-the-conversation) para um turno anterior. Rewind trunca de volta para um prefixo que já está em cache, em vez de construir um novo como a compactação faz.
</Tip>

<h3 id="upgrading-claude-code">
  Atualizar Claude Code
</h3>

Uma nova versão de Claude Code normalmente atualiza o prompt do sistema ou definições de ferramentas, então a primeira solicitação após uma atualização reconstrói o cache do início. [Auto-update](/docs/pt/setup#auto-updates) baixa novas versões em segundo plano, mas as aplica no próximo lançamento, nunca no meio da sessão, então você vê isso como um primeiro turno sem cache após reiniciar em vez de uma surpresa durante uma sessão. Defina `DISABLE_AUTOUPDATER=1` para controlar quando as atualizações se aplicam.

<Note>
  [Retomar uma sessão](/docs/pt/sessions#resume-a-session) após uma atualização reprocessa todo o histórico de conversa sem acertos de cache, já que o histórico agora fica atrás de um prompt do sistema diferente. O custo escala com o comprimento da conversa retomada, então o primeiro turno de volta para uma sessão longa pode ser a solicitação mais cara que você envia.
</Note>

<h2 id="actions-that-keep-the-cache">
  Ações que mantêm o cache
</h2>

Essas ações ou anexam ao final da conversa ou não tocam a solicitação. Algumas delas, como editar CLAUDE.md ou alterar o estilo de saída, também são o motivo pelo qual uma mudança de configuração aguarda uma reinicialização para ser aplicada.

* [Editar arquivos em seu repositório](#editing-files-in-your-repository)
* [Editar CLAUDE.md no meio da sessão](#editing-claude-md-mid-session)
* [Alterar estilo de saída](#changing-output-style)
* [Alterar modo de permissão](#changing-permission-mode)
* [Invocar skills e comandos](#invoking-skills-and-commands)
* [Executar `/recap`](#running-%2Frecap)
* [Rewind da conversa](#rewinding-the-conversation)
* [Spawning de um subagent](#subagents-and-the-cache)

<h3 id="editing-files-in-your-repository">
  Editar arquivos em seu repositório
</h3>

O conteúdo do arquivo entra em contexto apenas quando Claude o lê, e as leituras se anexam à conversa. Editar um arquivo que Claude leu anteriormente não muda retroativamente a leitura anterior no histórico. Em vez disso, Claude Code anexa um `<system-reminder>` observando que o arquivo mudou, e Claude o relê se necessário.

<h3 id="editing-claude-md-mid-session">
  Editar CLAUDE.md no meio da sessão
</h3>

Seus arquivos CLAUDE.md de raiz de projeto e nível de usuário são lidos uma vez no início da sessão e mantidos na memória. Editá-los no meio da sessão não invalida o cache, mas a edição também não se aplica. Claude continua trabalhando com a versão que foi carregada no início da sessão. O novo conteúdo carrega no próximo `/clear`, `/compact` ou reinicialização.

[Arquivos CLAUDE.md aninhados em subdiretórios](/docs/pt/memory) e [regras com frontmatter `paths:`](/docs/pt/memory#path-specific-rules) carregam depois, quando Claude primeiro lê um arquivo correspondente. Editar um antes de carregar tem efeito. Depois de carregar, o conteúdo faz parte do histórico de conversa, então uma edição no meio da sessão não muda retroativamente.

<h3 id="changing-output-style">
  Alterar estilo de saída
</h3>

[Estilo de saída](/docs/pt/output-styles) faz parte do prompt do sistema, que Claude Code lê uma vez no início da sessão. Alterá-lo via `/config` ou a configuração `outputStyle` no meio da sessão não invalida o cache, mas a mudança também não se aplica. Claude continua usando o estilo que foi carregado no início da sessão. O novo estilo carrega no próximo `/clear` ou reinicialização.

<h3 id="changing-permission-mode">
  Alterar modo de permissão
</h3>

Alternar entre [modos de permissão](/docs/pt/permission-modes), como de padrão para aceitar edições, não muda o prompt do sistema ou definições de ferramentas, então mudanças de modo são seguras para cache. A exceção é o modo de plano com a configuração de modelo [`opusplan`](/docs/pt/model-config#opusplan-model-setting), que alterna o modelo entre Opus e Sonnet conforme você entra ou sai do modo de plano. Isso torna a alternância de modo uma [mudança de modelo](#switching-models).

<h3 id="invoking-skills-and-commands">
  Invocar skills e comandos
</h3>

[Skills](/docs/pt/skills) e [comandos](/docs/pt/commands) injetam suas instruções como mensagens de usuário no ponto de invocação. Nada anterior na conversa muda.

<h3 id="running-/recap">
  Executar `/recap`
</h3>

[`/recap`](/docs/pt/interactive-mode#session-recap) gera um resumo para exibição em seu terminal. Ao contrário de `/compact`, ele anexa o resumo como saída de comando em vez de substituir seu histórico de mensagens, então o prefixo em cache permanece intacto.

<h3 id="rewinding-the-conversation">
  Rewind da conversa
</h3>

[`/rewind`](/docs/pt/checkpointing) trunca sua conversa de volta para um turno anterior. O histórico restante é o mesmo conteúdo do qual o cache foi construído naquele ponto, e as camadas de prompt do sistema e contexto do projeto não mudam, então a próxima solicitação acerta a entrada de cache anterior. Cada turno desde então leu através desse prefixo, que manteve a entrada aquecida mesmo se o turno original foi há mais tempo do que o TTL.

Restaurar checkpoints de arquivo junto com a conversa não tem efeito separado no cache. O conteúdo do arquivo entra em contexto apenas quando Claude o lê, o mesmo que [editar arquivos em seu repositório](#editing-files-in-your-repository).

<h2 id="cache-lifetime">
  Cache lifetime
</h2>

Prefixos em cache expiram após um período de inatividade. Cada solicitação que acerta o cache redefine o temporizador, então o cache permanece aquecido enquanto você continua trabalhando. Após um intervalo longo o suficiente, a próxima solicitação recomputa a entrada completa e reestabelece o cache, o que é por que o primeiro turno de volta após se afastar pode ser notavelmente mais lento.

O tempo de vida (TTL) controla quanto tempo um intervalo o cache sobrevive. A API oferece dois: um TTL de cinco minutos e um [TTL de uma hora](https://platform.claude.com/docs/pt/build-with-claude/prompt-caching#1-hour-cache-duration) que mantém o cache aquecido através de pausas mais longas, mas [cobra gravações de cache a uma taxa mais alta](https://platform.claude.com/docs/pt/build-with-claude/prompt-caching#pricing). Claude Code escolhe o TTL para você com base em como você se autentica, e você pode substituí-lo com variáveis de ambiente.

<h3 id="on-a-claude-subscription">
  Em uma assinatura Claude
</h3>

Em uma assinatura Claude, Claude Code solicita o TTL de uma hora automaticamente. O uso é incluído em seu plano em vez de ser cobrado por token, então o TTL mais longo não custa nada extra e apenas afeta quanto tempo seu cache permanece aquecido.

Se você ultrapassou o limite de uso do seu plano e Claude Code está usando [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), você é cobrado por esse uso, então Claude Code automaticamente reduz o TTL para cinco minutos.

<h3 id="on-an-api-key-or-third-party-provider">
  Em uma chave de API ou provedor de terceiros
</h3>

Em uma chave de API, Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry ou Claude Platform on AWS, você paga as taxas por token, então o TTL permanece nos cinco minutos mais baratos por padrão. Para optar pelo [TTL de uma hora](https://platform.claude.com/docs/pt/build-with-claude/prompt-caching#1-hour-cache-duration), defina `ENABLE_PROMPT_CACHING_1H=1`.

No Amazon Bedrock, suporte a prompt caching, comprimento mínimo de prefixo armazenável em cache e disponibilidade de TTL de uma hora variam por modelo. Se as contagens de tokens de cache permanecerem em zero, verifique [modelos, regiões e limites suportados](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) na documentação do Amazon Bedrock.

<h3 id="override-the-ttl">
  Substituir o TTL
</h3>

Defina `FORCE_PROMPT_CACHING_5M=1` para forçar o TTL de cinco minutos independentemente da autenticação. Isso é útil quando você está depurando o comportamento do cache, comparando os dois TTLs ou substituindo um `ENABLE_PROMPT_CACHING_1H` definido em [configurações gerenciadas](/docs/pt/settings#settings-files).

<h2 id="cache-scope">
  Escopo do cache
</h2>

Em Claude Code, o cache é efetivamente limitado a uma máquina e diretório. O prompt do sistema incorpora o diretório de trabalho, plataforma, shell, versão do SO e caminhos de memória automática, então duas sessões em diretórios diferentes constroem prefixos diferentes e perdem o cache uma da outra. Isso inclui worktrees do mesmo repositório, já que cada worktree tem seu próprio diretório de trabalho.

Sessões que você executa em paralelo no mesmo diretório constroem prefixos correspondentes e leem o cache uma da outra. Sessões sequenciais compartilham o prefixo apenas quando o snapshot de status git na inicialização corresponde, já que o prompt do sistema também captura branch e commits recentes.

O cache de API subjacente é mais amplo. Os caches são isolados entre organizações e, em alguns provedores, [entre workspaces dentro de uma organização](https://platform.claude.com/docs/pt/build-with-claude/prompt-caching#cache-storage-and-sharing). Dentro desses limites, quaisquer duas solicitações com o mesmo modelo e prefixo leem o mesmo cache. Para chamadores do Agent SDK executando frotas de processos automatizados, veja [melhorar prompt caching entre usuários e máquinas](/docs/pt/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) para suprimir as seções por máquina do prompt do sistema e compartilhar o cache entre máquinas.

<h2 id="check-cache-performance">
  Verificar desempenho do cache
</h2>

O desempenho do cache aparece como duas contagens de tokens que a API relata em cada resposta. A forma mais direta de observá-los ao vivo é um [script de statusline](/docs/pt/statusline) que lê o objeto `current_usage`:

| Campo                         | Significado                                                                                     |
| ----------------------------- | ----------------------------------------------------------------------------------------------- |
| `cache_creation_input_tokens` | Tokens escritos no cache neste turno, cobrados à taxa de gravação de cache                      |
| `cache_read_input_tokens`     | Tokens servidos do cache neste turno, cobrados em aproximadamente 10% da taxa de entrada padrão |

Uma alta proporção de leitura para criação significa que o caching está funcionando bem. Se a criação permanecer alta turno após turno, algo está mudando em seu prefixo. A seção [ações que invalidam o cache](#actions-that-invalidate-the-cache) lista as causas usuais.

Para visibilidade em toda uma organização, o exportador OpenTelemetry relata tokens de leitura e criação de cache por usuário e sessão. Veja [Monitorar uso](/docs/pt/monitoring-usage) para a referência de métrica e atributo de evento.

<h2 id="subagents-and-the-cache">
  Subagents e o cache
</h2>

Um [subagent](/docs/pt/sub-agents) inicia sua própria conversa com seu próprio prompt do sistema e conjunto de ferramentas, separado do pai. Ele constrói seu próprio cache, começando sem acertos de cache em sua primeira chamada e aquecendo através de seus próprios turnos. Subagents usam o TTL de cinco minutos mesmo em uma assinatura, já que o TTL automático de uma hora se aplica à conversa principal.

O cache do pai não é afetado. Do lado do pai, a chamada e resultado do subagent se anexam à conversa, deixando o prefixo do pai intacto.

Um [fork](/docs/pt/sub-agents#fork-the-current-conversation), por contraste, herda o prompt do sistema, ferramentas e histórico de conversa do pai exatamente, então sua primeira solicitação lê o cache do pai. A chamada de resumo de compactação descrita em [Compactar a conversa](#compacting-the-conversation) usa a mesma abordagem de compartilhamento de prefixo.

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

Para definir a política de caching em toda uma organização, coloque qualquer uma dessas ou as [variáveis de TTL](#cache-lifetime) no bloco `env` de [configurações gerenciadas](/docs/pt/settings#settings-files). Para uso normal, deixe o caching habilitado.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Lições de construir Claude Code: Prompt caching é tudo](https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything): a lógica de design para modo de plano, carregamento de ferramentas adiado e compactação
* [Explorar a janela de contexto](/docs/pt/context-window): o que carrega em contexto e quando
* [Reduzir uso de tokens](/docs/pt/costs#reduce-token-usage): estratégias além de caching para gerenciar tamanho de contexto
* [Rastrear e reduzir custos](/docs/pt/agent-sdk/cost-tracking): rastreamento de tokens de cache e configuração de TTL para chamadores do Agent SDK
* [Prompt caching](https://platform.claude.com/docs/pt/build-with-claude/prompt-caching): o mecanismo de API subjacente, breakpoints e preços
