> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Testar ambientes auto-hospedados de ponta a ponta

> Verifique uma imagem de executor auto-hospedado a partir de CI: despache uma sessão com a CLI, leia as respostas do Claude através de um hook Stop e execute o loop completo.

<Note>
  Ambientes auto-hospedados estão em beta público em planos Team e Enterprise; [Disponibilidade e limitações](/docs/pt/self-hosted-environments#availability-and-limitations) cobre o caminho de habilitação. Esta página é a receita de teste de CI; consulte o [guia de início rápido](/docs/pt/self-hosted-environments-quickstart) para configuração e [Implantar em produção](/docs/pt/self-hosted-environments-deploy) para as receitas de frota.
</Note>

Em um [ambiente auto-hospedado](/docs/pt/self-hosted-environments), as [sessões na nuvem](/docs/pt/claude-code-on-the-web) do Claude Code são executadas em uma imagem de executor que você constrói e mantém. Antes de implantar uma nova imagem em seu ambiente de produção, execute uma sessão completa contra um ambiente de teste a partir de um script: crie uma sessão, leia a resposta do Claude, envie um acompanhamento e leia essa resposta também. Esta é a forma de um teste de fumaça de CI que verifica sua imagem de executor, acesso ao git e quaisquer ferramentas personalizadas antes de promover uma alteração.

Esta receita assume que você já [configurou um ambiente e um executor](/docs/pt/self-hosted-environments-quickstart#set-up-an-environment-and-runner), e que seu trabalho de CI inicia o processo do executor no mesmo host que o script de teste, a configuração natural para testar uma nova imagem de executor. Um hook Stop que você instala no executor escreve a resposta final de cada turno em um arquivo local, e o script a lê de lá, portanto as únicas chamadas para a API Anthropic são os dois despachos em si. Se seus executores de teste estão em infraestrutura separada, consulte [Executores de teste remotos](#remote-test-runners).

<h2 id="install-the-capture-hook-on-your-test-runner">
  Instale o hook de captura em seu runner de teste
</h2>

A leitura funciona através de um [hook Stop](/docs/pt/hooks#stop) do Claude Code: quando Claude termina um turno, o hook recebe a mensagem final do assistente como `last_assistant_message` em seu JSON stdin e a anexa a `$E2E_REPLY_DIR/<session_id>.txt`. Instale-o da mesma forma que o [hook Stop commit-nudge](/docs/pt/self-hosted-environments-configuration#prompt-sessions-to-push-their-work), no `~/.claude/` do host do runner, que o runner semeia em cada sessão.

<h3 id="save-the-hook-files">
  Salve os arquivos do hook
</h3>

Salve os dois arquivos abaixo no host do runner:

* O bloco de configurações: mescle em `~/.claude/settings.json` no host do runner
* O script: salve como `~/.claude/hooks/e2e-stop-hook-capture.sh` no host do runner e torne-o executável

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "timeout": 10,
            "command": "\"$CLAUDE_CONFIG_DIR/hooks/e2e-stop-hook-capture.sh\""
          }
        ]
      }
    ]
  }
}
```

```sh theme={null}
#!/bin/sh
# Stop hook for testing a self-hosted environment end to end: writes each
# turn's final assistant reply to $E2E_REPLY_DIR/<session_id>.txt so a
# co-located test driver can read it without calling the Anthropic API.
# Install on the TEST runner only. Requires jq.

# No-op unless the driver is listening. Never fail the turn.
[ -n "${E2E_REPLY_DIR:-}" ] && [ -d "$E2E_REPLY_DIR" ] || exit 0

# CLAUDE_CODE_REMOTE_SESSION_ID is exported in cse_... form; the session
# id the dispatch CLI prints is in session_... form. Same id, different
# prefix.
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0

