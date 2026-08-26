> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Deixe Claude usar seu computador a partir da CLI

> Ative o computer use na Claude Code CLI para que Claude possa abrir aplicativos, clicar, digitar e ver sua tela no macOS. Teste aplicativos nativos, depure problemas visuais e automatize ferramentas apenas com GUI sem sair do seu terminal.

<Note>
  Computer use é uma visualização de pesquisa no macOS que requer um plano Pro ou Max. Não está disponível em planos Team ou Enterprise. Requer uma sessão interativa, portanto não está disponível em modo não interativo com a flag `-p`.
</Note>

Computer use permite que Claude abra aplicativos, controle sua tela e trabalhe em sua máquina da forma como você faria. A partir da CLI, Claude pode compilar um aplicativo Swift, iniciá-lo, clicar em cada botão e capturar uma tela do resultado, tudo na mesma conversa em que escreveu o código.

Esta página aborda como o computer use funciona na CLI. Para o aplicativo Desktop no macOS ou Windows, consulte [computer use em Desktop](/docs/pt/desktop#let-claude-use-your-computer).

<h2 id="what-you-can-do-with-computer-use">
  O que você pode fazer com computer use
</h2>

Computer use lida com tarefas que requerem uma GUI: qualquer coisa que você normalmente teria que sair do terminal e fazer manualmente.

* **Construir e validar aplicativos nativos**: peça a Claude para construir um aplicativo de barra de menu do macOS. Claude escreve o Swift, compila, inicia e clica em cada controle para verificar se funciona antes de você abri-lo.
* **Testes de UI de ponta a ponta**: aponte Claude para um aplicativo Electron local e diga "teste o fluxo de integração". Claude abre o aplicativo, clica na inscrição e captura cada etapa. Sem configuração do Playwright, sem teste harness.
* **Depurar problemas visuais e de layout**: diga a Claude "o modal está sendo cortado em janelas pequenas". Claude redimensiona a janela, reproduz o bug, captura uma tela, corrige o CSS e verifica a correção. Claude vê o que você vê.
* **Dirigir ferramentas apenas com GUI**: interaja com ferramentas de design, painéis de controle de hardware, o iOS Simulator ou aplicativos proprietários que não possuem CLI ou API.

<h2 id="when-computer-use-applies">
  Quando computer use se aplica
</h2>

Claude tem várias maneiras de interagir com um aplicativo ou serviço. Computer use é a mais ampla e lenta, portanto Claude tenta a ferramenta mais precisa primeiro:

* Se você tiver um [servidor MCP](/docs/pt/mcp) para o serviço, Claude usa isso.
* Se a tarefa for um comando shell, Claude usa Bash.
* Se a tarefa for trabalho de navegador e você tiver [Claude no Chrome](/docs/pt/chrome) configurado, Claude usa isso.
* Se nenhum desses se aplicar, Claude usa computer use.

O controle de tela é reservado para coisas que nada mais pode alcançar: aplicativos nativos, simuladores e ferramentas sem uma API.

<h2 id="enable-computer-use">
  Ativar computer use
</h2>

Computer use está disponível como um servidor MCP integrado chamado `computer-use`. Está desativado por padrão até que você o ative.

<Steps>
  <Step title="Abra o menu MCP">
    Em uma sessão interativa do Claude Code, execute:

    ```text theme={null}
    /mcp
    ```

    Encontre `computer-use` na lista de servidores. Ele aparece como desativado.
  </Step>

  <Step title="Ativar o servidor">
    Selecione `computer-use` e escolha **Enable**. A configuração persiste por projeto, portanto você faz isso apenas uma vez para cada projeto onde deseja usar computer use.
  </Step>

  <Step title="Conceder permissões do macOS">
    Na primeira vez que Claude tentar usar seu computador, você verá um prompt para conceder duas permissões do macOS:

    * **Accessibility**: permite que Claude clique, digite e role
    * **Screen Recording**: permite que Claude veja o que está em sua tela

    O prompt inclui links para abrir o painel System Settings relevante. Conceda ambos e selecione **Try again** no prompt. O macOS pode exigir que você reinicie Claude Code após conceder Screen Recording.
  </Step>
</Steps>

Após a configuração, peça a Claude para fazer algo que precise da GUI:

```text theme={null}
Build the app target, launch it, and click through each tab to make
sure nothing crashes. Screenshot any error states you find.
```

<h2 id="approve-apps-per-session">
  Aprovar aplicativos por sessão
</h2>

Ativar o servidor `computer-use` não concede a Claude acesso a todos os aplicativos em sua máquina. Na primeira vez que Claude precisar de um aplicativo específico em uma sessão, um prompt aparece em seu terminal mostrando:

* Quais aplicativos Claude deseja controlar
* Quaisquer permissões extras solicitadas, como acesso à área de transferência
* Quantos outros aplicativos serão ocultados enquanto Claude trabalha

Escolha **Allow for this session** ou **Deny**. As aprovações duram para a sessão atual. Você pode aprovar vários aplicativos de uma vez quando Claude os solicita juntos.

Aplicativos com amplo alcance mostram um aviso extra no prompt para que você saiba o que aprovar concede:

| Aviso                                 | Aplica-se a                                              |
| :------------------------------------ | :------------------------------------------------------- |
| Equivalente ao acesso shell           | Terminal, iTerm, VS Code, Warp e outros terminais e IDEs |
| Pode ler ou escrever qualquer arquivo | Finder                                                   |
| Pode alterar configurações do sistema | System Settings                                          |

Esses aplicativos não são bloqueados. O aviso permite que você decida se a tarefa justifica esse nível de acesso.

O nível de controle de Claude também varia por categoria de aplicativo: navegadores e plataformas de negociação são apenas visualização, terminais e IDEs são apenas clique, e tudo o mais obtém controle total. Consulte [permissões de aplicativo em Desktop](/docs/pt/desktop#app-permissions) para a divisão de camada completa.

<h2 id="how-claude-works-on-your-screen">
  Como Claude trabalha em sua tela
</h2>

Entender o fluxo ajuda você a antecipar o que Claude fará e como intervir.

<h3 id="one-session-at-a-time">
  Uma sessão por vez
</h3>

Computer use mantém um bloqueio em toda a máquina a partir da primeira ação de computer use até que a sessão que o adquiriu saia. A partir da v2.1.195, terminar a tarefa não libera o bloqueio; apenas sair da sessão faz isso. Se outra sessão do Claude Code já estiver usando seu computador, novas tentativas falharão com uma mensagem informando qual sessão mantém o bloqueio. Saia dessa sessão primeiro.

<h3 id="apps-are-hidden-while-claude-works">
  Os aplicativos são ocultados enquanto Claude trabalha
</h3>

Quando Claude começa a controlar sua tela, outros aplicativos visíveis são ocultados para que Claude interaja apenas com os aplicativos aprovados. Sua janela de terminal permanece visível e é excluída de capturas de tela, para que você possa assistir à sessão e Claude nunca veja sua própria saída.

Quando Claude termina a vez, os aplicativos ocultos são restaurados automaticamente.

<h3 id="screenshots-are-downscaled-automatically">
  As capturas de tela são redimensionadas automaticamente
</h3>

Claude Code redimensiona cada captura de tela antes de enviá-la ao modelo. Você não precisa reduzir sua resolução de exibição ou redimensionar janelas em Retina ou outras exibições de alta resolução. Um MacBook Pro de 16 polegadas em resolução Retina nativa captura em 3456×2234 e redimensiona para aproximadamente 1372×887, preservando a proporção de aspecto.

Não há configuração para alterar o tamanho de destino. Se o texto ou controles na tela forem muito pequenos para Claude ler após o redimensionamento, aumente seu tamanho no aplicativo em vez de alterar sua resolução de exibição.

<h3 id="stop-at-any-time">
  Parar a qualquer momento
</h3>

Quando Claude adquire o bloqueio, uma notificação do macOS aparece: "Claude is using your computer · press Esc to stop." Pressione `Esc` em qualquer lugar para abortar a ação atual imediatamente, ou pressione `Ctrl+C` no terminal. De qualquer forma, Claude para, mostra seus aplicativos novamente e retorna o controle a você. A sessão mantém o [bloqueio de computer use](#one-session-at-a-time) até que saia.

Uma segunda notificação aparece quando Claude termina.

<h2 id="safety-and-the-trust-boundary">
  Segurança e o limite de confiança
</h2>

<Warning>
  Ao contrário da [ferramenta Bash em sandbox](/docs/pt/sandboxing), computer use é executado em seu desktop real com acesso aos aplicativos que você aprova. Claude verifica cada ação e sinaliza possível injeção de prompt do conteúdo na tela, mas o limite de confiança é diferente. Consulte o [guia de segurança do computer use](https://support.claude.com/en/articles/14128542) para as melhores práticas.
</Warning>

Os guardrails integrados reduzem o risco sem exigir configuração:

* **Aprovação por aplicativo**: Claude pode controlar apenas aplicativos que você aprovou na sessão atual.
* **Avisos de sentinela**: aplicativos que concedem acesso shell, sistema de arquivos ou configurações do sistema são sinalizados antes de você aprovar.
* **Terminal excluído de capturas de tela**: Claude nunca vê sua janela de terminal, portanto prompts na tela em sua sessão não podem alimentar o modelo.
* **Escape global**: a tecla `Esc` aborta computer use de qualquer lugar, e o pressionamento de tecla é consumido para que injeção de prompt não possa usá-lo para descartar diálogos.
* **Arquivo de bloqueio**: apenas uma sessão pode controlar sua máquina por vez.

<h2 id="example-workflows">
  Fluxos de trabalho de exemplo
</h2>

Esses exemplos mostram maneiras comuns de combinar computer use com tarefas de codificação.

<h3 id="validate-a-native-build">
  Validar uma compilação nativa
</h3>

Após fazer alterações em um aplicativo macOS ou iOS, peça a Claude para compilar e verificar em uma única passagem:

```text theme={null}
Build the MenuBarStats target, launch it, open the preferences window,
and verify the interval slider updates the label. Screenshot the
preferences window when you're done.
```

Claude executa `xcodebuild`, inicia o aplicativo, interage com a UI e relata o que encontra.

<h3 id="reproduce-a-layout-bug">
  Reproduzir um bug de layout
</h3>

Quando um bug visual aparece apenas em certos tamanhos de janela, deixe Claude encontrá-lo:

```text theme={null}
The settings modal clips its footer on narrow windows. Resize the app
window down until you can reproduce it, screenshot the clipped state,
then check the CSS for the modal container.
```

Claude redimensiona a janela, captura o estado quebrado e lê as folhas de estilo relevantes.

<h3 id="test-a-simulator-flow">
  Testar um fluxo do simulador
</h3>

Dirija o iOS Simulator sem escrever XCTest:

```text theme={null}
Open the iOS Simulator, launch the app, tap through the onboarding
screens, and tell me if any screen takes more than a second to load.
```

Claude controla o simulador da mesma forma que você faria com um mouse.

<h2 id="differences-from-the-desktop-app">
  Diferenças do aplicativo Desktop
</h2>

As superfícies CLI e Desktop compartilham o mesmo mecanismo de computer use, com algumas diferenças:

| Recurso                      | Desktop                                                 | CLI                             |
| :--------------------------- | :------------------------------------------------------ | :------------------------------ |
| Plataformas                  | macOS e Windows                                         | Apenas macOS                    |
| Ativar                       | Alternar em **Settings > General** (em **Desktop app**) | Ativar `computer-use` em `/mcp` |
| Lista de aplicativos negados | Configurável em Settings                                | Ainda não disponível            |
| Alternância de auto-unhide   | Opcional                                                | Sempre ativado                  |
| Integração do Dispatch       | Sessões geradas por Dispatch podem usar computer use    | Não aplicável                   |

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="computer-use-is-in-use-by-another-claude-session">
  "Computer use is in use by another Claude session"
</h3>

Outra sessão do Claude Code mantém o bloqueio, que ela mantém até sair. Saia dessa sessão. Se a outra sessão travou, o bloqueio é liberado automaticamente quando Claude detecta que o processo não está mais em execução.

<h3 id="macos-permissions-prompt-keeps-reappearing">
  macOS permissions prompt keeps reappearing
</h3>

O macOS às vezes requer uma reinicialização do processo solicitante após você conceder Screen Recording. Saia completamente do Claude Code e inicie uma nova sessão. Se o prompt persistir, abra **System Settings > Privacy & Security > Screen Recording** e confirme que seu aplicativo de terminal está listado e ativado.

<h3 id="computer-use-doesn’t-appear-in-/mcp">
  `computer-use` doesn't appear in `/mcp`
</h3>

O servidor só aparece em configurações elegíveis. Verifique se:

* Você está no macOS. Computer use na CLI não está disponível no Linux ou Windows. No Windows, use [computer use em Desktop](/docs/pt/desktop#let-claude-use-your-computer) em vez disso.
* Você está em um plano Pro ou Max. Execute `/status` para confirmar sua assinatura.
* Você está autenticado através de claude.ai. Computer use não está disponível com provedores de terceiros como Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry. Se você acessar Claude exclusivamente através de um provedor de terceiros, você precisa de uma conta claude.ai separada para usar este recurso.
* Você está em uma sessão interativa. Computer use não está disponível em modo não interativo com a flag `-p`.

<h2 id="see-also">
  Veja também
</h2>

* [Computer use em Desktop](/docs/pt/desktop#let-claude-use-your-computer): a mesma capacidade com uma página de configurações gráfica
* [Claude no Chrome](/docs/pt/chrome): automação de navegador para tarefas baseadas na web
* [MCP](/docs/pt/mcp): conecte Claude a ferramentas e APIs estruturadas
* [Sandboxing](/docs/pt/sandboxing): como a ferramenta Bash de Claude isola o acesso ao sistema de arquivos e rede
* [Guia de segurança do computer use](https://support.claude.com/en/articles/14128542): melhores práticas para uso seguro de computer use
