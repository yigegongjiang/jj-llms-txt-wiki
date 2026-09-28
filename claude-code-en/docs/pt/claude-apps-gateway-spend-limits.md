> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Limites de gastos do gateway de aplicativos Claude

> Limite o gasto de cada desenvolvedor através do gateway de aplicativos Claude por dia, semana ou mês. Defina limites com uma API de administrador e o gateway os aplica em tempo real em cada solicitação.

Limites de gastos limitam quanto cada desenvolvedor pode gastar através do seu [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) em um determinado dia, semana ou mês. Quando um desenvolvedor ultrapassa seu limite, o gateway retorna `429` na próxima solicitação e os bloqueia até que o período seja redefinido ou um administrador aumente o limite. Use limites de gastos para dar a cada desenvolvedor, grupo ou toda a organização um teto de gastos em uma credencial que todos compartilham.

Um gateway de aplicativos Claude encaminha toda a inferência através de uma credencial upstream compartilhada, portanto a fatura do seu provedor atribui tudo a essa credencial, não a desenvolvedores individuais. Sem limites por desenvolvedor, uma frota de agentes descontrolada pode gastar todo o compromisso da organização. Limites de gastos são a visão por desenvolvedor do gateway e o disjuntor em cima dessa fatura compartilhada.

<h2 id="set-a-cap">
  Defina um limite
</h2>

Com o bloco [`admin:`](/docs/pt/claude-apps-gateway-config#admin) configurado em `gateway.yaml`, o gateway fornece uma API de administrador em `/v1/organizations/spend_limits` e aplica limites em tempo real em cada solicitação de inferência. Os limites em si são definidos através dessa API, não em `gateway.yaml`; cada solicitação `POST /v1/organizations/spend_limits` cria ou substitui um limite de `{scope, amount, period}`. A API espelha as formas de transmissão dos endpoints de limites de gastos da [API de Administrador](https://platform.claude.com/docs/en/manage-claude/admin-api) pública da Anthropic, portanto um cliente HTTP escrito contra esse contrato pode direcionar o gateway alterando sua URL base.

Esta solicitação define um padrão em toda a organização de \$500 por mês para cada desenvolvedor:

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "organization"}, "amount": "50000", "period": "monthly"}'
```

Esta solicitação adiciona um limite mais restritivo de \$100 por dia para cada membro do grupo `contractors`:

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "rbac_group", "rbac_group_id": "contractors"}, "amount": "10000", "period": "daily"}'
```

