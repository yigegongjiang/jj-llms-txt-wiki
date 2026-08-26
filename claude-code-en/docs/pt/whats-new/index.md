> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Novidades

> Um resumo semanal de recursos notáveis do Claude Code, com trechos de código, demonstrações e contexto sobre por que são importantes.

O resumo semanal para desenvolvedores destaca os recursos com maior probabilidade de mudar a forma como você trabalha. Cada entrada inclui código executável, uma breve demonstração e um link para a documentação completa. Para cada correção de bug e melhoria menor, consulte o [changelog](/docs/pt/changelog).

<Update label="Week 28" description="6–10 de julho de 2026" tags={["v2.1.202–v2.1.206"]}>
  **Navegador integrado no Desktop**: Claude Code no desktop recebe um navegador integrado, para que Claude possa abrir documentos, designs ou qualquer outro site e interagir com páginas da mesma forma que faz com suas visualizações de servidor de desenvolvimento local.

  Também esta semana: **`/doctor`** é uma verificação completa de configuração que diagnostica problemas e pode corrigi-los, com `/checkup` como seu alias; **auto mode** bloqueia adulteração de transcrição e pede confirmação antes de `rm -rf` em variáveis não resolvidas; e **linhas de visualização de agente** mostram uma palavra de estado colorida e um título escrito por classificador.

  [Leia o resumo da Week 28 →](/docs/pt/whats-new/2026-w28)
</Update>

<Update label="Week 27" description="29 de junho – 3 de julho de 2026" tags={["v2.1.195–v2.1.201"]}>
  **Claude Sonnet 5**: o novo modelo padrão para assentos de assinatura Pro, Team Standard e Enterprise, com codificação de primeira classe e uso de ferramentas ao preço de Sonnet, uma janela de contexto nativa de 1M de tokens e pensamento adaptativo ativado por padrão.

  Também esta semana: **Claude no Chrome** está geralmente disponível em todos os planos diretos da Anthropic; **subagentes são executados em segundo plano por padrão** para que Claude continue trabalhando enquanto eles são executados; **Claude Desktop no Linux** chega em beta no Ubuntu e Debian; e **`/radio`** sintoniza a rádio lo-fi Claude FM.

  [Leia o resumo da Week 27 →](/docs/pt/whats-new/2026-w27)
</Update>

<Update label="Week 26" description="22–26 de junho de 2026" tags={["v2.1.185–v2.1.193"]}>
  **`claude mcp login`**: autentique um servidor MCP configurado a partir do seu shell em vez do menu interativo `/mcp`, e limpe suas credenciais armazenadas posteriormente com `claude mcp logout`.

  Também esta semana: **shell mode responde à saída do comando** (`! npm test` recebe uma explicação sem um segundo prompt); **`/rewind`** pode retomar uma conversa de antes de `/clear` ser executado; e **subagentes de fundo** agora exibem prompts de permissão na sessão principal em vez de negar automaticamente.

  [Leia o resumo da Week 26 →](/docs/pt/whats-new/2026-w26)
</Update>

<Update label="Week 25" description="15–19 de junho de 2026" tags={["v2.1.178–v2.1.183"]}>
  **Artifacts**: transforme a saída de uma sessão em uma página ao vivo e compartilhável no claude.ai que se atualiza no local conforme a sessão funciona, agora em beta nos planos Team e Enterprise.

  Também esta semana: **regras de negação e solicitação correspondem aos parâmetros da ferramenta** com `Tool(param:value)`, por exemplo `Agent(model:opus)`; **`/config key=value`** define qualquer configuração a partir do prompt, no modo `-p`, e do Remote Control; e **auto mode bloqueia comandos git destrutivos** quando você não pediu para descartar trabalho local.

  [Leia o resumo da Week 25 →](/docs/pt/whats-new/2026-w25)
</Update>

