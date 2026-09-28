> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# File pengaturan dan urutan prioritas

> Ubah pengaturan Claude Code, pilih cakupan kunci, verifikasi perubahan, dan pelajari nilai mana yang digunakan Claude Code saat kunci diatur di beberapa tempat.

export const SettingsPrecedence = () => {
  const LEVELS = [{
    n: 1,
    name: 'Managed settings',
    file: 'managed-settings.json, MDM, or the claude.ai console',
    who: 'Your organization',
    w: 390
  }, {
    n: 2,
    name: 'Command line',
    file: 'claude --settings',
    who: 'You, this session',
    w: 420
  }, {
    n: 3,
    name: 'Project local',
    file: '.claude/settings.local.json',
    who: 'You, this project',
    w: 480
  }, {
    n: 4,
    name: 'Shared project',
    file: '.claude/settings.json',
    who: 'Everyone in the project',
    w: 540
  }, {
    n: 5,
    name: 'User',
    file: '~/.claude/settings.json',
    who: 'You, every project',
    w: 600
  }];
  const W = 760;
  const ROW = 58;
  const GAP = 8;
  const TOP = 34;
  const H = TOP + LEVELS.length * (ROW + GAP) + 30;
  const cx = W / 2;
  const mono = 'var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace)';
  const sans = 'var(--font-sans, system-ui, -apple-system, sans-serif)';
  return <div className="sp-root not-prose" role="img" aria-label="Settings precedence, highest first: managed settings, command line, project local, shared project, user. A key set at a higher level overrides the same key set lower down.">
      <style>{`
        .sp-root { --sp-text: #1A1918; --sp-sub: #5E5D59; --sp-faint: #8A8880; --sp-fill: #F5F4EF; --sp-stroke: rgba(0,0,0,0.12); --sp-top: #D97757; --sp-top-fill: rgba(217,119,87,0.14); --sp-arrow: #8A8880; margin: 1.25rem 0; }
        .dark .sp-root { --sp-text: #F1EFE9; --sp-sub: #B8B5AD; --sp-faint: #8A8880; --sp-fill: #24231F; --sp-stroke: rgba(255,255,255,0.12); --sp-top-fill: rgba(217,119,87,0.22); --sp-arrow: #8A8880; }
        .sp-root svg { width: 100%; height: auto; display: block; max-width: ${W}px; margin: 0 auto; }
      `}</style>
      <svg viewBox={`0 0 ${W} ${H}`} xmlns="http://www.w3.org/2000/svg">
        <text x={cx} y={18} textAnchor="middle" fontFamily={sans} fontSize="12.5" fontWeight="600" fill="var(--sp-sub)">Highest precedence</text>
        {LEVELS.map((l, i) => {
    const y = TOP + i * (ROW + GAP);
    const x = cx - l.w / 2;
    const top = i === 0;
    return <g key={l.n}>
              <rect x={x} y={y} width={l.w} height={ROW} rx={10} fill={top ? 'var(--sp-top-fill)' : 'var(--sp-fill)'} stroke={top ? 'var(--sp-top)' : 'var(--sp-stroke)'} strokeWidth={top ? 1.5 : 1} />
              <text x={x + 14} y={y + 24} fontFamily={sans} fontSize="14" fontWeight="600" fill="var(--sp-text)">{l.n}. {l.name}</text>
              <text x={x + 14} y={y + 43} fontFamily={mono} fontSize="11.5" fill="var(--sp-sub)">{l.file}</text>
              <text x={x + l.w - 14} y={y + 24} textAnchor="end" fontFamily={sans} fontSize="12" fill="var(--sp-faint)">{l.who}</text>
            </g>;
  })}
        <text x={cx} y={H - 10} textAnchor="middle" fontFamily={sans} fontSize="12.5" fontWeight="600" fill="var(--sp-sub)">Lowest precedence</text>
        <g stroke="var(--sp-arrow)" strokeWidth="1.5" fill="none">
          <line x1={W - 40} y1={TOP + 10} x2={W - 40} y2={H - 38} />
          <path d={`M ${W - 46} ${TOP + 18} L ${W - 40} ${TOP + 10} L ${W - 34} ${TOP + 18}`} />
        </g>
        <text x={W - 40} y={H - 22} textAnchor="middle" fontFamily={sans} fontSize="10.5" fill="var(--sp-faint)">overrides</text>
      </svg>
    </div>;
};

