> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Como Claude se lembra do seu projeto

> Dê a Claude instruções persistentes com arquivos CLAUDE.md ou AGENTS.md, e deixe Claude acumular aprendizados automaticamente com memória automática.

Cada sessão do Claude Code começa com uma janela de contexto limpa. Dois mecanismos carregam conhecimento entre sessões:

* **Arquivos CLAUDE.md**: instruções que você escreve para dar a Claude contexto persistente. Claude também pode ler arquivos [`AGENTS.md`](#agents-md) de um repositório, por conta própria ou ao lado de CLAUDE.md
* **Memória automática**: notas que Claude escreve para si mesma com base em suas correções e preferências

Esta página cobre como:

* [Escrever e organizar arquivos CLAUDE.md](#claude-md-files)
* [Usar um AGENTS.md existente](#agents-md) como suas instruções de projeto, por conta própria ou ao lado de CLAUDE.md
* [Escopear regras para tipos de arquivo específicos](#organize-rules-with-claude/rules/) com `.claude/rules/`
* [Configurar memória automática](#auto-memory) para que Claude tome notas automaticamente
* [Solucionar problemas](#troubleshoot-memory-issues) quando as instruções não estão sendo seguidas

<h2 id="claude-md-vs-auto-memory">
  CLAUDE.md vs memória automática
</h2>

Claude Code tem dois sistemas de memória complementares. Ambos são carregados no início de cada conversa. Claude os trata como contexto, não como configuração imposta. Para bloquear uma ação independentemente do que Claude decidir, use um [hook PreToolUse](/docs/pt/hooks-guide) em vez disso. Quanto mais específicas e concisas forem suas instruções, mais consistentemente Claude as seguirá.

|                  | Arquivos CLAUDE.md                                                 | Memória automática                                                                                               |
| :--------------- | :----------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| **Quem escreve** | Você                                                               | Claude                                                                                                           |
| **O que contém** | Instruções e regras                                                | Aprendizados e padrões                                                                                           |
| **Escopo**       | Projeto, usuário ou organização                                    | Por repositório, compartilhado entre worktrees                                                                   |
| **Carregado em** | Cada sessão                                                        | Cada sessão (primeiras 200 linhas ou 25KB)                                                                       |
| **Usar para**    | Padrões de codificação, fluxos de trabalho, arquitetura do projeto | Suas preferências, correções que você dá a Claude, contexto do projeto que Claude não consegue derivar do código |

Use arquivos CLAUDE.md quando quiser guiar o comportamento de Claude. A memória automática permite que Claude aprenda com suas correções sem esforço manual.

Subagents também podem manter sua própria memória automática. Veja [configuração de subagent](/docs/pt/sub-agents#enable-persistent-memory) para detalhes.

<h2 id="claude-md-files">
  Arquivos CLAUDE.md
</h2>

Os arquivos CLAUDE.md são arquivos markdown que fornecem instruções persistentes ao Claude para um projeto, seu fluxo de trabalho pessoal ou toda a sua organização. Você escreve esses arquivos em texto simples; Claude os lê no início de cada sessão. Se seu repositório usa `AGENTS.md` em vez disso, consulte [AGENTS.md](#agents-md).

<h3 id="when-to-add-to-claude-md">
  Quando adicionar ao CLAUDE.md
</h3>

Trate CLAUDE.md como o lugar onde você escreve o que de outra forma teria que re-explicar. Adicione a ele quando:

* Claude comete o mesmo erro uma segunda vez
* Uma revisão de código detecta algo que Claude deveria saber sobre este codebase
* Você digita a mesma correção ou esclarecimento no chat que digitou na sessão anterior
* Um novo colega de equipe precisaria do mesmo contexto para ser produtivo

Mantenha-o com fatos que Claude deve manter em cada sessão: comandos de compilação, convenções, layout do projeto, regras "sempre faça X". Se uma entrada é um procedimento de várias etapas ou importa apenas para uma parte do codebase, mova-a para uma [skill](/docs/pt/skills) ou uma [regra com escopo de caminho](#organize-rules-with-claude/rules/) em vez disso. A [visão geral da extensão](/docs/pt/features-overview#build-your-setup-over-time) cobre quando usar cada mecanismo.

<h3 id="choose-where-to-put-claude-md-files">
  Escolha onde colocar os arquivos CLAUDE.md
</h3>

Os arquivos CLAUDE.md podem estar em vários locais, cada um com um escopo diferente. A tabela abaixo lista-os em ordem de carregamento, do escopo mais amplo para o mais específico, para que uma instrução de projeto apareça em contexto após uma instrução do usuário.

| Escopo                    | Localização                                                                                                                                                           | Propósito                                                             | Exemplos de caso de uso                                                               | Compartilhado com                        |
| ------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- | ------------------------------------------------------------------------------------- | ---------------------------------------- |
| **Política gerenciada**   | • macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`<br />• Linux e WSL: `/etc/claude-code/CLAUDE.md`<br />• Windows: `C:\Program Files\ClaudeCode\CLAUDE.md` | Instruções em toda a organização gerenciadas por TI/DevOps            | Padrões de codificação da empresa, políticas de segurança, requisitos de conformidade | Todos os usuários da organização         |
| **Instruções do usuário** | `~/.claude/CLAUDE.md`                                                                                                                                                 | Preferências pessoais para todos os projetos                          | Preferências de estilo de código, atalhos de ferramentas pessoais                     | Apenas você (todos os projetos)          |
| **Instruções do projeto** | `./CLAUDE.md` ou `./.claude/CLAUDE.md`. Consulte [AGENTS.md](#agents-md) para quando `./AGENTS.md` carrega em vez de ou junto com eles                                | Instruções compartilhadas pela equipe para o projeto                  | Arquitetura do projeto, padrões de codificação, fluxos de trabalho comuns             | Membros da equipe via controle de versão |
| **Instruções locais**     | `./CLAUDE.local.md`                                                                                                                                                   | Preferências pessoais específicas do projeto; adicione a `.gitignore` | Suas URLs de sandbox, dados de teste preferidos                                       | Apenas você (projeto atual)              |

Os arquivos CLAUDE.md e CLAUDE.local.md no diretório acima do diretório de trabalho são carregados na inicialização. Os arquivos em subdiretórios carregam sob demanda quando Claude lê arquivos nesses diretórios. Consulte [Como os arquivos CLAUDE.md carregam](#how-claude-md-files-load) para a ordem de resolução completa.

Para projetos grandes, você pode dividir instruções em arquivos específicos de tópicos usando [regras de projeto](#organize-rules-with-claude/rules/). As regras permitem que você escope instruções para tipos de arquivo específicos ou subdiretórios.

<h3 id="set-up-a-project-claude-md">
  Configure um CLAUDE.md de projeto
</h3>

Um CLAUDE.md de projeto pode ser armazenado em `./CLAUDE.md` ou `./.claude/CLAUDE.md`. Crie este arquivo e adicione instruções que se apliquem a qualquer pessoa trabalhando no projeto: comandos de compilação e teste, padrões de codificação, decisões arquitetônicas, convenções de nomenclatura e fluxos de trabalho comuns. Essas instruções são compartilhadas com sua equipe através do controle de versão, portanto, concentre-se em padrões em nível de projeto em vez de preferências pessoais. Para confirmar que o arquivo foi carregado, execute `/context` em uma sessão e verifique a lista em **Memory files**.

<Tip>
  Execute `/init` para gerar um CLAUDE.md inicial automaticamente. Claude analisa seu codebase e cria um arquivo com comandos de compilação, instruções de teste e convenções de projeto que descobre. Se um CLAUDE.md já existe, `/init` sugere melhorias em vez de sobrescrever. Refine a partir daí com instruções que Claude não descobriria por conta própria.

  Para um fluxo interativo de várias fases em vez disso, defina a variável de ambiente `CLAUDE_CODE_NEW_INIT` como `1` antes de executar `/init`. Defina-a em seu shell ou no bloco `env` de um arquivo de configurações, conforme mostrado em [Defina variáveis de ambiente](/docs/pt/env-vars#set-environment-variables). Com ela definida, `/init` pergunta quais artefatos configurar: arquivos CLAUDE.md, skills e hooks. Em seguida, explora seu codebase com um subagente, preenche lacunas por meio de perguntas de acompanhamento e apresenta uma proposta revisável antes de escrever qualquer arquivo. A variável apenas muda como `/init` é executado, portanto você pode deixá-la definida.
</Tip>

<h3 id="write-effective-instructions">
  Escreva instruções eficazes
</h3>

Os arquivos CLAUDE.md são carregados na janela de contexto no início de cada sessão, consumindo tokens junto com sua conversa. A [visualização da janela de contexto](/docs/pt/context-window) mostra onde CLAUDE.md carrega em relação ao resto do contexto de inicialização. Como são contexto em vez de configuração imposta, como você escreve as instruções afeta o quão confiável Claude as segue. Instruções específicas, concisas e bem estruturadas funcionam melhor.

**Tamanho**: alvo de menos de 200 linhas por arquivo CLAUDE.md. Arquivos mais longos consomem mais contexto e reduzem a adesão. Se suas instruções estão crescendo muito, use [regras com escopo de caminho](#path-specific-rules) para que as instruções carreguem apenas quando Claude trabalha com arquivos correspondentes. Você também pode dividir o conteúdo em [importações](#import-additional-files) para organização, embora os arquivos importados ainda carreguem e entrem na janela de contexto na inicialização.

**Estrutura**: use cabeçalhos markdown e bullets para agrupar instruções relacionadas. Claude verifica a estrutura da mesma forma que os leitores fazem: seções organizadas são mais fáceis de seguir do que parágrafos densos.

**Especificidade**: escreva instruções que sejam concretas o suficiente para verificar. Por exemplo:

* "Use indentação de 2 espaços" em vez de "Formate o código adequadamente"
* "Execute `npm test` antes de fazer commit" em vez de "Teste suas alterações"
* "Os manipuladores de API vivem em `src/api/handlers/`" em vez de "Mantenha os arquivos organizados"

**Consistência**: se duas regras se contradizem, Claude pode escolher uma arbitrariamente. Revise seus arquivos CLAUDE.md, arquivos CLAUDE.md aninhados em subdiretórios e [`.claude/rules/`](#organize-rules-with-claude/rules/) periodicamente para remover instruções desatualizadas ou conflitantes. Em monorepos, use [`claudeMdExcludes`](#exclude-specific-claude-md-files) para pular arquivos CLAUDE.md de outras equipes que não são relevantes para seu trabalho.

<h3 id="import-additional-files">
  Importe arquivos adicionais
</h3>

Os arquivos CLAUDE.md podem importar arquivos adicionais usando a sintaxe `@path/to/import`. Os arquivos importados são expandidos e carregados em contexto na inicialização junto com o CLAUDE.md que os referencia.

Caminhos relativos e absolutos são permitidos. Caminhos relativos são resolvidos em relação ao arquivo que contém a importação, não ao diretório de trabalho. Os arquivos importados podem importar recursivamente outros arquivos, com uma profundidade máxima de quatro saltos.

A análise de importação ignora spans de código Markdown e blocos de código cercados. Para mencionar um caminho em seu CLAUDE.md sem importá-lo, envolva-o em backticks: escrever `` `@README` `` mantém o texto literal, enquanto `@README` fora de backticks importa o arquivo.

Para trazer um README, package.json e um guia de fluxo de trabalho, referencie-os com a sintaxe `@` em qualquer lugar em seu CLAUDE.md:

```text theme={null}
Consulte @README para visão geral do projeto e @package.json para comandos npm disponíveis para este projeto.

# Instruções Adicionais
- fluxo de trabalho git @docs/git-instructions.md
```

Para preferências pessoais por projeto que não devem ser verificadas no controle de versão, crie um `CLAUDE.local.md` na raiz do projeto. Ele carrega junto com `CLAUDE.md` e é tratado da mesma forma. Adicione `CLAUDE.local.md` ao seu `.gitignore` para que não seja confirmado. Com `CLAUDE_CODE_NEW_INIT=1` definido, executar `/init` e escolher a opção pessoal faz isso para você.

Se você trabalha em várias Git Worktrees do mesmo repositório, um `CLAUDE.local.md` ignorado pelo git existe apenas na worktree onde você o criou. Para compartilhar instruções pessoais entre worktrees, importe um arquivo do seu diretório inicial em vez disso:

```text theme={null}
# Preferências Individuais
- @~/.claude/my-project-instructions.md
```

<Warning>
  Uma importação em um arquivo de memória em nível de projeto é externa quando seu caminho é resolvido fora do seu diretório de trabalho, como a importação do diretório inicial acima. Na primeira vez que Claude Code encontra importações externas em um projeto, mostra um diálogo de aprovação listando os arquivos. Se você recusar, as importações permanecerão desabilitadas e o diálogo não aparecerá novamente.

  Claude Code mostra o diálogo para protegê-lo de arquivos que outras pessoas confirmam em um projeto compartilhado. Arquivos de memória com escopo de usuário, como `~/.claude/CLAUDE.md` e `~/.claude/rules/`, são arquivos que você mesmo escreveu. Exceto em sessões [Cowork](https://claude.com/product/cowork) em seu desktop, Claude Code carrega suas importações sem o diálogo e confia nelas como o resto de sua configuração pessoal.

  Em sessões Cowork em seu desktop, Claude Code ignora qualquer importação em um arquivo com escopo de usuário que seja resolvida para um caminho fora do diretório de trabalho da sessão e carrega o resto do arquivo. Nessas sessões, também ignora um `~/.claude/CLAUDE.md` que é em si um symlink ou hard link, e um diretório `~/.claude/rules/` symlinked ou arquivo de regra que aponta para fora do diretório de trabalho.
</Warning>

<h3 id="how-claude-md-files-load">
  Como os arquivos CLAUDE.md carregam
</h3>

Claude Code carrega `CLAUDE.md` e `CLAUDE.local.md` do seu diretório de trabalho atual e de cada diretório acima dele. Execute Claude Code em `foo/bar/` e ele carrega instruções de `foo/bar/CLAUDE.md`, `foo/CLAUDE.md` e qualquer arquivo `CLAUDE.local.md` ao lado deles.

Todos os arquivos descobertos são concatenados em contexto em vez de se sobreporem. Na árvore de diretórios, o conteúdo é ordenado da raiz do sistema de arquivos até seu diretório de trabalho. Para o exemplo `foo/bar/`, `foo/CLAUDE.md` aparece em contexto antes de `foo/bar/CLAUDE.md`, portanto as instruções mais próximas de onde você iniciou Claude são lidas por último. Dentro de cada diretório, `CLAUDE.local.md` é anexado após `CLAUDE.md`, portanto suas notas pessoais são a última coisa que Claude lê nesse nível.

Claude também descobre arquivos `CLAUDE.md` e `CLAUDE.local.md` em subdiretórios sob seu diretório de trabalho atual. Em vez de carregá-los na inicialização, eles são incluídos quando Claude lê arquivos nesses subdiretórios.

Se você trabalha em um grande monorepo onde os arquivos CLAUDE.md de outras equipes são detectados, use [`claudeMdExcludes`](#exclude-specific-claude-md-files) para ignorá-los. Para o layout completo de arquivos CLAUDE.md raiz e por diretório e regras, consulte [Monorepos e repositórios grandes](/docs/pt/large-codebases).

Comentários HTML em nível de bloco (`<!-- maintainer notes -->`) em arquivos CLAUDE.md são removidos antes do conteúdo ser injetado no contexto do Claude. Use-os para deixar notas para mantenedores humanos sem gastar tokens de contexto neles. Comentários dentro de blocos de código são preservados. Quando você abre um arquivo CLAUDE.md diretamente com a ferramenta Read, os comentários permanecem visíveis.

<h4 id="load-from-additional-directories">
  Carregue de diretórios adicionais
</h4>

O sinalizador `--add-dir` dá ao Claude acesso a diretórios adicionais fora do seu diretório de trabalho principal. Por padrão, os arquivos CLAUDE.md desses diretórios não são carregados.

Para também carregar arquivos de memória de diretórios adicionais, defina a variável de ambiente `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`:

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared-config
```

O formulário inline define a variável para esse único lançamento em Bash ou Zsh. Para mantê-la ativada para cada sessão, adicione-a ao bloco `env` em `~/.claude/settings.json` conforme mostrado em [Defina variáveis de ambiente](/docs/pt/env-vars#set-environment-variables).

Isso carrega `CLAUDE.md`, `.claude/CLAUDE.md`, `.claude/rules/*.md` e `CLAUDE.local.md` do diretório adicional. `CLAUDE.local.md` é ignorado se você excluir `local` de [`--setting-sources`](/docs/pt/cli-reference).

<h3 id="organize-rules-with-claude/rules/">
  Organize regras com `.claude/rules/`
</h3>

Para projetos maiores, você pode organizar instruções em vários arquivos usando o diretório `.claude/rules/`. Isso mantém as instruções modulares e mais fáceis para as equipes manterem. As regras também podem ser [escopo para caminhos de arquivo específicos](#path-specific-rules), portanto, carregam em contexto apenas quando Claude trabalha com arquivos correspondentes, reduzindo ruído e economizando espaço de contexto.

<Note>
  As regras carregam em contexto a cada sessão ou quando arquivos correspondentes são abertos. Para instruções específicas de tarefas que não precisam estar em contexto o tempo todo, use [skills](/docs/pt/skills) em vez disso, que carregam apenas quando você as invoca ou quando Claude determina que são relevantes para seu prompt.
</Note>

<h4 id="set-up-rules">
  Configure regras
</h4>

Coloque arquivos markdown no diretório `.claude/rules/` do seu projeto. Cada arquivo deve cobrir um tópico, com um nome de arquivo descritivo como `testing.md` ou `api-design.md`. Todos os arquivos `.md` são descobertos recursivamente, portanto você pode organizar regras em subdiretórios como `frontend/` ou `backend/`:

```text theme={null}
seu-projeto/
├── .claude/
│   ├── CLAUDE.md           # Instruções principais do projeto
│   └── rules/
│       ├── code-style.md   # Diretrizes de estilo de código
│       ├── testing.md      # Convenções de teste
│       └── security.md     # Requisitos de segurança
```

Regras sem [frontmatter `paths`](#path-specific-rules) são carregadas na inicialização com a mesma prioridade que `.claude/CLAUDE.md`.

As regras do projeto são ignoradas se você excluir `project` de [`--setting-sources`](/docs/pt/cli-reference). Antes da v2.1.211, regras que carregam sob demanda, incluindo regras com escopo de caminho e regras em diretórios `.claude/rules/` aninhados, carregavam mesmo quando `project` era excluído.

<h4 id="path-specific-rules">
  Regras com escopo de caminho
</h4>

As regras podem ser escopo para arquivos específicos usando frontmatter YAML com o campo `paths`. Essas regras condicionais se aplicam apenas quando Claude está trabalhando com arquivos que correspondem aos padrões especificados.

```markdown theme={null}
---
paths:
  - "src/api/**/*.ts"
---

# Regras de Desenvolvimento de API

- Todos os endpoints de API devem incluir validação de entrada
- Use o formato de resposta de erro padrão
- Inclua comentários de documentação OpenAPI
```

Regras sem um campo `paths` são carregadas incondicionalmente e se aplicam a todos os arquivos. As regras com escopo de caminho são acionadas quando Claude lê arquivos que correspondem ao padrão, não em cada uso de ferramenta. A partir da v2.1.198, a correspondência também funciona quando Claude alcança um arquivo através de um caminho symlinked para o diretório do projeto, por exemplo em um checkout symlinked.

Use padrões glob no campo `paths` para corresponder arquivos por extensão, diretório ou qualquer combinação:

| Padrão                 | Corresponde                                        |
| ---------------------- | -------------------------------------------------- |
| `**/*.ts`              | Todos os arquivos TypeScript em qualquer diretório |
| `src/**/*`             | Todos os arquivos sob o diretório `src/`           |
| `*.md`                 | Arquivos Markdown na raiz do projeto               |
| `src/components/*.tsx` | Componentes React em um diretório específico       |

Você pode especificar vários padrões e usar expansão de chaves para corresponder várias extensões em um padrão:

```markdown theme={null}
---
paths:
  - "src/**/*.{ts,tsx}"
  - "lib/**/*.ts"
  - "tests/**/*.test.ts"
---
```

Cada grupo de chaves multiplica o número de padrões expandidos: `src/*.{ts,tsx}` se expande para dois padrões, e `{a,b}/{c,d}/*.{ts,tsx}` para oito. Para manter a expansão limitada, a lista `paths` inteira de uma regra compartilha um orçamento de 1.000 padrões expandidos e 4 MiB, e padrões sem chaves não contam contra ele.

Claude Code usa qualquer padrão que excederia o orçamento não expandido, e suas chaves literais não correspondem a nenhum arquivo. Antes da v2.1.217, um valor `paths` com muitos grupos de chaves travava ou fazia o CLI falhar na inicialização.

A sintaxe Glob trata `[` como o início de uma expressão de colchete como `[abc]`. Um padrão com um `[` que não pode ser lido como uma expressão de colchete, como `photos [2024/**`, é inválido: não corresponde a nada, e os outros padrões da regra continuam funcionando. Para corresponder um `[` literal em um nome de arquivo, escape-o como `photos \[2024/**`. Antes da v2.1.207, um padrão inválido fazia a ferramenta Read falhar para cada arquivo em que a regra era avaliada, em vez de não corresponder a nada.

<h4 id="rules-frontmatter-reference">
  Referência de frontmatter de regra
</h4>

Configure uma regra com [frontmatter](/docs/pt/glossary#frontmatter) YAML entre marcadores `---` no topo do arquivo. `paths` é o único campo que Claude Code lê de uma regra; qualquer outro campo é ignorado sem um erro. Claude Code remove o frontmatter antes de carregar a regra em contexto.

| Campo   | Obrigatório | Descrição                                                                                                                                        |
| :------ | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------- |
| `paths` | Não         | Padrões glob que [escopo a regra para arquivos correspondentes](#path-specific-rules). Aceita uma lista YAML ou uma string separada por vírgulas |

Se o YAML entre os marcadores não for analisado, Claude Code ignora o frontmatter e carrega a regra como se não tivesse `paths`. Execute `claude --debug` para ver o erro de análise.

<h4 id="share-rules-across-projects-with-symlinks">
  Compartilhe regras entre projetos com symlinks
</h4>

O diretório `.claude/rules/` suporta symlinks, portanto você pode manter um conjunto compartilhado de regras e vinculá-las em vários projetos. Symlinks circulares são detectados e tratados graciosamente.

Claude Code trata um symlink cujo alvo está fora do seu diretório de trabalho como uma [importação externa](#import-additional-files). As regras vinculadas não carregam até que você aprove importações externas para o projeto, e depois apenas as sem um campo [`paths`](#path-specific-rules) carregam. Claude Code pede essa aprovação apenas quando um arquivo de memória do projeto importa um arquivo fora do diretório de trabalho com `@path`, não para symlinks sozinhos. Para carregar regras compartilhadas sem essa aprovação, mantenha-as em [`~/.claude/rules/`](#user-level-rules), onde se aplicam a cada projeto em sua máquina.

Este exemplo vincula um diretório compartilhado e um arquivo individual:

```bash theme={null}
ln -s ~/shared-claude-rules .claude/rules/shared
ln -s ~/company-standards/security.md .claude/rules/security.md
```

<h4 id="user-level-rules">
  Regras em nível de usuário
</h4>

Regras pessoais em `~/.claude/rules/` se aplicam a cada projeto em sua máquina. Use-as para preferências que não são específicas do projeto:

```text theme={null}
~/.claude/rules/
├── preferences.md    # Suas preferências pessoais de codificação
└── workflows.md      # Seus fluxos de trabalho preferidos
```

Claude Code carrega regras em nível de usuário antes das regras do projeto, portanto uma regra do projeto aparece mais tarde no contexto do Claude do que uma regra do usuário. Nenhum conjunto sobrescreve o outro: se uma regra do usuário e uma regra do projeto conflitarem, Claude pode seguir qualquer uma, portanto mantenha as duas consistentes.

<h3 id="manage-claude-md-for-large-teams">
  Gerencie CLAUDE.md para grandes equipes
</h3>

Para organizações implantando Claude Code em equipes, você pode centralizar instruções e controlar quais arquivos CLAUDE.md são carregados.

<h4 id="deploy-organization-wide-claude-md">
  Implante CLAUDE.md em toda a organização
</h4>

As organizações podem implantar um CLAUDE.md gerenciado centralmente que se aplica a todos os usuários em uma máquina. Este arquivo não pode ser excluído pelas configurações individuais.

<Steps>
  <Step title="Crie o arquivo no local da política gerenciada">
    * macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`
    * Linux e WSL: `/etc/claude-code/CLAUDE.md`
    * Windows: `C:\Program Files\ClaudeCode\CLAUDE.md`
  </Step>

  <Step title="Implante com seu sistema de gerenciamento de configuração">
    Use MDM, Group Policy, Ansible ou ferramentas similares para distribuir o arquivo entre máquinas de desenvolvedores. Consulte [configurações gerenciadas](/docs/pt/managed-settings) para outras opções de configuração em toda a organização.
  </Step>
</Steps>

A chave `claudeMd` permite que você coloque o conteúdo CLAUDE.md gerenciado diretamente dentro de `managed-settings.json` em vez de implantar um arquivo separado.

**Escopo**: cada sessão Claude Code na máquina, em cada repositório. Para orientação específica do repositório, confirme um CLAUDE.md de projeto em vez disso.

**Precedência**: igual a um arquivo CLAUDE.md gerenciado. Carrega antes de CLAUDE.md do usuário e do projeto.

**Onde é honrado**: apenas configurações gerenciadas e de política. Definir `claudeMd` em configurações de usuário, projeto ou local não tem efeito.

O exemplo abaixo adiciona instruções comportamentais diretamente em um arquivo de configurações gerenciadas:

```json theme={null}
{
  "claudeMd": "Always run `make lint` before committing.\nNever push directly to main."
}
```

Um CLAUDE.md gerenciado e [configurações gerenciadas](/docs/pt/managed-settings) servem a propósitos diferentes. Use configurações para imposição técnica e CLAUDE.md para orientação comportamental:

| Preocupação                                                       | Configure em                                                       |
| :---------------------------------------------------------------- | :----------------------------------------------------------------- |
| Bloqueie ferramentas, comandos ou caminhos de arquivo específicos | Configurações gerenciadas: `permissions.deny`                      |
| Imponha isolamento de sandbox                                     | Configurações gerenciadas: `sandbox.enabled`                       |
| Variáveis de ambiente e roteamento de provedor de API             | Configurações gerenciadas: `env`                                   |
| Método de login e restrições de organização                       | Configurações gerenciadas: `forceLoginMethod`, `forceLoginOrgUUID` |
| Diretrizes de estilo de código e qualidade                        | CLAUDE.md gerenciado                                               |
| Lembretes de tratamento de dados e conformidade                   | CLAUDE.md gerenciado                                               |
| Instruções comportamentais para Claude                            | CLAUDE.md gerenciado                                               |

As regras de configurações são impostas pelo cliente independentemente do que Claude decide fazer. As instruções CLAUDE.md moldam o comportamento do Claude, mas não são uma camada de imposição rígida.

<h4 id="exclude-specific-claude-md-files">
  Exclua arquivos CLAUDE.md específicos
</h4>

Em grandes monorepos, os arquivos CLAUDE.md ancestrais podem conter instruções que não são relevantes para seu trabalho. A configuração `claudeMdExcludes` permite que você pule arquivos específicos por caminho ou padrão glob.

Este exemplo exclui um CLAUDE.md de nível superior e um diretório de regras de uma pasta pai. Adicione-o a `.claude/settings.local.json` para que a exclusão permaneça local em sua máquina:

```json theme={null}
{
  "claudeMdExcludes": [
    "**/monorepo/CLAUDE.md",
    "/home/user/monorepo/other-team/.claude/rules/**"
  ]
}
```

Os padrões são correspondidos contra caminhos de arquivo absolutos usando sintaxe glob. Você pode configurar `claudeMdExcludes` em qualquer [camada de configurações](/docs/pt/settings#where-settings-live): usuário, projeto, local ou política gerenciada. Os arrays se mesclam entre camadas.

Para excluir um arquivo de regras que você alcança através de um [symlink](#share-rules-across-projects-with-symlinks), seja o arquivo ou seu diretório o link, escreva o padrão contra qualquer caminho: o caminho do arquivo sob `.claude/rules/` ou seu alvo de link. Um padrão que corresponde a qualquer caminho exclui o arquivo. Antes da v2.1.239, apenas um padrão que correspondia ao alvo do link excluía o arquivo.

Os arquivos CLAUDE.md de política gerenciada não podem ser excluídos. Isso garante que as instruções em toda a organização sempre se apliquem independentemente das configurações individuais.

<h2 id="agents-md">
  AGENTS.md
</h2>

Claude Code pode ler [`AGENTS.md`](/docs/pt/glossary#agents-md) como suas instruções de projeto, portanto um repositório já configurado para outros agentes de codificação funciona sem adicionar um `CLAUDE.md`, uma importação ou uma configuração. Esta tabela mostra o que Claude lê por padrão para cada combinação de arquivos de instruções em seu repositório:

| Seu repositório tem                                                                                 | Claude lê                                                       |
| :-------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------- |
| Um `AGENTS.md` e nenhum `CLAUDE.md` ou `CLAUDE.local.md` em seu diretório de trabalho ou acima dele | Seu `AGENTS.md`                                                 |
| Um `AGENTS.md` e um `CLAUDE.md` ou `CLAUDE.local.md` em seu diretório de trabalho ou acima dele     | Apenas seus arquivos `CLAUDE.md`                                |
| Um `CLAUDE.md` que já [importa `AGENTS.md`](#share-one-file-with-other-coding-tools)                | Seu `CLAUDE.md`, com `AGENTS.md` incluído através da importação |

Para alterar o padrão, por exemplo para fazer Claude sempre ler ambos os arquivos, ler apenas `CLAUDE.md` ou ler apenas suas instruções gerenciadas pela organização, [altere a configuração **Project instructions**](#choose-which-instruction-files-load).

<Note>
  A leitura de `AGENTS.md` diretamente requer Claude Code v2.1.277 ou posterior. Em algumas sessões Claude [não consegue ler `AGENTS.md`](#when-agents-md-support-is-unavailable), então [importe-o de um `CLAUDE.md`](#share-one-file-with-other-coding-tools) lá em vez disso.
</Note>

<h3 id="when-claude-code-reads-agents-md">
  When Claude Code reads AGENTS.md
</h3>

Por padrão, Claude lê `AGENTS.md` apenas quando você não tem `CLAUDE.md` em seu diretório de trabalho ou acima dele. Aqui estão quais de seus arquivos contam para essa verificação:

* **Contam, então Claude os lê em vez de `AGENTS.md`**: um `CLAUDE.md`, `.claude/CLAUDE.md` ou `CLAUDE.local.md` em seu diretório de trabalho ou em qualquer diretório acima dele
* **Não contam e continuam carregando junto com `AGENTS.md`**: seu `~/.claude/CLAUDE.md`, o `CLAUDE.md` gerenciado de sua organização e arquivos `.claude/rules/`

Quando nenhum conta, aqui está o que Claude lê e como você pode saber:

* **No início da sessão**: cada `AGENTS.md` e `.claude/AGENTS.md` em seu diretório de trabalho e nos diretórios acima dele. Em uma sessão interativa você vê uma linha como `no CLAUDE.md found; AGENTS.md loaded: /home/you/repo/AGENTS.md` na conversa
* **Conforme Claude trabalha em subdiretórios**: um `AGENTS.md` de subdiretório, quando Claude abre um arquivo lá com a ferramenta Read e esse subdiretório não tem nenhum dos três arquivos `CLAUDE.md` próprios
* **Dentro de cada `AGENTS.md`**: [importações `@path`](#import-additional-files) são expandidas, padrões [`claudeMdExcludes`](#exclude-specific-claude-md-files) se aplicam e subagentes que [pulam instruções de projeto](/docs/pt/sub-agents#what-loads-at-startup) também pulam esses arquivos
* **Não lido**: `AGENTS.local.md`, `AGENTS.override.md` ou qualquer coisa sob um diretório `.agents/`

<Note>
  Como `CLAUDE.local.md` conta, adicionar um para manter suas próprias instruções não confirmadas em um projeto que depende de `AGENTS.md` impede Claude de ler `AGENTS.md` para você. Para manter seu `CLAUDE.local.md` e ainda ter Claude lendo `AGENTS.md`, defina **Project instructions** como [`claude-md-and-agents-md`](#choose-which-instruction-files-load).
</Note>

<h3 id="choose-which-instruction-files-load">
  Choose which instruction files load
</h3>

Para alterar quais arquivos Claude lê, digite `/config` em uma sessão Claude Code para abrir o painel de configurações e defina **Project instructions** como um destes valores:

| Value                     | What Claude reads                                                                                                                                                                                                                                                                                                                                                                                                     |
| :------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude-md-or-agents-md`  | Seus arquivos `CLAUDE.md` ou seus arquivos `AGENTS.md` quando você não tem `CLAUDE.md` ou `CLAUDE.local.md` em seu diretório de trabalho ou acima dele. Este é o padrão                                                                                                                                                                                                                                               |
| `claude-md-and-agents-md` | Seus arquivos `CLAUDE.md` e `AGENTS.md` juntos, os arquivos `CLAUDE.md` de cada diretório primeiro e seu `AGENTS.md` depois. Claude Code pula um `AGENTS.md` que já carregou, então um que seu `CLAUDE.md` importa ou cria um symlink para não é lido duas vezes                                                                                                                                                      |
| `claude-md`               | Apenas seus arquivos `CLAUDE.md`                                                                                                                                                                                                                                                                                                                                                                                      |
| `managed-only`            | Apenas o `CLAUDE.md` gerenciado de sua organização e [auto memory](#auto-memory) no lançamento. Seus arquivos `CLAUDE.md` de projeto, local e usuário, seus arquivos `.claude/rules/` e cada `AGENTS.md` são deixados de fora. O `CLAUDE.md` de um subdiretório e os arquivos `.claude/rules/` ainda carregam quando Claude lê um arquivo lá, e [regras com escopo de caminho](#path-specific-rules) ainda se aplicam |

Você também pode definir o valor em um arquivo de configurações em vez de `/config`. Adicione-o sob o ID do plugin `agents-md` integrado em [`pluginConfigs`](/docs/pt/settings-reference#pluginconfigs), em `~/.claude/settings.json`, um arquivo `--settings` ou [configurações gerenciadas](/docs/pt/managed-settings). Claude Code o ignora em arquivos de configurações de projeto e local. Este exemplo faz Claude ler ambos os arquivos:

```json settings.json theme={null}
{
  "pluginConfigs": {
    "agents-md@builtin": {
      "options": { "instructionFiles": "claude-md-and-agents-md" }
    }
  }
}
```

Sua alteração se aplica a partir da próxima mensagem que você enviar e em cada nova sessão.

<h3 id="when-agents-md-support-is-unavailable">
  When AGENTS.md support is unavailable
</h3>

Nessas sessões Claude lê apenas arquivos `CLAUDE.md` e **Project instructions** não aparece no painel de configurações `/config`:

* Você está em uma versão Claude Code anterior à v2.1.277
* Você desabilitou o plugin `agents-md` integrado em `/plugin`
* Em alguns casos, é sua [primeira sessão depois que você atualiza](/docs/pt/env-vars#first-session-after-an-install-or-upgrade) de v2.1.276 ou anterior. Claude lê `AGENTS.md` a partir de sua próxima sessão

Antes de v2.1.281, algumas sessões, como aquelas no Amazon Bedrock ou com telemetria desabilitada, liam apenas arquivos `CLAUDE.md`. Nessas versões, atualize Claude Code. Para dar ao Claude seu `AGENTS.md` em qualquer uma dessas sessões, [importe-o de um `CLAUDE.md`](#share-one-file-with-other-coding-tools).

<h3 id="where-agents-md-differs-from-claude-md">
  Where AGENTS.md differs from CLAUDE.md
</h3>

Um `AGENTS.md` que Claude lê através da configuração **Project instructions** difere de um `CLAUDE.md` nestes lugares:

|                                                                                                                                                         | `CLAUDE.md`                                                                       | `AGENTS.md` lido através da configuração                                                                       |
| :------------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------- |
| [Hooks `InstructionsLoaded`](/docs/pt/hooks#instructionsloaded)                                                                                              | Disparam                                                                          | Não disparam. Eles disparam normalmente para um `AGENTS.md` que um `CLAUDE.md` importa ou cria um symlink para |
| Diretórios que você adiciona com `--add-dir` enquanto [`CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`](#load-from-additional-directories) está definido | Seu `CLAUDE.md` carrega                                                           | Seu `AGENTS.md` não carrega                                                                                    |
| Uma importação `@path` de um arquivo fora de seu diretório de trabalho                                                                                  | Claude Code pede que você aprove [importações externas](#import-additional-files) | Carrega apenas se você já aprovou importações externas para este projeto, sem prompt                           |

<h3 id="remove-an-earlier-agents-md-workaround">
  Remove an earlier AGENTS.md workaround
</h3>

Se você configurou Claude Code para ler `AGENTS.md` antes de fazer isso por conta própria, aqui está o que fazer com cada configuração comum:

* **Um `CLAUDE.md` contendo `@AGENTS.md`**: você pode deixá-lo. Manter a importação nunca faz Claude ler `AGENTS.md` duas vezes, qualquer que seja o valor de **Project instructions** que você use. Remova o `CLAUDE.md` se ele não contém nada mais, ou mantenha-o se algumas de suas sessões [não conseguem carregar `AGENTS.md` diretamente](#when-agents-md-support-is-unavailable).
* **Um `CLAUDE.md` que diz ao Claude em palavras para ler `AGENTS.md`**: Claude vê `AGENTS.md` apenas se decidir abrir o arquivo. Delete o `CLAUDE.md` para que Claude leia `AGENTS.md` diretamente, ou substitua a sentença por uma importação `@AGENTS.md`.
* **Um `CLAUDE.md` com symlink para `AGENTS.md`**: nada, ou delete o symlink. De qualquer forma, Claude lê o conteúdo uma vez.
* **Um hook `SessionStart` que imprime `AGENTS.md`**: remova-o. Depois que Claude lê `AGENTS.md` diretamente, o hook adiciona uma segunda cópia ao contexto.

<h3 id="share-one-file-with-other-coding-tools">
  Share one file with other coding tools
</h3>

Quando Claude não está lendo seu `AGENTS.md` diretamente, você ainda pode mantê-lo como o arquivo único que cada ferramenta compartilha colocando uma importação `@AGENTS.md` em um `CLAUDE.md` ao lado dele. Faça isso quando seu projeto também tiver um `CLAUDE.md`, quando você tiver definido **Project instructions** como `claude-md`, ou em sessões que [não conseguem carregar `AGENTS.md`](#when-agents-md-support-is-unavailable). Adicione qualquer instrução específica do Claude abaixo da importação, e Claude lê o arquivo importado primeiro, depois o resto:

```markdown CLAUDE.md theme={null}
@AGENTS.md

## Claude Code

Use plan mode for changes under `src/billing/`.
```

Se você não precisa de conteúdo específico do Claude, um symlink também funciona:

```bash theme={null}
ln -s AGENTS.md CLAUDE.md
```

O comando não imprime nada em caso de sucesso. Antes de escolher o symlink em vez da importação, verifique estas restrições:

* **Edição**: Claude lê `CLAUDE.md` através do link, mas as ferramentas Edit e Write [recusam-se a escrever através de um symlink](/docs/pt/errors#refusing-after-a-symlink-changed), e a recusa direciona Claude para editar o alvo do link, `AGENTS.md`, em vez disso
* **Windows**: se você ou qualquer pessoa que clona o repositório trabalha no Windows, use a importação `@AGENTS.md` em vez disso. Criar um symlink lá precisa de privilégios de Administrador ou Modo de Desenvolvedor, e Git verifica um symlink confirmado como um arquivo de texto simples a menos que `core.symlinks` esteja habilitado, o que deixa esse clone com um `CLAUDE.md` de uma linha no lugar de suas instruções

Com qualquer uma das abordagens, execute `/context` em sua próxima sessão e confirme que `CLAUDE.md` aparece em **Memory files**.

<h3 id="migrate-instructions-from-other-tools">
  Migrate instructions from other tools
</h3>

Executar [`/init`](/docs/pt/commands) lê arquivos de instruções de outras ferramentas e incorpora as partes relevantes ao `CLAUDE.md` gerado:

* Regras do Cursor em `.cursor/rules/` ou `.cursorrules`
* Regras do Copilot em `.github/copilot-instructions.md`
* Com `CLAUDE_CODE_NEW_INIT=1` definido: `AGENTS.md`, `.devin/rules/`, `.windsurf/rules/` ou `.windsurfrules`, e `.clinerules`

Você também pode executar [`/import`](/docs/pt/commands) para trazer a configuração de um agente de codificação suportado para Claude Code, que anexa uma cópia única de arquivos de instruções como `AGENTS.md` ao `CLAUDE.md` correspondente e carrega servidores MCP, comandos, subagentes e skills. Requer Claude Code v2.1.213 ou posterior.

<h2 id="auto-memory">
  Memória automática
</h2>

A memória automática permite que Claude acumule conhecimento entre sessões sem você escrever nada. Conforme funciona, Claude salva quatro tipos de notas para si mesma. Claude registra o tipo como um campo `type` no frontmatter do arquivo de memória:

* `user`: seu papel, expertise e preferências de trabalho
* `feedback`: correções que você dá a Claude e abordagens que você confirma
* `project`: trabalho em andamento, prazos e decisões que Claude não consegue derivar do código ou histórico do git
* `reference`: onde encontrar informações fora do projeto, como um rastreador de problemas ou dashboard

Claude pula qualquer coisa que possa derivar da base de código, como arquitetura, caminhos de arquivo ou correções de depuração. Também pula qualquer coisa que seus arquivos CLAUDE.md já dizem.

Claude não salva algo a cada sessão. Ela decide o que vale a pena lembrar com base em se a informação seria útil em uma conversa futura.

<h3 id="enable-or-disable-auto-memory">
  Ative ou desative a memória automática
</h3>

A memória automática está ativada por padrão. Para alterná-la, abra `/memory` em uma sessão e use o toggle de memória automática, que salva `autoMemoryEnabled` nas configurações do usuário em `~/.claude/settings.json`. Para desativá-la para um único projeto, defina `autoMemoryEnabled` nas configurações desse projeto:

```json theme={null}
{
  "autoMemoryEnabled": false
}
```

Para desabilitar a memória automática via variável de ambiente, defina `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`.

<h3 id="storage-location">
  Local de armazenamento
</h3>

Cada projeto obtém seu próprio diretório de memória em `~/.claude/projects/<project>/memory/`. O caminho `<project>` é derivado do repositório git, então todos os worktrees e subdiretórios dentro do mesmo repositório compartilham um diretório de memória automática. Fora de um repositório git, a raiz do projeto é usada em vez disso.

Se você definir [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/pt/sessions#name-the-project-directory-yourself) ao lado de `CLAUDE_CONFIG_DIR`, Claude Code usa esse nome como o diretório `<project>` sob `<config dir>/projects/` independentemente de qual repositório você o inicia, então projetos iniciados com esse diretório de configuração compartilham um diretório de memória automática. Requer Claude Code v2.1.234 ou posterior.

Para armazenar memória automática em um local diferente, defina `autoMemoryDirectory` em seu `settings.json`. Ele é lido de qualquer [escopo de configurações](/docs/pt/settings#settings-precedence): usuário, projeto, local, política, ou `--settings`.

```json theme={null}
{
  "autoMemoryDirectory": "~/my-custom-memory-dir"
}
```

O valor deve ser um caminho absoluto ou começar com `~/`.

Quando você o define no `.claude/settings.json` ou `.claude/settings.local.json` de um projeto, Claude Code o honra sob a mesma [regra de confiança de workspace que hooks em arquivos de configurações](/docs/pt/permissions#what-runs-before-you-trust-a-folder). Enquanto [`permissions.blockReadsOutsideWorkingDirectories`](/docs/pt/settings-reference#permissions-blockreadsoutsideworkingdirectories) está ativado, Claude Code não carrega nenhuma memória automática de um diretório que um [arquivo de configurações fornecido pelo repositório](/docs/pt/permissions#when-your-local-settings-file-needs-trust) escolhe e não salva nada nele, onde quer que esse diretório esteja.

O diretório contém um índice `MEMORY.md` e um arquivo de tópico por memória:

```text theme={null}
~/.claude/projects/<project>/memory/
├── MEMORY.md           # Índice, uma linha por memória, carregado em cada sessão
├── user_role.md        # Uma memória
├── feedback_testing.md # Uma memória
└── ...                 # Qualquer outro arquivo de tópico que Claude cria
```

`MEMORY.md` atua como um índice do diretório de memória. Claude lê e escreve arquivos neste diretório ao longo de sua sessão, usando `MEMORY.md` para acompanhar o que está armazenado onde.

A memória automática é local da máquina. Todos os worktrees e subdiretórios dentro do mesmo repositório git compartilham um diretório de memória automática. Os arquivos não são compartilhados entre máquinas ou ambientes em nuvem.

Claude Code exclui transcrições de sessão antigas após o período de retenção [`cleanupPeriodDays`](/docs/pt/settings-reference#cleanupperioddays), mas exclui os arquivos de memória no diretório de memória dessa [varredura de retenção](/docs/pt/claude-directory#cleaned-up-automatically). `MEMORY.md` e arquivos de tópico permanecem até você ou Claude editá-los ou deletá-los.

<h3 id="how-it-works">
  Como funciona
</h3>

As primeiras 200 linhas de `MEMORY.md`, ou os primeiros 25KB, o que vier primeiro, são carregados no início de cada conversa. Conteúdo além desse limite não é carregado no início da sessão. Claude mantém `MEMORY.md` conciso movendo notas detalhadas para arquivos de tópico separados.

Depois que Claude escreve em `MEMORY.md`, Claude Code mede o arquivo contra os limites de leitura de 200 linhas e 25KB. Se o arquivo está próximo de um limite, Claude Code lembra Claude de encurtá-lo: mantenha uma linha por entrada, mova detalhes para arquivos de tópico e mescle ou descarte entradas obsoletas. Se o arquivo está acima de um limite, a escrita ainda é bem-sucedida, mas Claude Code retorna um [erro dizendo a Claude para reescrever o índice](/docs/pt/errors#memory-index-is-over-its-read-limit), porque tudo além do limite é descartado no próximo carregamento.

Este limite se aplica apenas a `MEMORY.md`. Claude Code carrega um arquivo CLAUDE.md de até 4 MiB na íntegra e pula um arquivo maior. Arquivos mais curtos produzem melhor aderência.

Claude Code não carrega arquivos de tópico como `user_role.md` ou `feedback_testing.md` na inicialização. Claude os lê sob demanda usando suas ferramentas de arquivo padrão quando precisa da informação.

A memória automática da conversa principal não é carregada em [subagentes](/docs/pt/sub-agents#what-loads-at-startup); a exceção é um [fork](/docs/pt/sub-agents#fork-the-current-conversation), que herda a conversa pai e o prompt do sistema. A própria memória automática de um subagente, ativada com o campo `memory` do subagente, é um diretório separado.

Claude lê e escreve arquivos de memória durante sua sessão. Quando você vê mensagens como "Saved 2 memories" ou "Recalled 2 memories" na interface do Claude Code, Claude está ativamente atualizando ou lendo de `~/.claude/projects/<project>/memory/`.

Quando Claude escreve um arquivo de memória que começa com frontmatter YAML, Claude Code registra o tempo de escrita em um campo `modified` do frontmatter como um timestamp ISO 8601. O timestamp mostra como o fato é atual, tanto para você quanto para Claude quando o lê de volta. Qualquer arquivo que tenha frontmatter obtém o campo na próxima vez que Claude o escreve, incluindo arquivos criados em versões anteriores; Claude Code nunca adiciona frontmatter a um arquivo que não tenha nenhum. O campo `modified` requer Claude Code v2.1.214 ou posterior.

<h3 id="audit-and-edit-your-memory">
  Audite e edite sua memória
</h3>

Arquivos de memória automática são markdown simples que você pode editar ou deletar a qualquer momento. Execute [`/memory`](#view-and-edit-with-%2Fmemory) para navegar e abrir arquivos de memória de dentro de uma sessão.

<h2 id="view-and-edit-with-/memory">
  Visualize e edite com `/memory`
</h2>

O comando `/memory` lista seus arquivos CLAUDE.md, CLAUDE.local.md e outros locais de arquivo de memória em escopos de usuário e projeto, incluindo entradas CLAUDE.md de usuário e projeto para arquivos que ainda não existem. Também permite que você alterne a memória automática ativada ou desativada e fornece uma opção para abrir a pasta de memória automática. Selecione qualquer arquivo para abri-lo no seu editor; selecionar um que ainda não existe o cria primeiro. Para verificar quais arquivos `CLAUDE.md` e arquivos de regras foram carregados na sessão atual, execute `/context`.

Editores GUI como VS Code abrem o arquivo em uma janela separada, e você pode continuar usando a sessão enquanto está aberta. Antes da v2.1.216, `/memory` esperava você fechar o arquivo antes de responder. Editores de terminal como Vim assumem o controle do terminal até você sair.

Quando você pede a Claude para lembrar algo, como "sempre use pnpm, não npm" ou "lembre-se de que os testes de API requerem uma instância local de Redis," Claude salva em memória automática. Para adicionar instruções a CLAUDE.md em vez disso, peça a Claude diretamente, como "adicione isto a CLAUDE.md," ou edite o arquivo você mesmo via `/memory`.

<h2 id="troubleshoot-memory-issues">
  Solucione problemas de memória
</h2>

Estes são os problemas mais comuns com CLAUDE.md e memória automática, junto com passos para depurá-los.

<h3 id="claude-isn’t-following-my-claude-md">
  Claude não está seguindo meu CLAUDE.md
</h3>

O conteúdo de CLAUDE.md é entregue como uma mensagem de usuário após o prompt do sistema, não como parte do próprio prompt do sistema. Claude o lê e tenta segui-lo, mas não há garantia de conformidade estrita, especialmente para instruções vagas ou conflitantes.

Para depurar:

* Execute `/context` e verifique a lista sob **Memory files** para verificar se seus arquivos CLAUDE.md e CLAUDE.local.md foram carregados. Se um arquivo `CLAUDE.md` estiver faltando lá, Claude não pode vê-lo. Use `/memory` para abrir e editar os arquivos.
* Verifique se o CLAUDE.md relevante está em um local que é carregado para sua sessão (veja [Escolha onde colocar arquivos CLAUDE.md](#choose-where-to-put-claude-md-files)).
* Torne as instruções mais específicas. "Use indentação de 2 espaços" funciona melhor do que "formate o código adequadamente."
* Procure por instruções conflitantes entre arquivos CLAUDE.md. Se dois arquivos dão orientação diferente para o mesmo comportamento, Claude pode escolher um arbitrariamente.

Se a instrução é algo que deve ser executado em um ponto específico, como antes de cada commit ou após cada edição de arquivo, escreva-a como um [hook](/docs/pt/hooks-guide) em vez disso. Hooks são executados como comandos shell em eventos de ciclo de vida fixos e se aplicam independentemente do que Claude decidir fazer.

Para instruções que você quer no nível do prompt do sistema, use [`--append-system-prompt`](/docs/pt/cli-reference#system-prompt-flags). Você passa isso no lançamento, então é mais adequado para scripts e automação do que para uso interativo. Para como se comporta quando você retoma uma conversa, veja [Sinalizadores de prompt do sistema em conversas retomadas](/docs/pt/cli-reference#system-prompt-flags-in-resumed-conversations).

<Tip>
  Use o hook [`InstructionsLoaded`](/docs/pt/hooks#instructionsloaded) para registrar exatamente quais arquivos `CLAUDE.md` e arquivos de regras são carregados, quando são carregados e por quê. Isso é útil para depurar regras específicas de caminho ou arquivos carregados preguiçosamente em subdiretórios.
</Tip>

<h3 id="my-agents-md-isn’t-loading">
  Meu AGENTS.md não está carregando
</h3>

Se seu repositório tem um `AGENTS.md` e Claude não parece saber o que ele diz, a causa usual é um `CLAUDE.md` em algum lugar no caminho do projeto. Por padrão, Claude lê `AGENTS.md` apenas quando você não tem `CLAUDE.md` ou `CLAUDE.local.md` em seu diretório de trabalho ou acima dele. Verifique estes em ordem:

1. Procure por um `CLAUDE.md`, `.claude/CLAUDE.md`, ou `CLAUDE.local.md` em seu diretório de trabalho ou em qualquer diretório acima dele, exceto seu `~/.claude/CLAUDE.md`. Se você encontrar um, Claude o lê em vez de `AGENTS.md` a menos que você defina **Project instructions** como `claude-md-and-agents-md`.
2. Execute `claude --version` e confirme v2.1.277 ou posterior. Antes da v2.1.281, algumas sessões, como aquelas no Amazon Bedrock ou com telemetria desabilitada, [não conseguiam carregar `AGENTS.md`](#when-agents-md-support-is-unavailable) também, então nessas versões atualize para v2.1.281 ou posterior.
3. Digite `/config` em sua sessão para abrir o painel de configurações e confirme que **Project instructions** não está definido como `claude-md` ou `managed-only`. Se você não vir a configuração lá em absoluto, sua sessão é uma que [não pode carregar `AGENTS.md`](#when-agents-md-support-is-unavailable).

Para verificar se Claude leu seu `AGENTS.md`, execute `/memory` e procure por seu caminho na lista.

Antes da v2.1.280, `/memory` e `/context` não listavam um `AGENTS.md` que Claude leu diretamente. Nessas versões, pergunte a Claude o que suas instruções de projeto dizem em vez disso.

Se você quiser manter o `CLAUDE.md` que encontrou, ou sua sessão não pode carregar `AGENTS.md`, [adicione um `CLAUDE.md` ao lado de seu `AGENTS.md` que o importa](#share-one-file-with-other-coding-tools).

<h3 id="i-don’t-know-what-auto-memory-saved">
  Não sei o que a memória automática salvou
</h3>

Execute `/memory` e selecione a pasta de memória automática para navegar o que Claude salvou. Tudo é markdown simples que você pode ler, editar ou deletar.

<h3 id="my-claude-md-is-too-large">
  Meu CLAUDE.md é muito grande
</h3>

Arquivos com mais de 200 linhas consomem mais contexto e podem reduzir a aderência. Claude Code pula um arquivo com mais de 4 MiB. Use [regras com escopo de caminho](#path-specific-rules) para carregar instruções apenas quando Claude trabalha com arquivos correspondentes, ou reduza conteúdo que não é necessário em cada sessão. Dividir em [importações `@path`](#import-additional-files) ajuda na organização, mas não reduz contexto, já que arquivos importados são carregados no lançamento.

O checkup [`/doctor`](/docs/pt/commands#all-commands) propõe cortes para um CLAUDE.md verificado: ele corta conteúdo que Claude pode derivar da base de código, como layouts de diretório, listas de dependências e visões gerais de arquitetura, e mantém armadilhas, justificativa e convenções que diferem dos padrões de ferramentas. A verificação de corte requer Claude Code v2.1.206 ou posterior.

<h3 id="instructions-seem-lost-after-/compact">
  Instruções parecem perdidas após `/compact`
</h3>

CLAUDE.md de raiz de projeto sobrevive à compactação: após `/compact`, Claude relê do disco e reinjecta no contexto. Arquivos CLAUDE.md aninhados em subdiretórios e regras com [frontmatter `paths:`](#path-specific-rules) recarregam conforme Claude lê arquivos aos quais se aplicam.

Se uma instrução desapareceu após compactação, ela foi dada apenas em conversa, vive em um CLAUDE.md aninhado que ainda não recarregou, ou é uma regra com escopo de caminho que não correspondeu a um arquivo desde então. Adicione instruções apenas de conversa a CLAUDE.md para torná-las persistir. Veja [O que sobrevive à compactação](/docs/pt/context-window#what-survives-compaction) para o detalhamento completo.

Veja [Escreva instruções eficazes](#write-effective-instructions) para orientação sobre tamanho, estrutura e especificidade.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Debug sua configuração](/docs/pt/debug-your-config): diagnostique por que CLAUDE.md ou configurações não estão tendo efeito
* [Skills](/docs/pt/skills): empacote fluxos de trabalho repetíveis que carregam sob demanda
* [Settings](/docs/pt/settings): configure o comportamento do Claude Code com arquivos de configurações
* [Memória de subagent](/docs/pt/sub-agents#enable-persistent-memory): deixe subagents manter sua própria memória automática
