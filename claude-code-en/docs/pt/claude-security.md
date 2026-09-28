> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Digitalize seu código em busca de vulnerabilidades

> Instale o plugin Claude Security para digitalizar seu código em busca de vulnerabilidades em uma sessão Claude Code e transforme as descobertas em patches que você revisa e aplica.

O plugin Claude Security executa uma digitalização de vulnerabilidades multi-agente de seu código em uma sessão Claude Code. Uma equipe de agentes Claude mapeia sua arquitetura, constrói um modelo de ameaça, procura por vulnerabilidades e revisa independentemente cada descoberta antes de escrever o relatório. Use o plugin para digitalizar um repositório inteiro ou [apenas um conjunto de alterações](#scan-only-your-changes), como o diff de uma branch, o diff de uma solicitação de pull ou um único commit, depois transforme as descobertas que você escolher em patches que você revisa e aplica você mesmo.

O plugin é executado localmente em sua sessão, usa quaisquer modelos aos quais você tenha acesso no Claude Code, e cada digitalização conta contra os limites de uso do seu plano. Se você deseja um serviço gerenciado que monitore seus repositórios, ou deseja executar digitalizações no [Claude Mythos 5](https://platform.claude.com/docs/en/about-claude/models/introducing-claude-fable-5-and-claude-mythos-5), consulte o produto [Claude Security](https://claude.com/product/claude-security), disponível no plano Enterprise. O plugin alcança código que o produto gerenciado não consegue alcançar, como repositórios hospedados no GitLab ou Bitbucket, ou em redes que não permitem conexões de entrada.

O plugin também é distinto das ferramentas de revisão já presentes no Claude Code: o [plugin de orientação de segurança](/docs/pt/security-guidance) revisa o código conforme Claude o escreve, [`/security-review`](/docs/pt/commands#all-commands) executa uma única passagem em sua branch, e [Code Review](/docs/pt/code-review) revisa solicitações de pull. Para saber como as camadas se empilham, consulte [Como o plugin se encaixa com outras ferramentas de segurança](#how-the-plugin-fits-with-other-security-tools).

<h2 id="prerequisites">
  Pré-requisitos
</h2>

Para executar o plugin, você precisa de:

* Um plano pago, para os [fluxos de trabalho dinâmicos](/docs/pt/workflows) que a digitalização usa para orquestrar seus agentes. No Pro, ative-os a partir da linha Dynamic workflows em `/config`.
* Python 3.9 ou posterior disponível em seu `PATH` como `python3`. Verifique com `python3 --version`. A ferramenta do plugin usa apenas a biblioteca padrão do Python, portanto nada é instalado.
* Linux, macOS ou Windows.
* Git, para digitalizações de alterações e para transformar descobertas em patches; esses trabalhos não suportam outros sistemas de controle de versão. Uma digitalização completa funciona em qualquer diretório, com ou sem controle de versão.

<h2 id="install-the-plugin">
  Instale o plugin
</h2>

Em uma sessão Claude Code, instale a partir do [marketplace oficial da Anthropic](/docs/pt/plugins/anthropic-marketplaces):

```text theme={null}
/plugin install claude-security@claude-plugins-official
```

O comando abre os detalhes do plugin, onde você escolhe um [escopo de instalação](/docs/pt/plugins/install#install-a-plugin) para iniciar a instalação.

Se a instalação falhar, a correção depende de qual mensagem Claude Code relata:

* Se relatar `Marketplace "claude-plugins-official" not found`, adicione o marketplace com `/plugin marketplace add anthropics/claude-plugins-official`, depois tente novamente a instalação.
* Se relatar que não consegue [encontrar o plugin no marketplace](/docs/pt/plugins/install#install-a-plugin), verifique o nome do plugin para um erro de digitação.

Verifique o resumo da instalação. Se relatar `Run /reload-plugins to activate.`, consulte [Aplicar alterações de plugin sem reiniciar](/docs/pt/plugins/cli-reference#reload-plugins) para ativar o plugin em sua sessão atual.

Assim que o plugin estiver ativo, você está pronto para [digitalizar e corrigir seu repositório de código](#scan-and-fix-your-codebase).

<h3 id="uninstall-the-plugin">
  Desinstale o plugin
</h3>

Para remover o plugin, desinstale-o do menu `/plugin`, ou execute `claude plugin uninstall claude-security` em seu terminal.

<h2 id="scan-and-fix-your-codebase">
  Digitalize e corrija seu repositório de código
</h2>

O plugin adiciona um comando, `/claude-security`, que abre um menu de seus três trabalhos: digitalizar o repositório de código, digitalizar um conjunto de alterações e sugerir patches. O caminho feliz executa uma digitalização completa e depois transforma suas descobertas em patches:

<Steps>
  <Step title="Abra o menu Claude Security">
    Execute `/claude-security` e escolha **Scan codebase**.
  </Step>

  <Step title="Escolha o que digitalizar">
    O plugin lê seu repositório primeiro, depois oferece o repositório inteiro ou uma área focada, com a contagem de arquivos e custo relativo de cada opção indicados. Escolha o repositório inteiro, ou responda "I don't know" e o plugin escolhe um padrão sensato para o tamanho do seu repositório.
  </Step>

  <Step title="Confirme a execução">
    Uma digitalização pode levar um tempo, pode usar um número significativo de tokens e precisa que Claude Code permaneça aberto enquanto é concluída. Nada é executado até você confirmar.
  </Step>

  <Step title="Leia o relatório">
    Enquanto a digitalização é executada, ela relata cada estágio conforme começa, com o detalhe disponível em [`/workflows`](/docs/pt/workflows). Os resultados chegam em um diretório com timestamp em seu repositório, descrito em [Leia os resultados da digitalização](#read-the-scan-results).
  </Step>

  <Step title="Transforme descobertas em patches">
    Execute `/claude-security` novamente e escolha **Suggest patches**, depois escolha quais descobertas abordar. Os patches revisados chegam na pasta `patches/` do relatório; [Corrija descobertas](#fix-findings) cobre como cada patch é construído e revisado.
  </Step>

  <Step title="Aplique os patches que você aceita">
    Aplique cada patch do seu shell com `git apply`, em seu próprio pull request. Os patches nunca são aplicados automaticamente.
  </Step>
</Steps>

Você não precisa começar pelo menu: peça um trabalho diretamente, como argumentos para o comando, como `/claude-security scan my branch`, ou em linguagem simples, como "scan commit abc1234". O plugin funciona melhor em [modo automático](/docs/pt/permission-modes), que permite que os agentes da digitalização prossigam sem um prompt de permissão em cada etapa.

<h3 id="scan-only-your-changes">
  Digitalize apenas suas alterações
</h3>

Quando sua branch tem commits que sua base não tem, o menu `/claude-security` oferece digitalizar apenas esse diff, para que você possa verificar uma branch antes de fazer merge. Você também pode digitalizar um de seus pull requests abertos, ou um único commit pedindo por ele, como "scan commit abc1234". Apenas alterações confirmadas são digitalizadas: confirme ou faça stash de edições em andamento primeiro, ou execute uma digitalização completa, que lê a árvore de trabalho.

Digitalizações de alterações precisam de um repositório git; digitalizações completas de um diretório sem versão ainda funcionam. Encontrar seus pull requests abertos é o único passo que alcança a rede, e é oferecido apenas quando sua sessão já tem permissão para executar a CLI do GitHub e `gh` está conectado.

<h3 id="scope-large-repositories">
  Escopo de repositórios grandes
</h3>

Em um repositório grande, digitalize uma área por vez em vez de toda a árvore. Escolha um dos escopos focados que o plugin oferece, como sua camada de API ou seu código de autenticação, e a execução se dimensiona para o que você escolher. A seção de cobertura do relatório indica o que foi e o que não foi examinado. Execute outra digitalização em uma área diferente a qualquer momento.

<h3 id="read-the-scan-results">
  Leia os resultados da digitalização
</h3>

Cada digitalização escreve seus resultados em um diretório `CLAUDE-SECURITY-<timestamp>/` com timestamp em seu repositório:

* **`CLAUDE-SECURITY-RESULTS.md`**: o relatório, com o ID de cada descoberta, como `F1`, além de seu impacto, cenário de exploração, severidade, confiança e recomendação
* **`CLAUDE-SECURITY-RESULTS.jsonl`**: as mesmas descobertas em forma legível por máquina, um objeto JSON por linha
* **`CLAUDE-SECURITY-RESULTS.sarif`**: as mesmas descobertas como um log [SARIF 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html) para varredura de código do GitHub e qualquer outra ferramenta que leia o padrão. A digitalização classifica descobertas sob suas categorias de fraqueza [CWE](https://cwe.mitre.org/)
* **`CLAUDE-SECURITY-REVISION-<commit>.json`**: o carimbo de revisão, registrando qual commit foi digitalizado, com qual esforço, se alterações não confirmadas faziam parte da árvore digitalizada e quão completamente a execução foi verificada, para que um relatório sempre esteja vinculado ao código que descreve. Uma digitalização fora do controle de versão carimba `UNVERSIONED` no lugar do commit

Esse diretório é a única alteração que uma digitalização faz em seu checkout, e ele carrega seu próprio `.gitignore`, para que um `git add` perdido nunca varre um relatório para um commit. Para manter um relatório no histórico para uma trilha de auditoria, delete esse único arquivo `.gitignore` e confirme o diretório como qualquer outro.

As descobertas aparecem no relatório apenas após agentes verificadores independentes as analisarem, o que mantém os relatórios curtos e vale a pena ler. Digitalizações são não determinísticas: duas digitalizações do mesmo código podem descobrir diferentes descobertas. Execute digitalizações regularmente e use os carimbos de revisão para atribuir cada relatório ao código exato e às configurações que cobriu.

<h2 id="fix-findings">
  Corrija descobertas
</h2>

Inicie o fluxo de correção escolhendo **Suggest patches** no menu `/claude-security`, ou peça em linguagem simples, como "fix finding F3", depois escolha quais descobertas do relatório abordar. Os patches são construídos contra código confirmado, e o relatório tem que ainda descrever o código que você tem: descobertas cujo código mudou desde então são puladas com uma nota, e o plugin oferece uma digitalização fresca em vez de fazer patch de um relatório obsoleto. Cada patch é rascunhado em uma cópia de rascunho do seu repositório, para que seus arquivos de origem permaneçam intocados até você aplicar um patch você mesmo.

Antes da entrega, cada patch é revisado por um agente independente do que o escreveu, que executa os testes do seu projeto contra a alteração quando o código os tem e lê o diff por seus próprios termos para qualquer coisa nova que possa introduzir. Um patch é escrito apenas quando essa revisão pode garantir que a alteração aborda a descoberta, não introduz nenhuma nova vulnerabilidade e deixa o comportamento inalterado. Quando não consegue garantir todos os três, você recebe uma nota curta explicando por que em vez de um patch.

<h3 id="patches-are-never-applied-automatically">
  Os patches nunca são aplicados automaticamente
</h3>

Aplicar um patch é sempre sua decisão. Os patches chegam na pasta `patches/` do relatório, um `F<n>.patch` por descoberta com uma nota ao lado explicando a alteração. Aplique um do seu shell, ou peça a Claude para aplicá-lo e abrir um pull request:

```bash theme={null}
git apply CLAUDE-SECURITY-<timestamp>/patches/F1.patch
```

Quando o código com patch não tem testes, a nota do patch diz isso, para que você saiba que sua revisão foi executada sem uma passagem de teste. Aplique cada patch em seu próprio pull request para que possa ser revisado e testado por conta própria.

<h2 id="how-the-plugin-fits-with-other-security-tools">
  Como o plugin se encaixa com outras ferramentas de segurança
</h2>

O plugin Claude Security é a camada de digitalização profunda sob demanda em uma pilha de defesa em profundidade, ao lado do [plugin de orientação de segurança](/docs/pt/security-guidance), [`/security-review`](/docs/pt/commands#all-commands), [Code Review](/docs/pt/code-review), o produto [Claude Security](https://claude.com/product/claude-security) gerenciado e seus scanners existentes:

| Estágio                             | Ferramenta                                                                      | O que cobre                                                                                                          |
| :---------------------------------- | :------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------- |
| Na sessão                           | [Plugin de orientação de segurança](/docs/pt/security-guidance)                      | Vulnerabilidades comuns no código que Claude escreve, corrigidas na mesma sessão                                     |
| Sob demanda, passagem única         | [`/security-review`](/docs/pt/commands#all-commands)                                 | Uma passagem de segurança única na branch atual                                                                      |
| Sob demanda, digitalização profunda | Plugin Claude Security                                                          | Digitalização com múltiplos agentes de um repositório ou diff, com descobertas e patches revisados independentemente |
| No pull request                     | [Code Review](/docs/pt/code-review), planos Team e Enterprise                        | Revisão de correção e segurança com múltiplos agentes com contexto completo do repositório                           |
| Gerenciado                          | [Claude Security](https://claude.com/product/claude-security), plano Enterprise | Digitalização hospedada que monitora repositórios conectados                                                         |
| Em CI                               | Seus scanners de análise estática e dependência existentes                      | Regras específicas de linguagem, verificações de cadeia de suprimentos e aplicação de política                       |

O plugin não substitui suas ferramentas de segurança de código-fonte existentes. Execute-o ao lado de análise estática, digitalização de dependência e revisão de código: ele raciocina sobre seu código da forma como um pesquisador de segurança humano faria, o que complementa as verificações determinísticas que essas ferramentas fornecem.

<h2 id="troubleshooting">
  Solução de problemas
</h2>

**O menu `/claude-security` abre com um aviso do Python.** O plugin precisa de `python3` 3.9 ou posterior em seu `PATH`. Quando não consegue encontrar `python3` em tudo, o menu avisa que Claude Security não funcionará até que um seja instalado; quando o primeiro `python3` em seu `PATH` é mais antigo, o aviso nomeia a versão que encontrou. Instale Python 3, ou coloque um `python3` mais novo primeiro em seu `PATH`, depois inicie uma nova sessão.

**Você pode ver um aviso "safeguards flagged this message" ao digitalizar em um modelo Fable.** A mensagem nomeia o modelo, por exemplo "Fable 5.1's safeguards flagged this message". Os classificadores de segurança cibernética do Fable sinalizam certas solicitações, e Claude Code re-executa uma solicitação sinalizada em um modelo Opus através do [fallback automático de modelo](/docs/pt/model-config#automatic-model-fallback). Isso é esperado, e a digitalização ainda deve ser concluída com sucesso.

<h2 id="related-resources">
  Recursos relacionados
</h2>

Para aprofundar nos tópicos que esta página toca:

* [Plugin de orientação de segurança](/docs/pt/security-guidance): capture problemas no código conforme Claude o escreve, na mesma sessão
* [Code Review](/docs/pt/code-review): configure a revisão com múltiplos agentes no tempo de PR
* [Claude Security](https://claude.com/product/claude-security): o serviço gerenciado que monitora repositórios conectados
* [Segurança do Claude Code](/docs/pt/security): como Claude Code aborda confiança, permissões e salvaguardas
* [Instale e gerencie plugins](/docs/pt/plugins/install): encontre e instale outros plugins do marketplace oficial
