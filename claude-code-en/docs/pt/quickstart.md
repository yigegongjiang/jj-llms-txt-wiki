> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Guia de Início Rápido

> Bem-vindo ao Claude Code!

Este guia de início rápido o colocará usando assistência de codificação alimentada por IA em poucos minutos. Ao final, você entenderá como usar Claude Code para tarefas comuns de desenvolvimento.

<h2 id="before-you-begin">
  Antes de começar
</h2>

Certifique-se de que você tem:

* Um terminal ou prompt de comando aberto
  * Se você nunca usou o terminal antes, confira o [guia de terminal](/docs/pt/terminal-guide)
* Um projeto de código para trabalhar
* Uma [assinatura Claude](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_prereq) (Pro, Max, Team ou Enterprise), conta do [Claude Console](https://console.anthropic.com/), ou acesso através de um [provedor de nuvem suportado](/docs/pt/third-party-integrations)

<Note>
  Este guia cobre o CLI do terminal. Claude Code também está disponível na [web](https://claude.ai/code), como um [aplicativo de desktop](/docs/pt/desktop), em [VS Code](/docs/pt/vs-code) e [IDEs JetBrains](/docs/pt/jetbrains), no [Slack](/docs/pt/slack), e em CI/CD com [GitHub Actions](/docs/pt/github-actions) e [GitLab](/docs/pt/gitlab-ci-cd). Veja [todas as interfaces](/docs/pt/overview#use-claude-code-everywhere).
</Note>

<h2 id="step-1-install-claude-code">
  Passo 1: Instale Claude Code
</h2>

To install Claude Code, use one of the following methods:

<Tabs>
  <Tab title="Native Install (Recommended)">
    **macOS, Linux, WSL:**

    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```

    **Windows PowerShell:**

    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```

    **Windows CMD:**

    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```

    If you see `The token '&&' is not a valid statement separator`, you're in PowerShell, not CMD. If you see `'irm' is not recognized as an internal or external command`, you're in CMD, not PowerShell. Your prompt shows `PS C:\` when you're in PowerShell and `C:\` without the `PS` when you're in CMD.

    If the install command fails with `syntax error near unexpected token '<'`, a `403`, or another curl error, see [Troubleshoot installation](/docs/en/troubleshoot-install#find-your-error) to match the error to a fix and for alternative install methods.

    [Git for Windows](https://git-scm.com/downloads/win) is recommended on native Windows so Claude Code can use the Bash tool. If Git for Windows is not installed, Claude Code uses PowerShell as the shell tool instead. WSL setups do not need Git for Windows.

    <Info>
      Native installations automatically update in the background to keep you on the latest version.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew offers two casks. `claude-code` tracks the stable release channel, which is typically about a week behind and skips releases with major regressions. `claude-code@latest` tracks the latest channel and receives new versions as soon as they ship.

    <Info>
      Homebrew installations do not auto-update. Run `brew upgrade claude-code` or `brew upgrade claude-code@latest`, depending on which cask you installed, to get the latest features and security fixes.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      WinGet installations do not auto-update. Run `winget upgrade Anthropic.ClaudeCode` periodically to get the latest features and security fixes.
    </Info>
  </Tab>
</Tabs>

You can also install with [apt, dnf, or apk](/docs/en/setup#install-with-linux-package-managers) on Debian, Fedora, RHEL, and Alpine.

<h2 id="step-2-log-in-to-your-account">
  Passo 2: Faça login em sua conta
</h2>

Claude Code requer uma conta para usar. Inicie uma sessão interativa com o comando `claude` e você será solicitado a fazer login no primeiro uso:

```bash theme={null}
claude
```

Para contas de assinatura Claude ou Console, siga os prompts para concluir a autenticação no seu navegador. Para trocar de contas mais tarde ou fazer nova autenticação, digite `/login` dentro da sessão em execução:

```text theme={null}
/login
```

Você pode fazer login usando qualquer um destes tipos de conta:

* [Claude Pro, Max, Team ou Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_login) (recomendado)
* [Claude Console](https://console.anthropic.com/) (acesso à API com créditos pré-pagos). No primeiro login, um workspace "Claude Code" é criado automaticamente no Console para rastreamento centralizado de custos.
* [Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry](/docs/pt/third-party-integrations) (provedores de nuvem empresariais)
* Um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) auto-hospedado, se sua organização executar um: seu administrador pré-configura a URL do gateway, e `/login` abre diretamente na tela **Cloud gateway** para você fazer login com SSO corporativo

Depois de fazer login, suas credenciais são armazenadas e você não precisará fazer login novamente.

<h2 id="step-3-start-your-first-session">
  Passo 3: Inicie sua primeira sessão
</h2>

Abra seu terminal em qualquer diretório de projeto e inicie Claude Code:

```bash theme={null}
cd /path/to/your/project
claude
```

Você verá o prompt do Claude Code com a versão, modelo atual e diretório de trabalho mostrados acima. Digite `/help` para comandos disponíveis ou `/resume` para continuar uma conversa anterior.

<Tip>
  Depois de fazer login (Passo 2), suas credenciais são armazenadas em seu sistema. Saiba mais em [Gerenciamento de Credenciais](/docs/pt/authentication#credential-management).
</Tip>

<h2 id="step-4-ask-your-first-question">
  Passo 4: Faça sua primeira pergunta
</h2>

Vamos começar entendendo sua base de código. Tente um destes comandos:

```text theme={null}
what does this project do?
```

Claude analisará seus arquivos e fornecerá um resumo. Você também pode fazer perguntas mais específicas:

```text theme={null}
what technologies does this project use?
```

```text theme={null}
where is the main entry point?
```

```text theme={null}
explain the folder structure
```

Você também pode perguntar ao Claude sobre suas próprias capacidades:

```text theme={null}
what can Claude Code do?
```

```text theme={null}
how do I create custom skills in Claude Code?
```

```text theme={null}
can Claude Code work with Docker?
```

<Note>
  Claude Code lê seus arquivos de projeto conforme necessário. Você não precisa adicionar contexto manualmente.
</Note>

<h2 id="step-5-make-your-first-code-change">
  Passo 5: Faça sua primeira alteração de código
</h2>

Agora vamos fazer Claude Code fazer alguma codificação real. Tente uma tarefa simples:

```text theme={null}
add a hello world function to the main file
```

Claude Code irá:

1. Encontrar o arquivo apropriado
2. Mostrar as alterações propostas
3. Pedir sua aprovação
4. Fazer a edição

<Note>
  Claude Code sempre pede permissão antes de modificar arquivos. Você pode aprovar alterações individuais ou ativar o modo "Aceitar tudo" para uma sessão.
</Note>

<h2 id="step-6-use-git-with-claude-code">
  Passo 6: Use Git com Claude Code
</h2>

Claude Code torna as operações Git conversacionais:

```text theme={null}
what files have I changed?
```

```text theme={null}
commit my changes with a descriptive message
```

Você também pode solicitar operações Git mais complexas:

```text theme={null}
create a new branch called feature/quickstart
```

```text theme={null}
show me the last 5 commits
```

```text theme={null}
help me resolve merge conflicts
```

<h2 id="step-7-fix-a-bug-or-add-a-feature">
  Passo 7: Corrija um bug ou adicione um recurso
</h2>

Claude é proficiente em depuração e implementação de recursos.

Descreva o que você quer em linguagem natural:

```text theme={null}
add input validation to the user registration form
```

Ou corrija problemas existentes:

```text theme={null}
there's a bug where users can submit empty forms - fix it
```

Claude Code irá:

* Localizar o código relevante
* Entender o contexto
* Implementar uma solução
* Executar testes se disponíveis

<h2 id="step-8-test-out-other-common-workflows">
  Passo 8: Teste outros fluxos de trabalho comuns
</h2>

Existem várias maneiras de trabalhar com Claude:

**Refatore código**

```text theme={null}
refactor the authentication module to use async/await instead of callbacks
```

**Escreva testes**

```text theme={null}
write unit tests for the calculator functions
```

**Atualize documentação**

```text theme={null}
update the README with installation instructions
```

**Revisão de código**

```text theme={null}
review my changes and suggest improvements
```

<Tip>
  Fale com Claude como você falaria com um colega prestativo. Descreva o que você quer alcançar, e ele o ajudará a chegar lá.
</Tip>

<h2 id="essential-commands">
  Comandos essenciais
</h2>

Aqui estão os comandos mais importantes para uso diário. Comandos shell são executados a partir do seu terminal para iniciar ou retomar Claude Code. Comandos de sessão são executados dentro do Claude Code após ele iniciar.

**Comandos shell**

| Comando             | O que faz                                          | Exemplo                             |
| ------------------- | -------------------------------------------------- | ----------------------------------- |
| `claude`            | Iniciar modo interativo                            | `claude`                            |
| `claude "task"`     | Executar uma tarefa única                          | `claude "fix the build error"`      |
| `claude -p "query"` | Executar consulta única, depois sair               | `claude -p "explain this function"` |
| `claude -c`         | Continuar conversa mais recente no diretório atual | `claude -c`                         |
| `claude -r`         | Retomar uma conversa anterior                      | `claude -r`                         |

**Comandos de sessão**

| Comando           | O que faz                    | Exemplo  |
| ----------------- | ---------------------------- | -------- |
| `/clear`          | Limpar histórico de conversa | `/clear` |
| `/help`           | Mostrar comandos disponíveis | `/help`  |
| `/exit` ou Ctrl+D | Sair do Claude Code          | `/exit`  |

Veja a [referência CLI](/docs/pt/cli-reference) para a lista completa de comandos shell e a [referência de comandos](/docs/pt/commands) para a lista completa de comandos de sessão.

<h2 id="pro-tips-for-beginners">
  Dicas profissionais para iniciantes
</h2>

Para mais, veja [melhores práticas](/docs/pt/best-practices) e [fluxos de trabalho comuns](/docs/pt/common-workflows).

<AccordionGroup>
  <Accordion title="Seja específico com seus pedidos">
    Em vez de: "corrigir o bug"

    Tente: "corrigir o bug de login onde os usuários veem uma tela em branco após inserir credenciais incorretas"
  </Accordion>

  <Accordion title="Use instruções passo a passo">
    Divida tarefas complexas em etapas:

    ```text theme={null}
    1. criar uma nova tabela de banco de dados para perfis de usuário
    2. criar um endpoint de API para obter e atualizar perfis de usuário
    3. construir uma página da web que permite aos usuários ver e editar suas informações
    ```
  </Accordion>

  <Accordion title="Deixe Claude explorar primeiro">
    Antes de fazer alterações, deixe Claude entender seu código:

    ```text theme={null}
    analisar o esquema do banco de dados
    ```

    ```text theme={null}
    construir um painel mostrando produtos que são devolvidos com mais frequência por nossos clientes do Reino Unido
    ```
  </Accordion>

  <Accordion title="Economize tempo com atalhos">
    * Digite `/` para ver todos os comandos e skills
    * Use Tab para conclusão de comando
    * Pressione ↑ para histórico de comando
    * Pressione `Shift+Tab` para alternar modos de permissão
  </Accordion>
</AccordionGroup>

<h2 id="what’s-next">
  Próximos passos
</h2>

Agora que você aprendeu o básico, explore recursos mais avançados:

<CardGroup cols={2}>
  <Card title="Como Claude Code funciona" icon="microchip" href="/docs/pt/how-claude-code-works">
    Entenda o loop agêntico, ferramentas integradas e como Claude Code interage com seu projeto
  </Card>

  <Card title="Melhores práticas" icon="star" href="/docs/pt/best-practices">
    Obtenha melhores resultados com prompting eficaz e configuração de projeto
  </Card>

  <Card title="Fluxos de trabalho comuns" icon="graduation-cap" href="/docs/pt/common-workflows">
    Guias passo a passo para tarefas comuns
  </Card>

  <Card title="Estenda Claude Code" icon="puzzle-piece" href="/docs/pt/features-overview">
    Personalize com CLAUDE.md, skills, hooks, MCP e muito mais
  </Card>
</CardGroup>

<h2 id="getting-help">
  Obtendo ajuda
</h2>

* **Em Claude Code**: Digite `/help` ou pergunte "how do I..."
* **Documentação**: Você está aqui! Navegue por outros guias
* **Comunidade**: Junte-se ao nosso [Discord](https://www.anthropic.com/discord) para dicas e suporte
