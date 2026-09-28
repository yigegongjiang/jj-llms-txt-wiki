> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code セッションのチームを調整する

> 複数の Claude Code インスタンスがチームとして連携して動作するように調整し、共有タスク、エージェント間メッセージング、および一元管理を実現します。

<Warning>
  エージェントチームは実験的機能であり、デフォルトでは無効になっています。[settings.json](/docs/ja/settings) または環境に `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` を設定して有効にしてください。その変数がない場合、セッション開始時にチームが設定されず、チームディレクトリが書き込まれず、Claude はチームメンバーをスポーンまたは提案しません。エージェントチームには、セッション再開、タスク調整、シャットダウン動作に関する[既知の制限](#limitations)があります。
</Warning>

エージェントチームを使用すると、複数の Claude Code インスタンスが連携して動作するように調整できます。1 つのセッションがチームリーダーとして機能し、作業を調整し、タスクを割り当て、結果を統合します。チームメンバーは独立して動作し、それぞれ独自のコンテキストウィンドウで動作し、互いに直接通信します。リーダーを経由せずに、任意のチームメンバーと直接対話することもできます。

チームを設定する前に、より軽量なオプションで十分かどうかを確認してください。[Subagents](/docs/ja/sub-agents) は単一セッション内で動作し、[クロスセッションメッセージング](/docs/ja/cross-session-messaging) を使用すると Claude は自分で実行するセッション間で検出結果を渡すことができます。

<h2 id="when-to-use-agent-teams">
  エージェントチームを使用する場合
</h2>

エージェントチームは、並列探索が実際の価値を追加するタスクに最も効果的です。完全なシナリオについては、[ユースケース例](#use-case-examples)を参照してください。最も強力なユースケースは以下の通りです。

* **調査とレビュー**：複数のチームメンバーが問題のさまざまな側面を同時に調査し、その後、各自の調査結果を共有して相互に検証できます
* **新しいモジュールまたは機能**：チームメンバーが互いに干渉することなく、それぞれ別々の部分を担当できます
* **競合する仮説でのデバッグ**：チームメンバーが異なる理論を並列でテストし、より迅速に答えに収束できます
* **クロスレイヤー調整**：フロントエンド、バックエンド、テストにまたがる変更で、それぞれ異なるチームメンバーが担当します

エージェントチームは調整オーバーヘッドを追加し、単一セッションよりも大幅に多くのトークンを使用します。チームメンバーが独立して動作できる場合に最も効果的です。順序付きタスク、同じファイルの編集、または多くの依存関係を持つ作業の場合は、単一セッションまたは [subagents](/docs/ja/sub-agents) がより効果的です。

<h3 id="compare-with-subagents">
  subagents との比較
</h3>

エージェントチームと [subagents](/docs/ja/sub-agents) の両方を使用すると、作業を並列化できますが、動作方法が異なります。チームなしでメッセージを相互に渡す別々のセッションについては、[クロスセッションメッセージング](/docs/ja/cross-session-messaging)を参照してください。

<Frame caption="Subagents は結果をメインエージェントに報告します。エージェントチームでは、チームメンバーがタスクリストを共有し、作業を要求し、互いに直接通信します。">
  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-light.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=2f8db9b4f3705dd3ab931fbe2d96e42a" className="dark:hidden" alt="Subagent とエージェントチームのアーキテクチャを比較する図。Subagents はメインエージェントによって生成され、作業を実行し、結果を報告します。エージェントチームは共有タスクリストを通じて調整され、チームメンバーが互いに直接通信します。" width="4245" height="1615" data-path="images/subagents-vs-agent-teams-light.png" />

  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-dark.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=d573a037540f2ada6a9ae7d8285b46fd" className="hidden dark:block" alt="Subagent とエージェントチームのアーキテクチャを比較する図。Subagents はメインエージェントによって生成され、作業を実行し、結果を報告します。エージェントチームは共有タスクリストを通じて調整され、チームメンバーが互いに直接通信します。" width="4245" height="1615" data-path="images/subagents-vs-agent-teams-dark.png" />
</Frame>

|             | Subagents                                                                                                   | エージェントチーム                                                                                     |
| :---------- | :---------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------- |
| **コンテキスト**  | 独自のコンテキストウィンドウ。結果は呼び出し元に返される                                                                                | 独自のコンテキストウィンドウ。完全に独立                                                                          |
| **通信**      | 呼び出し元に結果を返します。Claude が生成した際に名前を付けた Subagents は、[互いにメッセージを送信](/docs/ja/sub-agents#what-loads-at-startup)することもできます | チームメンバーが互いに直接メッセージを送信                                                                         |
| **調整**      | メインエージェントがすべての作業を管理                                                                                         | メッセージを通じた自己調整、および [Task ツールを持つエージェント](/docs/ja/tools-reference#task-tool-availability)のための共有タスクリスト |
| **最適な用途**   | 結果のみが重要な焦点を絞ったタスク                                                                                           | 議論と協力が必要な複雑な作業                                                                                |
| **トークンコスト** | 低い：結果がメインコンテキストに要約されて返される                                                                                   | 高い：各チームメンバーが個別の Claude インスタンス                                                                 |

結果を報告する必要がある迅速で焦点を絞ったワーカーが必要な場合は subagents を使用してください。チームメンバーが調査結果を共有し、互いに検証し、独立して調整する必要がある場合は、エージェントチームを使用してください。

<h2 id="enable-agent-teams">
  エージェントチームを有効にする
</h2>

エージェントチームはデフォルトでは無効になっています。`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` 環境変数を `1` に設定して有効にしてください。シェル環境または [settings.json](/docs/ja/settings) を通じて設定できます。

```json settings.json theme={null}
{
  "env": {
    "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1"
  }
}
```

エージェントチームを有効にすると、通常の委譲も変わります。Claude は [サブエージェントに名前を付ける](/docs/ja/sub-agents#subagent-names) ことができます。エージェントチームが有効な間、Claude が名前を付けたサブエージェントはチームメイトとして起動するため、リクエストしなくてもチームが形成される可能性があります。詳細については、[Claude がエージェントチームを開始する方法](#how-claude-starts-agent-teams) を参照してください。この動作をオフにするには、[Claude はサブエージェントの代わりにチームメイトを生成する](#claude-spawns-teammates-instead-of-subagents) を参照してください。

チームメイトの生成には、インタラクティブセッションも必要です。`-p` フラグを含む [非インタラクティブモード](/docs/ja/headless) では、Agent SDK セッションを含めて、Claude はチームメイトを生成せず、Claude が名前を付けたサブエージェントはエージェントチームが有効な場合でも通常の [サブエージェント](/docs/ja/sub-agents) として実行されます。

<h2 id="start-your-first-agent-team">
  最初のエージェントチームを開始する
</h2>

エージェントチームを有効にした後、実行したいタスクとチームメンバーを自然言語で説明してください。Claude がチームメンバーを生成し、プロンプトに基づいて作業を調整します。

この例は、3 つの役割が独立しており、互いに待つことなく問題を探索できるため、うまく機能します。

```text wrap theme={null}
I'm designing a CLI tool that helps developers track TODO comments across
their codebase. Spawn three teammates to explore this from different angles:
one on UX, one on technical architecture, one playing devil's advocate.
```

その後、Claude は [共有タスクリスト](/docs/ja/interactive-mode#task-list) を作成し、[Task ツールを持つセッション](/docs/ja/tools-reference#task-tool-availability)内で各視点のチームメンバーを生成し、問題を探索させ、完了時に調査結果を統合します。

Claude は時々、チームを作成する代わりに [サブエージェント](/docs/ja/sub-agents)を使用することがあります。サブエージェントはチームメンバーと同じエージェントパネルに表示されるため、パネルだけではチームが形成されたことを確認できません。Claude がサブエージェントを生成した場合は、もう一度質問し、明示的にエージェントチームをリクエストしてください。

リーダーのターミナルには、プロンプト入力の下のエージェントパネルにチームメンバーが表示されます。パネルから以下の操作ができます。

* **上下矢印**: チームメンバーを選択する
* **Enter**: 選択したチームメンバーのトランスクリプトを開き、直接メッセージを送信する
* **Escape**: 選択を解除します。チームメンバーのトランスクリプトを表示している間、Escape はそのチームメンバーの現在のターンを中断します

v2.1.199 以降、アイドル状態のチームメンバーの行は、他のチームメンバーまたはサブエージェントがまだ作業中の間、パネルに留まるため、トランスクリプトを確認したり、さらに作業を割り当てたりするために選択できます。パネル内のすべてのエージェントがアイドル状態になると、アイドル行は 30 秒後に非表示になり、チームメンバーの次のターンで再表示されます。チームメンバーは非表示中も実行中で対応可能な状態が続きます。v2.1.181 から v2.1.198 では、アイドル行は他のチームメンバーがまだ作業中であっても、独自のターンが終了してから 30 秒後に非表示になりました。v2.1.181 より前のバージョンではアイドル行は非表示になりません。

3 人以上のチームメンバーが同時にアイドル状態の場合、最初の 3 行を超える行は、折りたたまれたチームメンバーをカウントする単一の行に折りたたまれます。例えば、5 人がアイドル状態の場合は `2 idle agents` のようになります。それを選択して Enter キーを押すと折りたたまれた行が展開され、Esc キーを押すと再び折りたたまれます。作業中のチームメンバー、失敗したチームメンバー、および表示中のチームメンバーは常に独自の行を保持します。

各チームメンバーを独自の分割ペインに配置したい場合は、[表示モードを選択](#choose-a-display-mode)を参照してください。

<h2 id="control-your-agent-team">
  エージェントチームを制御する
</h2>

リーダーに自然言語で実行したい内容を指示してください。チーム調整、タスク割り当て、および指示に基づいた委任を処理します。

<h3 id="choose-a-display-mode">
  表示モードを選択する
</h3>

エージェントチームは 2 つの表示モードをサポートしています。

* **In-process**：すべてのチームメンバーがメインターミナル内で実行されます。エージェントパネルで上下矢印キーを使用してチームメンバーを選択し、Enter キーを押してそれを表示して、直接メッセージを入力してください。追加のセットアップなしで任意のターミナルで動作します。
* **分割ペイン**：各チームメンバーが独自のペインを取得します。すべてのユーザーの出力を一度に表示でき、ペインをクリックして直接対話できます。tmux または iTerm2 が必要です。

<Note>
  `tmux` には特定のオペレーティングシステムでの既知の制限があり、従来は macOS で最も効果的に動作します。iTerm2 で `tmux -CC` を使用することが、`tmux` への推奨エントリーポイントです。
</Note>

デフォルトは `"in-process"` です。`"auto"` を設定して、既に tmux セッション内で実行している場合または使用しているターミナルが iTerm2 で `it2` CLI がインストールされている場合は分割ペインを有効にし、それ以外の場合は in-process にフォールバックします。`"tmux"` 設定は分割ペインモードを有効にし、ターミナルに基づいて tmux または iTerm2 を使用するかどうかを自動検出します。

`"iterm2"` を設定して iTerm2 ネイティブ分割ペインを明示的に使用してください。このモードは [`it2` CLI](https://github.com/mkusaka/it2) が必要で、`it2` が見つからない場合はインストールコマンド付きでエラーを表示します。`it2` をインストールするか tmux に切り替えるオプションを提供するセットアッププロンプトは、ターミナルが iTerm2 で tmux がフォールバックとして利用可能な場合、`"auto"` または `"tmux"` の下に表示されます。

デフォルトをオーバーライドするには、`~/.claude/settings.json` で [`teammateMode`](/docs/ja/settings-reference#teammatemode) を設定してください。

```json theme={null}
{
  "teammateMode": "auto"
}
```

単一セッションに対してモードを設定するには、フラグとして渡してください。

```bash theme={null}
claude --teammate-mode auto
```

`--teammate-mode` フラグは実験的であり、`claude --help` に表示されません。

分割ペインモードには、[tmux](https://github.com/tmux/tmux/wiki) または [`it2` CLI](https://github.com/mkusaka/it2) を備えた iTerm2 が必要です。手動でインストールするには、以下を実行してください。

* **tmux**：システムのパッケージマネージャーを通じてインストールしてください。プラットフォーム固有の手順については、[tmux wiki](https://github.com/tmux/tmux/wiki/Installing) を参照してください。
* **iTerm2**：[`it2` CLI](https://github.com/mkusaka/it2) をインストールし、**iTerm2 → Settings → General → Magic → Enable Python API** で Python API を有効にしてください。

<h3 id="specify-teammates-and-models">
  チームメンバーとモデルを指定する
</h3>

Claude はタスクに基づいて生成するチームメンバーの数を決定するか、正確に実行したい内容を指定できます。

```text wrap theme={null}
Spawn 4 teammates to refactor these modules in parallel. Use Sonnet for
each teammate.
```

Claude Code は各チームメンバーのモデルを、以下の最初に適用されるものから選択します。

1. スポーンプロンプトがそのチームメンバーに対して指定するモデル。
2. [サブエージェント定義](#use-subagent-definitions-for-teammates)からスポーンされたチームメンバーの場合、定義の `model`。ここで `inherit` はリーダーのモデルを選択します。
3. [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/ja/model-config#environment-variables)。`inherit` 以外に設定されている場合。
4. リーダーの現在のモデル。

[`CLAUDE_CODE_SUBAGENT_MODEL_FORCE=1`](/docs/ja/sub-agents#run-every-subagent-on-one-model) を設定した場合、最初の 2 つのソースは適用されません。Claude Code は `CLAUDE_CODE_SUBAGENT_MODEL` が `inherit` 以外に設定されている場合はそこからすべてのチームメンバーのモデルを選択し、それ以外の場合はリーダーの現在のモデルから選択します。Claude Code v2.1.257 以降が必要です。

v2.1.251 より前は、`CLAUDE_CODE_SUBAGENT_MODEL` がこの順序で最初に来ていました。

<Note>
  `teammateDefaultModel` は v2.1.234 で削除されました。Claude Code は残された値を無視します。代わりにプロンプトでモデルを指定してください。
</Note>

Claude Code は、チームメンバーに対して選択したモデルを、組織の [`availableModels`](/docs/ja/model-config#restrict-model-selection) 許可リストに対してチェックします。許可リストが値をブロックすると、Claude Code は別のモデルを代替します。

* **`opus` などのファミリーエイリアス**：Anthropic API および AWS 上の Claude Platform では、Claude Code はチームメンバーを許可リストが許可するそのファミリーの最新バージョンで実行します。プロバイダー固有のモデル ID を持つプロバイダーでは、[代替が動作しない](/docs/ja/model-config#restrict-model-selection)場合、ブロックされたエイリアスは次の箇条書きに従って他のブロックされた値のようにフォールバックします。
* **プロバイダー固有のモデル ID を持つプロバイダーで代替が動作しないファミリーエイリアス、またはそのファミリーに許可されたバージョンがないもの、またはその他のブロックされた値**：Claude Code はチームメンバーをリーダーのモデルで実行します。`CLAUDE_CODE_SUBAGENT_MODEL` を設定した場合、Claude Code はそのモデルを最初に試し、これらの同じルールの下で実行します。

チームメンバーはリーダーの[努力レベル](/docs/ja/model-config#adjust-effort-level)を継承します。分割ペインモードではこれは v2.1.186 から適用されます。それより前のバージョンではリーダーのセッション努力を分割ペインチームメンバーに渡しませんでした。

<h3 id="have-teammates-plan-before-implementing">
  チームメンバーが実装前にプランを立てるようにする
</h3>

複雑またはリスクの高いタスクの場合、チームメンバーが実装前にプランを立てることができます。リーダーが[プランモード](/docs/ja/permission-modes#analyze-before-you-edit-with-plan-mode)にある間に Claude がスポーンするチームメンバーは、プランが準備できるまで読み取り専用プランモードで動作します。まずリーダーをプランモードに切り替えてから、チームメンバーを要求してください。

```text wrap theme={null}
Spawn an architect teammate to refactor the authentication module.
```

チームメンバーがプランを完了すると、リーダーにプラン承認リクエストを送信します。Claude Code はリクエストが到着するとすぐにリーダーのセッションでプランを承認し、リーダーがそれをレビューすることはありません。チームメンバーの編集とコマンドは、[権限](#permissions)で説明されている権限プロンプトを通じて実行されます。承認されると、チームメンバーはプランモードを終了し、実装を開始します。

<h3 id="talk-to-teammates-directly">
  チームメンバーと直接通信する
</h3>

各チームメンバーは、完全で独立した Claude Code セッションです。任意のチームメンバーに直接メッセージを送信して、追加の指示を与えたり、フォローアップの質問をしたり、アプローチをリダイレクトしたりできます。

* **In-process モード**：エージェントパネルで上下矢印キーを使用してチームメンバーを選択し、Enter キーを押してそのセッションを表示して、メッセージを入力してください。選択したチームメンバーで `x` を押してそれを停止してください。Ctrl+T を押してタスクリストを切り替えてください。
* **分割ペインモード**：チームメンバーのペインをクリックして、セッションと直接対話してください。各チームメンバーは独自のターミナルの完全なビューを持っています。

In-process チームメンバーを表示している間、プレーンテキストと [skills](/docs/ja/skills) はそのチームメンバーに送信されますが、組み込みコマンドはリーダーのセッションで実行されます。

チームメンバーのモデルと高速モードはそれが生成されるときに固定されるため、`/model` と `/fast` はリーダーの設定のみを変更します。v2.1.199 以降、チームメンバーを表示している間にいずれかのコマンドを入力すると、変更がリーダーに適用されることを示す通知が表示されます。それより前のバージョンでは、指示なしでリーダーに適用されました。`/effort` はチームメンバーの後続のターンに適用されます。これはチームメンバーがリーダーの[努力レベル](/docs/ja/model-config#adjust-effort-level)に従うためです。

<h3 id="assign-and-claim-tasks">
  タスクを割り当てて要求する
</h3>

共有タスクリストはチーム全体の作業を調整します。リーダーがタスクを作成し、チームメンバーがそれらを処理します。タスクには 3 つの状態があります。保留中、進行中、完了。タスクは他のタスクに依存することもできます。未解決の依存関係を持つ保留中のタスクは、それらの依存関係が完了するまで要求できません。

[Task ツールを持たない](/docs/ja/tools-reference#task-tool-availability)エージェントは、共有タスクリストの代わりにメッセージを通じて調整します。

リーダーはタスクを明示的に割り当てるか、チームメンバーが自己要求できます。

* **リーダーが割り当て**：リーダーにどのタスクをどのチームメンバーに与えるかを指示してください
* **自己要求**：タスクを完了した後、チームメンバーは独立して次の未割り当て、ブロック解除されたタスクを選択します

タスク要求はファイルロックを使用して、複数のチームメンバーが同時に同じタスクを要求しようとするときの競合状態を防ぎます。

<h3 id="shut-down-teammates">
  チームメンバーをシャットダウンする
</h3>

チームメンバーのセッションを適切に終了するには、名前で参照してください。例えば、researcher という名前のチームメンバーの場合：

```text wrap theme={null}
Ask the researcher teammate to shut down
```

リーダーはシャットダウンリクエストを送信します。チームメンバーは承認して適切に終了するか、説明付きで却下できます。

チームの共有ディレクトリは、セッションが終了すると自動的にクリーンアップされるため、別のクリーンアップステップはありません。削除されるディレクトリと、再開されたセッションのために保持されるディレクトリについては、[Architecture](#architecture) を参照してください。

<h3 id="enforce-quality-gates-with-hooks">
  hooks で品質ゲートを実施する
</h3>

[hooks](/docs/ja/hooks) を使用して、チームメンバーが作業を完了したときまたはタスクが作成または完了したときのルールを実施してください。

* [`TeammateIdle`](/docs/ja/hooks#teammateidle)：チームメンバーがアイドル状態になろうとしているときに実行されます。終了コード 2 でフィードバックを送信し、チームメンバーを動作させ続けてください。
* [`TaskCreated`](/docs/ja/hooks#taskcreated)：タスクが作成されているときに実行されます。終了コード 2 で作成を防止し、フィードバックを送信してください。
* [`TaskCompleted`](/docs/ja/hooks#taskcompleted)：タスクが完了としてマークされているときに実行されます。終了コード 2 で完了を防止し、フィードバックを送信してください。

<h2 id="how-agent-teams-work">
  エージェントチームの動作方法
</h2>

このセクションでは、エージェントチームの背後にあるアーキテクチャとメカニクスについて説明します。使用を開始したい場合は、上記の [エージェントチームを制御する](#control-your-agent-team) を参照してください。

<h3 id="how-claude-starts-agent-teams">
  Claude がエージェントチームを開始する方法
</h3>

チームを開始するには、Claude にチームメンバーをリクエストしてください。Claude は [Agent ツール](/docs/ja/tools-reference) を呼び出して [`name`](/docs/ja/sub-agents#subagent-names) を指定し、エージェントチームが有効になっている場合にチームメンバーを起動します。ただし、呼び出しが [fork](/docs/ja/sub-agents#fork-the-current-conversation) であるか、呼び出し自体で `isolation` を渡す場合は除きます。Claude Code があなたに確認を求めることはありません。

Claude は通常の subagent にも独自に名前を付けるため、後でメッセージを送信できます。これらの呼び出しは同じルールに従うため、チームメンバーをリクエストしなくてもチームが形成される可能性があります。subagent の代わりに使用したい場合は、[エージェントチームをオフにしてください](#claude-spawns-teammates-instead-of-subagents)。

<h3 id="architecture">
  アーキテクチャ
</h3>

エージェントチームは以下で構成されています。

| コンポーネント     | 役割                                       |
| :---------- | :--------------------------------------- |
| **チームリーダー** | チームメンバーを生成し、作業を調整するメイン Claude Code セッション |
| **チームメンバー** | 割り当てられたタスクで動作する個別の Claude Code インスタンス    |
| **タスクリスト**  | チームメンバーが要求して完了する共有作業項目リスト                |
| **メールボックス** | エージェント間の通信用メッセージングシステム                   |

各エージェントのメールボックスは `~/.claude/teams/{team-name}/inboxes/{agent-name}.json` にある JSON ファイルです。Claude Code はメールボックスファイルを読み取るときにすべてのエントリを検証します。メッセージ形式と一致しないエントリはエラーとして報告され、ファイルから削除されます。有効なメッセージは引き続き配信されます。v2.1.207 より前では、1 つの不正なメールボックスエントリが毎秒繰り返しエラーを引き起こし、ファイルを手動で削除するまでそのメールボックスの配信をブロックしていました。

Claude Code は、メッセージがプレーンテキストであるか、プラン承認やシャットダウンリクエストなどの構造化プロトコルメッセージであるかに関わらず、受信者のメールボックスファイルへの書き込みが成功した場合にのみメッセージを送信済みとして報告します。ディスクがいっぱいであるか、メールボックスディレクトリが書き込み可能でないなど、書き込みが失敗した場合、送信エージェントはエラーを受け取り、何も送信されません。エラーメッセージと復旧手順については、[チームメンバーのインボックスへの書き込みに失敗しました](/docs/ja/errors#failed-to-write-to-a-teammate-inbox) を参照してください。

Claude Code はタスク依存関係を自動的に管理します。チームメンバーが他のタスクが依存するタスクを完了すると、ブロックされたタスクはあなたからのアクションなしにブロック解除されます。

チームとタスクはセッション派生名でローカルに保存されます。名前は `session-` の後にセッション ID の最初の 8 文字が続きます。

* **チーム設定**：`~/.claude/teams/{team-name}/config.json`
* **タスクリスト**：`~/.claude/tasks/{team-name}/`

Claude Code はセッション起動時にこれらの両方を自動的に生成し、チームメンバーが参加、アイドル状態になる、または離脱するときに更新します。チーム設定ディレクトリはセッション終了時に削除されます。タスクリストディレクトリはローカルに保持され、アップロードされることはないため、再開されたセッションはタスクを保持します。保持期間は、セッショントランスクリプト用に既に制御している同じ [`cleanupPeriodDays`](/docs/ja/settings-reference#cleanupperioddays) によって管理され、[保持スイープルール](/docs/ja/claude-directory#cleaned-up-automatically) に従います。

チーム設定はセッション ID と tmux ペイン ID などのランタイム状態を保持しているため、手動で編集したり、事前に作成したりしないでください。次の状態更新時に変更が上書きされます。

再利用可能なチームメンバーロールを定義するには、代わりに [subagent 定義を使用](#use-subagent-definitions-for-teammates) してください。

チーム設定には、各メンバーの名前とエージェント ID を含む `members` 配列が含まれています。リーダーのエントリは常にエージェントタイプ `team-lead` を持ちます。チームメンバーのエントリは、リーダーが生成時に名前を付けたエージェントタイプ（[組み込みタイプ](/docs/ja/sub-agents#built-in-subagents) または [subagent 定義](#use-subagent-definitions-for-teammates) のいずれか）を持ち、リーダーが何も名前を付けなかった場合はフィールドを省略します。チームメンバーはこのファイルを読み取って、他のチームメンバーを発見できます。

プロジェクトレベルのチーム設定に相当するものはありません。プロジェクトディレクトリ内の `.claude/teams/teams.json` のようなファイルは設定として認識されません。Claude はそれを通常のファイルとして扱います。

<h3 id="use-subagent-definitions-for-teammates">
  チームメンバーに subagent 定義を使用する
</h3>

どちらの表示モードでもチームメンバーを生成するときに、プロジェクト、ユーザー、または管理対象の [subagent スコープ](/docs/ja/sub-agents#choose-the-subagent-scope) から [subagent](/docs/ja/sub-agents) タイプを参照できます。これにより、セキュリティレビュアーやテストランナーなどのロールを 1 回定義し、委任された subagent とエージェントチームチームメンバーの両方として再利用できます。

subagent 定義を使用するには、Claude にチームメンバーを生成するよう指示するときに名前で言及してください。

```text wrap theme={null}
Spawn a teammate using the security-reviewer agent type to audit the auth module.
```

Claude Code は名前を付けた subagent 定義を読み取り、これらの部分をチームメンバーに適用します。部分がチームメンバーの [表示モード](#choose-a-display-mode) に依存する場合、エントリはそのことを示します。

* **`tools`**：Claude Code はチームメンバーを定義の `tools` リスト内のツールに制限します。インプロセスチームメンバーの場合、Claude Code はそのリストに `SendMessage` を追加し、[Task ツールを持つセッション](/docs/ja/tools-reference#task-tool-availability) では `TaskCreate`、`TaskGet`、`TaskList`、および `TaskUpdate` も追加します。
* **`model`**：Claude Code は、生成プロンプトが 1 つを名前で指定しない場合、どちらの表示モードでも定義の `model` を使用します。[Claude Code がチームメンバーのモデルを選択する方法](#specify-teammates-and-models) を参照してください。
* **本体**：インプロセスチームメンバーの場合、Claude Code は定義の本体をデフォルトシステムプロンプトに追加の指示として追加します。分割ペインチームメンバーの場合、Claude Code はデフォルトシステムプロンプトの代わりに本体を使用します。
* **`skills`**：Claude Code はどちらの表示モードでもチームメンバーに定義の `skills` を適用しません。チームメンバーはプロジェクトおよびユーザー設定から skills をロードします。
* **`mcpServers`**：分割ペインチームメンバーの場合、Claude Code は [そのフィールドのルール](/docs/ja/sub-agents#scope-mcp-servers-to-a-subagent) に従って定義の `mcpServers` を適用します。これは `--agent` で開始されたセッションもカバーします。インプロセスチームメンバーはフィールドを無視し、プロジェクトおよびユーザー設定から MCP サーバーをロードします。

Claude がインプロセスチームメンバーにメッセージを送信し、そのチームメンバーがもう実行されていない場合、Claude Code はそれを同じセッション内に戻し、保存されている会話を復元し、メッセージを次のプロンプトとして提供します。セッションを再開した後、チームメンバーはこの方法では戻されません。[再開の制限](#limitations) に従います。

Claude Code が戻すチームメンバーについて、プロジェクトの `.claude/agents/` ディレクトリまたは `--add-dir` ディレクトリから来た定義は、[エージェントファイルが存在するフォルダを信頼している](/docs/ja/permissions#what-runs-before-you-trust-a-folder) 場合にのみ再適用されます。親フォルダを信頼することはカウントされません。それまで、チームメンバーはすべてのインプロセスチームメンバーに Claude Code が追加するツールのみを保持して、定義のツールまたは指示なしで戻ります。[チームメンバーのエージェント定義が復元されませんでした](/docs/ja/errors#teammate-agent-definition-not-restored) を参照して、通知テキストを確認してください。

<h3 id="permissions">
  権限
</h3>

チームメンバーはリーダーの権限モードで開始します。ただし、[`dontAsk` モード](/docs/ja/permission-modes#allow-only-pre-approved-tools-with-dontask-mode) は継承しません。リーダーが `--dangerously-skip-permissions` で実行する場合、すべてのチームメンバーも同様に実行します。生成後、個別のチームメンバーの権限モードを変更できますが、生成時にチームメンバーごとの権限モードを設定することはできません。

チームメンバーの権限プロンプトはリーダーセッションに表示されるため、そこで自分で承認してください。[プラン承認](#have-teammates-plan-before-implementing) は設計された例外です。リーダーセッションはあなたへの別のプロンプトなしにチームメンバープラン承認を付与します。

<h4 id="messages-between-agents">
  エージェント間のメッセージ
</h4>

1 つのエージェントが `SendMessage` 経由で別のエージェントにメッセージを送信する場合、受信エージェントには、あなたからではなく別の Claude セッションから来たことが通知されます。チームメンバーは権限プロンプトを承認したり、あなたに代わって同意を提供したりすることはできません。また、アクションが拒否されたチームメンバーは、チェックをバイパスするために別のチームメンバーにそれをリレーすることはできません。[チーム外の他の Claude Code セッション](/docs/ja/cross-session-messaging#how-a-session-treats-an-incoming-message) から到着するメッセージにも同じルールが適用されます。

[auto モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode) では、分類器はエージェント間のメッセージに 2 つのチェックを適用します。

* 別のエージェントからリレーされた承認クレームは、あなたからの確認ではなく、信頼できない入力として扱われます。
* シャットダウンリクエストやプラン承認応答などの構造化プロトコルメッセージであるかどうかに関わらず、プレーンメッセージであるかどうかに関わらず、各メッセージを Claude Code が配信する前にレビューします。ブロックされたメッセージは受信者に到達することはありません。

<h3 id="context-and-communication">
  コンテキストと通信
</h3>

各チームメンバーは独自のコンテキストウィンドウを持っています。生成されると、チームメンバーは通常のセッションと同じプロジェクトコンテキストをロードします。CLAUDE.md、MCP サーバー、および skills。また、リーダーからの生成プロンプトを受け取ります。リーダーの会話履歴は引き継がれません。

**チームメンバーが情報を共有する方法：**

* **自動メッセージ配信**：チームメンバーがメッセージを送信すると、受信者に自動的に配信されます。リーダーは更新をポーリングする必要はありません。
* **アイドル通知**：チームメンバーが完了して停止すると、リーダーに自動的に通知し、最終的な回答を通知に含めます。ターンが API エラーで終了するチームメンバーは、リーダーに失敗したことを通知し、エラーテキストを含めます。
* **共有タスクリスト**：[Task ツールを持つエージェント](/docs/ja/tools-reference#task-tool-availability) はタスクステータスを表示でき、利用可能な作業を要求できます。
* **チームメンバーメッセージング**：その名前で特定のチームメンバーにメッセージを送信します。全員に到達するには、受信者ごとに 1 つのメッセージを送信してください。

リーダーは生成時に各チームメンバーに名前を割り当て、任意のチームメンバーはその名前で他のチームメンバーにメッセージを送信できます。後のプロンプトで参照できる予測可能な名前を取得するには、生成指示でリーダーに各チームメンバーを何と呼ぶかを指示してください。

<h3 id="token-usage">
  トークン使用量
</h3>

エージェントチームは単一セッションよりも大幅に多くのトークンを使用します。各チームメンバーは独自のコンテキストウィンドウを持ち、トークン使用量はアクティブなチームメンバーの数でスケールします。調査、レビュー、および新機能作業の場合、追加のトークンは通常価値があります。ルーチンタスクの場合、単一セッションがより費用効果的です。使用ガイダンスについては、[エージェントチームトークンコスト](/docs/ja/costs#agent-team-token-costs) を参照してください。

インプロセスチームメンバーのリクエストはメイン会話の [キャッシュ TTL バケット](/docs/ja/prompt-caching#which-ttl-each-request-gets) の外にあるため、Claude サブスクリプションを含め、デフォルトで 5 分間キャッシュが保持されます。1 時間保持するには、[`subagentPromptCacheTtl`](/docs/ja/settings-reference#subagentpromptcachettl) を `1h` に設定してください。API は 1 時間のキャッシュ書き込みをより高いレートで請求します。

<h2 id="use-case-examples">
  ユースケース例
</h2>

これらの例は、エージェントチームが並列探索が価値を追加するタスクをどのように処理するかを示しています。

<h3 id="run-a-parallel-code-review">
  並列コードレビューを実行する
</h3>

単一のレビュアーは一度に 1 つのタイプの問題に傾く傾向があります。レビュー基準を独立したドメインに分割することで、セキュリティ、パフォーマンス、およびテストカバレッジがすべて同時に徹底的に注意を受けます。プロンプトは各チームメンバーに異なるレンズを割り当てるため、重複しません。

```text wrap theme={null}
Spawn three teammates to review PR #142:
- One focused on security implications
- One checking performance impact
- One validating test coverage
Have them each review and report findings.
```

各レビュアーは同じ PR から動作しますが、異なるフィルターを適用します。リーダーは完了後、3 つすべてにわたって調査結果を統合します。

<h3 id="investigate-with-competing-hypotheses">
  競合する仮説で調査する
</h3>

根本原因が不明な場合、単一のエージェントは 1 つのもっともらしい説明を見つけて停止する傾向があります。プロンプトはチームメンバーを明示的に敵対的にすることでこれと戦います。各チームメンバーの仕事は、独自の理論を調査するだけでなく、他の理論に異議を唱えることです。

```text wrap theme={null}
Users report the app exits after one message instead of staying connected.
Spawn 5 agent teammates to investigate different hypotheses. Have them talk to
each other to try to disprove each other's theories, like a scientific
debate. Update the findings doc with whatever consensus emerges.
```

議論構造はここでの重要なメカニズムです。順序付き調査はアンカリングに悩まされます。1 つの理論が探索されると、その後の調査はそれに向かってバイアスされます。

複数の独立した調査官が互いに積極的に反証しようとしている場合、生き残る理論は実際の根本原因である可能性がはるかに高くなります。

<h2 id="best-practices">
  ベストプラクティス
</h2>

<h3 id="give-teammates-enough-context">
  チームメンバーに十分なコンテキストを提供する
</h3>

チームメンバーはプロジェクトコンテキスト（CLAUDE.md、MCP servers、および skills を含む）を自動的にロードしますが、リーダーの会話履歴は継承しません。詳細については、[コンテキストと通信](#context-and-communication)を参照してください。生成プロンプトにタスク固有の詳細を含めてください。

```text wrap theme={null}
Spawn a security reviewer teammate with the prompt: "Review the authentication module
at src/auth/ for security vulnerabilities. Focus on token handling, session
management, and input validation. The app uses JWT tokens stored in
httpOnly cookies. Report any issues with severity ratings."
```

<h3 id="choose-an-appropriate-team-size">
  適切なチームサイズを選択する
</h3>

チームメンバーの数に厳しい制限はありませんが、実際の制約が適用されます。

* **トークンコストは線形にスケール**：各チームメンバーは独自のコンテキストウィンドウを持ち、独立してトークンを消費します。詳細については、[エージェントチームトークンコスト](/docs/ja/costs#agent-team-token-costs)を参照してください。
* **調整オーバーヘッドが増加**：より多くのチームメンバーは、より多くの通信、タスク調整、および競合の可能性を意味します
* **収穫逓減**：ある時点を超えると、追加のチームメンバーは作業を比例的に高速化しません

ほとんどのワークフローでは、3～5 人のチームメンバーで開始してください。これは並列作業と管理可能な調整のバランスを取ります。15 個の独立したタスクがある場合、3 人のチームメンバーが良い出発点です。

チームメンバーが同時に動作することから本当に作業が利益を得る場合にのみスケールアップしてください。3 人の焦点を絞ったチームメンバーは、5 人の散らばったチームメンバーよりもしばしば優れています。

<h3 id="size-tasks-appropriately">
  タスクを適切にサイズ設定する
</h3>

* **小さすぎる**：調整オーバーヘッドが利益を超える
* **大きすぎる**：チームメンバーはチェックインなしで長時間動作し、無駄な努力のリスクが増加します
* **ちょうど良い**：関数、テストファイル、またはレビューなど、明確な成果物を生成する自己完結型ユニット

<Tip>
  リーダーは作業をタスクに分割し、チームメンバーに自動的に割り当てます。十分なタスクを作成していない場合は、作業をより小さな部分に分割するよう指示してください。チームメンバーあたり 5～6 個のタスクを持つことで、誰もが生産的に保たれ、誰かが立ち往生した場合、リーダーが作業を再割り当てできます。
</Tip>

<h3 id="wait-for-teammates-to-finish">
  チームメンバーが完了するまで待つ
</h3>

時々、リーダーはチームメンバーを待つ代わりに、タスク自体を実装し始めます。これに気付いた場合は、以下を実行してください。

```text wrap theme={null}
Wait for your teammates to complete their tasks before proceeding
```

<h3 id="start-with-research-and-review">
  調査とレビューから開始する
</h3>

エージェントチームが初めての場合は、明確な境界があり、コードを書く必要がないタスクから開始してください。PR をレビューする、ライブラリを調査する、またはバグを調査します。これらのタスクは、並列実装に伴う調整の課題なしに、並列探索の価値を示しています。

<h3 id="avoid-file-conflicts">
  ファイルの競合を回避する
</h3>

2 人のチームメンバーが同じファイルを編集すると、上書きが発生します。作業を分割して、各チームメンバーが異なるファイルセットを所有するようにしてください。

<h3 id="monitor-and-steer">
  監視と操舵
</h3>

チームメンバーの進捗をチェックし、機能していないアプローチをリダイレクトし、調査結果が入ってくるにつれて統合してください。チームを長時間無人で実行させると、無駄な努力のリスクが増加します。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

<h3 id="teammates-not-appearing">
  チームメンバーが表示されない
</h3>

Claude にチームメンバーを生成するよう指示した後、チームメンバーが表示されない場合は、以下を実行してください。

* In-process モードでは、チームメンバーはプロンプト入力の下のエージェントパネルに表示されます。上下矢印キーを使用して 1 つを選択し、Enter キーを押して表示してください。
* アイドル状態で消えたチームメンバー行は停止されていなく、非表示になっています。アイドル行はパネル全体がアイドル状態になってから 30 秒後に非表示になり、チームメンバーの次のターンで再表示されます。3 人以上のチームメンバーがアイドル状態の場合、余剰行は `N idle agents` という 1 つの行に折りたたまれ、Enter キーで展開されます。チームメンバーに名前でメッセージを送信して、非表示の行を戻してください。
* Claude に提供したタスクがチームを必要とするほど十分複雑であることを確認してください。Claude はタスクに基づいてチームメンバーを生成するかどうかを決定します。
* 明示的に分割ペインをリクエストした場合は、tmux がインストールされ、PATH で利用可能であることを確認してください。
  ```bash theme={null}
  which tmux
  ```
* iTerm2 の場合、`it2` CLI がインストールされ、Python API が iTerm2 の設定で有効になっていることを確認してください。

<h3 id="claude-spawns-teammates-instead-of-subagents">
  Claude がサブエージェントの代わりにチームメンバーを生成する
</h3>

エージェントチームが有効な場合、Claude がリーダーのセッションで名前を付けたサブエージェントはチームメンバーとして起動します。Claude は[独自にサブエージェントに名前を付けることができる](#how-claude-starts-agent-teams)ため、これはチームワークとしてフレーム化したことのない委任中に発生する可能性があります。

名前付きサブエージェントをサブエージェントとして再度起動するには、`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` を `0` に設定してエージェントチームをオフにしてください。

```json settings.json theme={null}
{
  "env": {
    "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "0"
  }
}
```

新しいセッションを開始する必要はありません。Claude Code は、保存時に実行中のセッションに設定ファイルの `env` 値を再適用し、Claude がサブエージェントを生成するたびに変数を再読み込みするため、次に Claude が名前を付けたサブエージェントはサブエージェントとして起動します。

ユーザー `settings.json` で変数を `0` に設定すると、シェルエクスポートがオーバーライドされます。他の設定ソースはエージェントチームを有効にできます。

* **優先度の高い設定ファイル**：プロジェクト設定、ローカル設定、および `--settings` ペイロードはユーザー設定の後に適用されるため、それらのいずれかで変数を `1` に設定する `env` エントリが優先されます。[設定の優先度](/docs/ja/settings#settings-precedence)を参照してください。
* **マネージド設定**：[マネージド設定](/docs/ja/server-managed-settings)は他のすべてのソースの後に適用されます。組織がそこでエージェントチームを有効にしている場合は、管理者にマネージド値の変更を依頼してください。

変更後、Claude はまだサブエージェントに名前を付ける可能性があり、その名前は [`SendMessage` アドレス](/docs/ja/sub-agents#resume-subagents)として機能し続けます。Claude は各サブエージェントが完了したときにその結果を受け取ります。

<h3 id="too-many-permission-prompts">
  権限プロンプトが多すぎる
</h3>

チームメンバーの権限リクエストはリーダーにバブルアップし、摩擦を生じさせる可能性があります。チームメンバーを生成する前に、[権限設定](/docs/ja/permissions)で一般的な操作を事前承認して、中断を減らしてください。

<h3 id="agents-stopping-early">
  エージェントが早期に停止する
</h3>

チームメンバーはエラーが発生した後、回復する代わりに停止する可能性があります。In-process モードでエージェントパネルでチームメンバーを選択して Enter キーを押すか、分割モードでペインをクリックして出力を確認し、以下のいずれかを実行してください。

* 直接追加の指示を与える
* 作業を続行するために置き換えチームメンバーを生成する

リーダーまたは別のチームメンバーからのメッセージは、失敗した API リクエストの再試行を待機している In-process チームメンバーをウェイクアップするため、完全な再試行遅延を待つ代わりに、すぐに再試行されます。

リーダーも早期に停止する可能性があり、すべてのタスクが実際に完了する前にチームが完了したと判断します。これが発生した場合は、続行するよう指示してください。

<h3 id="orphaned-tmux-sessions">
  孤立した tmux セッション
</h3>

チームが終了した後、tmux セッションが持続する場合、完全にクリーンアップされていない可能性があります。セッションをリストして、チームによって作成されたセッションを終了してください。

```bash theme={null}
tmux ls
tmux kill-session -t <session-name>
```

<h2 id="limitations">
  制限事項
</h2>

Agent teams は実験的な機能です。現在の制限事項は以下の通りです。

* **インプロセス teammates でのセッション再開非対応**: `/resume` と `/rewind` はインプロセス teammates を復元しません。セッションを再開した後、lead が存在しなくなった teammates にメッセージを送ろうとする可能性があります。この場合、lead に新しい teammates をスポーンするよう指示してください。
* **タスク状態が遅延することがある**: teammates は時々タスク完了のマークに失敗し、これが依存タスクをブロックします。タスクが止まっているように見える場合、実際に作業が完了しているかどうかを確認し、タスク状態を手動で更新するか、lead に teammate に促すよう指示してください。
* **シャットダウンが遅い可能性がある**: teammates は現在のリクエストまたはツール呼び出しを完了してからシャットダウンするため、時間がかかる可能性があります。
* **セッションごとに 1 つのチーム**: セッションは正確に 1 つのチームを持ち、そのセッションにスコープされています。追加の名前付きチームを作成したり、セッション間でチームを共有したりすることはできません。
* **ネストされたチーム非対応**: teammates は独自の teammates をスポーンできません。lead のみがチームを管理できます。
* **インプロセス teammates からのバックグラウンド subagents 非対応**: インプロセス teammate 自身の subagents はフォアグラウンドで実行されます。これは teammate のバックグラウンド作業が lead のプロセスより長く存続できないためです。Claude Code は、teammate が `background: true` を設定する定義を持つ subagent をスポーンする場合、エラーを返します。teammate の `run_in_background: true` リクエストも失敗し、[Claude Code がフォアグラウンドまたはバックグラウンドを選択する方法](/docs/ja/sub-agents#run-subagents-in-foreground-or-background)で説明されているようにエラーが発生するか、フォアグラウンドで静かに実行されます。メイン会話から起動された Subagents は[バックグラウンドデフォルト](/docs/ja/sub-agents#run-subagents-in-foreground-or-background)に従います。
* **Lead は固定**: メインセッションはその存続期間中、lead です。teammate を lead に昇格させたり、リーダーシップを譲渡したりすることはできません。
* **権限はスポーン時に設定**: teammates は[権限](#permissions)の下で説明されている権限モードで開始します。スポーン後に個別の teammate の権限モードを変更できますが、スポーン時に teammate ごとの権限モードを設定することはできません。
* **分割ペインは tmux または iTerm2 が必要**: デフォルトのインプロセスモードはすべてのターミナルで機能します。分割ペインモードは VS Code の統合ターミナル、Windows Terminal、または Ghostty ではサポートされていません。

<h2 id="next-steps">
  次のステップ
</h2>

並列作業と委任の関連アプローチを探索してください。

* **軽量委任**：[subagents](/docs/ja/sub-agents) はセッション内で調査または検証用のヘルパーエージェントを生成し、エージェント間調整が必要ないタスクに適しています
* **セッション間のメッセージング**：[クロスセッションメッセージング](/docs/ja/cross-session-messaging) により、Claude は自分で実行するセッション間で検出結果を渡すことができます
* **手動並列セッション**：[Git worktrees](/docs/ja/worktrees) を使用すると、自動チーム調整なしで複数の Claude Code セッションを自分で実行できます