# last_assistant_message is absent when the final assistant turn had no
# text, such as a tool-use-only turn. The `// empty` filter makes that a
# zero-byte write rather than the literal string "null".
jq -r '.last_assistant_message // empty' >> "$E2E_REPLY_DIR/$sid.txt" 2>/dev/null
exit 0
```

<h3 id="before-you-start-the-runner">
  Antes de iniciar o runner
</h3>

O hook tem estes requisitos:

* Instale-o antes de iniciar o runner. O runner captura `~/.claude/` uma vez na inicialização, portanto um hook adicionado a um runner em execução entra em vigor apenas após uma reinicialização.
* Exporte `E2E_REPLY_DIR` para o processo do runner. O hook é uma operação nula quando a variável não está definida ou o diretório não existe, portanto defina-a onde você inicia o runner, como a unidade systemd, especificação de pod ou etapa de CI. O script de teste abaixo também o requer.

Instale este hook apenas em runners que servem seu ambiente de teste. Ele escreve a resposta final de cada sessão em disco sempre que `E2E_REPLY_DIR` existe, o que é inofensivo em um runner de CI descartável, mas não algo para levar para uma imagem de runner de ambiente de produção onde a variável pode ser definida acidentalmente.

<h2 id="run-the-test-loop">
  Execute o loop de teste
</h2>

Os sinalizadores de dispatch `--environment` e `--ref` requerem Claude Code v2.1.224 ou posterior na máquina que executa o script, o mesmo piso que o próprio runner. Com o hook em vigor e um runner iniciado neste host, o script de teste:

1. Cria uma sessão no ambiente de teste com `claude -p "<prompt>" --environment <environment-id> --output-format json`, executado a partir de um checkout de git para que a CLI possa detectar automaticamente o repositório a partir do remote `origin`. O `--ref <branch>` opcional baseia o checkout da sessão em uma ref nomeada em vez do HEAD local. O comando cria a sessão, imprime uma linha de JSON contendo `session_id` e sai sem aguardar a resposta do Claude.
2. Aguarda a resposta aparecer em `$E2E_REPLY_DIR/<session_id>.txt`, escrita pelo hook Stop no runner assim que o turno é concluído.
3. Envia um acompanhamento com `claude -p "<message>" --cloud <session_id> --output-format json` (consulte [Enviar uma mensagem de acompanhamento para uma sessão em execução](/docs/pt/claude-code-on-the-web#send-follow-ups-from-the-cli)), que publica um evento de usuário na sessão existente e sai.
4. Aguarda a resposta do acompanhamento da mesma forma que a etapa 2.

<h3 id="environment-dispatch-behavior">
  Comportamento de dispatch `--environment`
</h3>

Claude Code cria a sessão, imprime o ID da sessão e um link para ela, e sai.

O sinalizador tem precedência sobre a configuração [`remote.defaultEnvironmentId`](/docs/pt/settings-reference#remote-defaultenvironmentid). Ele não suporta `--output-format stream-json` e não pode ser combinado com sinalizadores que retomam, anexam ou pré-configuram uma sessão, como `--resume`, `--continue`, `--teleport`, `--session-id` ou `--init-only`. `--cloud` é rejeitado com um ID de sessão ou URL, e em execuções não interativas quando carrega uma descrição. Um `--cloud` simples é tratado como ausente. A partir de um terminal, você pode passar a tarefa como a descrição `--cloud` em vez de um prompt posicional.

<h2 id="example-script">
  Script de exemplo
</h2>

O script abaixo executa o loop completo contra `$CLAUDE_TEST_ENVIRONMENT_ID`, o ID `ccpool_...` do seu ambiente de teste, mostrado no diálogo de detalhes do ambiente na página de administração ou retornado pela [chamada create-environment](#create-a-dedicated-test-environment), e afirma uma frase sentinela em cada resposta. Execute-o a partir de um checkout de git do repositório no qual você deseja que a sessão funcione, após iniciar um runner neste host com o hook de captura instalado e `E2E_REPLY_DIR` exportado.

```bash theme={null}
#!/usr/bin/env bash
# End-to-end test against a self-hosted environment, using Stop-hook read-back.
# Prereqs: `claude auth login` has been run on this machine (see "Authenticate
# from CI" below); jq is installed; CLAUDE_TEST_ENVIRONMENT_ID names an
# environment whose runner is the one on this host, with the capture hook
# installed and E2E_REPLY_DIR in its environment.

set -euo pipefail

: "${CLAUDE_TEST_ENVIRONMENT_ID:=${CLAUDE_TEST_POOL_ID:-}}"  # CLAUDE_TEST_POOL_ID is the legacy spelling
: "${CLAUDE_TEST_ENVIRONMENT_ID:?set CLAUDE_TEST_ENVIRONMENT_ID to a ccpool_... id served by a runner on this host}"
: "${E2E_REPLY_DIR:?set E2E_REPLY_DIR to the directory the Stop hook on your test runner writes to, and export it to the runner process}"
: "${TEST_REPO_REF:=main}"

[ -d "$E2E_REPLY_DIR" ] || {
  echo "FAIL: E2E_REPLY_DIR ($E2E_REPLY_DIR) does not exist. The Stop hook on the runner needs it." >&2
  exit 1
}

