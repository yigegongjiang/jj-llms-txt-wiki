> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Escolha um ambiente sandbox

> Compare as opções de sandbox do Claude Code: a ferramenta Bash em sandbox integrada, runtime sandbox, dev containers, Docker e VMs. Escolha o isolamento certo para seu modelo de ameaça.

Isolar o Claude Code limita o que uma sessão pode ler, escrever e alcançar na rede. Isso é mais importante quando você deixa o Claude trabalhar com menos prompts de permissão, executá-lo sem supervisão ou apontá-lo para código que você não confia completamente.

O Claude Code pode ser executado em vários tipos de ambientes isolados, variando de um sandbox leve por comando a uma máquina virtual completamente separada. Esta página compara-os pelo que isolam e pelo que exigem, ajuda você a escolher um para seu modelo de ameaça e mostra como impor essa escolha em toda uma organização.

<Info>
  Para o modelo de segurança mais amplo, consulte [Security](/docs/pt/security). Para implantações do Agent SDK, consulte [Secure deployment](/docs/pt/agent-sdk/secure-deployment).
</Info>

<h2 id="compare-sandboxing-approaches">
  Compare sandboxing approaches
</h2>

As duas primeiras abordagens na tabela abaixo são executadas no sistema operacional do host sem containers. O resto coloca o Claude Code dentro de um container ou máquina virtual.

