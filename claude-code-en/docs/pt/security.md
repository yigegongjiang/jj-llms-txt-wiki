> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Segurança

> Aprenda sobre as proteções de segurança do Claude Code e as melhores práticas para uso seguro.

<h2 id="how-we-approach-security">
  Como abordamos a segurança
</h2>

<h3 id="security-foundation">
  Fundação de segurança
</h3>

A segurança do seu código é fundamental. Claude Code é construído com segurança em seu núcleo, desenvolvido de acordo com o programa de segurança abrangente da Anthropic. Saiba mais e acesse recursos (relatório SOC 2 Type 2, certificado ISO 27001, etc.) no [Anthropic Trust Center](https://trust.anthropic.com).

<h3 id="permission-based-architecture">
  Arquitetura baseada em permissões
</h3>

Claude Code usa permissões somente leitura rigorosas por padrão. Quando ações adicionais são necessárias (editar arquivos, executar testes, executar comandos), Claude Code solicita permissão explícita. Os usuários controlam se devem aprovar ações uma única vez ou permitir automaticamente.

Claude Code requer aprovação antes de executar comandos Bash que podem modificar seu sistema. Um conjunto integrado de [comandos somente leitura](/docs/pt/permissions#read-only-commands) como `ls`, `cat` e `git status` é executado sem um prompt. Esta abordagem permite que usuários e organizações configurem permissões diretamente.

Para configuração detalhada de permissões, consulte [Permissions](/docs/pt/permissions).

<h3 id="built-in-protections">
  Proteções integradas
</h3>

Para mitigar riscos em sistemas agentic:

* **Ferramenta bash em sandbox**: [Sandbox](/docs/pt/sandboxing) comandos bash com isolamento de sistema de arquivos e rede, reduzindo prompts de permissão enquanto mantém a segurança. Ative com `/sandbox` para definir limites onde Claude Code pode trabalhar autonomamente
* **Restrição de diretório de trabalho**: Claude Code pode escrever apenas na pasta onde foi iniciado e suas subpastas, e não pode modificar arquivos em diretórios pai sem permissão explícita. Ler caminhos fora deste limite com as ferramentas Read, Grep e Glob é possível após um prompt de aprovação. Estenda o limite com [diretórios adicionais](/docs/pt/permissions#working-directories) para pular o prompt, ou restrinja o acesso de leitura mais amplo disponível para comandos Bash somente leitura com [regras de sandbox `denyRead`](/docs/pt/sandboxing#filesystem-isolation), que se aplicam apenas quando o sandboxing está ativado
* **Mitigação de fadiga de prompt**: Suporte para lista de permissões de comandos seguros frequentemente usados por usuário, por base de código ou por organização
* **Modo Accept Edits**: Aprova automaticamente edições de arquivo e um conjunto fixo de comandos Bash do sistema de arquivos como `mkdir`, `touch`, `rm`, `mv`, `cp` e `sed` para caminhos no diretório de trabalho. Outros comandos Bash e caminhos fora do escopo ainda solicitam aprovação

<h3 id="user-responsibility">
  Responsabilidade do usuário
</h3>

Claude Code tem apenas as permissões que você concede. Você é responsável por revisar código e comandos propostos quanto à segurança antes da aprovação.

<h2 id="protect-against-prompt-injection">
  Proteja-se contra injeção de prompt
</h2>

Injeção de prompt é uma técnica onde um atacante tenta substituir ou manipular as instruções de um assistente de IA inserindo texto malicioso. Claude Code inclui várias proteções contra esses ataques:

<h3 id="core-protections">
  Proteções principais
</h3>

* **Sistema de permissões**: Operações sensíveis requerem aprovação explícita
* **Análise com reconhecimento de contexto**: Detecta instruções potencialmente prejudiciais analisando a solicitação completa
* **Sanitização de entrada**: Previne injeção de comando processando entradas do usuário
* **Aprovação de comando de rede**: Comandos que buscam conteúdo da web como `curl` e `wget` não são aprovados automaticamente por padrão. Eles solicitam aprovação como qualquer outro comando Bash não somente leitura, portanto você ainda pode aprovar uma vez ou adicionar uma regra de permissão explícita como `Bash(curl *)`. Para bloqueá-los completamente, adicione-os a [`permissions.deny`](/docs/pt/permissions#tool-specific-permission-rules)

<h3 id="privacy-safeguards">
  Proteções de privacidade
</h3>

Implementamos várias proteções para proteger seus dados, incluindo:

* Períodos de retenção limitados para informações sensíveis (consulte o [Privacy Center](https://privacy.anthropic.com/en/articles/10023548-how-long-do-you-store-my-data) para saber mais)
* Acesso restrito aos dados de sessão do usuário
* Controle do usuário sobre preferências de treinamento de dados. Usuários consumidores podem alterar suas [configurações de privacidade](https://claude.ai/settings/privacy) a qualquer momento.

Para detalhes completos, consulte nossos [Termos de Serviço Comerciais](https://www.anthropic.com/legal/commercial-terms) (para usuários de Team, Enterprise e API) ou [Termos de Consumidor](https://www.anthropic.com/legal/consumer-terms) (para usuários de Free, Pro e Max) e [Política de Privacidade](https://www.anthropic.com/legal/privacy).

<h3 id="additional-safeguards">
  Proteções adicionais
</h3>

* **Aprovação de solicitação de rede**: Ferramentas que fazem solicitações de rede requerem aprovação do usuário por padrão
* **Janelas de contexto isoladas**: Web fetch usa uma janela de contexto separada para evitar injetar prompts potencialmente maliciosos
* **Verificação de confiança**: Primeiras execuções de base de código e novos MCP servers requerem verificação de confiança
  * Nota: A verificação de confiança é desabilitada ao executar de forma não interativa com a flag `-p`
  * Nota: Quando você inicia Claude Code diretamente no seu diretório inicial, a aceitação de confiança é mantida apenas para a sessão atual e não é gravada em disco, portanto o prompt reaparece a cada inicialização. Não há configuração para persistir isso. Inicie Claude Code a partir de um subdiretório do projeto, onde a aceitação de confiança é salva por diretório
* **Detecção de injeção de comando**: Comandos bash suspeitos requerem aprovação manual mesmo se previamente permitidos
* **Correspondência fail-closed**: Comandos não correspondidos padrão para exigir aprovação manual
* **Descrições em linguagem natural**: Comandos bash complexos incluem explicações para compreensão do usuário
* **Armazenamento seguro de credenciais**: Chaves de API e tokens são armazenados no Keychain do macOS quando disponível, e protegidos por permissões de arquivo no Windows e Linux. Consulte [Credential Management](/docs/pt/authentication#credential-management)

<Warning>
  **Risco de segurança do WebDAV no Windows**: Ao executar Claude Code no Windows, recomendamos contra ativar WebDAV ou permitir que Claude Code acesse caminhos como `\\*` que podem conter subdiretórios WebDAV. [WebDAV foi descontinuado pela Microsoft](https://learn.microsoft.com/en-us/windows/whats-new/deprecated-features#:~:text=The%20Webclient%20\(WebDAV\)%20service%20is%20deprecated) devido a riscos de segurança. Ativar WebDAV pode permitir que Claude Code dispare solicitações de rede para hosts remotos, contornando o sistema de permissões.
</Warning>

**Melhores práticas para trabalhar com conteúdo não confiável**:

1. Revise comandos sugeridos antes da aprovação
2. Evite canalizar conteúdo não confiável diretamente para Claude
3. Verifique alterações propostas em arquivos críticos
4. Use máquinas virtuais (VMs) para executar scripts e fazer chamadas de ferramentas, especialmente ao interagir com serviços web externos
5. Relate comportamento suspeito com `/feedback`

<Warning>
  Embora essas proteções reduzam significativamente o risco, nenhum sistema é
  completamente imune a todos os ataques. Sempre mantenha boas práticas de
  segurança ao trabalhar com qualquer ferramenta de IA.
</Warning>

<h2 id="mcp-security">
  Segurança do MCP
</h2>

Claude Code permite que os usuários configurem servidores Model Context Protocol (MCP). A lista de servidores MCP permitidos é configurada no seu código-fonte, como parte das configurações do Claude Code que os engenheiros verificam no controle de versão.

Encorajamos escrever seus próprios servidores MCP ou usar servidores MCP de provedores em que você confia. Você é capaz de configurar permissões do Claude Code para servidores MCP. Anthropic analisa conectores em relação aos seus [critérios de listagem](https://claude.com/docs/connectors/building/review-criteria) antes de adicioná-los ao [Diretório Anthropic](https://claude.ai/directory), mas não realiza auditoria de segurança ou gerencia nenhum servidor MCP.

<h2 id="ide-security">
  Segurança do IDE
</h2>

Consulte [VS Code security and privacy](/docs/pt/vs-code#security-and-privacy) para mais informações sobre como executar Claude Code em um IDE.

<h2 id="cloud-execution-security">
  Segurança de execução em nuvem
</h2>

Ao usar [Claude Code on the web](/docs/pt/claude-code-on-the-web), controles de segurança adicionais estão em vigor:

* **Máquinas virtuais isoladas**: Cada sessão em nuvem é executada em uma VM isolada gerenciada pela Anthropic
* **Controles de acesso à rede**: O acesso à rede é limitado por padrão e pode ser configurado para ser desabilitado ou permitir apenas domínios específicos
* **Proteção de credenciais**: A autenticação é tratada através de um proxy seguro que usa uma credencial com escopo dentro do sandbox, que é então traduzida para seu token de autenticação GitHub real
* **Restrições de branch**: Operações de git push são restritas ao branch de trabalho atual
* **Registro de auditoria**: Todas as operações em ambientes em nuvem são registradas para fins de conformidade e auditoria
* **Limpeza automática**: Ambientes em nuvem são automaticamente encerrados após a conclusão da sessão

Para mais detalhes sobre execução em nuvem, consulte [Claude Code on the web](/docs/pt/claude-code-on-the-web).

[Remote Control](/docs/pt/remote-control) as sessões funcionam de forma diferente: a interface web se conecta a um processo Claude Code em execução em sua máquina local. Toda execução de código e acesso a arquivos permanece local, e o tráfego da sessão viaja através da API Anthropic sobre TLS; enquanto conectado, a transcrição da sessão é armazenada nos servidores Anthropic para sincronizar a conversa entre dispositivos, conforme descrito em [Connection and security](/docs/pt/remote-control#connection-and-security). Nenhuma VM em nuvem ou sandboxing está envolvido. A conexão usa múltiplas credenciais de curta duração e escopo estreito, cada uma limitada a um propósito específico e expirando independentemente, para limitar o raio de explosão de qualquer credencial comprometida.

<h2 id="security-best-practices">
  Melhores práticas de segurança
</h2>

<h3 id="working-with-sensitive-code">
  Trabalhando com código sensível
</h3>

* Revise todas as alterações sugeridas antes da aprovação
* Use configurações de permissão específicas do projeto para repositórios sensíveis
* Considere usar [dev containers](/docs/pt/devcontainer) para isolamento adicional
* Audite regularmente suas configurações de permissão com `/permissions`

<h3 id="team-security">
  Segurança da equipe
</h3>

* Use [managed settings](/docs/pt/settings#settings-files) para impor padrões organizacionais
* Compartilhe configurações de permissão aprovadas através do controle de versão
* Treine membros da equipe sobre melhores práticas de segurança
* Monitore o uso do Claude Code através de [métricas OpenTelemetry](/docs/pt/monitoring-usage)
* Audite ou bloqueie alterações de configurações durante sessões com [`ConfigChange` hooks](/docs/pt/hooks#configchange)

<h3 id="reporting-security-issues">
  Relatando problemas de segurança
</h3>

Se você descobrir uma vulnerabilidade de segurança no Claude Code:

1. Não a divulgue publicamente
2. Relate-a através do nosso [programa HackerOne](https://hackerone.com/4f1f16ba-10d3-4d09-9ecc-c721aad90f24/embedded_submissions/new)
3. Inclua etapas detalhadas de reprodução
4. Permita tempo para que abordemos o problema antes da divulgação pública

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Plugin de orientação de segurança](/docs/pt/security-guidance): tenha Claude revisar e corrigir vulnerabilidades em suas próprias alterações de código durante a sessão
* [Ambientes sandbox](/docs/pt/sandbox-environments): compare abordagens de isolamento e escolha uma para seu modelo de ameaça
* [Sandboxing](/docs/pt/sandboxing): isolamento de sistema de arquivos e rede para comandos Bash
* [Permissions](/docs/pt/permissions): configure permissões e controles de acesso
* [Monitoramento de uso](/docs/pt/monitoring-usage): rastreie e audite a atividade do Claude Code
* [Development containers](/docs/pt/devcontainer): ambientes seguros e isolados
* [Anthropic Trust Center](https://trust.anthropic.com): certificações de segurança e conformidade
