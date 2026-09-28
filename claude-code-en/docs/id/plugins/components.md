> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Tambahkan komponen ke plugin

> Tambahkan skills, hooks, server MCP, dan setiap jenis komponen lainnya ke plugin Claude Code, dengan contoh yang memvalidasi untuk masing-masing.

export const Piece = ({id, children}) => <div className="pe-piece" data-piece={id}>{children}</div>;

export const PluginExplorer = ({children}) => {
  const PIECES = [{
    id: 'manifest',
    name: 'Manifest',
    path: '.claude-plugin/plugin.json',
    required: "Required by Anthropic's directory",
    lines: [{
      depth: 0,
      kind: 'folder',
      text: '.claude-plugin/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'plugin.json'
    }],
    href: '/en/plugins/manifest-reference#manifest-file',
    linkText: 'Go to the manifest reference'
  }, {
    id: 'skills',
    name: 'Skills',
    path: 'skills/review/SKILL.md',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'skills/'
    }, {
      depth: 1,
      kind: 'folder',
      text: 'review/'
    }, {
      depth: 2,
      kind: 'file',
      text: 'SKILL.md'
    }],
    href: '/en/plugins/components#skills',
    linkText: 'Go to the Skills section'
  }, {
    id: 'commands',
    name: 'Commands',
    path: 'commands/about.md',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'commands/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'about.md'
    }],
    href: '/en/plugins/components#commands',
    linkText: 'Go to the Commands section'
  }, {
    id: 'agents',
    name: 'Agents',
    path: 'agents/security-reviewer.md',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'agents/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'security-reviewer.md'
    }],
    href: '/en/plugins/components#agents',
    linkText: 'Go to the Agents section'
  }, {
    id: 'hooks',
    name: 'Hooks',
    path: 'hooks/hooks.json',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'hooks/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'hooks.json'
    }],
    href: '/en/plugins/components#hooks',
    linkText: 'Go to the Hooks section'
  }, {
    id: 'monitors',
    name: 'Monitors',
    path: 'monitors/monitors.json',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'monitors/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'monitors.json'
    }],
    href: '/en/plugins/components#monitors',
    linkText: 'Go to the Monitors section'
  }, {
    id: 'output-styles',
    name: 'Output styles',
    path: 'output-styles/terse.md',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'output-styles/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'terse.md'
    }],
    href: '/en/plugins/components#themes-and-output-styles',
    linkText: 'Go to the Themes and output styles section'
  }, {
    id: 'themes',
    name: 'Themes',
    path: 'themes/dracula.json',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'themes/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'dracula.json'
    }],
    href: '/en/plugins/components#themes-and-output-styles',
    linkText: 'Go to the Themes and output styles section'
  }, {
    id: 'workflows',
    name: 'Workflows',
    path: 'workflows/audit-routes.js',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'workflows/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'audit-routes.js'
    }],
    href: '/en/workflows#distribute-a-workflow-in-a-plugin',
    linkText: 'Go to Distribute a workflow in a plugin'
  }, {
    id: 'bin',
    name: 'Executables',
    path: 'bin/hello-plugin',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'bin/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'hello-plugin'
    }],
    href: '/en/plugins/components#executables',
    linkText: 'Go to the Executables section'
  }, {
    id: 'scripts',
    name: 'Scripts',
    path: 'scripts/format.sh',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'scripts/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'format.sh'
    }],
    href: '/en/plugins/components#hooks',
    linkText: 'Go to the Hooks section'
  }, {
    id: 'settings',
    name: 'Default settings',
    path: 'settings.json',
    lines: [{
      depth: 0,
      kind: 'file',
      text: 'settings.json'
    }],
    href: '/en/plugins/components#default-settings',
    linkText: 'Go to the Default settings section'
  }, {
    id: 'mcp',
    name: 'MCP servers',
    path: '.mcp.json',
    lines: [{
      depth: 0,
      kind: 'file',
      text: '.mcp.json'
    }],
    href: '/en/plugins/components#mcp-servers',
    linkText: 'Go to the MCP servers section'
  }, {
    id: 'lsp',
    name: 'LSP servers',
    path: '.lsp.json',
    lines: [{
      depth: 0,
      kind: 'file',
      text: '.lsp.json'
    }],
    href: '/en/plugins/components#lsp-servers',
    linkText: 'Go to the LSP servers section'
  }];
  const [selectedId, setSelectedId] = useState('manifest');
  const [isFullscreen, setIsFullscreen] = useState(false);
  const rootRef = useRef(null);
  useEffect(() => {
    const onFsChange = () => setIsFullscreen(!!document.fullscreenElement);
    document.addEventListener('fullscreenchange', onFsChange);
    return () => document.removeEventListener('fullscreenchange', onFsChange);
  }, []);
  const toggleFullscreen = () => {
    if (!rootRef.current) return;
    if (document.fullscreenElement) document.exitFullscreen(); else rootRef.current.requestFullscreen().catch(() => {});
  };
  const selected = PIECES.find(p => p.id === selectedId) || PIECES[0];
  const onTreeKeyDown = e => {
    const keys = ['ArrowDown', 'ArrowUp', 'Home', 'End'];
    if (keys.indexOf(e.key) === -1) return;
    const i = PIECES.findIndex(p => p.id === selectedId);
    let next = i;
    if (e.key === 'ArrowDown') next = Math.min(PIECES.length - 1, i + 1);
    if (e.key === 'ArrowUp') next = Math.max(0, i - 1);
    if (e.key === 'Home') next = 0;
    if (e.key === 'End') next = PIECES.length - 1;
    e.preventDefault();
    if (next === i) return;
    const id = PIECES[next].id;
    setSelectedId(id);
    const el = document.getElementById('pe-node-' + id);
    if (el) el.focus();
  };
  const FolderIcon = () => <svg className="pe-icon" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <path d="M1.5 4.5a1 1 0 0 1 1-1h3.2l1.3 1.5h6a1 1 0 0 1 1 1V12a1 1 0 0 1-1 1h-10.5a1 1 0 0 1-1-1z" />
    </svg>;
  const FileIcon = () => <svg className="pe-icon" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <path d="M4 1.5h5.5L13 5v9.5H4z" />
      <path d="M9.5 1.5V5H13" />
    </svg>;
  return <div ref={rootRef} className={isFullscreen ? 'pe-root pe-fullscreen not-prose' : 'pe-root not-prose'} data-selected={selected.id}>
      <style>{`
        .pe-root {
          --pe-mono: var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace);
          --pe-accent: #D97757;
          --pe-accent-text: #A8502F;
          --pe-accent-bg: rgba(217,119,87,0.10);
          --pe-bg: #FFFFFF;
          --pe-surface: #FAFAF7;
          --pe-hover: #F0EEE6;
          --pe-border: #E8E6DC;
          --pe-text: #141413;
          --pe-text-2: #3D3D3A;
          --pe-text-3: #5E5D59;
          font-family: inherit;
          background: var(--pe-bg);
          color: var(--pe-text);
          border: 1px solid var(--pe-border);
          border-radius: 12px;
          margin: 1.5rem 0;
          overflow: hidden;
          box-sizing: border-box;
        }
        .dark .pe-root {
          --pe-accent-text: #EBA98F;
          --pe-accent-bg: rgba(217,119,87,0.18);
          --pe-bg: #1A1918;
          --pe-surface: #232221;
          --pe-hover: #2E2D2B;
          --pe-border: #3A3936;
          --pe-text: #F1EFE9;
          --pe-text-2: #D6D4CA;
          --pe-text-3: #B8B5AD;
        }
        .pe-root *, .pe-root *::before, .pe-root *::after { box-sizing: border-box; }
        .pe-head { display: flex; align-items: flex-start; gap: 12px; padding: 18px 24px 16px; border-bottom: 1px solid var(--pe-border); }
        .pe-head-text { flex: 1; min-width: 0; }
        .pe-fs-btn { flex-shrink: 0; width: 32px; height: 32px; display: inline-flex; align-items: center; justify-content: center; border: 1px solid var(--pe-border); border-radius: 6px; background: var(--pe-surface); color: var(--pe-text-2); font-size: 15px; line-height: 1; cursor: pointer; }
        .pe-fs-btn:hover { background: var(--pe-hover); }
        .pe-fs-btn:focus-visible { outline: 2px solid var(--pe-accent); outline-offset: 2px; }
        .pe-fullscreen { border-radius: 0; height: 100vh; display: flex; flex-direction: column; overflow: auto; }
        .pe-fullscreen .pe-body { flex: 1; }
        .pe-title { font-size: 19px; font-weight: 600; line-height: 1.3; color: var(--pe-text); margin: 0; }
        .pe-sub { font-size: 15px; line-height: 1.5; color: var(--pe-text-3); margin: 4px 0 0; }
        .pe-sub code { font-family: var(--pe-mono); font-size: 0.88em; padding: 1px 5px; border-radius: 4px; background: var(--pe-surface); border: 1px solid var(--pe-border); }
        .pe-body { display: flex; align-items: stretch; }
        .pe-tree-pane { width: 270px; flex-shrink: 0; background: var(--pe-surface); border-right: 1px solid var(--pe-border); padding: 16px 0 12px; }
        .pe-panel { flex: 1; min-width: 0; padding: 16px 24px 24px; }
        .pe-caption { font-size: 13px; font-weight: 600; color: var(--pe-text-3); margin: 0 0 10px; }
        .pe-tree-pane .pe-caption { padding: 0 16px; }
        .pe-rootline { display: flex; align-items: center; gap: 7px; padding: 3px 16px; font-family: var(--pe-mono); font-size: 13.5px; color: var(--pe-text-3); }
        .pe-node {
          display: block; width: 100%; margin: 0; padding: 3px 16px 3px 30px; text-align: left; cursor: pointer;
          background: transparent; color: var(--pe-text-2);
          border: none; border-left: 3px solid transparent;
          font-family: var(--pe-mono); font-size: 13.5px; line-height: 1.4;
        }
        .pe-node:hover { background: var(--pe-hover); }
        .pe-node:focus-visible { outline: 2px solid var(--pe-accent); outline-offset: -2px; }
        .pe-node[aria-pressed="true"] { background: var(--pe-accent-bg); border-left-color: var(--pe-accent); color: var(--pe-accent-text); font-weight: 600; }
        .pe-line { display: flex; align-items: center; gap: 7px; padding: 2px 0; }
        .pe-line-tree { flex-wrap: wrap; }
        .pe-line-tree .pe-req { flex-basis: 100%; margin: 2px 0 0 22px; white-space: normal; width: fit-content; max-width: calc(100% - 22px); }
        .pe-line span { overflow-wrap: anywhere; }
        .pe-piece { display: none; font-size: 16px; line-height: 1.6; color: var(--pe-text-2); }
        .pe-root[data-selected="manifest"] .pe-piece[data-piece="manifest"],
        .pe-root[data-selected="skills"] .pe-piece[data-piece="skills"],
        .pe-root[data-selected="commands"] .pe-piece[data-piece="commands"],
        .pe-root[data-selected="agents"] .pe-piece[data-piece="agents"],
        .pe-root[data-selected="hooks"] .pe-piece[data-piece="hooks"],
        .pe-root[data-selected="monitors"] .pe-piece[data-piece="monitors"],
        .pe-root[data-selected="output-styles"] .pe-piece[data-piece="output-styles"],
        .pe-root[data-selected="themes"] .pe-piece[data-piece="themes"],
        .pe-root[data-selected="workflows"] .pe-piece[data-piece="workflows"],
        .pe-root[data-selected="bin"] .pe-piece[data-piece="bin"],
        .pe-root[data-selected="scripts"] .pe-piece[data-piece="scripts"],
        .pe-root[data-selected="settings"] .pe-piece[data-piece="settings"],
        .pe-root[data-selected="mcp"] .pe-piece[data-piece="mcp"],
        .pe-root[data-selected="lsp"] .pe-piece[data-piece="lsp"] { display: block; }
        .pe-piece p { margin: 0 0 10px; }
        .pe-piece p:last-child { margin-bottom: 0; }
        .pe-piece code { font-family: var(--pe-mono); font-size: 0.88em; padding: 1px 5px; border-radius: 4px; background: var(--pe-surface); border: 1px solid var(--pe-border); }
        .pe-piece .code-block { margin: 12px 0 0; }
        .pe-piece pre code { padding: 0; border: none; background: none; }
        .pe-piece a { color: var(--pe-accent-text); }
        .pe-line-compact { display: none; }
        .pe-icon { flex-shrink: 0; }
        .pe-req { margin-left: 8px; padding: 0 6px; border-radius: 999px; font-size: 11px; line-height: 18px; letter-spacing: .02em; color: var(--pe-accent-text); border: 1px solid var(--pe-border); background: var(--pe-surface); white-space: nowrap; font-weight: 500; vertical-align: middle; }
        .pe-name { font-size: 22px; font-weight: 600; line-height: 1.25; letter-spacing: -0.2px; color: var(--pe-text); margin: 0; }
        .pe-path { font-family: var(--pe-mono); font-size: 13.5px; color: var(--pe-accent-text); margin: 4px 0 0; overflow-wrap: anywhere; }
        .pe-block { margin: 20px 0 0; }
        .pe-link {
          display: inline-block; margin: 24px 0 0; padding: 8px 14px; border-radius: 8px;
          font-size: 14.5px; font-weight: 600; text-decoration: none;
          color: var(--pe-accent-text); background: var(--pe-accent-bg); border: 1px solid var(--pe-accent);
        }
        .pe-link:hover { filter: brightness(0.97); }
        .pe-link:focus-visible { outline: 2px solid var(--pe-accent); outline-offset: 2px; }
        @media (max-width: 700px) {
          .pe-head { padding: 16px 16px 14px; }
          .pe-body { flex-direction: column; }
          .pe-tree-pane { width: 100%; border-right: none; border-bottom: 1px solid var(--pe-border); }
          .pe-line-tree { display: none; }
          .pe-line-compact { display: flex; }
          .pe-panel { padding: 16px 16px 20px; }
        }
      `}</style>

      <div className="pe-head">
        <div className="pe-head-text">
          <div className="pe-title">What goes in a plugin</div>
          <div className="pe-sub">This example plugin, <code>my-plugin</code>, has one of every kind of component, each in its default location. Select a file or folder to read what it’s for and see what goes in it.</div>
        </div>
        <button type="button" className="pe-fs-btn" onClick={toggleFullscreen} aria-label={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'} title={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'}>
          {isFullscreen ? '⤡' : '⛶'}
        </button>
      </div>

      <div className="pe-body">
        <div className="pe-tree-pane">
          <div className="pe-caption" id="pe-tree-caption">Plugin directory</div>
          <div role="group" aria-labelledby="pe-tree-caption" onKeyDown={onTreeKeyDown}>
            <div className="pe-rootline"><FolderIcon /><span>my-plugin/</span></div>
            {PIECES.map(p => <button key={p.id} id={'pe-node-' + p.id} type="button" className="pe-node" aria-pressed={p.id === selected.id} aria-label={p.name + ', ' + p.path} onClick={() => setSelectedId(p.id)}>
                {p.lines.map((line, i) => <span key={i} className="pe-line pe-line-tree" style={{
    paddingLeft: line.depth * 18 + 'px'
  }}>
                    {line.kind === 'folder' ? <FolderIcon /> : <FileIcon />}
                    <span>{line.text}</span>
                    {p.required && i === p.lines.length - 1 ? <span className="pe-req">{p.required}</span> : null}
                  </span>)}
                <span className="pe-line pe-line-compact">
                  <FileIcon />
                  <span>{p.path}</span>
                  {p.required ? <span className="pe-req">{p.required}</span> : null}
                </span>
              </button>)}
          </div>
        </div>

        <div className="pe-panel" role="region" aria-labelledby="pe-panel-caption" aria-live="polite" aria-atomic="true">
          <div className="pe-caption" id="pe-panel-caption">Selected piece</div>
          <div className="pe-name">{selected.name}{selected.required ? <span className="pe-req">{selected.required}</span> : null}</div>
          <div className="pe-path">{selected.path}</div>

          <div className="pe-block">{children}</div>

          <a className="pe-link" href={selected.href}>{selected.linkText}</a>
        </div>
      </div>
    </div>;
};

