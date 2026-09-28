> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Legal e conformidade

> Acordos legais, certificações de conformidade e informações de segurança para Claude Code.

<h2 id="legal-agreements">
  Acordos legais
</h2>

<h3 id="license">
  Licença
</h3>

Seu uso do Claude Code está sujeito a:

* [Termos Comerciais de Serviço](https://www.anthropic.com/legal/commercial-terms) - para usuários de Team, Enterprise e Claude API
* [Termos de Serviço do Consumidor](https://www.anthropic.com/legal/consumer-terms) - para usuários de Free, Pro e Max

<h3 id="commercial-agreements">
  Acordos comerciais
</h3>

Se você está usando a Claude API diretamente (1P) ou acessando-a através do Amazon Bedrock ou Google Cloud's Agent Platform (3P), seu acordo comercial existente será aplicado ao uso do Claude Code, a menos que tenhamos acordado mutuamente de outra forma.

<h3 id="can-customers-offer-claude-code-in-their-products">
  Os clientes podem oferecer Claude Code em seus produtos?
</h3>

A menos que tenhamos acordado mutuamente de outra forma, pré-instalar ou executar Claude Code em seus produtos ou serviços (por exemplo, em sandboxes hospedados ou outra infraestrutura de agentes) requer concordar com nossos [Termos Comerciais de Serviço](https://www.anthropic.com/legal/commercial-terms) e cumprir as condições abaixo:

* **O binário do Claude Code não deve ser modificado.** Claude Code deve ser instalado e executado conforme publicado pela Anthropic, e os clientes não podem remover, desabilitar ou restringir nenhum método de autenticação integrado nele (incluindo métodos que permitem fazer login com uma conta Claude ou a chave API do próprio usuário).
* **Os clientes não podem pagar, revender ou intermediar o uso do Claude em nome de seus usuários finais.** Cada usuário final deve se autenticar com sua própria chave API Anthropic, credenciais do plano de assinatura Claude ou credencial do provedor de inferência de terceiros (Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry). Esse uso é cobrado diretamente do usuário final sob seu próprio acordo com a Anthropic ou, para provedores de inferência de terceiros, com o provedor aplicável.

**Usando o nome e logotipo do Claude Code.** Você pode dizer com precisão, em texto simples, que seu produto tem Claude Code pré-instalado ou que executa Claude Code. Mas você não pode usar os nomes ou logotipos do Claude Code ou Anthropic como parte do seu próprio nome de produto, recurso ou empresa, em seu próprio logotipo, ou de uma forma que sugira que a Anthropic construiu, endossa ou é parceira do seu produto. Qualquer outro uso dos nomes ou logotipos da Anthropic é regido por nossas [Diretrizes de Marca Registrada](https://www.anthropic.com/legal/trademark-guidelines) e requer nossa permissão por escrito.

Claude Code permanece regido pelos termos padrão da Anthropic (consulte as seções Licença e Acordos comerciais acima) independentemente da plataforma através da qual é acessado.

<h2 id="compliance">
  Conformidade
</h2>

<h3 id="healthcare-compliance-baa">
  Conformidade em saúde (BAA)
</h3>

Se um cliente executou um Business Associate Agreement (BAA) com a Anthropic e tem [Zero Data Retention (ZDR)](/docs/pt/zero-data-retention) ativado para a organização relevante, esse BAA se estende ao tráfego de API do cliente através do Claude Code.

<h2 id="usage-policy">
  Política de uso
</h2>

<h3 id="acceptable-use">
  Uso aceitável
</h3>

O uso do Claude Code está sujeito à [Política de Uso da Anthropic](https://www.anthropic.com/legal/aup). Os limites de uso anunciados para os planos Pro e Max assumem uso ordinário e individual do Claude Code e do Agent SDK.

<h3 id="authentication-and-credential-use">
  Autenticação e uso de credenciais
</h3>

Claude Code autentica com os servidores da Anthropic usando tokens OAuth ou chaves de API. Esses métodos de autenticação servem a propósitos diferentes:

* **Autenticação OAuth** é destinada exclusivamente para compradores dos planos de assinatura Claude Free, Pro, Max, Team e Enterprise e é projetada para suportar o uso ordinário do Claude Code e de outros aplicativos nativos da Anthropic. Para as etapas de login, consulte [Fazendo login em sua conta Claude](https://support.claude.com/en/articles/13189465-logging-in-to-your-claude-account); para saber como o Claude Code realiza autenticação OAuth, consulte [Autenticação](/docs/pt/authentication).
* **Desenvolvedores** que constroem produtos ou serviços que interagem com as capacidades do Claude, incluindo aqueles que usam o [Agent SDK](/docs/pt/agent-sdk/overview), devem usar autenticação por chave de API através do [Claude Console](https://platform.claude.com/) ou um provedor de nuvem suportado. A Anthropic não permite que desenvolvedores terceirizados ofereçam login Claude.ai em seus próprios aplicativos, ou que roteiem solicitações através de credenciais de plano Free, Pro ou Max em nome de seus usuários. Além disso, desenvolvedores não podem coletar, armazenar ou intermediar credenciais Claude.ai ou tokens de sessão — o login em uma conta Claude deve ser concluído através do fluxo próprio da Anthropic.

Isso não restringe como os clientes provisionam e gerenciam suas próprias chaves de API ou credenciais de provedores de inferência terceirizados — por exemplo, configurando uma chave de API em um ambiente de desenvolvimento, gerenciador de segredos ou imagem de máquina para uso pelos usuários autorizados do cliente — desde que o uso resultante seja cobrado ao proprietário da chave sob seu acordo com a Anthropic (ou o provedor aplicável) e não seja revendido ou intermediado conforme descrito acima. Também não impede que um usuário final faça login no binário Claude Code não modificado com sua própria assinatura Claude, incluindo quando uma plataforma hospeda Claude Code conforme descrito em *Os clientes podem oferecer Claude Code em seus produtos?* acima.

A Anthropic se reserva o direito de tomar medidas para fazer cumprir essas restrições e pode fazê-lo sem aviso prévio.

Para perguntas sobre métodos de autenticação permitidos para seu caso de uso, por favor [entre em contato com vendas](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=legal_compliance_contact_sales).

<h2 id="security-and-trust">
  Segurança e confiança
</h2>

<h3 id="trust-and-safety">
  Confiança e segurança
</h3>

Você pode encontrar mais informações no [Centro de Confiança da Anthropic](https://trust.anthropic.com) e [Hub de Transparência](https://www.anthropic.com/transparency).

<h3 id="security-vulnerability-reporting">
  Relatório de vulnerabilidades de segurança
</h3>

A Anthropic gerencia nosso programa de segurança através do HackerOne. [Use este formulário para relatar vulnerabilidades](https://hackerone.com/4f1f16ba-10d3-4d09-9ecc-c721aad90f24/embedded_submissions/new).

***

© Anthropic PBC. Todos os direitos reservados. O uso está sujeito aos Termos de Serviço aplicáveis da Anthropic.
