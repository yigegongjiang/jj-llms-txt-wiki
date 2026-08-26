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

* [Termos Comerciais](https://www.anthropic.com/legal/commercial-terms) - para usuários de Team, Enterprise e Claude API
* [Termos de Serviço do Consumidor](https://www.anthropic.com/legal/consumer-terms) - para usuários de Free, Pro e Max

<h3 id="commercial-agreements">
  Acordos comerciais
</h3>

Se você está usando a Claude API diretamente (1P) ou acessando-a através do Amazon Bedrock ou Google Cloud's Agent Platform (3P), seu acordo comercial existente será aplicado ao uso do Claude Code, a menos que tenhamos acordado mutuamente de outra forma.

<h2 id="compliance">
  Conformidade
</h2>

<h3 id="healthcare-compliance-baa">
  Conformidade em saúde (BAA)
</h3>

Se um cliente tem um Business Associate Agreement (BAA) conosco e deseja usar Claude Code, o BAA será automaticamente estendido para cobrir Claude Code se o cliente tiver executado um BAA e tiver [Zero Data Retention (ZDR)](/docs/pt/zero-data-retention) ativado. O BAA será aplicável ao tráfego de API desse cliente fluindo através do Claude Code. ZDR é habilitado por organização, portanto cada organização deve ter ZDR habilitado separadamente para ser coberta sob o BAA.

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
* **Desenvolvedores** que constroem produtos ou serviços que interagem com as capacidades do Claude, incluindo aqueles que usam o [Agent SDK](/docs/pt/agent-sdk/overview), devem usar autenticação por chave de API através do [Claude Console](https://platform.claude.com/) ou um provedor de nuvem suportado. A Anthropic não permite que desenvolvedores terceirizados ofereçam login Claude.ai ou roteiem solicitações através de credenciais de plano Free, Pro ou Max em nome de seus usuários.

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
