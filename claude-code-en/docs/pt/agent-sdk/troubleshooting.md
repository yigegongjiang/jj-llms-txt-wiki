> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Solucionar problemas do Agent SDK

> Corrija erros do Agent SDK quando a CLI do Claude Code falha ao iniciar, o processo da CLI sai, ou um resultado bem-sucedido chega sem saída estruturada.

Esta página cobre erros do Agent SDK na inicialização da CLI, saída do processo da CLI e saídas estruturadas. As entradas nesta página são organizadas de acordo com o erro que você vê. Cada uma nomeia a causa e o que fazer.

Os sintomas vinculados a um recurso, como um hook não disparando ou uma skill não sendo usada, têm uma seção de solução de problemas na página desse recurso. A tabela nomeia a seção ou página que cobre cada sintoma:

| Sintoma                                                                                                                                                                                                                                                                                                                               | Ir para                                                                                                                                        |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| Skills não encontradas, uma skill não sendo usada, erro `Invalid skill name`                                                                                                                                                                                                                                                          | [Solução de problemas de Skills](/docs/pt/agent-sdk/skills#troubleshooting)                                                                         |
| Servidor MCP mostra status `failed`, ferramentas não sendo chamadas, timeouts de conexão, saída de ferramenta que excede o máximo de tokens permitidos                                                                                                                                                                                | [Solução de problemas de MCP](/docs/pt/agent-sdk/mcp#troubleshooting)                                                                               |
| Plugin não carregando, skills de plugin não aparecendo                                                                                                                                                                                                                                                                                | [Solução de problemas de Plugins](/docs/pt/agent-sdk/plugins#troubleshooting)                                                                       |
| Claude não delegando para subagentes, agentes baseados em sistema de arquivos não carregando                                                                                                                                                                                                                                          | [Solução de problemas de Subagents](/docs/pt/agent-sdk/subagents#troubleshooting)                                                                   |
| Opções de checkpointing não reconhecidas, mensagens de usuário sem UUIDs, `No file checkpoint found`, `File rewinding is not enabled`, `ProcessTransport is not ready for writing`                                                                                                                                                    | [Solução de problemas de checkpointing de arquivo](/docs/pt/agent-sdk/file-checkpointing#troubleshooting)                                           |
| Hook não disparando, matcher não filtrando conforme esperado, timeout de hook, ferramenta bloqueada inesperadamente, entrada modificada não aplicada, hooks de sessão não disponíveis em Python, prompts de permissão de subagente se multiplicando, loops de hook recursivos com subagentes, `systemMessage` não aparecendo na saída | [Corrigir problemas comuns](/docs/pt/agent-sdk/hooks#fix-common-issues) na página de hooks                                                          |
| Um agente que funciona em sua máquina falha em um serviço implantado ou contêiner                                                                                                                                                                                                                                                     | [Solucionar falhas de implantação](/docs/pt/agent-sdk/hosting#troubleshoot-deployment-failures)                                                     |
| `Not logged in`, `Invalid API key`, `API Error`, `429`, `There's an issue with the selected model`                                                                                                                                                                                                                                    | [Referência de erro](/docs/pt/errors#find-your-error)                                                                                               |
| `CLINotFoundError`, `CLIConnectionError`, `ProcessError`, `Claude Code process exited with code N`, `Claude Code returned an error result`, `structured_output` é `None`                                                                                                                                                              | [Inicialização da CLI](#cli-startup), [Saída do processo da CLI](#cli-process-exit), e [Saídas estruturadas](#structured-outputs) nesta página |

<h2 id="cli-startup">
  Inicialização do CLI
</h2>

<h3 id="clinotfounderror-claude-code-not-found">
  CLINotFoundError: Claude Code not found
</h3>

O SDK Python inicia o CLI Claude Code como um subprocesso. Quando não consegue encontrar um executável `claude`, a conexão falha com um `CLINotFoundError`:

```
Claude Code not found at: /your/configured/path
```

A mensagem inclui o caminho configurado quando você define `ClaudeAgentOptions(cli_path=...)` e ele aponta para um arquivo ausente. Sem `cli_path`, o SDK pesquisa seu `PATH` e locais de instalação comuns, e a mensagem inclui instruções de instalação para sua plataforma.

Para corrigir:

* Instale Claude Code se não estiver instalado. Consulte [Install Claude Code](/docs/pt/setup#install-claude-code) para o comando em sua plataforma.
* Se você definir `cli_path`, confirme que o arquivo existe e é o executável `claude`.
* Se você depender da resolução de `PATH`, confirme que `claude --version` funciona no mesmo ambiente em que seu aplicativo é executado. Processos que você inicia fora do seu shell, como de um IDE ou gerenciador de serviços, geralmente são executados com um `PATH` diferente.

O SDK TypeScript procura o CLI em seu pacote de plataforma agrupado e no caminho que você define em `pathToClaudeCodeExecutable`. Corresponda à mensagem que você vê:

* `Native CLI binary for <platform>-<arch> not found`: o pacote de plataforma agrupado está ausente, na maioria das vezes porque a instalação pulou dependências opcionais. Reinstale `@anthropic-ai/claude-agent-sdk` sem pular dependências opcionais, ou aponte `pathToClaudeCodeExecutable` para uma [instalação nativa](/docs/pt/setup#install-claude-code). Em um executável de arquivo único construído com `bun build --compile`, a mesma mensagem tem uma causa e correção diferentes. Consulte [Compile to a single executable](/docs/pt/agent-sdk/typescript#compile-to-a-single-executable).
* `Claude Code native binary not found at <path>` ou `Claude Code executable not found at <path>. Is options.pathToClaudeCodeExecutable set?`: o arquivo no caminho resolvido está ausente, ou o processo não consegue acessá-lo. Confirme que o arquivo existe nesse caminho e que o processo pode acessá-lo.

<h3 id="cliconnectionerror-refusing-to-execute-batch-script">
  CLIConnectionError: Refusing to execute batch script
</h3>

No Windows, a conexão falha com um `CLIConnectionError` quando o caminho do CLI que o SDK Python usa é um script em lote `.bat` ou `.cmd`, incluindo o shim `claude.cmd` que uma instalação npm cria:

```
Refusing to execute batch script 'C:\\Users\\you\\AppData\\Roaming\\npm\\claude.cmd': Windows runs .bat/.cmd files via cmd.exe, which can execute commands injected through CLI arguments, and no reliable escaping for cmd.exe exists. Use a native claude executable instead: install Claude Code natively (irm https://claude.ai/install.ps1 | iex), point ClaudeAgentOptions(cli_path=...) at a claude.exe, or install the claude-agent-sdk wheel for a platform that bundles claude.exe (e.g. Windows x64).
```

A recusa é um endurecimento de segurança deliberado, não uma instalação quebrada. O Windows executa scripts em lote reescrevendo o spawn em uma invocação `cmd.exe /c`, e `cmd.exe` reanálisa toda a linha de comando no tempo de execução, portanto um valor de argumento pode executar comandos injetados.

A maioria das instalações do Windows nunca atinge esse erro. A wheel x64 do Windows de `claude-agent-sdk` agrupa um `claude.exe`, e o SDK prefere o CLI agrupado, depois qualquer `claude.exe` nativo que possa descobrir, antes de recorrer a um shim em lote. Você vê a recusa em dois casos:

* Você define `ClaudeAgentOptions(cli_path=...)` para um arquivo `.bat` ou `.cmd`, como o shim `claude.cmd` do npm.
* Sua instalação não tem um `claude.exe` agrupado ou nativo, por exemplo uma instalação de origem no ARM64 Windows onde o único `claude` em seu `PATH` é o shim npm.

Para corrigir, dê ao SDK um executável nativo em vez de um script em lote:

* Se você definir `ClaudeAgentOptions(cli_path=...)`, aponte-o para um `claude.exe` ou remova a opção. O SDK pula a descoberta enquanto `cli_path` está definido, portanto uma instalação nativa sozinha não pode ter efeito.
* Instale Claude Code nativamente no PowerShell: `irm https://claude.ai/install.ps1 | iex`
* No Windows x64, instale a wheel `claude-agent-sdk`, que agrupa `claude.exe`.

Antes de `claude-agent-sdk` 0.2.124, o SDK Python gerava scripts em lote através de `cmd.exe` sem essa verificação.

<h3 id="cliconnectionerror-failed-to-start-claude-code">
  CLIConnectionError: Failed to start Claude Code
</h3>

O SDK encontrou um arquivo no caminho resolvido, mas não conseguiu iniciá-lo. Python gera essas falhas como um `CLIConnectionError`. TypeScript rejeita a iteração de mensagem com um erro sem classe SDK. A tabela abaixo mapeia cada mensagem para o que ela diz a você. Corresponda à mensagem que você vê:

| Mensagem                                                          | SDK        | O que ela diz a você                                                          |
| ----------------------------------------------------------------- | ---------- | ----------------------------------------------------------------------------- |
| `Failed to start Claude Code: <detail>`                           | Python     | O resto da mensagem é o próprio erro do sistema operacional                   |
| `Claude Code executable at <path> exists but failed to launch`    | TypeScript | O script no caminho configurado não pode ser executado                        |
| `Claude Code native binary at <path> exists but failed to launch` | TypeScript | O binário não pode ser executado, com uma sugestão de libc anexada à mensagem |
| `Failed to spawn Claude Code process: <detail>`                   | TypeScript | Qualquer outra falha de inicialização                                         |

Em ambos os SDKs, a causa usual é um caminho resolvido que aponta para algo que não pode ser executado, como um arquivo de texto, um diretório ou um arquivo sem permissão de execução. Leia a sugestão de libc da mensagem de binário nativo como uma possível causa.

Para corrigir em qualquer SDK:

* Confirme que o caminho configurado aponta para o próprio executável `claude` e que o arquivo tem permissão de execução.
* Se você não precisar de um caminho personalizado, remova `cli_path` em Python ou `pathToClaudeCodeExecutable` em TypeScript para que o SDK encontre um CLI por conta própria, preferindo sua cópia agrupada.
* Quando o binário que falha é a cópia agrupada do SDK em uma imagem de contêiner, reinstale o SDK durante a construção da imagem para que o binário agrupado corresponda à plataforma do contêiner, ou reconstrua a imagem para a arquitetura em que é executada. A causa usual é um binário que não corresponde à arquitetura ou libc do contêiner, ou um que perdeu sua permissão de execução na construção da imagem.

<h3 id="cliconnectionerror-not-connected">
  CLIConnectionError: Not connected
</h3>

Chamar um método `ClaudeSDKClient` em Python antes do cliente ter se conectado, ou depois de ter se desconectado, gera um `CLIConnectionError` com esta mensagem:

```
Not connected. Call connect() first.
```

Faça o que a mensagem diz. Chame `await client.connect()` antes de qualquer outro método do cliente, ou abra o cliente com `async with ClaudeSDKClient() as client:`, que se conecta na entrada.

<h2 id="cli-process-exit">
  Saída do processo CLI
</h2>

As entradas nesta seção significam que o processo Claude Code terminou enquanto seu aplicativo o estava usando. Qual erro você vê depende da linguagem do SDK e se o CLI relatou um resultado de erro antes de sair.

<h3 id="processerror-command-failed-with-exit-code">
  ProcessError: Command failed with exit code
</h3>

O SDK Python gera um `ProcessError` quando o processo Claude Code sai com um código diferente de zero:

```
Command failed with exit code 1 (exit code: 1)
Error output: Check stderr output for details
```

A mensagem declara o código de saída duas vezes, e a linha `Error output` é texto fixo em vez da saída de erro do seu processo. O mesmo texto fixo preenche o atributo `stderr` da exceção. O atributo `exit_code` da exceção carrega o código. Para capturar o que o CLI realmente escreveu em stderr, passe um callback `stderr` em `ClaudeAgentOptions` e registre o que ele recebe.

Um `ProcessError` simples significa que o CLI saiu sem relatar um resultado de erro. Quando o CLI relatou um, o SDK gera [`ResultError`](/docs/pt/agent-sdk/python#resulterror) em vez disso, coberto em [Claude Code returned an error result](#claude-code-returned-an-error-result). `ResultError` é uma subclasse de `ProcessError`, portanto `except ProcessError` captura ambos. Para tratá-los de forma diferente, coloque a cláusula `except ResultError` primeiro.

Antes de `claude-agent-sdk` 0.2.140, o SDK Python gerava saídas de resultado de erro como uma `Exception` simples em vez de um `ResultError`.

<h3 id="claude-code-process-exited-with-code-n">
  Claude Code process exited with code N
</h3>

Wrappers IDE também imprimem esta mensagem, e a [referência de erro](/docs/pt/errors#claude-code-process-exited-with-code-n) a cobre para VS Code e outros inicializadores. Esta entrada cobre o que seu código SDK TypeScript recebe. O SDK apresenta uma saída CLI com código diferente de zero como um `Error` simples que rejeita o loop `for await` sobre as mensagens de `query()`. Não há classe de erro SDK para capturar, portanto envolva o loop em `try`/`catch` e corresponda à mensagem:

```
Claude Code process exited with code 1. stderr: <tail of the CLI's stderr>
```

Quando o CLI escreveu em stderr, a mensagem termina com a cauda dele. Para capturar o fluxo completo, passe um callback `stderr` nas opções de consulta. Um processo morto por um sinal relata `Claude Code process terminated by signal <name>` na mesma forma.

<h3 id="claude-code-returned-an-error-result">
  Claude Code returned an error result
</h3>

Ambos os SDKs substituem o erro de saída do processo por esta mensagem quando o CLI relatou um resultado de erro antes de sair:

```
Claude Code returned an error result: <the CLI's own error report>
```

O texto após os dois pontos é o relatório do CLI sobre o que deu errado, portanto comece por lá em vez de com a saída em si. Python gera isso como um [`ResultError`](/docs/pt/agent-sdk/python#resulterror), cujo atributo `data` carrega o resultado de erro completo. TypeScript rejeita o loop de mensagem com um `Error` simples carregando a mesma forma de mensagem.

<h2 id="structured-outputs">
  Saídas estruturadas
</h2>

<h3 id="structured_output-is-none-but-the-result-says-success">
  structured\_output is None but the result says success
</h3>

Uma mensagem de resultado pode terminar com `subtype: "success"` enquanto `structured_output` é `None` em Python ou `undefined` em TypeScript. A execução é concluída, mas nenhuma saída validada existe. Uma maneira de atingir isso é um esquema que nenhuma saída pode satisfazer, por exemplo restrições de comprimento conflitantes. A execução termina sem um erro de validação, e o único sinal é o `structured_output` ausente.

Trate este resultado como uma falha no código da aplicação. Verifique se `subtype` é `success` e se `structured_output` está presente antes de usá-lo. A seção [Error handling](/docs/pt/agent-sdk/structured-outputs#error-handling) mostra este padrão para ambos os SDKs.

Se isso acontecer repetidamente com um esquema que você acredita estar correto, verifique se o esquema é satisfazível, simplifique-o até que as saídas sejam validadas e reintroduza as restrições uma de cada vez.

<h2 id="report-a-new-issue">
  Relatar um novo problema
</h2>

Se seu erro não for coberto aqui, verifique os problemas abertos ou abra um novo nos repositórios do SDK: [claude-agent-sdk-typescript](https://github.com/anthropics/claude-agent-sdk-typescript/issues) ou [claude-agent-sdk-python](https://github.com/anthropics/claude-agent-sdk-python/issues). Inclua o texto de erro completo e sua versão do SDK.
