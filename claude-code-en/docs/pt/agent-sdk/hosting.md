> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Hospedagem do Agent SDK

> Implante o Agent SDK em produção: arquitetura de subprocess, persistência de sessão, escalabilidade, observabilidade e isolamento multi-tenant para Docker, Kubernetes e provedores de sandbox.

O Agent SDK cria e supervisiona um subprocess `claude` CLI que possui um shell, um diretório de trabalho e arquivos de sessão no disco. Hospedá-lo não é como hospedar um wrapper de API sem estado. Cada agente em execução é um processo de longa duração vinculado ao estado local, o que molda como você aloca recursos, persiste sessões e escala entre tenants.

Esta página aborda a auto-hospedagem em sua própria infraestrutura: compreenda [o modelo de subprocess](#the-subprocess-model), [escolha um padrão de sessão](#choose-a-session-pattern), [provisione o container](#provision-the-container) e [trate as preocupações de produção](#handle-production-concerns) como persistência, observabilidade, autenticação e isolamento multi-tenant. Para Dockerfiles implantáveis e manifestos Kubernetes, consulte o [hosting cookbook](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting).

Se você não precisa de controle de infraestrutura, isolamento personalizado ou seu próprio plano de dados, considere [Managed Agents](https://platform.claude.com/docs/pt/managed-agents/overview) em vez disso: uma API REST hospedada onde a Anthropic executa o agente e o sandbox, para que sua aplicação envie eventos e transmita resultados de volta sem nenhuma infraestrutura de hospedagem para operar.

<Info>
  Para endurecimento de segurança além da sandboxing básica, incluindo controles de rede, gerenciamento de credenciais e opções de isolamento, consulte [Implantação Segura](/docs/pt/agent-sdk/secure-deployment).
</Info>

<h2 id="the-subprocess-model">
  O modelo de subprocess
</h2>

Cada decisão de hospedagem nesta página segue de como o SDK executa o agente. Quando seu código chama `query()`, o SDK spawna um processo CLI `claude` separado e se comunica com ele via stdio. Esse subprocess possui o shell, o diretório de trabalho e os transcripts de sessão JSONL no disco local.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/hosting-subprocess.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=9dac857ca9d3b1410c3734900c386004" alt="Fluxo de solicitação: cliente para seu aplicativo, que spawna um subprocess CLI claude via stdio dentro do container; o subprocess escreve no disco local e chama api.anthropic.com via HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess.svg" />

Uma sessão de agente mapeia para um subprocess. Executar N sessões concorrentes significa N subprocessos, cada um com sua própria árvore de processos e arquivo de transcript. Por padrão, todos herdam o diretório de trabalho do seu aplicativo, então passe `cwd` em cada chamada `query()` quando as sessões precisarem de sistemas de arquivos separados:

<CodeGroup>
  ```typescript TypeScript theme={null}
  query({ prompt, options: { cwd: "/work/session-a" } })
  ```

  ```python Python theme={null}
  query(prompt=prompt, options=ClaudeAgentOptions(cwd="/work/session-a"))
  ```
</CodeGroup>

<h3 id="state-that-lives-on-local-disk">
  Estado que vive no disco local
</h3>

Três tipos de estado de agente vivem no sistema de arquivos do container por padrão. Nenhum deles sobrevive a um reinício de container, um scale-down ou uma mudança para um nó diferente.

| Estado                             | Localização padrão                                                                                        |
| ---------------------------------- | --------------------------------------------------------------------------------------------------------- |
| Transcripts de sessão              | `~/.claude/projects/`, ou o diretório `projects/` sob `CLAUDE_CONFIG_DIR` se definido                     |
| Arquivos de memória `CLAUDE.md`    | `~/.claude/CLAUDE.md` para o nível de usuário e o diretório de trabalho da sessão para o nível de projeto |
| Artefatos do diretório de trabalho | O diretório de trabalho da sessão                                                                         |

Para persistir transcripts entre hosts, configure um adaptador [`SessionStore`](/docs/pt/agent-sdk/session-storage). Arquivos de memória e outros artefatos do diretório de trabalho precisam de sua própria estratégia de armazenamento, como um volume montado ou uma sincronização de object-store.

Para saber como sessões, retomada e forking funcionam no nível da API, consulte [Sessions](/docs/pt/agent-sdk/sessions).

<h2 id="choose-a-session-pattern">
  Escolha um padrão de sessão
</h2>

Estes quatro padrões cobrem o ciclo de vida da sessão: quanto tempo um container vive em relação às sessões que serve. Para onde o container é executado, o [guia de hospedagem](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb) tem [código implantável](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) para Docker local, Modal e Kubernetes. Escolha um padrão de sessão aqui e um alvo de implantação do guia.

<h3 id="ephemeral-sessions">
  Sessões efêmeras
</h3>

Crie um container para cada tarefa do usuário e destrua-o quando a tarefa for concluída. Melhor para tarefas únicas. O usuário ainda pode interagir com a IA enquanto a tarefa está sendo concluída, mas uma vez concluída, o container é destruído.

Os exemplos de carga de trabalho incluem investigação e correção de bugs, extração de faturas e recibos, tradução de documentos e transformação de mídia.

O container executa um ponto de entrada único que chama o SDK e sai. O exemplo abaixo mostra uma versão TypeScript mínima. Salve-o como `entrypoint.mts` ou defina `"type": "module"` em `package.json` para que `await` de nível superior esteja disponível.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const prompt = process.env.TASK_PROMPT!;
for await (const message of query({ prompt, options: { maxTurns: 20 } })) {
  console.log(message);
}
```

<h3 id="long-running-sessions">
  Sessões de longa duração
</h3>

Execute instâncias de container persistentes, frequentemente hospedando múltiplos processos SDK por container, para servir trabalho contínuo. Melhor para agentes que tomam ações autônomas, servem conteúdo ou lidam com fluxos de mensagens de alto volume.

Os exemplos de carga de trabalho incluem um agente de email que triagem e responde a emails recebidos, um construtor de site que hospeda um site editável por usuário através de portas de container e um chatbot que lida com tráfego contínuo de uma plataforma como Slack.

O container expõe um endpoint HTTP ou WebSocket e mapeia cada sessão ativa para uma query de longa duração e o subprocesso por trás dela. Em TypeScript, use [`streamInput()`](/docs/pt/agent-sdk/typescript#query-object) para adicionar turnos a uma sessão ativa e [`startup()`](/docs/pt/agent-sdk/typescript#startup) para pré-aquecer subprocessos antes do tráfego recebido. Em Python, use [`ClaudeSDKClient`](/docs/pt/agent-sdk/python#claudesdkclient) para manter uma sessão aberta entre turnos. Dimensione o container para que ele possa manter o número máximo de sessões simultâneas na memória.

<h3 id="hybrid-sessions">
  Sessões híbridas
</h3>

Containers efêmeros que hidratam de um [`SessionStore`](/docs/pt/agent-sdk/session-storage) na inicialização e persistem atualizações de volta. Melhor para sessões que abrangem muitas interações mas ficam ociosas entre elas. O container desliga durante períodos ociosos e volta a ligar quando o usuário retorna.

Os exemplos de carga de trabalho incluem um gerenciador de projetos pessoais com check-ins intermitentes, pesquisa profunda que pausa e retoma ao longo de horas e um agente de suporte ao cliente que carrega histórico de tickets entre interações.

Ajuste o tempo limite de ociosidade do seu provedor para a frequência com que você espera que os usuários retornem. Desligar um container sem um `SessionStore` configurado perde a transcrição com ele, então o store é necessário para este padrão, não opcional.

O padrão depende de retomar uma sessão por ID com um store compartilhado anexado:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SessionStore } from "@anthropic-ai/claude-agent-sdk";

  declare const userInput: string;
  declare const sessionId: string;          // looked up from your database by user
  declare const sessionStore: SessionStore; // S3, Redis, Postgres, or your own adapter

  for await (const message of query({
    prompt: userInput,
    options: { resume: sessionId, sessionStore },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt=user_input,
      options=ClaudeAgentOptions(
          resume=session_id,            # looked up from your database by user
          session_store=session_store,  # S3, Redis, Postgres, or your own adapter
      ),
  ):
      ...
  ```
</CodeGroup>

Veja [Armazenamento de sessão](/docs/pt/agent-sdk/session-storage) para a interface completa `SessionStore` e adaptadores de referência.

<h3 id="multi-agent-container">
  Container multi-agente
</h3>

Execute múltiplos subprocessos SDK dentro de um container. Melhor para agentes que devem colaborar estreitamente, por exemplo simulações multi-agente onde os agentes interagem uns com os outros em um ambiente compartilhado.

Dê a cada agente seu próprio diretório de trabalho para que não sobrescrevam os arquivos uns dos outros e isole o carregamento de configurações para que arquivos `CLAUDE.md` por agente não vazem entre agentes. Veja [Isolamento multi-tenant](#multi-tenant-isolation) para as opções específicas.

<h2 id="provision-the-container">
  Provisionar o container
</h2>

<h3 id="container-based-sandboxing">
  Sandboxing baseado em container
</h3>

Execute o SDK dentro de um container seguro para isolamento de processo, limites de recursos, controle de rede e um sistema de arquivos efêmero. Vários provedores se especializam em ambientes de container seguro que se adequam ao modelo do Agent SDK.

Perguntas a responder ao escolher um provedor:

* **Quem executa o sandbox**: um provedor de sandbox-as-a-service opera a infraestrutura para você, enquanto as opções auto-hospedadas fornecem software para você executar em sua própria infraestrutura.
* **Latência de cold-start**: quanto tempo desde "criar um sandbox" até "pronto para aceitar a primeira solicitação". Padrões efêmeros precisam de inicializações em menos de um segundo. Padrões de longa duração toleram mais.
* **Armazenamento persistente**: se o provedor oferece volumes duráveis ou apenas disco efêmero. O padrão híbrido precisa de armazenamento durável em algum lugar, seja no sandbox ou ao lado dele.
* **Modelo de preços**: faturamento por segundo, por solicitação ou por hora fixa. Preços por segundo adequam-se a cargas de trabalho efêmeras intermitentes. Preços por hora adequam-se a sessões de longa duração.
* **Rede**: suporte para regras de egresso personalizadas, proxies de saída e peering de VPC privada para ambientes regulados.

Provedores a avaliar:

* [Modal Sandbox](https://modal.com/docs/guide/sandbox), com uma [implementação de demonstração](https://modal.com/docs/examples/claude-slack-gif-creator)
* [Cloudflare Sandboxes](https://github.com/cloudflare/sandbox-sdk)
* [Daytona](https://www.daytona.io/)
* [E2B](https://e2b.dev/)
* [Fly Machines](https://fly.io/docs/machines/)
* [Vercel Sandbox](https://vercel.com/docs/functions/sandbox)

Para opções auto-hospedadas como Docker, gVisor e Firecracker, e configuração de isolamento detalhada, consulte [Isolation Technologies](/docs/pt/agent-sdk/secure-deployment#isolation-technologies).

<h3 id="runtime-dependencies">
  Dependências de runtime
</h3>

O container precisa apenas do runtime da linguagem do seu SDK:

* Python 3.10+ para o Python SDK, ou Node.js 18+ para o TypeScript SDK
* Ambos os pacotes SDK incluem um binário Claude Code nativo para a plataforma do host, portanto nenhuma instalação separada de Claude Code ou Node.js é necessária para o CLI gerado

O binário incluído é fixado à versão do pacote SDK, portanto atualizar o SDK é como você atualiza o CLI. O SDK segue semver: aceite releases de patch continuamente e revise o changelog do [TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md) ou [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md) antes de aceitar uma versão menor.

<h3 id="resources">
  Recursos
</h3>

1 GiB de RAM, 5 GiB de disco e 1 CPU por agente é um ponto de partida razoável para uma instância recém-iniciada. O uso de memória cresce com a duração da sessão e atividade de ferramentas, portanto dimensione para as durações de sessão e concorrência que você realmente precisa em vez da linha de base ociosa. Consulte [Scaling and concurrency](#scaling-and-concurrency) para saber como calcular agentes por host.

<h3 id="network">
  Rede
</h3>

O SDK precisa de HTTPS de saída para `api.anthropic.com`, ou para o endpoint regional do seu provedor ao executar no Amazon Bedrock ou na Plataforma de Agentes do Google Cloud. Se seus agentes usarem [MCP servers](/docs/pt/agent-sdk/mcp) ou ferramentas externas, eles também precisam de acesso de saída para esses endpoints. Para produção, roteie o tráfego de saída através de um proxy de egresso que aplique listas de permissão de domínio, injete credenciais e registre solicitações. Consulte [Secure Deployment](/docs/pt/agent-sdk/secure-deployment) para o padrão completo.

Para tráfego de entrada, exponha uma porta HTTP ou WebSocket no container. Sua aplicação manipula solicitações de cliente nessa porta e chama o SDK internamente; o subprocesso em si não escuta na rede.

<h2 id="handle-production-concerns">
  Lidar com preocupações de produção
</h2>

Trabalhe através dessas decisões antes de enviar um agente auto-hospedado.

<h3 id="session-and-state-persistence">
  Persistência de sessão e estado
</h3>

O disco local padrão é perdido ao reiniciar, reduzir a escala ou mover para um nó diferente. Para qualquer sessão que um usuário espera retomar, espelhe a transcrição para armazenamento durável com um adaptador [`SessionStore`](/docs/pt/agent-sdk/session-storage). Veja [Implementações de referência](/docs/pt/agent-sdk/session-storage#reference-implementations) para adaptadores S3, Redis e Postgres e um conjunto de conformidade para o seu próprio.

Três coisas a saber sobre como `SessionStore` se comporta:

* **Apenas transcrições**: `SessionStore` espelha transcrições, não arquivos de memória `CLAUDE.md` ou outros artefatos do diretório de trabalho. Monte um volume compartilhado ou sincronize-os separadamente.
* **Espelho, não substituição**: o subprocess escreve no disco local primeiro, e o armazenamento recebe uma cópia de cada lote. As escritas locais permanecem autoritárias.
* **Mensagens `mirror_error`**: um lote que o armazenamento rejeita é enviado até três vezes no total, com um backoff curto antes de cada tentativa; uma chamada que expira não é retentada. Se o lote ainda falhar, o SDK o descarta, emite uma mensagem `{ type: "system", subtype: "mirror_error" }` e continua a consulta. Alerte sobre essas se a durabilidade do armazenamento for importante.

<h3 id="observability">
  Observabilidade
</h3>

Os agentes do Agent SDK são processos de longa duração que geram chamadas de ferramentas em muitas rodadas de API. Sem telemetria, você não pode ver quais ferramentas foram executadas, quanto tempo levaram ou onde uma sessão travou.

O SDK herda a configuração do OpenTelemetry do ambiente. Defina as variáveis de ambiente OTEL no nível do container ou orquestrador para que cada chamada `query()` exporte spans, métricas e eventos de log para seu coletor. O exemplo abaixo habilita a exportação OTLP para todos os três sinais. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` é necessário apenas para rastreamentos; omita-o se você exportar apenas métricas e logs.

```bash title=".env' theme={null}
CLAUDE_CODE_ENABLE_TELEMETRY=1
CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1
OTEL_TRACES_EXPORTER=otlp
OTEL_METRICS_EXPORTER=otlp
OTEL_LOGS_EXPORTER=otlp
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector.example.com:4318
```

O texto do prompt e as entradas de ferramentas não são incluídos nas exportações por padrão. Veja [Controlar dados sensíveis em exportações](/docs/pt/agent-sdk/observability#control-sensitive-data-in-exports) para os sinalizadores de aceitação e [Observabilidade](/docs/pt/agent-sdk/observability) para o catálogo completo de sinais.

<h3 id="auth-and-secrets">
  Autenticação e segredos
</h3>

Três preocupações de autenticação importam no momento da hospedagem:

* **API Anthropic**: o subprocess lê `ANTHROPIC_API_KEY` de seu ambiente. Forneça-o do seu gerenciador de segredos ou defina `ANTHROPIC_BASE_URL` para rotear chamadas de modelo através de um proxy que injeta a chave fora do container. Veja [Gerenciamento de credenciais](/docs/pt/agent-sdk/secure-deployment#credential-management) para o padrão de proxy e [Visão geral do SDK](/docs/pt/agent-sdk/overview#get-started) para métodos de autenticação suportados.
* **Entrada**: coloque a autenticação em um gateway na frente do container do agente. O agente deve receber solicitações pré-autenticadas e não deve ser o componente que valida tokens de usuário.
* **Ferramentas de saída**: mantenha as credenciais de ferramentas fora do ambiente do agente. Rotear chamadas de saída através de um proxy que injeta chaves de API depois que a solicitação sai do container. O agente faz a chamada; o proxy adiciona a credencial.

<h3 id="scaling-and-concurrency">
  Dimensionamento e concorrência
</h3>

Cada sessão é executada em seu próprio subprocess, portanto a concorrência em um host é limitada por quantos subprocessos sua RAM pode manter.

Dimensione cada host com esta fórmula:

```text theme={null}
agentes por host = (RAM do host - overhead) / (limite de RAM por sessão)
```

Meça o limite de RAM por sessão executando uma sessão representativa até seu comprimento alvo sob sua carga de ferramenta esperada e registrando o RSS de pico. O ponto de partida de 1 GiB em [Recursos](#resources) é um piso, não o limite.

O roteamento de escala horizontal depende do seu padrão. Para sessões de longa duração, onde containers mantêm muitas sessões, execute um pool de containers atrás de um balanceador de carga e fixe cada sessão a um container usando hash consistente em `sessionId`. Uma sessão fixada continua atingindo o mesmo container e, portanto, o mesmo subprocess em execução, até que seja despejada ou o container seja reiniciado.

Grandes fanouts de [subagentes](/docs/pt/agent-sdk/subagents) concorrentes de uma única sessão podem atingir limites de taxa de API. Divida o trabalho em lotes menores em vez de emitir um dispatch amplo.

<h3 id="cost">
  Custo
</h3>

O custo de token Anthropic normalmente domina o custo da infraestrutura do container por uma ordem de magnitude ou mais. Um container minimamente provisionado custa aproximadamente \$0,05 por hora, enquanto uma única sessão de agente longo pode gastar dólares em tokens. Veja [Rastreamento de custo](/docs/pt/agent-sdk/cost-tracking) para contabilidade de tokens por sessão.

<h3 id="multi-tenant-isolation">
  Isolamento multi-tenant
</h3>

O comportamento padrão do SDK lê configurações e arquivos de memória `CLAUDE.md` do sistema de arquivos. Em um container compartilhado que serve múltiplos tenants, esses arquivos podem vazar o contexto de um tenant para a sessão de outro tenant.

Para isolar tenants dentro de um container compartilhado:

* Passe `settingSources: []` em TypeScript ou `setting_sources=[]` em Python para que nenhuma configuração do sistema de arquivos seja carregada.
* Defina `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` em `env`. [Auto memory](/docs/pt/memory#auto-memory) em `~/.claude/projects/<project>/memory/` é carregada no prompt do sistema independentemente de `settingSources`. Veja [O que settingSources não controla](/docs/pt/agent-sdk/claude-code-features#what-settingsources-does-not-control) para as outras entradas que são carregadas incondicionalmente.
* Aponte `CLAUDE_CONFIG_DIR` para um diretório por tenant para que os tenants não compartilhem a configuração global `~/.claude.json`.
* Use um diretório de trabalho por tenant. Passe `cwd` explicitamente em cada chamada `query()`.
* Aplique regras de saída por tenant em seu proxy, como IPs de saída distintos, credenciais ou listas de permissão de domínio, para que um tenant comprometido não possa exfiltrar dados através da política de saída de outro tenant.

O exemplo abaixo aplica as quatro opções de nível SDK juntas. Construa `tenantDir` e `configDir` para que cada tenant obtenha um caminho que nenhum outro tenant possa ler. Em TypeScript, `env` substitui o ambiente do subprocess, então espalhe `...process.env` para manter variáveis herdadas como `PATH` e `ANTHROPIC_API_KEY`. Em Python, `env` é mesclado no topo do ambiente herdado.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  declare const prompt: string;
  declare const tenantDir: string;
  declare const configDir: string;

  for await (const message of query({
    prompt,
    options: {
      cwd: tenantDir,
      settingSources: [],
      env: {
        ...process.env,
        CLAUDE_CONFIG_DIR: configDir,
        CLAUDE_CODE_DISABLE_AUTO_MEMORY: "1",
      },
    },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt=prompt,
      options=ClaudeAgentOptions(
          cwd=tenant_dir,
          setting_sources=[],
          env={
              "CLAUDE_CONFIG_DIR": config_dir,
              "CLAUDE_CODE_DISABLE_AUTO_MEMORY": "1",
          },
      ),
  ):
      ...
  ```
</CodeGroup>

Para controles de rede por tenant, veja [Implantação Segura](/docs/pt/agent-sdk/secure-deployment).

<h2 id="known-limitations">
  Limitações conhecidas
</h2>

Planeje em torno destas em seu design de implantação.

| Limitação                                                             | O que fazer                                                                                                                                                                                                                                                                                                                  |
| --------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sem tempo limite de sessão de nível superior                          | Uma sessão não expira por conta própria. Defina `maxTurns` em `Options` para limitar quantas rodadas de uso de ferramentas o agente realiza antes de parar.                                                                                                                                                                  |
| Crescimento de memória em sessões longas                              | Limite o comprimento da sessão ou recicle subprocessos periodicamente. Veja [Scaling and concurrency](#scaling-and-concurrency).                                                                                                                                                                                             |
| Grandes fanouts de subagentes paralelos podem atingir limites de taxa | Divida o trabalho em lotes menores em vez de emitir um dispatch amplo.                                                                                                                                                                                                                                                       |
| Sem prazo de wall-clock por subagente                                 | Limite cada [subagent](/docs/pt/agent-sdk/subagents) com `maxTurns` em sua `AgentDefinition`. Apenas para subagentes em background, `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` define um watchdog de travamento que dispara quando um subagente `run_in_background` para de produzir saída; não é um prazo de tempo total de execução. |

<h2 id="next-steps">
  Próximas etapas
</h2>

* [Guia de hospedagem](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb): passo a passo do notebook com [código implantável](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting) para Docker, Modal e Kubernetes.
* [Armazenamento de sessão](/docs/pt/agent-sdk/session-storage): persistir transcrições entre hosts com um adaptador `SessionStore`.
* [Observabilidade](/docs/pt/agent-sdk/observability): exportar rastreamentos OTEL, métricas e logs para seu coletor.
* [Implantação segura](/docs/pt/agent-sdk/secure-deployment): controles de rede, gerenciamento de credenciais e endurecimento de isolamento.
* [Rastreamento de custos](/docs/pt/agent-sdk/cost-tracking): contabilidade de tokens e custos por sessão.
