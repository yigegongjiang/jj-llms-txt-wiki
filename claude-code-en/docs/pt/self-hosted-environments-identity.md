> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Verificar identidade de sessão em ambientes auto-hospedados

> Verifique o JWT CLAUDE_CODE_SESSION_ACCESS_TOKEN para que os serviços em sua rede possam confiar em solicitações de sessões em seu ambiente auto-hospedado.

<Note>
  Ambientes auto-hospedados estão em beta pública em planos Team e Enterprise; um [Owner](/docs/pt/cloud-environments#organization-shared-environments) os habilita ativando **Allow self-hosted environments** na [página de administração **Cloud environments**](https://claude.ai/admin-settings/cloud-environments). Esta página aborda verificação de identidade de sessão; consulte o [quickstart](/docs/pt/self-hosted-environments-quickstart) para configuração e [Deploy to production](/docs/pt/self-hosted-environments-deploy) para as receitas de frota.
</Note>

Um [ambiente auto-hospedado](/docs/pt/self-hosted-environments) permite que sessões do [Claude Code na web](/docs/pt/claude-code-on-the-web) sejam executadas em infraestrutura que você opera em vez de na Anthropic. Como a sessão é executada dentro de sua rede, Claude pode chamar seus serviços internos diretamente. Esses serviços precisam de uma forma de confirmar que uma solicitação veio de uma sessão Claude Code em seu ambiente e de identificar a identidade do usuário ou serviço que criou essa sessão.

Cada sessão em um ambiente auto-hospedado recebe um JSON Web Token (JWT) assinado na variável de ambiente `CLAUDE_CODE_SESSION_ACCESS_TOKEN`. Uma sessão apresenta o token como qualquer credencial de portador; por exemplo, um script que Claude executa pode chamar seu serviço com `curl -H "Authorization: Bearer $CLAUDE_CODE_SESSION_ACCESS_TOKEN"`. Anthropic assina o token e publica as chaves de verificação em um endpoint JWKS público. Seus serviços buscam essas chaves, verificam a assinatura e leem as declarações para decidir qual acesso conceder.

<h2 id="the-session-token">
  O token de sessão
</h2>

Antes de escrever código de verificação, saiba o que o token estabelece e a forma que sua biblioteca JWT verá.

<h3 id="what-the-token-proves">
  O que o token prova
</h3>

Um token válido estabelece alguns fatos e deliberadamente não estabelece outros:

* **Prova**: Anthropic emitiu o token para uma sessão específica em um ambiente específico, e como a sessão foi criada: por um usuário em sua organização, ou pela identidade de serviço de sua organização, que é como [sessões de canal Claude Tag](https://claude.com/docs/claude-tag/concepts/agent-identity) começam
* **Não prova**: qual processo no host do runner o apresenta. O token fica em uma variável de ambiente dentro da sessão, portanto qualquer código que Claude executa e qualquer ferramenta ou servidor MCP que a sessão inicia pode lê-lo e apresentá-lo.

Duas consequências para seus serviços:

* Verifique a declaração `aud` contra seu ID de ambiente, o valor `ccpool_...` mostrado com seu ambiente na [página de administração **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), para rejeitar tokens emitidos para o ambiente de qualquer outra organização.
* Escope as credenciais que você deriva do token para o que uma única sessão de codificação deve ser capaz de fazer, não para tudo que o criador da sessão pode fazer. Consulte [Escopo de credenciais derivadas](#scope-derived-credentials).

<h3 id="token-format">
  Formato do token
</h3>

O valor de `CLAUDE_CODE_SESSION_ACCESS_TOKEN` tem um prefixo `sk-ant-cc-` seguido por um JWT padrão de três partes:

```text theme={null}
sk-ant-cc-<base64url header>.<base64url payload>.<base64url signature>
```

Remova o prefixo antes de passar o valor para uma biblioteca JWT. Tokens emitidos para sessões de nuvem hospedadas pela Anthropic carregam um prefixo `sk-ant-si-` em vez disso e são assinados por um conjunto de chaves diferente, portanto rejeite qualquer valor que não comece com `sk-ant-cc-`.

O algoritmo de assinatura é `ES256`, que é ECDSA na curva P-256 com SHA-256. O cabeçalho do token carrega um `kid` que identifica qual chave no JWKS o assinou.

<h2 id="verify-the-token">
  Verificar o token
</h2>

A verificação é executada em um de dois lugares. Serviços em sua rede verificam o token criptograficamente contra as chaves publicadas pela Anthropic, e scripts de wrapper dentro da sessão podem usar o decodificador integrado do binário do runner.

<h3 id="verify-the-token-from-your-service">
  Verificar o token de seu serviço
</h3>

Anthropic publica as chaves de verificação em um endpoint público e não autenticado:

```text theme={null}
https://api.anthropic.com/v1/code/.well-known/jwks.json
```

A resposta é um [JSON Web Key Set](https://www.rfc-editor.org/rfc/rfc7517) padrão. Anthropic rotaciona as chaves de assinatura periodicamente, e as chaves anteriores a uma rotação permanecem no conjunto tempo suficiente para que os tokens que assinaram continuem a verificar, portanto não fixe uma única chave. O endpoint define `Cache-Control: public, max-age=300`, portanto armazenar em cache o conjunto de chaves e refazer a busca a cada cinco minutos é seguro.

Verifique cada token de entrada contra estas verificações:

<Steps>
  <Step title="Verificar o prefixo">
    Rejeite o valor se não começar com `sk-ant-cc-`, depois remova esse prefixo. O restante é um JWT compacto padrão.
  </Step>

  <Step title="Verificar a assinatura">
    Busque o JWKS, selecione a chave cujo `kid` corresponde ao cabeçalho do token e verifique a assinatura `ES256`. Rejeite tokens cujo cabeçalho `alg` não é `ES256`. Se um token chegar com um `kid` que não está em seu conjunto de chaves em cache, refaça a busca do JWKS uma vez antes de rejeitá-lo: após uma rotação, novos tokens são assinados com uma chave que seu conjunto em cache ainda não possui.
  </Step>

  <Step title="Verificar o emissor">
    Rejeite o token se `iss` não for exatamente `ccr`.
  </Step>

  <Step title="Verificar a audiência contra seu ambiente">
    A declaração `aud` é uma matriz. Rejeite o token a menos que contenha seu ID de ambiente, que tem a forma `ccpool_...`. O ID do ambiente é mostrado no diálogo de detalhes do seu ambiente na [página de administração **Cloud environments**](https://claude.ai/admin-settings/cloud-environments), e aparece como a declaração `ccr:pool_id` em qualquer um dos tokens de sessão do ambiente. Esta verificação é o que escopa o token para seu ambiente e rejeita tokens emitidos para outras organizações.
  </Step>

  <Step title="Verificar a função">
    Rejeite o token se `ccr:role` não for exatamente `session_worker`. Outros tokens emitidos para ambientes auto-hospedados, como segredos de ambiente, tokens de runner e ordens de trabalho, são assinados pelo mesmo conjunto de chaves, mas carregam funções diferentes.
  </Step>

  <Step title="Verificar expiração">
    Rejeite o token se `exp` estiver no passado. Anthropic emite tokens de sessão com um tempo de vida padrão de quatro horas e um máximo de oito horas. O runner atualiza o token antes da expiração e envia o novo valor para a sessão, portanto os subprocessos que Claude inicia após uma atualização o herdam. Uma sessão pode, portanto, apresentar vários tokens válidos distintos ao seu serviço ao longo de sua vida útil.
  </Step>

  <Step title="Ler a identidade">
    A identidade do usuário criador está na declaração `act`: `act.sub` é seu ID de usuário Anthropic no formulário prefixado `user:<id>`, e `act.email`, quando a superfície criadora registrou um, é seu endereço de email. Sessões que a identidade de serviço de sua organização cria, incluindo sessões de canal Claude Tag, carregam um assunto `agent:` em vez disso, portanto trate uma sessão como criada pelo usuário apenas quando `act.sub` carrega o prefixo `user:`, em vez de testar se as declarações de identidade estão ausentes. Consulte a [referência de declarações](#claims-reference) para a estrutura completa e as declarações duplicadas simples.
  </Step>
</Steps>

As verificações mapeiam diretamente para bibliotecas JWT padrão. Os exemplos abaixo implementam a sequência completa em Node.js com [`jose`](https://www.npmjs.com/package/jose), que lida com busca de JWKS, armazenamento em cache e seleção de `kid`, e em Python com [`PyJWT`](https://pyjwt.readthedocs.io/) e seu cliente JWKS integrado.

<Tabs>
  <Tab title="Node.js (jose)">
    ```typescript theme={null}
    import { createRemoteJWKSet, jwtVerify } from "jose";

    const JWKS = createRemoteJWKSet(
      new URL("https://api.anthropic.com/v1/code/.well-known/jwks.json")
    );

    const PREFIX = "sk-ant-cc-";
    const EXPECTED_POOL_ID = "ccpool_...";

    export async function verifySessionToken(raw: string) {
      if (!raw.startsWith(PREFIX)) {
        throw new Error("not a self-hosted runner session token");
      }
      const jwt = raw.slice(PREFIX.length);

      const { payload } = await jwtVerify(jwt, JWKS, {
        issuer: "ccr",
        audience: EXPECTED_POOL_ID,
        algorithms: ["ES256"],
      });

      if (payload["ccr:role"] !== "session_worker") {
        throw new Error("token is not a session_worker token");
      }

      const act = payload.act as { email?: string; sub?: string };
      return {
        sessionId: payload["ccr:session_id"] as string,
        poolId: payload["ccr:pool_id"] as string,
        orgId: payload["ccr:org_id"] as string,
        creatorEmail: act?.email,
        creatorSub: act?.sub,
      };
    }
    ```
  </Tab>

  <Tab title="Python (PyJWT)">
    ```python theme={null}
    import jwt
    from jwt import PyJWKClient

    JWKS_URL = "https://api.anthropic.com/v1/code/.well-known/jwks.json"
    PREFIX = "sk-ant-cc-"
    EXPECTED_POOL_ID = "ccpool_..."

    jwks = PyJWKClient(JWKS_URL)


    def verify_session_token(raw: str) -> dict:
        if not raw.startswith(PREFIX):
            raise ValueError("not a self-hosted runner session token")
        token = raw.removeprefix(PREFIX)

        signing_key = jwks.get_signing_key_from_jwt(token)
        payload = jwt.decode(
            token,
            signing_key.key,
            algorithms=["ES256"],
            issuer="ccr",
            audience=EXPECTED_POOL_ID,
        )

        if payload.get("ccr:role") != "session_worker":
            raise ValueError("token is not a session_worker token")

        act = payload.get("act") or {}
        return {
            "session_id": payload["ccr:session_id"],
            "pool_id": payload["ccr:pool_id"],
            "org_id": payload["ccr:org_id"],
            "creator_email": act.get("email"),
            "creator_sub": act.get("sub"),
        }
    ```
  </Tab>
</Tabs>

<h3 id="verify-the-token-inside-the-session">
  Verificar o token dentro da sessão
</h3>

[Scripts de wrapper](/docs/pt/self-hosted-environments-configuration#wrapper-scripts) são executados dentro da sessão, antes de Claude começar. Em vez de chamar uma biblioteca JWT, eles podem executar o subcomando `self-hosted-runner decode-token` do binário do runner. O subcomando lê o token de um argumento posicional, de `CLAUDE_CODE_SESSION_ACCESS_TOKEN` ou de stdin canalizado, nessa ordem, depois remove o prefixo, verifica a assinatura contra o endpoint JWKS, verifica expiração e imprime as declarações como JSON. O subcomando executa apenas as verificações de assinatura e expiração; não verifica `iss`, `aud` ou `ccr:role`. Quando a decisão de autenticação do seu wrapper depende dessas declarações, leia-as do JSON impresso e compare-as explicitamente.

Este comando extrai a identidade do criador, preferindo o assunto do provedor SSO, depois o endereço de email, depois o assunto `act.sub` do criador, `user:<id>` ou `agent:<id>`:

```bash theme={null}
"$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token | jq -re '.act.attested_by.sub // .act.email // .act.sub'
```

Wrappers recebem o caminho absoluto para o binário do próprio runner em `CLAUDE_RUNNER_CLAUDE_BIN`; use esse caminho em vez de um `claude` resolvido por PATH para que a decodificação seja executada no mesmo binário que o runner usa.

Use `jq -re` em vez de `jq -r` para que uma declaração ausente cause uma saída diferente de zero. Com apenas `-r`, uma declaração ausente imprime a string literal `null` e sai com zero, o que silenciosamente passa um valor ruim para jusante. Passe `--no-verify` para `decode-token` apenas para inspeção offline onde o endpoint JWKS está inacessível.

<h2 id="claims-reference">
  Referência de declarações
</h2>

A tabela abaixo lista as declarações de token de sessão relevantes para verificação. Leia a identidade do namespace `ccr:*` e da cadeia `act`; as declarações simples `account_email`, `organization_uuid` e `account_uuid` são duplicatas de compatibilidade com versões anteriores que podem ser removidas. Sessões que a identidade de serviço de sua organização cria, incluindo sessões de canal Claude Tag, carregam um assunto `agent:` em `act.sub` e omitem `act.email`, `ccr:account_id`, `account_email` e `account_uuid`. As duas declarações de email também são opcionais para sessões criadas pelo usuário: Anthropic as registra na criação da sessão apenas quando as credenciais da solicitação criadora carregam um email, e uma sessão despachada da CLI pode carecer de ambas, portanto baseie a identidade em `act.sub` ou `ccr:account_id` em vez de email. Tokens também podem carregar declarações adicionais além desta tabela; ignore declarações que você não reconheça.

| Declaração          | Tipo              | Descrição                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------ | :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `iss`               | string            | Sempre `ccr`.                                                                                                                                                                                                                                                                                                                                                                                                  |
| `sub`               | string            | `ccr:session:<session_id>`.                                                                                                                                                                                                                                                                                                                                                                                    |
| `aud`               | matriz de strings | Sempre contém `anthropic-api`. Para sessões em ambientes auto-hospedados, a matriz também contém seu ID de ambiente, como `ccpool_...`. Verifique o ID do ambiente, não `anthropic-api`.                                                                                                                                                                                                                       |
| `exp`               | número            | Expiração como um timestamp Unix. Tempo de vida padrão de quatro horas, máximo de oito horas.                                                                                                                                                                                                                                                                                                                  |
| `iat`               | número            | Emitido em como um timestamp Unix.                                                                                                                                                                                                                                                                                                                                                                             |
| `jti`               | string            | Identificador de token único.                                                                                                                                                                                                                                                                                                                                                                                  |
| `ccr:role`          | string            | Sempre `session_worker` para tokens de sessão.                                                                                                                                                                                                                                                                                                                                                                 |
| `ccr:session_id`    | string            | O ID da sessão. Mesmo valor que o sufixo de `sub`.                                                                                                                                                                                                                                                                                                                                                             |
| `ccr:pool_id`       | string            | Seu ID de ambiente. Mesmo valor que aparece em `aud`.                                                                                                                                                                                                                                                                                                                                                          |
| `ccr:org_id`        | string            | Seu ID de organização Anthropic.                                                                                                                                                                                                                                                                                                                                                                               |
| `ccr:account_id`    | string            | O ID de conta Anthropic do usuário criador: o valor de `act.sub` sem o prefixo `user:`, um ID marcado `user_...`. O mesmo valor que o [`spawn-runner` hook](/docs/pt/self-hosted-environments-configuration#the-spawn-runner-hook) carrega em `CLAUDE_RUNNER_ACCOUNT_ID` e [`--lock-to-account`](/docs/pt/self-hosted-environments-reference#runner-cli-flags) aceita, portanto os três se comparam como strings iguais. |
| `account_email`     | string            | Duplicata de `act.email`; ausente sempre que `act.email` está.                                                                                                                                                                                                                                                                                                                                                 |
| `organization_uuid` | string            | Seu UUID de organização Anthropic.                                                                                                                                                                                                                                                                                                                                                                             |
| `account_uuid`      | string            | O UUID de conta Anthropic do usuário criador.                                                                                                                                                                                                                                                                                                                                                                  |
| `act`               | objeto            | Cadeia de delegação [RFC 8693](https://www.rfc-editor.org/rfc/rfc8693). Consulte [A cadeia `act`](#the-act-chain).                                                                                                                                                                                                                                                                                             |

<h3 id="the-act-chain">
  A cadeia `act`
</h3>

A declaração `act` registra o caminho de delegação completo da identidade do usuário ou serviço que criou a sessão até o [ambiente](/docs/pt/self-hosted-environments#key-concepts) cujo segredo admitiu o runner, e a identidade que criou esse segredo. O criador é o ator mais externo, portanto `act.sub` os identifica diretamente.

| Caminho           | Descrição                                                                                                                                                                                                                                                              |
| :---------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `act.sub`         | O ID de usuário Anthropic do usuário criador, na forma `user:<id>`, ou `agent:<id>` quando a identidade de serviço de sua organização criou a sessão, como faz para sessões de canal Claude Tag.                                                                       |
| `act.email`       | O endereço de email do usuário criador, quando um foi registrado na criação da sessão. Não o exija; baseie-se em `act.sub`.                                                                                                                                            |
| `act.attested_by` | O atestado do provedor de identidade upstream para o usuário criador, quando disponível. `act.attested_by.sub` é o assunto que seu provedor SSO, como Google ou Okta, emitiu. Prefira isso em vez de `act.email` ao mapear para identidades em seus próprios sistemas. |
| `act.act`         | O runner que gerou a sessão. `act.act.sub` é `ccr:runner:<runner_id>`.                                                                                                                                                                                                 |
| `act.act.act`     | O ambiente. `act.act.act.sub` é `ccr:pool:<pool_id>`.                                                                                                                                                                                                                  |
| `act.act.act.act` | A identidade que criou o segredo do ambiente com o qual o runner se registrou. A cadeia termina aqui.                                                                                                                                                                  |

<h2 id="scope-derived-credentials">
  Escopo de credenciais derivadas
</h2>

O token de sessão identifica a identidade do usuário ou serviço que criou a sessão, mas não o trate como equivalente a esse criador fazendo login diretamente. O token fica em uma variável de ambiente dentro da sessão, portanto qualquer código que Claude executa e qualquer ferramenta ou servidor MCP que a sessão inicia pode lê-lo e apresentá-lo.

A verificação também é offline: um token que verifica contra o JWKS permanece válido até seu `exp`, seja o que for que tenha acontecido com a sessão desde então, e Anthropic não publica um feed de revogação para tokens de sessão. Vincule qualquer coisa que você derive do token de acordo.

Quando seu serviço troca o token por credenciais internas, emita credenciais escopadas para o que uma sessão de codificação deve alcançar:

* **Limitar capacidades**: conceda acesso de leitura e escrita aos recursos que a sessão precisa para tarefas de codificação, não às capacidades administrativas que o criador possui em outro lugar.
* **Limitar tempo de vida**: vincule credenciais derivadas ao `exp` do token, ou mais curto.
* **Auditar como a sessão**: registre `ccr:session_id` e `jti` junto com a identidade do criador para que você possa rastrear ações de volta a uma sessão específica.

<h2 id="related-environment-variables">
  Variáveis de ambiente relacionadas
</h2>

A identidade do criador também aparece em variáveis de ambiente simples em duas superfícies que nunca verificam o token:

* **O [hook `spawn-runner`](/docs/pt/self-hosted-environments-configuration#the-spawn-runner-hook), no orquestrador**: o hook é executado antes de qualquer runner existir para uma sessão enfileirada e recebe a identidade do criador em variáveis como `CLAUDE_RUNNER_ACCOUNT_EMAIL` e `CLAUDE_RUNNER_ACCOUNT_ID`. O orquestrador as lê da ordem de trabalho, o token de uso único assinado que autoriza a geração de um runner, sem verificar a assinatura da ordem de trabalho em si; as declarações são confiáveis porque a ordem de trabalho chega pela conexão do orquestrador com Anthropic, que o segredo do ambiente autentica.
* **[Scripts de wrapper](/docs/pt/self-hosted-environments-configuration#wrapper-scripts), dentro da sessão**: wrappers recebem `CCR_SESSION_ACCOUNT_EMAIL`, o email do criador pré-extraído do token sem verificação de assinatura. A variável é adequada para rotulagem, como trailers de commit, não para decisões de autenticação.

Use as variáveis simples para decisões do lado do orquestrador, como selecionar uma imagem de máquina. Use `CLAUDE_CODE_SESSION_ACCESS_TOKEN` quando um serviço downstream precisa de prova criptográfica independente em vez de confiar no ambiente do runner.

<h2 id="what’s-next">
  Próximas etapas
</h2>

* [Ambientes auto-hospedados](/docs/pt/self-hosted-environments): o ambiente, runner e modelo de sessão; o [quickstart](/docs/pt/self-hosted-environments-quickstart) e [Deploy to production](/docs/pt/self-hosted-environments-deploy) contêm configuração e operações
* [Personalizar sessões](/docs/pt/self-hosted-environments-configuration): scripts de wrapper que consomem o token e o hook `spawn-runner`
* [Referência](/docs/pt/self-hosted-environments-reference): sinalizadores CLI, variáveis de ambiente e métricas