<Update label="Week 24" description="8–12 de junho de 2026" tags={["v2.1.166–v2.1.176"]}>
  **`/cd`**: mova a sessão atual para um novo diretório de trabalho no meio da conversa sem reconstruir o cache de prompt.

  Também esta semana: **sub-agentes podem gerar seus próprios sub-agentes** (cadeias de fundo são limitadas a cinco níveis de profundidade); **`--safe-mode`** inicia Claude Code com todas as personalizações desabilitadas para solução de problemas; e **`fallbackModel`** configura até três modelos de fallback tentados em ordem.

  [Leia o resumo da Week 24 →](/docs/pt/whats-new/2026-w24)
</Update>

<Update label="Week 23" description="1–5 de junho de 2026" tags={["v2.1.158–v2.1.165"]}>
  **Auto mode no Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry**: auto mode agora está disponível em provedores de terceiros para Opus 4.7 e Opus 4.8, substituindo prompts de permissão por verificações de segurança em segundo plano.

  Também esta semana: **edições automáticas mais seguras** solicitam antes de escrever arquivos que podem executar código no modo `acceptEdits`; **`/plugin list`** imprime seus plugins instalados inline; e **requisitos de versão** permitem que implantações gerenciadas exijam um intervalo de versão aprovado do Claude Code.

  [Leia o resumo da Week 23 →](/docs/pt/whats-new/2026-w23)
</Update>

<Update label="Week 22" description="25–29 de maio de 2026" tags={["v2.1.150–v2.1.157"]}>
  **Claude Opus 4.8**: o novo modelo padrão para Max, Team Premium, Enterprise pay-as-you-go e contas da API Anthropic, com alto esforço por padrão e `/effort xhigh` para as tarefas mais difíceis.

  Também esta semana: **dynamic workflows** orquestram dezenas a centenas de subagentes a partir de um script que Claude escreve; o **security-guidance plugin** revisa as alterações de Claude em busca de vulnerabilidades enquanto funciona; e **fast mode** é executado no Opus 4.8 a \$10/\$50 por MTok.

  [Leia o resumo da Week 22 →](/docs/pt/whats-new/2026-w22)
</Update>

<Update label="Week 21" description="18–22 de maio de 2026" tags={["v2.1.143–v2.1.149"]}>
  **Auto mode no plano Pro**: auto mode agora é executado em contas Pro e suporta Sonnet 4.6 junto com Opus, substituindo prompts de permissão por verificações de segurança em segundo plano.

  Também esta semana: **`/usage`** detalha o que impulsiona seus limites de plano por skill, subagente, plugin e servidor MCP; o novo comando **`/code-review`** relata bugs de correção; e **background sessions** aparecem em `/resume` e permanecem ativas quando fixadas.

  [Leia o resumo da Week 21 →](/docs/pt/whats-new/2026-w21)
</Update>

<Update label="Week 20" description="11–15 de maio de 2026" tags={["v2.1.139–v2.1.142"]}>
  **Agent view**: `claude agents` abre uma tela para cada sessão do Claude Code, mostrando o que está em execução, o que está bloqueado esperando por você e o que está concluído.

  Também esta semana: **`/goal`** mantém Claude trabalhando entre turnos até que uma condição de conclusão seja atendida; **fast mode** agora é executado no Opus 4.7 por padrão; e o **menu Rewind** pode compactar contexto anterior com "Summarize up to here".

  [Leia o resumo da Week 20 →](/docs/pt/whats-new/2026-w20)
</Update>

<Update label="Week 19" description="4–8 de maio de 2026" tags={["v2.1.128–v2.1.136"]}>
  **Plugins carregam de arquivos `.zip` e URLs**: `--plugin-dir` agora aceita arquivos `.zip`, e `--plugin-url` busca um arquivo de plugin para a sessão atual.

  Também esta semana: **`worktree.baseRef`** escolhe se novas worktrees ramificam do padrão remoto ou do `HEAD` local; **regras de negação rígida do modo automático** bloqueiam ações incondicionalmente, independentemente de exceções de permissão; e **hooks veem o nível de esforço ativo** via `effort.level` e `$CLAUDE_EFFORT`.

  [Leia o resumo da Week 19 →](/docs/pt/whats-new/2026-w19)
</Update>