Plugin Claude Code dibangun dari komponen, seperti skills, agents, hooks, dan server MCP. Setiap komponen memiliki folder default di plugin, kunci manifest opsional di `.claude-plugin/plugin.json` yang menggantikan atau menambah folder tersebut, dan nama yang dilihat pengguna. Untuk tabel bidang lengkap setiap kunci, lihat [referensi manifest](/docs/id/plugins/manifest-reference#fields).

Gunakan halaman ini untuk menambahkan komponen ke plugin yang sudah dimuat.

Setelah Anda menambahkan komponen, jalankan `/reload-plugins` dalam sesi yang sedang berjalan atau mulai sesi baru sehingga Claude Code memuatnya. Untuk memeriksa file komponen sebelum memuatnya, jalankan [`claude plugin validate .`](/docs/id/plugins/cli-reference#plugin-validate) di shell Anda dari direktori plugin.

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Membangun plugin pertama Anda**: mulai dengan [Buat plugin](/docs/id/plugins/create)
  * **Memasang plugin orang lain**: lihat [Pasang plugins](/docs/id/plugins/install)
  * **Pengguna plugin Anda berada di claude.ai atau di Cowork**: serangkaian komponen yang berbeda dimuat di sana. Lihat [Plugins di claude.ai dan di Cowork](https://claude.com/docs/plugins/overview)
</Note>

<h2 id="explore-the-plugin-directory">
  Jelajahi direktori plugin
</h2>

Explorer menunjukkan plugin contoh, `my-plugin`, yang memiliki satu dari setiap jenis komponen di lokasi defaultnya:

* Skill review dan perintah `about`
* Subagent security-review
* Hook yang memformat file setelah Claude mengeditnya, dan folder `scripts/` yang dipanggilnya
* Monitor log
* Gaya output dan tema warna
* Workflow route-audit
* Executable `hello-plugin`
* Pengaturan default
* Server MCP lokal dan language server Go

Setiap file adalah contoh valid terkecil dari formatnya, ada untuk menunjukkan bentuknya daripada untuk berguna: skill atau agent nyata membawa instruksi lengkap dan sering kali file pendukung, dan hook atau monitor nyata melakukan pekerjaan nyata. Bagian setelah explorer menggunakan file yang sama sebagai contoh mereka dan menautkan ke yang lebih lengkap. Pilih file atau folder untuk membaca tujuannya, lihat apa yang ada di dalamnya, dan temukan bagian yang mencakupnya.

<PluginExplorer>
  <Piece id="manifest">
    [Manifest](/docs/id/plugins/manifest-reference) adalah file `plugin.json` di direktori `.claude-plugin/` plugin. Ini berisi metadata plugin dan nilai `userConfig` yang diminta Claude Code kepada pengguna. Hanya `name` yang diperlukan. Di sini, `description` adalah teks yang dilihat pengguna untuk plugin di `/plugin`, dan `version` membuat pengguna tetap pada versi itu sampai Anda mengubahnya:

    ```json theme={null}
    {
      "name": "my-plugin",
      "version": "1.0.0",
      "description": "Review, formatting, and database tools for this team"
    }
    ```
  </Piece>

  <Piece id="skills">
    [Skill](/docs/id/skills) adalah file `SKILL.md`. Simpan setiap skill di direktorinya sendiri di bawah `skills/`. Claude membaca `description` setiap skill, dan ketika apa yang diminta pengguna cocok dengannya, seperti meminta Claude untuk meninjau pull request di sini, Claude memuat instruksi skill dan mengikutinya. Pengguna juga dapat menjalankannya secara langsung sebagai `/my-plugin:review`:

    ```markdown theme={null}
    ---
    description: Reviews a pull request for style and test coverage. Use when asked to review code.
    ---

    Review the changed files. Report style problems first, then missing tests.
    ```
  </Piece>

  <Piece id="commands">
    Perintah adalah file Markdown tunggal yang dijalankan pengguna berdasarkan nama. Perintah adalah format yang lebih lama: skill berjalan berdasarkan nama dengan cara yang sama dan juga dapat membawa file pendukung di direktorinya sendiri, jadi tulis yang baru sebagai skills dan simpan `commands/` untuk file yang sudah Anda miliki. File ini menjadi `/my-plugin:about` dan mengambil frontmatter yang sama dengan skill:

    ```markdown theme={null}
    ---
    description: Summarize the repository
    ---

    Summarize what this repository does in three sentences.
    ```
  </Piece>

  <Piece id="agents">
    [Subagent](/docs/id/sub-agents) adalah asisten terpisah, dengan instruksinya sendiri dan jendela konteksnya sendiri, yang dapat didelegasikan Claude untuk menyelesaikan tugas dan mendapatkan hasil kembali. Setiap file Markdown di bawah `agents/` mendefinisikan satu: frontmatter menamainya dan mengatakan kapan menggunakannya, dan body adalah system prompt-nya. Yang ini dinamai `my-plugin:security-reviewer`, dan pengguna dapat memanggilnya dengan `@agent-my-plugin:security-reviewer`:

    ```markdown theme={null}
    ---
    name: security-reviewer
    description: Reviews code changes for security issues. Use after edits to authentication or input handling.
    model: sonnet
    ---

    You are a security reviewer. Read the changed files and report injection, authentication, and secrets-handling risks.
    ```
  </Piece>

  <Piece id="hooks">
    [Hook](/docs/id/hooks-guide) menjalankan sesuatu secara otomatis pada titik dalam siklus hidup Claude Code, seperti setelah setiap pengeditan file: perintah shell, permintaan HTTP, panggilan tool MCP, prompt ke model, atau subagent. Simpan hooks plugin di `hooks/hooks.json` di root plugin. Yang ini menjalankan `scripts/format.sh` plugin setelah Claude menulis atau mengedit file:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "\"${CLAUDE_PLUGIN_ROOT}/scripts/format.sh\""
              }
            ]
          }
        ]
      }
    }
    ```
  </Piece>

  <Piece id="monitors">
    Monitor adalah perintah shell yang dimulai Claude Code di latar belakang ketika sesi dimulai dan terus berjalan sampai berakhir, menggunakan [Monitor tool](/docs/id/tools-reference#monitor-tool). Apa yang dicetak mencapai Claude sebagai notifikasi. Field `when` dapat malah memulainya pertama kali skill bernama berjalan. Yang ini mengekor log kesalahan:

    ```json theme={null}
    [
      {
        "name": "error-log",
        "command": "tail -F ./logs/error.log",
        "description": "Application error log"
      }
    ]
    ```
  </Piece>

  <Piece id="output-styles">
    Plugin dapat menyertakan [output styles](/docs/id/output-styles), yang mengubah cara Claude memformat dan merumuskan balasannya. Simpan setiap output style sebagai `output-styles/<name>.md`. Yang ini muncul di `/output-style` sebagai `my-plugin:terse`:

    ```markdown theme={null}
    ---
    name: terse
    description: Answer in as few words as possible
    keep-coding-instructions: true
    ---

    Keep every reply short. Skip preambles and summaries.
    ```
  </Piece>

  <Piece id="themes">
    Plugin dapat menyertakan [color themes](/docs/id/terminal-config#create-a-custom-theme) untuk antarmuka Claude Code. Simpan setiap tema sebagai `themes/<slug>.json`. Yang ini muncul di `/theme` sebagai `Dracula`, ditandai sebagai dari `my-plugin`:

    ```json theme={null}
    {
      "name": "Dracula",
      "base": "dark",
      "overrides": {
        "claude": "#bd93f9",
        "error": "#ff5555"
      }
    }
    ```
  </Piece>

  <Piece id="workflows">
    Folder `workflows/` menyimpan file `.js` [workflow](/docs/id/workflows): blok `meta`, kemudian body script yang mengorkestra beberapa subagent. Yang ini berjalan sebagai `/my-plugin:audit-routes`:

    ```javascript theme={null}
    export const meta = {
      name: 'audit-routes',
      description: 'Audit every route handler for missing auth checks',
    }

    const found = await agent('List every .ts file under src/routes/.', {
      schema: { type: 'object', required: ['files'], properties: { files: { type: 'array', items: { type: 'string' } } } },
    })

    const audits = await pipeline(found.files, file =>
      agent(`Audit ${file} for missing authentication checks.`, { label: file }),
    )

    return audits.filter(Boolean)
    ```
  </Piece>

  <Piece id="bin">
    `bin/` adalah cara plugin mengirimkan alat command-line. Sementara plugin diaktifkan, Claude Code menempatkan folder ini di `PATH` shell tempat ia menjalankan perintah, sehingga Claude, atau instruksi skill, dapat menjalankan alat berdasarkan nama tanpa pengguna memasangnya. Dengan [executable](#executables) ini di tempat, `hello-plugin` adalah perintah yang dapat dijalankan Claude:

    ```bash theme={null}
    #!/bin/bash
    echo "hello from my-plugin"
    ```
  </Piece>

  <Piece id="scripts">
    Hook di `hooks/hooks.json` menjalankan script, dan folder ini adalah tempat contoh menyimpannya. Nama `scripts/` adalah konvensi, bukan sesuatu yang dicari Claude Code: hook menunjuk ke file berdasarkan jalurnya, `${CLAUDE_PLUGIN_ROOT}/scripts/format.sh`. Script formatter mungkin terlihat seperti ini:

    ```bash theme={null}
    #!/bin/bash
    npx prettier --write .
    ```
  </Piece>

  <Piece id="settings">
    `settings.json` di root plugin menyimpan [settings](/docs/id/settings-reference) yang berlaku sementara plugin diaktifkan, sehingga plugin dapat mengubah cara sesi berperilaku dan tidak hanya menambahkan komponen. Hanya dua kunci yang berlaku dari plugin, [`agent`](/docs/id/settings-reference#agent) dan [`subagentStatusLine`](/docs/id/settings-reference#subagentstatusline); setiap kunci lain dijatuhkan. Lihat [Default settings](#default-settings).

    Yang ini menetapkan `agent`, yang menjalankan thread utama sesi sebagai agent `security-reviewer` plugin sendiri, sehingga system prompt, pembatasan tool, dan model agent itu berlaku untuk seluruh sesi:

    ```json theme={null}
    {
      "agent": "security-reviewer"
    }
    ```
  </Piece>

  <Piece id="mcp">
    [Server MCP](/docs/id/mcp) memberikan Claude tools dari sistem eksternal. Deklarasikan di `.mcp.json` di root plugin. Yang ini memulai server lokal dari script di dalam plugin, dan muncul di `/mcp` sebagai `plugin:my-plugin:db`:

    ```json theme={null}
    {
      "mcpServers": {
        "db": {
          "command": "node",
          "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"]
        }
      }
    }
    ```
  </Piece>

  <Piece id="lsp">
    Server LSP memberikan Claude [diagnostics dan code navigation](/docs/id/plugins/code-intelligence) untuk bahasa. Deklarasikan server di `.lsp.json` di root plugin. Yang ini menghubungkan language server Go untuk file `.go`:

    ```json theme={null}
    {
      "gopls": {
        "command": "gopls",
        "args": ["serve"],
        "extensionToLanguage": {
          ".go": "go"
        }
      }
    }
    ```
  </Piece>
</PluginExplorer>

<h2 id="add-each-kind-of-component">
  Tambahkan setiap jenis komponen
</h2>

Setiap bagian di bawah mencakup satu jenis komponen: di mana file-filenya berada di plugin, contoh yang memvalidasi, apa yang dilihat pengguna setelah plugin dimuat, dan kunci manifest yang mengubah lokasi default. Tambahkan yang dibutuhkan plugin Anda; tidak ada yang diperlukan.

<h3 id="skills">
  Skills
</h3>

[Skill](/docs/id/skills) adalah file `SKILL.md` yang dapat dimuat Claude ketika deskripsinya cocok dengan tugas. Pengguna juga dapat menjalankannya sebagai perintah. Simpan setiap skill di direktorinya sendiri di bawah `skills/`:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── review/
        └── SKILL.md
```