export const SettingsScope = ({defaultSelected = 'project'}) => {
  const FILES = [{
    id: 'user',
    path: '~/.claude/settings.json'
  }, {
    id: 'project',
    path: 'acme-app/.claude/settings.json'
  }, {
    id: 'local',
    path: 'acme-app/.claude/settings.local.json'
  }, {
    id: 'managed',
    path: 'Managed settings',
    ring: 'managed-settings.json, MDM, or the claude.ai console'
  }];
  const SHORT = {
    user: '~/.claude/settings.json',
    project: 'acme-app/.claude/settings.json',
    local: 'acme-app/.claude/settings.local.json',
    managed: 'managed-settings.json, MDM, or the claude.ai console'
  };
  const TILE_MARK = {
    project: 'settings.json',
    local: 'settings.local.json'
  };
  const initial = FILES.some(f => f.id === defaultSelected) ? defaultSelected : 'project';
  const [sel, setSel] = useState(initial);
  const [scale, setScale] = useState(1);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const rootRef = useRef(null);
  const frameRef = useRef(null);
  const CANVAS_W = 862;
  const CANVAS_H = 240;
  useEffect(() => {
    const el = frameRef.current;
    if (!el) return;
    const measure = () => setScale(Math.min(1, el.clientWidth / CANVAS_W));
    measure();
    if (typeof ResizeObserver === 'undefined') {
      window.addEventListener('resize', measure);
      return () => window.removeEventListener('resize', measure);
    }
    const ro = new ResizeObserver(measure);
    ro.observe(el);
    return () => ro.disconnect();
  }, []);
  useEffect(() => {
    const onFsChange = () => setIsFullscreen(!!document.fullscreenElement);
    document.addEventListener('fullscreenchange', onFsChange);
    return () => document.removeEventListener('fullscreenchange', onFsChange);
  }, []);
  const toggleFullscreen = () => {
    if (!rootRef.current) return;
    if (document.fullscreenElement) document.exitFullscreen(); else rootRef.current.requestFullscreen().catch(() => {});
  };
  const COVERAGE = {
    user: ['website', 'api', 'yacme'],
    project: ['yacme', 'tacme', 'cacme'],
    local: ['yacme'],
    managed: ['website', 'api', 'yacme', 'tacme', 'cacme']
  };
  const RINGS = {
    local: {
      l: 282,
      t: 50,
      w: 142,
      h: 124
    },
    project: {
      l: 282,
      t: 50,
      w: 560,
      h: 124
    },
    user: {
      l: 2,
      t: 34,
      w: 446,
      h: 198
    },
    managed: {
      l: 0,
      t: 32,
      w: 862,
      h: 204
    }
  };
  const TILES = [{
    id: 'website',
    name: 'website/',
    left: 30,
    caption: ''
  }, {
    id: 'api',
    name: 'api/',
    left: 160,
    caption: ''
  }, {
    id: 'yacme',
    name: 'acme-app/',
    left: 290,
    caption: ''
  }, {
    id: 'tacme',
    name: 'acme-app/',
    left: 497,
    caption: sel === 'project' ? 'their clone, once you commit the file' : 'their clone'
  }, {
    id: 'cacme',
    name: 'acme-app/',
    left: 704,
    caption: sel === 'project' ? 'fresh clone, once you commit the file' : sel === 'managed' ? 'server-managed only' : 'fresh clone'
  }];
  const FILE_AT = {
    user: {
      machine: 'you',
      tiles: []
    },
    project: {
      machine: null,
      tiles: ['yacme', 'tacme', 'cacme']
    },
    local: {
      machine: null,
      tiles: ['yacme']
    },
    managed: {
      machine: null,
      tiles: []
    }
  };
  const fileAt = FILE_AT[sel];
  const coverage = COVERAGE[sel];
  const ring = RINGS[sel];
  const selFile = FILES.find(f => f.id === sel);
  const FolderIcon = ({open}) => <svg width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <path d="M1.5 4.5a1 1 0 0 1 1-1h3.2l1.3 1.5h6a1 1 0 0 1 1 1V12a1 1 0 0 1-1 1h-10.5a1 1 0 0 1-1-1z" />
      {open && <path d="M1.5 7.5h13" />}
    </svg>;
  const FileIcon = () => <svg width="10" height="10" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <path d="M4 1.5h5.5L13 5v9.5H4z" />
      <path d="M9.5 1.5V5H13" />
    </svg>;
  const CloudIcon = () => <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <path d="M4.5 12.5h7a2.5 2.5 0 0 0 .4-4.97A3.5 3.5 0 0 0 5.2 6.6 3 3 0 0 0 4.5 12.5z" />
    </svg>;
  const LaptopIcon = () => <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <rect x="2.5" y="3" width="11" height="7.5" rx="1" />
      <path d="M1 12.5h14" />
    </svg>;
  return <div ref={rootRef} className={'ssc-root not-prose' + (isFullscreen ? ' ssc-fs' : '')}>
      <style>{`
        .ssc-root {
          --ssc-bg: #FFFFFF;
          --ssc-text: #1A1918;
          --ssc-sub: #5E5D59;
          --ssc-faint: #8A8880;
          --ssc-border: rgba(0,0,0,0.12);
          --ssc-panel: #F5F4EF;
          --ssc-tile: #FAFAF8;
          --ssc-clay: #D97757;
          --ssc-clay-bg: rgba(217,119,87,0.14);
          --ssc-label: #B0562F;
          --ssc-hover: rgba(115,114,108,0.10);
          font-family: var(--font-sans, system-ui, -apple-system, sans-serif);
          background: var(--ssc-bg);
          color: var(--ssc-text);
          border: 1px solid var(--ssc-border);
          border-radius: 16px;
          padding: 20px 24px 24px;
          margin: 1.5rem 0;
          box-sizing: border-box;
        }
        .dark .ssc-root {
          --ssc-bg: #1B1A18;
          --ssc-text: #F1EFE9;
          --ssc-sub: #B8B5AD;
          --ssc-faint: #8A8880;
          --ssc-border: rgba(255,255,255,0.12);
          --ssc-panel: #24231F;
          --ssc-tile: #2A2925;
          --ssc-clay-bg: rgba(217,119,87,0.20);
          --ssc-label: #EBC9B7;
        }
        .ssc-fs { display: flex; flex-direction: column; justify-content: center; align-items: center; margin: 0; border-radius: 0; height: 100vh; }
        .ssc-fs .ssc-head { width: 100%; max-width: ${CANVAS_W}px; }
        .ssc-fs .ssc-frame { width: 100%; }
        .ssc-mono { font-family: var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace); }
        .ssc-head { display: flex; align-items: flex-start; justify-content: space-between; gap: 12px; margin-bottom: 20px; }
        .ssc-files { display: flex; gap: 8px; flex-wrap: wrap; }
        .ssc-file {
          font-size: 12.5px; font-weight: 430; padding: 8px 13px; border-radius: 10px; cursor: pointer;
          border: 0.5px solid var(--ssc-border); background: var(--ssc-tile); color: var(--ssc-text);
          white-space: nowrap; transition: background 0.2s, border-color 0.2s;
        }
        .ssc-file:hover { filter: brightness(0.97); }
        .ssc-file[aria-pressed="true"] { font-weight: 600; border: 1.5px solid var(--ssc-clay); background: var(--ssc-clay-bg); }
        .ssc-fsbtn {
          display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; flex-shrink: 0;
          border: none; background: none; border-radius: 6px; cursor: pointer; color: var(--ssc-faint); font-size: 15px;
        }
        .ssc-fsbtn:hover { background: var(--ssc-hover); }
        .ssc-frame { width: 100%; max-width: ${CANVAS_W}px; margin: 0 auto; }
        .ssc-canvas { position: relative; width: ${CANVAS_W}px; height: ${CANVAS_H}px; transform-origin: top left; }
        .ssc-machine { position: absolute; top: 42px; height: 182px; background: var(--ssc-panel); border-radius: 16px; }
        .ssc-machine-label { position: absolute; top: 192px; display: flex; align-items: center; gap: 8px; font-size: 13.5px; font-weight: 600; }
        .ssc-tile {
          position: absolute; top: 58px; width: 126px; height: 108px; border-radius: 12px; padding: 11px 12px; box-sizing: border-box;
          background: var(--ssc-tile); border: 0.5px solid var(--ssc-border); opacity: 0.6;
          transition: background 0.25s, border-color 0.25s, opacity 0.25s;
        }
        .ssc-tile.ssc-on { background: var(--ssc-clay-bg); border: 1px solid var(--ssc-clay); opacity: 1; }
        .ssc-tile-name { display: flex; align-items: center; gap: 6px; color: var(--ssc-faint); }
        .ssc-tile.ssc-on .ssc-tile-name { color: var(--ssc-clay); }
        .ssc-tile-name span { font-size: 12px; font-weight: 430; white-space: nowrap; color: var(--ssc-text); }
        .ssc-tile.ssc-on .ssc-tile-name span { font-weight: 600; }
        .ssc-tile-caption { font-size: 10.5px; color: var(--ssc-sub); margin-top: 5px; line-height: 1.35; }
        .ssc-filemark {
          position: absolute; left: 5px; right: 5px; bottom: 8px; display: inline-flex; align-items: center; justify-content: center; gap: 2px;
          font-size: 8.5px; color: var(--ssc-label); background: var(--ssc-bg); border: 1px solid var(--ssc-clay);
          border-radius: 6px; padding: 2px 3px; white-space: nowrap; overflow: hidden;
        }
        .ssc-filemark svg { flex-shrink: 0; }
        .ssc-machine-filemark { display: inline-flex; align-items: center; gap: 4px; margin-left: 10px; font-size: 10.5px; font-weight: 500; color: var(--ssc-label); }
        .ssc-ring {
          position: absolute; border: 2px solid var(--ssc-clay); border-radius: 18px; pointer-events: none;
          transition: left 0.35s ease, top 0.35s ease, width 0.35s ease, height 0.35s ease;
        }
        .ssc-ring-label {
          position: absolute; font-size: 12px; font-weight: 600; color: var(--ssc-label); white-space: nowrap; pointer-events: none;
          transition: left 0.35s ease, top 0.35s ease;
        }
      `}</style>

      <div className="ssc-head">
        <div className="ssc-files" role="group" aria-label="Settings file">
          {FILES.map(f => <button key={f.id} type="button" className="ssc-file ssc-mono" aria-pressed={f.id === sel} onClick={() => setSel(f.id)}>{f.path}</button>)}
        </div>
        <button type="button" className="ssc-fsbtn" onClick={toggleFullscreen} aria-label={isFullscreen ? 'Exit fullscreen' : 'Enter fullscreen'} title={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'}>{isFullscreen ? '⤡' : '⛶'}</button>
      </div>

      <div ref={frameRef} className="ssc-frame" style={{
    height: CANVAS_H * scale + 'px'
  }}>
        <div className="ssc-canvas" style={{
    transform: 'scale(' + scale + ')'
  }}>
          <div className="ssc-machine" style={{
    left: '10px',
    width: '430px'
  }} />
          <span className="ssc-machine-label" style={{
    left: '30px'
  }}><LaptopIcon />Your machine{fileAt.machine === 'you' && <span className="ssc-machine-filemark ssc-mono"><FileIcon />{selFile.path}</span>}</span>
          <div className="ssc-machine" style={{
    left: '460px',
    width: '200px'
  }} />
          <span className="ssc-machine-label" style={{
    left: '470px'
  }}><LaptopIcon />A teammate’s machine</span>
          <div className="ssc-machine" style={{
    left: '682px',
    width: '170px'
  }} />
          <span className="ssc-machine-label" style={{
    left: '692px'
  }}><CloudIcon />A cloud session</span>

          {TILES.map(t => {
    const on = coverage.includes(t.id);
    return <div key={t.id} className={'ssc-tile' + (on ? ' ssc-on' : '')} style={{
      left: t.left + 'px'
    }}>
                <div className="ssc-tile-name"><FolderIcon open={on} /><span className="ssc-mono">{t.name}</span></div>
                {t.caption && <div className="ssc-tile-caption">{t.caption}</div>}
                {fileAt.tiles.includes(t.id) && <span className="ssc-filemark ssc-mono" title={SHORT[sel]}><FileIcon />{TILE_MARK[sel]}</span>}
              </div>;
  })}

          <div className="ssc-ring" style={{
    left: ring.l + 'px',
    top: ring.t + 'px',
    width: ring.w + 'px',
    height: ring.h + 'px'
  }} />
          <span className="ssc-ring-label ssc-mono" style={{
    left: ring.l + 14 + 'px',
    top: ring.t - 26 + 'px'
  }}>{selFile.ring || selFile.path}</span>
        </div>
      </div>
    </div>;
};

Pengaturan adalah kunci JSON yang mengubah perilaku Claude Code: model mana yang dimulainya, apa yang dapat dijalankannya tanpa bertanya, file mana yang tidak dapat dibacanya, tampilannya di terminal Anda, dan apa yang diterapkan organisasi Anda.

<Tip>
  Untuk mencari kunci tertentu, buka [Semua pengaturan](/docs/id/settings-reference), yang mencantumkan setiap kunci dengan file tempat Anda menetapkannya, defaultnya, dan contohnya.
</Tip>

Claude Code membaca pengaturan dari file pengaturan JSON seperti `~/.claude/settings.json`. Ia mencarinya di beberapa lokasi, dan [file tempat ia membaca pengaturan menentukan siapa pengaturan itu berlaku untuk](#settings-files-and-who-they-affect). Halaman ini mencakup file-file tersebut: file mana yang harus Anda masukkan pengaturan, cara mengubah pengaturan dan mengonfirmasi bahwa pengaturan itu diterapkan, dan nilai mana yang digunakan Claude Code saat kunci yang sama diatur di lebih dari satu file. [Konfigurasikan izin](/docs/id/permissions) mencakup apa yang dapat dijalankan Claude Code tanpa bertanya dan cara menulis aturan `allow`, `ask`, dan `deny`.

<Note>
  Halaman ini mencakup Claude Code yang berjalan di mesin Anda: terminal, ekstensi [VS Code](/docs/id/vs-code) dan [JetBrains](/docs/id/jetbrains), dan [aplikasi desktop](/docs/id/desktop), yang semuanya membaca file pengaturan yang sama. Sesi cloud di [Claude Code di web](/docs/id/claude-code-on-the-web) berjalan di mesin yang berbeda dan hanya membaca beberapa di antaranya; lihat [Pengaturan dalam sesi cloud](#settings-in-cloud-sessions).
</Note>

<span id="settings-files" />

<span id="configuration-scopes" />

<span id="available-scopes" />

<span id="when-to-use-each-scope" />

<span id="what-uses-scopes" />

<span id="subagent-configuration" />

<span id="where-settings-live" />

<h2 id="settings-files-and-who-they-affect">
  File pengaturan dan siapa yang terpengaruh
</h2>

Claude Code membaca pengaturan dari empat file, dan organisasi juga dapat mengirimkan pengaturan terkelola dari konsol claude.ai. Setiap sumber memiliki cakupan: set orang dan proyek yang pengaturan yang disimpan di dalamnya berlaku, baik itu hanya Anda, semua orang dalam proyek, atau semua orang di organisasi Anda.

| Cakupan        | File                                                                                             | Siapa yang terpengaruh                                                                                                                                                                  | Gunakan untuk                                                                  |
| :------------- | :----------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------- |
| Pengguna       | `~/.claude/settings.json`                                                                        | Anda, di setiap proyek di mesin ini                                                                                                                                                     | Preferensi pribadi: tema, mode editor, model default, aturan izin Anda sendiri |
| Proyek bersama | `.claude/settings.json`                                                                          | Semua orang yang bekerja di folder yang memuatnya. Di repositori git, commit itu sehingga rekan kerja mendapatkannya                                                                    | Izin tim, hooks, plugins, dan variabel lingkungan yang dibutuhkan proyek       |
| Proyek lokal   | `.claude/settings.local.json`                                                                    | Anda, hanya di proyek ini. Claude Code menyimpannya di luar git saat membuat file; jika Anda membuatnya dengan tangan, tambahkan ke `.gitignore` sendiri                                | Penggantian pribadi untuk satu proyek, dan pengujian sebelum Anda berbagi      |
| Terkelola      | `managed-settings.json` dan [sumber terkelola](/docs/id/managed-settings#delivery-mechanisms) lainnya | Semua orang organisasi Anda menerapkannya; tidak ada yang Anda atur yang menggantikannya, kecuali beberapa [pengecualian sensitif keamanan](#exceptions-to-managed-settings-precedence) | Kebijakan keamanan dan persyaratan kepatuhan                                   |

Di kolom File, `~/.claude` adalah folder `.claude` di direktori home Anda, dan `.claude` kosong adalah folder `.claude` di dalam proyek Anda.

<span id="where-each-file-applies" />

<span id="compare-what-each-file-reaches" />

<h3 id="compare-the-scope-of-each-settings-file">
  Bandingkan cakupan setiap file pengaturan
</h3>

Misalkan Anda memiliki tiga proyek di mesin Anda, `website/`, `api/`, dan `acme-app/`, rekan kerja memiliki klon mereka sendiri dari `acme-app/`, dan Anda memulai [sesi cloud](#settings-in-cloud-sessions) di `acme-app/`.

Grafik di bawah menunjukkan folder mana yang pengaturan berlaku saat Anda memulai Claude Code dari mereka. Klik file pengaturan untuk melihat folder yang dijangkaunya.

<SettingsScope />

* **`~/.claude/settings.json`**: setiap proyek di mesin Anda, dan tidak ada di mesin rekan kerja atau di sesi cloud
* **`acme-app/.claude/settings.json`**: `acme-app/` Anda. Ini menjangkau klon rekan kerja dan sesi cloud hanya jika Anda commit file ke kontrol versi; sampai Anda melakukannya, itu adalah file di disk Anda seperti yang lain dan tidak ada orang lain yang memilikinya
* **`acme-app/.claude/settings.local.json`**: hanya `acme-app/` Anda. Claude Code menambahkannya ke pengecualian git global Anda pertama kali menulis file, sehingga tetap keluar dari commit Anda; jika Anda membuat file dengan tangan, [tambahkan ke `.gitignore` sendiri](#keep-personal-settings-out-of-a-repository)
* **Pengaturan terkelola**, baik file `managed-settings.json`, kebijakan MDM, atau [pengaturan yang dikelola server](/docs/id/server-managed-settings) dari konsol claude.ai: setiap proyek di setiap mesin organisasi Anda menerapkannya, atau yang Anda masuk dengan akun organisasi Anda. Hanya pengaturan yang dikelola server yang menjangkau sesi cloud

<span id="which-files-you-have" />

<h3 id="find-or-create-your-settings-files">
  Temukan atau buat file pengaturan Anda
</h3>

Menginstal Claude Code tidak membuat file pengaturan apa pun. Jika mesin atau proyek Anda sudah memiliki satu, itu berasal dari salah satu sumber ini:

* **Terkelola**: organisasi Anda menerapkannya. Anda tidak membuat atau mengeditnya.
* **Proyek bersama**: proyek yang sudah menggunakan Claude Code mungkin memiliki satu yang di-commit. Jika tidak, buatnya di `.claude/settings.json` di folder proyek.
* **Pengguna** dan **Proyek lokal**: buatnya sendiri, atau biarkan Claude Code membuatnya. Ini menulis `~/.claude/settings.json` pertama kali Anda mengubah opsi di menu `/config` yang disimpannya dalam pengaturan pengguna, seperti tema, dan `.claude/settings.local.json` pertama kali Anda memberikan persetujuan berdiri di prompt izin, seperti "Ya, dan jangan tanya lagi" untuk perintah Bash. Beberapa opsi `/config`, termasuk **Tampilkan tips**, disimpan ke `.claude/settings.local.json` sebagai gantinya dari file pengguna.

<Info>
  Di Windows, `~/.claude` berarti `%USERPROFILE%\.claude`. Untuk menyimpan file direktori home di tempat lain, atur [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars); Claude Code kemudian menyimpan pengaturan, riwayat sesi, dan plugins Anda di sana sebagai gantinya.
</Info>

Claude Code juga menyimpan file kelima, [`~/.claude.json`](/docs/id/claude-directory#ce-claude-json), yang ditulis untuk dirinya sendiri; Anda tidak perlu mengeditnya. Ini menyimpan sesi masuk Anda, konfigurasi [server MCP](/docs/id/mcp), status per-proyek seperti keputusan kepercayaan, dan [kunci konfigurasi global](/docs/id/settings-reference#global-config-settings) yang `/config` tulis untuk Anda.

<h3 id="share-settings-with-your-team">
  Bagikan pengaturan dengan tim Anda
</h3>

Commit `.claude/settings.json` sehingga semua orang yang mengkloning repositori mendapatkan izin, hooks, dan plugins yang sama. Setiap rekan kerja masih dapat menggantikannya untuk diri mereka sendiri di `.claude/settings.local.json` mereka sendiri, sehingga pengecualian pribadi tidak perlu commit. Untuk file tim lengkap, lihat [pengaturan bersama tim](/docs/id/settings-example#a-teams-shared-settings).

Beberapa dari apa yang Anda commit menunggu sampai setiap rekan kerja [mempercayai folder](/docs/id/permissions#project-allow-rules-and-workspace-trust), dan beberapa kunci tidak pernah berlaku dari file repositori; [Troubleshoot a setting that doesn't apply](#common-cases) mencakup keduanya.

<span id="local-settings-file" />

<span id="where-claude-code-saves-the-project-local-file" />

<span id="the-project-local-file" />

<span id="keep-personal-settings-out-of-the-repository" />

<h3 id="keep-personal-settings-out-of-a-repository">
  Jaga pengaturan pribadi keluar dari repositori
</h3>

Untuk mengubah pengaturan untuk diri sendiri di satu proyek tanpa mengubahnya untuk rekan kerja Anda, simpan di `.claude/settings.local.json` di dalam proyek. Claude Code menerapkan file itu di atas `.claude/settings.json` yang di-commit, jadi jika file tim Anda menetapkan `"model": "claude-sonnet-5"` dan Anda menginginkan Opus, masukkan `"model": "claude-opus-5-5"` di file lokal Anda dan hanya sesi Anda yang berubah.

Claude Code juga menulis ke file ini, menyimpannya keluar dari commit Anda, dan menerapkan aturan izinnya tanpa langkah kepercayaan:

* **Claude Code juga menulisnya.** Ketika Claude meminta izin untuk menjalankan perintah Bash dan Anda memilih "Ya, dan jangan tanya lagi", Claude Code menyimpan [persetujuan izin](/docs/id/permissions#permission-system) itu di sini sebagai aturan `allow`.
* **Anda tidak perlu gitignore sendiri, kecuali Anda membuatnya dengan tangan.** Pertama kali Claude Code menulis file di repositori git yang tidak sudah mengabaikannya, itu menambahkan `**/.claude/settings.local.json` ke file pengecualian git global Anda, sehingga file tetap keluar dari commit Anda di setiap repositori. File itu adalah `core.excludesFile` ketika konfigurasi git global Anda menetapkannya ke jalur absolut atau jalur dengan awalan `~`; jika tidak, itu adalah `$XDG_CONFIG_HOME/git/ignore`, atau `~/.config/git/ignore` ketika `XDG_CONFIG_HOME` tidak diatur. Jika Anda membuat file dengan tangan dan Claude Code belum menulis ke dalamnya, tambahkan ke `.gitignore` sendiri.
* **Aturan izinnya tidak menunggu kepercayaan sementara file tetap tidak dilacak.** Karena file adalah milik Anda dan bukan repositori, Claude Code menerapkan aturan `allow` tanpa langkah [kepercayaan ruang kerja](/docs/id/permissions#project-allow-rules-and-workspace-trust) yang diperlukan untuk file yang di-commit. Jika file dilacak oleh git, langkah kepercayaan juga berlaku untuk itu; lihat [Ketika file pengaturan lokal Anda memerlukan kepercayaan](/docs/id/permissions#when-your-local-settings-file-needs-trust).

<span id="where-claude-code-looks-for-each-file" />

<span id="how-claude-code-keeps-the-local-file-out-of-git" />

<span id="local-allow-rules-dont-wait-for-workspace-trust" />

<h4 id="where-claude-code-keeps-the-local-file-in-a-git-repository">
  Tempat Claude Code menyimpan file lokal di repositori git
</h4>

Ketika Claude meminta izin untuk menjalankan perintah Bash dan Anda memilih "Ya, dan jangan tanya lagi", Claude Code menyimpan persetujuan itu sebagai aturan `allow` di `.claude/settings.local.json`. Jika Anda memulai Claude Code di subdirektori repositori git, itu membaca dan menulis file itu di akar repositori dan menerapkan persetujuan di seluruh repositori. Di [worktree](/docs/id/worktrees), itu menggunakan file di akar checkout utama.

Dua aturan memenuhi syarat lokasi akar:

* **Ketika file tetap dengan `.claude/settings.json` sebagai gantinya**: di luar repositori git, ketika akar repositori adalah direktori home Anda, di Windows, atau ketika akar repositori atau entri `.git` atau `.claude` tidak dimiliki oleh pengguna Anda.
* **Jalur dalam file tidak jangkar di akar repositori**: aturan izin yang dimulai dengan `/` atau jalur sandbox relatif [jangkar di direktori kerja utama sesi](/docs/id/permissions#read-and-edit) sebagai gantinya.

Sebelum v2.1.211, Claude Code menyimpan file di direktori awal. Itu masih membaca file versi sebelumnya yang ditinggalkan di sana di samping file akar; di mana keduanya menetapkan kunci yang sama, nilai akar berlaku, dan aturan izin dari kedua file berlaku. Helper [`resolveSettings()`](/docs/id/agent-sdk/typescript#resolvesettings) SDK Agent selalu membaca file dari direktori awal.

Claude Code membaca `.claude/settings.json` bersama dari [direktori kerja utama](/docs/id/permissions#working-directories) sesi, jadi untuk menggunakan file yang di-commit di akar repositori, mulai Claude Code di sana. Setelah Anda [memindahkan sesi dengan `/cd`](/docs/id/permissions#move-the-session-to-another-directory), Claude Code membaca kedua file proyek dari direktori baru sebagai gantinya, menempatkan file lokal dengan aturan yang sama. Membaca mereka dari direktori yang Anda pindahkan memerlukan Claude Code v2.1.246 atau lebih baru.

<span id="managed-settings-delivery" />

<span id="precedence-within-the-managed-tier" />

<span id="parent-settings-from-embedding-hosts" />

<span id="enforce-settings-for-an-organization" />

<span id="settings-your-organization-manages" />

<h3 id="check-what-your-organization-enforces">
  Periksa apa yang organisasi Anda terapkan
</h3>

Jika organisasi Anda mengelola Claude Code, beberapa pengaturan diputuskan untuk Anda dan tidak ada yang Anda masukkan dalam file Anda sendiri yang mengubahnya. Untuk melihat mana, jalankan `/status`: baris `Setting sources` menamai sumber terkelola yang berlaku untuk Anda. Pengaturan terkelola berlaku di mana pun Claude Code berjalan di mesin ini; [Apa yang dapat diubah pengembang](/docs/id/managed-settings#what-a-developer-can-change) mencakup hak admin lokal dan alat selain Claude Code.

Pengaturan terkelola menjangkau Anda melalui [mekanisme pengiriman](/docs/id/managed-settings#delivery-mechanisms) di halaman pengaturan terkelola, paling umum:

* [Pengaturan yang dikelola server](/docs/id/server-managed-settings), yang Claude Code ambil dari konsol admin claude.ai atau [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang di-host sendiri
* Kebijakan MDM atau tingkat OS, dan file `managed-settings.json` di direktori sistem
* Host penyematan seperti Claude Desktop, melalui opsi SDK `managedSettings`; lihat [Kontrol kebijakan dari host penyematan](/docs/id/managed-settings#parent-settings-from-embedding-hosts)

Di sesi [Cowork](https://claude.com/docs/cowork/overview) yang berjalan di mesin Anda di aplikasi Claude Desktop, Claude Code tidak mengambil pengaturan yang dikelola server dari konsol admin claude.ai, dan itu membaca kebijakan yang diterapkan ke perangkat Anda kecuali konfigurasi Claude Desktop organisasi Anda menetapkan `requireCoworkFullVmSandbox`. [Di mana dan kapan kebijakan berlaku](/docs/id/managed-settings#where-and-when-a-policy-applies) mencakup Cowork dan sesi cloud.

Jika Anda adalah administrator, [Siapkan Claude Code untuk organisasi Anda](/docs/id/admin-setup) memandu memilih apa yang akan diterapkan, dan [Terapkan pengaturan terkelola](/docs/id/managed-settings) mencakup pengiriman dan cara mengonfirmasi kebijakan berlaku.

<h2 id="change-a-setting">
  Ubah pengaturan
</h2>

Anda dapat mengubah pengaturan dari menu `/config`, dengan mengedit file pengaturan, atau untuk satu sesi dari baris perintah.

<span id="system-prompt" />

Prompt sistem Claude Code tidak dipublikasikan. Untuk memberikan Claude instruksi berdiri, gunakan file [`CLAUDE.md`](/docs/id/memory) atau flag `--append-system-prompt`.

<h3 id="use-the-/config-menu">
  Gunakan menu /config
</h3>

Jalankan `/config` di dalam Claude Code dan buka tab **Config**. Ia mencantumkan set pendek opsi pribadi seperti tema, mode editor, dan output verbose, bukan setiap kunci pengaturan. Pilih opsi untuk mengubahnya; Claude Code menyimpannya untuk Anda:

* **Sebagian besar opsi**: `~/.claude/settings.json`
* **Beberapa opsi, seperti Tampilkan tips**: `.claude/settings.local.json`
* **[Opsi konfigurasi global](/docs/id/settings-reference#global-config-settings)**: `~/.claude.json`

Untuk menetapkan satu opsi tanpa menu, lewatkan `key=value`, seperti `/config verbose=true`.

<Note>
  `/config` adalah bagian dari antarmuka terminal. Panel chat [VS Code](/docs/id/vs-code) dan [aplikasi desktop](/docs/id/desktop) tidak membukanya; ubah pengaturan di sana dengan mengedit file pengaturan atau melalui pengaturan aplikasi itu sendiri.
</Note>

<h3 id="edit-a-settings-file">
  Edit file pengaturan
</h3>

Buka file pengaturan untuk cakupan yang Anda inginkan di editor Anda dan tambahkan atau ubah kunci. File pengaturan adalah JSON ketat: komentar `//` atau koma tertinggal adalah kesalahan sintaks, dan Claude Code melaporkan file sebagai [Kesalahan Pengaturan](#fix-a-broken-settings-file) saat startup berikutnya. Misalnya, untuk membiarkan Claude Code menjalankan perintah lint dan test Anda tanpa bertanya dan menghentikannya membaca file `.env`, tambahkan ini ke `~/.claude/settings.json`:

```json ~/.claude/settings.json theme={null}
{
  "$schema": "https://json.schemastore.org/claude-code-settings.json",
  "permissions": {
    "allow": [
      "Bash(npm run lint)",
      "Bash(npm run test *)"
    ],
    "deny": [
      "Read(./.env)",
      "Read(./.env.*)"
    ]
  }
}
```

Setiap entri di bawah `permissions` adalah aturan yang menamai tool dan apa yang boleh dilakukannya; [Konfigurasikan izin](/docs/id/permissions) menjelaskan sintaksnya. Baris `$schema` menunjuk ke [skema JSON yang dipublikasikan](https://json.schemastore.org/claude-code-settings.json) untuk pengaturan Claude Code, yang memberi Anda pelengkapan otomatis dan validasi inline di VS Code, Cursor, dan editor lain yang mendukung skema JSON. Skema dapat tertinggal di belakang rilis CLI terbaru, jadi peringatan validasi pada kunci yang baru didokumentasikan tidak berarti konfigurasi Anda tidak valid.

Setelah Anda menyimpan, jalankan `/status` di dalam Claude Code untuk mengonfirmasi file dimuat; [Konfirmasi apa yang dimuat](#check-what-loaded) mengatakan apa yang ditunjukkan baris `Setting sources` dan bagaimana file yang rusak dilaporkan.

Untuk file pribadi lengkap, file tim, dan file organisasi, masing-masing ditampilkan dengan komentar pada setiap kunci yang ditetapkannya, lihat [file pengaturan contoh](/docs/id/settings-example).

<span id="pass-settings-for-one-session" />

<h3 id="change-a-setting-for-one-session">
  Ubah pengaturan untuk satu sesi
</h3>

Untuk mencoba nilai tanpa menyimpannya, aturnya saat Anda memulai Claude Code. Nilai berlaku untuk sesi itu dan file pengaturan Anda tetap seperti sebelumnya. Anda memiliki tiga cara untuk melakukannya:

* **`--settings`**: lewatkan kunci sebagai JSON, inline atau sebagai jalur ke file. Claude Code menerapkannya di atas file pengguna, proyek, dan lokal Anda dan di bawah pengaturan yang dikelola. Ia dapat menetapkan kunci apa pun yang dapat ditetapkan file pengaturan pengguna Anda; ia tidak dapat menetapkan kunci `Managed` atau `Global config`.
* **Flag untuk kunci itu**: beberapa kunci memiliki flag mereka sendiri, seperti `--model` untuk `model` dan `--effort` untuk `effortLevel` dan `modelSettings`.
* **Variabel lingkungan**: ekspor variabel berpasangan kunci sebelum Anda menjalankan `claude`, seperti `ANTHROPIC_MODEL` untuk `model`.

Setiap entri kunci di [referensi pengaturan](/docs/id/settings-reference) mencantumkan penggantian per-sesinya dan mana yang memiliki prioritas, jadi periksa entri untuk kunci yang ingin Anda ubah.

Perintah yang Anda jalankan di dalam sesi sebagian besar menyimpan pilihan Anda: ketika Anda mengubah pengaturan di `/config`, Claude Code menulis ke file pengaturan Anda, dan `/model` menyimpan nilai sebagai default Anda untuk sesi baru.

Jika Anda menekan `s` di pemilih `/model`, Claude Code beralih model tanpa menyimpannya sebagai default pengguna Anda. [Sesuaikan level usaha](/docs/id/model-config#adjust-effort-level) mengatakan level `/effort` mana yang Claude Code simpan sebagai default Anda untuk model yang Anda gunakan dan mana yang berlaku hanya untuk sesi saat ini.

Misalnya, untuk memulai satu sesi di Opus tanpa mengubah default Anda:

```bash theme={null}
claude --settings '{"model": "claude-opus-5-5"}'
```

<h3 id="when-edits-take-effect">
  Saat pengeditan berlaku
</h3>

Claude Code memantau file pengaturan Anda dan memuat ulang saat berubah, jadi ia menerapkan sebagian besar pengeditan ke sesi yang sedang berjalan tanpa restart, termasuk pengeditan ke `permissions`, `hooks`, dan pembantu kredensial seperti `apiKeyHelper`. Claude Code juga memuat file pengaturan yang Anda buat di tengah sesi jika foldernya ada saat sesi dimulai. Untuk folder `.claude/` proyek, ia memuat file bahkan ketika Anda membuat folder di sesi yang sama.

Reload mencakup pengaturan pengguna, proyek, lokal, dan yang dikelola, dan Claude Code menjalankan hook [`ConfigChange`](/docs/id/hooks#configchange) untuk setiap perubahan file pengaturan yang terdeteksi, bukan untuk pengaturan yang dikelola yang tiba dari MDM atau konsol claude.ai. Pengaturan yang dikelola yang tiba melalui MDM atau dari konsol claude.ai mencapai sesi yang sedang berjalan sesuai jadwal daripada saat disimpan; [tabel pengiriman](/docs/id/managed-settings#choose-a-delivery-mechanism) memberikannya per sumber.

Claude Code membaca beberapa kunci hanya sekali, saat startup sesi, jadi pengeditan ke salah satunya tidak mencapai sesi yang sedang berjalan. Kunci tingkat admin yang juga menunggu restart, seperti `requiredMinimumVersion`, tercantum di bawah [tempat dan kapan kebijakan berlaku](/docs/id/managed-settings#where-and-when-a-policy-applies). Yang paling mungkin Anda edit di tengah sesi:

* [`model`](/docs/id/settings-reference#model): gunakan [`/model`](/docs/id/model-config#setting-your-model) untuk beralih di tengah sesi. Setiap model memiliki cache prompt-nya sendiri, jadi permintaan pertama setelah switch membaca ulang seluruh percakapan tanpa cache; lihat [Beralih model](/docs/id/prompt-caching#switching-models)
* [`effortLevel`](/docs/id/settings-reference#effortlevel) dan [`modelSettings`](/docs/id/settings-reference#modelsettings): gunakan [`/effort`](/docs/id/model-config#adjust-effort-level) untuk mengubah usaha di tengah sesi

<span id="verify-active-settings" />

<span id="check-what-loaded" />

<h3 id="confirm-what-loaded">
  Konfirmasi apa yang dimuat
</h3>

Jalankan `/status` di dalam Claude Code untuk melihat sumber pengaturan mana yang aktif. Tab **Status** mencakup baris `Setting sources` yang mencantumkan setiap file pengaturan yang Claude Code muat untuk sesi saat ini, seperti `User settings` atau `Project local settings`. Saat [pengaturan yang dikelola](/docs/id/admin-setup#decide-how-settings-reach-devices) berlaku, entri yang dikelola menunjukkan dalam tanda kurung bagaimana mereka mencapai mesin Anda.

Baris mengonfirmasi file mana yang dibaca Claude Code; ia tidak menunjukkan file mana yang memasok setiap kunci. Untuk mencantumkan entri yang Claude Code tolak, jalankan [`claude doctor`](/docs/id/debug-your-config); untuk model yang ditetapkan pengaturan proyek atau yang dikelola, header startup menamai file yang menetapkannya. `/status` dan `/config` membuka dialog yang sama di tab berbeda, dan tab **Config** bukan tampilan konten `settings.json` Anda.

<h3 id="fix-a-broken-settings-file">
  Perbaiki file pengaturan yang rusak
</h3>

Jika Anda salah ketik JSON atau menetapkan kunci ke nilai yang tidak diterima Claude Code, Claude Code memberi tahu Anda saat awal sesi interaktif. Apa yang ditampilkan tergantung pada berapa banyak file yang terpengaruh:

* **Kesalahan Pengaturan**: file pengguna, proyek, atau lokal memiliki JSON tidak valid atau nilai yang ditolak skema. Saat awal sesi interaktif Claude Code menampilkan dialog yang memungkinkan Anda memperbaiki file dengan bantuan Claude, keluar, atau lanjutkan tanpa pengaturan yang rusak.
* **Peringatan Pengaturan**: hanya entri individual yang gagal, seperti aturan izin yang salah bentuk atau nama acara hook yang tidak dikenal. Claude Code melewati nilai-nilai itu dan menjaga sisa file tetap berlaku.
* **Pengaturan yang dikelola**: Claude Code terus menegakkan sisa file. [Entri tidak valid dalam pengaturan yang dikelola](/docs/id/managed-settings#invalid-entries-in-managed-settings) mengatakan apa yang dijatuhkan dan kunci mana yang kembali ke nilai yang lebih ketat sampai Anda memperbaikinya. Untuk dokumen pengaturan yang dikelola yang bukan JSON valid, lihat [Dokumen pengaturan yang dikelola tidak dapat diuraikan](/docs/id/errors#managed-settings-document-could-not-be-parsed).
* **Kesalahan konfigurasi**: `~/.claude.json` tidak dapat diuraikan. Claude Code menyalin file yang rusak ke `~/.claude/backups/.claude.json.corrupted.<timestamp>` dan menanyakan apakah akan keluar dan memperbaikinya dengan tangan atau mengatur ulang ke konfigurasi default; run `-p` mencetak kesalahan dan keluar. Untuk memulihkan status sebelumnya, salin kembali salah satu dari lima file `.claude.json.backup.<timestamp>` terbaru di `~/.claude/backups/`, yang Claude Code simpan sebelum menulis file.

Setelah Anda lanjutkan, jalankan `/status` untuk melihat file yang terpengaruh dan `claude doctor` untuk detail setiap kesalahan.

Run `-p` menampilkan tidak ada dialog. Kecuali [dokumen pengaturan yang dikelola tidak dapat diuraikan](/docs/id/errors#managed-settings-document-could-not-be-parsed), Claude Code melewati file atau nilai yang rusak dan lanjutkan dengan sisanya, jadi setelah run `-p` yang mengabaikan pengaturan, jalankan `claude doctor` untuk melihat apa yang dijatuhkannya.

<span id="how-scopes-interact" />

<span id="key-points-about-the-configuration-system" />

<span id="which-value-claude-code-uses" />

<span id="which-value-wins" />

<h2 id="settings-precedence">
  Urutan prioritas pengaturan
</h2>

Saat kunci yang sama muncul di lebih dari satu tempat, Claude Code menggunakan nilai dari level tertinggi yang menetapkannya. Tumpukan di bawah menunjukkan level, tertinggi di atas; kunci di level yang lebih tinggi menimpa kunci yang sama di mana pun di bawahnya.

<SettingsPrecedence />

Secara berurutan, prioritas tertinggi terlebih dahulu:

1. **Pengaturan yang dikelola**: pengaturan yang diterapkan organisasi Anda, oleh file `managed-settings.json`, kebijakan MDM, atau [pengaturan yang dikelola server](/docs/id/server-managed-settings) dari konsol claude.ai. Tidak ada yang Anda atur menimpanya: kunci yang Anda lewatkan dengan `--settings` tidak menimpa kunci yang dikelola yang sama, dan flag seperti `--model` hanya memilih dari model yang diizinkan organisasi Anda. Model `model` yang dikelola menetapkan model yang dimulai setiap sesi, dan Anda masih dapat beralih dengan `/model`; kuncinya adalah [`availableModels`](/docs/id/settings-reference#availablemodels), yang membatasi `/model`, `--model`, dan kunci `model` dalam file Anda sendiri. Saat organisasi Anda mengirimkan lebih dari satu sumber yang dikelola, aturan untuk [urutan prioritas dalam tier yang dikelola](/docs/id/managed-settings#precedence-within-the-managed-tier) mengatakan apa yang dibaca Claude Code dari masing-masing.
2. **Argumen baris perintah**: flag yang Anda lewatkan saat memulai `claude` dari terminal, untuk satu sesi; lihat [Ubah pengaturan untuk satu sesi](#change-a-setting-for-one-session). Claude Code menggabungkan JSON yang Anda lewatkan dengan `--settings <file-or-json>` dengan file pengaturan Anda dengan aturan yang sama seperti level lain: ia mengambil kunci yang Anda atur di sini di atas kunci yang sama dalam pengaturan lokal, proyek, atau pengguna, dan menyimpan nilai level yang lebih rendah untuk kunci yang Anda lewatkan.
3. **Pengaturan proyek lokal** (`.claude/settings.local.json`): pengaturan pribadi Anda untuk proyek ini.
4. **Pengaturan proyek bersama** (`.claude/settings.json`): pengaturan yang tim Anda periksa ke kontrol sumber.
5. **Pengaturan pengguna** (`~/.claude/settings.json`): pengaturan pribadi Anda untuk setiap proyek.

Variabel lingkungan bukan level dalam tumpukan ini. Saat perilaku memiliki variabel shell dan kunci pengaturan, mana yang berlaku diputuskan per pasangan, bukan per level: `ANTHROPIC_MODEL` yang diekspor dalam shell Anda berlaku di atas kunci `model` dari file apa pun, sementara `ANTHROPIC_DEFAULT_MODEL` hanya berlaku saat tidak ada file yang menetapkan `model`. [Referensi variabel lingkungan](/docs/id/env-vars#precedence) mengatakan kunci mana yang memiliki pasangan dan mana yang dibaca Claude Code terlebih dahulu. Blok `env` di dalam file pengaturan adalah kunci biasa dan mengikuti level di atas.

Untuk beberapa kunci yang sensitif terhadap keamanan, Claude Code menghormati nilai yang lebih ketat dari level yang lebih rendah di atas nilai yang dikelola; [Pengecualian untuk urutan prioritas pengaturan yang dikelola](#exceptions-to-managed-settings-precedence) mencantumnya.

<h3 id="lists-merge-instead-of-overriding">
  Daftar digabungkan daripada ditimpa
</h3>

Saat Anda menetapkan kunci daftar yang sama, seperti `permissions.allow`, di lebih dari satu file, Claude Code menggabungkan daftar daripada memilih satu, jadi setiap file dapat menambahkan entri tanpa menghapus file lain. Empat kunci yang menyimpan daftar model atau entri per-model mengikuti aturan mereka sendiri:

* [`fallbackModel`](/docs/id/settings-reference#fallbackmodel) adalah rantai yang dipesan di mana posisi membawa makna, jadi Claude Code mengambil seluruh nilai dari file dengan prioritas tertinggi yang mendefinisikannya.
* [`modelPicker`](/docs/id/settings-reference#modelpicker) menyimpan satu daftar baris yang dipesan ditambah flag penggantian, jadi Claude Code tidak pernah menggabungkan baris dari dua sumber. Ia mengambil seluruh nilai dari yang tertinggi dari pengaturan yang dikelola, `--settings`, dan pengaturan pengguna yang mendefinisikannya, dan mengabaikan kunci dalam pengaturan proyek dan lokal. Memerlukan Claude Code v2.1.242 atau lebih baru.
* [`availableModels`](/docs/id/settings-reference#availablemodels): saat pengaturan yang dikelola yang diterapkan Claude Code mendefinisikannya, Claude Code menerapkan daftar itu apa adanya dan mengabaikan entri yang Anda tambahkan dalam pengaturan pengguna, proyek, atau lokal, kecuali aplikasi yang menyematkan Claude Code memasok daftar model-nya sendiri; lihat [Pengecualian untuk urutan prioritas pengaturan yang dikelola](#exceptions-to-managed-settings-precedence). Di seluruh sumber yang dikelola daftar tidak pernah digabungkan juga; [bagaimana Claude Code menggabungkan sumber yang dikelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources) mengatakan daftar sumber mana yang berlaku. Di seluruh cakupan non-managed Claude Code menggabungkan array seperti biasa.
* [`modelSettings`](/docs/id/settings-reference#modelsettings): Claude Code menyelesaikannya satu model pada satu waktu, bersama dengan [`effortLevel`](/docs/id/settings-reference#effortlevel). Entri `modelSettings` menyatakan file mana yang nilainya berlaku untuk model.

<span id="examples" />

<h3 id="precedence-examples">
  Contoh urutan prioritas
</h3>

Saat Claude bekerja, Claude Code menampilkan tip satu baris di bawah spinner, seperti "Gunakan /config untuk mengubah mode izin default Anda (termasuk Plan Mode)". Misalkan Anda menginginkan tips itu mati, jadi Anda menetapkan [`spinnerTipsEnabled`](/docs/id/settings-reference#spinnertipsenabled) ke `false` di `~/.claude/settings.json`. Setiap skenario di bawah adalah sesuatu yang dapat menghidupkannya kembali, dan apa yang dapat Anda lakukan tentangnya.

<h4 id="team-settings-override-personal-settings">
  Pengaturan tim menimpa pengaturan pribadi
</h4>

`.claude/settings.json` tim Anda menetapkannya ke `true`. Claude Code menggunakan nilai proyek karena proyek bersama duduk di atas pengguna, jadi Anda melihat tips di proyek itu dan tidak di tempat lain.

Anda dapat mendapatkan nilai Anda kembali: tambahkan `"spinnerTipsEnabled": false` ke `.claude/settings.local.json` di proyek itu. Proyek lokal duduk di atas proyek bersama, jadi sesi Anda di sana berhenti menampilkan tips dan sesi rekan kerja Anda tidak berubah.

<h4 id="organization-settings-override-everything">
  Pengaturan organisasi menimpa segalanya
</h4>

Pengaturan yang dikelola organisasi Anda menetapkannya ke `true`. Tidak ada yang Anda masukkan dalam pengaturan pengguna, proyek, atau lokal yang mematikan tips, dan juga tidak `--settings`. Managed adalah level teratas.

Anda tidak dapat mendapatkan nilai Anda kembali. Jalankan `/status` untuk melihat sumber yang dikelola mana yang berlaku, dan tanyakan administrator Anda apakah kebijakan harus berubah.

<h4 id="the-command-line-overrides-your-files-for-one-session">
  Baris perintah menimpa file Anda untuk satu sesi
</h4>

Anda memulai sesi dengan `claude --settings '{"spinnerTipsEnabled": true}'`. Baris perintah duduk di atas setiap file kecuali yang dikelola, jadi sesi itu menampilkan tips meskipun file Anda mengatakan `false`.

Anda mendapatkan nilai Anda kembali pada sesi berikutnya; `--settings` berlangsung satu sesi dan tidak menulis ke file apa pun.

<h4 id="a-flag-or-environment-variable-sets-the-same-thing">
  Flag atau variabel lingkungan menetapkan hal yang sama
</h4>

Beberapa kunci memiliki flag baris perintah atau variabel lingkungan yang menimpa nilai pengaturan terlepas dari file mana yang menetapkannya: `ANTHROPIC_MODEL` menimpa pengaturan [`model`](/docs/id/settings-reference#model), dan `--model` menimpa keduanya untuk sesi.

Apakah Anda dapat mendapatkan nilai Anda kembali tergantung pada kunci: batalkan variabel atau lepaskan flag, dan periksa entri kunci di [referensi pengaturan](/docs/id/settings-reference) dan baris variabel di [referensi variabel lingkungan](/docs/id/env-vars) untuk mana yang digunakan Claude Code.

<span id="keys-ignored-in-a-repository-file" />

<span id="keys-only-you-or-your-organization-can-set" />

<span id="common-cases" />

<span id="which-value-applies-in-common-situations" />

<h3 id="troubleshoot-a-setting-that-doesn’t-apply">
  Troubleshoot pengaturan yang tidak berlaku
</h3>

Saat Anda menetapkan kunci dan Claude Code tidak berperilaku seolah-olah Anda memilikinya, mulai dengan `/status` untuk melihat file mana yang dimuat, kemudian temukan gejala Anda di bawah. [Debug konfigurasi Anda](/docs/id/debug-your-config) mencakup pemeriksaan yang lebih luas, termasuk tes konfigurasi bersih.

<h4 id="a-value-you-set-is-ignored">
  Nilai yang Anda atur diabaikan
</h4>

Sesuatu yang lain menetapkan kunci yang sama, file tidak dapat menetapkan nilai itu, atau file tidak dimuat:

* **Level yang lebih tinggi menetapkannya.** File pengaturan lain, flag `--settings`, atau sumber yang dikelola menetapkan kunci di atas milik Anda; [tumpukan](#settings-precedence) mengatakan mana. Flag atau variabel lingkungan juga dapat menimpa kunci atas namanya sendiri, diputuskan kunci demi kunci; entri kunci di [referensi pengaturan](/docs/id/settings-reference) mengatakan mana yang digunakan Claude Code, dan [entri `env`](/docs/id/settings-reference#env) mencakup nilai `env` yang dikelola versus ekspor shell.
* **Kunci keamanan menyimpan nilai ketatnya.** Untuk beberapa kunci Claude Code menghormati nilai yang membatasi dari file apa pun, jadi `true` proyek untuk [`disableClaudeAiConnectors`](/docs/id/settings-reference#disableclaudeaiconnectors) tetap aktif; lihat [Pengecualian untuk urutan prioritas pengaturan yang dikelola](#exceptions-to-managed-settings-precedence).
* **File tidak dapat menetapkan nilai itu.** Nilai [`permissions.defaultMode`](/docs/id/settings-reference#permissions-defaultmode) `auto` dan `bypassPermissions` tidak berlaku dari pengaturan proyek atau lokal; aturnya dalam pengaturan pengguna atau yang dikelola sebagai gantinya, atau lewatkan `--permission-mode` untuk satu sesi. Sebelum v2.1.257, `bypassPermissions` berlaku dari file apa pun.

  Variabel ekspor telemetri dalam blok [`env`](/docs/id/settings-reference#env) juga tidak berlaku dari pengaturan proyek atau lokal, terlepas dari beberapa nilai off. [Variabel yang diabaikan Claude Code dalam `env`](/docs/id/settings-reference#variables-claude-code-ignores-in-env) mencantumkan variabel dan nilai-nilai tersebut.
* **File rusak.** JSON tidak valid atau nilai yang ditolak membuat Claude Code melewati file atau entri; lihat [Perbaiki file pengaturan yang rusak](#fix-a-broken-settings-file).

<h4 id="a-change-you-made-in-claude-code-is-lost-in-new-sessions">
  Perubahan yang Anda buat di Claude Code hilang di sesi baru
</h4>

Saat Anda menyimpan pilihan untuk sesi baru dari dalam Claude Code, seperti model default dengan `/model`, Claude Code menulisnya ke file pengaturan pengguna Anda, `~/.claude/settings.json`. Jika Anda tidak dapat menulis ke file itu, misalnya karena tool lain menghasilkannya atau menghubungkannya ke salinan read-only, perubahan berlaku untuk sesi saat ini dan hilang di sesi berikutnya. Atur kunci dalam tool yang menghasilkan file, atau ganti file dengan yang dapat Anda tulis.

Jika Anda dapat menulis ke file dan perubahan masih tidak bertahan, periksa apakah perubahan itu [hanya untuk satu sesi](#change-a-setting-for-one-session) atau [level yang lebih tinggi menetapkan kunci yang sama](#a-value-you-set-is-ignored). Untuk kunci `model`, [Sesi baru dimulai pada model yang berbeda dari yang Anda pilih](/docs/id/model-config#a-new-session-starts-on-a-different-model-than-you-picked) mencantumkan lebih banyak penyebab.

<h4 id="a-managed-change-hasn’t-reached-you">
  Perubahan yang dikelola belum mencapai Anda
</h4>

Sumber yang dikelola mencapai sesi yang sedang berjalan sesuai jadwal dalam [tabel pengiriman](/docs/id/managed-settings#choose-a-delivery-mechanism), jadi mulai ulang sesi terlebih dahulu. Jika `/status` kemudian menamai sumber yang berbeda dari yang diubah administrator Anda, sumber dengan prioritas lebih tinggi berlaku; [Bagaimana Claude Code menggabungkan sumber yang dikelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources) memberikan urutan.

<h4 id="a-committed-key-doesn’t-reach-teammates">
  Kunci yang dikomit tidak mencapai rekan kerja
</h4>

Dua hal menjaga kunci di `.claude/settings.json` dari penerapan untuk semua orang yang mengkloning:

* **Claude Code mengabaikan kunci dalam file repositori.** Cari `User, local, or managed`, `User or managed`, `Managed`, atau `Global config` di kolom Scope dari [indeks pengaturan](/docs/id/settings-reference#settings-index). Kunci-kunci itu tidak pernah berlaku dari file bersama, terlepas dari beberapa yang file repositori masih dapat matikan. Setiap entri itu mengatakan demikian pada baris Scope-nya. Kunci `Global config` hanya berlaku dari `~/.claude.json`.

  Di dalam kunci `env`, variabel ekspor telemetri tidak pernah berlaku dari file bersama juga, terlepas dari beberapa nilai off; lihat [Variabel yang diabaikan Claude Code dalam `env`](/docs/id/settings-reference#variables-claude-code-ignores-in-env).
* **Kunci menunggu kepercayaan.** Aturan `permissions.allow`, `permissions.additionalDirectories`, `extraKnownMarketplaces`, dan sebagian besar nilai [`env`](/docs/id/settings-reference#env) hanya berlaku setelah setiap rekan kerja [mempercayai folder](/docs/id/permissions#project-allow-rules-and-workspace-trust). Sampai saat itu mereka masih melihat prompt dan tidak mendapatkan plugins dari marketplace yang dideklarasikan file. Aturan `deny` dan `ask` berlaku segera.

<h4 id="permission-rules-combine-differently-than-you-expected">
  Aturan izin digabungkan berbeda dari yang Anda harapkan
</h4>

* **Anda memilih "Ya, dan jangan tanya lagi" pada prompt izin tetapi masih mendapat prompt untuk tool yang sama.** Pilihan itu menyimpan aturan `allow` ke file lokal Anda, dan aturan `allow` di sana tidak mengungguli aturan `ask` dari file proyek atau yang dikelola; [bagaimana aturan izin digabungkan](/docs/id/permissions#settings-precedence) menjelaskan urutan. Di ekstensi VS Code kartu persetujuan memungkinkan Anda memilih file tujuan, termasuk file bersama proyek, yang mengubah aturan untuk semua orang; di CLI, Claude Code hanya menulis ke file lokal Anda.
* **Aturan allow organisasi Anda masih berlaku bersama milik Anda.** Itu diharapkan: Claude Code menggabungkan [`permissions.allow`](/docs/id/settings-reference#permissions-allow) di seluruh cakupan, kecuali organisasi Anda menetapkan [`allowManagedPermissionRulesOnly`](/docs/id/settings-reference#allowmanagedpermissionrulesonly).

<span id="security-keys-where-the-stricter-value-applies" />

<h3 id="exceptions-to-managed-settings-precedence">
  Pengecualian untuk urutan prioritas pengaturan yang dikelola
</h3>

Untuk beberapa kunci yang nilainya membatasi sesi, Claude Code menghormati nilai yang membatasi dari cakupan yang sebaliknya tidak dapat menimpa pengaturan yang dikelola. Temukan kunci dalam tabel ini untuk melihat nilai mana yang dihormatinya dan dari mana.

| Kunci                                                                           | Nilai yang dihormati Claude Code                                                                                                   | Catatan                                                                                                                                                                                             |
| :------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`disableClaudeAiConnectors`](/docs/id/settings-reference#disableclaudeaiconnectors) | `true` dari cakupan apa pun                                                                                                        | Dihormati bahkan saat sumber yang dikelola menetapkan `false`                                                                                                                                       |
| [`enableArtifact`](/docs/id/settings-reference#enableartifact)                       | `false` dari cakupan apa pun, dan `disableArtifact: true` dari cakupan apa pun                                                     | Dihormati bahkan saat sumber yang dikelola menetapkan `true`; tidak ada yang menghidupkan [tool Artifact](/docs/id/artifacts#disable-artifacts) kembali. Memerlukan Claude Code v2.1.242 atau lebih baru |
| [`isolatePeerMachines`](/docs/id/settings-reference#isolatepeermachines)             | `true` dari cakupan apa pun                                                                                                        | Dihormati bahkan saat sumber yang dikelola menetapkan `false`                                                                                                                                       |
| [`remoteControlAtStartup`](/docs/id/settings-reference#remotecontrolatstartup)       | `false` dari `.claude/settings.json` atau `.claude/settings.local.json`                                                            | Dihormati bahkan saat sumber yang dikelola menetapkan `true`; `true` proyek atau lokal diabaikan                                                                                                    |
| [`crossSessionInbound`](/docs/id/settings-reference#crosssessioninbound)             | Nilai yang lebih ketat dari `.claude/settings.json` atau `.claude/settings.local.json`, pada tangga `accept` \< `hold` \< `refuse` | Dihormati di atas nilai yang dikelola, `--settings`, dan pengguna; nilai proyek atau lokal yang tidak lebih ketat diabaikan                                                                         |
| [`useAutoModeDuringPlan`](/docs/id/settings-reference#useautomodeduringplan)         | `false` dari sumber yang dikelola, `--settings`, `~/.claude/settings.json`, atau `.claude/settings.local.json` apa pun             | Dihormati bahkan saat sumber yang dikelola pemenang menetapkan `true`; `false` di `.claude/settings.json` diabaikan                                                                                 |
| [`syncClaudeAiSkills`](/docs/id/settings-reference#syncclaudeaiskills)               | `false` dari sumber yang dikelola, `--settings`, `~/.claude/settings.json`, atau `.claude/settings.local.json` apa pun             | Dihormati bahkan saat sumber yang dikelola pemenang menetapkan `true`; `false` di `.claude/settings.json` diabaikan                                                                                 |
| [`syncClaudeAiPlugins`](/docs/id/settings-reference#syncclaudeaiplugins)             | `false` dari sumber yang dikelola, `--settings`, `~/.claude/settings.json`, atau `.claude/settings.local.json` apa pun             | Dihormati bahkan saat sumber yang dikelola pemenang menetapkan `true`; `false` di `.claude/settings.json` diabaikan                                                                                 |
| [`maxEffortLevel`](/docs/id/settings-reference#maxeffortlevel)                       | Batas yang lebih rendah dari cakupan apa pun, termasuk `--settings`                                                                | Dihormati bahkan saat pengaturan yang dikelola yang diterapkan Claude Code menetapkan batas yang lebih tinggi; batas terendah berlaku. Memerlukan Claude Code v2.1.267 atau lebih baru              |

Aplikasi yang menjalankan Claude Code di dalamnya dan menetapkan [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/id/env-vars) juga merupakan pengecualian. Claude Code mengambil konfigurasi model aplikasi itu di atas kunci `model`, `fallbackModel`, `modelPicker`, dan `modelOverrides` dari setiap sumber yang dikelola, dan di atas variabel pemilihan model dalam blok `env` yang dikelola, seperti `ANTHROPIC_MODEL` dan keluarga `ANTHROPIC_DEFAULT_*_MODEL`. Claude Code menjaga daftar putih [`availableModels`](/docs/id/settings-reference#availablemodels) yang dikelola berlaku kecuali aplikasi memasok miliknya sendiri.

<h2 id="settings-in-cloud-sessions">
  Pengaturan dalam sesi cloud
</h2>

Sesi [cloud](/docs/id/claude-code-on-the-web) berjalan di [lingkungan cloud](/docs/id/cloud-environments) pada klon segar repositori Anda, bukan di mesin Anda. Itu mengubah pengaturan mana yang mencapainya:

* **Pengaturan proyek bersama** (`.claude/settings.json`): dibaca dalam sesi dengan satu repositori, karena file adalah bagian dari klon dan sesi dimulai di dalamnya. Komit pengaturan di sana untuk menerapkannya dalam sesi tersebut. Sesi dengan beberapa repositori dimulai di atas klon dan membaca hanya kunci `enabledPlugins` dan `extraKnownMarketplaces` dari `.claude/settings.json` setiap repositori, bukan aturan izin, hooks, `env`, atau kunci lainnya. Marketplace dan plugin yang dideklarasikan oleh dua kunci tersebut masih [tidak dimuat dalam sesi cloud](/docs/id/cloud-environments#what-carries-over-from-your-setup).
* **Pengaturan pengguna dan proyek lokal** (`~/.claude/settings.json` dan `.claude/settings.local.json`): tidak dibaca. Keduanya tetap di mesin Anda, dan file lokal tidak ada dalam klon.
* **Pengaturan yang dikelola**: hanya [pengaturan yang dikelola server](/docs/id/server-managed-settings) yang mencapai sesi cloud; file `managed-settings.json` atau profil MDM di perangkat Anda tidak. [Lingkungan yang di-host sendiri](/docs/id/self-hosted-environments) juga membaca file pengaturan yang dikelola dalam gambar runner-nya. [Bagaimana Claude Code menggabungkan sumber yang dikelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources) mengatakan kapan file itu berlaku.
* **`/config`**: di browser Anda di claude.ai/code, membuka bagian Claude Code dari pengaturan claude.ai Anda daripada mengubah nilai. Untuk mengubah pengaturan untuk sesi cloud, atur [variabel lingkungan](/docs/id/cloud-environments#set-environment-variables) di lingkungan, atau dalam sesi dengan satu repositori, komit kunci ke `.claude/settings.json` repositori tersebut.

[Apa yang dibawa dari pengaturan Anda](/docs/id/cloud-environments#what-carries-over-from-your-setup) mencantumkan sisanya: `CLAUDE.md`, skills, server MCP, plugins, dan kredensial.

<h2 id="what’s-next">
  Apa selanjutnya
</h2>

* [Semua pengaturan](/docs/id/settings-reference): setiap kunci, dengan tempat Anda menetapkannya dan contohnya
* [File pengaturan contoh](/docs/id/settings-example): file pribadi, file tim, dan file yang dikelola organisasi
* [Konfigurasikan izin](/docs/id/permissions): aturan allow, ask, dan deny, dan apa yang dijalankan Claude Code tanpa bertanya
* [Variabel lingkungan](/docs/id/env-vars): variabel yang dibaca Claude Code dan blok `env`
* [Debug konfigurasi Anda](/docs/id/debug-your-config): saat pengaturan tidak berlaku
* [Referensi direktori Claude](/docs/id/claude-directory): setiap file yang dibaca Claude Code, termasuk subagents, server MCP, plugins, dan `CLAUDE.md`
