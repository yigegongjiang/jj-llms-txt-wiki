> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins de inteligência de código

> Instale um plugin de servidor de linguagem para que Claude veja erros de tipo após edições e navegue pelo código por símbolo, e responda ao diálogo de recomendação do plugin LSP.

Um plugin de inteligência de código oferece a Claude os diagnósticos ao vivo e a funcionalidade de ir para a definição que seu editor possui, para que Claude detecte erros de tipo e importações ausentes que suas próprias edições introduzem antes de você executar sua compilação, e encontre definições e referências por símbolo em vez de por busca de texto.

Cada plugin conecta Claude Code a um servidor de linguagem para uma linguagem através do Language Server Protocol (LSP). Você instala o plugin do marketplace oficial da Anthropic e o binário do servidor de linguagem em sua máquina.

<Note>
  Os plugins de inteligência de código funcionam em sessões de terminal. Em [sessões na nuvem](/docs/pt/claude-code-on-the-web), Claude Code não inicia servidores de linguagem de plugin, portanto Claude não obtém diagnósticos ou navegação de código lá. Para escrever seu próprio plugin de servidor de linguagem, ou para conectar um servidor de linguagem que não possui plugin, consulte [Servidores LSP em componentes de plugin](/docs/pt/plugins/components#lsp-servers).
</Note>

Para começar, encontre sua linguagem na tabela em [Instalar um plugin de inteligência de código](#install-a-code-intelligence-plugin). Os plugins nessa tabela vêm do [marketplace oficial de plugins](/docs/pt/plugins/anthropic-marketplaces) da Anthropic.

Se você já viu um diálogo de **recomendação de plugin LSP**, consulte [Aceitar ou descartar o diálogo de recomendação](#accept-or-dismiss-the-recommendation-dialog) para saber o que cada escolha faz.

<h2 id="install-a-code-intelligence-plugin">
  Instalar um plugin de inteligência de código
</h2>

Um plugin de inteligência de código diz a Claude Code qual comando inicia o servidor de linguagem e quais extensões de arquivo ele manipula. Ele não inclui o servidor de linguagem. Instale primeiro o binário do servidor de linguagem, depois o plugin, e então confirme que o servidor inicia.

<Steps>
  <Step title="Instalar o binário do servidor de linguagem">
    Encontre sua linguagem na tabela abaixo e instale o binário em sua linha. Se sua linguagem não estiver listada, consulte [Adicionar uma linguagem sem um plugin oficial](#add-a-language-without-an-official-plugin).

    | Linguagem               | Plugin                                                                                                           | Binário                      |
    | :---------------------- | :--------------------------------------------------------------------------------------------------------------- | :--------------------------- |
    | C/C++                   | [`clangd-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/clangd-lsp)               | `clangd`                     |
    | C#                      | [`csharp-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/csharp-lsp)               | `csharp-ls`                  |
    | Go                      | [`gopls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/gopls-lsp)                 | `gopls`                      |
    | Java                    | [`jdtls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/jdtls-lsp)                 | `jdtls`                      |
    | Kotlin                  | [`kotlin-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/kotlin-lsp)               | `kotlin-lsp`                 |
    | Liquid                  | [`liquid-lsp`](https://github.com/Shopify/liquid-skills/tree/main/plugins/liquid-lsp)                            | `shopify`, do Shopify CLI    |
    | Lua                     | [`lua-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/lua-lsp)                     | `lua-language-server`        |
    | PHP                     | [`php-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/php-lsp)                     | `intelephense`               |
    | Python                  | [`pyright-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/pyright-lsp)             | `pyright-langserver`         |
    | Ruby                    | [`ruby-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/ruby-lsp)                   | `ruby-lsp`                   |
    | Rust                    | [`rust-analyzer-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/rust-analyzer-lsp) | `rust-analyzer`              |
    | Swift                   | [`swift-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/swift-lsp)                 | `sourcekit-lsp`              |
    | TypeScript e JavaScript | [`typescript-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/typescript-lsp)       | `typescript-language-server` |

    A Anthropic mantém todos os plugins na tabela exceto `liquid-lsp`, que a Shopify mantém e o marketplace oficial lista.

    Para encontrar o comando que instala o binário, siga o link do plugin na tabela para seu README. Para TypeScript, esse comando é `npm install -g typescript-language-server typescript`.

    Depois de instalar o binário, confirme que está no `PATH` do shell a partir do qual você inicia `claude`, por exemplo com `which typescript-language-server`, ou `Get-Command typescript-language-server` no PowerShell.
  </Step>

  <Step title="Instalar o plugin">
    Para instalar o plugin listado para sua linguagem na tabela da etapa 1, execute `/plugin install` em uma sessão Claude Code, substituindo `typescript-lsp` pelo nome desse plugin:

    ```
    /plugin install typescript-lsp@claude-plugins-official
    ```

    Uma mensagem de confirmação diz se o plugin está ativo agora ou precisa de `/reload-plugins`. Se a instalação falhar com `Marketplace "claude-plugins-official" not found`, consulte a [entrada de solução de problemas para esse erro](/docs/pt/plugins/troubleshooting#marketplace-claude-plugins-official-not-found). Para controlar onde o plugin é instalado, ou para executar a instalação a partir de seu shell em vez de dentro de Claude Code, consulte [Instalar plugins](/docs/pt/plugins/install).
  </Step>

  <Step title="Confirmar que o servidor inicia">
    O servidor de linguagem inicia na primeira vez que Claude edita um arquivo com uma das extensões do plugin. Para vê-lo funcionar, peça a Claude para introduzir um erro de tipo em um arquivo dessa linguagem e depois corrigi-lo. Em seguida, verifique a conversa para uma linha de diagnósticos:

    * **Uma linha de diagnósticos aparece**: `Found N new diagnostic issues in M files (ctrl+o to expand)` sob a edição que introduziu o erro significa que o servidor iniciou.
    * **Nenhuma linha de diagnósticos aparece**: execute `/plugin` e abra a aba **Errors**. Uma linha lendo `Executable not found in $PATH: "<binary>"` nomeia o binário a instalar. Se a aba não tiver tal linha, consulte [Solucionar problemas de inteligência de código](#troubleshoot-code-intelligence).

    Depois de instalar um binário ausente, Claude Code tenta novamente na próxima vez que Claude edita um arquivo correspondente. Se você instalou o binário em um diretório que não está no `PATH` do shell a partir do qual iniciou `claude`, inicie uma nova sessão a partir de um shell onde ele está.
  </Step>
</Steps>

<h2 id="see-what-claude-gains">
  Veja o que Claude ganha
</h2>

Com um servidor de linguagem em execução, Claude ganha diagnósticos e navegação de código:

* **Diagnósticos após edições**: cada vez que Claude edita ou escreve um arquivo que o servidor manipula, Claude obtém os erros e avisos que o servidor relata. Ele vê um erro de tipo, importação ausente ou erro de sintaxe que introduziu sem executar um compilador.
* **Navegação de código**: Claude obtém uma ferramenta `LSP` que procura símbolos através do servidor em vez de procurá-los por texto. A ferramenta é somente leitura. Para saber o que Claude pode procurar com a ferramenta e como as permissões se aplicam a ela, consulte [Comportamento da ferramenta LSP](/docs/pt/tools-reference#lsp-tool-behavior).

<h3 id="read-the-diagnostics-yourself">
  Leia os diagnósticos você mesmo
</h3>

Depois que Claude edita um arquivo que o servidor manipula, a conversa mostra apenas o resumo `Found N new diagnostic issues`. Para ler os problemas em si, pressione **Ctrl+O**.

<h2 id="accept-or-dismiss-the-recommendation-dialog">
  Aceitar ou descartar o diálogo de recomendação
</h2>

Se um binário de servidor de linguagem já está em seu `PATH` e o plugin que o usa não está instalado, Claude Code oferece instalar o plugin para você em um diálogo intitulado **LSP plugin recommendation**.

<h3 id="when-the-recommendation-dialog-appears">
  Quando o diálogo de recomendação aparece
</h3>

O diálogo **LSP plugin recommendation** pode aparecer depois que Claude edita um arquivo. Essas condições decidem se ele aparece e qual plugin oferece:

* **Um plugin corresponde ao arquivo**: um dos marketplaces que você adicionou, ou o marketplace oficial que Claude Code registrou para você, lista um plugin de inteligência de código para a extensão desse arquivo, e o binário do plugin está instalado.
* **Oficial primeiro**: quando mais de um marketplace oferece um plugin para a extensão, o diálogo oferece o plugin do marketplace oficial.
* **Uma vez por sessão**: o diálogo aparece no máximo uma vez em uma sessão, para o primeiro arquivo correspondente que Claude edita.
* **Não para sessões na nuvem**: o diálogo nunca aparece quando seu terminal está anexado a uma sessão na nuvem, como uma que você iniciou com [`claude --cloud`](/docs/pt/claude-code-on-the-web#from-terminal-to-cloud).

<h3 id="respond-to-the-recommendation-dialog">
  Responder ao diálogo de recomendação
</h3>

O diálogo **LSP plugin recommendation** nomeia o plugin e oferece essas escolhas:

* **Yes, install**: Claude Code instala o plugin para sua conta de usuário e imprime `<plugin> installed · restart to apply`. Inicie uma nova sessão para carregar o servidor.
* **No, not now**: o diálogo fecha, e uma sessão posterior pode oferecer o plugin novamente. Pressionar **Esc** faz o mesmo.
* **Never for this plugin**: o diálogo para de aparecer para esse plugin e ainda aparece para outros.
* **Disable all LSP recommendations**: o diálogo para de aparecer para cada linguagem.

Se você não escolher uma opção, Claude Code o fecha após 30 segundos e conta isso como ignorado. A contagem é mantida entre sessões. Depois de cinco diálogos ignorados, Claude Code para de recomendar plugins, o mesmo que se você tivesse escolhido **Disable all LSP recommendations**.

<h3 id="turn-recommendations-back-on">
  Ativar recomendações novamente
</h3>

O diálogo **LSP plugin recommendation** para de aparecer depois que você escolhe **Disable all LSP recommendations** ou o ignora cinco vezes.

* **Desabilitado ou ignorado cinco vezes**: para ativá-lo novamente em qualquer caso, remova as chaves `lspRecommendationDisabled` e `lspRecommendationIgnoredCount` de `~/.claude.json`, o arquivo de configuração próprio de Claude Code.
* **Nunca para este plugin**: se você escolheu **Never for this plugin** e quer que esse plugin seja oferecido novamente, remova seu id `name@marketplace` da lista `lspRecommendationNeverPlugins` no mesmo arquivo.

<h2 id="troubleshoot-code-intelligence">
  Solucionar problemas de inteligência de código
</h2>

A página de solução de problemas de plugins cobre os sintomas específicos dos plugins de inteligência de código em [Language server doesn't start, uses too much memory, or reports wrong diagnostics](/docs/pt/plugins/troubleshooting#language-server-doesnt-start):

* **O servidor de linguagem não inicia**: você vê `Executable not found in $PATH` na aba **Errors** de `/plugin`, ou Claude nunca relata diagnósticos para a linguagem.
* **Alto uso de memória**: o uso de memória aumenta enquanto o servidor indexa o projeto.
* **Diagnósticos falsos positivos em um monorepo**: diagnósticos relatam importações como não resolvidas quando não estão.

<h2 id="add-a-language-without-an-official-plugin">
  Adicionar uma linguagem sem um plugin oficial
</h2>

Se sua linguagem não estiver na [tabela de plugins oficiais](#install-a-code-intelligence-plugin), você ainda pode conectar um servidor de linguagem.

1. Escreva um plugin com um arquivo `.lsp.json` que nomeie o comando do servidor e as extensões de arquivo que ele manipula.
2. Em seguida, carregue o plugin com [`--plugin-dir`](/docs/pt/plugins/cli-reference#flags-that-load-a-plugin-for-one-session) ou publique-o em um marketplace.

Para os campos do arquivo e um exemplo trabalhado, consulte [LSP servers in plugin components](/docs/pt/plugins/components#lsp-servers).

<h2 id="next-steps">
  Próximas etapas
</h2>

* [LSP servers in plugin components](/docs/pt/plugins/components#lsp-servers): escreva o `.lsp.json` para um servidor de linguagem que não possui plugin oficial
* [Install and manage plugins](/docs/pt/plugins/install): escopos, atualizações e desinstalação
* [Troubleshoot plugins](/docs/pt/plugins/troubleshooting): erros de carregamento além dos relacionados ao servidor de linguagem nesta página
* [Find plugins in the official marketplace](/docs/pt/plugins/anthropic-marketplaces#find-plugins-in-the-official-marketplace): onde procurar o resto do marketplace oficial
