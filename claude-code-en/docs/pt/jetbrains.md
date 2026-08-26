> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# JetBrains IDEs

> Use Claude Code with JetBrains IDEs including IntelliJ, PyCharm, WebStorm, and more

Claude Code integra-se com JetBrains IDEs através de um plugin dedicado, fornecendo recursos como visualização de diff interativa, compartilhamento de contexto de seleção e muito mais.

<h2 id="supported-ides">
  IDEs Suportadas
</h2>

O plugin Claude Code funciona com a maioria dos JetBrains IDEs, incluindo:

* IntelliJ IDEA
* PyCharm
* Android Studio
* WebStorm
* PhpStorm
* GoLand

<h2 id="features">
  Recursos
</h2>

* **Inicialização rápida**: Use `Cmd+Esc` (Mac) ou `Ctrl+Esc` (Windows/Linux) para abrir Claude Code diretamente do seu editor, ou clique no botão Claude Code na interface
* **Visualização de diff**: As alterações de código podem ser exibidas diretamente no visualizador de diff do IDE em vez do terminal
* **Contexto de seleção**: A seleção ou aba atual no IDE é compartilhada automaticamente com Claude Code. As [regras de negação `Read`](/docs/pt/permissions#read-and-edit) bloqueiam esse compartilhamento para arquivos correspondentes
* **Atalhos de referência de arquivo**: Use `Cmd+Option+K` (Mac) ou `Alt+Ctrl+K` (Linux/Windows) para inserir referências de arquivo como `@src/auth.ts#L1-99`
* **Compartilhamento de diagnóstico**: Erros de diagnóstico do IDE, como erros de lint e sintaxe, são compartilhados automaticamente com Claude conforme você trabalha

<h2 id="installation">
  Instalação
</h2>

O plugin executa o comando `claude` no terminal integrado do seu IDE e se conecta a ele. Ele não agrupa sua própria cópia da CLI, portanto instale ambas as partes:

<Steps>
  <Step title="Instale o Claude Code CLI">
    Siga o [guia de início rápido](/docs/pt/quickstart) para instalar a CLI se você ainda não o fez. O plugin mostra uma notificação "Cannot launch Claude Code" quando `claude` não está no seu PATH.
  </Step>

  <Step title="Instale o plugin JetBrains">
    Instale o [plugin Claude Code](https://plugins.jetbrains.com/plugin/27310-claude-code-beta-) do JetBrains Marketplace e reinicie seu IDE.
  </Step>
</Steps>

Se `claude` estiver instalado em algum lugar que seu IDE não consiga encontrar, defina o caminho completo na [configuração do comando Claude](#general-settings) do plugin.

Claude Code funciona com qualquer assinatura Claude paga (Pro, Max, Team ou Enterprise) ou uma conta Claude Console, e nenhuma chave de API é necessária. Você será solicitado a [fazer login](/docs/pt/authentication#log-in-to-claude-code) na primeira vez que executar `claude`.

<Note>
  Após instalar o plugin, você pode precisar reiniciar completamente seu IDE para que ele entre em vigor.
</Note>

<h2 id="usage">
  Uso
</h2>

<h3 id="from-your-ide">
  Do Seu IDE
</h3>

Execute `claude` do terminal integrado do seu IDE, e todos os recursos de integração estarão ativos.

<h3 id="from-external-terminals">
  De Terminais Externos
</h3>

Use o comando `/ide` em qualquer terminal externo para conectar Claude Code ao seu JetBrains IDE e ativar todos os recursos:

```bash theme={null}
claude
```

```text theme={null}
/ide
```

Se você deseja que Claude tenha acesso aos mesmos arquivos do seu IDE, inicie Claude Code no mesmo diretório que a raiz do projeto do seu IDE.

<h2 id="configuration">
  Configuração
</h2>

<h3 id="claude-code-settings">
  Configurações do Claude Code
</h3>

Configure a integração do IDE através das configurações do Claude Code:

1. Execute `claude`
2. Digite o comando `/config`
3. Defina a ferramenta de diff como `auto` para mostrar diffs no IDE, ou `terminal` para mantê-los no terminal

<h3 id="plugin-settings">
  Configurações do Plugin
</h3>

Configure o plugin Claude Code acessando **Settings → Tools → Claude Code \[Beta]**:

<h4 id="general-settings">
  Configurações Gerais
</h4>

* **Claude command**: Especifique um comando personalizado para executar Claude, por exemplo `claude`, `/usr/local/bin/claude`, ou `npx @anthropic-ai/claude-code`
* **Suppress notification for Claude command not found**: Pule notificações sobre não encontrar o comando Claude
* **Enable using Option+Enter for multi-line prompts**: apenas no macOS. Quando ativado, Option+Enter insere novas linhas em prompts do Claude Code. Desative se a tecla Option estiver sendo capturada inesperadamente. Requer reinicialização do terminal.
* **Enable automatic updates**: Verifique e instale automaticamente atualizações do plugin, aplicadas na reinicialização

<Tip>
  Para usuários WSL: Defina `wsl -d Ubuntu -- bash -lic "claude"` como seu comando Claude (substitua `Ubuntu` pelo nome da sua distribuição WSL)
</Tip>

<h4 id="esc-key-configuration">
  Configuração da Tecla ESC
</h4>

Se a tecla ESC não interromper as operações do Claude Code nos terminais JetBrains:

1. Vá para **Settings → Tools → Terminal**
2. Faça um dos seguintes:
   * Desmarque "Move focus to the editor with Escape", ou
   * Clique em "Configure terminal keybindings" e delete o atalho "Switch focus to Editor"
3. Aplique as alterações

Isso permite que a tecla ESC interrompa adequadamente as operações do Claude Code.

<h2 id="special-configurations">
  Configurações Especiais
</h2>

<h3 id="remote-development">
  Desenvolvimento Remoto
</h3>

<Warning>
  Ao usar JetBrains Remote Development, você deve instalar o plugin no host remoto via **Settings → Plugin (Host)**.
</Warning>

O plugin deve ser instalado no host remoto, não na sua máquina cliente local.

<h3 id="wsl-configuration">
  Configuração WSL
</h3>

Se você estiver usando Claude Code no WSL2 com um JetBrains IDE e vir "No available IDEs detected", a causa geralmente é a rede NAT do WSL2 ou o Windows Firewall bloqueando a conexão entre WSL2 e o IDE em execução no host Windows. WSL1 usa a rede do host diretamente e não é afetado.

<h4 id="allow-wsl2-traffic-through-windows-firewall">
  Permitir tráfego WSL2 através do Windows Firewall
</h4>

Esta é a correção recomendada porque mantém seu modo de rede WSL2 existente.

<Steps>
  <Step title="Encontre seu endereço IP do WSL2">
    De dentro do seu shell WSL, execute:

    ```bash theme={null}
    hostname -I
    ```

    Anote a sub-rede, por exemplo `172.21.123.45` está em `172.21.0.0/16`.
  </Step>

  <Step title="Crie uma regra de firewall">
    Abra PowerShell como Administrador e execute o seguinte, ajustando o intervalo de IP para corresponder à sua sub-rede:

    ```powershell theme={null}
    New-NetFirewallRule -DisplayName "Allow WSL2 Internal Traffic" -Direction Inbound -Protocol TCP -Action Allow -RemoteAddress 172.21.0.0/16 -LocalAddress 172.21.0.0/16
    ```
  </Step>

  <Step title="Reinicie seu IDE e Claude Code">
    Feche e reabra ambos para que a nova regra entre em vigor.
  </Step>
</Steps>

<h4 id="switch-wsl2-to-mirrored-networking">
  Mude WSL2 para rede espelhada
</h4>

A rede espelhada requer Windows 11 22H2 ou posterior. Se você estiver no Windows 10, use a regra de firewall acima.

Adicione isto ao `.wslconfig` no seu diretório de usuário Windows:

```ini theme={null}
[wsl2]
networkingMode=mirrored
```

Em seguida, reinicie WSL com `wsl --shutdown` do PowerShell.

<h2 id="troubleshooting">
  Solução de Problemas
</h2>

<h3 id="plugin-not-working">
  Plugin não funcionando
</h3>

Se o plugin estiver instalado mas os recursos do Claude Code não aparecerem no seu IDE:

* Certifique-se de que você está executando Claude Code no diretório raiz do projeto
* Verifique se o plugin JetBrains está ativado nas configurações do IDE
* Reinicie completamente o IDE (você pode precisar fazer isso várias vezes)
* Para Remote Development, certifique-se de que o plugin está instalado no host remoto

<h3 id="ide-not-detected">
  IDE não detectado
</h3>

Se executar `claude` mostrar "No available IDEs detected":

* Verifique se o plugin está instalado e ativado
* Reinicie o IDE completamente
* Verifique se você está executando Claude Code no terminal integrado
* Para usuários WSL, consulte [Configuração WSL](#wsl-configuration) acima

<h3 id="command-not-found">
  Comando não encontrado
</h3>

Se clicar no ícone Claude mostrar "command not found":

1. Verifique se Claude Code está instalado executando `claude --version` em um terminal
2. Configure o caminho do comando Claude nas configurações do plugin
3. Para usuários WSL, use o formato de comando WSL mencionado na seção de configuração

<h2 id="security-considerations">
  Considerações de Segurança
</h2>

Quando Claude Code é executado em um JetBrains IDE no modo de permissão [`acceptEdits`](/docs/pt/permission-modes#auto-approve-file-edits-with-acceptedits-mode), ele pode ser capaz de modificar arquivos de configuração do IDE que podem ser executados automaticamente pelo seu IDE. Isso pode aumentar o risco de executar Claude Code no modo `acceptEdits` e permitir contornar os prompts de permissão do Claude Code para execução de bash.

Ao executar em JetBrains IDEs, considere:

* Usar modo de aprovação manual para edições
* Tomar cuidado extra para garantir que Claude seja usado apenas com prompts confiáveis
* Estar ciente de quais arquivos Claude Code tem acesso para modificar

Para problemas de instalação ou login do Claude Code fora do IDE, consulte [Solucionar problemas de instalação e login](/docs/pt/troubleshoot-install).

<h3 id="the-built-in-ide-mcp-server">
  O servidor MCP IDE integrado
</h3>

Quando o plugin está ativo, ele executa um servidor MCP local ao qual a CLI se conecta automaticamente. É assim que a CLI abre diffs no visualizador de diff nativo do IDE, lê sua seleção atual para menções `@` e extrai diagnósticos de inspeção para a conversa.

O servidor é nomeado `ide` e está oculto de `/mcp` porque não há nada para configurar. Se sua organização usa um [hook `PreToolUse`](/docs/pt/hooks#pretooluse) para criar uma lista de permissões de ferramentas MCP, porém, você precisará saber que ele existe.

**Contexto de seleção e arquivo aberto.** Enquanto conectado, a CLI inclui sua seleção atual do editor e o caminho do arquivo ativo como contexto em cada prompt que você envia. A transcrição mostra uma linha `⧉ Selected N lines from <file>` quando isso acontece. Para excluir um arquivo sensível como `.env`, adicione uma [regra de negação `Read`](/docs/pt/permissions#read-and-edit) para seu caminho. Uma regra de negação correspondente impede que o texto selecionado e o aviso de arquivo aberto para esse arquivo cheguem ao Claude.

**Transporte e autenticação.** O servidor escuta em uma porta efêmera atribuída pelo SO, e a porta não é configurável. O transporte é `ws://` não criptografado; em loopback, qualquer processo que pudesse capturar o tráfego também pode ler o token do arquivo de bloqueio, portanto TLS não adicionaria proteção contra um atacante local. Cada início do IDE gera um token de autenticação aleatório novo, o escreve em um arquivo de bloqueio em `~/.claude/ide/<port>.lock`, e a CLI deve apresentá-lo como o header `X-Claude-Code-Ide-Authorization` para se conectar. Se `CLAUDE_CONFIG_DIR` estiver definido, o arquivo de bloqueio será escrito em `$CLAUDE_CONFIG_DIR/ide/` em vez disso.

**Ferramentas expostas ao modelo.** O servidor hospeda várias ferramentas, mas apenas uma é visível ao modelo. O resto é RPC interno que a CLI usa para sua própria UI, como abrir diffs e ler seleções, e são filtrados antes da lista de ferramentas chegar ao Claude.

| Nome da ferramenta (conforme visto por hooks) | O que faz                                                                                                               | Somente leitura |
| --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- | --------------- |
| `mcp__ide__getDiagnostics`                    | Retorna os diagnósticos de inspeção do IDE, os erros e avisos mostrados no editor. Opcionalmente limitado a um arquivo. | Sim             |

O plugin JetBrains não expõe uma ferramenta de execução de código ao modelo.

**Interface de escuta.** Qual interface de rede o servidor se vincula é controlado por **Accept connections from all network interfaces** em **Settings → Tools → Claude Code \[Beta] → Networking (Advanced)**. Com a configuração desabilitada, o servidor escuta apenas em `127.0.0.1` e não é acessível de outros hosts. Com ela habilitada, a porta é acessível de sua rede local. A configuração existe para casos em que a CLI não consegue alcançar o IDE sobre loopback, como WSL2 com NAT networking padrão ou uma configuração de IDE remoto; consulte [Configuração WSL](#wsl-configuration) para esse cenário.

<Warning>
  Habilitar **Accept connections from all network interfaces** torna a porta MCP do IDE acessível de sua rede local. As conexões ainda exigem o token de autenticação do arquivo de bloqueio, mas como o transporte é `ws://` não criptografado, tanto o tráfego da sessão quanto esse token atravessam a rede em texto simples quando a configuração está ativada. Ative-a apenas quando loopback genuinamente não conseguir funcionar. Para WSL2, prefira [networking espelhado](#switch-wsl2-to-mirrored-networking) para que a interface loopback do Windows seja compartilhada com a VM Linux e o socket possa permanecer em loopback.
</Warning>
