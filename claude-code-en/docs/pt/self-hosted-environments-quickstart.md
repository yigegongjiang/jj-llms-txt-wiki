> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Guia de início rápido de ambientes auto-hospedados

> Configure seu primeiro ambiente auto-hospedado: instale Claude Code, crie o ambiente, inicie um runner e roteie uma sessão para ele.

<Note>
  Ambientes auto-hospedados estão em beta público em planos Team e Enterprise; [Disponibilidade e limitações](/docs/pt/self-hosted-environments#availability-and-limitations) cobre o caminho de habilitação. Esta página coloca sua primeira sessão em execução; consulte [Ambientes auto-hospedados](/docs/pt/self-hosted-environments) para saber o que são e [Implantar em produção](/docs/pt/self-hosted-environments-deploy) para endurecimento e receitas de frota.
</Note>

Um [ambiente auto-hospedado](/docs/pt/self-hosted-environments) executa [sessões na nuvem](/docs/pt/claude-code-on-the-web) do Claude Code em infraestrutura que sua organização opera, executado por processos runner que você implanta. Este guia de início rápido configura seu primeiro, o menor que funciona: um runner em um único host, executando uma sessão de teste. Há duas etapas: [criar o ambiente, iniciar um runner e rotear uma sessão para ele](#set-up-an-environment-and-runner), depois [enviar uma mensagem para essa sessão a partir do seu terminal](#send-a-follow-up-message-to-a-running-session). Você se moverá entre duas superfícies: claude.ai para criar o ambiente, verificar seu status e rotear uma sessão, e um terminal no host para tudo o que o runner faz.

Ao final, você terá um ambiente na [página de administração **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), um runner pesquisando por trabalho e uma sessão em execução no seu host. Antes de conectar repositórios reais ou sistemas internos, trabalhe através de [Implantar em produção](/docs/pt/self-hosted-environments-deploy), que cobre a postura de segurança, controle de egresso, credenciais git e orquestração.

<h2 id="prerequisites">
  Pré-requisitos
</h2>

<h3 id="organization-and-roles">
  Organização e funções
</h3>

O lado claude.ai precisa de:

* **Permitir ambientes auto-hospedados** ativado por um [Proprietário](/docs/pt/cloud-environments#organization-shared-environments) na [página de administração **Cloud environments**](https://claude.ai/admin-settings/cloud-environments); o botão **Novo** não aparece até que esteja. Se você não tiver a função, alguém que tiver pode criar o ambiente e passar seu segredo para você; as etapas de runner e terminal nesta página não precisam de nenhuma função claude.ai, e onde uma etapa verifica o status na interface de administração, as próprias linhas de log do runner fornecem o mesmo sinal.
* Uma [conexão GitHub](/docs/pt/claude-code-on-the-web#github-authentication-options) para sua organização, para que os desenvolvedores possam escolher repositórios quando iniciarem sessões.

<h3 id="host-and-network">
  Host e rede
</h3>

O host do runner precisa de:

* Um host ou container Linux ou macOS com HTTPS de saída para `api.anthropic.com`, para `claude.ai` e os hosts de download para os quais ele redireciona para a etapa de instalação abaixo, e para seu host git para o clone; a [tabela de requisitos de rede](/docs/pt/self-hosted-environments-deploy#network-requirements) tem a lista completa. Windows não é suportado como host de runner; execute o runner em um container Linux. Estações de trabalho de desenvolvedores não são afetadas, pois as sessões começam a partir de claude.ai em um navegador.
* Um relógio sincronizado com a hora real, por exemplo com NTP. A autenticação falha quando o relógio está mais de cinco minutos atrasado; consulte [Troubleshooting](/docs/pt/self-hosted-environments-deploy#troubleshooting).

<h3 id="software-on-the-runner-host">
  Software no host do runner
</h3>

Instale no host antes de começar:

* **Claude Code v2.1.224 ou posterior**, com qualquer um dos [métodos de instalação padrão](/docs/pt/setup). O runner faz parte do binário `claude` padrão, e versões anteriores não reconhecem o subcomando `self-hosted-runner`. O canal `latest` do instalador nativo padrão carrega cada versão assim que é publicada; o canal `stable`, o cask Homebrew `claude-code` e os repositórios apt, dnf e apk estáveis ficam para trás em cerca de uma semana. Para fixar a versão exata que sua frota executa, consulte [Instalar uma versão específica](/docs/pt/setup#install-a-specific-version). Para imagens de container, consulte o Dockerfile em [Implantar em produção](/docs/pt/self-hosted-environments-deploy#build-the-runner-image).
* **Git 2.24 ou mais recente**. Algumas opções git na página de implantação precisam de versões mais recentes; [Configurar git](/docs/pt/self-hosted-environments-deploy#configure-git) declara cada limite.

Confirme que o host está pronto:

```bash theme={null}
claude self-hosted-runner --help
```

Um host pronto imprime o texto de uso do runner, listando sinalizadores como `--environment-secret-file`. Em versões anteriores a 2.1.224, o comando imprime a saída geral `claude --help`; atualize com `claude update` ou reinstale do canal `latest`.

<h2 id="set-up-an-environment-and-runner">
  Configurar um ambiente e runner
</h2>

Claude Code inclui uma configuração guiada: uma sessão Claude Code interativa que o orienta na criação do ambiente na interface de administração, inicia um runner local com o arquivo de segredo que você salva, confirma que o runner se registra e escreve uma folha de dicas em `./runner-setup/CHEAT-SHEET.md`. Execute-o em uma máquina onde você se conectou com `claude auth login` usando uma conta que possui uma função de Proprietário; não está disponível com chaves de API ou provedores de modelo de terceiros. Em hosts onde uma sessão interativa não é possível, use as etapas manuais abaixo. Confirme que a [verificação de versão](#software-on-the-runner-host) passou primeiro: em versões anteriores a 2.1.224, este comando inicia uma sessão Claude comum com as palavras como o prompt em vez da configuração guiada. Para iniciar a configuração guiada, execute o subcomando setup e siga os prompts:

```bash theme={null}
claude self-hosted-runner setup
```

Para configurar manualmente:

<Steps>
  <Step title="Criar um ambiente">
    Vá para a [página **Cloud environments**](https://claude.ai/admin-settings/cloud-environments) nas configurações de administração. Em **Ambientes auto-hospedados**, selecione **Novo**, nomeie o ambiente e selecione **Criar**. Na segunda etapa do assistente, selecione **Copiar chave de ambiente** para copiar o segredo do ambiente, que a interface de administração rotula como uma chave de ambiente. claude.ai mostra o segredo uma vez, e você não pode recuperá-lo depois; ele expira 365 dias após a criação. O ID `ccpool_...` do ambiente permanece visível em seu diálogo de detalhes; você precisará dele para a verificação `aud` em [verificação de token](/docs/pt/self-hosted-environments-identity) e para despachar [sessões de teste a partir de CI](/docs/pt/self-hosted-environments-testing#run-the-test-loop).

    Se você perder o segredo ou precisar rotacioná-lo, crie um novo segredo na guia **Configuração** do ambiente, implante o novo segredo em seus runners e revogue o antigo. Runners que possuem um segredo revogado falham em sua próxima pesquisa autenticada e saem, registrando `poll auth failed`, e seu orquestrador os reinicia com o novo segredo.
  </Step>

  <Step title="Iniciar um runner">
    Crie o diretório de segredo. Esta etapa e a próxima precisam de root para o caminho `/etc/claude`; qualquer caminho que o processo runner possa ler funciona, então ajuste ambos os comandos e o valor `--environment-secret-file` juntos se você usar um diferente.

    ```bash theme={null}
    mkdir -p /etc/claude
    ```

    Escreva o segredo do ambiente em um arquivo. O comando abaixo lê do seu terminal para que o segredo fique fora do histórico do shell: cole o valor que você copiou, pressione Enter, depois Ctrl-D, e o `umask` do subshell torna o arquivo legível apenas por seu proprietário.

    ```bash theme={null}
    (umask 077 && cat > /etc/claude/environment-secret)
    ```

    Escolha um diretório base, substituindo `<writable-dir>` no comando runner abaixo por um caminho absoluto que o runner possa escrever ou criar. O runner cria o diretório na inicialização, depois verifica repositórios e cria diretórios por sessão sob ele. Sem `--base-dir` ele usa `/workspace`, que só funciona se esse diretório já existe e é gravável ou você inicia o runner como root.

    Se o runner não conseguir criar ou escrever no caminho, ele sai na inicialização com um erro nomeando o diretório em vez de se registrar. Consulte [Troubleshooting](/docs/pt/self-hosted-environments-deploy#troubleshooting).

    Depois inicie o runner com `--environment-secret-file` e `--base-dir`. O runner se registra com seu ambiente e começa a pesquisar por trabalho. Se o runner sair, reinicie-o manualmente. Implantações de produção executam o runner sob um orquestrador que reinicia runners que saíram, normalmente com um sistema de arquivos fresco por reinicialização; [Reutilizar um checkout pré-aquecido](/docs/pt/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout) cobre a configuração de disco persistente suportada.

    ```bash theme={null}
    claude self-hosted-runner --environment-secret-file '/etc/claude/environment-secret' --base-dir '<writable-dir>'
    ```
  </Step>

  <Step title="Verificar se o runner aparece">
    Retorne à [página **Cloud environments**](https://claude.ai/admin-settings/cloud-environments). O status do seu ambiente muda de **Nenhum runner implantado** para **Saudável** em alguns segundos após o runner iniciar; abra o ambiente e selecione **Atividade** para ver o runner em si.
  </Step>

  <Step title="Rotear uma sessão para o ambiente">
    Inicie uma sessão em claude.ai/code e selecione seu ambiente no seletor de ambiente, onde ambientes auto-hospedados aparecem ao lado dos hospedados pela Anthropic. O runner clona com quaisquer credenciais git que o host já tenha, então escolha um repositório que este host já possa clonar, ou um público; as opções de credencial para repositórios privados em produção estão em [Configurar git](/docs/pt/self-hosted-environments-deploy#configure-git). O próximo runner disponível pega a sessão enfileirada e registra `Picked up session <session-id>` junto com sua contagem ativa e capacidade, para que você possa confirmar a partir da própria saída do runner qual host pegou a sessão. Observe a sessão funcionar e leia as respostas do Claude em [claude.ai/code](https://claude.ai/code). Se a sessão ficar enfileirada, consulte [Troubleshooting](/docs/pt/self-hosted-environments-deploy#troubleshooting).
  </Step>
</Steps>

O runner sai por design uma vez que suas sessões ativas terminam; consulte [Ciclo de vida do runner](/docs/pt/self-hosted-environments#runner-lifecycle). Para produção, implante-o sob um orquestrador que o reinicia na saída. Consulte [Implantar em produção](/docs/pt/self-hosted-environments-deploy).

<h2 id="send-a-follow-up-message-to-a-running-session">
  Enviar uma mensagem de acompanhamento para uma sessão em execução
</h2>

Depois que uma sessão está em execução no seu ambiente, envie um acompanhamento a partir do CLI `claude` em qualquer máquina onde você esteja conectado com `claude auth login`; o comando não precisa ser executado a partir da máquina que iniciou a sessão. O comando publica uma mensagem:

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

Para `<session-id>`, passe o ID `session_...` ou `cse_...` simples ou a URL claude.ai/code da sessão. Um envio bem-sucedido imprime `Sent to cloud session.` com o ID da sessão e um link de visualização. Formas de ID aceitas, saída JSON, requisitos de conta e política e a referência de erro estão em [Enviar acompanhamentos a partir do CLI](/docs/pt/claude-code-on-the-web#send-follow-ups-from-the-cli), pois o comando funciona da mesma forma contra sessões hospedadas pela Anthropic.

<h2 id="what’s-next">
  Próximos passos
</h2>

* [Implantar em produção](/docs/pt/self-hosted-environments-deploy): endureça a implantação, controle o egresso, configure credenciais git e execute a frota sob Kubernetes ou Compose
* [Personalizar sessões](/docs/pt/self-hosted-environments-configuration): scripts wrapper, hooks de ciclo de vida, runners sob demanda, servidores MCP e permissões
* [Testar de ponta a ponta](/docs/pt/self-hosted-environments-testing): um teste de fumaça de CI que despacha uma sessão e lê as respostas do Claude
