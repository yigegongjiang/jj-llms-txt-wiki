> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Iniciar sessões a partir de links

> Abra uma sessão de terminal Claude Code a partir de uma URL. Incorpore links `claude-cli://` em runbooks, alertas e dashboards para que um clique abra Claude Code no repositório correto com o prompt correto.

Um deep link é uma URL `claude-cli://` que abre Claude Code em uma nova janela de terminal. A URL pode carregar um diretório de trabalho e um prompt para pré-preenchimento.

Isso permite que você compartilhe um ponto de partida com um clique para uma tarefa: qualquer pessoa com Claude Code instalado que clique no link verá uma sessão aberta com o prompt já digitado. O prompt é preenchido, mas não é enviado até que você pressione Enter.

Como um deep link é uma URL, você pode colocá-lo em qualquer lugar onde um link possa ir:

* Uma etapa de runbook de incidente que abre o repositório do serviço afetado com um prompt de diagnóstico
* Um alerta de monitoramento ou dashboard que vincula a um prompt de investigação para uma métrica específica
* Uma página README ou wiki que abre o projeto com um prompt de integração
* Uma notificação de falha de CI que pré-preenchimento o nome do trabalho que falhou

Esta página cobre como [construir um link](#build-a-link), [incorporá-lo em um runbook ou acioná-lo a partir do shell](#examples), e [gerenciar ou desabilitar o registro do handler](#registration-and-supported-platforms) em cada plataforma.

<h2 id="how-it-works">
  Como funciona
</h2>

O prefixo `claude-cli://` é um esquema de URL personalizado que Claude Code registra com seu sistema operacional, semelhante a como links `mailto:` abrem seu cliente de email. O link pode estar em uma página da web, em um wiki, em uma mensagem do Slack ou em qualquer aplicativo que renderize links. Quando você clica em um:

1. O navegador ou aplicativo passa a URL para seu sistema operacional.
2. O sistema operacional reconhece o prefixo `claude-cli://` e inicia Claude Code em sua máquina.
3. Uma nova janela de terminal abre com Claude Code executando no diretório que o link especificou, e o texto do prompt do link já está na caixa de entrada.
4. Você lê o prompt, edita-o se desejar e pressiona Enter para enviá-lo.

O link em si pode ser hospedado em qualquer lugar, mas a sessão sempre abre localmente no computador onde você clicou. Veja [Registro e plataformas suportadas](#registration-and-supported-platforms) para saber qual emulador de terminal abre em cada sistema operacional.

<Note>
  A plataforma que exibe o link deve permitir esquemas de URL personalizados. O Markdown renderizado pelo GitHub permite `http` e `https`, mas remove esquemas como `claude-cli://` em READMEs, issues, pull requests e wikis. Apenas o texto do link é exibido, sem link por trás e a URL oculta. Veja [Troubleshooting](#the-link-renders-as-plain-text-instead-of-being-clickable) para uma solução alternativa.
</Note>

<h3 id="what-a-launched-session-shows">
  O que uma sessão iniciada mostra
</h3>

Um deep link nunca executa nada por conta própria. O link apenas escolhe um diretório e preenche a caixa de prompt. Se você clicar em um link de uma página em que não confia, o prompt ainda é inerte: nada chega ao modelo até que você leia o que foi preenchido e pressione Enter.

Quando a sessão abre, uma linha de aviso abaixo da caixa de entrada lê `Prompt from an external link` e permanece visível até que você envie ou limpe o prompt. Para prompts com mais de 1.000 caracteres, o aviso inclui a contagem de caracteres e informa para você rolar e revisar o texto completo antes de pressionar Enter, já que prompts longos podem empurrar instruções para fora da tela. As regras de permissão, `CLAUDE.md` e prompts de confiança para o diretório selecionado se aplicam da mesma forma que para qualquer outra sessão.

<h2 id="build-a-link">
  Construir um link
</h2>

Cada deep link começa com `claude-cli://open`, que é o único caminho que o handler aceita, seguido por parâmetros de consulta opcionais. A forma mínima abre Claude Code em seu diretório inicial com um prompt vazio:

```text theme={null}
claude-cli://open
```

Adicione parâmetros para controlar onde a sessão começa e o que a caixa de prompt contém:

| Parâmetro | Descrição                                                                                                                                                                                                                                                               |
| --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `q`       | Texto para pré-preenchimento na caixa de prompt. [URL-encode](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent) o valor. Use `%0A` para quebras de linha em prompts de múltiplas linhas. Máximo de 5.000 caracteres. |
| `cwd`     | Caminho absoluto para usar como diretório de trabalho. Caminhos de rede e UNC são rejeitados, assim como caminhos que contêm caracteres de controle invisíveis ou bidirecionais.                                                                                        |
| `repo`    | Um slug `owner/name` do GitHub. Claude Code o resolve para um clone local que viu antes e começa lá. Se você não tiver um clone correspondente, a sessão abre em seu diretório inicial.                                                                                 |

`cwd` e `repo` são [duas maneiras de definir o diretório de trabalho](#choose-between-cwd-and-repo). Se você passar ambos, `cwd` tem precedência e `repo` é ignorado, mesmo que o caminho `cwd` não exista.

O link a seguir aponta para um repositório chamado `acme/payments` com um prompt de diagnóstico de duas linhas. Substitua `acme/payments` pelo slug `owner/name` do seu repositório quando construir o seu:

```text theme={null}
claude-cli://open?repo=acme/payments&q=Investigate%20the%20failed%20deploy%20of%20payments-api.%0ACheck%20recent%20commits%20to%20main%20and%20the%20last%20successful%20build.
```

Clicar nele abre uma nova janela de terminal, inicia Claude Code em seu clone local de `acme/payments` e preenche a caixa de prompt com o texto decodificado:

```text theme={null}
Investigate the failed deploy of payments-api.
Check recent commits to main and the last successful build.
```

Você pode editar o prompt antes de pressionar Enter para enviá-lo. Se você não tiver um clone local do repositório, a sessão abre em seu diretório inicial. Veja [Escolher entre `cwd` e `repo`](#choose-between-cwd-and-repo) para saber como o caminho local é selecionado quando você tem múltiplos clones ou worktrees.

<h3 id="choose-between-cwd-and-repo">
  Escolher entre `cwd` e `repo`
</h3>

Use `cwd` quando todos que clicarem no link tiverem o projeto no mesmo caminho absoluto, como um devcontainer padronizado ou imagem de VM.

Use `repo` quando o link é compartilhado e cada pessoa clona para um local diferente. Claude Code resolve o slug para um caminho local da seguinte forma:

* Cada vez que você executa `claude` em um repositório Git, o caminho do sistema de arquivos desse diretório é registrado contra o slug `owner/name` do GitHub do repositório.
* Quando um deep link chega, `repo` abre qualquer caminho correspondente que você usou mais recentemente. Múltiplos clones e worktrees são rastreados separadamente, então ele escolhe aquele em que você trabalhou por último.
* A busca apenas encontra caminhos onde você já executou Claude Code pelo menos uma vez.
* O link não muda qual branch está verificado. A sessão abre no estado em que esse diretório está atualmente.

O cabeçalho de boas-vindas mostra qual caminho foi escolhido para que você possa confirmar que o clone correto foi aberto.

<h2 id="examples">
  Exemplos
</h2>

As seções abaixo mostram duas maneiras comuns de usar um deep link: como um link Markdown em um documento e como um comando em um script ou alias de shell.

<h3 id="embed-a-link-in-a-runbook">
  Incorporar um link em um runbook
</h3>

Um deep link em um runbook oferece a quem está triando uma maneira com um clique de começar a investigar no repositório correto com um prompt preparado. A plataforma que renderiza o runbook deve permitir esquemas de URL personalizados. O Markdown renderizado pelo GitHub não permite `claude-cli://`, então um deep link em um README, issue ou wiki do GitHub mostra apenas seu rótulo sem link clicável. Veja [a nota de troubleshooting](#the-link-renders-as-plain-text-instead-of-being-clickable) para uma solução alternativa.

O prompt faz parte da URL e deve ser URL-codificado. Para produzir o valor codificado, passe seu texto de prompt através de `encodeURIComponent` em um console do navegador ou qualquer codificador de URL.

O exemplo abaixo adiciona um ponto de entrada de investigação a um runbook de incidente para um serviço chamado `web-gateway`:

```markdown theme={null}
## High 5xx rate on web-gateway

1. Acknowledge the page in PagerDuty.
2. [Open Claude Code in the gateway repo](claude-cli://open?repo=acme/web-gateway&q=5xx%20rate%20is%20elevated%20on%20web-gateway.%20Check%20recent%20deploys%2C%20error%20logs%20from%20the%20last%2030%20minutes%2C%20and%20open%20incidents%20in%20Linear.)
3. Post initial findings in #incident.
```

Para usar isso em seu próprio runbook, substitua `acme/web-gateway` pelo slug do repositório do seu serviço. Isso permite que engenheiros com Claude Code instalado e um clone local desse repositório cliquem na etapa 2 e comecem a investigar com o prompt pronto para enviar.

<h3 id="open-a-link-from-the-shell">
  Abrir um link a partir do shell
</h3>

Você também pode abrir um deep link a partir de um script de shell, alias ou automação em vez de clicar nele. Chame o comando de abertura de URL do seu sistema operacional com o link como argumento.

<Tabs>
  <Tab title="macOS">
    O comando `open` integrado passa a URL para o handler `claude-cli://` registrado:

    ```bash theme={null}
    open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Linux">
    A maioria dos ambientes de desktop fornece `xdg-open`, que passa a URL para o handler registrado:

    ```bash theme={null}
    xdg-open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Windows">
    No PowerShell, `Start-Process` passa a URL para o handler registrado:

    ```powershell theme={null}
    Start-Process "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```

    No `cmd.exe`, `start` trata seu primeiro argumento entre aspas como um título de janela, então passe um título vazio antes da URL:

    ```cmd theme={null}
    start "" "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>
</Tabs>

<h2 id="registration-and-supported-platforms">
  Registro e plataformas suportadas
</h2>

Claude Code registra o handler `claude-cli://` com seu sistema operacional na primeira vez que você inicia uma sessão interativa no macOS, Linux e Windows. Você não executa um comando de instalação separado. O registro escreve apenas em locais de nível de usuário:

| Plataforma | Localização do Handler                                                                                               |
| ---------- | -------------------------------------------------------------------------------------------------------------------- |
| macOS      | `~/Applications/Claude Code URL Handler.app`                                                                         |
| Linux      | `claude-code-url-handler.desktop` sob `$XDG_DATA_HOME/applications`, padronizando para `~/.local/share/applications` |
| Windows    | `HKEY_CURRENT_USER\Software\Classes\claude-cli`                                                                      |

O handler inicia Claude Code em um emulador de terminal detectado. No macOS, Claude Code lembra do terminal de sua sessão interativa mais recente e o reutiliza, suportando iTerm2, Ghostty, kitty, Alacritty, WezTerm e Terminal.app. No Linux, ele honra a variável de ambiente `$TERMINAL`, depois `x-terminal-emulator`, depois uma lista de emuladores comuns. No Windows, ele prefere Windows Terminal, depois PowerShell, depois `cmd.exe`.

Para evitar o registro completamente, defina [`disableDeepLinkRegistration`](/docs/pt/settings) como `"disable"` em `settings.json`. Para impor isso em toda uma organização para que os usuários não possam reabilitá-lo, defina-o em [managed settings](/docs/pt/server-managed-settings).

<h2 id="open-a-vs-code-tab-instead-of-a-terminal">
  Abrir uma aba do VS Code em vez de um terminal
</h2>

A extensão do VS Code registra seu próprio handler em `vscode://anthropic.claude-code/open`, que abre uma aba do editor Claude Code em vez de uma janela de terminal. Veja [Launch a VS Code tab from other tools](/docs/pt/vs-code#launch-a-vs-code-tab-from-other-tools) para os parâmetros dessa URL.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="clicking-the-link-does-nothing">
  Clicar no link não faz nada
</h3>

O handler provavelmente não está registrado ainda. Inicie uma sessão `claude` interativa uma vez nessa máquina, saia e tente o link novamente. Se você estiver no Linux sem um ambiente de desktop, `xdg-open` pode não ter nada para despachar.

<h3 id="the-link-renders-as-plain-text-instead-of-being-clickable">
  O link é renderizado como texto simples em vez de ser clicável
</h3>

Alguns renderizadores Markdown apenas permitem links `http` e `https` e removem outros esquemas de URL. O GitHub faz isso em READMEs, issues, pull requests e wikis: `[label](claude-cli://...)` é renderizado como apenas `label`, sem link e a URL removida. Nessas plataformas, coloque o deep link em um bloco de código para que os leitores possam ver a URL e colá-la na barra de endereços do navegador.

<h3 id="the-session-opens-in-my-home-directory-instead-of-the-repo">
  A sessão abre em meu diretório inicial em vez do repositório
</h3>

O parâmetro `repo` apenas resolve para clones que Claude Code já viu. Execute `claude` dentro do clone uma vez para que seu caminho seja registrado, ou mude o link para usar `cwd` com um caminho absoluto.

<h3 id="the-link-opens-the-wrong-terminal">
  O link abre o terminal errado
</h3>

No macOS, inicie `claude` em seu terminal preferido uma vez e o próximo deep link o usará. No Linux, defina a variável de ambiente `$TERMINAL` para o nome do comando do seu emulador preferido. No Windows, a ordem é fixa: instale Windows Terminal se quiser que os links abram lá em vez de uma janela PowerShell ou `cmd.exe`.

<h2 id="learn-more">
  Saiba mais
</h2>

Essas páginas cobrem maneiras relacionadas de iniciar ou estender sessões Claude Code:

* [Skills](/docs/pt/skills): armazene um prompt de runbook longo como um `/skill` no repositório para que o parâmetro `q` do deep link apenas tenha que nomeá-lo
* [Non-interactive mode](/docs/pt/headless): execute Claude a partir de um script e capture a saída sem abrir um terminal
