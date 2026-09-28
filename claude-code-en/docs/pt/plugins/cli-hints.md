> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Recomende seu plugin a partir de sua CLI

> Solicite aos usuários do Claude Code que instalem seu plugin do marketplace oficial emitindo uma tag claude-code-hint a partir de sua CLI ou SDK.

Se você mantém uma CLI ou SDK, sua ferramenta pode solicitar aos usuários do Claude Code que instalem seu plugin. Quando sua CLI detecta que está sendo executada dentro do Claude Code, faça-a escrever uma tag `<claude-code-hint />` de uma linha para stderr. Claude Code remove a linha da saída das ferramentas Bash e PowerShell antes do modelo ver a saída, e então mostra ao usuário um prompt de instalação único.

Esta página se aplica apenas se seu plugin está listado em `claude-plugins-official` ou em outro marketplace com um dos [nomes oficiais de marketplace](/docs/pt/plugins/security#official-marketplace-names) da Anthropic. O marketplace da comunidade, `claude-community`, não é um deles.

<Note>
  Para publicar um plugin, consulte [Publicar e distribuir um plugin](/docs/pt/plugins/publish).
</Note>

<h2 id="emit-the-hint">
  Emita a dica
</h2>

Emita a tag apenas quando `CLAUDECODE` ou `CLAUDE_CODE_CHILD_SESSION` estiver definida, para que não apareça quando uma pessoa executa sua CLI diretamente.

Claude Code define `CLAUDECODE=1` nos comandos que executa através das ferramentas Bash e PowerShell e em comandos hook. Na v2.1.172 e posterior, também define `CLAUDE_CODE_CHILD_SESSION=1` lá. As variáveis diferem em quais processos as carregam:

* **`CLAUDECODE`**: definida por todas as versões do Claude Code. As extensões IDE também a definem em seus terminais integrados, portanto um gate apenas em `CLAUDECODE` também emite a tag quando uma pessoa executa sua CLI diretamente em um desses terminais
* **`CLAUDE_CODE_CHILD_SESSION`**: definida apenas em subprocessos que o próprio Claude Code inicia. Use-a quando você puder exigir v2.1.172 ou posterior

A [referência de variáveis de ambiente](/docs/pt/env-vars) tem os detalhes.

Os exemplos a seguir fazem gate em `CLAUDECODE` para o alcance mais amplo e emitem uma dica para um plugin chamado `example-cli` no marketplace oficial:

<CodeGroup>
  ```javascript Node.js theme={null}
  if (process.env.CLAUDECODE) {
    process.stderr.write(
      '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />\n',
    )
  }
  ```

  ```python Python theme={null}
  import os, sys

  if os.environ.get("CLAUDECODE"):
      print(
          '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />',
          file=sys.stderr,
      )
  ```

  ```go Go theme={null}
  if os.Getenv("CLAUDECODE") != "" {
      fmt.Fprintln(os.Stderr,
          `<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />`)
  }
  ```

  ```shell Shell theme={null}
  if [ -n "$CLAUDECODE" ]; then
    printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
  fi
  ```
</CodeGroup>

Substitua `example-cli` pelo nome do seu plugin no marketplace oficial.

Você pode emitir a dica em cada invocação, porque Claude Code solicita para cada plugin uma vez.

Para verificar o emissor, execute `CLAUDECODE=1 example-cli` em um terminal e confirme que a linha da tag aparece em stderr, depois execute `example-cli` sem a variável e confirme que nada extra é impresso.

<h2 id="hint-format">
  Formato da dica
</h2>

A tag deve ocupar sua própria linha; Claude Code ignora uma tag incorporada no meio da linha.

A tag leva três atributos, todos obrigatórios:

| Atributo | Descrição                                           |
| :------- | :-------------------------------------------------- |
| `v`      | Versão do protocolo. `1` é o único valor suportado  |
| `type`   | Tipo de dica. `plugin` é o único valor suportado    |
| `value`  | Identificador do plugin na forma `name@marketplace` |

Os valores podem ser entre aspas duplas ou sem aspas; um valor sem aspas não pode conter espaços em branco.

Claude Code remove a linha da saída mesmo quando `v` ou `type` não é reconhecido.

<h2 id="check-when-the-prompt-appears">
  Verifique quando o prompt aparece
</h2>

O prompt aparece apenas em sessões de terminal interativas. Em execuções `claude -p`, em execuções de subagente e na saída de comandos hook, a tag é removida e nenhum prompt é mostrado. Todas essas verificações também devem passar:

* **Oficial e instalável**: `value` nomeia um plugin que Claude Code encontra em sua cópia local de um marketplace oficial, que ainda não está instalado e que nenhuma política bloqueia
* **Análise ativada**: uma sessão onde a análise do Claude Code está desativada nunca solicita, por exemplo uma com `DISABLE_TELEMETRY`, `DO_NOT_TRACK` ou `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` definida, ou uma em um provedor de terceiros como Amazon Bedrock, onde a [exclusão automática de telemetria](/docs/pt/data-usage#default-behaviors-by-api-provider) se aplica
* **Limites de frequência**: um prompt por sessão, um prompt sempre por plugin independentemente da resposta do usuário, e nenhum uma vez que 100 plugins tenham sido solicitados nessa máquina
* **Não desativado**: o usuário não escolheu **Não, e não mostre mais dicas de instalação de plugin**
* **Sessão local e assistida**: o workspace da sessão é local em vez de estar em uma máquina em nuvem ou remota, e a sessão não está sendo executada sem supervisão. Por exemplo, uma sessão iniciada com `--cloud`, uma servindo Remote Control ou um colega de equipe de agente nunca solicita

<h2 id="preview-what-the-user-sees">
  Visualize o que o usuário vê
</h2>

Quando as verificações em [Verifique quando o prompt aparece](#check-when-the-prompt-appears) passam, Claude Code mostra um diálogo de **Recomendação de plugin** como o seguinte:

```text theme={null}
─────────────────────────────────────────────────────────────
  Recomendação de plugin

    O comando example-cli sugere instalar um plugin.

    Plugin: example-cli
    Marketplace: claude-plugins-official
    Descrição: Integração oficial para implantações example-cli

    Você gostaria de instalá-lo?
    ❯ 1. Sim, instalar
      2. Não
      3. Não, e não mostre mais dicas de instalação de plugin

─────────────────────────────────────────────────────────────
```

O diálogo nomeia a primeira palavra do comando shell que Claude executou, para que os usuários possam detectar uma incompatibilidade. Cada resposta tem um efeito:

* **Sim, instalar**: instala o plugin no [escopo do usuário](/docs/pt/plugins/install)
* **Não, e não mostre mais dicas de instalação de plugin**: desativa futuros prompts de dica para esse usuário
* **Sem resposta por 30 segundos**: conta como **Não**

<h2 id="next-steps">
  Próximas etapas
</h2>

* [Publicar e distribuir um plugin](/docs/pt/plugins/publish): as rotas em cada marketplace, incluindo o marketplace oficial, que a dica requer
* [Referência de comandos de plugin](/docs/pt/plugins/cli-reference#plugin-install): o comando shell que instala o mesmo plugin fora de uma sessão
