> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Conectar Claude Code a ferramentas via MCP

> Aprenda como conectar Claude Code às suas ferramentas com o Model Context Protocol.

Claude Code pode se conectar a centenas de ferramentas e fontes de dados externas através do [Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction), um padrão de código aberto para integrações de IA com ferramentas. Os servidores MCP dão ao Claude Code acesso às suas ferramentas, bancos de dados e APIs.

Conecte um servidor quando você se encontrar copiando dados para o chat de outra ferramenta, como um rastreador de problemas ou um painel de monitoramento. Uma vez conectado, Claude pode ler e agir nesse sistema diretamente em vez de trabalhar com o que você cola.

Se você está conectando seu primeiro servidor, comece com o [guia de início rápido do MCP](/docs/pt/mcp-quickstart) para um passo a passo detalhado. Esta página é a referência completa.

<h2 id="what-you-can-do-with-mcp">
  O que você pode fazer com MCP
</h2>

Com servidores MCP conectados, você pode pedir ao Claude Code para:

* **Implementar recursos de rastreadores de problemas**: "Adicione o recurso descrito no problema JIRA ENG-4521 e crie um PR no GitHub."
* **Analisar dados de monitoramento**: "Verifique Sentry e Statsig para verificar o uso do recurso descrito em ENG-4521."
* **Consultar bancos de dados**: "Encontre emails de 10 usuários aleatórios que usaram o recurso ENG-4521, com base no nosso banco de dados PostgreSQL."
* **Integrar designs**: "Atualize nosso modelo de email padrão com base nos novos designs do Figma que foram postados no Slack"
* **Automatizar fluxos de trabalho**: "Crie rascunhos do Gmail convidando esses 10 usuários para uma sessão de feedback sobre o novo recurso."
* **Reagir a eventos externos**: Um servidor MCP também pode atuar como um [canal](/docs/pt/channels) que envia mensagens para sua sessão, para que Claude reaja a mensagens do Telegram, chats do Discord ou eventos de webhook enquanto você está ausente.

<h2 id="find-and-build-mcp-servers">
  Encontre e crie servidores MCP
</h2>

