> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Use Claude Code with Chrome

> Conecte Claude Code ao seu navegador Chrome para testar aplicativos web, depurar com logs de console, automatizar preenchimento de formulários e extrair dados de páginas web.

Claude Code integra-se com a [extensão Claude in Chrome do navegador](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) para oferecer recursos de automação de navegador a partir da CLI ou da [extensão VS Code](/docs/pt/vs-code#automate-browser-tasks-with-chrome). Construa seu código, depois teste e depure no navegador sem trocar de contexto.

Claude abre novas abas para tarefas do navegador e compartilha o estado de login do seu navegador, para que possa acessar qualquer site em que você já esteja conectado. As ações do navegador são executadas em uma janela Chrome visível em tempo real. Quando Claude encontra uma página de login ou CAPTCHA, ele pausa e pede que você a manipule manualmente.

A extensão coleta as abas que Claude abre em um grupo de abas do Chrome vinculado à sua sessão. Em sessões locais, se Claude Code fecha esse grupo quando a sessão termina depende de como ela termina:

* Quando você digita `/clear`, Claude Code fecha o grupo, incluindo páginas abertas, a menos que o trabalho que sobrevive ao clear ainda esteja em execução
* Quando você muda de sessões com um comando como `/resume`, sai de Claude Code ou executa um `/clear` enquanto o trabalho que sobrevive a ele ainda está em execução, Claude Code fecha o grupo apenas se ele contiver nada além de novas abas vazias, para que as páginas que você ainda pode estar lendo permaneçam abertas

<Note>
  A integração com Chrome funciona com Google Chrome e Microsoft Edge. Claude Code também detecta a extensão e configura a conexão em outros navegadores baseados em Chromium, incluindo Brave, Arc, Vivaldi e Opera. A integração com Chrome não é suportada no Windows Subsystem for Linux (WSL).
</Note>

<h2 id="capabilities">
  Recursos
</h2>

Com Chrome conectado, você pode encadear ações do navegador com tarefas de codificação em um único fluxo de trabalho:

* **Depuração ao vivo**: leia erros de console e estado do DOM diretamente, depois corrija o código que os causou
* **Verificação de design**: construa uma interface a partir de um mock do Figma, depois abra-a no navegador para verificar se corresponde
* **Teste de aplicativo web**: teste validação de formulário, verifique regressões visuais ou verifique fluxos de usuário
* **Aplicativos web autenticados**: interaja com Google Docs, Gmail, Notion ou qualquer aplicativo em que você esteja conectado sem conectores de API
* **Extração de dados**: extraia informações estruturadas de páginas web e salve-as localmente
* **Automação de tarefas**: automatize tarefas repetitivas do navegador como entrada de dados, preenchimento de formulários ou fluxos de trabalho em vários sites
* **Uploads de arquivo**: anexe arquivos da sua máquina para carregar em campos de upload em páginas web
* **Gravação de sessão**: grave interações do navegador como GIFs para documentar ou compartilhar o que aconteceu

<h2 id="prerequisites">
  Pré-requisitos
</h2>

Antes de usar Claude Code com Chrome, você precisa de:

* [Google Chrome](https://www.google.com/chrome/), [Microsoft Edge](https://www.microsoft.com/edge) ou outro navegador baseado em Chromium, como Brave, Arc, Vivaldi ou Opera
* Extensão [Claude in Chrome](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) versão 1.0.36 ou superior, disponível na Chrome Web Store
* [Claude Code](/docs/pt/quickstart#step-1-install-claude-code)
* Um plano Anthropic direto (Pro, Max, Team ou Enterprise)

A integração com Chrome também requer fazer login com `/login`. Se você se autenticar com uma chave de API ou um token de longa duração de [`claude setup-token`](/docs/pt/authentication#generate-a-long-lived-token), Claude Code mantém a integração com Chrome desativada, mesmo quando você passa `--chrome`, porque a extensão do navegador não consegue se autenticar com essas credenciais. Antes da v2.1.216, essas sessões poderiam ativar a integração com Chrome, mas cada tentativa de conexão com a extensão do navegador falhava com um erro 403.

<Note>
  A integração com Chrome não está disponível através de provedores terceirizados como Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry. Se você acessa Claude exclusivamente através de um provedor terceirizado, você precisa de uma conta claude.ai separada para usar este recurso.
</Note>

<h2 id="get-started-in-the-cli">
  Comece na CLI
</h2>

<Steps>
  <Step title="Inicie Claude Code com Chrome">
    Inicie Claude Code com a flag `--chrome`:

    ```bash theme={null}
    claude --chrome
    ```

    Na primeira vez que você inicia com Chrome, Claude Code mostra um diálogo único que apresenta a integração e explica como as permissões de site funcionam. Pressione Enter para continuar.

    Para ativar Chrome para futuras sessões sem a flag, consulte [Ativar Chrome por padrão](#enable-chrome-by-default).
  </Step>

  <Step title="Peça a Claude para usar o navegador">
    Este exemplo navega para uma página, interage com ela e relata o que encontra, tudo a partir do seu terminal ou editor:

    ```text wrap theme={null}
    Go to code.claude.com/docs, click on the search box,
    type "hooks", and tell me what results appear
    ```

    Se Claude Code solicitar permissão antes de uma ação do navegador, aprove-a. O diálogo começa com `Claude in Chrome wants to` e oferece uma opção para permitir todas as ações naquele site durante a sessão. Claude abre uma nova aba e inicia a tarefa.
  </Step>
</Steps>

Execute `/chrome` a qualquer momento para verificar o status da conexão, gerenciar permissões, reconectar a extensão ou escolher qual navegador conectado usar. A integração está funcionando quando o painel de status mostra "Status: Enabled" e "Extension: Installed".

Se mais de um navegador estiver conectado, você escolhe qual Claude usa. Quando uma ação do navegador começa antes de você ter escolhido, Claude solicita que você escolha um. Para alternar navegadores depois, execute `/chrome` e selecione **Select browser…**. Claude continua usando sua escolha mesmo quando outro navegador se conecta.

Para VS Code, consulte [automação de navegador em VS Code](/docs/pt/vs-code#automate-browser-tasks-with-chrome).

<h3 id="install-the-extension-when-claude-asks">
  Instale a extensão quando Claude pedir
</h3>

Quando Claude precisa do seu navegador em uma sessão interativa e Claude Code não detecta a extensão, Claude Code mostra um prompt de instalação intitulado "Claude wants to use your browser". Claude Code pergunta no máximo uma vez por sessão.

O prompt oferece três opções:

* **Install extension**: abre a página de instalação da extensão no seu navegador e inicia uma configuração guiada. Claude Code aguarda a instalação, conecta a extensão e ativa as ferramentas do navegador na mesma sessão. Quando a conexão estiver pronta, selecione "Continue with browser tools" e Claude retoma a tarefa no seu navegador. Você pode sair da configuração selecionando "Continue without browser tools" e terminar depois com `/chrome`.
* **Not now**: continua a tarefa sem ferramentas do navegador. Claude Code pode perguntar novamente em uma sessão posterior.
* **Don't ask again**: interrompe o prompt em futuras sessões. Você ainda pode configurar a integração a qualquer momento com `/chrome`.

Se sua organização bloqueia o servidor MCP `claude-in-chrome` com a [configuração gerenciada `deniedMcpServers`](/docs/pt/managed-mcp#policy-based-control-with-allowlists-and-denylists), Claude Code não mostra o prompt de instalação.

<h3 id="enable-chrome-by-default">
  Ativar Chrome por padrão
</h3>

Para evitar passar `--chrome` em cada sessão, execute `/chrome` e selecione "Enabled by default".

Claude Code inicia normalmente quando Chrome não está em execução. Antes da v2.1.211, a inicialização poderia travar quando a integração Chrome estava ativada mas Chrome não estava em execução.

Na [extensão VS Code](/docs/pt/vs-code#automate-browser-tasks-with-chrome), Chrome está disponível sempre que a extensão Chrome está instalada. Nenhuma flag adicional é necessária.

<Note>
  Ativar Chrome por padrão na CLI aumenta o uso de contexto, pois as ferramentas do navegador estão sempre carregadas. Se você notar aumento no consumo de contexto, desative esta configuração e use `--chrome` apenas quando necessário.
</Note>

<h3 id="manage-site-permissions">
  Gerenciar permissões de site
</h3>

As permissões no nível do site são herdadas da extensão Chrome. Gerencie permissões nas configurações da extensão Chrome para controlar quais sites Claude pode navegar, clicar e digitar.

<h3 id="browser-tools-in-plan-mode">
  Ferramentas do navegador no modo de plano
</h3>

No [modo de plano](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode), um prompt de permissão aparece antes de Claude gravar um GIF, abrir uma nova aba ou executar um atalho. Se o [modo de bypass de permissões estiver disponível](/docs/pt/permission-modes#skip-all-checks-with-bypasspermissions-mode) em sua sessão e a [busca de sinalizador de recurso](/docs/pt/env-vars#features-that-need-feature-flag-fetching) estiver desativada, essas chamadas são executadas sem um prompt.

Uma chamada `tabs_context_mcp` também solicita quando define `createIfEmpty`, e o mesmo ocorre com uma chamada `browser_batch` que inclui qualquer uma dessas ações.

<h2 id="example-workflows">
  Fluxos de trabalho de exemplo
</h2>

Estes exemplos mostram maneiras comuns de combinar ações do navegador com tarefas de codificação. Execute `/mcp`, selecione `claude-in-chrome` e, em seguida, selecione **View tools** para ver a lista completa de ferramentas de navegador disponíveis.

<h3 id="test-a-local-web-application">
  Teste um aplicativo web local
</h3>

Ao desenvolver um aplicativo web, peça a Claude para verificar se suas alterações funcionam corretamente:

```text wrap theme={null}
I just updated the login form validation. Can you open localhost:3000,
try submitting the form with invalid data, and check if the error
messages appear correctly?
```

Claude navega para seu servidor local, interage com o formulário e relata o que observa.

<h3 id="debug-with-console-logs">
  Depurar com logs de console
</h3>

Claude pode ler a saída do console para ajudar a diagnosticar problemas. Diga a Claude quais padrões procurar em vez de pedir toda a saída do console, pois os logs podem ser verbosos:

```text wrap theme={null}
Open the dashboard page and check the console for any errors when
the page loads.
```

Claude lê as mensagens do console e pode filtrar padrões específicos ou tipos de erro.

<h3 id="automate-form-filling">
  Automatizar preenchimento de formulários
</h3>

Acelere tarefas repetitivas de entrada de dados:

```text wrap theme={null}
I have a spreadsheet of customer contacts in contacts.csv. For each row,
go to the CRM at crm.example.com, click "Add Contact", and fill in the
name, email, and phone fields.
```

Claude lê seu arquivo local, navega pela interface web e insere os dados para cada registro.

<h3 id="upload-files-to-web-pages">
  Carregar arquivos em páginas web
</h3>

Claude pode anexar arquivos de sua máquina para fazer upload em campos de uma página. Claude Code lê o arquivo e envia seu conteúdo para o navegador, portanto os uploads funcionam em sessões locais e remotas. Requer Claude Code v2.1.211 ou posterior.

Este exemplo anexa um arquivo de log a um formulário:

```text wrap theme={null}
Open the bug tracker at bugs.example.com, create a new issue,
and attach logs/session.log to it
```

Três restrições se aplicam aos uploads:

* **Permissões**: Claude pode fazer upload de um arquivo apenas quando a sessão tem permissão para lê-lo, portanto [regras de permissão](/docs/pt/settings-reference#permission-settings) que negam acesso `Read` a um arquivo também bloqueiam seu upload.
* **Tamanho**: um único upload pode incluir até 10 MB de arquivos no total.
* **Hard links**: Claude recusa arquivos que possuem múltiplos hard links, o que é comum dentro de lojas de gerenciadores de pacotes como `node_modules`. Copie o arquivo e faça upload da cópia.

<h3 id="draft-content-in-google-docs">
  Rascunhar conteúdo no Google Docs
</h3>

Use Claude para escrever diretamente em seus documentos sem configuração de API:

```text wrap theme={null}
Draft a project update based on the recent commits and add it to my
Google Doc at docs.google.com/document/d/abc123
```

Claude abre o documento, clica no editor e digita o conteúdo. Isso funciona com qualquer aplicativo web em que você esteja conectado: Gmail, Notion, Sheets e muito mais.

<h3 id="extract-data-from-web-pages">
  Extrair dados de páginas web
</h3>

Extraia informações estruturadas de sites:

```text wrap theme={null}
Go to the product listings page and extract the name, price, and
availability for each item. Save the results as a CSV file.
```

Claude navega para a página, lê o conteúdo e compila os dados em um formato estruturado.

<h3 id="run-multi-site-workflows">
  Executar fluxos de trabalho em vários sites
</h3>

Coordene tarefas em vários sites:

```text wrap theme={null}
Check my calendar for meetings tomorrow, then for each meeting with
an external attendee, look up their company website and add a note
about what they do.
```

Claude trabalha em abas para reunir informações e concluir o fluxo de trabalho.

<h3 id="record-a-demo-gif">
  Gravar um GIF de demonstração
</h3>

Crie gravações compartilháveis de interações do navegador:

```text wrap theme={null}
Record a GIF showing how to complete the checkout flow, from adding
an item to the cart through to the confirmation page.
```

Claude grava a sequência de interação e a salva como um arquivo GIF. A gravação captura tudo o que está visível no navegador, incluindo detalhes da conta em páginas conectadas, portanto revise-a antes de compartilhá-la fora de sua equipe.

<h3 id="save-screenshots-to-disk">
  Salvar capturas de tela em disco
</h3>

Peça a Claude para manter uma captura de tela como um arquivo:

```text wrap theme={null}
Take a screenshot of the checkout page and save it to disk
```

Claude salva a imagem em disco e relata o caminho do arquivo. Antes da v2.1.211, a opção `save_to_disk` da ferramenta de captura de tela não gravava um arquivo.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="extension-not-detected">
  Extensão não detectada
</h3>

Se Claude Code não conseguir detectar a extensão Chrome:

1. Verifique se a extensão Chrome está instalada e ativada em `chrome://extensions`
2. Verifique se Claude Code está atualizado executando `claude --version`
3. Verifique se Chrome está em execução
4. Execute `/chrome` e selecione "Reconnect extension" para restabelecer a conexão
5. Se o problema persistir, reinicie Claude Code e Chrome

Na primeira vez que você ativa a integração com Chrome, Claude Code instala um arquivo de configuração do host de mensagens nativas. Chrome lê este arquivo na inicialização, portanto, se a extensão não for detectada na sua primeira tentativa, reinicie Chrome para pegar a nova configuração.

Claude Code abre uma aba do navegador solicitando que você conecte a extensão apenas nessa primeira instalação. Claude Code não a reabre quando uma sessão posterior reescreve o arquivo de configuração, por exemplo após alternar entre compilações ou diretórios de configuração.

Se a conexão ainda falhar, verifique se o arquivo de configuração do host existe em:

Para Chrome:

* **macOS**: `~/Library/Application Support/Google/Chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/google-chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: verifique `HKCU\Software\Google\Chrome\NativeMessagingHosts\` no Registro do Windows

Para Edge:

* **macOS**: `~/Library/Application Support/Microsoft Edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/microsoft-edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: verifique `HKCU\Software\Microsoft\Edge\NativeMessagingHosts\` no Registro do Windows

Outros navegadores baseados em Chromium leem o mesmo arquivo de seu próprio diretório de configuração, nomeado de acordo com o navegador. Por exemplo, Brave no macOS usa `~/Library/Application Support/BraveSoftware/Brave-Browser/NativeMessagingHosts/`, e no Windows cada navegador tem sua própria chave de registro, como `HKCU\Software\BraveSoftware\Brave-Browser\NativeMessagingHosts\`.

<h3 id="browser-not-responding">
  Navegador não respondendo
</h3>

Se os comandos do navegador de Claude pararem de funcionar:

1. Verifique se uma caixa de diálogo modal (alerta, confirmação, prompt) está bloqueando a página. Caixas de diálogo JavaScript bloqueiam eventos do navegador e impedem que Claude receba comandos. Feche a caixa de diálogo manualmente, depois diga a Claude para continuar.
2. Peça a Claude para criar uma nova aba e tentar novamente
3. Reinicie a extensão Chrome desativando-a e reativando-a em `chrome://extensions`

<h3 id="connection-drops-during-long-sessions">
  Conexão cai durante sessões longas
</h3>

O service worker da extensão Chrome pode ficar inativo durante sessões estendidas, o que quebra a conexão. Se as ferramentas do navegador pararem de funcionar após um período de inatividade, execute `/chrome` e selecione "Reconnect extension".

<h3 id="windows-specific-issues">
  Problemas específicos do Windows
</h3>

No Windows, você pode encontrar:

* **Conflitos de named pipe (EADDRINUSE)**: se outro processo estiver usando o mesmo named pipe, reinicie Claude Code. Feche qualquer outra sessão de Claude Code que possa estar usando Chrome.
* **Erros de host de mensagens nativas**: se o host de mensagens nativas falhar na inicialização, tente reinstalar Claude Code para regenerar a configuração do host.
* **Páginas de configuração falham ao abrir**: atualize Claude Code. Antes da v2.1.211, a aba do navegador solicitando que você conecte a extensão poderia falhar ao abrir no Windows.

<h3 id="common-error-messages">
  Mensagens de erro comuns
</h3>

Estes são os erros mais frequentemente encontrados e como resolvê-los:

| Erro                                        | Causa                                                                                                                                                                | Solução                                                                                                                                                                                                                                                              |
| ------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| "Browser extension is not connected"        | O host de mensagens nativas não consegue alcançar a extensão, ou a lista de permissões de IP da sua organização rejeita a conexão com `bridge.claudeusercontent.com` | Reinicie Chrome e Claude Code, depois execute `/chrome` para reconectar. Se sua organização usa lista de permissões de IP e o erro persistir, consulte [Organization IP allowlists and proxy egress](/docs/pt/network-config#organization-ip-allowlists-and-proxy-egress) |
| Extension shows "Not detected" in `/chrome` | A extensão Chrome não está instalada ou está desativada                                                                                                              | Instale ou ative a extensão em `chrome://extensions`                                                                                                                                                                                                                 |
| "No tab available"                          | Claude tentou agir antes de uma aba estar pronta                                                                                                                     | Peça a Claude para criar uma nova aba e tentar novamente                                                                                                                                                                                                             |
| "Receiving end does not exist"              | O service worker da extensão ficou inativo                                                                                                                           | Execute `/chrome` e selecione "Reconnect extension"                                                                                                                                                                                                                  |

<h2 id="see-also">
  Veja também
</h2>

* [Computer use](/docs/pt/computer-use): controle aplicativos nativos do macOS quando uma tarefa não pode ser feita em um navegador
* [Use Claude Code in VS Code](/docs/pt/vs-code#automate-browser-tasks-with-chrome): automação de navegador na extensão VS Code
* [Referência CLI](/docs/pt/cli-reference): flags de linha de comando incluindo `--chrome`
* [Fluxos de trabalho comuns](/docs/pt/common-workflows): mais maneiras de usar Claude Code
* [Dados e privacidade](/docs/pt/data-usage): como Claude Code manipula seus dados
* [Getting started with Claude in Chrome](https://support.claude.com/en/articles/12012173-getting-started-with-claude-in-chrome): documentação completa para a extensão Chrome, incluindo atalhos, agendamento e permissões
