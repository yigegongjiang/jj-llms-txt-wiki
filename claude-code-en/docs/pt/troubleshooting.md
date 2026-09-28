> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Troubleshooting

> Corrija o alto uso de CPU ou memória, travamentos, thrashing de auto-compact e problemas de pesquisa no Claude Code, e encontre a página correta para outros problemas.

Esta página cobre problemas de desempenho, estabilidade e pesquisa uma vez que Claude Code está em execução. Para outros problemas, comece com a página que corresponde ao local onde você está preso:

| Sintoma                                                                                                                                               | Ir para                                                                                  |
| :---------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------- |
| `command not found`, falha na instalação, problemas de PATH, `EACCES`, erros de TLS                                                                   | [Troubleshoot installation and login](/docs/pt/troubleshoot-install)                          |
| Atualização ou falha de download de instalação com `The connection dropped while downloading the update` ou `aborted`                                 | [Error reference](/docs/pt/errors#the-connection-dropped-while-downloading-the-update)        |
| Loops de login, erros OAuth, `403 Forbidden`, "organization disabled", credenciais Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry | [Troubleshoot installation and login](/docs/pt/troubleshoot-install#login-and-authentication) |
| Configurações não aplicadas, hooks não disparando, servidores MCP não carregando                                                                      | [Debug your configuration](/docs/pt/debug-your-config)                                        |
| Sessão iniciada em modo automático, ou Claude edita arquivos e executa comandos sem perguntar                                                         | [Which mode a session starts in](/docs/pt/permission-modes#which-mode-a-session-starts-in)    |
| `API Error: 5xx`, `529 Overloaded`, `429`, erros de validação de solicitação                                                                          | [Error reference](/docs/pt/errors)                                                            |
| `model not found` ou `you may not have access to it`                                                                                                  | [Error reference](/docs/pt/errors#theres-an-issue-with-the-selected-model)                    |
| Extensão VS Code não conectando ou detectando Claude                                                                                                  | [VS Code integration](/docs/pt/vs-code#fix-common-issues)                                     |
| `Claude Code process exited with code 1` no VS Code ou em um aplicativo SDK                                                                           | [Error reference](/docs/pt/errors#claude-code-process-exited-with-code-n)                     |
| Plugin JetBrains ou IDE não detectado                                                                                                                 | [JetBrains integration](/docs/pt/jetbrains#troubleshooting)                                   |
| Alto uso de CPU ou memória, respostas lentas, travamentos, pesquisa não encontrando arquivos                                                          | [Performance and stability](#performance-and-stability) abaixo                           |

Se você não tem certeza qual se aplica, execute `/doctor` dentro do Claude Code para uma verificação automatizada de sua instalação, configurações, extensões e uso de contexto; ele propõe correções que pode aplicar após você confirmar. Se `claude` não iniciar completamente, execute `claude doctor` do seu shell em vez disso. Execute `/mcp` para verificar o status do servidor MCP.

<h2 id="performance-and-stability">
  Desempenho e estabilidade
</h2>

Essas seções cobrem problemas relacionados ao uso de recursos, responsividade e comportamento de pesquisa.

<h3 id="high-cpu-or-memory-usage">
  Alto uso de CPU ou memória
</h3>

Claude Code é projetado para funcionar com a maioria dos ambientes de desenvolvimento, mas pode consumir recursos significativos ao processar grandes bases de código. Se você está experimentando problemas de desempenho:

1. Use `/compact` regularmente para reduzir o tamanho do contexto. Se retornar `Not enough messages to compact.`, a conversa tem muito poucas voltas para resumir; isso pode acontecer mesmo com um contexto completo quando uma única colagem grande o preencheu
2. Feche e reinicie Claude Code entre tarefas principais
3. Considere adicionar grandes diretórios de compilação ao seu arquivo `.gitignore`
4. Reinicie com [`claude --safe-mode`](/docs/pt/cli-reference#cli-flags) para verificar se um plugin, servidor MCP ou hook é a origem. Isso desabilita todas as personalizações para a sessão; se o uso diminuir, veja [Debug your configuration](/docs/pt/debug-your-config#test-against-a-clean-configuration) para encontrar qual é

Se o uso de memória permanecer alto após essas etapas, execute `/heapdump` para escrever dois arquivos em `~/Desktop`: um snapshot de heap JavaScript nomeado `<session-id>.heapsnapshot` e um detalhamento de memória nomeado `<session-id>-diagnostics.json`. Claude Code [oculta o comando do menu de comandos](/docs/pt/commands#how-the-command-menu-matches-what-you-type); digite-o por completo. No Linux sem uma pasta Desktop, os arquivos são escritos em seu diretório home.

<Warning>
  O arquivo `.heapsnapshot` contém todas as strings no processo, incluindo sua conversa completa e credenciais. Não o anexe a um problema público ou o compartilhe.
</Warning>

O comando também imprime um resumo na conversa, mostrando tamanho do conjunto residente, heap JS, buffers de array e memória nativa não contabilizada, além de quaisquer indicadores de vazamento que detectou, como uma alta taxa de crescimento de memória ou um número inusitadamente alto de identificadores abertos. O resumo diz se a maioria da memória está no heap JS, que o snapshot captura, ou em memória nativa, que não captura.

Faça uma de duas coisas com a saída:

* **Relate-a**: abra um [problema no GitHub](https://github.com/anthropics/claude-code/issues) e anexe apenas o arquivo `-diagnostics.json`, que contém as estatísticas por trás do resumo impresso e nenhum conteúdo de conversa ou credenciais
* **Investigue você mesmo**: se o resumo disser que a maioria da memória é heap JS, abra o arquivo `.heapsnapshot` no Chrome DevTools em Memory → Load e classifique por tamanho retido para ver o que está mantendo a memória

Se o resumo disser que a maioria da memória é nativa, o snapshot não pode mostrá-la; inclua os indicadores de vazamento do resumo em seu relatório.

<h3 id="large-tables-are-cut-off-in-the-terminal">
  Tabelas grandes são cortadas no terminal
</h3>

Uma tabela Markdown com mais de 200 linhas renderiza suas primeiras 200 linhas seguidas por uma linha `… N more rows not shown`. Apenas a exibição é limitada: a tabela completa permanece na conversa, e [`/copy`](/docs/pt/commands) copia cada linha. Para uma tabela muito grande para ler no terminal, peça ao Claude para escrevê-la em um arquivo em vez disso. Antes da v2.1.208, Claude Code renderizava cada linha, então retomar uma sessão que continha uma tabela muito grande poderia travar enquanto a re-renderizava.

<h3 id="auto-compaction-stops-with-a-thrashing-error">
  Auto-compactação para com erro de thrashing
</h3>

Se você vir `Autocompact is thrashing: the context refilled to the limit...`, a compactação automática foi bem-sucedida mas um arquivo ou saída de ferramenta imediatamente refilled a janela de contexto várias vezes seguidas. Claude Code para de tentar novamente para evitar desperdiçar chamadas de API em um loop que não está fazendo progresso.

Para recuperar:

1. Peça ao Claude para ler o arquivo oversized em pedaços menores, como um intervalo de linha específico ou função, em vez do arquivo inteiro
2. Execute `/compact` com um foco que descarta a saída grande, por exemplo `/compact keep only the plan and the diff`
3. Mova o trabalho de arquivo grande para um [subagent](/docs/pt/sub-agents) para que ele execute em uma janela de contexto separada
4. Execute `/clear` se a conversa anterior não for mais necessária

<h3 id="command-hangs-or-freezes">
  Comando trava ou congela
</h3>

Se Claude Code parece não responsivo:

1. Pressione Ctrl+C para tentar cancelar a operação atual
2. Se não responsivo, você pode precisar fechar o terminal e reiniciar

Reiniciar não perde sua conversa. Execute `claude --resume` no mesmo diretório para retomar a sessão.

<h3 id="garbled-or-corrupted-text-in-an-editor’s-integrated-terminal">
  Texto garbled ou corrompido no terminal integrado de um editor
</h3>

Se os caracteres renderizam como caixas, manchas ou glifos incorretos ao executar Claude Code no terminal integrado do VS Code, Cursor ou Devin Desktop, o renderizador GPU do terminal é provavelmente a causa. Execute `/terminal-setup` dentro do Claude Code para definir `terminal.integrated.gpuAcceleration` como `"off"`, ou defina-o manualmente nas configurações do seu editor e recarregue a janela. Veja [Terminal configuration](/docs/pt/terminal-config) para as outras configurações que `/terminal-setup` escreve.

<h3 id="mouse-wheel-scrolls-one-line-at-a-time-in-fullscreen-rendering">
  Roda do mouse rola uma linha por vez na renderização em tela cheia
</h3>

Na [renderização em tela cheia](/docs/pt/fullscreen), Claude Code rola a conversa em si em vez de deixá-la para seu terminal. Se cada entalhe da roda move menos linhas do que você quer, execute `/scroll-speed` para aumentar o número de linhas por entalhe e salve-o, ou defina a variável de ambiente `CLAUDE_CODE_SCROLL_SPEED`, exceto no terminal do IDE JetBrains, onde Claude Code aplica seu próprio tratamento de rolagem e nenhum dos dois tem efeito. Veja [Mouse wheel scrolling](/docs/pt/fullscreen#mouse-wheel-scrolling) para os valores que cada um aceita.

Para se mover mais rápido sem alterar a velocidade, pressione `PgUp` e `PgDn` para rolar meia tela por vez. Para devolver a rolagem ao backscroll nativo do seu terminal, execute `/tui default` para alternar para o renderizador clássico.

<h3 id="clipboard-commands-such-as-pbcopy-fail-inside-the-sandbox">
  Comandos de área de transferência como `pbcopy` falham dentro da sandbox
</h3>

Quando [sandboxing](/docs/pt/sandboxing) está ativado, utilitários de área de transferência como `pbcopy`, `xclip` e `wl-copy` podem falhar ao alcançar a área de transferência do sistema de dentro de um comando Bash em sandbox, deixando sua área de transferência inalterada após Claude canalizar texto para eles.

Para colocar a saída do Claude em sua área de transferência, peça ao Claude para imprimir o conteúdo em sua resposta, depois execute [`/copy`](/docs/pt/commands). `/copy` escreve na área de transferência do próprio processo Claude Code em vez de um comando em sandbox, então sandboxing não o bloqueia. Ele pode copiar um único bloco de código em vez de toda a resposta, e também escreve o que copiou em um arquivo e imprime o caminho, o que lhe dá um fallback quando a escrita da área de transferência não alcança seu terminal, por exemplo sobre SSH.

Para permitir que um comando canalizado alcance a área de transferência diretamente, adicione `pbcopy *`, `wl-copy *` ou `xclip *` a [`excludedCommands`](/docs/pt/settings-reference#sandbox-excludedcommands) para que o comando execute fora da sandbox.

<h3 id="search-and-discovery-issues">
  Problemas de pesquisa e descoberta
</h3>

Se a ferramenta Search, menções `@file`, agentes personalizados ou skills personalizados não estão encontrando arquivos, o binário `ripgrep` incluído pode não ser executado em seu sistema. Instale o pacote `ripgrep` da sua plataforma e diga ao Claude Code para usá-lo em vez disso:

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    brew install ripgrep
    ```
  </Tab>

  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt install ripgrep
    ```
  </Tab>

  <Tab title="Alpine">
    ```bash theme={null}
    apk add ripgrep
    ```

    `ripgrep` está no repositório community do Alpine. Se `apk` relatar que o pacote está faltando, veja [Alpine Linux setup](/docs/pt/setup#alpine-linux-and-musl-based-distributions).
  </Tab>

  <Tab title="Arch">
    ```bash theme={null}
    pacman -S ripgrep
    ```
  </Tab>

  <Tab title="Windows">
    ```powershell theme={null}
    winget install BurntSushi.ripgrep.MSVC
    ```
  </Tab>
</Tabs>

Depois defina `USE_BUILTIN_RIPGREP` como `0`, seja em seu [environment](/docs/pt/env-vars) de shell ou no bloco `env` do seu [`settings.json`](/docs/pt/settings-reference#all-settings):

```json theme={null}
{
  "env": {
    "USE_BUILTIN_RIPGREP": "0"
  }
}
```

Para confirmar que a mudança teve efeito, execute `claude doctor` em seu terminal e verifique se a linha Search mostra o caminho do seu ripgrep do sistema em vez de `OK (bundled)`.

<h3 id="slow-or-incomplete-search-results-on-wsl">
  Resultados de pesquisa lentos ou incompletos em WSL
</h3>

Penalidades de desempenho de leitura de disco ao [trabalhar entre sistemas de arquivos em WSL](https://learn.microsoft.com/en-us/windows/wsl/filesystems) podem resultar em menos correspondências do que o esperado ao usar Claude Code em WSL. A pesquisa ainda funciona, mas retorna menos resultados do que em um sistema de arquivos nativo.

<Note>
  `claude doctor` mostra Search como OK neste caso.
</Note>

**Soluções:**

1. **Envie pesquisas mais específicas**: reduza o número de arquivos pesquisados especificando diretórios ou tipos de arquivo: "Search for JWT validation logic in the auth-service package" ou "Find use of md5 hash in JS files".

2. **Mova o projeto para o sistema de arquivos Linux**: se possível, certifique-se de que seu projeto está localizado no sistema de arquivos Linux (`/home/`) em vez do sistema de arquivos do Windows (`/mnt/c/`).

3. **Use Windows nativo em vez disso**: considere executar Claude Code nativamente no Windows em vez de através de WSL, para melhor desempenho do sistema de arquivos.

<h2 id="get-more-help">
  Obtenha mais ajuda
</h2>

Se você está experimentando problemas não cobertos aqui:

1. Execute `/doctor` para uma verificação de configuração e `/mcp` para verificar o status do servidor MCP
2. Use o comando `/feedback` dentro do Claude Code para relatar problemas diretamente à Anthropic
3. Verifique o [repositório GitHub](https://github.com/anthropics/claude-code) para problemas conhecidos
4. Pergunte ao Claude diretamente sobre suas capacidades e recursos. Claude tem acesso integrado à sua documentação.

Para problemas de conta, faturamento ou assinatura, entre em contato com o suporte da Anthropic: faça login em [claude.ai](https://claude.ai) (Usuários do Console: [platform.claude.com](https://platform.claude.com)), clique em suas iniciais no canto inferior esquerdo e selecione **Obter ajuda**. Consulte [Como obter suporte](https://support.claude.com/en/articles/9015913-how-to-get-support) para o fluxo completo, incluindo quem pode alcançar um agente humano em cada plano.
