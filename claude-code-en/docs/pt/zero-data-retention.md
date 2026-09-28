> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Retenção zero de dados

> Saiba mais sobre Retenção Zero de Dados (ZDR) para Claude Code, disponível para contas qualificadas no Claude for Enterprise, incluindo escopo, recursos desabilitados e como solicitar ativação.

Retenção Zero de Dados (ZDR) para Claude Code está disponível para contas qualificadas no Claude for Enterprise. Quando ZDR está ativado, prompts e respostas do modelo geradas durante sessões do Claude Code são processadas em tempo real e não são armazenadas pela Anthropic após a resposta ser retornada, exceto quando necessário para cumprir a lei ou combater uso indevido.

<Note>
  ZDR não está incluído no plano padrão do Claude for Enterprise e não pode ser ativado nas suas configurações de administrador. Está disponível para contas qualificadas e requer ativação separada pela Anthropic. Se sua organização requer ZDR, [entre em contato com vendas](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=zero_data_retention_request) ou com sua equipe de conta da Anthropic para confirmar elegibilidade.
</Note>

ZDR no Claude for Enterprise oferece aos clientes empresariais a capacidade de usar Claude Code com retenção zero de dados e acesso a recursos administrativos:

* Controles de custo por usuário
* Dashboard de [Analytics](/docs/pt/analytics)
* [Configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings)
* Logs de auditoria

ZDR para Claude Code no Claude for Enterprise se aplica apenas à plataforma direta da Anthropic. Para implantações do Claude no Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry, consulte as políticas de retenção de dados dessas plataformas.

<h2 id="zdr-scope">
  Escopo do ZDR
</h2>

ZDR cobre inferência do Claude Code no Claude for Enterprise.

<Warning>
  ZDR é ativado por organização. Cada nova organização requer que ZDR seja ativado separadamente pela sua equipe de conta da Anthropic. ZDR não se aplica automaticamente a novas organizações criadas sob a mesma conta. Entre em contato com sua equipe de conta para ativar ZDR para qualquer nova organização.
</Warning>

<h3 id="route-claude-code-traffic-to-your-zdr-organization">
  Rotear o tráfego do Claude Code para sua organização ZDR
</h3>