# Waits until $E2E_REPLY_DIR/<session_id>.txt contains $2, or fails after
# 90 seconds. Tune the timeout to your environment's cold-start time. The
# file is written by the Stop hook on the runner.
await_reply() {
  local expect="$2" f="$E2E_REPLY_DIR/$1.txt"
  local deadline=$(($(date +%s) + 90))
  while :; do
    if [ -f "$f" ] && grep -qF -- "$expect" "$f"; then
      return
    fi
    [ "$(date +%s)" -lt "$deadline" ] || {
      echo "FAIL: '$expect' not in $f within 90s. The Stop hook on the runner did not write it." >&2
      echo "-- $E2E_REPLY_DIR contents --" >&2; ls -la "$E2E_REPLY_DIR" >&2
      [ -f "$f" ] && { echo "-- $f --" >&2; cat "$f" >&2; }
      exit 1
    }
    sleep 1
  done
}

# 1. Create the session on the test environment. Run from a git checkout
# so the CLI can auto-detect the repo. --ref pins the checkout to a named
# ref regardless of local HEAD.
TURN1="e2e-probe-$(date +%s)-$$: say exactly 'ok: custom tools are reachable' and nothing else"
EXPECT1="ok: custom tools are reachable"
create_json=$(claude -p "$TURN1" --environment "$CLAUDE_TEST_ENVIRONMENT_ID" \
  --ref "$TEST_REPO_REF" --output-format json)
echo "create: $create_json"
SESSION_ID=$(jq -er '.session_id' <<<"$create_json")

# 2. Wait for the turn-1 reply.
await_reply "$SESSION_ID" "$EXPECT1"
echo "turn-1 reply ok"

# 3. Post a follow-up via the CLI.
TURN2="e2e-probe-followup-$(date +%s): say exactly 'ok: follow-up delivered' and nothing else"
EXPECT2="ok: follow-up delivered"
followup_json=$(claude -p "$TURN2" --cloud "$SESSION_ID" --output-format json)
echo "followup: $followup_json"
jq -e '.ok == true' <<<"$followup_json" >/dev/null

# 4. Wait for the turn-2 reply.
await_reply "$SESSION_ID" "$EXPECT2"
echo "turn-2 reply ok"

echo "PASS: test-environment round-trip (session $SESSION_ID)"
```

Substitua os prompts `TURN1`/`TURN2` e as sentinelas `EXPECT1`/`EXPECT2` por qualquer coisa que exercite sua configuração, como pedir ao Claude para executar uma de suas ferramentas MCP personalizadas e afirmar sua saída.

<h2 id="remote-test-runners">
  Runners de teste remotos
</h2>

Se seus runners de teste estão em infraestrutura separada, como uma frota Kubernetes persistente com a qual seu trabalho de CI não pode compartilhar um sistema de arquivos, troque a escrita de arquivo no hook Stop por um POST para um endpoint que seu driver escuta:

```sh theme={null}
#!/bin/sh
# Variant of the capture hook for runners on separate infrastructure.
# Set E2E_REPLY_URL on the runner to an endpoint the driver controls.
[ -n "${E2E_REPLY_URL:-}" ] || exit 0
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0
jq -r '.last_assistant_message // empty' | \
  curl -fsS -X POST --data-binary @- "$E2E_REPLY_URL/$sid" >/dev/null 2>&1
