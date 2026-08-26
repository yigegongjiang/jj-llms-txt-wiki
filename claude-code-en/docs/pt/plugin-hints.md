> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Recomende seu plugin a partir de sua CLI

> Emita um marcador de uma linha a partir de sua CLI para que Claude Code solicite aos usuários que instalem seu plugin oficial.

Se você mantém uma CLI ou SDK e tem um plugin no marketplace oficial da Anthropic, sua ferramenta pode solicitar aos usuários do Claude Code que instalem esse plugin. Sua CLI escreve um marcador de uma linha para stderr quando detecta que está sendo executada dentro do Claude Code. Claude Code lê o marcador, remove-o da saída e mostra ao usuário um prompt de instalação única.

Claude Code remove a linha de dica da saída do comando antes de enviá-la ao modelo, portanto o marcador nunca aparece na conversa e não é contado para o uso de tokens. O protocolo não requer comandos extras e não altera o que sua CLI imprime para usuários fora do Claude Code.

Esta página é para mantenedores de CLI e SDK. Se você está procurando instalar plugins, consulte [Descobrir e instalar plugins](/docs/pt/discover-plugins).

<h2 id="how-it-works">
  Como funciona
</h2>

Claude Code define a variável de ambiente [`CLAUDECODE`](/docs/pt/env-vars) como `1` para cada comando que executa através das ferramentas Bash e PowerShell, e para comandos de [hook](/docs/pt/hooks). A partir da v2.1.172, também define [`CLAUDE_CODE_CHILD_SESSION`](/docs/pt/env-vars) como `1` nesses mesmos subprocessos. Quando sua CLI vê uma dessas variáveis, ela escreve uma tag auto-fechável `<claude-code-hint />` para stderr. Em comandos de hook, a tag de dica é removida e ignorada. Apenas a saída das ferramentas Bash e PowerShell dispara o prompt de instalação.

Quando Claude Code recebe a saída do comando, ele:

1. Verifica linhas de dica e as remove antes da saída chegar ao modelo
2. Verifica se a dica aponta para um plugin em um marketplace oficial da Anthropic
3. Verifica se o plugin ainda não foi instalado e não foi solicitado antes
4. Mostra ao usuário um prompt de instalação que nomeia o comando que emitiu a dica

Claude Code nunca instala um plugin automaticamente. O usuário sempre confirma.

<h2 id="emit-the-hint">
  Emita a dica
</h2>

