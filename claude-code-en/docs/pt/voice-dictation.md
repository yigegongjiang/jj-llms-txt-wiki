> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ditado por voz

> Fale seus prompts no Claude Code CLI com ditado por voz com manutenção ou toque para gravar.

Fale seus prompts em vez de digitá-los no Claude Code CLI. Sua fala é transcrita em tempo real na entrada do prompt, para que você possa misturar voz e digitação na mesma mensagem. Ative o ditado com `/voice`, depois mantenha uma tecla pressionada enquanto fala ou toque uma vez para começar e novamente para enviar.

<Note>
  Modo de toque requer Claude Code v2.1.116 ou posterior. Verifique sua versão com `claude --version`.
</Note>

O ditado também funciona na [visualização do agente](/docs/pt/agent-view#peek-and-reply). Mantenha ou toque sua tecla push-to-talk enquanto a entrada de despacho ou uma resposta do painel de visualização estiver em foco para ditar para uma sessão em segundo plano.

<h2 id="requirements">
  Requisitos
</h2>

O ditado por voz transmite seu áudio gravado para os servidores da Anthropic para transcrição. O áudio não é processado localmente. Ele precisa de todos os seguintes:

* **Uma conta Claude.ai**: o serviço de fala para texto está disponível apenas quando você se autentica com uma, e não está disponível quando Claude Code está configurado para usar uma chave API da Anthropic diretamente, Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry.
* **Uma organização sem conformidade HIPAA ativada**: `/voice` mostra `Voice mode is disabled by your organization's policy` quando essa restrição se aplica.
* **Um microfone local**: o ditado por voz não funciona em ambientes remotos como [Claude Code na web](/docs/pt/claude-code-on-the-web) ou sessões SSH.
* **WSLg, se você executar Claude Code no WSL**: WSLg está incluído no WSL2 quando instalado na Microsoft Store no Windows 10 ou 11. Se WSLg não estiver disponível, por exemplo no WSL1, execute Claude Code no Windows nativo.

A transcrição não consome mensagens Claude ou tokens e não conta para os limites mostrados em `/usage`. Consulte [data usage](/docs/pt/data-usage) para saber como a Anthropic lida com seus dados.

A gravação de áudio usa um módulo nativo integrado no macOS, Linux e Windows. No Linux, se o módulo nativo não conseguir carregar, Claude Code volta para `arecord` do ALSA utils ou `rec` do SoX. Se nenhum estiver disponível, `/voice` imprime um comando de instalação para seu gerenciador de pacotes.

A [extensão VS Code](/docs/pt/vs-code) do Claude Code também suporta ditado por voz com o mesmo requisito de conta Claude.ai. Não está disponível em sessões VS Code Remote, incluindo SSH, Dev Containers e Codespaces, porque o microfone está em sua máquina local e a extensão é executada no host remoto.

<h2 id="enable-voice-dictation">
  Ativar ditado por voz
</h2>

Execute `/voice` para ativar o ditado. Na primeira vez que você o ativa, Claude Code executa uma verificação de microfone. No macOS, isso dispara o prompt de permissão de microfone do sistema para seu terminal se nunca foi concedido.

```
/voice
Voice mode enabled (hold). Hold space to record. Dictation language: en (/config to change).
```

`/voice` aceita um argumento de modo opcional:

| Comando       | Efeito                                              |
| :------------ | :-------------------------------------------------- |
| `/voice`      | Alternar ativado ou desativado, manter o modo atual |
| `/voice hold` | Ativar no [modo de manutenção](#hold-to-record)     |
| `/voice tap`  | Ativar no [modo de toque](#tap-to-record-and-send)  |
| `/voice off`  | Desativar                                           |

O ditado por voz persiste entre sessões. Defina-o diretamente em seu [arquivo de configurações do usuário](/docs/pt/settings) em vez de executar `/voice`:

```json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

Enquanto o ditado por voz está ativado, o rodapé de entrada mostra uma dica `hold space to speak` quando o prompt está vazio. A dica reflete sua vinculação `voice:pushToTalk` atual e é atualizada se você [reassociar a tecla de ditado](#rebind-the-dictation-key). O texto da dica é o mesmo em ambos os modos e não aparece se você tiver um [status line personalizado](/docs/pt/statusline) configurado.

A transcrição é ajustada para vocabulário de codificação em ambos os modos. Termos de desenvolvimento comuns como `regex`, `OAuth`, `JSON` e `localhost` são reconhecidos corretamente, e o nome do seu projeto atual e o nome da ramificação git são adicionados automaticamente como dicas de reconhecimento.

<h2 id="hold-to-record">
  Manter pressionado para gravar
</h2>

O modo de manutenção é push-to-talk: a gravação é executada enquanto você mantém a tecla pressionada e para quando você a solta. Este é o modo padrão.

Mantenha `Space` pressionado para começar a gravar. Claude Code detecta uma tecla mantida observando eventos rápidos de repetição de tecla do seu terminal, portanto há um breve aquecimento antes da gravação começar. O rodapé mostra `keep holding…` durante o aquecimento e depois muda para uma forma de onda ao vivo quando a gravação está ativa.

Os primeiros caracteres de repetição de tecla digitam na entrada durante o aquecimento e são removidos automaticamente quando a gravação é ativada. Um único toque em `Space` ainda digita um espaço, pois a detecção de manutenção só é acionada na repetição rápida.

<Tip>
  Para pular o aquecimento, mude para [modo de toque](#tap-to-record-and-send) com `/voice tap`, ou [revinculação a uma combinação de modificador](#rebind-the-dictation-key) como `meta+k`. Combinações de modificadores começam a gravar no primeiro pressionamento de tecla.
</Tip>

Sua fala aparece no prompt conforme você fala, atenuada até que a transcrição seja finalizada. Solte `Space` para parar de gravar e finalizar o texto. A transcrição é inserida na posição do seu cursor e o cursor permanece no final do texto inserido, para que você possa misturar digitação e ditado em qualquer ordem. Mantenha `Space` pressionado novamente para anexar outra gravação, ou mova o cursor primeiro para inserir fala em outro lugar no prompt:

```
> refactor the auth middleware to ▮
  # hold Space, speak "use the new token validation helper"
> refactor the auth middleware to use the new token validation helper▮
```

Por padrão, soltar a tecla insere a transcrição e aguarda você pressionar `Enter`. Defina `"autoSubmit": true` no objeto de configurações `voice` para enviar o prompt automaticamente quando você soltar a tecla, desde que a transcrição tenha pelo menos três palavras.

<h2 id="tap-to-record-and-send">
  Tap to record and send
</h2>

O modo de toque alterna a gravação com um único pressionamento de tecla: toque uma vez para começar, fale e depois toque novamente para enviar o prompt. Não há aquecimento e você não precisa manter a tecla pressionada.

Ative o modo de toque com `/voice tap`. Com a entrada do prompt vazia, toque em `Space` para começar a gravar. O rodapé mostra uma forma de onda ao vivo durante a gravação. Toque em `Space` novamente para parar.

Claude Code insere a transcrição e envia o prompt automaticamente quando a transcrição tem pelo menos três palavras. Transcrições mais curtas são inseridas mas não enviadas, portanto um toque acidental não envia uma palavra isolada.

O limite de três palavras conta palavras para idiomas escritos sem espaços. A partir da v2.1.195, transcrições em japonês, chinês e tailandês contam palavras individuais, portanto elas são enviadas automaticamente no modo de toque e no modo de espera com `autoSubmit`. Versões anteriores contavam uma transcrição sem espaços como uma palavra e nunca a enviavam automaticamente.

O primeiro toque só começa a gravar quando a entrada do prompt está vazia, para que você ainda possa digitar espaços normalmente enquanto compõe uma mensagem. O segundo toque para a gravação independentemente do conteúdo da entrada. A gravação também para automaticamente após 15 segundos de silêncio ou dois minutos no total.

<h2 id="change-the-dictation-language">
  Alterar o idioma do ditado
</h2>

O ditado por voz usa a mesma [configuração `language`](/docs/pt/settings) que controla o idioma de resposta do Claude. Se essa configuração estiver vazia, o ditado usa o padrão em inglês. Na extensão VS Code, se `language` estiver vazio, o ditado usa a configuração `accessibility.voice.speechLanguage` do VS Code antes de usar o padrão em inglês.

<Accordion title="Idiomas de ditado suportados">
  | Idioma      | Código |
  | :---------- | :----- |
  | Tcheco      | `cs`   |
  | Dinamarquês | `da`   |
  | Holandês    | `nl`   |
  | Inglês      | `en`   |
  | Francês     | `fr`   |
  | Alemão      | `de`   |
  | Grego       | `el`   |
  | Hindi       | `hi`   |
  | Indonésio   | `id`   |
  | Italiano    | `it`   |
  | Japonês     | `ja`   |
  | Coreano     | `ko`   |
  | Norueguês   | `no`   |
  | Polonês     | `pl`   |
  | Português   | `pt`   |
  | Russo       | `ru`   |
  | Espanhol    | `es`   |
  | Sueco       | `sv`   |
  | Turco       | `tr`   |
  | Ucraniano   | `uk`   |
</Accordion>

Defina o idioma em `/config` ou diretamente nas configurações. Você pode usar o [código de idioma BCP 47](https://en.wikipedia.org/wiki/IETF_language_tag) ou o nome do idioma:

```json theme={null}
{
  "language": "japanese"
}
```

Se sua configuração `language` não estiver na lista de suporte, `/voice` avisa você ao ativar e volta para inglês para ditado. As respostas de texto do Claude não são afetadas por esse fallback.

<h2 id="rebind-the-dictation-key">
  Revinculação da tecla de ditado
</h2>

A tecla de ditado está vinculada a `voice:pushToTalk` no contexto `Chat` e usa como padrão `Space`. A mesma vinculação controla os modos de manutenção e toque. Revinculação em [`~/.claude/keybindings.json`](/docs/pt/keybindings):

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "meta+k": "voice:pushToTalk",
        "space": null
      }
    }
  ]
}
```

A ação `voice:pushToTalk` usa uma tecla por vez. Quando você vincula uma tecla personalizada, ela substitui a vinculação padrão `Space` em vez de adicionar um segundo gatilho, portanto a linha `"space": null` neste exemplo é para clareza e pode ser omitida sem alterar o comportamento.

No modo de manutenção, evite vincular uma tecla de letra simples como `v` pois a detecção de manutenção depende da repetição de tecla e a letra digita no prompt durante o aquecimento. Use `Space`, ou use uma combinação de modificador como `meta+k` para começar a gravar no primeiro pressionamento de tecla sem aquecimento. O modo de toque não tem aquecimento, portanto a maioria das teclas funciona.

Algumas teclas não são entregues a aplicativos de terminal e não podem ser vinculadas. Por exemplo, `Caps Lock` mostra um erro se você tentar vinculá-la. Consulte [customize keyboard shortcuts](/docs/pt/keybindings) para a sintaxe completa de vinculação de teclado e a lista de atalhos reservados.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Problemas comuns quando o ditado por voz não é ativado ou não grava:

* **`Voice mode requires a Claude.ai account`**: você está autenticado com uma chave API ou um provedor de terceiros. Execute `/login` para entrar com uma conta Claude.ai.
* **`Voice mode is disabled by your organization's policy`**: a configuração de conformidade da sua organização desativa o ditado por voz, conforme descrito em [Requirements](#requirements). Entre em contato com o administrador da sua organização para confirmar se o ditado por voz está disponível para sua organização.
* **`Microphone access is denied`**: conceda permissão de microfone ao seu terminal nas configurações do sistema. No macOS, vá para Configurações do Sistema → Privacidade e Segurança → Microfone e ative seu aplicativo de terminal, depois execute `/voice` novamente. No Windows, vá para Configurações → Privacidade e segurança → Microfone e ative o acesso ao microfone para aplicativos de desktop, depois execute `/voice` novamente. Se seu terminal não estiver listado nas configurações de Microfone do macOS, consulte [Terminal not listed in macOS Microphone settings](#terminal-not-listed-in-macos-microphone-settings).
* **`No audio recording tool found` no Linux**: o módulo de áudio nativo não conseguiu carregar e nenhum fallback está instalado. Instale SoX com o comando mostrado na mensagem de erro, por exemplo `sudo apt-get install sox`.
* **`Voice mode requires a microphone, but SoX could not open an audio capture device`**: SoX está instalado, mas o host não possui um dispositivo de captura de áudio, por exemplo um servidor sem cabeça ou um contêiner. Execute Claude Code em uma máquina com um microfone. A partir da v2.1.195, Claude Code no Linux relata esta mensagem nessa situação; versões anteriores pediam que você instalasse SoX mesmo quando já estava instalado.
* **`Voice mode could not find a working audio recorder in WSL`**: WSLg roteia áudio através do PulseAudio em vez de um dispositivo ALSA, portanto SoX precisa que seu backend PulseAudio esteja instalado explicitamente. Execute `sudo apt install sox libsox-fmt-pulse`. Instalar apenas `sox` puxa o backend ALSA, que não consegue gravar no WSL porque não há nenhum dispositivo `/dev/snd`.
* **`Voice input is failing repeatedly and has been paused`**: o ditado por voz atingiu várias falhas de captura seguidas e parou de tentar novas sessões até que uma tenha sucesso. Uma falha conta se o microfone falhar ao iniciar ou o gravador iniciar e depois parar sem produzir nenhum áudio. Isso geralmente significa que o microfone ou a pilha de áudio neste host não consegue capturar áudio, por exemplo um servidor sem cabeça, um shell remoto sem passagem de áudio, ou uma permissão de microfone negada. Confirme um dispositivo de entrada funcionando, corrija a causa subjacente das entradas acima, depois dispare a voz novamente. Antes da v2.1.202, apenas falhas de inicialização contavam para a pausa.
* **Nada acontece ao manter `Space` pressionado no modo de manutenção**: observe a entrada do prompt enquanto você mantém. Se espaços continuarem se acumulando, o ditado por voz provavelmente está desativado; execute `/voice hold` para ativá-lo. Se apenas um ou dois espaços aparecerem e depois nada, o ditado por voz está ativado mas a detecção de manutenção não está sendo acionada. A detecção de manutenção requer que seu terminal envie eventos de repetição de tecla, portanto não pode detectar uma tecla mantida se a repetição de tecla estiver desativada no nível do SO. Mude para o modo de toque com `/voice tap` para evitar o requisito de repetição de tecla.
* **Tocar `Space` digita um espaço em vez de gravar no modo de toque**: o primeiro toque só começa a gravar quando a entrada do prompt está vazia. Limpe a entrada primeiro, ou verifique se você está no modo de toque executando `/voice tap`.
* **`No audio detected from microphone`**: a gravação começou mas capturou silêncio. Confirme que o dispositivo de entrada correto está definido como padrão do sistema e que seu nível de entrada não está mudo ou próximo a zero. No Windows, abra Configurações → Sistema → Som → Entrada e selecione seu microfone. No macOS, abra Configurações do Sistema → Som → Entrada.
* **`Voice connection failed`**: sua gravação nunca chegou ao serviço de transcrição porque a conexão falhou. Verifique sua rede e tente novamente. Uma gravação que captura nenhum áudio relata `No audio detected from microphone` em vez desta mensagem. Antes da v2.1.200, um microfone silencioso poderia relatar uma falha de conexão, o que sugeria um problema de rede quando o problema real era o dispositivo de entrada.
* **`No speech detected`**: o áudio chegou ao serviço de transcrição mas nenhuma palavra foi reconhecida. Fale mais perto do microfone, reduza o ruído de fundo e confirme que seu [idioma de ditado](#change-the-dictation-language) corresponde ao idioma que você está falando.
* **A transcrição está distorcida ou no idioma errado**: o ditado usa o padrão em inglês. Se você estiver ditando em outro idioma, defina-o em `/config` primeiro. Consulte [Change the dictation language](#change-the-dictation-language).

<h3 id="terminal-not-listed-in-macos-microphone-settings">
  Terminal not listed in macOS Microphone settings
</h3>

Se seu aplicativo de terminal não aparecer em Configurações do Sistema → Privacidade e Segurança → Microfone, não há alternância que você possa ativar. Redefina o estado de permissão para seu terminal para que a próxima execução de `/voice` dispare um novo prompt de permissão do macOS.

<Steps>
  <Step title="Redefinir a permissão de microfone para seu terminal">
    Execute `tccutil reset Microphone <bundle-id>`, substituindo `<bundle-id>` pelo identificador do seu terminal: `com.apple.Terminal` para o Terminal integrado, ou `com.googlecode.iterm2` para iTerm2. Para outros terminais, procure o identificador com `osascript -e 'id of app "AppName"'`.

    <Warning>
      Você pode executar `tccutil reset Microphone` sem um ID de pacote, mas revoga o acesso ao microfone de todos os aplicativos no seu Mac, incluindo aplicativos como Zoom ou Slack. Cada aplicativo precisará solicitar acesso novamente no próximo uso, portanto não execute durante uma chamada ativa.
    </Warning>
  </Step>

  <Step title="Sair e relançar seu terminal">
    O macOS não solicitará novamente um processo que já está em execução. Saia do aplicativo de terminal com Cmd+Q, não apenas feche suas janelas, depois abra-o novamente.
  </Step>

  <Step title="Disparar um novo prompt">
    Inicie Claude Code e execute `/voice`. O macOS solicita acesso ao microfone; permita.
  </Step>
</Steps>

<h2 id="see-also">
  Veja também
</h2>

* [Personalize atalhos de teclado](/docs/pt/keybindings): revinculação `voice:pushToTalk` e outras ações de teclado CLI
* [Configure configurações](/docs/pt/settings): referência completa para `voice`, `language` e outras chaves de configurações
* [Modo interativo](/docs/pt/interactive-mode): atalhos de teclado, modos de entrada e controles de sessão
* [Comandos](/docs/pt/commands): referência para `/voice`, `/config` e todos os outros comandos