Navegue por conectores revisados no [Diretório Anthropic](https://claude.ai/directory). Os conectores do Diretório usam a mesma infraestrutura MCP que Claude Code, então você pode adicionar qualquer servidor remoto listado lá com `claude mcp add`.

<Warning>
  Verifique se você confia em cada servidor antes de conectá-lo. Servidores que buscam conteúdo externo podem expô-lo ao [risco de injeção de prompt](/docs/pt/security#protect-against-prompt-injection).
</Warning>

Para criar seu próprio servidor, consulte o [guia do servidor MCP](https://modelcontextprotocol.io/docs/develop/build-server) para os fundamentos do protocolo e a [documentação de construção de conectores Claude](https://claude.com/docs/connectors/building) para autenticação, testes e envio ao Diretório.

Você também pode fazer com que Claude crie um servidor para você com o plugin oficial [`mcp-server-dev`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/mcp-server-dev).

<Steps>
  <Step title="Instale o plugin">
    Em uma sessão Claude Code, execute:

    ```
    /plugin install mcp-server-dev@claude-plugins-official
    ```

    Se a instalação falhar, corresponda à mensagem que Claude Code relata:

    * `Marketplace "claude-plugins-official" não encontrado`: adicione o marketplace com `/plugin marketplace add anthropics/claude-plugins-official`, depois tente novamente a instalação.
    * O plugin [não foi encontrado no marketplace](/docs/pt/plugins/install#install-a-plugin): verifique o nome do plugin.

    Se o resumo da instalação relatar `Run /reload-plugins to activate.`, Claude Code então executa esse recarregamento para você. Se o recarregamento avisar que sua próxima mensagem releria a conversa, execute `/reload-plugins --force`.
  </Step>

  <Step title="Execute a skill de construção">
    ```
    /mcp-server-dev:build-mcp-server
    ```

    Claude pergunta sobre seu caso de uso e cria um servidor HTTP remoto ou servidor stdio local.
  </Step>
</Steps>

<h2 id="installing-mcp-servers">
  Instalando servidores MCP
</h2>

Os servidores MCP podem ser configurados de várias maneiras dependendo de suas necessidades:

<h3 id="option-1-add-a-remote-http-server">
  Opção 1: Adicionar um servidor HTTP remoto
</h3>

Os servidores HTTP são a opção recomendada para conectar a servidores MCP remotos. Este é o transporte mais amplamente suportado para serviços baseados em nuvem.

```bash theme={null}
# Sintaxe básica
claude mcp add --transport http <name> <url>

# Exemplo real: Conectar ao Notion
claude mcp add --transport http notion https://mcp.notion.com/mcp

# Exemplo com token Bearer
claude mcp add --transport http secure-api https://api.example.com/mcp \
  --header "Authorization: Bearer your-token"
```

Ao configurar servidores MCP via JSON em `.mcp.json`, `~/.claude.json`, ou `claude mcp add-json`, o campo `type` aceita `streamable-http` como um alias para `http`. A especificação MCP usa o nome `streamable-http` para este transporte, portanto as configurações copiadas da documentação do servidor funcionam sem modificação.

Uma entrada JSON que tem uma `url` mas nenhum `type` é um erro de configuração, porque Claude Code lê uma entrada sem `type` como um servidor stdio. Claude Code pula esse servidor e relata `MCP server "<name>" has a "url" but no "type"; add "type": "http" (or "sse" / "ws") to this entry`. Antes da v2.1.202, Claude Code relatava essa configuração incorreta como `command: expected string, received undefined`.

Em execuções `--output-format stream-json`, Claude Code também relata uma entrada `--mcp-config` pulada no campo [`mcp_server_errors` do evento `system/init`](/docs/pt/headless#stream-responses), para que scripts possam detectar que o servidor nunca foi carregado. Isso requer Claude Code v2.1.219 ou posterior.

<h3 id="option-2-add-a-remote-sse-server">
  Opção 2: Adicionar um servidor SSE remoto
</h3>

<Warning>
  O transporte SSE (Server-Sent Events) está descontinuado. Use servidores HTTP em vez disso, quando disponível.
</Warning>

Alguns serviços ainda expõem apenas um endpoint SSE. Adicione-os com o mesmo comando `claude mcp add --transport http <name> <url>` que um [servidor HTTP](#option-1-add-a-remote-http-server). Claude Code tenta o transporte HTTP primeiro e muda para SSE quando o servidor não o aceita. A mudança automática requer Claude Code v2.1.265 ou posterior.

Em uma versão anterior, ou para conectar sobre SSE diretamente, passe `--transport sse` em vez disso:

```bash theme={null}
# Sintaxe básica
claude mcp add --transport sse <name> <url>

# Exemplo real: Conectar ao Asana
claude mcp add --transport sse asana https://mcp.asana.com/sse

# Exemplo com cabeçalho de autenticação
claude mcp add --transport sse private-api https://api.company.com/sse \
  --header "X-API-Key: your-key-here"
```

<h3 id="option-3-add-a-local-stdio-server">
  Opção 3: Adicionar um servidor stdio local
</h3>

Os servidores Stdio são executados como processos locais em sua máquina. Eles são ideais para ferramentas que precisam de acesso direto ao sistema ou scripts personalizados.

Claude Code define `CLAUDE_PROJECT_DIR` no ambiente do servidor gerado para a raiz do projeto, para que seu servidor possa resolver caminhos relativos ao projeto sem depender do diretório de trabalho. Este é o mesmo diretório que hooks recebem em sua variável `CLAUDE_PROJECT_DIR`. Leia-o de dentro do seu processo de servidor, por exemplo `process.env.CLAUDE_PROJECT_DIR` em Node ou `os.environ["CLAUDE_PROJECT_DIR"]` em Python.

`CLAUDE_PROJECT_DIR` é a raiz do projeto estável e não muda quando você adiciona ou remove diretórios de trabalho no meio da sessão. Um servidor que limita seu próprio acesso ao sistema de arquivos a um conjunto de diretórios permitidos deve implementar a solicitação MCP `roots/list` em vez disso. Claude Code responde a `roots/list` com o diretório de inicialização da sessão mais cada [diretório de trabalho adicional](/docs/pt/permissions#working-directories) que você concedeu com `--add-dir`, `/add-dir`, ou a configuração `additionalDirectories`. Claude Code envia `notifications/roots/list_changed` quando esse conjunto muda. Antes da v2.1.203, `roots/list` retornava apenas o diretório de inicialização e Claude Code não enviava `notifications/roots/list_changed`.

Esta variável é definida no ambiente do servidor, não no ambiente do próprio Claude Code, portanto fazer referência a ela via expansão `${VAR}` no `command` ou `args` de uma entrada `.mcp.json` com escopo de projeto ou uma entrada de servidor com escopo local ou de usuário em `~/.claude.json` requer um padrão como `${CLAUDE_PROJECT_DIR:-.}`. As configurações MCP fornecidas por plugins substituem `${CLAUDE_PROJECT_DIR}` diretamente e não precisam do padrão.

```bash theme={null}
# Sintaxe básica
claude mcp add [options] <name> -- <command> [args...]

# Exemplo real: Adicionar servidor Airtable
claude mcp add --env AIRTABLE_API_KEY=YOUR_KEY --transport stdio airtable \
  -- npx -y airtable-mcp-server
```

<Note>
  **Importante: Separe argumentos do servidor com `--`**

  Para servidores stdio, o `--` (duplo travessão) separa as próprias opções do Claude, como `--transport`, `--env`, e `--scope`, do comando e argumentos que executam o servidor. Tudo após `--` é passado para o servidor intocado.

  Por exemplo:

  * `claude mcp add --transport stdio myserver -- npx server` → executa `npx server`
  * `claude mcp add --env KEY=value --transport stdio myserver -- python server.py --port 8080` → executa `python server.py --port 8080` com `KEY=value` no ambiente

  Sem `--`, Claude Code tentaria analisar os sinalizadores do servidor, como `--port` acima, como suas próprias opções.

  `--env` aceita múltiplos pares `KEY=value`. Se o nome do servidor vem imediatamente após `--env`, a CLI lê o nome como outro par e o rejeita, portanto coloque pelo menos uma outra opção, como `--transport stdio`, entre `--env` e o nome do servidor.
</Note>

<h3 id="option-4-add-a-remote-websocket-server">
  Opção 4: Adicionar um servidor WebSocket remoto
</h3>

Os servidores WebSocket mantêm uma conexão bidirecional persistente, o que é adequado para servidores MCP remotos que enviam eventos para Claude sem solicitação. Use HTTP em vez disso quando seu servidor apenas responde a solicitações, já que HTTP suporta OAuth e o sinalizador `claude mcp add --transport`, enquanto WebSocket não suporta nenhum dos dois.

Configure servidores WebSocket em `.mcp.json` ou com `claude mcp add-json`:

```bash theme={null}
claude mcp add-json events-server \
  '{"type":"ws","url":"wss://mcp.example.com/socket","headers":{"Authorization":"Bearer YOUR_TOKEN"}}'
```

A entrada `type: "ws"` aceita os mesmos campos `url`, `headers`, `headersHelper`, `timeout`, e `alwaysLoad` que `http`. A autenticação é apenas por cabeçalho, portanto passe um token estático em `headers` ou gere um no momento da conexão com [`headersHelper`](#use-dynamic-headers-for-custom-authentication). O sinalizador `claude mcp add --transport` não aceita `ws`.

<h3 id="add-a-server-from-setup-instructions-written-for-another-client">
  Adicionar um servidor a partir de instruções de configuração escritas para outro cliente
</h3>

Os servidores MCP não são específicos do Claude Code, portanto as instruções de configuração de um servidor podem ser escritas para Claude Desktop, Cursor, ou outro cliente MCP e não fornecer nenhum comando `claude mcp add`. Para adicionar o servidor mesmo assim, procure nessas instruções por uma destas três coisas:

* **Uma URL** como `https://mcp.example.com/mcp`: o servidor é remoto.
* **Um comando de inicialização** como `npx -y @example/mcp-server`: o servidor é executado em sua máquina.
* **Um bloco JSON `mcpServers`**: configuração escrita para o arquivo de configurações de outro cliente.

Cada um é uma das entradas que as quatro opções em [Instalando servidores MCP](#installing-mcp-servers) usam. Encontre a forma que você tem abaixo para transformá-la no comando que Claude Code aceita. Cada comando escreve no [escopo local](#local-scope) a menos que você adicione `--scope project` ou `--scope user`.

<h4 id="from-a-url">
  De uma URL
</h4>

Uma URL significa que o servidor é remoto. Para um endpoint `https://`, adicione-o com `--transport http`, ou siga a [Opção 2](#option-2-add-a-remote-sse-server) quando as instruções disserem que o endpoint usa SSE. Para um endpoint `wss://`, use a [Opção 4](#option-4-add-a-remote-websocket-server) em vez disso, já que `--transport` não aceita `ws`:

```bash theme={null}
claude mcp add --transport http example https://mcp.example.com/mcp
```

Se as instruções também fornecerem uma chave de API ou cabeçalho de token, passe-o com `--header` como mostrado na [Opção 1](#option-1-add-a-remote-http-server).

<h4 id="from-an-npx-uvx-or-binary-command">
  De um comando `npx`, `uvx`, ou binário
</h4>

Um comando de inicialização significa que o servidor é executado como um processo stdio local. Coloque o comando inteiro após `--`, para que Claude Code passe sinalizadores como `-y` para o comando que inicia o servidor em vez de lê-los como suas próprias opções. Passe quaisquer variáveis de ambiente que as instruções peçam com `--env`, após o nome do servidor e antes de `--`:

```bash theme={null}
claude mcp add example --env API_KEY=your-key -- npx -y @example/mcp-server
```

A [Opção 3](#option-3-add-a-local-stdio-server) cobre o separador `--` completamente.

<h4 id="from-an-mcpservers-json-block">
  De um bloco JSON `mcpServers`
</h4>

Um bloco `mcpServers` escrito para outro cliente MCP, como Claude Desktop, usa a chave wrapper e a forma de entrada que Claude Code lê. Passe `claude mcp add-json` o objeto dentro de `mcpServers`, não o wrapper. Duas entradas precisam de um reparo primeiro:

* **Uma `url` sem `type`**: adicione `"type": "http"`, `"type": "sse"`, ou `"type": "ws"` para corresponder ao endpoint. Claude Code lê uma entrada sem `type` como um servidor stdio, portanto uma entrada `url` sem `type` falha.
* **Uma chave com caracteres diferentes de letras, números, hífens e sublinhados**: escolha um nome de servidor que use apenas esses caracteres. Caso contrário, a chave é o nome do servidor.

Por exemplo, este bloco:

```json theme={null}
{
  "mcpServers": {
    "example": {
      "command": "npx",
      "args": ["-y", "@example/mcp-server"]
    }
  }
}
```

torna-se este comando:

```bash theme={null}
claude mcp add-json example '{"command":"npx","args":["-y","@example/mcp-server"]}'
```

[Adicionar servidores MCP a partir de configuração JSON](#add-mcp-servers-from-json-configuration) cobre escape de shell e o sinalizador `--scope` para `add-json`. Para compartilhar o servidor com sua equipe em vez disso, adicione `--scope project`, ou adicione a entrada sob `mcpServers` em `.mcp.json` na raiz do seu projeto e faça commit. [Escopo de projeto](#project-scope) cobre como Claude Code carrega e aprova esse arquivo.

Cada comando `claude mcp add` e `claude mcp add-json` imprime uma linha `Added ...`. Para verificar que Claude Code se conectou, execute `claude mcp get <name>`; [Status do servidor](#server-status) cobre os status que ele mostra e a etapa de aprovação para servidores `.mcp.json`.

<h3 id="managing-your-servers">
  Gerenciando seus servidores
</h3>

Uma vez configurados, você pode gerenciar seus servidores MCP com estes comandos:

```bash theme={null}
# Listar todos os servidores configurados
claude mcp list

# Obter detalhes para um servidor específico
claude mcp get notion

# Remover um servidor
claude mcp remove notion

# (dentro do Claude Code) Verificar status do servidor
/mcp
```

Quando você remove um servidor remoto, Claude Code também exclui os tokens OAuth e o registro de cliente que armazenou para esse servidor.

<h4 id="server-status">
  Status do servidor
</h4>

`claude mcp add` confirma uma adição bem-sucedida imprimindo uma linha `Added ...`, o que significa que a configuração foi escrita. `claude mcp list` então mostra um status de saúde ao lado de cada servidor que lista, como `✔ Connected`, `! Needs authentication`, ou `✘ Failed to connect`. Um status de falha significa que Claude Code não conseguiu se conectar a esse servidor, não que o comando list falhou.

Os status nesta lista relatam uma decisão de configuração em vez de uma tentativa de conexão, portanto Claude Code os imprime sem se conectar ao servidor:

* ``⏸ Pending approval (run `claude` to approve)``: um servidor com escopo de projeto de `.mcp.json` que você ainda não aprovou. Claude Code o mostra em `claude mcp list` e `claude mcp get <name>`. Execute `claude` interativamente para revisar e aprovar.
* `✘ Rejected (see disabledMcpjsonServers in settings)`: um servidor `.mcp.json` que uma entrada [`disabledMcpjsonServers`](/docs/pt/settings-reference#disabledmcpjsonservers) rejeita. Claude Code o mostra apenas em `claude mcp get <name>`.
* `⊘ Disabled for this project (re-enable via /mcp)`: um servidor que a lista [`disabledMcpServers`](#disable-a-server-without-removing-it) do projeto nomeia. Claude Code o mostra em `claude mcp list` e `claude mcp get <name>`. Ative o servidor novamente no painel `/mcp`. Antes da v2.1.238, ambos os comandos se conectavam a um servidor desabilitado para verificar sua saúde e relatavam o resultado da conexão.

Os servidores WebSocket não aparecem na saída de `claude mcp list`. Use `claude mcp get <name>` ou o painel `/mcp` para verificá-los.

<h4 id="project-server-approvals-and-workspace-trust">
  Aprovações de servidor de projeto e confiança do workspace
</h4>

A partir da v2.1.196, `claude mcp list` e `claude mcp get` leem aprovações `.mcp.json` apenas de arquivos de configurações que não são verificados no repositório até que você confie no workspace executando `claude` nele e aceitando o diálogo de confiança do workspace. Um repositório clonado não pode aprovar seus próprios servidores: [`enableAllProjectMcpServers`](/docs/pt/settings-reference#enableallprojectmcpservers) ou [`enabledMcpjsonServers`](/docs/pt/settings-reference#enabledmcpjsonservers) confirmados no `.claude/settings.json` do projeto é ignorado em uma pasta não confiável, e o servidor permanece em `⏸ Pending approval` em vez de ser conectado e verificado quanto à saúde.

As aprovações dessas fontes ainda se aplicam em uma pasta não confiável:

* seu `~/.claude/settings.json` de usuário
* configurações gerenciadas
* configurações passadas com `--settings`

Claude Code também aplica aprovações de um `.claude/settings.local.json` não rastreado, mas executa git para verificar se o arquivo é rastreado, e executa essa verificação apenas em uma [pasta confiável](/docs/pt/permissions#project-allow-rules-and-workspace-trust). Em uma pasta que você nunca confiou, Claude Code aguarda o diálogo de confiança antes de aplicar as aprovações do arquivo, a menos que a pasta seja seu próprio diretório de configuração: seu diretório inicial, ou um diretório cujo `.claude` você definiu como [`CLAUDE_CONFIG_DIR`](/docs/pt/env-vars). Antes da v2.1.207, Claude Code aplicava aprovações de um `.claude/settings.local.json` não rastreado mesmo em uma pasta que você nunca tinha confiado.

Uma entrada `disabledMcpjsonServers` em qualquer arquivo de configurações ainda rejeita o servidor.

<h4 id="server-status-detail">
  Detalhe do status do servidor
</h4>

Em `/mcp`, incluindo o menu de um servidor lá, e no [gerenciador `/plugin`](/docs/pt/plugins/install), um servidor HTTP ou SSE remoto que você usou antes pode mostrar um status `cached` como `cached 2h ago · connects on first use · 5 tools`. Claude Code carregou a lista de ferramentas do servidor de seu cache de descoberta, salvo em uma sessão anterior, em vez de se conectar na inicialização, e Claude Code conecta o servidor na primeira vez que Claude chama uma das ferramentas do servidor. As ferramentas estão disponíveis a partir de sua primeira mensagem, portanto você não precisa fazer nada. O cache de descoberta e seu status `cached` requerem Claude Code v2.1.221 ou posterior.

O cache de descoberta está desativado por padrão a menos que um lançamento gradual o tenha ativado para sua conta. Defina [`MCP_DISCOVERY_CACHE=1`](/docs/pt/env-vars) para ativá-lo, ou `0` para mantê-lo desativado mesmo quando o lançamento o tiver ativado. Antes da v2.1.238, o cache estava ativado por padrão.

Duas ações no menu de um servidor em `/mcp` também afetam a entrada de cache desse servidor:

* **Reconnect**: em um servidor `cached`, Claude Code o conecta agora em vez de em sua primeira chamada de ferramenta e mantém a entrada. Em um servidor conectado ou com falha, Claude Code o reconecta e também descarta a entrada.
* **Clear authentication**: Claude Code revoga a autenticação do servidor e também descarta a entrada.

Após descartar a entrada, Claude Code busca a lista de ferramentas do servidor do servidor em vez de do cache.

Quando o status de um servidor é `✘ Failed to connect`, `claude mcp list` acrescenta o detalhe da falha a essa linha de status, e `claude mcp get <name>` o mostra em uma linha `Issue:`: o status HTTP ou código de erro, mais qualquer texto de erro que o servidor retornou. A visualização de detalhe do servidor em `/mcp` inclui o mesmo texto relatado pelo servidor em sua linha `Issue:`. Claude Code redige texto semelhante a credenciais deste detalhe e nunca inclui a URL do servidor expandida, que pode carregar segredos. Claude Code não acrescenta detalhe a um status `✘ Connection error`, porque o texto de exceção que imprimiria lá pode incorporar essa URL. Antes da v2.1.219, ambos os comandos mostravam apenas o status de falha simples, sem o código de status ou o texto de erro do servidor.

Quando você completa a autenticação de `/mcp` e a conexão ainda falha com um status HTTP ou um código de erro de transporte, Claude Code adiciona esse código e a origem da URL que tentou à mensagem que imprime após a tentativa. A origem é o esquema e host, mais a porta quando a URL nomeia uma, como `https://mcp.example.com`.

* O caminho e a consulta nunca aparecem nessa mensagem.
* Para um servidor na [escopo](#mcp-installation-scopes) local, de projeto, ou de usuário ou em configuração MCP gerenciada, a origem mostra o host como escrito nessa configuração, portanto uma referência `${VAR}` no host não é expandida na mensagem.
* Para uma falha sem status ou código de erro, Claude Code mostra o texto de erro sem a origem.

Um servidor remoto cuja configuração tem uma `url` vazia mostra como `not configured` em `/mcp`, em `claude mcp list`, e no [gerenciador `/plugin`](/docs/pt/plugins/install), e Claude Code não tenta se conectar a ele. Um plugin pode incluir uma entrada de espaço reservado como esta para um conector que você configura depois, portanto Claude Code não a relata como um erro ou um problema de configuração. A visualização de detalhe do servidor em `/mcp` lê `No URL configured for this server`; defina a `url` da entrada para conectá-lo. Antes da v2.1.208, Claude Code relatava uma `url` vazia como um problema de configuração com um prompt para reconectar.

<h4 id="configuration-warnings">
  Avisos de configuração
</h4>

Claude Code avisa sobre os problemas de configuração abaixo. Cada entrada diz o que Claude Code verifica e como limpar o aviso:

* **Espaço em branco oculto**: Claude Code avisa quando um valor de configuração MCP carrega espaço em branco oculto à esquerda ou à direita, que frequentemente vem de colar um token com uma quebra de linha à direita. Claude Code verifica `command`, `url`, cada entrada `args`, e os valores e nomes de chave sob `env` e `headers`. Claude Code mostra o aviso na saída de `claude mcp list` e em `/mcp`, nomeando os campos afetados sem ecoar seus valores, por exemplo `Leading or trailing whitespace in: headers.Authorization`. Claude Code não aparenta o espaço em branco e usa os valores exatamente como escritos, portanto edite a configuração para removê-lo.
* **Mesmo nome em mais de um escopo**: se você definir o mesmo nome de servidor em mais de um [escopo](#mcp-installation-scopes) com endpoints diferentes, Claude Code avisa sobre o conflito na saída de `claude mcp list` e em `/mcp`. Claude Code armazena logins OAuth por endpoint, portanto quando você autentica a definição que carrega em um projeto, você ainda precisa fazer login separadamente em um projeto onde uma definição diferente carrega. Mantenha o endpoint que você quer e remova os outros com `claude mcp remove <name> --scope <scope>`. No aviso, Claude Code cita o endpoint de cada escopo como escrito em sua configuração, com referências [`${VAR}`](#environment-variable-expansion-in-mcp-json) não expandidas, portanto nunca mostra um valor resolvido como uma chave de API.
* **Nomes reservados**: Claude Code reserva os nomes de seus servidores integrados, incluindo `workspace`, `claude-in-chrome`, `computer-use`, `Claude Preview`, e `Claude Browser`. Se sua configuração definir um servidor com um nome reservado, Claude Code o pula no tempo de carregamento e mostra um aviso pedindo que você o renomeie. `claude mcp add` rejeita um nome reservado com um erro. `Claude Preview` e `Claude Browser` ambos nomeiam o servidor integrado que o [painel de visualização do aplicativo de desktop Claude Code](/docs/pt/desktop#preview-your-app) usa. Antes da v2.1.205, `Claude Browser` não era reservado, portanto um servidor configurado pelo usuário poderia se registrar sob esse nome.
* **Variável de ambiente ausente**: se uma referência [`${VAR}`](#environment-variable-expansion-in-mcp-json) na configuração de um servidor nomeia uma variável que não está definida e não tem `:-default`, Claude Code avisa na saída de `claude mcp list` e em `/mcp`, nomeando a variável, e ainda carrega o servidor com o texto `${VAR}` não expandido. Defina a variável ou adicione um fallback `${VAR:-default}`. Em uma URL remota do servidor e `headers`, algumas variáveis de credenciais [leem como vazias](#credential-variables-that-read-as-empty) em vez disso, sem aviso.

<h4 id="tool-availability">
  Disponibilidade de ferramentas
</h4>

O painel `/mcp` mostra a contagem de ferramentas ao lado de cada servidor conectado e sinaliza servidores que anunciam a capacidade de ferramentas mas não expõem ferramentas.

Se sua solicitação precisa de ferramentas de um servidor que ainda está se conectando em segundo plano, Claude aguarda esse servidor antes de continuar. Como a espera acontece depende de sua configuração:

* **Com [busca de ferramentas](#scale-with-mcp-tool-search), o padrão**: a espera acontece dentro da chamada `ToolSearch`.
* **Sem busca de ferramentas**: Claude usa a ferramenta `WaitForMcpServers` em vez disso. As configurações sem busca de ferramentas incluem um `ANTHROPIC_BASE_URL` personalizado, `ENABLE_TOOL_SEARCH=false`, e um modelo anterior à geração Claude 4.5 na Agent Platform do Google Cloud.
* **Em uma implantação Microsoft Foundry [hospedada no Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)**: Claude começa no caminho de busca de ferramentas em vez de com `WaitForMcpServers`, já que Claude Code descobre a rejeição do lado do servidor apenas da API. Depois que Claude Code muda essa implantação para [carregamento antecipado](#scale-with-mcp-tool-search), as ferramentas de um servidor que termina de se conectar ficam disponíveis na próxima solicitação do Claude.

Com busca de ferramentas ativada, quando um servidor termina de se conectar enquanto Claude está trabalhando, Claude Code lista os nomes de ferramentas do servidor para Claude em sua próxima solicitação no mesmo turno. Claude pode então procurar e chamar essas ferramentas sem aguardar sua próxima mensagem.

<h3 id="disable-a-server-without-removing-it">
  Desabilitar um servidor sem removê-lo
</h3>

Alterne um servidor no painel `/mcp` para impedir que Claude Code se conecte a ele sem perder sua configuração. Claude Code ainda lista o servidor em `/mcp`, marcado como desabilitado.

Quando você alterna um servidor, Claude Code registra sua escolha por projeto em `~/.claude.json`, em uma de duas listas que cobrem conjuntos disjuntos de servidores:

* `disabledMcpServers`: uma lista de exclusão para servidores configurados pelo usuário, servidores de plugin, servidores que sua organização [fornece através de configurações gerenciadas](/docs/pt/managed-mcp#provide-servers-through-managed-settings), os conectores claude.ai que Claude Code [busca a si mesmo](#how-connectors-reach-claude-code), e servidores integrados que padrão para ativado. Claude Code não se conecta a um servidor que você lista aqui. Quando você desabilita um conector claude.ai com o alternador `/mcp` por projeto descrito em [Desabilitar conectores claude.ai](#disable-claude-ai-connectors), Claude Code o escreve nesta lista sob seu nome de exibição, por exemplo `claude.ai Slack`.
* `enabledMcpServers`: uma lista de inclusão para servidores integrados que padrão para desabilitado, como `computer-use`. Claude Code se conecta a um servidor padrão-desabilitado apenas quando você o lista aqui.

Claude Code consulta exatamente uma das duas listas para cada servidor, portanto nenhuma lista substitui a outra. Se você adicionar um servidor regular a `enabledMcpServers`, ou um servidor integrado padrão-desabilitado a `disabledMcpServers`, Claude Code ignora a entrada.

`disabledMcpServers` e `enabledMcpServers` não estão relacionados a [`enabledMcpjsonServers`](/docs/pt/settings-reference#enabledmcpjsonservers) e [`disabledMcpjsonServers`](/docs/pt/settings-reference#disabledmcpjsonservers), que controlam a aprovação de servidores definidos no arquivo `.mcp.json` de um projeto.

<h3 id="mcp-client-runtimes">
  MCP client runtimes
</h3>

Claude Code se conecta a servidores MCP através de um de dois tempos de execução do cliente. O tempo de execução v1 é construído no MCP TypeScript SDK 1.x. O tempo de execução v2 é o mesmo código no [MCP TypeScript SDK 2.0](https://ts.sdk.modelcontextprotocol.io/v2/), que adiciona revisão de protocolo MCP 2026-07-28. O resto desta página se aplica a ambos os tempos de execução, exceto onde uma seção nomeia o tempo de execução v2.

Claude Code escolhe um tempo de execução cada vez que você o inicia e o mantém até você sair. Em sessões onde ele [busca sinalizadores de recurso](/docs/pt/env-vars#features-that-need-feature-flag-fetching), ele usa o tempo de execução v2 no Claude Code v2.1.232 ou posterior.

Nas sessões onde ele não busca sinalizadores de recurso, Claude Code usa o tempo de execução v2 por padrão no Claude Code v2.1.274 ou posterior:

* Sessões no Amazon Bedrock, Claude Platform no AWS, Agent Platform do Google Cloud, ou Microsoft Foundry, a menos que uma plataforma host que incorpora Claude Code defina [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/pt/env-vars)
* Sessões conectadas através de um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway)
* Sessões onde você desativa telemetria ou busca de sinalizador de recurso, por exemplo com `DISABLE_TELEMETRY`

No v2, Claude Code também:

* Pergunta aos servidores HTTP se eles suportam a revisão mais recente, e a usa com aqueles que fazem. Ele também pergunta aos servidores conectores claude.ai em sessões onde ele busca sinalizadores de recurso. Para tê-lo perguntar aos servidores stdio, ou aos servidores conectores em cada sessão, defina [`MCP_PROTOCOL_NEGOTIATION`](/docs/pt/env-vars) como `auto`. Ele se conecta a todos os outros servidores como v1 faz.
* Recebe notificações `list_changed` de servidores na revisão mais recente sobre um [stream que mantém aberto](#notification-streams-on-the-v2-runtime).
* Não registra um servidor [channel](#push-messages-with-channels) que se conecta na revisão mais recente, porque essa revisão não pode carregar mensagens de canal.
* Falha em um [login OAuth MCP](#authenticate-with-remote-mcp-servers) cuja resposta de autorização nomeia um emissor inesperado.

Anthropic pode manter um servidor específico no protocolo anterior, ou fora desse stream, com um sinalizador de recurso que Claude Code busca.

Para escolher o tempo de execução você mesmo, defina [`MCP_SDK_GENERATION`](/docs/pt/env-vars) como `v1` ou `v2`. Para decidir se Claude Code pergunta, defina [`MCP_PROTOCOL_NEGOTIATION`](/docs/pt/env-vars) como `auto` ou `legacy`.

<h3 id="dynamic-tool-updates">
  Dynamic tool updates
</h3>

Claude Code suporta notificações MCP `list_changed`, permitindo que servidores MCP atualizem dinamicamente suas ferramentas, prompts e recursos disponíveis sem exigir que você se desconecte e reconecte. Quando um servidor MCP envia uma notificação `list_changed`, Claude Code atualiza automaticamente as capacidades disponíveis desse servidor.

Se uma solicitação de atualização falhar, Claude Code mantém as ferramentas, prompts e recursos descobertos anteriormente do servidor até que uma atualização posterior tenha sucesso. Antes da v2.1.214, um erro transitório durante a atualização substituía as ferramentas, prompts e recursos do servidor por uma lista vazia.

<h4 id="notification-streams-on-the-v2-runtime">
  Notification streams on the v2 runtime
</h4>

No [tempo de execução v2](#mcp-client-runtimes), Claude Code recebe notificações `list_changed` de um servidor na revisão de protocolo mais recente sobre um stream que mantém aberto. Quando o stream fecha, Claude Code o reabre, com dois limites:

* **O stream fecha novamente dentro de 10 segundos**: Claude Code o reabre até três vezes, depois para para essa conexão.
* **O stream permanece aberto por mais de 10 segundos, depois fecha**, como streams para hosts sem servidor comumente fazem: após cinco reabertas em uma hora, Claude Code aguarda cerca de seis horas antes da próxima.

Até o stream reabrir, você mantém as ferramentas, prompts e recursos buscados pela última vez do servidor. Para pegar suas mudanças mais cedo, reconecte o servidor de `/mcp`.

<h3 id="automatic-reconnection">
  Automatic reconnection
</h3>

Claude Code reconecta um servidor remoto que cai no meio da sessão e tenta novamente a primeira conexão de um servidor HTTP ou SSE após um erro transitório. Os servidores Stdio são processos locais, e Claude Code não os reconecta automaticamente.

<h4 id="mid-session-drops-of-a-remote-server">
  Mid-session drops of a remote server
</h4>

Claude Code reconecta um servidor remoto caído com backoff exponencial: até cinco tentativas, começando com um atraso de um segundo e dobrando a cada vez. O que você vê depende de como você está executando Claude Code:

* **Em uma sessão interativa**: `/mcp` mostra o servidor como pendente enquanto Claude Code reconecta. Após cinco tentativas falhadas, Claude Code marca o servidor como com falha, ou como precisando de autenticação quando o servidor precisa ser autorizado novamente. Você pode tentar novamente manualmente de `/mcp`.
* **Em execuções [`claude -p`](/docs/pt/headless) e sessões [Agent SDK](/docs/pt/agent-sdk/overview)**: Claude Code reconecta no mesmo cronograma, sem painel `/mcp` para mostrar as tentativas.

<h4 id="failed-first-connections">
  Failed first connections
</h4>

Quando a primeira conexão de um servidor HTTP ou SSE falha com um erro transitório, como uma resposta 5xx, uma conexão recusada, ou um tempo limite, Claude Code tenta novamente até três vezes. Se a conexão ainda falhar, Claude Code marca o servidor como com falha. Claude Code tenta novamente dessa forma na inicialização e quando um servidor é adicionado no meio da sessão. Isso inclui um servidor que Claude Code adiciona a uma [sessão em nuvem](/docs/pt/claude-code-on-the-web) de sua configuração e um servidor que você adiciona com o [`setMcpServers()`](/docs/pt/agent-sdk/typescript) do Agent SDK.

Claude Code não tenta novamente nestes casos:

* Primeira conexão de um servidor WebSocket
* Um erro de autenticação ou não encontrado, porque requer uma mudança de configuração para resolver. Quando um [`headersHelper`](#use-dynamic-headers-for-custom-authentication) é a única fonte do servidor do cabeçalho `Authorization`, Claude Code tenta novamente um erro de autenticação mesmo assim, porque re-executa o helper em cada tentativa e pode pegar uma credencial fresca

<h4 id="failed-discovery-requests">
  Failed discovery requests
</h4>

Depois que um servidor se conecta, Claude Code o envia solicitações de descoberta de capacidade como `tools/list`, `prompts/list`, e `resources/list`. Claude Code tenta novamente essas solicitações até três vezes com backoff curto após um erro de rede ou servidor transitório. Ele não tenta novamente erros de autenticação, respostas 4xx, ou tempos limite de solicitação.

<h4 id="how-claude-learns-that-a-server-failed">
  How Claude learns that a server failed
</h4>

Se Claude Code diz a Claude sobre um servidor configurado que falhou em se conectar depende de [busca de ferramentas](#scale-with-mcp-tool-search), que está ativada por padrão:

* Com busca de ferramentas, Claude Code diz a Claude qual servidor falhou e seu erro de conexão, portanto Claude relata a falha de conexão em sua resposta. Claude Code inclui as mesmas informações nos resultados de `ToolSearch` que não encontram nenhuma ferramenta correspondente.
* Em qualquer [configuração sem busca de ferramentas](#configure-tool-search), Claude Code não relata falhas de conexão de servidor com falha a Claude.

<h3 id="push-messages-with-channels">
  Push messages with channels
</h3>

Um servidor MCP também pode enviar mensagens diretamente para sua sessão para que Claude possa reagir a eventos externos como resultados de CI, alertas de monitoramento, ou mensagens de chat. Para ativar isso, seu servidor declara a capacidade `claude/channel` e você o ativa com o sinalizador `--channels` na inicialização. Veja [Channels](/docs/pt/channels) para usar um canal oficialmente suportado, ou [Channels reference](/docs/pt/channels-reference) para construir o seu próprio.

No [tempo de execução v2](#mcp-client-runtimes), se você definir [`MCP_PROTOCOL_NEGOTIATION`](/docs/pt/env-vars) como `auto` e um servidor de canal negocia revisão de protocolo MCP 2026-07-28, ele não pode entregar mensagens de canal, portanto Claude Code não o registra como um canal. Deixar a variável não definida, ou defini-la como `legacy`, mantém servidores stdio no handshake anterior.

<Tip>
  Dicas:

  * Use o sinalizador `-s` ou `--scope` para especificar onde a configuração é armazenada:
    * `local` (padrão): disponível apenas para você no projeto atual
    * `project`: compartilhado com todos no projeto via arquivo `.mcp.json`
    * `user`: disponível para você em todos os projetos
  * Defina variáveis de ambiente com sinalizadores `-e` ou `--env` (por exemplo, `-e KEY=value`)
  * Os sinalizadores `--transport` e `--header` também aceitam formas curtas `-t` e `-H`
  * Configure o tempo limite de inicialização do servidor MCP usando a variável de ambiente `MCP_TIMEOUT` (por exemplo, `MCP_TIMEOUT=10000 claude` define um tempo limite de 10 segundos)
  * Defina um tempo limite de execução de ferramenta por servidor adicionando um campo `timeout` em milissegundos à entrada `.mcp.json` desse servidor, por exemplo `"timeout": 600000` para dez minutos. Isso substitui a variável de ambiente `MCP_TOOL_TIMEOUT` apenas para esse servidor
  * Claude Code exibe um aviso quando a saída da ferramenta MCP excede 10.000 tokens e limita a saída a 25.000 tokens por padrão. Para aumentar o limite, defina a variável de ambiente `MAX_MCP_OUTPUT_TOKENS` (por exemplo, `MAX_MCP_OUTPUT_TOKENS=50000`); o limite de aviso é fixo. Veja [MCP output limits and warnings](#mcp-output-limits-and-warnings)
  * Use `/mcp` para autenticar com servidores remotos que requerem autenticação OAuth 2.0
</Tip>

O `timeout` por servidor é um limite de parede de relógio duro por chamada de ferramenta, e notificações de progresso do servidor não o estendem. Valores abaixo de 1000 são ignorados e caem para `MCP_TOOL_TIMEOUT`, ou para seu padrão de cerca de 28 horas quando essa variável não está definida. Para um servidor HTTP, SSE, ou [conector claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai) há também um segundo temporizador por solicitação que cobre cada solicitação até o primeiro byte de resposta do servidor. Claude Code define esse temporizador para o maior de três valores: 60 segundos, o tempo limite de ferramenta que se aplica ao servidor, e `MCP_TIMEOUT`. O padrão de 28 horas de um `MCP_TOOL_TIMEOUT` não definido não entra nessa comparação, e um valor abaixo de 60 segundos não encurta o temporizador. Os servidores Stdio e WebSocket não têm temporizador por solicitação.

Um `timeout` por servidor de pelo menos 1000 também atua como um piso no tempo limite de inatividade descrito abaixo: Claude Code nunca aborta as chamadas de ferramenta desse servidor por inatividade mais cedo do que o `timeout` por servidor. Requer Claude Code v2.1.203 ou posterior.

Uma chamada de ferramenta para um servidor MCP que não envia resposta e nenhuma notificação de progresso para a janela de inatividade aborta com um erro em vez de aguardar o limite de parede de relógio. Aplica-se a todos os tipos de servidor exceto servidores IDE e SDK em processo. A janela de inatividade padrão é de cinco minutos para servidores HTTP, SSE, WebSocket, e [conector claude.ai](#use-mcp-servers-from-claude-ai), e de 30 minutos para servidores stdio. Antes da v2.1.203, servidores stdio eram isentos do tempo limite de inatividade.

Defina a variável de ambiente [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/pt/env-vars) em milissegundos para alterar a janela de inatividade, ou defina-a como `0` para desabilitar a verificação.

Esses tempos limite limitam quanto tempo uma chamada pode ser executada, nem sempre quanto tempo bloqueia a sessão: uma chamada de conversa principal que é executada por mais de dois minutos se move para uma tarefa em segundo plano primeiro. Veja [Automatic backgrounding of long tool calls](#automatic-backgrounding-of-long-tool-calls).

<h3 id="automatic-backgrounding-of-long-tool-calls">
  Automatic backgrounding of long tool calls
</h3>

Uma chamada de ferramenta MCP na conversa principal que ainda está em execução após dois minutos se move para uma tarefa em segundo plano em vez de bloquear a sessão. Claude recebe o ID da tarefa imediatamente e continua trabalhando, e o resultado chega como uma notificação de tarefa quando a chamada se resolve. O backgrounding automático requer Claude Code v2.1.212 ou posterior.

A tarefa aparece em [`/tasks`](/docs/pt/commands#all-commands), onde você também pode pará-la, e não sobrevive ao sair da sessão. Os limites por chamada ainda se aplicam enquanto a chamada é executada em segundo plano: o limite de parede de relógio definido pelo `timeout` por servidor ou [`MCP_TOOL_TIMEOUT`](/docs/pt/env-vars), e o tempo limite de inatividade definido por [`CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`](/docs/pt/env-vars).

Defina a variável de ambiente [`CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS`](/docs/pt/env-vars) em milissegundos para alterar o limite, ou defina-a como `0` para desativar o backgrounding automático. Definir `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` como `1` também o desativa, junto com todos os outros recursos de tarefa em segundo plano.

Algumas chamadas nunca se movem para o segundo plano:

* Chamadas de [subagentes](/docs/pt/sub-agents); Claude Code coloca em segundo plano apenas chamadas de conversa principal
* Chamadas para servidores IDE
* Chamadas em [modo não interativo](/docs/pt/headless), a menos que `CLAUDE_AUTO_BACKGROUND_TASKS` esteja definido como `1`, já que uma execução única pode terminar antes do resultado chegar

Uma chamada aguardando um [diálogo de elicitação](#respond-to-mcp-elicitation-requests) aberto não é colocada em segundo plano enquanto o diálogo está aberto; o servidor está bloqueado em sua entrada, não lento, portanto Claude Code adia a mudança até o diálogo fechar.

<h3 id="plugin-provided-mcp-servers">
  Plugin-provided MCP servers
</h3>

[Plugins](/docs/pt/plugins/overview) podem agrupar servidores MCP que fornecem ferramentas e integrações quando você ativa o plugin. Os servidores MCP de plugin funcionam de forma idêntica aos servidores configurados pelo usuário.

**Como funcionam os servidores MCP de plugin**:

* Plugins definem servidores MCP em `.mcp.json` na raiz do plugin ou inline em `plugin.json`
* Quando você ativa um plugin, Claude Code inicia seus servidores MCP automaticamente
* Claude Code oferece ferramentas MCP de plugin ao lado de ferramentas MCP configuradas manualmente
* Você adiciona e remove servidores de plugin instalando ou desinstalando o plugin, não com comandos `/mcp`. Você ainda pode [alternar um servidor de plugin instalado desativado](#disable-a-server-without-removing-it) em `/mcp`, o que impede que Claude Code se conecte a ele sem remover o plugin

**Exemplo de configuração MCP de plugin**:

Em `.mcp.json` na raiz do plugin:

```json theme={null}
{
  "mcpServers": {
    "database-tools": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_URL": "${DB_URL}"
      }
    }
  }
}
```

Ou inline em `plugin.json`:

```json theme={null}
{
  "name": "my-plugin",
  "mcpServers": {
    "plugin-api": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/api-server",
      "args": ["--port", "8080"]
    }
  }
}
```

**Recursos MCP de plugin**:

* **Ciclo de vida automático**: servidores se conectam e desconectam nestes pontos:
  * Na inicialização da sessão, Claude Code conecta os servidores para plugins ativados automaticamente. Em `/mcp`, um servidor de plugin remoto (HTTP ou SSE) que você usou antes pode mostrar o status [`cached`](#server-status-detail) em vez disso; Claude Code o conecta quando Claude chama pela primeira vez uma de suas ferramentas
  * Se você ativar ou desativar um plugin durante uma sessão, Claude Code conecta ou desconecta seus servidores MCP quando a mudança se aplica. [Apply plugin changes without restarting](/docs/pt/plugins/cli-reference#reload-plugins) descreve quando isso é. Em uma sessão sem um terminal interativo, `/reload-plugins` não conecta ou desconecta servidores MCP de plugin; essas mudanças entram em vigor em sua próxima sessão
  * Quando você recarrega, Claude Code mantém as conexões ativas de servidores de plugin cuja configuração não mudou, e faz o mesmo quando você [substitui a lista de servidores MCP da sessão](/docs/pt/agent-sdk/typescript#mcpsetserversresult) do Agent SDK sem nomeá-los
  * Quando você [move a sessão com `/cd`](/docs/pt/permissions#move-the-session-to-another-directory) na v2.1.246 ou posterior, Claude Code conecta os servidores de plugins que as configurações do novo diretório ativam e desconecta os servidores de plugins que não estão mais ativados, portanto você não precisa executar `/reload-plugins` após a mudança
  * Em [sessões web](/docs/pt/claude-code-on-the-web), uma chamada MCP para um servidor de plugin que ainda não está conectado, como logo após uma sessão ociosa acordar, inicia o servidor sob demanda e aguarda sua conexão
* **Espaços reservados de caminho**: `${CLAUDE_PLUGIN_ROOT}` resolve para o diretório de instalação do plugin, `${CLAUDE_PLUGIN_DATA}` para seu diretório de [estado persistente](/docs/pt/plugins/components#path-variables-and-persistent-data), e `${CLAUDE_PROJECT_DIR}` para a raiz do projeto estável. A substituição se aplica a:
  * servidores `stdio`: `command`, `args`, `env`
  * servidores `http`, `sse`, e `ws`: `url`, `headers`, e `headersHelper`. Antes da v2.1.195, `headersHelper` passava o espaço reservado como uma string literal
* **Acesso ao ambiente do usuário**: acesso às mesmas variáveis de ambiente que servidores configurados manualmente
* **Múltiplos tipos de transporte**: suporte para transportes stdio, SSE, HTTP, e WebSocket, embora o suporte de transporte possa variar por servidor

Os servidores de plugin aparecem em `/mcp` com indicadores mostrando que vêm de plugins.

**Nomes de ferramentas MCP de plugin**:

As ferramentas de um servidor MCP agrupado em plugin incluem tanto o nome do plugin quanto a chave do servidor em seu nome chamável. A forma completa é `mcp__plugin_<plugin-name>_<server-name>__<tool-name>`, onde qualquer caractere fora de `A-Z`, `a-z`, `0-9`, `_`, e `-` é substituído por `_`. Para o servidor `database-tools` agrupado em um plugin nomeado `my-plugin`, uma ferramenta `query` é chamável como:

```
mcp__plugin_my-plugin_database-tools__query
```

Use este nome completo ao fazer referência à ferramenta em [regras de permissão](/docs/pt/permissions), lista `allowed-tools` de uma skill, campo `tools` de um [subagente](/docs/pt/sub-agents#available-tools), ou [matcher de hook](/docs/pt/hooks#match-mcp-tools). Um matcher de hook escrito contra a chave do servidor simples, como `mcp__database-tools__.*`, nunca dispara para um servidor agrupado em plugin.

O servidor em si se registra sob o nome com escopo `plugin:<plugin-name>:<server-name>`, como `plugin:my-plugin:database-tools`. Use esse nome onde um nome de servidor configurado é esperado, como um [campo `server` de hook `mcp_tool`](/docs/pt/hooks#mcp-tool-hook-fields).

Veja a [referência de componentes de plugin](/docs/pt/plugins/components#mcp-servers) para detalhes sobre agrupamento de servidores MCP com plugins.

<h2 id="mcp-installation-scopes">
  Escopos de instalação de MCP
</h2>

Os servidores MCP podem ser configurados em três escopos. O escopo que você escolhe controla em quais projetos o servidor é carregado e se a configuração é compartilhada com sua equipe. Os administradores também podem implantar ou fornecer servidores para cada usuário via [configuração gerenciada](#managed-mcp-configuration).

| Escopo                    | Carrega em             | Compartilhado com equipe    | Armazenado em                  |
| ------------------------- | ---------------------- | --------------------------- | ------------------------------ |
| [Local](#local-scope)     | Apenas projeto atual   | Não                         | `~/.claude.json`               |
| [Projeto](#project-scope) | Apenas projeto atual   | Sim, via controle de versão | `.mcp.json` na raiz do projeto |
| [Usuário](#user-scope)    | Todos os seus projetos | Não                         | `~/.claude.json`               |

<h3 id="local-scope">
  Escopo local
</h3>

O escopo local é o padrão. Um servidor com escopo local carrega apenas no projeto onde você o adicionou e permanece privado para você. Claude Code o armazena em `~/.claude.json` sob o caminho desse projeto, então o mesmo servidor não aparecerá em seus outros projetos. Use o escopo local para servidores de desenvolvimento pessoal, configurações experimentais ou servidores com credenciais que você não deseja no controle de versão.

<Note>
  O termo "escopo local" para servidores MCP difere das configurações locais gerais. Os servidores MCP com escopo local são armazenados em `~/.claude.json` (seu diretório inicial), enquanto as configurações locais gerais usam `.claude/settings.local.json` (no diretório do projeto). Veja [Configurações](/docs/pt/settings#where-settings-live) para detalhes sobre localizações de arquivos de configuração.
</Note>

```bash theme={null}
# Adicionar um servidor com escopo local (padrão)
claude mcp add --transport http stripe https://mcp.stripe.com

# Especificar explicitamente escopo local
claude mcp add --transport http stripe --scope local https://mcp.stripe.com
```

O comando escreve o servidor na entrada do seu projeto atual dentro de `~/.claude.json`. O exemplo abaixo mostra o resultado quando você o executa de `/path/to/your/project`:

```json theme={null}
{
  "projects": {
    "/path/to/your/project": {
      "mcpServers": {
        "stripe": {
          "type": "http",
          "url": "https://mcp.stripe.com"
        }
      }
    }
  }
}
```

<h3 id="project-scope">
  Escopo de projeto
</h3>

Servidores com escopo de projeto permitem colaboração em equipe armazenando configurações em um arquivo `.mcp.json` no diretório raiz do seu projeto. Quando você adiciona um servidor com escopo de projeto, Claude Code cria ou atualiza automaticamente este arquivo com a estrutura de configuração apropriada. Verifique `.mcp.json` no controle de versão para que todos na sua equipe obtenham as mesmas ferramentas e serviços MCP.

```bash theme={null}
# Adicionar um servidor com escopo de projeto
claude mcp add --transport http shared-server --scope project https://example.com/mcp
```

O arquivo `.mcp.json` resultante segue um formato padronizado:

```json theme={null}
{
  "mcpServers": {
    "shared-server": {
      "type": "http",
      "url": "https://example.com/mcp"
    }
  }
}
```

Por razões de segurança, Claude Code solicita aprovação em sessões interativas antes de usar servidores com escopo de projeto de arquivos `.mcp.json`. Para redefinir essas escolhas de aprovação, execute `claude mcp reset-project-choices`.

Em execuções `claude -p`, sessões do [Agent SDK](/docs/pt/headless) e [sessões na nuvem](/docs/pt/claude-code-on-the-web), Claude Code não pode mostrar esse prompt: ele carrega servidores com escopo de projeto sem perguntar. Claude Code também ignora o prompt em uma sessão que você inicia no modo `bypassPermissions` com [`skipDangerousModePermissionPrompt`](/docs/pt/settings-reference#skipdangerousmodepermissionprompt) definido em suas configurações de usuário ou em configurações gerenciadas. Para manter um servidor fora mesmo assim:

* Adicione-o a [`disabledMcpjsonServers`](/docs/pt/settings-reference#disabledmcpjsonservers), que o bloqueia em todos os modos de permissão.
* Exclua as configurações do projeto inteiramente com [`--setting-sources`](/docs/pt/cli-reference#cli-flags) ou a opção `settingSources` do SDK.
* Inicie a sessão com [`--strict-mcp-config`](/docs/pt/cli-reference#cli-flags). Claude Code então usa apenas os servidores MCP que você passa com `--mcp-config`. Ignorar o prompt de aprovação para os servidores com escopo de projeto que Claude Code não está carregando requer Claude Code v2.1.246 ou posterior; antes de v2.1.246, uma sessão estrita ainda aguardava aprovação para eles, o que deixava sessões em segundo plano aguardando na inicialização. Veja [Controle exclusivo com managed-mcp.json](/docs/pt/managed-mcp#exclusive-control-with-managed-mcp-json) para o que o sinalizador faz sob um arquivo MCP gerenciado.

[Aprovações de servidor de projeto e confiança do espaço de trabalho](#project-server-approvals-and-workspace-trust) aborda como as aprovações confirmadas no repositório interagem com a confiança do espaço de trabalho.

<h3 id="user-scope">
  Escopo de usuário
</h3>

Servidores com escopo de usuário são armazenados em `~/.claude.json` e fornecem acessibilidade entre projetos, tornando-os disponíveis em todos os projetos em sua máquina enquanto permanecem privados para sua conta de usuário. Este escopo funciona bem para servidores de utilitários pessoais, ferramentas de desenvolvimento ou serviços que você usa frequentemente em diferentes projetos.

```bash theme={null}
# Adicionar um servidor de usuário
claude mcp add --transport http hubspot --scope user https://mcp.hubspot.com/anthropic
```

<h3 id="scope-hierarchy-and-precedence">
  Hierarquia de escopo e precedência
</h3>

Quando o mesmo servidor é definido em mais de um lugar, Claude Code se conecta a ele uma vez, usando a definição da fonte com maior precedência. A entrada inteira do servidor dessa fonte é usada; os campos não são mesclados entre escopos.

1. Escopo local
2. Escopo de projeto
3. Escopo de usuário
4. [Servidores fornecidos por plugins](/docs/pt/plugins/components#mcp-servers)
5. [Conectores claude.ai](#use-mcp-servers-from-claude-ai)

Os três escopos correspondem duplicatas por nome. Plugins e conectores correspondem por endpoint, então um que aponta para a mesma URL ou comando que um servidor acima é tratado como uma duplicata.

Um servidor que sua organização fornece através da configuração gerenciada [`managedMcpServers`](/docs/pt/managed-mcp#provide-servers-through-managed-settings) classifica-se acima de todos esses, então quando um deles o duplica, Claude Code conecta a definição da organização. Requer Claude Code v2.1.259 ou posterior.

Se você abrir uma sessão local na [guia Code do aplicativo Desktop](/docs/pt/desktop#mcp-servers-from-the-claude-desktop-chat-app) com o mesmo nome de servidor stdio no nível superior de `~/.claude.json` (escopo de usuário) e em `.mcp.json`, a guia Code usa a definição `~/.claude.json`.

<h3 id="environment-variable-expansion-in-mcp-json">
  Expansão de variáveis de ambiente em `.mcp.json`
</h3>

Claude Code suporta expansão de variáveis de ambiente em arquivos `.mcp.json`, permitindo que equipes compartilhem configurações mantendo flexibilidade para caminhos específicos da máquina e valores sensíveis como chaves de API.

<h4 id="supported-syntax">
  Sintaxe suportada
</h4>

* `${VAR}`: expande para o valor da variável de ambiente `VAR`
* `${VAR:-default}`: expande para `VAR` se definida, caso contrário usa `default`

<h4 id="expansion-locations">
  Locais de expansão
</h4>

As variáveis de ambiente podem ser expandidas em:

* `command`: o caminho do executável do servidor
* `args`: argumentos de linha de comando
* `env`: variáveis de ambiente passadas para o servidor
* `url`: para tipos de servidor HTTP
* `headers`: para autenticação de servidor HTTP

<h4 id="example-with-variable-expansion">
  Exemplo com expansão de variável
</h4>

```json theme={null}
{
  "mcpServers": {
    "api-server": {
      "type": "http",
      "url": "${API_BASE_URL:-https://api.example.com}/mcp",
      "headers": {
        "Authorization": "Bearer ${API_KEY}"
      }
    }
  }
}
```

<h4 id="unset-variables-without-a-default">
  Variáveis não definidas sem um padrão
</h4>

Se uma variável de ambiente necessária não estiver definida e não tiver um valor padrão, a configuração ainda é carregada: Claude Code relata um aviso de variável ausente para esse servidor na saída `claude mcp list` e usa o texto `${VAR}` não expandido como está. Defina a variável ou adicione um fallback `:-default` para que o servidor inicie com o valor que você pretende. Em uma URL e headers de servidor remoto, algumas variáveis de credencial [leem como vazias](#credential-variables-that-read-as-empty) em vez disso, sem aviso.

<h4 id="credential-variables-that-read-as-empty">
  Variáveis de credencial que leem como vazias
</h4>

Em uma URL e headers de servidor remoto, Claude Code lê variáveis de credencial do seu ambiente como vazias em vez de expandi-las. Isso impede que a `.mcp.json` de um projeto ou um plugin envie suas credenciais Claude Code ou de provedor de nuvem para um servidor que ele nomeia. Se você escrever `Bearer ${ANTHROPIC_AUTH_TOKEN}`, o servidor recebe `Bearer ` sem credencial e rejeita a solicitação, geralmente com um `401`. Claude Code relata isso como uma conexão falhada.

Os nomes cobertos são:

* Credenciais próprias do Claude Code, como `ANTHROPIC_API_KEY` e `ANTHROPIC_AUTH_TOKEN`
* Credenciais do seu provedor de nuvem, como `AWS_BEARER_TOKEN_BEDROCK`
* Outras credenciais que seu ambiente carrega, como `HTTPS_PROXY` e `NPM_TOKEN`

Um nome coberto lê como vazio independentemente de você ter definido a variável, e um fallback `:-default` nele é ignorado. Uma URL base do provedor como `ANTHROPIC_BASE_URL` ainda expande, então `"url": "${ANTHROPIC_BASE_URL}/mcp"` funciona, a menos que o valor da URL em si incorpore uma credencial como um nome de usuário e senha.

Um nome fora deste conjunto, como `API_KEY`, expande conforme escrito. Para dar ao servidor uma das credenciais cobertas, copie-a para uma variável com um nome de sua escolha e referencie esse nome em vez disso.

Quando a URL ou headers de um servidor remoto referencia uma variável coberta que você definiu, Claude Code a nomeia em uma linha de log de depuração. Para ler a linha, execute `claude --debug-file /tmp/claude-debug.log` e procure nesse arquivo por `never expanded toward a remote server`.

<h4 id="how-references-appear-in-/mcp-and-cli-output">
  Como referências aparecem em `/mcp` e saída de CLI
</h4>

Para um servidor no [escopo](#mcp-installation-scopes) local, de projeto ou de usuário, as seguintes superfícies mostram uma referência `${VAR}` por nome em vez de seu valor resolvido:

* A URL ou linha de comando na visualização de detalhes `/mcp` de um servidor
* Saída de `claude mcp list` e `claude mcp get`

A visualização de detalhes `/mcp` mostra referências desta forma em Claude Code v2.1.268 ou posterior.

Para um servidor que sua organização fornece através da configuração `managedMcpServers`, essas superfícies mostram [apenas o host da URL](/docs/pt/managed-mcp#what-users-can-see-and-change).

Para verificar o que `claude mcp list`, `claude mcp get` e `/mcp` mostram quando uma conexão falha, veja [Detalhe do status do servidor](#server-status-detail).

<h2 id="practical-examples">
  Exemplos práticos
</h2>

<h3 id="example-connect-to-github-for-code-reviews">
  Exemplo: Conectar ao GitHub para revisões de código
</h3>

O servidor MCP remoto do GitHub autentica com um token de acesso pessoal do GitHub passado como cabeçalho. Para obter um, abra suas [configurações de token do GitHub](https://github.com/settings/personal-access-tokens), gere um novo token refinado com acesso aos repositórios com os quais você deseja que Claude trabalhe, então adicione o servidor:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Substitua `YOUR_GITHUB_PAT` pelo seu token de acesso pessoal. O comando `claude mcp add` salva a configuração sem validar credenciais, portanto um valor de espaço reservado é aceito aqui, mas o servidor falha ao conectar mais tarde. Para verificar a conexão, execute `/mcp` e verifique se o servidor mostra `connected`. Um servidor com credenciais inválidas mostra `failed`, e o detalhe da falha inclui o status HTTP que o servidor retornou, como um 401.

Então trabalhe com GitHub:

```text wrap theme={null}
Revise o PR #456 e sugira melhorias
```

```text wrap theme={null}
Crie um novo problema para o bug que acabamos de encontrar
```

```text wrap theme={null}
Mostre-me todos os PRs abertos atribuídos a mim
```

<h3 id="example-query-your-postgresql-database">
  Exemplo: Consultar seu banco de dados PostgreSQL
</h3>

[DBHub](https://github.com/bytebase/dbhub), o pacote `@bytebase/dbhub`, é um servidor MCP que conecta Claude a um banco de dados relacional através da string de conexão que você passa em `--dsn`. Use um usuário de banco de dados somente leitura na string de conexão para que as consultas que Claude executa não possam modificar dados:

```bash theme={null}
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"
```

Para confirmar que o servidor inicia, execute `/mcp` e verifique se `db` mostra `connected`.

Então consulte seu banco de dados naturalmente:

```text wrap theme={null}
Qual é nossa receita total este mês?
```

```text wrap theme={null}
Mostre-me o esquema para a tabela de pedidos
```

```text wrap theme={null}
Encontre clientes que não fizeram uma compra em 90 dias
```

<h2 id="authenticate-with-remote-mcp-servers">
  Autenticar com servidores MCP remotos
</h2>

Muitos servidores MCP baseados em nuvem exigem autenticação. Claude Code suporta OAuth 2.0 para conexões seguras.

Claude Code marca um servidor remoto como necessitando autenticação quando o servidor responde com `401 Unauthorized` ou `403 Forbidden`. O que Claude Code mostra depende do servidor:

* Para um servidor no qual você ainda não fez login, qualquer código de status o sinaliza em `/mcp` para que você possa completar o fluxo OAuth.
* Para um [conector claude.ai](#use-mcp-servers-from-claude-ai), um `401` causado por claude.ai rejeitando seu token de sessão não sinaliza o conector, porque re-autorizar o conector não pode corrigir seu login. Claude Code mostra o [estado de token de sessão rejeitado](/docs/pt/errors#claude-ai-rejected-the-session-token) em vez disso.
* Para um servidor cujo cabeçalho `Authorization` você configurou, em `headers` ou através de um [`headersHelper`](#use-dynamic-headers-for-custom-authentication), um `401` ou `403` ao conectar não sinaliza o servidor, porque a credencial a corrigir é a que você configurou. Claude Code relata a conexão como falha em vez disso. Se você definiu esse cabeçalho a partir de uma referência `${VAR}`, verifique se essa variável é uma que Claude Code [lê como vazia](#credential-variables-that-read-as-empty).
* Para um conector [entregue a uma sessão em nuvem](#how-connectors-reach-claude-code), Claude Code não executa um fluxo de login, porque o proxy da sessão se autentica no conector com a autorização que você concedeu em claude.ai. Quando um conector lá precisa ser autorizado novamente, reconecte-o em [claude.ai/customize/connectors](https://claude.ai/customize/connectors) em vez de a partir da sessão.

Quando uma solicitação para um servidor OAuth no qual você já fez login retorna `401 Unauthorized`, Claude Code atualiza o token armazenado, reconecta e tenta a solicitação novamente uma vez. Ele sinaliza o servidor em `/mcp` apenas se essa tentativa também falhar. Antes da v2.1.206, uma atualização de token que falhava por um motivo transitório, como um erro de rede, sinalizava um servidor OAuth como necessitando autenticação pelo resto da sessão, mesmo que seu token de atualização ainda fosse válido.

Quando o servidor rejeita o token de atualização armazenado, Claude Code imediatamente mostra um aviso apontando para `/mcp`. Abra `/mcp` e selecione **Re-autenticar** no servidor para fazer login novamente antes que a próxima chamada de ferramenta falhe.

Um servidor personalizado que retorna um cabeçalho `WWW-Authenticate` apontando para seu servidor de autorização obtém a mesma descoberta automática que qualquer outro servidor remoto.

Claude Code também mostra um aviso de inicialização quando um ou mais servidores configurados precisam de autenticação, para que você não tenha que abrir `/mcp` para descobrir quais servidores precisam de login. O aviso requer Claude Code v2.1.193 ou posterior. Ele conta apenas servidores nos quais você pode fazer login a partir do Claude Code. Antes da v2.1.218, ele também contava [conectores claude.ai](#use-mcp-servers-from-claude-ai) que não estavam conectados em claude.ai, que você pode conectar apenas a partir das configurações de claude.ai.

O aviso anuncia cada servidor uma vez e o deixa de fora da contagem em inicializações posteriores até que esse servidor tenha se conectado e precise de login novamente. `/mcp` ainda lista todos os servidores que precisam de login.

No modo não interativo não há painel `/mcp`, então Claude Code não pode executar o fluxo OAuth para você. A partir da v2.1.196, quando um servidor configurado precisa de autenticação durante uma execução `claude -p` ou Agent SDK com [busca de ferramentas](#scale-with-mcp-tool-search) ativada, que é o padrão, Claude Code informa ao Claude que as ferramentas do servidor estão indisponíveis até que você o autorize. Claude pode então nomear o servidor que precisa de login em vez de responder como se o servidor não estivesse configurado. Complete o login de uma sessão interativa com `/mcp` ou `claude mcp login <name>`.

Se você configurou `headers.Authorization` para o servidor e o servidor rejeita esse cabeçalho, Claude Code relata a conexão como falha em vez de voltar para OAuth. Verifique se o token é válido para o endpoint MCP, ou remova o cabeçalho para usar o fluxo OAuth.

<Steps>
  <Step title="Adicione o servidor que requer autenticação">
    Se você já adicionou o servidor `sentry` no [guia de início rápido MCP](/docs/pt/mcp-quickstart#connect-a-server-that-requires-sign-in), pule esta etapa: executar `claude mcp add` novamente com o mesmo nome de servidor no mesmo escopo falha com `MCP server sentry already exists in local config`. Caso contrário, execute:

    ```bash theme={null}
    claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
    ```
  </Step>

  <Step title="Use o comando /mcp dentro do Claude Code">
    No Claude Code, use o comando:

    ```text wrap theme={null}
    /mcp
    ```

    Então siga os passos no seu navegador para fazer login.
  </Step>
</Steps>

<Tip>
  Dicas:

  * Os tokens de autenticação são armazenados com segurança e atualizados automaticamente
  * Use "Clear authentication" no menu `/mcp` para revogar acesso
  * Se seu navegador não abrir automaticamente, copie a URL fornecida e abra-a manualmente
  * Se o redirecionamento do navegador falhar com um erro de conexão após autenticar, cole a URL de callback completa da barra de endereços do seu navegador no prompt de URL que aparece no Claude Code
  * A autenticação OAuth funciona com servidores HTTP
</Tip>

<h3 id="authenticate-from-the-command-line">
  Autenticar a partir da linha de comando
</h3>

O comando `claude mcp login <name>` executa o fluxo OAuth de um servidor configurado diretamente do seu shell, para que você não precise abrir o painel `/mcp` dentro de uma sessão.

```bash theme={null}
claude mcp login sentry
```

Para limpar credenciais armazenadas depois, execute `claude mcp logout <name>`.

`claude mcp login` detecta quando nenhum navegador local está disponível, como durante uma sessão SSH ou no Linux sem um servidor de exibição, e imprime a URL de autorização em vez de tentar abrir um navegador. Abra a URL na sua máquina local, depois cole a URL de redirecionamento completa da barra de endereços do seu navegador de volta no prompt. O comando precisa de um terminal interativo para a etapa de colagem, então conecte com `ssh -t`. Passe `--no-browser` para forçar o prompt de URL mesmo quando um navegador local é detectado.

```bash theme={null}
claude mcp login sentry --no-browser
```

<h3 id="use-a-fixed-oauth-callback-port">
  Usar uma porta de callback OAuth fixa
</h3>

Alguns servidores MCP exigem um URI de redirecionamento específico registrado antecipadamente. Por padrão, Claude Code escolhe uma porta aleatória disponível para o callback OAuth. Use `--callback-port` para fixar a porta para que corresponda a um URI de redirecionamento pré-registrado do formulário `http://localhost:PORT/callback`. Se o login em Claude Code v2.1.229 falhar com uma incompatibilidade de URI de redirecionamento, consulte a nota de versão em [Usar credenciais OAuth pré-configuradas](#use-pre-configured-oauth-credentials).

Você pode usar `--callback-port` sozinho (com registro dinâmico de cliente) ou junto com `--client-id` (com credenciais pré-configuradas).

```bash theme={null}
# Porta de callback fixa com registro dinâmico de cliente
claude mcp add --transport http \
  --callback-port 8080 \
  my-server https://mcp.example.com/mcp
```

<h3 id="use-pre-configured-oauth-credentials">
  Usar credenciais OAuth pré-configuradas
</h3>

Alguns servidores MCP não suportam configuração automática de OAuth via Registro Dinâmico de Cliente. Se você vir um erro como "Incompatible auth server: does not support dynamic client registration," o servidor requer credenciais pré-configuradas. Claude Code também suporta servidores que usam um Documento de Metadados de ID do Cliente (CIMD) em vez de Registro Dinâmico de Cliente, e descobre esses automaticamente. Se a descoberta automática falhar, registre um aplicativo OAuth através do portal do desenvolvedor do servidor primeiro, depois forneça as credenciais ao adicionar o servidor.

<Steps>
  <Step title="Registre um aplicativo OAuth com o servidor">
    Crie um aplicativo através do portal do desenvolvedor do servidor e anote seu ID do cliente e segredo do cliente.

    Muitos servidores também exigem um URI de redirecionamento. Se assim for, escolha uma porta e registre um URI de redirecionamento no formato `http://localhost:PORT/callback`. Use essa mesma porta com `--callback-port` na próxima etapa.

    Na v2.1.229, Claude Code enviava `http://127.0.0.1:PORT/callback` em vez disso, e servidores que correspondiam exatamente ao URI de redirecionamento registrado rejeitavam o login com uma incompatibilidade de URI de redirecionamento. Claude Code v2.1.231 restaurou a forma `localhost`. Para recuperar na v2.1.229, atualize Claude Code, ou adicione temporariamente a forma `http://127.0.0.1:PORT/callback` aos URIs de redirecionamento registrados do servidor.
  </Step>

  <Step title="Adicione o servidor com suas credenciais">
    Escolha um dos seguintes métodos. A porta usada para `--callback-port` pode ser qualquer porta disponível. Ela apenas precisa corresponder ao URI de redirecionamento que você registrou na etapa anterior.

    <Tabs>
      <Tab title="claude mcp add">
        Use `--client-id` para passar o ID do cliente do seu aplicativo. A flag `--client-secret` solicita o segredo com entrada mascarada:

        ```bash theme={null}
        claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>

      <Tab title="claude mcp add-json">
        Inclua o objeto `oauth` na configuração JSON e passe `--client-secret` como uma flag separada:

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' \
          --client-secret
        ```
      </Tab>

      <Tab title="claude mcp add-json (apenas porta de callback)">
        Use `--callback-port` sem um ID de cliente para fixar a porta enquanto usa registro dinâmico de cliente:

        ```bash theme={null}
        claude mcp add-json my-server \
          '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"callbackPort":8080}}'
        ```
      </Tab>

      <Tab title="CI / variável de ambiente">
        Defina o segredo via variável de ambiente para pular o prompt interativo:

        ```bash theme={null}
        MCP_CLIENT_SECRET=your-secret claude mcp add --transport http \
          --client-id your-client-id --client-secret --callback-port 8080 \
          my-server https://mcp.example.com/mcp
        ```
      </Tab>
    </Tabs>
  </Step>

  <Step title="Autentique no Claude Code">
    Execute `/mcp` no Claude Code e siga o fluxo de login do navegador.
  </Step>
</Steps>

<Tip>
  Dicas:

  * O segredo do cliente é armazenado com segurança no seu chaveiro do sistema (macOS) ou em um arquivo de credenciais, não na sua configuração
  * Você pode definir o segredo do cliente apenas quando adiciona o servidor. Quando você se autentica com `claude mcp login` ou a partir de `/mcp`, Claude Code usa o segredo armazenado e não solicita um ou lê `MCP_CLIENT_SECRET`
  * Para adicionar ou alterar o segredo depois, remova o servidor com `claude mcp remove <name>`, depois adicione-o novamente com `--client-secret` e o mesmo `--scope`
  * Se o servidor usar um cliente OAuth público sem segredo, use apenas `--client-id` sem `--client-secret`
  * Essas flags se aplicam apenas aos transportes HTTP e SSE. Elas não têm efeito em servidores stdio
  * Use `claude mcp get <name>` para verificar se as credenciais OAuth estão configuradas para um servidor
</Tip>

<h3 id="override-oauth-metadata-discovery">
  Substituir descoberta de metadados OAuth
</h3>

Aponte Claude Code para uma URL de metadados específica de servidor de autorização OAuth para contornar a cadeia de descoberta padrão. Defina `authServerMetadataUrl` quando os endpoints padrão do servidor MCP falharem, ou quando você deseja rotear a descoberta através de um proxy interno. Por padrão, Claude Code primeiro verifica os Metadados de Recurso Protegido RFC 9728 em `/.well-known/oauth-protected-resource`, depois volta para os metadados do servidor de autorização RFC 8414 em `/.well-known/oauth-authorization-server`.

Defina `authServerMetadataUrl` no objeto `oauth` da configuração do seu servidor em `.mcp.json`:

```json theme={null}
{
  "mcpServers": {
    "my-server": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "oauth": {
        "authServerMetadataUrl": "https://auth.example.com/.well-known/openid-configuration"
      }
    }
  }
}
```

A URL deve usar `https://`. Os `scopes_supported` da URL de metadados substituem os escopos que o servidor upstream anuncia.

<h3 id="restrict-oauth-scopes">
  Restringir escopos OAuth
</h3>

Defina `oauth.scopes` para fixar os escopos que Claude Code solicita durante o fluxo de autorização. Esta é a forma suportada de restringir um servidor MCP a um subconjunto aprovado pela equipe de segurança quando o servidor de autorização upstream anuncia mais escopos do que você deseja conceder. O valor é uma única string separada por espaço, correspondendo ao formato do parâmetro `scope` em RFC 6749 §3.3.

```json theme={null}
{
  "mcpServers": {
    "slack": {
      "type": "http",
      "url": "https://mcp.slack.com/mcp",
      "oauth": {
        "scopes": "channels:read chat:write search:read"
      }
    }
  }
}
```

`oauth.scopes` tem precedência sobre `authServerMetadataUrl` e os escopos que o servidor descobre em `/.well-known`. Deixe-o indefinido para permitir que o servidor MCP determine o conjunto de escopos solicitado.

A partir da v2.1.196, quando `oauth.scopes` não está definido, Claude Code solicita o escopo fornecido pelo cabeçalho `WWW-Authenticate` do servidor ou seus metadados de recurso protegido, e não envia nenhum parâmetro `scope` quando nenhum dos dois fornece um. Ele não solicita mais o catálogo completo de `scopes_supported` dos metadados do servidor de autorização descobertos automaticamente. Solicitar esse catálogo fez com que provedores de identidade que anunciam escopos apenas para administrador ou escopos de modelo rejeitassem a solicitação de autorização com um erro `invalid_scope`. Os metadados obtidos de um `authServerMetadataUrl` configurado ainda fornecem seus `scopes_supported` como os escopos solicitados.

Se o servidor de autorização anuncia `offline_access` em `scopes_supported`, Claude Code o acrescenta aos escopos fixados para que o token de acesso possa ser atualizado sem um novo login no navegador.

Se o servidor depois retorna um 403 `insufficient_scope` para uma chamada de ferramenta, a chamada falha com uma mensagem [`precisa de permissões adicionais`](/docs/pt/errors#mcp-server-needs-you-to-sign-in-again) que nomeia o escopo que o servidor solicita. O servidor aparece como necessitando autenticação em `/mcp`.

Se esse escopo não estiver em seu `oauth.scopes` fixado, adicione-o, depois execute `/mcp` e autentique o servidor novamente. Claude Code solicita os escopos fixados em vez do escopo que o servidor nomeou, então se você se autenticar novamente sem adicioná-lo, o token que você obtém ainda não o possui.

<h3 id="use-dynamic-headers-for-custom-authentication">
  Usar cabeçalhos dinâmicos para autenticação personalizada
</h3>

Se seu servidor MCP usar um esquema de autenticação diferente de OAuth, como Kerberos, tokens de curta duração ou um SSO interno, use `headersHelper` para gerar cabeçalhos de solicitação no momento da conexão. Claude Code executa o comando e mescla sua saída nos cabeçalhos de conexão.

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "/opt/bin/get-mcp-auth-headers.sh"
    }
  }
}
```

O comando também pode ser inline:

```json theme={null}
{
  "mcpServers": {
    "internal-api": {
      "type": "http",
      "url": "https://mcp.internal.example.com",
      "headersHelper": "echo '{\"Authorization\": \"Bearer '\"$(get-token)\"'\"}'"
    }
  }
}
```

**Requisitos:**

* O comando deve escrever um objeto JSON de pares chave-valor de string para stdout
* Claude Code executa o comando em um shell e desiste dele após 10 segundos
* Claude Code escolhe o diretório de trabalho do comando por [onde você configurou o servidor](#where-the-helper-runs), então forneça o script como um caminho absoluto ou coloque-o em `PATH`
* Cabeçalhos dinâmicos substituem qualquer `headers` estático com o mesmo nome

Claude Code executa o auxiliar novamente em cada conexão, no início da sessão e ao reconectar, uma vez que a [regra de confiança para servidores de escopo de projeto e local](#trust-a-folder-before-its-headershelper-runs) permite que ele seja executado. Ele não armazena em cache o resultado, então seu script é responsável por qualquer reutilização de token.

Se uma chamada de ferramenta retorna `401 Unauthorized` ou `403 Forbidden`, Claude Code automaticamente executa novamente o auxiliar sob a mesma regra, reconecta com os cabeçalhos atualizados e tenta novamente a chamada uma vez. Claude Code marca o servidor como necessitando autenticação em `/mcp` apenas se essa tentativa também falhar.

Quando a saída do auxiliar inclui um cabeçalho `Authorization`, Claude Code usa essa credencial como a autenticação do servidor e não volta para OAuth para o servidor.

Se o servidor rejeita a credencial do auxiliar ao conectar, Claude Code relata a conexão como falha em vez de marcar o servidor como necessitando autenticação. Corrija a credencial que seu auxiliar retorna, depois reconecte de `/mcp` para executar novamente o auxiliar.

Claude Code define essas variáveis de ambiente ao executar o auxiliar:

| Variável                      | Valor                                                                                                                 |
| :---------------------------- | :-------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_MCP_SERVER_NAME` | o nome do servidor MCP                                                                                                |
| `CLAUDE_CODE_MCP_SERVER_URL`  | a URL do servidor MCP                                                                                                 |
| `CLAUDE_PLUGIN_ROOT`          | o diretório raiz do plugin. Definido apenas quando um [plugin](/docs/pt/plugins/components#mcp-servers) fornece o servidor |

Use essas para escrever um único script auxiliar que serve múltiplos servidores MCP.

Um `headersHelper` fornecido por plugin não pode referenciar os valores [`${user_config.*}`](/docs/pt/plugins/manifest-reference#user-configuration) do plugin, porque o comando é executado através de um shell. Claude Code relata o servidor como mal configurado com um [erro](/docs/pt/errors#plugin-command-references-user-config) e não substitui o valor. Coloque `${user_config.KEY}` no campo `headers` do servidor, que não é analisado por shell, ou faça o script auxiliar ler o valor de um arquivo de configuração. Antes da v2.1.207, `headersHelper` substituía valores `${user_config.*}`.

<h4 id="where-the-helper-runs">
  Onde o auxiliar é executado
</h4>

Claude Code escolhe o diretório de trabalho do comando `headersHelper` a partir da configuração que declara o servidor. Um `cd` que Claude executa em Bash não o move, e [`/cd`](/docs/pt/permissions#move-the-session-to-another-directory) o move apenas para servidores que são executados a partir do diretório de trabalho primário da sessão. Cada linha abaixo fornece o diretório contra o qual um caminho relativo em seu comando `headersHelper` é resolvido.

| Onde você configurou o servidor                                                                                                                                                                                         | Diretório de trabalho                                                                                  |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------- |
| Um [plugin](/docs/pt/plugins/components#mcp-servers)                                                                                                                                                                         | O diretório raiz do plugin. Requer Claude Code v2.1.195 ou posterior                                   |
| Um `.mcp.json` de projeto ou um servidor de [escopo local](#local-scope)                                                                                                                                                | O diretório do projeto no qual o servidor é declarado                                                  |
| Um arquivo de agente em seu projeto, um servidor da opção `mcpServers` do SDK ou método `setMcpServers()`, ou [`--mcp-config`](/docs/pt/cli-reference)                                                                       | O [diretório de trabalho primário](/docs/pt/permissions#working-directories) da sessão                      |
| [Escopo de usuário](#user-scope), [MCP gerenciado](/docs/pt/managed-mcp), um [conector claude.ai](#use-mcp-servers-from-claude-ai), ou um arquivo de agente de fora de seu projeto, incluindo um de um diretório `--add-dir` | Seu diretório de configuração, `~/.claude` a menos que você defina [`CLAUDE_CONFIG_DIR`](/docs/pt/env-vars) |

Antes da v2.1.238, Claude Code também executava os auxiliares de servidores de escopo de usuário, gerenciados e conector claude.ai, e de arquivos de agente de fora de seu projeto, a partir do diretório no qual você o iniciou.

<h4 id="which-variables-a-helper-can-read">
  Quais variáveis um auxiliar pode ler
</h4>

Um `headersHelper` que um repositório ou plugin fornece é um comando que você não escreveu, então Claude Code o executa sem as variáveis de credencial do seu ambiente, como `ANTHROPIC_API_KEY`. Onde você configurou o servidor decide se isso se aplica:

* **Removidas**: um servidor em um `.mcp.json` de projeto ou em um plugin, e um servidor inline em um arquivo de agente de seu projeto ou de um diretório `--add-dir`
* **Não removidas**: um servidor em [escopo de usuário](#user-scope) ou [escopo local](#local-scope), em [MCP gerenciado](/docs/pt/managed-mcp), de um [conector claude.ai](#use-mcp-servers-from-claude-ai), ou fornecido pelo SDK ou [`--mcp-config`](/docs/pt/cli-reference), e um servidor inline em um arquivo de agente de `~/.claude/agents/`, de configurações gerenciadas, ou passado com `--agents`

Além das variáveis `GIT_CONFIG_KEY_<n>` do Git, Claude Code remove todas as variáveis do seu ambiente cujo nome parece uma credencial, como um nome com `TOKEN`, `SECRET`, `PASSWORD`, `KEY`, ou `AUTH` nele em qualquer caso de letra, então `ANTHROPIC_API_KEY` e `MY_REGISTRY_TOKEN` são ambos removidos. Claude Code também remove uma lista fixa de variáveis de credencial cujos nomes não seguem esse padrão, como `ANTHROPIC_CUSTOM_HEADERS`.

Quando isso se aplica ao seu auxiliar, faça o script ler sua credencial de um arquivo ou de um armazenamento de credenciais. Se a `url` do servidor [expande uma dessas variáveis](#environment-variable-expansion-in-mcp-json), o valor `CLAUDE_CODE_MCP_SERVER_URL` que o auxiliar recebe tem essa parte substituída por `REDACTED` também.

<h4 id="trust-a-folder-before-its-headershelper-runs">
  Confiar em uma pasta antes de seu headersHelper ser executado
</h4>

Claude Code executa um `headersHelper` como um comando shell arbitrário. Para um servidor em um `.mcp.json` de projeto ou em [escopo local](#local-scope), ele executa o auxiliar apenas após você aceitar o [diálogo de confiança](/docs/pt/permissions#project-allow-rules-and-workspace-trust) para o diretório do projeto no qual o servidor é declarado. Antes da v2.1.238, uma sessão `claude -p` ou SDK executava esses auxiliares sem verificar confiança, e uma sessão interativa os executava uma vez que você tinha confiado em uma pasta pai.

* **Confiança que não conta**: a confiança de uma pasta pai, e a confiança automática que uma sessão `claude -p` ou SDK obtém para [hooks em arquivos de configurações](/docs/pt/permissions#what-runs-before-you-trust-a-folder)
* **Até você confiar na pasta**: Claude Code conecta o servidor apenas com seus `headers` estáticos. Em uma sessão `claude -p` ou SDK ele também imprime uma linha [`headersHelper not run`](/docs/pt/errors#headershelper-not-run) por servidor para stderr, dizendo-lhe como conceder a confiança.
* **Confiança sem um diálogo**: defina `projects["<path>"].hasTrustDialogAccepted` para `true` em `~/.claude.json`. `<path>` é a pasta que [Regras de permissão de projeto e confiança de espaço de trabalho](/docs/pt/permissions#project-allow-rules-and-workspace-trust) diz que Claude Code usa como chave para a confiança.

Claude Code aplica a mesma regra a um servidor declarado inline em um [arquivo de agente](/docs/pt/sub-agents#scope-mcp-servers-to-a-subagent), verificando onde esse arquivo de agente veio: seu projeto, para um arquivo em seu diretório `.claude/agents/`, ou um diretório `--add-dir`. Até você [confiar nesse projeto ou diretório em si](/docs/pt/permissions#what-runs-before-you-trust-a-folder), Claude Code não carrega o servidor, então seu auxiliar nunca é executado também.

<h2 id="add-mcp-servers-from-json-configuration">
  Adicionar servidores MCP de configuração JSON
</h2>

Se você tiver uma configuração JSON para um servidor MCP, você pode adicioná-la diretamente:

<Steps>
  <Step title="Adicione um servidor MCP de JSON">
    ```bash theme={null}
    # Sintaxe básica
    claude mcp add-json <name> '<json>'

    # Exemplo: Adicionar um servidor HTTP com configuração JSON
    claude mcp add-json weather-api '{"type":"http","url":"https://api.weather.com/mcp","headers":{"Authorization":"Bearer token"}}'

    # Exemplo: Adicionar um servidor stdio com configuração JSON
    claude mcp add-json local-weather '{"type":"stdio","command":"/path/to/weather-cli","args":["--api-key","abc123"],"env":{"CACHE_DIR":"/tmp"}}'

    # Exemplo: Adicionar um servidor HTTP com credenciais OAuth pré-configuradas
    claude mcp add-json my-server '{"type":"http","url":"https://mcp.example.com/mcp","oauth":{"clientId":"your-client-id","callbackPort":8080}}' --client-secret
    ```
  </Step>

  <Step title="Verifique se o servidor foi adicionado">
    ```bash theme={null}
    claude mcp get weather-api
    ```
  </Step>
</Steps>

<Tip>
  Dicas:

  * Certifique-se de que o JSON está adequadamente escapado no seu shell
  * O JSON deve estar em conformidade com o esquema de configuração do servidor MCP
  * Você pode usar `--scope user` para adicionar o servidor à sua configuração de usuário em vez da específica do projeto
</Tip>

<h2 id="import-mcp-servers-from-claude-desktop">
  Importar servidores MCP do Claude Desktop
</h2>

Se você já configurou servidores MCP no Claude Desktop, você pode importá-los:

<Steps>
  <Step title="Importe servidores do Claude Desktop">
    ```bash theme={null}
    # Sintaxe básica 
    claude mcp add-from-claude-desktop 
    ```
  </Step>

  <Step title="Selecione quais servidores importar">
    Após executar o comando, você verá um diálogo interativo que permite selecionar quais servidores você deseja importar.
  </Step>

  <Step title="Verifique se os servidores foram importados">
    ```bash theme={null}
    claude mcp list 
    ```
  </Step>
</Steps>

Os nomes de servidores adicionados através de comandos `claude mcp` podem conter apenas letras, números, hífens e sublinhados. O Claude Desktop não aplica essa restrição, portanto um servidor do Claude Desktop cujo nome contém qualquer outro caractere, como um espaço, não pode ser importado. A importação relata cada nome que rejeita e ainda importa os outros servidores que você selecionou. Antes da v2.1.205, o primeiro nome inválido interrompia a importação e nenhum dos servidores selecionados era adicionado.

<Tip>
  Dicas:

  * Este recurso funciona apenas em macOS e Windows Subsystem for Linux (WSL)
  * Ele lê o arquivo de configuração do Claude Desktop de sua localização padrão nessas plataformas
  * Use a flag `--scope user` para adicionar servidores à sua configuração de usuário
  * Os servidores importados mantêm os mesmos nomes que no Claude Desktop quando o nome contém apenas letras, números, hífens e sublinhados. O Claude Code relata um servidor cujo nome contém qualquer outro caractere e o ignora
  * Se servidores com os mesmos nomes já existirem, eles receberão um sufixo numérico (por exemplo, `server_1`)
</Tip>

<h2 id="use-mcp-servers-from-claude-ai">
  Usar servidores MCP do claude.ai
</h2>

Se você fez login no Claude Code com uma conta [claude.ai](https://claude.ai), os servidores MCP que você adicionou no claude.ai, conhecidos como [conectores](https://claude.com/docs/connectors), estão automaticamente disponíveis no Claude Code:

<Steps>
  <Step title="Configurar servidores MCP no claude.ai">
    Adicione servidores em [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Nos planos Team e Enterprise, apenas administradores podem adicionar servidores.
  </Step>

  <Step title="Autenticar o servidor MCP">
    Conclua quaisquer etapas de autenticação necessárias no claude.ai.
  </Step>

  <Step title="Visualizar e gerenciar servidores no Claude Code">
    No Claude Code, use o comando:

    ```text wrap theme={null}
    /mcp
    ```

    Os servidores do claude.ai aparecem na lista com indicadores mostrando que vêm do claude.ai.
  </Step>
</Steps>

O Claude Code marca um conector como `managed` em `/mcp` e no gerenciador [`/plugin`](/docs/pt/plugins/install) quando sua organização gerencia sua autenticação no claude.ai. O status de gerenciado não altera como o Claude Code se conecta ao conector ou aplica os [controles de ferramentas](#organization-controls-on-connector-tools) da sua organização.

Os conectores aos quais você nunca fez login estão recolhidos atrás de uma linha `Show unused connectors` no final da seção claude.ai, para que uma lista provisionada pela organização não preencha o painel. Selecione a linha para expandi-los. Um conector ao qual você fez login antes permanece visível mesmo quando atualmente precisa de reautenticação.

Os conectores do claude.ai são buscados apenas quando seu [método de autenticação](/docs/pt/authentication#authentication-precedence) ativo é um login de assinatura do claude.ai. Eles não são carregados, mesmo se você executou `/login` anteriormente, quando:

* `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, ou `apiKeyHelper` está ativo
* Um provedor de terceiros, como Amazon Bedrock ou Agent Platform do Google Cloud, está ativo
* `ANTHROPIC_PROFILE`, as variáveis de federação, ou um [perfil Anthropic](/docs/pt/authentication#anthropic-profiles-and-federation-credentials) ativo fornece a credencial
* `CLAUDE_CODE_OAUTH_TOKEN` contém um token de [`claude setup-token`](/docs/pt/authentication#generate-a-long-lived-token), que pode apenas fazer solicitações de modelo

Se `/mcp` não listar um conector que você adicionou, execute `/status` para confirmar qual método de autenticação está ativo. Desdefina essa variável de ambiente, remova a configuração `apiKeyHelper`, ou [desative o perfil](/docs/pt/authentication#anthropic-profiles-and-federation-credentials), depois execute `/login` para selecionar sua conta claude.ai.

Se um problema de rede temporário impedir que a lista de conectores seja carregada quando sua sessão inicia, o Claude Code tenta novamente a busca até três vezes em segundo plano, e os conectores aparecem assim que uma tentativa é bem-sucedida. Se ainda não tiverem aparecido, reinicie o Claude Code para buscar a lista novamente.

Se `/mcp` mostrar um conector como `connected · session token rejected`, ou sua visualização de detalhes mostrar [`claude.ai rejected the session token`](/docs/pt/errors#claude-ai-rejected-the-session-token), o claude.ai rejeitou o token de seu login do Claude Code, geralmente porque o login expirou e não pôde ser atualizado. Autorizar o conector novamente não limpa esse estado, porque a autorização própria do conector no claude.ai não é o que foi rejeitado. Para limpá-lo:

1. Execute `/login` para fazer login novamente.
2. Reconecte o conector de `/mcp`.

Antes da v2.1.222, o Claude Code marcava os conectores como precisando de autenticação, e autorizá-los não resolvia.

Um servidor que você adicionou no Claude Code tem [precedência](#scope-hierarchy-and-precedence) sobre um conector claude.ai que aponta para a mesma URL. Quando isso acontece, `/mcp` lista o conector como oculto e mostra como remover a duplicata se você preferir usar o conector.

Alguns conectores hospedados pela Anthropic, como Microsoft 365, Gmail e Google Calendar, não suportam OAuth local do Claude Code porque o provedor de identidade upstream aceita apenas a URL de redirecionamento que o claude.ai registrou. Quando um servidor que você adicionou com `claude mcp add` ou em `.mcp.json` aponta para um desses hosts e você faz login nele de `/mcp` ou com `claude mcp login`, o Claude Code mostra [`is Anthropic-hosted and doesn't support local OAuth`](/docs/pt/errors#anthropic-hosted-and-doesnt-support-local-oauth), direcionando você para conectar o serviço em [claude.ai/customize/connectors](https://claude.ai/customize/connectors).

Depois de remover sua entrada com `claude mcp remove <name>` e conectar o serviço no claude.ai, o conector aparece no Claude Code automaticamente.

<h3 id="how-connectors-reach-claude-code">
  Como os conectores chegam ao Claude Code
</h3>

Quais configurações governam um conector claude.ai depende de onde sua sessão é executada, porque apenas algumas sessões buscam conectores do claude.ai. Cada linha abaixo nomeia como os conectores chegam em um tipo de sessão e o que os controla lá. As [sessões WSL](/docs/pt/desktop-wsl#what-works-in-a-wsl-session) do aplicativo desktop não têm uma linha porque os conectores ainda não estão disponíveis nelas.

| Onde a sessão é executada                                                                                               | Como os conectores chegam                   | O que os governa                                                                                                                                                                                                                                            |
| :---------------------------------------------------------------------------------------------------------------------- | :------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Terminal, [VS Code](/docs/pt/vs-code), [JetBrains](/docs/pt/jetbrains), e sessões [Agent SDK](/docs/pt/agent-sdk/claude-code-features) | O Claude Code os busca do claude.ai         | As configurações nesta seção e [configuração MCP gerenciada](/docs/pt/managed-mcp)                                                                                                                                                                               |
| [Sessões na nuvem](/docs/pt/claude-code-on-the-web)                                                                          | O host remoto os passa                      | Suas configurações de organização claude.ai, mais as configurações de [lista de permissões e lista de negações](/docs/pt/managed-mcp#policy-based-control-with-allowlists-and-denylists) que chegam à sessão e qualquer `managed-mcp.json` no host que a executa |
| [Aplicativo desktop](/docs/pt/desktop) sessões locais e SSH                                                                  | O aplicativo desktop os entrega em processo | Entradas `blocked` nos [controles de ferramentas de conector](#organization-controls-on-connector-tools) da sua organização                                                                                                                                 |

[`disableClaudeAiConnectors`](#disable-claude-ai-connectors), `ENABLE_CLAUDEAI_MCP_SERVERS`, e [`allowAllClaudeAiMcps`](/docs/pt/settings-reference#allowallclaudeaimcps) atuam apenas na primeira linha, os conectores que o Claude Code busca. As outras duas linhas diferem dela nestas formas:

* **Sessões na nuvem**: entradas `allowedMcpServers` e `deniedMcpServers` que chegam à sessão, por exemplo através de [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings), também filtram os conectores entregues. O proxy da sessão reescreve a URL de cada conector, então um padrão `serverUrl` escrito para a URL própria do conector não o corresponde. Para admitir conectores entregues junto com uma lista de permissões de URL em um ambiente auto-hospedado, adicione as entradas `serverUrl` listadas em [Tráfego de conector sai de sua rede](/docs/pt/self-hosted-environments-deploy#connector-traffic-leaves-your-network). O Claude Code descarta os conectores entregues quando um `managed-mcp.json` está presente no host que executa a sessão, como um [host de executor auto-hospedado](/docs/pt/self-hosted-environments-configuration#mcp-servers), independentemente de você definir `allowAllClaudeAiMcps`.
* **Sessões locais e SSH do aplicativo desktop**: o aplicativo desktop registra os conectores como servidores `type: "sdk"` em processo, e nenhuma configuração MCP ou `managed-mcp.json` os alcança. Um usuário mantém um conector fora de suas próprias sessões desconectando-o em [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Uma organização bloqueia as [ferramentas](#organization-controls-on-connector-tools) de um conector ou desativa [Claude Code no aplicativo desktop](/docs/pt/desktop#admin-console-controls) inteiramente.

<h3 id="organization-controls-on-connector-tools">
  Controles de organização em ferramentas de conector
</h3>

Sua organização pode definir controles por ferramenta em [conectores claude.ai](https://claude.com/docs/connectors). O Claude Code lê essas configurações na inicialização e as aplica localmente, exceto nas [sessões locais e SSH](#how-connectors-reach-claude-code) do aplicativo desktop. Lá, o aplicativo desktop retém ferramentas `blocked` antes de entregar um conector, e a configuração `ask` não chega ao Claude Code, então ele aplica as [regras de permissão](/docs/pt/permissions) ordinárias da sessão a essas ferramentas em vez de solicitar a cada chamada. Em sessões onde o Claude Code busca conectores, execute `/mcp` para ver qual configuração se aplica a cada ferramenta em um conector.

* **Ferramenta definida como `ask`**: O Claude Code solicita a cada chamada com o motivo `Your organization requires approval for this tool`. O prompt aparece mesmo em [modos de permissão](/docs/pt/permissions#permission-modes) `acceptEdits`, `auto` e `bypassPermissions`, e nunca oferece uma opção para lembrar sua escolha. [Regras de permissão](/docs/pt/permissions) que correspondem à ferramenta também não pulam o prompt. No modo `dontAsk`, que nunca solicita, o Claude Code nega a chamada.
* **Ferramenta definida como `blocked`**: O Claude Code filtra a ferramenta antes de Claude vê-la, então ela nunca aparece na lista de ferramentas. O aplicativo desktop e o chat claude.ai aplicam a mesma configuração `blocked`, então Claude não pode usar a ferramenta lá também, e você não pode reter uma ferramenta das sessões do aplicativo desktop enquanto a mantém disponível no chat. O aplicativo desktop pula um conector cujas ferramentas estão todas bloqueadas.

<h3 id="disable-claude-ai-connectors">
  Desabilitar conectores claude.ai
</h3>

O Claude Code aplica [`disableClaudeAiConnectors`](/docs/pt/settings-reference#disableclaudeaiconnectors) apenas aos conectores que [busca](/docs/pt/settings-reference#disableclaudeaiconnectors), não aos conectores que um host na nuvem ou o aplicativo desktop entrega. Para desativar os conectores que busca, defina a configuração como `true` em qualquer escopo de configurações:

```json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Esta configuração usa semântica any-source-true: `true` em qualquer fonte de configurações tem precedência. Um `.claude/settings.json` de projeto verificado pode optar um repositório pelos conectores que o Claude Code busca, mas um `false` em nível de projeto não pode reabilitar conectores que um `true` em nível de usuário ou política desabilitou. Servidores passados explicitamente via `--mcp-config` não são afetados.

Você também pode definir a variável de ambiente `ENABLE_CLAUDEAI_MCP_SERVERS` como `false`, que tem o mesmo efeito para a sessão de shell atual:

```bash theme={null}
ENABLE_CLAUDEAI_MCP_SERVERS=false claude
```

Para bloquear conectores claude.ai individuais em vez de todos eles, adicione-os a [`deniedMcpServers`](/docs/pt/managed-mcp) por nome ou por padrão de URL. Por exemplo, uma entrada `serverName` de `"claude.ai Slack"` bloqueia o conector Slack. Você também pode executar `/mcp` para alternar qualquer conector que o Claude Code busca ativado ou desativado apenas para o projeto atual.

<h2 id="use-claude-code-as-an-mcp-server">
  Use Claude Code as an MCP server
</h2>

Você pode usar Claude Code como um servidor MCP que outros aplicativos podem conectar:

```bash theme={null}
# Start Claude as a stdio MCP server
claude mcp serve
```

O comando não imprime nada quando inicia. Um servidor MCP stdio se comunica através de stdin e stdout, então um terminal silencioso e bloqueado significa que o servidor está em execução e aguardando um cliente para se conectar.

Você pode usar isso no Claude Desktop adicionando esta configuração ao claude\_desktop\_config.json:

```json theme={null}
{
  "mcpServers": {
    "claude-code": {
      "type": "stdio",
      "command": "claude",
      "args": ["mcp", "serve"],
      "env": {}
    }
  }
}
```

<Warning>
  **Configurando o caminho do executável**: o campo `command` deve fazer referência ao executável Claude Code. Se o comando `claude` não estiver no PATH do seu sistema, você precisará especificar o caminho completo para o executável.

  Para encontrar o caminho completo:

  ```bash theme={null}
  which claude
  ```

  Em seguida, use o caminho completo em sua configuração:

  ```json theme={null}
  {
    "mcpServers": {
      "claude-code": {
        "type": "stdio",
        "command": "/full/path/to/claude",
        "args": ["mcp", "serve"],
        "env": {}
      }
    }
  }
  ```

  Sem o caminho correto do executável, você encontrará erros como `spawn claude ENOENT`.
</Warning>

<Tip>
  Dicas:

  * No Claude Desktop, tente pedir ao Claude para ler arquivos em um diretório, fazer edições e muito mais.
  * Este servidor MCP apenas expõe as ferramentas do Claude Code ao seu cliente MCP, portanto seu próprio cliente é responsável por implementar confirmação do usuário para chamadas de ferramentas individuais.
</Tip>

<h2 id="mcp-output-limits-and-warnings">
  Limites de saída do MCP e avisos
</h2>

Quando as ferramentas do MCP produzem grandes saídas, Claude Code ajuda a gerenciar o uso de tokens para evitar sobrecarregar o contexto da sua conversa:

* **Limite de aviso de saída**: Claude Code exibe um aviso quando qualquer saída de ferramenta do MCP excede 10.000 tokens
* **Limite configurável**: você pode ajustar o máximo de tokens de saída do MCP permitidos usando a variável de ambiente `MAX_MCP_OUTPUT_TOKENS`
* **Limite padrão**: o máximo padrão é 25.000 tokens
* **Escopo**: a variável de ambiente se aplica a ferramentas que não declaram seu próprio limite. Ferramentas que definem [`anthropic/maxResultSizeChars`](#raise-the-limit-for-a-specific-tool) usam esse valor em vez disso para conteúdo de texto, independentemente do que `MAX_MCP_OUTPUT_TOKENS` está definido. Ferramentas que retornam dados de imagem ainda estão sujeitas a `MAX_MCP_OUTPUT_TOKENS`
* **Acima do limite**: quando um resultado sem conteúdo de imagem excede o limite, Claude Code o salva em um arquivo e o substitui na conversa com uma mensagem que nomeia o caminho do arquivo, para que Claude leia o arquivo quando precisar do conteúdo. O arquivo fica no diretório `tool-results` da sessão em [`~/.claude/projects/`](/docs/pt/claude-directory#cleaned-up-automatically).

Para aumentar o limite para ferramentas que produzem grandes saídas:

```bash theme={null}
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

<h3 id="raise-the-limit-for-a-specific-tool">
  Aumentar o limite para uma ferramenta específica
</h3>

Se você está construindo um servidor MCP, você pode permitir que ferramentas individuais retornem resultados maiores do que o limite padrão de persistência em disco, definindo `_meta["anthropic/maxResultSizeChars"]` na entrada de resposta `tools/list` da ferramenta. Claude Code aumenta o limite dessa ferramenta para o valor anotado, até um limite máximo de 500.000 caracteres.

Isso é útil para ferramentas que retornam saídas inerentemente grandes, mas necessárias, como esquemas de banco de dados ou árvores de arquivos completas. Sem a anotação, resultados que excedem o limite padrão são persistidos em disco e substituídos por uma referência de arquivo na conversa.

```json theme={null}
{
  "name": "get_schema",
  "description": "Returns the full database schema",
  "_meta": {
    "anthropic/maxResultSizeChars": 200000
  }
}
```

A anotação se aplica independentemente de `MAX_MCP_OUTPUT_TOKENS` para conteúdo de texto, portanto os usuários não precisam aumentar a variável de ambiente para ferramentas que a declaram. Ferramentas que retornam dados de imagem ainda estão sujeitas ao limite de tokens.

<Warning>
  Se você encontrar frequentemente avisos de saída com servidores MCP específicos que você não controla, considere aumentar o limite `MAX_MCP_OUTPUT_TOKENS`. Você também pode pedir ao autor do servidor para adicionar a anotação `anthropic/maxResultSizeChars` ou para paginar suas respostas. A anotação não tem efeito em ferramentas que retornam conteúdo de imagem; para essas, aumentar `MAX_MCP_OUTPUT_TOKENS` é a única opção.
</Warning>

<h2 id="tool-input-schemas-with-a-root-level-combinator">
  Esquemas de entrada de ferramentas com um combinador no nível raiz
</h2>

Alguns servidores MCP declaram o esquema de entrada de uma ferramenta como uma união JSON Schema, com `anyOf`, `oneOf` ou `allOf` no nível superior do esquema. A API Claude não aceita essas palavras-chave na raiz do esquema. Ela aceita combinadores aninhados dentro de `properties`, que Claude Code envia sem alterações.

Ferramentas com um combinador no nível raiz permanecem disponíveis. Antes de enviar a ferramenta para a API, Claude Code achata o esquema em um único objeto e prepara uma frase para a descrição da ferramenta que diz ao Claude quais grupos de parâmetros pertencem juntos:

* `allOf`: as propriedades de cada branch são mescladas, e a lista `required` de cada branch ainda se aplica
* `anyOf` e `oneOf`: as propriedades de cada branch são mescladas, e a lista `required` de cada branch é descrita na descrição da ferramenta em vez de ser imposta pelo esquema

Seu servidor recebe quaisquer argumentos que Claude escolheu, portanto continue validando a combinação no lado do servidor.

Quando Claude Code não consegue produzir um esquema que a API aceita, ou em uma implantação que não recebe a configuração remota que habilita a reescrita, ele pula essa ferramenta, registra o motivo no log do servidor e deixa as outras ferramentas do servidor disponíveis. Versões anteriores à v2.1.195 pulam todas as ferramentas cujo esquema de entrada tem um `anyOf`, `oneOf` ou `allOf` no nível raiz.

<h2 id="tools-with-invalid-input-schemas">
  Ferramentas com esquemas de entrada inválidos
</h2>

A API Claude verifica o esquema de entrada de cada ferramenta em uma solicitação e rejeita toda a solicitação quando qualquer esquema falha, portanto uma única ferramenta MCP com um esquema malformado faria com que toda solicitação que a inclua falhasse com um erro 400. Claude Code executa duas das verificações da API por conta própria quando carrega as ferramentas de um servidor e exclui cada ferramenta que falharia nelas, para que as outras ferramentas do servidor continuem funcionando:

* Os nomes de propriedades de nível superior devem ter de 1 a 64 caracteres e usar apenas letras ASCII e dígitos, `_`, `.` e `-`
* O esquema deve ser válido em relação ao meta-esquema JSON Schema draft 2020-12. Claude Code aplica essa verificação a esquemas que não declaram `$schema` e esquemas que declaram draft 2020-12. Um esquema que declara qualquer outro dialeto ignora essa verificação, embora a verificação de nome de propriedade acima ainda se aplique

Claude Code executa as verificações após a [reescrita do combinador de nível raiz](#tool-input-schemas-with-a-root-level-combinator), no esquema que realmente enviaria.

Quando Claude Code exclui uma ferramenta, registra o motivo no log do servidor e informa ao Claude quais ferramentas foram excluídas e por quê, para que você possa perguntar ao Claude por que uma ferramenta está faltando. Se você corrigir o esquema no servidor, a ferramenta reaparece na próxima vez que Claude Code carregar as ferramentas do servidor.

Claude Code ativa a exclusão através de um sinalizador de recurso que busca da Anthropic. Em uma [implantação onde a busca de sinalizadores está desativada](/docs/pt/env-vars#features-that-need-feature-flag-fetching), ou em uma máquina cujos sinalizadores nunca chegaram, como uma máquina isolada, Claude Code ainda executa as verificações e registra no log do servidor qual ferramenta seria rejeitada, mas envia o esquema da ferramenta para a API mesmo assim. A API rejeita uma solicitação que inclua esse esquema com [um erro 400 nomeando a ferramenta por sua posição](/docs/pt/errors#tool-input-schema-is-invalid). Antes da v2.1.216, nenhuma implantação executava essas verificações.

O [tratamento do combinador de nível raiz](#tool-input-schemas-with-a-root-level-combinator) é separado e mantém seu próprio comportamento quando a busca de sinalizadores está desativada ou os sinalizadores nunca chegaram.

<h2 id="require-approval-for-a-specific-tool">
  Exigir aprovação para uma ferramenta específica
</h2>

Se você está construindo um servidor MCP, pode marcar uma ferramenta como exigindo aprovação explícita em cada chamada definindo `_meta["anthropic/requiresUserInteraction"]` como `true` na entrada de resposta `tools/list` da ferramenta. O valor deve ser o booleano JSON `true`; qualquer outro valor é ignorado.

Claude Code mostra o prompt de permissão dessa ferramenta em cada chamada, mesmo em [modos de permissão](/docs/pt/permissions#permission-modes) `acceptEdits`, `auto` e `bypassPermissions`, e não oferece uma opção "não perguntar novamente" para ela. [Regras de permissão](/docs/pt/permissions#permission-rule-syntax) que correspondem à ferramenta também não pulam o prompt. No modo `dontAsk`, que nunca solicita, Claude Code nega a chamada.

O prompt tem que chegar a uma pessoa. No modo não interativo com [`--permission-prompt-tool`](/docs/pt/cli-reference#cli-flags), um resultado `allow` da ferramenta de prompt para uma ferramenta marcada é convertido em uma negação com a mensagem `MCP tool requires user interaction; not supported via --permission-prompt-tool`. O callback [`canUseTool`](/docs/pt/agent-sdk/permissions) do Agent SDK recebe essas chamadas e pode aprová-las, porque sua aplicação SDK é esperada que as mostre a um usuário.

Use isso para ferramentas cujo prompt de permissão é em si o ponto, como uma etapa de consentimento ou concessão de acesso onde a aprovação automática significaria que nenhum humano nunca concordou. Outras ferramentas do mesmo servidor mantêm seu comportamento de permissão normal.

A seguinte entrada `tools/list` marca uma ferramenta como sempre exigindo aprovação.

```json theme={null}
{
  "name": "grant_access",
  "description": "Requests access to a protected resource",
  "_meta": {
    "anthropic/requiresUserInteraction": true
  }
}
```

A anotação `anthropic/requiresUserInteraction` requer Claude Code v2.1.199 ou posterior. Versões anteriores a ignoram e aplicam o fluxo de permissão padrão.

Algumas superfícies, como [Remote Control](/docs/pt/remote-control) e aplicações construídas no [Agent SDK](/docs/pt/agent-sdk/overview), normalmente permitem que você aprove chamadas de ferramentas com um toque. Para uma ferramenta marcada com essa anotação, Claude Code retém a ação de um toque e mostra o prompt de permissão completo da ferramenta, então a aprovação ainda vem de uma pessoa respondendo ao prompt em vez de um toque.

Claude Code retém a aprovação de um toque da mesma forma para qualquer solicitação de permissão que apenas o diálogo do terminal possa renderizar completamente, como uma que carrega um aviso de segurança ou uma opção de sempre permitir que a superfície remota não possa mostrar. Você responde essa solicitação no diálogo do terminal em vez de Remote Control. Requer Claude Code v2.1.214 ou posterior.

<h2 id="respond-to-mcp-elicitation-requests">
  Responder a solicitações de elicitação MCP
</h2>

Os servidores MCP podem solicitar entrada estruturada de você durante uma tarefa usando elicitação. Quando um servidor precisa de informações que não consegue obter por conta própria, Claude Code exibe um diálogo interativo e passa sua resposta de volta para o servidor. Nenhuma configuração é necessária do seu lado: diálogos de elicitação aparecem automaticamente quando um servidor os solicita.

Os servidores podem solicitar entrada de duas maneiras:

* **Modo de formulário**: Claude Code mostra um diálogo com campos de formulário definidos pelo servidor (por exemplo, um prompt de nome de usuário e senha). Preencha os campos e envie.
* **Modo de URL**: Claude Code abre uma URL do navegador para autenticação ou aprovação. Conclua o fluxo no navegador e confirme na CLI.

No modo de URL, Claude Code passa a URL como um argumento de linha de comando para o manipulador de URL do seu sistema e limita o tamanho desse argumento. Quando a URL, uma vez escapada para a linha de comando, ultrapassa esse limite, você só pode recusar a solicitação. Cada caractere que precisa ser escapado, como `%` ou `&`, conta quatro vezes em relação ao limite: seu próprio caractere mais três caracteres de escape. Uma URL sem nenhum deles atinge o limite em aproximadamente 8.000 caracteres. Uma URL construída principalmente com percent-escapes, onde cada terceiro caractere é um `%`, atinge em aproximadamente 4.000.

Para responder automaticamente a solicitações de elicitação sem mostrar um diálogo, use o hook [`Elicitation`](/docs/pt/hooks#elicitation).

Se você está construindo um servidor MCP que usa elicitação, consulte a [especificação de elicitação MCP](https://modelcontextprotocol.io/docs/learn/client-concepts#elicitation) para detalhes do protocolo e exemplos de esquema.

<h2 id="use-mcp-resources">
  Usar recursos MCP
</h2>

Os servidores MCP podem expor recursos que você pode referenciar usando menções @, de forma semelhante a como você referencia arquivos.

<h3 id="reference-mcp-resources">
  Referenciar recursos MCP
</h3>

<Steps>
  <Step title="Listar recursos disponíveis">
    Digite `@` no seu prompt para ver recursos disponíveis de todos os servidores MCP conectados. Os recursos aparecem junto com arquivos no menu de preenchimento automático.
  </Step>

  <Step title="Referenciar um recurso específico">
    Use o formato `@server:protocol://resource/path` para referenciar um recurso:

    ```text wrap theme={null}
    Can you analyze @github:issue://123 and suggest a fix?
    ```

    ```text wrap theme={null}
    Please review the API documentation at @docs:file://api/authentication
    ```
  </Step>

  <Step title="Múltiplas referências de recursos">
    Você pode referenciar múltiplos recursos em um único prompt:

    ```text wrap theme={null}
    Compare @postgres:schema://users with @docs:file://database/user-model
    ```
  </Step>
</Steps>

<Tip>
  Dicas:

  * Os recursos são automaticamente buscados e incluídos como anexos quando referenciados
  * Os caminhos dos recursos são pesquisáveis por correspondência aproximada no preenchimento automático de menção @
  * Claude Code fornece automaticamente ferramentas para listar e ler recursos MCP quando os servidores as suportam
  * Os recursos podem conter qualquer tipo de conteúdo que o servidor MCP fornece (texto, JSON, dados estruturados, etc.)
</Tip>

<h2 id="scale-with-mcp-tool-search">
  Escalar com busca de ferramentas MCP
</h2>

A busca de ferramentas mantém o uso de contexto MCP baixo ao adiar as definições de ferramentas até que Claude as necessite. Apenas nomes de ferramentas e instruções do servidor são carregados no início da sessão, portanto adicionar mais servidores MCP tem impacto mínimo na sua janela de contexto. Claude Code não impõe um limite fixo de ferramentas por servidor; o limite prático é o seu orçamento de janela de contexto.

<Note>
  A busca de ferramentas não é suportada em implantações do Microsoft Foundry [hospedadas no Azure](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options), que a rejeitam no lado do servidor: Claude Code detecta a rejeição e carrega as ferramentas MCP antecipadamente para essa implantação. [`ENABLE_TOOL_SEARCH`](#configure-tool-search) não pode substituir isso, pois a rejeição vem da implantação em si.
</Note>

<h3 id="for-mcp-server-authors">
  Para autores de servidores MCP
</h3>

Se você está construindo um servidor MCP, o campo de instruções do servidor se torna mais útil com a busca de ferramentas ativada. As instruções do servidor ajudam Claude a entender quando procurar por suas ferramentas, de forma semelhante a como [skills](/docs/pt/skills) funcionam.

Adicione instruções de servidor claras e descritivas que expliquem:

* Que categoria de tarefas suas ferramentas lidam
* Quando Claude deve procurar por suas ferramentas
* Capacidades principais que seu servidor fornece

Claude Code trunca cada descrição de ferramenta e instruções de cada servidor em 2.048 caracteres por padrão. Mantenha-as concisas e coloque detalhes críticos perto do início.

Para alterar o limite para cada servidor MCP em sua sessão, defina [`CLAUDE_CODE_MAX_MCP_DESCRIPTION_LENGTH`](/docs/pt/env-vars#variables) como um número de caracteres. Esta variável requer Claude Code v2.1.280 ou posterior.

<h3 id="configure-tool-search">
  Configurar busca de ferramentas
</h3>

A busca de ferramentas é ativada por padrão: as ferramentas MCP são adiadas e descobertas sob demanda. Claude Code a desativa quando `ANTHROPIC_BASE_URL` aponta para um host que não é de primeira parte, pois a maioria dos proxies não encaminha blocos `tool_reference`. Defina `ENABLE_TOOL_SEARCH` explicitamente para substituir esse fallback.

Definir [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/pt/env-vars) mantém a busca de ferramentas desativada. Você não pode substituí-la definindo `ENABLE_TOOL_SEARCH` você mesmo. Sua organização pode manter a busca de ferramentas ativada através de [configurações gerenciadas](/docs/pt/managed-settings), no Claude Code v2.1.227 ou posterior. [Desativar capacidades de pré-lançamento](/docs/pt/llm-gateway-protocol#disable-pre-release-capabilities) cobre onde a substituição se aplica e o que a variável remove.

A busca de ferramentas requer um modelo que suporte blocos `tool_reference`: Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 e modelos posteriores. Consulte [compatibilidade de modelo na documentação da API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-search-tool#model-compatibility) para a lista atual.

Na Agent Platform do Google Cloud, Claude Code decide por geração de modelo:

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5 e posteriores**: a busca de ferramentas está ativada por padrão, o mesmo que na API Anthropic.
* **Modelos anteriores da Agent Platform**: Claude Code carrega todas as ferramentas MCP antecipadamente, porque suas pilhas de serviço rejeitam o cabeçalho beta necessário. `ENABLE_TOOL_SEARCH=true` não substitui isso.

Antes da v2.1.221, Claude Code desativava a busca de ferramentas para todos os modelos na Agent Platform do Google Cloud, a menos que você definisse `ENABLE_TOOL_SEARCH=true`.

Controle o comportamento da busca de ferramentas com a variável de ambiente `ENABLE_TOOL_SEARCH`:

| Valor          | Comportamento                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (não definido) | Todas as ferramentas MCP adiadas e carregadas sob demanda. Retorna ao carregamento antecipado na Agent Platform do Google Cloud com modelos anteriores à geração Claude 4.5, quando `ANTHROPIC_BASE_URL` é um host que não é de primeira parte, ou em uma implantação do Microsoft Foundry hospedada no Azure                                                                                                                                                        |
| `true`         | Todas as ferramentas MCP adiadas, exceto em uma implantação do Microsoft Foundry hospedada no Azure, onde a rejeição no lado do servidor ainda força o carregamento antecipado, e em modelos da Agent Platform do Google Cloud anteriores à geração Claude 4.5, onde Claude Code continua carregando ferramentas antecipadamente. Claude Code envia o cabeçalho beta através de proxies e as solicitações falham em proxies que não suportam blocos `tool_reference` |
| `auto`         | Modo de limite: Claude Code carrega as ferramentas que de outra forma adiaria antecipadamente enquanto suas definições totalizam menos de 10% da janela de contexto e adia todas elas uma vez que as definições atingem 10%                                                                                                                                                                                                                                          |
| `auto:N`       | Modo de limite com uma porcentagem personalizada, onde `N` é 0-100. Por exemplo, `auto:5` para 5%                                                                                                                                                                                                                                                                                                                                                                    |
| `false`        | Todas as ferramentas MCP carregadas antecipadamente, sem adiamento                                                                                                                                                                                                                                                                                                                                                                                                   |

```bash theme={null}
# Use a custom 5% threshold
ENABLE_TOOL_SEARCH=auto:5 claude

# Disable tool search entirely
ENABLE_TOOL_SEARCH=false claude
```

Ou defina o valor no campo `env` do seu [settings.json](/docs/pt/settings-reference#env).

Você também pode desativar a ferramenta `ToolSearch` especificamente:

```json theme={null}
{
  "permissions": {
    "deny": ["ToolSearch"]
  }
}
```

<h3 id="exempt-a-server-from-deferral">
  Isentar um servidor do adiamento
</h3>

Se as ferramentas de um servidor devem estar sempre visíveis para Claude sem uma etapa de busca, defina `alwaysLoad` como `true` na configuração desse servidor. Cada ferramenta desse servidor é então carregada no contexto no início da sessão, independentemente da configuração `ENABLE_TOOL_SEARCH`. Use isso para um pequeno número de ferramentas que Claude precisa a cada turno, pois cada ferramenta antecipada consome contexto que de outra forma estaria disponível para sua conversa.

A seguinte entrada `.mcp.json` isenta um servidor HTTP enquanto deixa outros servidores adiados:

```json theme={null}
{
  "mcpServers": {
    "core-tools": {
      "type": "http",
      "url": "https://mcp.example.com/mcp",
      "alwaysLoad": true
    }
  }
}
```

O campo `alwaysLoad` está disponível em todos os tipos de servidor. Um servidor MCP também pode marcar ferramentas individuais como sempre carregadas incluindo `"anthropic/alwaysLoad": true` no objeto `_meta` da ferramenta, que tem o mesmo efeito apenas para essa ferramenta.

Definir `alwaysLoad: true` também faz a inicialização aguardar as ferramentas do servidor, limitado ao tempo limite de conexão padrão de 5 segundos, pois elas devem estar presentes quando o primeiro prompt é construído. Um servidor remoto com uma entrada [`cached`](#server-status-detail) válida fornece suas ferramentas do cache sem se conectar, portanto não retarda a inicialização. Outros servidores se conectam em segundo plano por padrão; defina [`MCP_CONNECTION_NONBLOCKING=0`](/docs/pt/env-vars) para fazer a inicialização aguardá-los também.

<h2 id="use-mcp-prompts-as-commands">
  Use MCP prompts as commands
</h2>

MCP servers can expose prompts that become available as commands in Claude Code.

<h3 id="execute-mcp-prompts">
  Execute MCP prompts
</h3>

<Steps>
  <Step title="Discover available prompts">
    Type `/` to see the commands available to you, including those from MCP servers. Claude Code lists each MCP prompt as `/servername:promptname (MCP)`. Typing `/mcp__servername__promptname` also runs it.
  </Step>

  <Step title="Execute a prompt without arguments">
    ```text wrap theme={null}
    /mcp__github__list_prs
    ```
  </Step>

  <Step title="Execute a prompt with arguments">
    Many prompts accept arguments. Pass them space-separated after the command. Claude Code splits the arguments on whitespace, so each argument is a single token:

    ```text wrap theme={null}
    /mcp__github__pr_review 456
    ```

    ```text wrap theme={null}
    /mcp__jira__create_issue login-bug high
    ```
  </Step>
</Steps>

<Tip>
  Tips:

  * MCP prompts are dynamically discovered from connected servers
  * Arguments are parsed based on the prompt's defined parameters
  * Prompt results are injected directly into the conversation
  * In the `/mcp__servername__promptname` form, Claude Code replaces any character in the server name outside `A-Z`, `a-z`, `0-9`, `_`, and `-` with `_`, and uses the prompt name as the server declares it
</Tip>

<h2 id="managed-mcp-configuration">
  Configuração MCP gerenciada
</h2>

Para organizações que precisam de controle centralizado sobre quais servidores MCP os usuários podem conectar, consulte [Configuração MCP gerenciada](/docs/pt/managed-mcp). Ela aborda a implantação de um conjunto de servidor fixo com `managed-mcp.json`, fornecimento de servidores para cada usuário com `managedMcpServers`, restrição de servidores com `allowedMcpServers` e `deniedMcpServers`, e o que os usuários veem quando um servidor é bloqueado.