As dicas de prompt só são acionadas para plugins listados no marketplace oficial da Anthropic. Consulte [Coloque seu plugin no marketplace oficial](#get-your-plugin-into-the-official-marketplace) antes de enviar a integração.

Gate a emissão em uma variável de ambiente para que o marcador seja improvável de aparecer quando um humano executa seu CLI diretamente, depois escreva a tag para stderr em sua própria linha. Escolha qual variável verificar:

* `CLAUDECODE`: definida em todas as versões do Claude Code, portanto atinge a maioria das sessões. Também é definida em sessões tmux e subprocessos do servidor MCP stdio que Claude Code inicia. Extensões IDE também a definem em seus terminais integrados, onde um humano pode estar executando seu CLI diretamente.
* `CLAUDE_CODE_CHILD_SESSION`: definida apenas em subprocessos que o próprio Claude Code gera, como chamadas de ferramenta, comandos hook e comandos da [linha de status](/docs/pt/statusline), portanto a tag normalmente não atinge um terminal humano. Um processo de longa duração que foi iniciado dentro de uma sessão, como um servidor tmux, captura a variável, portanto shells iniciados posteriormente a partir desse processo ainda mostram a tag bruta. Requer Claude Code v2.1.172 ou posterior, portanto sessões em versões mais antigas perdem a dica.

Os exemplos a seguir fazem gate em `CLAUDECODE` para máximo alcance e emitem uma dica para um plugin chamado `example-cli` no marketplace oficial:

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
  [ -n "$CLAUDECODE" ] &&
    printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
  ```
</CodeGroup>

Substitua `example-cli` pelo nome do seu plugin no marketplace oficial.

<h2 id="choose-where-to-emit">
  Escolha onde emitir
</h2>

Você controla quais caminhos de código emitem a dica. Claude Code deduplica por plugin, portanto emitir em cada invocação não tem desvantagem. Os pontos de contato que funcionam bem incluem:

| Posicionamento                               | Por que funciona                                                    |
| :------------------------------------------- | :------------------------------------------------------------------ |
| Saída de `--help`                            | Claude frequentemente executa help ao explorar uma CLI desconhecida |
| Erros de subcomando desconhecido             | Atinge o momento em que Claude está confuso sobre sua interface     |
| Sucesso de login ou autenticação             | O usuário já está em uma mentalidade de configuração                |
| Mensagem de boas-vindas na primeira execução | Um momento natural de integração                                    |

<h2 id="what-the-user-sees">
  O que o usuário vê
</h2>

Quando a dica passa em todas as verificações, Claude Code mostra um prompt como o seguinte:

```text theme={null}
─────────────────────────────────────────────────────────────
  Recomendação de Plugin

    O comando example-cli sugere instalar um plugin.

    Plugin: example-cli
    Marketplace: claude-plugins-official
    Integração oficial para implantações example-cli

    Você gostaria de instalá-lo?
    ❯ 1. Sim, instalar example-cli
      2. Não
      3. Não, e não mostrar dicas de instalação de plugin novamente

─────────────────────────────────────────────────────────────
```

O prompt nomeia o comando que produziu a dica para que os usuários possam detectar uma incompatibilidade entre a ferramenta e o plugin que ela recomenda. Se o usuário não responder dentro de 30 segundos, o prompt é descartado como **Não**.

A frequência do prompt é limitada:

* **Uma vez por plugin**: após o prompt ser exibido, Claude Code registra o plugin e nunca o solicita novamente, independentemente da resposta do usuário.
* **Uma vez por sessão**: em todas as CLIs da máquina, no máximo um prompt de dica aparece por sessão do Claude Code.

Selecionar **Sim** instala o plugin no escopo do usuário. Selecionar **Não, e não mostrar dicas de instalação de plugin novamente** desabilita todos os prompts de dica futuros para o usuário.

<h2 id="hint-format">
  Formato da dica
</h2>

A dica é uma tag auto-fechável com três atributos obrigatórios.

```text theme={null}
<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />
```

| Atributo | Obrigatório | Descrição                                           |
| :------- | :---------- | :-------------------------------------------------- |
| `v`      | Sim         | Versão do protocolo. `1` é o único valor suportado  |
| `type`   | Sim         | Tipo de dica. `plugin` é o único valor suportado    |
| `value`  | Sim         | Identificador do plugin na forma `name@marketplace` |

Os valores dos atributos podem ser citados com aspas duplas ou deixados sem aspas. Valores sem aspas não podem conter espaços em branco. Sequências de escape não são suportadas.

<h2 id="requirements">
  Requisitos
</h2>

Claude Code impõe duas condições antes de agir em uma dica. Dicas que falham em qualquer uma das verificações são descartadas:

* **Linha própria**: a tag deve ocupar sua própria linha. Uma tag incorporada no meio da linha, por exemplo dentro de uma instrução de log, é ignorada. Espaço em branco à esquerda e à direita na linha é permitido.
* **Marketplace oficial**: o `value` deve fazer referência a um plugin em um marketplace controlado pela Anthropic, como `claude-plugins-official`. Dicas que apontam para outros marketplaces são silenciosamente descartadas.

A linha de dica é sempre removida da saída antes de chegar ao modelo, mesmo quando a versão ou tipo não é reconhecido, portanto o marcador nunca é contado para o uso de tokens.

As orientações restantes são recomendadas, mas não obrigatórias. Claude Code não pode observar se sua CLI as segue:

* **Escrever para stderr**: stderr mantém a tag fora de pipelines de shell, como `example-cli deploy | jq`. Claude Code verifica ambos os fluxos, portanto stdout também funciona.
* **Gate em uma variável de ambiente**: emita apenas quando `CLAUDECODE` ou `CLAUDE_CODE_CHILD_SESSION` estiver definido. Consulte [Emitir a dica](#emit-the-hint) para saber como as duas variáveis diferem.

<h2 id="get-your-plugin-into-the-official-marketplace">
  Coloque seu plugin no marketplace oficial
</h2>

O protocolo de dica só entra em vigor para plugins listados no marketplace oficial da Anthropic, `claude-plugins-official`. A Anthropic cura esse marketplace a seu critério, e os formulários de envio no aplicativo adicionam plugins ao [marketplace da comunidade](/docs/pt/plugins#submit-your-plugin-to-the-community-marketplace), que o protocolo de dica não verifica. Se você está trabalhando com um contato de parceiro da Anthropic, entre em contato com ele para coordenar uma listagem no marketplace oficial.

<h2 id="see-also">
  Veja também
</h2>

* [Criar plugins](/docs/pt/plugins): construa o plugin que sua CLI recomenda
* [Criar e distribuir um marketplace de plugins](/docs/pt/plugin-marketplaces): hospede plugins fora do marketplace oficial
* [Variáveis de ambiente](/docs/pt/env-vars): referência completa para `CLAUDECODE` e variáveis relacionadas
