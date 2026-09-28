> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Checkpointing

> Rastreie, reverta e resuma as edições e conversas do Claude para gerenciar o estado da sessão.

Claude Code rastreia automaticamente as edições de arquivo do Claude conforme você trabalha, permitindo que você desfaça rapidamente as alterações e reverta para estados anteriores se algo sair do caminho.

<h2 id="how-checkpoints-work">
  Como o checkpointing funciona
</h2>

Conforme você trabalha com Claude, o checkpointing captura automaticamente o estado do seu código antes de cada prompt que você envia e que inicia um turno.

<h3 id="automatic-tracking">
  Rastreamento automático
</h3>

Claude Code rastreia todas as alterações feitas por suas ferramentas de edição de arquivo:

* Cada prompt que você envia e que inicia um turno cria um novo checkpoint
* Claude Code mantém snapshots de arquivo para os 100 checkpoints mais recentes em uma sessão. Descartar um checkpoint mais antigo deleta os arquivos de snapshot que nenhum checkpoint restante referencia, exceto o primeiro snapshot de cada arquivo, que a extensão VS Code usa como baseline para seus diffs de sessão.
* Claude Code salva checkpoints com a conversa, para que você ainda possa executar `/rewind` após retomar uma sessão
* Claude Code deleta os snapshots de arquivo de uma sessão na [varredura de retenção](/docs/pt/claude-directory#cleaned-up-automatically), por padrão cerca de 30 dias após a sessão salvar um pela última vez. Fazer rewind para um checkpoint cujos snapshots desapareceram pode falhar com [`Nenhum arquivo foi restaurado`](/docs/pt/errors#no-files-were-restored). Para manter snapshots por mais tempo, defina [`cleanupPeriodDays`](/docs/pt/settings-reference#cleanupperioddays).

<h3 id="rewind-and-summarize">
  Rewind e resumo
</h3>

Execute `/rewind`, ou pressione `Esc` duas vezes quando o campo de entrada de prompt estiver vazio, para abrir o menu de rewind.

<Note>
  Se o campo de entrada de prompt contiver texto, duplo `Esc` o limpa em vez de abrir o menu. O texto limpo é salvo no seu histórico de entrada, então pressione `Up` para recuperá-lo após terminar no menu de rewind.
</Note>

O menu de rewind lista cada prompt que você enviou durante a sessão, exceto [mensagens que se juntaram a um turno em andamento](#messages-sent-mid-turn-not-checkpointed). Selecione o ponto em que deseja agir e escolha uma ação:

* **Restaurar código e conversa**: reverte tanto o código quanto a conversa para esse ponto
* **Restaurar conversa**: reverte para essa mensagem mantendo o código atual
* **Restaurar código**: reverte as alterações de arquivo mantendo a conversa
* **Resumir a partir daqui**: compacta a conversa a partir deste ponto em diante em um resumo, liberando espaço da context window
* **Resumir até aqui**: compacta a conversa antes deste ponto em um resumo, mantendo as mensagens posteriores intactas
* **Nunca importa**: retorna à lista de mensagens sem fazer alterações

As duas opções de restauração de código aparecem apenas quando o checkpoint selecionado tem alterações de arquivo rastreadas para reverter. Se nenhuma edição de arquivo foi capturada após esse ponto, o menu oferece apenas **Restaurar conversa**, as opções de resumo e **Nunca importa**.

Após restaurar a conversa ou escolher Resumir a partir daqui, o prompt original da mensagem selecionada é restaurado no campo de entrada para que você possa reenviá-lo ou editá-lo.

Escolher Resumir até aqui o deixa no final da conversa com a entrada vazia. Com qualquer opção de resumo, um marcador **Conversa resumida** aparece na conversa onde as mensagens compactadas estavam.

<h4 id="rewind-past-a-cleared-conversation">
  Rewind passado uma conversa limpa
</h4>

Se você executou `/clear` anteriormente no mesmo processo Claude Code, o menu de rewind mostra uma entrada adicional no topo da lista rotulada `/resume <session-id> (sessão anterior)`. Selecione-a para retomar a conversa que estava ativa antes de `/clear` ser executado. A entrada está disponível até você sair do Claude Code ou retomar uma sessão diferente.

<h4 id="guide-a-summary">
  Guiar um resumo
</h4>

Resumir não altera arquivos no disco, e as mensagens originais permanecem na transcrição da sessão, para que Claude ainda possa fazer referência aos detalhes. Para guiar o que o resumo se concentra, destaque uma opção **Resumir** com as teclas de seta e digite instruções onde a linha lê **adicionar contexto (opcional)**, então pressione `Enter`. Selecionar a opção com sua tecla de número resume imediatamente sem instruções.

<Note>
  Resumir mantém você na mesma sessão e compacta o contexto, como um `/compact` direcionado. Para ramificar e tentar uma abordagem diferente enquanto preserva a sessão original intacta, use [`/branch`](/docs/pt/sessions#branch-a-session) ou `claude --continue --fork-session` em vez disso.
</Note>

<h2 id="common-use-cases">
  Casos de uso comuns
</h2>

Os checkpoints são particularmente úteis quando:

* **Explorando alternativas**: tente diferentes abordagens de implementação sem perder seu ponto de partida
* **Recuperando de erros**: desfaça rapidamente as alterações que introduziram bugs ou quebraram a funcionalidade
* **Iterando em recursos**: experimente variações sabendo que você pode reverter para estados funcionais
* **Liberando espaço de contexto**: resuma uma sessão de depuração verbosa a partir do ponto médio em diante, mantendo suas instruções iniciais intactas

<h2 id="limitations">
  Limitações
</h2>

<h3 id="bash-command-changes-not-tracked">
  Alterações de comando Bash não rastreadas
</h3>

O checkpointing não rastreia arquivos modificados por comandos Bash. Por exemplo, se Claude Code executar:

```bash theme={null}
rm file.txt
mv old.txt new.txt
cp source.txt dest.txt
```

Essas modificações de arquivo não podem ser desfeitas através de rewind. Apenas edições diretas de arquivo feitas através das ferramentas de edição de arquivo do Claude são rastreadas.

<h3 id="subagent-edits-not-restored">
  Edições de subagent não restauradas
</h3>

Um [subagent](/docs/pt/sub-agents) faz edições com as ferramentas de edição de arquivo do Claude, mas Claude Code geralmente não captura essas edições nos checkpoints da sua sessão. Se o rewind restaura essas edições depende de como o subagent é executado:

* **Skill forked em foreground**: uma [skill com `context: fork`](/docs/pt/skills#run-skills-in-a-subagent) que é executada em foreground edita sua árvore de trabalho durante seu próprio turno, então o rewind restaura suas edições como de costume. Defina `background: false` para executar um fork em foreground; algumas situações, [listadas na página de skills](/docs/pt/skills#run-skills-in-a-subagent), o executam lá independentemente da configuração.
* **Qualquer outro subagent**: o rewind não restaura as edições. Use git para revertê-las. Isso inclui uma skill forked que é executada em background, o padrão, e uma execução de [`/code-review --fix`](/docs/pt/code-review) em background.

<h3 id="external-changes-not-tracked">
  Alterações externas não rastreadas
</h3>

O checkpointing rastreia apenas arquivos que foram editados na sessão atual. Alterações manuais que você faz em arquivos fora do Claude Code e edições de outras sessões simultâneas normalmente não são capturadas, a menos que aconteçam de modificar os mesmos arquivos da sessão atual.

<h3 id="messages-sent-mid-turn-not-checkpointed">
  Mensagens enviadas no meio do turno não checkpointed
</h3>

Quando uma mensagem que você [enfileira enquanto Claude trabalha](/docs/pt/interactive-mode#queue-messages-while-claude-works) chega ao Claude dentro do turno em execução, ela se junta a esse turno em vez de iniciar um novo. A mensagem aparece na conversa, mas Claude Code não cria um checkpoint para ela, e o menu de rewind não a lista. Uma mensagem enfileirada que Claude Code envia como seu próprio turno recebe um checkpoint como de costume.

Para remover tal mensagem, ou desfazer as edições que Claude fez depois dela, faça rewind para o prompt que iniciou o turno. Isso faz rewind de todo o turno, incluindo o trabalho que Claude fez antes de sua mensagem chegar.

<h3 id="symlinked-and-hard-linked-paths-not-restored">
  Caminhos symlinked e hard-linked não restaurados
</h3>

O checkpointing não faz rewind de arquivos symlinked ou hard-linked. Quando você seleciona **Restore code** ou **Restore code and conversation** no menu `/rewind`, Claude Code pula qualquer caminho rastreado que seja um symlink ou hard link e mostra um aviso `Restored the code, but skipped N files`. Os arquivos pulados mantêm seu conteúdo atual. Para desfazer as alterações da sessão em um deles, peça ao Claude para reverter a edição ou edite o arquivo você mesmo. Arquivos de configuração que um gerenciador de dotfiles symlinks em seu projeto e arquivos que pnpm hard-links em seu lugar ambos se enquadram nesta categoria.

Para ver quais caminhos um restore pula, ative o debug logging com `/debug` antes de restaurar: o debug log em `~/.claude/debug/<session-id>.txt` nomeia cada caminho pulado. Para cada razão de skip e as etapas de recuperação, veja [a entrada skipped-files na referência de erros](/docs/pt/errors#restored-the-code-but-skipped-files).

<h3 id="not-a-replacement-for-version-control">
  Não é um substituto para controle de versão
</h3>

Os checkpoints são projetados para recuperação rápida no nível da sessão. Para histórico de versão permanente e colaboração, continue usando controle de versão, como Git, para commits, branches e histórico de longo prazo.

<h2 id="see-also">
  Veja também
</h2>

* [Modo interativo](/docs/pt/interactive-mode) - Atalhos de teclado e controles de sessão
* [Comandos](/docs/pt/commands) - Acessando checkpoints usando `/rewind`
* [Referência CLI](/docs/pt/cli-reference) - Opções de linha de comando
