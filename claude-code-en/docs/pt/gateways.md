> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Executar Claude Code através de um gateway

> Rotear Claude Code através de um gateway auto-hospedado para credenciais centralizadas, rastreamento de uso e controles de custo. Abrange a arquitetura, o gateway de aplicativos Claude da Anthropic e o uso de outros produtos de gateway.

Um gateway é um proxy que sua organização executa entre Claude Code e um provedor de modelo. Claude Code envia tráfego de API para o gateway em vez de diretamente para o provedor, e o gateway o encaminha usando uma credencial que sua organização mantém. Os desenvolvedores se autenticam no gateway em vez de manter credenciais do provedor, portanto autenticação, rastreamento de uso, orçamentos e registro de auditoria acontecem em um único lugar que você controla.

Claude Code inclui um gateway auto-hospedado, [Claude apps gateway](/docs/pt/claude-apps-gateway), no binário `claude`, portanto você não precisa adotar um produto de gateway separado para executar um. Se sua organização já executa um [LLM gateway](/docs/pt/llm-gateway), Claude Code também funciona com esse.

Esta página aborda:

* [Como um gateway fica entre Claude Code e seu provedor](#how-a-gateway-works)
* [Escolhendo entre Claude apps gateway e um gateway que você já executa](#choose-a-gateway)
* [Como gateways interagem com assinaturas claude.ai](#subscriptions-and-gateways)
* [O que é configurado separadamente do gateway](#configure-separately-from-the-gateway)

<h2 id="how-a-gateway-works">
  Como um gateway funciona
</h2>

Cada Claude Code do desenvolvedor é apontado para o endereço do gateway e se autentica com uma credencial emitida pelo gateway.

O gateway autentica o desenvolvedor, aplica quaisquer regras de acesso e orçamento que você configure e encaminha a solicitação para seu provedor com a credencial da organização. O provedor pode ser a API da Anthropic ou um [provedor de nuvem](/docs/pt/third-party-integrations) como Amazon Bedrock, Agent Platform do Google Cloud ou Microsoft Foundry; a configuração do gateway decide. Com Claude apps gateway ou outro gateway que expõe um único endpoint no formato Anthropic, mudar de provedor não requer tocar em máquinas de desenvolvedores.

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/llm-gateway-flow.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=1c1a8dcc0cfcc3a58652cc8e28cd3e20" alt="Diagrama mostrando Claude Code roteando através de um gateway. Em uma zona de máquinas de desenvolvedores, a CLI Claude Code e a extensão VS Code enviam solicitações para o endereço do gateway com uma credencial por desenvolvedor. Em uma zona rotulada sua infraestrutura, o gateway lida com autenticação, rastreamento de uso, orçamentos e roteamento, e encaminha solicitações com a credencial de sua organização. Em uma zona de provedores de modelo, uma seta sólida leva ao provedor que você configura, mostrado como a API Anthropic, e setas tracejadas levam a outras opções de provedor, ilustradas com Amazon Bedrock, Google Cloud e Microsoft Foundry como exemplos." width="780" height="322" data-path="images/llm-gateway-flow.svg" />
</Frame>

Dois tipos de credencial estão envolvidos:

* **Credencial do desenvolvedor**: cada desenvolvedor mantém a sua própria, emitida pelo gateway. Ela os autentica no gateway e os identifica no rastreamento de uso
* **Credencial do provedor**: o gateway mantém uma credencial para sua conta de provedor, compartilhada por todo o tráfego encaminhado

<h2 id="choose-a-gateway">
  Escolha um gateway
</h2>

Claude Code funciona com o próprio gateway da Anthropic ou com um gateway que sua organização já executa.

<h3 id="claude-apps-gateway">
  Claude apps gateway
</h3>

Claude apps gateway é o gateway auto-hospedado da Anthropic, incluído no binário `claude`. Ele roteia para Amazon Bedrock, Claude Platform on AWS, Google Cloud, Microsoft Foundry ou a API Anthropic como upstream. Os desenvolvedores fazem login com seu provedor de identidade corporativa através de `/login`, o gateway impõe acesso a modelo e [configurações gerenciadas](/docs/pt/permissions#managed-settings) por grupo IdP, e emite métricas de uso [OpenTelemetry Protocol (OTLP)](/docs/pt/monitoring-usage) para sua própria pilha de observabilidade.

Como é construído e testado junto com cada lançamento de Claude Code, ele encaminha os cabeçalhos e campos de solicitação que Claude Code envia. Um gateway mantido separadamente precisa ter suas [regras de encaminhamento atualizadas](/docs/pt/llm-gateway-protocol#forward-as-open-lists) conforme esses cabeçalhos e campos mudam a cada lançamento; Claude apps gateway é lançado com a CLI, portanto não há lista para manter atualizada. Veja [Disponibilidade e limitações](/docs/pt/claude-apps-gateway#availability-and-limitations) para o pequeno conjunto de recursos que se comportam de forma diferente em uma sessão de gateway.

O login do gateway é uma etapa de SSO do navegador, e não há fluxo de token de serviço, portanto um pipeline de CI sem um desenvolvedor para aprovar o login não pode se autenticar através dele; configure-os contra seu provedor diretamente. Sessões do Agent SDK e execuções `claude -p` em uma máquina onde um desenvolvedor fez login usam a sessão de gateway dessa máquina e são governadas por suas políticas. Veja [Pipelines de CI e máquinas remotas](/docs/pt/claude-apps-gateway#ci-pipelines-and-remote-machines).

Veja [Claude apps gateway](/docs/pt/claude-apps-gateway) para implantá-lo.

<h3 id="other-gateways">
  Outros gateways
</h3>

Se sua organização já executa um LLM gateway ou API gateway, você pode usá-lo em vez disso. A Anthropic não endossa, mantém ou audita outros produtos de gateway, e não suporta roteamento de Claude Code para modelos não-Claude através de qualquer gateway. Veja [Outros LLM gateways](/docs/pt/llm-gateway) para a lista de verificação de implementação do administrador, o que um gateway deve implementar e como apontar Claude Code para ele.

<h2 id="subscriptions-and-gateways">
  Assinaturas e gateways
</h2>

Quando os desenvolvedores se conectam através de um gateway com uma credencial de gateway, o uso é faturado para a conta de provedor de sua organização com taxas de API, e suas assinaturas claude.ai não são usadas ou cobradas. Definir [`ANTHROPIC_AUTH_TOKEN`](/docs/pt/env-vars) para um gateway que você executa, ou fazer login em um Claude apps gateway com `/login`, desativa o login de assinatura para essa sessão. Cada solicitação encaminhada sob essa credencial é cobrada da conta atrás da credencial do provedor do gateway.

A exceção é definir apenas `ANTHROPIC_BASE_URL`, sem credencial de gateway. As solicitações ainda são roteadas através do gateway, mas um login claude.ai salvo permanece como a credencial ativa, portanto os limites de uso e faturamento da assinatura se aplicam. [Outros LLM gateways](/docs/pt/llm-gateway#subscriptions-and-gateways) aborda essa configuração e o que o gateway precisa encaminhar para que funcione.

<h2 id="configure-separately-from-the-gateway">
  Configure separadamente do gateway
</h2>

Um gateway roteia solicitações de API de modelo. Algumas coisas que você pode esperar que ele manipule são configuradas em outro lugar:

* **Qual modelo responde**: escolha o modelo com o comando `/model` ou [variáveis de ambiente de modelo](/docs/pt/model-config#setting-your-model). O gateway decide para onde as solicitações vão, não qual modelo o desenvolvedor seleciona. Claude apps gateway pode limitar a escolha com uma lista de permissões `availableModels` por grupo, mas o desenvolvedor ainda escolhe dentro dela.
* **Outro tráfego de rede**: Claude Code em si envia verificações de versão e downloads diretamente para a Anthropic, separado do caminho do gateway. Se o fluxo de telemetria do cliente opcional também está ativado depende do seu provedor; a [tabela de padrões de telemetria](/docs/pt/data-usage#telemetry-services) aborda cada caso. Em uma sessão de Claude apps gateway conectada, a credencial do gateway desativa a análise vinculada à Anthropic e, quando [encaminhamento de telemetria](/docs/pt/claude-apps-gateway-config#telemetry) é configurado, fixa a exportação OTLP para o gateway. Sua rede ainda precisa de saída para os [domínios necessários](/docs/pt/network-config), ou defina [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/pt/env-vars) para desativar os fluxos opcionais.
* **Proxies HTTP corporativos**: um `HTTPS_PROXY` fica entre Claude Code e cada servidor com o qual ele se comunica, incluindo o gateway. Se sua rede exigir um, [configure o proxy](/docs/pt/network-config) além do gateway. Para um Claude apps gateway que você hospeda, [o login verifica se o host do proxy também está em uma rede privada](/docs/pt/claude-apps-gateway#prerequisites); se não estiver, adicione o host do gateway a `NO_PROXY` para que a CLI se conecte a ele diretamente.

<h2 id="next-steps">
  Próximas etapas
</h2>

A próxima página depende de quem executa o gateway. O gateway da Anthropic é executado a partir do binário `claude` e tem seu próprio guia de configuração; um gateway que sua organização já executa tem um protocolo a implementar e uma lista de verificação de implementação do administrador.

* [Claude apps gateway](/docs/pt/claude-apps-gateway) para implantar o gateway auto-hospedado da Anthropic com login SSO e telemetria OTLP
* [Outros LLM gateways](/docs/pt/llm-gateway) para o que um gateway que sua organização já executa deve implementar e como apontar Claude Code para ele
* [Configure Claude Code para sua organização](/docs/pt/admin-setup) para as decisões de implementação mais amplas das quais um gateway é uma parte
