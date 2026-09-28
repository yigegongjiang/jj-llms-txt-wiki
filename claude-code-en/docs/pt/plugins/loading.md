> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referência de carregamento de plugins

> Rastreie de onde Claude Code carrega cada plugin, qual arquivo de configurações decide se ele carrega e por que uma atualização não mudou nada.

Use esta página quando um plugin não carregou, carregou uma cópia diferente da esperada, ou não pegou uma atualização, e você quer ver qual fonte, escopo de configurações ou arquivo em disco decidiu isso. Ela fornece as regras que Claude Code aplica quando uma sessão inicia e cada vez que você executa `/reload-plugins`. Você também pode pedir ao Claude para ler esta página e diagnosticar sua configuração.

<Note>
  Estes casos são cobertos em outras páginas:

  * **Passos de instalação, habilitação, desabilitação e atualização**: veja [Instalar e gerenciar plugins](/docs/pt/plugins/install)
  * **Você tem uma mensagem de erro específica**: veja [Solucionar problemas de plugins](/docs/pt/plugins/troubleshooting)
</Note>

Comece com [Verificar qual estágio um plugin atingiu](#check-which-stage-a-plugin-reached) para os três estágios pelos quais um plugin instalado passa, ou vá para a seção que corresponde ao que você está vendo:

* Um plugin que você desligou ainda carrega: [Encontrar onde um plugin está habilitado](#find-where-a-plugin-is-enabled)
* Uma atualização não mudou nada: [Versões e atualizações](#versions-and-updates)
* Você está olhando para os arquivos sob `~/.claude/plugins/`: [Encontrar plugins em disco](#find-plugins-on-disk)
* Um plugin `--plugin-dir` não carregou, ou um plugin com o mesmo nome carregou em seu lugar: [Conflitos de nome](#name-conflicts)

<h2 id="check-which-stage-a-plugin-reached">
  Verificar qual estágio um plugin atingiu
</h2>

Uma entrada `enabledPlugins` se torna um plugin que você pode usar em estágios: suas configurações a declaram, Claude Code a busca para disco, e a sessão em execução a carrega. Quando um plugin não se comporta como um arquivo de configurações sugere, verifique qual estágio ele atingiu:

* **Declarado, em configurações**: `enabledPlugins` diz quais plugins devem estar ligados, e `extraKnownMarketplaces` diz quais marketplaces devem existir. Quando você executa `claude plugin marketplace add`, Claude Code escreve o marketplace para `extraKnownMarketplaces` em suas configurações de usuário, bem como para disco
* **Buscado, em disco sob `~/.claude/plugins/`**: os registros do que Claude Code buscou, e os arquivos buscados em si:
  * `known_marketplaces.json` registra cada marketplace que Claude Code buscou, com sua `source`, `installLocation`, `lastUpdated` e `autoUpdate`. Há um `known_marketplaces.json` por usuário, então um marketplace que você adiciona em um projeto está disponível em cada projeto
  * `installed_plugins.json` registra cada instalação com seu `scope`, `installPath` e `version`
  * `cache/` contém os arquivos do plugin
* **Carregado, na sessão em execução**: o conjunto de plugins que Claude Code carregou na inicialização ou no último `/reload-plugins`. Mudanças em configurações ou em disco não chegam a esta camada até você executar `/reload-plugins` ou iniciar uma nova sessão. É por isso que `claude plugin update` termina com `Restart to apply changes.` e atualizações em segundo plano o solicitam com `Run /reload-plugins to apply`

<h3 id="plugins-and-marketplaces-that-aren’t-on-disk-at-session-start">
  Plugins e marketplaces que não estão em disco no início da sessão
</h3>

Plugins carregam no início da sessão a partir de `installed_plugins.json` e do cache sem usar a rede. Após a sessão iniciar, Claude Code verifica os marketplaces declarados em segundo plano:

* **Um marketplace que configurações declaram mas `known_marketplaces.json` não tem**: Claude Code o clona, depois recarrega plugins e baixa plugins habilitados que ainda não estão em cache
* **Um marketplace declarado cuja fonte mudou em configurações**: Claude Code o busca novamente da nova fonte e mostra `Plugins changed. Run /reload-plugins to activate.`

Um plugin habilitado que nenhum caminho buscou e que não tem diretório de cache utilizável mostra `Plugin "<name>" not cached at <path>` na aba **Errors** do `/plugin`, e `claude plugin list` adiciona `— run /plugin to refresh` à mesma linha. Para a correção, veja [`Plugin "<name>" not cached at <path>`](/docs/pt/plugins/troubleshooting#plugin-not-cached-at).

<h2 id="find-where-a-plugin-came-from">
  Encontrar de onde um plugin veio
</h2>

Todo plugin tem um id da forma `<name>@<origin>`, que é o que você vê em arquivos de configurações e em `claude plugin list --json`. A parte após `@` diz onde Claude Code encontrou o plugin:

| ID termina em    | Como o plugin chegou lá                                                                                                                                                                                              | Como você o liga ou desliga                                                                                                                                                                                  |
| :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `@<marketplace>` | Você o instalou a partir de um marketplace que adicionou                                                                                                                                                             | `"<name>@<marketplace>": true` ou `false` sob `enabledPlugins` em um arquivo de configurações                                                                                                                |
| `@inline`        | Você iniciou Claude Code com `--plugin-dir` ou `--plugin-url`, definiu [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/pt/env-vars#variables), ou um aplicativo Agent SDK passou a opção `plugins`. Ele carrega apenas para essa sessão | Ligado para a sessão a menos que o manifesto defina `defaultEnabled: false` ou um arquivo de configurações defina `"<name>@inline": false`                                                                   |
| `@skills-dir`    | Você salvou um diretório de plugin que tem um `.claude-plugin/plugin.json` sob `~/.claude/skills/` ou o `.claude/skills/` do projeto                                                                                 | O `defaultEnabled` do manifesto, a menos que um arquivo de configurações defina `"<name>@skills-dir"` como `true` ou `false`                                                                                 |
| `@synced`        | Você ou sua organização o ligou para sua conta claude.ai, e Claude Code o [baixou](#synced-plugins)                                                                                                                  | Ligado a menos que o manifesto defina `defaultEnabled: false` ou um arquivo de configurações defina `"<name>@synced": false`. Um plugin que sua organização marca como obrigatório carrega independentemente |

Para um plugin de marketplace, `<name>` é o nome da entrada em `marketplace.json`; para `@inline` e `@skills-dir` é o `name` no manifesto do plugin.

Os nomes de origem nesta tabela são reservados, então nenhum marketplace pode ser nomeado `inline`, `skills-dir` ou `synced`.

<h3 id="entry-name-and-manifest-name">
  Nome da entrada e nome do manifesto
</h3>

Um plugin de marketplace tem dois nomes, e eles podem diferir:

* **O nome da entrada em `marketplace.json`**: a chave de instalação e habilitação. É o que você escreve em `enabledPlugins`, o que o diretório de cache é nomeado, e o que `claude plugin list` mostra
* **O `name` no manifesto**: sob o qual os componentes do plugin são nomeados, e o que [conflitos de nome](#name-conflicts) comparam

<h3 id="plugins-shared-through-a-repository">
  Plugins compartilhados através de um repositório
</h3>

Para compartilhar um plugin através de um repositório, liste-o sob `enabledPlugins` em `.claude/settings.json` ou coloque-o sob `.claude/skills/`. Claude Code não verifica o diretório `.claude/plugins/` de um projeto.

Uma sessão em nuvem não adiciona os marketplaces que um repositório lista sob [`extraKnownMarketplaces`](/docs/pt/settings-reference#extraknownmarketplaces), porque isso requer o diálogo de confiança do workspace, que uma sessão em nuvem nunca mostra.

Um plugin de diretório de skills com escopo de projeto carrega apenas a partir do `.claude/skills/` do [diretório de trabalho primário](/docs/pt/permissions#working-directories) da sessão, e apenas depois que você aceita o [diálogo de confiança do workspace](/docs/pt/permissions#what-runs-before-you-trust-a-folder) para essa pasta. Ele não [procura diretórios pai até a raiz do repositório](/docs/pt/skills#discovery-from-parent-and-nested-directories) da forma que skills e comandos simples fazem. Se você iniciar a partir de um subdiretório, um plugin na raiz do repositório não carrega. Inicie a partir da raiz do repositório em vez disso, ou [mova a sessão para lá com `/cd`](/docs/pt/permissions#move-the-session-to-another-directory) na v2.1.246 ou posterior.

Um plugin com escopo de projeto é verificado no repositório e chega a cada colaborador que o clona. Como esse conteúdo vem do repositório em vez de você, ele carrega apenas após a mesma verificação de confiança que se aplica às regras de permissão de projeto em `.claude/settings.json`. Confiar em uma pasta pai ou executar com `-p` não é suficiente. Componentes que executam código são ainda mais restritos:

* Servidores MCP que ele declara passam pela [mesma aprovação por servidor](/docs/pt/mcp) que um `.mcp.json` de projeto
* Servidores MCP que ele declara como um [pacote MCP](/docs/pt/plugins/manifest-reference#mcpservers), um arquivo `.mcpb` ou `.dxt`, ou de um arquivo fora do diretório do plugin são ignorados. Declare-os inline ou em um `.mcp.json` dentro do diretório do plugin
* [Monitores em segundo plano](/docs/pt/plugins/components#monitors) não carregam

Plugins com escopo pessoal não têm nenhuma dessas restrições.

Para como escrever plugins `--plugin-dir` e de diretório de skills, veja [Criar plugins](/docs/pt/plugins/create).

<h3 id="synced-plugins">
  Plugins sincronizados de claude.ai
</h3>

Um plugin que você liga para sua conta claude.ai também carrega em Claude Code, ao lado dos plugins que você instala a partir de marketplaces. Isso inclui plugins que sua organização liga para seus membros. Cada um desses plugins carrega como `<name>@synced`, sem marketplace e sem [registro de instalação](#check-which-stage-a-plugin-reached).

Em sessões de terminal, as skills, agentes, hooks, servidores MCP e servidores LSP de um plugin sincronizado todos carregam, com a mesma confiança que um plugin de marketplace que você instalou.

Para os componentes que Cowork carrega, veja [Plugins em claude.ai e em Cowork](https://claude.com/docs/plugins/overview) em claude.com.

Plugins sincronizados carregam em sessões Cowork e em sessões de terminal onde você se conecta com sua conta claude.ai:

* **[Cowork](https://claude.com/product/cowork)**: Claude Code os baixa para o ambiente próprio da sessão quando a sessão inicia
* **Sessões de terminal**: cada vez que você inicia Claude Code, ele sincroniza uma vez em segundo plano, baixando plugins novos e atualizados e removendo aqueles que você ou sua organização desligou. A sincronização em sessões de terminal requer Claude Code v2.1.273 ou posterior

<h4 id="sync-timing-in-terminal-sessions">
  Tempo de sincronização em sessões de terminal
</h4>

Como a sincronização de terminal é executada em segundo plano, ela pode terminar após sua sessão ter iniciado. Quando ela adiciona, atualiza ou remove um plugin sincronizado em uma sessão interativa, você vê `Plugins changed. Run /reload-plugins to activate.` Execute `/reload-plugins` para carregar a mudança nessa sessão, ou deixe para a próxima vez que você iniciar Claude Code.

Se você habilitar um plugin em claude.ai enquanto uma sessão está em execução, o plugin baixa na próxima vez que você iniciar Claude Code.

<h4 id="sign-in-requirements-for-terminal-sync">
  Requisitos de conexão para sincronização de terminal
</h4>

Em seu terminal, plugins sincronizam apenas em sessões onde você se conecta com sua conta claude.ai.

Se você se conectou em uma versão anterior de Claude Code, essa conexão não cobre plugins até Claude Code renová-la em segundo plano. Para obter acesso mais cedo, execute `/login` novamente. A sincronização de plugins então inicia na próxima vez que você iniciar Claude Code.

<h4 id="control-which-synced-plugins-load">
  Controlar quais plugins sincronizados carregam
</h4>

Você pode desligar plugins sincronizados um de cada vez, exceto um plugin que sua organização exige, ou desligar cada plugin sincronizado na máquina:

* **Um plugin**: `claude plugin disable <name>@synced` em seu shell e a aba **Installed** do `/plugin` em uma sessão ambos salvam `"<name>@synced": false` em seu [`enabledPlugins`](/docs/pt/settings-reference#enabledplugins) de nível de usuário. Para manter o plugin fora de um projeto em cada ambiente, defina a mesma chave no `.claude/settings.json` comprometido do projeto
* **Cada plugin sincronizado em uma máquina**: defina [`syncClaudeAiPlugins`](/docs/pt/settings-reference#syncclaudeaiplugins) como `false` em suas configurações de usuário, ou sua organização o define em [configurações gerenciadas](/docs/pt/managed-settings). Claude Code para de baixar, e na próxima vez que você o inicia, ele move os plugins que já sincronizou para `~/.claude/plugins/.trash/` e não os carrega mais. Se sua organização desligar Skills em claude.ai, plugins param de sincronizar também
* **Um plugin que sua organização exige**: um plugin que sua organização marca como obrigatório em claude.ai carrega mesmo se você o desabilitou anteriormente. `claude plugin disable` o recusa com `Plugin "<name>@synced" is required by your organization and can't be disabled here. Contact your admin to change it.`, e `claude plugin list` o marca `required by your org`

Para remover um plugin em claude.ai, veja [Gerenciar plugins instalados](/docs/pt/plugins/install#manage-installed-plugins).

<h2 id="find-where-a-plugin-is-enabled">
  Encontrar onde um plugin está habilitado
</h2>

Você pode definir uma entrada `enabledPlugins` em qualquer uma de seis fontes. A tabela as lista de precedência mais baixa para mais alta, e quem cada uma se aplica. Para os próprios arquivos de configurações, veja [Arquivos de configurações e quem eles afetam](/docs/pt/settings#where-settings-live).

| Fonte       | Onde você a define                                                                                      | Alcança                                                                                                         |
| :---------- | :------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------- |
| `--add-dir` | `.claude/settings.json` ou `.claude/settings.local.json` em um diretório que você passa com `--add-dir` | Apenas esta sessão. Apenas um valor `true` tem efeito, e todas as outras fontes o substituem                    |
| `user`      | `~/.claude/settings.json`                                                                               | Você, em cada projeto                                                                                           |
| `project`   | `.claude/settings.json`                                                                                 | Todos que clonam o repositório                                                                                  |
| `local`     | `.claude/settings.local.json`                                                                           | Você, apenas neste repositório                                                                                  |
| `flag`      | O valor `--settings` que você passa no lançamento                                                       | Apenas esta sessão                                                                                              |
| `managed`   | [Configurações gerenciadas](/docs/pt/managed-settings)                                                       | Cada usuário que a política cobre. `true` força-habilita e `false` bloqueia, e nenhuma outra fonte as substitui |

Essas fontes se mesclam chave por chave. Para cada id de plugin, o valor que se aplica é o de fonte de precedência mais alta que menciona o id. Uma fonte que não menciona o id deixa o valor da fonte de precedência mais baixa em efeito.

<h3 id="disabled-in-user-settings-but-still-loads">
  Desabilitado em configurações de usuário mas ainda carrega
</h3>

Se você definir um plugin como `false` em `~/.claude/settings.json` e ele ainda carregar, um `true` em uma fonte de precedência mais alta o está substituindo. A linha do plugin em `claude plugin list` e em `/plugin` mostra `Disabled in ~/.claude/settings.json but still loads — project settings enable it, which overrides your user setting`. A mensagem nomeia a fonte que o substituiu: `project`, `project, gitignored` para `.claude/settings.local.json`, `cli flag` ou `managed`.

Para optar por não participar de um plugin habilitado por projeto em sua máquina, defina o id como `false` em `.claude/settings.local.json`, que tem precedência mais alta que o arquivo do projeto.

<h3 id="enabled-in-project-settings-but-not-installed">
  Habilitado em configurações de projeto mas não instalado
</h3>

Quando o único `true` de um plugin está em `.claude/settings.json` do projeto, Claude Code não o busca para uma máquina onde ele não está instalado, a menos que sua entrada de marketplace tenha uma [fonte de caminho relativo](/docs/pt/plugins/marketplace-reference#plugin-sources) ou um [diretório seed](/docs/pt/plugins/org#seed-containers-and-ci) já o contenha. Em vez disso, a aba **Errors** do `/plugin` mostra `Plugin "<name>" is enabled in project settings but isn't installed here`.

Um plugin de caminho relativo não precisa de registro de instalação porque carrega do próprio marketplace.

Claude Code busca um plugin com uma fonte externa apenas quando uma dessas fontes o define como `true`:

* Suas configurações de usuário
* Um `.claude/settings.local.json` que git não rastreia
* O sinalizador `--settings`
* Configurações gerenciadas

<h2 id="find-plugins-on-disk">
  Encontrar plugins em disco
</h2>

Claude Code mantém arquivos de plugin e registros de estado sob uma raiz de plugins, que é `~/.claude/plugins` a menos que você defina [`CLAUDE_CODE_PLUGIN_CACHE_DIR`](/docs/pt/env-vars). Cada caminho na tabela é relativo a essa raiz.

| Caminho                                              | O que ele contém                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :--------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cache/<marketplace>/<plugin>/<version>/`            | Um diretório por versão instalada de um plugin de marketplace. `<plugin>` é o nome da entrada de marketplace e `<version>` é a [versão resolvida](#versions-and-updates). `${CLAUDE_PLUGIN_ROOT}` aponta para este diretório                                                                                                                                                                                                                           |
| `data/<plugin-id>/`                                  | O diretório persistente do plugin, exposto como `${CLAUDE_PLUGIN_DATA}`. Para como `<plugin-id>` é formado, veja [Variáveis de caminho e dados persistentes](/docs/pt/plugins/components#path-variables-and-persistent-data). Claude Code o cria quando um componente de plugin o usa pela primeira vez e o mantém através de atualizações. Claude Code o deleta quando você desinstala o plugin de seu último escopo, a menos que você passe `--keep-data` |
| `marketplaces/<name>/`                               | O clone ou download de um marketplace adicionado do GitHub, outro host Git ou uma URL. Um marketplace adicionado de uma fonte local `file` ou `directory` não tem cópia aqui, e seu `installLocation` em `known_marketplaces.json` é o caminho que você forneceu                                                                                                                                                                                       |
| `synced/`                                            | Os plugins que Claude Code [sincronizou de sua conta claude.ai](#synced-plugins)                                                                                                                                                                                                                                                                                                                                                                       |
| `.trash/`                                            | Plugins que a sincronização claude.ai removeu, como depois que você desliga um em claude.ai ou para de sincronizar                                                                                                                                                                                                                                                                                                                                     |
| `installed_plugins.json` e `known_marketplaces.json` | Os registros do que Claude Code instalou e quais marketplaces ele buscou, descritos sob [Verificar qual estágio um plugin atingiu](#check-which-stage-a-plugin-reached). Um [marketplace hospedado em claude.ai](/docs/pt/plugins/install#add-from-claude-ai) é registrado em `known_marketplaces_claudeai.json` em vez disso                                                                                                                               |
| `flagged-plugins.json`                               | Plugins que Claude Code desinstalou porque seus marketplaces os removeram da lista. Eles aparecem na seção **Flagged** do `/plugin`; veja [Hospedar um marketplace](/docs/pt/plugins/host-marketplace)                                                                                                                                                                                                                                                      |

Como `${CLAUDE_PLUGIN_ROOT}` aponta para um diretório de versão, o caminho raiz de um plugin muda a cada versão. Mantenha os arquivos duráveis de um plugin em `${CLAUDE_PLUGIN_DATA}` em vez disso.

<h3 id="in-place-and-copied-plugins">
  Plugins in-place e copiados
</h3>

Claude Code carrega alguns plugins in-place de onde você os mantém e copia o resto para o cache, de acordo com sua origem:

* **Plugins `--plugin-dir` e de diretório de skills**: o diretório carrega in-place e nunca é copiado. Um arquivo `--plugin-url` ou um `.zip` de `--plugin-dir` é extraído para um diretório temporário de sessão primeiro
* **Plugins de caminho relativo em um marketplace que você adicionou de um diretório local**: o plugin carrega in-place de seu caminho dentro da pasta do marketplace. Suas edições no diretório de origem têm efeito no próximo início de sessão ou `/reload-plugins`, e você não precisa aumentar a versão. Os processos de hook do plugin e servidores MCP e LSP recebem um `CLAUDE_PLUGIN_ROOT` que aponta para o diretório de origem. Para suas dependências de pacote Node.js, veja [Quando a instalação de dependência é executada](#when-the-dependency-install-runs)
* **Plugins de fonte `command` em [modo de link](/docs/pt/plugins/marketplace-reference#command-plugin-source)**: o diretório que o comando imprimiu carrega in-place, através de links na entrada de cache
* **Cada outro plugin de marketplace**: Claude Code copia o plugin para `cache/<marketplace>/<plugin>/<version>/` na instalação e carrega essa cópia. Arquivos fora do diretório do plugin não são copiados, então quando um script dentro de um plugin copiado lê um caminho acima da raiz do plugin, como `../shared`, ele não os encontra

<h3 id="paths-that-escape-the-plugin-directory">
  Caminhos que escapam do diretório do plugin
</h3>

Se um plugin carrega in-place ou de uma cópia em cache, Claude Code não deixa que ele declare componentes fora de seu próprio diretório. Ele rejeita um caminho de componente que se resolve fora da raiz do plugin, se o caminho é declarado em `plugin.json` ou em uma entrada de marketplace:

* **Um caminho que aponta para fora do plugin como escrito**, como `../shared-utils`
* **Um symlink que leva para fora do plugin**, outro que [links entre plugins dentro de um marketplace](/docs/pt/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks)
* **Em macOS e Linux, um caminho que contém uma barra invertida em qualquer lugar nele**, mesmo quando o caminho fica dentro do plugin. Componentes declarados com caminhos de barra invertida portanto carregam apenas em Windows, então escreva caminhos de componentes com barras normais, como `./commands/deploy.md`

Um caminho rejeitado aparece como um erro [`path escapes plugin directory`](/docs/pt/errors#path-escapes-plugin-directory), e o plugin carrega sem esse componente.

<h3 id="cleanup-of-previous-versions">
  Limpeza de versões anteriores
</h3>

Quando você atualiza ou desinstala um plugin, Claude Code escreve um marcador `.orphaned_at` no diretório de versão anterior. Ele remove esse diretório em uma limpeza em segundo plano 14 dias depois, então uma sessão que já carregou a versão antiga continua em execução.

A varredura é executada apenas enquanto `installed_plugins.json` registra pelo menos uma instalação. Depois que você desinstala seu último plugin, diretórios órfãos ficam até você instalar outro.

<h3 id="node-js-package-dependencies">
  Dependências de pacote Node.js
</h3>

Quando Claude Code copia um plugin para o cache, ele também instala as dependências de pacote Node.js do plugin lá, então os hooks e servidores MCP do plugin podem carregá-las.

Esta seção cobre os pacotes npm e Bun que um plugin declara em seu próprio `package.json`. Para plugins que dependem de outros plugins, veja [versões de dependência de plugin](/docs/pt/plugins/dependencies).

<h4 id="when-the-dependency-install-runs">
  Quando a instalação de dependência é executada
</h4>

Claude Code executa a instalação dentro do diretório de versão copiado cada vez que cria um:

* Quando você instala um plugin
* Quando Claude Code atualiza um plugin para uma nova versão
* No início da sessão quando um plugin habilitado ainda não está em cache, como em uma máquina nova

Para um plugin de caminho relativo [carregado in-place](#in-place-and-copied-plugins) de um marketplace de diretório local, Claude Code não instala as dependências no diretório de origem. Instale-as lá você mesmo, ou de um hook para [`${CLAUDE_PLUGIN_DATA}`](/docs/pt/plugins/components#path-variables-and-persistent-data).

A instalação é executada apenas quando o diretório raiz do plugin contém tanto um `package.json` quanto um lockfile suportado. O lockfile decide qual comando Claude Code executa:

| Lockfile                                     | Comando                                          |
| :------------------------------------------- | :----------------------------------------------- |
| `bun.lock` ou `bun.lockb`                    | `bun install --frozen-lockfile --ignore-scripts` |
| `npm-shrinkwrap.json` ou `package-lock.json` | `npm ci --ignore-scripts`                        |

Se um plugin contém mais de um desses lockfiles, Claude Code usa a primeira correspondência, verificando em ordem: `bun.lock`, `bun.lockb`, `npm-shrinkwrap.json`, `package-lock.json`.

Claude Code pula a instalação para lockfiles Yarn e pnpm e para um `bunfig.toml` ao lado do lockfile Bun:

* Se seu plugin tem apenas um `yarn.lock` ou `pnpm-lock.yaml`, substitua-o por um lockfile npm
* Se um `bunfig.toml` está no mesmo diretório que o lockfile Bun, remova o `bunfig.toml`, ou substitua o lockfile Bun por um lockfile npm

Inclua um lockfile npm para alcançar a maioria dos usuários. Claude Code executa o gerenciador de pacotes do lockfile correspondente do PATH do usuário e não tenta o outro lockfile em vez disso se esse gerenciador de pacotes está faltando.

Para um plugin distribuído através de uma fonte npm, use `npm-shrinkwrap.json`, porque npm exclui `package-lock.json` de pacotes publicados.

<h4 id="limits-on-the-dependency-install">
  Limites na instalação de dependência
</h4>

Claude Code restringe essa instalação de dependência para que nenhum código do plugin ou seus pacotes seja executado durante ela, e limita quanto tempo ela pode levar:

* **Resolução congelada**: Bun e npm instalam exatamente o que o lockfile fixa, e falham em vez de re-resolver versões quando `package.json` e o lockfile discordam
* **Sem scripts de ciclo de vida**: `--ignore-scripts` mantém scripts `preinstall`, `install` e `postinstall` de serem executados, então dependências que constroem módulos nativos nesses scripts baixam mas não compilam durante essa instalação
* **Tempo limite de 60 segundos**: Claude Code para uma instalação que é executada mais tempo e a trata como falha

Claude Code busca um plugin de fonte npm antes dessa instalação de dependência, e nenhum dos scripts de instalação próprios do pacote é executado durante a busca. Veja [fonte de plugin npm](/docs/pt/plugins/marketplace-reference#npm-plugin-source).

Você não pode desligar a instalação automática. Nenhuma configuração ou variável de ambiente a desabilita.

Em redes restritas, veja os [requisitos de acesso à rede](/docs/pt/network-config#network-access-requirements) para os hosts a permitir.

<h4 id="when-the-dependency-install-fails-or-is-skipped">
  Quando a instalação de dependência falha ou é ignorada
</h4>

Uma instalação falha ou ignorada nunca bloqueia o plugin, e cada caso deixa um sinal diferente:

* Uma instalação falha, ou uma ignorada por causa de um lockfile Yarn ou pnpm ou um `bunfig.toml`, aparece como um aviso na saída `claude --debug`
* Um plugin com um `package.json` e nenhum lockfile é ignorado sem uma entrada de log
* Uma instalação com tempo limite pode deixar uma árvore `node_modules` parcial na cópia em cache

Quando a instalação automática não pode fornecer uma dependência, instale-a de um hook para o [diretório de dados persistentes](/docs/pt/plugins/components#path-variables-and-persistent-data). Isso inclui pacotes que precisam de seus scripts de ciclo de vida para construir, dependências Python e plugins bloqueados com Yarn ou pnpm.

<h2 id="versions-and-updates">
  Versões e atualizações
</h2>

Se o autor de um plugin empurrou novos commits e `claude plugin update` imprime `<name> is already at the latest version (<version>).`, a versão que Claude Code computa para o plugin é inalterada, então nada muda em disco.

Claude Code computa uma versão para cada plugin que instala, e essa versão é como ele detecta uma atualização. `claude plugin update` e auto-atualização em segundo plano computam a versão novamente e pulam o plugin quando ela corresponde ao que `installed_plugins.json` registra.

A versão também nomeia o diretório de cache do plugin.

Um manifesto que fixa `"version"` é uma forma da versão computada ficar a mesma através de commits. Veja [Como Claude Code computa a versão](#how-claude-code-computes-the-version) para a ordem de resolução.

Um plugin [carregado in-place](#in-place-and-copied-plugins) de um marketplace de diretório local carrega seus arquivos de origem atuais em cada início de sessão, qualquer que seja sua string de versão. Para um plugin de um [marketplace hospedado em claude.ai](/docs/pt/plugins/install#add-from-claude-ai), a versão que claude.ai registra para o plugin é sua versão, e o `version` do manifesto não é lido.

<h3 id="how-claude-code-computes-the-version">
  Como Claude Code computa a versão
</h3>

Para um marketplace que você adicionou por fonte, Claude Code escolhe a regra pelo tipo `source` da entrada de marketplace do plugin. A [referência de marketplace](/docs/pt/plugins/marketplace-reference#plugin-sources) lista os tipos de fonte. Para cada tipo de fonte nessa lista exceto `command`:

1. O campo `version` no manifesto do plugin vem primeiro
2. Então o campo `version` na entrada de marketplace do plugin
3. Quando nenhum é definido, a versão vem do tipo de fonte:

| Tipo de fonte                                                                              | Versão quando nenhum campo `version` é definido                                                                                              |
| :----------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `github`, `url` ou `git-subdir`                                                            | O SHA do commit da fonte, encurtado para 12 caracteres. Uma versão `git-subdir` também carrega um hash do caminho do subdiretório            |
| `archive`                                                                                  | O resumo SHA-256, encurtado para 12 caracteres: o pino `sha256` na entrada de marketplace, ou o resumo do arquivo baixado quando não há pino |
| Caminho relativo dentro de um marketplace hospedado em Git                                 | O SHA do commit do diretório instalado                                                                                                       |
| Diretório local, quando nem o diretório do plugin nem seu marketplace é um repositório git | `unknown`                                                                                                                                    |
| `npm`                                                                                      | `unknown`                                                                                                                                    |

Claude Code não tira a versão de um repositório que enclausura o caminho de instalação, como um `~/.claude` gerenciado por git.

Para uma fonte `command`, Claude Code sempre deriva a versão do que o comando produziu: um hash de 12 caracteres por si só, ou `<manifest version>-<hash>` quando o manifesto define um. A `version` da entrada de marketplace é ignorada para fontes de comando. Para o que o hash cobre, veja [Modo de cópia e modo de link](/docs/pt/plugins/marketplace-reference#copy-mode-and-link-mode).

Como o manifesto vem primeiro, um manifesto que fixa `"version": "1.0.0"` mantém cada usuário na cópia em cache até seu autor mudar a string, quantos commits eles empurrem. Para deixar usuários rastrearem commits em vez disso, deixe `version` fora tanto do manifesto quanto da entrada. [Hospedar um marketplace](/docs/pt/plugins/host-marketplace) cobre qual escolha se encaixa em qual configuração de lançamento.

<h3 id="when-claude-code-refreshes-a-marketplace-before-an-install">
  Quando Claude Code atualiza um marketplace antes de uma instalação
</h3>

Quando você instala um plugin, Claude Code o procura em sua cópia local do catálogo de marketplace. Você pode executar `/plugin install` em uma sessão ou `claude plugin install` em seu shell, e nomear o plugin com ou sem seu marketplace. A tabela mostra quais dessas combinações atualizam a cópia local.

| Nome do plugin     | Comando                                      | O que Claude Code atualiza                                                          |
| :----------------- | :------------------------------------------- | :---------------------------------------------------------------------------------- |
| `name@marketplace` | `/plugin install` ou `claude plugin install` | O marketplace nomeado, antes da procura                                             |
| `name` sozinho     | `/plugin install`                            | Apenas marketplaces que têm auto-atualização ligada, e apenas após a procura falhar |
| `name` sozinho     | `claude plugin install`                      | Nada. Ele lê os catálogos em cache sem atualizar                                    |

A atualização antes de uma instalação `name@marketplace` não depende da configuração de auto-atualização do marketplace ou de `DISABLE_AUTOUPDATER`.

Quando a atualização falha, a instalação prossegue do catálogo em cache e `claude plugin install` relata `marketplace not refreshed`.

Claude Code pula a atualização antes de uma instalação `name@marketplace` quando:

* O marketplace foi adicionado de uma fonte local `file` ou `directory`, ou é definido inline em configurações com uma [fonte `settings`](/docs/pt/settings-reference#extraknownmarketplaces)
* Um [diretório seed](/docs/pt/env-vars) fornece o marketplace
* Claude Code atualizou o marketplace nos últimos 30 segundos
* Você definiu `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`
* [Configurações gerenciadas](/docs/pt/plugins/org#restrict-what-users-can-install) bloqueiam o marketplace, nesse caso Claude Code também recusa a instalação

<h3 id="when-auto-update-runs">
  Quando auto-atualização é executada
</h3>

Em uma sessão interativa, depois que você envia sua primeira mensagem, Claude Code aguarda um atraso aleatório de até dez minutos. Ele então atualiza cada marketplace com auto-atualização ligada e atualiza os plugins instalados deles em disco.

A sessão em execução mantém as versões que carregou, e você vê `Plugin updated: <name> · Run /reload-plugins to apply`. Se você recarrega ou não, as novas versões carregam no seu próximo lançamento.

<h4 id="which-marketplaces-and-plugins-auto-update">
  Quais marketplaces e plugins auto-atualizam
</h4>

Se um marketplace auto-atualiza segue o primeiro destes que é definido:

1. **`autoUpdate` em sua entrada `extraKnownMarketplaces`** em um arquivo de configurações
2. **`autoUpdate` em sua entrada `known_marketplaces.json`**, que o botão **Enable auto-update** sob `/plugin` **Marketplaces** escreve. Quando um arquivo de configurações também declara o marketplace sob `extraKnownMarketplaces`, o botão escreve `autoUpdate` para essa entrada de configurações também
3. **O padrão**: ligado para marketplaces oficiais da Anthropic como `claude-plugins-official`, desligado para `knowledge-work-plugins` e `first-party-plugins`, ligado para [marketplaces adicionados de claude.ai](/docs/pt/plugins/install#add-from-claude-ai), e desligado para cada outro marketplace

Se você definir `DISABLE_UPDATES=1`, `DISABLE_AUTOUPDATER=1` ou `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`, a passagem inteira é desligada e o botão **Enable auto-update** é ocultado, a menos que você também defina `FORCE_AUTOUPDATE_PLUGINS=1`. A [referência de variáveis de ambiente](/docs/pt/env-vars) cobre o efeito mais amplo de cada variável.

Auto-atualização também pula um plugin cuja entrada de marketplace declara um `headersHelper`. [Instalações e atualizações que recusam um comando em vez de perguntar](/docs/pt/plugins/host-marketplace#installs-and-updates-that-refuse-the-command-instead-of-asking) explica quando tal plugin aparece na aba **Errors** do `/plugin` e como você o atualiza de lá.

Quando um plugin copiado atualiza no meio da sessão, comandos de hook, monitores, servidores MCP e servidores LSP continuam usando o caminho da versão anterior. Execute `/reload-plugins` para mudar hooks, servidores MCP e servidores LSP para o novo caminho. Monitores requerem reinicialização de sessão.

<h3 id="when-a-command-source-re-runs">
  Quando uma fonte de comando é re-executada
</h3>

Plugins com uma fonte `command` não esperam pela [passagem de auto-atualização](#when-auto-update-runs). O diretório impresso reflete o estado da ferramenta no momento em que o comando foi executado, então Claude Code executa o [comando que você aceitou](/docs/pt/plugins/host-marketplace#change-the-command-of-a-command-source) novamente nestes momentos:

* Cada vez que você instala ou atualiza o plugin
* Uma vez por sessão para cada plugin habilitado com fonte de comando, em segundo plano, pouco depois que a sessão inicia. Esta execução não depende da configuração de auto-atualização do marketplace ou de `DISABLE_AUTOUPDATER`
* Na inicialização ou em `/reload-plugins`, quando a versão instalada de um plugin habilitado está faltando do cache de plugin

Claude Code pula as duas execuções em segundo plano quando você define [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/pt/env-vars). Instalações e atualizações explícitas ainda executam o comando com essa variável definida.

Quando a saída com hash do comando mudou, Claude Code instala o resultado como uma nova versão e o recarrega na sessão interativa em execução, mudando [os mesmos componentes que `/reload-plugins` muda](/docs/pt/plugins/cli-reference#reload-plugins). Você vê uma notificação de que o plugin foi recarregado.

Se recarregar in-place invalidaria o cache de prompt da sessão, Claude Code em vez disso o solicita a executar `/reload-plugins`, que [avisa sobre o custo do cache e se aplica quando re-executado com `--force`](/docs/pt/prompt-caching#enabling-or-disabling-a-plugin).

<h2 id="name-conflicts">
  Conflitos de nome
</h2>

Quando plugins habilitados de diferentes origens compartilham um nome de manifesto, esta ordem decide qual carrega, de precedência mais alta para mais baixa:

1. Um plugin cujo id aparece em configurações gerenciadas `enabledPlugins`, como `true` ou `false`. Uma cópia `--plugin-dir` cujo nome de manifesto corresponde à parte de nome do id não é carregada, e você vê `--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings`
2. Um plugin `--plugin-dir`, `--plugin-url` ou `CLAUDE_CODE_PLUGIN_DIRS` habilitado. Ele substitui um plugin de marketplace instalado com o mesmo nome ou um plugin de diretório de skills:
   * **Um plugin de marketplace instalado**: substituído silenciosamente. `claude plugin list` ainda mostra a linha de marketplace como habilitada, porque essa linha reflete suas configurações. Apenas o log que Claude Code escreve sob `~/.claude/debug/` quando você inicia com `--debug` registra `Plugin "<name>" from --plugin-dir overrides installed version`
   * **Um plugin de diretório de skills**: substituído com uma linha de aba **Errors** do `/plugin` que lê `Not loaded — the name "<name>" is already taken by a session-only plugin (--plugin-dir / --plugin-url), which takes precedence`
3. Um plugin de marketplace instalado. Um plugin de diretório de skills com o mesmo nome recebe a mesma linha `Not loaded`, nomeando o plugin instalado
4. Um plugin de diretório de skills. Entre dois destes, a cópia sob `~/.claude/skills/` carrega e a cópia do `.claude/skills/` do projeto é descartada, com uma linha que diz qual caminho a sombreou
5. Um plugin [sincronizado de claude.ai](#synced-plugins). Quando um plugin habilitado de qualquer outra origem corresponde a seu nome, Claude Code carrega esse plugin e relata a cópia sincronizada como não carregada. Para usar a cópia claude.ai em vez disso, desabilite sua própria cópia

Como a ordem compara nomes de manifesto, um plugin `--plugin-dir` nomeado `hello-plugin` substitui `hello@example-marketplace` quando o manifesto desse plugin também diz `"name": "hello-plugin"`.

<h3 id="keep-a-session-only-plugin-from-loading">
  Manter um plugin de sessão única de carregar
</h3>

Para manter um plugin `--plugin-dir` de sombrear qualquer coisa, ou desligá-lo quando um processo pai passa o sinalizador para você, defina seu id como `false` em qualquer arquivo de configurações. Para um plugin cujo nome de manifesto é `hello-plugin`, a entrada é `"enabledPlugins": {"hello-plugin@inline": false}`. Um plugin de sessão única desabilitado não sombra, então a cópia de marketplace ou diretório de skills carrega em vez disso.

<h2 id="next-steps">
  Próximos passos
</h2>

* [Instalar e gerenciar plugins](/docs/pt/plugins/install): os passos de instalação, habilitação, desabilitação e atualização em si
* [Solucionar problemas de plugins](/docs/pt/plugins/troubleshooting): mensagens de erro pelo estágio que as produz
* [Referência de comandos de plugin](/docs/pt/plugins/cli-reference): os sinalizadores e comandos nomeados nesta página
* [Gerenciar plugins para sua organização](/docs/pt/plugins/org): as configurações gerenciadas que força-habilitam ou bloqueiam plugins
