> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code no Microsoft Foundry

> Saiba como configurar Claude Code através do Microsoft Foundry, incluindo configuração, instalação e resolução de problemas.

export const ContactSalesCard = ({surface}) => {
  const utm = content => `utm_source=claude_code&utm_medium=docs&utm_content=${surface}_${content}`;
  const iconArrowRight = (size = 13) => <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
      <line x1="5" y1="12" x2="19" y2="12" />
      <polyline points="12 5 19 12 12 19" />
    </svg>;
  const STYLES = `
.cc-cs {
  --cs-slate: #141413;
  --cs-clay: #d97757;
  --cs-clay-deep: #c6613f;
  --cs-gray-000: #ffffff;
  --cs-gray-700: #3d3d3a;
  --cs-border-default: rgba(31, 30, 29, 0.15);
  font-family: inherit;
}
.dark .cc-cs {
  --cs-slate: #f0eee6;
  --cs-gray-000: #262624;
  --cs-gray-700: #bfbdb4;
  --cs-border-default: rgba(240, 238, 230, 0.14);
}
.cc-cs-card {
  display: flex; align-items: center; justify-content: space-between;
  gap: 16px; padding: 14px 16px; margin: 0;
  background: var(--cs-gray-000); border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; flex-wrap: wrap;
}
.cc-cs-text { font-size: 13px; color: var(--cs-gray-700); line-height: 1.5; flex: 1; min-width: 240px; }
.cc-cs-text strong { font-weight: 550; color: var(--cs-slate); }
.cc-cs-actions { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
.cc-cs-btn-clay {
  display: inline-flex; align-items: center; gap: 8px;
  background: var(--cs-clay-deep); color: #fff; border: none;
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
  transition: background-color 0.15s; white-space: nowrap;
}
.cc-cs-btn-clay:hover { background: var(--cs-clay); }
.cc-cs-btn-ghost {
  display: inline-flex; align-items: center; gap: 8px;
  background: transparent; color: var(--cs-gray-700);
  border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
}
.cc-cs-btn-ghost:hover { background: rgba(0, 0, 0, 0.04); }
.dark .cc-cs-btn-ghost:hover { background: rgba(255, 255, 255, 0.04); }
@media (max-width: 720px) {
  .cc-cs-actions { width: 100%; }
}
`;
  return <div className="cc-cs not-prose">
      <style>{STYLES}</style>
      <div className="cc-cs-card">
        <div className="cc-cs-text">
          <strong>Deploying Claude Code across your organization?</strong> Talk to sales about enterprise plans, SSO, and centralized billing.
        </div>
        <div className="cc-cs-actions">
          <a href={`https://claude.com/pricing?${utm('view_plans')}#plans-business`} className="cc-cs-btn-ghost">
            View plans
          </a>
          <a href={`https://claude.com/contact-sales?${utm('contact_sales')}`} className="cc-cs-btn-clay">
            Contact sales {iconArrowRight()}
          </a>
        </div>
      </div>
    </div>;
};

<ContactSalesCard surface="foundry" />

<h2 id="prerequisites">
  Pré-requisitos
</h2>

Antes de configurar Claude Code com Microsoft Foundry, certifique-se de que você tem:

* Uma assinatura do Azure com acesso ao Microsoft Foundry
* Permissões RBAC para criar recursos e implantações do Microsoft Foundry
* Azure CLI instalado e configurado (opcional - necessário apenas se você não tiver outro mecanismo para obter credenciais)

