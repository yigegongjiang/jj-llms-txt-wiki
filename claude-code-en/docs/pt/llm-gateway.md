> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Outros gateways LLM

> Rotear Claude Code através de um gateway LLM que sua organização já executa. Abrange conectar Claude Code a um gateway, implantar um para sua organização e o que Claude Code envia a um gateway.

Esta seção aborda o uso de um produto gateway que sua organização já executa, em vez de [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway). Para saber o que é um gateway, como ele fica entre Claude Code e seu provedor, e como escolher entre gateway de aplicativos Claude e outro produto, consulte a [visão geral de gateway](/docs/pt/gateways).

<Note>
  * Se você é um desenvolvedor conectando a um gateway existente: [conectar Claude Code ao seu gateway](/docs/pt/llm-gateway-connect)
  * Se você é um administrador implantando um gateway para sua organização: [implantar e distribuir um gateway](/docs/pt/llm-gateway-rollout)
  * Se você está configurando um produto gateway: a [referência de protocolo de gateway](/docs/pt/llm-gateway-protocol)
</Note>

Qualquer gateway que exponha um [formato de API suportado](/docs/pt/llm-gateway-protocol#api-formats) funciona. Anthropic não endossa, mantém ou audita produtos gateway de terceiros, e não oferece suporte ao roteamento de Claude Code para modelos não-Claude através de nenhum gateway. Implante o gateway seguindo sua própria documentação, depois complete o lado Claude Code com as [etapas de rollout abaixo](#roll-out-a-gateway).

<h2 id="what-a-gateway-provides">
  O que um gateway fornece
</h2>

Um gateway oferece à sua organização um único lugar para gerenciar:

* **Credenciais**: a chave do provedor fica no lado do servidor; desenvolvedores mantêm credenciais de gateway em vez disso
* **Rastreamento de uso**: atribua uso por desenvolvedor ou equipe, independentemente de qual provedor atende a solicitação
* **Controles de custo**: aplique orçamentos e limites de taxa em um único lugar
* **Registro de auditoria**: registre cada solicitação de modelo para conformidade
* **Alternância de provedor**: altere o provedor na configuração do gateway, sem tocar nas máquinas dos desenvolvedores

Todos esses, exceto alternância de provedor, se aplicam se o upstream é a API da Anthropic ou um [provedor de nuvem](/docs/pt/third-party-integrations). A alternância de provedor sem reconfigurar máquinas de desenvolvedores também depende do gateway expor um único [endpoint em formato Anthropic](/docs/pt/llm-gateway-protocol#api-formats) independentemente do upstream; um gateway que expõe o próprio formato de um provedor vincula a configuração do cliente a esse provedor.

O tradeoff é que o gateway se torna infraestrutura que sua organização opera. Claude Code adiciona capacidades com cada lançamento, e um gateway que não as encaminha quebra os recursos correspondentes, então o produto gateway precisa ser mantido atualizado conforme Claude Code evolui. A [referência de protocolo de gateway](/docs/pt/llm-gateway-protocol) aborda o que encaminhar.

<h2 id="roll-out-a-gateway">
  Implantar um gateway
</h2>

Quando você estiver pronto para implantar um gateway LLM para sua organização, a sequência é a mesma qualquer que seja o produto gateway que você escolha:

1. Implante o gateway e dê a ele sua credencial de provedor, para que ele possa autenticar as solicitações que encaminha.
2. Emita a cada desenvolvedor uma credencial de gateway, para que o uso seja atribuído ao desenvolvedor e o offboarding revogue uma credencial.
3. Distribua a configuração através de um [arquivo de configurações gerenciadas](/docs/pt/settings#settings-files) e sua ferramenta de segredos, para que cada máquina receba a URL base e uma credencial. Quando ambos forem distribuídos, os desenvolvedores não configuram nada. Se você não tiver distribuição de configurações em vigor, os desenvolvedores seguem a [página de conexão](/docs/pt/llm-gateway-connect) para definir as variáveis eles mesmos.
4. Faça cada desenvolvedor [verificar a configuração no Claude Code](/docs/pt/llm-gateway-connect#check-for-an-existing-configuration), para que problemas de distribuição apareçam antes de dependerem do gateway.

[Implantar um gateway LLM para sua organização](/docs/pt/llm-gateway-rollout) percorre cada etapa e mostra os arquivos de configuração para distribuir em cada uma. O gateway é uma parte da configuração da organização; para aplicação de política, visibilidade de uso e decisões de tratamento de dados, consulte [Configurar Claude Code para sua organização](/docs/pt/admin-setup).

<h2 id="subscriptions-and-gateways">
  Assinaturas e gateways
</h2>

Enquanto uma [variável de credencial de gateway](/docs/pt/llm-gateway-connect#set-the-credential-variable) ou `apiKeyHelper` está ativa, a assinatura claude.ai de um desenvolvedor não é usada: a credencial substitui o login da assinatura para essa sessão, e os limites de uso da assinatura não se aplicam. Esse tráfego é cobrado por token para quem quer que possua a credencial que o gateway encaminha, como sua conta Anthropic Console da organização, ou sua conta Amazon Bedrock, Google Cloud's Agent Platform, ou Microsoft Foundry quando o gateway roteia para lá.

[`ANTHROPIC_BASE_URL`](/docs/pt/llm-gateway-connect#set-the-base-url-and-credential) é a variável que aponta Claude Code para o gateway. Definir apenas essa variável, sem uma credencial de gateway, não substitui a assinatura. As solicitações ainda roteiam através do gateway, mas um login claude.ai salvo permanece como a credencial ativa, então seus limites de uso e cobrança se aplicam. Gateways que passam esse tráfego para Anthropic devem encaminhar a capacidade OAuth em `anthropic-beta`; consulte a [referência de cabeçalhos de solicitação](/docs/pt/llm-gateway-protocol#request-headers).

<h2 id="related-pages">
  Páginas relacionadas
</h2>

* [Visão geral de gateway](/docs/pt/gateways): como um gateway funciona e como escolher entre gateway de aplicativos Claude e outro produto
* [Gateway de aplicativos Claude](/docs/pt/claude-apps-gateway): gateway auto-hospedado da Anthropic com entrada SSO e telemetria OTLP
* [Conectar Claude Code a um gateway LLM](/docs/pt/llm-gateway-connect): defina a URL base e credencial em sua própria máquina, com configuração por superfície e uma tabela de solução de problemas
* [Implantar um gateway LLM para sua organização](/docs/pt/llm-gateway-rollout): a lista de verificação do administrador para implantar um gateway, emitir credenciais de desenvolvedor e distribuir configurações gerenciadas
* [Referência de protocolo de gateway](/docs/pt/llm-gateway-protocol): o que Claude Code envia a um gateway, para operadores configurando um, abrangendo endpoints, cabeçalhos para encaminhar e passagem de recursos