exit 0
```

No lado do driver, execute qualquer coisa que aceite o POST e mantenha a resposta até que o teste a solicite, como um pequeno listener HTTP dentro do trabalho de CI ou um receptor de webhook que você já executa. O hook é executado em sua infraestrutura, portanto o endpoint só precisa ser acessível a partir de seus runners.

<h2 id="authenticate-from-ci">
  Autentique a partir de CI
</h2>

Tanto `claude -p ... --environment` quanto `claude -p ... --cloud` autenticam com um token OAuth claude.ai; chaves de API, como `sk-ant-xxxxx`, não são aceitas para nenhuma das duas chamadas. Duas abordagens disponibilizam um token em CI.

<h3 id="long-lived-ci-host">
  Host de CI de longa duração
</h3>

Execute `claude auth login` uma vez interativamente na máquina que executa o script, usando uma conta de usuário dedicada para automação. Claude Code armazena o token no chaveiro do SO no macOS, ou em `~/.claude/.credentials.json` no Linux e Windows. Em um host macOS cujo Keychain não pode ser escrito, como é típico em uma sessão SSH onde o Keychain de login permanece bloqueado, Claude Code armazena o token em `~/.claude/.credentials.json` lá também. Consulte [Gerenciamento de credenciais](/docs/pt/authentication#credential-management).

A CLI atualiza o token de acesso de curta duração automaticamente em cada invocação, mas a concessão de token de atualização subjacente é limitada a 30 dias a partir do login inicial, portanto execute `claude auth login` interativamente nesse host a cada 30 dias.

<h3 id="ephemeral-ci-runners">
  Runners de CI efêmeros
</h3>

Não há token de CI de longa duração para isso hoje. O escopo que concede controle de sessão remota, `user:sessions:claude_code`, é limitado no servidor a 30 dias, portanto `claude setup-token`, que cria um token somente de inferência de um ano, não o cobre. O [segredo do ambiente](/docs/pt/self-hosted-environments-quickstart#set-up-an-environment-and-runner) também não é aceito, pois apenas autoriza um runner a se registrar no ambiente, não a criar sessões.

Para provisionar um login armazenado em um runner efêmero, defina [`CLAUDE_CODE_OAUTH_REFRESH_TOKEN` e `CLAUDE_CODE_OAUTH_SCOPES`](/docs/pt/env-vars#variables) para que `claude auth login` troque o token sem um navegador; o mesmo limite de 30 dias se aplica à concessão de atualização. Entre em contato com sua equipe de conta Anthropic se você precisar de um caminho de identidade de máquina que não esteja vinculado a uma conta humana.

<h2 id="create-a-dedicated-test-environment">
  Crie um ambiente de teste dedicado
</h2>

Crie e exclua ambientes programaticamente para que cada execução de CI obtenha um limpo; o runner que seu trabalho de CI inicia se registra no ambiente novo. As chamadas de criação e exclusão abaixo são os mesmos endpoints que a página de administração **Cloud environments** em claude.ai usa, e requerem o cabeçalho `anthropic-beta: ccr-byoc-2025-07-29`.

<h3 id="mint-the-admin-token">
  Crie o token de administrador
</h3>

`$ADMIN_TOKEN` é um token de acesso OAuth claude.ai para uma conta que possui uma função de Proprietário, criado da mesma forma que [Autentique a partir de CI](#authenticate-from-ci):

* **Crie-o**: execute `claude auth login` com uma conta que possui uma função de Proprietário, depois leia o token de acesso atual de onde [Host de CI de longa duração](#long-lived-ci-host) diz que Claude Code o armazenou.
* **Leia-o novo em cada execução**: a CLI rotaciona o token de acesso, e o mesmo limite de concessão de atualização de 30 dias se aplica, portanto não armazene uma cópia.
* **Passe-o via stdin**: como o exemplo faz, para que o token nunca chegue à lista de argumentos do curl ou seu log de compilação.

<h3 id="create-the-environment">
  Crie o ambiente
</h3>

Capture a resposta sem ecoá-la: `pool_secret` é uma credencial de longa duração que pode registrar runners no ambiente, portanto armazene-a como um segredo de CI mascarado e imprima apenas o ID do ambiente. O formulário `-H @-` que mantém o token fora da lista de processos requer curl 7.55 ou posterior; curl mais antigo trata `@-` como um cabeçalho literal e envia a solicitação sem autorização.

```bash theme={null}
create=$(curl -fsS -X POST -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  -H "content-type: application/json" \
  -d '{"name":"ci-test-environment"}' \
  https://api.anthropic.com/v1/code/runners/self-hosted/pools \
  <<<"Authorization: Bearer $ADMIN_TOKEN")
ENVIRONMENT_ID=$(jq -er .pool.pool_id <<<"$create")
ENVIRONMENT_SECRET=$(jq -er .pool_secret <<<"$create")
```

Até que um [Proprietário ative **Allow self-hosted environments**](/docs/pt/self-hosted-environments#availability-and-limitations) para a organização, a chamada falha com um `403` `permission_error` lendo `self-hosted runners are disabled by your organization's policy`.

Inicie um runner neste host com `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET=$ENVIRONMENT_SECRET`, mais o hook de captura e `E2E_REPLY_DIR` por [Instale o hook de captura](#install-the-capture-hook-on-your-test-runner), depois execute o script de teste.

<h3 id="delete-the-environment">
  Exclua o ambiente
</h3>

Exclua o ambiente quando a execução terminar, para que cada execução de CI comece limpa:

```bash theme={null}
curl -fsS -X DELETE -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  "https://api.anthropic.com/v1/code/runners/self-hosted/pools/$ENVIRONMENT_ID" \
  <<<"Authorization: Bearer $ADMIN_TOKEN"
```
