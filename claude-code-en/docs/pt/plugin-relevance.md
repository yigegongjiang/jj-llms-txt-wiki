> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Recomende plugins para sua organização

> Adicione um bloco de relevância às entradas de plugins do marketplace para que Claude Code os sugira quando o trabalho de um usuário corresponder.

Se você opera um marketplace de plugins para sua organização, pode fazer com que Claude Code sugira plugins específicos aos usuários com base no que estão trabalhando. Adicione um bloco `relevance` à entrada de um plugin em `marketplace.json`, depois coloque o marketplace na lista de permissões nas configurações gerenciadas. Quando a sessão de um usuário corresponder a um dos sinais declarados, Claude Code exibe uma sugestão de instalação para esse plugin.

As sugestões declaradas pelo marketplace são opcionais por marketplace através das [configurações gerenciadas](/docs/pt/settings#settings-files). Nenhuma declaração de `relevance` de um marketplace produz sugestões até que um administrador a adicione à lista de permissões, incluindo o marketplace oficial da Anthropic. Claude Code também inclui uma sugestão integrada que é independente dessa lista de permissões; essa dica e todas as dicas declaradas pelo marketplace são desabilitadas quando [`spinnerTipsEnabled`](/docs/pt/settings#available-settings) é definido como `false`.

Este recurso requer Claude Code v2.1.152 ou posterior. Clientes mais antigos ignoram o campo `relevance`.

Esta página é para operadores de marketplace e administradores corporativos. Se você está procurando instalar plugins, consulte [Descobrir e instalar plugins](/docs/pt/discover-plugins).

<h2 id="how-it-works">
  Como funciona
</h2>

Cada entrada de plugin em `marketplace.json` pode conter um objeto `relevance`. O objeto nomeia um tópico e um ou mais sinais. Um sinal é um padrão que Claude Code testa contra a sessão atual, como o diretório de trabalho ou arquivos que Claude leu.

A correspondência de sinais acontece localmente na máquina do usuário. A correspondência não adiciona tráfego de rede e não relata quais sinais corresponderam, ou seus valores, à Anthropic ou ao operador do marketplace.

Quando um sinal corresponde e o plugin ainda não está instalado, Claude Code mostra o plugin em três lugares:

* **Dica do spinner**: uma mensagem "Trabalhando com *tópico*? Instale o plugin *plugin*" com o comando `/plugin install` aparece abaixo do spinner enquanto Claude está respondendo.
* **Sugestão de início de sessão**: se o sinal `cwd` corresponder ao diretório de trabalho, uma notificação de uma linha `plugin suggestion: <name>@<marketplace> · /plugin` aparece antes do primeiro turno. Esta superfície requer Claude Code v2.1.153 ou posterior.
* **Aba Discover do `/plugin`**: o plugin é fixado no topo da lista Discover com uma anotação como "sugerido para este diretório" ou "sugerido para comandos stripe". Esta superfície requer Claude Code v2.1.154 ou posterior.

A dica do spinner e a notificação de início de sessão fazem parte do sistema de dicas do spinner. Ambas são desabilitadas quando o usuário ou projeto define `spinnerTipsEnabled` como `false`, ou quando um `spinnerTipsOverride` personalizado é configurado com `excludeDefault`. O pino da aba Discover é independente das configurações de dicas.

Claude Code nunca instala um plugin automaticamente. O usuário sempre confirma.

<h2 id="add-relevance-to-a-plugin-entry">
  Adicione relevância a uma entrada de plugin
</h2>

Adicione um objeto `relevance` à entrada do plugin em seu `marketplace.json`. O exemplo a seguir declara que o plugin `terraform-helpers` é relevante quando Claude lê um arquivo `.tf` ou quando Claude executa `terraform`:

```json theme={null}
{
  "name": "acme-corp-plugins",
  "owner": { "name": "Acme Platform Team" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Acme conventions and helpers for Terraform",
      "relevance": {
        "topic": "Terraform",
        "signals": {
          "cli": ["terraform"],
          "filesRead": ["**/*.tf"]
        }
      }
    }
  ]
}
```

Um plugin com um bloco `relevance` mas sem sinal correspondente se comporta como qualquer outra entrada do marketplace. Ele aparece na lista Discover em sua posição normal e nunca aparece como uma dica do spinner.

<h2 id="field-reference">
  Referência de campos
</h2>

<h3 id="relevance">
  `relevance`
</h3>

| Campo     | Tipo   | Descrição                                                                                                                                                                                                                                                                                                                                                                     |
| :-------- | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `topic`   | string | Opcional. A frase que preenche "Trabalhando com *tópico*?" na dica do spinner. Geralmente o nome do produto, por exemplo `Stripe`. Use um domínio como `design` quando o nome do plugin não se lê naturalmente como um tópico. Padrão é o nome do plugin com cada segmento de hífen capitalizado. A notificação de início de sessão não usa este valor. Máximo 64 caracteres. |
| `signals` | object | Correspondentes que determinam quando o plugin é relevante. Pelo menos um sinal é necessário para que o plugin seja sugerível. Veja a tabela abaixo.                                                                                                                                                                                                                          |

<h3 id="relevance-signals">
  `relevance.signals`
</h3>

| Campo          | Tipo             | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :------------- | :--------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cwd`          | array of strings | Padrões Glob correspondidos contra o diretório de trabalho da sessão. Correspondido como um caminho absoluto e, quando dentro de um repositório git, como um caminho relativo à raiz do repositório. Normalizado com barra invertida e insensível a maiúsculas/minúsculas. Cada padrão corresponde ao diretório em si e a tudo sob ele, então `infra`, `infra/`, e `infra/**` se comportam de forma idêntica. Este é o único sinal que pode corresponder no início da sessão, antes do primeiro turno. Máximo 10 padrões de 256 caracteres cada.                                                                                                                                                                                                                                                                                                                                                                                                                |
| `cli`          | array of strings | Nomes de comando de comandos shell que Claude executou nesta sessão, por exemplo `["stripe"]`. Aplica-se em todas as plataformas: comandos executados no Windows através do PowerShell ou Git Bash são registrados da mesma forma. Claude Code registra um nome de comando por invocação de ferramenta shell: o primeiro token após qualquer atribuição de variável de ambiente inicial e `sudo`. Comandos compostos contribuem apenas com seu comando inicial, então `cd infra && terraform plan` registra `cd`, não `terraform`. Correspondência exata. Máximo 10 entradas de 64 caracteres cada.                                                                                                                                                                                                                                                                                                                                                             |
| `hosts`        | array of strings | Nomes de host vistos em URLs `http://` ou `https://` em comandos Bash nesta sessão, por exemplo `["api.stripe.com"]`. Apenas nome de host em minúsculas: sem esquema, porta ou caminho. Correspondência exata insensível a maiúsculas/minúsculas. Máximo 20 entradas de 128 caracteres cada.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `filesRead`    | array of strings | Padrões Glob correspondidos contra os caminhos de arquivos que Claude leu nesta sessão, por exemplo `["**/*.tf"]`. Normalizado com barra invertida e insensível a maiúsculas/minúsculas. Máximo 10 padrões de 256 caracteres cada.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `manifestDeps` | array of objects | Dependências declaradas em manifestos de pacote que Claude leu nesta sessão. Cada entrada é `{ "file": "...", "pattern": "..." }`, onde `file` é uma expressão regular correspondida contra o caminho do arquivo de manifesto conforme registrado no estado da sessão, normalmente um caminho absoluto, e `pattern` é uma expressão regular correspondida contra o conteúdo desse arquivo. Âncora `file` no final, por exemplo `[/\\\\]package\\.json$` em forma com escape JSON, porque um padrão ancorado no início nunca corresponde a um caminho absoluto. Os caminhos não são normalizados por separador para este sinal, então os caminhos do Windows usam barras invertidas. Arquivos de manifesto maiores que 512 KB são ignorados. Ambos os valores são strings de origem `RegExp` do JavaScript de no máximo 256 caracteres. `file` corresponde insensível a maiúsculas/minúsculas. `pattern` é sensível a maiúsculas/minúsculas. Máximo 10 entradas. |

Os sinais `cli`, `hosts`, `filesRead` e `manifestDeps` precisam de histórico de sessão, então eles só podem corresponder na dica do spinner e na aba Discover. Apenas `cwd` pode corresponder no início da sessão. Os sinais `filesRead` e `manifestDeps` testam o estado de arquivo registrado da sessão, que também inclui arquivos que Claude escreveu ou editou e arquivos de memória `CLAUDE.md` carregados automaticamente.

O exemplo a seguir usa `manifestDeps` para sugerir um plugin Stripe uma vez que Claude leu um `package.json` que depende de `stripe`. O padrão `file` usa `[/\\\\]` para que corresponda tanto a separadores de caminho com barra invertida quanto com barra invertida, e `\\.` para que o ponto seja literal. Em JSON, cada barra invertida na expressão regular é escrita duas vezes.

```json theme={null}
{
  "name": "stripe-helpers",
  "source": "./plugins/stripe-helpers",
  "relevance": {
    "topic": "Stripe",
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"stripe\"\\s*:"
        }
      ]
    }
  }
}
```

<Note>
  Campos desconhecidos sob `relevance` e `relevance.signals` são ignorados no tempo de carregamento para que clientes Claude Code mais antigos continuem carregando seu marketplace. Execute `claude plugin validate` para exibi-los como avisos.
</Note>

<h2 id="enable-suggestions-in-managed-settings">
  Ative sugestões nas configurações gerenciadas
</h2>

Declarar `relevance` em `marketplace.json` não é suficiente por si só. Um administrador deve colocar o marketplace na lista de permissões nas [configurações gerenciadas](/docs/pt/settings#settings-files) antes que suas sugestões apareçam aos usuários.

Adicione o nome do marketplace a `pluginSuggestionMarketplaces`. Para qualquer marketplace que não seja o marketplace oficial da Anthropic, também declare a fonte do marketplace nas mesmas configurações gerenciadas, seja como entrada desse nome em `extraKnownMarketplaces` ou como entrada em `strictKnownMarketplaces`. O nome colocado na lista de permissões é ignorado se o marketplace registrado na máquina veio de uma fonte diferente. Isso impede que uma fonte não relacionada se registre sob um nome colocado na lista de permissões para ter seus plugins sugeridos em toda sua organização.

O `managed-settings.json` a seguir registra um marketplace de organização de um repositório GitHub e ativa suas sugestões:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-corp-plugins": {
      "source": {
        "source": "github",
        "repo": "acme-corp/claude-plugins"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

O marketplace oficial está isento do requisito de declaração de fonte porque seu nome só pode ser registrado da fonte oficial da Anthropic. Colocar apenas o nome na lista de permissões é suficiente:

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

Consulte a [referência de configurações](/docs/pt/settings) para `pluginSuggestionMarketplaces` e [`extraKnownMarketplaces`](/docs/pt/settings#extraknownmarketplaces) para detalhes completos de configuração.

<h2 id="what-the-user-sees">
  O que o usuário vê
</h2>

Quando um sinal corresponde durante uma sessão, a dica do spinner lê:

```text theme={null}
Working with Terraform? Install the terraform-helpers plugin:
/plugin install terraform-helpers@acme-corp-plugins
```

No início da sessão, um sinal `cwd` correspondente exibe a notificação de uma linha:

```text theme={null}
plugin suggestion: terraform-helpers@acme-corp-plugins · /plugin
```

A sugestão de um determinado plugin aparece no máximo uma vez a cada três sessões entre a dica do spinner e a notificação de início de sessão combinadas, e nenhuma se repete uma vez que o plugin está instalado. A notificação de início de sessão também para de aparecer após a sugestão ter sido mostrada duas vezes.

Na aba Discover do `/plugin`, o plugin é fixado acima dos outros resultados com uma anotação que nomeia o sinal correspondente, como `suggested for this directory` ou `suggested for terraform commands`. A aba Discover fixa um determinado plugin uma vez; visitas posteriores o listam em ordem normal. O pino da aba Discover requer Claude Code v2.1.154 ou posterior. Na v2.1.152 apenas a dica do spinner aparece; a notificação de início de sessão é adicionada na v2.1.153.

<h2 id="validate-your-marketplace">
  Valide seu marketplace
</h2>

Execute `claude plugin validate` contra seu diretório de marketplace para verificar o bloco `relevance` antes de publicar:

```
claude plugin validate ./my-marketplace
```

O validador relata chaves desconhecidas sob `relevance` e `relevance.signals` como avisos, sinaliza um valor `relevance` que não é um objeto, e rejeita uma entrada `signals.hosts` que inclui um esquema, porta ou caminho.

<h2 id="see-also">
  Veja também
</h2>

* [Crie e distribua um marketplace de plugins](/docs/pt/plugin-marketplaces): construa o marketplace que hospeda seus plugins
* [Recomende seu plugin a partir de sua CLI](/docs/pt/plugin-hints): solicite aos usuários a partir de sua própria CLI em vez de dos sinais de sessão do Claude Code
* [Configurações](/docs/pt/settings): referência completa para `pluginSuggestionMarketplaces` e `extraKnownMarketplaces`
