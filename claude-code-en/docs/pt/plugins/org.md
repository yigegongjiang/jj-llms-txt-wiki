> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gerenciar plugins do Claude Code para sua organização

> Controle quais plugins o Claude Code instala e permite em toda a sua organização através de configurações gerenciadas.

As configurações gerenciadas permitem que você decida quais plugins o Claude Code instala e permite em cada máquina da sua organização. Os usuários não podem substituí-las. Você as entrega como [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) do console de administração do claude.ai ou como configurações gerenciadas por endpoint através de MDM ou um arquivo `managed-settings.json`. A maioria dos controles nesta página funciona apenas a partir de configurações gerenciadas.

Esta página é para administradores e as configurações aqui governam o Claude Code.

<Note>
  Estes casos são cobertos em outras páginas:

  * **Instalando plugins para você mesmo**: comece em [Install plugins](/docs/pt/plugins/install)
  * **Controlando quais plugins os membros podem usar no claude.ai e Cowork**: veja [Manage plugins for your organization](https://support.claude.com/en/articles/13837433) no centro de ajuda
  * **A página de plugins nas configurações de administração do claude.ai**: [**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) ativa plugins para as contas do claude.ai dos membros, e esses chegam ao Claude Code como [synced plugins](/docs/pt/plugins/loading#synced-plugins). Não define nenhuma das chaves nesta página
</Note>

As seções seguem a ordem que a maioria dos lançamentos segue: [exigir plugins](#pre-install-and-require-plugins) para todos ou por repositório, [seed containers e CI](#seed-containers-and-ci), [restringir](#restrict-what-users-can-install) o que os usuários podem adicionar por conta própria, [definir política de atualização](#set-update-policy), depois [auditar](#audit-and-review) o que está instalado. Para revisar cada chave de política em um único lugar, veja a [matriz de controle](#control-matrix).

<h2 id="pre-install-and-require-plugins">
  Pre-install and require plugins
</h2>

Um marketplace é um catálogo de plugins que o Claude Code busca de um repositório git, uma URL ou um caminho local. Depois de registrar um marketplace em uma máquina, o Claude Code pode instalar plugins dele.

Para instalar plugins para uma frota, defina duas chaves juntas em [managed settings](/docs/pt/managed-settings), o arquivo de política ou a política entregue pelo servidor que cada máquina da sua organização lê: `extraKnownMarketplaces` registra um marketplace em cada máquina, e `enabledPlugins` nomeia os plugins a instalar e ativar dele. [Choose a delivery mechanism](#choose-a-delivery-mechanism) cobre como as configurações gerenciadas chegam a cada máquina.

<h3 id="choose-a-delivery-mechanism">
  Choose a delivery mechanism
</h3>

As configurações gerenciadas chegam a uma máquina através de um de três mecanismos de entrega:

* **Server-managed settings**: defina as chaves de plugin como JSON em [**Organization settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code). Requer uma [função Owner](/docs/pt/server-managed-settings#access-control) na sua organização Claude. Uma sessão em nuvem busca essas configurações antes de instalar plugins.
* **MDM policies**: no macOS, entregue um plist cujas chaves de nível superior são as chaves de configurações. No Windows, armazene o documento JSON inteiro como uma string em um valor de registro. O domínio plist e a chave de registro estão em [Where each mechanism stores the policy](/docs/pt/managed-settings#where-each-mechanism-stores-the-policy).
* **Managed settings file**: coloque um `managed-settings.json` no caminho do sistema da plataforma. Você também pode adicionar arquivos ao diretório drop-in `managed-settings.d/` ao lado dele. Os caminhos de arquivo por plataforma estão em [Where each mechanism stores the policy](/docs/pt/managed-settings#where-each-mechanism-stores-the-policy), e as regras de mesclagem drop-in estão em [Split a file-based policy across teams](/docs/pt/managed-settings#split-a-file-based-policy-across-teams).

Use configurações gerenciadas pelo servidor se você tiver uma organização Claude for Teams ou Enterprise no claude.ai e seus dispositivos não estão todos sob MDM. Caso contrário, use uma política MDM ou o arquivo de configurações gerenciadas. Para a compensação, veja [Choose between server-managed and endpoint-managed settings](/docs/pt/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings).

<h4 id="which-managed-source-applies-on-a-machine">
  Which managed source applies on a machine
</h4>

Por padrão, apenas uma dessas três fontes se aplica em uma máquina. O Claude Code usa a primeira que entrega uma chave de política, verificando primeiro as configurações gerenciadas pelo servidor, depois as políticas MDM, depois o arquivo de configurações gerenciadas. Se as configurações gerenciadas pelo servidor entregarem até mesmo uma chave de política não relacionada, o Claude Code ignora as chaves de plugin em uma política MDM ou arquivo de configurações gerenciadas nessa máquina, exceto pelas [chaves que lê de cada fonte](/docs/pt/managed-settings#keys-read-from-every-admin-source).

Para aplicar cada fonte em vez disso, defina [`managedSourcesBehavior`](/docs/pt/managed-settings#compose-every-managed-source) como `"merge"`.

[How Claude Code combines managed sources](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) também lista as chaves que o Claude Code lê de cada fonte em ambos os modos.

<h3 id="require-a-marketplace-and-its-plugins">
  Require a marketplace and its plugins
</h3>

Adicione o marketplace sob `extraKnownMarketplaces`, com chave do próprio `name` do marketplace de seu `marketplace.json`. Depois adicione cada plugin sob `enabledPlugins` como `plugin-name@marketplace-name`. Cada entrada de marketplace carrega um objeto `source` com um campo `source` nomeando o tipo, como `github`. Este exemplo de configurações gerenciadas registra um marketplace de organização e força-ativa dois plugins dele:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" },
      "autoUpdate": true
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  }
}
```

Depois que as configurações chegam a uma máquina, o Claude Code registra o marketplace e instala os dois plugins no início da próxima sessão do usuário. Os usuários os veem em `/plugin`, e desativar um em seu próprio escopo não impede que ele carregue, porque as configurações gerenciadas têm precedência sobre cada outro escopo.

Para bloquear um plugin em cada escopo e ocultá-lo da listagem do marketplace, defina-o como `false` no `enabledPlugins` gerenciado em vez disso.

Ajuste os campos `autoUpdate` e `source` para seu marketplace:

* **`autoUpdate`**: `true` mantém o marketplace e seus plugins atualizando em segundo plano, e `false` desativa isso. Veja [Set update policy](#set-update-policy).
* **`source`**: `github` é um de vários tipos de fonte. Uma fonte `git` leva uma `url` para GitLab ou um host interno, e uma fonte `url` leva o endereço de um `marketplace.json` hospedado. Cada forma de fonte está na [marketplace reference](/docs/pt/plugins/marketplace-reference).

Se o marketplace é um repositório git privado, cada usuário precisa de acesso de leitura a ele. O clone de um marketplace baseado em git é executado com git na máquina do usuário, usando credenciais armazenadas e sem prompts. Para usuários sem contas de host git, use um [seed](#seed-containers-and-ci) em vez disso.

Uma entrada gerenciada também substitui uma entrada de marketplace com o mesmo nome ou cópia `--plugin-dir` de outra fonte:

* **Marketplaces**: uma entrada de marketplace gerenciada substitui uma entrada de precedência mais baixa com o mesmo nome, e os campos das duas entradas não se mesclam.
* **Cópias `--plugin-dir`**: `--plugin-dir` carrega um plugin de um diretório local para uma sessão. Para o que acontece quando o nome dessa cópia corresponde a um plugin que seu `enabledPlugins` gerenciado nomeia, veja [Name conflicts](/docs/pt/plugins/loading#name-conflicts).

O marketplace oficial da Anthropic `claude-plugins-official` não precisa de uma entrada `extraKnownMarketplaces` quando `enabledPlugins` define um de seus plugins como `true`. Essa entrada `name@claude-plugins-official` declara o marketplace por si só, onde quer que essas chaves se apliquem. Se você não ativar nenhum de seus plugins e ainda quiser que ele seja registrado em cada máquina, dê a ele uma entrada explícita, como [Allow the official marketplace and your own](#allow-the-official-marketplace-and-your-own) faz.

<h3 id="require-plugins-per-repository">
  Require plugins per repository
</h3>

Para cobrir os contribuidores de um repositório em vez de toda a sua frota, defina `extraKnownMarketplaces` e `enabledPlugins` no `.claude/settings.json` desse repositório. As entradas `extraKnownMarketplaces` se aplicam apenas em uma pasta que o contribuidor confiou, e em uma pasta não confiável o Claude Code as ignora sem uma mensagem:

* **Sessões interativas**: o Claude Code registra o marketplace apenas depois que o contribuidor aceita o [workspace trust dialog](/docs/pt/permissions#what-runs-before-you-trust-a-folder) para essa pasta.
* **[Execuções não-interativas `-p`](/docs/pt/headless)**: as entradas se aplicam apenas em uma pasta cuja confiança o usuário já aceitou interativamente, ou cuja flag `hasTrustDialogAccepted` você definiu em `~/.claude.json`.

Um plugin que o marketplace lista por um caminho relativo carrega da cópia do marketplace uma vez que as entradas `extraKnownMarketplaces` do repositório se apliquem. Um plugin cujo marketplace aponta para uma fonte externa em vez disso, como o próprio repositório GitHub do plugin, não instala apenas das configurações do repositório. Cada contribuidor vê `Plugin "<name>" is enabled in project settings but isn't installed` até executar `claude plugin install <name>@<marketplace> --scope project`, como [Install plugins](/docs/pt/plugins/install) descreve.

Se você usar uma fonte `directory` ou `file` local com um caminho relativo, o caminho se resolve contra o checkout principal do seu repositório. Quando você executa o Claude Code de um git worktree, o caminho ainda aponta para o checkout principal, então todos os worktrees compartilham o mesmo local do marketplace.

Para lançar um pacote de plugins com dependências, coloque o plugin do pacote em `enabledPlugins`, como [Plugin dependencies](/docs/pt/plugins/dependencies) descreve.

<h3 id="when-each-surface-applies-the-plugin-keys">
  When each surface applies the plugin keys
</h3>

A tabela mostra quando cada tipo de sessão do Claude Code aplica `extraKnownMarketplaces` e `enabledPlugins`, de configurações gerenciadas e do `.claude/settings.json` de um repositório. Para o aplicativo Desktop e as extensões IDE, veja [Install a plugin](/docs/pt/plugins/install#install-a-plugin).

| Surface               | Managed `extraKnownMarketplaces` and `enabledPlugins`                                                                                                                                                                                                                                                                               | Repository `.claude/settings.json`                                                           |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- |
| Terminal, interactive | Applied at session start on every machine that receives the settings                                                                                                                                                                                                                                                                | `extraKnownMarketplaces` applied after trust; `enabledPlugins` applied at session start      |
| `-p` and CI           | Applied at session start, with installs running in the background                                                                                                                                                                                                                                                                   | `extraKnownMarketplaces` in trusted folders only; `enabledPlugins` applied                   |
| Cloud sessions        | In an Anthropic-hosted environment, only server-managed settings reach the session, which waits for them before it installs plugins. MDM policies and managed settings files stay on the user's machine. For a self-hosted environment, see [Where and when a policy applies](/docs/pt/managed-settings#where-and-when-a-policy-applies) | See the **Cloud session** tab under [Install a plugin](/docs/pt/plugins/install#install-a-plugin) |

Em uma execução `-p` ou CI, marketplaces e plugins instalam em segundo plano, então um plugin pode estar faltando da primeira volta. Defina `CLAUDE_CODE_SYNC_PLUGIN_INSTALL=1` para fazer a execução esperar pela instalação antes de sua primeira consulta.

<h3 id="confirm-the-rollout">
  Confirm the rollout
</h3>

Verifique se o marketplace e os plugins chegaram em uma máquina ou em uma execução de CI:

* **Em uma máquina**: inicie o Claude Code e execute `/plugin`. O marketplace e os plugins estão listados.
* **Em CI**: execute `claude -p` com `--output-format stream-json --verbose`. O evento `init` lista os plugins carregados sob `plugins`.

<h2 id="seed-containers-and-ci">
  Seed containers and CI
</h2>

Para imagens de container e executores de CI que não podem clonar em tempo de execução, pré-popule um diretório de plugins no tempo de construção e aponte `CLAUDE_CODE_PLUGIN_SEED_DIR` para ele. O Claude Code registra os marketplaces do seed na inicialização e carrega caches de plugin do seed no local, sem clonar.

Um seed também serve usuários que não têm uma conta de host git.

<Note>
  Em ambientes CI/CD, configure um auxiliar de credencial git antes de instalar plugins de repositórios privados. No GitHub Actions, exporte um token com acesso de leitura ao repositório do marketplace como `GH_TOKEN`, depois execute `gh auth setup-git`. O token de fluxo de trabalho padrão pode acessar apenas o repositório do próprio fluxo de trabalho, então um marketplace privado em outro repositório precisa de um token de acesso pessoal ou token de aplicativo.
</Note>

<Steps>
  <Step title="Install into the seed at build time">
    Defina `CLAUDE_CODE_PLUGIN_CACHE_DIR` para o caminho do seed para que o marketplace e os plugins instalem lá em vez de `~/.claude/plugins`:

    ```bash theme={null}
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/your-marketplace
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install code-formatter@your-marketplace
    ```

    O seed tem o mesmo layout que `~/.claude/plugins`: `known_marketplaces.json`, `marketplaces/<name>/`, e `cache/<marketplace>/<plugin>/<version>/`. Você pode montar o seed em um caminho diferente de onde o construiu.
  </Step>

  <Step title="Point the runtime at the seed">
    Defina `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed` no ambiente do container. Para usar vários seeds, separe seus caminhos com `:` no Unix ou `;` no Windows. O Claude Code usa o primeiro seed que contém um determinado marketplace ou cache de plugin.
  </Step>

  <Step title="Enable the plugins">
    Os plugins em um seed não são ativados por conta própria. Defina `enabledPlugins` para cada plugin de seed que você quer carregado, em configurações gerenciadas ou no `.claude/settings.json` do repositório.
  </Step>
</Steps>

Para verificar um seed, execute `claude -p` com `--output-format stream-json --verbose` na imagem. Na lista `plugins` do evento `init`, o `path` de cada plugin carregado está sob o seed, como `/opt/claude-seed/cache/your-marketplace/code-formatter/1.0.0`.

Os marketplaces de seed seguem estas regras:

* **Read-only**: o Claude Code nunca escreve no seed e força `autoUpdate` desligado para marketplaces de seed.
* **Entradas de seed têm precedência**: em cada inicialização, um marketplace declarado no seed sobrescreve a entrada do usuário com o mesmo nome. Os usuários desativam um plugin de seed com `claude plugin disable`, não removendo o marketplace.
* **Atualizar e remover falham**: `claude plugin marketplace update <name>` e `remove` sem `--scope` em um marketplace de seed falham com uma mensagem que nomeia o diretório do seed.
* **A política ainda se aplica**: a [allowlist e blocklist](#restrict-what-users-can-install) verificam a fonte registrada de um marketplace de seed também. Permita a fonte de onde você construiu o seed.

Para frotas sem acesso git de saída, combine um seed com fontes de marketplace `directory` ou `file` em uma montagem compartilhada. Defina `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` também, que também desativa [plugin auto-update](/docs/pt/plugins/loading#when-auto-update-runs). Se um proxy estiver disponível, veja [Proxy configuration](/docs/pt/network-config#proxy-configuration) para as variáveis a definir.

<h2 id="restrict-what-users-can-install">
  Restrict what users can install
</h2>

A allowlist gerenciada `strictKnownMarketplaces` e a blocklist `blockedMarketplaces` decidem de quais fontes de marketplace os plugins podem vir. A fonte de um marketplace é o repositório git, URL ou caminho local que o Claude Code busca dele. Ambas as listas correspondem à fonte do marketplace de onde um plugin vem, não à entrada do próprio plugin dentro desse marketplace.

Para o lockdown comum, que permite o marketplace oficial e o seu, veja [Allow the official marketplace and your own](#allow-the-official-marketplace-and-your-own). Emparelhe-o com [`disableSideloadFlags`](#control-matrix) para que os usuários não possam carregar plugins de um diretório local ou URL também.

Ambas as listas se aplicam antes de qualquer coisa baixar e novamente no início da sessão:

* **Antes de um download**: as listas se aplicam quando um usuário adiciona um marketplace e em cada instalação, atualização, atualização e auto-atualização.
* **No início da sessão**: as listas se aplicam novamente aos plugins que já estão instalados, então um plugin instalado cuja fonte de marketplace não corresponde mais não carrega. `/plugin` o lista com `Marketplace "<name>" is not in the allowed marketplace list` ou `Marketplace "<name>" is blocked by enterprise policy`.

Onde as duas listas são aplicadas depende de onde você as define:

* **O console de administração do claude.ai**: o Claude Code aplica ambas as listas nas sessões que [leem configurações gerenciadas pelo servidor](/docs/pt/managed-settings#where-and-when-a-policy-applies). O claude.ai também as verifica quando qualquer pessoa na sua organização adiciona um novo marketplace de um repositório git no claude.ai, ou de **Customize** no aplicativo Claude Desktop fora de sua aba Code. Isso cobre um marketplace que um membro adiciona para sua própria conta e um adicionado para toda a organização sob [**Organization settings > Plugins**](https://claude.ai/admin-settings/plugins). O claude.ai recusa um repositório que a allowlist não admite ou que a blocklist nomeia. Ele não re-verifica um marketplace que foi adicionado em qualquer lugar antes de você definir as listas, e não verifica plugins carregados.
* **Um arquivo de configurações gerenciadas, política de nível do SO ou outra fonte gerenciada**: o Claude Code aplica ambas as listas onde lê essa fonte. O claude.ai não a lê.

Enquanto qualquer allowlist estiver definida, ou uma blocklist nomear qualquer fonte que não seja [`skills-dir`](#blocklist-with-blockedmarketplaces), um plugin cujo marketplace o Claude Code não consegue encontrar não carrega. `/plugin` mostra o erro de política para ele em vez de um erro de não encontrado. O caso comum é uma entrada `enabledPlugins` obsoleta para um marketplace que ninguém registrou.

<h3 id="control-matrix">
  Control matrix
</h3>

A tabela lista cada chave de política de plugin, o que ela aplica e o que não pode fazer.

| Key                                                                      | What it enforces                                                                                                                                                                                                                                                                           | What it can't do                                                                                                                                                                                                |
| :----------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `strictKnownMarketplaces`                                                | Allowlist de fontes de marketplace. `[]` bloqueia cada fonte, incluindo o marketplace oficial. Alias: `allowedMarketplaces`                                                                                                                                                                | Não registra um marketplace, restringe entradas dentro de um marketplace permitido, ou bloqueia `--plugin-dir`                                                                                                  |
| `blockedMarketplaces`                                                    | Blocklist de fontes de marketplace, verificada antes da allowlist                                                                                                                                                                                                                          | Não bloqueia um marketplace já registrado de uma fonte que não corresponde                                                                                                                                      |
| `syncClaudeAiPlugins`                                                    | Defina `false` para parar o Claude Code de baixar e carregar os plugins [sincronizados do claude.ai](/docs/pt/plugins/loading#synced-plugins) para a conta de cada usuário. Requer Claude Code v2.1.273 ou posterior                                                                            | Não desativa um plugin sincronizado. Para isso, defina `"<name>@synced": false` em [`enabledPlugins`](/docs/pt/settings-reference#enabledplugins)                                                                    |
| `enabledPlugins`                                                         | `true` força-ativa, `false` bloqueia em cada escopo e oculta o plugin                                                                                                                                                                                                                      | Não instala um plugin cujo marketplace não está registrado ou permitido                                                                                                                                         |
| `disableSideloadFlags`                                                   | Rejeita `--plugin-dir`, `--plugin-url`, `--agents`, a opção `plugins` do Agent SDK, e `--mcp-config` não-SDK na inicialização, e rejeita pastas nomeadas na variável [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/pt/env-vars#variables) da mesma forma                                                    | Não restringe `.mcp.json`, `claude mcp add`, ou servidores fornecidos por SDK. Emparelhe-o com [`allowedMcpServers`](/docs/pt/managed-mcp)                                                                           |
| `disableCommandPluginSources`                                            | Bloqueia plugins com uma fonte `command` de instalar, atualizar ou carregar. Uma fonte `command` é aquela cujo diretório de plugin é produzido executando um comando na máquina. Quando não definido, leva o valor de `allowManagedHooksOnly`                                              | Não afeta outros tipos de fonte                                                                                                                                                                                 |
| `allowManagedHooksOnly`                                                  | Restringe quais hooks executam. Veja [`allowManagedHooksOnly`](/docs/pt/settings-reference#allowmanagedhooksonly)                                                                                                                                                                               | Não confia em hooks de plugins que os usuários ativam por conta própria                                                                                                                                         |
| `strictPluginOnlyCustomization`                                          | Bloqueia skills, agents, hooks e servidores MCP que não vêm de um plugin, configurações gerenciadas ou built-ins do Claude Code. Defina `true` para cobrir todos os quatro tipos, ou um array de valores `skills`, `agents`, `hooks` e `mcp` como `["skills", "hooks"]` para cobrir alguns | Não restringe quais plugins os usuários instalam. Emparelhe-o com `strictKnownMarketplaces`                                                                                                                     |
| `pluginSuggestionMarketplaces`                                           | Marketplaces cujos plugins podem aparecer como sugestões de instalação. Veja [Recommend plugins](#recommend-plugins)                                                                                                                                                                       | Não afeta as dicas built-in                                                                                                                                                                                     |
| `pluginTrustMessage`                                                     | Anexa seu texto ao aviso de confiança que `/plugin` mostra antes de um plugin instalar                                                                                                                                                                                                     | Não muda o próprio texto do aviso                                                                                                                                                                               |
| `allowedChannelPlugins`                                                  | Substitui a lista padrão de plugins permitidos para enviar mensagens de canal. Requer `channelsEnabled: true`                                                                                                                                                                              | Veja [Restrict which channel plugins can run](/docs/pt/channels#restrict-which-channel-plugins-can-run)                                                                                                              |
| [`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL=1`](/docs/pt/env-vars) | Para sessões de terminal interativas de auto-registrar o marketplace oficial                                                                                                                                                                                                               | Não remove um marketplace já registrado. A allowlist e blocklist controlam o mesmo auto-registro sem ele. Uma máquina que começou uma vez com ele definido não retoma auto-registro depois que você o desdefine |

Cada chave na tabela é uma configuração gerenciada, exceto `enabledPlugins`, `syncClaudeAiPlugins` e `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`:

* **`enabledPlugins`**: você pode defini-lo em qualquer escopo, e as configurações gerenciadas o bloqueiam.
* **`syncClaudeAiPlugins`**: cada usuário também pode defini-lo em suas próprias configurações de usuário ou local. Veja seu [escopo na referência de configurações](/docs/pt/settings-reference#syncclaudeaiplugins).
* **`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`**: esta é uma variável de ambiente que você entrega através do bloco `env` gerenciado mostrado sob [Turn updates off for the whole fleet](#turn-updates-off-for-the-whole-fleet).

Cada chave de configurações aqui tem uma entrada na [referência de configurações](/docs/pt/settings-reference).

<h4 id="aliases-for-the-marketplace-keys">
  Aliases for the marketplace keys
</h4>

`strictKnownMarketplaces` também pode ser escrito `allowedMarketplaces`, e `extraKnownMarketplaces` também pode ser escrito `additionalMarketplaces`.

* **Version**: os aliases requerem Claude Code v2.1.232 ou posterior, e clientes mais antigos os ignoram. Em um arquivo que uma frota mista lê, mantenha os nomes canônicos.
* **Ambas as grafias definidas**: quando um arquivo define ambas as grafias, o valor da chave canônica se aplica.

<h3 id="allowlist-with-strictknownmarketplaces">
  Allowlist with `strictKnownMarketplaces`
</h3>

Defina a allowlist para uma lista desses objetos de fonte. A maioria das entradas corresponde exatamente, entradas `hostPattern` e `pathPattern` correspondem como expressões regulares, e wildcards de proprietário `github` correspondem por proprietário:

* **`github`**: `{ "source": "github", "repo": "your-org/approved-plugins" }`, com `ref` e `path` opcionais.
* **Wildcard de proprietário `github`**: `{ "source": "github", "repo": "your-org/*" }` corresponde a cada repositório sob esse proprietário. O `*` deve representar o nome inteiro do repositório. O Claude Code ignora entradas como `*/plugins` e `your-org/tools-*` como inválidas, então não correspondem a nada. Requer Claude Code v2.1.223 ou posterior.
* **`git`**: `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git" }`, com `ref` e `path` opcionais.
* **`url`**: `{ "source": "url", "url": "https://plugins.example.com/marketplace.json" }`, com `headers` opcionais.
* **`file` e `directory`**: `{ "source": "file", "path": "/opt/marketplace/marketplace.json" }` ou `{ "source": "directory", "path": "/opt/marketplace/plugins" }`, com caminhos absolutos.
* **`hostPattern`**: `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`, correspondido contra o host de fontes `github`, `git` e `url`. O padrão corresponde em qualquer lugar no nome do host, então ancorá-lo com `^` e `$` como mostrado para corresponder ao host inteiro. Uma fonte `github` sempre conta como `github.com`. Use uma entrada `hostPattern` para um GitHub Enterprise Server ou host GitLab onde desenvolvedores criam seus próprios marketplaces. A [página GHES](/docs/pt/github-enterprise-server#allowlist-ghes-marketplaces-in-managed-settings) tem o exemplo trabalhado.
* **`pathPattern`**: `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`, correspondido contra o `path` de fontes `file` e `directory`. O padrão corresponde em qualquer lugar no caminho, então comece-o com `^` para fixar um prefixo de diretório. `".*"` permite cada caminho local.
* **`skills-dir`**: `{ "source": "skills-dir" }` mantém [skills-directory plugins](#keep-skills-directory-plugins-loading) carregando enquanto uma allowlist está definida, e não corresponde a nenhum marketplace.

<h4 id="how-entries-match">
  How entries match
</h4>

Uma entrada `url` corresponde em seu valor `url`; `headers` não são comparados. Para entradas `github` e `git`, o `repo` ou `url`, o `ref` e o `path` devem todos corresponder, ou estar ausentes em ambos os lados:

* Uma entrada sem `ref` não cobre uma fonte com `ref: "main"`.
* Uma entrada para `your-org/your-marketplace` não cobre uma URL `git` que clona o mesmo repositório.
* Uma barra final, um sufixo `.git` ou `ssh://` no lugar de `https://` é um valor diferente. Quando um marketplace pode ser clonado por mais de uma URL, prefira uma entrada `hostPattern`.

Entradas de wildcard de proprietário seguem as regras exatas para `ref` e correspondem a qualquer `path` dentro do repositório a menos que a entrada fixe um. A correspondência de wildcard é sensível a maiúsculas e minúsculas na allowlist.

<h4 id="keep-skills-directory-plugins-loading">
  Keep skills-directory plugins loading
</h4>

Plugins de diretório de skills são os plugins que os usuários mantêm sob `~/.claude/skills/` ou `.claude/skills/` de um projeto em pastas que carregam um `.claude-plugin/plugin.json`. Se você definir qualquer allowlist sem uma entrada `{ "source": "skills-dir" }`, eles param de carregar. [Skills](/docs/pt/skills) simples, significando um `SKILL.md` sem esse manifesto, continuam carregando.

<h4 id="marketplaces-hosted-on-claude-ai">
  Marketplaces hosted on claude.ai
</h4>

A allowlist e blocklist correspondem a um [marketplace hospedado no claude.ai](/docs/pt/plugins/install#add-from-claude-ai) por seu host. Para permitir ou bloquear um, adicione uma entrada `hostPattern` que corresponda a `claude.ai` a `strictKnownMarketplaces` ou `blockedMarketplaces`. Na allowlist, tal entrada admite os marketplaces do claude.ai da sua organização e os marketplaces padrão do claude.ai, mas não um marketplace feito de uploads do próprio claude.ai de um membro ou um cujo escopo o claude.ai não declarou. Requer Claude Code v2.1.273 ou posterior.

<h4 id="lock-every-source-out">
  Lock every source out
</h4>

Uma allowlist vazia, `[]`, bloqueia cada fonte de marketplace, incluindo o marketplace oficial.

Este lockdown não cobre os plugins [sincronizados do claude.ai](/docs/pt/plugins/loading#synced-plugins), que o Claude Code baixa da conta de cada usuário em vez de um marketplace. Para parar aqueles também, defina [`syncClaudeAiPlugins`](/docs/pt/settings-reference#syncclaudeaiplugins) como `false` em configurações gerenciadas, ou desative Skills para sua organização no claude.ai.

<h3 id="blocklist-with-blockedmarketplaces">
  Blocklist with `blockedMarketplaces`
</h3>

`blockedMarketplaces` leva os mesmos objetos de fonte que [`strictKnownMarketplaces`](#allowlist-with-strictknownmarketplaces) e é verificado primeiro, então uma fonte em ambas as listas é bloqueada. A correspondência de blocklist é mais ampla que a correspondência de allowlist:

* URLs de Git são canonicalizadas, então as formas `git@` e `https://`, sufixos `.git` e barras finais de um repositório `github.com` todos correspondem à mesma entrada.
* Uma entrada `github` também bloqueia a URL `git` equivalente, e vice-versa.
* Para uma entrada `owner/*`, a comparação de proprietário é insensível a maiúsculas e minúsculas.
* Uma entrada sem `ref` ou `path` bloqueia cada ref e caminho dos repositórios que corresponde.

Esta entrada bloqueia cada repositório sob um proprietário GitHub:

```json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted-org/*" }
  ]
}
```

As entradas `url` em `blockedMarketplaces` também se aplicam quando um usuário adiciona uma URL de repositório `https://` que o Claude Code [clona em vez de buscar](/docs/pt/plugins/cli-reference#plugin-marketplace-add), como uma URL de repositório `github.com` ou `gitlab.com` simples. O usuário não pode adicionar essa URL se uma entrada a nomeia. A correspondência ignora o sufixo `.git` e qualquer ref que o usuário anexe após `#`. Requer Claude Code v2.1.232 ou posterior.

Uma entrada `{ "source": "skills-dir" }` aqui para [skills-directory plugins](#keep-skills-directory-plugins-loading) de carregar, de ambos `~/.claude/skills/` e `.claude/skills/` de um projeto.

Uma blocklist que nomeia apenas essa entrada não conta como uma restrição ativa, então não [para plugins cujo marketplace o Claude Code não consegue encontrar](#restrict-what-users-can-install) de carregar.

<h3 id="allow-the-official-marketplace-and-your-own">
  Allow the official marketplace and your own
</h3>

A maioria das organizações permite o marketplace oficial e o seu, e registra ambos para que cada máquina os tenha. Esta política de configurações gerenciadas permite ambos os marketplaces, registra ambos, força-ativa dois plugins e rejeita `--plugin-dir`:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" },
    { "source": "github", "repo": "your-org/*" },
    { "source": "skills-dir" }
  ],
  "extraKnownMarketplaces": {
    "claude-plugins-official": {
      "source": { "source": "github", "repo": "anthropics/claude-plugins-official" }
    },
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" }
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  },
  "disableSideloadFlags": true
}
```

Em uma máquina com esta política, adicionar qualquer fonte fora da lista, por exemplo `/plugin marketplace add https://example.com/other-marketplace.git`, falha com uma mensagem contendo `is blocked by enterprise policy` seguida pelas fontes permitidas. `claude --plugin-dir ./x` sai com uma mensagem nomeando `disableSideloadFlags`.

A entrada `{ "source": "skills-dir" }` mantém [skills-directory plugins](#keep-skills-directory-plugins-loading) carregando sob esta allowlist. Remova essa entrada e eles param de carregar.

Registre ambos os marketplaces com entradas `extraKnownMarketplaces` explícitas, como esta política faz, em vez de confiar na allowlist ou no marketplace oficial se registrando:

* **A allowlist não registra nada**: uma entrada `extraKnownMarketplaces` faz, e ela mesma deve passar na allowlist. O Claude Code recusa registrar um marketplace gerenciado cuja fonte a allowlist não corresponde.
* **O marketplace oficial se registra apenas em uma sessão de terminal interativa**: mesmo lá, ele se registra apenas quando a allowlist o permite. Uma execução `-p` ou um terminal anexado a uma sessão em nuvem nunca o registra.
* **Uma tentativa bloqueada é lembrada**: se uma máquina já executou sob uma política que bloqueou o marketplace oficial, o Claude Code registra a tentativa bloqueada e não tenta novamente depois que a política muda. Um lockdown `[]` é uma tal política. Essa máquina o registra novamente apenas através de uma entrada `extraKnownMarketplaces` como a nesta política, uma entrada `enabledPlugins` para um de seus plugins, ou um `/plugin marketplace add` manual.

<h2 id="set-update-policy">
  Set update policy
</h2>

Você pode definir a política de atualização por marketplace, para toda a frota ou por grupo de usuários através de canais de lançamento.

<h3 id="turn-auto-update-on-or-off-per-marketplace">
  Turn auto-update on or off per marketplace
</h3>

A auto-atualização de plugin é executada em segundo plano após a inicialização para marketplaces que a têm ativada. Para quais marketplaces a têm ativada por padrão, veja [When auto-update runs](/docs/pt/plugins/loading#when-auto-update-runs). Para decidir para a frota, defina `"autoUpdate": true` ou `false` em uma entrada `extraKnownMarketplaces` gerenciada:

* Se a entrada gerenciada define o campo, o Claude Code recusa o toggle `/plugin` do usuário com um erro que começa `Auto-update for '<name>' is set by`.
* Se a entrada gerenciada deixa o campo indefinido, o toggle do usuário persiste.

<h3 id="turn-updates-off-for-the-whole-fleet">
  Turn updates off for the whole fleet
</h3>

Para desativar a auto-atualização de plugin para cada marketplace, defina `DISABLE_AUTOUPDATER` no bloco `env` gerenciado, como este exemplo faz. A mesma variável também para as próprias atualizações do Claude Code:

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

Para parar as próprias atualizações do Claude Code mas manter a auto-atualização de plugin, adicione `"FORCE_AUTOUPDATE_PLUGINS": "1"` ao mesmo bloco. As outras [variáveis de ambiente que param a auto-atualização de plugin](/docs/pt/plugins/loading#when-auto-update-runs) funcionam da mesma forma.

`DISABLE_AUTOUPDATER` não cobre plugins com uma [fonte `command`](/docs/pt/plugins/marketplace-reference#command-plugin-source). O Claude Code re-executa o comando de cada um ativado a cada sessão e instala a saída quando mudou. Para o que para essas execuções, veja [When a command source re-runs](/docs/pt/plugins/loading#when-a-command-source-re-runs).

<h3 id="assign-release-channels-to-user-groups">
  Assign release channels to user groups
</h3>

Para executar canais estáveis e de acesso antecipado, hospede dois marketplaces que apontam para refs diferentes dos mesmos plugins. Depois dê a cada grupo de usuários seu próprio marketplace através de configurações gerenciadas por endpoint separadas ou uma política de gateway. As configurações gerenciadas pelo servidor do console de administração [se aplicam a cada usuário na sua organização](/docs/pt/server-managed-settings#current-limitations), então não podem atribuir configurações diferentes a grupos diferentes.

* Implante [configurações gerenciadas por endpoint](/docs/pt/managed-settings#delivery-mechanisms) separadas, como um arquivo de configurações gerenciadas ou um perfil MDM, para os dispositivos de cada grupo. Para verificar se o arquivo ou perfil por grupo se aplica em um dispositivo que também tem uma fonte de nível de organização, veja [How Claude Code combines managed sources](/docs/pt/managed-settings#precedence-within-the-managed-tier).
* Defina uma [política de gateway de aplicativos Claude](/docs/pt/claude-apps-gateway-config#managed) por grupo. O gateway aplica a primeira política cuja regra de correspondência se encaixa em um usuário, então ordene as políticas para que cada usuário chegue à política do seu grupo. O mapa `extraKnownMarketplaces` dessa política não se mescla com o de qualquer outra política, então liste cada marketplace que o grupo precisa nele, não apenas seu marketplace de canal.

Com qualquer mecanismo, o grupo estável recebe esta configuração:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": { "source": "github", "repo": "your-org/stable-tools" }
    }
  }
}
```

O grupo de acesso antecipado recebe `latest-tools` em vez disso. Para configurar os dois marketplaces, veja [Run release channels](/docs/pt/plugins/host-marketplace#run-release-channels).

<h2 id="recommend-plugins">
  Recommend plugins
</h2>

Os proprietários de marketplace podem anexar sinais `relevance` a entradas para que o Claude Code sugira o plugin quando um projeto corresponde.

As sugestões de um marketplace aparecem apenas quando ele está registrado na máquina do usuário, você lista seu nome em `pluginSuggestionMarketplaces` em configurações gerenciadas, e você declara sua fonte na mesma política. Declare a fonte como a entrada `extraKnownMarketplaces` do marketplace ou como uma entrada de allowlist. O marketplace oficial precisa apenas do nome. Veja [Enable suggestions in managed settings](/docs/pt/plugins/relevance#enable-suggestions-in-managed-settings).

<h2 id="audit-and-review">
  Audit and review
</h2>

Eventos OpenTelemetry e a API Analytics dizem o que sua frota instala e executa.

Para o que um plugin pode executar em uma máquina e o que cada nível de confiança permite, leia [Plugin security](/docs/pt/plugins/security) antes de aprovar um marketplace.

<h3 id="opentelemetry-events">
  OpenTelemetry events
</h3>

`claude_code.plugin_installed` registra cada instalação, e `claude_code.plugin_loaded` registra cada plugin ativado no início da sessão. Ambos os eventos reduzem ou omitem nomes de plugin e marketplace de terceiros a menos que você defina `OTEL_LOG_TOOL_DETAILS=1`, como [Redacted plugin names in your backend](/docs/pt/plugins/measure#redacted-plugin-names-in-your-backend) mostra. As listas de campos estão sob [Plugin installed event](/docs/pt/monitoring-usage#plugin-installed-event) e [Plugin loaded event](/docs/pt/monitoring-usage#plugin-loaded-event).

<h3 id="analytics-api">
  Analytics API
</h3>

No plano Enterprise, `GET /v1/organizations/analytics/plugins` retorna contagens de instalação e invocação por plugin, por dia em Claude Code e Cowork. Você pode agrupar as contagens por usuário ou grupo RBAC. A atividade de plugin que chega à Anthropic sem um nome de plugin aparece em uma linha `third-party` agregada. Veja a [referência de endpoint](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) e [Access data programmatically](/docs/pt/analytics#access-data-programmatically) para a chave que precisa.

<h2 id="plan-for-what-managed-settings-can’t-enforce">
  Plan for what managed settings can't enforce
</h2>

Estes pedidos de revisões de segurança não têm uma chave dedicada no esquema de configurações atual. Os controles existentes mais próximos são:

* **Direcionamento por usuário ou por grupo**: cada chave de plugin se aplica a cada usuário que recebe as configurações. As configurações gerenciadas pelo servidor entregam uma configuração por organização. Para política por grupo, use configurações gerenciadas por endpoint separadas ou políticas de gateway, como sob [Assign release channels to user groups](#assign-release-channels-to-user-groups).
* **Restringindo entradas dentro de um marketplace permitido**: a allowlist corresponde a fontes de marketplace. Para bloquear um plugin de um marketplace permitido, defina-o como `false` em `enabledPlugins` gerenciado.
* **Ocultando `/plugin`**: nenhuma chave desativa o comando. O equivalente mais próximo combina uma allowlist nomeando apenas seu marketplace, entradas `enabledPlugins` gerenciadas para os plugins que você fornece, e `disableSideloadFlags`.
* **Controlando `--plugin-dir` através da allowlist**: a allowlist não cobre `--plugin-dir`. `disableSideloadFlags` cobre.
* **Aplicando os toggles de plugin do claude.ai através dessas chaves**: [**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) não define as chaves nesta página. O que os membros e sua organização ativam lá chega ao CLI como [synced plugins](/docs/pt/plugins/loading#synced-plugins), que têm seus próprios controles.

<h2 id="troubleshoot-policy">
  Troubleshoot policy
</h2>

Se a política de plugin não se comportar como esperado em uma máquina, verifique primeiro estes sintomas:

* **O arquivo gerenciado não foi analisado**: quando um `managed-settings.json` não é JSON válido, o Claude Code recusa iniciar e imprime [um erro nomeando o arquivo](/docs/pt/errors#managed-settings-document-could-not-be-parsed). Um arquivo que analisa mas tem uma entrada inválida mantém o resto de sua política. Veja [Invalid entries in managed settings](/docs/pt/managed-settings#invalid-entries-in-managed-settings).
* **A fonte gerenciada não carregou**: execute `/status` e procure por `Enterprise managed settings` na linha `Setting sources`. Se estiver faltando, a fonte não carregou.
* **Um usuário relata `blocked by enterprise policy`**: a mensagem nomeia o marketplace ou sua fonte. Para uma allowlist, também lista as fontes permitidas. As entradas voltadas para o usuário estão em [Troubleshoot plugins](/docs/pt/plugins/troubleshooting).
* **Um plugin que o usuário desativou em `~/.claude/settings.json` ainda carrega**: outra fonte de configurações o re-ativou, como uma entrada `enabledPlugins` gerenciada que o força-ativa. `/plugin` e `claude plugin list` mostram `Disabled in ~/.claude/settings.json but still loads` com essa fonte de configurações.

<h2 id="next-steps">
  Next steps
</h2>

* [Marketplace reference](/docs/pt/plugins/marketplace-reference#marketplace-sources): os valores `source` que `extraKnownMarketplaces`, `strictKnownMarketplaces` e `blockedMarketplaces` aceitam
* [Host and maintain a marketplace](/docs/pt/plugins/host-marketplace): execute o marketplace que sua política aponta
* [Plugin security and trust](/docs/pt/plugins/security): o que um plugin pode fazer em uma máquina e como revisar um antes de instalar
* [Server-managed settings](/docs/pt/server-managed-settings): entregue essas chaves do console de administração do claude.ai
* [Troubleshoot plugins](/docs/pt/plugins/troubleshooting#blocked-by-your-organization): as mensagens que os usuários veem quando a política os bloqueia
