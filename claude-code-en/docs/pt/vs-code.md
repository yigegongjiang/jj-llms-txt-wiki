> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Use Claude Code in VS Code

> Instale e configure a extensão Claude Code para VS Code. Obtenha assistência de codificação com IA com diffs inline, @-mentions, revisão de planos e atalhos de teclado.

<img src="https://mintcdn.com/claude-code/-YhHHmtSxwr7W8gy/images/vs-code-extension-interface.jpg?fit=max&auto=format&n=-YhHHmtSxwr7W8gy&q=85&s=300652d5678c63905e6b0ea9e50835f8" alt="Editor VS Code com o painel de extensão Claude Code aberto no lado direito, mostrando uma conversa com Claude" width="2500" height="1155" data-path="images/vs-code-extension-interface.jpg" />

A extensão VS Code fornece uma interface gráfica nativa para Claude Code, integrada diretamente ao seu IDE. Esta é a forma recomendada de usar Claude Code no VS Code.

Com a extensão, você pode revisar e editar os planos do Claude antes de aceitá-los, aceitar automaticamente edições conforme são feitas, @-mencionar arquivos com intervalos de linhas específicas da sua seleção, acessar o histórico de conversas e abrir múltiplas conversas em abas separadas ou janelas.

<h2 id="prerequisites">
  Pré-requisitos
</h2>

Antes de instalar, certifique-se de que você tem:

