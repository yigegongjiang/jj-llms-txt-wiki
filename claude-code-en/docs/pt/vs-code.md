> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Use Claude Code in VS Code

> Instale e configure a extensão Claude Code para VS Code. Obtenha assistência de codificação com IA com diffs inline, @-mentions, revisão de planos e atalhos de teclado.

<img src="https://mintcdn.com/claude-code/-YhHHmtSxwr7W8gy/images/vs-code-extension-interface.jpg?fit=max&auto=format&n=-YhHHmtSxwr7W8gy&q=85&s=300652d5678c63905e6b0ea9e50835f8" alt="Editor VS Code com o painel de extensão Claude Code aberto no lado direito, mostrando uma conversa com Claude" width="2500" height="1155" data-path="images/vs-code-extension-interface.jpg" />

A extensão VS Code fornece uma interface gráfica nativa para Claude Code, integrada diretamente ao seu IDE. Esta é a forma recomendada de usar Claude Code no VS Code.

Com a extensão, você pode revisar e editar os planos do Claude antes de aceitá-los, aceitar automaticamente edições conforme são feitas, @-mencionar arquivos com intervalos de linhas específicas da sua seleção, acessar o histórico de conversas e abrir múltiplas conversas em abas separadas ou janelas.

<h2 id="prerequisites">
  Pré-requisitos
</h2>

Antes de instalar, certifique-se de que você tem:

