> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configurar modo automático

> Diga ao classificador do modo automático quais repositórios, buckets e domínios sua organização confia. Defina o contexto do ambiente, substitua as regras de bloqueio e permissão padrão e inspecione sua configuração efetiva com os subcomandos da CLI do modo automático.

[Modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) permite que Claude Code seja executado sem prompts de permissão rotineiros, roteando chamadas de ferramentas através de um classificador que bloqueia qualquer coisa irreversível, destrutiva ou direcionada para fora do seu ambiente. Regras de negação e solicitação explícita são avaliadas antes do classificador e ainda bloqueiam ou solicitam. Use o bloco de configurações `autoMode` para dizer ao classificador quais repositórios, buckets e domínios sua organização confia, para que ele pare de bloquear operações internas rotineiras.

<Note>
  Modo automático está disponível para todos os usuários em todos os provedores, incluindo a API Anthropic, [Claude Platform on AWS](/docs/pt/claude-platform-on-aws), Amazon Bedrock, Agent Platform do Google Cloud, Microsoft Foundry e sessões do [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) conectadas. Se Claude Code relatar que o modo automático não está disponível para sua conta, verifique os [requisitos completos](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode), que também cobrem os modelos suportados e o controle no nível da organização em planos Team e Enterprise. Nas versões v2.1.158 a v2.1.206, o modo automático no Amazon Bedrock, Agent Platform do Google Cloud, Microsoft Foundry e sessões do gateway de aplicativos Claude exigiam a definição de `CLAUDE_CODE_ENABLE_AUTO_MODE=1`; v2.1.207 removeu o requisito.
</Note>

Por padrão, o classificador confia apenas no diretório de trabalho e nos remotos configurados do repositório atual. Ações como enviar para a organização de controle de fonte da sua empresa ou escrever em um bucket de nuvem da equipe são bloqueadas até que você as adicione a `autoMode.environment`.

Para saber como as sessões acabam em modo automático e o que o classificador bloqueia por padrão, consulte [modo automático na página Modos de permissão](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode). Esta página é a referência de configuração.

Esta página cobre como:

