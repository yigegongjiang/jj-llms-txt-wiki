> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Desktop no Linux (beta)

> Instale e atualize o aplicativo desktop Claude no Ubuntu e Debian

<Note>
  O suporte ao Linux para o aplicativo desktop Claude está em beta. As abas Chat, Cowork e Code estão todas disponíveis.
</Note>

O aplicativo desktop no Linux oferece a mesma experiência de Chat, Cowork e Claude Code que macOS e Windows: sessões paralelas, revisão de diff visual, um terminal e editor integrados e visualização ao vivo do aplicativo. Consulte [Usar Claude Code Desktop](/docs/pt/desktop) para a referência completa de recursos.

<h2 id="requirements">
  Requisitos
</h2>

* Ubuntu 22.04 ou posterior, ou Debian 12 ou posterior
* x86\_64 ou arm64

Outras distribuições baseadas em Debian que atendem a esses requisitos podem funcionar, mas não são oficialmente testadas.

<h2 id="install">
  Instalar
</h2>

Instale a partir do repositório apt da Anthropic para que as atualizações cheguem através das atualizações regulares de pacotes do seu sistema. Abra um terminal e execute os comandos em cada etapa.

<Steps>
  <Step title="Adicionar o repositório apt da Anthropic">
    Esta etapa baixa a chave de assinatura com `curl`, que instalações recentes de Debian e Ubuntu podem não incluir. Se o comando de download falhar com `sudo: curl: command not found`, instale curl primeiro:

    ```bash theme={null}
    sudo apt install curl
    ```

    Baixe a chave de assinatura da Anthropic:

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

    Registre o repositório:

    ```bash theme={null}
    echo "deb [arch=amd64,arm64 signed-by=/usr/share/keyrings/claude-desktop-archive-keyring.asc] https://downloads.claude.ai/claude-desktop/apt/stable stable main" | sudo tee /etc/apt/sources.list.d/claude-desktop.list
    ```
  </Step>

  <Step title="Instalar o pacote">
    ```bash theme={null}
    sudo apt update && sudo apt install claude-desktop
    ```
  </Step>

  <Step title="Iniciar e fazer login">
    Inicie **Claude** a partir do seu inicializador de aplicativos, ou execute `claude-desktop` a partir de um terminal, e faça login com sua conta Anthropic.

    O aplicativo Linux faz login da mesma forma que no macOS e Windows: com uma assinatura claude.ai, ou através do SSO da sua organização. O Desktop não aceita uma chave de API do Claude Console diretamente; use a [CLI](/docs/pt/quickstart) para autenticação com chave de API. Para implantações empresariais que roteiam o Desktop para a Agent Platform do Google Cloud ou um gateway LLM, consulte [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) e [configuração de rede](/docs/pt/network-config).
  </Step>
</Steps>

<Accordion title="Verificar a chave de assinatura">
  Você pode confirmar que a chave de assinatura baixada pertence à Anthropic:

  ```bash theme={null}
  gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
  ```

  A impressão digital deve ser `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE`.
</Accordion>

<h3 id="install-from-a-downloaded-file">
  Instalar a partir de um arquivo baixado
</h3>

Se você não conseguir instalar através do repositório apt, baixe o pacote `.deb` diretamente do pool de pacotes do repositório. Este comando procura o pacote mais recente para sua arquitetura no índice do repositório e o baixa para o diretório atual:

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

Se o comando falhar com `Remote file name has no length`, a pesquisa não retornou nenhum caminho de pacote. Isso pode significar que o índice do repositório não pôde ser obtido, por exemplo quando sua rede bloqueia `downloads.claude.ai`, ou que nenhum pacote existe para sua arquitetura. Confirme que sua rede pode alcançar `downloads.claude.ai` e que `dpkg --print-architecture` imprime `amd64` ou `arm64`; o repositório não publica pacotes para outras arquiteturas.

Em seguida, abra o arquivo baixado com seu instalador de software, como GNOME Software, ou instale-o com apt a partir do diretório que contém o arquivo baixado:

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

Se apt relatar `E: Unsupported file ./claude-desktop_*.deb given on commandline`, o padrão não correspondeu a um arquivo `.deb` no diretório atual. Confirme que o download foi concluído e execute o comando novamente a partir do diretório que contém o arquivo.

Um `.deb` instalado desta forma não recebe atualizações. Para obter atualizações através do apt, registre o repositório a partir da etapa [Adicionar o repositório apt da Anthropic](#install). O pacote também escreve uma entrada de repositório comentada em `/etc/apt/sources.list.d/claude-desktop.list`; descomentando sua linha `deb` é equivalente.

<h2 id="update">
  Atualizar
</h2>

O aplicativo desktop não se atualiza automaticamente no Linux. As atualizações chegam com as atualizações regulares de pacotes do seu sistema:

```bash theme={null}
sudo apt update && sudo apt upgrade
```

O atualizador de software gráfico da sua distribuição também detectará novas versões.

<h2 id="uninstall">
  Desinstalar
</h2>

```bash theme={null}
sudo apt remove claude-desktop
```

Isso remove a chave de assinatura junto com o aplicativo, portanto, se você adicionou a entrada do repositório durante a instalação, remova-a também:

```bash theme={null}
sudo rm /etc/apt/sources.list.d/claude-desktop.list
```

<h2 id="troubleshoot">
  Troubleshooting
</h2>

<h3 id="unable-to-locate-package-claude-desktop">
  Não é possível localizar o pacote claude-desktop
</h3>

Se `sudo apt install claude-desktop` falhar com `E: Unable to locate package claude-desktop`, o apt não encontrou o repositório que você adicionou. Verifique o seguinte:

* Confirme que a entrada do repositório foi escrita. `cat /etc/apt/sources.list.d/claude-desktop.list` deve mostrar a linha `deb` da etapa [Adicionar repositório apt da Anthropic](#install). Se o arquivo estiver vazio ou ausente, execute essa etapa novamente.
* Confirme que sua arquitetura é suportada. `dpkg --print-architecture` deve imprimir `amd64` ou `arm64`. O repositório não publica pacotes para outras arquiteturas.
* Execute `sudo apt update` novamente e verifique sua saída para erros relacionados a `downloads.claude.ai`. Um erro de rede ou chave lá significa que o repositório foi adicionado, mas não pôde ser alcançado ou verificado.

Se o repositório estiver em vigor e acessível e o pacote ainda não for encontrado, [instale a partir de um arquivo baixado](#install-from-a-downloaded-file).

<h2 id="what’s-not-in-the-linux-beta-yet">
  O que ainda não está no beta do Linux
</h2>

* **Computer Use**: [controle de aplicativo e tela](/docs/pt/desktop#let-claude-use-your-computer) não está disponível no Linux.
* **Dictation**: entrada de voz não está disponível no aplicativo desktop Linux. Use [ditado por voz](/docs/pt/voice-dictation) na CLI em vez disso.
* **Quick Entry global hotkey**: funciona no X11. No Wayland nativo, requer o portal GlobalShortcuts do seu ambiente de desktop.
* **Fedora e RHEL**: apenas distribuições baseadas em Debian são suportadas atualmente. O suporte para distribuições adicionais virá no futuro.

Para qualquer coisa ainda não disponível no aplicativo desktop, a [CLI](/docs/pt/quickstart) executa o mesmo mecanismo Claude Code e suporta uma gama mais ampla de distribuições Linux; consulte os [requisitos do sistema](/docs/pt/setup#system-requirements).
