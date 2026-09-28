> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Publicar e distribuir um plugin

> Publique um plugin Claude Code através do seu próprio marketplace ou do marketplace da comunidade da Anthropic, com uma lista de verificação de pré-lançamento e como os usuários recebem atualizações.

Publicar um plugin Claude Code significa listá-lo em um marketplace, um catálogo JSON que lista plugins e onde buscar cada um, para que outras pessoas possam instalá-lo pelo nome e receber suas atualizações. Você pode executar seu próprio marketplace ou enviar seu plugin para o marketplace da comunidade da Anthropic. Para compartilhar um plugin sem publicá-lo, envie o diretório do plugin ou um `.zip` dele para que as pessoas carreguem por conta própria.

Esta página é para o autor de um plugin funcional que está pronto para compartilhá-lo.

<Note>
  Estes casos são cobertos em outras páginas:

  * **Seu plugin ainda não está pronto**: comece com [Criar um plugin](/docs/pt/plugins/create)
  * **Você mantém uma CLI ou SDK com um plugin em um marketplace oficial**: veja [Recomende seu plugin a partir de sua CLI](/docs/pt/plugins/cli-hints)
</Note>

Comece com [Escolha como distribuir](#choose-how-to-distribute) para comparar as opções de distribuição. Se você já conhece sua rota, vá para [Prepare seu plugin para lançamento](#prepare-your-plugin-for-release), depois siga a seção da sua rota para o que contar aos seus usuários e como eles recebem suas atualizações.

<h2 id="choose-how-to-distribute">
  Escolha como distribuir
</h2>

Escolha uma opção de distribuição com base em quem precisa instalar o plugin:

| Rota                                                                           | Quem pode instalar                                                                                  | O que você precisa                                                                             | Os usuários recebem suas atualizações automaticamente? |
| :----------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :----------------------------------------------------- |
| [Sem marketplace](#share-a-plugin-without-a-marketplace)                       | As pessoas para as quais você envia a pasta do plugin ou um `.zip` dele                             | A pasta do plugin                                                                              | Nenhuma. Eles carregam a cópia que você enviou         |
| [Seu próprio marketplace](#publish-through-your-own-marketplace)               | Qualquer pessoa que possa acessar o repositório, que pode ser um privado que sua equipe pode clonar | Um repositório git ou outro host com um `.claude-plugin/marketplace.json` que lista seu plugin | Desativado                                             |
| [Marketplace da comunidade da Anthropic](#submit-to-the-community-marketplace) | Qualquer pessoa que adicione `anthropics/claude-plugins-community`                                  | Um envio através do formulário de envio do diretório de plugins                                | Desativado                                             |

Auto-atualização é uma configuração por marketplace no lado do usuário que busca novas versões em segundo plano.

<h2 id="prepare-your-plugin-for-release">
  Prepare seu plugin para lançamento
</h2>

O nome, a versão, validação e uma instalação a partir de um marketplace decidem se um lançamento funciona para as pessoas que o instalam. Verifique-os antes do primeiro lançamento e novamente antes de cada um posterior.

<Steps>
  <Step title="Escolha um nome permanente">
    Os usuários instalam, habilitam e configuram seu plugin por `name@marketplace`, então um plugin renomeado é um plugin diferente para cada instalação existente. Escolha um nome em kebab-case como `deploy-helper`, porque `claude plugin validate` avisa sobre outras formas, e trate-o como permanente. Defina `displayName` em `plugin.json` para o rótulo que os usuários veem.
  </Step>

  <Step title="Decida como você versionar">
    Se você definir `version` em `plugin.json` e depois fazer push de commits sem alterá-lo, `claude plugin update` imprime `<name> is already at the latest version (1.0.0).` e os usuários mantêm a cópia antiga. Incremente `version` em cada lançamento ou omita-o em um marketplace hospedado em git para que Claude Code use o SHA do commit. Veja [Versões e atualizações](/docs/pt/plugins/loading#versions-and-updates).
  </Step>

  <Step title="Valide">
    Em seu shell, execute `claude plugin validate --strict ./your-plugin`. Uma execução limpa imprime `✔ Validation passed`.

    * **Em CI**: mantenha `--strict`, que também falha a execução com código de saída 1 em avisos como um campo de manifesto desconhecido ou um `version` ausente. Remova `--strict` se você escolheu omitir `version` na etapa anterior.
    * **Caminhos**: a validação relata caminhos de componentes que não começam com `./`. Dentro de comandos hook e configurações de servidor MCP, consulte arquivos como `${CLAUDE_PLUGIN_ROOT}/...`. Veja [regras de caminho](/docs/pt/plugins/manifest-reference#path-rules).
  </Step>

  <Step title="Instale-o a partir de um marketplace local">
    Em seu shell, adicione um marketplace local que lista o plugin com `claude plugin marketplace add ./path-to-marketplace`, instale o plugin a partir dele e inicie uma sessão para confirmar que ele carrega.

    * Para o menor marketplace que funciona, veja [Criar um marketplace](/docs/pt/plugins/create-marketplace).
    * Para saber se uma instalação carrega seu diretório de origem ou uma cópia em cache, veja [Plugins in-place e copiados](/docs/pt/plugins/loading#in-place-and-copied-plugins).
  </Step>

  <Step title="Preencha os metadados que os usuários veem">
    Defina `description`, `author`, `homepage` e `repository` em `plugin.json` e adicione um `README.md` na raiz do plugin. `homepage` deve ser analisado como uma URL. A [referência de manifesto](/docs/pt/plugins/manifest-reference#fields) lista todos os campos.
  </Step>

  <Step title="Execute sua suite de avaliação">
    Se você tiver uma suite de avaliação, execute `claude plugin eval` em seu shell. Ele executa os casos de teste do plugin e pontua os resultados, o que detecta regressões quando você altera o plugin. Veja [Teste plugins com evals](/docs/pt/plugin-evals).
  </Step>
</Steps>

<h2 id="share-a-plugin-without-a-marketplace">
  Compartilhe um plugin sem um marketplace
</h2>

Se o plugin estiver em um repositório git, as pessoas podem cloná-lo e carregar o checkout, ou iniciar Claude Code a partir de seu shell com `--plugin-url` apontando para um `.zip` que você anexa a um lançamento. Para obter sua próxima versão, eles puxam ou baixam novamente. Se não estiver em um repositório, envie-lhes o diretório ou um `.zip` dele. Eles o carregam de uma de duas maneiras:

* **Para uma sessão**: eles iniciam Claude Code a partir de seu shell com `claude --plugin-dir ./deploy-helper`, onde o caminho é o clone, a pasta descompactada ou o próprio `.zip`. Veja [Sinalizadores que carregam um plugin para uma sessão](/docs/pt/plugins/cli-reference#flags-that-load-a-plugin-for-one-session).
* **Para cada sessão**: eles movem o diretório do plugin, com seu `.claude-plugin/plugin.json`, sob `~/.claude/skills/` para que Claude Code [o carregue em cada sessão](/docs/pt/plugins/loading#find-where-a-plugin-came-from).

Adicionar um `.claude-plugin/marketplace.json` ao mesmo repositório é o que permite que as pessoas instalem pelo nome e atualizem com um comando; veja [Publique através de seu próprio marketplace](#publish-through-your-own-marketplace).

<h3 id="ship-a-plugin-with-your-own-tool">
  Envie um plugin com sua própria ferramenta
</h3>

Se você mantém uma CLI ou SDK, publique o plugin em um marketplace e faça seu instalador ou mensagem pós-instalação executar ou imprimir os dois comandos que um usuário precisa: `claude plugin marketplace add <source>`, depois `claude plugin install <name>@<marketplace>`. Para descoberta em sessão quando alguém usa sua ferramenta, veja [Recomende seu plugin a partir de sua CLI](/docs/pt/plugins/cli-hints).

<h2 id="publish-through-your-own-marketplace">
  Publique através de seu próprio marketplace
</h2>

Seu próprio marketplace é um arquivo `.claude-plugin/marketplace.json` que lista seu plugin, adicionado a um repositório git. Uma vez que o arquivo está no repositório, o plugin é publicado, sem formulário de envio. Você pode manter o arquivo no próprio repositório do plugin ou em um separado.

<h3 id="add-the-marketplace-file-to-your-repository">
  Adicione o arquivo de marketplace ao seu repositório
</h3>

Para publicar a partir do próprio repositório do plugin, salve o arquivo de marketplace ao lado de `plugin.json` em `.claude-plugin/`, com uma entrada cuja `source` é `"./"`, a raiz do repositório. Dê à entrada o mesmo `name` que `plugin.json`, de acordo com [Mantenha o nome da entrada e o nome do manifesto iguais](/docs/pt/plugins/create-marketplace#keep-the-entry-name-and-the-manifest-name-the-same):

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Name" },
  "plugins": [
    { "name": "deploy-helper", "source": "./" }
  ]
}
```

Em seu shell, execute `claude plugin validate .` no repositório para verificar o arquivo antes de fazer push.

[Criar um marketplace](/docs/pt/plugins/create-marketplace) cobre o layout com vários plugins em um repositório.

<h3 id="control-who-can-install">
  Controle quem pode instalar
</h3>

Qualquer pessoa que possa clonar o repositório pode instalar a partir dele, então se o repositório for privado, o marketplace também será privado. Para hosts diferentes de um repositório git, veja [Hospede um marketplace](/docs/pt/plugins/host-marketplace). Para alcançar todos em uma empresa, incluindo pessoas que não usam git, veja [Implante para uma empresa inteira](/docs/pt/plugins/host-marketplace#roll-out-to-a-whole-company).

<h3 id="tell-users-how-to-install">
  Diga aos usuários como instalar
</h3>

Diga aos seus usuários para adicionar o marketplace e depois instalar o plugin a partir de seu shell, substituindo a fonte e os nomes pelos seus:

* Adicione o marketplace uma vez: `claude plugin marketplace add your-org/your-marketplace`, onde o argumento é um atalho GitHub `owner/repo`, uma URL ou um caminho
* Instale o plugin: `claude plugin install deploy-helper@your-marketplace`
* Ou faça ambos de dentro de uma sessão: `/plugin install deploy-helper --marketplace your-org/your-marketplace`. Requer Claude Code v2.1.275 ou posterior. Veja [Adicione um marketplace e instale em um comando](/docs/pt/plugins/install#add-a-marketplace-and-install-in-one-command)

<h3 id="ship-updates-to-users">
  Envie atualizações aos usuários
</h3>

Os usuários recebem um lançamento quando o solicitam ou quando a auto-atualização está ativada para seu marketplace:

* **Sob demanda**: `claude plugin update deploy-helper@your-marketplace` no shell do usuário atualiza o marketplace e instala a nova cópia quando a versão do seu plugin foi alterada
* **Auto-atualização**: desativada por padrão para seu marketplace. Veja [Ative a auto-atualização](/docs/pt/plugins/host-marketplace#turn-on-auto-update). Uma vez ativada, ela faz o mesmo que `claude plugin update` com um atraso após o início da sessão

[Instale plugins](/docs/pt/plugins/install) cobre os comandos do lado do usuário, e [quando a auto-atualização é executada](/docs/pt/plugins/loading#when-auto-update-runs) cobre o tempo.

<h2 id="submit-to-the-community-marketplace">
  Envie para o marketplace da comunidade
</h2>

O marketplace da comunidade da Anthropic, `claude-community`, é o marketplace público que lista plugins enviados através do formulário de envio do diretório de plugins.

Os usuários adicionam o marketplace da comunidade em uma sessão Claude Code com `/plugin marketplace add anthropics/claude-plugins-community` e instalam a partir dele como `@claude-community`.

Para saber como o marketplace da comunidade difere do marketplace oficial, veja [Marketplaces da Anthropic](/docs/pt/plugins/anthropic-marketplaces).

Para enviar seu plugin para o marketplace da comunidade, use um dos formulários no aplicativo:

* **claude.ai**: [claude.ai/admin-settings/directory/submissions/plugins/new](https://claude.ai/admin-settings/directory/submissions/plugins/new)
* **Console**: [platform.claude.com/plugins/submit](https://platform.claude.com/plugins/submit)

O formulário claude.ai requer uma organização Team ou Enterprise e a permissão Directory, que os Owners possuem por padrão. Autores individuais que não fazem parte de uma organização Team ou Enterprise podem usar o formulário Console.

Em seu shell, execute `claude plugin validate ./your-plugin` localmente antes de enviar, substituindo `./your-plugin` pelo caminho para seu diretório de plugin. Quando a validação passa, Claude Code imprime `✔ Validation passed`, ou `✔ Validation passed with warnings` se houver avisos. Avisos não falham na validação; adicione `--strict` para tratá-los como erros.

Os plugins listados aparecem no catálogo [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community), em quase todos os casos fixados a um SHA de commit específico.

Pode haver um atraso entre o envio e seu plugin aparecer em `marketplace.json`. Para verificar se seu plugin já é instalável, procure por seu nome no [catálogo da comunidade](https://github.com/anthropics/claude-plugins-community/blob/main/.claude-plugin/marketplace.json).

O marketplace oficial, `claude-plugins-official`, não aceita envios através desses formulários. Se você trabalha com um contato de parceiro da Anthropic, pergunte-lhes sobre uma listagem no marketplace oficial.

<h2 id="ship-updates-renames-and-removals">
  Envie atualizações, renomeações e remoções
</h2>

<h3 id="release-a-new-version">
  Libere uma nova versão
</h3>

Se você publicar através de seu próprio marketplace e seu `plugin.json` definir `version`, incremente-o e faça push. Os usuários que executam `claude plugin update` ou têm auto-atualização ativada recebem a nova versão, conforme descrito em [Envie atualizações aos usuários](#ship-updates-to-users).

<h3 id="tag-a-release">
  Marque um lançamento
</h3>

Marque o lançamento em git quando outros plugins declaram um intervalo de versão no seu, porque esses intervalos se resolvem contra tags. Caso contrário, você não precisa de uma tag.

Para marcar, execute `claude plugin tag` em seu shell a partir do diretório do plugin. Ele cria uma tag `{name}--v{version}`. Adicione `--push` para enviar a tag para `origin`. A [referência `plugin tag`](/docs/pt/plugins/cli-reference#plugin-tag) lista seus sinalizadores.

<h3 id="rename-or-remove-a-plugin">
  Renomeie ou remova um plugin
</h3>

Nunca altere o `name` de um plugin publicado. Após uma renomeação, os usuários que já o instalaram perdem o plugin, porque sua instalação é registrada sob o nome antigo. Uma entrada `renames` em seu arquivo de marketplace os migra. Altere `displayName` quando quiser um rótulo diferente.

Se uma renomeação for inevitável, use o mapa `renames` do arquivo de marketplace para que as instalações existentes migrem em vez de falhar com [`Plugin "<name>" not found in marketplace`](/docs/pt/plugins/troubleshooting#plugin-not-found-in-marketplace). Para remover um plugin do marketplace ou para os detalhes completos de `renames`, veja [Renomeie ou remova um plugin](/docs/pt/plugins/host-marketplace#rename-or-remove-a-plugin) na página de hospedagem. A [referência de marketplace](/docs/pt/plugins/marketplace-reference#top-level-fields) tem o campo.

<h2 id="declare-dependencies">
  Declare dependências
</h2>

Se seu plugin precisa que outro plugin do mesmo marketplace seja habilitado, liste-o no array `dependencies` de `plugin.json`. Cada entrada é um nome simples ou um objeto com um intervalo de semver `version`. Quando um usuário instala seu plugin, Claude Code instala e habilita a dependência também.

[Dependências de plugin](/docs/pt/plugins/dependencies) cobre a sintaxe de intervalo, dependências entre marketplaces e como os usuários removem dependências que não precisam mais.

<h2 id="next-steps">
  Próximas etapas
</h2>

* [Hospede e mantenha um marketplace](/docs/pt/plugins/host-marketplace): libere novas versões e mantenha os usuários atualizados
* [Dependências de plugin](/docs/pt/plugins/dependencies): declare e versione os plugins dos quais o seu depende
* [Recomende seu plugin a partir de sua CLI](/docs/pt/plugins/cli-hints): solicite aos usuários Claude Code de sua CLI que instalem o plugin
* [Meça o custo e o uso do plugin](/docs/pt/plugins/measure): veja quanto seu plugin custa em contexto e se as pessoas o usam
