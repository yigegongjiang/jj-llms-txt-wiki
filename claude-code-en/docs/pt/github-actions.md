> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions

> Execute Claude Code em fluxos de trabalho do GitHub Actions para responder a menções @claude, automatizar tarefas e transformar issues em pull requests

[Claude Code GitHub Actions](https://github.com/anthropics/claude-code-action) é uma GitHub Action que executa Claude Code dentro dos fluxos de trabalho do seu repositório. Mencione `@claude` em um comentário de pull request ou issue para que Claude analise código, implemente alterações e faça push de commits. Você também pode fornecer um prompt à Claude Code GitHub Action para executar automaticamente em qualquer evento do GitHub. Use-a para transformar issues em pull requests, corrigir bugs a partir de um comentário ou automatizar tarefas recorrentes.

Vários produtos compartilham o nome Claude Code. Esta página cobre a integração de fluxo de trabalho `claude-code-action`, que você configura com arquivos de fluxo de trabalho em seu repositório. Para os produtos relacionados, consulte:

* [Code Review](/docs/pt/code-review): revisão automática em cada pull request, sem escrever um fluxo de trabalho
* [Claude Code na web](/docs/pt/claude-code-on-the-web): sessões de Claude Code que executam em infraestrutura em nuvem em vez de sua máquina
* [Claude Agent SDK](/docs/pt/agent-sdk/overview): automação personalizada fora do GitHub Actions. A Claude Code GitHub Action é construída sobre o SDK
* [GitHub Enterprise Server](/docs/pt/github-enterprise-server): Claude Code com GitHub auto-hospedado

<h2 id="setup">
  Configuração
</h2>

Você pode configurar a Claude Code GitHub Action de duas maneiras:

* **Configuração rápida**: execute `/install-github-app` a partir do Claude Code. Claude Code instala a GitHub App, adiciona seu secret de autenticação e prepara o pull request de fluxo de trabalho para você
* **Configuração manual**: instale a app, adicione o secret e copie o arquivo de fluxo de trabalho em seu repositório você mesmo. Use este caminho quando você não executa Claude Code localmente, quando o comando falha ou quando você quer controle total dos arquivos de fluxo de trabalho

Para qualquer caminho, você precisa de acesso de administrador ao repositório.

<h3 id="quick-setup">
  Configuração rápida
</h3>

`/install-github-app` funciona apenas com repositórios github.com. Se o git remote do seu repositório estiver em gitlab.com ou bitbucket.org, o comando imprime um aviso e sai em vez de iniciar a configuração. Para executar Claude Code a partir de pipelines do GitLab, consulte [Claude Code GitLab CI/CD](/docs/pt/gitlab-ci-cd).

Antes de começar, instale a [GitHub CLI](https://cli.github.com) e autentique-a com `gh auth login`. Claude Code verifica se existe e avisa você se estiver faltando.

Abra `claude` no repositório que você quer conectar, execute `/install-github-app` e siga os prompts. Claude Code instala a Claude GitHub App e depois configura um secret de autenticação para os fluxos de trabalho:

* Se Claude Code já tiver uma chave de API, ele reutiliza essa chave e oferece manter o secret `ANTHROPIC_API_KEY` existente do repositório se um já estiver definido
* Caso contrário, escolha entre criar um token de longa duração com sua assinatura Claude e colar uma chave de API

Claude Code salva a credencial como um secret do repositório, nomeado `ANTHROPIC_API_KEY` para uma chave de API ou `CLAUDE_CODE_OAUTH_TOKEN` para um token de assinatura.

Claude Code então faz push de um branch com os arquivos de fluxo de trabalho que você seleciona, já configurados para usar esse secret, e abre o GitHub em seu navegador com um pull request pronto para criar. Crie e faça merge desse pull request, e `@claude` funciona no repositório.

Se você selecionar o fluxo de trabalho de revisão, Claude publica cada revisão no próprio pull request, como um comentário inline em cada issue que encontra ou como um comentário de resumo quando não encontra nenhum. Claude pula alguns pull requests, como rascunhos. O [exemplo de fluxo de trabalho de revisão](#run-a-skill) usa a mesma skill e os lista. Antes da v2.1.229, Claude escrevia sua revisão apenas no log de execução do fluxo de trabalho.

Para atualizar um fluxo de trabalho de revisão que uma versão anterior gerou, faça um dos seguintes:

* Execute `/install-github-app` novamente. Quando o repositório já tiver um `claude.yml`, selecione **Update workflow file with latest version**. Claude Code faz push de cópias novas dos arquivos de fluxo de trabalho para um novo branch e abre o pull request, igual a uma primeira instalação.
* Adicione o argumento `--comment` e a linha `claude_args` do [exemplo de fluxo de trabalho de revisão](#run-a-skill) ao arquivo verificado você mesmo, o que mantém quaisquer outras edições que você fez nele.

Após instalar a GitHub App, Claude Code pergunta se deseja continuar com a configuração do GitHub Actions. Escolha **Skip for now** para parar apenas com a GitHub App instalada. Execute `/install-github-app` novamente mais tarde para terminar os passos de fluxo de trabalho e secret.

<Note>
  * Quando você instala a GitHub App, você concede a ela várias permissões. Consulte [Permissões da GitHub App](#github-app-permissions) para o conjunto completo
  * A configuração rápida funciona com a Claude API e assinaturas Claude. Se você usar Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry, consulte [Use Claude Code GitHub Actions com provedores de nuvem](/docs/pt/github-actions-cloud-providers)
</Note>

<h3 id="manual-setup">
  Configuração manual
</h3>

Para configurar a Claude Code GitHub Action sem executar `/install-github-app`, instale a app, adicione um secret e copie um arquivo de fluxo de trabalho você mesmo:

<Steps>
  <Step title="Instale a Claude GitHub App">
    Instale a [Claude GitHub App](https://github.com/apps/claude) em seu repositório. A Claude Code GitHub Action depende de três das permissões da app:

    * **Contents**: leitura e escrita, para que Claude possa modificar arquivos do repositório
    * **Issues**: leitura e escrita, para que Claude possa responder a issues
    * **Pull requests**: leitura e escrita, para que Claude possa criar PRs e fazer push de alterações

    Durante a instalação, você também concede permissões que outros recursos Claude usam. Consulte [Permissões da GitHub App](#github-app-permissions) para o conjunto completo.
  </Step>

  <Step title="Adicione um secret de autenticação">
    Adicione um dos seguintes secrets ao seu repositório, dependendo de como você se autentica. Consulte o guia do GitHub sobre [usando secrets no GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions).

    * `ANTHROPIC_API_KEY`: uma chave de API Claude do [Claude Console](https://platform.claude.com)
    * `CLAUDE_CODE_OAUTH_TOKEN`: um token OAuth que se autentica com sua assinatura Claude, disponível em planos Pro, Max, Team e Enterprise. Gere um executando `claude setup-token` localmente. Consulte [Gerar um token de longa duração](/docs/pt/authentication#generate-a-long-lived-token)

    Em arquivos de fluxo de trabalho, passe o secret para a entrada correspondente: `anthropic_api_key` para uma chave de API ou `claude_code_oauth_token` para um token OAuth.
  </Step>

  <Step title="Copie o arquivo de fluxo de trabalho">
    Copie [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) para o diretório `.github/workflows/` do seu repositório. O arquivo é um fluxo de trabalho funcional, não apenas um exemplo. Conforme confirmado, Claude responde sempre que alguém menciona `@claude` em uma issue ou pull request, autenticando com o secret `ANTHROPIC_API_KEY`. Se você adicionou `CLAUDE_CODE_OAUTH_TOKEN` em vez disso, altere a linha `anthropic_api_key` do fluxo de trabalho para `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`.
  </Step>
</Steps>

<Tip>
  Após a configuração, teste a Claude Code GitHub Action marcando `@claude` em um comentário de issue ou PR.
</Tip>

<h3 id="set-up-for-an-organization">
  Configurar para uma organização
</h3>

Com configuração rápida ou manual, você configura um repositório por vez. Para distribuir a Claude Code GitHub Action em uma organização:

* Instale a [Claude GitHub App](https://github.com/apps/claude) uma vez no nível da organização, escolhendo todos os repositórios ou uma lista selecionada
* Armazene o secret de autenticação como um secret de Actions no nível da organização para que cada repositório não precise de sua própria cópia
* Adicione o arquivo de fluxo de trabalho a cada repositório que deve executar a Claude Code GitHub Action, ou defina o job uma vez como um [fluxo de trabalho reutilizável](https://docs.github.com/en/actions/using-workflows/reusing-workflows) que cada repositório chama

Para um secret compartilhado entre repositórios, autentique com uma chave de API do [Claude Console](https://platform.claude.com) em vez de um token OAuth, já que um token OAuth está vinculado à assinatura da pessoa que executou `claude setup-token`.

Para evitar armazenar um secret de longa duração completamente, autentique através de federação de identidade de workload, onde a Claude Code GitHub Action troca o token OpenID Connect (OIDC) do GitHub do fluxo de trabalho por acesso à Claude API através de uma conta de serviço do Claude Console. Defina estas entradas:

* `anthropic_federation_rule_id`: o ID da regra de federação, `fdrl_...`
* `anthropic_organization_id`: seu ID de organização Anthropic
* `anthropic_service_account_id`: o ID da conta de serviço, `svac_...`. Opcional, já que a regra de federação que você cria no Console já tem como alvo uma conta de serviço
* `anthropic_workspace_id`: o ID do workspace, `wrkspc_...`. Opcional quando a regra de federação tem como alvo um único workspace

Conceda ao fluxo de trabalho a permissão `id-token: write`, que a Claude Code GitHub Action precisa para a troca de federação mesmo quando você passa seu próprio `github_token`. Consulte o [guia de configuração da Claude Code GitHub Action](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md) para a configuração do lado do Console.

Para perguntas sobre tratamento de dados e retenção em uma revisão de segurança, consulte [uso de dados](/docs/pt/data-usage) e [segurança](/docs/pt/security).

<h3 id="uninstall">
  Desinstalar
</h3>

Para remover a Claude Code GitHub Action, desfaça cada parte da configuração que se aplica à sua instalação:

* **Arquivos de fluxo de trabalho**: delete os fluxos de trabalho que usam `anthropics/claude-code-action` de `.github/workflows/`. Se você usou configuração rápida, procure por `claude.yml` e, se você selecionou o fluxo de trabalho de revisão, `claude-code-review.yml`. Com os fluxos de trabalho deletados, a Claude Code GitHub Action não é mais executada
* **Secrets**: delete o secret `ANTHROPIC_API_KEY` ou `CLAUDE_CODE_OAUTH_TOKEN` do repositório e dos secrets de Actions no nível da organização se você [compartilhou entre repositórios](#set-up-for-an-organization). Se você deletar um secret, a credencial que ele continha permanece válida. Para desativar uma chave de API completamente, também delete a chave no [Claude Console](https://platform.claude.com)
* **GitHub App**: desinstale a Claude GitHub App nas configurações do seu repositório ou organização em GitHub Apps, mas apenas se você não a usar para outro recurso Claude, como Code Review ou web auto-fix

Se você configurou um [provedor de nuvem](/docs/pt/github-actions-cloud-providers), também delete os secrets do provedor, como `AWS_ROLE_TO_ASSUME`, os secrets `GCP_*` ou os secrets `AZURE_*`, e desinstale a GitHub App personalizada junto com seus secrets `APP_ID` e `APP_PRIVATE_KEY`.

<h3 id="github-app-permissions">
  Permissões da GitHub App
</h3>

A [Claude GitHub App](https://github.com/apps/claude) é compartilhada por cada recurso Claude que se integra com o GitHub, incluindo a Claude Code GitHub Action, [Code Review](/docs/pt/code-review) e [auto-fix para pull requests](/docs/pt/claude-code-on-the-web#auto-fix-pull-requests) em sessões na nuvem. Uma GitHub App tem um único conjunto de permissões cobrindo todos os seus recursos, então o conjunto inclui algumas permissões que a Claude Code GitHub Action não usa.

Quando você instala a app, você concede as seguintes permissões:

| Permissão        | Acesso            |
| ---------------- | ----------------- |
| Actions          | Leitura e escrita |
| Checks           | Leitura e escrita |
| Contents         | Leitura e escrita |
| Discussions      | Leitura e escrita |
| Issues           | Leitura e escrita |
| Members          | Leitura           |
| Metadata         | Leitura           |
| Pull requests    | Leitura e escrita |
| Repository hooks | Leitura e escrita |
| Statuses         | Leitura           |
| Workflows        | Leitura e escrita |

O conjunto de permissões também pode mudar antes dos recursos que o usam. Quando a app solicita uma permissão que não tinha antes, o GitHub solicita ao proprietário da conta que a aprove, um proprietário da organização para uma instalação de organização, e a instalação mantém suas permissões antigas até que façam isso. Por exemplo, quando o acesso de Actions muda de leitura para escrita, a app pode re-executar fluxos de trabalho em vez de apenas visualizar execuções e logs, então o GitHub pede ao proprietário que aprove a alteração.

Quando você instala a app, você aceita seu conjunto completo de permissões. O GitHub não permite que você aceite um subconjunto. Se sua organização exigir apenas as permissões que a Claude Code GitHub Action usa, crie uma GitHub App personalizada com Contents, Issues e Pull requests em vez disso, seguindo o [guia de configuração da Claude Code GitHub Action](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md). Uma app personalizada cobre apenas a Claude Code GitHub Action. Code Review e web auto-fix ainda exigem a app oficial.

Para detalhes sobre como a Claude Code GitHub Action limita o que Claude pode fazer com essas permissões, consulte a [documentação de segurança](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

<h2 id="interactive-and-automation-modes">
  Modos interativo e de automação
</h2>

A Claude Code GitHub Action detecta como executar a partir de sua configuração de fluxo de trabalho:

* **Modo interativo**: quando o fluxo de trabalho não fornece entrada `prompt`, Claude aguarda a frase de gatilho, `@claude` por padrão, em um comentário de issue ou pull request, em uma revisão de pull request ou no corpo ou título de uma issue recém-aberta, então responde a essa solicitação. Progresso e resultados aparecem como um comentário na issue ou PR que acionou.
* **Modo de automação**: quando o fluxo de trabalho fornece uma entrada `prompt`, Claude é executado sem aguardar uma menção, sujeito apenas às [verificações sobre quem pode acionar execuções](#who-can-trigger-runs). Por padrão, os resultados aparecem no log de execução do fluxo de trabalho em vez de um comentário. Claude pode postar na issue ou pull request quando o prompt o direciona e ele tem uma ferramenta que pode postar, como no [exemplo de code-review](#run-a-skill).

<h3 id="who-can-trigger-runs">
  Quem pode acionar execuções
</h3>

Em ambos os modos, a Claude Code GitHub Action executa duas verificações no ator que aciona antes de Claude começar, e a execução falha quando qualquer verificação a rejeita:

* **Acesso de escrita**: em eventos de issue e pull request, o usuário que aciona deve ter acesso de escrita ao repositório. Para permitir usuários específicos sem acesso de escrita, defina `allowed_non_write_users` e passe sua própria entrada `github_token`. Eventos que nenhum usuário cria, como um gatilho `schedule`, pulam essa verificação.
* **Ator humano**: em cada evento, a Claude Code GitHub Action rejeita um ator bot a menos que você o liste em `allowed_bots`, o que impede que bots acionem Claude em um loop. Essa verificação também se aplica a execuções agendadas, que o GitHub atribui a um usuário do repositório, geralmente aquele que alterou pela última vez o cronograma `cron` do fluxo de trabalho. Se esse usuário for um bot, liste-o em `allowed_bots`.

<h2 id="example-use-cases">
  Exemplos de casos de uso
</h2>

O [diretório de exemplos](https://github.com/anthropics/claude-code-action/tree/main/examples) contém fluxos de trabalho prontos para uso em diferentes cenários.

Os exemplos nesta página mostram autenticação de chave de API. Se você se autenticar com uma assinatura Claude, substitua a linha `anthropic_api_key` em qualquer exemplo por `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`.

<h3 id="respond-to-claude-mentions">
  Responder a menções @claude
</h3>

Este fluxo de trabalho executa a Claude Code GitHub Action em modo interativo, para que Claude responda sempre que alguém menciona `@claude` em um comentário de issue ou PR.

```yaml theme={null}
name: Claude Code
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
jobs:
  claude:
    if: contains(github.event.comment.body, '@claude')
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
      issues: write
      id-token: write
      actions: read
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
```

As partes deste fluxo de trabalho que não são boilerplate:

* `id-token: write`: necessário para a autenticação padrão da GitHub App da Claude Code GitHub Action
* `actions: read`: permite que Claude leia resultados de CI em PRs
* `actions/checkout`: fornece a Claude uma cópia local do repositório para trabalhar
* `if`: impede que runners iniciem em comentários que não mencionam `@claude`. A Claude Code GitHub Action também verifica a frase de gatilho em si antes de responder

Uma vez que o fluxo de trabalho está em vigor, mencione `@claude` em qualquer comentário de issue ou PR com uma solicitação:

```text wrap theme={null}
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude responde em um comentário na mesma issue ou PR e a atualiza conforme trabalha.

<h3 id="run-a-skill">
  Executar uma skill
</h3>

A entrada `prompt` aceita uma invocação de [skill](/docs/pt/skills) bem como texto simples:

* Para uma skill no diretório `.claude/skills/` do seu repositório, execute `actions/checkout` antes da etapa `anthropics/claude-code-action` para que os arquivos de skill estejam disponíveis no runner, então passe `/skill-name` como o `prompt`.
* Para uma skill empacotada em um [plugin](/docs/pt/plugins/overview), instale o plugin com as entradas `plugin_marketplaces` e `plugins`, então passe o `/plugin-name:skill-name` com namespace como o `prompt`. A entrada `plugins` recebe `plugin-name@marketplace-name`, onde o nome do marketplace vem do próprio manifesto do marketplace em vez de sua URL de repositório.

O fluxo de trabalho a seguir instala o plugin `code-review` e executa sua skill quando um pull request é aberto, atualizado, reaberto ou marcado como pronto para revisão. Ele executa o mesmo plugin que o fluxo de trabalho de revisão da configuração rápida. Use um fluxo de trabalho como este quando você quer controlar o prompt, modelo e gatilhos você mesmo. Para revisões automáticas sem manter um arquivo de fluxo de trabalho, consulte [Code Review](/docs/pt/code-review). Em repositórios públicos, o GitHub retém secrets de execuções acionadas por pull requests de fork, então a revisão é executada apenas em pull requests de branches no mesmo repositório.

```yaml theme={null}
name: Code Review
on:
  pull_request:
    types: [opened, synchronize, ready_for_review, reopened]
jobs:
  review:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: read
      issues: read
      id-token: write
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          plugin_marketplaces: "https://github.com/anthropics/claude-code.git"
          plugins: "code-review@claude-code-plugins"
          prompt: "/code-review:code-review --comment ${{ github.repository }}/pull/${{ github.event.pull_request.number }}"
          claude_args: '--allowedTools "mcp__github_inline_comment__create_inline_comment"'
```

Duas linhas neste fluxo de trabalho controlam onde a revisão vai:

* **`--comment`**: Claude publica sua revisão no pull request, como um comentário inline em cada issue que encontra ou como um comentário de resumo quando não encontra nenhum. Sem ele, Claude não publica nada, e você lê os achados no log de execução do fluxo de trabalho.
* **`claude_args`**: mantenha esta linha mesmo que o frontmatter `allowed-tools` próprio da skill nomeie a mesma ferramenta, porque a Claude Code GitHub Action inicia o servidor MCP que publica comentários inline apenas quando `--allowedTools` em `claude_args` o nomeia.

Claude pula pull requests de rascunho e fechados, pull requests que ele julga não precisarem de revisão, como automatizados ou triviais, e pull requests que já têm um comentário de Claude.

<h3 id="run-on-a-schedule">
  Executar em um cronograma
</h3>

Com uma entrada `prompt`, a Claude Code GitHub Action é executada em modo de automação em qualquer evento do GitHub, incluindo um cronograma cron. Para um prompt de texto simples, Claude não tem acesso a shell ou GitHub API até que você conceda as ferramentas que o prompt precisa, com `--allowedTools` em `claude_args` ou uma [regra `permissions.allow`](/docs/pt/permissions#permission-rule-syntax) na entrada `settings`. Se você invocar uma skill em vez disso, Claude pode usar as ferramentas que seu [frontmatter `allowed-tools`](/docs/pt/skills#pre-approve-tools-for-a-skill) concede. O GitHub executa fluxos de trabalho agendados apenas a partir do branch padrão e, em repositórios públicos, desabilita o cronograma após 60 dias sem atividade do repositório.

Este fluxo de trabalho gera um relatório no log de execução do fluxo de trabalho às 09:00 UTC cada dia. Sua linha `claude_args` [passa argumentos de CLI](#pass-cli-arguments) que selecionam o modelo e permitem duas ferramentas MCP do GitHub. Claude lê commits e issues através da GitHub API com essas ferramentas, então você pode omitir a etapa de checkout:

```yaml theme={null}
name: Daily Report
on:
  schedule:
    - cron: "0 9 * * *"
jobs:
  report:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      issues: read
      id-token: write
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "Generate a summary of yesterday's commits and open issues"
          claude_args: |
            --model claude-opus-5-5
            --allowedTools "mcp__github__list_commits,mcp__github__list_issues"
```

<h2 id="best-practices">
  Melhores práticas
</h2>

<h3 id="define-project-standards-in-claude-md">
  Defina padrões de projeto em CLAUDE.md
</h3>

Crie um arquivo `CLAUDE.md` na raiz do seu repositório para definir diretrizes de estilo de código, critérios de revisão, regras específicas do projeto e padrões preferidos. Claude segue essas diretrizes ao criar PRs e responder a solicitações. Consulte a [documentação de memory](/docs/pt/memory) para detalhes.

<h3 id="protect-your-credentials">
  Proteja suas credenciais
</h3>

<Warning>
  Nunca faça commit de chaves de API ou tokens OAuth diretamente em seu repositório. Sempre armazene-os como GitHub Secrets e referencie-os em fluxos de trabalho, por exemplo `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`.
</Warning>

Conceda ao fluxo de trabalho apenas as permissões que ele precisa e revise as alterações de Claude antes de fazer merge.

Para orientação abrangente de segurança incluindo permissões e autenticação, consulte a [documentação de segurança da Claude Code Action](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

<h3 id="manage-costs">
  Gerencie custos
</h3>

Cada execução consome dois tipos de recursos:

* **Minutos do GitHub Actions**: a Claude Code GitHub Action é executada em runners hospedados pelo GitHub, que consomem seus minutos do GitHub Actions. Consulte a [documentação de faturamento do GitHub](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions) para preços e limites de minutos.
* **Tokens de API**: cada interação consome tokens com base no comprimento de prompts e respostas, complexidade da tarefa e tamanho da base de código. Consulte a [página de preços do Claude](https://claude.com/platform/api) para taxas de token atuais. Se você se autenticar com um token OAuth, as execuções usam sua assinatura Claude em vez de faturamento de API.

Você pode reduzir ambos os tipos de custo dando a Claude contexto mais claro e limitando quanto trabalho cada execução pode fazer:

* Escreva solicitações específicas de `@claude` para que Claude precise de menos turnos para terminar
* Use templates de issue para fornecer contexto antecipadamente
* Mantenha seu `CLAUDE.md` conciso, já que Claude o lê em cada execução
* Defina `--max-turns` em `claude_args` para limitar iterações
* Defina timeouts no nível do fluxo de trabalho para evitar jobs descontrolados
* Use controles de concorrência do GitHub para limitar execuções paralelas

Para rastreamento de uso em sua organização, consulte o [painel de análise](/docs/pt/analytics) e [monitoramento](/docs/pt/monitoring-usage). Para como o uso é medido e faturado, consulte [custos](/docs/pt/costs).

<h2 id="use-a-cloud-provider">
  Use um provedor de nuvem
</h2>

Por padrão, a Claude Code GitHub Action chama a Claude API diretamente com sua chave de API ou token OAuth. Para rotear inferência através de sua própria conta de nuvem em vez disso, defina a entrada para seu provedor e siga [Use Claude Code GitHub Actions com provedores de nuvem](/docs/pt/github-actions-cloud-providers):

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud's Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

Com todos os três provedores, você se autentica através de federação de identidade OIDC em vez de uma chave de API Claude, então você não armazena credenciais de nuvem estáticas em seu repositório.

<h2 id="troubleshooting">
  Solução de problemas
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude não responde aos comandos @claude
</h3>

* Verifique se a GitHub App está instalada no repositório
* Verifique se os fluxos de trabalho estão habilitados para o repositório
* Garanta que sua chave de API ou token OAuth esteja definido em secrets do repositório
* Confirme que o comentário contém `@claude` como uma palavra completa, não `/claude` ou `@claude-bot`
* Confirme que o usuário que comenta tem acesso de escrita ao repositório. Consulte [Quem pode acionar execuções](#who-can-trigger-runs) para as exceções

<h3 id="ci-not-running-on-claude’s-commits">
  CI não está sendo executado nos commits de Claude
</h3>

* O GitHub não aciona fluxos de trabalho em commits feitos com o `GITHUB_TOKEN` padrão. Se você passar `github_token: ${{ secrets.GITHUB_TOKEN }}` para a Claude Code GitHub Action, remova-o para que ele se autentique como a Claude GitHub App, ou passe um token de app personalizado em vez disso
* Verifique se os gatilhos do fluxo de trabalho de CI incluem os eventos que os pushes de Claude produzem, como `push` ou `pull_request`

<h3 id="authentication-errors">
  Erros de autenticação
</h3>

* Confirme que a chave de API ou token OAuth é válido testando-o localmente com `claude` antes de depurar o fluxo de trabalho
* Para Bedrock, Agent Platform e Foundry, consulte a [seção de solução de problemas](/docs/pt/github-actions-cloud-providers#troubleshooting) da página do provedor de nuvem

Para mais soluções, consulte o [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md) da Claude Code GitHub Action.

<h2 id="advanced-configuration">
  Configuração avançada
</h2>

<h3 id="action-parameters">
  Parâmetros da Action
</h3>

Estas são as entradas mais comumente usadas. Cada uma mapeia para uma chave `with:` na etapa `anthropics/claude-code-action`.

| Parâmetro                 | Descrição                                                                                                                                                                                | Necessário                                                                                                                                                                                 |
| ------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `prompt`                  | Instruções para Claude, como texto simples ou uma invocação de [skill](/docs/pt/skills). Quando omitido, Claude responde à [frase de gatilho](#interactive-and-automation-modes) em vez disso | Não                                                                                                                                                                                        |
| `claude_args`             | Argumentos de CLI passados para Claude Code                                                                                                                                              | Não                                                                                                                                                                                        |
| `anthropic_api_key`       | Chave de API Claude                                                                                                                                                                      | Para a Claude API, a menos que você use `claude_code_oauth_token` ou [federação de identidade de workload](#set-up-for-an-organization). Não usado para Bedrock, Agent Platform ou Foundry |
| `claude_code_oauth_token` | Token OAuth para autenticação com uma assinatura Claude, gerado com `claude setup-token`                                                                                                 | Não                                                                                                                                                                                        |
| `github_token`            | Token para operações do GitHub. Quando omitido, a Claude Code GitHub Action se autentica como a Claude GitHub App                                                                        | Não                                                                                                                                                                                        |
| `plugin_marketplaces`     | Lista separada por quebra de linha de URLs Git do marketplace de plugins                                                                                                                 | Não                                                                                                                                                                                        |
| `plugins`                 | Lista separada por quebra de linha de nomes de plugins para instalar antes da execução                                                                                                   | Não                                                                                                                                                                                        |
| `settings`                | Configurações de Claude Code, como uma string JSON ou um caminho para um arquivo JSON de configurações                                                                                   | Não                                                                                                                                                                                        |
| `trigger_phrase`          | Frase de gatilho que Claude responde. Padrão: `@claude`                                                                                                                                  | Não                                                                                                                                                                                        |
| `use_bedrock`             | Use Amazon Bedrock em vez da Claude API                                                                                                                                                  | Não                                                                                                                                                                                        |
| `use_vertex`              | Use Google Cloud's Agent Platform em vez da Claude API                                                                                                                                   | Não                                                                                                                                                                                        |
| `use_foundry`             | Use Microsoft Foundry em vez da Claude API                                                                                                                                               | Não                                                                                                                                                                                        |

Para a lista completa de entradas, consulte a [referência de configuração](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs) da Claude Code GitHub Action.

<h3 id="pass-cli-arguments">
  Passe argumentos de CLI
</h3>

O parâmetro `claude_args` aceita qualquer [argumento de CLI do Claude Code](/docs/pt/cli-reference):

```yaml theme={null}
claude_args: "--max-turns 5 --model claude-sonnet-5 --mcp-config /path/to/config.json"
```

Argumentos comuns:

* `--max-turns`: limita o número de turnos de conversa
* `--model`: modelo a usar, por exemplo `claude-sonnet-5`. Sem este argumento, a Claude Code GitHub Action usa o [modelo padrão](/docs/pt/model-config) do Claude Code
* `--mcp-config`: caminho para [configuração MCP](/docs/pt/mcp)
* `--allowedTools`: lista separada por vírgula de ferramentas permitidas. O alias `--allowed-tools` também funciona
* `--debug`: habilita saída de debug

<h2 id="upgrade-from-beta">
  Atualizar da versão beta
</h2>

Se seus fluxos de trabalho ainda referenciam `anthropics/claude-code-action@beta`, atualize-os para v1:

1. Altere `@beta` para `@v1` na linha `uses`
2. Remova a entrada `mode`, já que a Claude Code GitHub Action agora [detecta o modo automaticamente](#interactive-and-automation-modes)
3. Substitua `direct_prompt` por `prompt`
4. Mova opções de CLI como `max_turns` e `model` para `claude_args`. `custom_instructions` não tem um sinalizador de mesmo nome e se torna `--append-system-prompt`

Para o mapeamento completo de entrada e exemplos antes e depois, consulte o [guia de migração](https://github.com/anthropics/claude-code-action/blob/main/docs/migration-guide.md).

<h2 id="what’s-next">
  Próximos passos
</h2>

* [Use Claude Code GitHub Actions com provedores de nuvem](/docs/pt/github-actions-cloud-providers): rotear inferência através de Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry
* [Referência de configuração](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs): a lista completa de entradas da action
* [Diretório de exemplos](https://github.com/anthropics/claude-code-action/tree/main/examples): fluxos de trabalho prontos para uso em mais cenários
* [Code Review](/docs/pt/code-review): revisão automática de pull request sem manter um arquivo de fluxo de trabalho
