> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code Desktop em WSL

> Execute sessões de Code dentro de uma distribuição WSL 2 no Windows

No Windows, a aba Code pode executar uma sessão dentro de uma distribuição WSL 2 em vez de no próprio Windows. O processo Claude Code da sessão, suas ferramentas e git são todos executados dentro da distribuição, usando sua cadeia de ferramentas Linux e caminhos nativos do Linux, o mesmo ambiente que seu projeto visa.

Use uma sessão WSL quando seu repositório reside dentro do sistema de arquivos da distribuição. Trabalhar nesses arquivos a partir do Windows passa por um sistema de arquivos de rede, que é lento e quebra a observação de arquivos; executar a sessão dentro da distribuição evita ambos.

<h2 id="requirements">
  Requisitos
</h2>

* Windows 10 ou 11 com [WSL 2](https://learn.microsoft.com/windows/wsl/install). WSL 1 não é suportado.
* Pelo menos uma distribuição instalada (por exemplo, Ubuntu).
* `git` instalado dentro da distribuição.

<h2 id="start-a-wsl-session">
  Iniciar uma sessão WSL
</h2>

<Steps>
  <Step title="Escolha uma distribuição">
    Inicie uma nova sessão na aba Code e abra o seletor de ambiente. Suas distribuições WSL 2 instaladas aparecem em uma seção **WSL**. Escolha uma.
  </Step>

  <Step title="Escolha uma pasta">
    A sessão inicia no diretório inicial da distribuição. Use o seletor de pasta para escolher uma pasta de projeto. A navegação acontece dentro da distribuição, com caminhos Linux como `/home/you/project`.
  </Step>

  <Step title="Confie na pasta">
    A primeira sessão em uma pasta mostra o diálogo de confiança do espaço de trabalho. A confiança é concedida por distribuição e pasta; confiar em uma pasta em uma distribuição não se aplica a outra distribuição ou ao mesmo caminho no Windows.
  </Step>
</Steps>

A primeira sessão em uma distribuição leva um pouco mais de tempo enquanto Claude se configura dentro dela. Você também pode abrir uma pasta `\\wsl.localhost\...` do seletor de pasta normal, e ela reabre dentro dessa distribuição.

As pastas que você usou recentemente aparecem no seletor por distribuição, portanto reconectar a um projeto é um clique.

<h2 id="what-works-in-a-wsl-session">
  O que funciona em uma sessão WSL
</h2>

Sessões paralelas, chats laterais, revisão de diff visual, status de branch e pull request, e worktrees funcionam todos, apoiados pelo git e cadeia de ferramentas dentro da distribuição. "Abrir no editor" abre VS Code conectado à distribuição através de [Remote - WSL](https://code.visualstudio.com/docs/remote/wsl).

Alguns recursos ainda não estão disponíveis em sessões WSL: o terminal integrado, conectores e plugins, bifurcação de sessão, o painel do navegador de arquivos e sugestões de arquivo quando você digita `@` no compositor.

<h2 id="managed-devices">
  Dispositivos gerenciados
</h2>

Em dispositivos gerenciados por uma organização, as sessões WSL podem estar indisponíveis. Se o início da sessão falhar com uma mensagem de que o dispositivo é gerenciado, isso é controlado pelo seu administrador. Administradores: consulte [como as configurações chegam aos dispositivos](/docs/pt/admin-setup#decide-how-settings-reach-devices) no guia de implantação.
