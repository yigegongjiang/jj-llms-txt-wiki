> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Medir custo e uso do plugin

> Meça o custo de token de um plugin Claude Code, descubra se as pessoas ainda o usam e escolha os eventos de telemetria para perguntas sobre plugins em toda a organização.

Cada sessão em que um plugin está ativado inclui os nomes e descrições de suas skills, agentes e comandos no contexto do Claude, e esses tokens contam contra o uso do usuário, independentemente de o plugin ser usado ou não. Esta página mostra como ver esse número para um plugin, como reduzi-lo se você mantém o plugin e onde o uso aparece para que você possa saber se um plugin ainda está sendo usado.

Esta página é para autores e mantenedores de plugins. Se você administra Claude Code para uma organização, [Medir em toda a frota](#measure-across-a-fleet) cobre as mesmas perguntas em cada máquina.

<Note>
  Estes casos são cobertos em outras páginas:

  * **Testar com que confiabilidade o plugin muda o comportamento do Claude**: veja [Testar plugins com evals](/docs/pt/plugin-evals)
  * **Aparar o contexto de sua própria sessão**: veja [Gerenciar plugins instalados](/docs/pt/plugins/install#manage-installed-plugins) e a página [janela de contexto](/docs/pt/context-window)
</Note>

Comece com [Medir o que um plugin custa](#measure-what-a-plugin-costs).

<h2 id="measure-what-a-plugin-costs">
  Medir o que um plugin custa
</h2>

Para ver o que um plugin adiciona ao contexto do Claude, execute [`claude plugin details`](/docs/pt/plugins/cli-reference#plugin-details) com o nome do plugin. Você o executa em seu shell, não no prompt de uma sessão Claude Code em execução. O plugin deve estar carregado: instalado, em um diretório de skills ou passado com `--plugin-dir` no mesmo comando, como em `claude --plugin-dir ./formatter plugin details formatter`.

Este exemplo lê um plugin instalado chamado `formatter` que tem duas skills, um comando, um agente, um hook e um servidor MCP:

```bash theme={null}
claude plugin details formatter
```

```text theme={null}
formatter 1.0.0
  Description: Formats and lints code on save
  Source: formatter@my-marketplace

Component inventory
  Skills (3)  format-all, format-code, lint-fix
  Agents (1)  style-reviewer
  Hooks (1)  PostToolUse  (harness-only — no model context cost)
  MCP servers (1)  formatter-tools  (tool schemas resolved at runtime; not counted)
  LSP servers (0)

Projected token cost
  Always-on:   ~146 tok   added to every session

Per-component (rounded)
  component       always-on  on-invoke
  format-code           ~40        ~30
  lint-fix              ~50        ~30
  style-reviewer        ~40        ~40
  format-all           < 20        ~30

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

Cada parte da saída responde a uma pergunta diferente:

* **Component inventory**: o que Claude Code encontrou no plugin. Comandos são contados com skills, então `format-all` aparece em `Skills`. Hooks e servidores MCP não recebem estimativa de custo e nenhuma linha por componente; para ver o que as ferramentas MCP de um plugin adicionam, execute `/context` em uma sessão com o plugin ativado e leia a categoria `MCP tools`.
* **Always-on**: os tokens que os nomes e descrições das skills, agentes e comandos do plugin adicionam a cada sessão em que o plugin está ativado, independentemente de algo ser executado ou não. Este é o número que cada usuário carrega e o que deve ser reduzido.
* **Per-component**: cada linha divide uma skill, agente ou comando em sua parte sempre ativa e seu custo ao invocar, que é o corpo que carrega apenas quando esse componente é executado. Use a coluna sempre ativa para encontrar qual componente contribui mais.

<h3 id="lower-the-always-on-figure">
  Reduzir a figura sempre ativa
</h3>

Se você mantém o plugin, essas mudanças reduzem o que ele adiciona a cada sessão. Se você apenas o usa, suas opções são desativá-lo ou desinstalá-lo; veja [Gerenciar plugins instalados](/docs/pt/plugins/install#manage-installed-plugins).

A figura sempre ativa conta o nome de cada componente mais sua `description` e `when_to_use` frontmatter. Para reduzi-la:

* Encurte as descrições de skills e agentes.
* Divida um plugin grande para que os usuários instalem apenas os componentes que precisam.

A descrição de uma skill também é o que Claude corresponde a uma solicitação, então uma mais curta pode impedir que a skill seja acionada. Depois de aparar as descrições, verifique o acionamento com um [avaliador `tool_used: Skill`](/docs/pt/plugin-evals#create-your-first-eval-suite) em sua suíte de avaliação.

Para saber o que cada tipo de componente contribui, veja [componentes de plugin](/docs/pt/plugins/components).

<h3 id="cost-shown-to-users-before-install">
  Custo mostrado aos usuários antes da instalação
</h3>

Plugins no marketplace oficial mostram seu custo aos usuários antes da instalação. Em `/plugin`, quando um usuário navega pela lista de plugins de um marketplace e seleciona um plugin, o painel de detalhes mostra uma seção **Context cost** com uma linha `Every turn:` e uma linha `When invoked:`. Quando a figura sempre ativa é 2.000 tokens ou mais, a linha `Every turn:` aparece destacada.

Um plugin em seu próprio marketplace não tem uma seção **Context cost**.

<h2 id="check-whether-a-plugin-is-used">
  Verificar se um plugin é usado
</h2>

Claude Code não relata o uso de um plugin de volta ao seu autor. O uso é registrado na máquina de cada pessoa que instalou o plugin, então o que você pode aprender depende de seu relacionamento com essas pessoas:

* **Você administra Claude Code para sua organização**: os eventos OpenTelemetry e a API Analytics contam instalações e ativações de skills em cada máquina. Veja [Medir em toda a frota](#measure-across-a-fleet).
* **São colegas de equipe que você pode perguntar**: o próprio Claude Code de cada usuário mostra a eles se ainda usam o plugin, em quatro lugares: o painel [`/plugin`](#not-used-recently-in-/plugin), [`/skill-doctor`](#find-skills-that-never-run), [`/doctor`](#unused-plugins-in-/doctor) e [`/usage`](#usage-share-in-/usage). Todos os quatro são comandos que o usuário executa no prompt Claude Code em uma sessão em sua própria máquina.
* **Nenhum dos dois**: você não tem sinal de uso de Claude Code para esse plugin.

<h3 id="not-used-recently-in-/plugin">
  Não usado recentemente em `/plugin`
</h3>

Na aba **Installed** de `/plugin`, um plugin que o usuário instalou de um marketplace se move sob um cabeçalho **Not used recently** uma vez que ficou sem uso por pelo menos 14 dias e 10 sessões. Os detalhes do plugin também mostram uma linha `Last used:`. Para saber o que os usuários fazem com esse cabeçalho e linha, veja [Encontrar plugins que você não usa mais](/docs/pt/plugins/install#find-plugins-you-no-longer-use).

O cabeçalho **Not used recently** nunca aparece para:

* Plugins carregados com `--plugin-dir` ou de um diretório de skills
* Plugins ativados através de configurações gerenciadas ou montados de um [diretório seed](/docs/pt/plugins/org#seed-containers-and-ci)
* Plugins que incluem um tema, estilo de saída, monitor ou workflow, porque esses estão em uso sem uma invocação rastreada

Um [servidor de linguagem](/docs/pt/plugins/components#lsp-servers) de um plugin conta como usado quando entrega diagnósticos ou responde a uma solicitação de navegação de código, então um plugin LSP cujo servidor está ativo em suas sessões não está listado como não usado.

Quando a organização do usuário define [`strictKnownMarketplaces`](/docs/pt/plugins/org#restrict-what-users-can-install), nem o cabeçalho nem a linha `Last used:` aparece.

<h3 id="find-skills-that-never-run">
  Encontrar skills que nunca são executadas
</h3>

Execute `/skill-doctor` para ver o que cada uma de suas skills custa e com que frequência é usada. Ele sinaliza skills que estão na listagem de skills do Claude mas nunca foram invocadas, incluindo skills de plugins.

Em uma sessão interativa, o relatório abre na aba **Stats** do gerenciador `/plugin`. Veja [Encontrar skills não usadas](/docs/pt/skills#find-unused-skills) para saber o que o relatório cobre e onde está disponível.

<h3 id="unused-plugins-in-/doctor">
  Plugins não usados em `/doctor`
</h3>

A verificação `/doctor` lista cada skill instalada pelo usuário, servidor MCP e plugin, e recomenda desativar os que não foram usados. Veja [`/doctor` na referência de comandos](/docs/pt/commands#all-commands).

<h3 id="usage-share-in-/usage">
  Compartilhamento de uso em `/usage`
</h3>

Em um plano Pro, Max, Team ou Enterprise, o detalhamento `/usage` atribui o uso recente a skills, subagentes, plugins e servidores MCP como uma parte do total. Veja [Usando o comando `/usage`](/docs/pt/costs#using-the-/usage-command).

<h2 id="measure-across-a-fleet">
  Medir em toda a frota
</h2>

Se você administra Claude Code para uma organização, você pode medir custo e uso de plugin em cada máquina de uma dessas fontes:

* **Eventos OpenTelemetry**: Claude Code exporta esses para seu próprio backend uma vez que você [configure um exportador](/docs/pt/monitoring-usage). Veja [Eventos OpenTelemetry para instalações e uso de plugins](#pick-the-opentelemetry-event-for-each-question).
* **API Analytics**: servida dos registros da Anthropic, sem necessidade de exportador. Veja [Consultar a API Analytics](#query-the-analytics-api).

<h3 id="pick-the-opentelemetry-event-for-each-question">
  Eventos OpenTelemetry para instalações e uso de plugins
</h3>

Esses eventos e atributos OpenTelemetry respondem cada pergunta de plugin de seu backend:

| Pergunta                                          | Evento ou atributo OpenTelemetry                                                                                                                              |
| :------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Quais plugins são instalados e de onde            | [`claude_code.plugin_installed`](/docs/pt/monitoring-usage#plugin-installed-event), um por instalação                                                              |
| Quais plugins estão ativos em quantas sessões     | [`claude_code.plugin_loaded`](/docs/pt/monitoring-usage#plugin-loaded-event), um por plugin ativado no início da sessão                                            |
| Quais skills são ativadas e qual plugin as possui | [`claude_code.skill_activated`](/docs/pt/monitoring-usage#skill-activated-event), com `plugin.name` e `marketplace.name` para skills de plugin                     |
| O que os hooks de um plugin relatam               | [`claude_code.hook_plugin_metrics`](/docs/pt/monitoring-usage#hook-plugin-metrics-event), emitido apenas para hooks em plugins do marketplace oficial              |
| O que um plugin custa em gastos de API            | `plugin.name` e `marketplace.name` no [contador de custo](/docs/pt/monitoring-usage#cost-counter), definido quando a skill ativa ou subagente pertence a um plugin |

<h3 id="redacted-plugin-names-in-your-backend">
  Nomes de plugin redatados em seu backend
</h3>

Plugins do marketplace oficial relatam seu nome de plugin e nome de marketplace para seu backend literalmente. Todos os outros nomes de plugin são redatados ou omitidos por padrão, incluindo um plugin do próprio marketplace de sua organização. O [nível de confiança](/docs/pt/plugins/security#find-plugins-in-telemetry) do plugin decide qual.

Para obter nomes reais em alguns eventos, defina a variável de ambiente [`OTEL_LOG_TOOL_DETAILS`](/docs/pt/monitoring-usage#common-configuration-variables) como `1` nas máquinas que exportam telemetria, por exemplo no bloco `env` das mesmas [configurações gerenciadas](/docs/pt/monitoring-usage#administrator-configuration) que configuram o exportador:

| Evento                                | Padrão                                                                                           | Com `OTEL_LOG_TOOL_DETAILS=1`                        |
| :------------------------------------ | :----------------------------------------------------------------------------------------------- | :--------------------------------------------------- |
| `plugin_loaded`                       | `plugin.name` e `marketplace.name` são a string literal `third-party`                            | Nomes reais                                          |
| `plugin_installed`, `skill_activated` | `plugin.name` e `marketplace.name` omitidos; em `skill_activated`, `skill.name` é `custom_skill` | Nomes reais                                          |
| Contador de custo                     | `plugin.name` é `third-party`; `marketplace.name` ausente                                        | `plugin.name` real; `marketplace.name` ainda ausente |

Em `plugin_loaded`, `plugin_id_hash` ainda identifica cada plugin por padrão, então você pode contar plugins de terceiros distintos.

<h3 id="query-the-analytics-api">
  Consultar a API Analytics
</h3>

No plano Enterprise, a API Analytics responde "quais plugins minha organização instala e invoca" dos registros da Anthropic, sem necessidade de exportador. [`GET /v1/organizations/analytics/plugins`](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) retorna contagens de instalação e invocação por plugin, por dia em Claude Code e Cowork, que você pode agrupar por usuário, grupo RBAC ou produto.

A atividade de plugin que chega à Anthropic sem um nome de plugin aparece em uma linha agregada `third-party`. [Encontrar plugins em telemetria](/docs/pt/plugins/security#find-plugins-in-telemetry) diz quais plugins Claude Code relata por nome.

Autentique a solicitação com uma chave de API que tenha o escopo `read:analytics`, que um Proprietário Primário cria conforme descrito em [Acessar dados programaticamente](/docs/pt/analytics#access-data-programmatically).

Veja a [referência de endpoint](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) para os parâmetros e campos de resposta.

<h2 id="next-steps">
  Próximos passos
</h2>

* [Testar plugins com evals](/docs/pt/plugin-evals): meça com que confiabilidade o plugin orienta o Claude, não apenas o que custa
* [Reduzir a figura sempre ativa](#lower-the-always-on-figure): o que mudar no plugin para reduzir seu custo por turno
* [Segurança e confiança de plugin](/docs/pt/plugins/security#find-plugins-in-telemetry): quais campos de telemetria carregam nomes de plugin e quando são redatados
* [Monitoramento de uso](/docs/pt/monitoring-usage): a referência completa de eventos OpenTelemetry
