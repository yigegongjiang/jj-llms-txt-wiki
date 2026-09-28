> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Hospedar e manter um marketplace

> Publique um marketplace de plugins onde os usuários possam acessá-lo, conceda acesso a um privado e lance atualizações e renomeações sem quebrar as instalações.

Hospedar um marketplace significa colocar seu catálogo `marketplace.json` onde outras pessoas possam adicioná-lo com `/plugin marketplace add`, instalar seus plugins e continuar recebendo suas alterações após você fazer push.

Esta página é para a pessoa que opera um marketplace.

<Note>
  Estes casos são cobertos em outras páginas:

  * **Você ainda não escreveu o arquivo de catálogo**: comece com [Create a marketplace](/docs/pt/plugins/create-marketplace)
  * **Você é um administrador que exige, restringe ou pré-instala marketplaces nas máquinas da sua organização**: leia [Manage plugins for your organization](/docs/pt/plugins/org)
</Note>

Comece com [Host your marketplace](#host-your-marketplace) para escolher um host e o comando que seus usuários executam. Leia [Keep users up to date](#keep-users-up-to-date) antes de seu primeiro lançamento. Leia [Rename or remove a plugin](#rename-or-remove-a-plugin) antes de alterar o `name` de um plugin.

<h2 id="host-your-marketplace">
  Host your marketplace
</h2>

Você pode hospedar o marketplace no GitHub, em outro host git, como uma URL `marketplace.json` hospedada ou em um diretório em um sistema de arquivos compartilhado. Envie aos seus usuários o comando add para seu host e diga-lhes o que eles precisam em sua máquina:

| Host                                                          | Os usuários executam, em uma sessão Claude Code                        | O que os usuários precisam                                                                                                                |
| :------------------------------------------------------------ | :--------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| GitHub                                                        | `/plugin marketplace add your-org/your-marketplace`                    | `git`, e para um repositório privado o acesso descrito em [Grant access to a private marketplace](#grant-access-to-a-private-marketplace) |
| GitLab, Bitbucket, GitHub Enterprise Server ou outro host git | `/plugin marketplace add https://gitlab.example.com/team/plugins.git`  | `git`, e acesso ao host a partir de sua máquina. Envie a URL completa, porque o atalho `owner/repo` sempre significa github.com           |
| Uma URL `marketplace.json` hospedada                          | `/plugin marketplace add https://plugins.example.com/marketplace.json` | Acesso HTTPS à URL. Os usuários não precisam de `git` para o catálogo em si                                                               |
| Um diretório em um sistema de arquivos compartilhado          | `/plugin marketplace add /Volumes/shared/claude-plugins`               | Acesso de leitura ao caminho                                                                                                              |

Para fixar uma branch ou tag de um marketplace GitHub ou git-URL, diga aos usuários para anexar `#<ref>`, como em `your-org/your-marketplace#stable`. A [plugin commands reference](/docs/pt/plugins/cli-reference#plugin-marketplace-add) lista todas as formas que o comando aceita.

Um add bem-sucedido imprime `Successfully added marketplace: your-marketplace`. Claude Code pega esse nome do campo `name` em seu `marketplace.json`, não do nome do repositório.

Os usuários então instalam um plugin pelo `name` da entrada e pelo `name` do marketplace, como em `/plugin install code-formatter@your-marketplace`.

<h3 id="register-the-marketplace-for-everyone-in-a-repository">
  Register the marketplace for everyone in a repository
</h3>

Para compartilhar o marketplace com todos que trabalham em um repositório, execute `claude plugin marketplace add your-org/your-marketplace --scope project` lá uma vez a partir de seu shell e faça commit do `.claude/settings.json` que ele escreve. Claude Code então registra o marketplace para cada colega de trabalho que [trusts the folder](/docs/pt/plugins/org#require-plugins-per-repository).

<h3 id="avoid-relative-path-entries-in-a-url-hosted-marketplace">
  Avoid relative-path entries in a URL-hosted marketplace
</h3>

Quando os usuários adicionam seu marketplace como uma URL `marketplace.json` simples, Claude Code baixa apenas esse arquivo. Uma entrada em seu array `plugins` cujo `source` é um caminho relativo como `./plugins/formatter` então falha na instalação com [`its marketplace entry path does not stay inside the marketplace directory`](/docs/pt/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces). Dê a cada entrada um source que possa ser buscado por conta própria, como um repositório `github` ou uma URL `archive`, ou hospede o marketplace em um repositório git para que Claude Code clone a árvore inteira.

<h3 id="edit-plugins-in-place-on-a-shared-directory">
  Edit plugins in place on a shared directory
</h3>

Quando os usuários adicionam seu marketplace a partir de um diretório compartilhado, Claude Code lê plugins com sources de caminho relativo diretamente desse diretório em vez de copiá-los. Os usuários veem suas edições quando iniciam a próxima sessão ou executam `/reload-plugins`, sem uma etapa de atualização ou um bump de versão.

<h3 id="keep-plugin-files-out-of-git-lfs">
  Keep plugin files out of Git LFS
</h3>

Mantenha os arquivos que seus plugins precisam fora de [Git LFS](https://git-lfs.com). Quando os usuários adicionam um marketplace hospedado em um repositório git ou instalam um plugin baseado em git que ele lista, Claude Code clona esse marketplace ou repositório de plugin em sua máquina. O clone nunca baixa conteúdo LFS, então arquivos rastreados por LFS chegam como arquivos de ponteiro.

<h3 id="share-files-within-a-marketplace-with-symlinks">
  Share files within a marketplace with symlinks
</h3>

Para compartilhar arquivos entre seu plugin e outras partes do mesmo marketplace, crie links simbólicos dentro do diretório do seu plugin. Quando Claude Code copia o plugin em seu cache, ele lida com cada symlink por onde o alvo se resolve:

* **Dentro do próprio diretório do plugin**: o symlink é preservado como um symlink relativo no cache, para que continue resolvendo para o alvo copiado em tempo de execução.
* **Em outro lugar dentro do mesmo marketplace**: o symlink é desreferenciado. O conteúdo do alvo é copiado para o cache em seu lugar. Isso permite que o diretório `skills/` de um meta-plugin vincule a skills definidas por outros plugins no marketplace.
* **Fora do marketplace**: o symlink é ignorado por segurança.

Para plugins instalados a partir de um caminho local ou de um [`command` source](/docs/pt/plugins/marketplace-reference#command-plugin-source) cujo `mode` é o padrão `copy`, Claude Code preserva apenas symlinks que se resolvem dentro do próprio diretório do plugin e ignora todos os outros.

O comando a seguir cria um link de dentro de um plugin de marketplace para uma skill compartilhada definida por um plugin irmão. No Windows, use `mklink /D` a partir de um Prompt de Comando elevado ou ative o Modo de Desenvolvedor:

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

<h2 id="distribute-through-organization-settings">
  Distribute through organization settings
</h2>

Em um plano Team ou Enterprise, você também pode distribuir o marketplace através de [**Organization settings > Plugins & skills**](https://claude.ai/admin-settings/skills?tab=inventory) em claude.ai em vez de hospedá-lo em algum lugar onde os usuários o adicionem. Organization sync lê o repositório através da conexão GitHub ou GitLab da sua organização em claude.ai, então as credenciais git dos seus usuários não estão envolvidas.

Organization sync é mais rigoroso sobre o repositório do que `/plugin marketplace add` é:

* **Repositório de marketplace**: em github.com e gitlab.com, deve ser privado ou interno
* **Plugin sources**: cada plugin source deve ser do tipo `github`, `url` ou `git-subdir`, ou um [relative path](/docs/pt/plugins/marketplace-reference#relative-path-plugin-source) que comece com `./`
* **Diretório `bin/` de nível superior**: claude.ai rejeita um plugin que tem um e sincroniza o resto do marketplace. A mensagem de erro começa com `Plugin contains a top-level bin/ directory`. Mantenha executáveis em outro diretório, como `scripts/`, e referencie-os como `${CLAUDE_PLUGIN_ROOT}/scripts/<name>` a partir de seus hooks ou configurações de servidor MCP

Veja [Manage plugins for your organization](https://support.claude.com/en/articles/13837433) para o fluxo de trabalho do administrador.

<h2 id="grant-access-to-a-private-marketplace">
  Grant access to a private marketplace
</h2>

Quando um usuário adiciona, instala a partir de ou atualiza seu marketplace, Claude Code executa `git` em sua máquina com prompts interativos desativados e depende de quaisquer credenciais que essa máquina já tenha. Claude Code não tem seu próprio token git, e `marketplace.json` não tem campo para um.

Você escolhe se o clone é executado sobre SSH ou HTTPS pela forma do comando add que você envia aos usuários:

* **GitHub `owner/repo`**: Claude Code testa `ssh -T git@github.com` e clona sobre SSH quando o teste é bem-sucedido. Se o teste falhar ou o próprio clone SSH falhar, ele clona sobre HTTPS. Os usuários em máquinas sem uma chave SSH do GitHub podem definir `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1` para pular o teste e clonar sobre HTTPS.
* **`git@host:path.git`**: SSH.
* **`https://example.com/repo.git`**: HTTPS.

Diga aos usuários o que cada protocolo precisa em sua máquina:

* **SSH**: a chave deve funcionar sem um prompt de passphrase, por exemplo porque está carregada em `ssh-agent`. O host já deve estar em `known_hosts`.
* **HTTPS**: Claude Code deixa o helper de credencial git do usuário ativado, mas proíbe-o de solicitar. Uma credencial que o helper já armazena funciona; uma que ele teria que pedir falha. No GitHub, `gh auth login` seguido de `gh auth setup-git` armazena uma.

Para um host GitHub Enterprise Server, os usuários precisam de acesso git a esse host a partir de sua máquina. Veja [Plugin marketplaces on GHES](/docs/pt/github-enterprise-server#plugin-marketplaces-on-ghes) para o que cada superfície Claude Code precisa para alcançar um marketplace hospedado em GHES.

Se você distribuir através de **Organization settings > Plugins & skills** em claude.ai em vez disso, as credenciais git dos seus usuários não estão envolvidas. Veja [Distribute through organization settings](#distribute-through-organization-settings) para quais plugin sources podem ser privados lá.

<h3 id="serve-users-who-have-no-git-host-account">
  Serve users who have no git-host account
</h3>

Os usuários sem uma conta de host git podem adicionar um marketplace que você serve como uma URL `marketplace.json` ou a partir de um diretório compartilhado, mas podem instalar apenas os plugins cujas entradas sources eles também podem alcançar. Uma entrada que aponta para um repositório `github` privado ainda falha na instalação para eles, porque Claude Code a busca com o mesmo `git` não-interativo que usa para um marketplace hospedado em git.

Estas entry sources não precisam de conta git:

* **`archive`**: um zip baixado sobre HTTPS. Os usuários não precisam de `git` nem de uma conta, apenas acesso de rede à URL. Requer Claude Code v2.1.224 ou posterior. Fixe cada archive com `sha256` para que Claude Code recuse um download alterado. Para enviar credenciais com o download, veja [Authenticate archive downloads](#authenticate-archive-downloads).
* **Um repositório git público**: Claude Code clona um source `url` ou `git-subdir` público sobre HTTPS sem credenciais quando a entrada fornece uma URL `https://`. Para um source `github` ou um source `git-subdir` escrito como `owner/repo`, os usuários sem uma chave SSH do GitHub definem `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`.

Para uma equipe em uma rede, um marketplace `directory` em um sistema de arquivos compartilhado também funciona sem contas git. Os usuários precisam apenas de acesso de leitura ao caminho.

<h3 id="what-background-auto-update-does-with-credentials">
  What background auto-update does with credentials
</h3>

Background auto-update é a atualização desatendida de Claude Code de marketplaces e plugins instalados após uma sessão iniciar. Está desativado para seu marketplace até que um usuário ou administrador o ative, conforme coberto em [Keep users up to date](#keep-users-up-to-date).

Quando está ativado para um marketplace privado, a verificação de fundo de novos commits usa os helpers de credencial git configurados do usuário e nunca solicita. Cada tipo de remoto e helper fornece um resultado diferente:

* **Remotes SSH**: uma chave carregada em `ssh-agent` autentica a verificação.
* **Remotes HTTPS com uma credencial armazenada**: um helper que pode fornecer uma credencial armazenada sem solicitar autentica a verificação. Git Credential Manager, o helper Keychain do macOS e `git-credential-store` funcionam dessa forma uma vez que mantêm uma credencial para o host.
* **Remotes HTTPS com um helper que precisa solicitar**: o helper não pode responder em segundo plano. A atualização falha silenciosamente e o checkout existente permanece no lugar, para que os plugins do usuário continuem funcionando a partir do último estado sincronizado.

Após a verificação, Claude Code faz um dos seguintes:

* **O checkout está atualizado**: Claude Code o deixa como está.
* **A verificação encontra novos commits ou falha porque não consegue alcançar ou autenticar para o remoto**: Claude Code clona o marketplace novamente e substitui o checkout existente pelo novo clone. Se esse clone falhar, o checkout existente permanece no lugar. O re-clone pode [time out on large repositories](/docs/pt/plugins/troubleshooting#git-clone-timed-out-after-120s).

Para manter um marketplace privado atual, um usuário pode fazer um dos seguintes:

* **Armazenar uma credencial**: faça login no helper de credencial primeiro para que ele mantenha uma credencial para o host. Para GitHub, execute `gh auth login`, depois `gh auth setup-git`.
* **Manter o checkout em falha**: se o usuário definir `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1`, Claude Code mantém o checkout existente sem tentar o re-clone quando a verificação de fundo não consegue alcançar ou autenticar para o remoto. Os plugins continuam funcionando a partir do último estado sincronizado.

Se um usuário definir `GITHUB_TOKEN` ou outro token de provedor no ambiente, isso sozinho não autentica a verificação de fundo. Um token entra em vigor através de um helper de credencial, como o helper da CLI `gh`, que lê `GH_TOKEN` e `GITHUB_TOKEN`.

<h2 id="roll-out-to-a-whole-company">
  Roll out to a whole company
</h2>

Lançar um plugin para uma empresa envolve você como proprietário do marketplace, um administrador que controla configurações gerenciadas e cada pessoa que usa Claude Code. Você pode executar o lançamento sem o administrador, caso em que cada pessoa adiciona o marketplace e instala o plugin por conta própria.

| Quem                                | O que eles fazem                                                                                                                                                               | Onde está coberto                                                                                                                                       |
| :---------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Você, o proprietário do marketplace | Mantenha o catálogo em um repositório que apenas a empresa pode ler, envie o comando add para seu host e diga o que cada pessoa precisa em sua máquina                         | [Host your marketplace](#host-your-marketplace) e [Grant access to a private marketplace](#grant-access-to-a-private-marketplace)                       |
| Um administrador                    | Registra o marketplace e ativa seus plugins para todos com `extraKnownMarketplaces` e `enabledPlugins` em configurações gerenciadas, e define `autoUpdate` lá                  | [Require a marketplace and its plugins](/docs/pt/plugins/org#require-a-marketplace-and-its-plugins) e [Set update policy](/docs/pt/plugins/org#set-update-policy) |
| Cada pessoa                         | Precisa de acesso de leitura a um repositório git privado, com credenciais já armazenadas em sua máquina. Sem um administrador, eles também executam os comandos add e install | [Add a private marketplace](/docs/pt/plugins/install#add-a-private-marketplace)                                                                              |

Para pessoas que não têm conta de host git, estas seções cobrem uma forma cada de alcançá-las:

* **Entry sources que não precisam de conta git**: [Serve users who have no git-host account](#serve-users-who-have-no-git-host-account)
* **Um diretório de plugins pré-preenchido**: [Seed containers and CI](/docs/pt/plugins/org#seed-containers-and-ci), que também serve usuários que não têm conta de host git
* **Configurações de organização claude.ai**: [Distribute through organization settings](#distribute-through-organization-settings), onde as credenciais git dos seus usuários não estão envolvidas

<h2 id="keep-users-up-to-date">
  Keep users up to date
</h2>

Suas alterações chegam aos usuários através de background auto-update, uma vez que está ativado para seu marketplace, ou quando os usuários atualizam o plugin por conta própria. Em ambos os casos, um usuário obtém uma nova cópia de um plugin apenas quando sua versão computada muda, conforme descrito em [Release a new version](#release-a-new-version).

<h3 id="turn-on-auto-update">
  Turn on auto-update
</h3>

Background auto-update está desativado para seu marketplace por padrão, e `marketplace.json` não tem campo para ativá-lo. Um usuário ou um administrador o ativa:

* **Diga aos usuários para ativá-lo**: cada usuário vai para **Marketplaces** em `/plugin`, seleciona seu marketplace e seleciona **Enable auto-update**.
* **Peça a um administrador para defini-lo**: se um administrador definir `"autoUpdate": true` na entrada `extraKnownMarketplaces` do seu marketplace em configurações gerenciadas, está ativado para todos que recebem essas configurações. Veja [Set update policy](/docs/pt/plugins/org#set-update-policy).

Sem auto-update, os usuários recebem suas alterações quando executam `/plugin marketplace update <name>` em uma sessão ou `claude plugin update <plugin>@<name>` no shell.

Para o que os usuários veem quando uma atualização os alcança, veja [When auto-update runs](/docs/pt/plugins/loading#when-auto-update-runs).

<h3 id="release-a-new-version">
  Release a new version
</h3>

Para lançar uma nova versão aos usuários, altere o `version` do plugin. Os usuários obtêm uma nova cópia apenas quando a versão computada do plugin difere da que eles têm. Essa versão vem de `plugin.json` primeiro, depois da entrada do marketplace, por [Versions and updates](/docs/pt/plugins/loading#versions-and-updates).

Um plugin que os usuários [load in place](/docs/pt/plugins/loading#find-plugins-on-disk) a partir de um marketplace que adicionaram como um diretório local não é controlado por `version`. Ele carrega seus arquivos atuais em cada início de sessão, seja qual for sua string de versão.

Para cada instalação que não seja um carregamento in-place ou um de um source `command`, aumente `version` em cada lançamento ou omita-o:

* **Bump `version` em cada lançamento**: os usuários permanecem em sua cópia em cache até a string mudar. Se você definir `"version": "1.0.0"` e fazer push de novos commits sem alterá-lo, os usuários não os recebem.
* **Omita `version`**: os usuários rastreiam seus commits em vez disso. Deixe `version` fora de `plugin.json` e da entrada do marketplace.

Não defina `version` em `plugin.json` e na entrada do marketplace. Se você fizer, Claude Code usa o valor `plugin.json` sem aviso, e `claude plugin validate` relata a incompatibilidade como `Entry declares version "<a>" but <path>/plugin.json says "<b>"`.

<h3 id="hold-users-on-one-version">
  Hold users on one version
</h3>

Um marketplace serve uma versão de cada plugin por vez, então você mantém os usuários em uma versão escolhendo o que cada entrada aponta:

* **`ref` e `sha` na entrada do plugin**: `ref` nomeia uma branch ou tag e `sha` nomeia um commit para um source `github`, `url` ou `git-subdir`. Veja [Plugin sources](/docs/pt/plugins/marketplace-reference#plugin-sources).
* **`#<ref>` no comando add**: os usuários que adicionam `your-org/your-marketplace#stable` obtêm essa branch ou tag do catálogo. Para duas linhas de lançamento ao mesmo tempo, veja [Run release channels](#run-release-channels).
* **Tags `<plugin>--v<version>`**: um intervalo de versão de uma dependência se resolve contra essas tags. Veja [Release a plugin that others depend on](/docs/pt/plugins/dependencies#tag-plugin-releases-for-version-resolution).

[Release a new version](#release-a-new-version) diz quando uma entrada alterada alcança os usuários.

<h3 id="change-the-command-of-a-command-source">
  Change the command of a command source
</h3>

Se você alterar o `command` de um [`command` source](/docs/pt/plugins/marketplace-reference#command-plugin-source) ou alternar seu `mode`, cada usuário tem que aceitar o novo comando antes de Claude Code executá-lo. Claude Code executa apenas o comando exato que um usuário aceitou quando instalou ou atualizou pela última vez o plugin.

Depois que a cópia do marketplace de um usuário pega a alteração, esse usuário vê o seguinte:

* **Sem mais execuções de fundo**: a [once-per-session run](/docs/pt/plugins/loading#when-a-command-source-re-runs) do comando para para esse usuário, então a nova saída da ferramenta não os alcança.
* **Uma entrada na aba Errors do `/plugin`**: a entrada mostra o novo comando e o comando `claude plugin update` para executar.

Diga aos usuários para executar o comando `claude plugin update` que essa entrada mostra, em um terminal. Claude Code mostra a eles o novo comando e pede que o aceitem.

<h2 id="run-release-channels">
  Run release channels
</h2>

Para oferecer faixas estáveis e de acesso antecipado, hospede dois marketplaces cujas entradas apontam para diferentes refs do mesmo plugin e deixe cada usuário adicionar o que quiser. Claude Code não tem conceito de canal de lançamento, e um marketplace serve uma versão de cada plugin por vez.

Dê aos dois arquivos `marketplace.json` valores `name` diferentes. Claude Code identifica um marketplace por seu `name`, então um usuário não pode ter dois marketplaces com o mesmo nome registrados ao mesmo tempo.

Com estes dois catálogos, os usuários que adicionam `stable-tools` instalam `code-formatter` a partir da branch `stable`, e os usuários que adicionam `latest-tools` instalam a partir de `latest`:

```json theme={null}
{
  "name": "stable-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "stable" } }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "latest" } }
  ]
}
```

Dê aos dois refs versões `plugin.json` diferentes ou omita `version` para que o commit SHA os distinga. As atualizações são detectadas comparando versões, então uma ref que se move sem uma mudança de versão deixa os usuários na cópia em cache.

Para atribuir os canais a grupos de usuários em vez de deixar os usuários escolherem, um administrador dá a cada grupo a entrada `extraKnownMarketplaces` correspondente, conforme descrito em [Set update policy](/docs/pt/plugins/org#set-update-policy).

<h2 id="rename-or-remove-a-plugin">
  Rename or remove a plugin
</h2>

O `name` de um plugin é seu identificador. Os usuários o referenciam nas chaves de configurações `enabledPlugins` e `pluginConfigs` e em `/plugin install`, então alterá-lo quebra cada instalação existente.

Para alterar o rótulo que os usuários veem em `/plugin` sem quebrar nada, defina `displayName` em `plugin.json` e mantenha `name` inalterado.

<h3 id="migrate-users-with-a-renames-map">
  Migrate users with a renames map
</h3>

Quando você deve alterar um `name`, adicione um mapa `renames` de nível superior a `marketplace.json` para que Claude Code migre usuários existentes em vez de relatar [`Plugin "<name>" not found in marketplace`](/docs/pt/plugins/troubleshooting#plugin-not-found-in-marketplace). Faça o mesmo quando remover uma entrada de `plugins`. A migração automática requer Claude Code v2.1.193 ou posterior.

Mapeie cada nome anterior para seu nome atual ou para `null` quando o plugin se foi. Este marketplace renomeia `formatter` para `code-formatter` e registra que `legacy-linter` foi removido:

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

Depois que você faz push, um usuário que ainda tem o nome antigo ativado vê um destes resultados:

* **Entrada renomeada**: o plugin carrega sob seu novo nome. `claude plugin list` e os detalhes do plugin em `/plugin` mostram `Renamed to "code-formatter" in the "your-marketplace" marketplace` uma vez, e Claude Code reescreve a chave antiga para a nova em `enabledPlugins` e `pluginConfigs` nos escopos de configurações do usuário, projeto e local.
* **Entrada `null`**: a chave antiga é removida desses escopos e o usuário vê `Removed from the "your-marketplace" marketplace`.
* **Ativado em configurações gerenciadas**: o plugin ainda carrega sob seu novo nome, mas Claude Code não pode reescrever configurações gerenciadas, então o aviso recorre até que um administrador atualize `enabledPlugins` lá.

Para um marketplace que os usuários adicionaram a partir de um repositório git ou URL, um plugin renomeado relata [`Plugin "<name>" not cached at <path>`](/docs/pt/plugins/troubleshooting#plugin-not-cached-at) até que o usuário execute `/plugin install code-formatter@your-marketplace` uma vez em uma sessão.

Trate `renames` como histórico apenas de acréscimo. Mantenha entradas antigas depois que todos migrarem. Quando renomear novamente, adicione uma segunda entrada em vez de editar a primeira, porque Claude Code segue a cadeia a partir do nome mais antigo.

Em seu shell, execute `claude plugin validate .` após editar o mapa. Ele rejeita uma cadeia que cicla ou que termina em qualquer lugar que não seja `null` ou um nome em `plugins`, com `renames.<name>: chain does not resolve`.

<h3 id="uninstall-removed-plugins-from-users’-machines">
  Uninstall removed plugins from users' machines
</h3>

Para desinstalar um plugin removido das máquinas dos usuários em vez de deixar uma cópia para trás, defina `"forceRemoveDeletedPlugins": true` no nível superior de `marketplace.json`. Sem o campo, um plugin removido permanece instalado e relata `Plugin "<name>" not found in marketplace` quando uma sessão o carrega. Com ele, Claude Code faz o seguinte em cada início de sessão:

1. Compara o que os usuários instalaram a partir de seu marketplace contra as entradas e o mapa `renames`, e trata qualquer plugin que não esteja listado nem renomeado como removido.
2. Desinstala cada plugin removido do usuário, projeto e escopos locais. Os plugins que apenas configurações gerenciadas instalaram permanecem no lugar.
3. Lista cada plugin removido em um cabeçalho **Flagged** em `/plugin` com o status `Removed from marketplace`.

<h2 id="authenticate-archive-downloads">
  Authenticate archive downloads
</h2>

Para autenticar um download [`archive`](/docs/pt/plugins/marketplace-reference#archive-plugin-source), como um download de um registro privado, defina os cabeçalhos HTTP que Claude Code envia com ele. Você pode definir `headers` em um destes lugares:

* **O source `url` do marketplace**: o source `url` que você registrou o marketplace a partir de, como uma entrada [`extraKnownMarketplaces`](/docs/pt/settings-reference#extraknownmarketplaces).
* **A entrada do plugin**: em Claude Code v2.1.238 ou posterior, você pode defini-lo na entrada `marketplace.json` do plugin em vez disso, ao lado de `source`.

Em um destes lugares, defina um comando `headersHelper` em vez de `headers` quando o valor é de curta duração, como um token que seu registro gera sob demanda. Claude Code executa o comando e envia o objeto JSON que ele imprime como os headers desse lugar. Requer Claude Code v2.1.238 ou posterior.

A [marketplace reference](/docs/pt/plugins/marketplace-reference#plugin-entries) lista os campos de entrada `headers` e `headersHelper`.

O lugar que você escolhe decide quais downloads obtêm os headers e quando Claude Code executa o comando:

| Lugar                       | Downloads que obtêm os headers                                                                  | Quando Claude Code executa um `headersHelper` definido lá                                                                                                                    |
| :-------------------------- | :---------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Source `url` do marketplace | Downloads de archive na origem da URL do marketplace, significando o mesmo scheme, host e porta | Antes de cada busca do `marketplace.json` do marketplace e antes de cada download de archive nessa origem. Claude Code reutiliza a saída de uma execução por até 60 segundos |
| Entrada do plugin           | Apenas o download dessa entrada                                                                 | Apenas quando um usuário instala ou atualiza apenas esse plugin e [aceita o comando](#how-users-accept-a-headershelper-command)                                              |

Onde ambos os lugares definem um header do mesmo nome, Claude Code envia o valor da entrada. Dentro de um lugar, um header que o comando imprime substitui um header do mesmo nome listado em `headers`.

<h3 id="add-a-headershelper-to-a-plugin-entry">
  Add a headersHelper to a plugin entry
</h3>

Esta entrada define `headersHelper` ao lado de `source`. Ela também define [`"strict": false`](/docs/pt/plugins/marketplace-reference#strict-mode), que Claude Code exige de uma entrada `marketplace.json` que define `headersHelper`:

```json theme={null}
{
  "name": "my-plugin",
  "description": "Formatting commands for internal services",
  "strict": false,
  "source": {
    "source": "archive",
    "url": "https://registry.example.com/plugins/my-plugin-2.1.0.zip"
  },
  "headersHelper": "/opt/bin/mint-registry-token.sh"
}
```

Para verificar a entrada, execute `claude plugin install my-plugin@your-marketplace` em seu shell. Claude Code mostra a você o comando e a URL do archive, e baixa o zip depois que você aceita.

<h3 id="write-the-headershelper-command">
  Write the headersHelper command
</h3>

Se você define `headersHelper` em um source `url` de um marketplace ou em uma entrada de plugin, escreva o comando para atender a estes requisitos:

* **Texto do comando**: no máximo 500 caracteres de ASCII imprimível, sem uma sequência de quatro ou mais espaços.
* **Saída**: imprima um objeto JSON de nomes de headers e valores de string em stdout, depois saia com 0 dentro de 10 segundos.
* **Shell e diretório de trabalho**: Claude Code executa o comando através de `sh` ou através de `cmd.exe` no Windows. O diretório de trabalho é o diretório de configuração, que é `~/.claude` ou [`CLAUDE_CONFIG_DIR`](/docs/pt/env-vars#variables). Dê um caminho absoluto ou um comando em `PATH`, porque um caminho relativo se resolve contra esse diretório, não o projeto do usuário.
* **Variáveis que Claude Code remove**: quando o comando é definido em uma entrada `marketplace.json` ou no `.claude/settings.json` ou `.claude/settings.local.json` de um projeto, Claude Code remove do ambiente cada variável cujo nome parece uma credencial, pela [mesma regra que aplica a um `headersHelper` MCP](/docs/pt/mcp#which-variables-a-helper-can-read). `ANTHROPIC_API_KEY` e `MY_REGISTRY_TOKEN` são ambos removidos, então tenha o comando ler sua credencial de um arquivo ou um armazenamento de credenciais. Esta remoção não se aplica a um comando definido em configurações de usuário, um arquivo `--settings` ou configurações gerenciadas.
* **Variáveis que Claude Code define**: `CLAUDE_CODE_MARKETPLACE_URL` e `CLAUDE_CODE_MARKETPLACE_NAME` para o comando de um source `url`, e `CLAUDE_CODE_PLUGIN_NAME` e `CLAUDE_CODE_PLUGIN_ARCHIVE_URL` para o comando de uma entrada. `CLAUDE_CODE_MARKETPLACE_NAME` não está definido na primeira busca depois que um usuário adiciona um marketplace por URL, porque essa busca é o que fornece o nome.

Um comando que cria um token bearer imprime um objeto como este:

```json theme={null}
{"Authorization": "Bearer eyJhbGciOiJSUzI1NiJ9"}
```

<h3 id="when-claude-code-skips-a-headershelper-command-or-drops-its-output">
  When Claude Code skips a headersHelper command or drops its output
</h3>

Um comando `headersHelper` não é executado ou headers de `headers` ou da saída do comando são descartados quando um dos seguintes se aplica:

* **Comando falha**: se o comando sair com não-zero, executar por mais de 10 segundos ou imprimir qualquer coisa que não seja um objeto JSON de valores de string, a busca ou download para o qual o comando foi executado não acontece.
* **URL do marketplace não começa com `https://`**: o comando desse source `url` não é executado e as solicitações carregam apenas os headers listados em seu campo `headers`.
* **Redirecionamento deixa a origem**: quando um download é redirecionado para fora da origem da URL do archive, a solicitação redirecionada não carrega valores de `headers` de nenhum source `url` do marketplace ou entrada de plugin.
* **Entrada define um header de roteamento ou identidade**: Claude Code descarta nomes de roteamento de solicitação e identidade de cliente como `Host`, `Cookie` e `X-Forwarded-*` de `headers` de uma entrada e saída de comando, e mantém nomes de autenticação como `Authorization`. Cada entrada `marketplace.json` é filtrada dessa forma. Para uma entrada de plugin inline em configurações, veja [`extraKnownMarketplaces`](/docs/pt/settings-reference#extraknownmarketplaces).
* **Comando definido em configurações de um diretório `--add-dir`**: o comando é ignorado, em um source `url` e em uma [entrada de plugin inline](/docs/pt/settings-reference#extraknownmarketplaces) igualmente, e apenas os `headers` desse arquivo são enviados.
* **Configurações gerenciadas bloqueiam o comando**: definir [`disableCommandPluginSources`](/docs/pt/settings-reference#disablecommandpluginsources) para `true` bloqueia comandos `headersHelper`, e [`allowManagedHooksOnly`](/docs/pt/settings-reference#allowmanagedhooksonly) também os bloqueia a menos que `disableCommandPluginSources` seja explicitamente `false`. Sob um desses bloqueios, Claude Code ainda executa o comando para um marketplace que as próprias configurações gerenciadas declaram.

<h3 id="how-users-accept-a-headershelper-command">
  How users accept a headersHelper command
</h3>

Um usuário aceita o comando de uma entrada de plugin cada vez que instala ou atualiza apenas esse plugin. Eles fazem isso a partir da própria visualização do plugin em `/plugin` ou com `claude plugin install` ou `claude plugin update`. Claude Code mostra o comando e a URL do archive, e executa o comando apenas depois que o usuário aceita.

Em um shell não-interativo, passe [`--yes`](/docs/pt/plugins/cli-reference#plugin-install) para aceitar o comando. Para aceitar apenas o comando que uma execução anterior `--json` exibiu, passe [`--accept-command`](/docs/pt/plugins/cli-reference#plugin-install) com o `sha256` que a execução relatou.

Claude Code executa apenas o comando que mostrou, para a URL do archive que mostrou. Se o comando da entrada ou a URL do archive mudaram no meio, Claude Code recusa a instalação ou atualização. Uma mudança na string de consulta sozinha não conta.

<h3 id="installs-and-updates-that-refuse-the-command-instead-of-asking">
  Installs and updates that refuse a command instead of asking
</h3>

Em qualquer operação que não seja uma instalação ou atualização de um único plugin, Claude Code não executa o comando de uma entrada nem baixa seu archive. O plugin permanece em sua versão instalada ou permanece desinstalado, e o usuário vê um destes resultados:

* **Instalando vários plugins ao mesmo tempo, a partir de uma sugestão de plugin ou como dependência de outro plugin**: Claude Code recusa o plugin que tem o comando e direciona o usuário para a própria visualização desse plugin em `/plugin`. Os outros plugins em uma instalação em massa ainda instalam. Um plugin que depende do plugin recusado falha em instalar até que o usuário instale o plugin recusado por conta própria.
* **Background auto-update ou início de sessão para um plugin cujo archive nunca foi baixado**: Claude Code lista o plugin na aba Errors do `/plugin` para que o usuário saiba instalá-lo ou atualizá-lo por conta própria.

<h3 id="when-a-marketplace-url-sources-command-runs">
  When a marketplace `url` source's command runs
</h3>

Você declara o `headersHelper` de um source `url` do marketplace em um arquivo de configurações, como uma entrada [`extraKnownMarketplaces`](/docs/pt/settings-reference#extraknownmarketplaces), em vez de no catálogo que o marketplace publica. Claude Code portanto não pede ao usuário para aceitá-lo em cada instalação ou atualização. Em vez disso, o arquivo de configurações que o declara decide quando Claude Code o executa:

| Arquivo de configurações                                                                                | Quando Claude Code executa o comando                                                                                                                                                                                                   |
| :------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Configurações de usuário, um arquivo `--settings` ou um arquivo de configurações gerenciadas na máquina | Sem pedir, incluindo durante uma atualização de marketplace de fundo                                                                                                                                                                   |
| `.claude/settings.json` ou `.claude/settings.local.json` de um projeto                                  | Apenas depois que o usuário aceita o [workspace trust dialog](/docs/pt/permissions#what-runs-before-you-trust-a-folder) para essa pasta em si. Uma sessão `-p` ou SDK não conta como aceitá-lo, e nem a confiança concedida a uma pasta pai |
| Configurações gerenciadas pelo servidor                                                                 | Em uma sessão interativa, apenas depois que o usuário aprova as configurações entregues no [security approval dialog](/docs/pt/server-managed-settings#security-approval-dialogs)                                                           |

Para uma [entrada de plugin inline](/docs/pt/settings-reference#extraknownmarketplaces) em um desses arquivos, Claude Code exige a mesma confiança de pasta ou aprovação de configurações que para um comando de nível de marketplace nesse arquivo, e o usuário também aceita o comando da entrada em cada instalação ou atualização.

<h2 id="depend-on-and-recommend-other-plugins">
  Depend on and recommend other plugins
</h2>

Uma entrada pode declarar dependências em outros plugins.

* **Intervalos de versão**: uma dependência pode carregar um intervalo semver.
* **Dependências entre marketplaces**: uma dependência de outro marketplace instala apenas quando seu marketplace lista esse marketplace em `allowCrossMarketplaceDependenciesOn`.

Para intervalos de versão, a convenção de tag git `<plugin>--v<version>` que eles se resolvem contra e confiança entre marketplaces, veja [Plugin dependencies](/docs/pt/plugins/dependencies).

Para ter Claude Code sugerir um plugin quando um projeto o corresponde, adicione um bloco `relevance` à entrada com os sinais que identificam o projeto. Os usuários veem sugestões do seu marketplace apenas quando um administrador o lista em `pluginSuggestionMarketplaces`. Para os sinais e a etapa de habilitação, veja [Plugin relevance](/docs/pt/plugins/relevance).

<h2 id="work-around-what-a-marketplace-can’t-do">
  Work around what a marketplace can't do
</h2>

Algumas coisas que proprietários pedem não têm campo em `marketplace.json`. Aqui está a opção mais próxima para cada:

* **Restringir o que mais os usuários instalam**: a lista de permissões do marketplace é uma configuração gerenciada, `strictKnownMarketplaces`. Veja [Restrict what users can install](/docs/pt/plugins/org#restrict-what-users-can-install).
* **Instalar ou ativar um plugin sem o usuário pedir**: nenhum campo de entrada instala um plugin. `enabledPlugins` gerenciado faz isso para uma frota; veja [Pre-install and require plugins](/docs/pt/plugins/org#pre-install-and-require-plugins).
* **Mostrar entradas diferentes para usuários diferentes**: as entradas não carregam campo de audiência, e cada usuário que adiciona o marketplace vê o catálogo inteiro. Hospede marketplaces separados para audiências separadas.
* **Marcar um plugin como descontinuado**: não há estado de descontinuação. A opção é remover a entrada, mapear seu nome para `null` em `renames` e opcionalmente definir `forceRemoveDeletedPlugins`.
* **Ativar auto-update para seus usuários**: cada usuário o ativa em **Marketplaces** em `/plugin` ou um administrador define `autoUpdate` em configurações gerenciadas. Veja [Turn on auto-update](#turn-on-auto-update).
* **Carregar credenciais git**: nenhum campo de marketplace mantém um token git. O acesso a um marketplace ou plugin hospedado em git segue a configuração git do usuário, por [Grant access to a private marketplace](#grant-access-to-a-private-marketplace). Para sources `archive`, uma entrada pode definir [`headers` ou `headersHelper`](#authenticate-archive-downloads) em vez disso.

<h2 id="next-steps">
  Next steps
</h2>

* [Marketplace reference](/docs/pt/plugins/marketplace-reference): campos `marketplace.json`, tipos de source e mensagens de validação
* [Manage plugins for your organization](/docs/pt/plugins/org): exija, restrinja ou semeie seu marketplace nas máquinas da sua organização
* [Plugin dependencies](/docs/pt/plugins/dependencies): marque lançamentos para que plugins que dependem do seu possam resolver versões
* [Troubleshoot plugins](/docs/pt/plugins/troubleshooting): os erros que seus usuários veem ao adicionar ou atualizar a partir de seu marketplace
