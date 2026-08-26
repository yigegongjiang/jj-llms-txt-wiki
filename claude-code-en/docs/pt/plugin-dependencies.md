> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Restringir versões de dependências de plugins

> Declare restrições de versão em dependências de plugins e agrupe um conjunto de plugins curado atrás de uma única instalação.

Um plugin pode depender de outros plugins listando-os em `plugin.json` ou em sua entrada de marketplace. Por padrão, uma dependência rastreia a versão mais recente disponível, portanto um lançamento upstream pode alterar a dependência sob seu plugin sem aviso. Restrições de versão permitem que você mantenha uma dependência em um intervalo de versão testado até que você escolha se mover.

Quando você instala um plugin que declara dependências, Claude Code resolve e instala automaticamente e lista quais dependências foram adicionadas no final da saída de instalação. Se uma dependência desaparecer posteriormente, `/reload-plugins` e a atualização automática de plugin em segundo plano a reinstalam, desde que seu marketplace já esteja em seus marketplaces configurados. Executar novamente `claude plugin install` no plugin dependente, ou adicionar um marketplace com `claude plugin marketplace add`, também resolve quaisquer dependências ausentes pendentes. Dependências de um marketplace que você não adicionou são deixadas não resolvidas.

Este guia é para autores de plugins que declaram dependências em `plugin.json` e para mantenedores de marketplace que marcam lançamentos. Para instalar plugins que têm dependências, consulte [Descobrir e instalar plugins](/docs/pt/discover-plugins). Para o esquema de manifesto completo, consulte a [referência de Plugins](/docs/pt/plugins-reference).

<h2 id="why-constrain-dependency-versions">
  Por que restringir versões de dependências
</h2>

Considere um marketplace interno onde dois times publicam plugins. O time de plataforma mantém `secrets-vault`, um servidor MCP que envolve um backend de segredos. O time de deploy mantém `deploy-kit`, que chama `secrets-vault` para buscar credenciais durante deploys.

`deploy-kit` é testado contra `secrets-vault` v2.1.0. Sem uma restrição de versão, na próxima vez que o time de plataforma marcar um lançamento que renomeia uma ferramenta MCP, a atualização automática move `secrets-vault` de cada engenheiro para a nova versão e `deploy-kit` quebra.

Com uma restrição de versão, `deploy-kit` declara que precisa de `secrets-vault` no intervalo `~2.1.0`. Engenheiros com `deploy-kit` instalado permanecem na versão patch `2.1.x` mais alta correspondente. O time de deploy faz upgrade em seu próprio cronograma publicando uma nova versão de `deploy-kit` com uma restrição mais ampla.

<h2 id="declare-a-dependency-with-a-version-constraint">
  Declare uma dependência com uma restrição de versão
</h2>

Liste dependências no array `dependencies` do `plugin.json` do seu plugin. Cada entrada é um nome de plugin ou um objeto com uma restrição de versão.

O manifesto a seguir declara uma dependência sem versão e uma dependência restrita:

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

Uma entrada pode ser uma string simples com apenas o nome do plugin, como `"audit-logger"` no exemplo acima, que depende de qualquer versão que o marketplace desse plugin forneça. Para mais controle, use um objeto com estes campos:

