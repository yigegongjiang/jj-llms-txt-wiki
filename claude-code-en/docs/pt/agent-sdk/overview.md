> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Visão geral do Agent SDK

> Construa agentes de IA em produção com Claude Code como uma biblioteca

Um agente é uma aplicação que completa uma tarefa planejando seus próprios passos e chamando ferramentas que leem arquivos, executam comandos ou editam código. O Agent SDK oferece as mesmas ferramentas, [loop de agente](/docs/pt/agent-sdk/agent-loop), e gerenciamento de contexto que alimentam Claude Code, programável em Python e TypeScript.

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  Compare o Agent SDK com outras ferramentas Claude
</h2>

O Agent SDK, a CLI, o Client SDK e Managed Agents diferem em quem executa o agente, o que vem integrado e como você o acessa. Encontre a linha que corresponde a como você deseja construir e executar o seu.

| Você quer                                                                                                    | Use                                                                               | O que você obtém                                                                                                                                                                                                                                                                                                                                                                                      |
| ------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Incorporar o agente Claude Code em sua própria aplicação Python ou TypeScript, em um processo que você opera | **Agent SDK**                                                                     | Uma biblioteca que executa o binário Claude Code, com as [capacidades](#capabilities) do Claude Code, como ferramentas integradas, permissões, sessões e hooks.                                                                                                                                                                                                                                       |
| Fazer desenvolvimento interativo ou executar tarefas únicas de um terminal                                   | [**Claude Code CLI**](/docs/pt/overview)                                               | A interface do terminal, construída para uso interativo diário.                                                                                                                                                                                                                                                                                                                                       |
| Chamar a API Claude diretamente do seu próprio código                                                        | [**Client SDK**](https://platform.claude.com/docs/en/cli-sdks-libraries/overview) | Acesso direto à API Claude de qualquer uma das linguagens do Client SDK. Você escreve o loop de ferramentas você mesmo, ou deixa o [tool runner](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-runner) beta do Client SDK conduzi-lo.                                                                                                                                            |
| Ter a Anthropic hospedando o agente, configurado através da API Claude                                       | [**Managed Agents**](https://platform.claude.com/docs/en/managed-agents/overview) | Um agente hospedado que executa o loop do agente, com sessões em um sandbox gerenciado pela Anthropic ou um [sandbox auto-hospedado](https://platform.claude.com/docs/en/managed-agents/self-hosted-sandboxes) em sua própria infraestrutura. Use-o a partir do [SDK para sua linguagem](https://platform.claude.com/docs/en/managed-agents/quickstart#install-the-sdk), da CLI `ant` ou da API REST. |

Para conduzir o mesmo loop de agente de uma linguagem diferente de Python ou TypeScript, [execute a CLI como um subprocesso](/docs/pt/headless) com a flag `-p` e `--output-format json`.

<h2 id="capabilities">
  Capacidades
</h2>

Essas capacidades do Claude Code estão disponíveis no SDK:

| Capacidade                 | O que faz                                                                                     | Saiba mais                                                                                                                                                                                                             |
| -------------------------- | --------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ferramentas integradas     | Ler, escrever, editar arquivos, executar comandos e pesquisar na web                          | [Referência de ferramentas](/docs/pt/tools-reference)                                                                                                                                                                       |
| Hooks                      | Executar código personalizado em pontos-chave do ciclo de vida do agente                      | [Hooks](/docs/pt/agent-sdk/hooks)                                                                                                                                                                                           |
| Subagentes                 | Gerar agentes especializados para subtarefas focadas                                          | [Subagentes](/docs/pt/agent-sdk/subagents)                                                                                                                                                                                  |
| MCP                        | Conectar ferramentas externas e fontes de dados via o Model Context Protocol                  | [MCP](/docs/pt/agent-sdk/mcp)                                                                                                                                                                                               |
| Permissões                 | Controlar quais ferramentas são executadas automaticamente, quais precisam de aprovação       | [Permissões](/docs/pt/agent-sdk/permissions)                                                                                                                                                                                |
| Sessões                    | Manter contexto entre trocas, retomar ou bifurcar depois                                      | [Sessões](/docs/pt/agent-sdk/sessions)                                                                                                                                                                                      |
| Skills, comandos e memória | Carregar automaticamente do `.claude/` do seu projeto e de `~/.claude/`, igual ao Claude Code | [Skills](/docs/pt/agent-sdk/skills), [Comandos](/docs/pt/agent-sdk/skills#commands-in-agent-sdk-sessions), [Memória](/docs/pt/agent-sdk/modifying-system-prompts), [Carregamento de configuração](/docs/pt/agent-sdk/claude-code-features) |
| Plugins                    | Empacotar skills, agentes, hooks e servidores MCP, e carregá-los por caminho local            | [Plugins](/docs/pt/agent-sdk/plugins)                                                                                                                                                                                       |

<h2 id="get-started">
  Comece agora
</h2>

Siga o [Quickstart](/docs/pt/agent-sdk/quickstart) para instalar o SDK, definir sua chave de API e construir seu primeiro agente, um que encontra e corrige bugs em código existente.

<Note>
  A menos que previamente aprovado, a Anthropic não permite que desenvolvedores terceirizados ofereçam login claude.ai ou limites de taxa para seus produtos, incluindo agentes construídos no Claude Agent SDK. Use os métodos de autenticação de chave de API descritos no [Quickstart](/docs/pt/agent-sdk/quickstart) em vez disso.
</Note>

<h2 id="changelog">
  Changelog
</h2>

Veja o changelog completo para atualizações do SDK, correções de bugs e novos recursos:

* **TypeScript SDK**: [ver CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **Python SDK**: [ver CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="report-bugs">
  Relatando bugs
</h2>

Se você encontrar bugs ou problemas com o Agent SDK:

* **TypeScript SDK**: [relatar problemas no GitHub](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **Python SDK**: [relatar problemas no GitHub](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  Diretrizes de marca
</h2>

Para parceiros integrando o Claude Agent SDK, o uso de marca Claude é opcional. Ao fazer referência a Claude em seu produto:

**Permitido:**

* "Claude Agent", preferido para menus suspensos
* "Claude", quando dentro de um menu já rotulado "Agents"
* "\{YourAgentName} Powered by Claude", se você tiver um nome de agente existente

**Não permitido:**

* "Claude Code" ou "Claude Code Agent"
* Arte ASCII com marca Claude Code ou elementos visuais que imitam Claude Code

Seu produto deve manter sua própria marca e não parecer ser Claude Code ou qualquer produto Anthropic. Para perguntas sobre conformidade de marca, entre em contato com a [equipe de vendas](https://www.anthropic.com/contact-sales) da Anthropic.

<h2 id="license-and-terms">
  Licença e termos
</h2>

O uso do Claude Agent SDK é regido pelos [Termos de Serviço Comercial da Anthropic](https://www.anthropic.com/legal/commercial-terms), incluindo quando você o usa para alimentar produtos e serviços que você disponibiliza para seus próprios clientes e usuários finais, exceto na medida em que um componente específico ou dependência seja coberto por uma licença diferente conforme indicado no arquivo LICENSE desse componente.

<h2 id="next-steps">
  Próximos passos
</h2>

Estes recursos cobrem detalhes técnicos mais profundos e projetos de exemplo para construir com o Agent SDK.

* [Guia de Início Rápido](/docs/pt/agent-sdk/quickstart): construa seu primeiro agente que encontra e corrige bugs
* [Guia de migração](/docs/pt/agent-sdk/migration-guide): migre dos pacotes Claude Code SDK para o Agent SDK
* [Loop do agente](/docs/pt/agent-sdk/agent-loop): como Claude planeja, chama ferramentas e decide quando uma tarefa está concluída
* [Agentes de exemplo](https://github.com/anthropics/claude-agent-sdk-demos): aplicativos de demonstração para desenvolvimento local
* [TypeScript SDK](/docs/pt/agent-sdk/typescript): referência completa da API TypeScript e exemplos
* [Python SDK](/docs/pt/agent-sdk/python): referência completa da API Python e exemplos
* [Design do harness do agente](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code): como o time Claude Code usa fluxos de trabalho dinâmicos para orquestrar muitos subagentos simultaneamente