* [Adicionar um checkpoint humano](#add-a-human-checkpoint) para pushes e pull requests com `permissions.ask`
* [Escolher onde definir regras](#where-the-classifier-reads-configuration) em CLAUDE.md, configurações do usuário e configurações gerenciadas
* [Definir infraestrutura confiável](#define-trusted-infrastructure) com `autoMode.environment`
* [Gerar entradas de ambiente](#generate-environment-entries) com `/auto-mode-setup`
* [Substituir as regras de bloqueio e permissão](#override-the-block-and-allow-rules) quando os padrões não se adequam ao seu pipeline
* [Editar regras de `/permissions`](#edit-rules-from-permissions) sem abrir um arquivo de configurações
* [Rotear todos os comandos shell através do classificador](#route-all-shell-commands-through-the-classifier) com `autoMode.classifyAllShell`
* [Inspecionar sua configuração efetiva](#inspect-the-defaults-and-your-effective-config) com os subcomandos `claude auto-mode`
* [Revisar negações](#review-denials) para saber o que adicionar a seguir

<h2 id="common-boundaries">
  Limites comuns
</h2>

O modo automático permite pushes para qualquer branch do repositório em que você está trabalhando, incluindo a branch padrão, e criação de pull request por padrão. Uma branch não padrão cujo nome a marca como alvo de deploy ou publicação, como `production`, `release` ou `gh-pages`, não é coberta por esse padrão: o classificador julga um push lá em seus próprios termos, incluindo como um deploy de produção. O conteúdo do push ainda é verificado, portanto um force push, um segredo entrando no commit ou uma mudança que enviaria segredos fora do repositório quando CI ou um pipeline de deploy o executa permanece bloqueado.

<Info>Antes da v2.1.211, o classificador permitia pushes apenas para sua branch de trabalho, branches que Claude criou e pushes rotineiros para a branch padrão.</Info>

Se você quiser um checkpoint humano antes dos comandos push e pull request do Claude, adicione regras de permissão: as [receitas abaixo](#add-a-human-checkpoint) mantêm o modo automático ativado para tudo mais.

<h3 id="add-a-human-checkpoint">
  Adicionar um checkpoint humano
</h3>

O mecanismo mais direto é [`permissions.ask`](/docs/pt/permissions#permission-rule-syntax). Regras ask com escopo de conteúdo como as abaixo são avaliadas antes do classificador e sempre forçam um prompt de permissão, mesmo em modo automático, porque uma regra ask explícita é sua intenção declarada de ser solicitado para essa ação. Adicione as regras em suas [settings](/docs/pt/settings#where-settings-live):

```json theme={null}
{
  "permissions": {
    "ask": [
      "Bash(git push *)",
      "Bash(gh pr create *)"
    ]
  }
}
```

Essas regras correspondem a comandos que começam com `git push` ou `gh pr create`. Um push que Claude escreve de outra forma, como `git -C <dir> push` ou `git -c <key>=<value> push`, [não corresponde à regra](/docs/pt/permissions#bash-rule-limits), portanto não é checkpointed. Para um checkpoint que inspeciona o texto completo do comando, adicione um [hook PreToolUse](/docs/pt/hooks#pretooluse).

Escolha o mecanismo que corresponde ao quão firme o limite precisa ser:

| Limite                        | Mecanismo                                                | Comportamento em modo automático                                                                                                                                                                                                   |
| :---------------------------- | :------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Solicitar antes da ação       | `permissions.ask`                                        | Sempre solicita para um comando que corresponde a uma regra com escopo de conteúdo como a receita acima. O classificador não pode aprovar automaticamente uma ação correspondente.                                                 |
| Nunca executar a ação         | `permissions.deny`                                       | Bloqueia antes do classificador ser consultado. Nem o classificador nem a intenção do usuário podem substituí-lo.                                                                                                                  |
| Limite único para esta sessão | Declare na conversa, como "não faça push até eu revisar" | O classificador bloqueia ações correspondentes, mas o limite pode ser perdido se a [compactação de contexto](/docs/pt/costs#reduce-token-usage) remover a mensagem que o declarou. Use uma regra ask ou deny para uma garantia durável. |

<h2 id="where-the-classifier-reads-configuration">
  Onde o classificador lê a configuração
</h2>

O classificador lê o mesmo conteúdo [CLAUDE.md](/docs/pt/memory) que o próprio Claude carrega, portanto uma instrução como "nunca force push" no CLAUDE.md do seu projeto orienta tanto Claude quanto o classificador ao mesmo tempo. Comece lá para convenções de projeto e regras de comportamento.

Para regras que se aplicam em todos os projetos, como infraestrutura confiável ou regras de negação em toda a organização, use o bloco de configurações `autoMode`. O classificador lê `autoMode` dos seguintes escopos:

| Escopo                         | Arquivo                                                  | Use para                                                           |
| :----------------------------- | :------------------------------------------------------- | :----------------------------------------------------------------- |
| Um desenvolvedor               | `~/.claude/settings.json`                                | Infraestrutura confiável pessoal                                   |
| Em toda a organização          | [Configurações gerenciadas](/docs/pt/server-managed-settings) | Infraestrutura confiável distribuída para todos os desenvolvedores |
| Flag `--settings` ou Agent SDK | JSON inline                                              | Substituições por invocação para automação                         |

O classificador não lê `autoMode` das configurações do projeto em `.claude/settings.json` ou `.claude/settings.local.json`. Ambos os arquivos residem no diretório do repositório, portanto um repositório verificado ou uma etapa de compilação poderia injetar suas próprias regras de permissão. Antes da v2.1.207, o classificador também lia `.claude/settings.local.json`; mova qualquer bloco `autoMode` nesse arquivo para `~/.claude/settings.json`. Excluir `.claude/settings.local.json` também fecha o caso em que um repositório confirma o arquivo ou uma ferramenta local ou etapa de compilação o escreve.

As entradas de cada escopo são combinadas. Um desenvolvedor pode estender `environment`, `allow`, `soft_deny` e `hard_deny` com entradas pessoais, mas não pode remover entradas que as configurações gerenciadas fornecem. Como as regras de permissão atuam como exceções às regras de bloqueio suave dentro do classificador, uma entrada `allow` adicionada pelo desenvolvedor pode substituir uma entrada `soft_deny` da organização: a combinação é aditiva, não um limite de política rígida.

<Note>
  O classificador é um segundo portão que é executado após o [sistema de permissões](/docs/pt/permissions). Para ações que nunca devem ser executadas independentemente da intenção do usuário ou da configuração do classificador, use `permissions.deny` nas configurações gerenciadas, que bloqueia a ação antes do classificador ser consultado e não pode ser substituída.
</Note>

<h2 id="define-trusted-infrastructure">
  Defina infraestrutura confiável
</h2>

Para a maioria das organizações, `autoMode.environment` é o único campo que você precisa definir. Ele informa ao classificador quais repositórios, buckets e domínios são confiáveis: o classificador o usa para decidir o que significa "externo", portanto qualquer destino não listado é um alvo potencial de exfiltração.

A partir do Claude Code v2.1.198, `claude auto-mode defaults` imprime três tipos de entrada de ambiente. Versões anteriores à v2.1.195 imprimem apenas os primeiros cinco slots de confiança.

* **Context slots**: descrevem sua organização, stack e postura de segurança para que o classificador leia as outras regras em seu contexto. Cada um é padronizado para `None configured` ou para a suposição conservadora nomeada ao lado:
  * **Organization**
  * **Primary use of Claude Code**: padronizado para desenvolvimento de software
  * **Cloud provider(s)**
  * **Repository visibility**: um repositório é assumido como privado a menos que seu host remoto e nome indiquem o contrário, ou o classificador leia uma verificação de visibilidade anterior na conversa mostrando que é público.

    Nas solicitações do classificador enviadas pelo Claude Code em si, o classificador lê suas mensagens e os comandos que Claude executa, não sua saída. A evidência deve ser algo que o classificador possa ler, como sua própria mensagem nomeando o repositório como público; a saída de um `gh repo view` por si só não o alcança. A verificação de evidência de transcrição requer Claude Code v2.1.200 ou posterior
  * **Internal sharing / snippet hosting**: serviços públicos de paste e gist são tratados como fora do limite de confiança até você nomear um
  * **Org-specific CLIs**
  * **Secrets management**
  * **CI/CD deploy targets**
  * **Network posture**
  * **Host containment**: padronizado para uma máquina de desenvolvedor comum ou executor de CI com internet aberta. Se Claude Code é executado em um container, VM ou pod com uma lista de permissão de egresso ou vizinhos que não deve tocar, nomeie os hosts permitidos, se o endpoint de metadados da nuvem deve ser alcançável e qual projeto de nuvem, cluster ou registro a tarefa usa e sob qual identidade. Até que esta entrada nomeie essa identidade, o classificador [bloqueia](/docs/pt/permission-modes#what-the-classifier-blocks-by-default) solicitações pelas credenciais do próprio host. Requer Claude Code v2.1.257 ou posterior
  * **Protected deployment namespaces / environments**: volta para a heurística de Sensitive remote targets até você nomeá-los
  * **Data retention / declassification**
* **Trust slots**: nomeiam o que o classificador trata como dentro de seu limite. Os slots são Trusted repo, Source control, Trusted internal domains, Trusted cloud buckets, Key internal services e Internal package registry. As entradas de repo e source-control são padronizadas para o repositório de trabalho e seus remotes configurados. Todos os outros slots de confiança são padronizados para `None configured`, portanto nada mais é confiável até você adicioná-lo. A visibilidade de um repositório abrange apenas material confidencial: um repositório privado é um destino aceitável para material confidencial, mas tornar um repositório privado nunca limpa segredos ou dados pessoais ou confiados nele, e o classificador trata o conteúdo portado, repontado ou lido pela primeira vez de fora do repositório de trabalho como não sendo trabalho do próprio repositório. Este escopo requer Claude Code v2.1.203 ou posterior.
* **Sensitivity slots**: nomeiam o que as regras de proteção tratam como alto risco. Os slots são Sensitive data locations & audiences, Sensitive remote targets e Protected IaC scopes. Cada um é padronizado para uma heurística ampla, como tratar qualquer host ou namespace cujo nome carregue `prod` ou `production` como um alvo remoto sensível, portanto as regras de proteção estão ativas antes de você configurar qualquer coisa. Nomear destinos concretos em um slot de sensibilidade faz com que essas regras se apliquem aos destinos nomeados em vez da heurística.

<Info>Antes da v2.1.211, os context slots também incluíam uma entrada Default / protected branches que tratava `main` e `master` como protegidos até você nomear outros. v2.1.211 removeu: [pushes para qualquer branch do repositório em que você está trabalhando](#common-boundaries) são permitidos por padrão, portanto não há padrão de branch protegido para configurar.</Info>

Para adicionar suas próprias entradas junto aos padrões, inclua a string literal `"$defaults"` no array. As entradas padrão são inseridas nessa posição, portanto suas entradas personalizadas podem ir antes ou depois delas.

O exemplo a seguir mantém as entradas padrão e adiciona repositórios, buckets, domínios e serviços de uma organização.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it",
      "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
      "Trusted internal domains: *.corp.example.com, api.internal.example.com",
      "Key internal services: Jenkins at ci.example.com, Artifactory at artifacts.example.com"
    ]
  }
}
```

Depois de salvar suas configurações, execute `claude auto-mode config` para [confirmar que as regras efetivas](#inspect-the-defaults-and-your-effective-config) incluem suas entradas.

As entradas são prosa, não regex ou padrões de ferramenta. O classificador as lê como regras em linguagem natural. Escreva-as da forma como você descreveria sua infraestrutura para um novo engenheiro. Uma seção de ambiente completa cobre:

* **Organization**: o nome da sua empresa e para o que Claude Code é principalmente usado, como desenvolvimento de software, automação de infraestrutura ou engenharia de dados
* **Source control**: cada GitHub, GitLab ou Bitbucket org para o qual seus desenvolvedores fazem push
* **Cloud providers and trusted buckets**: nomes ou prefixos de bucket que Claude deve ser capaz de ler e escrever
* **Trusted internal domains**: nomes de host para APIs, dashboards e serviços dentro de sua rede, como `*.internal.example.com`
* **Key internal services**: CI, registros de artefatos, índices de pacotes internos, ferramentas de incidentes
* **Internal package registry**: o registro npm, PyPI ou outro privado que as instalações devem rotear, para que instalações que o contornem para um registro público sejam bloqueadas
* **Sensitive data locations & audiences**: os buckets, bancos de dados ou caminhos que contêm dados pessoais, dados comerciais confidenciais, credenciais, dados regulados ou material similarmente sensível, e os públicos com os quais os dados em cada local podem ser compartilhados, para que o classificador proteja esses locais em vez de adivinhar pelo conteúdo. Claude Code v2.1.195 através v2.1.197 nomeiam esta entrada PII / regulated-data locations e cobrem apenas locais que contêm dados pessoais ou regulados, sem a dimensão de público
* **Sensitive remote targets**: os namespaces, hosts ou containers que contam como produção, portanto shells remotos e port-forwards neles precisam de sua aprovação explícita
* **Protected IaC scopes**: os recursos de infraestrutura cujo apply ou destroy sempre deve exigir que você nomeie a mudança
* **Additional context**: restrições de indústria regulada, infraestrutura multi-tenant ou requisitos de conformidade que afetam o que o classificador deve tratar como arriscado

As entradas Internal package registry, Sensitive data locations & audiences, Sensitive remote targets e Protected IaC scopes requerem Claude Code v2.1.195 ou posterior. Versões anteriores ainda as leem como contexto simples, mas não têm as regras integradas que as direcionam.

Um template inicial útil: preencha os campos entre colchetes e remova qualquer linha que não se aplique.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Organization: {COMPANY_NAME}. Primary use: {PRIMARY_USE_CASE, e.g. software development, infrastructure automation}",
      "Source control: {SOURCE_CONTROL, e.g. GitHub org github.example.com/acme-corp}",
      "Cloud provider(s): {CLOUD_PROVIDERS, e.g. AWS, GCP, Azure}",
      "Trusted cloud buckets: {TRUSTED_BUCKETS, e.g. s3://acme-builds, gs://acme-datasets}",
      "Trusted internal domains: {TRUSTED_DOMAINS, e.g. *.internal.example.com, api.example.com}",
      "Key internal services: {SERVICES, e.g. Jenkins at ci.example.com, Artifactory at artifacts.example.com}",
      "Additional context: {EXTRA, e.g. regulated industry, multi-tenant infrastructure, compliance requirements}"
    ]
  }
}
```

Quanto mais contexto específico você fornecer, melhor o classificador poderá distinguir operações internas rotineiras de tentativas de exfiltração.

Você não precisa preencher tudo de uma vez. Um rollout razoável: comece com os padrões e adicione sua org de controle de fonte e serviços internos principais, o que resolve os falsos positivos mais comuns, como fazer push para seus próprios repositórios. Adicione domínios confiáveis e buckets de nuvem em seguida. Preencha o resto conforme os bloqueios surgirem.

<h2 id="generate-environment-entries">
  Gerar entradas de ambiente com `/auto-mode-setup`
</h2>

Execute `/auto-mode-setup` para que Claude Code elabore entradas de `autoMode.environment` e, às vezes, também [entradas de regras](#override-the-block-and-allow-rules), a partir do seu projeto e de suas sessões recentes nele. Se você aceitar o rascunho, Claude Code o escreve em `~/.claude/settings.json`.

<Note>
  `/auto-mode-setup` requer um plano Pro, Max ou Team e Claude Code v2.1.228 ou posterior. No Windows nativo, requer v2.1.233 ou posterior. Você não pode executá-lo em [Claude Code na web](/docs/pt/claude-code-on-the-web). Ele também precisa de [busca de sinalizadores de recurso](/docs/pt/env-vars#features-that-need-feature-flag-fetching), portanto você não pode executá-lo em uma sessão onde você desativou a busca de sinalizadores.
</Note>

<h3 id="what-auto-mode-setup-reads">
  O que `/auto-mode-setup` lê
</h3>

Se `~/.claude/settings.json` já contiver entradas de `autoMode`, Claude Code começa perguntando se deve adicionar à sua lista de ambiente ou substituí-la, e mantém as regras que você escreveu de qualquer forma. Claude Code então pergunta como você usa este projeto e oferece duas verificações opcionais antes de verificar qualquer coisa. Na verificação, Claude Code sempre lê estas fontes:

* `CLAUDE.md`, `README.md`, arquivos de configuração e remotes git do projeto
* Suas configurações de `autoMode` e `permissions.allow`
* Os hosts, buckets e nomes de comando dos comandos que Claude executou em suas sessões recentes neste projeto, nunca suas mensagens

As duas verificações opcionais adicionam uma fonte cada:

* A primeira palavra de cada comando no seu histórico de shell
* Os hosts remotos e nomes dos repositórios sob seu diretório home

<h3 id="review-and-save-the-draft">
  Revisar e salvar o rascunho
</h3>

Claude Code verifica em segundo plano e então mostra o rascunho. Você o aceita ou descarta como um todo, portanto edite `~/.claude/settings.json` depois para ajustar entradas individuais. Quando você aceita, Claude Code escreve o rascunho e o reconcilia com as configurações que você já tem:

* Claude Code escreve a lista de `environment` sem `"$defaults"`, porque o rascunho especifica as entradas integradas que deixou inalteradas
* Claude Code inclui `"$defaults"` em cada uma das listas `allow`, `soft_deny` e `hard_deny` às quais o rascunho adiciona entradas, a menos que você já tenha escrito uma lista `allow` sem ela, portanto as [regras integradas](#override-the-block-and-allow-rules) que você não substituiu permanecem em vigor
* Após salvar, Claude Code oferece remover regras de `permissions.allow` em `~/.claude/settings.json` que o modo automático ignora, como `Bash(*)`, ou que aprovam automaticamente comandos destrutivos

Em seguida, execute `claude auto-mode config` para [ver o resultado efetivo](#inspect-the-defaults-and-your-effective-config).

<h3 id="turn-off-auto-mode-setup">
  Desativar `/auto-mode-setup`
</h3>

Depois que o modo automático bloqueou várias ações e você ainda não tem entradas de `autoMode.environment`, Claude Code mostra um diálogo intitulado "Ensinar ao modo automático sobre seu ambiente?" no final de um turno e oferece executar `/auto-mode-setup` para você. Para parar a oferta mas manter o comando, selecione **Não mostrar novamente** naquele diálogo.

Para desativar tanto o comando quanto a oferta, adicione esta entrada [`skillOverrides`](/docs/pt/skills#override-skill-visibility-from-settings) a `~/.claude/settings.json`:

```json theme={null}
{
  "skillOverrides": {
    "auto-mode-setup": "off"
  }
}
```

`/auto-mode-setup` é um comando integrado em vez de um [skill agrupado](/docs/pt/skills#bundled-skills), portanto esta entrada `skillOverrides` ainda se aplica a ele, mas [`disableBundledSkills`](/docs/pt/settings-reference#disablebundledskills) não o desativa.

<h2 id="override-the-block-and-allow-rules">
  Substituir as regras de bloqueio e permissão
</h2>

Três campos adicionais permitem que você substitua as listas de regras integradas do classificador:

* `autoMode.hard_deny`: limites de segurança incondicionais
* `autoMode.soft_deny`: ações destrutivas que a intenção do usuário pode contornar
* `autoMode.allow`: exceções às regras de bloqueio soft

Cada um é uma matriz de descrições em prosa, lidas como regras em linguagem natural. Para bloqueios baseados em padrões de ferramentas que são executados antes do classificador, use [`permissions.deny`](/docs/pt/permissions).

Dentro do classificador, a precedência funciona em quatro camadas:

* Regras `hard_deny` bloqueiam incondicionalmente. A intenção do usuário e exceções `allow` não se aplicam.
* Regras `soft_deny` bloqueiam em seguida. A intenção do usuário e exceções `allow` podem substituir estas.
* Regras `allow` então substituem regras `soft_deny` correspondentes como exceções.
* A intenção explícita do usuário substitui os bloqueios soft restantes: se a mensagem do usuário descreve direta e especificamente a ação exata que Claude está prestes a executar, o classificador a permite mesmo quando uma regra `soft_deny` corresponde.

Solicitações gerais não contam como intenção explícita. Pedir ao Claude para "limpar o repositório" não autoriza force-push, mas pedir ao Claude para "force-push este branch" autoriza.

Para afrouxar, adicione a `allow` quando o classificador sinalizar repetidamente um padrão rotineiro que as exceções padrão não cobrem. Para apertar, adicione a `soft_deny` para riscos destrutivos específicos do seu ambiente que os padrões perdem, ou a `hard_deny` para limites de segurança que nunca devem ser ultrapassados.

Para manter as regras integradas enquanto adiciona as suas próprias, inclua a string literal `"$defaults"` na matriz. As regras padrão são inseridas nessa posição, portanto suas regras personalizadas podem vir antes ou depois delas, e você continua a herdar atualizações conforme a lista integrada muda entre versões.

O exemplo a seguir mantém os padrões em todas as quatro listas e adiciona regras específicas da organização a cada uma.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it"
    ],
    "allow": [
      "$defaults",
      "Deploying to the staging namespace is allowed: staging is isolated from production and resets nightly",
      "Writing to s3://acme-scratch/ is allowed: ephemeral bucket with a 7-day lifecycle policy"
    ],
    "soft_deny": [
      "$defaults",
      "Never run database migrations outside the migrations CLI, even against dev databases",
      "Never modify files under infra/terraform/prod/: production infrastructure changes go through the review workflow"
    ],
    "hard_deny": [
      "$defaults",
      "Never send repository contents to third-party code-review APIs"
    ]
  }
}
```

<Danger>
  Definir qualquer um de `environment`, `allow`, `soft_deny` ou `hard_deny` sem `"$defaults"` substitui a lista padrão inteira para essa seção. Se você definir uma matriz sem `"$defaults"`, descartará as regras integradas para essa seção:

  * `soft_deny`: todas as regras de bloqueio soft integradas, incluindo force push, `curl | bash`, implantações em produção e bypass de auto-mode
  * `hard_deny`: a regra integrada de exfiltração de dados
</Danger>

Cada seção é avaliada independentemente, portanto definir `environment` sozinho deixa as listas padrão `allow`, `soft_deny` e `hard_deny` intactas.

Omita `"$defaults"` apenas quando você pretender assumir a propriedade total da lista. Para fazer isso com segurança, execute `claude auto-mode defaults` para imprimir as regras integradas, copie-as para seu arquivo de configurações e depois revise cada regra em relação ao seu próprio pipeline e tolerância ao risco.

<h2 id="edit-rules-from-permissions">
  Editar regras de `/permissions`
</h2>

Para visualizar e editar regras do classificador sem abrir um arquivo de configurações, execute [`/permissions`](/docs/pt/permissions#manage-permissions) e selecione a aba **Auto mode**. A aba requer Claude Code v2.1.246 ou posterior, e aparece apenas quando [o modo automático está disponível](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) para sua sessão.

A aba lista as entradas `allow`, `soft_deny`, `hard_deny` e `environment` de cada um dos [escopos que o classificador lê](#where-the-classifier-reads-configuration), e mostra se as regras integradas estão em vigor para cada seção. Claude Code mostra entradas de [configurações gerenciadas](/docs/pt/server-managed-settings) ou a flag `--settings` como somente leitura, e salva todas as alterações que você faz na aba em `~/.claude/settings.json`. A partir da aba você pode:

* Adicionar, editar ou excluir regras nas seções `allow`, `soft_deny` e `hard_deny`. Quando você adiciona a primeira regra a uma seção, Claude Code também insere `"$defaults"` para que as [regras integradas](#override-the-block-and-allow-rules) permaneçam em vigor.
* Desativar ou reativar as regras integradas para `allow`, `soft_deny` ou `hard_deny`. Claude Code registra a escolha adicionando ou removendo `"$defaults"` em sua lista para essa seção, portanto uma seção precisa de pelo menos uma regra sua antes que você possa desativar suas regras integradas.
* Editar as entradas `environment` como um documento em seu editor. Se você ainda não configurou nenhuma entrada `environment`, Claude Code primeiro pergunta se deseja substituir o ambiente integrado, depois abre o editor no texto integrado completo. Quando você salva, Claude Code substitui sua matriz `autoMode.environment` pelo documento. Inclua a linha `"$defaults"` para [manter as entradas integradas](#define-trusted-infrastructure).

<h2 id="route-all-shell-commands-through-the-classifier">
  Rotear todos os comandos shell através do classificador
</h2>

Por padrão, as regras estreitas de Bash e PowerShell, como `Bash(npm test)`, permanecem em vigor no modo automático. Claude Code as resolve antes do classificador ser executado, a menos que o comando tenha [domínios permitidos por comando](/docs/pt/sandboxing#per-command-allowed-domains-in-auto-mode). Claude Code suspende apenas as regras amplas que concedem execução arbitrária de código, como `Bash(*)` ou intérpretes com caracteres curinga, juntamente com cada regra que nomeia [`Monitor`](/docs/pt/tools-reference#monitor-tool), porque os comandos Monitor são executados através do shell. Isso significa que uma regra estreita ainda pode deixar um argumento destrutivo passar sem o classificador vê-lo, por exemplo um caminho de script ou sinalizador que o prefixo da regra não antecipou.

Defina `autoMode.classifyAllShell` como `true` para suspender cada regra de permissão de Bash e PowerShell enquanto o modo automático estiver ativo, para que o classificador avalie cada comando shell independentemente da sua lista de permissões.

```json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Isso troca latência por cobertura: um comando que uma regra de permissão teria aprovado instantaneamente agora aguarda uma decisão do classificador, e cada comando shell conta como uma chamada do classificador.

A configuração se aplica apenas enquanto o modo automático estiver ativo, e suas regras de permissão se comportam normalmente em outros modos de permissão.

<Note>
  `autoMode.classifyAllShell` requer Claude Code v2.1.193 ou posterior. Versões anteriores ignoram a chave e continuam a levar regras de permissão de shell estreitas para o modo automático.
</Note>

<h2 id="inspect-the-defaults-and-your-effective-config">
  Inspecione os padrões e sua configuração efetiva
</h2>

Os subcomandos `claude auto-mode` ajudam você a inspecionar, validar e redefinir sua configuração.

Imprima as regras `environment`, `allow`, `soft_deny` e `hard_deny` integradas como JSON:

```bash theme={null}
claude auto-mode defaults
```

Para ler a redação completa de uma regra sem canalizar através de `jq`, passe `--label` com o início do rótulo da regra, como `claude auto-mode defaults --label 'Git Destructive'`. A correspondência é um prefixo case-insensitive no rótulo de cada regra, e seções sem correspondência são impressas como listas vazias. Requer Claude Code v2.1.208 ou posterior.

Imprima o que o classificador realmente usa como JSON, com suas configurações aplicadas onde definidas e padrões caso contrário:

```bash theme={null}
claude auto-mode config
```

Tanto `defaults` quanto `config` imprimem as quatro listas de regras como um único objeto JSON, com cada regra como uma string em prosa. Este é um exemplo truncado:

```json theme={null}
{
  "allow": [
    ...
    "Test Artifacts: Hardcoded test API keys, placeholder credentials in examples, or hardcoding test cases. Placeholder means authored as a placeholder — a file or value copied from a real secret or sensitive path is never a test artifact (see Sensitive-Source Provenance).",
    ...
  ],
  "soft_deny": [
    "Git Destructive [named+specifics — **must name:** the destructive operation and its target]: Force pushing (`git push --force`), deleting remote branches, tags, or releases, or rewriting remote history. Also `git commit --amend` when the commit being rewritten is not the agent's own unpushed work: either no prior `git commit` is visible (HEAD pre-dates the session), or a `git push` of the current branch is visible after the most recent commit (it has been pushed). Clears when the user asked to amend/reword/fixup, or when it is a message-only reword (`--amend -m …`, nothing newly staged) of a commit the agent visibly created this session.",
    ...
  ],
  "hard_deny": [...],
  "environment": [
    ...
    "**Trusted repo**: The git repository the agent started in (its working directory) and its configured remote(s). When the repo's public/private visibility is given — by the Repository visibility entry or the user's own message — use it to scope what is OK to commit or push there: confidential material is fine in a private repo; in a public one, only that repo's own work is — and content ported, repointed, or first read from outside this session's repo is not its own work, whoever directed the port. Visibility scopes confidential material only: secrets and sensitive data (personal & entrusted) are never cleared into any repo by its visibility (see Definitions).",
    ...
  ]
}
```

Obtenha feedback de IA sobre suas regras personalizadas `allow`, `soft_deny` e `hard_deny`:

```bash theme={null}
claude auto-mode critique
```

Execute `claude auto-mode config` após salvar suas configurações para confirmar que as regras efetivas são o que você espera, com `"$defaults"` expandido no lugar. Se você escreveu regras personalizadas, `claude auto-mode critique` as revisa e sinaliza entradas que são ambíguas, redundantes ou provavelmente causarão falsos positivos.

Para descartar suas personalizações e retornar aos padrões integrados, execute o subcomando reset. Requer Claude Code v2.1.212 ou posterior e remove a seção `autoMode` do seu arquivo de configurações do usuário:

```bash theme={null}
claude auto-mode reset
```

O comando resume o que será removido e pergunta `Reset auto mode configuration to defaults?` antes de escrever; passe `--yes` para pular a confirmação. Reset altera apenas `~/.claude/settings.json`: regras `autoMode` de [managed settings](/docs/pt/server-managed-settings) ou a flag `--settings` ainda se aplicam.

<h2 id="review-denials">
  Revisar negações
</h2>

Para revisar e tentar novamente ações que o classificador do modo automático negou, abra `/permissions` e selecione a aba **Recently denied**, onde Claude Code registra cada negação. Pressione `r` em uma ação negada para marcá-la para retry: quando você sair do diálogo, Claude Code envia uma mensagem informando ao modelo que ele pode tentar novamente essa chamada de ferramenta e retoma a conversa.

Quando o classificador produz [nenhum veredicto sobre a ação](/docs/pt/errors#auto-mode-cannot-determine-the-safety-of-an-action), porque uma verificação de segurança separada do modo automático recusou a própria solicitação do classificador ou sua resposta não foi analisada, Claude Code nega a ação sem registrá-la em **Recently denied**. A entrada de erro vinculada cobre o que Claude é informado e como executar a ação se você precisar dela.

<h3 id="fix-a-denial-with-an-allow-rule-an-environment-entry-or-a-retry">
  Corrigir uma negação com uma regra de permissão, uma entrada de ambiente ou um retry
</h3>

Para ver o que o classificador bloqueou, encontre a chamada de ferramenta na conversa. Se a chamada aparecer encurtada ou dobrada em uma linha de resumo como `Ran 3 shell commands`, pressione `Ctrl+O` para abrir o [visualizador de transcrição](/docs/pt/interactive-mode#transcript-viewer), que a expande.

Dois outros lugares na tela que relatam negações omitem o comando ou URL: o aviso perto da caixa de entrada, como `bash denied by auto mode · [Data Exfiltration] · /permissions`, fornece a ferramenta e o motivo, e a aba **Recently denied** lista um comando shell pela descrição que Claude escreveu para ele. Para capturar a entrada exata dessas negações programaticamente, adicione um [hook `PermissionDenied`](/docs/pt/hooks#permissiondenied), que a recebe como `tool_input`.

O texto sob a chamada informa se há algo a corrigir. Texto que relata um problema com o próprio classificador, como um modelo que `is temporarily unavailable` ou um erro do classificador, significa que Claude Code bloqueou a chamada sem um veredicto final do classificador; veja [Auto mode cannot determine the safety of an action](/docs/pt/errors#auto-mode-cannot-determine-the-safety-of-an-action) para saber o que fazer. Caso contrário, uma linha lendo `Denied by auto mode classifier` com um motivo como `[Production Deploy]` ou `Blocked by classifier` significa que o classificador julgou a chamada insegura, então escolha a correção do que a chamada estava tentando alcançar ou fazer:

* Um destino que Claude precisa durante toda a tarefa, como um registro de pacotes, um domínio interno ou um host de repositório: adicione-o a `autoMode.environment`.
* Um comando que você deseja executar sem revisão a partir de agora: adicione uma regra `allow`.
* Uma ação única que você pretendia: declare essa intenção em sua próxima mensagem e deixe Claude tentar novamente.

Você pode adicionar a entrada de ambiente ou regra `allow` a partir da aba [**Auto mode**](#edit-rules-from-permissions) do diálogo `/permissions`.

Na maioria das sessões o nome do motivo nomeia a regra que o classificador correspondeu, entre colchetes, como `[Data Exfiltration]` ou `[Production Deploy]`, e algumas sessões executam um modelo classificador que adiciona uma breve explicação. Claude Code seleciona o modelo classificador, então qual forma você vê não é algo que você configura.

<h3 id="fix-repeated-denials">
  Corrigir negações repetidas
</h3>

Negações repetidas para o mesmo destino geralmente significam que o classificador está perdendo contexto. Adicione esse destino a `autoMode.environment`, ou [execute `/auto-mode-setup`](#generate-environment-entries) para que Claude Code rascunhe as entradas, depois execute `claude auto-mode config` para confirmar que a mudança entrou em vigor.

Para reagir a negações programaticamente, use o [hook `PermissionDenied`](/docs/pt/hooks#permissiondenied).

<h2 id="see-also">
  Veja também
</h2>

* [Modos de permissão](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode): o que é o modo automático, o que ele bloqueia por padrão e quais sessões começam nele
* [Configurações gerenciadas](/docs/pt/server-managed-settings): implante a configuração `autoMode` em toda a sua organização
* [Permissões](/docs/pt/permissions): regras de permitir, perguntar e negar que se aplicam antes do classificador ser executado
* [Todas as configurações](/docs/pt/settings-reference#automode): todas as chaves de configurações, incluindo `autoMode`
