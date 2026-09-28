> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Executar Claude Code atrás de um launcher corporativo

> Rotear os processos que Claude Code inicia a partir de seu próprio binário, incluindo o serviço de fundo e cada sessão de visualização de agente, através de um launcher obrigatório com CLAUDE_CODE_PROCESS_WRAPPER ou a configuração processWrapper.

Algumas organizações exigem que cada processo em uma estação de trabalho seja iniciado através de um launcher obrigatório. O launcher aplica a sandbox, controles de rede ou injeção de credenciais das quais a postura de segurança da empresa depende, e um binário que inicia sem isso é uma violação de política.

`CLAUDE_CODE_PROCESS_WRAPPER` inicia cada processo que Claude Code lança a partir de seu próprio binário através do seu launcher: o serviço de fundo, cada sessão que hospeda em [agent view](/docs/pt/agent-view), e os relançamentos do Claude Code após uma atualização. Defina-o como o caminho absoluto do seu launcher, e Claude Code executa o launcher com o comando Claude Code como seus argumentos.

Um launcher que envolve o comando `claude` no seu `PATH` não consegue alcançar o serviço de fundo ou as sessões que ele hospeda, porque eles iniciam a partir do caminho direto do binário sem consultar `claude`.

<Note>
  `CLAUDE_CODE_PROCESS_WRAPPER` requer Claude Code v2.1.208 ou posterior. Versões anteriores ignoram a variável e iniciam cada processo sem envolvimento. A configuração equivalente [`processWrapper`](/docs/pt/settings-reference#processwrapper) requer v2.1.210 ou posterior. Versões anteriores a ignoram como uma chave desconhecida, não aplicam nenhum launcher e não relatam nenhum erro.

  Após implantar qualquer uma das formas, use a etapa [Verificar](#set-up-the-launcher) para confirmar que a versão em execução a aplica.
</Note>

<h2 id="what-the-launcher-covers">
  O que o launcher cobre
</h2>

Com `CLAUDE_CODE_PROCESS_WRAPPER` definido, Claude Code inicia cada um dos seguintes processos através do seu launcher:

* O serviço de fundo que `claude agents` e sessões de fundo iniciam sob demanda.
* O host do terminal e a sessão Claude Code dentro de cada linha de agent view, incluindo as sessões de espera quente que o serviço mantém prontas.
* Sessões que o serviço reinicia após uma atualização ou falha.
* O relançamento que Claude Code realiza de si mesmo para terminar de instalar uma atualização, incluindo a ação restart-for-update do agent view.
* Os processos de sessão que [Remote Control](/docs/pt/remote-control) inicia. Requer Claude Code v2.1.210 ou posterior.
* As sessões de teammate em split-pane que [agent teams](/docs/pt/agent-teams) iniciam em tmux ou iTerm2. Os painéis de teammate são interativos em vez de processos de fundo, mas Claude Code os inicia a partir de seu próprio binário, então o launcher os cobre. Requer Claude Code v2.1.210 ou posterior.

No Windows, a variável é ignorada: o contrato do launcher depende de `exec`, que Windows não suporta. Uma máquina Windows com a variável definida executa cada processo sem envolvimento e continua funcionando, e o único sinal é um aviso no [debug log](/docs/pt/troubleshooting). Se sua política de launcher cobre Windows, a variável não a satisfaz lá: conte máquinas Windows como sem envolvimento quando você planejar o lançamento.

<h3 id="processes-that-start-outside-the-launcher">
  Processos que iniciam fora do launcher
</h3>

Os seguintes processos não iniciam através do launcher:

* Um [serviço de fundo instalado](/docs/pt/agent-view#the-supervisor-process) cuja unidade foi escrita antes do launcher ser configurado: `launchd` ou `systemd` inicia esse processo a partir de seu arquivo de unidade. `/status` e `claude daemon status` avisam quando o serviço em execução e o launcher configurado não correspondem, e as sessões que o serviço gera ainda iniciam através do launcher uma vez que o serviço reinicia com a variável em suas configurações.
* Uma sessão que você inicia você mesmo em um terminal, que executa da forma como você a invocou. Para cobrir essas sessões, coloque um script chamado `claude` em um diretório anterior no `PATH` que executa seu launcher com o binário real; não substitua o symlink gerenciado. O serviço de fundo e suas sessões iniciam sem uma consulta `PATH`, então os dois launchers não se empilham lá.
* O primeiro processo de um deep link `claude-cli://`, que o manipulador de protocolo do sistema operacional inicia diretamente. Tudo que essa sessão inicia em segundo plano depois executa através do launcher. Para fechar esse caminho completamente, [impeça o registro do manipulador](/docs/pt/deep-links#registration-and-supported-platforms) com a configuração `disableDeepLinkRegistration`.
* O relançamento que `--worktree` combinado com `--tmux` realiza: o multiplexador de terminal inicia esse painel, não o binário do Claude Code.
* O host de native-messaging que [Claude in Chrome](/docs/pt/chrome) registra: o navegador o inicia, não o binário do Claude Code.

<h3 id="helper-process-names-in-process-monitors">
  Nomes de processos auxiliares em monitores de processos
</h3>

Com um launcher configurado, `ps` e Activity Monitor não mostram mais os rótulos `claude bg-pty-host` e `claude bg-spare` do Claude Code para os processos auxiliares de fundo, porque o `exec` do launcher reconstrói a lista de argumentos. Perder os rótulos é um efeito colateral, não ocultação: os processos são de outra forma inalterados, e Claude Code identifica seus próprios processos pelo caminho do binário, nunca pelo nome de exibição.

<h2 id="set-up-the-launcher">
  Configure o launcher
</h2>

<Steps>
  <Step title="Escreva o script do launcher">
    Crie um script executável em um caminho absoluto, como `/opt/corp/launcher`. Claude Code o executa com o comando Claude Code completo como seus argumentos, e o script deve terminar chamando `exec "$@"` para que se substitua pelo Claude Code:

    ```bash theme={null}
    #!/bin/sh
    # Configuração da sua organização: entre na sandbox, aplique
    # controles de rede ou injete credenciais.
    exec "$@"
    ```

    Torne-o executável com `chmod +x`. A porção de configuração é o que seu launcher deve fazer antes de Claude Code executar; [o contrato do launcher](#the-launcher-contract) abaixo lista as regras que o script deve seguir.

    <Note>
      Se você substituiu anteriormente o symlink `~/.local/bin/claude` pelo seu launcher, restaure o symlink original na mesma mudança. Um symlink substituído faz a primeira sessão envolvida iniciar o serviço de fundo através de ambos os launchers de uma vez, e coloca a instalação em um estado gerenciado externamente: `/doctor` relata isso, auto-update deixa o arquivo no lugar, e limpeza de versões antigas permanece desabilitada até que o instalador gerencie esse caminho novamente.
    </Note>
  </Step>

  <Step title="Defina CLAUDE_CODE_PROCESS_WRAPPER nas configurações">
    Defina a variável no bloco `env` de um arquivo de configurações para que o serviço de fundo desanexado a herde. Um `export` de shell não é suficiente: o serviço de fundo inicia sob demanda, sobrevive ao seu shell, e nunca relê perfis de shell.

    Para uma máquina, adicione-o a `~/.claude/settings.json`. Para implantá-lo em cada máquina da sua organização, coloque o mesmo bloco em [managed settings](/docs/pt/managed-settings):

    ```json theme={null}
    {
      "env": {
        "CLAUDE_CODE_PROCESS_WRAPPER": "/opt/corp/launcher"
      }
    }
    ```

    Quando mais de uma fonte define a variável, o valor de managed settings substitui tanto `~/.claude/settings.json` quanto um valor exportado no shell, para que os usuários não possam apontar self-spawns para um launcher diferente.

    A configuração [`processWrapper`](/docs/pt/settings-reference#processwrapper) carrega o mesmo valor que uma chave de configurações nomeada e de nível superior. Defina-a quando sua organização envia configurações como chaves individuais em vez de um bloco `env`. A configuração `processWrapper` requer Claude Code v2.1.210 ou posterior. O arquivo de configurações a seguir define o mesmo launcher através da chave:

    ```json theme={null}
    {
      "processWrapper": "/opt/corp/launcher"
    }
    ```

    `CLAUDE_CODE_PROCESS_WRAPPER` tem precedência quando ambas estão definidas.

    Como `processWrapper` é uma configuração nomeada, uma organização que a entrega através de [managed settings remotas](/docs/pt/managed-settings#delivery-mechanisms) a vê listada no [diálogo de aprovação de segurança](/docs/pt/server-managed-settings#security-approval-dialogs) ao lado das outras configurações que executam executáveis fornecidos pelo administrador.

    Configurações de projeto e local não podem configurar o launcher. Um arquivo confirmado em um repositório não deve ser capaz de colocar um binário na frente de cada processo Claude Code na máquina, então Claude Code ignora `CLAUDE_CODE_PROCESS_WRAPPER` em `.claude/settings.json` ou `.claude/settings.local.json` com um aviso no [debug log](/docs/pt/troubleshooting), e nunca lê a chave `processWrapper` desses arquivos.
  </Step>

  <Step title="Reinicie o serviço de fundo e suas sessões">
    Um serviço de fundo em execução e quaisquer sessões `claude` abertas leem a variável uma vez na inicialização, então continuam lançando processos sem envolvimento até serem reiniciados. Execute `claude daemon stop --any` para parar o serviço sob demanda; o próximo comando que o necessita, como `claude agents`, inicia um envolvido. Um [serviço instalado](/docs/pt/agent-view#the-supervisor-process) leva `claude daemon stop` sem `--any`. Então reinicie suas sessões `claude` abertas.

    Em máquinas que você não pode reiniciar manualmente, a primeira sessão iniciada após o push de configurações aposenta um serviço sob demanda sem envolvimento restante automaticamente. Uma máquina onde nenhuma nova sessão inicia mantém seu serviço sem envolvimento até que uma o faça, e um serviço instalado sempre precisa do reinício nesta etapa.
  </Step>

  <Step title="Verifique">
    Execute `/status` em uma sessão: a entrada Self-exec mostra o comando de lançamento resolvido e avisa quando o serviço de fundo em execução não corresponde a ele. `claude daemon status` imprime as mesmas informações do shell, incluindo depois que você desdefine a variável, quando `/status` não mostra mais a entrada.
  </Step>
</Steps>

<h2 id="the-launcher-contract">
  O contrato do launcher
</h2>

Quando o launcher não consegue executar, Claude Code recusa iniciar o processo em vez de iniciá-lo sem envolvimento. No Windows, [a variável é ignorada](#what-the-launcher-covers) e os processos iniciam sem envolvimento. Claude Code mantém o script a essas regras:

* **Termine com `exec "$@"`.** Um launcher que bifurca um filho e sai deixa um processo Claude Code órfão que o serviço de fundo não consegue rastrear. Agent view marca tal sessão como falha com uma mensagem nomeando o launcher, e o serviço colhe o que o launcher deixou para trás.
* **Não reordene, absorva ou antecipe argumentos.** O primeiro argumento é o binário Claude Code e tudo depois dele é seu argv.
* **Passe cada variável de ambiente herdada através para `exec`.** Adicionar variáveis, como credenciais injetadas, é bom; descartar as herdadas não é.
  * Os tokens de autenticação por sessão, a seleção de modelo e provedor, e `CLAUDE_CODE_PROCESS_WRAPPER` em si viajam no ambiente herdado, então um launcher que o reconstrói a partir de uma lista de permissões quebra as sessões que inicia, e `/status` relata uma incompatibilidade de launcher.
  * Se o launcher deve entrar em um namespace ou sandbox que redefine o ambiente, re-exporte o ambiente herdado dentro dele verbatim.
* **Alcance `exec` dentro de cerca de três segundos cada vez que o launcher executa.** Uma expedição de fundo fria executa o launcher duas vezes em série antes do primeiro byte de saída, então faça trabalho lento como uma troca de single sign-on preguiçosamente ou a partir de um cache.
* **Tolere ser invocado de dentro de si mesmo.** Claude Code aplica o launcher a cada self-spawn aninhado, então um launcher que adquire um recurso exclusivo deve detectar que já o mantém.
* **Não escreva no terminal antes de Claude Code iniciar.** Qualquer coisa impressa antes do `exec` é relatada como a causa do crash se a sessão morrer antes de inicializar.

<h3 id="format-of-the-launcher-value">
  Formato do valor do launcher
</h3>

`CLAUDE_CODE_PROCESS_WRAPPER` e a configuração `processWrapper` usam o mesmo formato. Para a maioria dos launchers, o valor é o caminho absoluto do script, como `/opt/corp/launcher`.

Para passar argumentos do seu launcher, escreva-os após o caminho. Claude Code analisa o valor como uma lista de argumentos, não um comando de shell:

* Espaço em branco separa tokens, e aspas duplas agrupam um token que contém espaços.
* Um valor que começa com `[` é lido como um array de string JSON, como `["/opt/corp/launcher", "--profile", "cc"]`.
* Sintaxe de shell não funciona: não há expansão de variável ou globbing, e um operador sem aspas como `;`, `|`, `&`, ou `$(` é rejeitado como um erro de configuração em vez de reinterpretado.

Quando o valor não pode ser usado, Claude Code recusa iniciar o processo afetado e [relata o motivo](/docs/pt/errors#claude_code_process_wrapper-launcher-errors).

<h2 id="relationship-to-claude_code_shell_prefix">
  Relação com `CLAUDE_CODE_SHELL_PREFIX`
</h2>

`CLAUDE_CODE_PROCESS_WRAPPER` envolve os próprios processos do Claude Code e passa o comando através como tokens argv separados para o launcher fazer `exec`. [`CLAUDE_CODE_SHELL_PREFIX`](/docs/pt/env-vars) envolve os comandos de shell que Claude Code executa em seu nome, como chamadas de ferramenta Bash, hooks, e os comandos que iniciam servidores MCP stdio, e passa cada um como uma única string com aspas de shell em `$1` para o wrapper reavaliar. Um launcher escrito para um não funciona como o outro.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Agent view](/docs/pt/agent-view): as sessões de fundo e processo supervisor que o launcher cobre
* [Environment variables](/docs/pt/env-vars): a entrada de referência `CLAUDE_CODE_PROCESS_WRAPPER`
* [Managed settings](/docs/pt/permissions#managed-settings): entregar o bloco `env` em toda uma frota
* [Launcher error reference](/docs/pt/errors#claude_code_process_wrapper-launcher-errors): as mensagens de recusa e como se recuperar