| Campo         | Tipo   | Descrição                                                                                                                                                                                                                                                                             |
| :------------ | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`        | string | Nome do plugin. Resolve dentro do mesmo marketplace que o plugin declarante. Obrigatório.                                                                                                                                                                                             |
| `version`     | string | Um [intervalo semver](https://github.com/npm/node-semver#ranges) como `~2.1.0`, `^2.0`, `>=1.4`, ou `=2.1.0`. A dependência é buscada na versão marcada mais alta que satisfaz este intervalo.                                                                                        |
| `marketplace` | string | Um marketplace diferente para resolver `name`. Dependências entre marketplaces são bloqueadas a menos que o marketplace de destino esteja listado em [`allowCrossMarketplaceDependenciesOn`](#depend-on-a-plugin-from-another-marketplace) no `marketplace.json` do marketplace raiz. |

O campo `version` aceita qualquer expressão suportada pelo pacote `semver` do Node, incluindo intervalos de circunflexo, til, hífen e comparador. Versões pré-lançamento como `2.0.0-beta.1` são excluídas a menos que seu intervalo opte por um sufixo pré-lançamento como `^2.0.0-0`.

<h2 id="bundle-plugins-for-a-team">
  Agrupar plugins para uma equipe
</h2>

Além do `name` obrigatório, um manifesto de plugin pode consistir apenas em um array `dependencies`. Instalá-lo puxa todas as dependências, o que o torna uma forma de empacotar um conjunto de plugins curado atrás de uma única instalação.

Por exemplo, uma equipe de plataforma pode publicar bundles específicos de função em um marketplace interno para que os engenheiros executem um único `claude plugin install` em vez de instalar cada ferramenta separadamente:

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

Instalar `backend-standard` resolve e instala todas as quatro dependências.

Para adicionar uma ferramenta ao conjunto padrão posteriormente, publique uma nova versão de `backend-standard` com a dependência extra. A atualização automática está desativada por padrão para marketplaces não-Anthropic, portanto os engenheiros pegam a nova versão de uma de duas formas:

* Ative a atualização automática para o marketplace em `/plugin`. A próxima atualização automática move o bundle para a nova versão e instala quaisquer dependências que ele adiciona.
* Execute `claude plugin update backend-standard`, depois `/reload-plugins` para instalar as dependências recém-adicionadas.

Para distribuir bundles em toda uma organização, adicione o plugin bundle a `enabledPlugins` nas [configurações gerenciadas](/docs/pt/settings#enabledplugins).

<h2 id="depend-on-a-plugin-from-another-marketplace">
  Dependa de um plugin de outro marketplace
</h2>

Por padrão, Claude Code recusa auto-instalar uma dependência que vive em um marketplace diferente do plugin que a declara. Isso evita que um marketplace puxe silenciosamente plugins de uma fonte que você não revisou.

Para permitir, o mantenedor do marketplace raiz adiciona o nome do marketplace de destino a `allowCrossMarketplaceDependenciesOn` em `marketplace.json`. O marketplace raiz é aquele que hospeda o plugin que o usuário está instalando; apenas sua lista de permissões é consultada, portanto a confiança não se encadeia através de marketplaces intermediários.

O seguinte `marketplace.json` permite que `deploy-kit` dependa de um plugin de `acme-shared`:

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "allowCrossMarketplaceDependenciesOn": ["acme-shared"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "acme-shared" }
      ]
    }
  ]
}
```

Se o campo estiver faltando ou não incluir o marketplace de destino, a instalação falha com um erro `cross-marketplace` nomeando o campo a ser definido. Os usuários ainda podem instalar a dependência manualmente primeiro, o que satisfaz a restrição sem alterar a lista de permissões.

<h2 id="tag-plugin-releases-for-version-resolution">
  Marque lançamentos de plugins para resolução de versão
</h2>

Restrições de versão resolvem contra tags git no repositório do marketplace. Para Claude Code encontrar as versões disponíveis de uma dependência, os lançamentos do plugin upstream devem ser marcados usando uma convenção de nomenclatura específica.

Marque cada lançamento como `{plugin-name}--v{version}`, onde `{version}` corresponde ao campo `version` no `plugin.json` daquele commit. Do diretório do plugin, execute:

```bash theme={null}
claude plugin tag --push
```

O comando `claude plugin tag` deriva o nome da tag do manifesto do plugin e da entrada do marketplace envolvente. Antes de criar a tag, ele valida o conteúdo do plugin, verifica se `plugin.json` e a entrada do marketplace concordam sobre a versão, requer uma árvore de trabalho limpa sob o diretório do plugin e recusa se a tag já existe. Adicione `--dry-run` para ver o que seria marcado sem criar. Executar `git tag secrets-vault--v2.1.0` diretamente é equivalente se você manter `plugin.json` e a entrada do marketplace em sincronização você mesmo.

O prefixo de nome do plugin permite que um repositório de marketplace hospede múltiplos plugins com linhas de versão independentes. O separador `--v` é analisado como uma correspondência de prefixo no nome completo do plugin, portanto nomes de plugins que contêm hífens são tratados corretamente.

Quando você instala um plugin que declara `{ "name": "secrets-vault", "version": "~2.1.0" }`, Claude Code lista as tags do marketplace, filtra aquelas começando com `secrets-vault--v`, e busca a versão mais alta satisfazendo `~2.1.0`. Se nenhuma tag correspondente existir, o plugin dependente é desabilitado com um erro listando as versões disponíveis.

Um marketplace adicionado como um caminho de pasta local resolve tags da mesma forma quando a pasta é um repositório git. Isso requer Claude Code v2.1.196 ou posterior. Em dois casos Claude Code instala a dependência do conteúdo atual da pasta:

* Versões anteriores não leem tags de um marketplace de pasta local, portanto uma dependência restrita carrega apenas se essa cópia satisfaz o intervalo.
* Uma pasta local que não é um repositório git não tem tags, independentemente da versão.

