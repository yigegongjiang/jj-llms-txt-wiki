> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Solucionar problemas de instalação e login

> Corrija erros de comando não encontrado, PATH, permissão, rede e autenticação ao instalar ou fazer login no Claude Code.

Se a instalação falhar ou você não conseguir fazer login, encontre seu erro abaixo. Para problemas de tempo de execução após o Claude Code estar funcionando, consulte [Troubleshooting](/docs/pt/troubleshooting). Para problemas de configuração, como configurações não sendo aplicadas ou hooks não disparando, consulte [Debug your configuration](/docs/pt/debug-your-config).

<h2 id="find-your-error">
  Encontre seu erro
</h2>

Corresponda a mensagem de erro ou sintoma que você está vendo a uma solução:

| O que você vê                                                                                               | Solução                                                                                                                                         |
| :---------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `command not found: claude` ou `'claude' is not recognized`                                                 | [Corrija seu PATH](#command-not-found-claude-after-installation)                                                                                |
| `syntax error near unexpected token '<'`                                                                    | [O script de instalação retorna HTML](#install-script-returns-html-instead-of-a-shell-script)                                                   |
| `curl: (22) The requested URL returned error: 403`                                                          | [O script de instalação retornou 403](#install-script-returns-html-instead-of-a-shell-script)                                                   |
| `curl: (23)` ou `curl: (56) Failure writing output to destination`                                          | [Verifique a conectividade ou use um instalador alternativo](#curl-56-failure-writing-output-to-destination)                                    |
| `Killed` durante a instalação no Linux, ou `Installation was killed before it could finish (exit code 137)` | [Libere memória ou adicione espaço de troca](#install-killed-on-low-memory-linux-servers)                                                       |
| `TLS connect error` ou `SSL/TLS secure channel`                                                             | [Atualize os certificados CA](#tls-or-ssl-connection-errors)                                                                                    |
| `Failed to fetch version` ou não consegue alcançar o servidor de download                                   | [Verifique as configurações de rede e proxy](#check-network-connectivity)                                                                       |
| `irm is not recognized` ou `&& is not valid`                                                                | [Use o comando correto para seu shell](#wrong-install-command-on-windows)                                                                       |
| `Cask 'claude-code' is unavailable: No Cask with this name exists`                                          | [Atualize o Homebrew](#homebrew-cask-unavailable-or-outdated)                                                                                   |
| `'bash' is not recognized as the name of a cmdlet`                                                          | [Use o comando do instalador do Windows](#wrong-install-command-on-windows)                                                                     |
| `A parameter cannot be found that matches parameter name 'fsSL'`                                            | [Use o comando do instalador do Windows](#wrong-install-command-on-windows)                                                                     |
| `Claude Code on Windows requires either Git for Windows (for bash) or PowerShell`                           | [Instale um shell](#claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell)                                              |
| `Claude Code does not support 32-bit Windows`                                                               | [Abra o Windows PowerShell, não a entrada x86](#claude-code-does-not-support-32-bit-windows)                                                    |
| `The process cannot access the file ... because it is being used by another process`                        | [Limpe a pasta de downloads e tente novamente](#the-process-cannot-access-the-file-during-windows-install)                                      |
| `Error loading shared library`                                                                              | [Variante binária incorreta para seu sistema](#linux-musl-or-glibc-binary-mismatch)                                                             |
| `Illegal instruction`                                                                                       | [Incompatibilidade de arquitetura ou conjunto de instruções da CPU](#illegal-instruction)                                                       |
| `cannot execute binary file: Exec format error` em WSL                                                      | [Regressão de binário nativo WSL1](#exec-format-error-on-wsl1)                                                                                  |
| O instalador PowerShell é concluído mas `claude` não é encontrado ou mostra uma versão antiga               | [Adicione o diretório de instalação ao seu PATH](#verify-your-path), depois abra um novo terminal                                               |
| `dyld: cannot load`, `dyld: Symbol not found`, ou `Abort trap` no macOS                                     | [Incompatibilidade binária](#dyld-cannot-load-on-macos)                                                                                         |
| `Invoke-Expression: Missing argument in parameter list`                                                     | [O script de instalação retorna HTML](#install-script-returns-html-instead-of-a-shell-script)                                                   |
| `App unavailable in region`                                                                                 | Claude Code não está disponível em seu país. Consulte [países suportados](https://www.anthropic.com/supported-countries).                       |
| `unable to get local issuer certificate`                                                                    | [Configure certificados CA corporativos](#tls-or-ssl-connection-errors)                                                                         |
| `OAuth error` ou `403 Forbidden`                                                                            | [Corrija a autenticação](#login-and-authentication)                                                                                             |
| `Could not load the default credentials` ou `Could not load credentials from any providers`                 | [Credenciais do Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `ChainedTokenCredential authentication failed` ou `CredentialUnavailableError`                              | [Credenciais do Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `API Error: 500`, `529 Overloaded`, `429`, ou outros erros 4xx e 5xx não listados acima                     | Consulte a [referência de erros](/docs/pt/errors)                                                                                                    |

Se seu problema não estiver listado, trabalhe através das verificações de diagnóstico abaixo para estreitar a causa.

<Tip>
  Se você preferir pular o terminal completamente, o [Claude Code Desktop app](/docs/pt/desktop-quickstart) permite que você instale e use Claude Code através de uma interface gráfica. Baixe-o para [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs) ou [Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs) e comece a codificar sem nenhuma configuração de linha de comando. No Linux, instale o aplicativo com apt seguindo as [instruções de instalação do Linux](/docs/pt/desktop-linux).
</Tip>

<h2 id="run-diagnostic-checks">
  Execute verificações de diagnóstico
</h2>

<h3 id="check-network-connectivity">
  Verifique a conectividade de rede
</h3>

O instalador baixa de `downloads.claude.ai`. Verifique se você consegue alcançá-lo:

```bash theme={null}
curl -sI https://downloads.claude.ai/claude-code-releases/latest
```

No PowerShell, execute `curl.exe -sI` em vez disso. O PowerShell cria um alias de `curl` para `Invoke-WebRequest`, que rejeita os sinalizadores `-sI`.

Uma linha `HTTP/2 200` significa que você alcançou o servidor. Se você vir nenhuma saída, `Could not resolve host`, ou um tempo limite de conexão, sua rede está bloqueando a conexão. Causas comuns:

* Firewalls corporativos ou proxies bloqueando `downloads.claude.ai`
* Restrições de rede regional: tente uma VPN ou rede alternativa
* Problemas de TLS/SSL: atualize os certificados CA do seu sistema, ou verifique se `HTTPS_PROXY` está configurado

Se você estiver atrás de um proxy corporativo, defina `HTTPS_PROXY` e `HTTP_PROXY` para o endereço do seu proxy antes de instalar. Peça à sua equipe de TI pela URL do proxy se você não souber, ou verifique as configurações de proxy do seu navegador.

Este exemplo define ambas as variáveis de proxy e executa o instalador através do seu proxy:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    export HTTP_PROXY=http://proxy.example.com:8080
    export HTTPS_PROXY=http://proxy.example.com:8080
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:HTTP_PROXY = 'http://proxy.example.com:8080'
    $env:HTTPS_PROXY = 'http://proxy.example.com:8080'
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>
</Tabs>

<h3 id="verify-your-path">
  Verifique seu PATH
</h3>

Se a instalação foi bem-sucedida mas você recebe um erro `command not found` ou `not recognized` ao executar `claude`, o diretório de instalação não está em seu PATH. Seu shell procura por programas em diretórios listados em PATH, e o instalador coloca `claude` em `~/.local/bin/claude` no macOS/Linux ou `%USERPROFILE%\.local\bin\claude.exe` no Windows.

<Note>
  A [extensão VS Code](/docs/pt/vs-code) não coloca `claude` neste local. Ela agrupa uma cópia privada da CLI dentro do diretório da extensão para seu próprio painel de chat e não a adiciona ao PATH. Se você tiver instalado apenas a extensão, `~/.local/bin/claude` não existirá. Execute a [instalação autônoma](/docs/pt/setup) para usar `claude` a partir de um terminal, depois continue abaixo.
</Note>

Verifique se o diretório de instalação está em seu PATH listando suas entradas de PATH e filtrando por `local/bin`:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    echo $PATH | tr ':' '\n' | grep -Fx "$HOME/.local/bin"
    ```

    Se isso imprimir `/Users/you/.local/bin` ou `/home/you/.local/bin`, o diretório está em seu PATH e você pode pular para [Verifique se há instalações conflitantes](#check-for-conflicting-installations). Se não houver saída, adicione-o à sua configuração de shell.

    Para Zsh, o padrão no macOS:

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
    source ~/.zshrc
    ```

    Para Bash, o padrão na maioria das distribuições Linux:

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    source ~/.bashrc
    ```

    Alternativamente, feche e reabra seu terminal.

    Para outros shells como fish ou Nushell, adicione `~/.local/bin` ao seu PATH usando a sintaxe de configuração do seu próprio shell, depois reinicie seu terminal.

    Verifique se a correção funcionou:

    ```bash theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:PATH -split ';' | Select-String '\.local\\bin'
    ```

    Se não houver saída, adicione o diretório de instalação ao seu User PATH:

    ```powershell theme={null}
    $currentPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
    [Environment]::SetEnvironmentVariable('PATH', "$currentPath;$env:USERPROFILE\.local\bin", 'User')
    ```

    Reinicie seu terminal para que a alteração tenha efeito.

    Verifique se a correção funcionou:

    ```powershell theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    echo %PATH% | findstr /i "local\bin"
    ```

    Se não houver saída, abra Configurações do Sistema, vá para Variáveis de Ambiente e adicione `%USERPROFILE%\.local\bin` à sua variável User PATH. Reinicie seu terminal.

    Verifique se a correção funcionou:

    ```batch theme={null}
    claude --version
    ```
  </Tab>
</Tabs>

<h3 id="check-for-conflicting-installations">
  Verifique se há instalações conflitantes
</h3>

Múltiplas instalações do Claude Code podem causar incompatibilidades de versão ou comportamento inesperado. Verifique o que está instalado:

<Tabs>
  <Tab title="macOS/Linux">
    Liste todos os binários `claude` encontrados em seu PATH:

    ```bash theme={null}
    which -a claude
    ```

    Se isso não imprimir nada, nenhum `claude` está em seu PATH ainda. Volte para [Verifique seu PATH](#verify-your-path).

    Verifique os três locais de onde um binário `claude` pode vir. `~/.local/bin/claude` é o instalador nativo, `~/.claude/local/` é uma instalação npm local legada criada por versões antigas do Claude Code, e a lista npm global mostra uma instalação `-g`:

    ```bash theme={null}
    ls -la ~/.local/bin/claude
    ```

    Uma instalação nativa mostra um symlink em `~/.local/share/claude/versions/`. Um script ou um symlink que você criou por conta própria neste caminho é um inicializador personalizado, que [auto-update deixa no lugar](/docs/pt/setup#auto-updates).

    Se algum comando `ls` imprimir `No such file or directory`, isso não é um erro. Significa que nada está instalado naquele local, então passe para a próxima verificação.

    ```bash theme={null}
    ls -la ~/.claude/local/
    ```

    ```bash theme={null}
    npm -g ls @anthropic-ai/claude-code 2>/dev/null
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    Liste todos os binários `claude` encontrados em seu PATH:

    ```powershell theme={null}
    where.exe claude
    ```

    Verifique se o instalador nativo colocou um binário:

    ```powershell theme={null}
    Test-Path "$env:USERPROFILE\.local\bin\claude.exe"
    ```
  </Tab>
</Tabs>

Se você encontrar múltiplas instalações, mantenha apenas uma. A instalação nativa em `~/.local/bin/claude` no macOS/Linux ou `%USERPROFILE%\.local\bin\claude.exe` no Windows é recomendada. Remova as extras:

Desinstale uma instalação npm global:

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

Remova a instalação npm local legada:

```bash theme={null}
rm -rf ~/.claude/local
```

No Windows, use PowerShell:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\local"
```

Remova uma instalação Homebrew no macOS. Se você instalou o cask `claude-code@latest`, substitua esse nome:

```bash theme={null}
brew uninstall --cask claude-code
```

Remova uma instalação WinGet no Windows:

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="check-directory-permissions">
  Verifique permissões de diretório
</h3>

O instalador precisa de acesso de escrita a `~/.local/bin/` e `~/.claude/` no macOS e Linux. No Windows, o local de instalação está sob `%USERPROFILE%`, que é gravável pelo seu usuário por padrão, então esta seção raramente se aplica lá.

Verifique se os diretórios são graváveis:

```bash theme={null}
test -w ~/.local/bin && echo "writable" || echo "not writable"
test -w ~/.claude && echo "writable" || echo "not writable"
```

Se algum diretório não for gravável, crie o diretório de instalação e defina seu usuário como proprietário:

```bash theme={null}
sudo mkdir -p ~/.local/bin
sudo chown -R $(whoami) ~/.local
```

<h3 id="verify-the-binary-works">
  Verifique se o binário funciona
</h3>

Se `claude --version` imprime uma versão mas `claude` falha ou trava na inicialização, execute estas verificações para estreitar a causa. Se `claude --version` disser comando não encontrado, vá para [Verifique seu PATH](#verify-your-path) primeiro; os comandos abaixo assumem que `claude` está em seu PATH.

Confirme que o binário existe e é executável:

```bash theme={null}
ls -la "$(command -v claude)"
```

No Windows, use PowerShell:

```powershell theme={null}
Get-Command claude | Select-Object Source
```

No Linux, verifique se há bibliotecas compartilhadas ausentes. Se `ldd` mostrar bibliotecas ausentes, você pode precisar instalar pacotes do sistema. No Alpine Linux e outras distribuições baseadas em musl, consulte [Alpine Linux setup](/docs/pt/setup#alpine-linux-and-musl-based-distributions).

```bash theme={null}
ldd "$(command -v claude)" | grep "not found"
```

Confirme que o binário pode executar:

```bash theme={null}
claude --version
```

<h2 id="common-installation-issues">
  Problemas comuns de instalação
</h2>

Estes são os problemas de instalação mais frequentemente encontrados e suas soluções.

<h3 id="install-script-returns-html-instead-of-a-shell-script">
  Install script returns HTML instead of a shell script
</h3>

Ao executar o comando de instalação, você pode ver um destes erros:

```text theme={null}
bash: line 1: syntax error near unexpected token `<'
bash: line 1: `<!DOCTYPE html>'
```

No PowerShell, o mesmo problema aparece como:

```text theme={null}
Invoke-Expression: Missing argument in parameter list.
```

Dependendo de como a solicitação foi roteada, você pode ver um 403 sem corpo HTML:

```text theme={null}
curl: (22) The requested URL returned error: 403
```

Todos esses significam que a URL de instalação retornou uma página HTML ou um status de erro em vez do script de instalação. Se a página HTML disser "App unavailable in region," Claude Code não está disponível em seu país. Consulte [supported countries](https://www.anthropic.com/supported-countries).

Um 403 simples sem corpo frequentemente tem a mesma causa, mas também pode vir de um proxy corporativo ou firewall bloqueando o download. Se você estiver em um país suportado e ainda vir o 403, trabalhe através de [Check network connectivity](#check-network-connectivity) antes de tentar os instaladores alternativos abaixo, já que esses alcançam os mesmos hosts.

Caso contrário, isso pode acontecer devido a problemas de rede, roteamento regional ou uma interrupção temporária do serviço.

**Soluções:**

1. **Use um método de instalação alternativo**:

   No macOS, instale via Homebrew:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   No Windows, instale via WinGet:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

2. **Tente novamente após alguns minutos**: o problema é frequentemente temporário. Aguarde e tente o comando original novamente.

<h3 id="command-not-found-claude-after-installation">
  `command not found: claude` after installation
</h3>

A instalação foi concluída mas `claude` não funciona. O erro exato varia por plataforma:

| Plataforma  | Mensagem de erro                                                       |
| :---------- | :--------------------------------------------------------------------- |
| macOS       | `zsh: command not found: claude`                                       |
| Linux       | `bash: claude: command not found`                                      |
| Windows CMD | `'claude' is not recognized as an internal or external command`        |
| PowerShell  | `claude : The term 'claude' is not recognized as the name of a cmdlet` |

Isso significa que o diretório de instalação não está no caminho de pesquisa do seu shell. Consulte [Verify your PATH](#verify-your-path) para a correção em cada plataforma.

<h3 id="curl-56-failure-writing-output-to-destination">
  `curl: (56) Failure writing output to destination`
</h3>

O comando `curl ... | bash` baixa o script e o encanua para Bash para execução. Este erro, e o relacionado `curl: (23) Failure writing output to destination`, significa que Bash não recebeu o script completo. O código de saída 56 indica que o download em si foi interrompido, e o código de saída 23 indica que curl não conseguiu escrever o que recebeu para o pipe, geralmente porque Bash saiu cedo.

**Soluções:**

1. **Verifique a estabilidade da rede**: Os binários do Claude Code são hospedados em `downloads.claude.ai`. Teste se você consegue alcançá-lo:
   ```bash theme={null}
   curl -sI https://downloads.claude.ai/claude-code-releases/latest
   ```
   Uma linha `HTTP/2 200` significa que você alcançou o servidor e a falha original foi provavelmente intermitente; tente novamente o comando de instalação. Se você vir `Could not resolve host` ou um tempo limite de conexão, sua rede está bloqueando o download.

2. **Tente um método de instalação alternativo**:

   No macOS:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   No Windows:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

<h3 id="homebrew-cask-unavailable-or-outdated">
  Homebrew cask unavailable or outdated
</h3>

Homebrew relata `Error: Cask 'claude-code' is unavailable: No Cask with this name exists` quando sua cópia local do índice de cask do Homebrew é anterior à publicação do cask. Atualize o índice e tente novamente:

```bash theme={null}
brew update
brew install --cask claude-code
```

Se Homebrew instalar uma versão mais antiga do Claude Code do que você espera, o mesmo índice desatualizado é geralmente a causa. O cask `claude-code` rastreia o canal estável e é tipicamente cerca de uma semana atrás da versão mais recente; para a versão mais recente execute `brew install --cask claude-code@latest` em vez disso. Consulte [Configure release channel](/docs/pt/setup#configure-release-channel) para a diferença entre os dois casks.

<h3 id="tls-or-ssl-connection-errors">
  TLS or SSL connection errors
</h3>

Erros como `curl: (35) TLS connect error`, `schannel: next InitializeSecurityContext failed`, ou `Could not establish trust relationship for the SSL/TLS secure channel` do PowerShell indicam falhas de handshake TLS.

**Soluções:**

1. **Atualize seus certificados CA do sistema**:

   No Ubuntu/Debian:

   ```bash theme={null}
   sudo apt-get update && sudo apt-get install ca-certificates
   ```

   No macOS, o curl do sistema usa o armazenamento de confiança do Keychain; atualizar o macOS em si atualiza os certificados raiz.

2. **No Windows, ative TLS 1.2** no PowerShell antes de executar o instalador:
   ```powershell theme={null}
   [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
   irm https://claude.ai/install.ps1 | iex
   ```

3. **Verifique se há interferência de proxy ou firewall**: proxies corporativos que realizam inspeção TLS podem causar esses erros, incluindo `unable to get local issuer certificate` e `SELF_SIGNED_CERT_IN_CHAIN`. Para a etapa de instalação, aponte curl para seu pacote CA corporativo com `--cacert`:
   ```bash theme={null}
   curl --cacert /path/to/corporate-ca.pem -fsSL https://claude.ai/install.sh | bash
   ```
   Para o Claude Code em si uma vez instalado, defina `NODE_EXTRA_CA_CERTS` para que as solicitações de API confiem no mesmo pacote:
   ```bash theme={null}
   export NODE_EXTRA_CA_CERTS=/path/to/corporate-ca.pem
   ```
   Peça à sua equipe de TI pelo arquivo de certificado se você não tiver. Você também pode tentar em uma conexão direta para confirmar que o proxy é a causa.

4. **No Windows, mude de instaladores se sua rede bloquear verificações de revogação**. Os erros `CRYPT_E_NO_REVOCATION_CHECK (0x80092012)` e `CRYPT_E_REVOCATION_OFFLINE (0x80092013)` significam que curl alcançou o servidor mas sua rede bloqueia a pesquisa de revogação de certificado, o que é comum atrás de firewalls corporativos. Adicionar o sinalizador `--ssl-revoke-best-effort` do curl não corrige isso: o sinalizador se aplica apenas ao download do `install.cmd` em si, e os downloads do próprio script são executados sem ele, então a instalação falha com o mesmo erro. Use um método de instalação que tolere a pesquisa bloqueada em vez disso. Abra PowerShell e execute o instalador PowerShell, que baixa através do .NET e não falha quando o servidor de revogação está inacessível:
   ```powershell theme={null}
   irm https://claude.ai/install.ps1 | iex
   ```
   Você também pode instalar com `winget install Anthropic.ClaudeCode`, que evita curl completamente.

<h3 id="failed-to-fetch-version-from-downloads-claude-ai">
  `Failed to fetch version from downloads.claude.ai`
</h3>

O instalador não conseguiu alcançar o servidor de download. Isso normalmente significa que `downloads.claude.ai` está bloqueado em sua rede.

**Soluções:**

1. **Teste a conectividade diretamente**:
   ```bash theme={null}
   curl -sI https://downloads.claude.ai/claude-code-releases/latest
   ```

2. **Se atrás de um proxy**, defina `HTTPS_PROXY` para que o instalador possa rotear através dele. Consulte [proxy configuration](/docs/pt/network-config#proxy-configuration) para detalhes.
   ```bash theme={null}
   export HTTPS_PROXY=http://proxy.example.com:8080
   curl -fsSL https://claude.ai/install.sh | bash
   ```

3. **Se em uma rede restrita**, tente uma rede diferente ou VPN, ou use um método de instalação alternativo:

   No macOS:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   No Windows:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

<h3 id="wrong-install-command-on-windows">
  Wrong install command on Windows
</h3>

Se você vir `'irm' is not recognized`, `The token '&&' is not valid`, `A parameter cannot be found that matches parameter name 'fsSL'`, ou `'bash' is not recognized as the name of a cmdlet`, você copiou o comando de instalação para um shell ou sistema operacional diferente.

* **`irm` não reconhecido**: você está em CMD, não PowerShell. Você tem duas opções:

  Abra PowerShell procurando por "PowerShell" no menu Iniciar e execute o comando de instalação original:

  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

  Ou fique em CMD e use o instalador CMD em vez disso:

  ```batch theme={null}
  curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
  ```

* **`&&` não válido**: você está em PowerShell mas executou o comando do instalador CMD. Use o instalador PowerShell:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`A parameter cannot be found that matches parameter name 'fsSL'`**: você executou o instalador macOS/Linux `curl -fsSL ... | bash` no Windows PowerShell, onde `curl` é um alias para `Invoke-WebRequest` e rejeita os sinalizadores `-fsSL`. Use o instalador PowerShell em vez disso:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`bash` não reconhecido**: você executou o instalador macOS/Linux no Windows. Use o instalador PowerShell em vez disso:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

<h3 id="the-process-cannot-access-the-file-during-windows-install">
  `The process cannot access the file` during Windows install
</h3>

Se o instalador PowerShell falhar com `Failed to download binary: The process cannot access the file ... because it is being used by another process`, o instalador não conseguiu escrever em `%USERPROFILE%\.claude\downloads`. Isso geralmente significa que uma tentativa de instalação anterior ainda está em execução, ou o software antivírus está verificando um binário parcialmente baixado nessa pasta.

Feche qualquer outra janela do PowerShell executando o instalador e aguarde as verificações de antivírus liberarem o arquivo. Depois delete a pasta de downloads e execute o instalador novamente:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\downloads"
irm https://claude.ai/install.ps1 | iex
```

<h3 id="install-killed-on-low-memory-linux-servers">
  Install killed on low-memory Linux servers
</h3>

Uma mensagem `Killed` durante a instalação geralmente significa que o assassino de falta de memória (OOM) do Linux encerrou a etapa `claude install` porque o sistema ficou sem memória livre. Isso é comum em VPS pequenos e instâncias em nuvem. O script de instalação relata a causa e sai com código 137:

```text theme={null}
Setting up Claude Code...
bash: line 142: 34803 Killed    "$binary_path" install ${TARGET:+"$TARGET"}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Antes da v2.1.200, o script saía apenas com a linha `Killed` simples do shell e sem explicação.

A instalação precisa de aproximadamente 512 MB de memória livre, e executar Claude Code precisa de mais. Consulte os [system requirements](/docs/pt/setup#system-requirements).

**Soluções:**

1. **Adicione espaço de swap** se seu servidor tiver RAM limitada. Swap usa espaço em disco como memória de overflow, permitindo que a instalação seja concluída mesmo com RAM física baixa.

   Crie um arquivo de swap de 2 GB e ative-o:

   ```bash theme={null}
   sudo fallocate -l 2G /swapfile
   sudo chmod 600 /swapfile
   sudo mkswap /swapfile
   sudo swapon /swapfile
   ```

   Depois tente a instalação novamente:

   ```bash theme={null}
   curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Feche outros processos** para liberar memória antes de instalar.

3. **Use uma instância maior** se possível. Claude Code requer pelo menos 4 GB de RAM.

<h3 id="install-hangs-in-docker">
  Install hangs in Docker
</h3>

Ao instalar Claude Code em um contêiner Docker, instalar como root em `/` pode causar travamentos.

**Soluções:**

1. **Defina um diretório de trabalho** antes de executar o instalador. Quando executado de `/`, o instalador verifica todo o sistema de arquivos, o que causa uso excessivo de memória. Definir `WORKDIR` limita a verificação a um pequeno diretório:
   ```dockerfile theme={null}
   WORKDIR /tmp
   RUN curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Aumente os limites de memória do Docker** se usar Docker Desktop:
   ```bash theme={null}
   docker build --memory=4g .
   ```

<h3 id="claude-desktop-overrides-the-claude-command-on-windows">
  Claude Desktop overrides the `claude` command on Windows
</h3>

Se você instalou uma versão mais antiga do Claude Desktop, ele pode registrar um `Claude.exe` no diretório `WindowsApps` que tem prioridade de PATH sobre Claude Code CLI. Executar `claude` abre o aplicativo Desktop em vez do CLI.

Atualize Claude Desktop para a versão mais recente para corrigir este problema.

<h3 id="claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell">
  Claude Code on Windows requires either Git for Windows (for bash) or PowerShell
</h3>

Git for Windows é opcional. Claude Code usa a [PowerShell tool](/docs/pt/tools-reference#powershell-tool) quando Git Bash está ausente, então este erro significa que nenhum shell foi encontrado.

**Se PowerShell estiver faltando do seu PATH**, sua localização padrão é `C:\Windows\System32\WindowsPowerShell\v1.0\`. Adicione esse diretório ao seu `PATH`, ou instale [PowerShell 7](https://aka.ms/powershell), que fornece `pwsh`.

**Para instalar Git for Windows em vez disso**, baixe de [git-scm.com/downloads/win](https://git-scm.com/downloads/win). Durante a configuração, selecione "Add to PATH." Reinicie seu terminal após instalar. Instalá-lo ativa a ferramenta Bash, útil ao trabalhar com scripts e ferramentas baseadas em Bash.

**Se Git já estiver instalado** mas Claude Code não conseguir encontrá-lo, defina o caminho em seu [settings.json file](/docs/pt/settings):

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
  }
}
```

Se seu Git estiver instalado em outro lugar, encontre o caminho executando `where.exe git` no PowerShell e use o caminho `bin\bash.exe` desse diretório.

**Se o caminho estiver correto e o arquivo existir** mas Claude Code ainda relatar que não foi encontrado, o software de segurança de endpoint como AppLocker, políticas de restrição de software de Política de Grupo ou agentes EDR podem estar interferindo. Em versões anteriores a v2.1.116, Claude Code gerava um processo filho (`cmd.exe`) para verificar o caminho, que essas políticas podem bloquear — um sinal comum é que `cmd.exe /c dir "C:\Program Files\Git\bin\bash.exe"` funciona quando você o executa diretamente no PowerShell mas falha silenciosamente quando iniciado por `claude.exe`.

Claude Code v2.1.116 e posterior verificam o sistema de arquivos diretamente, então atualize primeiro. Se o erro persistir em uma versão atual, peça à sua equipe de TI para colocar na lista de permissões `claude.exe` e os processos que ele gera, incluindo `cmd.exe` e `bash.exe`, em sua política de proteção de endpoint.

<h3 id="claude-code-does-not-support-32-bit-windows">
  Claude Code does not support 32-bit Windows
</h3>

O Windows inclui duas entradas do PowerShell no menu Iniciar: `Windows PowerShell` e `Windows PowerShell (x86)`. A entrada x86 é executada como um processo de 32 bits e dispara este erro mesmo em uma máquina de 64 bits. Para verificar qual caso você está, execute isto na mesma janela que produziu o erro:

```powershell theme={null}
[Environment]::Is64BitOperatingSystem
```

Se isso imprimir `True`, seu sistema operacional está bem. Feche a janela, abra `Windows PowerShell` sem o sufixo x86 e execute o comando de instalação novamente.

Se isso imprimir `False`, você está em uma edição de 32 bits do Windows. Claude Code requer um sistema operacional de 64 bits. Consulte os [system requirements](/docs/pt/setup#system-requirements).

<h3 id="linux-musl-or-glibc-binary-mismatch">
  Linux musl or glibc binary mismatch
</h3>

Se você vir erros sobre bibliotecas compartilhadas ausentes como `libstdc++.so.6` ou `libgcc_s.so.1` após a instalação, o instalador pode ter baixado a variante binária errada para seu sistema.

```text theme={null}
Error loading shared library libstdc++.so.6: No such file or directory
```

Isso pode acontecer em sistemas baseados em glibc que têm pacotes de compilação cruzada musl instalados, fazendo o instalador detectar incorretamente o sistema como musl.

**Soluções:**

1. **Verifique qual libc seu sistema usa**:
   ```bash theme={null}
   ldd --version 2>&1 | head -1
   ```
   A saída mencionando `GNU libc` ou `GLIBC` significa glibc. A saída mencionando `musl` significa musl.

2. **Se você estiver em glibc mas recebeu o binário musl**, remova a instalação e reinstale. Você também pode baixar manualmente o binário correto usando o manifesto em `https://downloads.claude.ai/claude-code-releases/{VERSION}/manifest.json`. Abra um [GitHub issue](https://github.com/anthropics/claude-code/issues) com a saída de `ldd --version` e `ls /lib/libc.musl*`.

3. **Se você estiver realmente em musl**, como Alpine Linux, instale os pacotes necessários:
   ```bash theme={null}
   apk add libgcc libstdc++ ripgrep
   ```

<h3 id="illegal-instruction">
  `Illegal instruction`
</h3>

Se executar `claude` ou o instalador imprimir `Illegal instruction`, o binário nativo usa instruções de CPU que seu processador não suporta. Existem duas causas distintas.

**Incompatibilidade de arquitetura.** O instalador baixou o binário errado, por exemplo x86 em um servidor ARM. Verifique com `uname -m` no macOS ou Linux, ou `$env:PROCESSOR_ARCHITECTURE` no PowerShell. Se o resultado não corresponder ao binário que você recebeu, [abra um GitHub issue](https://github.com/anthropics/claude-code/issues) com a saída.

**Conjunto de instruções AVX ausente.** Se sua arquitetura estiver correta mas você ainda vir `Illegal instruction`, seu CPU provavelmente não tem AVX ou outra instrução que o binário requer. Isso afeta aproximadamente processadores Intel e AMD anteriores a 2013, e máquinas virtuais onde o hipervisor não passa AVX para o convidado.

Em um VPS ou VM, execute `grep -m1 -ow avx /proc/cpuinfo`; um resultado vazio significa que AVX não está disponível para o convidado.

Não há solução alternativa de binário nativo; acompanhe [issue #50384](https://github.com/anthropics/claude-code/issues/50384) para status e inclua seu modelo de CPU de `grep -m1 "model name" /proc/cpuinfo` no Linux ou `sysctl -n machdep.cpu.brand_string` no macOS ao relatar.

Métodos de instalação alternativos baixam o mesmo binário nativo e não resolverão nenhuma das causas.

<h3 id="dyld-cannot-load-on-macos">
  `dyld: cannot load` on macOS
</h3>

Se você vir `dyld: cannot load`, `dyld: Symbol not found`, ou `Abort trap: 6` durante a instalação, o binário é incompatível com sua versão ou hardware do macOS.

```text theme={null}
dyld: cannot load 'claude-2.1.42-darwin-x64' (load command 0x80000034 is unknown)
Abort trap: 6
```

Um erro `Symbol not found` que referencia `libicucore` também indica que sua versão do macOS é mais antiga do que o binário suporta:

```text theme={null}
dyld: Symbol not found: _ubrk_clone
  Referenced from: claude-darwin-x64 (which was built for Mac OS X 13.0)
  Expected in: /usr/lib/libicucore.A.dylib
```

**Soluções:**

1. **Verifique sua versão do macOS**: Claude Code requer macOS 13.0 ou posterior. Abra o menu Apple e selecione About This Mac para verificar sua versão.

2. **Atualize o macOS** se você estiver em uma versão mais antiga. O binário usa comandos de carregamento e bibliotecas do sistema que versões mais antigas do macOS não suportam. Métodos de instalação alternativos como Homebrew baixam o mesmo binário e não resolverão este erro.

<h3 id="exec-format-error-on-wsl1">
  `Exec format error` on WSL1
</h3>

Se executar `claude` em WSL imprimir `cannot execute binary file: Exec format error`, você está em WSL1 e atingindo uma regressão de binário nativo conhecida rastreada em [issue #38788](https://github.com/anthropics/claude-code/issues/38788). Os cabeçalhos do programa do binário mudaram de uma forma que o carregador do WSL1 não consegue lidar.

A correção mais limpa é converter sua distribuição para WSL2 do PowerShell:

```powershell theme={null}
wsl --set-version <DistroName> 2
```

Se você precisar ficar em WSL1, invoque o binário através do vinculador dinâmico. Adicione esta função a `~/.bashrc` dentro do WSL, substituindo o caminho se seu diretório inicial for diferente:

```bash theme={null}
claude() {
  /lib64/ld-linux-x86-64.so.2 "$(readlink -f "$HOME/.local/bin/claude")" "$@"
}
```

Depois execute `source ~/.bashrc` e tente novamente `claude`.

<h3 id="npm-install-errors-in-wsl">
  npm install errors in WSL
</h3>

Estes problemas se aplicam se você instalou Claude Code com `npm install -g` dentro do WSL. Se você usou o [native installer](/docs/pt/setup), pule esta seção.

**Problemas de detecção de SO ou plataforma.** Se npm relatar uma incompatibilidade de plataforma durante a instalação, WSL provavelmente está pegando o `npm` do Windows. Execute `npm config set os linux` primeiro, depois instale com `npm install -g @anthropic-ai/claude-code --force`. Não use `sudo`.

**`exec: node: not found` ao executar `claude`.** Seu ambiente WSL provavelmente está usando a instalação do Windows do Node.js. Confirme com `which npm` e `which node`: caminhos começando com `/mnt/c/` são binários do Windows, enquanto caminhos Linux começam com `/usr/`. Para corrigir isso, instale Node via gerenciador de pacotes da sua distribuição Linux ou via [`nvm`](https://github.com/nvm-sh/nvm).

**Conflitos de versão nvm.** Se você tiver nvm instalado tanto em WSL quanto em Windows, alternar versões do Node em WSL pode quebrar porque WSL importa o PATH do Windows por padrão e o nvm do Windows tem prioridade. A causa mais comum é que nvm não está carregado em seu shell. Adicione o carregador nvm a `~/.bashrc` ou `~/.zshrc`:

```bash theme={null}
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"
```

Ou carregue-o em sua sessão atual:

```bash theme={null}
source ~/.nvm/nvm.sh
```

Se nvm está carregado mas caminhos do Windows ainda têm prioridade, coloque explicitamente seu caminho do Node Linux:

```bash theme={null}
export PATH="$HOME/.nvm/versions/node/$(node -v)/bin:$PATH"
```

<Warning>
  Evite desabilitar a importação de PATH do Windows via `appendWindowsPath = false` pois isso quebra a capacidade de chamar executáveis do Windows do WSL. Da mesma forma, evite desinstalar Node.js do Windows se você o usa para desenvolvimento do Windows.
</Warning>

<h3 id="permission-errors-during-installation">
  Permission errors during installation
</h3>

Se o instalador nativo falhar com erros de permissão, o diretório de destino pode não ser gravável. Consulte [Check directory permissions](#check-directory-permissions).

Se você instalou anteriormente com npm e está atingindo erros de permissão específicos do npm, mude para o instalador nativo:

```bash theme={null}
curl -fsSL https://claude.ai/install.sh | bash
```

<h3 id="native-binary-not-found-after-npm-install">
  Native binary not found after npm install
</h3>

O pacote npm `@anthropic-ai/claude-code` puxa o binário nativo através de uma dependência opcional por plataforma como `@anthropic-ai/claude-code-darwin-arm64`. Se executar `claude` após a instalação imprimir `Could not find native binary package "@anthropic-ai/claude-code-<platform>"`, verifique as seguintes causas:

* **Dependências opcionais estão desabilitadas.** Remova `--omit=optional` do seu comando npm install, `--no-optional` do pnpm, ou `--ignore-optional` do yarn, e verifique que `.npmrc` não define `optional=false`. Depois reinstale. O binário nativo é entregue apenas como uma dependência opcional, então não há fallback JavaScript se for ignorado.
* **Plataforma não suportada.** Binários pré-compilados são publicados para `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `linux-x64-musl`, `linux-arm64-musl`, `win32-x64` e `win32-arm64`. Claude Code não envia um binário para outras plataformas; consulte os [system requirements](/docs/pt/setup#system-requirements). No FreeBSD, o instalador relata a plataforma como não suportada. Antes da v2.1.205, ele tratava FreeBSD como Linux e baixava um binário que não conseguia executar.
* **Espelho npm corporativo está faltando os pacotes de plataforma.** Certifique-se de que seu registro espelha todos os oito pacotes `@anthropic-ai/claude-code-*` de plataforma além do pacote meta.

Instalar com `--ignore-scripts` não dispara este erro. A etapa de pós-instalação que vincula o binário no lugar é ignorada, então Claude Code volta a um wrapper que localiza e gera o binário de plataforma em cada inicialização. Isso funciona mas inicia mais lentamente; reinstale com scripts habilitados para execução direta.

<h2 id="login-and-authentication">
  Login e autenticação
</h2>

Estas seções abordam falhas de login, erros OAuth e problemas de token.

<h3 id="reset-your-login">
  Redefinir seu login
</h3>

Quando o login falha e a causa não é óbvia, uma re-autenticação limpa resolve a maioria dos casos:

1. Execute `/logout` para sair completamente
2. Feche Claude Code
3. Reinicie com `claude` e complete o processo de autenticação novamente

Se o navegador não abrir automaticamente durante o login, pressione `c` para copiar a URL OAuth para sua área de transferência e depois cole-a em um navegador manualmente. Isso também funciona quando a URL se estende por várias linhas em um terminal estreito ou SSH e não pode ser clicada diretamente.

<h3 id="oauth-error-invalid-code">
  OAuth error: Invalid code
</h3>

Se você vir `OAuth error: Invalid code. Please make sure the full code was copied`, o código de login expirou ou foi truncado durante cópia e cola.

**Soluções:**

* Pressione Enter para tentar novamente e complete o login rapidamente após o navegador abrir
* Digite `c` para copiar a URL completa se o navegador não abrir automaticamente
* Se usar uma sessão remota/SSH, o navegador pode abrir na máquina errada. Copie a URL exibida no terminal e abra-a em seu navegador local em vez disso.

<h3 id="403-forbidden-after-login">
  403 Forbidden after login
</h3>

Se você vir `API Error: 403 {"error":{"type":"forbidden","message":"Request not allowed"}}` após fazer login:

* **Usuários Claude Pro/Max**: verifique se sua assinatura está ativa em [claude.ai/settings](https://claude.ai/settings)
* **Usuários do Anthropic Console**: confirme que sua conta tem a função "Claude Code" ou "Developer". Os administradores atribuem isso no Anthropic Console em Settings → Members.
* **Atrás de um proxy**: proxies corporativos podem interferir com solicitações de API. Consulte [network configuration](/docs/pt/network-config) para configuração de proxy.

<h3 id="this-organization-has-been-disabled-with-an-active-subscription">
  This organization has been disabled with an active subscription
</h3>

Se você vir `API Error: 400 ... "This organization has been disabled"` apesar de ter uma assinatura Claude ativa, uma variável de ambiente `ANTHROPIC_API_KEY` está substituindo sua assinatura. Isso comumente acontece quando uma chave de API antiga de um empregador anterior ou projeto ainda está definida em seu perfil de shell.

Quando `ANTHROPIC_API_KEY` está presente e você a aprovou, Claude Code usa essa chave em vez das credenciais OAuth da sua assinatura. Em modo não interativo com a flag `-p`, a chave é sempre usada quando presente. Consulte [authentication precedence](/docs/pt/authentication#authentication-precedence) para a ordem de resolução completa.

Para usar sua assinatura em vez disso, desdefina a variável de ambiente e remova-a do seu perfil de shell:

```bash theme={null}
unset ANTHROPIC_API_KEY
claude
```

Verifique `~/.zshrc`, `~/.bashrc` ou `~/.profile` para linhas `export ANTHROPIC_API_KEY=...` e remova-as para tornar a alteração permanente. No Windows, verifique seu perfil PowerShell em `$PROFILE` e suas variáveis de ambiente do usuário para `ANTHROPIC_API_KEY`. Execute `/status` dentro do Claude Code para confirmar qual método de autenticação está ativo.

<h3 id="oauth-login-fails-in-wsl2-ssh-or-containers">
  OAuth login fails in WSL2, SSH, or containers
</h3>

Quando Claude Code é executado em WSL2, em uma máquina remota via SSH ou dentro de um container, o navegador geralmente abre em um host diferente e seu redirecionamento não consegue alcançar o servidor de callback local do Claude Code. Depois que você faz login, o navegador mostra um código de login em vez de redirecionar automaticamente. Cole esse código no terminal no prompt `Paste code here if prompted` para completar o login.

Se o navegador não abrir nada do WSL2, defina a variável de ambiente `BROWSER` para o caminho do seu navegador do Windows:

```bash theme={null}
export BROWSER="/mnt/c/Program Files/Google/Chrome/Application/chrome.exe"
claude
```

Alternativamente, pressione `c` no prompt de login interativo para copiar a URL OAuth, ou copie a URL que `claude auth login` imprime, e abra-a em um navegador em sua máquina local.

Se colar o código no prompt interativo não fizer nada, o atalho de cola do seu terminal provavelmente não está alcançando o campo de entrada. Tente o atalho de cola alternativo do seu terminal, frequentemente clique direito ou Shift+Insert no Windows Terminal, ou use `claude auth login` em vez disso, que lê o código colado da entrada padrão:

```bash theme={null}
claude auth login
```

Este fallback também se aplica no Windows nativo ou qualquer terminal onde colar no prompt interativo falha.

<h3 id="not-logged-in-or-token-expired">
  Not logged in or token expired
</h3>

Se Claude Code solicitar que você faça login novamente após uma sessão, seu token OAuth pode ter expirado.

Execute `/login` para re-autenticar. Se isso acontecer frequentemente, verifique se seu relógio do sistema está preciso, pois a validação de token depende de timestamps corretos.

No macOS, o login também pode falhar quando o Keychain está bloqueado ou sua senha está fora de sincronização com sua senha de conta, o que impede Claude Code de salvar credenciais. Execute `claude doctor` para verificar o acesso ao Keychain. Para desbloquear o Keychain manualmente, execute `security unlock-keychain ~/Library/Keychains/login.keychain-db`. Se desbloquear não ajudar, abra Keychain Access, selecione o keychain `login` e escolha Edit > Change Password for Keychain "login" para ressincronizá-lo com sua senha de conta.

<h3 id="bedrock-agent-platform-or-foundry-credentials-not-loading">
  Bedrock, Agent Platform, or Foundry credentials not loading
</h3>

Se você configurou Claude Code para usar um provedor de nuvem e vê `Could not load credentials from any providers` no Amazon Bedrock, `Could not load the default credentials` no Google Cloud's Agent Platform, ou `ChainedTokenCredential authentication failed` no Microsoft Foundry, seu CLI do provedor de nuvem provavelmente não está autenticado no shell atual.

Para Amazon Bedrock, confirme que suas credenciais AWS são válidas:

```bash theme={null}
aws sts get-caller-identity
```

Para Google Cloud's Agent Platform, confirme que `ANTHROPIC_VERTEX_PROJECT_ID` e `CLOUD_ML_REGION` estão definidos em seu shell, depois defina credenciais padrão de aplicativo:

```bash theme={null}
gcloud auth application-default login
```

Para Microsoft Foundry, confirme que `ANTHROPIC_FOUNDRY_API_KEY` está definido, ou faça login com a CLI do Azure para que a cadeia de credenciais padrão possa encontrar sua conta:

```bash theme={null}
az login
```

Se as credenciais funcionam em seu terminal mas não na extensão VS Code ou JetBrains, o processo IDE provavelmente não herdou seu ambiente de shell. Defina as variáveis de ambiente do provedor nas configurações do próprio IDE, ou inicie o IDE a partir de um terminal onde elas já estão exportadas.

Consulte [Amazon Bedrock](/docs/pt/amazon-bedrock), [Google Cloud's Agent Platform](/docs/pt/google-vertex-ai), ou [Microsoft Foundry](/docs/pt/microsoft-foundry) para configuração completa do provedor.

<h2 id="still-stuck">
  Still stuck
</h2>

Se nenhum dos itens acima resolver seu problema:

1. Verifique o [GitHub repository](https://github.com/anthropics/claude-code/issues) para problemas conhecidos, ou abra um novo com seu sistema operacional, o comando de instalação que você executou e a saída de erro completa
2. Se `claude --version` funciona mas algo mais está errado, execute `claude doctor` para um relatório de diagnóstico automatizado
3. Se você conseguir iniciar uma sessão, use `/feedback` dentro do Claude Code para relatar o problema