| Approach                                          | What is isolated                                                                         | Requires Docker | Setup effort                                  |
| :------------------------------------------------ | :--------------------------------------------------------------------------------------- | :-------------- | :-------------------------------------------- |
| [Sandboxed Bash tool](#sandboxed-bash-tool)       | Comandos Bash e seus processos filhos                                                    | Não             | Mínimo no macOS; baixo no Linux e WSL2        |
| [Sandbox runtime](#sandbox-runtime)               | Todo o processo do Claude Code, incluindo ferramentas de arquivo, servidores MCP e hooks | Não             | Baixo                                         |
| [Dev container](#dev-containers)                  | Ambiente de desenvolvimento completo                                                     | Sim             | Médio                                         |
| [Custom container](#custom-container)             | Ambiente de desenvolvimento completo                                                     | Sim             | Médio a alto                                  |
| [Virtual machine](#virtual-machine)               | Sistema operacional completo                                                             | Não             | Alto                                          |
| [Claude Code on the web](#claude-code-on-the-web) | Sistema operacional completo, hospedado pela Anthropic                                   | Não             | Nenhum; requer uma assinatura Claude e GitHub |

A [sandboxed Bash tool](/docs/pt/sandboxing) é integrada ao Claude Code e restringe apenas comandos Bash. As ferramentas de arquivo integradas, servidores MCP e hooks ainda são executados diretamente no seu host. Todas as outras abordagens na tabela colocam todo o processo do Claude Code dentro do limite de isolamento, portanto ferramentas de arquivo, servidores MCP e hooks também são restritos.

<Warning>
  O isolamento sandbox reduz o impacto de uma violação, mas não elimina o risco. Qualquer abordagem que permita egresso de rede ainda pode vazar dados que o agente pode ler, e qualquer abordagem que monta seu diretório de projeto com permissão de escrita ainda pode modificar esse código. Revise as [security limitations](/docs/pt/sandboxing#security-limitations) antes de confiar em um sandbox como um controle rígido.

  O isolamento também não muda o que é enviado para o modelo. Seus prompts e os arquivos que o Claude lê são transmitidos para a API Anthropic ou seu provedor configurado com ou sem um sandbox. Consulte [Data usage](/docs/pt/data-usage) para saber o que o Claude Code envia e como reduzi-lo.
</Warning>

<h2 id="choose-an-approach">
  Escolha uma abordagem
</h2>

Combine seu objetivo com uma linha abaixo e leia a seção de detalhes que segue.

| You want to                                                                                      | Start with                                                                                                                                             |
| :----------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- |
| Reduzir prompts de permissão durante o trabalho diário em sua própria máquina                    | A [sandboxed Bash tool](/docs/pt/sandboxing), ativada com `/sandbox`                                                                                        |
| Deixar o Claude trabalhar sem supervisão com `--dangerously-skip-permissions` ou modo automático | O [dev container](/docs/pt/devcontainer) pré-configurado, qualquer container ou VM, ou o [sandbox runtime](#sandbox-runtime)                                |
| Isolar servidores MCP e hooks bem como Bash, sem Docker                                          | O sandbox runtime                                                                                                                                      |
| Trabalhar em um repositório não confiável                                                        | Uma máquina virtual dedicada, ou [Claude Code on the web](/docs/pt/claude-code-on-the-web) se você tiver uma assinatura Claude e uma conta GitHub conectada |
| Padronizar um ambiente sandbox em toda uma equipe                                                | O [dev container](/docs/pt/devcontainer) pré-configurado, copiado para seu repositório                                                                      |
| Usar Claude Code de um dispositivo sem configuração local                                        | [Claude Code on the web](/docs/pt/claude-code-on-the-web), que requer uma assinatura Claude e uma conta GitHub conectada                                    |
| Exigir isolamento para cada desenvolvedor em sua organização                                     | [Enforce isolation across an organization](#enforce-isolation-across-an-organization)                                                                  |
| Trabalhar em um host Windows nativo                                                              | Um container ou VM, ou executar o sandbox Bash dentro do WSL2                                                                                          |

<h3 id="how-isolation-relates-to-permission-modes">
  Como o isolamento se relaciona com os modos de permissão
</h3>

Os [Permission modes](/docs/pt/permission-modes) decidem se uma chamada de ferramenta é executada e se você é solicitado primeiro. O isolamento restringe o que um comando pode acessar uma vez que é executado. Os dois trabalham juntos: quando um modo de permissão permite que ações sejam executadas sem pedir a você, um limite de isolamento restringe o que essas ações podem alcançar.

Quando você passa `--dangerously-skip-permissions`, o Claude age sem pedir a você primeiro; você é solicitado apenas para [ask rules](/docs/pt/permissions#manage-permissions) explícitas, ferramentas de conector [que sua organização definiu como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools), ferramentas MCP marcadas como [`requiresUserInteraction`](/docs/pt/mcp#require-approval-for-a-specific-tool), e remoções direcionadas a `/` ou seu diretório inicial. Sem prompts para capturar erros, o limite de isolamento que você escolhe é o que protege seu sistema. Sempre execute sessões `--dangerously-skip-permissions` dentro de um container, uma VM, ou o [sandbox runtime](#sandbox-runtime), para que ferramentas de arquivo, servidores MCP e hooks também estejam dentro do limite.

[Auto mode](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) substitui o prompt por um classificador que revisa ações e bloqueia aquelas que escalam além da solicitação, visam infraestrutura não reconhecida, ou parecem impulsionadas por conteúdo hostil que o Claude leu. O classificador é um controle por ação, não um limite de isolamento, portanto um limite de isolamento ainda adiciona defesa em profundidade para execuções sem supervisão, e não é necessário da forma que é para `--dangerously-skip-permissions`.

A [sandboxed Bash tool](#sandboxed-bash-tool) por si só restringe apenas Bash, portanto não é suficiente para execuções totalmente sem supervisão em nenhum dos modos. Você pode combinar abordagens: executar a sandboxed Bash tool dentro de um container ou VM oferece restrições de comando no nível do SO além do limite do ambiente externo. Para como o sandbox Bash em si interage com regras de permissão e modos, consulte [How sandboxing relates to permissions and permission modes](/docs/pt/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes).

<h2 id="sandboxed-bash-tool">
  Sandboxed Bash tool
</h2>

<Note>
  Esta opção não suporta Windows nativo. Em hosts Windows, use WSL2 ou uma das abordagens de container ou VM abaixo.
</Note>

A sandboxed Bash tool é integrada ao Claude Code. Ela usa primitivos do sistema operacional para restringir o acesso ao sistema de arquivos e rede de cada comando Bash que o Claude executa: Seatbelt, o sandbox integrado do macOS, e [bubblewrap](https://github.com/containers/bubblewrap) no Linux e WSL2. Por padrão, permite escritas no diretório de trabalho e solicita a primeira vez que um comando precisa de um novo domínio de rede.

Ative-a com o comando `/sandbox`. O guia [Sandboxing](/docs/pt/sandboxing) cobre os modos de aprovação, o limite padrão, e como ampliá-lo ou estreitá-lo.

O sandbox por comando não cobre tudo que é executado em uma sessão:

* Outras [built-in tools](/docs/pt/tools-reference) como Read, Edit e WebFetch são executadas dentro do processo do Claude Code e não geram código arbitrário. [Permission rules](/docs/pt/permissions) para caminho ou domínio as controlam.
* Servidores [MCP](/docs/pt/mcp) e hooks são processos separados que são executados sem restrições no host.

Para colocar ferramentas integradas, servidores MCP e hooks todos atrás de um limite do SO, execute todo o processo do Claude Code dentro do [sandbox runtime](#sandbox-runtime), do [dev container](#dev-containers), ou de um [custom container](#custom-container).

<h2 id="sandbox-runtime">
  Sandbox runtime
</h2>

O pacote [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) envolve um processo inteiro no mesmo isolamento Seatbelt ou bubblewrap que o sandbox Bash integrado usa. Executar o Claude Code através dele restringe cada ferramenta, hook e servidor MCP na sessão, não apenas Bash. O runtime é uma visualização prévia de pesquisa beta, e seu formato de configuração pode mudar conforme o pacote evolui.

O runtime nega todo acesso de escrita e rede por padrão, portanto configure-o antes de iniciar o Claude Code através dele. Em `~/.srt-settings.json`, ou um arquivo que você passa com `--settings`, permita acesso de escrita a pelo menos seu diretório de projeto e caminhos de configuração do Claude Code `~/.claude` e `~/.claude.json`. Permita os domínios de rede que sua sessão precisa, incluindo `api.anthropic.com` ou o endpoint do seu provedor configurado. Consulte o [README](https://github.com/anthropic-experimental/sandbox-runtime) do pacote para o esquema de configuração completo.

Uma vez que o arquivo de configurações está em vigor, inicie o Claude Code com `npx` e passe `claude` como o comando a envolver:

```bash theme={null}
npx @anthropic-ai/sandbox-runtime claude
```

O Claude Code inicia dentro do sandbox com os limites de sistema de arquivos e rede que você configurou. O mesmo comando funciona para sandboxing de servidores MCP autônomos ou outros processos auxiliares.

<h2 id="dev-containers">
  Dev containers
</h2>

Um dev container executa o Claude Code dentro de um container Docker que VS Code ou um editor compatível gerencia, com seu projeto montado. Você pode definir o seu próprio com um diretório `.devcontainer/` em seu repositório.

O repositório claude-code publica um [example dev container](/docs/pt/devcontainer) com um firewall iptables padrão-negado como ponto de partida. Copie-o para seu repositório e ajuste a lista de permissões do firewall, imagem base e versão do Claude Code fixada para se adequar ao seu ambiente. Como o firewall bloqueia egresso não aprovado, uma configuração como esta suporta executar o Claude Code com `--dangerously-skip-permissions` para trabalho sem supervisão.

<h2 id="custom-container">
  Custom container
</h2>

Você pode executar o Claude Code em qualquer imagem de container Docker ou OCI com suas próprias políticas de rede, volumes montados e perfis seccomp. Este é o caminho mais comum para organizações com infraestrutura de container existente ou executores de CI.

Vários serviços gerenciados de sandbox e execução remota podem hospedar o container para você. A mesma lista de verificação se aplica como para qualquer container que você opera: revise o que é montado com permissão de escrita, quais credenciais e tokens são alcançáveis dentro dele, e o que a política de egresso de rede permite.

Você pode combinar o sandbox Bash integrado dentro do container para restrições por comando. Containers sem privilégios precisam da configuração nested-sandbox descrita em [Sandboxing troubleshooting](/docs/pt/sandboxing#troubleshooting).

<h2 id="virtual-machine">
  Virtual machine
</h2>

Uma máquina virtual dedicada fornece a separação mais forte, com seu próprio kernel e, em implantações em nuvem ou microVM, seu próprio hardware virtualizado. As opções incluem instâncias em nuvem, hipervisores locais e microVMs como Firecracker.

Use esta abordagem quando você está avaliando código não confiável, quando sua política de segurança exige separação no nível do kernel entre o agente e o host, ou quando nenhuma abordagem no nível do host atende aos seus requisitos de conformidade. O recurso [sandboxes](https://docs.docker.com/ai/sandboxes/) do Docker Desktop fornece uma microVM com seu próprio daemon Docker e sincronização de workspace, que pode executar o Claude Code em hosts que já têm o Docker Desktop.

<h2 id="claude-code-on-the-web">
  Claude Code on the web
</h2>

[Claude Code on the web](/docs/pt/claude-code-on-the-web) executa cada sessão em uma máquina virtual isolada e gerenciada pela Anthropic. Um proxy de rede impõe uma lista de permissões padrão, e um proxy separado mantém seu token GitHub fora do sandbox enquanto emite credenciais com escopo para acesso ao repositório dentro dele.

Use esta abordagem quando você quer isolamento completo de VM sem provisionar infraestrutura você mesmo, ou quando você está delegando tarefas de um dispositivo que não tem um ambiente de desenvolvimento local. Requer uma assinatura Claude e uma conta GitHub conectada, e as sessões clonam seu repositório do GitHub. Consulte [Claude Code on the web](/docs/pt/claude-code-on-the-web) para disponibilidade de plano e opções de autenticação GitHub.

<h2 id="enforce-isolation-across-an-organization">
  Enforce isolation across an organization
</h2>

Desenvolvedores individuais podem optar por qualquer abordagem acima. O que uma organização pode impor, e com quais ferramentas, depende da abordagem:

* **Built-in Bash sandbox**: a única abordagem que o Claude Code impõe a si mesmo. Entregue as chaves de configurações `sandbox` através de [managed settings](/docs/pt/settings#settings-files), seja como um arquivo gerenciado por seu MDM ou através de [server-managed settings](/docs/pt/server-managed-settings) no Claude.ai. Consulte [Enforce sandboxing with managed settings](/docs/pt/sandboxing#enforce-sandboxing-with-managed-settings) para as chaves a implantar e como impedir que desenvolvedores ampliem a política.
* **Dev containers**: confirme o [example dev container](/docs/pt/devcontainer) em seus repositórios para padronizar o ambiente em toda uma equipe. Esta é uma convenção em vez de um limite de imposição, porque o Claude Code não requer um container. Se os desenvolvedores não devem ser capazes de executar o Claude Code fora dele, imponha isso com as ferramentas de gerenciamento de dispositivos ou software allowlisting da sua organização.
* **Custom containers and VMs**: distribua o Claude Code através da imagem aprovada e use as ferramentas de gerenciamento de dispositivos ou software allowlisting da sua organização para impedir a instalação fora dela.

<h2 id="see-also">
  Veja também
</h2>

Estas páginas cobrem detalhes de configuração e política para as abordagens acima.

* [Sandboxing](/docs/pt/sandboxing): configure a ferramenta Bash sandboxed integrada
* [Dev container](/docs/pt/devcontainer): o container de desenvolvimento Docker pré-configurado
* [Security](/docs/pt/security): o modelo de segurança completo do Claude Code
* [Secure deployment](/docs/pt/agent-sdk/secure-deployment): orientação de isolamento para aplicações do Agent SDK
* [Settings](/docs/pt/settings#sandbox-settings): todas as chaves de configuração de sandbox, incluindo entrega de configurações gerenciadas
