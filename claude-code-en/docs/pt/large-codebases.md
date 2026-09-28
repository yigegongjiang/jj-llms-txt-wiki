> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar Claude Code em um monorepo ou grande base de código

> Configure Claude Code para monorepos e grandes bases de código de árvore única com arquivos CLAUDE.md aninhados, worktrees esparsos, inteligência de código e skills por pacote para que Claude permaneça focado no código em que você está trabalhando.

Uma grande base de código pode ser um repositório com milhões de linhas ou um monorepo com muitos pacotes. Claude Code funciona em qualquer tamanho, mas conforme a base de código cresce, os padrões ajustados para projetos menores podem preencher a janela de contexto com instruções e leituras de arquivo não relacionadas à tarefa, custando tokens e degradando o desempenho do Claude.

Este guia mostra a desenvolvedores individuais e equipes de engenharia como escopar Claude para a parte da base de código que uma tarefa toca. Cada seção observa se uma configuração é pessoal à sua máquina ou confirmada no repositório.

<h2 id="what-this-guide-covers">
  O que este guia cobre
</h2>

A [tabela abaixo](#settings-on-this-page) lista cada configuração e o que ela realiza. A [árvore de arquivos depois dela](#the-example-monorepo) é o monorepo de exemplo ao qual cada amostra de código nesta página se refere.

<h3 id="settings-on-this-page">
  Configurações nesta página
</h3>

Cada configuração abaixo é independente. Elas se sobrepõem em vez de se substituírem, então aplique aquelas que se adequarem ao seu repositório. [Escolha onde iniciar Claude](#choose-where-to-start-claude) determina onde seus arquivos de configuração vivem, então leia primeiro. [Junte tudo](#put-it-together) mostra todos eles combinados.

| Eu quero                                                                                                        | Usar                                                                                          |
| :-------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------- |
| Carregar apenas as convenções para o código que você toca, em vez de um arquivo raiz cobrindo cada subsistema   | Arquivos [CLAUDE.md](#layer-claude-md-files-by-directory) por diretório                       |
| Excluir arquivos CLAUDE.md para pacotes em que você nunca trabalha                                              | [`claudeMdExcludes`](#exclude-irrelevant-claude-md-files)                                     |
| Impedir que Claude abra saída de compilação, código gerado e dependências fornecidas                            | Regras de negação [`Read`](#block-reads-of-generated-and-vendored-code) em `permissions.deny` |
| Encontrar a definição de um símbolo ou chamadores através do servidor de linguagem em vez de verificar arquivos | Um [plugin de inteligência de código](#reduce-file-reads-with-code-intelligence)              |
| Verificar apenas os diretórios que uma tarefa precisa quando Claude cria um worktree                            | [`worktree.sparsePaths`](#check-out-only-the-directories-you-need)                            |
| Ler e editar um pacote irmão ou outro repositório da mesma sessão                                               | [`--add-dir`](#grant-access-across-packages-or-repositories) ou `additionalDirectories`       |
| Dar ao Claude procedimentos específicos de uma área que carregam apenas quando relevante                        | [Skills](#add-per-directory-skills) por diretório                                             |
| Substituir muitos arquivos CLAUDE.md por diretório com um conjunto de convenções que todos instalam             | Um [plugin](#centralize-conventions-when-layering-stops-scaling) em um marketplace interno    |

<Tip>
  Para técnicas de fluxo de trabalho que mantêm o contexto pequeno em qualquer repositório, como [executar exploração em um subagente](/docs/pt/best-practices#use-subagents-for-investigation) para que leituras de arquivo fiquem fora da conversa principal, veja [Melhores práticas para Claude Code](/docs/pt/best-practices). Para distribuir uma configuração de linha de base para cada desenvolvedor em sua organização, veja [Configurar Claude Code para sua organização](/docs/pt/admin-setup).
</Tip>

<h3 id="the-example-monorepo">
  O monorepo de exemplo
</h3>

Os exemplos ao longo desta página se referem a um monorepo com três pacotes. Os mesmos padrões funcionam em uma grande base de código de árvore única: onde um exemplo usa `packages/api/`, substitua seu próprio diretório de subsistema como `src/backend/` ou `lib/core/`.

```text theme={null}
monorepo/
  CLAUDE.md                     # instruções raiz
  packages/
    api/
      CLAUDE.md                 # instruções específicas da API
      .claude/skills/
      src/
    web/
      CLAUDE.md                 # instruções específicas do frontend
      .claude/skills/
      src/
    shared/
      CLAUDE.md                 # instruções da biblioteca compartilhada
      src/
```

<h2 id="choose-where-to-start-claude">
  Escolha onde iniciar Claude
</h2>

Onde você inicia `claude` determina quais arquivos Claude pode ler e editar sem uma concessão de permissão adicional, quais arquivos CLAUDE.md carregam no contexto na inicialização e quais configurações de projeto se aplicam.

| Iniciar de          | Acesso a arquivos                             | CLAUDE.md carregado na inicialização                                           | Use quando                                        |
| :------------------ | :-------------------------------------------- | :----------------------------------------------------------------------------- | :------------------------------------------------ |
| Raiz do repositório | Cada arquivo                                  | Apenas raiz; arquivos de subdiretório carregam sob demanda quando Claude lê lá | Tarefas abrangem múltiplos pacotes ou subsistemas |
| Um subdiretório     | Apenas essa subárvore, até você conceder mais | Desse diretório mais cada ancestral                                            | O trabalho é escopo de um pacote ou subsistema    |

As configurações de projeto em `.claude/settings.json` não são herdadas de diretórios pai da maneira que os arquivos CLAUDE.md são. Para saber qual diretório `.claude/settings.json` uma sessão lê, consulte [onde Claude Code procura por cada arquivo](/docs/pt/settings#where-claude-code-looks-for-each-file).

Cada seção abaixo afirma se seu arquivo de configurações pertence à raiz do repositório ou ao subdiretório do qual você inicia, e se é confirmado ou mantido localmente.

<h2 id="layer-claude-md-files-by-directory">
  Camadas de arquivos CLAUDE.md por diretório
</h2>

Em uma grande base de código, um único CLAUDE.md na raiz do repositório tende a crescer para cobrir as convenções de cada subsistema, custando contexto em instruções não relacionadas à tarefa atual, ou permanecer muito genérico para ser útil. Dividir instruções entre arquivos por diretório significa que Claude carrega regras em toda a repositório mais apenas as convenções para o código em que você está trabalhando.

Claude Code carrega cada arquivo [CLAUDE.md](/docs/pt/memory) do seu diretório de trabalho e cada diretório pai na inicialização, depois carrega o arquivo de cada subdiretório sob demanda quando lê arquivos lá. Um arquivo raiz define regras em toda a repositório e cada subdiretório adiciona a sua própria.

Uma divisão comum é dois níveis:

* **CLAUDE.md raiz**: instruções que se aplicam em todos os lugares, como padrões de codificação e convenções de commit
* **CLAUDE.md por subdiretório**: convenções específicas da pilha dessa área. Em um monorepo é um por pacote. Em uma grande árvore única é um por subsistema como `src/db/` ou `src/api/`

Confirme esses arquivos no repositório para que colegas de trabalho os herdem. O proprietário de cada diretório normalmente mantém seu arquivo.

Para aparar um arquivo que já está verificado, execute o [checkup `/doctor`](/docs/pt/memory#my-claude-md-is-too-large). O CLAUDE.md raiz contém as regras que se aplicam em cada pacote:

```markdown CLAUDE.md theme={null}
Execute scripts de pacote do diretório do pacote, não da raiz do monorepo.
Prefixe assuntos de commit com o nome do pacote, por exemplo `api: add rate limiting`.
Nunca edite arquivos sob packages/*/generated/. Execute `npm run codegen` no pacote em vez disso.
```

O CLAUDE.md de cada subdiretório, aqui `packages/api/CLAUDE.md`, adiciona as convenções específicas dessa área:

```markdown packages/api/CLAUDE.md theme={null}
Copie `.env.example` para `.env` antes de executar qualquer coisa. Testes e o servidor dev falham sem ele.
Escreva consultas de banco de dados com o construtor de consultas Knex. Nunca coloque strings SQL brutas em manipuladores de rota.
Nunca edite uma migração depois que ela foi mesclada. Adicione uma nova migração em vez disso.
```

Quando você inicia Claude de `packages/api/`, ele carrega tanto `packages/api/CLAUDE.md` quanto o CLAUDE.md raiz. Claude vê as instruções locais ao lado das regras em toda a repositório, sem instruções de `packages/web/` no contexto. O mesmo vale para qualquer subdiretório em uma árvore não-monorepo. Para confirmar quais arquivos carregaram, execute `/context` e verifique a lista sob **Memory files**.

Algumas maneiras de manter os arquivos atualizados conforme a base de código e os modelos mudam:

* **Revisar em pull requests**: trate edições CLAUDE.md como qualquer outra mudança de documentação para que as convenções rastreiem o código
* **Revisitar após lançamentos de modelo principais**: instruções que funcionavam em torno de uma limitação de um modelo mais antigo podem se tornar sobrecarga uma vez que um modelo mais novo lida com o caso por conta própria. Por exemplo, uma regra que força refatorações de arquivo único pode ser deletada uma vez que a limitação se foi
* **Adicionar um hook Stop que propõe atualizações**: um [`Stop` hook](/docs/pt/hooks#stop) recebe o caminho para a transcrição da sessão quando Claude termina de responder, então um script pode revisar a sessão e propor atualizações CLAUDE.md enquanto a lacuna que expôs é fresca

Para mais sobre como os arquivos CLAUDE.md carregam e interagem, veja [Memória e instruções de projeto](/docs/pt/memory).

<h3 id="choose-between-per-directory-claude-md-and-path-scoped-rules">
  Escolha entre CLAUDE.md por diretório e regras com escopo de caminho
</h3>

Arquivos CLAUDE.md por diretório e [regras com escopo de caminho](/docs/pt/memory#path-specific-rules) sob `.claude/rules/` ambos deixam você direcionar instruções para parte da árvore. Eles diferem em onde o arquivo vive e quando carrega.

| Abordagem                                       | Localização do arquivo                     | Carrega quando                                                                                  | Use quando                                                                                          |
| :---------------------------------------------- | :----------------------------------------- | :---------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- |
| CLAUDE.md por diretório                         | Dentro do diretório, ao lado de seu código | Na inicialização quando iniciado desse diretório, ou sob demanda quando Claude lê um arquivo lá | Proprietários de diretório mantêm suas próprias convenções; instruções são versionadas com o código |
| Regra com escopo de caminho em `.claude/rules/` | `.claude/` central na raiz do repo         | Quando Claude trabalha com um arquivo correspondendo ao glob `paths:` da regra                  | Você quer todas as convenções em um lugar, ou a mesma regra se aplica a muitos caminhos espalhados  |

Para uma comparação que também cobre skills, veja [Comparar recursos similares](/docs/pt/features-overview#compare-similar-features).

<h3 id="exclude-irrelevant-claude-md-files">
  Excluir arquivos CLAUDE.md irrelevantes
</h3>

Quando você inicia Claude da raiz do repositório, cada CLAUDE.md de subdiretório carrega assim que Claude lê um arquivo nesse diretório. A configuração `claudeMdExcludes` pula arquivos específicos por caminho ou padrão glob para que nunca carreguem.

Use isso para diretórios em que você nunca trabalha, como pacotes de outras equipes, código legado ou subárvores fornecidas. A lista de exclusão é estática, não um switch por tarefa. Para focar em um pacote hoje e outro amanhã, [inicie Claude do diretório desse pacote](#choose-where-to-start-claude) em vez de editar exclusões.

Se você quer apenas essas exclusões para você, coloque a configuração em `.claude/settings.local.json`. Claude Code adiciona esse arquivo ao seu gitignore global quando salva uma configuração lá. Como você está criando manualmente aqui, adicione-o ao seu gitignore você mesmo. Os padrões usam sintaxe glob correspondida contra caminhos de arquivo absolutos, então comece padrões de estilo relativo com `**/` para corresponder em qualquer lugar da árvore. O exemplo abaixo exclui um pacote pertencente a outra equipe:

```json .claude/settings.local.json theme={null}
{
  "claudeMdExcludes": [
    "**/packages/web/**"
  ]
}
```

Isso pula cada CLAUDE.md e arquivo de regras sob esse pacote. O CLAUDE.md raiz e os pacotes em que você trabalha ainda carregam normalmente.

Esses padrões cobrem outros casos comuns:

* `"**/packages/*/CLAUDE.md"`: exclui o CLAUDE.md de cada pacote enquanto mantém a raiz
* `"**/packages/legacy-*/**"`: exclui cada pacote cujo nome corresponde ao glob, incluindo regras
* `"/home/user/monorepo/legacy/CLAUDE.md"`: exclui um arquivo específico por caminho absoluto

Arquivos CLAUDE.md de política gerenciada não podem ser excluídos, então instruções em toda a organização sempre se aplicam. Você pode definir `claudeMdExcludes` em qualquer [escopo de configurações](/docs/pt/settings#where-settings-live): usuário, projeto, local ou gerenciado. Os arrays se mesclam entre escopos, então uma equipe pode definir padrões em nível de projeto enquanto indivíduos adicionam substituições locais.

Para a documentação completa de exclusão, veja [Excluir arquivos CLAUDE.md específicos](/docs/pt/memory#exclude-specific-claude-md-files).

<h2 id="reduce-what-claude-reads">
  Reduza o que Claude lê
</h2>

As instruções são apenas parte do que acaba no contexto do Claude. As leituras de arquivo são outro custo que cresce com a base de código. As configurações abaixo bloqueiam leituras de caminhos irrelevantes e substituem varreduras exaustivas de arquivos por pesquisas de servidor de linguagem.

<h3 id="block-reads-of-generated-and-vendored-code">
  Bloqueie leituras de código gerado e fornecido
</h3>

As pesquisas de conteúdo do Claude respeitam `.gitignore` por padrão, portanto, caminhos já listados lá, como `node_modules/`, `dist/` e `build/`, ficam fora dos resultados de pesquisa sem configuração adicional.

Para caminhos que são verificados, como um SDK fornecido ou código gerado confirmado, adicione regras de negação `Read` em `permissions.deny` para impedir que Claude abra esses arquivos.

As regras de negação podem cobrir todos que trabalham no repositório, apenas você ou cada sessão na máquina, dependendo de qual arquivo de configurações você as colocar:

* **Todos que trabalham no repositório**: confirme as regras em `.claude/settings.json`, na raiz do repositório se você iniciar o Claude lá, ou em cada `.claude/` do pacote se você iniciar a partir de subdiretórios. Como outras configurações de projeto nesta página, esse arquivo não é herdado de diretórios pai.
* **Apenas você**: use `.claude/settings.local.json` na raiz do repositório, que carrega em cada sessão CLI dentro do repositório independentemente do diretório inicial, exceto nos casos em que Claude Code [não usa a raiz do repositório](/docs/pt/settings#where-claude-code-looks-for-each-file), como no Windows. Padrões relativos como o `Read(./**/vendor/**/*)` do exemplo ainda [ancoram no diretório de trabalho atual da sessão](/docs/pt/permissions#read-and-edit) em vez da raiz do repositório, portanto, se você iniciar sessões a partir de subdiretórios, escreva as regras neste arquivo como caminhos absolutos `//`, como `Read(//absolute/path/to/repo/**/vendor/**/*)`. Antes da v2.1.211, `.claude/settings.local.json` também carregava apenas a partir do diretório inicial.
* **Todos, aplicado em cada sessão**: defina as regras em [configurações gerenciadas](/docs/pt/managed-settings), que as configurações de usuário e projeto não podem substituir.

O exemplo abaixo bloqueia artefatos de compilação e um SDK fornecido. Seus padrões de diretório terminam com `/**/*` em vez de `/**` para que cada regra cubra tudo dentro do diretório, mas não o próprio diretório. Claude pode então ainda listar esses diretórios ou entrar neles, por exemplo com `ls dist` ou `cd build`.

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**/*)",
      "Read(./**/build/**/*)",
      "Read(./**/*.generated.*)",
      "Read(./**/vendor/**/*)"
    ]
  }
}
```

As regras de negação cobrem as ferramentas de arquivo integradas do Claude. Em Bash, elas cobrem os comandos de arquivo que Claude Code reconhece, como `cat`, `head`, `grep` e `find`, quando um caminho negado aparece como um argumento, e o alvo de um [redirecionamento](/docs/pt/permissions#redirections) como `< file`. Claude Code também faz uma tentativa de melhor esforço para manter caminhos negados fora dos resultados das ferramentas Grep e Glob integradas. Uma pesquisa Bash como `grep -r` ou `find` em um diretório que contém arquivos negados ainda os inclui em sua saída.

As regras de negação não cobrem subprocessos que abrem arquivos por conta própria. Para a sintaxe de padrão completa, consulte [Regras de permissão Read e Edit](/docs/pt/permissions#read-and-edit).

<h3 id="reduce-file-reads-with-code-intelligence">
  Reduza leituras de arquivo com inteligência de código
</h3>

Em uma base de código grande, encontrar onde um símbolo é definido ou usado pode custar muitas leituras de arquivo e chamadas grep. [Plugins de inteligência de código](/docs/pt/plugins/code-intelligence) conectam Claude a um servidor de linguagem para que ele possa pular para definições, encontrar referências e exibir erros de tipo diretamente em vez de varrer a árvore.

O marketplace oficial tem plugins para TypeScript, Python, Go, Rust e outras linguagens comuns. Execute o comando abaixo dentro de uma sessão Claude Code para instalar o plugin TypeScript:

```shell theme={null}
/plugin install typescript-lsp@claude-plugins-official
```

Se a instalação falhar, corresponda à mensagem que Claude Code relata:

* `Marketplace "claude-plugins-official" not found`: adicione o marketplace com `/plugin marketplace add anthropics/claude-plugins-official`, depois tente novamente a instalação.
* O plugin [não foi encontrado no marketplace](/docs/pt/plugins/install#install-a-plugin): verifique o nome do plugin.

Para habilitar um plugin para todos no repositório em vez de instalá-lo você mesmo, adicione-o à [configuração de projeto `enabledPlugins`](/docs/pt/settings-reference#plugin-settings).

Os plugins de inteligência de código exigem o binário do servidor de linguagem da linguagem em cada máquina do desenvolvedor. Veja [qual binário cada linguagem exige](/docs/pt/plugins/code-intelligence). A instalação do marketplace oficial requer acesso à rede para GitHub, onde o marketplace é hospedado. Em uma rede restrita, [adicione o marketplace de um host Git interno ou caminho local](/docs/pt/plugins/install#add-a-marketplace) em vez disso.

Isso funciona bem com `claudeMdExcludes` e as regras de negação `Read` acima. Aqueles mantêm conteúdo irrelevante fora do contexto, e a inteligência de código impede que Claude leia o que permanece para localizar uma definição.

<h2 id="scope-worktrees-and-file-access">
  Escopo de worktrees e acesso a arquivos
</h2>

Essas configurações controlam o que está no disco em worktrees e quais diretórios Claude pode ler e escrever além do seu ponto de partida.

<h3 id="check-out-only-the-directories-you-need">
  Faça checkout apenas dos diretórios que você precisa
</h3>

A flag `--worktree` inicia uma sessão em uma nova git worktree para que as alterações fiquem isoladas do seu checkout principal. Por padrão, ela faz checkout de todo o repositório. Em um repositório grande, a configuração `worktree.sparsePaths` usa git sparse-checkout para escrever apenas os diretórios listados mais arquivos no nível raiz no disco, para que worktrees iniciem mais rapidamente e usem menos espaço.

Se todos que trabalham neste diretório precisam dos mesmos caminhos, confirme a configuração em `.claude/settings.json`. Para adicionar caminhos para você mesmo, use `.claude/settings.local.json`: as listas se mesclam entre escopos, então um arquivo local pode adicionar caminhos à lista confirmada, mas não removê-los.

Os exemplos JSON nesta página mostram uma configuração por vez. Se seu `.claude/settings.json` já contiver outras chaves, como as regras `permissions.deny` acima, adicione a chave `worktree` ao lado delas em vez de substituir o arquivo. [Coloque tudo junto](#put-it-together) mostra o resultado combinado.

O exemplo abaixo mostra o arquivo confirmado:

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ]
  }
}
```

Quando Claude cria uma worktree, ele faz checkout apenas de `.claude/`, `packages/api/` e `packages/shared/` em vez da árvore completa. Os caminhos em `sparsePaths` são relativos à raiz do repositório, independentemente de qual subdiretório você inicia Claude. Qualquer caminho de diretório funciona aqui, não apenas raízes de pacotes.

Isso é particularmente útil para [isolamento de worktree de subagentes](/docs/pt/worktrees#isolate-subagents-with-worktrees). Subagentes são instâncias paralelas de Claude geradas para subtarefas, e cada uma que é executada em uma worktree obtém um checkout leve em vez da árvore completa. Todas as worktrees em uma sessão compartilham o mesmo `sparsePaths`, então se um subagente precisa de `packages/api/` e outro precisa de `packages/web/`, liste ambos.

Liste diretórios em `sparsePaths`, não arquivos individuais. Arquivos no nível raiz como `package.json`, `tsconfig.base.json` e arquivos de lock são sempre feitos checkout ao lado dos diretórios que você lista. Diretórios no nível raiz não são, então inclua `.claude` na lista se você quiser que o `.claude/settings.json` da raiz do repositório ou `.claude/rules/` estejam disponíveis dentro da worktree. Para skills de projeto, agentes e comandos, veja [O que worktrees compartilham com o checkout principal](/docs/pt/worktrees#what-worktrees-share-with-the-main-checkout).

O sparse checkout requer que o git habilite `extensions.worktreeConfig` no `.git/config` compartilhado do repositório enquanto uma worktree esparsa existe. Claude Code remove essa entrada após a última worktree ser removida, mas apenas se Claude Code a adicionou. Nunca remove um valor que você definiu você mesmo. Antes da v2.1.207, a entrada permanecia após a última worktree ser removida, e ferramentas baseadas em go-git como `tea` falhavam ao abrir o repositório até que você executasse `git config --unset extensions.worktreeConfig`.

Para evitar duplicar diretórios grandes como `node_modules` entre worktrees, combine `sparsePaths` com `symlinkDirectories` no mesmo `.claude/settings.json`:

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  }
}
```

Isso cria um symlink de cada `node_modules/` da worktree de volta para a cópia do repositório principal em vez de duplicá-lo no disco.

<Note>
  As configurações `sparsePaths` e `symlinkDirectories` são lidas do seu diretório inicial antes da worktree ser criada. Após a criação, o diretório de trabalho da sessão é a raiz da worktree, não o subdiretório do qual você iniciou. As configurações de projeto dentro da worktree, portanto, carregam do `.claude/settings.json` da raiz da worktree, a cópia verificada do arquivo da raiz do repositório. Coloque qualquer outra configuração que você precise dentro de worktrees, como regras de permissão ou hooks, no `.claude/settings.json` da raiz do repositório.
</Note>

Para a referência completa de configurações de worktree, veja [Configurações de Worktree](/docs/pt/settings-reference#worktree).

<h3 id="grant-access-across-packages-or-repositories">
  Conceda acesso entre pacotes ou repositórios
</h3>

Esta seção se aplica quando você inicia Claude de um subdiretório, ou quando uma tarefa abrange múltiplos checkouts. Se você iniciar da raiz do repositório em uma única árvore grande, Claude já tem acesso a todos os arquivos e você pode pular isso.

Quando você inicia Claude de `packages/api/`, ele pode ler e escrever arquivos dentro desse diretório. Se uma tarefa requer alterações entre pacotes, como atualizar um tipo compartilhado que tanto `api` quanto `web` importam, você precisa conceder acesso ao diretório irmão. O mesmo mecanismo concede acesso a um repositório separadamente verificado.

A configuração `additionalDirectories` em `.claude/settings.json` dá a Claude acesso a diretórios fora do diretório de trabalho. O exemplo abaixo concede acesso a dois pacotes irmãos:

```json packages/api/.claude/settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": [
      "../shared",
      "../web"
    ]
  }
}
```

Os caminhos relativos são resolvidos em relação ao diretório do qual você inicia Claude. Com essa configuração, Claude pode ler e editar arquivos em `packages/shared/` e `packages/web/` enquanto trabalha de `packages/api/`.

Você também pode conceder acesso em tempo de execução sem editar configurações passando `--add-dir` quando você inicia Claude:

```bash theme={null}
claude --add-dir ../shared
```

Porém você adicione um diretório, Claude pode ler e editar arquivos nele. Se o CLAUDE.md do diretório, arquivos `.claude/rules/` e skills também carregam depende de como você o adicionou:

| Adicionado com                         | Carrega CLAUDE.md e regras               | Carrega skills |
| :------------------------------------- | :--------------------------------------- | :------------- |
| Configuração `additionalDirectories`   | Nunca                                    | Nunca          |
| Flag `--add-dir` ou comando `/add-dir` | Apenas com a variável de ambiente abaixo | Sim            |

Para carregar arquivos CLAUDE.md e rules de um diretório adicionado com `--add-dir` ou `/add-dir`, defina a variável de ambiente `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`:

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared
```

A variável de ambiente não tem efeito em diretórios listados na configuração `additionalDirectories`. Veja [Carregar de diretórios adicionais](/docs/pt/memory#load-from-additional-directories) para detalhes.

Para diretórios irmãos que todos nesta área precisam, confirme `additionalDirectories` em `.claude/settings.json`. Para uma seleção pessoal ou acesso único, use `.claude/settings.local.json` ou passe `--add-dir` no lançamento.

<h2 id="add-per-directory-skills">
  Adicione skills por diretório
</h2>

Qualquer subdiretório pode definir [skills](/docs/pt/skills) escopo para sua própria pilha. Uma skill carrega sob demanda quando Claude determina que é relevante, então ferramentas específicas da API não consomem contexto durante trabalho frontend.

As skills vivem sob `.claude/skills/` dentro do diretório. Confirme-as ao lado do código dessa área para que qualquer um que clone o repositório as obtenha. Em um monorepo isso pode ser um conjunto de skills por pacote. Em uma grande base de código de árvore única é um conjunto por subsistema como `src/db/.claude/skills/`.

Crie um diretório de skill dentro do subdiretório:

```bash theme={null}
mkdir -p packages/api/.claude/skills/api-testing
```

Então escreva `SKILL.md` dentro desse diretório, aqui `packages/api/.claude/skills/api-testing/SKILL.md`. Este exemplo ensina ao Claude os padrões de teste do pacote API:

```markdown packages/api/.claude/skills/api-testing/SKILL.md theme={null}
---
name: api-testing
description: Padrões de teste para o pacote API. Use ao escrever ou modificar testes em packages/api/.
---

## Estrutura de teste

Os testes estão em `src/__tests__/` espelhando a estrutura do diretório `src/`.
Cada arquivo de rota tem um arquivo `.test.ts` correspondente.

## Executando testes

- Todos os testes: `npm test`
- Arquivo único: `npm test -- src/__tests__/routes/users.test.ts`
- Modo watch: `npm test -- --watch`

## Utilitários de teste

- `src/__tests__/helpers/db.ts`: fornece `setupTestDb()` e `teardownTestDb()` para testes de banco de dados
- `src/__tests__/helpers/auth.ts`: fornece `createTestUser()` e `getAuthToken()` para endpoints autenticados

## Padrões

- Use `supertest` para asserções HTTP, não fetch bruto
- Sempre envolva testes de banco de dados em uma transação que reverte
- Mock de serviços externos em `src/__tests__/mocks/`
```

Um subdiretório diferente mantém diferentes skills da mesma maneira: `packages/web/.claude/skills/component-patterns/` descreve as convenções de componente do frontend em vez de teste. Quando Claude trabalha em um arquivo em `packages/api/`, ele carrega a skill api-testing. Quando trabalha em `packages/web/`, carrega component-patterns em vez disso. As skills de nenhum diretório carregam durante as tarefas do outro.

Você também pode escopar uma skill por padrão de arquivo em vez de por colocação. O [campo frontmatter `paths`](/docs/pt/skills#frontmatter-reference) leva padrões glob, e Claude carrega a skill automaticamente apenas quando trabalha com arquivos correspondentes. Use isso para uma skill que vive no `.claude/skills/` da raiz do repositório mas se aplica apenas a certos arquivos onde quer que apareçam, como uma skill de migração de banco de dados escopo para `**/migrations/**`.

Para mais sobre criar e organizar skills, veja [Skills](/docs/pt/skills).

<h3 id="keep-skills-discoverable">
  Mantenha skills descobríveis
</h3>

Com skills espalhadas por muitos diretórios, a lista da qual Claude escolhe pode crescer grande. Claude escolhe uma skill lendo o nome e descrição de cada skill descoberta, e apenas o conteúdo completo da skill escolhida carrega no contexto. Esta seção cobre como manter essa lista pequena.

Quais skills estão em escopo depende de onde você inicia Claude:

* **De um subdiretório como `packages/api/`**: skills desse diretório, cada pai até a raiz do repositório, e os níveis de usuário e empresa
* **Da raiz do repositório**: skills de raiz, mais skills de cada subdiretório que Claude toca durante a sessão, que pode acumular em centenas
* **Depois de adicionar um irmão com [`--add-dir`](#grant-access-across-packages-or-repositories)**: as skills desse irmão também carregam. A configuração `additionalDirectories` concede apenas acesso a arquivo e não carrega skills

Os nomes sempre carregam, mas [quando há muitas, algumas skills perdem suas descrições inteiramente](/docs/pt/skills#skill-descriptions-are-cut-short), que pode remover as palavras-chave que Claude usa para decidir se uma skill se aplica. Mantenha descrições curtas e comece com palavras que uma solicitação conteria, como "escrevendo ou modificando testes em `packages/api/`".

Para skills que muitos diretórios compartilham, como convenções de PR ou uma checklist de deploy, coloque-as no `.claude/skills/` da raiz do repositório para que carreguem de qualquer diretório inicial. Quando skills compartilhadas precisam de seu próprio histórico de versão ou devem funcionar entre repositórios, empacote-as como um [plugin](/docs/pt/plugins/overview) em vez disso. As skills de plugin usam um namespace `plugin-name:skill-name`, então nunca colidem com skills por diretório. Uma equipe de plataforma pode versioná-las e atualizá-las em um lugar.

Para encontrar quais skills vão não utilizadas, habilite o exportador OpenTelemetry [logs](/docs/pt/monitoring-usage) e defina `OTEL_LOG_TOOL_DETAILS=1` para que nomes de skill sejam registrados verbatim em vez de redacted. O evento [`skill_activated`](/docs/pt/monitoring-usage#skill-activated-event) registra cada invocação em seu atributo `skill.name`, e `invocation_trigger` registra se um comando, Claude ou uma skill aninhada o invocou, que te diz o que consolidar ou aposentar.

<h2 id="centralize-conventions-when-layering-stops-scaling">
  Centralize convenções quando camadas param de escalar
</h2>

Arquivos CLAUDE.md por diretório podem se tornar difíceis de governar conforme a base de código cresce. As convenções derivam, os arquivos ficam obsoletos, e ninguém possui a raiz. Resolver isso tipicamente cai para a equipe que mantém a configuração Claude Code do repositório em vez de para cada desenvolvedor trabalhando em sua própria área.

Mova convenções e conteúdo de referência para fora de CLAUDE.md sempre carregado e para mecanismos que carregam sob demanda:

* [Skills](/docs/pt/skills): material de referência que Claude carrega apenas quando relevante para a tarefa
* [Plugins](/docs/pt/plugins/overview): pacotes versionados de skills, hooks e comandos que uma equipe de plataforma possui centralmente
* [Servidores MCP](/docs/pt/mcp): se sua organização já executa uma busca de código ou índice RAG sobre o repositório, exponha-o como uma ferramenta MCP para que Claude a consulte em vez de ler arquivos diretamente

Veja [configurações gerenciadas por servidor ou endpoint](/docs/pt/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings) para como equipes de plataforma podem impor essas centralmente.

<h3 id="recommend-the-right-plugin-at-session-start">
  Recomende o plugin certo na inicialização da sessão
</h3>

Uma vez que convenções vivem em plugins, um colega iniciando Claude em uma parte desconhecida da árvore não tem sinal sobre qual plugin os proprietários dessa área mantêm. Um [`SessionStart` hook](/docs/pt/hooks#sessionstart) pode fechar essa lacuna, já que Claude Code adiciona texto simples que o hook imprime para stdout ao contexto do Claude antes do primeiro prompt.

Por exemplo, você pode escrever um script que lê o diretório de inicialização da [entrada do hook](/docs/pt/hooks#common-input-fields), o procura em um mapa de caminho para plugin confirmado no repositório, e imprime a recomendação para Claude retransmitir em sua primeira resposta. Veja [Automatize ações com hooks](/docs/pt/hooks-guide) para escrever e registrar o hook.

<h2 id="put-it-together">
  Junte tudo
</h2>

A configuração combinada abaixo usa o layout do monorepo. Os mesmos arquivos funcionam para qualquer subdiretório em uma grande árvore única. O `.claude/settings.json` de cada subdiretório deve ser autossuficiente em vez de em camadas em um arquivo raiz.

O exemplo confirma `worktree`, `additionalDirectories` e as regras de negação `Read` em `.claude/settings.json` para que cada desenvolvedor em `packages/api/` obtenha o mesmo acesso irmão, caminhos esparsos e exclusões. O arquivo abaixo é as configurações por área confirmadas para `packages/api/`:

```json packages/api/.claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  },
  "permissions": {
    "additionalDirectories": [
      "../shared"
    ],
    "deny": [
      "Read(./**/dist/**/*)",
      "Read(./**/build/**/*)"
    ]
  }
}
```

Porque essa sessão inicia de `packages/api/`, os arquivos CLAUDE.md de pacotes irmãos já estão fora de escopo, então `claudeMdExcludes` não é necessário aqui. Adicione-o ao `.claude/settings.local.json` da raiz do repositório em vez disso se você também inicia sessões da raiz.

A entrada `additionalDirectories` se aplica quando você inicia Claude de `packages/api/` diretamente. Dentro de um worktree criado dessa sessão, o diretório de trabalho é a raiz do worktree, então esse arquivo de configurações não carrega. Os pacotes irmãos já são alcançáveis dentro do worktree sem ele, mas as regras de negação precisam de uma segunda cópia no `.claude/settings.json` da raiz do repositório para que sessões de worktree as peguem, como a [nota de configurações de worktree](#check-out-only-the-directories-you-need) descreve:

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**/*)",
      "Read(./**/build/**/*)"
    ]
  }
}
```

Após a configuração, o repositório tem esse layout:

```text theme={null}
monorepo/
  CLAUDE.md
  .claude/settings.json                           # regras de negação para sessões de worktree
  packages/
    api/
      CLAUDE.md
      .claude/settings.json                       # worktree, additionalDirectories, regras de negação
      .claude/skills/api-testing/SKILL.md
    web/
      CLAUDE.md
      .claude/skills/component-patterns/SKILL.md
    shared/
      CLAUDE.md
```

Com essa configuração, iniciando Claude de `packages/api/`:

* Carrega o CLAUDE.md raiz e `packages/api/CLAUDE.md`, pula `packages/web/CLAUDE.md`
* Pode ler e editar arquivos em `packages/api/` e `packages/shared/`
* Pula leituras de saída de compilação sob `dist/` e `build/` em `packages/api/`
* Tem a skill api-testing disponível sob demanda
* Cria worktrees contendo `.claude/`, `packages/api/`, `packages/shared/` e arquivos em nível raiz, com as regras de negação aplicadas através do worktree do arquivo de configurações raiz

<h2 id="scope-and-plan-changes-that-span-packages">
  Escopo e plano de mudanças que abrangem pacotes
</h2>

A configuração acima controla o que Claude vê. Quando uma única mudança toca vários pacotes, como atualizar um tipo compartilhado junto com cada site de chamada que o usa, como você escopeia e sequencia a tarefa também afeta o resultado.

Duas técnicas ajudam a manter uma mudança entre pacotes consistente:

* **Dê ao Claude a mudança inteira em uma sessão**: entregar a edição compartilhada e seus sites de chamada juntos mantém as decisões atrás de cada edição consistentes, em vez de re-derivá-las por pacote
* **Planeje antes de editar**: [planeje primeiro](/docs/pt/best-practices#explore-first-then-plan-then-code) em [plan mode](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode), e Claude escreve o plano em um arquivo. Uma longa sessão entre pacotes [compacta seu contexto](/docs/pt/context-window#what-survives-compaction) ao longo do caminho. Claude Code reinjecta o arquivo de plano após cada compactação, então o plano sobrevive onde o histórico de conversa pode não

<h2 id="next-steps">
  Próximos passos
</h2>

Uma vez que essa configuração está em lugar, você pode refiná-la:

* Use [hooks](/docs/pt/hooks-guide) para executar linters ou verificadores de tipo por diretório após Claude editar arquivos
* Revise [Gerencie custos efetivamente](/docs/pt/costs) para entender como o tamanho da base de código afeta o uso de tokens e como definir limites de gastos antes de um rollout mais amplo
* Leia [Como Claude Code funciona em grandes bases de código](https://claude.com/blog/how-claude-code-works-in-large-codebases-best-practices-and-where-to-start) no blog Claude para padrões de rollout organizacional e modelos de propriedade que ficam acima da configuração por repositório nesta página