ZDR se aplica a solicitações que se autenticam em uma organização habilitada para ZDR. Se um desenvolvedor fizer login no Claude Code com uma conta pessoal ou com uma chave de API de uma organização diferente, essas sessões não são cobertas. Para exigir que os logins claude.ai dos desenvolvedores pertençam à sua organização ZDR, implante as configurações gerenciadas `forceLoginMethod` e `forceLoginOrgUUID`; consulte [Restringir login à sua organização](/docs/pt/authentication#restrict-login-to-your-organization), que também explica como essas chaves tratam logins do Claude Console.

<h3 id="what-zdr-covers">
  O que ZDR cobre
</h3>

ZDR cobre chamadas de inferência do modelo feitas através do Claude Code no Claude for Enterprise. Quando você usa Claude Code em seu terminal, os prompts que você envia e as respostas que Claude gera não são retidas pela Anthropic. Isso se aplica a todos os modelos disponíveis para sua organização ZDR. Alguns modelos requerem retenção de dados por padrão; consulte [Disponibilidade de modelos sob ZDR](#model-availability-under-zdr).

<h3 id="what-zdr-does-not-cover">
  O que ZDR não cobre
</h3>

ZDR não se estende aos seguintes itens, mesmo para organizações com ZDR ativado. Esses recursos seguem [políticas padrão de retenção de dados](/docs/pt/data-usage#data-retention):

| Recurso                              | Detalhes                                                                                                                                                                                                                                                                      |
| ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Chat no claude.ai                    | Conversas de chat através da interface web do Claude for Enterprise não são cobertas por ZDR.                                                                                                                                                                                 |
| Cowork                               | Sessões de Cowork não são cobertas por ZDR.                                                                                                                                                                                                                                   |
| Claude Code Analytics                | Não armazena prompts ou respostas do modelo, mas coleta metadados de produtividade como emails de conta e estatísticas de uso. Métricas de contribuição não estão disponíveis para organizações ZDR; o [dashboard de analytics](/docs/pt/analytics) mostra apenas métricas de uso. |
| Gerenciamento de usuários e assentos | Dados administrativos como emails de conta e atribuições de assentos são retidos sob políticas padrão.                                                                                                                                                                        |
| Integrações de terceiros             | Dados processados por ferramentas de terceiros, MCP servers ou outras integrações externas não são cobertos por ZDR. Revise as práticas de tratamento de dados desses serviços independentemente.                                                                             |

<h2 id="features-disabled-under-zdr">
  Recursos desabilitados sob ZDR
</h2>

Quando ZDR está ativado para uma organização do Claude Code no Claude for Enterprise, certos recursos que requerem armazenamento de prompts ou conclusões são automaticamente desabilitados no nível do backend:

| Recurso                                                                                                                                  | Motivo                                                                                                          |
| ---------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| [Sessões na nuvem](/docs/pt/claude-code-on-the-web), incluindo aquelas iniciadas a partir do [aplicativo Desktop](/docs/pt/desktop#cloud-sessions) | Requer armazenamento no servidor de dados de sessão, incluindo histórico de conversas com prompts e conclusões. |
| [Claude Tag](https://claude.com/docs/claude-tag)                                                                                         | Retém memória de canal e transcrições de sessão.                                                                |
| [Artefatos](/docs/pt/artifacts)                                                                                                               | Requer armazenamento de conteúdo de página publicado na infraestrutura operada pela Anthropic.                  |
| Envio de feedback (`/feedback`, `/bug`, `/share`)                                                                                        | Enviar feedback envia dados de conversas para a Anthropic.                                                      |
| [Controle Remoto](/docs/pt/remote-control)                                                                                                    | Armazena a transcrição da sessão nos servidores da Anthropic para sincronizar a conversa entre dispositivos.    |

Esses recursos são bloqueados no backend independentemente da exibição no lado do cliente. Se você vir um recurso desabilitado no terminal do Claude Code durante a inicialização, tentar usá-lo retorna um erro indicando que as políticas da organização não permitem essa ação.

Recursos futuros também podem ser desabilitados se exigirem armazenamento de prompts ou conclusões.

<h3 id="model-availability-under-zdr">
  Disponibilidade de modelos sob ZDR
</h3>

Claude Fable 5.1 e Fable 5 são [Modelos Cobertos](https://support.claude.com/en/articles/15425695-covered-models) que [requerem retenção de dados](https://platform.claude.com/docs/en/manage-claude/api-and-data-retention#model-specific-data-retention-requirements) por padrão, e se uma organização ou workspace ZDR pode usá-los é governado pelas políticas de Modelos Cobertos em vez de pelo Claude Code. Onde sua organização não pode usá-los, os modelos estão ausentes do seletor `/model` ou são exibidos como desabilitados, e o servidor rejeita solicitações para eles independentemente da configuração do cliente.

Outros modelos permanecem disponíveis sob ZDR. Modelos Fable não são o padrão, e o alias `best`, que resolve para o modelo Fable mais recente onde está disponível, resolve para Opus para organizações onde não está.

<h2 id="data-retention-for-policy-violations">
  Retenção de dados para violações de política
</h2>

Mesmo com ZDR ativado, a Anthropic pode reter dados quando exigido por lei ou para resolver violações da Política de Uso. Se uma sessão for sinalizada para uma violação de política, a Anthropic pode reter as entradas e saídas associadas por até 2 anos, consistente com a política ZDR padrão da Anthropic.

<h2 id="request-zdr">
  Solicitar ZDR
</h2>

Para solicitar ZDR para Claude Code no Claude for Enterprise, [entre em contato com vendas](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=zero_data_retention_request) ou sua equipe de conta da Anthropic. Sua equipe de conta enviará a solicitação internamente, e a Anthropic revisará e ativará ZDR em sua organização após confirmar a elegibilidade. Todas as ações de ativação são registradas em log de auditoria.

Se você está usando ZDR para Claude Code através de chaves de API pay-as-you-go, você pode fazer a transição para Claude for Enterprise para ganhar acesso a recursos administrativos enquanto mantém ZDR para Claude Code. Entre em contato com sua equipe de conta para coordenar a migração.