Berikan `SKILL.md` `description` sehingga Claude tahu kapan menggunakannya:

```markdown skills/review/SKILL.md theme={null}
---
description: Reviews a pull request for style and test coverage. Use when asked to review code.
---

Review the changed files. Report style problems first, then missing tests.
```

Setelah Anda memuat plugin, `/my-plugin:review` menjalankan skill. Nama perintah dan siapa yang dapat memanggilnya mengikuti aturan ini:

* **Nama perintah**: `/<plugin>:<directory>`, jadi `skills/review/SKILL.md` di `my-plugin` adalah `/my-plugin:review`. Jika Anda menetapkan `name` di frontmatter, itu menggantikan segmen terakhir dan awalan plugin tetap. Lihat [bagaimana skill mendapatkan nama perintahnya](/docs/id/skills#how-a-skill-gets-its-command-name)
* **Siapa yang memanggilnya**: Claude, pengguna, atau keduanya, dikendalikan oleh frontmatter. Lihat [Kontrol siapa yang memanggilnya skill](/docs/id/skills#control-who-invokes-a-skill)

Anda juga dapat menempatkan skills di luar direktori default `skills/`:

* **Direktori tambahan**: daftarkan di kunci manifest `skills`. Mereka menambah pemindaian default `skills/` daripada menggantinya, tidak seperti `commands` dan `agents`
* **Skill tunggal di root plugin**: tanpa direktori `skills/` dan tanpa kunci manifest `skills`, `SKILL.md` di root plugin dimuat sebagai satu skill. Tetapkan `name` di frontmatter-nya, karena jika tidak, instalasi marketplace menamakan skill setelah [cache directory](/docs/id/plugins/loading#find-plugins-on-disk)-nya daripada plugin Anda

Untuk menyertakan instruksi dalam plugin, tulislah sebagai skill. Claude Code tidak memuat `CLAUDE.md` di root plugin, dan `claude plugin validate` memperingatkan `CLAUDE.md at the plugin root is not loaded as project context`.

Untuk field frontmatter dan file pendukung, lihat [Skills](/docs/id/skills).

<h3 id="commands">
  Commands
</h3>

Perintah adalah file Markdown tunggal yang dijalankan pengguna berdasarkan nama, seperti `/my-plugin:about`.

<Note>
  Perintah adalah format yang lebih lama, dan [skills](#skills) menggantikannya untuk pekerjaan baru. Skill berjalan berdasarkan nama dengan cara yang sama, dan itu juga dapat membawa file pendukung di direktorinya. Simpan `commands/` untuk file yang Anda pindahkan dari `.claude/commands/`.
</Note>

Simpan perintah di `commands/<file>.md` dan itu menjadi `/<plugin>:<file>`. Subdirektori menambah segmen, jadi `commands/db/migrate.md` adalah `/my-plugin:db:migrate`.

File perintah mengambil frontmatter yang sama dengan skills.

<h4 id="define-commands-in-the-manifest">
  Tentukan perintah di manifest
</h4>

Anda hanya membutuhkan ini jika Anda ingin menyimpan file perintah di tempat lain selain `commands/`, atau untuk mendefinisikan perintah pendek di dalam `plugin.json` tanpa file Markdown terpisah. Tetapkan kunci manifest `commands`, dan Claude Code membacanya daripada memindai `commands/`. Kunci mengambil jalur, array jalur, atau objek yang memetakan setiap nama perintah ke file `source` atau `content` inline.

Manifest ini mendefinisikan `/my-plugin:about` inline, tanpa file Markdown:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "commands": {
    "about": {
      "content": "Summarize what this repository does in three sentences.",
      "description": "Summarize the repository"
    }
  }
}
```

Muat plugin dan jalankan `/my-plugin:about` dalam sesi untuk mengonfirmasi itu dimuat.

Untuk sintaks kunci lengkap, lihat [`commands`](/docs/id/plugins/manifest-reference#commands).

<h3 id="agents">
  Agents
</h3>

[Subagent](/docs/id/sub-agents) adalah asisten terpisah, dengan instruksinya sendiri dan jendela konteks, yang dapat didelegasikan Claude untuk menyelesaikan tugas. Setiap file Markdown di bawah `agents/` mendefinisikan satu:

```markdown agents/security-reviewer.md theme={null}
---
name: security-reviewer
description: Reviews code changes for security issues. Use after edits to authentication or input handling.
model: sonnet
---

You are a security reviewer. Read the changed files and report injection, authentication, and secrets-handling risks.
```

Agent ini dinamai `my-plugin:security-reviewer`, dan pengguna dapat [memanggilnya secara eksplisit](/docs/id/sub-agents#invoke-subagents-explicitly) dengan `@agent-my-plugin:security-reviewer`. Bentuk nama adalah `<plugin>:<name>`, di mana `<name>` berasal dari frontmatter, atau dari nama file ketika tidak ada.

Kunci manifest `agents` menggantikan pemindaian `agents/`.

<h4 id="organize-agents-in-subfolders">
  Atur agents dalam subfolder
</h4>

Anda dapat menempatkan file agent plugin dalam subfolder `agents/`. Claude Code [memuatnya secara rekursif](/docs/id/sub-agents#choose-the-subagent-scope) dan menggabungkan nama plugin, setiap nama subfolder, dan nama file dengan titik dua untuk membentuk nama scoped agent. Misalnya, `agents/review/security.md` dalam plugin bernama `my-plugin` dimuat sebagai `my-plugin:review:security`. Dua pengaturan mengubah nama itu:

* Frontmatter `name`: itu menggantikan hanya nama file, jadi `name: audit` di `agents/review/security.md` dimuat sebagai `my-plugin:review:audit`
* Field manifest [`agents`](/docs/id/plugins/manifest-reference#fields): file yang Anda daftarkan di sana dimuat tanpa nama subfolder, jadi `"agents": "./custom/review/security.md"` dimuat sebagai `my-plugin:security`

<h4 id="frontmatter-fields-in-plugin-agents">
  Field frontmatter dalam agent plugin
</h4>

Frontmatter agent plugin mengikuti aturan ini:

* **Field yang didukung**: `name`, `description`, `model`, `effort`, `maxTurns`, `tools`, `disallowedTools`, `skills`, `memory`, `background`, `omitClaudeMd`, `isolation`, `color`, dan kunci `cacheTtl` dari `experimental`. Satu-satunya nilai `isolation` yang valid adalah `"worktree"`. Lihat [field frontmatter yang didukung](/docs/id/sub-agents#supported-frontmatter-fields) untuk apa yang dilakukan masing-masing
* **Field yang diabaikan**: `permissionMode`, `hooks`, `mcpServers`, dan `initialPrompt`. File agent tidak dapat menambahkan hooks atau server MCP sendiri, jadi tambahkan itu sebagai plugin [hooks](#hooks) dan [server MCP](#mcp-servers) sebagai gantinya
* **Frontmatter yang tidak diuraikan**: agent masih dimuat dengan setiap field diabaikan. Itu dinamai setelah file, dan deskripsinya berbunyi `Agent from my-plugin plugin`. Jalankan [`claude plugin validate`](/docs/id/plugins/cli-reference#plugin-validate) di shell Anda untuk menemukan file-file ini

Untuk apa yang dilakukan setiap field dan aturan prioritas, lihat [Subagents](/docs/id/sub-agents#supported-frontmatter-fields).

<h3 id="hooks">
  Hooks
</h3>

[Hook](/docs/id/hooks-guide) menjalankan sesuatu secara otomatis pada titik dalam siklus hidup Claude Code, seperti setelah setiap pengeditan file: perintah shell, permintaan HTTP, panggilan tool MCP, prompt ke model, atau subagent. Simpan hooks plugin di `hooks/hooks.json` di root plugin, di bawah kunci top-level `"hooks"`, dalam bentuk yang sama dengan objek `hooks` di `settings.json`. Itu memungkinkan Anda menyalin hook pengaturan yang ada tanpa perubahan.

Hook ini menjalankan script bundled setelah setiap `Write` atau `Edit`:

```json hooks/hooks.json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}/scripts/format.sh\""
          }
        ]
      }
    ]
  }
}
```

Simpan script di `scripts/format.sh` dan buat dapat dieksekusi.

Muat plugin dan minta Claude untuk mengedit file. Hook `PostToolUse` yang keluar 0 tidak menunjukkan apa pun dalam transkrip, jadi konfirmasi itu berjalan dengan [debug logging](/docs/id/hooks#debug-hooks) atau dengan apa yang diubah script itu sendiri.

Hooks di `hooks/hooks.json` dan di kunci manifest `hooks` keduanya dimuat. Untuk setiap event dan payload-nya, lihat [Hook events](/docs/id/hooks#hook-events).

<h4 id="when-plugin-hooks-fire">
  Kapan hook plugin dipecat
</h4>

Hook plugin tidak menunggu salah satu skill atau perintah plugin digunakan. Claude Code mendaftarkannya ketika sesi memuat plugin, dan mereka dipecat pada event mereka sejak saat itu. Untuk membatasi kapan hook berjalan, persempit `matcher`-nya.

Jika hook tidak pernah dipecat, lihat [hooks yang tidak dipecat](/docs/id/plugins/troubleshooting#failed-to-load-hooks-from-and-hooks-that-dont-fire).

<h4 id="environment-quoting-and-matching-mcp-tools">
  Lingkungan, quoting, dan pencocokan tool MCP
</h4>

Lingkungan hook, quoting `${CLAUDE_PLUGIN_ROOT}`, dan matcher untuk tool MCP plugin sendiri bekerja sebagai berikut:

* **Lingkungan**: setiap proses hook menerima `CLAUDE_PLUGIN_ROOT` dan `CLAUDE_PLUGIN_DATA` di lingkungannya, ditambah `CLAUDE_PLUGIN_OPTION_<KEY>` untuk setiap nilai [konfigurasi pengguna](#user-configuration), sehingga script Anda dapat membacanya dari sana
* **Quoting**: ketika `command` tidak memiliki `args`, itu berjalan melalui shell, jadi bungkus jalur `${CLAUDE_PLUGIN_ROOT}` dalam tanda kutip ganda, seperti contoh `hooks/hooks.json` di bawah [Hooks](#hooks), untuk menjaga jalur yang diperluas sebagai satu kata shell. Ketika Anda melewatkan `args` sebagai gantinya, setiap elemen dilewatkan sebagai satu argumen tanpa shell dan tidak memerlukan quoting. Lihat [exec form dan shell form](/docs/id/hooks#exec-form-and-shell-form)
* **Pencocokan tool MCP plugin sendiri**: tool dari [server MCP yang dideklarasikan plugin ini](#mcp-servers) dinamai `mcp__plugin_<plugin>_<server>__<tool>`, jadi tulis nama lengkap itu di matcher. Matcher pada nama server saja tidak pernah dipecat. Lihat [Match MCP tools](/docs/id/hooks#match-mcp-tools)

<h3 id="mcp-servers">
  Server MCP
</h3>

Server MCP memberikan Claude tools dari sistem eksternal. Deklarasikan di `.mcp.json` di root plugin, dalam bentuk yang sama dengan [project `.mcp.json`](/docs/id/mcp#project-scope). `.mcp.json` ini mendeklarasikan satu server bernama `db`:

```json .mcp.json theme={null}
{
  "mcpServers": {
    "db": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"]
    }
  }
}
```

Anda juga dapat menghilangkan wrapper `mcpServers` dan menempatkan `db` di level atas file.

Muat plugin dan jalankan `/mcp` untuk mengonfirmasi server muncul sebagai `plugin:my-plugin:db`.

`claude plugin validate` memeriksa `.mcp.json` dan melaporkan entri server yang akan dijatuhkan Claude Code pada waktu muat sebagai kesalahan. Memerlukan Claude Code v2.1.281 atau lebih baru.

Untuk di mana entri buruk muncul pada waktu muat, lihat [Server MCP yang tidak dimulai](/docs/id/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start).

Kunci manifest `mcpServers` mengambil peta server inline, jalur ke file JSON, atau array dari itu. Ketika server manifest memiliki nama yang sama dengan yang di `.mcp.json`, server manifest menggantinya.

<h4 id="reach-users-on-claude-ai-and-cowork">
  Jangkau pengguna di claude.ai dan Cowork
</h4>

Server stdio lokal, seperti server `db` di bawah [Server MCP](#mcp-servers), berjalan di Claude Code dan dalam sesi Cowork yang berjalan di mesin Anda di aplikasi Claude Desktop, tetapi bukan di claude.ai. Untuk menjangkau pengguna di sana juga, referensikan server jarak jauh dengan URL `https://`-nya, yang claude.ai dan Cowork tawarkan kepada pengguna sebagai konektor.