A semver da tag resolvida é registrada separadamente da `version` do `plugin.json`, portanto verificações de restrição usam a tag que foi realmente buscada mesmo se `plugin.json` naquele commit tiver um valor obsoleto. O nome do diretório de cache para uma instalação resolvida por tag inclui um sufixo de commit-SHA de 12 caracteres, portanto se um mantenedor move uma tag à força para um commit diferente, a próxima instalação obtém um diretório de cache fresco em vez de reutilizar conteúdo obsoleto.

<Note>
  Para fontes de marketplace `npm`, a restrição não controla qual versão é buscada, já que a resolução baseada em tag se aplica apenas a fontes apoiadas por git. A restrição ainda é verificada no tempo de carregamento, e o plugin dependente é desabilitado com `dependency-version-unsatisfied` se a versão instalada não a satisfizer.
</Note>

<h2 id="how-constraints-interact">
  Como restrições interagem
</h2>

Quando vários plugins instalados restringem a mesma dependência, Claude Code intersecciona seus intervalos e resolve a dependência para a versão mais alta que satisfaz todos eles. A tabela abaixo mostra como combinações comuns resolvem.

| Plugin A requer | Plugin B requer | Resultado                                                                                                    |
| :-------------- | :-------------- | :----------------------------------------------------------------------------------------------------------- |
| `^2.0`          | `>=2.1`         | Uma instalação na tag `2.x` mais alta em ou acima de `2.1.0`. Ambos os plugins carregam.                     |
| `~2.1`          | `~3.0`          | Instalação do plugin B falha com `range-conflict`. Plugin A e a dependência permanecem como estavam.         |
| `=2.1.0`        | nenhum          | A dependência permanece em `2.1.0`. Auto-update pula versões mais recentes enquanto plugin A está instalado. |

Auto-update busca uma dependência restrita na tag git mais alta que satisfaz o intervalo de cada plugin instalado, em vez de na versão mais recente do marketplace, portanto a dependência continua a receber atualizações dentro de seu intervalo permitido. Se nenhuma tag satisfizer todos os intervalos, auto-update pula essa dependência e lista o pulo na aba Errors do `/plugin`, nomeando o plugin restringidor.

Quando você desinstala o último plugin que restringe uma dependência, a dependência não é mais mantida e retoma o rastreamento de sua entrada de marketplace na próxima atualização.

<h2 id="enable-or-disable-a-plugin-with-dependencies">
  Ativar ou desativar um plugin com dependências
</h2>

Ativar um plugin também ativa os plugins dos quais ele depende, e desativar um plugin é bloqueado se outro plugin ativado ainda precisar dele. Ambos os comportamentos requerem Claude Code v2.1.143 ou posterior. Versões anteriores ativam ou desativam apenas o plugin nomeado e exibem um erro `dependency-unsatisfied` no próximo carregamento.

Quando você ativa um plugin, Claude Code também ativa suas dependências no mesmo escopo. Se uma dependência tiver suas próprias dependências, Claude Code ativa aquelas também. A mensagem de sucesso lista o que mais foi ativado junto com o plugin que você nomeou. Se uma dependência não puder ser ativada, o comando recusa e diz o que está bloqueando e como corrigir:

| Condição                                                                                                  | Resultado                                                                                                       |
| :-------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| Uma dependência não está instalada                                                                        | Ativar falha e imprime o comando `claude plugin install` para cada dependência ausente.                         |
| Uma dependência é bloqueada pela política de plugins da sua organização                                   | Ativar falha e nomeia a dependência bloqueada.                                                                  |
| Uma dependência está definida como `false` em um escopo com precedência mais alta que o escopo de destino | Ativar falha. Ative a dependência naquele escopo, ou passe `--scope` para escrever lá.                          |
| Todas as dependências estão instaladas e permitidas                                                       | Ativar sucede e escreve `true` para o plugin e cada dependência que não estava já ativada no escopo de destino. |