* VS Code 1.94.0 ou superior
* Uma conta Anthropic: qualquer assinatura paga do Claude (Pro, Max, Team ou Enterprise) ou uma conta Claude Console funciona, e nenhuma chave de API é necessária. Você fará [login](/docs/pt/authentication#log-in-to-claude-code) com essa conta quando abrir a extensão pela primeira vez. Se você acessar Claude através de um provedor de terceiros como Amazon Bedrock ou Google Cloud's Agent Platform, consulte [Use third-party providers](#use-third-party-providers) para instruções de configuração.

<Tip>
  A extensão inclui sua própria cópia da CLI (interface de linha de comando) para o painel de chat. Para executar `claude` no terminal integrado do VS Code, você também precisa da [instalação da CLI autônoma](/docs/pt/setup). Consulte [VS Code extension vs. Claude Code CLI](#vs-code-extension-vs-claude-code-cli) para detalhes.
</Tip>

<h2 id="install-the-extension">
  Instale a extensão
</h2>

Clique no link do seu IDE para instalar diretamente:

* [Instalar para VS Code](vscode:extension/anthropic.claude-code)
* [Instalar para Cursor](cursor:extension/anthropic.claude-code)

Ou no VS Code, pressione `Cmd+Shift+X` (Mac) ou `Ctrl+Shift+X` (Windows/Linux) para abrir a visualização de Extensões, procure por "Claude Code" e clique em **Instalar**.

A extensão também é instalada em outros forks do VS Code como Devin Desktop ou Kiro. Procure por "Claude Code" na visualização de Extensões do editor, ou instale a partir do [registro Open VSX](https://open-vsx.org/extension/Anthropic/claude-code). Se o seu editor não conseguir instalar a extensão, [instale a CLI](/docs/pt/quickstart) e execute `claude` no seu terminal integrado. A CLI funciona em qualquer terminal.

<Note>Se a extensão não aparecer após a instalação, reinicie o VS Code ou execute "Developer: Reload Window" na Paleta de Comandos.</Note>

<h2 id="get-started">
  Comece agora
</h2>

Após a instalação, você pode começar a usar Claude Code através da interface do VS Code:

<Steps>
  <Step title="Abra o painel Claude Code">
    Em todo o VS Code, o ícone Spark indica Claude Code: <img src="https://mintcdn.com/claude-code/c5r9_6tjPMzFdDDT/images/vs-code-spark-icon.svg?fit=max&auto=format&n=c5r9_6tjPMzFdDDT&q=85&s=3ca45e00deadec8c8f4b4f807da94505" alt="Ícone Spark" style={{display: "inline", height: "0.85em", verticalAlign: "middle"}} width="16" height="16" data-path="images/vs-code-spark-icon.svg" />

    A forma mais rápida de abrir Claude é clicar no ícone Spark na **Barra de Ferramentas do Editor** (canto superior direito do editor). O ícone só aparece quando você tem um arquivo aberto.

    <img src="https://mintcdn.com/claude-code/mfM-EyoZGnQv8JTc/images/vs-code-editor-icon.png?fit=max&auto=format&n=mfM-EyoZGnQv8JTc&q=85&s=eb4540325d94664c51776dbbfec4cf02" alt="VS Code editor mostrando o ícone Spark na Barra de Ferramentas do Editor" width="2796" height="734" data-path="images/vs-code-editor-icon.png" />

    Outras formas de abrir Claude Code:

    * **Activity Bar**: clique no ícone Spark na barra lateral esquerda para abrir a lista de sessões. Clique em qualquer sessão para abri-la no seu [local preferido](#extension-settings), ou inicie uma nova. Este ícone está sempre visível na Activity Bar.
    * **Command Palette**: `Cmd+Shift+P` (Mac) ou `Ctrl+Shift+P` (Windows/Linux), digite "Claude Code" e selecione uma opção como "Open in New Tab"
    * **Status Bar**: se você definiu [`preferredLocation`](#extension-settings) como `sidebar`, ou abriu Claude com **Claude Code: Open in Side Bar**, clique em **✻ Claude Code** no canto inferior direito da janela. Isso funciona mesmo quando nenhum arquivo está aberto.

    Você pode arrastar o painel Claude para reposicioná-lo em qualquer lugar no VS Code. Veja [Personalize seu fluxo de trabalho](#customize-your-workflow) para detalhes.
  </Step>

  <Step title="Faça login">
    A primeira vez que você abre o painel, uma tela de login aparece. Clique em **Sign in** e complete a autorização no seu navegador.

    Se você vir **Not logged in · Please run /login** mais tarde, a extensão reabre a tela de login automaticamente. Se ela não aparecer, recarregue a janela a partir da Command Palette com **Developer: Reload Window**.

    Se você tem `ANTHROPIC_API_KEY` definida no seu shell mas ainda vê o prompt de login, o VS Code pode não ter herdado o ambiente do seu shell. Inicie o VS Code a partir de um terminal com `code .` para que ele herde suas variáveis de ambiente, ou faça login com sua conta Claude.

    Após fazer login, uma lista de verificação **Learn Claude Code** aparece. Trabalhe em cada item clicando em **Show me**, ou descarte-a com o X. Para reabri-la mais tarde, desmarque **Hide Onboarding** nas configurações do VS Code em Extensions → Claude Code.
  </Step>

  <Step title="Envie um prompt">
    Peça a Claude para ajudar com seu código ou arquivos, seja explicando como algo funciona, depurando um problema ou fazendo alterações.

    <Tip>Claude vê automaticamente seu texto selecionado. Pressione `Option+K` (Mac) / `Alt+K` (Windows/Linux) para também inserir uma referência @-mention (como `@file.ts#5-10`) no seu prompt.</Tip>

    Aqui está um exemplo de pergunta sobre uma linha particular em um arquivo:

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-send-prompt.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=ede3ed8d8d5f940e01c5de636d009cfd" alt="VS Code editor com as linhas 2-3 selecionadas em um arquivo Python, e o painel Claude Code mostrando uma pergunta sobre essas linhas com uma referência @-mention" width="3288" height="1876" data-path="images/vs-code-send-prompt.png" />
  </Step>

  <Step title="Revise as alterações">
    O que você vê depende do [modo de permissão](/docs/pt/permission-modes#which-mode-a-session-starts-in) mostrado na parte inferior da caixa de prompt:

    * No modo Auto ou Edit automatically, Claude edita a maioria dos arquivos no seu workspace sem perguntar.
    * No modo Manual, quando Claude quer editar um arquivo, ele mostra uma comparação lado a lado do original e das alterações propostas, depois pede permissão. Você pode aceitar, rejeitar ou dizer a Claude o que fazer em vez disso. Se você editar o conteúdo proposto diretamente na visualização de diff antes de aceitar, Claude é informado de que você o modificou para que não assuma que o arquivo corresponde à sua proposta original.

          <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-edits.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=e005f9b41c541c5c7c59c082f7c4841c" alt="VS Code mostrando um diff das alterações propostas por Claude com um prompt de permissão perguntando se deve fazer a edição" width="3292" height="1876" data-path="images/vs-code-edits.png" />

    Para revisar uma edição proposta uma alteração por vez, use os botões **Accept this change** e **Reject this change** sob cada alteração no diff. Rejeitar uma alteração a reverte no conteúdo proposto; aceitar a marca como revisada. Aceitar ou rejeitar o arquivo inteiro ainda finaliza a revisão. Um diff com mais de 100 alterações abre sem os botões por alteração, então revise-o como um arquivo inteiro. A revisão por alteração requer Claude Code v2.1.275 ou posterior.

    As mesmas ações estão disponíveis no cursor a partir do menu de contexto do editor e da Command Palette como **Claude Code: Accept Change at Cursor** e **Claude Code: Reject Change at Cursor**.
  </Step>
</Steps>

Para mais ideias sobre o que você pode fazer com Claude Code, veja [Fluxos de trabalho comuns](/docs/pt/common-workflows).

<Tip>
  Execute "Claude Code: Open Walkthrough" a partir da Command Palette para um tour guiado dos conceitos básicos.
</Tip>

<h2 id="use-the-prompt-box">
  Use the prompt box
</h2>

The prompt box supports several features:

* **Permission modes**: click the mode indicator at the bottom of the prompt box to switch permission modes. On Pro, Max, and Team plans, Auto is the built-in starting permission mode. See [how the extension chooses the starting permission mode](/docs/pt/permission-modes#switch-permission-modes) for what changes that, and every permission mode the indicator offers.
  * **Auto**: a classifier reviews most actions instead of asking you. See [auto mode](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) for what it reviews and blocks.
  * **Manual**: Claude asks permission before file edits and most shell commands.
  * **Plan**: Claude describes what it will do and waits for approval before making changes. VS Code automatically opens the plan as a full Markdown document where you can add inline comments to give feedback before Claude begins.

    You can also type `/plan` in the prompt box. Requires Claude Code v2.1.280 or later.

    * `/plan`: switches to plan mode. If you're already in plan mode, shows the current plan instead.
    * `/plan` with a task, such as `/plan fix the auth bug`: switches to plan mode and starts planning that task.
    * `/plan open`: when you're already in plan mode, opens the plan file in the editor.
  * **Edit automatically**: Claude makes edits without asking.
* **Model**: select **Switch model…** from the command menu to change the model mid-session. You can also click the model name at the bottom of the prompt box to open the same picker.

  When the current model supports [effort levels](/docs/pt/model-config#adjust-effort-level), the picker also shows an **Effort** row and the model name button shows the selected level. When you pick a level other than `max`, Claude Code saves it for the current model as your default, under [`modelSettings`](/docs/pt/settings-reference#modelsettings) in your user settings; `max` applies to the current session only. The model name button and the **Effort** row require Claude Code v2.1.257 or later.
* **Command menu**: click `/` or type `/` to open the command menu. Options include attaching files, switching models, and toggling extended thinking.

  The Customize section includes entries such as MCP servers, commands, output styles, hooks, memory, instructions, permissions, and plugins. Items with a terminal icon open in the integrated terminal.

  * To browse commands such as `/usage` or [`/remote-control`](/docs/pt/remote-control), select **Slash commands** in the Customize section. A dialog lists them with a filter box. Pick one to run it. Typing `/` in the prompt box still suggests commands inline. Requires Claude Code v2.1.257 or later.

    Typing `/skills` also opens this dialog. Each [skill](/docs/pt/skills) row shows its [visibility](/docs/pt/skills#override-skill-visibility-from-settings), such as **On** or **Name only**. Click the visibility to change it, except on rows marked **locked**, such as plugin skills. The `/skills` shortcut and the visibility controls require Claude Code v2.1.280 or later.
  * Select **Output styles** in the Customize section to pick an [output style](/docs/pt/output-styles), including your custom styles. Requires Claude Code v2.1.257 or later.

    To create a custom style instead, select **Build a custom style** from the **Output styles** menu. Claude Code writes the [style file](/docs/pt/output-styles#create-a-custom-output-style) for you at the project or user level. Requires Claude Code v2.1.261 or later.
  * Select **Hooks** in the Customize section to view the [hooks](/docs/pt/hooks) loaded in the session, grouped by event. You can add, edit, or remove hooks saved in your user, project, and local settings files. Hooks from other sources, such as managed settings or plugins, are read-only. Requires Claude Code v2.1.269 or later.
  * Select **Permissions** in the Customize section to view the session's [permission rules](/docs/pt/permissions), grouped into Allow, Ask, and Deny. You can add rules to your user, project, or local settings and remove rules saved there. Rules from other sources, such as managed settings or approvals made for this session only, are read-only. Requires Claude Code v2.1.269 or later.
  * Select **Memory** in the Customize section to turn [auto memory](/docs/pt/memory#auto-memory) on or off. While it's on, you can also browse the memories Claude has saved and reveal the folders that store them in your file manager. Requires Claude Code v2.1.274 or later.

    Click a saved memory to read it in the dialog, where you can edit the text, delete the memory, or open its file in the editor. Viewing, editing, and deleting a memory in the dialog require Claude Code v2.1.275 or later.
  * Select **Instructions** in the Customize section to edit the [CLAUDE.md files](/docs/pt/memory#claude-md-files) Claude reads. Pick a file to open it in the editor. If the file doesn't exist yet, Claude Code creates it first. Requires Claude Code v2.1.274 or later.
  * Select **Status** in the Customize section, or type `/status`, to check the session's Claude Code version, account, model, and MCP server details. Requires Claude Code v2.1.280 or later.
  * Select **Sandbox** in the Customize section, or type `/sandbox`, to see whether Claude's Bash commands run [sandboxed](/docs/pt/sandboxing). You can switch the sandbox mode and add [excluded commands](/docs/pt/settings-reference#sandbox-excludedcommands) there. Requires Claude Code v2.1.280 or later.
  * Select **Claude in Chrome** in the Customize section, or type `/chrome`, to check and manage the [Claude in Chrome](/docs/pt/chrome) connection. Both require signing in with a claude.ai account. Requires Claude Code v2.1.280 or later.
  * Select **Export conversation** in the Context section, or type `/export`, to copy the conversation as plain text or save it to a file. Add a file name, such as `/export notes.txt`, to skip the dialog and choose where to save the file. Requires Claude Code v2.1.280 or later.
  * The Settings section includes **Enable Remote Control for all sessions**, which sets [`remoteControlAtStartup`](/docs/pt/settings-reference#remotecontrolatstartup) to control whether [new interactive sessions connect to Remote Control automatically](/docs/pt/remote-control#enable-remote-control-for-all-sessions). Requires Claude Code v2.1.203 or later.

    When you turn the toggle on or off in a VS Code window, the change applies to the sessions already open in that VS Code window, not only to sessions you start afterwards. If you turn it off, the open sessions disconnect. With Claude Code v2.1.261 or later, the change also reaches sessions open in your other VS Code windows.
  * The Settings section also includes **Focus view**, which hides tool calls, tool results, and thinking behind expandable rows, leaving your prompts and Claude's responses. Toggle it there, with `Ctrl+Option+F` (Mac) / `Ctrl+Alt+F` (Windows/Linux), or from the Command Palette with **Claude Code: Toggle Focus view**. The change applies to every open session and persists across sessions. Requires Claude Code v2.1.221 or later.

    Claude's latest to-do list stays visible, and so does the text a pending question from Claude is asking about; this requires Claude Code v2.1.225 or later. While Claude runs [subagents](/docs/pt/sub-agents), live progress rows with their latest activity appear under the tool-call group that started them. This requires Claude Code v2.1.269 or later.
  * To sign out of your Anthropic account, select **Sign out** in the Settings section, or type `/logout`. On a [third-party provider](#use-third-party-providers), the menu doesn't offer either. Requires Claude Code v2.1.277 or later.
  * To report a bug, click **Report a problem** at the bottom of the menu, or type `/bug` or `/feedback` with an optional description that prefills the report. When you submit the report and you're signed in to Anthropic on a first-party connection, Claude Code sends it to Anthropic. On a third-party provider, or without Anthropic credentials, the dialog still opens, but submitting shows an error and sends nothing: unlike the CLI's `/bug`, the extension doesn't write a local archive. Requires Claude Code v2.1.229 or later.

    If your organization's policy turns product feedback off, **Report a problem** doesn't appear in the menu, and `/bug` and `/feedback` show a `Feedback is turned off by your organization's policy or this environment's settings.` notice instead of opening the report.
* **Side questions**: type `/btw` followed by a question to ask about your session [without adding to the conversation](/docs/pt/interactive-mode#side-questions-with-%2Fbtw). The answer opens in a panel beside the chat, where you can ask follow-up questions. The thread survives window reloads. Claude Code keeps the newest 20 exchanges and expires stored threads on the [`cleanupPeriodDays`](/docs/pt/settings-reference#cleanupperioddays) schedule, as long as Claude Code can [safely determine the retention period](/docs/pt/claude-directory#cleaned-up-automatically). To clear a thread, click the trash icon in the panel. Requires Claude Code v2.1.227 or later.
* **Copy a response**: hover over a response and click **Copy response** to copy it to your clipboard, or type `/copy` to copy the latest response. `/copy 2` copies the second-to-last. Requires Claude Code v2.1.277 or later.
* **Context indicator**: the prompt box shows how much of Claude's context window you're using. Claude automatically compacts when needed, or you can run `/compact` manually.
* **Prompt cache clock**: a clock icon next to the context indicator estimates how much time the conversation's [prompt cache](/docs/pt/prompt-caching) has left before it expires. It counts down from the cache's five-minute or one-hour [lifetime](/docs/pt/prompt-caching#cache-lifetime), and each response that uses the cache restarts the countdown. Apart from compaction, the [actions that invalidate the cache](/docs/pt/prompt-caching#actions-that-invalidate-the-cache) don't reset the clock, so it can still show minutes left after you switch models.
  * Until the countdown runs out, the icon shows the minutes left, such as **12m**.
  * When the countdown runs out, the minutes disappear and the icon turns red, or your theme's error color, until the next response. The cache has likely expired, so expect a slower, more expensive response to your next message while the cache rebuilds. If the five-minute lifetime keeps running out between your messages, see [Choose the TTL yourself](/docs/pt/prompt-caching#choose-the-ttl-yourself).
  * Right after the conversation is [compacted](/docs/pt/prompt-caching#compacting-the-conversation), the icon also turns red without minutes until the next response, because the cache doesn't cover the compacted conversation yet.
* **Agent map**: when the conversation includes [subagents](/docs/pt/sub-agents), an agent count such as **2 agents** appears at the bottom of the prompt box. Its dot shows whether any subagent is working or waiting for your permission.

  Click the agent count to open the agent map, which draws the conversation's subagents as a tree under the main agent, each with its status, elapsed time, and token count. Click a subagent to see its prompt and tool calls, open its read-only transcript, or stop it while it runs. Requires Claude Code v2.1.269 or later.

  The map also lists the session's other [background tasks](/docs/pt/tools-reference#background-commands), such as background shell commands and [monitors](/docs/pt/tools-reference#monitor-tool), below the agents. Click a row to open the task's card and stop it there.

  To open the map when no agent count is showing, such as when Claude has started a background shell but no subagents, type `/tasks` in the prompt box. Background tasks in the map and the typed `/tasks` require Claude Code v2.1.277 or later.
* **Extended thinking**: lets Claude spend more time reasoning through complex problems. Toggle it on via the command menu (`/`). Claude's reasoning appears in the conversation as collapsed blocks: click a block to read it, or press `Ctrl+O` to expand or collapse every thinking block in the session. See [Extended thinking](/docs/pt/model-config#extended-thinking) for details.
* **Multi-line input**: press `Shift+Enter` to add a new line without sending. This also works in the "Other" free-text input of question dialogs.

<h3 id="reference-files-and-folders">
  Reference files and folders
</h3>

Use @-mentions to give Claude context about specific files or folders. When you type `@` followed by a file or folder name, Claude reads that content and can answer questions about it or make changes to it. Claude Code supports fuzzy matching, so you can type partial names to find what you need:

```text wrap theme={null}
Explain the logic in @auth (fuzzy matches auth.js, AuthService.ts, etc.)
What's in @src/components/ (include a trailing slash for folders)
```

For large PDFs, you can ask Claude to read specific pages instead of the whole file: a single page, a range like pages 1-10, or an open-ended range like page 3 onward.

When you select text in the editor, Claude can see your highlighted code automatically. The prompt box footer shows how many lines are selected. Press `Option+K` (Mac) / `Alt+K` (Windows/Linux) to insert an @-mention with the file path and line numbers (e.g., `@app.ts#5-10`). Click the **X** on the selection indicator to remove it so Claude doesn't receive the selection. The indicator comes back when you select other text.

The extension withholds selected text from some files. When the file is inside your workspace and matches your `files.exclude` or `search.exclude` settings, Claude receives at most the file's path and not the text you selected. The same applies to a file git ignores, as long as VS Code's `search.useIgnoreFiles` setting and the extension's [`respectGitIgnore` setting](#extension-settings) are both on, which is the default. This filter covers the chat panel only: when Claude Code runs in the integrated terminal, the CLI sends your selected text whatever the file, so add a [`Read` deny rule](#the-built-in-ide-mcp-server) to keep a file's contents from Claude there.

Claude also sees which file you have open in the editor, even when nothing is selected, and the prompt box shows its name. To add only your selected text, turn off the [Attach Open File setting](vscode://settings/claudeCode.attachOpenFile). The setting requires Claude Code v2.1.271 or later.

You can also attach images and files to your message:

* To attach an image, paste it from your clipboard into the prompt box.
* To attach files, hold `Shift` while dragging them into the prompt box.
* To remove an attachment from context, click the X on it.

<h3 id="paste-text">
  Paste text
</h3>

Text you paste stays visible in the prompt box, rather than collapsing to a placeholder as it does [in the terminal](/docs/pt/terminal-config#paste-large-content). In sessions where Claude Code [marks pasted text](/docs/pt/terminal-config#how-claude-treats-pasted-text), Claude still sees a large paste as text you pasted rather than typed.

Claude Code also removes [invisible Unicode characters](/docs/pt/interactive-mode#invisible-characters-in-prompts) from text you paste into the prompt box and from anything else you send:

* If a notice such as `Removed 3 invisible characters from the pasted text` appears when you paste, the text went in without those characters.
* If a notice about removed characters appears when you send, nothing was sent. The cleaned text is back in the prompt box. Send again to send the text as shown.

<h3 id="resume-past-conversations">
  Resume past conversations
</h3>

Click the **Session history** button at the top of the Claude Code panel to access your conversation history. You can search by keyword or browse by time.

Click any conversation to resume it with the full message history. If the conversation is already open in another tab of the current window, clicking it switches to that tab. For more on resuming sessions, see [Manage sessions](/docs/pt/sessions).

* **Session titles**: new sessions receive AI-generated titles based on your first message.
* **Rename and archive**: hover over a session to reveal these actions. Rename to give it a descriptive title, or archive to move it to the **Archived sessions** group at the bottom of the list.

By default, a session with no activity for 14 days moves to **Archived sessions** automatically, unless it is open, unread, or in a [group](#organize-sessions-into-groups). Automatic archiving requires Claude Code v2.1.265 or later. To change the period or turn it off, open the [Archive Inactive Sessions setting](vscode://settings/claudeCode.archiveInactiveSessions) and select a number of days or **Never**.

To restore an archived session, expand **Archived sessions** and click **Unarchive session**. To restore every archived session at once, hover over the **Archived sessions** header in the sessions list in the Activity Bar and click its unarchive icon, which requires Claude Code v2.1.277 or later. Before v2.1.257, the action was **Delete session**, which hid a session with no way to restore it. Sessions you deleted then appear under **Archived sessions** after you upgrade.

When the conversation you resume ended in plan mode, Claude Code restores plan mode. Requires Claude Code v2.1.246 or later. Claude Code doesn't restore it in two cases:

* The extension [chooses the starting permission mode](/docs/pt/permission-modes#switch-permission-modes) from `claudeCode.initialPermissionMode` or a pick that carries over from an earlier conversation
* You have `claudeCode.claudeProcessWrapper` configured

<h3 id="resume-cloud-sessions-from-claude-ai">
  Resume cloud sessions from Claude.ai
</h3>

If you run [cloud sessions](/docs/pt/claude-code-on-the-web), you can resume them directly in VS Code. This requires signing in with **Claude.ai Subscription**, not Anthropic Console.

<Steps>
  <Step title="Open session history">
    Click the **Session history** button at the top of the Claude Code panel.
  </Step>

  <Step title="Select the Web tab">
    The dialog shows two tabs: Local and Web. Click **Web** to see sessions from claude.ai.
  </Step>

  <Step title="Select a session to resume">
    Browse or search your cloud sessions. Click any session to download it and continue the conversation locally.
  </Step>
</Steps>

<Note>
  Only cloud sessions started with a GitHub repository appear in the Web tab. Resuming loads the conversation history locally; changes are not synced back to claude.ai.
</Note>

<h3 id="check-account-and-usage">
  Check account and usage
</h3>

Run `/usage` to open the Account & usage dialog. It shows your signed-in account, and the usage it reports differs by sign-in:

* **claude.ai plan**: usage bars for your plan's limits, such as the current session and the week. Each bar shows how long until its limit resets.

  The dialog also breaks down what is contributing to your plan limits. It flags behaviors that account for 10% or more of recent usage, such as cache misses, long context, and subagent-heavy or highly parallel sessions, each with a tip to reduce it. Attribution tables show how much usage came from each skill, subagent, plugin, and MCP server.

  Use the Day and Week toggle to switch between the last 24 hours and the last 7 days. The figures are approximate and computed from local sessions on this machine, so usage from other devices or claude.ai is not included.
* **Other sign-ins**: when plan limits don't apply to your sign-in, such as on a [third-party provider](#use-third-party-providers) or with an API key, the Usage section shows the session's own cost and token usage instead. The CLI's `/usage` shows the same totals in its [Session block](/docs/pt/costs#track-your-costs). The sessions list in the Activity Bar also shows the active session's totals under its **Account & usage** header. Requires Claude Code v2.1.277 or later.

For more on tracking and reducing usage, see [Track your costs](/docs/pt/costs#track-your-costs).

<h2 id="customize-your-workflow">
  Personalize seu fluxo de trabalho
</h2>

Você pode reposicionar o painel Claude, executar múltiplas conversas, organizar a lista de sessões em grupos ou alternar para o modo terminal.

<h3 id="choose-where-claude-lives">
  Escolha onde Claude fica
</h3>

Você pode arrastar o painel Claude para reposicioná-lo em qualquer lugar no VS Code. Pegue na aba ou barra de título do painel e arraste para:

* **Barra lateral secundária**: o lado direito da janela. Mantém Claude visível enquanto você codifica.
* **Barra lateral primária**: a barra lateral esquerda com ícones para Explorer, Search, etc.
* **Área do editor**: abre Claude como uma aba ao lado de seus arquivos. Útil para tarefas secundárias.

Quando Claude abre uma aba em um novo grupo de editor, a extensão bloqueia esse grupo, então os arquivos que você abre enquanto a aba Claude está em foco vão para outro grupo em vez de ficar ao lado dela.

Para impedir que a extensão bloqueie grupos, desative a [configuração Lock Editor Groups](vscode://settings/claudeCode.lockEditorGroups). Os grupos que já estão bloqueados permanecem bloqueados até que você os desbloqueie. A configuração requer Claude Code v2.1.274 ou posterior.

<Tip>
  Use a barra lateral para sua sessão principal do Claude e abra abas adicionais para tarefas secundárias. Claude lembra sua localização preferida. O ícone da lista de sessões da Activity Bar é separado do painel Claude: a lista de sessões está sempre visível na Activity Bar, enquanto o ícone do painel Claude só aparece lá quando o painel está encaixado na barra lateral esquerda.
</Tip>

Depois de executar **Developer: Reload Window** ou reiniciar o VS Code, se um chat volta com sua conversa depende de onde estava aberto:

* **Aba do editor**: a conversa volta com sua aba.
* **Barra lateral**: a conversa volta se você enviou uma mensagem ou Claude respondeu nela nos últimos 10 minutos. Se ela não voltar, retome a conversa do [Histórico de sessões](#resume-past-conversations).

Se o recarregamento interrompeu Claude no meio de uma etapa, Claude continua essa etapa quando a conversa volta, e um aviso no chat marca a continuação. Requer Claude Code v2.1.274 ou posterior. Se a etapa foi interrompida há mais de uma hora ou a sessão está aberta em outro lugar, a conversa volta inativa em vez disso.

Para desativar a continuação, abra a [configuração Continue After Reload](vscode://settings/claudeCode.continueAfterReload) e desmarque-a.

<h3 id="run-multiple-conversations">
  Execute múltiplas conversas
</h3>

Use **Open in New Tab** ou **Open in New Window** da Paleta de Comandos para iniciar conversas adicionais. Cada conversa mantém seu próprio histórico e contexto, permitindo que você trabalhe em diferentes tarefas em paralelo.

Ao usar abas, um pequeno ponto colorido no ícone de spark indica o status: azul significa que uma solicitação de permissão está pendente, laranja significa que Claude terminou enquanto a aba estava oculta.

<h3 id="organize-sessions-into-groups">
  Organize sessões em grupos
</h3>

Na lista de sessões da Activity Bar, você pode coletar sessões relacionadas em grupos nomeados e recolhíveis. Requer Claude Code v2.1.229 ou posterior.

* **Agrupar ou desagrupar uma sessão**: clique com o botão direito em uma sessão para criar um grupo a partir dela, movê-la para um grupo existente ou removê-la de seu grupo. Cada sessão pertence a um grupo por vez, então movê-la para outro grupo a remove do primeiro.
* **Mover várias sessões de uma vez**: `Cmd`-clique (Mac) / `Ctrl`-clique (Windows/Linux) em cada sessão, ou `Shift`-clique para selecionar um intervalo, depois clique com o botão direito na seleção.
* **Agrupar uma sessão a partir de sua aba**: execute **Claude Code: Add Session Tab to Group** da Paleta de Comandos, depois escolha ou crie um grupo. Requer Claude Code v2.1.257 ou posterior.
* **Renomear ou excluir um grupo**: clique com o botão direito em um cabeçalho de grupo. Excluir um grupo remove apenas o grupo, e suas sessões retornam à lista desagrupada.

A extensão salva grupos por pasta de workspace, então eles sobrevivem a recarregamentos de janela e aparecem em cada janela onde você abre a mesma pasta. Quando você pesquisa a lista, a extensão mostra correspondências em uma lista plana única em todos os grupos.

<h3 id="switch-to-terminal-mode">
  Alterne para o modo terminal
</h3>

Por padrão, a extensão abre um painel de chat gráfico. Se você preferir a interface no estilo CLI, abra a [configuração Use Terminal](vscode://settings/claudeCode.useTerminal) e marque a caixa.

Você também pode abrir as configurações do VS Code (`Cmd+,` no Mac ou `Ctrl+,` no Windows/Linux), ir para Extensions → Claude Code e marcar **Use Terminal**.

<h2 id="manage-plugins">
  Gerenciar plugins
</h2>

A extensão VS Code inclui uma interface gráfica para instalar e gerenciar [plugins](/docs/pt/plugins/overview). Digite `/plugins` na caixa de prompt para abrir a interface **Gerenciar plugins**.

<h3 id="install-plugins">
  Instalar plugins
</h3>

O diálogo de plugin mostra duas abas: **Plugins** e **Marketplaces**.

Na aba Plugins:

* **Plugins instalados** aparecem no topo com interruptores de alternância para ativá-los ou desativá-los
* **Plugins disponíveis** de seus marketplaces configurados aparecem abaixo
* Pesquise para filtrar plugins por nome ou descrição
* Clique em **Instalar** em qualquer plugin disponível

Quando você instala um plugin, escolha o escopo de instalação:

* **Instalar para você**: disponível em todos os seus projetos (escopo de usuário)
* **Instalar para este projeto**: compartilhado com colaboradores do projeto (escopo de projeto)
* **Instalar localmente**: apenas para você, apenas neste repositório (escopo local)

<h3 id="share-a-plugin-install-link">
  Compartilhar um link de instalação de plugin
</h3>

Para enviar alguém diretamente para instalar um plugin específico, forneça a URL `install-plugin` da extensão. Abri-la inicia ou foca VS Code, abre o painel Claude Code e abre o diálogo **Gerenciar plugins** na escolha de escopo daquele plugin. Nada é instalado até que a pessoa escolha um escopo. Se o marketplace do plugin ainda não estiver configurado no Claude Code deles, o diálogo primeiro pede que eles o adicionem.

```text theme={null}
vscode://anthropic.claude-code/install-plugin?plugin=code-review&marketplace=anthropics/claude-plugins-official
```

A URL aceita dois parâmetros de consulta:

| Parâmetro     | Descrição                                                                                                                                                                                      |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugin`      | O nome do plugin conforme seu marketplace o lista. Obrigatório.                                                                                                                                |
| `marketplace` | De onde o plugin vem: um `owner/repo` do GitHub, uma URL `https://`, ou uma URL git SSH como `git@github.com:owner/repo.git`. Padrão para `anthropics/claude-plugins-official` quando omitido. |

Alguns valores que a [aba Marketplaces](#manage-marketplaces) aceita não funcionam em um link, como um caminho local ou um endereço `http://`. Para esses, VS Code mostra uma mensagem de erro e o diálogo não abre.

Dois casos terminam em uma mensagem no diálogo em vez da escolha de escopo:

* **O marketplace não lista um plugin com esse nome**: o diálogo relata que o plugin não foi encontrado. Verifique o valor `plugin` contra a listagem do marketplace.
* **O plugin já está instalado**: o diálogo diz isso, e nada muda.

READMEs do GitHub, problemas e alguns outros hosts Markdown removem links cujo esquema não é `http` ou `https`, então um link `vscode://` lá é renderizado como texto simples. Coloque a URL em um bloco de código nesses hosts, conforme [O link é renderizado como texto simples em vez de ser clicável](/docs/pt/deep-links#the-link-renders-as-plain-text-instead-of-being-clickable) descreve para links `claude-cli://`.

<h3 id="manage-marketplaces">
  Gerenciar marketplaces
</h3>

Alterne para a aba **Marketplaces** para adicionar ou remover fontes de plugin:

* Digite um repositório GitHub, URL ou caminho local para adicionar um novo marketplace
* Clique no ícone de atualização para atualizar a lista de plugins de um marketplace
* Clique no ícone de lixeira para remover um marketplace

As alterações de plugin que você faz no diálogo se aplicam imediatamente às sessões Claude Code abertas naquela janela VS Code. Se a sessão a partir da qual você abriu o diálogo não conseguir recarregar seus plugins, o diálogo oferece tentar novamente ou reiniciar Claude naquela sessão.

<Note>
  O gerenciamento de plugins no VS Code usa os mesmos comandos CLI sob o capô. Plugins e marketplaces que você configura na extensão também estão disponíveis na CLI, e vice-versa.
</Note>

Para mais informações sobre o sistema de plugins, consulte [Plugins](/docs/pt/plugins/overview) e [Plugin marketplaces](/docs/pt/plugins/overview).

<h2 id="automate-browser-tasks-with-chrome">
  Automatizar tarefas do navegador com Chrome
</h2>

Conecte Claude ao seu navegador Chrome para testar aplicativos web, depurar com logs do console e automatizar fluxos de trabalho do navegador sem sair do VS Code. Isso requer a [extensão Claude in Chrome](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) versão 1.0.36 ou superior.

Digite `@browser` na caixa de prompt seguido pelo que você deseja que Claude faça:

```text wrap theme={null}
@browser go to localhost:3000 and check the console for errors
```

Você também pode abrir o menu de anexos para selecionar ferramentas específicas do navegador, como abrir uma nova aba ou ler o conteúdo da página.

Claude abre novas abas para tarefas do navegador e compartilha o estado de login do seu navegador, para que possa acessar qualquer site em que você já esteja conectado.

Para instruções de configuração, a lista completa de recursos e solução de problemas, consulte [Use Claude Code with Chrome](/docs/pt/chrome).

<h2 id="vs-code-commands-and-shortcuts">
  Comandos e atalhos de teclado do VS Code
</h2>

Abra a Paleta de Comandos (`Cmd+Shift+P` no Mac ou `Ctrl+Shift+P` no Windows/Linux) e digite "Claude Code" para ver todos os comandos disponíveis do VS Code para a extensão Claude Code.

Alguns atalhos de teclado dependem de qual painel está "focado" (recebendo entrada de teclado). Quando seu cursor está em um arquivo de código, o editor está focado. Quando seu cursor está na caixa de prompt do Claude, o Claude está focado. Use `Cmd+Esc` / `Ctrl+Esc` para alternar entre eles.

<Note>
  Estes são comandos do VS Code para controlar a extensão. Nem todos os comandos Claude Code integrados estão disponíveis na extensão. Consulte [Extensão VS Code vs. Claude Code CLI](#vs-code-extension-vs-claude-code-cli) para obter detalhes.
</Note>

| Comando                    | Atalho de teclado                                        | Descrição                                                                                                                                                                                                                                                                                |
| -------------------------- | -------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Focus Input                | `Cmd+Esc` (Mac) / `Ctrl+Esc` (Windows/Linux)             | Alterna o foco entre o editor e o Claude                                                                                                                                                                                                                                                 |
| Focus last message         | -                                                        | Mova o foco do teclado para a mensagem mais recente na conversa, ou para um prompt de permissão pendente, para que você possa ler a partir daí com o teclado ou um leitor de tela. Não disponível em [modo terminal](#switch-to-terminal-mode). Requer Claude Code v2.1.268 ou posterior |
| Open in Side Bar           | -                                                        | Abrir Claude na barra lateral                                                                                                                                                                                                                                                            |
| Open in Terminal           | -                                                        | Abrir Claude no modo terminal                                                                                                                                                                                                                                                            |
| Open in New Tab            | `Cmd+Shift+Esc` (Mac) / `Ctrl+Shift+Esc` (Windows/Linux) | Abrir uma nova conversa como uma aba do editor                                                                                                                                                                                                                                           |
| Open in New Window         | -                                                        | Abrir uma nova conversa em uma janela separada                                                                                                                                                                                                                                           |
| New Conversation           | `Cmd+N` (Mac) / `Ctrl+N` (Windows/Linux)                 | Iniciar uma nova conversa. Requer que o Claude esteja focado e `enableNewConversationShortcut` definido como `true`                                                                                                                                                                      |
| Reopen Closed Session      | `Cmd+Shift+T` (Mac) / `Ctrl+Shift+T` (Windows/Linux)     | Reabrir a aba de sessão do Claude fechada mais recentemente. Volta para a reabertura normal de editor fechado do VS Code quando a última aba fechada não era uma sessão do Claude. Desabilitar com `enableReopenClosedSessionShortcut`                                                   |
| Insert @-Mention Reference | `Option+K` (Mac) / `Alt+K` (Windows/Linux)               | Inserir uma referência ao arquivo atual e seleção (requer que o editor esteja focado)                                                                                                                                                                                                    |
| Accept Change at Cursor    | -                                                        | Aceitar a alteração no cursor enquanto [revisa uma edição proposta](#get-started) uma alteração por vez. Requer Claude Code v2.1.275 ou posterior                                                                                                                                        |
| Reject Change at Cursor    | -                                                        | Reverter a alteração no cursor enquanto revisa uma edição proposta uma alteração por vez. Requer Claude Code v2.1.275 ou posterior                                                                                                                                                       |
| Toggle Focus view          | `Ctrl+Option+F` (Mac) / `Ctrl+Alt+F` (Windows/Linux)     | Ocultar ou mostrar atividade de ferramenta na conversa. Funciona enquanto um painel ou barra lateral do Claude está visível. Requer Claude Code v2.1.221 ou posterior                                                                                                                    |
| Rename Session Tab         | -                                                        | Renomear a sessão na aba ativa do Claude. Requer Claude Code v2.1.257 ou posterior                                                                                                                                                                                                       |
| Add Session Tab to Group   | -                                                        | Adicionar a sessão na aba ativa do Claude a um [grupo de sessão](#organize-sessions-into-groups) que você escolher ou criar. Requer Claude Code v2.1.257 ou posterior                                                                                                                    |
| Mark Session as Unread     | -                                                        | Marcar a sessão na aba ativa do Claude como não lida na lista de sessões. Requer Claude Code v2.1.257 ou posterior                                                                                                                                                                       |
| Show Logs                  | -                                                        | Visualizar logs de depuração da extensão                                                                                                                                                                                                                                                 |
| Logout                     | -                                                        | Sair da sua conta Anthropic                                                                                                                                                                                                                                                              |

<h3 id="launch-a-vs-code-tab-from-other-tools">
  Iniciar uma aba do VS Code a partir de outras ferramentas
</h3>

A extensão registra um manipulador de URI em `vscode://anthropic.claude-code/open`. Use-o para abrir uma nova aba Claude Code a partir de suas próprias ferramentas: um alias de shell, um bookmarklet de navegador ou qualquer script que possa abrir uma URL. Se o VS Code ainda não estiver em execução, abrir a URL o inicia primeiro. Se o VS Code já estiver em execução, a URL abre na janela que está atualmente focada.

Invoque o manipulador com o abridor de URL do seu sistema operacional.

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    open "vscode://anthropic.claude-code/open"
    ```
  </Tab>

  <Tab title="Linux">
    ```bash theme={null}
    xdg-open "vscode://anthropic.claude-code/open"
    ```

    O comando `xdg-open` vem do pacote `xdg-utils`. Se o shell relatar que não foi encontrado, consulte [xdg-open is not found on Linux](/docs/pt/deep-links#xdg-open-is-not-found-on-linux).
  </Tab>

  <Tab title="Windows">
    No PowerShell:

    ```powershell theme={null}
    Start-Process "vscode://anthropic.claude-code/open"
    ```

    No `cmd.exe`, `start` trata seu primeiro argumento entre aspas como um título de janela, então passe um título vazio antes da URL:

    ```cmd theme={null}
    start "" "vscode://anthropic.claude-code/open"
    ```
  </Tab>
</Tabs>

O manipulador aceita dois parâmetros de consulta opcionais:

| Parâmetro | Descrição                                                                                                                                                                                                                                                                                                                                                                                         |
| --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`  | Texto para pré-preenchimento na caixa de prompt. Deve ser codificado em URL. O prompt é pré-preenchido, mas não é enviado automaticamente.                                                                                                                                                                                                                                                        |
| `session` | Um ID de sessão para retomar em vez de iniciar uma nova conversa. A sessão deve pertencer ao espaço de trabalho atualmente aberto no VS Code. Se a sessão não for encontrada, uma conversa nova é iniciada. Se a sessão já estiver aberta em uma aba, essa aba é focada. Para capturar um ID de sessão programaticamente, consulte [Continue conversations](/docs/pt/headless#continue-conversations). |

Por exemplo, para abrir uma aba pré-preenchida com "review my changes":

```text theme={null}
vscode://anthropic.claude-code/open?prompt=review%20my%20changes
```

A extensão também manipula `vscode://anthropic.claude-code/install-plugin`, que [abre o diálogo de plugin em um plugin](#share-a-plugin-install-link). Para iniciar uma sessão de terminal em vez de uma aba do VS Code, use o manipulador `claude-cli://` da CLI. Consulte [Launch sessions from links](/docs/pt/deep-links).

<h2 id="configure-settings">
  Configurar configurações
</h2>

A extensão tem dois tipos de configurações:

* **Configurações da extensão** no VS Code: controlam o comportamento da extensão dentro do VS Code. Abra com `Cmd+,` (Mac) ou `Ctrl+,` (Windows/Linux), depois vá para Extensões → Claude Code. Você também pode digitar `/` e selecionar **General config…** para abrir as configurações.
* **Configurações do Claude Code** em `~/.claude/settings.json`: compartilhadas entre a extensão e CLI. Use-a para comandos permitidos, variáveis de ambiente, hooks e servidores MCP. Nos planos Pro, Max e Team, também é uma entrada para o modo de permissão em que as conversas começam. [Switch permission modes](/docs/pt/permission-modes#switch-permission-modes) lista a ordem. Veja [Settings](/docs/pt/settings) para detalhes.

<Tip>
  Adicione `"$schema": "https://json.schemastore.org/claude-code-settings.json"` ao seu `settings.json` para obter preenchimento automático e validação inline para todas as configurações disponíveis diretamente no VS Code.
</Tip>

<h3 id="extension-settings">
  Configurações da extensão
</h3>

O VS Code lê `initialPermissionMode` das suas configurações de usuário e ignora valores de workspace. Antes da v2.1.225, o VS Code padronizava a configuração para `default` e aplicava valores de workspace.

| Configuração                        | Padrão  | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ----------------------------------- | ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `useTerminal`                       | `false` | Inicie Claude no modo terminal em vez do painel gráfico                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `initialPermissionMode`             | -       | Controla prompts de aprovação para novas conversas: `default`, `plan`, `acceptEdits` ou `bypassPermissions`. `manual` é um alias para `default` e seleciona o modo rotulado **Manual** no indicador de modo. Quando você deixa sem definir, a extensão escolhe o modo de permissão inicial conforme descrito em [Switch permission modes](/docs/pt/permission-modes#switch-permission-modes).                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `preferredLocation`                 | `panel` | Onde Claude abre: `sidebar` (direita) ou `panel` (nova aba)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `lockEditorGroups`                  | `true`  | [Bloquear os grupos de editor que Claude inicia para suas abas](#choose-where-claude-lives), para que os arquivos que você abre enquanto uma aba Claude está em foco vão para outro grupo. Quando desativado, a extensão nunca bloqueia um grupo de editor. Requer Claude Code v2.1.274 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `autosave`                          | `true`  | Salvar automaticamente arquivos antes de Claude ler ou escrever neles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `attachOpenFile`                    | `true`  | Adicione o arquivo que está aberto no editor às suas mensagens e mostre-o na caixa de prompt. Quando desativado, apenas o texto selecionado é adicionado. Requer Claude Code v2.1.271 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `useCtrlEnterToSend`                | `false` | Use Ctrl/Cmd+Enter em vez de Enter para enviar prompts                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `scrollToBottomOnSend`              | `true`  | Rolar a conversa para o final quando você enviar uma mensagem. Quando desativado, a conversa permanece onde você a deixou. Requer Claude Code v2.1.275 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `enableNewConversationShortcut`     | `false` | Ativar Cmd/Ctrl+N para iniciar uma nova conversa                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `enableReopenClosedSessionShortcut` | `true`  | Use Cmd/Ctrl+Shift+T para reabrir a aba de sessão Claude fechada mais recentemente. Quando a última aba fechada não era uma sessão Claude, o atalho executa o comando normal de reabrir editor fechado do VS Code.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `archiveInactiveSessions`           | `14`    | [Arquivar uma sessão automaticamente](#resume-past-conversations) após este número de dias sem atividade: `1`, `2`, `7` ou `14`. Defina `0` para desativar. Requer Claude Code v2.1.265 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `continueAfterReload`               | `true`  | Após um recarregamento de janela, Claude [continua a etapa que foi interrompida](#choose-where-claude-lives) na sessão restaurada. Requer Claude Code v2.1.274 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `hideOnboarding`                    | `false` | Ocultar a lista de verificação de integração (ícone de chapéu de formatura)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `focusView`                         | `false` | Ocultar chamadas de ferramenta, resultados de ferramenta e pensamento atrás de linhas expansíveis, deixando seus prompts e respostas do Claude. A lista de tarefas mais recente do Claude permanece visível; isso requer Claude Code v2.1.225 ou posterior. Você também pode alternar a visualização de foco no menu de comandos. Requer Claude Code v2.1.221 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `respectGitIgnore`                  | `true`  | Excluir padrões .gitignore de buscas de arquivo e de [contexto de seleção](#reference-files-and-folders)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `usePythonEnvironment`              | `true`  | Ativar o ambiente Python do workspace ao executar Claude. Requer a extensão Python.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `environmentVariables`              | `[]`    | Definir variáveis de ambiente para o processo Claude. Use as configurações do Claude Code em vez disso para configuração compartilhada.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `disableLoginPrompt`                | `false` | Pular prompts de autenticação (para configurações de provedor de terceiros)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `allowDangerouslySkipPermissions`   | `false` | Adiciona Bypass permissions ao seletor de modo. Use apenas em sandboxes sem acesso à internet.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `claudeProcessWrapper`              | -       | Executável usado para iniciar o processo Claude. O caminho do binário agrupado é passado como um argumento quando presente. Defina isso para um binário `claude` instalado separadamente se a compilação da extensão não incluir um para sua plataforma. Em uma configuração encapsulada, as conversas começam no modo Manual a menos que você defina `initialPermissionMode` ou tenha escolhido Manual, Edit automatically ou Auto em uma conversa anterior, porque a extensão pula as configurações e as etapas padrão integradas lá; veja [Switch permission modes](/docs/pt/permission-modes#switch-permission-modes). Um erro "Unsupported platform" na ativação significa que nenhum binário está agrupado para sua plataforma; veja [which platforms have prebuilt binaries](/docs/pt/troubleshoot-install#native-binary-not-found-after-npm-install). |

<h2 id="use-a-screen-reader">
  Use a screen reader
</h2>

The extension's chat panel works with screen readers. You don't need to turn anything on: the extension announces conversation activity for every user, with no visual change. This is separate from the CLI's opt-in [screen reader mode](/docs/pt/accessibility), which adapts the terminal interface.

Screen reader support in the chat panel requires Claude Code v2.1.236 or later.

During a conversation, the extension announces:

* **Claude's replies**: the extension announces each reply once, when it's complete, and stays silent while text streams in. Your screen reader reads code blocks as a line-count summary, reads links by their label, and reads tables cell by cell; the full reply stays readable in the transcript.
* **Permission requests and questions**: the extension announces a request when its permission prompt appears, naming the tool Claude wants to use. It announces in the same way when Claude asks you a question and when Claude finishes a plan and waits for your review.
* **Status changes**: the extension announces when Claude starts working, when Claude is ready for your input, and when Claude Code starts compacting the conversation.
* **Errors and model prompts**: the extension announces errors in the conversation, and announces when the [usage-credits consent prompt](/docs/pt/model-config#fable-and-usage-credits) or the [flagged-request prompt](/docs/pt/model-config#ask-before-switching) appears.

While Claude works, your screen reader reads a text label in place of the progress spinner's animation.

When you reopen a session or switch to another one, the extension announces nothing: restored history, pending permission prompts, and in-progress status stay silent until something new happens.

<h3 id="use-the-chat-panel-from-the-keyboard">
  Use the chat panel from the keyboard
</h3>

Each turn in the transcript starts with a visually hidden heading labeled with the prompt that started the turn, so you can jump between turns with your screen reader's heading navigation.

Within a turn, your screen reader announces whose message you're on as you move through it:

* **Your messages**: "You"
* **Claude's messages**: "Claude"
* **Tool steps**: "Claude" plus the tool name, such as "Claude, Bash"
* **Thinking blocks**: "Claude, thinking"

Because the extension exposes the transcript as a labeled region, you can also move focus to the transcript itself with `Tab` and read it at your own pace. To move focus to the newest message or a waiting permission prompt instead, run **Claude Code: Focus last message** from the [Command Palette](#vs-code-commands-and-shortcuts).

When an option on a permission prompt saves a permission rule or directory access, its label ends by naming where the approval is saved, such as "all projects" or "this session". With that option focused, press the `Left` or `Right` arrow key to change the destination, and the extension announces each destination as you move to it. You can also click the destination in the label. The arrow keys require Claude Code v2.1.268 or later.

<h2 id="vs-code-extension-vs-claude-code-cli">
  Extensão VS Code vs. Claude Code CLI
</h2>

Claude Code está disponível tanto como uma extensão VS Code (painel gráfico) quanto como um CLI (interface de linha de comando no terminal). Alguns recursos estão disponíveis apenas no CLI. Se você precisar de um recurso exclusivo do CLI, execute `claude` no terminal integrado do VS Code. Isso requer a [instalação do CLI autônomo](/docs/pt/setup): a extensão não adiciona `claude` ao seu PATH. Veja [Executar CLI no VS Code](#run-cli-in-vs-code).

| Recurso                      | CLI                   | Extensão VS Code                                                                                           |
| ---------------------------- | --------------------- | ---------------------------------------------------------------------------------------------------------- |
| Comandos e skills            | [Todos](/docs/pt/commands) | Subconjunto (digite `/` para ver os disponíveis)                                                           |
| Configuração do servidor MCP | Sim                   | Sim ([adicionar e gerenciar servidores](#connect-to-external-tools-with-mcp) com `/mcp` no painel de chat) |
| Checkpoints                  | Sim                   | Sim                                                                                                        |
| Atalho bash `!`              | Sim                   | Não                                                                                                        |
| Conclusão de abas            | Sim                   | Não                                                                                                        |

<h3 id="rewind-with-checkpoints">
  Rewind com checkpoints
</h3>

A extensão VS Code suporta checkpoints, que rastreiam as edições de arquivo do Claude e permitem que você faça rewind para um estado anterior. Passe o mouse sobre qualquer mensagem para revelar o botão de rewind e escolha entre três opções:

* **Fork conversation from here**: inicie uma nova ramificação de conversa a partir desta mensagem mantendo todas as alterações de código intactas
* **Rewind code to here**: reverta as alterações de arquivo para este ponto na conversa mantendo o histórico completo da conversa
* **Fork conversation and rewind code**: inicie uma nova ramificação de conversa e reverta as alterações de arquivo para este ponto

Para detalhes completos sobre como os checkpoints funcionam e suas limitações, veja [Checkpointing](/docs/pt/checkpointing).

<h3 id="run-cli-in-vs-code">
  Executar CLI no VS Code
</h3>

Para usar o CLI enquanto permanece no VS Code, abra o terminal integrado (`` Ctrl+` `` no Windows/Linux ou `` Cmd+` `` no Mac) e execute `claude`. O CLI se integra automaticamente com seu IDE para recursos como visualização de diff e compartilhamento de diagnósticos.

Instalar a extensão não coloca `claude` no PATH do seu shell. A extensão agrupa uma cópia privada do CLI para seu painel de chat, mas digitar `claude` em um terminal requer a [instalação do CLI autônomo](/docs/pt/setup). Execute a instalação uma vez e os comandos nesta página, incluindo `claude mcp add` e `claude --resume`, funcionam em qualquer terminal. Se `claude` ainda não for encontrado após a instalação, [verifique seu PATH](/docs/pt/troubleshoot-install#verify-your-path).

Se estiver usando um terminal externo, execute `/ide` dentro do Claude Code para conectá-lo ao VS Code.

<h3 id="switch-between-extension-and-cli">
  Alternar entre extensão e CLI
</h3>

A extensão e o CLI compartilham o mesmo histórico de conversa. Para continuar uma conversa de extensão no CLI, execute `claude --resume` no terminal. Isso abre um seletor interativo onde você pode pesquisar e selecionar sua conversa.

<h3 id="include-terminal-output-in-prompts">
  Incluir saída do terminal em prompts
</h3>

Referencie a saída do terminal em seus prompts usando `@terminal:name` onde `name` é o título do terminal. Isso permite que Claude veja a saída do comando, mensagens de erro ou logs sem copiar e colar.

<h3 id="monitor-background-processes">
  Monitorar processos em segundo plano
</h3>

Digite `/tasks` na caixa de prompt para abrir o [mapa de agente](#use-the-prompt-box), que lista as tarefas em segundo plano da sessão, como um servidor de desenvolvimento que Claude deixou em execução como um comando de shell em segundo plano. Clique em uma tarefa para abrir seu cartão e interrompê-la lá. Requer Claude Code v2.1.277 ou posterior.

<h3 id="connect-to-external-tools-with-mcp">
  Conectar a ferramentas externas com MCP
</h3>

MCP (Model Context Protocol) servers dão ao Claude acesso a ferramentas externas, bancos de dados e APIs.

Para gerenciar servidores MCP sem sair do VS Code, digite `/mcp` no painel de chat. A partir do diálogo que se abre, você pode adicionar servidores, remover servidores salvos no [escopo](/docs/pt/mcp#mcp-installation-scopes) local, de usuário ou de projeto, ativar ou desativar servidores, reconectar a um servidor e gerenciar autenticação OAuth. Adicionar e remover servidores no diálogo requer Claude Code v2.1.261 ou posterior.

Você também pode executar `claude mcp add` no terminal integrado do VS Code (`` Ctrl+` `` ou `` Cmd+` ``). O diálogo e o comando do terminal salvam na mesma configuração MCP, e as alterações de qualquer um deles entram em vigor nas conversas que você inicia depois. O exemplo abaixo adiciona o servidor MCP remoto do GitHub, que autentica com um [token de acesso pessoal](https://github.com/settings/personal-access-tokens) passado como um cabeçalho:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Substitua `YOUR_GITHUB_PAT` pelo seu token de acesso pessoal. O comando `claude mcp add` salva a configuração sem validar credenciais, portanto um valor de espaço reservado é aceito aqui, mas o servidor falha ao conectar depois. Para verificar a conexão, inicie uma nova conversa, digite `/mcp` e verifique se o servidor mostra **Connected**. Um servidor com credenciais ruins mostra **Failed**.

Uma vez configurado, peça ao Claude para usar as ferramentas (por exemplo, "Review PR #456").

Para encontrar servidores para conectar, veja [Find and build MCP servers](/docs/pt/mcp#find-and-build-mcp-servers).

<h2 id="work-with-git">
  Trabalhar com git
</h2>

Claude Code integra-se com git para ajudar com fluxos de trabalho de controle de versão diretamente no VS Code. Peça ao Claude para fazer commit de alterações, criar pull requests ou trabalhar em diferentes branches. Para iniciar Claude em uma worktree isolada com seus próprios arquivos e branch, consulte [Executar sessões paralelas com worktrees](/docs/pt/worktrees).

<h3 id="create-commits-and-pull-requests">
  Criar commits e pull requests
</h3>

Claude pode preparar alterações, escrever mensagens de commit e criar pull requests com base no seu trabalho:

```text wrap theme={null}
commit my changes with a descriptive message
create a pr for this feature
summarize the changes I've made to the auth module
```

Ao criar pull requests, Claude gera descrições com base nas alterações de código reais e pode adicionar contexto sobre decisões de teste ou implementação.

<h2 id="use-third-party-providers">
  Usar provedores de terceiros
</h2>

Por padrão, Claude Code se conecta diretamente à API da Anthropic. Se sua organização usa Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry para acessar Claude, configure a extensão para usar seu provedor:

<Steps>
  <Step title="Desabilitar prompt de login">
    Abra a [configuração Desabilitar Prompt de Login](vscode://settings/claudeCode.disableLoginPrompt) e marque a caixa.

    Você também pode abrir as configurações do VS Code (`Cmd+,` no Mac ou `Ctrl+,` no Windows/Linux), procurar por "Claude Code login" e marcar **Desabilitar Prompt de Login**.
  </Step>

  <Step title="Configurar seu provedor">
    Siga o guia de configuração para seu provedor:

    * [Claude Code on Amazon Bedrock](/docs/pt/amazon-bedrock)
    * [Claude Code on Google Cloud's Agent Platform](/docs/pt/google-vertex-ai)
    * [Claude Code on Microsoft Foundry](/docs/pt/microsoft-foundry)

    Estes guias cobrem a configuração de seu provedor em `~/.claude/settings.json`, o que garante que suas configurações sejam compartilhadas entre a extensão do VS Code e a CLI.
  </Step>
</Steps>

Em um provedor de terceiros, a extensão não oferece recursos que exigem uma conta claude.ai, como barras de uso do plano, [ditado por voz](/docs/pt/voice-dictation) e a aba Web para [sessões na nuvem](#resume-cloud-sessions-from-claude-ai). Para o que a caixa de diálogo Conta e uso mostra nestes logins, consulte [Verificar conta e uso](#check-account-and-usage).

Um login claude.ai deixado de um `/login` anterior permanece não utilizado: a extensão não o envia com nenhuma solicitação.

<h2 id="security-and-privacy">
  Segurança e privacidade
</h2>

Seu código permanece privado. Claude Code processa seu código para fornecer assistência, mas não o utiliza para treinar modelos. Para detalhes sobre tratamento de dados e como desativar o registro, consulte [Data and privacy](/docs/pt/data-usage).

Com permissões de auto-edição ativadas, Claude Code pode modificar arquivos de configuração do VS Code (como `settings.json` ou `tasks.json`) que o VS Code pode executar automaticamente. Para reduzir riscos ao trabalhar com código não confiável:

* Ative o [VS Code Restricted Mode](https://code.visualstudio.com/docs/editor/workspace-trust#_restricted-mode) para espaços de trabalho não confiáveis
* Use o modo Manual em vez de Edit automatically ou Auto para edições
* Revise as alterações cuidadosamente antes de aceitá-las

<h3 id="the-built-in-ide-mcp-server">
  The built-in IDE MCP server
</h3>

Quando a extensão está ativa, ela executa um servidor MCP local ao qual a CLI se conecta automaticamente. É assim que a CLI abre diffs no visualizador de diff nativo do VS Code, lê sua seleção atual para menções `@` e — quando você está trabalhando em um notebook Jupyter — pede ao VS Code para executar células.

O servidor é nomeado `ide` e está oculto de `/mcp` porque não há nada para configurar. Se sua organização usa um hook `PreToolUse` para criar uma lista de permissões de ferramentas MCP, porém, você precisará saber que ele existe.

**Selection and open-file context.** Enquanto conectado, a CLI inclui sua seleção atual do editor e o caminho do arquivo ativo como contexto em cada prompt que você envia. A transcrição mostra uma linha `⧉ Selected N lines from <file>` quando isso acontece.

Para excluir um arquivo sensível como `.env`, adicione uma [regra de negação `Read`](/docs/pt/permissions#read-and-edit) para seu caminho. Uma regra de negação correspondente impede que o texto selecionado e o aviso de arquivo aberto para esse arquivo cheguem a Claude.

Se você desativar a configuração [Attach Open File](#extension-settings), a CLI receberá o caminho do arquivo ativo apenas enquanto você tiver texto selecionado nele.

**Transport and authentication.** O servidor se vincula a `127.0.0.1` em uma porta aleatória no intervalo 10000–65535, e a porta não é configurável. O transporte é `ws://` não criptografado; como o socket é apenas loopback, qualquer processo que pudesse capturar o tráfego também pode ler o token do arquivo de bloqueio, portanto TLS não adicionaria proteção. Cada ativação de extensão gera um token de autenticação aleatório novo, o escreve em um arquivo de bloqueio em `~/.claude/ide/<port>.lock`, e a CLI deve apresentá-lo como o cabeçalho `X-Claude-Code-Ide-Authorization` para se conectar. O arquivo de bloqueio tem permissões `0600` em um diretório `0700`, portanto apenas o usuário que executa o VS Code pode lê-lo. Se `CLAUDE_CONFIG_DIR` estiver definido, o arquivo de bloqueio será escrito em `$CLAUDE_CONFIG_DIR/ide/` em vez disso.

**Tools exposed to the model.** O servidor hospeda uma dúzia de ferramentas, mas apenas duas são visíveis para o modelo. O resto é RPC interno que a CLI usa para sua própria interface — abrindo diffs, lendo seleções, salvando arquivos — e são filtrados antes da lista de ferramentas chegar a Claude.

| Tool name (as seen by hooks) | What it does                                                                                                                          | Read-only |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | --------- |
| `mcp__ide__getDiagnostics`   | Retorna diagnósticos do servidor de linguagem — os erros e avisos no painel Problems do VS Code. Opcionalmente limitado a um arquivo. | Yes       |
| `mcp__ide__executeCode`      | Executa código Python no kernel do notebook Jupyter ativo. Consulte o fluxo de confirmação abaixo.                                    | No        |

**Jupyter execution always asks first.** `mcp__ide__executeCode` não pode executar nada silenciosamente. Em cada chamada, o código é inserido como uma nova célula no final do notebook ativo, o VS Code a rola para a visualização, e um Quick Pick nativo pede para você **Execute** ou **Cancel**. Cancelar — ou descartar o seletor com `Esc` — retorna um erro a Claude e nada é executado. A ferramenta também se recusa completamente quando não há um notebook ativo, quando a extensão Jupyter (`ms-toolsai.jupyter`) não está instalada, ou quando o kernel não é Python.

<Note>
  A confirmação do Quick Pick é separada dos hooks `PreToolUse`. Uma entrada de lista de permissões para `mcp__ide__executeCode` permite que Claude *proponha* executar uma célula; o Quick Pick dentro do VS Code é o que permite que ela *realmente* seja executada.
</Note>

<a id="troubleshooting" />

<h2 id="fix-common-issues">
  Corrigir problemas comuns
</h2>

<h3 id="extension-won’t-install">
  A extensão não é instalada
</h3>

* Certifique-se de que você tem uma versão compatível do VS Code (1.94.0 ou posterior)
* Verifique se o VS Code tem permissão para instalar extensões
* Tente instalar diretamente do [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=anthropic.claude-code)

<h3 id="spark-icon-not-visible">
  Ícone Spark não visível
</h3>

O ícone Spark aparece na **Editor Toolbar** (canto superior direito do editor) quando você tem um arquivo aberto. Se você não o vir:

1. **Abra um arquivo**: O ícone requer que um arquivo esteja aberto. Apenas ter uma pasta aberta não é suficiente.
2. **Verifique a versão do VS Code**: Requer 1.94.0 ou superior (Help → About)
3. **Reinicie o VS Code**: Execute "Developer: Reload Window" na Paleta de Comandos
4. **Desabilite extensões conflitantes**: Desabilite temporariamente outras extensões de IA (Cline, Continue, etc.)
5. **Verifique a confiança do workspace**: A extensão não funciona em Modo Restrito

Alternativamente, se você definiu [`preferredLocation`](#extension-settings) como `sidebar`, ou abriu Claude com **Claude Code: Open in Side Bar**, clique em "✻ Claude Code" na **Status Bar** (canto inferior direito). Isso funciona mesmo sem um arquivo aberto. Você também pode usar a **Paleta de Comandos** (`Cmd+Shift+P` / `Ctrl+Shift+P`) e digitar "Claude Code".

<h3 id="cmd-esc-does-nothing-on-macos">
  Cmd+Esc não faz nada no macOS
</h3>

No macOS Tahoe e posterior, o atalho do sistema Game Overlay está vinculado a `Cmd+Esc` por padrão e intercepta a tecla antes que ela chegue ao VS Code. Para liberar o atalho:

1. Abra System Settings
2. Vá para Keyboard, depois Keyboard Shortcuts, depois Game Controllers
3. Desmarque a caixa de seleção Game Overlay

Alternativamente, reassine a extensão para uma tecla diferente: abra o editor de [Keyboard Shortcuts](https://code.visualstudio.com/docs/configure/keybindings) do VS Code (`Cmd+K Cmd+S`), procure por `Claude Code: Focus input`, e atribua uma nova vinculação.

<h3 id="claude-code-never-responds">
  Claude Code nunca responde
</h3>

Se Claude Code não está respondendo aos seus prompts:

1. **Verifique sua conexão com a internet**: Certifique-se de que você tem uma conexão com a internet estável
2. **Inicie uma nova conversa**: Tente iniciar uma conversa nova para ver se o problema persiste
3. **Tente a CLI**: Execute `claude` no terminal para ver se você obtém mensagens de erro mais detalhadas

Se os problemas persistirem, [abra uma issue no GitHub](https://github.com/anthropics/claude-code/issues) com detalhes sobre o erro.

<h2 id="uninstall-the-extension">
  Desinstalar a extensão
</h2>

Para desinstalar a extensão Claude Code:

1. Abra a visualização de Extensões (`Cmd+Shift+X` no Mac ou `Ctrl+Shift+X` no Windows/Linux)
2. Procure por "Claude Code"
3. Clique em **Desinstalar**

Se você executar `claude` em um terminal integrado do VS Code, Claude Code reinstala a extensão automaticamente. Para mantê-la desinstalada, desative **Auto-install IDE extension** em `/config`, ou defina [`autoInstallIdeExtension`](/docs/pt/settings-reference#autoinstallideextension) como `false`. Você também pode definir a variável de ambiente [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/pt/env-vars) como `1`.

Para também remover dados da extensão e redefinir todas as configurações, delete o diretório de armazenamento da extensão para sua plataforma.

No macOS:

```bash theme={null}
rm -rf ~/Library/"Application Support"/Code/User/globalStorage/anthropic.claude-code
```

No Linux:

```bash theme={null}
rm -rf ~/.config/Code/User/globalStorage/anthropic.claude-code
```

No Windows, no PowerShell:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:APPDATA\Code\User\globalStorage\anthropic.claude-code"
```

Para obter ajuda adicional, consulte o [guia de solução de problemas](/docs/pt/troubleshooting).

<h2 id="next-steps">
  Próximos passos
</h2>

Agora que você tem Claude Code configurado no VS Code:

* [Explore fluxos de trabalho comuns](/docs/pt/common-workflows) para aproveitar ao máximo Claude Code
* [Configure servidores MCP](/docs/pt/mcp) para estender os recursos do Claude com ferramentas externas. Adicione e gerencie-os com `/mcp` no painel de chat.
* [Configure as definições do Claude Code](/docs/pt/settings) para personalizar comandos permitidos, hooks e muito mais. Essas configurações são compartilhadas entre a extensão e CLI.
