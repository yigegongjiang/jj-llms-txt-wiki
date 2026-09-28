> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dependências de plugin

> Declare os plugins dos quais seu plugin depende, com intervalos de versão como ^1.2, e veja como Claude Code instala, resolve e remove as dependências.

Uma dependência de plugin é outro plugin do qual seu plugin depende, como um cujo servidor MCP ou skill ele chama. Cada dependência rastreia a versão mais recente que seu marketplace fornece, a menos que você declare uma restrição de versão, um intervalo de versão semântica como `^2.0` ou `~2.1.0` que você testou.

Esta página é para autores de plugins que declaram dependências em `plugin.json` e para mantenedores de marketplace que marcam versões.

<Note>
  Estes casos são cobertos em outras páginas:

  * **Instalando um plugin que tem dependências**: veja [Gerenciar plugins instalados](/docs/pt/plugins/install#manage-installed-plugins)
  * **Lendo um erro de dependência**: veja [Erros de dependência](/docs/pt/plugins/troubleshooting#dependency-errors)
  * **Declarando os pacotes npm e Bun que o código do seu próprio plugin precisa**: veja [Dependências de pacotes Node.js](/docs/pt/plugins/loading#node-js-package-dependencies)
</Note>

Para adicionar uma restrição, comece em [Declare uma dependência com uma restrição de versão](#declare-a-dependency-with-a-version-constraint). Se você mantém um plugin do qual outros dependem, [marque suas versões](#tag-plugin-releases-for-version-resolution) para que suas restrições possam ser resolvidas.

<h2 id="declare-dependencies">
  Declare dependências
</h2>

<span id="decide-whether-to-constrain-dependency-versions" />Sem uma restrição de versão, uma dependência se move para cada nova versão que seu marketplace publica na próxima vez que os usuários atualizam. Se essa versão renomear uma ferramenta MCP que seu plugin chama, seu plugin quebra para todos que atualizam.

Com uma restrição como `~2.1.0` em uma dependência de uma fonte baseada em git, os usuários que têm seu plugin instalado continuam recebendo patches `2.1.x` da dependência e nunca se movem para `2.2`. Para atualizar em seu próprio cronograma, teste contra uma versão mais recente e depois publique uma nova versão do seu plugin com uma restrição mais ampla.

<h3 id="declare-a-dependency-with-a-version-constraint">
  Declare uma dependência com uma restrição de versão
</h3>

Liste as dependências no array `dependencies` do `plugin.json` do seu plugin. O manifesto a seguir declara uma dependência sem versão e uma dependência com restrição:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "deploy-kit",
  "version": "3.1.0",
  "dependencies": [
    "audit-logger",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

Uma entrada pode ser uma string: apenas o nome do plugin, como `"audit-logger"` neste manifesto, ou `"name@marketplace"` para resolvê-lo em outro marketplace. Com uma string simples, seu plugin depende de qualquer versão que o marketplace desse plugin forneça.

Para definir uma restrição de versão, use um objeto com estes campos, cada um uma string:

| Campo         | Descrição                                                                                                                                                                                                                                                                                                   |
| :------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | O nome do plugin da dependência, como aparece em sua entrada de marketplace. Claude Code o procura no mesmo marketplace que o plugin declarante, a menos que você defina `marketplace`. Obrigatório.                                                                                                        |
| `version`     | Um [intervalo de versão semântica](https://github.com/npm/node-semver#ranges) como `~2.1.0`, `^2.0`, `>=1.4`, ou `=2.1.0`. A dependência instala na tag git mais alta que satisfaz este intervalo, portanto o mantenedor da dependência deve [marcar versões](#tag-plugin-releases-for-version-resolution). |
| `marketplace` | Um marketplace diferente para resolver `name`. Uma lista de permissões controla dependências entre marketplaces, descrita em [Depender de um plugin de outro marketplace](#depend-on-a-plugin-from-another-marketplace).                                                                                    |

Um intervalo não corresponde a versões de pré-lançamento como `2.0.0-beta.1` a menos que você opte por um sufixo de pré-lançamento como `^2.0.0-0`.

<h3 id="bundle-plugins-for-a-team">
  Agrupe plugins para uma equipe
</h3>

Para permitir que engenheiros instalem um conjunto curado de plugins com um comando, publique um plugin cujo manifesto contenha um `name` e um array `dependencies`. Um manifesto de plugin precisa apenas de `name`, portanto este é um plugin válido, e instalá-lo instala todas as dependências.

Por exemplo, uma equipe de plataforma pode publicar bundles específicos de função em um marketplace interno para que engenheiros executem um `claude plugin install` em vez de instalar cada plugin separadamente:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "backend-standard",
  "version": "1.0.0",
  "description": "Standard plugin set for backend engineers",
  "dependencies": [
    "secrets-vault",
    "deploy-kit",
    { "name": "db-migrate", "version": "^3.0" },
    "oncall-runbook"
  ]
}
```

Para adicionar um plugin ao conjunto padrão mais tarde, publique uma nova versão `backend-standard` com a dependência extra. Quando o marketplace não [atualiza automaticamente por padrão](/docs/pt/plugins/loading#which-marketplaces-and-plugins-auto-update), os engenheiros ativam a atualização automática para o marketplace ou atualizam manualmente:

* **Ativar atualização automática para o marketplace**: a próxima atualização automática move o bundle para a nova versão e instala todas as dependências que ele adiciona.
* **Atualizar manualmente**: execute `claude plugin update backend-standard` em um shell, depois `/reload-plugins` em uma sessão aberta para instalar as dependências recém-adicionadas.

Para as etapas do lado do engenheiro, veja [Manter plugins atualizados](/docs/pt/plugins/install#keep-plugins-updated).

Para implantar um bundle para todos em uma organização, um administrador o adiciona a `enabledPlugins` nas configurações gerenciadas. Veja [Pré-instalar e exigir plugins](/docs/pt/plugins/org#pre-install-and-require-plugins).

<h3 id="depend-on-a-plugin-from-another-marketplace">
  Depender de um plugin de outro marketplace
</h3>

Por padrão, Claude Code não instala uma dependência de um marketplace diferente do próprio plugin declarante, a menos que o usuário já tenha essa dependência instalada e ativada no mesmo escopo. Este padrão impede que um marketplace instale silenciosamente plugins de uma fonte que o usuário não revisou.

Para permitir a instalação, adicione o nome do marketplace de destino a `allowCrossMarketplaceDependenciesOn` no `marketplace.json` do marketplace raiz. O marketplace raiz é aquele que hospeda o plugin que o usuário está instalando. Apenas a lista de permissões do marketplace raiz se aplica.

O seguinte `marketplace.json` permite que `deploy-kit` dependa de um plugin de `your-shared-marketplace`:

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "allowCrossMarketplaceDependenciesOn": ["your-shared-marketplace"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "your-shared-marketplace" }
      ]
    }
  ]
}
```

Se `allowCrossMarketplaceDependenciesOn` estiver faltando ou não incluir o marketplace de destino, Claude Code não instala a dependência. Quando a dependência é declarada na entrada do marketplace, a própria instalação é recusada com uma mensagem que começa com `Dependency "audit-logger@your-shared-marketplace" (required by deploy-kit@your-marketplace) is in marketplace "your-shared-marketplace", which is not in the allowlist` e nomeia o campo a definir. Quando é declarada em `plugin.json`, a instalação é concluída sem a dependência e seu plugin falha ao carregar.

A verificação da lista de permissões não se aplica a uma dependência que já está ativada. Se um usuário instalar `audit-logger` de `your-shared-marketplace` primeiro, no mesmo escopo, `deploy-kit` então instala sem qualquer alteração na lista de permissões.

<h3 id="test-a-plugin-and-its-dependency-locally">
  Teste um plugin e sua dependência localmente
</h3>

Se você está desenvolvendo um plugin e o plugin do qual ele depende ao mesmo tempo, inicie Claude Code a partir do seu shell e carregue ambos com [`--plugin-dir`](/docs/pt/plugins/cli-reference#flags-that-load-a-plugin-for-one-session):

```bash theme={null}
claude --plugin-dir ./my-dependency --plugin-dir ./my-plugin
```

A cópia local da dependência satisfaz a entrada de dependência do seu plugin, portanto você não precisa instalar a dependência de seu marketplace.

* **Sem `version` necessária**: o `plugin.json` local também não precisa de uma `version`, porque uma [restrição de versão](#declare-a-dependency-with-a-version-constraint) não é verificada contra uma cópia local.
* **Entradas que nomeiam um marketplace**: uma entrada que nomeia um marketplace também corresponde à cópia local no Claude Code v2.1.242 ou posterior.

Até você instalar a dependência de seu marketplace, seu plugin para de carregar sempre que a cópia local é desativada ou ausente:

* **Você desativou a cópia local**: seu plugin é desativado no próximo carregamento de plugin, com um erro que termina com `is disabled — enable it or remove the dependency`. Quando o erro nomeia a dependência como `<name>@inline`, esse identificador se refere à cópia `--plugin-dir`.
* **Você iniciou uma sessão sem a flag `--plugin-dir` da dependência**: o erro relata a dependência como não instalada. Passe a flag novamente ou instale a dependência de seu marketplace.

Quando ambos os plugins estão em uma pasta pai, você pode passar essa pasta para `--plugin-dir` uma vez. Se a pasta não for ela mesma um plugin, Claude Code carrega cada pasta filha que tem um `.claude-plugin/plugin.json`. Requer Claude Code v2.1.265 ou posterior.

<h2 id="tag-plugin-releases-for-version-resolution">
  Libere um plugin do qual outros dependem
</h2>

Se você mantém um plugin do qual outros plugins dependem com uma restrição de versão, marque suas versões para que essas restrições possam ser resolvidas. Uma restrição é resolvida contra tags git no repositório que hospeda o plugin. Marque o repositório que a [fonte do plugin](/docs/pt/plugins/marketplace-reference#plugin-sources) do plugin em `marketplace.json` aponta:

* **Fonte `github`, `url`, ou `git-subdir`**: o repositório do próprio plugin, portanto o autor do plugin cria as tags
* **Caminho relativo como `./plugins/secrets-vault`**: o repositório do marketplace, portanto o mantenedor do marketplace cria as tags

<h3 id="create-a-release-tag">
  Crie uma tag de versão
</h3>

Marque cada versão como `<plugin-name>--v<version>`, onde `<version>` corresponde ao campo `version` no `plugin.json` desse commit. O prefixo plugin-name permite que um repositório de marketplace hospede vários plugins com históricos de versão independentes.

Crie a tag a partir do diretório do plugin, com um remote `origin` configurado para receber a tag enviada, usando [`claude plugin tag`](/docs/pt/plugins/cli-reference#plugin-tag):

```bash theme={null}
claude plugin tag --push
```

O comando constrói o nome da tag a partir do manifesto do plugin. Antes de criar a tag, ele executa estas verificações:

* Valida o plugin
* Verifica se `plugin.json` e a entrada do marketplace concordam sobre a versão, quando o diretório do plugin está dentro de um checkout de marketplace
* Requer uma árvore de trabalho limpa sob o diretório do plugin
* Recusa se a tag já existe

Uma execução bem-sucedida imprime `Created tag secrets-vault--v2.1.0`. Com `--push`, também imprime `Pushed to origin`. Sem `--push`, imprime o comando `git push` para você executar.

Passe `--dry-run` para ver o plano sem criar nada.

A [referência `claude plugin tag`](/docs/pt/plugins/cli-reference#plugin-tag) lista as flags restantes.

Você também pode executar `git tag secrets-vault--v2.1.0` diretamente, desde que mantenha a `version` em `plugin.json` e na entrada do marketplace em sincronização você mesmo.

<h3 id="constrain-a-dependency-that-has-a-non-git-source">
  Restrinja uma dependência que tem uma fonte não-git
</h3>

A resolução baseada em tag se aplica apenas a fontes baseadas em git. Para uma dependência com uma [fonte de plugin](/docs/pt/plugins/marketplace-reference#plugin-sources) `npm`, `archive`, ou `command`, a restrição não controla qual versão é buscada. Ainda é verificada quando o plugin carrega, e o plugin dependente é desativado se a versão instalada não a satisfaz.

Para fontes `npm`, `archive`, e `command`, a versão verificada é a `version` no `plugin.json` da dependência. Defina uma lá antes de restringir essa dependência, porque um `plugin.json` que não define versão satisfaz nenhuma restrição.

Claude Code nunca instala uma dependência com uma fonte `command` em si, portanto os usuários [a instalam primeiro](/docs/pt/plugins/marketplace-reference#command-plugin-source). Também nunca executa o [`headersHelper`](/docs/pt/plugins/host-marketplace#authenticate-archive-downloads) de uma dependência, portanto os usuários também instalam uma dependência cuja entrada de marketplace define um antes de instalar seu plugin.

Além de `claude plugin install`, estas operações também instalam qualquer dependência declarada faltante, e os limites `command` e `headersHelper` se aplicam a elas também:

* `/reload-plugins`
* Auto-atualização do marketplace do plugin dependente
* Re-executar `claude plugin install` no plugin dependente
* `claude plugin marketplace add`

<h2 id="how-dependencies-behave-for-your-users">
  Como as dependências se comportam para seus usuários
</h2>

Estas seções descrevem como Claude Code resolve, verifica e combina as restrições que você declara uma vez que seu plugin é instalado junto com outros.

<h3 id="how-a-constraint-resolves-against-tags">
  Como uma restrição é resolvida contra tags
</h3>

Quando um usuário instala um plugin que declara `{ "name": "secrets-vault", "version": "~2.1.0" }`, a dependência instala a partir da tag `secrets-vault--v` mais alta que satisfaz `~2.1.0` no repositório que hospeda `secrets-vault`. Quando nenhuma tag satisfaz o intervalo, a instalação falha ou usa a cópia atual do marketplace:

* **Plugin com seu próprio repositório**: a instalação falha com uma mensagem contendo `Dependency "secrets-vault@your-marketplace" has no git tag satisfying`.
* **Plugin referenciado por um caminho relativo**: a instalação usa a cópia atual do marketplace em vez disso, e a restrição é verificada quando o plugin carrega. Se essa cópia estiver fora do intervalo, o plugin dependente permanece desativado e `claude plugin list` mostra `Requires "secrets-vault@your-marketplace" ~2.1.0, installed 3.0.0`.

Para um plugin que o marketplace referencia por um caminho relativo, um marketplace que você adicionou como um caminho de pasta local também resolve restrições contra as tags git dessa pasta, quando a pasta é um repositório git. Isto requer Claude Code v2.1.196 ou posterior. Uma pasta local que não é um repositório git não tem tags, portanto Claude Code instala a dependência a partir do conteúdo atual da pasta em vez disso.

<h3 id="confirm-the-resolved-version">
  Confirme a versão resolvida
</h3>

Para confirmar qual versão uma restrição foi resolvida, execute `claude plugin list` em seu shell. Uma dependência resolvida por tag mostra sua versão com um sufixo de commit de 12 caracteres, como `2.1.0-8713c5b11005`.

As verificações de restrição usam a versão da tag em vez da `version` em `plugin.json`, mesmo se `plugin.json` nesse commit ficar para trás.

Se você forçar a movimentação de uma tag para um commit diferente, a próxima instalação busca o conteúdo desse commit em vez de reutilizar uma cópia em cache obsoleta. Veja [Versões e atualizações](/docs/pt/plugins/loading#versions-and-updates) para como a versão de um plugin se torna sua chave de cache.

<h3 id="combine-constraints-from-several-plugins">
  Combine restrições de vários plugins
</h3>

Quando vários plugins instalados restringem a mesma dependência, a dependência é resolvida para a versão mais alta que satisfaz todos os seus intervalos. Combinações comuns são resolvidas assim:

| Plugin A requer | Plugin B requer | Resultado                                                                                                                                 |
| :-------------- | :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `^2.0`          | `>=2.1`         | Uma instalação na tag `2.x` mais alta em ou acima de `2.1.0`. Ambos os plugins carregam.                                                  |
| `~2.1`          | `~3.0`          | A instalação do plugin B falha com uma mensagem `has conflicting version requirements`. Plugin A e a dependência permanecem como estavam. |
| `=2.1.0`        | nenhum          | A dependência permanece em `2.1.0`. A atualização automática pula versões mais recentes enquanto o plugin A está instalado.               |

A atualização automática busca uma dependência restrita na tag git mais alta que satisfaz o intervalo de cada plugin instalado, em vez de na versão mais recente do marketplace. Se os intervalos dos plugins instalados não se sobrepõem, a atualização automática deixa essa dependência em sua versão atual, e a aba **Errors** do `/plugin` mostra uma entrada nomeando o plugin restritivo. Se eles se sobrepõem mas nenhuma tag cai no intervalo, a atualização automática busca a cópia atual do marketplace e pula a atualização quando a `version` dessa cópia cai fora do intervalo de qualquer plugin instalado.

Quando um usuário desinstala o último plugin que restringe uma dependência, a dependência não é mais restrita a um intervalo de versão e retoma o rastreamento de sua entrada de marketplace na próxima atualização.

<h2 id="see-also">
  Veja também
</h2>

* [`claude plugin prune`](/docs/pt/plugins/cli-reference#plugin-prune): remova dependências auto-instaladas que nenhum plugin precisa mais
* [Hospede um marketplace](/docs/pt/plugins/host-marketplace): canais de lançamento e recomendação de outros plugins