Isto se aplica mesmo quando uma dependência define [`defaultEnabled: false`](/docs/pt/plugins-reference#default-enablement) em seu manifesto, porque Claude Code escreve um `true` explícito para ela. O mesmo se aplica na instalação: uma dependência trazida para satisfazer um plugin ativo instala com `true` independentemente de seu próprio padrão.

Quando você desativa um plugin, Claude Code recusa se outro plugin ativado ainda depender dele. O erro nomeia os plugins que dependem dele e dá a você um comando encadeado que os desativa na ordem correta, terminando com o que você pediu.

Por exemplo, se `deploy-kit` depende de `secrets-vault`, desativar `secrets-vault` sozinho falha com saída similar à seguinte:

```text theme={null}
secrets-vault is still required by deploy-kit. Disable that plugin first, or
disable everything together: claude plugin disable deploy-kit@acme-tools && claude plugin disable secrets-vault@acme-tools
```

Copie o comando encadeado do erro para desativar o conjunto completo em uma etapa.

<h2 id="remove-orphaned-auto-installed-dependencies">
  Remova dependências auto-instaladas órfãs
</h2>

Dependências auto-instaladas permanecem no disco após os plugins que as instalaram serem desinstalados, no caso de você reinstalar um plugin dependente ou querer continuar usando a dependência diretamente. Para limpá-las, execute `claude plugin prune` para listar as dependências auto-instaladas que não têm mais nenhum plugin instalado exigindo-as e removê-las após um prompt de confirmação. Isso requer Claude Code v2.1.121 ou posterior.

```bash theme={null}
claude plugin prune
```

Por padrão, prune opera no escopo do usuário. Use `--scope project` ou `--scope local` para direcionar um escopo diferente. Passe `--dry-run` para listar o que seria removido sem alterar nada. Passe `-y` para pular o prompt de confirmação. Quando stdin ou stdout não é um terminal, prune lista os órfãos e sai sem removê-los a menos que `-y` seja passado.

Para prune como parte de uma desinstalação, passe `--prune` para `claude plugin uninstall`. Após remover o plugin nomeado, Claude Code verifica e remove quaisquer dependências auto-instaladas que agora estão órfãs. Plugins que você instalou você mesmo nunca são podados, apenas aqueles instalados automaticamente através do array `dependencies` de outro plugin.

Por exemplo, para desinstalar `deploy-kit` e limpar as dependências que deixa para trás:

```bash theme={null}
claude plugin uninstall deploy-kit --prune
```

<h2 id="resolve-dependency-errors">
  Resolva erros de dependência
</h2>

Problemas de dependência aparecem em `claude plugin list` e na interface `/plugin`. Claude Code desabilita o plugin afetado até que você resolva o erro. A tabela abaixo lista os erros mais comuns e como resolvê-los.

| Erro                             | Significado                                                                                                                                                                                                                                                           | Como resolver                                                                                                                                                                                                                                                                        |
| :------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `dependency-unsatisfied`         | Uma dependência declarada não está instalada, ou está instalada mas desabilitada.                                                                                                                                                                                     | Execute o comando `claude plugin install` mostrado na mensagem de erro. Se o marketplace da dependência ainda não está configurado, adicione-o com `claude plugin marketplace add` e Claude Code resolve a dependência automaticamente. Se a dependência está desabilitada, ative-a. |
| `range-conflict`                 | Os requisitos de versão para uma dependência não podem ser combinados. A mensagem de erro nomeia a causa: nenhuma versão satisfaz todos os intervalos, um intervalo não é sintaxe semver válida, ou os intervalos combinados são muito complexos para interseccionar. | Desinstale ou atualize um dos plugins conflitantes, corrija qualquer string `version` inválida, simplifique cadeias `\|\|` longas, ou peça ao autor upstream para ampliar sua restrição.                                                                                             |
| `dependency-version-unsatisfied` | A versão da dependência instalada está fora do intervalo declarado deste plugin.                                                                                                                                                                                      | Execute `claude plugin install <dependency>@<marketplace>` para re-resolver a dependência contra todas as restrições atuais.                                                                                                                                                         |
| `no-matching-tag`                | O repositório da dependência não tem uma tag `{name}--v*` satisfazendo o intervalo.                                                                                                                                                                                   | Verifique se o upstream marcou lançamentos usando a convenção acima, ou relaxe seu intervalo.                                                                                                                                                                                        |

Para verificar esses erros programaticamente, execute `claude plugin list --json` e leia o campo `errors` em cada plugin.

<h2 id="see-also">
  Veja também
</h2>

* [Criar plugins](/docs/pt/plugins): construa plugins com skills, agents e hooks
* [Criar e distribuir um marketplace de plugins](/docs/pt/plugin-marketplaces): hospede plugins para seu time
* [Referência de Plugins](/docs/pt/plugins-reference#plugin-manifest-schema): o esquema completo de `plugin.json`
* [Gerenciamento de versão](/docs/pt/plugins-reference#version-management): como a versão própria de um plugin é resolvida e usada como a chave de cache
