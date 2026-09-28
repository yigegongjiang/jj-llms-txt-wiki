> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Desktop no Linux (beta)

> Instale e atualize o aplicativo desktop Claude no Ubuntu e Debian

<Note>
  O suporte ao Linux para o aplicativo desktop Claude está em beta.
</Note>

O aplicativo desktop no Linux oferece a mesma experiência de Chat, Cowork e Claude Code que no macOS e Windows: sessões paralelas, revisão de diff visual, um terminal e editor integrados e visualização ao vivo do aplicativo. Consulte [Usar Claude Code Desktop](/docs/pt/desktop) para a referência de recursos.

<h2 id="requirements">
  Requisitos
</h2>

* Uma distribuição baseada em Debian: Ubuntu 22.04 ou posterior, ou Debian 12 ou posterior
* x86\_64 ou arm64

Outras distribuições baseadas em Debian que atendem a esses requisitos podem funcionar, mas não são oficialmente testadas. Em distribuições que não são baseadas em Debian, como Fedora ou Arch, execute a [CLI](/docs/pt/setup#system-requirements) em vez disso. Se você trabalha no Windows com WSL 2, instale o aplicativo de desktop do Windows e execute sessões dentro de sua distribuição; consulte [Claude Code Desktop em WSL](/docs/pt/desktop-wsl).

<h3 id="cowork-requirements">
  Requisitos do Cowork
</h3>

Cowork é a aba de desktop para [Dispatch e trabalho agentic mais longo](https://claude.com/docs/cowork/overview). No Linux, Cowork executa essas tarefas em uma máquina virtual que o aplicativo de desktop hospeda com QEMU e KVM. Para usar Cowork, sua máquina precisa de:

* **Virtualização de hardware**: ativada nas configurações do firmware. Sem ela, a aba Cowork relata "Cowork requer virtualização de hardware (KVM)".
* **Firmware QEMU e UEFI**: `qemu-system-x86`, `ovmf` e `virtiofsd` em x86\_64, ou `qemu-system-arm`, `qemu-efi-aarch64` e `virtiofsd` em arm64. `apt install claude-desktop` os instala por padrão como pacotes recomendados. Se você instalou com `--no-install-recommends`, ou seu sistema é uma imagem mínima que pula pacotes recomendados, a aba Cowork relata "Cowork requer QEMU" e mostra o comando `apt install` a executar. Ubuntu 22.04 não possui pacote `virtiofsd`; o aplicativo usa uma cópia agrupada lá.
* **Acesso a `/dev/kvm`**: adicione seu usuário ao grupo `kvm` com `sudo usermod -aG kvm $USER`, depois faça logout e login novamente. Alguns ambientes de desktop concedem ao usuário conectado acesso a `/dev/kvm` sem o grupo, mas Cowork também precisa de `/dev/vhost-vsock`, que apenas membros do grupo `kvm` podem abrir. Junte-se ao grupo mesmo se `/dev/kvm` já funcionar para você.

O aplicativo verifica esses requisitos uma vez no lançamento: reinicie-o após instalar pacotes e faça logout e login novamente após ingressar no grupo. Se `/dev/vhost-vsock` estiver faltando e seu kernel em execução não tiver diretório de módulo em `/lib/modules`, a aba Cowork relata que o kernel não inclui o suporte de virtualização que Cowork precisa e que não pode ser adicionado manualmente. Essa combinação é comum no ChromeOS e em ambientes Linux baseados em contêiner.

<h2 id="install">
  Instalar
</h2>

Instale a partir do repositório apt da Anthropic para que as atualizações cheguem através das atualizações regulares de pacotes do seu sistema. Abra um terminal e execute os comandos em cada etapa.

<Steps>
  <Step title="Adicionar o repositório apt da Anthropic">
    Esta etapa baixa a chave de assinatura com `curl` e a verifica com `gpg`, que instalações recentes de Debian e Ubuntu podem não incluir. Se algum comando relatar `command not found`, instale ambos primeiro:

    ```bash theme={null}
    sudo apt install curl gnupg
    ```

    Baixe a chave de assinatura da Anthropic:

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

    O comando não imprime nada quando tem sucesso e um erro `curl:` quando não tem. Uma chave ausente ou incorreta faz `apt update` falhar mais tarde com `NO_PUBKEY BAA929FF1A7ECACE`, então confirme que a chave foi baixada e pertence à Anthropic antes de continuar:

    ```bash theme={null}
    gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
    ```

    A impressão digital que gpg imprime deve ser `31DDDE24DDFAB679F42D7BD2BAA929FF1A7ECACE`. Se gpg relatar que o arquivo não pode ser aberto ou não contém dados OpenPGP válidos, o download falhou ou retornou o conteúdo errado: confirme que sua rede pode alcançar `downloads.claude.ai` e execute novamente o comando de download.

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

<h3 id="install-from-a-downloaded-file">
  Instalar a partir de um arquivo baixado
</h3>

Se você não conseguir instalar através do repositório apt, baixe o pacote `.deb` diretamente do pool de pacotes do repositório. Este comando procura o pacote mais recente para sua arquitetura no índice do repositório e o baixa para o diretório atual:

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

Se o comando falhar com `Remote file name has no length`, a pesquisa não retornou nenhum caminho de pacote. Isso pode significar que o índice do repositório não pôde ser obtido, por exemplo quando sua rede bloqueia `downloads.claude.ai`, ou que nenhum pacote existe para sua arquitetura. Confirme que sua rede pode alcançar `downloads.claude.ai` e que `dpkg --print-architecture` imprime `amd64` ou `arm64`; o repositório não publica pacotes para outras arquiteturas.

Para instalar sem registrar o repositório apt da Anthropic, primeiro crie `/etc/default/claude-desktop` com a linha `CLAUDE_DESKTOP_ADD_REPO="false"`. Sem o repositório, apt não entrega novas versões; para atualizar, execute novamente o comando de download e reinstale, ou [registre o repositório](#install) mais tarde.

Em seguida, abra o arquivo baixado com seu instalador de software, como GNOME Software, ou instale-o com apt a partir do diretório que contém o arquivo baixado:

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

Se apt relatar `E: Unsupported file ./claude-desktop_*.deb given on commandline`, o padrão não correspondeu a um arquivo `.deb` no diretório atual. Confirme que o download foi concluído e execute o comando novamente a partir do diretório que contém o arquivo.

Instalar o `.deb` também registra o repositório apt da Anthropic em `/etc/apt/sources.list.d/claude-desktop.list`, para que as atualizações futuras cheguem com as [atualizações regulares de pacotes](#update) do seu sistema.

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

Desinstalar o pacote também remove a entrada do repositório e a chave de assinatura que ele registrou. Se você adicionou a entrada do repositório você mesmo com a etapa [Adicionar repositório apt da Anthropic](#install), remova-a também:

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

* Execute `sudo apt update` após adicionar o repositório. `apt install` por si só não vê um repositório que você adicionou após a última vez que executou `apt update`.
* Confirme que a entrada do repositório foi escrita. `cat /etc/apt/sources.list.d/claude-desktop.list` deve mostrar a linha `deb` da etapa [Adicionar repositório apt da Anthropic](#install). Se o arquivo estiver vazio ou ausente, execute essa etapa novamente.
* Confirme que sua arquitetura é suportada. `dpkg --print-architecture` deve imprimir `amd64` ou `arm64`. O repositório não publica pacotes para outras arquiteturas.
* Execute `sudo apt update` novamente e verifique sua saída para erros relacionados a `downloads.claude.ai`. Um erro de rede ou chave lá significa que o repositório foi adicionado, mas não pôde ser alcançado ou verificado.

Se o repositório estiver em vigor e acessível e o pacote ainda não for encontrado, [instale a partir de um arquivo baixado](#install-from-a-downloaded-file).

<h3 id="unmet-dependencies">
  Dependências não atendidas
</h3>

Se `apt` parar com `The following packages have unmet dependencies` ou `Unsatisfied dependencies`, leia qual dependência ele nomeia:

* `libc6 (>= 2.34)`: sua distribuição é mais antiga do que o pacote suporta. Ubuntu 20.04 fornece `libc6` 2.31. Atualize para Ubuntu 22.04 ou posterior, ou Debian 12 ou posterior.
* Todas as dependências ausentes mostram `not installable` com um sufixo `:amd64` ou `:arm64`: você baixou o `.deb` para uma arquitetura diferente da sua máquina. Execute `dpkg --print-architecture` e baixe o `.deb` correspondente, ou [instale a partir do repositório apt](#install), que seleciona o pacote para sua arquitetura.

<h3 id="running-as-root-without-no-sandbox-is-not-supported">
  Executar como root sem --no-sandbox não é suportado
</h3>

Se `claude-desktop` sair com esta mensagem, você o iniciou como root. Faça login como um usuário regular e inicie-o a partir daí.

<h3 id="cowork-isn’t-available">
  Cowork não está disponível
</h3>

Se a aba Cowork mostrar uma dessas mensagens, corrija o requisito que ela nomeia e reinicie o aplicativo:

* **Cowork requer QEMU**: instale os [pacotes QEMU e firmware UEFI](#cowork-requirements) que a mensagem lista.
* **Cowork requer virtualização de hardware (KVM)**: ative a [virtualização de hardware](#cowork-requirements) nas configurações do seu firmware.
* **Claude não tem permissão para usar virtualização (/dev/kvm)**: adicione seu usuário ao [grupo `kvm`](#cowork-requirements), depois faça logout e login novamente.
* **Cowork requer o módulo kernel `vhost_vsock`**: execute `sudo modprobe vhost_vsock`, depois reinicie o aplicativo. Isso carrega o módulo apenas para a inicialização atual. Para carregá-lo em cada inicialização, execute `echo vhost_vsock | sudo tee /etc/modules-load.d/vhost_vsock.conf`.

<h2 id="what’s-not-in-the-linux-beta-yet">
  O que ainda não está no beta do Linux
</h2>

* **Computer Use**: [controle de aplicativo e tela](/docs/pt/desktop#let-claude-use-your-computer) não está disponível no Linux.
* **Dictation**: entrada de voz não está disponível no aplicativo desktop Linux. Use [ditado por voz](/docs/pt/voice-dictation) na CLI em vez disso.
* **Quick Entry global hotkey**: funciona no X11. No Wayland nativo, requer o portal GlobalShortcuts do seu ambiente de desktop.
* **Fedora e RHEL**: apenas distribuições baseadas em Debian são suportadas atualmente. O suporte para distribuições adicionais virá no futuro.

Para qualquer coisa ainda não disponível no aplicativo desktop, a [CLI](/docs/pt/quickstart) executa o mesmo mecanismo Claude Code e suporta uma gama mais ampla de distribuições Linux; consulte os [requisitos do sistema](/docs/pt/setup#system-requirements).