* VS Code 1.98.0 ou superior
* Uma conta Anthropic: qualquer assinatura paga do Claude (Pro, Max, Team ou Enterprise) ou uma conta Claude Console funciona, e nenhuma chave de API é necessária. Você fará [login](/docs/pt/authentication#log-in-to-claude-code) com essa conta quando abrir a extensão pela primeira vez. Se você acessar Claude através de um provedor de terceiros como Amazon Bedrock ou Google Cloud's Agent Platform, consulte [Use third-party providers](#use-third-party-providers) para instruções de configuração.

<Tip>
  A extensão inclui sua própria cópia da CLI (interface de linha de comando) para o painel de chat. Para executar `claude` no terminal integrado do VS Code, você também precisa da [instalação da CLI autônoma](/docs/pt/setup). Consulte [VS Code extension vs. Claude Code CLI](#vs-code-extension-vs-claude-code-cli) para detalhes.
</Tip>

<h2 id="install-the-extension">
  Instale a extensão
</h2>

Clique no link do seu IDE para instalar diretamente:

* [Instalar para VS Code](vscode:extension/anthropic.claude-code)
* [Instalar para Cursor](cursor:extension/anthropic.claude-code)

Ou no VS Code, pressione `Cmd+Shift+X` (Mac) ou `Ctrl+Shift+X` (Windows/Linux) para abrir a visualização de Extensões, procure por "Claude Code" e clique em **Instalar**.

A extensão também é instalada em outros forks do VS Code como Devin Desktop ou Kiro. Procure por "Claude Code" na visualização de Extensões do editor, ou instale a partir do [registro Open VSX](https://open-vsx.org/extension/Anthropic/claude-code). Se o seu editor não conseguir instalar a extensão, [instale a CLI](/docs/pt/quickstart) e execute `claude` no seu terminal integrado. A CLI funciona em qualquer terminal.

<Note>Se a extensão não aparecer após a instalação, reinicie o VS Code ou execute "Developer: Reload Window" na Paleta de Comandos.</Note>

<h2 id="get-started">
  Comece
</h2>

Depois de instalada, você pode começar a usar Claude Code através da interface VS Code:

<Steps>
  <Step title="Abra o painel Claude Code">
    Em todo o VS Code, o ícone Spark indica Claude Code: <img src="https://mintcdn.com/claude-code/c5r9_6tjPMzFdDDT/images/vs-code-spark-icon.svg?fit=max&auto=format&n=c5r9_6tjPMzFdDDT&q=85&s=3ca45e00deadec8c8f4b4f807da94505" alt="Spark icon" style={{display: "inline", height: "0.85em", verticalAlign: "middle"}} width="16" height="16" data-path="images/vs-code-spark-icon.svg" />

    A forma mais rápida de abrir Claude é clicar no ícone Spark na **Editor Toolbar** (canto superior direito do editor). O ícone só aparece quando você tem um arquivo aberto.

    <img src="https://mintcdn.com/claude-code/mfM-EyoZGnQv8JTc/images/vs-code-editor-icon.png?fit=max&auto=format&n=mfM-EyoZGnQv8JTc&q=85&s=eb4540325d94664c51776dbbfec4cf02" alt="Editor VS Code mostrando o ícone Spark na Editor Toolbar" width="2796" height="734" data-path="images/vs-code-editor-icon.png" />

    Outras formas de abrir Claude Code:

    * **Activity Bar**: clique no ícone Spark na barra lateral esquerda para abrir a lista de sessões. Clique em qualquer sessão para abri-la como uma aba de editor completa, ou inicie uma nova. Este ícone está sempre visível na Activity Bar.
    * **Command Palette**: `Cmd+Shift+P` (Mac) ou `Ctrl+Shift+P` (Windows/Linux), digite "Claude Code" e selecione uma opção como "Open in New Tab"
    * **Status Bar**: clique em **✱ Claude Code** no canto inferior direito da janela. Isso funciona mesmo quando nenhum arquivo está aberto.

    Você pode arrastar o painel Claude para reposicioná-lo em qualquer lugar do VS Code. Consulte [Personalize seu fluxo de trabalho](#customize-your-workflow) para detalhes.
  </Step>

  <Step title="Faça login">
    A primeira vez que você abre o painel, uma tela de login aparece. Clique em **Sign in** e complete a autorização no seu navegador.

    Se você vir **Not logged in · Please run /login** mais tarde, a extensão reabre a tela de login automaticamente. Se não aparecer, recarregue a janela na Paleta de Comandos com **Developer: Reload Window**.

    Se você tem `ANTHROPIC_API_KEY` definida no seu shell mas ainda vê o prompt de login, VS Code pode não ter herdado seu ambiente de shell. Inicie VS Code de um terminal com `code .` para que ele herde suas variáveis de ambiente, ou faça login com sua conta Claude em vez disso.

    Depois que você faz login, uma lista de verificação **Learn Claude Code** aparece. Trabalhe em cada item clicando em **Show me**, ou descarte-a com o X. Para reabri-la mais tarde, desmarque **Hide Onboarding** nas configurações do VS Code em Extensions → Claude Code.
  </Step>

  <Step title="Envie um prompt">
    Peça ao Claude para ajudar com seu código ou arquivos, seja explicando como algo funciona, depurando um problema ou fazendo alterações.

    <Tip>Claude vê automaticamente seu texto selecionado. Pressione `Option+K` (Mac) / `Alt+K` (Windows/Linux) para também inserir uma referência @-mention (como `@file.ts#5-10`) em seu prompt.</Tip>

    Aqui está um exemplo de pergunta sobre uma linha específica em um arquivo:

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-send-prompt.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=ede3ed8d8d5f940e01c5de636d009cfd" alt="Editor VS Code com as linhas 2-3 selecionadas em um arquivo Python, e o painel Claude Code mostrando uma pergunta sobre essas linhas com uma referência @-mention" width="3288" height="1876" data-path="images/vs-code-send-prompt.png" />
  </Step>

  <Step title="Revise as alterações">
    Quando Claude quer editar um arquivo, ele mostra uma comparação lado a lado do original e das alterações propostas, depois pede permissão. Você pode aceitar, rejeitar ou dizer ao Claude o que fazer em vez disso. Se você editar o conteúdo proposto diretamente na visualização de diff antes de aceitar, Claude é informado de que você o modificou para que não assuma que o arquivo corresponde à sua proposta original.

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-edits.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=e005f9b41c541c5c7c59c082f7c4841c" alt="VS Code mostrando um diff das alterações propostas por Claude com um prompt de permissão perguntando se deve fazer a edição" width="3292" height="1876" data-path="images/vs-code-edits.png" />
  </Step>
</Steps>

Para mais ideias sobre o que você pode fazer com Claude Code, consulte [Fluxos de trabalho comuns](/docs/pt/common-workflows).

<Tip>
  Execute "Claude Code: Open Walkthrough" na Paleta de Comandos para um tour guiado dos conceitos básicos.
</Tip>

<h2 id="use-the-prompt-box">
  Use a caixa de prompt
</h2>

A caixa de prompt suporta vários recursos:

* **Permission modes**: clique no indicador de modo na parte inferior da caixa de prompt para alternar modos, ou defina o padrão nas configurações do VS Code em `claudeCode.initialPermissionMode`. Consulte [permission modes](/docs/pt/permission-modes#switch-permission-modes) para cada modo que o indicador oferece.
  * **Manual**: Claude pede permissão antes de edições de arquivo e a maioria dos comandos shell.
  * **Plan**: Claude descreve o que fará e aguarda aprovação antes de fazer alterações. VS Code abre automaticamente o plano como um documento Markdown completo onde você pode adicionar comentários inline para fornecer feedback antes de Claude começar.
  * **Edit automatically**: Claude faz edições sem perguntar.
* **Command menu**: clique em `/` ou digite `/` para abrir o menu de comandos. As opções incluem anexar arquivos, alternar modelos, alternar pensamento estendido, visualizar uso de plano (`/usage`) e iniciar uma sessão de [Remote Control](/docs/pt/remote-control) (`/remote-control`). A seção Customize fornece acesso a MCP servers, hooks, memory, permissions e plugins. Itens com um ícone de terminal abrem no terminal integrado.
  * A seção Settings inclui **Enable Remote Control for all sessions**, que define [`remoteControlAtStartup`](/docs/pt/settings#available-settings) para que [cada nova sessão interativa se conecte ao Remote Control automaticamente](/docs/pt/remote-control#enable-remote-control-for-all-sessions). Requer Claude Code v2.1.203 ou posterior.
* **Context indicator**: a caixa de prompt mostra quanto da context window do Claude você está usando. Claude compacta automaticamente quando necessário, ou você pode executar `/compact` manualmente.
* **Extended thinking**: permite que Claude gaste mais tempo raciocinando sobre problemas complexos. Alterne-o via menu de comandos (`/`). O raciocínio do Claude aparece na conversa como blocos recolhidos: clique em um bloco para lê-lo, ou pressione `Ctrl+O` para expandir ou recolher cada bloco de pensamento na sessão. Consulte [Extended thinking](/docs/pt/model-config#extended-thinking) para detalhes.
* **Multi-line input**: pressione `Shift+Enter` para adicionar uma nova linha sem enviar. Isso também funciona na entrada de texto livre "Other" de diálogos de pergunta.

<h3 id="reference-files-and-folders">
  Reference files and folders
</h3>

Use @-mentions para dar ao Claude contexto sobre arquivos ou pastas específicas. Quando você digita `@` seguido de um nome de arquivo ou pasta, Claude lê esse conteúdo e pode responder perguntas sobre ele ou fazer alterações nele. Claude Code suporta fuzzy matching, então você pode digitar nomes parciais para encontrar o que precisa:

```text theme={null}
> Explain the logic in @auth (fuzzy matches auth.js, AuthService.ts, etc.)
> What's in @src/components/ (include a trailing slash for folders)
```

Para PDFs grandes, você pode pedir ao Claude para ler páginas específicas em vez do arquivo inteiro: uma única página, um intervalo como páginas 1-10, ou um intervalo aberto como página 3 em diante.

Quando você seleciona texto no editor, Claude pode ver seu código destacado automaticamente. O rodapé da caixa de prompt mostra quantas linhas estão selecionadas. Pressione `Option+K` (Mac) / `Alt+K` (Windows/Linux) para inserir um @-mention com o caminho do arquivo e números de linha (por exemplo, `@app.ts#5-10`). Clique no indicador de seleção para alternar se Claude pode ver seu texto destacado - o ícone de barra de olho significa que a seleção está oculta do Claude.

Você também pode manter `Shift` pressionado enquanto arrasta arquivos para a caixa de prompt para adicioná-los como anexos. Clique no X em qualquer anexo para removê-lo do contexto.

<h3 id="resume-past-conversations">
  Resume past conversations
</h3>

Clique no botão **Session history** na parte superior do painel Claude Code para acessar seu histórico de conversas. Você pode pesquisar por palavra-chave ou navegar por tempo (Today, Yesterday, Last 7 days, etc.). Clique em qualquer conversa para retomá-la com o histórico completo de mensagens. Novas sessões recebem títulos gerados por IA com base em sua primeira mensagem. Passe o mouse sobre uma sessão para revelar ações de renomear e remover: renomeie para dar um título descritivo, ou remova para deletá-la da lista. Para mais sobre retomar sessões, consulte [Manage sessions](/docs/pt/sessions).

<h3 id="resume-cloud-sessions-from-claude-ai">
  Resume cloud sessions from Claude.ai
</h3>

Se você usar [Claude Code on the web](/docs/pt/claude-code-on-the-web), você pode retomar essas sessões remotas diretamente no VS Code. Isso requer fazer login com **Claude.ai Subscription**, não Anthropic Console.

<Steps>
  <Step title="Open session history">
    Clique no botão **Session history** na parte superior do painel Claude Code.
  </Step>

  <Step title="Select the Remote tab">
    O diálogo mostra duas abas: Local e Remote. Clique em **Remote** para ver sessões do claude.ai.
  </Step>

  <Step title="Select a session to resume">
    Navegue ou pesquise suas sessões remotas. Clique em qualquer sessão para baixá-la e continuar a conversa localmente.
  </Step>
</Steps>

<Note>
  Apenas sessões web iniciadas com um repositório GitHub aparecem na aba Remote. Retomar carrega o histórico de conversas localmente; as alterações não são sincronizadas de volta para claude.ai.
</Note>

<h3 id="check-account-and-usage">
  Check account and usage
</h3>

Execute `/usage` no menu de comandos para abrir o diálogo Account & usage. Ele mostra sua conta conectada, plano e barras de uso para a sessão atual e semana com quanto tempo falta até cada limite ser redefinido.

O diálogo também detalha o que está contribuindo para seus limites de plano. Ele sinaliza comportamentos que representam 10% ou mais do uso recente, como falhas de cache, contexto longo e sessões com muitos subagentes ou altamente paralelas, cada uma com uma dica para reduzi-la. Tabelas de atribuição mostram quanto uso veio de cada skill, subagente, plugin e servidor MCP. Requer Claude Code v2.1.174 ou posterior.

Use o alternador Day e Week para alternar entre as últimas 24 horas e os últimos 7 dias. Os números são aproximados e calculados a partir de sessões locais nesta máquina, portanto o uso de outros dispositivos ou claude.ai não está incluído. Para mais sobre rastreamento e redução de uso, consulte [Track your costs](/docs/pt/costs#track-your-costs).

<h2 id="customize-your-workflow">
  Personalize seu fluxo de trabalho
</h2>

Depois que você estiver funcionando, você pode reposicionar o painel Claude, executar múltiplas sessões ou alternar para modo terminal.

<h3 id="choose-where-claude-lives">
  Escolha onde Claude fica
</h3>

Você pode arrastar o painel Claude para reposicioná-lo em qualquer lugar do VS Code. Pegue a aba ou barra de título do painel e arraste para:

* **Secondary sidebar**: o lado direito da janela. Mantém Claude visível enquanto você codifica.
* **Primary sidebar**: a barra lateral esquerda com ícones para Explorer, Search, etc.
* **Editor area**: abre Claude como uma aba ao lado de seus arquivos. Útil para tarefas secundárias.

<Tip>
  Use a barra lateral para sua sessão principal do Claude e abra abas adicionais para tarefas secundárias. Claude lembra sua localização preferida. O ícone da lista de sessões da Activity Bar é separado do painel Claude: a lista de sessões está sempre visível na Activity Bar, enquanto o ícone do painel Claude só aparece lá quando o painel está encaixado na barra lateral esquerda.
</Tip>

<h3 id="run-multiple-conversations">
  Execute múltiplas conversas
</h3>

Use **Open in New Tab** ou **Open in New Window** na Paleta de Comandos para iniciar conversas adicionais. Cada conversa mantém seu próprio histórico e contexto, permitindo que você trabalhe em diferentes tarefas em paralelo.

Ao usar abas, um pequeno ponto colorido no ícone spark indica status: azul significa que uma solicitação de permissão está pendente, laranja significa que Claude terminou enquanto a aba estava oculta.

<h3 id="switch-to-terminal-mode">
  Alterne para modo terminal
</h3>

Por padrão, a extensão abre um painel de chat gráfico. Se você preferir a interface estilo CLI, abra a [Use Terminal setting](vscode://settings/claudeCode.useTerminal) e marque a caixa.

Você também pode abrir as configurações do VS Code (`Cmd+,` no Mac ou `Ctrl+,` no Windows/Linux), ir para Extensions → Claude Code e marcar **Use Terminal**.

<h2 id="manage-plugins">
  Manage plugins
</h2>

A extensão VS Code inclui uma interface gráfica para instalar e gerenciar [plugins](/docs/pt/plugins). Digite `/plugins` na caixa de prompt para abrir a interface **Manage plugins**.

<h3 id="install-plugins">
  Install plugins
</h3>

O diálogo de plugin mostra duas abas: **Plugins** e **Marketplaces**.

Na aba Plugins:

* **Installed plugins** aparecem no topo com switches de alternância para habilitá-los ou desabilitá-los
* **Available plugins** de seus marketplaces configurados aparecem abaixo
* Pesquise para filtrar plugins por nome ou descrição
* Clique em **Install** em qualquer plugin disponível

Quando você instala um plugin, escolha o escopo de instalação:

* **Install for you**: disponível em todos os seus projetos (escopo de usuário)
* **Install for this project**: compartilhado com colaboradores do projeto (escopo de projeto)
* **Install locally**: apenas para você, apenas neste repositório (escopo local)

<h3 id="manage-marketplaces">
  Manage marketplaces
</h3>

Alterne para a aba **Marketplaces** para adicionar ou remover fontes de plugin:

* Digite um repositório GitHub, URL ou caminho local para adicionar um novo marketplace
* Clique no ícone de atualização para atualizar a lista de plugins de um marketplace
* Clique no ícone de lixeira para remover um marketplace

Depois de fazer alterações, um banner o solicita a reiniciar Claude Code para aplicar as atualizações.

<Note>
  O gerenciamento de plugins no VS Code usa os mesmos comandos CLI sob o capô. Plugins e marketplaces que você configura na extensão também estão disponíveis na CLI, e vice-versa.
</Note>

Para mais sobre o sistema de plugins, consulte [Plugins](/docs/pt/plugins) e [Plugin marketplaces](/docs/pt/plugin-marketplaces).

<h2 id="automate-browser-tasks-with-chrome">
  Automate browser tasks with Chrome
</h2>

Conecte Claude ao seu navegador Chrome para testar aplicativos web, depurar com logs de console e automatizar fluxos de trabalho do navegador sem sair do VS Code. Isso requer a [Claude in Chrome extension](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) versão 1.0.36 ou superior.

Digite `@browser` na caixa de prompt seguido do que você quer que Claude faça:

```text theme={null}
@browser go to localhost:3000 and check the console for errors
```

Você também pode abrir o menu de anexos para selecionar ferramentas específicas do navegador como abrir uma nova aba ou ler conteúdo da página.

Claude abre novas abas para tarefas do navegador e compartilha o estado de login do seu navegador, então pode acessar qualquer site em que você já esteja conectado.

Para instruções de configuração, a lista completa de capacidades e solução de problemas, consulte [Use Claude Code with Chrome](/docs/pt/chrome).

<h2 id="vs-code-commands-and-shortcuts">
  Comandos e atalhos de teclado do VS Code
</h2>

Abra a Paleta de Comandos (`Cmd+Shift+P` no Mac ou `Ctrl+Shift+P` no Windows/Linux) e digite "Claude Code" para ver todos os comandos VS Code disponíveis para a extensão Claude Code.

Alguns atalhos dependem de qual painel está "focused" (recebendo entrada de teclado). Quando seu cursor está em um arquivo de código, o editor está focado. Quando seu cursor está na caixa de prompt do Claude, Claude está focado. Use `Cmd+Esc` / `Ctrl+Esc` para alternar entre eles.

<Note>
  Estes são comandos VS Code para controlar a extensão. Nem todos os comandos Claude Code integrados estão disponíveis na extensão. Consulte [VS Code extension vs. Claude Code CLI](#vs-code-extension-vs-claude-code-cli) para detalhes.
</Note>

| Command                    | Shortcut                                                 | Description                                                                                                                                                                                                                    |
| -------------------------- | -------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Focus Input                | `Cmd+Esc` (Mac) / `Ctrl+Esc` (Windows/Linux)             | Alterne o foco entre editor e Claude                                                                                                                                                                                           |
| Open in Side Bar           | -                                                        | Abra Claude na barra lateral esquerda                                                                                                                                                                                          |
| Open in Terminal           | -                                                        | Abra Claude em modo terminal                                                                                                                                                                                                   |
| Open in New Tab            | `Cmd+Shift+Esc` (Mac) / `Ctrl+Shift+Esc` (Windows/Linux) | Abra uma nova conversa como uma aba de editor                                                                                                                                                                                  |
| Open in New Window         | -                                                        | Abra uma nova conversa em uma janela separada                                                                                                                                                                                  |
| New Conversation           | `Cmd+N` (Mac) / `Ctrl+N` (Windows/Linux)                 | Inicie uma nova conversa. Requer que Claude esteja focado e `enableNewConversationShortcut` definido como `true`                                                                                                               |
| Reopen Closed Session      | `Cmd+Shift+T` (Mac) / `Ctrl+Shift+T` (Windows/Linux)     | Reabra a aba de sessão Claude fechada mais recentemente. Volta para a reabertura normal de editor fechado do VS Code quando a última aba fechada não era uma sessão Claude. Desabilite com `enableReopenClosedSessionShortcut` |
| Insert @-Mention Reference | `Option+K` (Mac) / `Alt+K` (Windows/Linux)               | Insira uma referência ao arquivo atual e seleção (requer que o editor esteja focado)                                                                                                                                           |
| Show Logs                  | -                                                        | Visualize logs de depuração da extensão                                                                                                                                                                                        |
| Logout                     | -                                                        | Saia de sua conta Anthropic                                                                                                                                                                                                    |

<h3 id="launch-a-vs-code-tab-from-other-tools">
  Inicie uma aba VS Code a partir de outras ferramentas
</h3>

A extensão registra um manipulador de URI em `vscode://anthropic.claude-code/open`. Use-o para abrir uma nova aba Claude Code a partir de suas próprias ferramentas: um alias de shell, um bookmarklet de navegador ou qualquer script que possa abrir uma URL. Se VS Code não estiver já em execução, abrir a URL o inicia primeiro. Se VS Code já estiver em execução, a URL abre na janela que está atualmente focada.

Invoque o manipulador com o abridor de URL do seu sistema operacional.

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    open "vscode://anthropic.claude-code/open"
    ```
  </Tab>

  <Tab title="Linux">
    ```bash theme={null}
    xdg-open "vscode://anthropic.claude-code/open"
    ```
  </Tab>

  <Tab title="Windows">
    No PowerShell:

    ```powershell theme={null}
    Start-Process "vscode://anthropic.claude-code/open"
    ```

    No `cmd.exe`, `start` trata seu primeiro argumento entre aspas como um título de janela, então passe um título vazio antes da URL:

    ```cmd theme={null}
    start "" "vscode://anthropic.claude-code/open"
    ```
  </Tab>
</Tabs>

O manipulador aceita dois parâmetros de consulta opcionais:

| Parameter | Description                                                                                                                                                                                                                                                                                                                                                                                                    |
| --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`  | Texto para pré-preenchimento na caixa de prompt. Deve ser codificado em URL. O prompt é pré-preenchido mas não enviado automaticamente.                                                                                                                                                                                                                                                                        |
| `session` | Um ID de sessão para retomar em vez de iniciar uma nova conversa. A sessão deve pertencer ao espaço de trabalho atualmente aberto no VS Code. Se a sessão não for encontrada, uma conversa nova é iniciada em vez disso. Se a sessão já estiver aberta em uma aba, essa aba é focada. Para capturar um ID de sessão programaticamente, consulte [Continue conversations](/docs/pt/headless#continue-conversations). |

Por exemplo, para abrir uma aba pré-preenchida com "review my changes":

```text theme={null}
vscode://anthropic.claude-code/open?prompt=review%20my%20changes
```

Para iniciar uma sessão de terminal em vez de uma aba VS Code, use o manipulador `claude-cli://` da CLI. Consulte [Launch sessions from links](/docs/pt/deep-links).

<h2 id="configure-settings">
  Configurar configurações
</h2>

A extensão tem dois tipos de configurações:

* **Extension settings** no VS Code: controlam o comportamento da extensão dentro do VS Code. Abra com `Cmd+,` (Mac) ou `Ctrl+,` (Windows/Linux), depois vá para Extensions → Claude Code. Você também pode digitar `/` e selecionar **General Config** para abrir as configurações.
* **Claude Code settings** em `~/.claude/settings.json`: compartilhadas entre a extensão e CLI. Use para comandos permitidos, variáveis de ambiente, hooks e MCP servers. Consulte [Settings](/docs/pt/settings) para detalhes.

<Tip>
  Adicione `"$schema": "https://json.schemastore.org/claude-code-settings.json"` ao seu `settings.json` para obter autocomplete e validação inline para todas as configurações disponíveis diretamente no VS Code.
</Tip>

<h3 id="extension-settings">
  Extension settings
</h3>

| Setting                             | Default   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ----------------------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `useTerminal`                       | `false`   | Inicie Claude em modo terminal em vez de painel gráfico                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `initialPermissionMode`             | `default` | Controla prompts de aprovação para novas conversas: `default`, `plan`, `acceptEdits` ou `bypassPermissions`. `manual` é um alias para `default` e seleciona o modo rotulado **Manual** no indicador de modo. Requer Claude Code v2.1.200 ou posterior. Consulte [permission modes](/docs/pt/permission-modes).                                                                                                                                                                                  |
| `preferredLocation`                 | `panel`   | Onde Claude abre: `sidebar` (direita) ou `panel` (nova aba)                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `autosave`                          | `true`    | Auto-salve arquivos antes de Claude lê-los ou escrevê-los                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `useCtrlEnterToSend`                | `false`   | Use Ctrl/Cmd+Enter em vez de Enter para enviar prompts                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `enableNewConversationShortcut`     | `false`   | Habilite Cmd/Ctrl+N para iniciar uma nova conversa                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `enableReopenClosedSessionShortcut` | `true`    | Use Cmd/Ctrl+Shift+T para reabrir a aba de sessão Claude fechada mais recentemente. Quando a última aba fechada não era uma sessão Claude, o atalho executa o comando normal de reabrir editor fechado do VS Code.                                                                                                                                                                                                                                                                         |
| `hideOnboarding`                    | `false`   | Oculte a lista de verificação de onboarding (ícone de chapéu de formatura)                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `respectGitIgnore`                  | `true`    | Exclua padrões .gitignore de pesquisas de arquivo                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `usePythonEnvironment`              | `true`    | Ative o ambiente Python do espaço de trabalho ao executar Claude. Requer a extensão Python.                                                                                                                                                                                                                                                                                                                                                                                                |
| `environmentVariables`              | `[]`      | Defina variáveis de ambiente para o processo Claude. Use configurações Claude Code em vez disso para configuração compartilhada.                                                                                                                                                                                                                                                                                                                                                           |
| `disableLoginPrompt`                | `false`   | Pule prompts de autenticação (para configurações de provedor de terceiros)                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `allowDangerouslySkipPermissions`   | `false`   | Adiciona Bypass permissions ao seletor de modo. Use apenas em sandboxes sem acesso à internet.                                                                                                                                                                                                                                                                                                                                                                                             |
| `claudeProcessWrapper`              | -         | Executável usado para iniciar o processo Claude. O caminho do binário incluído é passado como um argumento quando presente. Defina isso para um binário `claude` instalado separadamente se a compilação da extensão não incluir um para sua plataforma. Um erro "Unsupported platform" na ativação significa que nenhum binário está incluído para sua plataforma; consulte [which platforms have prebuilt binaries](/docs/pt/troubleshoot-install#native-binary-not-found-after-npm-install). |

<h2 id="vs-code-extension-vs-claude-code-cli">
  VS Code extension vs. Claude Code CLI
</h2>

Claude Code está disponível tanto como uma extensão VS Code (painel gráfico) quanto como uma CLI (interface de linha de comando no terminal). Alguns recursos estão disponíveis apenas na CLI. Se você precisar de um recurso apenas da CLI, execute `claude` no terminal integrado do VS Code. Isso requer a [instalação da CLI autônoma](/docs/pt/setup): a extensão não adiciona `claude` ao seu PATH. Consulte [Executar CLI no VS Code](#run-cli-in-vs-code).

| Feature             | CLI                 | VS Code Extension                                                                                  |
| ------------------- | ------------------- | -------------------------------------------------------------------------------------------------- |
| Commands and skills | [All](/docs/pt/commands) | Subset (digite `/` para ver disponíveis)                                                           |
| MCP server config   | Yes                 | Partial (adicione servidores via CLI; gerencie servidores existentes com `/mcp` no painel de chat) |
| Checkpoints         | Yes                 | Yes                                                                                                |
| `!` bash shortcut   | Yes                 | No                                                                                                 |
| Tab completion      | Yes                 | No                                                                                                 |

<h3 id="rewind-with-checkpoints">
  Rewind with checkpoints
</h3>

A extensão VS Code suporta checkpoints, que rastreiam edições de arquivo do Claude e permitem que você retroceda para um estado anterior. Passe o mouse sobre qualquer mensagem para revelar o botão de retrocesso, depois escolha entre três opções:

* **Fork conversation from here**: inicie um novo ramo de conversa a partir desta mensagem mantendo todas as alterações de código intactas
* **Rewind code to here**: reverta alterações de arquivo de volta a este ponto na conversa mantendo o histórico completo de conversas
* **Fork conversation and rewind code**: inicie um novo ramo de conversa e reverta alterações de arquivo para este ponto

Para detalhes completos sobre como checkpoints funcionam e suas limitações, consulte [Checkpointing](/docs/pt/checkpointing).

<h3 id="run-cli-in-vs-code">
  Run CLI in VS Code
</h3>

Para usar a CLI enquanto permanece no VS Code, abra o terminal integrado (`` Ctrl+` `` no Windows/Linux ou `` Cmd+` `` no Mac) e execute `claude`. A CLI se integra automaticamente ao seu IDE para recursos como visualização de diff e compartilhamento de diagnósticos.

Instalar a extensão não coloca `claude` no PATH do seu shell. A extensão agrupa uma cópia privada da CLI para seu painel de chat, mas digitar `claude` em um terminal requer a [instalação da CLI autônoma](/docs/pt/setup). Execute a instalação uma vez e os comandos nesta página, incluindo `claude mcp add` e `claude --resume`, funcionam em qualquer terminal. Se `claude` ainda não for encontrado após a instalação, [verifique seu PATH](/docs/pt/troubleshoot-install#verify-your-path).

Se estiver usando um terminal externo, execute `/ide` dentro de Claude Code para conectá-lo ao VS Code.

<h3 id="switch-between-extension-and-cli">
  Switch between extension and CLI
</h3>

A extensão e CLI compartilham o mesmo histórico de conversas. Para continuar uma conversa de extensão na CLI, execute `claude --resume` no terminal. Isso abre um seletor interativo onde você pode pesquisar e selecionar sua conversa.

<h3 id="include-terminal-output-in-prompts">
  Include terminal output in prompts
</h3>

Referencie a saída do terminal em seus prompts usando `@terminal:name` onde `name` é o título do terminal. Isso permite que Claude veja a saída do comando, mensagens de erro ou logs sem copiar e colar.

<h3 id="monitor-background-processes">
  Monitor background processes
</h3>

Quando Claude executa comandos de longa duração, a extensão mostra progresso na barra de status. No entanto, a visibilidade para tarefas em segundo plano é limitada em comparação com a CLI. Para melhor visibilidade, peça ao Claude para exibir o comando para que você possa executá-lo no terminal integrado do VS Code.

<h3 id="connect-to-external-tools-with-mcp">
  Connect to external tools with MCP
</h3>

MCP (Model Context Protocol) servers dão ao Claude acesso a ferramentas externas, bancos de dados e APIs.

Para adicionar um MCP server, abra o terminal integrado (`` Ctrl+` `` ou `` Cmd+` ``) e execute `claude mcp add`. O exemplo abaixo adiciona o MCP server remoto do GitHub, que autentica com um [personal access token](https://github.com/settings/personal-access-tokens) passado como um cabeçalho:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Uma vez configurado, peça ao Claude para usar as ferramentas (por exemplo, "Review PR #456").

Para gerenciar MCP servers sem sair do VS Code, digite `/mcp` no painel de chat. O diálogo de gerenciamento de MCP permite que você habilite ou desabilite servidores, reconecte a um servidor e gerencie autenticação OAuth. Consulte a [MCP documentation](/docs/pt/mcp) para servidores disponíveis.

<h2 id="work-with-git">
  Trabalhar com git
</h2>

Claude Code se integra com git para ajudar com fluxos de trabalho de controle de versão diretamente no VS Code. Peça ao Claude para fazer commit de alterações, criar pull requests ou trabalhar em branches.

<h3 id="create-commits-and-pull-requests">
  Criar commits e pull requests
</h3>

Claude pode preparar alterações, escrever mensagens de commit e criar pull requests com base em seu trabalho:

```text theme={null}
> commit my changes with a descriptive message
> create a pr for this feature
> summarize the changes I've made to the auth module
```

Ao criar pull requests, Claude gera descrições com base nas alterações de código reais e pode adicionar contexto sobre testes ou decisões de implementação.

<h3 id="use-git-worktrees-for-parallel-tasks">
  Usar git worktrees para tarefas paralelas
</h3>

Use a flag `--worktree` (`-w`) para iniciar Claude em um worktree isolado com seus próprios arquivos e branch:

```bash theme={null}
claude --worktree feature-auth
```

Cada worktree mantém estado de arquivo independente enquanto compartilha histórico git. Isso evita que instâncias do Claude interfiram uma com a outra ao trabalhar em diferentes tarefas. Para mais detalhes, consulte [Run parallel sessions with Git worktrees](/docs/pt/worktrees).

<h2 id="use-third-party-providers">
  Use third-party providers
</h2>

Por padrão, Claude Code se conecta diretamente à API da Anthropic. Se sua organização usa Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry para acessar Claude, configure a extensão para usar seu provedor em vez disso:

<Steps>
  <Step title="Disable login prompt">
    Abra a [Disable Login Prompt setting](vscode://settings/claudeCode.disableLoginPrompt) e marque a caixa.

    Você também pode abrir as configurações do VS Code (`Cmd+,` no Mac ou `Ctrl+,` no Windows/Linux), pesquisar por "Claude Code login" e marcar **Disable Login Prompt**.
  </Step>

  <Step title="Configure your provider">
    Siga o guia de configuração para seu provedor:

    * [Claude Code on Amazon Bedrock](/docs/pt/amazon-bedrock)
    * [Claude Code on Google Cloud's Agent Platform](/docs/pt/google-vertex-ai)
    * [Claude Code on Microsoft Foundry](/docs/pt/microsoft-foundry)

    Estes guias cobrem a configuração de seu provedor em `~/.claude/settings.json`, o que garante que suas configurações sejam compartilhadas entre a extensão VS Code e a CLI.
  </Step>
</Steps>

<h2 id="security-and-privacy">
  Segurança e privacidade
</h2>

Seu código permanece privado. Claude Code processa seu código para fornecer assistência, mas não o usa para treinar modelos. Para detalhes sobre manipulação de dados e como desativar o logging, consulte [Data and privacy](/docs/pt/data-usage).

Com permissões de auto-edição habilitadas, Claude Code pode modificar arquivos de configuração do VS Code (como `settings.json` ou `tasks.json`) que o VS Code pode executar automaticamente. Para reduzir o risco ao trabalhar com código não confiável:

* Habilite [VS Code Restricted Mode](https://code.visualstudio.com/docs/editor/workspace-trust#_restricted-mode) para espaços de trabalho não confiáveis
* Use modo de aprovação manual em vez de auto-accept para edições
* Revise as alterações cuidadosamente antes de aceitá-las

<h3 id="the-built-in-ide-mcp-server">
  The built-in IDE MCP server
</h3>

Quando a extensão está ativa, ela executa um servidor MCP local ao qual a CLI se conecta automaticamente. É assim que a CLI abre diffs no visualizador de diff nativo do VS Code, lê sua seleção atual para `@`-mentions e — quando você está trabalhando em um notebook Jupyter — pede ao VS Code para executar células.

O servidor é nomeado `ide` e está oculto de `/mcp` porque não há nada para configurar. Se sua organização usa um hook `PreToolUse` para criar uma lista de permissões de ferramentas MCP, porém, você precisará saber que ele existe.

**Seleção e contexto de arquivo aberto.** Enquanto conectado, a CLI inclui sua seleção atual do editor e o caminho do arquivo ativo como contexto em cada prompt que você envia. A transcrição mostra uma linha `⧉ Selected N lines from <file>` quando isso acontece. Para excluir um arquivo sensível como `.env`, adicione uma [regra de negação `Read`](/docs/pt/permissions#read-and-edit) para seu caminho. Uma regra de negação correspondente impede que tanto o texto selecionado quanto o aviso de arquivo aberto para esse arquivo cheguem ao Claude.

**Transporte e autenticação.** O servidor se vincula a `127.0.0.1` em uma porta aleatória no intervalo 10000–65535, e a porta não é configurável. O transporte é `ws://` não criptografado; porque o socket é apenas loopback, qualquer processo que pudesse capturar o tráfego também pode ler o token do arquivo de lock, então TLS não adicionaria proteção. Cada ativação de extensão gera um token de autenticação aleatório novo, escreve-o em um arquivo de lock em `~/.claude/ide/<port>.lock`, e a CLI deve apresentá-lo como o header `X-Claude-Code-Ide-Authorization` para se conectar. O arquivo de lock tem permissões `0600` em um diretório `0700`, então apenas o usuário executando VS Code pode lê-lo. Se `CLAUDE_CONFIG_DIR` estiver definido, o arquivo de lock é escrito em `$CLAUDE_CONFIG_DIR/ide/` em vez disso.

**Ferramentas expostas ao modelo.** O servidor hospeda uma dúzia de ferramentas, mas apenas duas são visíveis para o modelo. O resto é RPC interno que a CLI usa para sua própria UI — abrindo diffs, lendo seleções, salvando arquivos — e são filtrados antes da lista de ferramentas chegar ao Claude.

| Tool name (as seen by hooks) | What it does                                                                                                                     | Read-only |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------- | --------- |
| `mcp__ide__getDiagnostics`   | Retorna diagnósticos do language-server — os erros e avisos no painel Problems do VS Code. Opcionalmente escopo para um arquivo. | Yes       |
| `mcp__ide__executeCode`      | Executa código Python no kernel do notebook Jupyter ativo. Consulte fluxo de confirmação abaixo.                                 | No        |

**Execução Jupyter sempre pergunta primeiro.** `mcp__ide__executeCode` não pode executar nada silenciosamente. Em cada chamada, o código é inserido como uma nova célula no final do notebook ativo, VS Code a rola para a vista, e um Quick Pick nativo pergunta se você quer **Execute** ou **Cancel**. Cancelar — ou descartar o seletor com `Esc` — retorna um erro ao Claude e nada é executado. A ferramenta também se recusa completamente quando não há um notebook ativo, quando a extensão Jupyter (`ms-toolsai.jupyter`) não está instalada, ou quando o kernel não é Python.

<Note>
  A confirmação do Quick Pick é separada dos hooks `PreToolUse`. Uma entrada de lista de permissões para `mcp__ide__executeCode` permite que Claude *proponha* executar uma célula; o Quick Pick dentro do VS Code é o que permite que ele *realmente* execute.
</Note>

<a id="troubleshooting" />

<h2 id="fix-common-issues">
  Corrigir problemas comuns
</h2>

<h3 id="extension-won’t-install">
  Extension won't install
</h3>

* Certifique-se de que você tem uma versão compatível do VS Code (1.98.0 ou posterior)
* Verifique se o VS Code tem permissão para instalar extensões
* Tente instalar diretamente do [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=anthropic.claude-code)

<h3 id="spark-icon-not-visible">
  Spark icon not visible
</h3>

O ícone Spark aparece na **Editor Toolbar** (canto superior direito do editor) quando você tem um arquivo aberto. Se você não o vir:

1. **Open a file**: O ícone requer um arquivo aberto. Ter apenas uma pasta aberta não é suficiente.
2. **Check VS Code version**: Requer 1.98.0 ou superior (Help → About)
3. **Restart VS Code**: Execute "Developer: Reload Window" na Paleta de Comandos
4. **Disable conflicting extensions**: Desabilite temporariamente outras extensões de IA (Cline, Continue, etc.)
5. **Check workspace trust**: A extensão não funciona em Restricted Mode

Alternativamente, clique em "✱ Claude Code" na **Status Bar** (canto inferior direito). Isso funciona mesmo sem um arquivo aberto. Você também pode usar a **Command Palette** (`Cmd+Shift+P` / `Ctrl+Shift+P`) e digitar "Claude Code".

<h3 id="cmd-esc-does-nothing-on-macos">
  Cmd+Esc does nothing on macOS
</h3>

No macOS Tahoe e posterior, o atalho do sistema Game Overlay está vinculado a `Cmd+Esc` por padrão e intercepta a tecla antes que chegue ao VS Code. Para liberar o atalho:

1. Abra System Settings
2. Vá para Keyboard, depois Keyboard Shortcuts, depois Game Controllers
3. Desmarque a caixa de seleção Game Overlay

Alternativamente, reassine a extensão para uma chave diferente: abra o editor de [Keyboard Shortcuts](https://code.visualstudio.com/docs/configure/keybindings) do VS Code (`Cmd+K Cmd+S`), procure por `Claude Code: Focus input`, e atribua uma nova vinculação.

<h3 id="claude-code-never-responds">
  Claude Code never responds
</h3>

Se Claude Code não está respondendo aos seus prompts:

1. **Check your internet connection**: Certifique-se de que você tem uma conexão de internet estável
2. **Start a new conversation**: Tente iniciar uma nova conversa para ver se o problema persiste
3. **Try the CLI**: Execute `claude` do terminal para ver se você obtém mensagens de erro mais detalhadas

Se os problemas persistirem, [file an issue on GitHub](https://github.com/anthropics/claude-code/issues) com detalhes sobre o erro.

<h2 id="uninstall-the-extension">
  Desinstalar a extensão
</h2>

Para desinstalar a extensão Claude Code:

1. Abra a visualização de Extensões (`Cmd+Shift+X` no Mac ou `Ctrl+Shift+X` no Windows/Linux)
2. Pesquise por "Claude Code"
3. Clique em **Uninstall**

Executar `claude` em um terminal integrado do VS Code reinstala a extensão automaticamente. Para mantê-la desinstalada, desative **Auto-install IDE extension** em `/config`, ou defina [`autoInstallIdeExtension`](/docs/pt/settings#global-config-settings) como `false`. Você também pode definir a variável de ambiente [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/pt/env-vars) como `1`.

Para também remover dados de extensão e redefinir todas as configurações, delete o diretório de armazenamento da extensão para sua plataforma.

No macOS:

```bash theme={null}
rm -rf ~/Library/"Application Support"/Code/User/globalStorage/anthropic.claude-code
```

No Linux:

```bash theme={null}
rm -rf ~/.config/Code/User/globalStorage/anthropic.claude-code
```

No Windows, no PowerShell:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:APPDATA\Code\User\globalStorage\anthropic.claude-code"
```

Para ajuda adicional, consulte o [guia de solução de problemas](/docs/pt/troubleshooting).

<h2 id="next-steps">
  Próximos passos
</h2>

Agora que você tem Claude Code configurado no VS Code:

* [Explore common workflows](/docs/pt/common-workflows) para aproveitar ao máximo Claude Code
* [Set up MCP servers](/docs/pt/mcp) para estender as capacidades do Claude com ferramentas externas. Adicione servidores usando a CLI, depois gerencie-os com `/mcp` no painel de chat.
* [Configure Claude Code settings](/docs/pt/settings) para personalizar comandos permitidos, hooks e muito mais. Essas configurações são compartilhadas entre a extensão e CLI.
