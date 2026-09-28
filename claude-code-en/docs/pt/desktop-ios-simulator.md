> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Testar aplicativos iOS no simulador

> Claude Code Desktop abre seu aplicativo no painel iOS Simulator quando Claude constrói, executa ou verifica, com um simulador separado para cada sessão.

<Note>
  O painel iOS Simulator está em beta pública no Claude Code Desktop no macOS. Está disponível nos planos Pro, Max, Team e Enterprise, exceto em organizações Enterprise que têm uma configuração HIPAA ativada.
</Note>

O painel iOS Simulator mostra seu aplicativo em execução no iOS Simulator da Apple ao lado de sua conversa no Claude Code Desktop. Quando Claude constrói, instala, inicia ou verifica seu aplicativo em um simulador, o painel abre automaticamente e transmite a tela do dispositivo ao vivo. Use-o para assistir Claude executar e testar seu aplicativo, ou toque no aplicativo você mesmo enquanto Claude continua trabalhando.

O painel do simulador controla o simulador diretamente, portanto não precisa de [computer use](/docs/pt/desktop#let-claude-use-your-computer) e nunca assume o controle de sua tela ou oculta suas outras janelas. A partir da CLI, Claude acessa o iOS Simulator através de [computer use](/docs/pt/computer-use#test-a-simulator-flow), que controla o simulador em sua tela da mesma forma que você faria com um mouse.

<h2 id="requirements">
  Requisitos
</h2>

O painel do simulador usa as ferramentas de simulador da Apple, que o aplicativo desktop não inclui. Antes de iniciar uma sessão, certifique-se de que você tem:

* Claude Desktop v1.24012.0 ou posterior
* Um Mac, já que o iOS Simulator da Apple é executado apenas no macOS
* [Xcode](https://developer.apple.com/xcode/) com a plataforma iOS instalada, que fornece os dispositivos simuladores. Se o Xcode ainda não listar simuladores, consulte [O painel do simulador diz que nenhum simulador foi encontrado](#the-simulator-pane-says-no-simulators-were-found)
  * Use Xcode 26.x. O painel ainda não funciona com Xcode 27, que substitui o aplicativo Simulator por Device Hub. Se `xcode-select` aponta para Xcode 27 em seu Mac, consulte [O painel do simulador falha com Xcode 27](#the-simulator-pane-fails-with-xcode-27)

<Note>
  Nesta página, "dispositivo" refere-se a um iPhone ou iPad simulado, um dos mesmos dispositivos simuladores que você gerencia no Xcode em **Window → Devices and Simulators**, não hardware físico.
</Note>

O painel do simulador está disponível apenas em sessões locais. Em sessões [cloud](/docs/pt/desktop#run-long-running-tasks-in-the-cloud) e [SSH](/docs/pt/desktop#ssh-sessions), Claude é executado em uma máquina que não consegue alcançar os simuladores em seu Mac.

<h2 id="run-your-app-in-the-simulator">
  Execute seu aplicativo no simulador
</h2>

Você não precisa de um comando ou configuração para abrir o painel do simulador. Claude o abre quando executa seu aplicativo em um simulador.

<Steps>
  <Step title="Abra seu projeto iOS">
    No Claude Code Desktop, abra a aba **Code** e inicie uma sessão com a pasta do projeto do seu aplicativo como a [pasta do projeto](/docs/pt/desktop#start-a-session). Qualquer projeto que constrói um aplicativo para o iOS Simulator funciona.
  </Step>

  <Step title="Peça a Claude para executar ou testar o aplicativo">
    Formule a tarefa em torno da execução ou verificação do aplicativo. Por exemplo:

    ```text theme={null}
    Build the app and run it in the simulator to check the onboarding flow.
    ```
  </Step>

  <Step title="Assista o aplicativo no painel do simulador">
    Quando o aplicativo é iniciado em um simulador, o painel iOS Simulator abre ao lado da conversa. A primeira vez que Claude usa um dispositivo, o aplicativo desktop pede permissão; consulte [Conceder acesso de Claude a um dispositivo](#grant-claude-access-to-a-device). Claude instala o aplicativo, toca nele e lê a tela para verificar suas próprias alterações enquanto você assiste.
  </Step>
</Steps>

O painel do simulador abre sempre que Claude inicia o aplicativo em um simulador, em qualquer ponto da sessão. Quando sua solicitação é sobre ver o aplicativo, por exemplo "a nova tela parece correta?", Claude inicia um simulador antes de começar o trabalho. Depois que Claude corrige um bug ou altera uma tela, peça-lhe para verificar a alteração: reiniciar o aplicativo reabre o painel se ele não estiver aberto.

O painel do simulador mostra qualquer dispositivo em que o aplicativo foi realmente iniciado. Para testar em um dispositivo específico, nomeie-o em sua solicitação, por exemplo "execute-o no simulador iPhone SE", e Claude direcionará esse dispositivo quando construir e iniciar.

Um dispositivo que Claude inicia também aparece no aplicativo Simulator da Apple, e Claude pode instalar o aplicativo em um dispositivo que você já tem iniciado.

Você também pode abrir o painel do simulador você mesmo. Depois que a sessão tem um simulador anexado ou editou arquivos Swift, o menu **Views** na barra de ferramentas da sessão mostra uma entrada **iOS Simulator**. Se o painel ainda não está mostrando um dispositivo, clique em **Attach simulator**, ou escolha um dispositivo específico no menu de dispositivos ao lado; escolher um dispositivo desligado o inicia. Se Xcode ou seus simuladores estão faltando, o painel mostra as etapas de configuração em vez disso e as marca conforme você as completa.

<h2 id="control-the-simulator-yourself">
  Controle o simulador você mesmo
</h2>

O painel do simulador é interativo, não apenas um visualizador. Enquanto Claude trabalha, ou entre tarefas, você pode:

* Tocar e deslizar clicando e arrastando na tela do dispositivo
* Pressionar botões de hardware com os mesmos atalhos de teclado do aplicativo Simulator da Apple: **Cmd+Shift+H** para Home, **Cmd+L** para bloquear, **Cmd+Up Arrow** e **Cmd+Down Arrow** para volume
* Girar o dispositivo um quarto de volta no sentido horário com o botão girar ou **Cmd+Right Arrow**
* Alternar qual dispositivo o painel mostra no menu de dispositivos, que lista a versão do SO de cada simulador e se está iniciado
* Salvar uma captura de tela com **Cmd+S** ou uma gravação de tela com **Cmd+R**, usando os botões de captura do painel ou os atalhos de teclado; os arquivos são salvos em sua Desktop
* Parar de transmitir um dispositivo sem desligá-lo clicando em **Detach simulator**, que retorna o painel ao estado **Attach simulator**

A linha sob o nome do dispositivo ajusta o fluxo de vídeo do simulador. Reduza **Frame rate** ou **Resolution** se o painel sobrecarregar seu Mac, alterne **Encoding** entre H.264 e JPEG, ou marque **FPS** para exibir a taxa de quadros que o painel está recebendo. Essas configurações alteram como o painel exibe o dispositivo, não como o aplicativo é executado.

Você e Claude controlam o mesmo dispositivo, portanto seus toques alteram o estado do aplicativo que Claude vê. Para fazer Claude verificar uma tela específica, navegue até ela tocando e depois pergunte. Enquanto Claude está controlando o dispositivo, o painel mostra um crachá **Claude is using this device** acima da tela; espere tocar até que o crachá desapareça, para que o resultado reflita o aplicativo em vez de sua entrada.

<h2 id="how-sessions-manage-devices">
  Como as sessões gerenciam dispositivos
</h2>

Cada dispositivo pertence à sessão que o iniciou, portanto [sessões paralelas](/docs/pt/desktop#work-in-parallel-with-sessions) não compartilham um dispositivo: o que você vê no painel de uma sessão reflete o trabalho dessa sessão, não de outra. Alternar sessões na barra lateral alterna a visualização do simulador junto com a conversa, e alternar de volta retoma o mesmo dispositivo onde parou. Se Claude trabalha com mais de um dispositivo, cada um abre seu próprio painel, até 4 por sessão.

Claude Code Desktop desliga os simuladores que iniciou quando não estão mais em uso: quando você sai do aplicativo, quando você arquiva a sessão, ou 10 minutos depois que você desanexa um dispositivo de seu painel. Dispositivos que você inicia você mesmo, seja no painel ou no aplicativo Simulator da Apple, nunca são desligados automaticamente. Para desligar o dispositivo anexado imediatamente, use o botão de desligamento no painel.

<h2 id="grant-claude-access-to-a-device">
  Conceder acesso de Claude a um dispositivo
</h2>

Claude pede seu consentimento antes de controlar um dispositivo, enquanto construir o aplicativo ou abrir uma URL nele segue o modo de permissão de sua sessão. Você ou sua organização também podem desativar completamente o acesso de Claude.

<h3 id="allow-a-device-the-first-time">
  Permitir um dispositivo pela primeira vez
</h3>

A primeira vez que Claude usa um simulador, o aplicativo desktop pede permissão. O consentimento cobre controlar esse dispositivo e tirar capturas de tela dele, e você o concede uma vez por dispositivo em vez de uma vez por sessão. As capturas de tela de Claude do dispositivo são enviadas para Anthropic e mantidas sob suas configurações normais de retenção de conversa, portanto não faça login em contas reais em um dispositivo que Claude usa.

Depois de permitir um dispositivo, as ações de Claude nele, como tocar, digitar, iniciar o aplicativo e tirar capturas de tela, são executadas sem mais prompts. Elas têm a mesma confiança que você clicando no painel, e elas apenas tocam o dispositivo simulado, portanto o painel não precisa das permissões de Acessibilidade e Gravação de Tela do macOS que o computer use requer.

Se você recusar, o dispositivo ainda inicia e o painel ainda funciona para seus próprios toques; apenas o acesso de Claude permanece desativado. Para mudar de ideia depois, clique em **Let Claude use it** no painel.

<h3 id="actions-that-follow-your-permission-mode">
  Ações que seguem seu modo de permissão
</h3>

Duas ações seguem o [modo de permissão](/docs/pt/permissions#permission-modes) de sua sessão em vez do consentimento único:

* Abrir uma URL no dispositivo, por exemplo para testar um deep link ou carregar uma página no Safari do dispositivo, porque uma URL pode levar dados para fora do dispositivo.
* Construir o aplicativo, porque `xcodebuild` executa os scripts de construção do seu projeto em seu Mac. Verificar uma construção já em andamento não solicita.

<h3 id="turn-off-simulator-access">
  Desativar acesso ao simulador
</h3>

Você pode desativar o acesso do simulador de Claude nas configurações do aplicativo desktop. As organizações têm duas maneiras de desativá-lo para todos:

* A [configuração gerenciada](/docs/pt/desktop#managed-settings) `disableMobileSimulatorTools` bloqueia as ferramentas de simulador de Claude. O painel do simulador permanece utilizável para seus próprios toques, e a configuração não pode ser substituída de dentro do aplicativo.
* A chave de política `requireCoworkFullVmSandbox`, que executa as ferramentas de Claude dentro de uma máquina virtual isolada em vez de em seu Mac, desativa o painel do simulador e as ferramentas de simulador de Claude inteiramente, portanto o painel não consegue anexar um dispositivo enquanto está definido.

Claude informa quando qualquer um deles se aplica.

<h2 id="limitations">
  Limitações
</h2>

Claude controla apenas dispositivos simulados e não consegue controlar um iPhone ou iPad físico. Para testar em um, execute o aplicativo nele a partir do Xcode você mesmo, depois descreva o que você vê ou anexe uma captura de tela à conversa para Claude trabalhar.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="the-simulator-pane-doesn’t-open-when-claude-runs-the-app">
  O painel do simulador não abre quando Claude executa o aplicativo
</h3>

Claude pode não ter reconhecido que você queria executar ou testar o aplicativo, ou as ferramentas de simulador podem estar faltando. Verifique o seguinte:

* Declare o objetivo explicitamente, por exemplo "execute o aplicativo no iOS Simulator e toque no fluxo de inscrição".
* Confirme que Xcode e os simuladores iOS estão instalados e que sua versão do Xcode atende aos [requisitos](#requirements).
* Se sua organização gerencia Claude Code, as [ferramentas de simulador podem estar desativadas por política](#turn-off-simulator-access).
* Se você está em uma organização Enterprise que tem uma configuração HIPAA ativada, o painel do simulador não está disponível para você.
* O painel do simulador requer Claude Desktop v1.24012.0 ou posterior. Abra **Claude → Check for Updates**, depois reinicie o aplicativo.

<h3 id="the-simulator-pane-says-no-simulators-were-found">
  O painel do simulador diz que nenhum simulador foi encontrado
</h3>

Se `xcode-select` aponta para Xcode 27, o painel pode relatar que nenhum simulador foi encontrado mesmo que dispositivos existam; consulte [O painel do simulador falha com Xcode 27](#the-simulator-pane-fails-with-xcode-27). Caso contrário, Xcode está instalado mas não tem simuladores iOS para listar. O painel do simulador mostra as etapas de configuração a seguir e as marca conforme cada uma é concluída. Para instalar a peça faltante manualmente, baixe o tempo de execução do simulador iOS das configurações do Xcode, ou execute `xcodebuild -downloadPlatform iOS`.

<h3 id="the-simulator-pane-fails-with-xcode-27">
  O painel do simulador falha com Xcode 27
</h3>

O painel ainda não funciona com Xcode 27, que substitui o aplicativo Simulator por Device Hub. Com Xcode 27 selecionado, anexar um dispositivo falha, ou o painel relata que nenhum simulador foi encontrado mesmo que dispositivos existam.

O painel usa qualquer Xcode que `xcode-select` aponta. Se Xcode 27 é sua única instalação, instale Xcode 26.x ao lado primeiro. Depois selecione a instalação 26.x por seu caminho. Por exemplo, se está instalado como `/Applications/Xcode-26.4.app`:

```bash theme={null}
sudo xcode-select -s /Applications/Xcode-26.4.app
```

Execute `xcode-select -p` para verificar qual instalação está selecionada.

<h2 id="see-also">
  Veja também
</h2>

* [Computer use in Desktop](/docs/pt/desktop#let-claude-use-your-computer): controle de tela para aplicativos sem um painel dedicado
* [Computer use from the CLI](/docs/pt/computer-use): como a CLI alcança o iOS Simulator
* [Work in parallel with sessions](/docs/pt/desktop#work-in-parallel-with-sessions): como as sessões isolam alterações
* [Get started with Claude Code Desktop](/docs/pt/desktop-quickstart)