<h4 id="server-names-tool-names-and-reloads">
  Nama server, nama tool, dan reload
</h4>

Nama server, substitusi variabel, dan perilaku reload mengikuti aturan ini:

* **Nama server**: `plugin:<plugin>:<server>`, jadi server `db` di `my-plugin` adalah `plugin:my-plugin:db` di `/mcp`. Gunakan bentuk yang sama untuk menamakan server dalam hook [`mcp_tool`](/docs/id/hooks#mcp-tool-hook-fields)
* **Nama tool**: `mcp__plugin_<plugin>_<server>__<tool>`, jadi tool `query` pada server `db` itu adalah `mcp__plugin_my-plugin_db__query`. Itu adalah nama yang digunakan dalam [aturan izin](/docs/id/permissions) dan [matcher hook](#hooks)
* **Substitusi**: `${CLAUDE_PLUGIN_ROOT}` dan [variabel jalur](#path-variables-and-persistent-data) lainnya disubstitusikan dalam `command`, `args`, dan `env`. Tidak ada quoting yang diperlukan dalam `args`, karena setiap elemen dilewatkan sebagai satu argumen
* **Reload**: ketika pengguna menjalankan `/reload-plugins` dan [reload berlaku](/docs/id/plugins/cli-reference#reloads-that-change-mcp-tools), server yang konfigurasinya tidak berubah menjaga koneksinya. Server yang konfigurasinya berubah terhubung kembali, dan yang Anda hapus terputus

<h4 id="include-a-packaged-mcpb-server">
  Sertakan server MCPB yang dikemas
</h4>

Kunci `mcpServers` juga menerima server yang dikemas sebagai file [MCPB](https://github.com/modelcontextprotocol/mcpb), yang ekstensinya adalah `.mcpb` atau `.dxt` yang lebih lama. Arahkan kunci ke file, sebagai jalur di dalam plugin atau URL `https://`:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "mcpServers": "./servers/db.mcpb"
}
```

Server mengambil namanya dari `name` dalam manifest bundle.

Untuk transport dan autentikasi, lihat [MCP](/docs/id/mcp#plugin-provided-mcp-servers).

<h3 id="lsp-servers">
  Server LSP
</h3>

Server LSP memberikan Claude diagnostics dan code navigation untuk bahasa. Jika [plugin code intelligence resmi](/docs/id/plugins/code-intelligence) sudah mencakup bahasa Anda, pasang itu daripada menulis satu. Jika tidak, deklarasikan server di `.lsp.json` di root plugin:

```json .lsp.json theme={null}
{
  "gopls": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

File memetakan setiap nama server langsung ke konfigurasinya, tanpa objek wrapper di sekitar peta. `command` adalah nama binary, dengan argumennya di `args`. `extensionToLanguage` memerlukan setidaknya satu ekstensi, masing-masing dimulai dengan `.`.

`claude plugin validate` tidak membaca file ini. Ketika entri apa pun tidak valid, seluruh file dilewati pada waktu muat dan `Invalid LSP server config for ".lsp.json"` muncul di tab **Errors** `/plugin`.

Plugin Anda mengonfigurasi koneksi tetapi tidak memasang binary server, dan setiap ekstensi file mendapat satu server:

* **Binary yang hilang**: Claude Code memulai `command` berdasarkan nama dari `PATH` pengguna. Ketika binary tidak ada, server gagal dimulai dan `claude --debug` mencatat `LSP server <name> failed to start`
* **Konflik ekstensi**: ketika dua server yang diaktifkan mengklaim ekstensi yang sama, yang pertama terdaftar menangani file-file itu dan yang lain tidak digunakan untuk mereka, apakah server berasal dari satu plugin atau dua. Tab **Errors** `/plugin` menunjukkan peringatan `LSP server "<name>" is not used for <ext> files`

Kunci manifest `lspServers` mengambil peta yang sama inline, jalur ke file JSON, atau array dari itu, dan server-nya menambah yang di `.lsp.json`. Ketika server manifest memiliki nama yang sama dengan yang di `.lsp.json`, server manifest menggantinya.

Untuk `transport`, timeout, restart, dan field lainnya, lihat [`lspServers`](/docs/id/plugins/manifest-reference#lspservers).

Kirim output log ke stderr, bukan stdout. Claude Code membaca stdout server sebagai pesan protokol saja, dan menerima header pesan hingga 64 KiB dan body pesan hingga 32 MiB.

Claude Code memutuskan server yang melebihi batas apa pun atau menulis output non-protokol ke stdout, dan menghitung putus sebagai crash untuk `restartOnCrash` dan `maxRestarts`. Ketika Anda menjalankan dengan `--debug`, Claude Code menulis kesalahan yang menamai penyebabnya ke log debug.

<h3 id="executables">
  Executables
</h3>

File di `bin/` di root plugin berada di `PATH` shell tool Bash sementara plugin diaktifkan, sehingga Claude dapat menjalankannya sebagai perintah bare. Tambahkan script yang dapat dieksekusi:

```bash bin/hello-plugin theme={null}
#!/bin/bash
echo "hello from my-plugin"
```

Buat dapat dieksekusi dengan `chmod +x bin/hello-plugin` dan muat plugin. Ketika Anda meminta Claude untuk menjalankan `hello-plugin`, hasil tool Bash menunjukkan output script.

Direktori `bin/` plugin datang setelah entri `PATH` pengguna sendiri, jadi plugin tidak dapat menaungi `git`, `ls`, atau perintah sistem lainnya.

claude.ai dan Cowork tidak memasang plugin yang memiliki direktori `bin/` level atas, termasuk yang Anda [distribusikan melalui pengaturan organisasi claude.ai](/docs/id/plugins/host-marketplace#distribute-through-organization-settings).

<h3 id="default-settings">
  Pengaturan default
</h3>

Untuk menetapkan default yang berlaku sementara plugin diaktifkan, tambahkan `settings.json` di root plugin, atau letakkan objek yang sama inline di kunci manifest `settings`. Dua kunci berlaku, `agent` dan `subagentStatusLine`, dan setiap kunci lain dijatuhkan.

Tetapkan `agent` untuk menjalankan salah satu agent plugin sendiri sebagai thread utama:

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

Muat plugin dan mulai sesi. Claude kemudian menjawab dalam percakapan utama dengan system prompt dan model agent `security-reviewer`.

Untuk semua yang dikontrol kunci, lihat pengaturan [`agent`](/docs/id/settings-reference#agent).

Ketika kunci yang sama ditetapkan di lebih dari satu tempat, aturan ini memutuskan nilai mana yang berlaku:

* **File atas manifest**: ketika keduanya ada dan `settings.json` menetapkan setidaknya satu kunci yang didukung, `settings.json` berlaku dan `settings` manifest diabaikan
* **Pengaturan pengguna atas default plugin**: di seluruh sumber pengaturan, default plugin adalah layer terendah, jadi `agent` pengguna sendiri di `~/.claude/settings.json` menggantikan milik Anda
* **Dua plugin menetapkan kunci yang sama**: nilai dari plugin yang dimuat terakhir berlaku, dan `claude --debug` mencatat `overrides setting`

Untuk bentuk `subagentStatusLine`, lihat [subagent status lines](/docs/id/statusline#subagent-status-lines).

<h3 id="themes-and-output-styles">
  Tema dan output styles
</h3>

Plugin dapat menyertakan color themes dan output styles. Keduanya muncul di picker yang sama dengan pengguna sendiri. Untuk salah satu, menetapkan kunci manifest menggantikan pemindaian folder.

| Komponen     | Simpan sebagai            | Format                                                                                                                    | Muncul di                                  | Kunci manifest        |
| :----------- | :------------------------ | :------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------- | :-------------------- |
| Tema         | `themes/<slug>.json`      | Format [file tema kustom](/docs/id/terminal-config#create-a-custom-theme) yang ditulis pengguna di `~/.claude/themes/`         | `/theme`, di bawah `name` file             | `experimental.themes` |
| Output style | `output-styles/<name>.md` | Format [output style kustom](/docs/id/output-styles#create-a-custom-output-style), dengan frontmatter `name` dan `description` | `/output-style`, sebagai `<plugin>:<name>` | `outputStyles`        |

Tema plugin adalah read-only, jadi ketika pengguna mengedit satu di `/theme`, edit disimpan sebagai salinan di direktori tema mereka sendiri.

Tema ini mengubah warna prompt accent dan error text pada preset dark:

```json themes/dracula.json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555"
  }
}
```

<h3 id="channels">
  Channels
</h3>

[Channel](/docs/id/channels) memungkinkan sistem luar seperti aplikasi chat mengirim pesan ke sesi. Dalam plugin, channel adalah salah satu server MCP ditambah entri `channels` yang mengikat ke itu dan dapat meminta konfigurasinya sendiri. Manifest ini mengikat channel ke server `telegram` dan meminta token bot:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "mcpServers": {
    "telegram": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": { "BOT_TOKEN": "${user_config.bot_token}" }
    }
  },
  "channels": [
    {
      "server": "telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Telegram bot token",
          "sensitive": true
        }
      }
    }
  ]
}
```

`server` harus cocok dengan kunci di `mcpServers`. Per-channel `userConfig` mengambil bentuk yang sama dengan kunci [`userConfig` level atas](#user-configuration).

Untuk apa yang harus diimplementasikan server dan bagaimana pengguna mengaktifkan plugin channel, lihat [Package as a plugin](/docs/id/channels-reference#package-as-a-plugin) dalam referensi channels. Untuk tabel field, lihat [`channels`](/docs/id/plugins/manifest-reference#channels).

<h3 id="monitors">
  Monitors
</h3>

Monitor adalah perintah shell yang berjalan di latar belakang untuk seluruh sesi. Apa yang dicetak mencapai Claude sebagai notifikasi, sehingga Claude dapat bereaksi terhadap log atau perubahan status tanpa diminta untuk menontonnya. Simpan entri di `monitors/monitors.json`:

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

Perintah berjalan dalam shell, di direktori kerja tempat sesi dimulai.

Perintah monitor dibatasi di mana itu dimulai dan apa yang dapat direferensikan:

* **Sesi interaktif saja**: monitor plugin dimulai dalam sesi interaktif dan tidak pernah dalam mode non-interaktif dengan flag `-p`. Mereka juga dimulai hanya di mana [Monitor tool](/docs/id/tools-reference#monitor-tool) tersedia
* **Tidak ada konfigurasi pengguna**: `command` mendapat [variabel jalur](#path-variables-and-persistent-data) dan `${ENV_VAR}` dari lingkungan, tetapi tidak pernah `${user_config.*}`. Monitor yang mereferensikan satu tidak dimulai, dan proses monitor tidak menerima `CLAUDE_PLUGIN_OPTION_<KEY>` juga
* **Menonaktifkan mid-session**: jika Anda menonaktifkan plugin mid-session, Claude Code tidak menghentikan monitor yang sudah berjalan. Mereka berhenti ketika sesi berakhir

Kunci manifest `experimental.monitors` mengambil array yang sama inline atau jalur ke file JSON, dan dibaca daripada `monitors/monitors.json`.

Untuk trigger `when` dan field lainnya, lihat [`monitors`](/docs/id/plugins/manifest-reference#monitors).

<h2 id="user-configuration">
  Minta pengguna untuk nilai konfigurasi
</h2>

Deklarasikan nilai yang dibutuhkan plugin Anda dari pengguna di kunci manifest `userConfig`, sehingga pengguna tidak mengedit `settings.json` sendiri. Setiap opsi muncul dalam dialog dengan `title`-nya sebagai label dan `description`-nya di bawahnya.

Tetapkan `"sensitive": true` untuk token atau password. Dialog kemudian menutupi input, dan nilai disimpan dalam penyimpanan aman daripada `settings.json`.

Manifest ini meminta endpoint dan token:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "userConfig": {
    "api_url": {
      "type": "string",
      "title": "API URL",
      "description": "Base URL of your team's API"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "Token for your team's API",
      "sensitive": true
    }
  }
}
```

<h3 id="when-the-configuration-dialog-appears">
  Kapan dialog konfigurasi muncul
</h3>

Dialog muncul hanya dalam antarmuka `/plugin` interaktif. Itu terbuka untuk opsi apa pun yang belum ditetapkan ketika pengguna melakukan salah satu dari berikut:

* Memasang plugin di `/plugin`
* Menjalankan `/plugin install <plugin>@<marketplace>` di dalam sesi
* Mengaktifkan plugin dari tab **Installed** di `/plugin`

Untuk membuka dialog yang sama kapan saja, pengguna menjalankan `/plugin configure <plugin>@<marketplace>`.

Perintah shell `claude plugin install` tidak pernah meminta nilai `userConfig`. Untuk menetapkan nilai dari shell, lewatkan masing-masing sebagai `--config KEY=VALUE`. Ketika opsi tetap tidak ditetapkan, perintah mencetak baris `userConfig options not yet set` yang menamai kedua cara untuk menetapkannya. [Dialog `userConfig` tidak pernah muncul](/docs/id/plugins/troubleshooting#the-userconfig-dialog-never-appears) mengutip baris.

Untuk field opsi, di mana setiap nilai disimpan, bagaimana komponen mereferensikan nilai yang disimpan, dan field mana yang menolak `${user_config.*}`, lihat [User configuration](/docs/id/plugins/manifest-reference#user-configuration).

<h2 id="path-variables-and-persistent-data">
  Referensikan jalur plugin dan simpan data
</h2>

Anda tidak tahu di mana plugin Anda akan dipasang, jadi referensikan file dan data-nya melalui variabel ini daripada jalur tetap. Mereka disubstitusikan dalam skill, command, dan agent content, dalam hook dan monitor commands, dan dalam konfigurasi server MCP dan LSP. Mereka juga diekspor ke hook, MCP, dan proses LSP:

* **`${CLAUDE_PLUGIN_ROOT}`**: direktori instalasi plugin. Setiap versi memiliki [cache directory](/docs/id/plugins/loading#find-plugins-on-disk)-nya sendiri, jadi jalur berubah ketika plugin diperbarui. Jangan tulis state di sana
* **`${CLAUDE_PLUGIN_DATA}`**: direktori yang bertahan dari update, untuk `node_modules`, virtual environments, dan caches. Itu diselesaikan ke `~/.claude/plugins/data/<id>/` dan dibuat ketika pertama kali direferensikan
* **`${CLAUDE_PROJECT_DIR}`**: root proyek, nilai yang sama yang diterima hooks

Dalam jalur direktori data, `<id>` adalah identifier plugin dengan setiap karakter selain huruf, digit, `_`, dan `-` diganti dengan `-`, jadi `my-plugin@my-marketplace` menjadi `my-plugin-my-marketplace`.

Di Windows, jalur yang disubstitusikan menggunakan forward slashes sehingga shell tidak membaca backslashes sebagai escapes.

<h3 id="install-dependencies-into-the-data-directory">
  Pasang dependensi ke direktori data
</h3>

Untuk plugin yang dipasang marketplace, Claude Code memasang [dependensi paket Node.js](/docs/id/plugins/loading#node-js-package-dependencies) yang memenuhi syarat secara otomatis ketika itu cache plugin, jadi Anda mungkin tidak perlu memasangnya sendiri. Ketika Anda melakukannya, hook `SessionStart` ini memasang `node_modules` ke `${CLAUDE_PLUGIN_DATA}` pada run pertama dan lagi setelah update mengubah `package.json`:

```json hooks/hooks.json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "diff -q \"${CLAUDE_PLUGIN_ROOT}/package.json\" \"${CLAUDE_PLUGIN_DATA}/package.json\" >/dev/null 2>&1 || (cd \"${CLAUDE_PLUGIN_DATA}\" && cp \"${CLAUDE_PLUGIN_ROOT}/package.json\" . && npm install) || rm -f \"${CLAUDE_PLUGIN_DATA}/package.json\""
          }
        ]
      }
    ]
  }
}
```

Setelah sesi pertama, `~/.claude/plugins/data/<id>/node_modules` ada. Server MCP kemudian dapat menetapkan `NODE_PATH` ke `${CLAUDE_PLUGIN_DATA}/node_modules` di `env`-nya. Untuk field mana yang mensubstitusikan variabel mana, lihat [Environment variables](/docs/id/plugins/manifest-reference#environment-variables).

<h2 id="next-steps">
  Langkah berikutnya
</h2>

* [Referensi manifest plugin](/docs/id/plugins/manifest-reference): field `plugin.json`, aturan jalur, dan layout standar
* [Test plugins dengan evals](/docs/id/plugin-evals): periksa bahwa komponen yang Anda tambahkan mengubah perilaku Claude dengan cara yang Anda maksudkan
* [Publikasikan dan distribusikan plugin](/docs/id/plugins/publish): versi plugin dan letakkan di marketplace
* [Troubleshoot plugins](/docs/id/plugins/troubleshooting): apa yang harus dilakukan ketika komponen tidak dimuat atau hook tidak dipecat