| Campo        | Valores                                             | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ------------ | --------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `scope.type` | `user`, `rbac_group`, `organization`                | `user` direciona um desenvolvedor pelo seu OpenID Connect (OIDC) `sub`, o ID de usuário estável que seu provedor de identidade atribui; passe-o como `scope.user_id`. `rbac_group` direciona um [grupo IdP](/docs/pt/claude-apps-gateway-config#managed) por nome; passe-o como `scope.rbac_group_id`. `organization` é o padrão em toda a organização. O gateway aceita todos os três; o `POST` público da Anthropic é apenas para usuários hoje. |
| `amount`     | String de número inteiro de centavos USD, ou `null` | `null` é ilimitado. `"0"` é um limite zero, que bloqueia todas as solicitações.                                                                                                                                                                                                                                                                                                                                                               |
| `period`     | `daily`, `weekly`, `monthly`                        | Um escopo pode conter um limite por período, e cada um é aplicado independentemente: um desenvolvedor é bloqueado se ultrapassar qualquer um deles.                                                                                                                                                                                                                                                                                           |

Um limite de grupo ou organização é um padrão por assento que cada membro herda, não um pool compartilhado. Por período, o limite efetivo de um desenvolvedor é resolvido nesta ordem: uma substituição por usuário, depois a mais restritiva de seus limites de grupo, depois o padrão da organização, depois ilimitado. [`admin.group_limit_mode: max`](/docs/pt/claude-apps-gateway-config#admin) inverte o desempate de múltiplos grupos para menos restritivo em vez disso.

<h3 id="authenticate-to-the-admin-api">
  Autentique-se na API de administrador
</h3>

Envie um dos seguintes:

* Um cabeçalho `x-api-key` correspondendo a uma chave em [`admin.write_keys`](/docs/pt/claude-apps-gateway-config#admin) para acesso total, ou `admin.read_keys` para acesso somente `GET`. Cada chave carrega um `id` que aparece no log de auditoria como `admin-key:<id>`, portanto dê ao Terraform, CI e cada automação a sua própria.
* Um token de portador do gateway cujo `groups` claim inclui um dos [`admin.admin_groups`](/docs/pt/claude-apps-gateway-config#admin). Este é acesso total e é auditado como `oidc:<sub>`, portanto prefira-o para administradores humanos.

<h2 id="how-enforcement-works">
  Como a aplicação funciona
</h2>

Em cada solicitação `/v1/messages`, o gateway resolve os limites do desenvolvedor e o gasto até a data do período em uma consulta Postgres. Um desenvolvedor acima de qualquer limite recebe um `429` com `error.type: billing_error` e o cabeçalho `x-should-retry: false`.

A mensagem nomeia o período e a hora de redefinição, como `spend limit reached (daily; resets 2026-08-08 00:00 UTC)`, seguida pelo seu [`admin.blocked_message`](/docs/pt/claude-apps-gateway-config#admin) se definido. Quando um desenvolvedor excede vários limites ao mesmo tempo, a mensagem nomeia o limite que é redefinido por último. A resposta também carrega um cabeçalho `retry-after` com os segundos restantes até essa redefinição. Antes de v2.1.225 no servidor do gateway, a mensagem era `spend limit reached` sem período, hora de redefinição ou cabeçalho `retry-after`.

Na v2.1.227 ou posterior, a referência de protocolo em `<public_url>/protocol` também lista os cabeçalhos de resposta de limite de uso exato e o corpo `429`.

Os limites são redefinidos em limites de calendário UTC: diariamente às 00:00 UTC, semanalmente na segunda-feira e mensalmente no primeiro dia. O gateway nunca bloqueia `/v1/messages/count_tokens`, porque a contagem de tokens é gratuita.

<h3 id="how-requests-are-priced">
  Como as solicitações são precificadas
</h3>

Após cada resposta, um medidor de uso lê as contagens de tokens e adiciona o custo aos contadores diário, semanal e mensal. Ele nunca toca nos bytes enviados ao cliente, portanto uma falha de medição não pode quebrar uma resposta. Os valores são estimativas em USD, um disjuntor em vez de uma fatura; para faturamento, reconcilie contra o relatório de uso do seu provedor.

O medidor escolhe as taxas de cada solicitação nesta ordem:

1. Uma linha [`pricing.overrides`](/docs/pt/claude-apps-gateway-config#pricing) correspondente para o upstream que serviu a solicitação. Requer v2.1.227 ou posterior.
2. Preço de lista para o ID do modelo upstream, a string que o gateway envia ao provedor, quando a tabela de custo do Claude Code a reconhece. A tabela aceita formulários Anthropic, Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry ID.
3. Preço de lista para o [`models[].id`](/docs/pt/claude-apps-gateway-config#models) que você mapeou para esse ID upstream, para strings upstream que não carregam nome de modelo, como um ARN de perfil de inferência de aplicação do Amazon Bedrock ou um nome de implantação do Microsoft Foundry. Requer v2.1.218 ou posterior.
4. O nível de modelo desconhecido de \$5/\$25 por milhão de tokens de entrada/saída, portanto um ID que o medidor não consegue colocar nunca é gratuito. O gateway avisa na inicialização e uma vez por ID em tempo de execução quando usa este nível.

Qualquer que seja a taxa aplicável, o medidor então multiplica o valor por [`pricing.multiplier`](/docs/pt/claude-apps-gateway-config#pricing), padrão `1`.

Aborts de cliente também são faturados. Quando um stream termina sem o quadro de uso final do upstream, o medidor fatura uma estimativa de piso de aproximadamente quatro caracteres por token de saída para o texto já enviado ao cliente, portanto abortar solicitações antecipadamente não evita um limite.

<h3 id="postgres-availability">
  Disponibilidade do Postgres
</h3>

A pré-verificação consulta o Postgres com um tempo limite de dois segundos. Se o armazenamento estiver inacessível ou expirar o tempo limite, a aplicação falha aberta por padrão: a solicitação prossegue, o gateway registra um aviso e a resposta não carrega cabeçalhos `anthropic-ratelimit-unified-*`. Defina [`enforcement.fail_closed_on_error: true`](/docs/pt/claude-apps-gateway-config#enforcement) para falhar fechado em vez disso, que retorna o mesmo `429 billing_error` mas com a mensagem `spend limit unavailable` e sem período, hora de redefinição ou cabeçalho `retry-after`. Falha aberta mantém uma interrupção de armazenamento de se tornar uma interrupção de inferência; falha fechada garante nenhum gasto sem medição.

<h3 id="usage-warnings-in-claude-code">
  Avisos de uso no Claude Code
</h3>

Claude Code avisa um desenvolvedor conforme ele se aproxima do seu limite: uma vez que a utilização passa 75%, e novamente acima de 95% do seu limite mais consumido. Quando o gateway bloqueia uma solicitação, Claude Code mostra a mensagem `429` do gateway como está, incluindo seu `admin.blocked_message`.

O aviso funciona com cabeçalhos de resposta:

* Com v2.1.225 ou posterior no servidor do gateway, cada resposta bem-sucedida de `/v1/messages` para um desenvolvedor que tem um limite carrega sua própria utilização de limite e hora de redefinição nos cabeçalhos `anthropic-ratelimit-unified-*`.
* Com v2.1.225 ou posterior na máquina do desenvolvedor também, Claude Code lê os cabeçalhos e mostra o aviso.

Os cabeçalhos sempre descrevem o limite próprio do desenvolvedor: o gateway remove os cabeçalhos de limite de taxa do provedor upstream, que descrevem sua cota compartilhada, e nunca os encaminha.

Com v2.1.251 ou posterior na máquina do desenvolvedor, Claude Code também lê os mesmos cabeçalhos para mostrar uma barra **Spend limit** em `/usage`, com a porcentagem do seu limite usado e quando ele é redefinido, e para adicionar um objeto `rate_limits.spend_limit` à [linha de status](/docs/pt/statusline#rate-limit-usage) entrada. Claude Code mostra ambos como uma porcentagem em vez de um valor em dólar, e não precisa de nada mais recente que v2.1.225 no servidor do gateway.

<h2 id="admin-api-reference">
  Referência da API de administrador
</h2>

Os endpoints abaixo são servidos em `/v1/organizations/spend_limits`.

| Método e caminho                               | Descrição                                                                                                                                                                  |
| ---------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `GET /v1/organizations/spend_limits`           | Listar limites configurados, opcionalmente filtrados para um `scope_type` de `organization`, `rbac_group` ou `user`. Consulta: `?limit=&after_id=&before_id=&scope_type=`. |
| `POST /v1/organizations/spend_limits`          | Criar ou substituir um limite para `{scope, period}`.                                                                                                                      |
| `GET /v1/organizations/spend_limits/{id}`      | Buscar um limite por seu ID com prefixo `spl_`.                                                                                                                            |
| `DELETE /v1/organizations/spend_limits/{id}`   | Deletar um limite. Retorna `{type: "spend_limit_deleted", id}`.                                                                                                            |
| `GET /v1/organizations/spend_limits/effective` | Limite resolvido e gasto até a data por principal por período.                                                                                                             |
| `GET /v1/organizations/spend_limits/audit`     | Trilha de mutação de administrador, mais recente primeiro. Consulta: `?limit=&after_id=`.                                                                                  |

As convenções espelham a API de Administrador da Anthropic:

* Um `type` em cada objeto
* IDs com prefixo `spl_`
* Quantidades como strings de número inteiro de centavos USD; `POST` rejeita qualquer outro `currency` com `400`
* O envelope de erro `{type: "error", error: {type, message}, request_id}`
* Um cabeçalho de resposta `request-id` em cada resposta de administrador, sucesso ou erro; corpos de erro também o carregam como `request_id`

Cada mutação escreve uma linha antes/depois para `admin_audit` na mesma transação, atribuída a `admin-key:<id>` ou `oidc:<sub>`.

O gateway serve os endpoints de limites de gasto apenas. Outras superfícies da API de Administrador, como a fila `spend_limit_increase_requests`, não fazem parte da API de administrador do gateway.

<h3 id="/effective">
  `/effective`
</h3>

`GET /v1/organizations/spend_limits/effective` retorna o schema `SpendSummary` da Anthropic: cada linha é um principal para um período, com o limite resolvido, gasto até a data do período e um objeto `actor`. Diferenças específicas do gateway:

* `user_id` é o OIDC `sub`.
* `actor.name` e `actor.email_address` são `null` até a primeira solicitação de inferência do principal através do gateway. O gateway não tem diretório de usuários; ele registra valores vistos pela última vez do JWT de sessão de cada usuário.
* Cada linha também carrega um array `groups`, os últimos grupos IdP vistos do principal. Esta é uma extensão do gateway para que uma UI de administrador possa mostrar cada nível de limite que se aplica; clientes em forma de Anthropic a ignoram.
* Sem um filtro `user_ids[]`, ele lista principais com gasto registrado, porque o gateway não consegue enumerar todos os membros da organização.

Limites originários de grupo são resolvidos contra esses últimos grupos vistos com o mesmo desempate `group_limit_mode` que a aplicação usa, portanto o visualizador mostra o limite que realmente se aplica.

| Parâmetro de consulta | Descrição                                                                                                                      |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `user_ids[]`          | Repetível. Filtrar para principais específicos por OIDC `sub`.                                                                 |
| `period[]`            | Repetível. Filtrar para linhas `daily`, `weekly` ou `monthly`.                                                                 |
| `sort`                | `spend_desc` lista os maiores gastadores primeiro. Requer exatamente um `period[]`.                                            |
| `q`                   | Filtro de substring insensível a maiúsculas/minúsculas sobre o OIDC `sub`, último email visto e último nome de exibição visto. |
| `limit` / `page`      | Tamanho da página, 1–1000 com padrão de 20, e o cursor opaco do `next_page` da resposta anterior.                              |

<Warning>
  `q=` e `user_ids[]=` usam strings de consulta GET, portanto qualquer proxy frontal ou balanceador de carga captura-os em seus logs de acesso. Se sua política de log de PII for rigorosa, limpe esses parâmetros lá.
</Warning>

<h3 id="/audit">
  `/audit`
</h3>

Retorna a trilha de mutação de limite de gasto: quem mudou qual limite, com snapshots antes/depois, mais recente primeiro. `has_more` é exato. Este endpoint segue as convenções da API de Administrador local em vez de uma forma de transmissão de primeira parte.

<h3 id="pagination">
  Paginação
</h3>

A lista bruta pagina por `after_id` e `before_id`, que são IDs `spl_…` mutuamente exclusivos; os resultados são ordenados por criação e `has_more` reflete a direção da travessia. `/effective` pagina pelo token opaco `next_page` passado de volta como `?page=`, com principais ordenados em ordem crescente para que as páginas permaneçam estáveis enquanto o gasto está sendo registrado. `limit` é 1–1000, padrão 20, em ambos. `/audit` pagina por `after_id`, o `id` numérico do último evento na página anterior, e seu `limit` padrão é 100.

<h2 id="data-lifecycle">
  Ciclo de vida dos dados
</h2>

O gateway mantém quatro tabelas relacionadas a gastos; uma varredura horária aplica as janelas de retenção:

| Tabela             | Conteúdo                                                                         | Retenção                                                                                                  |
| ------------------ | -------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| `spend`            | Contadores até a data do período por principal em centavos                       | [`admin.spend_retention_months`](/docs/pt/claude-apps-gateway-config#admin), padrão 13                         |
| `spend_limits`     | Os limites configurados                                                          | Até deletado via API                                                                                      |
| `admin_audit`      | A trilha de mutação                                                              | [`admin.audit_retention_days`](/docs/pt/claude-apps-gateway-config#admin), padrão 365                          |
| `principal_emails` | Último email visto de cada principal, nome de exibição e grupos IdP. Contém PII. | [`admin.identity_retention_days`](/docs/pt/claude-apps-gateway-config#admin) desde última atividade, padrão 90 |

Quando um desenvolvedor sai, delete qualquer limite por usuário via `DELETE /v1/organizations/spend_limits/{id}`; seu gasto e linhas de identidade envelhecem nas janelas de retenção acima. Para apagar uma pessoa imediatamente, para offboarding ou uma solicitação de acesso de assunto de dados (DSAR), execute `DELETE FROM principal_emails WHERE principal = '<sub>'` diretamente contra o banco de dados do gateway. Isso remove a única tabela contendo seu email, nome e grupos. As linhas `spend` e `admin_audit` referenciam apenas o pseudônimo OIDC `sub` e envelhecem em suas próprias janelas.

<h2 id="related">
  Relacionado
</h2>

* [Configuração de `admin` e `enforcement`](/docs/pt/claude-apps-gateway-config#admin): habilitando a API de administrador e ajustando a retenção
* [Guia de implantação](/docs/pt/claude-apps-gateway-deploy#postgres): schema do Postgres e orientação de backup