<Note>
  Se você está implantando Claude Code para vários usuários, [fixe suas versões de modelo](#4-pin-model-versions) antes de fazer o lançamento.
</Note>

<h2 id="setup">
  Configuração
</h2>

<h3 id="1-provision-microsoft-foundry-resource">
  1. Provisionar recurso do Microsoft Foundry
</h3>

Primeiro, crie um recurso Claude no Azure:

1. Navegue até o [portal do Microsoft Foundry](https://ai.azure.com/)
2. Crie um novo recurso, anotando o nome do seu recurso
3. Crie implantações para os modelos Claude, anotando o nome de implantação que você atribui a cada um; você definirá esses nomes como as variáveis de modelo na etapa 4:
   * Claude Opus
   * Claude Sonnet
   * Claude Haiku

<h3 id="2-configure-azure-credentials">
  2) Configurar credenciais do Azure
</h3>

Claude Code suporta três métodos de autenticação para Microsoft Foundry. Escolha o método que melhor se adequa aos seus requisitos de segurança.

**Opção A: Autenticação por chave de API**

1. Navegue até seu recurso no portal do Microsoft Foundry
2. Vá para a seção **Endpoints e chaves**
3. Copie a **Chave de API**
4. Defina a variável de ambiente, substituindo `your-azure-api-key` pela chave que você copiou:

```bash theme={null}
export ANTHROPIC_FOUNDRY_API_KEY=your-azure-api-key
```

**Opção B: Autenticação do Microsoft Entra ID**

Quando nem `ANTHROPIC_FOUNDRY_API_KEY` nem `ANTHROPIC_FOUNDRY_AUTH_TOKEN` estão definidos, Claude Code usa automaticamente a [cadeia de credenciais padrão](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview) do Azure SDK.
Isso suporta uma variedade de métodos para autenticar cargas de trabalho locais e remotas.

Em ambientes locais, você pode usar comumente a Azure CLI:

```bash theme={null}
az login
```

**Opção C: Autenticação por token de portador**

Claude Code envia o valor de `ANTHROPIC_FOUNDRY_AUTH_TOKEN` em cada solicitação como o cabeçalho `Authorization: Bearer`. Use esta opção quando outro processo, como um aplicativo host ou um script de login, já tiver obtido um token de acesso para você. Requer Claude Code v2.1.203 ou posterior.

Defina a variável para um token de portador que o Microsoft Entra ID emitiu para seu recurso:

```bash theme={null}
export ANTHROPIC_FOUNDRY_AUTH_TOKEN=your-entra-access-token
```

`ANTHROPIC_FOUNDRY_AUTH_TOKEN` tem precedência sobre `ANTHROPIC_FOUNDRY_API_KEY` e sobre a cadeia de credenciais padrão.

<Note>
  Ao usar Microsoft Foundry, o comando `/logout` não está disponível, pois a autenticação é tratada através de credenciais do Azure.
</Note>

<h3 id="3-configure-claude-code">
  3. Configurar Claude Code
</h3>

Defina as seguintes variáveis de ambiente para ativar Microsoft Foundry:

```bash theme={null}
# Ativar integração do Microsoft Foundry
export CLAUDE_CODE_USE_FOUNDRY=1

# Nome do recurso do Azure (substitua {resource} pelo nome do seu recurso)
export ANTHROPIC_FOUNDRY_RESOURCE={resource}
# Ou forneça a URL base completa:
# export ANTHROPIC_FOUNDRY_BASE_URL=https://{resource}.services.ai.azure.com/anthropic
```

<h3 id="4-pin-model-versions">
  4. Fixar versões de modelo
</h3>

<Warning>
  Fixe versões de modelo específicas para cada implantação. Sem fixar, aliases de modelo como `sonnet` e `opus` resolvem para o padrão integrado do Claude Code para Microsoft Foundry, que pode ficar atrás da versão mais recente e pode ainda não estar disponível em sua conta. Microsoft Foundry não possui verificação de modelo na inicialização, portanto as solicitações falham quando o padrão não está disponível. Quando você criar implantações do Azure, selecione uma versão de modelo específica em vez de "atualizar automaticamente para a mais recente".
</Warning>

Defina as variáveis de modelo para corresponder aos nomes de implantação que você criou na etapa 1.

Sem `ANTHROPIC_DEFAULT_OPUS_MODEL`, o alias `opus` no Microsoft Foundry resolve para Opus 4.6. Defina-o para o ID Opus 4.8 para usar o modelo mais recente:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5'
```

As tarefas em segundo plano, como geração de título de sessão, usam o modelo pequeno/rápido, normalmente um modelo da classe Haiku. No Microsoft Foundry, Claude Code usa como padrão o modelo primário porque nem toda conta tem uma implantação Haiku. Para usar Haiku para tarefas em segundo plano, defina `ANTHROPIC_DEFAULT_HAIKU_MODEL` para uma implantação Haiku que esteja disponível em sua conta, conforme mostrado acima.

Para IDs de modelo atuais e legados, consulte [Visão geral de modelos](https://platform.claude.com/docs/en/about-claude/models/overview). Consulte [Configuração de modelo](/docs/pt/model-config#pin-models-for-third-party-deployments) para a lista completa de variáveis de ambiente.

[Prompt caching](/docs/pt/prompt-caching) está ativado automaticamente. Para solicitar um TTL de cache de 1 hora em vez do padrão de 5 minutos, defina a seguinte variável; gravações de cache com TTL de 1 hora são cobradas a uma taxa mais alta:

```bash theme={null}
export ENABLE_PROMPT_CACHING_1H=1
```

<h3 id="5-run-claude-code">
  5. Executar Claude Code
</h3>

Com as variáveis de ambiente definidas, inicie Claude Code a partir do diretório do seu projeto:

```bash theme={null}
claude
```

Claude Code lê `CLAUDE_CODE_USE_FOUNDRY` e as outras variáveis Microsoft Foundry do ambiente e se conecta ao seu recurso Azure no primeiro prompt. Diferentemente de Amazon Bedrock e Google Cloud's Agent Platform, Microsoft Foundry não possui um assistente de configuração interativo, portanto as variáveis de ambiente nas etapas 3 e 4 são o único caminho de configuração.

Para verificar sua configuração, execute `/status` dentro do Claude Code. A linha do provedor de API mostra `Microsoft Foundry`, junto com o nome do recurso ou URL base que você configurou.

<h2 id="azure-rbac-configuration">
  Configuração do Azure RBAC
</h2>

As funções padrão `Azure AI User` e `Cognitive Services User` incluem todas as permissões necessárias para invocar modelos Claude.

Para permissões mais restritivas, crie uma função personalizada com o seguinte:

```json theme={null}
{
  "permissions": [
    {
      "dataActions": [
        "Microsoft.CognitiveServices/accounts/providers/*"
      ]
    }
  ]
}
```

Para detalhes, consulte [documentação RBAC do Microsoft Foundry](https://learn.microsoft.com/en-us/azure/ai-foundry/concepts/rbac-azure-ai-foundry).

<h2 id="troubleshooting">
  Resolução de problemas
</h2>

Se você receber um erro "Failed to get token from azureADTokenProvider: ChainedTokenCredential authentication failed":

* Configure Entra ID no ambiente, ou defina `ANTHROPIC_FOUNDRY_API_KEY`.

Se as solicitações falharem com erros de conexão repetidos no primeiro prompt:

* Verifique se `ANTHROPIC_FOUNDRY_RESOURCE` está definido para o nome do recurso real em vez de um espaço reservado. Claude Code constrói a URL do endpoint a partir deste valor, portanto, um nome incorreto aponta para um host que não existe.

<h2 id="additional-resources">
  Recursos adicionais
</h2>

* [Documentação do Microsoft Foundry](https://learn.microsoft.com/en-us/azure/ai-foundry/what-is-azure-ai-foundry)
* [Modelos do Microsoft Foundry](https://ai.azure.com/explore/models)
* [Preços do Microsoft Foundry](https://azure.microsoft.com/en-us/pricing/details/ai-foundry/)
