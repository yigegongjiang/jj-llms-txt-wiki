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
* Uma [assinatura Claude](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_prereq) (Pro, Max, Team ou Enterprise), conta do [Claude Console](https://platform.claude.com/), ou acesso através de um [provedor de nuvem suportado](/docs/pt/third-party-integrations)

<Note>
  Este guia cobre o CLI do terminal. Claude Code também está disponível na [web](https://claude.ai/code), como um [aplicativo de desktop](/docs/pt/desktop), em [VS Code](/docs/pt/vs-code) e [IDEs JetBrains](/docs/pt/jetbrains), no [Slack](/docs/pt/slack), e em CI/CD com [GitHub Actions](/docs/pt/github-actions) e [GitLab](/docs/pt/gitlab-ci-cd). Veja [todas as interfaces](/docs/pt/overview#use-claude-code-everywhere).
</Note>

<h2 id="step-1-install-claude-code">
  Passo 1: Instale Claude Code
</h2>

Para instalar Claude Code, use um dos seguintes métodos:

<Tabs>
  <Tab title="Instalação Nativa (Recomendado)">
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

    Se você vir `The token '&&' is not a valid statement separator`, você está no PowerShell, não no CMD. Se você vir `'irm' is not recognized as an internal or external command`, você está no CMD, não no PowerShell. Seu prompt mostra `PS C:\` quando você está no PowerShell e `C:\` sem o `PS` quando você está no CMD.

    Se o comando de instalação falhar com `syntax error near unexpected token '<'`, um `403`, ou outro erro de curl, consulte [Solucionar problemas de instalação](/docs/pt/troubleshoot-install#find-your-error) para corresponder o erro a uma correção e para métodos alternativos de instalação.

    [Git for Windows](https://git-scm.com/downloads/win) é recomendado no Windows nativo para que Claude Code possa usar a ferramenta Bash. Se Git for Windows não estiver instalado, Claude Code usa PowerShell como ferramenta de shell. Configurações WSL não precisam de Git for Windows.

    <Info>
      As instalações nativas são atualizadas automaticamente em segundo plano para mantê-lo na versão mais recente.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew oferece dois casks. `claude-code` rastreia o canal de versão estável, que normalmente fica cerca de uma semana atrás e pula versões com regressões importantes. `claude-code@latest` rastreia o canal mais recente e recebe novas versões assim que são lançadas.

    <Info>
      As instalações do Homebrew não são atualizadas automaticamente. Execute `brew upgrade claude-code` ou `brew upgrade claude-code@latest`, dependendo de qual cask você instalou, para obter os recursos mais recentes e correções de segurança.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      As instalações do WinGet não são atualizadas automaticamente. Execute `winget upgrade Anthropic.ClaudeCode` periodicamente para obter os recursos mais recentes e correções de segurança.
    </Info>
  </Tab>
</Tabs>

Você também pode instalar com [apt, dnf, ou apk](/docs/pt/setup#install-with-linux-package-managers) no Debian, Fedora, RHEL e Alpine.

Para confirmar que a instalação funcionou, execute:

```bash theme={null}
claude --version
```

O comando imprime um número de versão seguido por `(Claude Code)`.

<h2 id="step-2-log-in-to-your-account">
  Passo 2: Faça login em sua conta
</h2>

Claude Code requer uma conta para usar. Inicie uma sessão interativa com o comando `claude` e você será solicitado a fazer login no primeiro uso:

```bash theme={null}
claude
```

Para contas de assinatura Claude ou Console, siga os prompts para concluir a autenticação no seu navegador. Se você tiver definido a variável de ambiente `ANTHROPIC_API_KEY`, Claude Code ignora o prompt de login e pede que você aprove a chave. Para trocar de contas mais tarde ou fazer nova autenticação, digite `/login` dentro da sessão em execução:

```text wrap theme={null}
/login
```

Você pode fazer login usando qualquer um destes tipos de conta:

* [Claude Pro, Max, Team ou Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_login) (recomendado)
* [Claude Console](https://platform.claude.com/) (acesso à API com créditos pré-pagos). No primeiro login, um workspace "Claude Code" é criado automaticamente no Console para rastreamento centralizado de custos.
* [Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry](/docs/pt/third-party-integrations) (provedores de nuvem empresariais)
* Um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) auto-hospedado, se sua organização executar um: seu administrador pré-configura a URL do gateway, e `/login` abre diretamente na tela **Cloud gateway** para você fazer login com SSO corporativo

Depois de fazer login, suas credenciais são armazenadas e você não precisará fazer login novamente. Saiba mais em [Gerenciamento de Credenciais](/docs/pt/authentication#credential-management).

<h2 id="step-3-start-your-first-session">
  Passo 3: Inicie sua primeira sessão
</h2>

Abra seu terminal em qualquer diretório de projeto e inicie Claude Code:

```bash theme={null}
cd /path/to/your/project
claude
```

Substitua `/path/to/your/project` pelo caminho do projeto em que você deseja trabalhar.

Você verá o prompt do Claude Code com a versão, modelo atual e diretório de trabalho mostrados acima. Digite `/help` para comandos disponíveis ou `/resume` para continuar uma conversa anterior.

<h2 id="step-4-ask-your-first-question">
  Passo 4: Faça sua primeira pergunta
</h2>

Vamos começar entendendo sua base de código. Tente um destes comandos:

```text wrap theme={null}
what does this project do?
```

Claude analisará seus arquivos e fornecerá um resumo. Você também pode fazer perguntas mais específicas:

```text wrap theme={null}
what technologies does this project use?
```

```text wrap theme={null}
where is the main entry point?
```

```text wrap theme={null}
explain the folder structure
```

Você também pode perguntar ao Claude sobre suas próprias capacidades:

```text wrap theme={null}
what can Claude Code do?
```

```text wrap theme={null}
how do I create custom skills in Claude Code?
```

```text wrap theme={null}
can Claude Code work with Docker?
```

<Note>
  Claude Code lê seus arquivos de projeto conforme necessário. Você não precisa adicionar contexto manualmente.
</Note>

<h2 id="step-5-make-your-first-code-change">
  Passo 5: Faça sua primeira alteração de código
</h2>

Agora vamos fazer Claude Code fazer alguma codificação real. Tente uma tarefa simples:

```text wrap theme={null}
add a hello world function to the main file
```

Claude Code encontra o arquivo apropriado e mostra a alteração. Se ele pedir antes de fazer a alteração, selecione **Sim** para aprovar.

O modo Auto é o [modo de permissão inicial integrado](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) para sessões de terminal interativas nos planos Pro, Max e Team: um classificador revisa as ações em vez de você, e Claude edita a maioria dos arquivos e executa a maioria dos comandos sem pedir. Em outros planos, o modo Manual é o modo de permissão inicial integrado. Para a sessão que você inicia logo após a instalação, consulte [Primeira sessão após uma instalação ou atualização](/docs/pt/env-vars#first-session-after-an-install-or-upgrade).

<Note>
  Suas configurações ou sua organização podem definir um modo de permissão inicial diferente. [Qual modo de permissão uma sessão inicia](/docs/pt/permission-modes#which-mode-a-session-starts-in) lista o que faz. Pressione `Shift+Tab` a qualquer momento para alternar o modo de permissão da sessão em que você está.
</Note>

<h2 id="step-6-use-git-with-claude-code">
  Passo 6: Use Git com Claude Code
</h2>

Claude Code torna as operações Git conversacionais:

```text wrap theme={null}
what files have I changed?
```

```text wrap theme={null}
commit my changes with a descriptive message
```

Você também pode solicitar operações Git mais complexas:

```text wrap theme={null}
create a new branch called feature/quickstart
```

```text wrap theme={null}
show me the last 5 commits
```

```text wrap theme={null}
help me resolve merge conflicts
```

<h2 id="step-7-fix-a-bug-or-add-a-feature">
  Passo 7: Corrija um bug ou adicione um recurso
</h2>

Claude é proficiente em depuração e implementação de recursos.

Descreva o que você quer em linguagem natural:

```text wrap theme={null}
add input validation to the user registration form
```

Ou corrija problemas existentes:

```text wrap theme={null}
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

```text wrap theme={null}
refactor the authentication module to use async/await instead of callbacks
```

**Escreva testes**

```text wrap theme={null}
write unit tests for the calculator functions
```

**Atualize documentação**

```text wrap theme={null}
update the README with installation instructions
```

**Revisão de código**

```text wrap theme={null}
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
| `claude "task"`     | Iniciar modo interativo com um prompt inicial      | `claude "fix the build error"`      |
| `claude -p "query"` | Executar consulta única, depois sair               | `claude -p "explain this function"` |
| `claude -c`         | Continuar conversa mais recente no diretório atual | `claude -c`                         |
| `claude -r`         | Retomar uma conversa anterior                      | `claude -r`                         |

**Comandos de sessão**

| Comando                      | O que faz                    | Exemplo  |
| ---------------------------- | ---------------------------- | -------- |
| `/clear`                     | Limpar histórico de conversa | `/clear` |
| `/help`                      | Mostrar comandos disponíveis | `/help`  |
| `/exit` ou Ctrl+D duas vezes | Sair do Claude Code          | `/exit`  |

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

    ```text wrap theme={null}
    1. criar uma nova tabela de banco de dados para perfis de usuário
    2. criar um endpoint de API para obter e atualizar perfis de usuário
    3. construir uma página da web que permite aos usuários ver e editar suas informações
    ```
  </Accordion>

  <Accordion title="Deixe Claude explorar primeiro">
    Antes de fazer alterações, deixe Claude entender seu código:

    ```text wrap theme={null}
    analisar o esquema do banco de dados
    ```

    ```text wrap theme={null}
    construir um painel mostrando produtos que são devolvidos com mais frequência por nossos clientes do Reino Unido
    ```
  </Accordion>

  <Accordion title="Economize tempo com atalhos">
    * Digite `/` para ver todos os comandos e skills disponíveis para você
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
* **Cursos**: Faça [Claude Code 101](https://academy.claude.com/courses/claude-code-101) e outros cursos gratuitos no seu próprio ritmo em [Claude Academy](https://academy.claude.com/)
* **Comunidade**: Junte-se ao nosso [Discord](https://www.anthropic.com/discord) para dicas e suporte