<Update label="Week 18" description="27 de abril – 1º de maio de 2026" tags={["v2.1.120–v2.1.126"]}>
  **Windows sem Git Bash**: Git para Windows não é mais necessário, e Claude Code usa PowerShell como a ferramenta de shell quando Bash está ausente.

  Também esta semana: **`claude ultrareview`** traz revisão de código em nuvem para CI e scripts; **`claude project purge`** limpa o estado local de um projeto; e colar uma **URL de PR em `/resume`** encontra a sessão que a criou.

  [Leia o resumo da Week 18 →](/docs/pt/whats-new/2026-w18)
</Update>

<Update label="Week 17" description="20–24 de abril de 2026" tags={["v2.1.114–v2.1.119"]}>
  **`/ultrareview`** abre como uma visualização pública de pesquisa: uma frota de agentes de caça a bugs é executada na nuvem e os resultados chegam automaticamente ao seu CLI ou Desktop.

  Também esta semana: **session recap** mostra o que aconteceu enquanto um terminal estava desfocado; **custom themes** permite que você crie e implante paletas de cores de `/theme` ou de um plugin; e **Claude Code na web** recebe um redesign com uma nova barra lateral de sessões e layout de arrastar e soltar.

  [Leia o resumo da Week 17 →](/docs/pt/whats-new/2026-w17)
</Update>

<Update label="Week 16" description="13–17 de abril de 2026" tags={["v2.1.105–v2.1.113"]}>
  **Claude Opus 4.7** chega como o novo padrão no Max e Team Premium, com um novo nível de esforço `xhigh` que é a configuração recomendada para a maioria do trabalho de codificação e um controle deslizante `/effort` interativo para ajustá-lo.

  Também esta semana: **Routines** no Claude Code na web disparam agentes de nuvem templados a partir de um cronograma, evento do GitHub ou chamada de API; **notificações push móveis** alertam seu telefone quando uma tarefa longa termina ou Claude precisa de você; `/usage` mostra o que está impulsionando seus limites; e o CLI passa para binários nativos.

  [Leia o resumo da Week 16 →](/docs/pt/whats-new/2026-w16)
</Update>

<Update label="Week 15" description="6–10 de abril de 2026" tags={["v2.1.92–v2.1.101"]}>
  **Ultraplan** entra em visualização antecipada: elabore um plano na nuvem a partir do seu CLI, revise e comente sobre ele em um editor da web, depois execute-o remotamente ou puxe-o de volta para o local. A primeira execução agora cria automaticamente um ambiente de nuvem para você.

  Também esta semana: a ferramenta **Monitor** transmite eventos de fundo para a conversa para que Claude possa monitorar logs e reagir em tempo real, `/loop` auto-avança quando você omite o intervalo, `/team-onboarding` empacota sua configuração em um guia reproduzível, e `/autofix-pr` ativa a correção automática de PR a partir do seu terminal.

  [Leia o resumo da Week 15 →](/docs/pt/whats-new/2026-w15)
</Update>

<Update label="Week 14" description="30 de março – 3 de abril de 2026" tags={["v2.1.86–v2.1.91"]}>
  **Computer use** chega ao CLI em visualização de pesquisa: Claude pode abrir aplicativos nativos, clicar pela interface do usuário e verificar alterações a partir do seu terminal. Melhor para fechar o loop em coisas que apenas uma GUI pode verificar.

  Também esta semana: lições interativas `/powerup`, renderização de tela alternativa sem cintilação, uma substituição de tamanho de resultado MCP por ferramenta até 500K, e executáveis de plugin no `PATH` da ferramenta Bash.

  [Leia o resumo da Week 14 →](/docs/pt/whats-new/2026-w14)
</Update>

<Update label="Week 13" description="23–27 de março de 2026" tags={["v2.1.83–v2.1.85"]}>
  **Auto mode** chega em visualização de pesquisa: um classificador lida com seus prompts de permissão para que ações seguras sejam executadas sem interrupção e as arriscadas sejam bloqueadas. O meio termo entre aprovar tudo e `--dangerously-skip-permissions`.

  Também esta semana: computer use no aplicativo Desktop, PR auto-fix na Web, busca de transcrição com `/`, uma ferramenta PowerShell nativa para Windows, e hooks `if` condicionais.

  [Leia o resumo da Week 13 →](/docs/pt/whats-new/2026-w13)
</Update>
