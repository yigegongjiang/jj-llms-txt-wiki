> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Enviar eventos para uma sessão em execução com canais

> Use canais para enviar mensagens, alertas e webhooks para sua sessão Claude Code de um servidor MCP. Encaminhe resultados de CI, mensagens de chat e eventos de monitoramento para que Claude possa reagir enquanto você está ausente.

<Note>
  Os canais estão em [visualização de pesquisa](#research-preview). Eles exigem autenticação Anthropic através de claude.ai ou uma chave de API do Console, e não estão disponíveis no Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry. As organizações Team e Enterprise devem [habilitá-los explicitamente](#enterprise-controls).
</Note>

Um canal é um servidor MCP que envia eventos para sua sessão Claude Code em execução, para que Claude possa reagir a coisas que acontecem enquanto você não está no terminal. Os canais podem ser bidirecionais: Claude lê o evento e responde através do mesmo canal, como uma ponte de chat. Os eventos chegam apenas enquanto a sessão está aberta, portanto, para uma configuração sempre ativa, você executa Claude em um processo de fundo ou terminal persistente.

Ao contrário das integrações que geram uma nova sessão na nuvem ou aguardam para serem consultadas, o evento chega na sessão que você já tem aberta: veja [como os canais se comparam](#how-channels-compare).

Você instala um canal como um plugin e o configura com suas próprias credenciais. Telegram, Discord e iMessage estão incluídos na visualização de pesquisa.

Quando Claude responde através de um canal, você vê a mensagem de entrada em seu terminal, mas não o texto da resposta. O terminal mostra a chamada de ferramenta e uma confirmação (como "enviado"), e a resposta real aparece na outra plataforma.

Se você gerencia uma organização Team, Enterprise ou Console, consulte [Habilitar canais para sua organização](#enterprise-controls). Para criar seu próprio canal, consulte a [referência de Canais](/docs/pt/channels-reference).

<h2 id="supported-channels">
  Canais suportados
</h2>

Cada canal suportado é um plugin que requer [Bun](https://bun.sh). Para uma demonstração prática do fluxo de plugin antes de conectar uma plataforma real, tente o [quickstart fakechat](#quickstart).

<Tabs>
  <Tab title="Telegram">
    Veja o [código-fonte completo do plugin Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram).

    <Steps>
      <Step title="Criar um bot do Telegram">
        Abra [BotFather](https://t.me/BotFather) no Telegram e envie `/newbot`. Dê a ele um nome de exibição e um nome de usuário único terminando em `bot`. Copie o token que BotFather retorna.
      </Step>

      <Step title="Instalar o plugin">
        No Claude Code, execute:

        ```
        /plugin install telegram@claude-plugins-official
        ```

        Se Claude Code relatar que o plugin não foi encontrado em nenhum marketplace, seu marketplace está ausente ou desatualizado. Execute `/plugin marketplace update claude-plugins-official` para atualizá-lo, ou `/plugin marketplace add anthropics/claude-plugins-official` se você ainda não o adicionou. Em seguida, tente novamente a instalação.

        Após instalar, execute `/reload-plugins` para ativar o comando de configuração do plugin.
      </Step>

      <Step title="Configurar seu token">
        Execute o comando de configuração com o token do BotFather:

        ```
        /telegram:configure <token>
        ```

        Isso o salva em `~/.claude/channels/telegram/.env`. Você também pode definir `TELEGRAM_BOT_TOKEN` em seu ambiente de shell antes de iniciar Claude Code.
      </Step>

      <Step title="Reiniciar com canais habilitados">
        Saia do Claude Code e reinicie com a flag de canal. Isso inicia o plugin Telegram, que começa a pesquisar mensagens do seu bot:

        ```bash theme={null}
        claude --channels plugin:telegram@claude-plugins-official
        ```
      </Step>

      <Step title="Emparelhar sua conta">
        Abra o Telegram e envie qualquer mensagem para seu bot. O bot responde com um código de emparelhamento.

        <Note>Se seu bot não responder, certifique-se de que Claude Code está em execução com `--channels` da etapa anterior. O bot só pode responder enquanto o canal está ativo.</Note>

        De volta ao Claude Code, execute:

        ```
        /telegram:access pair <code>
        ```

        Em seguida, bloqueie o acesso para que apenas sua conta possa enviar mensagens:

        ```
        /telegram:access policy allowlist
        ```
      </Step>
    </Steps>
  </Tab>

  <Tab title="Discord">
    Veja o [código-fonte completo do plugin Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord).

    <Steps>
      <Step title="Criar um bot do Discord">
        Vá para o [Portal de Desenvolvedores do Discord](https://discord.com/developers/applications), clique em **New Application** e nomeie-o. Na seção **Bot**, crie um nome de usuário e clique em **Reset Token** e copie o token.
      </Step>

      <Step title="Habilitar Message Content Intent">
        Nas configurações do seu bot, role até **Privileged Gateway Intents** e habilite **Message Content Intent**.
      </Step>

      <Step title="Convidar o bot para seu servidor">
        Vá para **OAuth2 > URL Generator**. Selecione o escopo `bot` e habilite estas permissões:

        * View Channels
        * Send Messages
        * Send Messages in Threads
        * Read Message History
        * Attach Files
        * Add Reactions

        Abra a URL gerada para adicionar o bot ao seu servidor.
      </Step>

      <Step title="Instalar o plugin">
        No Claude Code, execute:

        ```
        /plugin install discord@claude-plugins-official
        ```

        Se Claude Code relatar que o plugin não foi encontrado em nenhum marketplace, seu marketplace está ausente ou desatualizado. Execute `/plugin marketplace update claude-plugins-official` para atualizá-lo, ou `/plugin marketplace add anthropics/claude-plugins-official` se você ainda não o adicionou. Em seguida, tente novamente a instalação.

        Após instalar, execute `/reload-plugins` para ativar o comando de configuração do plugin.
      </Step>

      <Step title="Configurar seu token">
        Execute o comando de configuração com o token do bot que você copiou:

        ```
        /discord:configure <token>
        ```

        Isso o salva em `~/.claude/channels/discord/.env`. Você também pode definir `DISCORD_BOT_TOKEN` em seu ambiente de shell antes de iniciar Claude Code.
      </Step>

      <Step title="Reiniciar com canais habilitados">
        Saia do Claude Code e reinicie com a flag de canal. Isso conecta o plugin Discord para que seu bot possa receber e responder a mensagens:

        ```bash theme={null}
        claude --channels plugin:discord@claude-plugins-official
        ```
      </Step>

      <Step title="Emparelhar sua conta">
        Envie uma mensagem direta para seu bot no Discord. O bot responde com um código de emparelhamento.

        <Note>Se seu bot não responder, certifique-se de que Claude Code está em execução com `--channels` da etapa anterior. O bot só pode responder enquanto o canal está ativo.</Note>

        De volta ao Claude Code, execute:

        ```
        /discord:access pair <code>
        ```

        Em seguida, bloqueie o acesso para que apenas sua conta possa enviar mensagens:

        ```
        /discord:access policy allowlist
        ```
      </Step>
    </Steps>
  </Tab>

  <Tab title="iMessage">
    Veja o [código-fonte completo do plugin iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage).

    O canal iMessage lê seu banco de dados de Mensagens diretamente e envia respostas através de AppleScript. Requer macOS e não precisa de token de bot ou serviço externo.

    <Steps>
      <Step title="Conceder Acesso Total ao Disco">
        O banco de dados de Mensagens em `~/Library/Messages/chat.db` é protegido pelo macOS. Na primeira vez que o servidor o lê, o macOS solicita acesso: clique em **Allow**. O prompt nomeia qualquer aplicativo que iniciou o Bun, como Terminal, iTerm ou seu IDE.

        Se o prompt não aparecer ou você clicou em Don't Allow, conceda acesso manualmente em **System Settings > Privacy & Security > Full Disk Access** e adicione seu terminal. Sem isso, o servidor sai imediatamente com `authorization denied`.
      </Step>

      <Step title="Instalar o plugin">
        No Claude Code, execute:

        ```
        /plugin install imessage@claude-plugins-official
        ```

        Se Claude Code relatar que o plugin não foi encontrado em nenhum marketplace, seu marketplace está ausente ou desatualizado. Execute `/plugin marketplace update claude-plugins-official` para atualizá-lo, ou `/plugin marketplace add anthropics/claude-plugins-official` se você ainda não o adicionou. Em seguida, tente novamente a instalação.
      </Step>

      <Step title="Reiniciar com canais habilitados">
        Saia do Claude Code e reinicie com a flag de canal:

        ```bash theme={null}
        claude --channels plugin:imessage@claude-plugins-official
        ```
      </Step>

      <Step title="Envie uma mensagem para si mesmo">
        Abra Mensagens em qualquer dispositivo conectado à sua Apple ID e envie uma mensagem para si mesmo. Ela chega ao Claude imediatamente: o auto-chat ignora o controle de acesso sem configuração.

        <Note>A primeira resposta que Claude envia dispara um prompt de Automação do macOS perguntando se seu terminal pode controlar Mensagens. Clique em **OK**.</Note>
      </Step>

      <Step title="Permitir outros remetentes">
        Por padrão, apenas suas próprias mensagens passam. Para permitir que outro contato alcance Claude, adicione seu identificador:

        ```
        /imessage:access allow +15551234567
        ```

        Os identificadores são números de telefone no formato `+country` ou e-mails de Apple ID como `user@example.com`.
      </Step>
    </Steps>
  </Tab>
</Tabs>

Você também pode [criar seu próprio canal](/docs/pt/channels-reference) para sistemas que ainda não têm um plugin.

<h2 id="quickstart">
  Quickstart
</h2>

Fakechat é um canal de demonstração oficialmente suportado que executa uma interface de chat no localhost, sem nada para autenticar e nenhum serviço externo para configurar.

Depois de instalar e habilitar fakechat, você pode digitar no navegador e a mensagem chega em sua sessão Claude Code. Claude responde e a resposta aparece de volta no navegador. Depois de testar a interface fakechat, tente [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram), [Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) ou [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage).

Para tentar a demonstração fakechat, você precisará de:

* Claude Code [instalado e autenticado](/docs/pt/quickstart#step-1-install-claude-code) com uma conta claude.ai ou uma chave de API do Console Claude
* [Bun](https://bun.sh) instalado. Os plugins de canal pré-construídos são scripts Bun. Verifique com `bun --version`; se isso falhar, [instale Bun](https://bun.sh/docs/installation).
* **Organização Team, Enterprise ou Console gerenciada**: seu administrador deve [habilitar canais](#enterprise-controls) nas configurações gerenciadas

<Steps>
  <Step title="Instalar o plugin de canal fakechat">
    Inicie uma sessão Claude Code e execute o comando de instalação:

    ```text theme={null}
    /plugin install fakechat@claude-plugins-official
    ```

    Se Claude Code relatar que o plugin não foi encontrado em nenhum marketplace, seu marketplace está ausente ou desatualizado. Execute `/plugin marketplace update claude-plugins-official` para atualizá-lo, ou `/plugin marketplace add anthropics/claude-plugins-official` se você ainda não o adicionou. Em seguida, tente novamente a instalação.
  </Step>

  <Step title="Reiniciar com o canal habilitado">
    Saia do Claude Code e reinicie com `--channels` e passe o plugin fakechat que você instalou:

    ```bash theme={null}
    claude --channels plugin:fakechat@claude-plugins-official
    ```

    O servidor fakechat inicia automaticamente.

    <Tip>
      Você pode passar vários plugins para `--channels`, separados por espaço.
    </Tip>
  </Step>

  <Step title="Enviar uma mensagem">
    Abra a interface fakechat em [http://localhost:8787](http://localhost:8787) e digite uma mensagem:

    ```text theme={null}
    hey, what's in my working directory?
    ```

    A mensagem chega em sua sessão Claude Code como um evento `<channel source="fakechat">`. Claude a lê, faz o trabalho e chama a ferramenta `reply` do fakechat. A resposta aparece na interface de chat.
  </Step>
</Steps>

Se Claude atingir um prompt de permissão enquanto você está longe do terminal, a sessão pausa até que você responda. Os servidores de canal que declaram a [capacidade de retransmissão de permissão](/docs/pt/channels-reference#relay-permission-prompts) podem encaminhar esses prompts para você para que você possa aprovar ou negar remotamente. Para uso sem supervisão, [`--dangerously-skip-permissions`](/docs/pt/permission-modes#skip-all-checks-with-bypasspermissions-mode) ignora a maioria dos prompts, mas use apenas em ambientes em que você confia. Regras de solicitação explícita, ferramentas de conector [que sua organização definiu como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) e ferramentas MCP marcadas como [`requiresUserInteraction`](/docs/pt/mcp#require-approval-for-a-specific-tool) ainda solicitam confirmação.

Quando você executa canais em modo não interativo com `-p`, ferramentas que precisam de entrada de terminal, como perguntas de múltipla escolha e aprovação de modo de plano, são desabilitadas para que a sessão nunca fique travada aguardando entrada.

<h2 id="security">
  Segurança
</h2>

Cada plugin de canal aprovado mantém uma lista de permissão de remetentes: apenas IDs que você adicionou podem enviar mensagens, e todos os outros são silenciosamente descartados.

Telegram e Discord inicializam a lista por emparelhamento:

1. Encontre seu bot no Telegram ou Discord e envie-lhe qualquer mensagem
2. O bot responde com um código de emparelhamento
3. Em sua sessão Claude Code, aprove o código quando solicitado
4. Seu ID de remetente é adicionado à lista de permissão

iMessage funciona de forma diferente: enviar uma mensagem para si mesmo ignora a porta automaticamente, e você adiciona outros contatos por identificador com `/imessage:access allow`.

Além disso, você controla quais servidores estão habilitados em cada sessão com `--channels`, e sua organização controla a disponibilidade com [`channelsEnabled`](#enterprise-controls) em planos Team e Enterprise de claude.ai e em organizações Console que implantam configurações gerenciadas.

Estar em `.mcp.json` não é suficiente para enviar mensagens: um servidor também deve ser nomeado em `--channels`.

A lista de permissão também controla a [retransmissão de permissão](/docs/pt/channels-reference#relay-permission-prompts) se o canal a declarar. Qualquer pessoa que possa responder através do canal pode aprovar ou negar o uso de ferramentas em sua sessão, portanto, apenas adicione à lista de permissão remetentes em quem você confia com essa autoridade.

<h2 id="enterprise-controls">
  Controles empresariais
</h2>

Os administradores controlam a disponibilidade através de duas [configurações gerenciadas](/docs/pt/settings) que os usuários não podem substituir. O padrão depende de como você se autentica:

* **claude.ai Team e Enterprise**: os canais são bloqueados até que um administrador os habilite.
* **Anthropic Console com autenticação de chave de API**: os canais são permitidos por padrão. Você só precisa dessa configuração se sua organização implanta configurações gerenciadas.

Em todos os casos, nenhum canal é executado até que um usuário o opte para a sessão com `--channels`.

| Configuração            | Propósito                                                                                                                                                                                                                                                                                                               | Quando não configurado                                                                                                                                                                                            |
| :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `channelsEnabled`       | Chave mestra. Deve ser `true` para que qualquer canal entregue mensagens. Defina através do toggle do [console de administração claude.ai](https://claude.ai/admin-settings/claude-code) ou diretamente nas configurações gerenciadas. Bloqueia todos os canais, incluindo a flag de desenvolvimento quando desativado. | claude.ai Team e Enterprise: canais bloqueados. Console: canais permitidos, a menos que sua organização implante configurações gerenciadas, caso em que os canais são bloqueados até que essa chave seja definida |
| `allowedChannelPlugins` | Quais plugins podem se registrar uma vez que os canais estão habilitados. Substitui a lista mantida pela Anthropic quando definido. Aplica-se apenas quando `channelsEnabled` é `true`.                                                                                                                                 | A lista padrão da Anthropic se aplica                                                                                                                                                                             |

Usuários Pro e Max sem uma organização ignoram essas verificações completamente: os canais estão disponíveis e os usuários optam por sessão com `--channels`.

<h3 id="enable-channels-for-your-organization">
  Habilitar canais para sua organização
</h3>

Os administradores podem habilitar canais em [**claude.ai → Admin settings → Claude Code → Channels**](https://claude.ai/admin-settings/claude-code), o que requer a função de administrador, ou definindo `channelsEnabled` como `true` nas configurações gerenciadas.

Uma vez habilitado, os usuários em sua organização podem usar `--channels` para optar por servidores de canal em sessões individuais. Se a configuração estiver desabilitada ou não definida, o servidor MCP ainda se conecta e suas ferramentas funcionam, mas as mensagens de canal não chegarão. Um aviso de inicialização informa ao usuário para ter um administrador habilitar a configuração.

<h3 id="restrict-which-channel-plugins-can-run">
  Restringir quais plugins de canal podem ser executados
</h3>

Por padrão, qualquer plugin na lista de permissão mantida pela Anthropic pode se registrar como um canal. Os administradores em planos Team e Enterprise podem substituir essa lista de permissão pela sua própria definindo `allowedChannelPlugins` nas configurações gerenciadas. Use isso para restringir quais plugins oficiais são permitidos, aprovar canais do seu próprio marketplace interno, ou ambos. Cada entrada nomeia um plugin e o marketplace de onde vem:

```json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" },
    { "marketplace": "claude-plugins-official", "plugin": "discord" },
    { "marketplace": "acme-corp-plugins", "plugin": "internal-alerts" }
  ]
}
```

Quando `allowedChannelPlugins` é definido, ele substitui completamente a lista de permissão da Anthropic: apenas os plugins listados podem se registrar. Deixe-o indefinido para voltar à lista de permissão padrão da Anthropic. Se você definir uma matriz vazia, você bloqueia todos os plugins de canal da lista de permissão, mas `--dangerously-load-development-channels` ainda pode contorná-lo para testes locais. Para bloquear canais completamente, incluindo a flag de desenvolvimento, deixe `channelsEnabled` indefinido.

Esta configuração requer `channelsEnabled: true`. Se um usuário passar um plugin para `--channels` que não esteja em sua lista, Claude Code inicia normalmente, mas o canal não se registra, e o aviso de inicialização explica que o plugin não está na lista aprovada da organização.

<h2 id="research-preview">
  Visualização de pesquisa
</h2>

Os canais são um recurso de visualização de pesquisa. A disponibilidade está sendo lançada gradualmente, e a sintaxe da flag `--channels` e o contrato de protocolo podem mudar com base no feedback.

Durante a visualização, `--channels` aceita apenas plugins de uma lista de permissão mantida pela Anthropic, ou da lista de permissão da sua organização se um administrador tiver definido [`allowedChannelPlugins`](#restrict-which-channel-plugins-can-run). Os plugins de canal em [claude-plugins-official](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins) são o conjunto aprovado padrão. Se você passar algo que não esteja na lista de permissão efetiva, Claude Code inicia normalmente, mas o canal não se registra, e o aviso de inicialização informa por quê.

Para testar um canal que você está criando, use `--dangerously-load-development-channels`. Veja [Testar durante a visualização de pesquisa](/docs/pt/channels-reference#test-during-the-research-preview) para informações sobre como testar canais personalizados que você cria.

Relate problemas ou feedback no [repositório GitHub do Claude Code](https://github.com/anthropics/claude-code/issues).

<h2 id="how-channels-compare">
  Como os canais se comparam
</h2>

Vários recursos do Claude Code se conectam a sistemas fora do terminal, cada um adequado para um tipo diferente de trabalho:

| Recurso                                          | O que faz                                                                  | Bom para                                                          |
| ------------------------------------------------ | -------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| [Claude Code na web](/docs/pt/claude-code-on-the-web) | Executa tarefas em uma nova sandbox na nuvem, clonada do GitHub            | Delegar trabalho assíncrono independente que você verifica depois |
| [Claude no Slack](/docs/pt/slack)                     | Gera uma sessão web a partir de uma menção `@Claude` em um canal ou thread | Iniciar tarefas diretamente do contexto de conversa da equipe     |
| Servidor [MCP](/docs/pt/mcp) padrão                   | Claude o consulta durante uma tarefa; nada é enviado para a sessão         | Dar ao Claude acesso sob demanda para ler ou consultar um sistema |
| [Remote Control](/docs/pt/remote-control)             | Você dirige sua sessão local de claude.ai ou do aplicativo móvel Claude    | Dirigir uma sessão em andamento enquanto está longe de sua mesa   |

Os canais preenchem a lacuna nessa lista enviando eventos de fontes não-Claude para sua sessão local já em execução.

* **Ponte de chat**: pergunte algo ao Claude do seu telefone via Telegram, Discord ou iMessage, e a resposta volta no mesmo chat enquanto o trabalho é executado em sua máquina contra seus arquivos reais.
* **[Receptor de webhook](/docs/pt/channels-reference#example-build-a-webhook-receiver)**: um webhook de CI, seu rastreador de erros, um pipeline de implantação ou outro serviço externo chega onde Claude já tem seus arquivos abertos e se lembra do que você estava depurando.

<h2 id="next-steps">
  Próximas etapas
</h2>

Depois de ter um canal em execução, explore esses recursos relacionados:

* [Criar seu próprio canal](/docs/pt/channels-reference) para sistemas que ainda não têm plugins
* [Remote Control](/docs/pt/remote-control) para dirigir uma sessão local do seu telefone em vez de encaminhar eventos para ela
* [Tarefas agendadas](/docs/pt/scheduled-tasks) para pesquisar em um cronômetro em vez de reagir a eventos enviados
