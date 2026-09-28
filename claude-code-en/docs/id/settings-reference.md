> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Semua pengaturan

> Referensi lengkap untuk setiap kunci settings.json Claude Code: di mana masing-masing berada, tipe dan defaultnya, serta contoh siap tempel, dengan indeks setiap kunci.

export const BackToIndex = ({href = '#all-settings', label = 'Back to index'}) => {
  const [show, setShow] = useState(false);
  useEffect(() => {
    const onScroll = () => setShow(window.scrollY > window.innerHeight);
    onScroll();
    window.addEventListener('scroll', onScroll, {
      passive: true
    });
    return () => window.removeEventListener('scroll', onScroll);
  }, []);
  return <div className="not-prose">
      <style>{`
        .bti-btn {
          position: fixed; right: 20px; bottom: 20px; z-index: 40;
          display: inline-flex; align-items: center; gap: 6px;
          padding: 8px 12px; border-radius: 999px;
          font-size: 13px; font-weight: 500; line-height: 1; text-decoration: none;
          color: #1f1f1f; background: #ffffff; border: 1px solid #d9d9d9;
          box-shadow: 0 2px 8px rgba(0,0,0,0.12);
          opacity: 0; pointer-events: none; transform: translateY(6px);
          transition: opacity 160ms ease, transform 160ms ease;
        }
        .bti-btn.bti-show { opacity: 1; pointer-events: auto; transform: translateY(0); }
        .bti-btn:hover { border-color: #b3b3b3; }
        .dark .bti-btn { color: #ececec; background: #1e1e1e; border-color: #3a3a3a; box-shadow: 0 2px 8px rgba(0,0,0,0.5); }
        .dark .bti-btn:hover { border-color: #5a5a5a; }
        @media (max-width: 1279px) { .bti-btn { bottom: 112px; } }
        @media print { .bti-btn { display: none; } }
      `}</style>
      <a className={'bti-btn' + (show ? ' bti-show' : '')} href={href} aria-hidden={!show} tabIndex={show ? 0 : -1}>
        <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true"><path d="M8 13V3M3.5 7.5 8 3l4.5 4.5" /></svg>
        {label}
      </a>
    </div>;
};

export const ReferenceFilter = ({placeholder, noun, facets, facetOrder, columnHelp, children}) => {
  const useLive = init => {
    const [v, setV] = useState(init);
    const ref = useRef(init);
    return [v, ref, x => {
      ref.current = x;
      setV(x);
    }];
  };
  const cap = s => s.charAt(0).toUpperCase() + s.slice(1);
  const plural = s => s.endsWith('y') ? s.slice(0, -1) + 'ies' : s + 's';
  const facetNames = facets || ['category', 'topic', 'scope', 'where'];
  const orderOf = {};
  Object.keys(facetOrder || ({})).forEach(k => {
    orderOf[k] = facetOrder[k].map(x => String(x).toLowerCase());
  });
  const rankIn = (col, v) => {
    const list = orderOf[col];
    if (!list) return -1;
    const i = list.indexOf(String(v).toLowerCase());
    return i < 0 ? list.length : i;
  };
  const cmpValues = col => (a, b) => {
    const ra = rankIn(col, a);
    const rb = rankIn(col, b);
    if (ra !== rb) return ra - rb;
    return a < b ? -1 : a > b ? 1 : 0;
  };
  const help = columnHelp || ({});
  const FIRST_COL_HELP = 'Click an entry to open it.';
  const optionLabel = (f, c) => c === 'All' ? 'All ' + plural(f.label.toLowerCase()) : c;
  const nounText = noun || 'entries';
  const placeholderText = placeholder || 'Filter this reference';
  const rootRef = useRef(null);
  const tablesRef = useRef(null);
  const searchRef = useRef(null);
  const menuRef = useRef({});
  const [q, qRef, setQ] = useLive('');
  const [sel, selRef, setSel] = useLive({});
  const [sortBy, sortRef, setSortBy] = useLive(null);
  const [menuOpen, menuOpenRef, setMenu] = useLive(null);
  const [facetList, setFacetList] = useState([]);
  const [firstHead, setFirstHead] = useState('');
  const [counts, setCounts] = useState({
    shown: 0,
    total: 0
  });
  const [disabled, setDisabled] = useState(false);
  const menuBtn = name => menuRef.current[name] ? menuRef.current[name].querySelector(':scope > button') : null;
  const menuList = name => menuRef.current[name] ? menuRef.current[name].querySelector('[role="listbox"]') : null;
  const closeMenu = name => {
    setMenu(null);
    const btn = menuBtn(name);
    if (btn) btn.focus();
  };
  const focusSelected = name => {
    const list = menuList(name);
    if (!list) return;
    const btn = list.querySelector('button[aria-selected="true"]') || list.querySelector('button');
    if (btn) btn.focus();
  };
  const setFacet = (name, value) => {
    setSel(Object.assign({}, selRef.current, {
      [name]: value
    }));
    apply(qRef.current);
    closeMenu(name);
  };
  const sortTables = by => {
    (tablesRef.current || []).forEach(tab => {
      const t = tab.el;
      const idx = tab.heads.indexOf(by);
      const body = t.querySelector('tbody');
      if (idx < 0 || !body) return;
      const rows = [...body.querySelectorAll('tr')];
      const keyOf = r => r.children[idx] ? r.children[idx].textContent.trim().toLowerCase() : '';
      const cmp = cmpValues(by);
      rows.map((r, i) => ({
        r,
        i: Number(r.dataset.sfIndex !== undefined ? r.dataset.sfIndex : i),
        k: keyOf(r)
      })).sort((a, b) => cmp(a.k, b.k) || a.i - b.i).forEach(x => body.appendChild(x.r));
      [...t.querySelectorAll('thead th')].forEach((h, i) => {
        const sortable = tab.heads[i] === tab.heads[0] || facetNames.indexOf(tab.heads[i]) > -1;
        if (sortable) h.setAttribute('aria-sort', i === idx ? 'ascending' : 'none'); else h.removeAttribute('aria-sort');
      });
    });
  };
  const scan = () => {
    const tables = [];
    let el = rootRef.current ? rootRef.current.nextElementSibling : null;
    while (el) {
      if (el.tagName === 'H2' || el.querySelector(':scope > h2')) break;
      const found = el.tagName === 'TABLE' ? [el] : [...el.querySelectorAll('table')];
      found.forEach(t => {
        const headCells = [...t.querySelectorAll('thead th, thead td')];
        const heads = headCells.map(h => h.textContent.trim().toLowerCase());
        if (heads.length === 0) return;
        const facetIdx = {};
        heads.forEach((h, i) => {
          if (facetNames.indexOf(h) > -1) facetIdx[h] = i;
        });
        if (!t.dataset.sfDecorated) {
          t.dataset.sfDecorated = '1';
          headCells.forEach((h, i) => {
            const text = i === 0 ? help[heads[0]] || FIRST_COL_HELP : help[heads[i]];
            if (text) h.title = text;
          });
        }
        const rows = [...t.querySelectorAll('tbody tr')].map((r, i) => {
          if (r.dataset.sfIndex === undefined) r.dataset.sfIndex = String(i);
          const cells = r.querySelectorAll('td');
          const fv = {};
          Object.keys(facetIdx).forEach(h => {
            fv[h] = cells[facetIdx[h]] ? cells[facetIdx[h]].textContent.trim() : '';
          });
          return {
            el: r,
            text: [...cells].map(c => c.textContent).join(' ').toLowerCase(),
            facets: fv,
            anchors: [...r.querySelectorAll('a[href^="#"]')].map(a => a.getAttribute('href').slice(1)),
            ids: [...r.querySelectorAll('[id]')].map(n => n.id)
          };
        });
        tables.push({
          el: t,
          box: t.closest('[data-table-wrapper]') || t,
          rows,
          heads
        });
      });
      el = el.nextElementSibling;
    }
    tablesRef.current = tables;
    if (sortRef.current) sortTables(sortRef.current);
    return tables;
  };
  const apply = query => {
    let tables = tablesRef.current || scan();
    if (tables.some(t => !t.el.isConnected)) tables = scan();
    const needle = query.trim().toLowerCase();
    const sel = selRef.current;
    const activeFacets = Object.keys(sel).filter(h => sel[h] && sel[h] !== 'All');
    const show = (el, on) => {
      const want = on ? '' : 'none';
      if (el.style.display !== want) el.style.display = want;
    };
    let total = 0;
    let shown = 0;
    const visibleTargets = new Set();
    tables.forEach(t => {
      let tableVisible = 0;
      t.rows.forEach(row => {
        total += 1;
        const catOk = activeFacets.every(h => {
          const v = row.facets[h];
          return v === sel[h] || v === '' || v === undefined;
        });
        const match = catOk && (needle === '' || row.text.includes(needle));
        show(row.el, match);
        if (match) {
          tableVisible += 1;
          row.anchors.forEach(a => visibleTargets.add(a));
        }
      });
      show(t.box, !(t.rows.length > 0 && tableVisible === 0));
      shown += tableVisible;
    });
    if (shown < total && visibleTargets.size > 0) {
      tables.forEach(t => {
        t.rows.forEach(row => {
          if (row.el.style.display === 'none' && row.ids.some(id => visibleTargets.has(id))) {
            show(row.el, true);
            show(t.box, true);
            shown += 1;
          }
        });
      });
    }
    setCounts({
      shown,
      total
    });
    return total;
  };
  const deriveFacets = tables => {
    const seen = {};
    tables.forEach(t => t.rows.forEach(r => {
      Object.keys(r.facets).forEach(h => {
        if (!seen[h]) seen[h] = [];
        if (r.facets[h] && seen[h].indexOf(r.facets[h]) === -1) seen[h].push(r.facets[h]);
      });
    }));
    const list = facetNames.filter(h => seen[h] && seen[h].length > 0).map(h => ({
      name: h,
      label: cap(h),
      values: seen[h].sort(cmpValues(h))
    }));
    setFacetList(list);
    const first = tables[0] ? tables[0].heads[0] : '';
    setFirstHead(first);
    if (!sortRef.current && first) {
      setSortBy(first);
      sortTables(first);
    }
    const init = {};
    list.forEach(f => {
      init[f.name] = selRef.current[f.name] || 'All';
    });
    setSel(init);
  };
  const onChange = value => {
    setQ(value);
    apply(value);
  };
  const clearAll = () => {
    const next = {};
    Object.keys(selRef.current).forEach(k => {
      next[k] = 'All';
    });
    setSel(next);
    setQ('');
    apply('');
    if (searchRef.current) searchRef.current.focus();
  };
  useEffect(() => {
    const tables = scan();
    deriveFacets(tables);
    const total = apply('');
    let retryTimer;
    if (total === 0) {
      retryTimer = setTimeout(() => {
        tablesRef.current = null;
        if (apply(qRef.current) > 0) deriveFacets(tablesRef.current); else setDisabled(true);
      }, 500);
    }
    const onKey = e => {
      if (e.key === 'Escape' && menuOpenRef.current !== null) closeMenu(menuOpenRef.current);
      if (!searchRef.current) return;
      if (e.metaKey || e.ctrlKey || e.altKey) return;
      const active = document.activeElement;
      const tag = active && active.tagName;
      const editable = active && active.isContentEditable;
      const interactive = tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT' || tag === 'BUTTON' || tag === 'A' || editable || active && active.getAttribute && active.getAttribute('role');
      if (e.key === '/' && !interactive) {
        const r = rootRef.current ? rootRef.current.getBoundingClientRect() : null;
        if (r && r.bottom > 0 && r.top < (window.innerHeight || 0)) {
          e.preventDefault();
          setMenu(null);
          searchRef.current.focus();
        }
      }
      if (e.key === 'Escape' && menuOpenRef.current === null && active === searchRef.current) {
        onChange('');
        searchRef.current.blur();
      }
    };
    const onDocClick = e => {
      const open = menuOpenRef.current;
      if (open !== null && menuRef.current[open] && !menuRef.current[open].contains(e.target)) setMenu(null);
    };
    window.addEventListener('keydown', onKey);
    document.addEventListener('mousedown', onDocClick);
    return () => {
      if (retryTimer) clearTimeout(retryTimer);
      window.removeEventListener('keydown', onKey);
      document.removeEventListener('mousedown', onDocClick);
      (tablesRef.current || []).forEach(t => {
        t.box.style.display = '';
        t.rows.forEach(row => {
          row.el.style.display = '';
        });
      });
    };
  }, []);
  useEffect(() => {
    if (menuOpen !== null) focusSelected(menuOpen);
  }, [menuOpen]);
  if (disabled) return null;
  const facetActive = Object.keys(sel).some(h => sel[h] && sel[h] !== 'All');
  const sortOptions = [firstHead].concat(facetList.map(f => f.name)).filter((h, i, a) => h && a.indexOf(h) === i);
  return <>
      <style>{`
        .sf-root {
          --sf-accent: #D97757;
          --sf-bg: #fff;
          --sf-border: #E8E6DC;
          --sf-text: #141413;
          --sf-text-3: #73726C;
          --sf-text-4: #9C9A92;
        }
        .dark .sf-root {
          --sf-bg: #1a1918;
          --sf-border: #3a3936;
          --sf-text: #e8e6dc;
          --sf-text-3: #9c9a92;
          --sf-text-4: #73726c;
        }
        .sf-root .sf-end {
          position: absolute;
          right: 10px;
          top: 50%;
          transform: translateY(-50%);
        }
        .sf-root .sf-x {
          background: none;
          border: none;
          cursor: pointer;
          color: var(--sf-text-3);
          font-size: 14px;
          padding: 2px 4px;
          line-height: 1;
        }
      `}</style>
      <div ref={rootRef} className="sf-root" style={{
    margin: '16px 0 8px'
  }}>
        <div style={{
    display: 'flex',
    gap: '8px',
    flexWrap: 'wrap',
    alignItems: 'center'
  }}>
        <div style={{
    position: 'relative',
    flex: '1 1 260px',
    maxWidth: '480px'
  }}>
          <input ref={searchRef} value={q} onChange={e => onChange(e.target.value)} placeholder={placeholderText} aria-label={placeholderText} style={{
    width: '100%',
    padding: '8px 56px 8px 12px',
    borderRadius: '8px',
    border: '1px solid var(--sf-border)',
    background: 'var(--sf-bg)',
    color: 'var(--sf-text)',
    fontSize: '14px',
    outline: 'none',
    boxSizing: 'border-box'
  }} />
          {q ? <button type="button" onClick={() => {
    onChange('');
    if (searchRef.current) searchRef.current.focus();
  }} aria-label="Clear text" className="sf-end sf-x">
              ×
            </button> : <span className="sf-end" style={{
    fontFamily: 'var(--font-mono, ui-monospace, monospace)',
    fontSize: '11px',
    color: 'var(--sf-text-4)',
    border: '1px solid var(--sf-border)',
    borderRadius: '3px',
    padding: '0 5px',
    pointerEvents: 'none'
  }}>
              /
            </span>}
        </div>
        {facetList.map(f => {
    const cur = sel[f.name] || 'All';
    const isOpen = menuOpen === f.name;
    return <div key={f.name} ref={el => {
      menuRef.current[f.name] = el;
    }} style={{
      position: 'relative'
    }}>
            <button type="button" onClick={() => setMenu(isOpen ? null : f.name)} onKeyDown={e => {
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        if (!isOpen) setMenu(f.name); else focusSelected(f.name);
      }
    }} aria-haspopup="listbox" aria-expanded={isOpen} style={{
      display: 'flex',
      alignItems: 'center',
      gap: '8px',
      padding: cur !== 'All' ? '8px 30px 8px 12px' : '8px 12px',
      borderRadius: '8px',
      border: '1px solid ' + (cur !== 'All' ? 'var(--sf-accent)' : 'var(--sf-border)'),
      background: 'var(--sf-bg)',
      color: cur === 'All' ? 'var(--sf-text-3)' : 'var(--sf-text)',
      fontSize: '13.5px',
      cursor: 'pointer',
      whiteSpace: 'nowrap',
      maxWidth: '260px'
    }}>
              <span style={{
      overflow: 'hidden',
      textOverflow: 'ellipsis'
    }}>
                {f.label + ': ' + optionLabel(f, cur)}
              </span>
              <span aria-hidden="true" style={{
      fontSize: '9px',
      color: 'var(--sf-text-4)',
      transform: isOpen ? 'rotate(180deg)' : 'none',
      transition: 'transform 120ms'
    }}>
                ▼
              </span>
            </button>
            {cur !== 'All' && <button type="button" onClick={() => setFacet(f.name, 'All')} aria-label={'Clear ' + f.label + ' filter'} title={'Clear ' + f.label + ' filter'} className="sf-end sf-x">
                ×
              </button>}
            {isOpen && <div role="listbox" aria-label={f.label} onKeyDown={e => {
      const items = [...e.currentTarget.querySelectorAll('button')];
      const idx = items.indexOf(document.activeElement);
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        (items[idx + 1] || items[0]).focus();
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        (items[idx - 1] || items[items.length - 1]).focus();
      } else if (e.key === 'Home') {
        e.preventDefault();
        if (items[0]) items[0].focus();
      } else if (e.key === 'End') {
        e.preventDefault();
        if (items[items.length - 1]) items[items.length - 1].focus();
      } else if (e.key === 'Tab') {
        closeMenu(f.name);
      }
    }} style={{
      position: 'absolute',
      top: 'calc(100% + 6px)',
      left: 0,
      zIndex: 1000,
      minWidth: '260px',
      maxHeight: '340px',
      overflowY: 'auto',
      background: 'var(--sf-bg)',
      border: '1px solid var(--sf-border)',
      borderRadius: '10px',
      boxShadow: '0 8px 24px rgba(0,0,0,0.12)',
      padding: '5px'
    }}>
                {['All'].concat(f.values).map(c => {
      const selected = cur === c;
      return <button key={c} role="option" aria-selected={selected} tabIndex={-1} onClick={() => setFacet(f.name, c)} style={{
        display: 'flex',
        alignItems: 'center',
        gap: '8px',
        width: '100%',
        textAlign: 'left',
        padding: '7px 10px',
        borderRadius: '6px',
        border: 'none',
        background: 'transparent',
        color: selected ? 'var(--sf-accent)' : 'var(--sf-text)',
        fontWeight: selected ? 600 : 400,
        fontSize: '13.5px',
        cursor: 'pointer'
      }}>
                      <span aria-hidden="true" style={{
        width: '14px',
        color: 'var(--sf-accent)',
        flexShrink: 0
      }}>
                        {selected ? '✓' : ''}
                      </span>
                      {optionLabel(f, c)}
                    </button>;
    })}
              </div>}
          </div>;
  })}
        {sortOptions.length > 1 && <div role="group" aria-label="Sort by" style={{
    display: 'flex',
    alignItems: 'center',
    gap: '4px',
    fontSize: '13px',
    color: 'var(--sf-text-3)',
    whiteSpace: 'nowrap'
  }}>
            <span style={{
    marginRight: '4px'
  }}>Sort by</span>
            {sortOptions.map(o => {
    const on = sortBy === o;
    return <button key={o} type="button" aria-pressed={on} onClick={() => {
      setSortBy(o);
      sortTables(o);
    }} style={{
      padding: '6px 10px',
      borderRadius: '8px',
      border: '1px solid ' + (on ? 'var(--sf-accent)' : 'var(--sf-border)'),
      background: 'var(--sf-bg)',
      color: on ? 'var(--sf-text)' : 'var(--sf-text-3)',
      fontSize: '13px',
      cursor: 'pointer'
    }}>
                  {o.charAt(0).toUpperCase() + o.slice(1)}
                </button>;
  })}
          </div>}
        </div>
        <div aria-live="polite" style={{
    margin: '8px 0 0',
    fontSize: '13px',
    color: 'var(--sf-text-3)',
    minHeight: '1px'
  }}>
          {q.trim() === '' && !facetActive ? <>{counts.total} {nounText}</> : counts.shown === 0 ? <>
                {q.trim() === '' ? 'No ' + nounText + ' match the selected filters.' : facetActive ? 'No ' + nounText + ' match \u201c' + q + '\u201d with the selected filters.' : 'No ' + nounText + ' match \u201c' + q + '\u201d.'}{' '}
                <button type="button" onClick={clearAll} style={{
    background: 'none',
    border: 'none',
    padding: 0,
    color: 'var(--sf-accent)',
    cursor: 'pointer',
    font: 'inherit',
    textDecoration: 'underline'
  }}>
                  Clear filters
                </button>
                {children ? <> {children}</> : null}
              </> : <>
                Showing {counts.shown} of {counts.total} {nounText}
              </>}
        </div>
      </div>
    </>;
};

<BackToIndex href="#all-settings" label="Kembali ke indeks" />

Halaman referensi ini mencantumkan setiap kunci yang dibaca Claude Code dari file pengaturan, ditambah [kelompok kunci pendek](#global-config-settings) yang disimpannya di `~/.claude.json` sebagai gantinya. Untuk memilih file, atau memeriksa prioritas, mulai dengan [File pengaturan dan prioritas](/docs/id/settings).

<span id="available-settings" />

<span id="scopes" />

<span id="all-settings" />

<h2 id="settings-index">
  Indeks pengaturan
</h2>

Setiap kunci di bawah menautkan ke entrinya. Scope mencantumkan [file](/docs/id/settings#settings-files-and-who-they-affect) tempat kunci dapat digunakan: `User` adalah `~/.claude/settings.json`, `Project` adalah `.claude/settings.json`, `Local` adalah `.claude/settings.local.json`, dan `Managed` adalah [apa yang organisasi Anda terapkan](/docs/id/managed-settings). `Any file` berarti keempat file tersebut, dan `Global config` berarti [`~/.claude.json`](#global-config-settings).

<ReferenceFilter
  noun="settings"
  placeholder="Filter settings by key or purpose"
  facetOrder={{ scope: ["Any file", "User, local, or managed", "User or managed", "Managed", "Global config"] }}
  columnHelp={{
topic: "The section of this page that holds the entry. Use Sort by to group the table by topic.",
scope: "Which settings files can set the key: user (~/.claude/settings.json), project (.claude/settings.json), local (.claude/settings.local.json), or managed (deployed by your organization). Global config keys are in ~/.claude.json instead.",
}}
/>

| Kunci                                                                                                 | Deskripsi                                                                                                                                                                                                                                                     | Topik                              | Scope                   |
| :---------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------- | :---------------------- |
| [`advisorModel`](#advisormodel)                                                                       | Pilih model mana yang menjawab ketika Claude menggunakan [alat advisor](/docs/id/advisor)                                                                                                                                                                          | Model dan respons                  | Any file                |
| [`agent`](#agent)                                                                                     | Mulai setiap sesi sebagai [subagent](/docs/id/sub-agents) bernama dengan prompt, alat, dan modelnya                                                                                                                                                                | Agents, sessions, and worktrees    | Any file                |
| [`agentPushNotifEnabled`](#agentpushnotifenabled)                                                     | Biarkan Claude mengirim [notifikasi push ke ponsel Anda](/docs/id/remote-control#mobile-push-notifications) ketika Claude memutuskan untuk melakukannya                                                                                                            | Remote, desktop, and notifications | Any file                |
| [`allowAllClaudeAiMcps`](#allowallclaudeaimcps)                                                       | Muat [konektor claude.ai](/docs/id/mcp) yang Claude Code ambil sendiri bersama dengan [`managed-mcp.json`](/docs/id/managed-mcp#exclusive-control-with-managed-mcp-json) yang diterapkan                                                                                | MCP                                | Managed                 |
| [`allowedChannelPlugins`](#allowedchannelplugins)                                                     | Ganti daftar izin default [plugin channel](/docs/id/channels#restrict-which-channel-plugins-can-run) yang dapat mendorong pesan                                                                                                                                    | Plugins and skills                 | Managed                 |
| [`allowedHttpHookUrls`](#allowedhttphookurls)                                                         | Batasi URL mana yang dapat ditargetkan oleh [HTTP hooks](/docs/id/hooks)                                                                                                                                                                                           | Hooks and automation               | Any file                |
| [`allowedMcpServers`](#allowedmcpservers)                                                             | Daftar izin server [MCP](/docs/id/mcp) mana yang dapat ditambahkan pengguna                                                                                                                                                                                        | MCP                                | Any file                |
| [`allowManagedHooksOnly`](#allowmanagedhooksonly)                                                     | Jalankan hanya [hooks](/docs/id/hooks) yang organisasi Anda terapkan                                                                                                                                                                                               | Hooks and automation               | Managed                 |
| [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly)                                           | Buat daftar izin [MCP](/docs/id/mcp) yang dikelola menjadi satu-satunya yang berlaku                                                                                                                                                                               | MCP                                | Managed                 |
| [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly)                                 | Buat [pengaturan yang dikelola](/docs/id/managed-settings) menjadi satu-satunya sumber pengaturan [aturan izin](/docs/id/permissions#managed-settings)                                                                                                                  | Permission settings                | Managed                 |
| [`alwaysThinkingEnabled`](#alwaysthinkingenabled)                                                     | Matikan [pemikiran yang diperluas](/docs/id/model-config#extended-thinking) untuk setiap sesi                                                                                                                                                                      | Model dan respons                  | Any file                |
| [`apiKeyHelper`](#apikeyhelper)                                                                       | Hasilkan [kredensial API](/docs/id/authentication#credential-management) dengan perintah Anda sendiri                                                                                                                                                              | Authentication and providers       | Any file                |
| [`askUserQuestionTimeout`](#askuserquestiontimeout)                                                   | Biarkan pertanyaan yang tidak terjawab [lanjut otomatis](/docs/id/tools-reference#question-auto-continue-timeout) setelah waktu idle                                                                                                                               | Interface and terminal             | User or managed         |
| [`attribution`](#attribution)                                                                         | Sesuaikan atribusi yang Claude Code tambahkan ke commit dan pull request                                                                                                                                                                                      | Git and attribution                | Any file                |
| [`attribution.commit`](#attribution-commit)                                                           | Ubah atau sembunyikan trailer yang Claude Code tambahkan ke commit                                                                                                                                                                                            | Git and attribution                | Any file                |
| [`attribution.pr`](#attribution-pr)                                                                   | Ubah atau sembunyikan baris atribusi dalam deskripsi pull request                                                                                                                                                                                             | Git and attribution                | Any file                |
| [`attribution.sessionUrl`](#attribution-sessionurl)                                                   | Hilangkan tautan sesi claude.ai dari commit [cloud](/docs/id/claude-code-on-the-web) dan [Remote Control](/docs/id/remote-control)                                                                                                                                      | Git and attribution                | Any file                |
| [`autoCompactEnabled`](#autocompactenabled)                                                           | Matikan atau nyalakan [pemadatan otomatis](/docs/id/context-window)                                                                                                                                                                                                | Memory and context                 | Any file                |
| [`autoCompactWindow`](#autocompactwindow)                                                             | Atur seberapa penuh konteks sebelum Claude Code [memadatkan](/docs/id/context-window)                                                                                                                                                                              | Memory and context                 | Any file                |
| [`autoConnectIde`](#autoconnectide)                                                                   | Terhubung ke [VS Code](/docs/id/vs-code) atau [JetBrains](/docs/id/jetbrains#from-external-terminals) IDE yang sedang berjalan secara otomatis dari terminal eksternal                                                                                                  | Global config settings             | Global config           |
| [`autoContinueAtUsageLimit`](#autocontinueatusagelimit)                                               | Tunggu di sesi terbuka dan [lanjutkan tugas secara otomatis](/docs/id/interactive-mode#wait-for-a-usage-limit-to-reset) setelah batas penggunaan claude.ai direset                                                                                                 | Interface and terminal             | User or managed         |
| [`autoInstallIdeExtension`](#autoinstallideextension)                                                 | Matikan instalasi otomatis [ekstensi IDE](/docs/id/vs-code#install-the-extension) dari terminal VS Code                                                                                                                                                            | Global config settings             | Global config           |
| [`autoMemoryDirectory`](#automemorydirectory)                                                         | Simpan [memori otomatis](/docs/id/memory#auto-memory) di direktori pilihan Anda                                                                                                                                                                                    | Memory and context                 | Any file                |
| [`autoMemoryEnabled`](#automemoryenabled)                                                             | Matikan atau nyalakan [memori otomatis](/docs/id/memory#auto-memory)                                                                                                                                                                                               | Memory and context                 | Any file                |
| [`autoMode`](#automode)                                                                               | Tambahkan aturan izin dan tolak Anda sendiri ke pengklasifikasi [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode)                                                                                                                        | Permission settings                | User or managed         |
| [`autoMode.classifyAllShell`](#automode-classifyallshell)                                             | Kirim setiap perintah shell melalui pengklasifikasi [mode otomatis](/docs/id/permission-modes#what-the-classifier-blocks-by-default), bahkan yang cocok dengan aturan izin sempit                                                                                  | Permission settings                | User or managed         |
| [`autoScrollEnabled`](#autoscrollenabled)                                                             | [Ikuti output baru](/docs/id/fullscreen#auto-follow) ke bawah dalam rendering fullscreen                                                                                                                                                                           | Interface and terminal             | Any file                |
| [`autoUpdatesChannel`](#autoupdateschannel)                                                           | Ikuti [saluran rilis](/docs/id/setup#configure-release-channel) stabil daripada terbaru                                                                                                                                                                            | Updates and versioning             | Any file                |
| [`availableModels`](#availablemodels)                                                                 | [Batasi model mana](/docs/id/model-config#restrict-model-selection) yang dapat dipilih orang                                                                                                                                                                       | Model dan respons                  | Any file                |
| [`awaySummaryEnabled`](#awaysummaryenabled)                                                           | Matikan [ringkasan sesi](/docs/id/interactive-mode#session-recap) yang ditampilkan ketika Anda kembali ke terminal                                                                                                                                                 | Remote, desktop, and notifications | Any file                |
| [`awsAuthRefresh`](#awsauthrefresh)                                                                   | Segarkan [kredensial Bedrock](/docs/id/amazon-bedrock#advanced-credential-configuration) yang kedaluwarsa di `.aws` dengan perintah Anda sendiri                                                                                                                   | Authentication and providers       | Any file                |
| [`awsCredentialExport`](#awscredentialexport)                                                         | Sediakan [kredensial Bedrock](/docs/id/amazon-bedrock#advanced-credential-configuration) sebagai JSON dari perintah Anda sendiri                                                                                                                                   | Authentication and providers       | Any file                |
| [`axScreenReader`](#axscreenreader)                                                                   | Render [output yang ramah pembaca layar](/docs/id/accessibility)                                                                                                                                                                                                   | Interface and terminal             | Any file                |
| [`bashEditDiffEnabled`](#basheditdiffenabled)                                                         | Catat [file yang diubah perintah Bash](/docs/id/hooks#bash) dalam setiap mode izin                                                                                                                                                                                 | Interface and terminal             | User or managed         |
| [`bashOutputMaxChars`](#bashoutputmaxchars)                                                           | Atur berapa banyak [output](/docs/id/tools-reference#output-limits) perintah yang berhasil yang Claude terima secara inline                                                                                                                                        | Memory and context                 | Any file                |
| [`blockedMarketplaces`](#blockedmarketplaces)                                                         | Blokir sumber [marketplace plugin](/docs/id/plugins/overview) untuk organisasi Anda                                                                                                                                                                                | Plugins and skills                 | Managed                 |
| [`browserExternalPageTools`](#browserexternalpagetools)                                               | Tetap tidak aktifkan alat Claude di halaman eksternal dalam pane Browser [desktop](/docs/id/desktop)                                                                                                                                                               | Tools                              | Managed                 |
| [`channelsEnabled`](#channelsenabled)                                                                 | Izinkan [channel](/docs/id/channels#enable-channels-for-your-organization) untuk organisasi Anda                                                                                                                                                                   | Plugins and skills                 | Managed                 |
| [`claudeMd`](#claudemd)                                                                               | Injeksikan instruksi [CLAUDE.md](/docs/id/memory#deploy-organization-wide-claude-md) di seluruh organisasi dari pengaturan yang dikelola                                                                                                                           | Memory and context                 | Managed                 |
| [`claudeMdExcludes`](#claudemdexcludes)                                                               | Lewati file [CLAUDE.md](/docs/id/memory#exclude-specific-claude-md-files) tertentu ketika memori dimuat                                                                                                                                                            | Memory and context                 | Any file                |
| [`cleanupPeriodDays`](#cleanupperioddays)                                                             | Pilih berapa hari Claude Code menyimpan [transkrip](/docs/id/data-usage#data-retention) sebelum menghapusnya                                                                                                                                                       | Privacy and telemetry              | Any file                |
| [`companyAnnouncements`](#companyannouncements)                                                       | Tampilkan pengumuman organisasi Anda saat startup                                                                                                                                                                                                             | Interface and terminal             | Any file                |
| [`copyOnSelect`](#copyonselect)                                                                       | Matikan penyalinan otomatis teks yang Anda pilih dengan mouse dalam [rendering fullscreen](/docs/id/fullscreen#use-the-mouse) dan tampilan agent                                                                                                                   | Global config settings             | Global config           |
| [`crossSessionInbound`](#crosssessioninbound)                                                         | Pilih apakah Claude Code mengirimkan [pesan dari sesi lain Anda](/docs/id/cross-session-messaging#control-inbound-messages), menampilkan pemberitahuan tanpa mengirimkannya, atau menolaknya                                                                       | Agents, sessions, and worktrees    | Any file                |
| [`defaultShell`](#defaultshell)                                                                       | Pilih apakah Bash atau PowerShell menjalankan perintah shell yang Anda ketik dengan awalan [`!`](/docs/id/interactive-mode#shell-mode-with-prefix)                                                                                                                 | Interface and terminal             | Any file                |
| [`deniedMcpServers`](#deniedmcpservers)                                                               | Blokir server [MCP](/docs/id/mcp) tertentu berdasarkan URL, perintah, atau nama                                                                                                                                                                                    | MCP                                | Any file                |
| [`desktopSessionCleanupPeriodDays`](#desktopsessioncleanupperioddays)                                 | Atur batas usia dalam hari untuk [transkrip Claude Desktop dan Cowork](/docs/id/claude-directory#cleaned-up-automatically)                                                                                                                                         | Privacy and telemetry              | User or managed         |
| [`dialogExpiry`](#dialogexpiry)                                                                       | Atur berapa lama Claude Code menunggu [Remote Control](/docs/id/remote-control) atau host SDK menjawab dialog yang diteruskan sebelum membatalkan dialog                                                                                                           | Interface and terminal             | User or managed         |
| [`diffTool`](#difftool)                                                                               | Pilih apakah perubahan file yang diusulkan Claude terbuka di penampil diff [VS Code](/docs/id/vs-code) atau [JetBrains](/docs/id/jetbrains#features) atau tetap di terminal                                                                                             | Global config settings             | Global config           |
| [`disableAgentView`](#disableagentview)                                                               | Matikan agen latar belakang dan [tampilan agent](/docs/id/agent-view)                                                                                                                                                                                              | Agents, sessions, and worktrees    | Any file                |
| [`disableAllHooks`](#disableallhooks)                                                                 | Matikan [hooks](/docs/id/hooks), [baris status](/docs/id/statusline) kustom, dan perintah saran file [`@`](/docs/id/interactive-mode#quick-commands) kustom sekaligus                                                                                                        | Hooks and automation               | Any file                |
| [`disableArtifact`](#disableartifact)                                                                 | Usang; gunakan `enableArtifact` untuk mematikan [alat Artifact](/docs/id/artifacts)                                                                                                                                                                                | Remote, desktop, and notifications | Any file                |
| [`disableAutoMode`](#disableautomode)                                                                 | Hapus [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) dari siklus mode izin                                                                                                                                                            | Permission settings                | Any file                |
| [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation)                               | Batasi pane Browser [desktop](/docs/id/desktop) ke localhost untuk orang dan Claude                                                                                                                                                                                | Tools                              | Managed                 |
| [`disableBundledSkills`](#disablebundledskills)                                                       | Matikan [skill](/docs/id/skills#bundled-skills) dan [workflow](/docs/id/workflows) yang disertakan dengan Claude Code                                                                                                                                                   | Plugins and skills                 | Any file                |
| [`disableClaudeAiConnectors`](#disableclaudeaiconnectors)                                             | Matikan [konektor claude.ai](/docs/id/mcp#disable-claude-ai-connectors) sehingga Claude Code tidak mengambilnya                                                                                                                                                    | MCP                                | Any file                |
| [`disableCommandPluginSources`](#disablecommandpluginsources)                                         | Blokir [plugin](/docs/id/plugins/overview) yang dipasang dengan menjalankan perintah yang dideklarasikan marketplace                                                                                                                                               | Plugins and skills                 | Managed                 |
| [`disableDeepLinkRegistration`](#disabledeeplinkregistration)                                         | Hentikan Claude Code dari mendaftarkan penanganan [`claude-cli://`](/docs/id/deep-links)                                                                                                                                                                           | Remote, desktop, and notifications | Any file                |
| [`disableDesktopLocalSessions`](#disabledesktoplocalsessions)                                         | Matikan sesi [Desktop Code](/docs/id/desktop#local-sessions-on-managed-devices) yang berjalan di perangkat, meninggalkan SSH untuk host lain dan cloud                                                                                                             | Remote, desktop, and notifications | Managed                 |
| [`disabledMcpjsonServers`](#disabledmcpjsonservers)                                                   | Tolak server tertentu dari [`.mcp.json`](/docs/id/mcp#project-scope) proyek                                                                                                                                                                                        | MCP                                | Any file                |
| [`disableMobileSimulatorTools`](#disablemobilesimulatortools)                                         | Blokir alat Claude dalam pane [desktop](/docs/id/desktop) iOS Simulator                                                                                                                                                                                            | Tools                              | Managed                 |
| [`disableRemoteControl`](#disableremotecontrol)                                                       | Matikan [Remote Control](/docs/id/remote-control) di mana pun dapat dimulai                                                                                                                                                                                        | Remote, desktop, and notifications | Any file                |
| [`disableSideloadFlags`](#disablesideloadflags)                                                       | Tolak flag CLI yang sideload [plugin](/docs/id/plugins/overview), [subagent](/docs/id/sub-agents), dan [server MCP](/docs/id/mcp)                                                                                                                                            | Enterprise and managed settings    | Managed                 |
| [`disableSkillShellExecution`](#disableskillshellexecution)                                           | Hentikan [skill](/docs/id/skills) dan perintah kustom dari menjalankan shell inline                                                                                                                                                                                | Plugins and skills                 | Any file                |
| [`disableWorkflows`](#disableworkflows)                                                               | Matikan [workflow dinamis](/docs/id/workflows) untuk semua orang; gunakan `enableWorkflows` untuk diri sendiri                                                                                                                                                     | Hooks and automation               | Any file                |
| [`editorMode`](#editormode)                                                                           | Gunakan [pintasan keyboard vim](/docs/id/interactive-mode#vim-editor-mode) dalam prompt input                                                                                                                                                                      | Interface and terminal             | Any file                |
| [`effortLevel`](#effortlevel)                                                                         | Atur [tingkat usaha](/docs/id/model-config#adjust-effort-level) default untuk model tanpa tingkat yang disimpan                                                                                                                                                    | Model dan respons                  | Any file                |
| [`emojiCompletionEnabled`](#emojicompletionenabled)                                                   | Matikan saran dan penggantian emoji [`:shortcode:`](/docs/id/interactive-mode#emoji-shortcodes) dalam input prompt                                                                                                                                                 | Interface and terminal             | Any file                |
| [`enableAllProjectMcpServers`](#enableallprojectmcpservers)                                           | Setujui setiap server dalam file [`.mcp.json`](/docs/id/mcp#project-server-approvals-and-workspace-trust) proyek tanpa prompt                                                                                                                                      | MCP                                | Any file                |
| [`enableArtifact`](#enableartifact)                                                                   | Matikan [alat Artifact](/docs/id/artifacts) dengan `false` di file mana pun; tidak ada file yang dapat menghidupkannya kembali                                                                                                                                     | Remote, desktop, and notifications | Any file                |
| [`enabledMcpjsonServers`](#enabledmcpjsonservers)                                                     | Setujui server tertentu dari [`.mcp.json`](/docs/id/mcp#project-server-approvals-and-workspace-trust) proyek                                                                                                                                                       | MCP                                | Any file                |
| [`enabledPlugins`](#enabledplugins)                                                                   | Nyalakan atau matikan [plugin](/docs/id/plugins/overview) individual per scope                                                                                                                                                                                     | Plugins and skills                 | Any file                |
| [`enableWorkflows`](#enableworkflows)                                                                 | Nyalakan atau matikan [workflow dinamis](/docs/id/workflows) terhadap default rencana Anda                                                                                                                                                                         | Hooks and automation               | Any file                |
| [`enforceAvailableModels`](#enforceavailablemodels)                                                   | Jaga pilihan Default [`/model`](/docs/id/model-config#enforce-the-allowlist-for-the-default-model) tetap dalam daftar izin `availableModels` Anda                                                                                                                  | Model dan respons                  | Any file                |
| [`env`](#env)                                                                                         | Atur [variabel lingkungan](/docs/id/env-vars#in-settings-files) untuk setiap sesi dan subproses-nya                                                                                                                                                                | Memory and context                 | Any file                |
| [`externalEditorContext`](#externaleditorcontext)                                                     | Tampilkan respons terakhir Claude sebagai komentar ketika Anda menekan [Ctrl+G](/docs/id/interactive-mode#general-controls) untuk mengedit                                                                                                                         | Global config settings             | Global config           |
| [`extraKnownMarketplaces`](#extraknownmarketplaces)                                                   | Daftarkan [marketplace](/docs/id/plugins/overview) untuk repositori atau organisasi                                                                                                                                                                                | Plugins and skills                 | Any file                |
| [`fallbackModel`](#fallbackmodel)                                                                     | Beri nama [model cadangan](/docs/id/model-config#fallback-model-chains) untuk ketika model utama kelebihan beban                                                                                                                                                   | Model dan respons                  | Any file                |
| [`fastMode`](#fastmode)                                                                               | Nyalakan [mode cepat](/docs/id/fast-mode) untuk sesi tempat tersedia                                                                                                                                                                                               | Model dan respons                  | Any file                |
| [`fastModePerSessionOptIn`](#fastmodepersessionoptin)                                                 | Perlukan orang untuk menghidupkan [mode cepat](/docs/id/fast-mode) setiap sesi                                                                                                                                                                                     | Model dan respons                  | Any file                |
| [`feedbackDrafts`](#feedbackdrafts)                                                                   | Kontrol apakah Claude mengantre [draft umpan balik](/docs/id/tools-reference#sendfeedback-tool-behavior) untuk Anda tinjau                                                                                                                                         | Privacy and telemetry              | User or managed         |
| [`feedbackSurveyRate`](#feedbacksurveyrate)                                                           | Ubah seberapa sering [survei kualitas sesi](/docs/id/data-usage#session-quality-surveys) muncul                                                                                                                                                                    | Privacy and telemetry              | Any file                |
| [`fileCheckpointingEnabled`](#filecheckpointingenabled)                                               | Matikan atau nyalakan snapshot file yang [`/rewind`](/docs/id/checkpointing) pulihkan                                                                                                                                                                              | Memory and context                 | Any file                |
| [`fileSuggestion`](#filesuggestion)                                                                   | Sediakan pelengkapan otomatis file [`@`](/docs/id/interactive-mode#quick-commands) dari perintah Anda sendiri                                                                                                                                                      | Interface and terminal             | Any file                |
| [`footerLinksRegexes`](#footerlinksregexes)                                                           | Buat ID masalah atau tinjauan dalam output menjadi [tautan yang dapat diklik](/docs/id/statusline#clickable-links) di bawah kotak input                                                                                                                            | Interface and terminal             | User or managed         |
| [`forceLoginGatewayUrl`](#forcelogingatewayurl)                                                       | Atur [URL gateway](/docs/id/claude-apps-gateway#set-the-gateway-url) yang layar login terhubung                                                                                                                                                                    | Authentication and providers       | Managed                 |
| [`forceLoginMethod`](#forceloginmethod)                                                               | [Batasi login](/docs/id/authentication#restrict-login-to-your-organization) ke claude.ai, Claude Console, atau [gateway cloud](/docs/id/claude-apps-gateway)                                                                                                            | Authentication and providers       | Any file                |
| [`forceLoginOrgUUID`](#forceloginorguuid)                                                             | [Pin login claude.ai ke organisasi Anda](/docs/id/authentication#restrict-login-to-your-organization); hanya sumber yang dikelola yang memberlakukannya                                                                                                            | Authentication and providers       | Any file                |
| [`forceRemoteSettingsRefresh`](#forceremotesettingsrefresh)                                           | Blokir startup hingga [pengaturan yang dikelola server](/docs/id/server-managed-settings) segar diambil                                                                                                                                                            | Enterprise and managed settings    | Managed                 |
| [`gatewayInternalNetworks`](#gatewayinternalnetworks)                                                 | Biarkan `/login` menjangkau [gateway cloud](/docs/id/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) pada ruang IPv4 publik yang organisasi Anda gunakan secara internal                                                                      | Authentication and providers       | Managed                 |
| [`gcpAuthRefresh`](#gcpauthrefresh)                                                                   | Segarkan [kredensial Google Cloud](/docs/id/google-vertex-ai#advanced-credential-configuration) dengan perintah Anda sendiri                                                                                                                                       | Authentication and providers       | Any file                |
| [`hooks`](#hooks)                                                                                     | Jalankan perintah Anda sendiri sebagai [hooks](/docs/id/hooks) pada titik dalam siklus hidup Claude Code                                                                                                                                                           | Hooks and automation               | Any file                |
| [`httpHookAllowedEnvVars`](#httphookallowedenvvars)                                                   | Batasi variabel env mana yang dapat dimasukkan [HTTP hooks](/docs/id/hooks) ke header                                                                                                                                                                              | Hooks and automation               | Any file                |
| [`includeCoAuthoredBy`](#includecoauthoredby)                                                         | Usang; gunakan `attribution` untuk menyembunyikan atau mengubah atribusi commit dan PR                                                                                                                                                                        | Git and attribution                | Any file                |
| [`includeGitInstructions`](#includegitinstructions)                                                   | Hapus instruksi commit dan PR bawaan dari konteks Claude                                                                                                                                                                                                      | Git and attribution                | Any file                |
| [`inputNeededNotifEnabled`](#inputneedednotifenabled)                                                 | Dapatkan [notifikasi push](/docs/id/remote-control#mobile-push-notifications) ketika Claude menunggu Anda                                                                                                                                                          | Remote, desktop, and notifications | Any file                |
| [`isolatePeerMachines`](#isolatepeermachines)                                                         | Tanyakan Anda sebelum Claude [mengirim pesan ke salah satu sesi Anda di mesin lain](/docs/id/cross-session-messaging#require-approval-for-cross-machine-messages)                                                                                                  | Agents, sessions, and worktrees    | Any file                |
| [`keybindingFlavor`](#keybindingflavor)                                                               | Usang dan tidak berpengaruh; pintasan pengeditan kata selalu [mengikuti konvensi readline](/docs/id/interactive-mode#make-ctrl-w-delete-back-to-whitespace)                                                                                                        | Interface and terminal             | Any file                |
| [`language`](#language)                                                                               | Biarkan Claude merespons dalam bahasa selain Inggris                                                                                                                                                                                                          | Model dan respons                  | Any file                |
| [`managedMcpServers`](#managedmcpservers)                                                             | Sediakan server [MCP](/docs/id/managed-mcp#provide-servers-through-managed-settings) jarak jauh kepada setiap pengguna bersama dengan yang mereka tambahkan                                                                                                        | MCP                                | Managed                 |
| [`managedSourcesBehavior`](#managedsourcesbehavior)                                                   | Komposisi setiap [sumber yang dikelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources) yang Anda terapkan daripada menggunakan satu-satunya prioritas tertinggi                                                                               | Enterprise and managed settings    | Managed                 |
| [`maxEffortLevel`](#maxeffortlevel)                                                                   | Batasi [tingkat usaha](/docs/id/model-config#adjust-effort-level) untuk setiap model atau per model, di setiap penyedia                                                                                                                                            | Model dan respons                  | Any file                |
| [`minimumVersion`](#minimumversion)                                                                   | Jaga [pembaruan otomatis](/docs/id/setup#pin-a-minimum-version) dari memasang apa pun di bawah versi                                                                                                                                                               | Updates and versioning             | Any file                |
| [`model`](#model)                                                                                     | Ubah [model](/docs/id/model-config#set-a-default-model-for-new-sessions) yang Claude Code mulai                                                                                                                                                                    | Model dan respons                  | Any file                |
| [`modelOverrides`](#modeloverrides)                                                                   | [Petakan ID model](/docs/id/model-config#override-model-ids-per-version) ke ID penyedia Anda, seperti ARN Bedrock                                                                                                                                                  | Model dan respons                  | Any file                |
| [`modelPicker`](#modelpicker)                                                                         | Pilih model mana yang dicantumkan oleh pemilih [`/model`](/docs/id/model-config#available-models), dalam urutan Anda sendiri dan dengan label Anda sendiri                                                                                                         | Model dan respons                  | User or managed         |
| [`modelPricing`](#modelpricing)                                                                       | Laporkan pengeluaran dengan tarif kontrak organisasi Anda daripada harga daftar                                                                                                                                                                               | Model dan respons                  | Managed                 |
| [`modelSettings`](#modelsettings)                                                                     | Simpan [tingkat usaha](/docs/id/model-config#adjust-effort-level) per model, atau batasi usaha satu model                                                                                                                                                          | Model dan respons                  | Any file                |
| [`otelHeadersHelper`](#otelheadershelper)                                                             | Hasilkan header [OpenTelemetry](/docs/id/monitoring-usage#dynamic-headers) yang berputar dengan perintah Anda sendiri                                                                                                                                              | Authentication and providers       | Any file                |
| [`outputStyle`](#outputstyle)                                                                         | Ubah peran, nada, dan format output Claude dengan [gaya output](/docs/id/output-styles)                                                                                                                                                                            | Model dan respons                  | Any file                |
| [`parentSettingsBehavior`](#parentsettingsbehavior)                                                   | Terapkan atau lepaskan pembatasan yang diteruskan oleh [host SDK atau IDE](/docs/id/managed-settings#let-an-embedding-host-add-policy) ketika Anda menerapkan [pengaturan yang dikelola](/docs/id/managed-settings)                                                     | Enterprise and managed settings    | Managed                 |
| [`permissionExplainerEnabled`](#permissionexplainerenabled)                                           | Dihapus di v2.1.257, bersama dengan penjelasan perintah `Ctrl+E` pada prompt izin shell                                                                                                                                                                       | Global config settings             | Global config           |
| [`permissions`](#permissions)                                                                         | Atur aturan izin, tanya, dan tolak serta [mode izin](/docs/id/permission-modes) awal                                                                                                                                                                               | Permission settings                | Any file                |
| [`permissions.additionalDirectories`](#permissions-additionaldirectories)                             | Berikan akses file Claude ke [direktori di luar yang saat ini](/docs/id/permissions#working-directories)                                                                                                                                                           | Permission settings                | Any file                |
| [`permissions.allow`](#permissions-allow)                                                             | Setujui [penggunaan alat](/docs/id/permissions#permission-rule-syntax) yang tercantum tanpa prompt                                                                                                                                                                 | Permission settings                | Any file                |
| [`permissions.ask`](#permissions-ask)                                                                 | Selalu minta sebelum [penggunaan alat](/docs/id/permissions#permission-rule-syntax) yang tercantum                                                                                                                                                                 | Permission settings                | Any file                |
| [`permissions.blockReadsOutsideWorkingDirectories`](#permissions-blockreadsoutsideworkingdirectories) | Buat alat file menolak pembacaan di luar [direktori kerja](/docs/id/permissions#working-directories) dalam setiap mode izin                                                                                                                                        | Permission settings                | Any file                |
| [`permissions.defaultMode`](#permissions-defaultmode)                                                 | Atur [mode izin](/docs/id/permission-modes#which-mode-a-session-starts-in) yang dimulai sesi baru                                                                                                                                                                  | Permission settings                | Any file                |
| [`permissions.deny`](#permissions-deny)                                                               | Blokir [penggunaan alat](/docs/id/permissions#permission-rule-syntax) yang tercantum, termasuk pembacaan file yang menyimpan rahasia                                                                                                                               | Permission settings                | Any file                |
| [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode)               | Cegah siapa pun memasuki [mode bypassPermissions](/docs/id/permission-modes#skip-all-checks-with-bypasspermissions-mode)                                                                                                                                           | Permission settings                | Any file                |
| [`plansDirectory`](#plansdirectory)                                                                   | Pilih tempat [mode plan](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode) menulis file rencana                                                                                                                                                    | Memory and context                 | Any file                |
| [`pluginConfigs`](#pluginconfigs)                                                                     | Simpan jawaban yang Anda berikan dialog konfigurasi [plugin](/docs/id/plugins/overview)                                                                                                                                                                            | Plugins and skills                 | User or managed         |
| [`pluginSuggestionMarketplaces`](#pluginsuggestionmarketplaces)                                       | Pilih [marketplace](/docs/id/plugins/overview) mana yang dapat menampilkan saran instalasi plugin di `/plugin`                                                                                                                                                     | Plugins and skills                 | Managed                 |
| [`pluginTrustMessage`](#plugintrustmessage)                                                           | Tambahkan teks Anda sendiri ke peringatan kepercayaan [plugin](/docs/id/plugins/overview)                                                                                                                                                                          | Plugins and skills                 | Managed                 |
| [`policyHelper`](#policyhelper)                                                                       | Jalankan executable yang menghitung [pengaturan yang dikelola](/docs/id/managed-settings#compute-the-policy-with-a-helper-program) saat startup                                                                                                                    | Enterprise and managed settings    | Managed                 |
| [`policyHelper.path`](#policyhelper-path)                                                             | Beri nama [executable pembantu](/docs/id/managed-settings#compute-the-policy-with-a-helper-program) yang Claude Code jalankan                                                                                                                                      | Enterprise and managed settings    | Managed                 |
| [`policyHelper.refreshIntervalMs`](#policyhelper-refreshintervalms)                                   | Jalankan kembali [pembantu](/docs/id/managed-settings#compute-the-policy-with-a-helper-program) di latar belakang pada interval                                                                                                                                    | Enterprise and managed settings    | Managed                 |
| [`policyHelper.timeoutMs`](#policyhelper-timeoutms)                                                   | Atur berapa lama Claude Code menunggu [pembantu](/docs/id/managed-settings#compute-the-policy-with-a-helper-program)                                                                                                                                               | Enterprise and managed settings    | Managed                 |
| [`preferredNotifChannel`](#preferrednotifchannel)                                                     | Pilih [bel terminal atau notifikasi desktop](/docs/id/terminal-config#get-a-terminal-bell-or-notification) untuk penyelesaian tugas                                                                                                                                | Remote, desktop, and notifications | Any file                |
| [`prefersReducedMotion`](#prefersreducedmotion)                                                       | [Kurangi atau matikan](/docs/id/accessibility#accessibility-settings) animasi spinner, shimmer, dan flash                                                                                                                                                          | Interface and terminal             | Any file                |
| [`processWrapper`](#processwrapper)                                                                   | Jalankan proses latar belakang Claude Code melalui [peluncur perusahaan](/docs/id/corporate-launcher) di macOS dan Linux                                                                                                                                           | Agents, sessions, and worktrees    | User or managed         |
| [`promptCacheTtl`](#promptcachettl)                                                                   | Pilih [masa pakai cache prompt](/docs/id/prompt-caching#cache-lifetime) untuk percakapan utama                                                                                                                                                                     | Model dan respons                  | Any file                |
| [`promptSuggestionEnabled`](#promptsuggestionenabled)                                                 | Sembunyikan [saran prompt](/docs/id/interactive-mode#prompt-suggestions) yang dikelabu dalam kotak input                                                                                                                                                           | Interface and terminal             | Any file                |
| [`prUrlTemplate`](#prurltemplate)                                                                     | Arahkan tautan PR ke alat tinjauan kode internal daripada github.com                                                                                                                                                                                          | Git and attribution                | Any file                |
| [`remote.defaultEnvironmentId`](#remote-defaultenvironmentid)                                         | Pilih [lingkungan cloud](/docs/id/cloud-environments) default untuk `claude --cloud`; ID `ccpool_` yang di-host sendiri hanya dibaca dari pengaturan pengguna dan yang dikelola serta `--settings`                                                                 | Remote, desktop, and notifications | Any file                |
| [`remoteControlAtStartup`](#remotecontrolatstartup)                                                   | Hubungkan [Remote Control](/docs/id/remote-control#enable-remote-control-for-all-sessions) secara otomatis ketika sesi dimulai                                                                                                                                     | Remote, desktop, and notifications | Any file                |
| [`requiredMaximumVersion`](#requiredmaximumversion)                                                   | [Tolak untuk memulai](/docs/id/setup#pin-a-minimum-version) pada versi yang lebih baru daripada yang diizinkan organisasi Anda                                                                                                                                     | Updates and versioning             | Managed                 |
| [`requiredMinimumVersion`](#requiredminimumversion)                                                   | [Tolak untuk memulai](/docs/id/setup#pin-a-minimum-version) pada versi yang lebih lama daripada yang diperlukan organisasi Anda                                                                                                                                    | Updates and versioning             | Managed                 |
| [`respectGitignore`](#respectgitignore)                                                               | Jaga file yang diabaikan git keluar dari [pemilih file `@`](/docs/id/interactive-mode#quick-commands)                                                                                                                                                              | Interface and terminal             | Any file                |
| [`respondToBashCommands`](#respondtobashcommands)                                                     | Hentikan Claude dari merespons setelah perintah shell [`!`](/docs/id/interactive-mode#shell-mode-with-prefix) berjalan                                                                                                                                             | Interface and terminal             | Any file                |
| [`sandbox`](#sandbox)                                                                                 | [Isolasi perintah Bash](/docs/id/sandboxing) dari sistem file dan jaringan Anda di macOS, Linux, dan WSL2                                                                                                                                                          | Sandbox settings                   | Any file                |
| [`sandbox.allowAppleEvents`](#sandbox-allowappleevents)                                               | Biarkan perintah [bersandbox](/docs/id/sandboxing) mengirim Apple Events di macOS                                                                                                                                                                                  | Sandbox settings                   | User or managed         |
| [`sandbox.allowUnsandboxedCommands`](#sandbox-allowunsandboxedcommands)                               | Biarkan Claude mencoba ulang perintah yang diblokir di luar [sandbox](/docs/id/sandboxing#the-unsandboxed-retry-escape-hatch), atau larangnya                                                                                                                      | Sandbox settings                   | Any file                |
| [`sandbox.autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed)                               | Jalankan perintah [bersandbox](/docs/id/sandboxing#auto-allow-mode) tanpa prompt izin                                                                                                                                                                              | Sandbox settings                   | Any file                |
| [`sandbox.bwrapPath`](#sandbox-bwrappath)                                                             | Arahkan [sandbox](/docs/id/sandboxing) ke biner bubblewrap di luar `PATH`                                                                                                                                                                                          | Sandbox settings                   | Managed                 |
| [`sandbox.credentials`](#sandbox-credentials)                                                         | Sembunyikan atau topeng file kredensial dan variabel di dalam [sandbox](/docs/id/sandboxing#protect-credentials)                                                                                                                                                   | Sandbox settings                   | Any file                |
| [`sandbox.credentials.allowPlaintextInject`](#sandbox-credentials-allowplaintextinject)               | Biarkan [kredensial yang ditopeng](/docs/id/sandboxing#mask-credentials) mencapai layanan HTTP biasa di jaringan uji terpercaya                                                                                                                                    | Sandbox settings                   | User or managed         |
| [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs)                                       | Tautkan variabel kunci AWS bernama kustom ke satu kredensial untuk [penandatanganan ulang](/docs/id/sandboxing#re-sign-aws-requests)                                                                                                                               | Sandbox settings                   | User or managed         |
| [`sandbox.credentials.envVars`](#sandbox-credentials-envvars)                                         | Batalkan atau topeng variabel lingkungan di dalam [sandbox](/docs/id/sandboxing#mask-environment-variables)                                                                                                                                                        | Sandbox settings                   | Any file                |
| [`sandbox.credentials.files`](#sandbox-credentials-files)                                             | Blokir atau topeng pembacaan file kredensial di dalam [sandbox](/docs/id/sandboxing#mask-credential-files)                                                                                                                                                         | Sandbox settings                   | Any file                |
| [`sandbox.credentials.sigv4`](#sandbox-credentials-sigv4)                                             | Pilih apakah streaming, presigned, atau permintaan [AWS SigV4A](/docs/id/sandboxing#re-sign-aws-requests) gagal atau lulus                                                                                                                                         | Sandbox settings                   | User or managed         |
| [`sandbox.enabled`](#sandbox-enabled)                                                                 | Nyalakan [sandboxing Bash](/docs/id/sandboxing#get-started) di macOS, Linux, dan WSL2                                                                                                                                                                              | Sandbox settings                   | Any file                |
| [`sandbox.enableWeakerNestedSandbox`](#sandbox-enableweakernestedsandbox)                             | Jalankan [sandbox](/docs/id/sandboxing) Linux di dalam kontainer yang tidak istimewa                                                                                                                                                                               | Sandbox settings                   | Any file                |
| [`sandbox.enableWeakerNetworkIsolation`](#sandbox-enableweakernetworkisolation)                       | Biarkan `gh`, `gcloud`, dan `terraform` memverifikasi TLS di balik proxy MITM di dalam [sandbox](/docs/id/sandboxing#troubleshooting) di macOS                                                                                                                     | Sandbox settings                   | Any file                |
| [`sandbox.excludedCommands`](#sandbox-excludedcommands)                                               | Beri nama perintah yang selalu berjalan di luar [sandbox](/docs/id/sandboxing)                                                                                                                                                                                     | Sandbox settings                   | Any file                |
| [`sandbox.failIfUnavailable`](#sandbox-failifunavailable)                                             | Tolak untuk memulai ketika [sandbox](/docs/id/sandboxing) tidak bisa, daripada berjalan tanpa sandbox                                                                                                                                                              | Sandbox settings                   | Any file                |
| [`sandbox.filesystem`](#sandbox-filesystem)                                                           | Kontrol jalur mana yang dapat dibaca dan ditulis perintah [bersandbox](/docs/id/sandboxing#filesystem-isolation)                                                                                                                                                   | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly)       | Hentikan pengembang dari membuka kembali [jalur baca yang organisasi Anda blokir](/docs/id/sandboxing#keep-developers-from-widening-the-policy)                                                                                                                    | Sandbox settings                   | Managed                 |
| [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread)                                       | Buka kembali pembacaan di dalam wilayah yang [`denyRead`](#sandbox-filesystem-denyread) blokir                                                                                                                                                                | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.allowWrite`](#sandbox-filesystem-allowwrite)                                     | Tambahkan jalur yang dapat ditulis perintah [bersandbox](/docs/id/sandboxing)                                                                                                                                                                                      | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread)                                         | Blokir perintah [bersandbox](/docs/id/sandboxing) dari membaca jalur tertentu                                                                                                                                                                                      | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.denyWrite`](#sandbox-filesystem-denywrite)                                       | Blokir perintah [bersandbox](/docs/id/sandboxing) dari menulis ke jalur tertentu                                                                                                                                                                                   | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.disabled`](#sandbox-filesystem-disabled)                                         | [Matikan isolasi sistem file](/docs/id/sandboxing#disable-filesystem-isolation) sambil menjaga isolasi jaringan                                                                                                                                                    | Sandbox settings                   | User or managed         |
| [`sandbox.ignoreViolations`](#sandbox-ignoreviolations)                                               | Senyapkan laporan pelanggaran untuk jalur yang diharapkan diperiksa perintah                                                                                                                                                                                  | Sandbox settings                   | Any file                |
| [`sandbox.network`](#sandbox-network)                                                                 | Kontrol host, port, dan soket mana yang dapat dijangkau perintah [bersandbox](/docs/id/sandboxing#network-isolation)                                                                                                                                               | Sandbox settings                   | Any file                |
| [`sandbox.network.allowAllUnixSockets`](#sandbox-network-allowallunixsockets)                         | Biarkan perintah [bersandbox](/docs/id/sandboxing) terhubung ke setiap soket Unix                                                                                                                                                                                  | Sandbox settings                   | Any file                |
| [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains)                                   | Izinkan domain sebelumnya sehingga perintah [bersandbox](/docs/id/sandboxing) tidak meminta mereka                                                                                                                                                                 | Sandbox settings                   | Any file                |
| [`sandbox.network.allowLocalBinding`](#sandbox-network-allowlocalbinding)                             | Biarkan perintah [bersandbox](/docs/id/sandboxing) mengikat ke port localhost di macOS                                                                                                                                                                             | Sandbox settings                   | Any file                |
| [`sandbox.network.allowMachLookup`](#sandbox-network-allowmachlookup)                                 | Biarkan alat macOS [bersandbox](/docs/id/sandboxing) seperti iOS Simulator atau Playwright menjangkau layanan XPC mereka                                                                                                                                           | Sandbox settings                   | Any file                |
| [`sandbox.network.allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly)                 | Kunci daftar izin jaringan ke [pengaturan yang dikelola](/docs/id/sandboxing#keep-developers-from-widening-the-policy)                                                                                                                                             | Sandbox settings                   | Managed                 |
| [`sandbox.network.allowUnixSockets`](#sandbox-network-allowunixsockets)                               | Daftar jalur soket Unix yang dapat digunakan perintah [bersandbox](/docs/id/sandboxing) di macOS                                                                                                                                                                   | Sandbox settings                   | Any file                |
| [`sandbox.network.deniedDomains`](#sandbox-network-denieddomains)                                     | Blokir domain untuk perintah [bersandbox](/docs/id/sandboxing), bahkan di dalam wildcard yang diizinkan                                                                                                                                                            | Sandbox settings                   | Any file                |
| [`sandbox.network.httpProxyPort`](#sandbox-network-httpproxyport)                                     | Rute lalu lintas [sandbox](/docs/id/sandboxing#custom-proxy-configuration) HTTP melalui proxy Anda sendiri                                                                                                                                                         | Sandbox settings                   | Any file                |
| [`sandbox.network.socksProxyPort`](#sandbox-network-socksproxyport)                                   | Rute lalu lintas [sandbox](/docs/id/sandboxing#custom-proxy-configuration) SOCKS melalui proxy Anda sendiri                                                                                                                                                        | Sandbox settings                   | Any file                |
| [`sandbox.network.strictAllowlist`](#sandbox-network-strictallowlist)                                 | Tolak host di luar [daftar izin](/docs/id/sandboxing#network-isolation) daripada meminta                                                                                                                                                                           | Sandbox settings                   | User or managed         |
| [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate)                                       | Biarkan [sandbox](/docs/id/sandboxing#network-isolation) proxy menghentikan TLS sehingga dapat membaca permintaan HTTPS                                                                                                                                            | Sandbox settings                   | User or managed         |
| [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                 | Gunakan biner ripgrep Anda sendiri di dalam [sandbox](/docs/id/sandboxing)                                                                                                                                                                                         | Sandbox settings                   | User or managed         |
| [`sandbox.socatPath`](#sandbox-socatpath)                                                             | Arahkan proxy [sandbox](/docs/id/sandboxing) ke biner `socat` di luar `PATH`                                                                                                                                                                                       | Sandbox settings                   | Managed                 |
| [`showClearContextOnPlanAccept`](#showclearcontextonplanaccept)                                       | Tampilkan opsi "clear context" pada [layar penerimaan rencana](/docs/id/permission-modes#review-and-approve-a-plan)                                                                                                                                                | Interface and terminal             | Any file                |
| [`showThinkingSummaries`](#showthinkingsummaries)                                                     | Lihat ringkasan [pemikiran](/docs/id/model-config#extended-thinking) Claude daripada stub yang runtuh                                                                                                                                                              | Model dan respons                  | Any file                |
| [`showTurnDuration`](#showturnduration)                                                               | Sembunyikan durasi "Cooked for" setelah setiap respons                                                                                                                                                                                                        | Interface and terminal             | Any file                |
| [`skillListingBudgetFraction`](#skilllistingbudgetfraction)                                           | Cadangkan lebih banyak atau lebih sedikit konteks untuk [daftar skill](/docs/id/skills#skill-descriptions-are-cut-short)                                                                                                                                           | Memory and context                 | Any file                |
| [`skillListingMaxDescChars`](#skilllistingmaxdescchars)                                               | Batasi panjang deskripsi setiap skill dalam [daftar skill](/docs/id/skills#skill-descriptions-are-cut-short)                                                                                                                                                       | Memory and context                 | Any file                |
| [`skillOverrides`](#skilloverrides)                                                                   | [Sembunyikan atau lipat skill](/docs/id/skills#override-skill-visibility-from-settings) tanpa mengedit SKILL.md-nya                                                                                                                                                | Plugins and skills                 | Any file                |
| [`skipAutoPermissionPrompt`](#skipautopermissionprompt)                                               | Lewati pemberitahuan satu kali yang ditampilkan Claude Code ketika Anda pertama kali memasuki [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) sendiri daripada melalui default bawaan                                                  | Permission settings                | User or managed         |
| [`skipDangerousModePermissionPrompt`](#skipdangerousmodepermissionprompt)                             | Lewati dialog konfirmasi sebelum [mode bypassPermissions](/docs/id/permission-modes#skip-all-checks-with-bypasspermissions-mode)                                                                                                                                   | Permission settings                | User, local, or managed |
| [`skipWebFetchPreflight`](#skipwebfetchpreflight)                                                     | Lewati [pemeriksaan nama host WebFetch](/docs/id/tools-reference#webfetch-tool-behavior) ketika Anthropic tidak dapat dijangkau                                                                                                                                    | Privacy and telemetry              | Any file                |
| [`spellcheck`](#spellcheck)                                                                           | Garis bawahi kata-kata yang salah eja dalam input prompt dengan [pemeriksa ejaan](/docs/id/interactive-mode#check-spelling-as-you-type) yang Anda pasang                                                                                                           | Interface and terminal             | User or managed         |
| [`spinnerTipsEnabled`](#spinnertipsenabled)                                                           | Sembunyikan tips dalam spinner saat Claude bekerja                                                                                                                                                                                                            | Interface and terminal             | Any file                |
| [`spinnerTipsOverride`](#spinnertipsoverride)                                                         | Tambahkan tips Anda sendiri ke rotasi spinner, atau ganti tips bawaan                                                                                                                                                                                         | Interface and terminal             | Any file                |
| [`spinnerVerbs`](#spinnerverbs)                                                                       | Tambahkan atau ganti kata kerja yang ditampilkan saat giliran berjalan                                                                                                                                                                                        | Interface and terminal             | Any file                |
| [`sshConfigs`](#sshconfigs)                                                                           | Tambahkan [koneksi SSH](/docs/id/desktop#pre-configure-ssh-connections-for-your-team) ke dropdown lingkungan Desktop                                                                                                                                               | Remote, desktop, and notifications | User or managed         |
| [`sshHostAllowlist`](#sshhostallowlist)                                                               | Batasi host mana yang dapat dijangkau [sesi SSH Desktop](/docs/id/desktop#restrict-which-ssh-hosts-users-can-connect-to)                                                                                                                                           | Remote, desktop, and notifications | Managed                 |
| [`statusLine`](#statusline)                                                                           | Jalankan perintah Anda sendiri untuk merender [baris status](/docs/id/statusline) di bawah prompt                                                                                                                                                                  | Interface and terminal             | Any file                |
| [`strictKnownMarketplaces`](#strictknownmarketplaces)                                                 | Daftar izin sumber [marketplace](/docs/id/plugins/overview) yang dapat ditambahkan dan dipasang pengguna                                                                                                                                                           | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)                                     | Blokir [skill](/docs/id/skills), [agent](/docs/id/sub-agents), [hooks](/docs/id/hooks), dan [server MCP](/docs/id/mcp) dari sumber pengguna dan proyek                                                                                                                            | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.agents`](#strictpluginonlycustomization-agents)                       | Kunci [agent](/docs/id/sub-agents) ke sumber plugin dan yang dikelola                                                                                                                                                                                              | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.hooks`](#strictpluginonlycustomization-hooks)                         | Kunci [hooks](/docs/id/hooks) ke sumber plugin dan yang dikelola                                                                                                                                                                                                   | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.mcp`](#strictpluginonlycustomization-mcp)                             | Kunci [server MCP](/docs/id/mcp) ke sumber plugin dan yang dikelola                                                                                                                                                                                                | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.skills`](#strictpluginonlycustomization-skills)                       | Kunci [skill](/docs/id/skills) ke sumber plugin dan yang dikelola                                                                                                                                                                                                  | Plugins and skills                 | Managed                 |
| [`subagentPromptCacheTtl`](#subagentpromptcachettl)                                                   | Pilih [masa pakai cache prompt](/docs/id/prompt-caching#cache-lifetime) untuk subagent dan permintaan lain di luar percakapan utama                                                                                                                                | Model dan respons                  | Any file                |
| [`subagentStatusLine`](#subagentstatusline)                                                           | Tulis ulang baris dalam tampilan tugas [subagent](/docs/id/sub-agents) dengan perintah Anda sendiri                                                                                                                                                                | Interface and terminal             | Any file                |
| [`switchModelsOnFlag`](#switchmodelsonflag)                                                           | Alihkan model secara otomatis atau jeda ketika [pengklasifikasi keamanan](/docs/id/model-config#ask-before-switching) menandai permintaan                                                                                                                          | Model dan respons                  | Any file                |
| [`syncClaudeAiPlugins`](#syncclaudeaiplugins)                                                         | Hentikan pemuatan [plugin yang diaktifkan di akun claude.ai Anda](/docs/id/plugins/overview) dan hentikan pengunduhan yang baru                                                                                                                                    | Plugins and skills                 | User, local, or managed |
| [`syncClaudeAiSkills`](#syncclaudeaiskills)                                                           | Hentikan pemuatan [skill yang diaktifkan di akun claude.ai Anda](/docs/id/skills#how-synced-skills-behave) dan hentikan pengunduhan yang baru                                                                                                                      | Plugins and skills                 | User, local, or managed |
| [`syntaxHighlightingDisabled`](#syntaxhighlightingdisabled)                                           | Matikan penyorotan sintaks dalam diff dan blok kode                                                                                                                                                                                                           | Interface and terminal             | Any file                |
| [`taskOutputMaxChars`](#taskoutputmaxchars)                                                           | Dihapus di v2.1.277, bersama dengan alat `TaskOutput` yang disizingnya                                                                                                                                                                                        | Memory and context                 | Any file                |
| [`teammateDefaultModel`](#teammatedefaultmodel)                                                       | Dihapus di v2.1.234; lihat [Tentukan rekan tim dan model](/docs/id/agent-teams#specify-teammates-and-models) untuk cara Claude Code memilih model rekan tim                                                                                                        | Global config settings             | Global config           |
| [`teammateMode`](#teammatemode)                                                                       | Pilih cara [rekan tim tim agent](/docs/id/agent-teams#choose-a-display-mode) ditampilkan                                                                                                                                                                           | Agents, sessions, and worktrees    | Any file                |
| [`terminalProgressBarEnabled`](#terminalprogressbarenabled)                                           | Sembunyikan bilah kemajuan terminal di terminal yang mendukungnya                                                                                                                                                                                             | Interface and terminal             | Any file                |
| [`terminalTitleFromRename`](#terminaltitlefromrename)                                                 | Hentikan [`/rename`](/docs/id/sessions#name-your-sessions) dan `--name` dari mengubah judul tab terminal                                                                                                                                                           | Interface and terminal             | Any file                |
| [`theme`](#theme)                                                                                     | Pilih [tema warna](/docs/id/terminal-config#match-the-color-theme) antarmuka, bawaan atau kustom                                                                                                                                                                   | Interface and terminal             | Any file                |
| [`timeFormat`](#timeformat)                                                                           | Tampilkan waktu dalam antarmuka pada jam 12 jam atau 24 jam, dalam UTC, atau dengan pola strftime                                                                                                                                                             | Interface and terminal             | Any file                |
| [`timeZone`](#timezone)                                                                               | Tampilkan waktu dalam antarmuka di zona waktu selain sistem Anda                                                                                                                                                                                              | Interface and terminal             | Any file                |
| [`tui`](#tui)                                                                                         | Pilih renderer [fullscreen](/docs/id/fullscreen) atau terminal klasik                                                                                                                                                                                              | Interface and terminal             | Any file                |
| [`ultracode`](#ultracode)                                                                             | Biarkan Claude merencanakan [workflow](/docs/id/workflows#let-claude-decide-with-ultracode) untuk setiap tugas substansial tanpa diminta                                                                                                                           | Model dan respons                  | Any file                |
| [`useAutoModeDuringPlan`](#useautomodeduringplan)                                                     | Biarkan pengklasifikasi [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) meninjau perintah shell dalam [mode plan](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode); atur `false` untuk mendapatkan prompt sebagai gantinya | Permission settings                | User, local, or managed |
| [`verbose`](#verbose)                                                                                 | Tampilkan [output alat lengkap](/docs/id/cli-reference#cli-flags) daripada ringkasan terpotong; `viewMode` mengambil alih ketika keduanya diatur                                                                                                                   | Interface and terminal             | Any file                |
| [`viewMode`](#viewmode)                                                                               | Mulai setiap sesi dalam [tampilan default, verbose, atau focus](/docs/id/cli-reference#cli-flags)                                                                                                                                                                  | Interface and terminal             | Any file                |
| [`vimInsertModeRemaps`](#viminsertmoderemaps)                                                         | Petakan [urutan mode INSERT](/docs/id/interactive-mode#remap-insert-mode-key-sequences) dua kunci seperti `jj` ke Escape                                                                                                                                           | Interface and terminal             | User or managed         |
| [`voice`](#voice)                                                                                     | Nyalakan [dikte suara](/docs/id/voice-dictation) dan pilih mode tahan atau ketuk                                                                                                                                                                                   | Interface and terminal             | Any file                |
| [`voiceEnabled`](#voiceenabled)                                                                       | Nyalakan [dikte suara](/docs/id/voice-dictation) dengan bentuk kunci tunggal yang lebih lama                                                                                                                                                                       | Interface and terminal             | Any file                |
| [`wheelScrollAccelerationEnabled`](#wheelscrollaccelerationenabled)                                   | Matikan [akselerasi roda mouse](/docs/id/fullscreen#mouse-wheel-scrolling) dalam rendering fullscreen                                                                                                                                                              | Interface and terminal             | Any file                |
| [`workflowKeywordTriggerEnabled`](#workflowkeywordtriggerenabled)                                     | Biarkan kata `ultracode` dalam prompt memulai [workflow](/docs/id/workflows); atur `false` untuk mengetiknya tanpa memulai satu                                                                                                                                    | Hooks and automation               | Any file                |
| [`workflowSizeGuideline`](#workflowsizeguideline)                                                     | Atur jumlah agent yang Claude targetkan dalam [workflow dinamis](/docs/id/workflows)                                                                                                                                                                               | Hooks and automation               | Any file                |
| [`worktree`](#worktree)                                                                               | Konfigurasikan cara Claude Code membuat git [worktree](/docs/id/worktrees)                                                                                                                                                                                         | Agents, sessions, and worktrees    | Any file                |
| [`worktree.baseRef`](#worktree-baseref)                                                               | Cabang [worktree](/docs/id/worktrees) baru dari cabang default jarak jauh atau HEAD lokal Anda                                                                                                                                                                     | Agents, sessions, and worktrees    | Any file                |
| [`worktree.bgIsolation`](#worktree-bgisolation)                                                       | Biarkan sesi latar belakang mengedit salinan kerja tanpa [worktree](/docs/id/worktrees)                                                                                                                                                                            | Agents, sessions, and worktrees    | Any file                |
| [`worktree.sparsePaths`](#worktree-sparsepaths)                                                       | Periksa hanya direktori yang Anda butuhkan di setiap [worktree](/docs/id/worktrees)                                                                                                                                                                                | Agents, sessions, and worktrees    | Any file                |
| [`worktree.symlinkDirectories`](#worktree-symlinkdirectories)                                         | Symlink direktori besar ke setiap [worktree](/docs/id/worktrees) daripada menduplikasinya                                                                                                                                                                          | Agents, sessions, and worktrees    | Any file                |
| [`wslInheritsWindowsSettings`](#wslinheritswindowssettings)                                           | Biarkan WSL membaca [pengaturan yang dikelola](/docs/id/managed-settings) dari rantai kebijakan Windows                                                                                                                                                            | Enterprise and managed settings    | Managed                 |

<h2 id="model-and-responses">
  Model dan respons
</h2>

Pilih model mana yang digunakan Claude Code dan cara meresponsnya. Untuk cara pengaturan ini berinteraksi dengan perintah `/model` dan variabel lingkungan, lihat [Konfigurasi model](/docs/id/model-config).

<h3 id="advisormodel">
  `advisorModel`
</h3>

Pilih model mana yang menjawab ketika Claude memanggil [alat advisor](/docs/id/advisor) di sisi server. Batalkan pengaturannya untuk mematikan advisor. Advisor harus setidaknya sama mampu dengan model utama Anda. Lihat [Pilih model advisor](/docs/id/advisor#choose-an-advisor-model) untuk pasangan yang diterima dan apa yang terjadi ketika Anda memilih yang tidak diterima.

Anda biasanya tidak mengedit kunci ini secara manual. Jalankan `/advisor` untuk membuka pemilih yang menunjukkan pilihan saat ini, model yang dapat memberikan saran, dan **No advisor**. Claude Code menyimpan pilihan Anda ke kunci ini di `~/.claude/settings.json`. Jika Anda memilih dari klien [Remote Control](/docs/id/remote-control) atau dalam sesi yang terhubung ke pekerja jarak jauh, pilihan tersebut hanya berlaku untuk sesi itu dan tidak mengubah kunci ini.

Jika akun Anda memerlukan [persetujuan usage-credits](/docs/id/advisor#fable-advisor-and-usage-credits), terima terlebih dahulu dengan menjalankan `/model fable`. Sampai Anda melakukannya, memilih Fable di `/advisor` tidak menyimpan apa pun dan Claude Code memberi tahu Anda untuk menjalankan `/model fable` terlebih dahulu.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, salah satu alias `"fable"`, `"opus"`, atau `"sonnet"`, yang diselesaikan ke versi default Claude Code saat ini dari keluarga model itu, atau ID model lengkap seperti `"claude-opus-5-5"`
* **Default**: tidak diatur, jadi advisor dimatikan
* **Per-session overrides**: `--advisor` mengambil alih kunci ini untuk satu sesi. [`CLAUDE_CODE_DISABLE_ADVISOR_TOOL`](/docs/id/env-vars) mematikan advisor, dan kunci ini tidak dapat menghidupkannya kembali

```json settings.json theme={null}
{
  "advisorModel": "opus"
}
```

Kunci ini tidak berpengaruh pada penyedia di mana advisor [tidak tersedia](/docs/id/advisor#requirements), seperti Amazon Bedrock dan Claude Platform di AWS. `"fable"` memerlukan [akses Fable](/docs/id/advisor#choose-an-advisor-model).

<h3 id="alwaysthinkingenabled">
  `alwaysThinkingEnabled`
</h3>

Matikan [extended thinking](/docs/id/model-config#extended-thinking) untuk setiap sesi dengan mengatur ini ke `false`. Thinking aktif secara default, jadi `true` tidak mengubah apa pun. Sebagian besar orang mengatur ini melalui `/config` daripada mengedit file.

Pada model yang selalu berpikir, seperti Opus 5.5 dan model Fable, `false` tidak berpengaruh. Pada [penyedia pihak ketiga](/docs/id/third-party-integrations) Claude Code menghilangkan parameter `thinking` daripada mematikan thinking, jadi model adaptive-reasoning mungkin masih berpikir. Dengan thinking dimatikan di Anthropic API, Claude Code mengirim effort `high` daripada level yang lebih tinggi ke model yang diketahui [tidak menerima kombinasi itu](/docs/id/errors#effort-isnt-available-with-thinking-turned-off), seperti Opus 5.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: tidak berpengaruh; thinking sudah aktif
  * `false`: Claude Code mematikan extended thinking untuk setiap sesi
* **Default**: tidak diatur, jadi thinking aktif untuk model yang mendukungnya
* **Per-session overrides**: [`MAX_THINKING_TOKENS`](/docs/id/env-vars) mengambil alih kunci ini untuk satu sesi: `0` mematikan thinking, di bawah batasan model dan penyedia yang sama seperti `false`, dan nilai positif menghidupkan thinking bahkan ketika kunci ini adalah `false`. Pada model adaptive-reasoning angka itu sendiri diabaikan

```json settings.json theme={null}
{
  "alwaysThinkingEnabled": false
}
```

<h3 id="availablemodels">
  `availableModels`
</h3>

Batasi model mana yang dapat dipilih orang untuk sesi utama, [subagents](/docs/id/sub-agents), [skills](/docs/id/skills), dan [advisor](/docs/id/advisor). Daftar terkelola membatasi `/model`, `--model`, dan kunci `model` dalam file pengembang sendiri; model di luar itu tidak dapat dipilih. Dengan sendirinya ini tidak menyentuh opsi Default; pasangkan dengan [`enforceAvailableModels`](#enforceavailablemodels) untuk itu.

* **Scope**: [`Any file`](#scopes). Terapkan dalam pengaturan terkelola untuk memberlakukannya untuk organisasi.
* **Type**: array alias model atau ID
* **Default**: tidak diatur, jadi setiap model tersedia

Contoh ini memungkinkan orang memilih hanya model Sonnet dan Haiku:

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

Lihat [Batasi pemilihan model](/docs/id/model-config#restrict-model-selection).

<h3 id="effortlevel">
  `effortLevel`
</h3>

Atur [effort level](/docs/id/model-config#adjust-effort-level) default untuk model yang belum Anda simpan levelnya. Level yang lebih rendah lebih cepat dan lebih murah untuk tugas-tugas langsung, dan level yang lebih tinggi bernalar lebih dalam pada masalah kompleks.

Ketika Anda menjalankan `/effort low`, `medium`, `high`, atau `xhigh` dalam sesi interaktif di mesin Anda, Claude Code menyimpan level untuk model aktif di bawah [`modelSettings`](#modelsettings) daripada menulis kunci ini. Sebelum v2.1.251, `/effort` menulis kunci ini.

Dalam file pengaturan yang sama, Claude Code menggunakan level yang disimpan model daripada kunci ini. [`modelSettings`](#modelsettings) menyatakan preseden lintas file.

Dalam sesi yang terhubung ke pekerja jarak jauh, dalam jalankan `-p`, dan dalam Agent SDK, `/effort` hanya berlaku untuk sesi itu. [Sesuaikan effort level](/docs/id/model-config#adjust-effort-level) mencantumkan pilihan interaktif yang juga hanya berlaku untuk sesi itu. Pesan yang dicetak `/effort` mengatakan apa yang terjadi.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, salah satu dari:
  * `"low"`: penalaran paling sedikit, untuk tugas pendek, terbatas, sensitif latensi yang tidak sensitif intelijen
  * `"medium"`: mengurangi penggunaan token untuk pekerjaan sensitif biaya yang dapat menukar beberapa intelijen
  * `"high"`: menyeimbangkan penggunaan token dan intelijen
  * `"xhigh"`: penalaran lebih dalam dengan pengeluaran token lebih tinggi
* **Default**: tidak diatur
* **Per-session overrides**: `--effort` mengambil alih kunci ini untuk satu sesi, dan [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/id/env-vars) mengambil alih keduanya

```json settings.json theme={null}
{
  "effortLevel": "xhigh"
}
```

Dalam file pengaturan pengguna Anda, `~/.claude/settings.json`, kunci ini adalah bentuk yang lebih lama `/effort` tulis sebelum menyimpan level per model, dan terus berlaku di mana itu berlaku sebelumnya, pada Opus 5, Fable 5.1, dan model sebelumnya. Opus 5.5 dan model yang dirilis setelahnya mengabaikannya dan mulai pada default mereka sendiri sampai Anda menyimpan level untuk mereka, yang `/effort` tulis di bawah [`modelSettings`](#modelsettings). Dalam pengaturan proyek, lokal, dan terkelola, dan dengan `--settings`, kunci ini berlaku untuk setiap model.

<h3 id="enforceavailablemodels">
  `enforceAvailableModels`
</h3>

Pemilih `/model` memiliki opsi **Default** yang diselesaikan ke [model default organisasi Anda](/docs/id/model-config#organization-default-model) ketika satu berlaku, dan sebaliknya ke default tipe akun Anda. Daftar allowlist [`availableModels`](#availablemodels) membatasi model yang dapat Anda beri nama, tetapi dengan sendirinya meninggalkan **Default** saja, jadi **Default** masih dapat diselesaikan ke model di luar daftar. Kunci ini menutup celah itu. Memerlukan Claude Code v2.1.175 atau lebih baru.

Ketika organisasi Anda menerapkan pengaturan terkelola apa pun, Claude Code membaca kunci ini dari sumber terkelola saja dan mengabaikannya di file lain Anda.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: ketika **Default** akan diselesaikan ke model di luar `availableModels`, Claude Code menyelesaikannya ke model pertama yang tersedia dalam daftar
  * `false`: **Default** diselesaikan seperti biasa, bahkan ke model di luar `availableModels`
* **Default**: `false`

Contoh ini membatasi pilihan bernama ke model Sonnet dan Haiku dan membuat **Default** diselesaikan ke yang pertama dari mereka yang tersedia:

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

Kunci ini tidak berpengaruh ketika `availableModels` tidak diatur atau kosong. Lihat [Berlakukan allowlist untuk model Default](/docs/id/model-config#enforce-the-allowlist-for-the-default-model). Memerlukan Claude Code v2.1.175 atau lebih baru.

<h3 id="fallbackmodel">
  `fallbackModel`
</h3>

Beri nama model cadangan untuk Claude Code coba, secara berurutan, ketika model utama Anda kelebihan beban atau tidak tersedia. Claude Code beralih ke model berikutnya yang tersedia dalam rantai untuk sisa giliran dan menampilkan pemberitahuan. Tanpa rantai, Claude Code mencoba ulang model yang sama dan kemudian menampilkan kesalahan server, dan Anda mencoba ulang atau beralih model sendiri.

Sakelar berarti satu giliran dengan [prompt cache](/docs/id/prompt-caching#switching-models) dingin pada model fallback; pesan berikutnya Anda mencoba model utama terlebih dahulu lagi.

* **Scope**: [`Any file`](#scopes)
* **Type**: array alias model atau ID; `"default"` berkembang ke model default
* **Default**: tidak diatur, jadi permintaan yang gagal tidak dicoba ulang pada model lain
* **Per-session overrides**: `--fallback-model` mengambil alih kunci ini untuk satu sesi

Contoh ini mencoba Sonnet 5 terlebih dahulu, kemudian Haiku 4.5, ketika model utama Anda gagal:

```json settings.json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

Tidak seperti sebagian besar pengaturan array, kunci ini tidak menggabungkan di seluruh file pengaturan: file dengan preseden tertinggi yang mendefinisikannya memasok seluruh rantai. Jika file proyek Anda menetapkan `["claude-sonnet-5"]` dan file pengguna Anda menetapkan `["claude-haiku-4-5"]`, rantainya adalah `["claude-sonnet-5"]` saja. Claude Code menyimpan paling banyak tiga model yang berbeda yang diizinkan dari daftar dan mengabaikan sisanya. Lihat [Rantai model fallback](/docs/id/model-config#fallback-model-chains).

<h3 id="fastmode">
  `fastMode`
</h3>

Hidupkan [fast mode](/docs/id/fast-mode) untuk sesi di mana tersedia, untuk pekerjaan interaktif seperti iterasi cepat atau debugging langsung di mana Anda menginginkan kecepatan dengan biaya lebih tinggi per token. Anda biasanya tidak mengedit kunci ini secara manual: menjalankan `/fast` menulis `fastMode: true` ke `~/.claude/settings.json`, dan menjalankannya lagi untuk mematikan fast mode menghapus kunci. Fast mode hanya berjalan di Opus 5.5, Opus 5, dan Opus 4.8: menghidupkannya dari model lain beralih Anda ke Opus, dan beralih ke model yang tidak didukung mematikannya. Lihat [Beralih model saat fast mode aktif](/docs/id/fast-mode#switch-models-while-fast-mode-is-on).

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code menghidupkan fast mode untuk sesi di mana tersedia
  * `false`: fast mode tetap mati
* **Default**: tidak diatur, jadi fast mode mati
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/id/env-vars) mematikan fast mode untuk satu sesi, dan kunci ini tidak dapat menghidupkannya kembali

```json settings.json theme={null}
{
  "fastMode": true
}
```

<h3 id="fastmodepersessionoptin">
  `fastModePerSessionOptIn`
</h3>

Biasanya, menjalankan `/fast` menyimpan [`fastMode`](#fastmode) ke pengaturan pengguna seseorang, jadi fast mode aktif di awal setiap sesi kemudian. Atur kunci ini ke `true` untuk menghentikan itu: `fastMode: true` yang disimpan tidak lagi menghidupkan fast mode di awal sesi, dan setiap orang harus menjalankan `/fast` di setiap sesi yang mereka inginkan. Claude Code meninggalkan kunci `fastMode` di file mereka, jadi mematikan kunci ini mengembalikan perilaku lama.

Pemilik pada paket Team atau Enterprise dapat menerapkannya di seluruh organisasi melalui [pengaturan terkelola server](/docs/id/server-managed-settings). Ketika pengaturan terkelola menetapkan kunci, `/fast on` ditolak di luar sesi terminal interaktif dan melaporkan bahwa organisasi Anda telah menonaktifkan fast mode. Itu mencakup [mode non-interaktif](/docs/id/headless), [ekstensi VS Code](/docs/id/vs-code), dan [sesi cloud](/docs/id/claude-code-on-the-web).

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: `fastMode: true` yang disimpan tidak lagi menghidupkan fast mode di awal sesi, jadi setiap orang menjalankan `/fast` di setiap sesi yang mereka inginkan; `fastMode: true` yang dilewatkan dengan `--settings` masih dihitung untuk sesi itu kecuali pengaturan terkelola menetapkan kunci ini
  * `false`: `fastMode: true` yang disimpan menghidupkan fast mode di awal setiap sesi kemudian
* **Default**: `false`

```json settings.json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Lihat [Memerlukan opt-in per sesi](/docs/id/fast-mode#require-per-session-opt-in).

<h3 id="language">
  `language`
</h3>

Buat Claude merespons dalam bahasa selain Inggris secara default. Tidak ada daftar tetap untuk respons: Claude Code menambahkan nilai secara verbatim ke prompt sistem sebagai instruksi untuk selalu merespons dalam bahasa itu, jadi nama bahasa apa pun yang dapat dibaca Claude berfungsi. Claude Code tidak memeriksa nilai, jadi nama yang salah eja mencapai Claude seperti yang ditulis daripada menghasilkan kesalahan. Nilai yang sama menetapkan bahasa untuk [voice dictation](/docs/id/voice-dictation#change-the-dictation-language), yang memiliki daftar tetap [bahasa dictation yang didukung](/docs/id/voice-dictation#change-the-dictation-language), dan untuk judul sesi yang dibuat secara otomatis.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, nama bahasa apa pun, seperti `"japanese"`, `"spanish"`, atau `"french"`; Claude Code tidak memvalidasinya
* **Default**: tidak diatur; judul sesi kemudian cocok dengan bahasa percakapan Anda

```json settings.json theme={null}
{
  "language": "japanese"
}
```

<h3 id="maxeffortlevel">
  `maxEffortLevel`
</h3>

Batasi [effort level](/docs/id/model-config#adjust-effort-level) yang dapat digunakan sesi, meninggalkan level yang lebih rendah tersedia. Level yang lebih tinggi apa pun berjalan pada batas sebagai gantinya, termasuk dari `/effort`, pemilih `/model`, `--effort`, [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/id/env-vars), frontmatter `effort` skill atau subagent, atau default model itu sendiri. Claude Code menerapkan batas itu sendiri sebelum setiap permintaan, jadi itu berlaku pada setiap penyedia, termasuk Amazon Bedrock, Agent Platform Google Cloud, dan Microsoft Foundry. Memerlukan Claude Code v2.1.267 atau lebih baru.

* **Scope**: [`Any file`](#scopes). Terapkan dalam pengaturan terkelola untuk memberlakukannya untuk organisasi. Ketika beberapa scope menetapkan batas, yang terendah berlaku, jadi batas yang ditetapkan dalam satu scope tidak dapat dinaikkan dari scope lain
* **Type**: string, salah satu dari `"low"`, `"medium"`, `"high"`, `"xhigh"`, atau `"max"`. Nilai `"max"` tidak menetapkan batas
* **Default**: tidak diatur, jadi tidak ada batas yang berlaku
* **Effect on ultracode**: batas di bawah `xhigh` membuat [ultracode](#ultracode) tidak tersedia pada model yang batas berlaku
* **Per-model caps**: tambahkan `maxEffortLevel` ke entri [`modelSettings`](#modelsettings) model. Entri itu menggantikan kunci ini untuk model saja dalam sumber pengaturan yang menetapkan keduanya, seperti pengaturan pengguna Anda atau satu [sumber terkelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources). Atur `"max"` di sana untuk mengecualikan model dari batas sumber itu; Claude Code masih menerapkan batas dari sumber lain

Contoh ini membatasi setiap model pada `medium` dan mengecualikan Sonnet 4.6:

```json settings.json theme={null}
{
  "maxEffortLevel": "medium",
  "modelSettings": {
    "claude-sonnet-4-6": {
      "maxEffortLevel": "max"
    }
  }
}
```

Ketika organisasi Anda juga menetapkan [batas effort](/docs/id/model-config#organization-effort-limits) untuk model, batas yang lebih rendah dari dua batas berlaku.

<h3 id="model">
  `model`
</h3>

Atur model yang digunakan setiap sesi baru, jadi Anda tidak harus memilih satu dengan `/model` setiap kali. Menetapkannya di sini tidak menghentikan Anda dari beralih di tengah sesi. Jika admin Anda menetapkan [model default organisasi](/docs/id/model-config#organization-default-model) untuk mengesampingkan pilihan pengguna, Anda mendapatkan model itu bahkan ketika Anda menetapkan kunci ini dalam pengaturan pengguna, proyek, atau lokal.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, alias model atau ID model lengkap
* **Default**: tidak diatur, jadi Claude Code menggunakan model default akun Anda
* **Per-session overrides**: `--model` mengambil alih [`ANTHROPIC_MODEL`](/docs/id/env-vars), dan keduanya mengambil alih kunci ini untuk satu sesi, termasuk atas `model` terkelola; daftar [`availableModels`](#availablemodels) masih berlaku untuk pilihan

```json settings.json theme={null}
{
  "model": "claude-sonnet-5"
}
```

Nilai di sini mengalahkan [`ANTHROPIC_DEFAULT_MODEL`](/docs/id/model-config#set-a-default-model-for-new-sessions), yang Claude Code gunakan hanya ketika tidak ada yang lain memilih model.

<h3 id="modeloverrides">
  `modelOverrides`
</h3>

Petakan ID model Anthropic ke ID model khusus penyedia, seperti ARN profil inferensi Amazon Bedrock. Setiap entri pemilih model kemudian menggunakan nilai yang dipetakan saat memanggil API penyedia. Administrator menggunakan ini pada [Amazon Bedrock, Agent Platform Google Cloud, dan Microsoft Foundry](/docs/id/model-config#override-model-ids-per-version) untuk merutekan setiap versi model ke profil inferensi, nama versi, atau deployment tertentu untuk tata kelola, alokasi biaya, atau perutean regional.

* **Scope**: [`Any file`](#scopes)
* **Type**: object memetakan ID model ke ID model penyedia
* **Default**: tidak diatur

Contoh ini merutekan setiap panggilan untuk Opus 4.6 ke profil inferensi Bedrock bernama:

```json settings.json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-6": "arn:aws:bedrock:us-east-1:123456789012:inference-profile/example"
  }
}
```

Lihat [Ganti ID model per versi](/docs/id/model-config#override-model-ids-per-version).

<h3 id="modelpicker">
  `modelPicker`
</h3>

Daftar model yang ditawarkan pemilih `/model`, dalam urutan yang Anda tulis dan di bawah label yang Anda pilih, jadi pemilih mencantumkan model yang dijalankan organisasi Anda, setelah lineup bawaan atau sebagai gantinya. `model` setiap baris diambil secara verbatim, jadi menerima apa pun yang diterima `--model`: alias seperti `opus`, ID model Anthropic, atau ID format penyedia untuk Amazon Bedrock, Agent Platform Google Cloud, Microsoft Foundry, atau gateway LLM. Memerlukan Claude Code v2.1.242 atau lebih baru.

* **Scope**: [`User or managed`](#scopes). Claude Code membaca kunci dari pengaturan terkelola, `--settings`, dan pengaturan pengguna, dan mengabaikannya dalam pengaturan proyek dan lokal jadi repositori yang Anda klon tidak dapat memberi label ulang pemilih. Yang tertinggi dari ketiga itu yang menetapkan kunci memasok seluruh lineup, dan Claude Code tidak pernah menggabungkan lineup dari dua sumber.
* **Type**: object dengan array `options` dari baris dan Boolean `replaceBuiltInOptions` opsional
* **Default**: tidak diatur, jadi pemilih menampilkan lineup bawaan

Contoh ini menambahkan dua deployment Bedrock setelah lineup bawaan, di bawah nama yang dikenali tim Anda:

```json managed-settings.json theme={null}
{
  "modelPicker": {
    "options": [
      { "model": "us.anthropic.claude-opus-4-8", "label": "Opus (production)" },
      {
        "model": "us.anthropic.claude-sonnet-4-6",
        "label": "Sonnet (production)",
        "description": "Day-to-day work"
      }
    ]
  }
}
```

<span id="modelpicker-options" />

<span id="modelpicker-replacebuiltinoptions" />

<h4 id="fields-for-modelpicker">
  Fields untuk `modelPicker`
</h4>

Kunci mengambil dua field, satu untuk baris itu sendiri dan satu untuk apakah mereka menggantikan lineup bawaan atau menambahnya.

| Field                   | Type                                                                                             | Apa yang dilakukannya                                                                                                                                                                                                                                                                 |
| :---------------------- | :----------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `options`               | array baris, masing-masing dengan `model` yang diperlukan dan `label` dan `description` opsional | Baris yang ditampilkan pemilih, dalam urutan ini, kecuali baris yang digelapkan bergerak ke bawah. Tanpa `label`, Claude Code memberi judul baris dengan nama bawaan untuk model yang diketahuinya, atau ID model sebaliknya, dan tanpa `description` itu menulis baris kedua generik |
| `replaceBuiltInOptions` | Boolean, default `false`                                                                         | Atur ke `true` untuk menampilkan hanya baris ini, **Default**, dan baris untuk model yang sudah digunakan sesi. Biarkan tidak diatur untuk menambahkan baris ini setelah lineup bawaan                                                                                                |

Dengan `replaceBuiltInOptions` aktif, Claude Code menyembunyikan setiap baris lain: lineup bawaan, baris yang ditambahkannya untuk entri [`availableModels`](#availablemodels), model yang ditemukan [gateway discovery](/docs/id/llm-gateway-protocol#model-discovery), dan [`ANTHROPIC_CUSTOM_MODEL_OPTION`](/docs/id/model-config#add-a-custom-model-option). Dengan itu mati, Claude Code melewati model yang tercantum yang sudah dicakup lineup bawaan. Label mengubah apa yang ditampilkan pemilih, bukan model mana yang dijalankan Claude Code.

Daftar allowlist [`availableModels`](#availablemodels) masih berlaku untuk baris ini. Sebelum Anda menambahkan model yang tercantum ke allowlist, baca [Merge behavior](/docs/id/model-config#merge-behavior): ID model tertentu mempersempit entri wildcard keluarganya. Claude Code juga memeriksa setiap baris terhadap sesi sebelum menampilkan pemilih:

* **Dropped**: baris yang tidak dapat disajikan Claude Code, seperti model yang pensiun atau model yang tidak memiliki akses organisasi Anda
* **Grayed out**: baris yang tidak dapat Anda pilih belum, ditampilkan dengan alasannya
* **No row survives**: Claude Code menyimpan lineup bawaan, disaring oleh allowlist seperti biasa

Claude Code menjatuhkan baris yang tidak dapat diuraikan dan menyimpan sisanya. Lihat [Perbaiki file pengaturan yang rusak](/docs/id/settings#fix-a-broken-settings-file).

<h3 id="modelpricing">
  `modelPricing`
</h3>

Laporkan pengeluaran pada tarif yang dibayar organisasi Anda daripada harga daftar. Atur ketika organisasi Anda memiliki tarif kontrak, jadi angka dolar yang dilihat pengembang cocok dengan tagihan Anda. Claude Code menerapkan tarif di `/usage`, [baris status](/docs/id/statusline), `total_cost_usd` Agent SDK, batas [`--max-budget-usd`](/docs/id/cli-reference), dan [OpenTelemetry](/docs/id/monitoring-usage) metrik biaya dan acara. Anda memasok tarif: Claude Code tidak membacanya dari kontrak atau Claude Console Anda. Memerlukan Claude Code v2.1.242 atau lebih baru.

* **Scope**: [`Managed`](#scopes). Terapkan kunci melalui pengaturan terkelola server, kebijakan MDM, file `managed-settings.json`, atau [helper kebijakan](/docs/id/managed-settings#compute-the-policy-with-a-helper-program). Claude Code mengabaikannya dalam pengaturan pengguna, proyek, dan lokal, di `--settings`, dan di Windows dalam [registri HKCU](/docs/id/managed-settings#where-each-mechanism-stores-the-policy) yang dapat ditulis pengguna. Dengan pengaturan terkelola server, setiap sesi melaporkan biaya pada harga daftar sampai [pengambilan pengaturan](/docs/id/server-managed-settings#fetch-and-caching-behavior) sesi itu telah mengkonfirmasi pengaturan. Aplikasi host yang menyematkan Claude Code dan menetapkan [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/id/env-vars) dapat memasok tabel miliknya sendiri melalui opsi SDK [`managedSettings`](/docs/id/agent-sdk/typescript#options), yang Claude Code gunakan hanya ketika tidak ada sumber terkelola yang menetapkan kunci dan hanya dalam Claude Code v2.1.246 atau lebih baru.
* **Type**: object dengan `multiplier` opsional dan peta `overrides` opsional
* **Default**: tidak diatur, jadi Claude Code melaporkan harga daftar kecuali aplikasi host memasok tabel

Atur `multiplier` saja untuk diskon datar atau markup, `overrides` saja untuk tarif per model, atau keduanya.

Contoh ini menetapkan tarif kontrak untuk Sonnet 4.6 dan kemudian mengurangi setiap angka, baris Sonnet disertakan, sebesar 15%:

```json managed-settings.json theme={null}
{
  "modelPricing": {
    "multiplier": 0.85,
    "overrides": {
      "claude-sonnet-4-6": {
        "input": 2.4,
        "output": 12,
        "cacheRead": 0.24,
        "cacheWrite": 3
      }
    }
  }
}
```

Atur `multiplier` di atas 1, hingga 10, untuk menandai setiap angka. Markup memerlukan Claude Code v2.1.271 atau lebih baru. Versi sebelumnya mengabaikan `multiplier` di atas 1 dengan peringatan dan menyimpan sisa pengaturan.

Untuk langkah-langkahnya, termasuk cara mengkonfirmasi tarif berlaku, lihat [Laporkan pengeluaran pada tarif kontrak Anda](/docs/id/costs#report-spend-at-your-contracted-rates).

<span id="modelpricing-multiplier" />

<span id="modelpricing-overrides" />

<h4 id="fields-for-modelpricing">
  Fields untuk `modelPricing`
</h4>

| Field        | Type                                                                                                               | Apa yang dilakukannya                                                                                                                                                                                                        |
| :----------- | :----------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `multiplier` | angka lebih besar dari 0 dan paling banyak 10                                                                      | Menskalakan setiap biaya yang dihitung Claude Code, apakah baris `overrides` mencakupnya atau tidak. Di bawah 1 adalah diskon, di atas 1 adalah markup                                                                       |
| `overrides`  | peta ID model ke objek tarif dengan `input`, `output`, `cacheRead`, dan `cacheWrite`, masing-masing 0 hingga 10000 | Tarif USD-per-juta-token untuk model itu, keempat diperlukan. `cacheWrite` mencakup penulisan cache lima menit dan satu jam. Lihat [Model mana yang berlaku baris modelPricing](#which-models-a-modelpricing-row-applies-to) |

Claude Code menggunakan tarif baris persis seperti yang Anda tulis, tanpa menambahkan surcharge fast-mode atau [tarif inferensi hanya-AS](https://platform.claude.com/docs/en/about-claude/pricing). Jika Anda juga menetapkan `multiplier`, Claude Code menerapkannya di atas tarif baris. Claude Code menjatuhkan baris dengan tarif yang tidak dapat diuraikan, atau `multiplier` yang tidak dapat diuraikan, dan menyimpan sisanya; lihat [Perbaiki file pengaturan yang rusak](/docs/id/settings#fix-a-broken-settings-file).

<h4 id="which-models-a-modelpricing-row-applies-to">
  Model mana yang berlaku baris `modelPricing`
</h4>

Claude Code memutuskan model mana yang berlaku baris dari kunci baris:

* **ID model bawaan**: kunci yang digunakan Claude Code itu sendiri untuk model bawaan, apakah kunci itu ID model itu sendiri, seperti `claude-sonnet-4-6`, atau ID Bedrock, Agent Platform, atau Foundry-nya. Claude Code menerapkan baris ke setiap ID snapshot bertanggal dan ID khusus penyedia dari model itu.
* **Kunci lain apa pun**: kunci yang bukan ID model bawaan, seperti alias model gateway. Claude Code menerapkan baris ke ID itu saja. Ketika ID model cocok dengan salah satu kunci Anda persis dan juga jatuh di bawah baris yang dikunci oleh ID model bawaan, Claude Code menggunakan kecocokan persis.
* **Profil inferensi aplikasi Bedrock**: setelah Claude Code telah menyelesaikan profil ke model yang dirutnya, melalui peta [`modelOverrides`](#modeloverrides) Anda atau pencarian [`bedrock:GetInferenceProfile`](/docs/id/amazon-bedrock#iam-configuration), Claude Code menerapkan baris model itu ke profil.

<h3 id="modelsettings">
  `modelSettings`
</h3>

Simpan [effort level](/docs/id/model-config#adjust-effort-level) untuk setiap model yang Anda gunakan. Memerlukan Claude Code v2.1.251 atau lebih baru.

Dalam sesi interaktif di mesin Anda, ketika Anda menyimpan `low`, `medium`, `high`, atau `xhigh` sebagai default Anda dengan `/effort` atau slider effort pemilih `/model`, Claude Code menulis level itu di sini di bawah model yang Anda gunakan, jadi Anda jarang mengedit kunci ini sendiri. Ketika Anda memilih salah satu level itu di [pemilih model ekstensi VS Code](/docs/id/vs-code#use-the-prompt-box), Claude Code menyimpannya di sini dengan cara yang sama. Entri [`effortLevel`](#effortlevel) mencantumkan sesi di mana `/effort` hanya berlaku untuk sesi itu.

Edit kunci secara manual untuk mengubah atau menghapus level yang Anda simpan.

`effortLevel` model di sini mengambil preseden atas [`effortLevel`](#effortlevel) tingkat atas dalam file pengaturan yang sama. Di seluruh file, Claude Code menyelesaikan setiap model secara terpisah: [file pengaturan](/docs/id/settings#settings-precedence) dengan preseden tertinggi yang menetapkan `effortLevel` untuk model itu atau `effortLevel` tingkat atas yang [berlaku untuk model itu](#effortlevel) memutuskan, jadi `effortLevel` dalam pengaturan terkelola mengalahkan level yang Anda simpan dalam pengaturan pengguna. [Sesuaikan effort level](/docs/id/model-config#adjust-effort-level) mencantumkan apa lagi yang dapat mengesampingkan level yang disimpan, seperti `--effort` saat peluncuran.

Untuk membatasi effort satu model daripada menetapkan levelnya, tambahkan field [`maxEffortLevel`](#maxeffortlevel) ke entri model itu. Field memerlukan Claude Code v2.1.267 atau lebih baru.

* **Scope**: [`Any file`](#scopes)
* **Type**: object memetakan nama model ke object dengan field `effortLevel`, salah satu dari `"low"`, `"medium"`, `"high"`, atau `"xhigh"`, field [`maxEffortLevel`](#maxeffortlevel), atau keduanya
* **Default**: tidak diatur

Claude Code menulis setiap entri di bawah nama kanonik model, seperti `claude-opus-5-5`, dan mencocokkan alias model itu, bertanggal, `[1m]`, dan ID khusus penyedia yang dikenali ke entri yang sama.

Contoh ini menyimpan Opus 5.5 pada `high` sementara model lain menggunakan level yang disimpan atau default mereka sendiri:

```json settings.json theme={null}
{
  "modelSettings": {
    "claude-opus-5-5": {
      "effortLevel": "high"
    }
  }
}
```

Jalankan `/effort auto` untuk menghapus level yang disimpan untuk model yang Anda gunakan. Claude Code meninggalkan entri lain dan `effortLevel` tingkat atas apa pun di tempat.

<h3 id="outputstyle">
  `outputStyle`
</h3>

Pilih [output style](/docs/id/output-styles) berdasarkan nama. Output style adalah set instruksi yang disimpan yang mengubah peran, nada, dan format output Claude, seperti style Explanatory dan Learning bawaan atau yang Anda tulis sendiri.

Jika Anda mengubah kunci ini selama sesi, Claude menggunakan style baru mulai dari pesan Anda berikutnya. Untuk apa biaya pesan itu dalam prompt caching, lihat [Mengubah output style](/docs/id/prompt-caching#changing-output-style). Sebelum v2.1.251, edit hanya berlaku setelah Anda menjalankan `/clear` atau memulai sesi baru.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, nama [bawaan](/docs/id/output-styles#built-in-output-styles) atau [custom](/docs/id/output-styles#create-a-custom-output-style) output style
* **Default**: tidak diatur, jadi Claude Code menggunakan style default

Contoh ini memilih style Explanatory bawaan, yang menambahkan wawasan pendidikan antara tugas:

```json settings.json theme={null}
{
  "outputStyle": "Explanatory"
}
```

<h3 id="promptcachettl">
  `promptCacheTtl`
</h3>

Pilih berapa lama [prompt cache](/docs/id/prompt-caching) menyimpan percakapan utama. Kunci ini berlaku untuk giliran interaktif, `-p`, dan Agent SDK Anda, bersama dengan helper yang dijalankan Claude Code secara inline dengannya. Lifetime satu jam menjaga cache tetap hangat di seluruh jeda yang lebih lama, dan API [menagih setiap penulisan cache pada tarif yang lebih tinggi](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) daripada lifetime lima menit. Memerlukan Claude Code v2.1.242 atau lebih baru.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, salah satu dari:
  * `"5m"`: cache menyimpan selama lima menit
  * `"1h"`: cache menyimpan selama satu jam
* **Default**: tidak diatur, jadi setiap permintaan percakapan utama mendapat [lifetime defaultnya](/docs/id/prompt-caching#which-ttl-each-request-gets)
* **Per-session overrides**: [`FORCE_PROMPT_CACHING_5M`](/docs/id/env-vars) mengambil alih segalanya, kemudian [`CLAUDE_CODE_PROMPT_CACHE_TTL`](/docs/id/env-vars), kemudian kunci ini, dan terakhir [`ENABLE_PROMPT_CACHING_1H`](/docs/id/env-vars)

Contoh ini menyimpan percakapan utama pada lifetime satu jam dan meninggalkan subagents pada lima menit:

```json settings.json theme={null}
{
  "promptCacheTtl": "1h",
  "subagentPromptCacheTtl": "5m"
}
```

Untuk apa biaya setiap lifetime, lihat [Cache lifetime](/docs/id/prompt-caching#cache-lifetime).

<h3 id="showthinkingsummaries">
  `showThinkingSummaries`
</h3>

Lihat ringkasan [extended thinking](/docs/id/model-config#extended-thinking) Claude dalam sesi interaktif. Atur jika Anda menginginkan ringkasan lengkap ketika Anda memperluas thinking dengan `Ctrl+O`. Ketika tidak diatur atau `false`, Anthropic API menyensor blok thinking dan Claude Code menampilkan stub yang runtuh; penyedia pihak ketiga tidak menyensor.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Anda melihat ringkasan thinking lengkap ketika Anda memperluas thinking dengan `Ctrl+O`
  * `false`: Anthropic API menyensor blok thinking dan Claude Code menampilkan stub yang runtuh
* **Default**: `false`

```json settings.json theme={null}
{
  "showThinkingSummaries": true
}
```

Penyensoran hanya mengubah apa yang Anda lihat, bukan apa yang dihasilkan model. Untuk mengurangi pengeluaran thinking, [turunkan anggaran atau matikan thinking](/docs/id/model-config#extended-thinking) sebagai gantinya.

<h3 id="subagentpromptcachettl">
  `subagentPromptCacheTtl`
</h3>

Pilih berapa lama [prompt cache](/docs/id/prompt-caching) menyimpan permintaan yang dibuat Claude Code di luar percakapan utama. Kunci ini berlaku untuk [subagents](/docs/id/sub-agents), [workflows](/docs/id/workflows), dan permintaan background dan helper Claude Code sendiri, seperti compaction dan judul sesi. Lifetime satu jam menjaga cache tetap hangat di seluruh jeda yang lebih lama, dan API [menagih setiap penulisan cache pada tarif yang lebih tinggi](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) daripada lifetime lima menit. Memerlukan Claude Code v2.1.242 atau lebih baru.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, salah satu dari:
  * `"5m"`: cache menyimpan selama lima menit
  * `"1h"`: cache menyimpan selama satu jam
* **Default**: tidak diatur, jadi setiap permintaan ini mendapat [lifetime defaultnya](/docs/id/prompt-caching#which-ttl-each-request-gets)
* **Per-session overrides**: [`FORCE_PROMPT_CACHING_5M`](/docs/id/env-vars) mengambil alih segalanya, kemudian [`CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`](/docs/id/env-vars), kemudian kunci ini, kemudian [`ENABLE_PROMPT_CACHING_1H`](/docs/id/env-vars), yang meminta lifetime satu jam pada setiap permintaan. Untuk di mana nilai frontmatter subagent sendiri berada, lihat [Pilih TTL sendiri](/docs/id/prompt-caching#choose-the-ttl-yourself)

Contoh ini memberikan subagents dan permintaan lain di luar percakapan utama lifetime satu jam:

```json settings.json theme={null}
{
  "subagentPromptCacheTtl": "1h"
}
```

Kunci ini mencakup permintaan yang tidak dicakup [`promptCacheTtl`](#promptcachettl), jadi atur keduanya untuk memilih lifetime untuk setiap permintaan yang dibuat Claude Code. Untuk bagaimana cache subagent berbeda dari percakapan utama, lihat [Subagents dan cache](/docs/id/prompt-caching#subagents-and-the-cache).

<h3 id="switchmodelsonflag">
  `switchModelsOnFlag`
</h3>

Pilih apa yang terjadi ketika [pengklasifikasi keamanan menandai permintaan](/docs/id/model-config#automatic-model-fallback): beralih ke model fallback dan lanjutkan, atau jeda sehingga Anda dapat memilih antara beralih dan mengedit prompt.

* **Scope**: [`Any file`](#scopes). Muncul di `/config` sebagai **Switch models when a message is flagged**.
* **Type**: Boolean
  * `true`: Claude Code beralih ke model fallback dan lanjutkan
  * `false`: dalam sesi interaktif Claude Code berhenti sehingga Anda dapat memilih antara beralih dan mengedit prompt; di mana tidak ada dialog yang dapat ditampilkan, seperti jalankan `-p`, permintaan yang ditandai berakhir sebagai kesalahan
* **Default**: `true`, beralih secara otomatis

```json settings.json theme={null}
{
  "switchModelsOnFlag": false
}
```

Lihat [Tanya sebelum beralih](/docs/id/model-config#ask-before-switching).

<h3 id="ultracode">
  `ultracode`
</h3>

Mulai sesi dengan [ultracode](/docs/id/workflows#let-claude-decide-with-ultracode) aktif. Dengan itu aktif, Claude merencanakan workflow untuk setiap tugas substansial daripada menunggu Anda meminta. Claude merencanakan workflow hanya ketika [dynamic workflows](/docs/id/workflows) diaktifkan untuk Anda, model Anda mendukung effort `xhigh`, dan tidak ada [effort cap](/docs/id/model-config#organization-effort-limits) di bawah `xhigh` yang berlaku. Bagaimanapun, `ultracode: true` menjalankan sesi pada effort `xhigh`, atau pada batas ketika batas effort lebih rendah. Claude Code membaca kunci ini tetapi tidak pernah menulisnya: `/effort ultracode` menghidupkan ultracode untuk sesi saat ini saja.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: sesi dimulai pada effort `xhigh`, dengan ultracode aktif ketika dynamic workflows diaktifkan untuk Anda, model Anda mendukung `xhigh`, dan tidak ada effort cap di bawah `xhigh`
  * `false`: sesi dimulai dengan ultracode mati
* **Default**: tidak diatur, jadi ultracode mati
* **Per-session overrides**: `/effort ultracode` menghidupkan ultracode untuk satu sesi tanpa kunci ini. Begitu juga `--effort ultracode`, yang memerlukan Claude Code v2.1.203 atau lebih baru

```json settings.json theme={null}
{
  "ultracode": true
}
```

Ultracode menjalankan sesi pada effort `xhigh` dan mengambil preseden atas `effortLevel` dan entri [`modelSettings`](#modelsettings). Jika [effort cap](/docs/id/model-config#organization-effort-limits) di bawah `xhigh` berlaku untuk model, seperti pengaturan [`maxEffortLevel`](#maxeffortlevel), sesi berjalan pada batas sebagai gantinya dan ultracode tetap mati. Claude kemudian tidak merencanakan workflow dengan sendirinya, dan `/effort` tidak menawarkan `ultracode`. Permintaan kontrol `apply_flag_settings` Agent SDK juga menerima kunci.

<h2 id="permission-settings">
  Pengaturan izin
</h2>

Tentukan apa yang dapat dilakukan Claude tanpa bertanya, mode izin mana yang dimulai sesi, dan apa yang diizinkan pengklasifikasi mode otomatis. Untuk sintaks aturan dan model izin, lihat [Konfigurasi izin](/docs/id/permissions).

<h3 id="allowmanagedpermissionrulesonly">
  `allowManagedPermissionRulesOnly`
</h3>

Jadikan pengaturan terkelola satu-satunya sumber pengaturan aturan izin. Claude Code kemudian mengabaikan aturan `allow`, `ask`, dan `deny` dalam file pengguna, proyek, lokal, dan `--settings`, mengabaikan `--allowedTools`, menyembunyikan pilihan selalu-izinkan dalam prompt izin, dan berhenti menyimpan aturan baru.

Ketika [pengaturan induk dari host penyematan](/docs/id/managed-settings#let-an-embedding-host-add-policy) berlaku, Claude Code memperlakukan mereka sebagai bagian dari tingkat terkelola. Claude Code menghapus aturan `allow` dan `additionalDirectories` mereka, dan mempertahankan aturan `deny` dan `ask` mereka kecuali aturan `Read` dan `Edit` yang polanya dimulai dengan `!`. Host tidak dapat mengukir jalur dari aturan terkelola dengan aturan `!`, terlepas dari apakah Anda menetapkan kunci ini.

Aturan `--disallowedTools` dan aturan `deny` dan `ask` sesi saat ini masih berlaku, termasuk setelah Claude Code memuat ulang pengaturan di tengah sesi. Mereka hanya membatasi, jadi mereka tidak dapat memperluas apa yang diberikan aturan terkelola. Sebelum v2.1.257, Claude Code menghapus aturan baris perintah dan sesi tersebut pada pemuatan ulang pengaturan pertama.

Untuk apa yang dapat diukir pola `!` dalam aturan `--disallowedTools` atau sesi, lihat [aturan Read dan Edit](/docs/id/permissions#read-and-edit).

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: pengaturan terkelola menjadi satu-satunya sumber pengaturan aturan izin
  * `false`: Claude Code menerapkan aturan izin dari file pengguna, proyek, lokal, dan `--settings` selain yang terkelola
* **Default**: tidak diatur, jadi Claude Code menerapkan aturan izin dari pengaturan pengguna, proyek, dan lokal serta dari `--settings`, selain yang terkelola

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true
}
```

Kunci ini tidak mengunci daftar server MCP yang diizinkan; untuk itu, atur [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly). Lihat [Pengaturan hanya terkelola](/docs/id/managed-settings#managed-only-settings).

<h3 id="automode">
  `autoMode`
</h3>

Tambahkan aturan Anda sendiri ke apa yang diblokir dan diizinkan oleh pengklasifikasi [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode). Gunakan untuk memberi tahu pengklasifikasi repo, bucket, dan domain mana yang dipercaya organisasi Anda, sehingga berhenti memblokir operasi internal rutin. Pengklasifikasi dilengkapi dengan [aturan izin dan blokir bawaan](/docs/id/auto-mode-config#inspect-the-defaults-and-your-effective-config). Sertakan string literal `"$defaults"` dalam array untuk mempertahankan aturan bawaan tersebut pada posisi itu dan tambahkan milik Anda di sekitarnya; tinggalkan untuk menggantinya dengan milik Anda.

* **Scope**: [`User or managed`](#scopes)
* **Type**: objek dengan array `environment`, `allow`, `soft_deny`, dan `hard_deny` dari aturan prosa, ditambah Boolean [`classifyAllShell`](#automode-classifyallshell)
* **Default**: tidak diatur, jadi pengklasifikasi hanya menggunakan [aturan bawaannya](/docs/id/auto-mode-config#inspect-the-defaults-and-your-effective-config)

Contoh ini mempertahankan aturan `soft_deny` bawaan, melalui `"$defaults"`, dan menambahkan satu lagi yang memblokir `terraform apply`:

```json settings.json theme={null}
{
  "autoMode": {
    "soft_deny": ["$defaults", "Never run terraform apply"]
  }
}
```

Ketika lebih dari satu file tersebut menetapkan array yang sama, Claude Code menggabungkan entri. Untuk format aturan dan bagaimana setiap array diterapkan, lihat [Konfigurasi mode otomatis](/docs/id/auto-mode-config).

<h3 id="automode-classifyallshell">
  `autoMode.classifyAllShell`
</h3>

Kirim setiap perintah Bash dan PowerShell melalui pengklasifikasi mode otomatis saat mode otomatis aktif. Secara default, mode otomatis hanya menangguhkan aturan izin yang dapat menjalankan kode arbitrer: aturan lebar alat dan wildcard seperti `Bash(*)`, dan awalan interpreter atau shell-wrapper seperti `Bash(python *)`. Perintah yang cocok dengan aturan izin lain, seperti `Bash(npm test)`, melewati pengklasifikasi kecuali membawa [domain yang diizinkan per-perintah](/docs/id/sandboxing#per-command-allowed-domains-in-auto-mode). Ketika melewati, argumen destruktif yang tidak diantisipasi awalan aturan dapat lolos tanpa terlihat. Menetapkan kunci ini menangguhkan setiap aturan izin shell untuk sesi sehingga pengklasifikasi melihat setiap perintah. Memerlukan Claude Code v2.1.193 atau lebih baru.

* **Scope**: [`User or managed`](#scopes). Baca di mana pun [`autoMode`](#automode) dibaca.
* **Type**: Boolean
  * `true`: saat mode otomatis aktif, Claude Code mengirim setiap perintah Bash dan PowerShell melalui pengklasifikasi dan menangguhkan aturan izin shell Anda; di luar mode otomatis aturan masih berlaku
  * `false`: mode otomatis hanya menangguhkan aturan izin yang dapat menjalankan kode arbitrer, seperti `Bash(*)` dan `Bash(python *)`; perintah yang cocok dengan aturan izin lain melewati pengklasifikasi kecuali membawa [domain yang diizinkan per-perintah](/docs/id/sandboxing#per-command-allowed-domains-in-auto-mode), dan setiap perintah shell lainnya melaluinya
* **Default**: `false`

```json settings.json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Lihat [Arahkan semua perintah shell melalui pengklasifikasi](/docs/id/auto-mode-config#route-all-shell-commands-through-the-classifier). Memerlukan Claude Code v2.1.193 atau lebih baru.

<h3 id="disableautomode">
  `disableAutoMode`
</h3>

Hapus [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) dari siklus `Shift+Tab`. Setiap sesi yang sebaliknya akan [dimulai dalam mode otomatis](/docs/id/permission-modes#which-mode-a-session-starts-in), baik dari `--permission-mode auto`, file pengaturan, atau default bawaan, dimulai dalam `default` sebagai gantinya. Administrator menetapkannya dalam pengaturan terkelola untuk mencegah pengembang di organisasi mereka menggunakan mode otomatis.

* **Scope**: [`Any file`](#scopes). Paling berguna dalam [pengaturan terkelola](/docs/id/managed-settings), di mana pengguna tidak dapat menggantinya. Juga diterima di bawah `permissions` sebagai `permissions.disableAutoMode`.
* **Type**: string `"disable"`
* **Default**: tidak diatur

```json settings.json theme={null}
{
  "disableAutoMode": "disable"
}
```

<h3 id="permissions">
  `permissions`
</h3>

Kontrol alat mana yang dapat digunakan Claude tanpa bertanya, alat mana yang selalu meminta, dan alat mana yang diblokir, serta atur [mode izin](/docs/id/permission-modes) yang dimulai sesi. Setiap kunci `permissions.*` di bawah bersarang di bawah objek ini.

* **Scope**: [`Any file`](#scopes)
* **Type**: objek dengan `allow`, `ask`, `deny`, `additionalDirectories`, `blockReadsOutsideWorkingDirectories`, `defaultMode`, `disableBypassPermissionsMode`, dan `disableAutoMode`
* **Default**: tidak diatur

Contoh ini menyetujui perintah `npm run` tanpa bertanya, meminta sebelum `git push`, memblokir pembacaan `.env`, dan memulai sesi dalam `acceptEdits`:

```json settings.json theme={null}
{
  "permissions": {
    "allow": ["Bash(npm run *)"],
    "ask": ["Bash(git push *)"],
    "deny": ["Read(./.env)"],
    "defaultMode": "acceptEdits"
  }
}
```

Tiga array aturan berbagi satu sintaks; lihat [Sintaks aturan izin](#permission-rule-syntax) di bawah `permissions.allow`. Untuk bagaimana aturan izin dari file berbeda bergabung, lihat [bagaimana aturan izin bergabung di seluruh scope](/docs/id/permissions#settings-precedence); untuk bagaimana kunci pengaturan secara umum bergabung, lihat [Preseden pengaturan](/docs/id/settings#settings-precedence) pada panduan pengaturan.

<h3 id="useautomodeduringplan">
  `useAutoModeDuringPlan`
</h3>

Pilih apakah Claude Code menggunakan pengklasifikasi mode otomatis untuk meninjau perintah shell dalam mode rencana. Dengan default `true`, pengklasifikasi meninjau setiap perintah selama perencanaan ketika mode otomatis tersedia dan Anda tidak melihat prompt. Atur `false` untuk mendapatkan prompt izin untuk setiap perintah di luar set bawaan hanya-baca. Muncul dalam `/config` sebagai **Use auto mode during plan**.

* **Scope**: [`User, local, or managed`](#scopes). Repositori tidak dapat mematikannya untuk Anda.
* **Type**: Boolean
  * `true`: sama dengan tidak diatur; ketika mode otomatis tersedia, pengklasifikasi meninjau setiap perintah shell selama perencanaan alih-alih meminta Anda. `false` di salah satu file ini masih mematikannya
  * `false`: Anda mendapatkan prompt izin untuk setiap perintah di luar set bawaan hanya-baca
* **Default**: `true`

```json settings.json theme={null}
{
  "useAutoModeDuringPlan": false
}
```

<h3 id="permissions-allow">
  `permissions.allow`
</h3>

Daftar penggunaan alat yang disetujui Claude Code tanpa bertanya kepada Anda. Dalam aturan MCP, `*` hanya dapat muncul dalam nama alat setelah awalan `mcp__<server>__`, seperti `mcp__github__get_*`; tidak dapat muncul dalam nama server.

* **Scope**: [`Any file`](#scopes)
* **Type**: array string aturan izin
* **Default**: tidak diatur
* **Per-session overrides**: `--allowedTools` menambahkan aturan izin untuk satu sesi, dan aturan blokir dari file pengaturan apa pun masih memblokir alat yang dinamainya

Contoh ini menyetujui `git diff` dan memungkinkan Claude Code membaca `.zshrc` Anda tanpa bertanya:

```json settings.json theme={null}
{
  "permissions": {
    "allow": ["Bash(git diff *)", "Read(~/.zshrc)"]
  }
}
```

Claude Code menerapkan aturan `allow` dari `.claude/settings.json` proyek hanya setelah Anda menerima [dialog kepercayaan ruang kerja](/docs/id/permissions#project-allow-rules-and-workspace-trust) untuk folder itu.

<h4 id="permission-rule-syntax">
  Sintaks aturan izin
</h4>

Aturan izin mengikuti format `Tool` atau `Tool(specifier)`. Claude Code mengevaluasi aturan `deny` terlebih dahulu, kemudian `ask`, kemudian `allow`, dan kecocokan pertama memutuskan terlepas dari seberapa spesifik setiap aturan; lihat [urutan evaluasi aturan izin](/docs/id/permissions#manage-permissions).

Setiap baris menunjukkan satu bentuk aturan dan apa yang cocok.

| Rule                           | What it matches                  |
| :----------------------------- | :------------------------------- |
| `Bash`                         | Every Bash command               |
| `Bash(npm run *)`              | Commands starting with `npm run` |
| `Read(./.env)`                 | Reads of the `.env` file         |
| `WebFetch(domain:example.com)` | Fetch requests to example.com    |

Untuk sintaks aturan lengkap, termasuk perilaku wildcard, pola khusus alat untuk Read, Edit, WebFetch, MCP, dan aturan Agent, dan keterbatasan keamanan pola Bash, lihat [Sintaks aturan izin](/docs/id/permissions#permission-rule-syntax).

<h3 id="permissions-ask">
  `permissions.ask`
</h3>

Daftar penggunaan alat yang meminta Anda untuk konfirmasi bahkan dalam mode izin yang sebaliknya akan menyetujuinya, seperti `acceptEdits` atau `bypassPermissions`. Dalam mode `dontAsk` Claude Code menolak penggunaan alat yang cocok alih-alih meminta.

* **Scope**: [`Any file`](#scopes)
* **Type**: array string aturan izin
* **Default**: tidak diatur

```json settings.json theme={null}
{
  "permissions": {
    "ask": ["Bash(git push *)"]
  }
}
```

<span id="exclude-sensitive-files" />

<h3 id="permissions-deny">
  `permissions.deny`
</h3>

Daftar penggunaan alat yang diblokir Claude Code. Gunakan untuk file yang menyimpan kunci API, rahasia, atau nilai lingkungan: Claude Code mengecualikan file yang cocok dari penemuan file dan hasil pencarian, menolak pembacaan mereka, dan memblokir [alat Edit dan Write](/docs/id/permissions#read-and-edit) pada jalur yang cocok.

Aturan blokir Read dan Edit berlaku untuk alat file bawaan Claude, untuk perintah file yang dikenali Claude Code dalam Bash, seperti `cat`, `head`, `tail`, `sed`, dan `tee`, dan untuk target Bash [pengalihan](/docs/id/permissions#redirections) seperti `> file` dan `< file`; mereka tidak berlaku untuk perintah yang membaca file tanpa menamakannya, seperti `grep -r pattern .`, atau untuk subprocess arbitrer, jadi untuk penegakan tingkat OS [aktifkan sandbox](/docs/id/sandboxing).

* **Scope**: [`Any file`](#scopes)
* **Type**: array string aturan izin
* **Default**: tidak diatur
* **Per-session overrides**: `--disallowedTools` menambahkan aturan blokir untuk satu sesi bersama kunci ini

Contoh ini menolak pembacaan file `.env`, direktori `secrets`, dan file kredensial, serta memblokir perintah `curl`:

```json settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./.env)",
      "Read(./.env.*)",
      "Read(./secrets/**)",
      "Read(./config/credentials.json)",
      "Bash(curl *)"
    ]
  }
}
```

Nama alat menerima pola glob, jadi `"*"` memblokir setiap alat dan `"mcp__*"` memblokir setiap alat MCP. Claude Code mengabaikan aturan blokir untuk alat [`EndConversation`](/docs/id/tools-reference#endconversation-tool-behavior) selama alat lain masih tersedia untuk Claude. Aturan blokir `Bash` cocok dengan perintah seperti yang ditulis Claude, jadi `Bash(curl *)` tidak menghentikan `/usr/bin/curl` atau `sh -c 'curl …'`; lihat [apa yang tidak cocok dengan aturan Bash](/docs/id/permissions#bash-rule-limits). Kunci ini menggantikan konfigurasi `ignorePatterns` yang sudah usang.

<h3 id="permissions-additionaldirectories">
  `permissions.additionalDirectories`
</h3>

Berikan akses file Claude ke direktori di luar yang Anda mulai, sebagai [direktori kerja](/docs/id/permissions#working-directories) tambahan. Sebagian besar konfigurasi `.claude/` [tidak ditemukan](/docs/id/permissions#additional-directories-grant-file-access-not-configuration) dari direktori ini.

* **Scope**: [`Any file`](#scopes)
* **Type**: array jalur direktori
* **Default**: tidak diatur
* **Per-session overrides**: `--add-dir` dan `/add-dir` menambahkan direktori untuk satu sesi bersama kunci ini

```json settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": ["../docs/"]
  }
}
```

Seperti aturan `allow`, entri dalam `.claude/settings.json` proyek berlaku hanya setelah Anda menerima [dialog kepercayaan ruang kerja](/docs/id/permissions#project-allow-rules-and-workspace-trust) untuk folder itu.

<h3 id="permissions-blockreadsoutsideworkingdirectories">
  `permissions.blockReadsOutsideWorkingDirectories`
</h3>

Hentikan Claude dari membaca jalur di luar [direktori kerja](/docs/id/permissions#working-directories) sesi dengan alat Read, Grep, Glob, dan LSP, dalam setiap mode izin termasuk `bypassPermissions`. Perintah Bash yang membaca jalur yang cocok melalui perintah file yang dikenali Claude Code, seperti `cat`, meminta Anda bahkan dalam mode otomatis dan mode `bypassPermissions`. Memerlukan Claude Code v2.1.257 atau lebih baru.

Perintah Bash yang tidak dapat dilacak oleh parser shell, seperti yang mengubah direktori lebih dari sekali atau menjalankan subshell, meminta Anda bahkan dalam mode otomatis dan mode `bypassPermissions`. Prompt muncul bahkan ketika perintah tidak menamai jalur apa pun di luar direktori kerja. Prompt ini tidak berlaku ketika perintah berjalan dalam [sandbox](/docs/id/sandboxing) dan sandbox memberlakukan blokir.

Claude Code juga menulis `true` di sini ketika Anda memilih untuk memblokir pembacaan tersebut pada [prompt mode otomatis sebelum pembacaan pertama di luar direktori kerja](/docs/id/permission-modes#first-read-outside-the-working-directories).

* **Scope**: [`Any file`](#scopes). Jika sumber pengaturan apa pun menetapkan `true`, blokir berlaku, jadi file yang diperiksa repositori dapat mengaktifkan blokir untuk proyek tetapi tidak dapat mengangkat blokir yang Anda atur.
* **Type**: Boolean
  * `true`: pembacaan file di luar direktori kerja diblokir
  * `false`: sama dengan tidak diatur; `true` di file pengaturan lain apa pun masih memblokir
* **Default**: tidak diatur, jadi pembacaan di luar direktori kerja mengikuti mode izin dan aturan Anda

```json settings.json theme={null}
{
  "permissions": {
    "blockReadsOutsideWorkingDirectories": true
  }
}
```

Jika hanya file pengaturan yang diperiksa repositori yang menambahkan direktori, blokir masih berlaku untuk pembacaan di sana. Ketika [`autoMemoryDirectory`](#automemorydirectory) berasal dari `.claude/settings.json` proyek, atau dari `.claude/settings.local.json` [diperlakukan sebagai disediakan repositori](/docs/id/permissions#when-your-local-settings-file-needs-trust), Claude Code tidak memuat [memori otomatis](/docs/id/memory#storage-location) apa pun dari direktori itu dan tidak menyimpan apa pun ke dalamnya. File yang dibutuhkan Claude Code sendiri tetap dapat dibaca, seperti keterampilan, plugin, aturan, agen, perintah Anda, dan file memori `CLAUDE.md` di bawah `~/.claude/`.

Ketika [sandbox](/docs/id/sandboxing) aktif, blokir juga menolak perintah sandboxed akses baca ke direktori home dan akar volume yang dipasang di luar direktori kerja. Percobaan ulang yang memerlukan persetujuan untuk [berjalan di luar sandbox](/docs/id/sandboxing#the-unsandboxed-retry-escape-hatch) meminta Anda bahkan dalam mode `bypassPermissions`. File yang dibaca alat dari direktori home Anda, seperti `~/.gitconfig`, ditolak dengan sisanya; buka kembali jalur spesifik dengan [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread) ketika alat membutuhkannya.

Ketika direktori kerja sesi adalah [git worktree](/docs/id/worktrees) yang ditautkan, termasuk yang dimasuki Claude Code di tengah sesi, direktori `.git` umum repositori tetap dapat dibaca dan ditulis ke perintah sandboxed, sehingga git terus bekerja di sana.

<h3 id="permissions-defaultmode">
  `permissions.defaultMode`
</h3>

Atur [mode izin](/docs/id/permission-modes) yang dimulai sesi baru. Ketika Anda membiarkannya tidak diatur, sesi dimulai dalam [default bawaan](/docs/id/permission-modes#which-mode-a-session-starts-in) untuk rencana dan permukaan Anda.

* **Scope**: [`Any file`](#scopes). `auto` dan `bypassPermissions` tidak berlaku dari pengaturan proyek atau lokal, jadi atur mereka dalam `~/.claude/settings.json` sebagai gantinya. Sebelum v2.1.257, `bypassPermissions` berlaku dari file apa pun. Untuk percakapan yang dimulai ekstensi VS Code, Claude Code hanya membaca nilai pengguna, terkelola, dan `--settings`.
* **Type**: string, salah satu dari:
  * `"default"`: Claude Code hanya menjalankan pembacaan tanpa bertanya
  * `"acceptEdits"`: Claude Code juga menjalankan pengeditan file dan perintah sistem file umum seperti `mkdir` dan `mv` tanpa bertanya
  * `"plan"`: Claude Code membaca dan merencanakan tetapi memblokir pengeditan sampai Anda menyetujui rencana
  * `"auto"`: Claude Code menjalankan semuanya, dengan pemeriksaan keamanan latar belakang
  * `"dontAsk"`: Claude Code secara otomatis menolak setiap panggilan yang sebaliknya akan meminta; pembacaan, tindakan lain yang tidak memerlukan persetujuan, dan alat yang sudah disetujui sebelumnya masih berjalan
  * `"bypassPermissions"`: Claude Code menjalankan semuanya tanpa bertanya
  * `"manual"`: alias untuk `"default"`, dalam Claude Code v2.1.200 atau lebih baru
* **Default**: tidak diatur
* **Per-session overrides**: `--permission-mode`, dan ekuivalennya `--dangerously-skip-permissions` untuk `bypassPermissions`, mengambil alih kunci ini untuk satu sesi

```json settings.json theme={null}
{
  "permissions": {
    "defaultMode": "acceptEdits"
  }
}
```

Aturan izin berlapis di atas setiap mode: aturan `deny` memblokir dalam setiap mode, termasuk `bypassPermissions`. Lihat [Mode izin](/docs/id/permission-modes). `manual` menamai mode izin berlabel Manual dalam CLI dan ekstensi VS Code; alias memerlukan Claude Code v2.1.200 atau lebih baru. Dalam Claude Code di web, Claude Code hanya menghormati `acceptEdits`, `plan`, `default`, dan `auto` dari kunci ini. Untuk percakapan yang dimulai ekstensi VS Code, lihat [pengaturan mana yang dibaca ekstensi untuk mode izin awal](/docs/id/permission-modes#switch-permission-modes).

<h3 id="permissions-disablebypasspermissionsmode">
  `permissions.disableBypassPermissionsMode`
</h3>

Cegah siapa pun memasuki mode `bypassPermissions`. Claude Code kemudian menolak flag `--dangerously-skip-permissions`, dan mengabaikan [definisi agen](/docs/id/sub-agents#permission-modes) `permissionMode: bypassPermissions`, jadi subagen berjalan dengan mode izin sesi induk.

* **Scope**: [`Any file`](#scopes). Biasanya diatur dalam [pengaturan terkelola](/docs/id/managed-settings) untuk menegakkan kebijakan organisasi.
* **Type**: string `"disable"`
* **Default**: tidak diatur
* **Per-session overrides**: kunci ini mengambil alih `--dangerously-skip-permissions`, yang ditolak Claude Code saat kunci diatur

```json settings.json theme={null}
{
  "permissions": {
    "disableBypassPermissionsMode": "disable"
  }
}
```

Sebelum v2.1.223, Claude Code menerapkan mode izin frontmatter bahkan dengan bypass dinonaktifkan.

<h3 id="skipautopermissionprompt">
  `skipAutoPermissionPrompt`
</h3>

Lewati pemberitahuan satu kali yang menjelaskan [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) yang ditampilkan Claude Code ketika Anda pertama kali memasuki mode otomatis sendiri, misalnya melalui pengaturan Anda sendiri atau pemilih mode, bukan ketika default bawaan memulai sesi di dalamnya. Claude Code menampilkan pemberitahuan itu sekali dan kemudian mencatat bahwa itu ditampilkan, jadi kunci ini hanya penting di mana pemberitahuan belum muncul.

* **Scope**: [`User or managed`](#scopes). Repositori tidak dapat menetapkannya untuk Anda.
* **Type**: Boolean
  * `true`: Claude Code melewati pemberitahuan
  * `false`: sama dengan tidak diatur; pemberitahuan muncul sekali kecuali file lain ini menetapkan `true`
* **Default**: tidak diatur, jadi pemberitahuan muncul sekali

```json settings.json theme={null}
{
  "skipAutoPermissionPrompt": true
}
```

<h3 id="skipdangerousmodepermissionprompt">
  `skipDangerousModePermissionPrompt`
</h3>

Lewati dialog konfirmasi yang ditampilkan Claude Code sebelum sesi memasuki mode `bypassPermissions`, baik dari `--dangerously-skip-permissions` atau dari `defaultMode: "bypassPermissions"`. Claude Code menulis `true` di sini dalam pengaturan pengguna Anda ketika Anda menerima dialog itu sekali.

* **Scope**: [`User, local, or managed`](#scopes). Repositori yang tidak dipercaya tidak dapat melewati dialog untuk Anda.
* **Type**: Boolean
  * `true`: Claude Code melewati dialog konfirmasi sebelum sesi memasuki mode `bypassPermissions`
  * `false`: sama dengan tidak diatur; dialog muncul kecuali file lain ini menetapkan `true`
* **Default**: tidak diatur, jadi dialog muncul

```json settings.json theme={null}
{
  "skipDangerousModePermissionPrompt": true
}
```

<h2 id="sandbox-settings">
  Pengaturan sandbox
</h2>

Isolasi perintah yang dijalankan Claude dari sistem file, jaringan, dan kredensial Anda. Untuk cara kerja sandboxing dan persyaratan platform, lihat [Sandboxing](/docs/id/sandboxing).

<h3 id="sandbox">
  `sandbox`
</h3>

Isolasi perintah Bash yang dijalankan Claude dari sistem file dan jaringan Anda dengan [sandboxing](/docs/id/sandboxing). Aktifkan sandbox dengan `enabled`, kemudian perluas atau perkecil apa yang dapat disentuh perintah bersandbox dengan sub-objek `filesystem`, `network`, dan `credentials`. Sandbox berjalan di macOS, Linux, dan WSL2.

* **Scope**: [`Any file`](#scopes)
* **Type**: object dengan `enabled`, `failIfUnavailable`, `autoAllowBashIfSandboxed`, `excludedCommands`, `allowUnsandboxedCommands`, `enableWeakerNestedSandbox`, `enableWeakerNetworkIsolation`, `allowAppleEvents`, `bwrapPath`, `socatPath`, `ignoreViolations`, dan `ripgrep`, ditambah objek `filesystem`, `network`, dan `credentials`
* **Default**: tidak diatur, jadi Claude Code menjalankan perintah tanpa sandbox

Ini mengaktifkan sandbox, melewati prompt izin untuk perintah bersandbox, menjalankan `docker` di luar sandbox, membuka dua jalur penulisan tambahan, menyembunyikan file kredensial AWS Anda, dan pra-mengizinkan GitHub dan npm:

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": true,
    "excludedCommands": ["docker *"],
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"],
      "denyRead": ["~/.aws/credentials"]
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

Claude Code mengambil nilai kunci Boolean dari scope pengaturan dengan prioritas tertinggi yang menetapkannya, jadi `enabled` atau `failIfUnavailable` yang dikelola menggantikan apa pun yang ditetapkan pengembang. Ini menggabungkan kunci array di setiap scope pengaturan yang dimuat sesi, jadi pengembang dapat menambahkan entri; lihat [Keep developers from widening the policy](/docs/id/sandboxing#keep-developers-from-widening-the-policy) untuk kunci khusus yang dikelola. Untuk memerlukan sandbox untuk organisasi, lihat [Enforce sandboxing with managed settings](/docs/id/sandboxing#enforce-sandboxing-with-managed-settings).

<h3 id="sandbox-enabled">
  `sandbox.enabled`
</h3>

Aktifkan [sandboxing](/docs/id/sandboxing) untuk perintah Bash. Ketika Anda memilih mode di panel `/sandbox`, Claude Code menulis kunci ini ke `.claude/settings.local.json` untuk proyek saat ini; atur di `~/.claude/settings.json` untuk sandbox setiap proyek.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code membuat sandbox untuk perintah Bash
  * `false`: Perintah Bash berjalan tanpa sandbox
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true
  }
}
```

Di Linux dan WSL2 sandbox memerlukan `bubblewrap` dan `socat`; lihat [Set up Linux and WSL2](/docs/id/sandboxing#set-up-linux-and-wsl2). Ketika sandbox tidak dapat dimulai, Claude Code menampilkan peringatan dan menjalankan perintah tanpa sandbox kecuali Anda juga menetapkan [`failIfUnavailable`](#sandbox-failifunavailable).

<h3 id="sandbox-failifunavailable">
  `sandbox.failIfUnavailable`
</h3>

Buat Claude Code keluar dengan kesalahan saat startup ketika `sandbox.enabled` adalah `true` tetapi sandbox tidak dapat dimulai, karena dependensi hilang atau platform tidak didukung. Tanpanya, Claude Code menampilkan peringatan dan menjalankan perintah tanpa sandbox. Gunakan di pengaturan yang dikelola ketika organisasi Anda memerlukan sandboxing sebagai gerbang keras.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code keluar dengan kesalahan saat startup ketika `sandbox.enabled` adalah `true` tetapi sandbox tidak dapat dimulai
  * `false`: Claude Code menampilkan peringatan dan menjalankan perintah tanpa sandbox
* **Default**: `false`

Ini membuat setiap mesin yang dikelola membuat sandbox untuk perintah atau menolak untuk memulai:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true
  }
}
```

Lihat [Enforce sandboxing with managed settings](/docs/id/sandboxing#enforce-sandboxing-with-managed-settings).

<h3 id="sandbox-autoallowbashifsandboxed">
  `sandbox.autoAllowBashIfSandboxed`
</h3>

Biarkan Claude Code menjalankan perintah Bash bersandbox tanpa prompt izin. Perintah yang tidak dapat berjalan di sandbox masih melalui alur izin reguler, dan aturan `deny` serta aturan `ask` yang dibatasi konten seperti `Bash(git push *)` masih berlaku; aturan `Bash` ask yang kosong dilewati untuk perintah bersandbox. Atur ke `false` untuk mengirim perintah bersandbox melalui alur izin reguler juga, yang tab **Mode** `/sandbox` sebut mode izin reguler.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code menjalankan perintah Bash bersandbox tanpa prompt izin, tunduk pada aturan `deny` dan aturan `ask` yang dibatasi konten; `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB` mematikan auto-allow
  * `false`: perintah bersandbox melalui alur izin reguler, jadi aturan allow dan mode izin Anda memutuskan. Tab **Mode** `/sandbox` menyebut ini mode izin reguler
* **Default**: `true`

Ini menjaga sandbox tetap aktif dan mengirim perintah bersandbox melalui alur izin reguler:

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": false
  }
}
```

Lihat [Sandbox modes](/docs/id/sandboxing#sandbox-modes) untuk apa yang masih diminta auto-allow mode dan cara kerjanya dalam plan mode.

<h3 id="sandbox-excludedcommands">
  `sandbox.excludedCommands`
</h3>

Beri nama perintah yang Claude Code jalankan di luar sandbox, seperti alat yang tidak berfungsi di bawahnya. Setiap entri menggunakan sintaks yang sama dengan konten aturan izin `Bash(...)` [permission rule](/docs/id/permissions#permission-rule-syntax): perintah yang tepat, awalan seperti `docker *`, atau pola wildcard.

Entri Anda mengeluarkan panggilan Bash dari sandbox hanya ketika mereka mencakup setiap perintah di dalamnya, dan beberapa bentuk panggilan tetap bersandbox bahkan kemudian. Entri `docker *` saja tidak mengeluarkan `npm ci && docker build .` dari sandbox.

* **Scope**: [`Any file`](#scopes)
* **Type**: array pola perintah
* **Default**: tidak diatur, jadi tidak ada perintah yang dikecualikan

```json settings.json theme={null}
{
  "sandbox": {
    "excludedCommands": ["docker *"]
  }
}
```

Claude Code menjaga panggilan Bash bersandbox ketika memiliki salah satu bentuk ini, di antara lainnya:

* Perintah yang dimulai dengan `sudo`, `eval`, atau `xargs`
* `cd`, `pushd`, atau `popd`, di mana pun muncul dalam panggilan
* Substitusi perintah, subshell, atau blok alur kontrol seperti `if` atau `for`
* Pengalihan, seperti `docker build . > build.log`, selain yang hanya menduplikasi deskriptor file, seperti `2>&1`
* Nama perintah yang berasal dari variabel

Misalnya, `cd build && docker compose up` tetap bersandbox di bawah entri `docker *`, dan menambahkan entri `cd` tidak mengubah itu.

Perintah yang dikecualikan masih melalui alur izin reguler. Pengecualian adalah kenyamanan, bukan batas keamanan: lebih suka [`filesystem.allowWrite`](#sandbox-filesystem-allowwrite) ketika alat hanya perlu menulis ke tempat tertentu. Claude Code menggabungkan entri di setiap scope pengaturan yang dimuat sesi, dan tidak ada kunci khusus yang dikelola untuk daftar ini, jadi jaga daftar yang dikelola tetap sempit.

<h3 id="sandbox-allowunsandboxedcommands">
  `sandbox.allowUnsandboxedCommands`
</h3>

Biarkan Claude mencoba ulang perintah di luar sandbox dengan parameter `dangerouslyDisableSandbox` setelah sandbox membloknya. Atur ke `false` sehingga Claude Code mengabaikan parameter itu sepenuhnya dan setiap perintah yang dijalankan Claude harus bersandbox atau muncul di [`excludedCommands`](#sandbox-excludedcommands). Tab **Overrides** `/sandbox` menampilkan status itu sebagai **Strict sandbox mode**. Gunakan `false` di pengaturan yang dikelola untuk kebijakan yang memerlukan sandboxing ketat.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude dapat mencoba ulang perintah di luar sandbox dengan parameter `dangerouslyDisableSandbox` setelah sandbox membloknya
  * `false`: Claude Code mengabaikan parameter itu, jadi setiap perintah yang dijalankan Claude bersandbox atau muncul di `excludedCommands`
* **Default**: `true`

Ini memberlakukan mode sandbox ketat untuk semua orang yang dicakup pengaturan yang dikelola:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowUnsandboxedCommands": false
  }
}
```

Percobaan ulang tanpa sandbox melalui alur izin reguler, dengan prompt dalam Manual mode. Lihat [The unsandboxed retry escape hatch](/docs/id/sandboxing#the-unsandboxed-retry-escape-hatch).

Untuk melihat kapan perintah yang Anda ketik sendiri di prompt mode shell [`!`](/docs/id/interactive-mode#shell-mode-with-prefix) berjalan bersandbox, lihat [strict sandbox mode](/docs/id/sandboxing#the-unsandboxed-retry-escape-hatch).

<h3 id="sandbox-filesystem">
  `sandbox.filesystem`
</h3>

Kontrol jalur mana yang dapat dibaca dan ditulis perintah bersandbox. Secara default mereka dapat menulis ke direktori kerja, direktori temp sesi, dan direktori yang Anda tambahkan dengan `--add-dir`, `/add-dir`, atau `permissions.additionalDirectories`, dan dapat membaca sisa sistem file, termasuk file kredensial. Perluas atau perkecil itu dengan empat daftar jalur, atau matikan lapisan sistem file dengan `disabled`. Lihat [Filesystem isolation](/docs/id/sandboxing#filesystem-isolation) untuk batas default.

* **Scope**: [`Any file`](#scopes)
* **Type**: object dengan array `allowWrite`, `denyWrite`, `denyRead`, dan `allowRead`, ditambah Boolean `allowManagedReadPathsOnly` dan `disabled`
* **Default**: tidak diatur, jadi batas baca dan tulis default berlaku

Ini memungkinkan perintah bersandbox menulis ke direktori build dan kubeconfig Anda, dan menyembunyikan file kredensial AWS Anda:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"],
      "denyRead": ["~/.aws/credentials"]
    }
  }
}
```

Claude Code memberlakukan daftar ini di batas sandbox OS, jadi mereka berlaku untuk setiap subprocess yang dimulai perintah bersandbox, seperti `kubectl`, `terraform`, atau `npm`. Claude Code menambahkan [permission rules](/docs/id/sandboxing#permission-rules) Anda ke daftar yang sama: aturan allow dan deny `Edit` ke `allowWrite` dan `denyWrite`, aturan deny `Read` ke `denyRead`, dan aturan allow dan deny `WebFetch(domain:...)` ke daftar domain [`network`](#sandbox-network).

Kecuali kunci khusus yang dikelola diatur, Claude Code menggabungkan setiap daftar di file pengaturan yang dimuat sesi. [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly) membatasi `allowRead` ke entri dari pengaturan yang dikelola, dan [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) melakukan hal yang sama untuk domain yang diizinkan.

[Configure sandboxing](/docs/id/sandboxing#configure-sandboxing) mencakup sumber yang Anda kecualikan dengan `--setting-sources`. Ketika Anda mengedit daftar selama sesi, Claude Code [menerapkan perubahan ke sesi yang berjalan](/docs/id/settings#when-edits-take-effect).

<h4 id="sandbox-path-prefixes">
  Awalan jalur sandbox
</h4>

Jalur di `allowWrite`, `denyWrite`, `denyRead`, `allowRead`, dan [`credentials.files`](#sandbox-credentials-files) diselesaikan dengan awalan mereka:

| Awalan                 | Arti                                                                                                | Contoh                                                                        |
| :--------------------- | :-------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------- |
| `/`                    | Jalur absolut dari akar sistem file                                                                 | `/tmp/build` tetap `/tmp/build`                                               |
| `~/`                   | Relatif terhadap direktori home                                                                     | `~/.kube` menjadi `$HOME/.kube`                                               |
| `./` atau tanpa awalan | Relatif terhadap akar proyek untuk pengaturan proyek, atau ke `~/.claude` untuk pengaturan pengguna | `./output` di `.claude/settings.json` diselesaikan ke `<project-root>/output` |

Awalan `//path` untuk jalur absolut juga berfungsi. Jika Anda menggunakan `/path` dengan slash tunggal mengharapkan resolusi relatif proyek, beralih ke `./path`. Sintaks ini berbeda dari [Read and Edit permission rules](/docs/id/permissions#read-and-edit), yang menggunakan `//path` untuk absolut dan `/path` untuk relatif proyek: jalur sistem file sandbox menggunakan konvensi standar, jadi `/tmp/build` adalah jalur absolut.

Claude Code menghapus garis miring trailing dari jalur direktori, jadi `~/.aws` dan `~/.aws/` cocok dengan direktori yang sama. Sebelum v2.1.224, Claude Code meneruskan garis miring trailing ke sandbox, dan Claude masih dapat membaca atau menulis jalur di bawah entri `denyRead` atau `denyWrite` yang ditulis dengan satu.

Claude Code juga menghapus `/**` trailing, jadi `~/build/**` dan `~/build` mencakup direktori yang sama. Apakah wildcard seperti `*` berfungsi tergantung pada daftar mana entri berada dan pada platform:

* **`allowWrite` dan `denyWrite`**: di macOS, wildcard berfungsi. Di Linux dan WSL2, sandbox memasang jalur konkret, jadi Claude Code melewati entri yang berisi `*`, `?`, atau `[` setelah `/**` trailing dihapus, dan entri itu tidak berpengaruh. Claude Code menambahkan jalur dari aturan izin `Edit` Anda ke daftar ini, jadi batas yang sama berlaku untuk mereka, dan tab **Config** `/sandbox` memperingatkan tentang aturan izin `Edit` dan `Read` yang berisi wildcard.
* **`denyRead` dan `allowRead`**: wildcard berfungsi di setiap platform. Di Linux dan WSL2, Claude Code memperluas entri baca ke jalur konkret yang cocok, yang tidak dilakukannya untuk daftar tulis.

<h3 id="sandbox-filesystem-allowwrite">
  `sandbox.filesystem.allowWrite`
</h3>

Tambahkan jalur tempat perintah bersandbox dapat menulis, di luar direktori kerja, direktori temp sesi, dan direktori yang telah Anda tambahkan dengan `--add-dir`, `/add-dir`, atau `permissions.additionalDirectories`. Gunakan ketika subprocess seperti `kubectl` atau alat build perlu menulis di luar proyek.

* **Scope**: [`Any file`](#scopes)
* **Type**: array string jalur, menggunakan [sandbox path prefixes](#sandbox-path-prefixes)
* **Default**: tidak diatur, jadi perintah bersandbox dapat menulis ke direktori kerja, direktori temp sesi, direktori yang telah Anda tambahkan dengan `--add-dir` atau `/add-dir`, dan direktori di [`permissions.additionalDirectories`](#permissions-additionaldirectories)

Ini memungkinkan build menulis di bawah `/tmp/build` dan memungkinkan `kubectl` memperbarui kubeconfig Anda:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"]
    }
  }
}
```

Claude Code menggabungkan entri di setiap scope pengaturan yang dimuat sesi: jalur pengguna, proyek, lokal, dan yang dikelola digabungkan daripada menggantikan satu sama lain, dan Claude Code menambahkan jalur dari aturan izin allow `Edit(...)` Anda. Entri `allowWrite` tidak dapat mengangkat [protected path](/docs/id/sandboxing#protected-paths).

<h3 id="sandbox-filesystem-denywrite">
  `sandbox.filesystem.denyWrite`
</h3>

Blokir perintah bersandbox dari menulis ke jalur tertentu, termasuk jalur di dalam direktori yang dapat ditulis.

* **Scope**: [`Any file`](#scopes)
* **Type**: array string jalur, menggunakan [sandbox path prefixes](#sandbox-path-prefixes)
* **Default**: tidak diatur

Ini menjaga perintah bersandbox dari mengubah konfigurasi sistem atau memasang binari:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyWrite": ["/etc", "/usr/local/bin"]
    }
  }
}
```

Claude Code menggabungkan entri di setiap scope pengaturan yang dimuat sesi, dan menambahkan jalur dari aturan izin deny `Edit(...)` Anda.

<h3 id="sandbox-filesystem-denyread">
  `sandbox.filesystem.denyRead`
</h3>

Blokir perintah bersandbox dari membaca jalur tertentu, seperti file kredensial yang kebijakan baca default akan mengekspos. Untuk melindungi file kredensial dan menjaganya tetap dapat digunakan melalui proxy sandbox, lihat [`sandbox.credentials`](#sandbox-credentials) sebagai gantinya.

* **Scope**: [`Any file`](#scopes)
* **Type**: array string jalur, menggunakan [sandbox path prefixes](#sandbox-path-prefixes)
* **Default**: tidak diatur, jadi perintah bersandbox menjaga [default read access](/docs/id/sandboxing#filesystem-isolation), yang mencakup file kredensial seperti `~/.aws/credentials`

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/.aws/credentials"]
    }
  }
}
```

Claude Code menggabungkan entri di setiap scope pengaturan yang dimuat sesi, dan menambahkan jalur dari aturan izin deny `Read(...)` Anda. Ketika [`filesystem.disabled`](#sandbox-filesystem-disabled) adalah `true`, Claude Code tidak memberlakukan entri ini.

<h3 id="sandbox-filesystem-allowread">
  `sandbox.filesystem.allowRead`
</h3>

Buka kembali pembacaan untuk jalur tertentu di dalam wilayah yang diblokir [`denyRead`](#sandbox-filesystem-denyread), untuk membangun akses baca hanya workspace. Entri `denyRead` yang tepat atau wildcard tetap diblokir di dalam `allowRead` yang lebih luas, seperti yang ditunjukkan [overlap table](/docs/id/sandboxing#configure-sandboxing). Ketika entri `denyRead` wildcard seperti `~/**/.env` cocok dengan direktori, Claude Code memblokir pembacaan isinya juga. Sebelum v2.1.236 di macOS, Claude Code membuka kembali jalur yang cocok dengan entri `denyRead` wildcard di mana pun entri `allowRead` yang lebih luas mencakupnya, dan meninggalkan isi direktori yang cocok dapat dibaca.

* **Scope**: [`Any file`](#scopes)
* **Type**: array string jalur, menggunakan [sandbox path prefixes](#sandbox-path-prefixes)
* **Default**: tidak diatur

Ini memblokir pembacaan direktori home Anda kecuali proyek itu sendiri:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["."]
    }
  }
}
```

Claude Code menyelesaikan entri `.` ke akar proyek dalam pengaturan proyek dan ke `~/.claude` dalam pengaturan pengguna. Claude Code menggabungkan entri di setiap file pengaturan yang dimuat sesi kecuali [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly) diatur.

<h3 id="sandbox-filesystem-allowmanagedreadpathsonly">
  `sandbox.filesystem.allowManagedReadPathsOnly`
</h3>

Hormati hanya entri [`allowRead`](#sandbox-filesystem-allowread) yang berasal dari pengaturan yang dikelola, jadi pengembang tidak dapat membuka kembali akses baca ke jalur yang diblokir organisasi Anda. Claude Code masih menggabungkan entri `denyRead` dari setiap scope pengaturan yang dimuat sesi.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code menghormati hanya entri `allowRead` dari pengaturan yang dikelola
  * `false`: entri `allowRead` menggabungkan dari setiap scope pengaturan yang dimuat sesi
* **Default**: `false`

Ini memblokir pembacaan direktori home, membuka kembali `~/work`, dan menghentikan pengembang dari membuka kembali apa pun:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["~/work"],
      "allowManagedReadPathsOnly": true
    }
  }
}
```

Lihat [Keep developers from widening the policy](/docs/id/sandboxing#keep-developers-from-widening-the-policy).

<h3 id="sandbox-filesystem-disabled">
  `sandbox.filesystem.disabled`
</h3>

Lewati isolasi sistem file sambil menjaga isolasi jaringan. Perintah bersandbox mendapatkan akses baca dan tulis tanpa batas ke sistem file host, dan egress jaringan mereka tetap terbatas pada [`network.allowedDomains`](#sandbox-network-alloweddomains). Gunakan ketika Anda membuat sandbox untuk mengontrol tempat perintah terhubung daripada apa yang mereka tulis. Memerlukan Claude Code v2.1.216 atau lebih baru.

* **Scope**: [`User or managed`](#scopes). Ketika pengaturan yang dikelola mengonfigurasi `sandbox.filesystem` sama sekali, atau mencantumkan entri `sandbox.credentials.files` dengan `"mode": "deny"`, hanya pengaturan yang dikelola yang dapat menetapkannya.
* **Type**: Boolean
  * `true`: Claude Code melewati isolasi sistem file dan menjaga isolasi jaringan
  * `false`: isolasi sistem file tetap aktif
* **Default**: `false`, jadi isolasi sistem file tetap aktif

Ini membiarkan sistem file terbuka dan membatasi egress jaringan ke GitHub dan npm:

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "disabled": true
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

Dengan lapisan mati, Claude Code tidak memberlakukan entri `denyRead` atau `credentials.files` `deny`, sementara entri `credentials.envVars` dan entri `mask` yang diterapkan tetap bekerja. [`autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed) masih default ke `true`, jadi atur ke `false` untuk terus meminta. Lihat [Disable filesystem isolation](/docs/id/sandboxing#disable-filesystem-isolation) untuk daftar lengkap sumber yang dapat menetapkannya dan apa yang berubah ketika isolasi mati. Memerlukan Claude Code v2.1.216 atau lebih baru.

<h3 id="sandbox-ignoreviolations">
  `sandbox.ignoreViolations`
</h3>

Senyapkan laporan pelanggaran sandbox untuk jalur yang Anda harapkan perintah untuk menyelidiki dan ditolak, seperti alat yang memeriksa `/etc/hosts` saat startup, jadi penolakan itu tidak muncul sebagai pelanggaran atau dalam apa yang Claude lihat. Sandbox masih memblokir akses; hanya laporan yang ditekan. Kunci adalah substring untuk cocok dengan perintah, dengan `*` cocok dengan setiap perintah, dan nilai adalah substring pelanggaran untuk diabaikan untuk perintah itu, seperti jalur sistem file.

* **Scope**: [`Any file`](#scopes)
* **Type**: object memetakan substring perintah ke array substring pelanggaran, biasanya jalur
* **Default**: tidak diatur, jadi setiap pelanggaran dilaporkan

```json settings.json theme={null}
{
  "sandbox": {
    "ignoreViolations": {
      "*": ["/etc/hosts"]
    }
  }
}
```

<h3 id="sandbox-enableweakernestedsandbox">
  `sandbox.enableWeakerNestedSandbox`
</h3>

Jalankan sandbox Linux di dalam kontainer Docker yang tidak berprivilese, di mana bubblewrap tidak dapat memasang `/proc` segar. Sebagai gantinya sandbox dalam bind-mount `/proc` kontainer yang ada, yang mengekspos informasi proses yang mount segar akan sembunyikan. Ini mengurangi keamanan; gunakan hanya ketika kontainer luar sudah menyediakan isolasi yang Anda butuhkan.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: sandbox dalam bind-mount `/proc` kontainer yang ada daripada memasang yang segar
  * `false`: sandbox memasang `/proc` segar, yang tidak berfungsi dalam kontainer Docker yang tidak berprivilese
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNestedSandbox": true
  }
}
```

Hanya Linux dan WSL2. Lihat [Bubblewrap fails to start inside a container](/docs/id/sandboxing#troubleshooting).

<h3 id="sandbox-enableweakernetworkisolation">
  `sandbox.enableWeakerNetworkIsolation`
</h3>

Biarkan perintah bersandbox di macOS menjangkau layanan kepercayaan TLS sistem, `com.apple.trustd.agent`. Alat berbasis Go seperti `gh`, `gcloud`, dan `terraform` membutuhkannya untuk memverifikasi sertifikat TLS ketika Anda menggunakan [`network.httpProxyPort`](#sandbox-network-httpproxyport) dengan proxy MITM dan CA kustom. Ini mengurangi keamanan dengan membuka jalur eksfiltrasi data potensial melalui layanan kepercayaan.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: perintah bersandbox di macOS dapat menjangkau `com.apple.trustd.agent`
  * `false`: perintah bersandbox di macOS tidak dapat menjangkau layanan kepercayaan TLS sistem
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNetworkIsolation": true
  }
}
```

Jika Anda tidak menggunakan proxy MITM, cantumkan alat yang gagal di [`excludedCommands`](#sandbox-excludedcommands) sebagai gantinya; lihat [Go-based CLIs fail TLS verification on macOS](/docs/id/sandboxing#troubleshooting).

<h3 id="sandbox-allowappleevents">
  `sandbox.allowAppleEvents`
</h3>

Biarkan perintah bersandbox di macOS mengirim Apple Events, yang `open`, `osascript`, dan alat yang membuka URL di browser butuhkan; tanpanya mereka gagal dengan kesalahan `-600`. Ini menghapus isolasi eksekusi kode: perintah bersandbox dapat meluncurkan aplikasi lain tanpa sandbox tanpa prompt pengguna, dan dapat mengirim perintah AppleScript ke aplikasi yang berjalan seperti Terminal, tunduk pada prompt persetujuan otomasi per-aplikasi macOS (TCC).

* **Scope**: [`User or managed`](#scopes)
* **Type**: Boolean
  * `true`: perintah bersandbox di macOS dapat mengirim Apple Events
  * `false`: perintah bersandbox di macOS tidak dapat mengirim Apple Events, jadi `open` dan `osascript` gagal dengan kesalahan `-600`
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowAppleEvents": true
  }
}
```

Untuk menjaga isolasi dan masih menjalankan satu alat seperti itu, tambahkan ke [`excludedCommands`](#sandbox-excludedcommands) sebagai gantinya. Lihat [Apple Events on macOS](/docs/id/sandboxing#security-limitations).

<h3 id="sandbox-ripgrep">
  `sandbox.ripgrep`
</h3>

Arahkan sandbox ke binari ripgrep Anda sendiri daripada yang digunakan Claude Code, misalnya ketika platform Anda memerlukan `rg` yang dibangun berbeda.

* **Scope**: [`User or managed`](#scopes)
* **Type**: object dengan `command`, jalur ke binari ripgrep, dan opsional `args`, array argumen untuk ditambahkan di depan
* **Default**: tidak diatur, jadi sandbox menggunakan binari ripgrep yang sama dengan Claude Code. Itu adalah binari bundel kecuali Anda menetapkan [`USE_BUILTIN_RIPGREP`](/docs/id/env-vars) ke `0`

```json settings.json theme={null}
{
  "sandbox": {
    "ripgrep": {
      "command": "/usr/local/bin/rg"
    }
  }
}
```

<h3 id="sandbox-bwrappath">
  `sandbox.bwrapPath`
</h3>

Arahkan sandbox ke binari bubblewrap yang dipasang di luar `PATH`, seperti salinan yang dijual di host yang terisolasi udara. Claude Code menggunakan jalur baik untuk pemeriksaan dependensi startup maupun ketika membungkus setiap perintah bersandbox.

* **Scope**: [`Managed`](#scopes). Claude Code membacanya hanya dari pengaturan yang dikelola sehingga file pengguna, proyek, atau lokal tidak dapat menunjuk sandbox ke binari yang berbeda.
* **Type**: string, jalur absolut; Claude Code menjatuhkan jalur relatif dan kembali ke pencarian `PATH`
* **Default**: tidak diatur, jadi Claude Code menemukan `bwrap` di `PATH`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "bwrapPath": "/opt/admin/bwrap"
  }
}
```

Hanya Linux dan WSL2.

<h3 id="sandbox-socatpath">
  `sandbox.socatPath`
</h3>

Arahkan proxy jaringan sandbox ke binari `socat` yang dipasang di luar `PATH`.

* **Scope**: [`Managed`](#scopes)
* **Type**: string, jalur absolut; Claude Code menjatuhkan jalur relatif dan kembali ke pencarian `PATH`
* **Default**: tidak diatur, jadi Claude Code menemukan `socat` di `PATH`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "socatPath": "/opt/admin/socat"
  }
}
```

Hanya Linux dan WSL2.

<h3 id="sandbox-credentials">
  `sandbox.credentials`
</h3>

Deklarasikan file kredensial dan variabel lingkungan untuk [melindungi dari perintah bersandbox](/docs/id/sandboxing#protect-credentials). Setiap entri menamai file `path` atau variabel `name` dan `mode`: `deny` menyembunyikan kredensial di dalam sandbox, dan `mask` menunjukkan perintah bersandbox placeholder sementara [sandbox proxy](/docs/id/sandboxing#mask-credentials) mengganti nilai nyata pada permintaan keluar. Claude Code melindungi hanya entri yang Anda cantumkan; tidak ada daftar deny kredensial bawaan.

* **Scope**: [`Any file`](#scopes). Claude Code menghormati entri `mask`, `allowPlaintextInject`, `awsPairs`, dan `sigv4` hanya dari pengaturan pengguna, pengaturan yang dikelola, dan flag `--settings`.
* **Type**: object dengan `files`, `envVars`, `allowPlaintextInject`, `awsPairs`, dan `sigv4`
* **Default**: tidak diatur, jadi tidak ada kredensial yang dilindungi

Ini menyembunyikan file kredensial AWS Anda dan menghapus `GITHUB_TOKEN` dari perintah bersandbox:

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "files": [{ "path": "~/.aws/credentials", "mode": "deny" }],
      "envVars": [{ "name": "GITHUB_TOKEN", "mode": "deny" }]
    }
  }
}
```

Perlindungan file `deny` adalah bagian dari lapisan sistem file, jadi tidak berlaku ketika Anda [menonaktifkan isolasi sistem file](/docs/id/sandboxing#disable-filesystem-isolation); perlindungan variabel lingkungan masih berlaku.

<h4 id="invalid-credential-entries-in-managed-settings">
  Entri kredensial tidak valid dalam pengaturan yang dikelola
</h4>

Ketika entri `sandbox.credentials` yang dikelola gagal validasi, Claude Code terus melindungi kredensial di mana pun dapat:

* Entri di `files` atau `envVars` yang masih memiliki `path` atau `name` yang valid dan `mode` dari `mask` atau `deny`, seperti yang `extract` pola tidak memiliki grup penangkap, diturunkan ke `mode: "deny"` dengan peringatan, jadi kredensial tetap diblokir, bukan disamarkan, sampai Anda memperbaiki entri. Entri `files` yang diturunkan pin [`filesystem.disabled`](/docs/id/sandboxing#disable-filesystem-isolation) seperti entri `deny` eksplisit, dan peringatan mencatat bahwa blok bacanya tidak diberlakukan jika pengaturan yang dikelola mematikan isolasi sistem file.
* Entri dengan `mode` yang tidak dikenal atau `path` atau `name` yang tidak valid dihapus.
* Setiap kasus memperingatkan; apakah entri diturunkan atau dihapus, entri valid yang tersisa masih diberlakukan, dan nilai `credentials` yang sepenuhnya tidak valid dijatuhkan sementara sisa `sandbox` masih berlaku.

Berlaku di v2.1.191 dan lebih baru; sebelum v2.1.221, setiap entri tidak valid dihapus. Untuk kunci yang dikelola lainnya dengan penanganan per-bidang, lihat [Invalid entries in managed settings](/docs/id/managed-settings#invalid-entries-in-managed-settings).

<h3 id="sandbox-credentials-files">
  `sandbox.credentials.files`
</h3>

Lindungi file atau direktori kredensial dari perintah bersandbox. Dengan `"mode": "deny"`, Claude Code memblokir pembacaan jalur di dalam sandbox, blok baca yang sama dengan [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread). Dengan `"mode": "mask"`, perintah bersandbox di Linux dan WSL2 membaca salinan sentinel file, dan proxy sandbox mengganti nilai nyata pada permintaan keluar ke `injectHosts` entri itu; di macOS file tidak dapat dibaca di dalam sandbox sebagai gantinya. `"mode": "mask"` memerlukan Claude Code v2.1.221 atau lebih baru.

* **Scope**: [`Any file`](#scopes). Claude Code menjatuhkan entri `mask` dari `.claude/settings.json` proyek dan `.claude/settings.local.json` lokal.
* **Type**: array objek, masing-masing dengan `path` dan `mode` dari `"deny"` atau `"mask"`, ditambah [mask fields for files](#mask-fields-for-files) opsional
* **Default**: tidak diatur, jadi tidak ada file kredensial yang dilindungi

Ini menyembunyikan file kredensial AWS Anda dan menyamarkan file host `gh`, mengganti nilai nyata hanya pada permintaan ke `api.github.com`:

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "files": [
        { "path": "~/.aws/credentials", "mode": "deny" },
        { "path": "~/.config/gh/hosts.yml", "mode": "mask", "injectHosts": ["api.github.com"] }
      ]
    }
  }
}
```

Jalur menggunakan [prefixes](#sandbox-path-prefixes) yang sama dengan pengaturan `sandbox.filesystem.*`, dan Claude Code menggabungkan array dari setiap scope pengaturan yang dimuat sesi. [Protect credentials](/docs/id/sandboxing#protect-credentials) mencakup apa yang masih berlaku dari sumber yang Anda kecualikan dengan `--setting-sources`. `mask` entri memerlukan Claude Code v2.1.221 atau lebih baru.

Substitusi `mask` berjalan hanya melalui proxy sandbox, jadi atur [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate), atau [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject) untuk jaringan uji HTTP biasa. `mask` berlaku untuk file tunggal, jadi cantumkan setiap file kredensial secara terpisah. Claude Code menerima tetapi mengabaikan bidang `mask` pada entri `deny`. [Mask credential files](/docs/id/sandboxing#mask-credential-files) mencakup sumber pengaturan mana yang dihormati dan kapan entri kembali ke `deny`.

<span id="sandbox-credentials-files-extract" />

<span id="sandbox-credentials-files-onextractnomatch" />

<span id="sandbox-credentials-files-decode" />

<span id="sandbox-credentials-files-maskclaims" />

<span id="sandbox-credentials-files-maskduplicates" />

<span id="sandbox-credentials-files-injecthosts" />

<h4 id="mask-fields-for-files">
  Bidang mask untuk file
</h4>

Entri `mask` menerima bidang opsional ini. Tanpa `extract` atau `decode`, Claude Code mengganti seluruh konten file dengan satu sentinel. Di macOS dengan isolasi sistem file aktif, Claude Code menerapkan entri `mask` sebagai `deny` sebelum `extract` atau `decode` berjalan; lihat [Mask credential files](/docs/id/sandboxing#mask-credential-files).

| Bidang             | Tipe                                                                                                                    | Apa yang dilakukannya                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :----------------- | :---------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `extract`          | string, ekspresi reguler dengan setidaknya satu grup penangkap                                                          | Samarkan hanya teks yang ditangkap oleh grup 1 dari setiap kecocokan, jadi sisa file tetap dapat diurai. Dengan `decode` juga diatur, Claude Code memeriksa setiap penangkapan sebagai JWT yang mungkin daripada menggantinya langsung. Memerlukan v2.1.221 atau lebih baru                                                                                                                                                                                                                                                                                                                                                                                        |
| `onExtractNoMatch` | `"warn"`, `"deny"`, atau `"error"`; default `"warn"`                                                                    | Apa yang terjadi ketika `extract` atau `decode` tidak menemukan apa pun untuk disamarkan. `warn` membiarkan file dapat dibaca apa adanya di dalam sandbox, `deny` membuatnya tidak dapat dibaca, dan `error` menghentikan penyiapan sandbox sampai Anda memperbaiki konfigurasi. Claude Code memperlakukan `deny` sebagai `error` ketika blok baca tidak akan diberlakukan, karena Anda [menonaktifkan isolasi sistem file](/docs/id/sandboxing#disable-filesystem-isolation) atau entri [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread) membuka kembali jalur. Memerlukan v2.1.221 atau lebih baru; kasus `decode` memerlukan v2.1.224 atau lebih baru |
| `decode`           | string `"jwt"`                                                                                                          | Temukan JSON Web Tokens (JWTs) di file, dengan pola bawaan atau dengan `extract` ketika diatur, verifikasi setiap kandidat, dan ganti dengan token palsu yang valid secara struktural, jadi kode di dalam sandbox yang mendekode token tetap bekerja. Ketika tidak ada kandidat yang memverifikasi, `onExtractNoMatch` mengatur hasil. Memerlukan v2.1.224 atau lebih baru                                                                                                                                                                                                                                                                                         |
| `maskClaims`       | array string, setidaknya satu nama klaim; memerlukan `decode`                                                           | Samarkan hanya klaim payload tingkat atas yang dinamai di dalam setiap JWT yang diverifikasi dan bangun kembali token di sekitar payload yang dimodifikasi, jadi klaim lain tetap dapat dibaca. Ketika tidak ada klaim bernama yang cocok, `onExtractNoMatch` mengatur hasil. Memerlukan v2.1.224 atau lebih baru                                                                                                                                                                                                                                                                                                                                                  |
| `maskDuplicates`   | Boolean, default `false`                                                                                                | Juga ganti salinan verbatim dari setiap nilai yang disamarkan di tempat lain di file, seperti rahasia yang ditempel ke dalam komentar. Claude Code cocok dengan substring mentah, jadi cadangkan untuk rahasia panjang dengan entropi tinggi. Dikonsultasikan hanya ketika `extract` atau `decode` diatur. Memerlukan v2.1.221 atau lebih baru                                                                                                                                                                                                                                                                                                                     |
| `injectHosts`      | array string, masing-masing host yang [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) juga mengakui | Perkecil host tempat proxy sandbox mengganti nilai nyata. Ketika tidak diatur, proxy mengganti pada permintaan ke setiap host di `sandbox.network.allowedDomains`. Memerlukan v2.1.221 atau lebih baru                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

Ini menyamarkan hanya nilai `oauth_token` di file host `gh`, mengganti setiap salinan lain darinya di file, membuat file tidak dapat dibaca jika pola tidak cocok dengan apa pun, dan mengganti token nyata hanya pada permintaan ke `api.github.com`:

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "files": [
        {
          "path": "~/.config/gh/hosts.yml",
          "mode": "mask",
          "extract": "oauth_token:\\s*(\\S+)",
          "maskDuplicates": true,
          "onExtractNoMatch": "deny",
          "injectHosts": ["api.github.com"]
        }
      ]
    }
  }
}
```

<h3 id="sandbox-credentials-envvars">
  `sandbox.credentials.envVars`
</h3>

Lindungi variabel lingkungan dari perintah bersandbox. Dengan `"mode": "deny"`, Claude Code menghapus variabel dari lingkungan perintah bersandbox. Dengan `"mode": "mask"`, perintah bersandbox melihat nilai sentinel per-sesi, dan proxy sandbox mengganti nilai nyata pada permintaan keluar ke `injectHosts` entri itu, jadi alat seperti `gh` dan `npm` terus mengautentikasi tanpa pernah memegang kredensial nyata. `"mode": "mask"` memerlukan Claude Code v2.1.199 atau lebih baru.

* **Scope**: [`Any file`](#scopes). Claude Code menjatuhkan entri `mask` dari `.claude/settings.json` proyek dan `.claude/settings.local.json` lokal.
* **Type**: array objek, masing-masing dengan `name` dan `mode` dari `"deny"` atau `"mask"`, ditambah [mask fields for environment variables](#mask-fields-for-environment-variables) opsional
* **Default**: tidak diatur, jadi tidak ada variabel lingkungan yang dilindungi

Ini menghapus `NPM_TOKEN` dari perintah bersandbox dan menyamarkan `GITHUB_TOKEN`, mengganti nilai nyata hanya pada permintaan ke `api.github.com`:

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "envVars": [
        { "name": "NPM_TOKEN", "mode": "deny" },
        { "name": "GITHUB_TOKEN", "mode": "mask", "injectHosts": ["api.github.com"] }
      ]
    }
  }
}
```

`name` harus dimulai dengan huruf atau garis bawah dan berisi hanya huruf, digit, dan garis bawah. Claude Code menggabungkan array dari setiap scope pengaturan yang dimuat sesi, dan menerapkan `deny` ketika variabel yang sama muncul dengan kedua mode. [Protect credentials](/docs/id/sandboxing#protect-credentials) mencakup apa yang masih berlaku dari sumber yang Anda kecualikan dengan `--setting-sources`. `mask` entri memerlukan Claude Code v2.1.199 atau lebih baru.

Substitusi `mask` berjalan hanya melalui proxy sandbox, jadi atur [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate), atau [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject) untuk jaringan uji HTTP biasa; lihat [Mask environment variables](/docs/id/sandboxing#mask-environment-variables). Claude Code menerima tetapi mengabaikan bidang `mask` pada entri `deny`.

<span id="sandbox-credentials-envvars-extract" />

<span id="sandbox-credentials-envvars-onextractnomatch" />

<span id="sandbox-credentials-envvars-decode" />

<span id="sandbox-credentials-envvars-maskclaims" />

<span id="sandbox-credentials-envvars-injecthosts" />

<h4 id="mask-fields-for-environment-variables">
  Bidang mask untuk variabel lingkungan
</h4>

Entri `mask` menerima bidang opsional ini. Tanpa `extract` atau `decode`, Claude Code mengganti seluruh nilai dengan satu sentinel. `extract` dan `decode` tidak dapat digabungkan pada entri yang sama.

| Bidang             | Tipe                                                                                                                    | Apa yang dilakukannya                                                                                                                                                                                                                                                                                                                                                                                               |
| :----------------- | :---------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `extract`          | string, ekspresi reguler dengan setidaknya satu grup penangkap                                                          | Samarkan hanya teks yang ditangkap oleh grup 1 dari setiap kecocokan, seperti kata sandi di dalam string koneksi `DATABASE_URL`, jadi sisa nilai tetap dapat diurai. Memerlukan v2.1.224 atau lebih baru                                                                                                                                                                                                            |
| `onExtractNoMatch` | `"warn"`, `"deny"`, atau `"error"`; default `"warn"`. Pada entri dengan `decode`, hanya `"warn"` yang diterima          | Apa yang terjadi ketika `extract` tidak cocok dengan apa pun. `warn` meneruskan variabel tanpa topeng, `deny` membatalkan pengaturannya di dalam sandbox, dan `error` menghentikan penyiapan sandbox sampai Anda memperbaiki konfigurasi. Memerlukan v2.1.224 atau lebih baru                                                                                                                                       |
| `decode`           | string `"jwt"`                                                                                                          | Verifikasi seluruh nilai adalah JWT dan ganti dengan token palsu yang valid secara struktural, jadi kode di dalam sandbox yang mendekode token tetap bekerja; proxy mengganti seluruh token nyata pada egress. Nilai yang tidak memverifikasi meneruskan tanpa topeng dengan peringatan. Memerlukan v2.1.224 atau lebih baru                                                                                        |
| `maskClaims`       | array string, setidaknya satu nama klaim; memerlukan `decode`                                                           | Samarkan hanya klaim payload tingkat atas yang dinamai di dalam JWT yang didekode dan bangun kembali token di sekitar payload yang dimodifikasi, jadi klaim lain tetap dapat dibaca. Ketika tidak ada klaim bernama yang cocok, variabel meneruskan tanpa topeng dengan peringatan. Memerlukan v2.1.224 atau lebih baru                                                                                             |
| `injectHosts`      | array string, masing-masing host yang [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) juga mengakui | Perkecil host tempat proxy sandbox mengganti nilai nyata. Ketika tidak diatur, proxy mengganti pada permintaan ke setiap host di `sandbox.network.allowedDomains`. Tulis tujuan IPv6 sebagai alamat terkompresi telanjang, seperti `"::1"`, bukan bentuk yang diberi tanda kurung; lihat [IPv6 destinations in `injectHosts`](/docs/id/sandboxing#ipv6-destinations-in-injecthosts). Memerlukan v2.1.199 atau lebih baru |

Ini menyamarkan hanya kata sandi di dalam `DATABASE_URL`, membatalkan pengaturan variabel jika pola tidak cocok dengan apa pun, dan menyamarkan JWT di `SERVICE_JWT` sambil meninggalkan setiap klaim kecuali `api_key` dapat dibaca:

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "envVars": [
        {
          "name": "DATABASE_URL",
          "mode": "mask",
          "extract": "://[^:]+:([^@]+)@",
          "onExtractNoMatch": "deny"
        },
        {
          "name": "SERVICE_JWT",
          "mode": "mask",
          "decode": "jwt",
          "maskClaims": ["api_key"]
        }
      ]
    }
  }
}
```

<h3 id="sandbox-credentials-allowplaintextinject">
  `sandbox.credentials.allowPlaintextInject`
</h3>

Izinkan substitusi `mask` pada permintaan HTTP biasa serta HTTPS yang dihentikan TLS. Pada HTTP biasa identitas hulu tidak diverifikasi dan kredensial berjalan dalam cleartext, jadi matikan di luar jaringan uji terpercaya. Memerlukan Claude Code v2.1.199 atau lebih baru.

* **Scope**: [`User or managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code memungkinkan substitusi `mask` pada permintaan HTTP biasa serta HTTPS yang dihentikan TLS
  * `false`: Claude Code memungkinkan substitusi `mask` hanya pada HTTPS yang dihentikan TLS
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "allowPlaintextInject": true
    }
  }
}
```

Memerlukan Claude Code v2.1.199 atau lebih baru.

<h3 id="sandbox-credentials-awspairs">
  `sandbox.credentials.awsPairs`
</h3>

Kelompokkan variabel lingkungan yang disamarkan yang membentuk satu kredensial AWS untuk [SigV4 re-signing](/docs/id/sandboxing#re-sign-aws-requests) ketika kredensial Anda hidup dalam variabel dengan nama non-standar. Claude Code menghubungkan trio konvensional `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, dan `AWS_SESSION_TOKEN` secara otomatis ketika Anda menyamarkan nilai seluruhnya, jadi Anda memerlukan kunci ini hanya untuk nama lain. Memerlukan Claude Code v2.1.224 atau lebih baru.

* **Scope**: [`User or managed`](#scopes)
* **Type**: array objek, masing-masing dengan `accessKeyIdVar`, `secretAccessKeyVar`, dan opsional `sessionTokenVar`, penamaan entri `sandbox.credentials.envVars`
* **Default**: tidak diatur, jadi hanya trio konvensional yang dipasangkan

Ini menghubungkan tiga variabel bernama kustom menjadi satu kredensial AWS untuk re-signing:

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "awsPairs": [
        {
          "accessKeyIdVar": "MY_KEY_ID",
          "secretAccessKeyVar": "MY_SECRET_KEY",
          "sessionTokenVar": "MY_SESSION_TOKEN"
        }
      ]
    }
  }
}
```

Setiap variabel bernama harus menjadi entri `mask` nilai-seluruh di [`sandbox.credentials.envVars`](#sandbox-credentials-envvars), tanpa `extract` atau `decode`, dan hanya dapat mengisi satu slot di semua pasangan.

<h3 id="sandbox-credentials-sigv4">
  `sandbox.credentials.sigv4`
</h3>

Pilih apa yang dilakukan proxy sandbox dengan bentuk permintaan AWS yang [tidak dapat di-re-sign](/docs/id/sandboxing#re-sign-aws-requests): `streaming` untuk unggahan streaming aws-chunked, `presigned` untuk URL yang sudah ditandatangani sebelumnya, dan `sigv4a` untuk tanda tangan asimetris SigV4A. Ini berlaku hanya untuk permintaan yang ditandatangani dengan ID kunci akses placeholder pasangan yang disamarkan. Memerlukan Claude Code v2.1.224 atau lebih baru.

* **Scope**: [`User or managed`](#scopes)
* **Type**: object dengan `streaming`, `presigned`, dan `sigv4a`, masing-masing satu dari:
  * `"deny"`: proxy gagal permintaan
  * `"passthrough"`: proxy meneruskan permintaan yang ditandatangani dengan placeholder yang disamarkan, jadi alat menerima penolakan AWS sendiri
* **Default**: tidak diatur, jadi setiap bentuk adalah `"deny"`

Ini meneruskan unggahan streaming daripada gagal di proxy:

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "sigv4": {
        "streaming": "passthrough"
      }
    }
  }
}
```

Dengan `deny`, proxy gagal permintaan. Dengan `passthrough`, proxy meneruskan permintaan dengan tanda tangannya dihitung dari placeholder yang disamarkan, jadi AWS menolaknya dan alat pemanggil menerima respons AWS sendiri daripada kesalahan proxy.

<h3 id="sandbox-network">
  `sandbox.network`
</h3>

Kontrol host, port, dan soket mana yang dapat dijangkau perintah bersandbox. Sandbox merutekan lalu lintas keluar melalui proxy yang memberlakukan daftar ini; lihat [Network isolation](/docs/id/sandboxing#network-isolation) untuk cara proxy memutuskan dan kapan meminta.

* **Scope**: [`Any file`](#scopes). `strictAllowlist`, `allowManagedDomainsOnly`, dan `tlsTerminate` dibaca dari sumber lebih sedikit, seperti entri mereka katakan.
* **Type**: object dengan sub-kunci di bawah
* **Default**: tidak diatur, jadi tidak ada domain yang pra-diizinkan dan sandbox meminta untuk setiap host baru

Ini pra-mengizinkan GitHub dan npm, memblokir `uploads.github.com`, dan memungkinkan perintah mengikat ke localhost:

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"],
      "deniedDomains": ["uploads.github.com"],
      "allowLocalBinding": true
    }
  }
}
```

Claude Code menggabungkan sub-kunci array di scope pengaturan dan menghilangkan duplikat, jadi proyek dapat menambahkan domain ke daftar pengguna Anda. Aturan izin `WebFetch(domain:...)` allow dan deny [permission rules](/docs/id/sandboxing#permission-rules) memberi makan daftar allow dan deny yang sama.

<h3 id="sandbox-network-allowunixsockets">
  `sandbox.network.allowUnixSockets`
</h3>

Cantumkan jalur soket Unix yang dapat dihubungkan perintah bersandbox di macOS. Claude Code mengabaikan daftar ini di Linux dan WSL2, di mana filter seccomp tidak dapat memeriksa jalur soket; gunakan [`allowAllUnixSockets`](#sandbox-network-allowallunixsockets) sebagai gantinya.

* **Scope**: [`Any file`](#scopes)
* **Type**: array string, masing-masing jalur soket
* **Default**: tidak diatur, jadi sandbox macOS memblokir setiap soket Unix

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowUnixSockets": ["~/.ssh/agent-socket"]
    }
  }
}
```

Jalur soket dapat memberikan akses luas: mengizinkan `/var/run/docker.sock`, misalnya, memungkinkan perintah bersandbox mengontrol daemon Docker. Lihat [Security limitations](/docs/id/sandboxing#security-limitations).

<h3 id="sandbox-network-allowallunixsockets">
  `sandbox.network.allowAllUnixSockets`
</h3>

Biarkan perintah bersandbox terhubung ke setiap soket Unix. Di Linux dan WSL2, [seccomp filter](/docs/id/sandboxing#set-up-linux-and-wsl2) sandbox memblokir panggilan `socket(AF_UNIX, ...)`, jadi ini adalah satu-satunya cara untuk mengizinkan soket Unix di sana. Ketika filter hilang, yang dilaporkan `/sandbox` di tab Dependencies, sandbox tidak memblokir panggilan soket-Unix. Lihat [Set up Linux and WSL2](/docs/id/sandboxing#set-up-linux-and-wsl2) untuk tempat filter berasal.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: perintah bersandbox dapat terhubung ke setiap soket Unix
  * `false`: sandbox memblokir koneksi soket-Unix: di macOS kecuali jalur di `allowUnixSockets`, dan di Linux dan WSL2 melalui filter seccomp ketika ada
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowAllUnixSockets": true
    }
  }
}
```

Di WSL2, `true` juga membuka kembali soket interop yang meluncurkan binari Windows seperti `cmd.exe` dan `powershell.exe`.

<h3 id="sandbox-network-allowlocalbinding">
  `sandbox.network.allowLocalBinding`
</h3>

Biarkan perintah bersandbox mengikat ke port localhost di macOS, misalnya untuk memulai server dev.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: perintah bersandbox dapat mengikat ke port localhost di macOS
  * `false`: perintah bersandbox di macOS tidak dapat mengikat ke port localhost
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowLocalBinding": true
    }
  }
}
```

<h3 id="sandbox-network-allowmachlookup">
  `sandbox.network.allowMachLookup`
</h3>

Cantumkan nama layanan XPC dan Mach tambahan yang dapat dicari sandbox macOS. Alat yang berkomunikasi melalui XPC, seperti iOS Simulator atau Playwright, memerlukan layanan mereka tercantum di sini.

* **Scope**: [`Any file`](#scopes)
* **Type**: array string, masing-masing nama layanan; garis miring trailing tunggal `*` cocok dengan awalan, dan `"*"` saja cocok dengan setiap layanan
* **Default**: tidak diatur

Ini memungkinkan setiap layanan di bawah awalan `com.apple.coresimulator.`:

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowMachLookup": ["com.apple.coresimulator.*"]
    }
  }
}
```

<h3 id="sandbox-network-alloweddomains">
  `sandbox.network.allowedDomains`
</h3>

Pra-izinkan domain untuk lalu lintas keluar dari perintah bersandbox, jadi sandbox tidak meminta mereka. Wildcard seperti `*.example.com` cocok dengan subdomain, dan akhiran `:port` opsional membatasi entri ke satu port; entri tanpa port cocok dengan setiap port.

* **Scope**: [`Any file`](#scopes). Hanya pengaturan yang dikelola ketika [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) diatur.
* **Type**: array string, masing-masing domain, pola wildcard, atau literal IP, dengan akhiran `:port` opsional
* **Default**: tidak diatur, jadi sandbox meminta pertama kali perintah menjangkau host baru

Ini pra-mengizinkan GitHub di setiap port, setiap subdomain npm, dan satu host API di port 443 saja:

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org", "api.example.com:443"]
    }
  }
}
```

Tulis literal IPv6 dengan tanda kurung, dengan port opsional: `"[::1]"` memungkinkan setiap port dan `"[::1]:443"` satu port. Bentuk yang diberi tanda kurung memerlukan Claude Code v2.1.229 atau lebih baru. Lihat [IPv6 addresses in domain lists](/docs/id/sandboxing#ipv6-addresses-in-domain-lists).

<h3 id="sandbox-network-denieddomains">
  `sandbox.network.deniedDomains`
</h3>

Blokir domain untuk lalu lintas keluar dari perintah bersandbox, menggunakan wildcard, port, dan sintaks IPv6 yang sama dengan [`allowedDomains`](#sandbox-network-alloweddomains). Domain yang ditolak tetap diblokir bahkan ketika entri `allowedDomains` cocok juga.

* **Scope**: [`Any file`](#scopes)
* **Type**: array string, masing-masing domain, pola wildcard, atau literal IP, dengan akhiran `:port` opsional
* **Default**: tidak diatur

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "deniedDomains": ["sensitive.cloud.example.com"]
    }
  }
}
```

Claude Code menggabungkan daftar ini dari setiap sumber pengaturan yang dimuat sesi bahkan ketika `allowManagedDomainsOnly` diatur, jadi pengembang selalu dapat memperketat daftar deny. Untuk literal IPv6, lihat [IPv6 addresses in domain lists](/docs/id/sandboxing#ipv6-addresses-in-domain-lists).

Entri yang ditulis dengan titik trailing yang menandai nama domain yang sepenuhnya memenuhi syarat, seperti `example.com.`, memblokir koneksi yang sama dengan `example.com`.

<h3 id="sandbox-network-strictallowlist">
  `sandbox.network.strictAllowlist`
</h3>

Tolak akses perintah bersandbox ke host di luar daftar izin daripada meminta persetujuan. Daftar izin adalah [`allowedDomains`](#sandbox-network-alloweddomains) ditambah domain dari aturan allow `WebFetch(domain:...)`, atau hanya entri pengaturan yang dikelola ketika [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) diatur. Memerlukan Claude Code v2.1.219 atau lebih baru.

* **Scope**: [`User or managed`](#scopes). Repositori tidak dapat mengaktifkan atau menonaktifkannya.
* **Type**: Boolean
  * `true`: Claude Code menolak akses perintah bersandbox ke host di luar daftar izin
  * `false`: kecuali file pengaturan terpercaya lain menetapkan `true`, Claude Code memutuskan host di luar daftar izin dengan mode izin daripada menolaknya langsung: dalam mode otomatis memeriksa host terhadap [per-command allowed domains](/docs/id/sandboxing#per-command-allowed-domains-in-auto-mode), dalam mode `dontAsk` menolak, dalam mode `bypassPermissions` dan dalam sesi plan mode interaktif terminal di mana bypass tersedia memungkinkan, dan sebaliknya meminta Anda
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "strictAllowlist": true
    }
  }
}
```

Claude Code memberlakukan ini untuk perintah bersandbox saja; alat dalam proses seperti `WebFetch` masih mengikuti [permission rules](/docs/id/sandboxing#permission-rules) mereka. Ketika salah satu sumber yang dihormati menetapkannya ke `true`, itu tetap aktif. Lihat [Network isolation](/docs/id/sandboxing#network-isolation). Memerlukan Claude Code v2.1.219 atau lebih baru.

<h3 id="sandbox-network-allowmanageddomainsonly">
  `sandbox.network.allowManagedDomainsOnly`
</h3>

Kunci daftar izin jaringan ke apa yang didefinisikan pengaturan yang dikelola. Claude Code kemudian menghormati hanya `allowedDomains` dan aturan allow `WebFetch(domain:...)` dari pengaturan yang dikelola, mengabaikan domain dari pengaturan pengguna, proyek, lokal, dan `--settings`, dan memblokir domain yang tidak diizinkan secara otomatis daripada meminta.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code menghormati hanya `allowedDomains` dan aturan allow `WebFetch(domain:...)` dari pengaturan yang dikelola dan memblokir domain yang tidak diizinkan daripada meminta
  * `false`: domain dari pengaturan pengguna, proyek, lokal, dan `--settings` menggabungkan ke dalam daftar izin
* **Default**: `false`

Ini mengunci daftar izin ke GitHub dan npm dan mengabaikan domain apa pun yang ditambahkan pengembang:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowManagedDomainsOnly": true,
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

Domain yang ditolak masih menggabungkan dari setiap sumber yang dimuat sesi. Lihat [Keep developers from widening the policy](/docs/id/sandboxing#keep-developers-from-widening-the-policy).

<h3 id="sandbox-network-httpproxyport">
  `sandbox.network.httpProxyPort`
</h3>

Arahkan sandbox ke proxy HTTP Anda sendiri daripada yang dijalankan Claude Code. Organisasi melakukan ini untuk memeriksa lalu lintas HTTPS, menerapkan aturan penyaringan mereka sendiri, atau mencatat setiap permintaan. Ketika tidak diatur, Claude Code memulai proxy sendiri untuk lalu lintas HTTP.

* **Scope**: [`Any file`](#scopes)
* **Type**: number, port TCP lokal
* **Default**: tidak diatur, jadi Claude Code menjalankan proxy sendiri

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080
    }
  }
}
```

Atur [`socksProxyPort`](#sandbox-network-socksproxyport) juga jika proxy Anda harus membawa lalu lintas SOCKS juga; dengan hanya satu dari dua yang diatur, Claude Code masih menjalankan proxy sendiri untuk protokol lainnya. Lihat [Custom proxy configuration](/docs/id/sandboxing#custom-proxy-configuration).

<h3 id="sandbox-network-socksproxyport">
  `sandbox.network.socksProxyPort`
</h3>

Arahkan sandbox ke proxy SOCKS5 Anda sendiri daripada yang dijalankan Claude Code. Ketika tidak diatur, Claude Code memulai proxy sendiri untuk lalu lintas SOCKS.

* **Scope**: [`Any file`](#scopes)
* **Type**: number, port TCP lokal
* **Default**: tidak diatur, jadi Claude Code menjalankan proxy sendiri

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "socksProxyPort": 8081
    }
  }
}
```

Lihat [Custom proxy configuration](/docs/id/sandboxing#custom-proxy-configuration).

<h3 id="sandbox-network-tlsterminate">
  `sandbox.network.tlsTerminate`
</h3>

Buat proxy sandbox menghentikan TLS sehingga dapat membaca konten permintaan HTTPS. Ini bersifat eksperimental, dan [credential substitution](/docs/id/sandboxing#mask-credentials) `mask` memerlukan ini. Atur `{}` untuk menghasilkan otoritas sertifikat sementara untuk sesi, atau atur `caCertPath` dan `caKeyPath` untuk menggunakan milik Anda sendiri.

* **Scope**: [`User or managed`](#scopes). Repositori tidak dapat mengaktifkannya atau menyediakan otoritas sertifikat.
* **Type**: object dengan string `caCertPath` dan `caKeyPath` opsional, masing-masing jalur file
* **Default**: tidak diatur, jadi proxy tidak menghentikan atau memeriksa TLS

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "tlsTerminate": {}
    }
  }
}
```

Ketika lebih dari satu sumber yang dihormati menetapkannya, Claude Code menggunakan nilai dari sumber dengan prioritas tertinggi: pengaturan yang dikelola, kemudian flag `--settings`, kemudian pengaturan pengguna. Memerlukan Claude Code v2.1.199 atau lebih baru.

<span id="context-and-memory" />

<h2 id="memory-and-context">
  Memori dan konteks
</h2>

Kontrol apa yang dimuat Claude Code ke dalam konteks, bagaimana cara mengompaknya, dan di mana menyimpan memori dan rencana. Lihat [Kelola konteks](/docs/id/context-window) dan [Memori](/docs/id/memory).

<h3 id="autocompactenabled">
  `autoCompactEnabled`
</h3>

Buat Claude Code [mengompak percakapan secara otomatis](/docs/id/context-window#when-your-context-fills-up) ketika konteks mendekati batas. Muncul di `/config` sebagai **Auto-compact**, dan mengalihkannya di sana menulis kunci ini ke pengaturan pengguna Anda.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code mengompak percakapan secara otomatis ketika konteks mendekati batas
  * `false`: Claude Code tidak mengompak secara otomatis
* **Default**: `true`
* **Per-session overrides**: [`DISABLE_AUTO_COMPACT`](/docs/id/env-vars) mematikan auto-compact untuk satu sesi; mana pun dari keduanya yang mematikannya, yang lain tidak dapat menghidupkannya kembali

```json settings.json theme={null}
{
  "autoCompactEnabled": false
}
```

Perintah manual `/compact` terus bekerja saat auto-compact dimatikan.

<h3 id="autocompactwindow">
  `autoCompactWindow`
</h3>

Atur seberapa penuh jendela konteks sebelum Claude Code [mengompak secara otomatis](/docs/id/context-window#when-your-context-fills-up).

* **Scope**: [`Any file`](#scopes)
* **Type**: jumlah token, dari `100000` hingga `1000000`. Claude Code membatasi nilai pada jendela konteks model Anda; [ringkasan model](https://platform.claude.com/docs/en/about-claude/models/overview) mencantumkan jendela setiap model
* **Default**: tidak diatur, jadi Claude Code memilih jendela yang disesuaikan untuk model Anda
* **Per-session overrides**: [`--autocompact`](/docs/id/cli-reference#cli-flags) mengambil alih kunci ini untuk satu sesi, dan [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/id/env-vars) mengambil alih keduanya

```json settings.json theme={null}
{
  "autoCompactWindow": 500000
}
```

Atur dengan perintah [`/autocompact`](/docs/id/commands#all-commands), yang menulis kunci ini ke pengaturan pengguna Anda. [Atur jendela auto-compact](/docs/id/model-config#set-the-auto-compact-window) mencakup cara perintah, flag, variabel, dan pengaturan berinteraksi.

<h3 id="automemorydirectory">
  `autoMemoryDirectory`
</h3>

Simpan [memori otomatis](/docs/id/memory#storage-location) di direktori pilihan Anda alih-alih default per-proyek.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, jalur direktori absolut atau dengan awalan `~/`
* **Default**: tidak diatur, jadi Claude Code menggunakan `~/.claude/projects/<project>/memory/`

```json settings.json theme={null}
{
  "autoMemoryDirectory": "~/my-memory-dir"
}
```

Dari pengaturan proyek atau lokal, Claude Code menghormati kunci ini di bawah [aturan kepercayaan ruang kerja yang sama dengan hooks](/docs/id/permissions#what-runs-before-you-trust-a-folder), karena repositori yang dikloning dapat menyediakan file-file tersebut.

<h3 id="automemoryenabled">
  `autoMemoryEnabled`
</h3>

Hidupkan atau matikan [memori otomatis](/docs/id/memory#enable-or-disable-auto-memory). Ketika `false`, Claude tidak membaca dari atau menulis ke direktori memori otomatis. Anda juga dapat mengalihkannya dengan `/memory` selama sesi, yang menulis kunci ini ke pengaturan pengguna Anda.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: sama dengan tidak diatur; memori otomatis tetap aktif kecuali sesuatu yang mengungguli kunci ini mematikannya untuk sesi, seperti `--bare`, mode aman, atau `CLAUDE_CODE_DISABLE_AUTO_MEMORY`
  * `false`: Claude tidak membaca dari atau menulis ke direktori memori otomatis
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_AUTO_MEMORY`](/docs/id/env-vars) mengambil alih kunci ini untuk satu sesi, dalam kedua arah

```json settings.json theme={null}
{
  "autoMemoryEnabled": false
}
```

<h3 id="bashoutputmaxchars">
  `bashOutputMaxChars`
</h3>

Atur berapa banyak karakter dari [output](/docs/id/tools-reference#output-limits) perintah Bash atau PowerShell yang berhasil yang diterima Claude secara inline. Ketika output melampaui batas, Claude Code menyimpannya ke file dan Claude menerima pratinjau singkat ditambah jalur file. Naikkan batas ketika output perintah, seperti build verbose atau log test-suite lengkap, secara rutin melampaui default dan Anda ingin Claude membacanya tanpa membuka file. Memerlukan Claude Code v2.1.261 atau lebih baru.

* **Scope**: [`Any file`](#scopes)
* **Type**: jumlah karakter, bilangan bulat positif. Claude Code menjepit nilai ke dalam rentang `4000` hingga `128000`
* **Default**: tidak diatur, jadi Claude menerima hingga 30.000 karakter secara inline

```json settings.json theme={null}
{
  "bashOutputMaxChars": 100000
}
```

Ketika Anda menetapkan kunci ini, Claude Code mengabaikan variabel lingkungan [`BASH_MAX_OUTPUT_LENGTH`](/docs/id/env-vars).

<h3 id="claudemd">
  `claudeMd`
</h3>

Injeksikan instruksi gaya CLAUDE.md sebagai memori yang dikelola organisasi tanpa menerapkan file terpisah. Claude Code memuat teks sebagai entri memori terkelola di depan file CLAUDE.md pengguna dan proyek.

* **Scope**: [`Managed`](#scopes)
* **Type**: string, teks file CLAUDE.md; tuliskan seperti yang Anda lakukan pada file, Markdown disertakan, dengan jeda baris sebagai `\n`
* **Default**: tidak diatur

Contoh ini menerapkan dua aturan sebagai daftar Markdown pendek:

```json managed-settings.json theme={null}
{
  "claudeMd": "# Engineering rules\n\n- Always run make lint before committing.\n- Never push directly to main."
}
```

Lihat [Terapkan CLAUDE.md di seluruh organisasi](/docs/id/memory#deploy-organization-wide-claude-md).

<h3 id="claudemdexcludes">
  `claudeMdExcludes`
</h3>

Lewati file `CLAUDE.md` tertentu ketika Claude Code memuat [memori](/docs/id/memory#exclude-specific-claude-md-files). Dalam monorepo besar, gunakan untuk melewati file CLAUDE.md dari tim lain yang tidak relevan dengan pekerjaan Anda; [Kecualikan file CLAUDE.md yang tidak relevan](/docs/id/large-codebases#exclude-irrelevant-claude-md-files) dalam panduan codebases besar memandu kasus tersebut. Pola cocok dengan jalur file absolut.

* **Scope**: [`Any file`](#scopes)
* **Type**: array string, masing-masing pola glob atau jalur absolut
* **Default**: tidak diatur, jadi Claude Code memuat setiap CLAUDE.md yang ditemukannya

```json settings.json theme={null}
{
  "claudeMdExcludes": ["**/vendor/**/CLAUDE.md"]
}
```

Pengecualian hanya berlaku untuk file memori pengguna, proyek, dan lokal; file CLAUDE.md kebijakan terkelola tidak dapat dikecualikan.

<span id="environment-variables" />

<h3 id="env">
  `env`
</h3>

Atur variabel lingkungan untuk setiap sesi dan untuk subproses yang dimulai Claude Code darinya. Variabel apa pun dalam [referensi variabel lingkungan](/docs/id/env-vars) dapat masuk di sini, yang merupakan cara Anda menerapkannya ke setiap sesi atau meluncurkannya ke tim Anda. Pengaturan proyek dan lokal tidak dapat menetapkan [beberapa di antaranya](#variables-claude-code-ignores-in-env).

* **Scope**: [`Any file`](#scopes)
* **Type**: objek yang memetakan nama variabel ke nilai string
* **Default**: tidak diatur

Contoh ini mematikan pemadatan otomatis dan merutekan permintaan API melalui proxy:

```json settings.json theme={null}
{
  "env": {
    "DISABLE_AUTO_COMPACT": "1",
    "ANTHROPIC_BASE_URL": "https://proxy.example.com"
  }
}
```

<h4 id="how-env-values-interact-with-your-shell">
  Bagaimana nilai `env` berinteraksi dengan shell Anda
</h4>

* Nilai di sini menimpa variabel yang sama yang diekspor di shell Anda, dan ketika lebih dari satu file pengaturan menetapkan variabel, [yang dengan preseden tertinggi](/docs/id/settings#settings-precedence) berlaku. [Variabel yang Claude Code abaikan dalam `env`](#variables-claude-code-ignores-in-env) mencantumkan pengecualian untuk pengaturan proyek dan lokal.
* Untuk membatalkan ekspor shell, atur variabel ke `""`. Claude Code memperlakukan nilai kosong sebagai tidak diatur untuk pemilihan penyedia, dan subproses mewarisi nilai kosong.
* `NO_COLOR` dan `FORCE_COLOR` yang ditetapkan di sini hanya mencapai subproses. Untuk mengubah warna antarmuka Claude Code sendiri, atur di shell Anda sebelum meluncurkan `claude`.
* Nilai di sini adalah teks biasa dalam file pengaturan dan mencapai setiap subproses yang dimulai Claude Code. Untuk token pembawa OTLP yang berputar, gunakan [`otelHeadersHelper`](#otelheadershelper); untuk kredensial API, gunakan [`apiKeyHelper`](#apikeyhelper).

<h4 id="when-claude-code-applies-env-values">
  Ketika Claude Code menerapkan nilai `env`
</h4>

* Dari pengaturan pengguna, `--settings`, dan pengaturan terkelola: saat startup, dan lagi dalam sesi yang berjalan ketika perubahan tersimpan mengubah `env` yang digabungkan.
* Dari pengaturan proyek dan lokal: setelah Anda mempercayai ruang kerja, atau saat startup dalam mode `-p`, yang tidak pernah menampilkan dialog kepercayaan, dan lagi ketika perubahan tersimpan mengubah `env` yang digabungkan.
* Variabel yang Claude Code klasifikasikan sebagai aman, seperti pemilihan model, batas waktu dan batas, dan toggle fitur: saat startup dari setiap file pengaturan, terlepas dari [variabel yang tidak dapat ditetapkan pengaturan proyek dan lokal dalam `env`](#variables-claude-code-ignores-in-env).
* Setelah Anda [memindahkan sesi dengan `/cd`](/docs/id/permissions#move-the-session-to-another-directory) pada v2.1.246 atau lebih baru: nilai `env` direktori baru, di atas direktori sebelumnya.

<h4 id="variables-claude-code-ignores-in-env">
  Variabel yang Claude Code abaikan dalam `env`
</h4>

* Pengaturan proyek dan lokal tidak dapat menetapkan variabel yang tidak boleh dikontrol repositori yang diperiksa; atur di shell, pengaturan pengguna, atau pengaturan terkelola Anda. Claude Code menghapus masing-masing dan mencatat peringatan yang dapat Anda lihat dengan `claude --debug`. Mereka termasuk:

  * Variabel yang memilih di mana Claude Code menyimpan atau menulis file-nya sendiri: `CLAUDE_CONFIG_DIR`, `CLAUDE_CODE_TMPDIR`, dan variabel direktori sistem operasi seperti `HOME`, `TMPDIR`, `TMP`, `TEMP`, dan keluarga `XDG_*`.
  * Variabel yang mengekspor konten sesi: [`OTEL_LOG_RAW_API_BODIES`](/docs/id/env-vars#variables) dan pasangan pelacakan beta terperinci `ENABLE_BETA_TRACING_DETAILED` dan `BETA_TRACING_ENDPOINT`.
  * Variabel [OpenTelemetry exporter](/docs/id/monitoring-usage) yang menghidupkan telemetri, memilih ke mana telemetri pergi, atau memilih konten apa yang ditangkapnya:

    * `CLAUDE_CODE_ENABLE_TELEMETRY`, ditambah pasangan telemetri yang ditingkatkan beta `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` dan `ENABLE_ENHANCED_TELEMETRY_BETA`
    * Pemilih exporter `OTEL_LOGS_EXPORTER`, `OTEL_METRICS_EXPORTER`, dan `OTEL_TRACES_EXPORTER`
    * Variabel konten `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_ASSISTANT_RESPONSES`, `OTEL_LOG_TOOL_CONTENT`, dan `OTEL_LOG_TOOL_DETAILS`
    * Variabel `OTEL_EXPORTER_OTLP_*` yang namanya berakhir dengan `_ENDPOINT`, `_HEADERS`, `_PROTOCOL`, `_CERTIFICATE`, `_CLIENT_KEY`, atau `_INSECURE`, dalam bentuk generik dan per-sinyal, seperti `OTEL_EXPORTER_OTLP_ENDPOINT` dan `OTEL_EXPORTER_OTLP_METRICS_HEADERS`
    * `OTEL_EXPORTER_PROMETHEUS_HOST` dan `OTEL_EXPORTER_PROMETHEUS_PORT`

    Hanya nilai-nilai ini yang masih berlaku dari pengaturan proyek dan lokal, karena mereka mematikan sesuatu: `none` untuk tiga pemilih exporter, dan nilai off seperti `0` untuk `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_CONTENT`, dan `OTEL_LOG_TOOL_DETAILS`. Nilai seperti itu menimpa variabel yang sama dalam pengaturan pengguna Anda, tetapi bukan yang ditetapkan lingkungan tempat Anda memulai Claude Code, file `--settings`, atau pengaturan terkelola.

    Ketika file pengaturan proyek atau lokal menetapkan variabel dalam grup ini, sesi interaktif lokal menampilkan pemberitahuan saat startup. Jalankan `/status` atau `claude doctor` untuk melihat mana yang Claude Code abaikan dan mana yang mematikan telemetri; keduanya mencantumkan nama, tidak pernah nilai. Jalankan non-interaktif dengan `-p` atau sesi Agent SDK tidak menampilkan pemberitahuan, jadi periksa bahwa kolektor Anda masih menerima data setelah Anda upgrade. Jika tidak, atur variabel dalam pengaturan pengguna Anda, pengaturan terkelola, lingkungan pekerjaan, atau file yang Anda lewatkan dengan `--settings`.

    Mengabaikan grup ini dalam pengaturan proyek dan lokal memerlukan Claude Code v2.1.282 atau lebih baru.
  * Variabel yang mengubah cara Claude Code dimulai atau disinkronkan, seperti `CLAUDE_CODE_PROCESS_WRAPPER`, `CLAUDE_CODE_SYNC_SKILLS`, `CLAUDE_CODE_SYNC_PLUGINS`, `CLAUDE_CODE_PLUGIN_CACHE_DIR`, dan `CLAUDE_CODE_PLUGIN_SEED_DIR`.

  Sebelum v2.1.251, pengaturan proyek dan lokal dapat menetapkan variabel dalam daftar ini yang memilih di mana Claude Code menulis file-nya atau yang mengekspor konten sesi, kecuali `HOME` dan `XDG_CONFIG_HOME`.
* Variabel identitas yang dimiliki lingkungan hosting Claude Code, seperti `CLAUDE_CODE_REMOTE` dan `CLAUDE_CODE_ACCOUNT_UUID`, diabaikan dari setiap file.
* [`CLAUDE_CODE_MESSAGING_SOCKET` dan `CLAUDE_CODE_MESSAGING_TOKEN`](/docs/id/env-vars#variables), yang diekspor Claude Code sendiri, diabaikan dari setiap file. Mengabaikan variabel soket memerlukan Claude Code v2.1.224 atau lebih baru, dan mengabaikan token memerlukan v2.1.228 atau lebih baru.
* [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/id/sessions#name-the-project-directory-yourself), yang dibaca Claude Code hanya dari lingkungan peluncuran, diabaikan dari setiap file; memerlukan v2.1.234 atau lebih baru.
* [`CLAUDE_CODE_RESTRICTED`](/docs/id/env-vars#variables), yang dibaca Claude Code hanya dari lingkungan peluncuran, diabaikan dari setiap file.

<h3 id="filecheckpointingenabled">
  `fileCheckpointingEnabled`
</h3>

Buat Claude Code membuat snapshot file sebelum setiap edit sehingga [`/rewind`](/docs/id/checkpointing) dapat memulihkannya. Muncul di `/config` sebagai **Rewind code (checkpoints)**, dan mengalihkannya di sana menulis kunci ini ke pengaturan pengguna Anda.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code membuat snapshot file sebelum setiap edit sehingga `/rewind` dapat memulihkannya
  * `false`: Claude Code tidak membuat snapshot file, jadi `/rewind` tidak dapat memulihkannya
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_FILE_CHECKPOINTING`](/docs/id/env-vars) mematikan checkpointing untuk satu sesi; mana pun dari keduanya yang mematikannya, yang lain tidak dapat menghidupkannya kembali

```json settings.json theme={null}
{
  "fileCheckpointingEnabled": false
}
```

Dalam jalankan `-p` atau sesi Agent SDK, Claude Code mengabaikan kunci ini. SDK menghidupkan checkpointing dengan opsi `enableFileCheckpointing`-nya, dan jalankan `-p` bare memerlukan `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true`. Lihat [File checkpointing dalam Agent SDK](/docs/id/agent-sdk/file-checkpointing).

<h3 id="plansdirectory">
  `plansDirectory`
</h3>

Pilih di mana Claude Code menyimpan file rencana yang ditulisnya dalam [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode). Claude Code menyelesaikan jalur relatif terhadap akar proyek dan menyimpan default ketika jalur diselesaikan di luarnya.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, jalur relatif terhadap akar proyek
* **Default**: tidak diatur, jadi Claude Code menggunakan `~/.claude/plans`

```json settings.json theme={null}
{
  "plansDirectory": "./plans"
}
```

<h3 id="skilllistingbudgetfraction">
  `skillListingBudgetFraction`
</h3>

Setiap giliran, Claude melihat [daftar keterampilan Anda](/docs/id/skills#skill-descriptions-are-cut-short) dengan deskripsinya, dan Claude Code membatasi daftar itu pada bagian jendela konteks. Ketika daftar melampaui batas, Claude Code menyimpan nama setiap keterampilan tetapi menghapus deskripsi keterampilan yang paling jarang digunakan, sehingga Claude masih dapat memanggil keterampilan tersebut tetapi kurang mungkin memilih satu sendiri. Naikkan kunci ini untuk menyimpan lebih banyak deskripsi yang terlihat dengan mengorbankan lebih banyak konteks per giliran.

* **Scope**: [`Any file`](#scopes)
* **Type**: angka, fraksi lebih besar dari `0` dan paling banyak `1`
* **Default**: `0.01`, yang mereservasi 1% dari jendela konteks

```json settings.json theme={null}
{
  "skillListingBudgetFraction": 0.02
}
```

Untuk melihat berapa banyak konteks yang digunakan daftar dan keterampilan mana yang berkontribusi paling banyak, jalankan `/doctor`.

<h3 id="skilllistingmaxdescchars">
  `skillListingMaxDescChars`
</h3>

Setiap giliran, Claude melihat [daftar keterampilan Anda](/docs/id/skills#skill-descriptions-are-cut-short) yang menampilkan teks `description` dan `when_to_use` setiap keterampilan. Kunci ini membatasi berapa banyak karakter dari teks itu yang ditampilkan Claude Code per keterampilan; teks yang lebih panjang dipotong pada batas.

* **Scope**: [`Any file`](#scopes)
* **Type**: jumlah karakter, bilangan bulat positif
* **Default**: `1536`

```json settings.json theme={null}
{
  "skillListingMaxDescChars": 2048
}
```

Naikkan untuk menyimpan deskripsi panjang utuh dengan mengorbankan lebih banyak konteks per giliran; turunkan untuk menyesuaikan lebih banyak keterampilan di bawah [`skillListingBudgetFraction`](#skilllistingbudgetfraction).

<h3 id="taskoutputmaxchars">
  `taskOutputMaxChars`
</h3>

<Warning>
  Dihapus dalam v2.1.277, bersama dengan alat `TaskOutput` yang diukurnya. Menetapkannya tidak berpengaruh pada versi saat ini. Claude membaca [file output](/docs/id/tools-reference#background-commands) tugas latar belakang dengan `Read` sebagai gantinya.
</Warning>

Melalui v2.1.276, Anda menetapkan kunci ini ke jumlah karakter dari [output tugas latar belakang](/docs/id/tools-reference#background-commands) yang diterima Claude secara inline ketika membacanya dengan alat `TaskOutput`.

<h2 id="interface-and-terminal">
  Interface dan terminal
</h2>

Ubah tampilan dan perilaku Claude Code di terminal Anda: tema, mode editor, status line, spinner, notifikasi dalam sesi, dan aksesibilitas. Lihat [Konfigurasi terminal](/docs/id/terminal-config).

<h3 id="askuserquestiontimeout">
  `askUserQuestionTimeout`
</h3>

Biarkan dialog [`AskUserQuestion`](/docs/id/tools-reference) yang belum dijawab melanjutkan secara otomatis setelah periode waktu idle, mengirimkan opsi apa pun yang telah Anda pilih. Atur ini ketika Anda pergi dan ingin Claude melanjutkan tanpa Anda. Dengan default, pertanyaan menunggu sampai Anda menjawabnya. Memerlukan Claude Code v2.1.200 atau lebih baru.

* **Scope**: [`User or managed`](#scopes)
* **Type**: string, salah satu dari `"60s"`, `"5m"`, `"10m"`, atau `"never"`
* **Default**: `"never"`
* **Per-session overrides**: [`CLAUDE_AFK_TIMEOUT_MS`](/docs/id/env-vars) mengambil alih kunci ini untuk satu sesi

```json settings.json theme={null}
{
  "askUserQuestionTimeout": "5m"
}
```

Muncul di `/config` sebagai **Question auto-continue timeout**, yang menulis kunci ini ke pengaturan pengguna; Claude Code menyembunyikan baris saat pengaturan terkelola atau flag `--settings` mengatur kunci. Memerlukan Claude Code v2.1.200 atau lebih baru.

<h3 id="autocontinueatusagelimit">
  `autoContinueAtUsageLimit`
</h3>

Setelah batas penggunaan claude.ai menghentikan sesi Anda, tunggu di sesi terbuka dan lanjutkan tugas secara otomatis setelah reset. Lihat [Matikan automatic continue](/docs/id/interactive-mode#turn-automatic-continue-off). Memerlukan Claude Code v2.1.234 atau lebih baru.

* **Scope**: [`User or managed`](#scopes). Baca dari pengaturan pengguna, `--settings`, dan pengaturan terkelola saja. Ketika tidak ada yang mengatur kunci, file pengaturan proyek atau lokal yang mengaturnya mematikan fitur daripada diabaikan.
* **Type**: Boolean
  * `true`: setelah batas penggunaan claude.ai menghentikan sesi Anda, Claude Code menunggu di sesi terbuka dan melanjutkan tugas secara otomatis setelah reset
  * `false`: Claude Code tidak memulai penantian dengan sendirinya. Anda masih dapat [memulai penantian sendiri](/docs/id/interactive-mode#start-a-wait-yourself) dari menu opsi batas penggunaan
* **Default**: `true`

```json settings.json theme={null}
{
  "autoContinueAtUsageLimit": false
}
```

Muncul di `/config` sebagai **Continue automatically at usage limit**, yang menulis kunci ini ke pengaturan pengguna; Claude Code menyembunyikan baris saat pengaturan terkelola atau flag `--settings` mengatur kunci.

<h3 id="autoscrollenabled">
  `autoScrollEnabled`
</h3>

Ikuti output baru ke bagian bawah percakapan dalam [fullscreen rendering](/docs/id/fullscreen). Matikan untuk tetap di tempat Anda menggulir sementara Claude terus bekerja; prompt izin masih menggulir ke tampilan.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: percakapan mengikuti output baru ke bagian bawah
  * `false`: Anda tetap di tempat Anda menggulir sementara Claude terus bekerja; prompt izin masih muncul di bawah transkrip
* **Default**: `true`

```json settings.json theme={null}
{
  "autoScrollEnabled": false
}
```

Muncul di `/config` sebagai **Auto-scroll** ketika fullscreen rendering aktif, yang menulis kunci ini ke pengaturan pengguna.

<h3 id="axscreenreader">
  `axScreenReader`
</h3>

Render output yang ramah pembaca layar: teks datar tanpa batas dekoratif atau animasi. Mode pembaca layar menggunakan renderer klasik, jadi pengaturan `tui` tidak berpengaruh saat aktif; [sesi latar belakang](/docs/id/agent-view) yang terlampir masih merender fullscreen.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code merender teks datar tanpa batas dekoratif atau animasi, menggunakan renderer klasik
  * `false`: Claude Code merender secara normal
* **Default**: unset, jadi mode pembaca layar mati
* **Per-session overrides**: [`--ax-screen-reader`](/docs/id/cli-reference#cli-flags) mengambil alih [`CLAUDE_AX_SCREEN_READER`](/docs/id/env-vars), dan keduanya mengambil alih kunci ini untuk satu sesi

```json settings.json theme={null}
{
  "axScreenReader": true
}
```

<h3 id="basheditdiffenabled">
  `bashEditDiffEnabled`
</h3>

Pilih apakah Claude Code mencatat file yang diubah dalam repositori Git saat perintah Bash berjalan. Ketika mencatatnya, Anda melihat diff mereka di terminal setelah perintah, dan [PostToolUse Bash hooks](/docs/id/hooks#bash) Anda menerima daftar file yang diubah.

File yang terdaftar tidak selalu merupakan file yang diubah perintah. Perubahan yang dibuat program lain atau panggilan Bash lain saat perintah berjalan juga dapat muncul di sana.

Atur kunci ke `true` untuk mencatatnya dalam setiap mode izin. Memerlukan Claude Code v2.1.269 atau lebih baru.

* **Scope**: [`User or managed`](#scopes). `true` hanya dihitung dari pengaturan pengguna Anda, JSON yang dilewatkan dengan `--settings`, atau [pengaturan terkelola](/docs/id/managed-settings), jadi `true` dalam `.claude/settings.json` atau `.claude/settings.local.json` repositori tidak dapat mengaktifkan pencatatan. `false` dalam file repositori apa pun masih mematikannya kecuali file [preseden lebih tinggi](/docs/id/settings#settings-precedence) mengatur `true`.
* **Type**: Boolean
* **Default**: unset, jadi Claude Code mencatat perubahan dalam mode auto dan mode `bypassPermissions` ketika mengarahkan Claude untuk mengedit file melalui Bash
* **Per-session overrides**: [`CLAUDE_CODE_BASH_EDIT_DIFF`](/docs/id/env-vars) mengambil alih kunci ini untuk satu sesi

```json settings.json theme={null}
{
  "bashEditDiffEnabled": true
}
```

<h3 id="companyannouncements">
  `companyAnnouncements`
</h3>

Tampilkan pengumuman organisasi Anda kepada pengguna saat startup. Ketika Anda mencantumkan lebih dari satu, Claude Code memilih satu secara acak untuk setiap sesi; pada peluncuran pertama seseorang, ini menampilkan entri pertama.

* **Scope**: [`Any file`](#scopes)
* **Type**: array of strings
* **Default**: unset, jadi tidak ada pengumuman yang ditampilkan

```json settings.json theme={null}
{
  "companyAnnouncements": [
    "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
  ]
}
```

<h3 id="defaultshell">
  `defaultShell`
</h3>

Pilih apakah Bash atau PowerShell menjalankan perintah shell yang Anda ketik dengan [prefix `!`](/docs/id/interactive-mode#shell-mode-with-prefix) di kotak input, yang dijalankan Claude Code secara langsung dan ditambahkan ke sesi.

`"powershell"` hanya berfungsi saat [tool PowerShell](/docs/id/tools-reference#powershell-tool) aktif. Tool ini aktif secara default di Windows tanpa Git Bash, dan di Windows dengan Git Bash untuk akun claude.ai dan Console. Di sesi Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry, serta di macOS, Linux, dan WSL, atur `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` untuk mengaktifkan tool. Atur variabel itu ke `0` untuk mematikan tool.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, salah satu dari:
  * `"bash"`: Claude Code menjalankan perintah `!` Anda di Bash
  * `"powershell"`: Claude Code menjalankan perintah `!` Anda di PowerShell
* **Default**: `"bash"`, atau `"powershell"` di Windows ketika Bash tidak tersedia

```json settings.json theme={null}
{
  "defaultShell": "powershell"
}
```

Jika shell yang Anda beri nama tidak tersedia, Claude Code menggunakan yang lain: `"powershell"` kembali ke Bash ketika tool PowerShell mati, dan `"bash"` kembali ke PowerShell ketika Bash tidak terinstal.

<h3 id="dialogexpiry">
  `dialogExpiry`
</h3>

Atur tenggat waktu untuk dialog yang Claude Code [teruskan ke klien jarak jauh](/docs/id/remote-control#limitations), seperti host Remote Control atau SDK, dan untuk dialog persetujuan untuk [pesan lintas sesi yang ditahan](/docs/id/cross-session-messaging#control-inbound-messages). Di Claude Code v2.1.236 atau lebih baru, tenggat waktu yang sama membatasi prompt persetujuan kredit penggunaan [Fable](/docs/id/model-config#fable-and-usage-credits) pertengahan sesi dalam sesi yang mungkin tidak ada siapa pun di terminal. Ketika tidak ada jawaban sebelum tenggat waktu, Claude Code membatalkan dialog dan melanjutkan dengan default tanpa tindakan. Memerlukan Claude Code v2.1.224 atau lebih baru.

* **Scope**: [`User or managed`](#scopes)
* **Type**: string, salah satu dari `"60s"`, `"5m"`, `"10m"`, atau `"never"`, yang menonaktifkan tenggat waktu
* **Default**: `"5m"`
* **Per-session overrides**: [`CLAUDE_CODE_USER_DIALOG_TIMEOUT_MS`](/docs/id/env-vars) mengambil alih kunci ini untuk satu sesi

```json settings.json theme={null}
{
  "dialogExpiry": "10m"
}
```

Prompt izin dan pertanyaan [`AskUserQuestion`](/docs/id/tools-reference#askuserquestion-tool-behavior) menggunakan alur mereka sendiri dan tidak diatur oleh tenggat waktu ini. Muncul di `/config` sebagai **Dialog expiry**, yang menulis kunci ini ke pengaturan pengguna; baris memerlukan Claude Code v2.1.232 atau lebih baru, dan Claude Code menyembunyikannya saat pengaturan terkelola atau flag `--settings` mengatur kunci.

<h3 id="editormode">
  `editorMode`
</h3>

Pilih mode pengikatan kunci untuk prompt input.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, salah satu dari:
  * `"normal"`: pintasan keyboard standar dalam input prompt
  * `"vim"`: pengeditan gaya vim dengan mode NORMAL, INSERT, dan VISUAL
* **Default**: `"normal"`

```json settings.json theme={null}
{
  "editorMode": "vim"
}
```

Muncul di `/config` sebagai **Editor mode**, yang menulis kunci ini ke pengaturan pengguna.

<h3 id="emojicompletionenabled">
  `emojiCompletionEnabled`
</h3>

Tampilkan saran emoji ketika Anda mengetik `:` ditambah shortcode dalam input prompt, dan ganti shortcode yang selesai seperti `:heart:` dengan emojinya. Atur ke `false` untuk mematikan keduanya.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code menampilkan saran emoji setelah `:` dan mengganti shortcode yang selesai dengan emojinya
  * `false`: Claude Code tidak menyarankan emoji atau mengganti shortcode
* **Default**: `true`

```json settings.json theme={null}
{
  "emojiCompletionEnabled": false
}
```

Lihat [Emoji shortcodes](/docs/id/interactive-mode#emoji-shortcodes). Memerlukan Claude Code v2.1.217 atau lebih baru.

<span id="file-suggestion-settings" />

<h3 id="filesuggestion">
  `fileSuggestion`
</h3>

Jalankan perintah Anda sendiri untuk menyediakan pelengkapan otomatis jalur file `@` daripada saran file bawaan. Saran bawaan menggunakan traversal filesystem cepat; monorepo besar mungkin lebih baik dengan pengindeksan khusus proyek seperti indeks file yang sudah dibangun.

* **Scope**: [`Any file`](#scopes). Di bawah [status line dan file suggestion gates](#status-line-and-file-suggestion-gates), Claude Code mematikan perintah atau menjalankan hanya nilai terkelola, dan melewati milik Anda tanpa peringatan.
* **Type**: object dengan `type`, selalu `"command"`, dan `command`, perintah shell untuk dijalankan
* **Default**: unset, jadi Claude Code menggunakan saran file bawaan

```json settings.json theme={null}
{
  "fileSuggestion": {
    "type": "command",
    "command": "~/.claude/file-suggestion.sh"
  }
}
```

Setelah Anda menyimpan ini, ketik `@` diikuti bagian dari jalur dalam prompt: saran berasal dari output perintah Anda.

<h4 id="command-input-and-output">
  Command input dan output
</h4>

Claude Code menjalankan perintah dengan variabel lingkungan yang sama seperti [hooks](/docs/id/hooks), termasuk `CLAUDE_PROJECT_DIR`, dan berhenti menunggu setelah lima detik. Perintah menerima JSON di stdin dengan field `query` yang menampung apa yang telah Anda ketik sejauh ini:

```json theme={null}
{"query": "src/comp"}
```

Cetak jalur file yang dipisahkan baris baru ke stdout. Claude Code menampilkan paling banyak 15:

```text theme={null}
src/components/Button.tsx
src/components/Modal.tsx
src/components/Form.tsx
```

Skrip berikut membaca query dan menyerahkannya ke indeks file repositori:

```bash theme={null}
#!/bin/bash
query=$(cat | jq -r '.query')
# Replace your-repo-file-index with your own file search command
your-repo-file-index --query "$query" | head -20
```

<span id="footer-link-badges" />

<h3 id="footerlinksregexes">
  `footerLinksRegexes`
</h3>

Render lencana yang dapat diklik tambahan di footer di bawah kotak input ketika regex cocok dengan output giliran: hasil tool, termasuk konten file dan halaman yang diambil, dan respons Claude sendiri. Gunakan untuk mengubah ID yang dicetak oleh CLI proyek, seperti alat review dan pelacak masalah, menjadi tautan sesi.

* **Scope**: [`User or managed`](#scopes)
* **Type**: array of objects, masing-masing dengan `type` diatur ke `"regex"`, regex `pattern`, template `url`, dan `label` opsional; placeholder `{name}` dalam `url` dan `label` diisi dari grup penangkapan bernama dalam `pattern`
* **Default**: unset, jadi tidak ada lencana yang dirender

Contoh ini cocok dengan kunci masalah seperti `PROJ-1234` dan membangun setiap tautan dari kunci yang ditangkap:

```json settings.json theme={null}
{
  "footerLinksRegexes": [
    {
      "type": "regex",
      "pattern": "\\b(?<key>PROJ-\\d+)\\b",
      "url": "https://issues.example.com/browse/{key}",
      "label": "{key}"
    }
  ]
}
```

Dengan ini dikonfigurasi, ketika `PROJ-1234` muncul dalam hasil tool atau dalam balasan Claude, lencana `PROJ-1234` muncul di footer menautkan ke `https://issues.example.com/browse/PROJ-1234`.

<h4 id="badge-constraints">
  Batasan lencana
</h4>

Setiap URL, label, dan jumlah lencana entri dibatasi sebagai berikut:

| Constraint  | Behavior                                                                                                                                                                                                 |
| :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| URL origin  | Nilai yang ditangkap dikodekan URL dan URL yang dibangun harus berbagi asal literal template. Penangkapan dapat mengisi segmen jalur atau nilai query tetapi tidak dapat mengubah tempat tautan menunjuk |
| URL length  | URL yang dibangun lebih panjang dari 2048 karakter dijatuhkan                                                                                                                                            |
| URL scheme  | Harus `https`, `http`, atau skema deep-link editor atau workspace yang diakui: `vscode`, `vscode-insiders`, `cursor`, `windsurf`, `zed`, `jetbrains`, `idea`, `slack`, `linear`, `notion`, `figma`       |
| Label       | Default ke teks yang cocok dan dipotong menjadi 28 kolom tampilan                                                                                                                                        |
| Badge count | Paling banyak 5 lencana dirender. Yang tertua digantikan oleh kecocokan yang lebih baru dan `/clear` menghapusnya                                                                                        |

Ketika giliran selesai, Claude Code mencocokkan regex `pattern` setiap entri terhadap output giliran di thread utama, jadi regex yang lambat memblokir UI sampai selesai. Quantifier bersarang seperti `(a+)+$` dapat memakan waktu secara eksponensial terhadap input tertentu dan membekukan sesi, jadi jaga setiap `pattern` linear dan hindari penggandaan `+` atau `*`.

Lencana footer dirender bersama [status line kustom](/docs/id/statusline) ketika satu dikonfigurasi; tidak ada yang menggantikan yang lain. Gunakan status line untuk baris yang didorong skrip yang menghitung kontennya sendiri dari data sesi, dan lencana footer untuk mengubah ID dari percakapan menjadi tautan tanpa skrip.

<h3 id="keybindingflavor">
  `keybindingFlavor`
</h3>

<Warning>
  Tidak direkomendasikan sejak v2.1.261 dan tidak berpengaruh. Kunci pengeditan kata prompt selalu [mengikuti konvensi readline](/docs/id/interactive-mode#make-ctrl-w-delete-back-to-whitespace), seperti di Bash. Claude Code masih menerima `keybindingFlavor`, jadi file pengaturan yang mengaturnya tetap valid.
</Warning>

Di v2.1.238 hingga v2.1.260, mengaturnya ke `"readline"` membuat `Ctrl+W` menghapus kembali ke spasi sebelumnya daripada hanya kata sebelumnya.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, `"classic"` atau `"readline"`
* **Default**: unset

<h3 id="prefersreducedmotion">
  `prefersReducedMotion`
</h3>

Kurangi atau matikan animasi antarmuka seperti spinner, shimmer, dan efek flash. Muncul di `/config` sebagai **Reduce motion**.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code mengurangi atau mematikan animasi antarmuka seperti spinner, shimmer, dan efek flash
  * `false`: sama dengan unset; Claude Code menampilkan animasinya
* **Default**: `false`

```json settings.json theme={null}
{
  "prefersReducedMotion": true
}
```

<h3 id="promptsuggestionenabled">
  `promptSuggestionEnabled`
</h3>

Tampilkan atau sembunyikan [prompt suggestions](/docs/id/interactive-mode#prompt-suggestions), prediksi yang dikelabuhi yang muncul dalam input prompt Anda. Atur ke `false`, atau matikan **Prompt suggestions** di `/config`, untuk menyembunyikannya.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Anda melihat saran prompt dalam input prompt Anda
  * `false`: Claude Code menyembunyikan saran prompt
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/id/env-vars) mengambil alih kunci ini untuk satu sesi

```json settings.json theme={null}
{
  "promptSuggestionEnabled": false
}
```

Saran prompt memerlukan akun claude.ai atau Console dengan telemetri aktif. Di Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry, atau dengan telemetri dimatikan, seperti oleh [`DISABLE_TELEMETRY`](/docs/id/env-vars), kunci ini tidak berpengaruh dan hanya `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=1` mengaktifkannya.

<h3 id="respectgitignore">
  `respectGitignore`
</h3>

Kontrol apakah pemilih file `@` mengeluarkan file yang cocok dengan pola `.gitignore`. Muncul di `/config` sebagai **Respect .gitignore in file picker**.

* **Scope**: [`Any file`](#scopes). Ketika tidak ada file pengaturan yang mengaturnya, Claude Code kembali ke `respectGitignore` di `~/.claude.json`, yang toggle `/config` tulis.
* **Type**: Boolean
  * `true`: pemilih file `@` mengeluarkan file yang cocok dengan pola `.gitignore`
  * `false`: pemilih file `@` menyertakan file yang cocok dengan pola `.gitignore`
* **Default**: `true`

```json settings.json theme={null}
{
  "respectGitignore": false
}
```

<h3 id="respondtobashcommands">
  `respondToBashCommands`
</h3>

Pilih apakah Claude merespons setelah Anda menjalankan perintah shell dengan [prefix `!`](/docs/id/interactive-mode#shell-mode-with-prefix) di kotak input. Secara default, Claude Code menambahkan output perintah ke percakapan dan Claude membalasnya. Atur kunci ini ke `false` untuk menambahkan output ke konteks tanpa balasan, sehingga Anda dapat menjalankan beberapa perintah dan menanyakan tentang mereka bersama-sama.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code menambahkan output perintah ke percakapan dan Claude membalasnya
  * `false`: Claude Code menambahkan output ke konteks tanpa balasan
* **Default**: `true`

```json settings.json theme={null}
{
  "respondToBashCommands": false
}
```

Lihat [Shell mode dengan prefix `!`](/docs/id/interactive-mode#shell-mode-with-prefix).

<h3 id="showclearcontextonplanaccept">
  `showClearContextOnPlanAccept`
</h3>

Ketika Claude menyelesaikan rencana dalam [plan mode](/docs/id/permission-modes#review-and-approve-a-plan), ini menampilkan menu persetujuan. Perencanaan dapat menggunakan banyak konteks, jadi kunci ini menambahkan opsi pertama ke menu itu, **Yes, clear context and …**, yang menyetujui rencana, menghapus konteks percakapan, dan mulai mengimplementasikan dari rencana saja. Sisa label menamai mode izin sesi berlanjut, dan menunjukkan berapa banyak konteks Anda yang digunakan perencanaan.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: menu persetujuan rencana mendapat opsi pertama, **Yes, clear context and …**, yang menyetujui rencana dan menghapus konteks percakapan
  * `false`: menu persetujuan rencana tidak menampilkan opsi clear-context
* **Default**: `false`

```json settings.json theme={null}
{
  "showClearContextOnPlanAccept": true
}
```

<h3 id="showturnduration">
  `showTurnDuration`
</h3>

Tampilkan atau sembunyikan pesan durasi giliran setelah setiap respons, seperti "Cooked for 1m 6s · done 6:05 PM". Jam setelah "done" menunjukkan kapan giliran selesai; [`timeFormat`](#timeformat) dan [`timeZone`](#timezone) mengontrol format dan zonanya. Muncul di `/config` sebagai **Show turn duration**.

* **Scope**: [`Any file`](#scopes). Nilai di `~/.claude.json` dari versi yang lebih lama berlaku ketika tidak ada file pengaturan yang mengaturnya.
* **Type**: Boolean
  * `true`: Anda melihat pesan durasi giliran setelah setiap respons
  * `false`: Claude Code menyembunyikan pesan durasi giliran
* **Default**: `true`

```json settings.json theme={null}
{
  "showTurnDuration": false
}
```

<h3 id="spellcheck">
  `spellcheck`
</h3>

Garis bawahi kata-kata yang salah eja dalam input prompt saat Anda mengetik, menggunakan pemeriksa ejaan yang Anda instal. Claude Code hanya memeriksa teks dalam kotak input. [Check spelling as you type](/docs/id/interactive-mode#check-spelling-as-you-type) mencakup penginstalan aspell, hunspell, atau ispell dan apa yang diperiksa pemeriksa. Memerlukan Claude Code v2.1.235 atau lebih baru.

* **Scope**: [`User or managed`](#scopes). Blok dari tier tertinggi yang mengaturnya berlaku secara keseluruhan.
* **Type**: object dengan `enabled` (Boolean), `checker` (`"aspell"`, `"hunspell"`, `"ispell"`, atau `"auto"`), `language` (string, dilewatkan ke pemeriksa sebagai nama kamus), dan `color` (string, nama warna terminal, `#rrggbb`, `rgb(r,g,b)`, `ansi256(n)`, atau `ansi:<name>`)
* **Default**: unset, jadi pemeriksaan ejaan mati; `checker` default ke `"auto"`, yang pertama dari tiga ditemukan di `PATH`; `language` default ke kamus pemeriksa sendiri; `color` default ke warna kesalahan tema

```json settings.json theme={null}
{
  "spellcheck": { "enabled": true, "language": "en_GB" }
}
```

<h3 id="spinnertipsenabled">
  `spinnerTipsEnabled`
</h3>

Saat Claude bekerja, baris spinner berputar melalui tips pendek tentang fitur Claude Code, seperti "Use Plan Mode to prepare for a complex request before making changes. Press Shift+Tab twice to enable." Atur kunci ini ke `false` untuk menyembunyikannya. Muncul di `/config` sebagai **Show tips**.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Anda melihat tips dalam spinner saat Claude bekerja
  * `false`: Claude Code menyembunyikan tips spinner
* **Default**: `true`

```json settings.json theme={null}
{
  "spinnerTipsEnabled": false
}
```

<h3 id="spinnertipsoverride">
  `spinnerTipsOverride`
</h3>

Tambahkan tips Anda sendiri ke [spinner tips](#spinnertipsenabled) yang Claude Code tampilkan saat Claude bekerja, atau ganti tips bawaan dengan milik Anda. Claude Code menempatkan tips Anda dalam rotasi yang sama dengan yang bawaan: ia memilih tip yang belum ditampilkan paling lama, melewati tips masih dalam cooldown mereka, dan memecahkan seri berdasarkan prioritas.

Jika Anda mengatur [`spinnerTipsEnabled`](#spinnertipsenabled) ke `false`, Claude Code menyembunyikan semua tips, milik Anda termasuk.

* **Scope**: [`Any file`](#scopes). Claude Code menghormati objek tip, `tipsFile`, `label`, dan `excludeDefault` dari pengaturan pengguna, flag `--settings`, dan pengaturan terkelola; dari pengaturan proyek dan lokal hanya membaca tips string biasa.
* **Type**: object dengan field `tips`, `tipsFile`, `label`, dan `excludeDefault`, masing-masing opsional
* **Default**: unset, jadi Claude Code hanya menampilkan tips bawaan

Objek tip, `tipsFile`, `label`, dan aturan baris Scope yang pengaturan proyek dan lokal hanya berkontribusi string biasa memerlukan Claude Code v2.1.247 atau lebih baru. Pada versi sebelumnya, `excludeDefault` file proyek atau lokal juga berlaku.

Setiap entri `tips` adalah string biasa atau objek dengan field ini:

| Field              | Required | Description                                                                                                                                                                                                                 |
| :----------------- | :------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`               | Yes      | Hingga 64 huruf, digit, `.`, `_`, atau `-`. Claude Code mengetik riwayat tampilan tip padanya, jadi cooldown tip bertahan pengurutan ulang daftar. Dari dua entri dengan id yang sama, Claude Code menggunakan yang pertama |
| `text`             | Yes      | Tip, satu baris hingga 500 karakter. Claude Code menghapus escape ANSI dan karakter kontrol dan meruntuhkan spasi                                                                                                           |
| `cooldownSessions` | No       | Sesi Claude Code menunggu sebelum menampilkan tip lagi, `0` hingga `1000`, default `0`                                                                                                                                      |
| `priority`         | No       | Urutan di antara tips yang belum ditampilkan sama lamanya, lebih tinggi dulu, `-10` hingga `10`, default `0`                                                                                                                |

Claude Code membaca string biasa sebagai tip dengan default tersebut dan id berbasis posisi, jadi riwayat tampilan direset ketika Anda mengurutkan ulang daftar. Berikan tip `id` untuk menjaga riwayatnya di seluruh edit.

Claude Code membaca paling banyak 200 tips di seluruh `tips` dan `tipsFile`, dan menjatuhkan entri tidak valid dengan peringatan debug daripada menolak file pengaturan.

Gunakan field yang tersisa untuk menamai file tips, atur awalan, dan sembunyikan tips bawaan:

* `tipsFile`: jalur absolut atau `~/` ke file JSON lokal yang menampung array entri yang sama, atau objek dengan array `tips`, hingga 256 KB. Claude Code membaca file sekali per proses, jadi memuat edit Anda pada awal berikutnya. Anda tidak dapat mengaturnya melalui [pengaturan terkelola server](/docs/id/server-managed-settings); deploy inline `tips` di sana, atau deploy jalur di `managed-settings.json` on-disk.
* `label`: awalan Claude Code menampilkan sebelum tips dari pengaturan pengguna, `--settings`, dan terkelola, hingga 40 karakter. Default adalah `Tip`, awalan yang sama seperti tips bawaan, dan tips dari pengaturan proyek dan lokal selalu menggunakannya.
* `excludeDefault`: atur ke `true` untuk menyembunyikan tips bawaan dan hanya menampilkan milik Anda. Ketika Claude Code tidak dapat memuat salah satu tips Anda, misalnya karena `tipsFile` tidak ada atau setiap entri tidak valid, ia menjaga rotasi bawaan daripada spinner kosong.

Ketika lebih dari satu file pengaturan mengatur kunci, Claude Code menampilkan tips dari semuanya dan mengambil `tipsFile`, `label`, dan `excludeDefault` dari mana pun pengaturan terkelola, flag `--settings`, dan pengaturan pengguna adalah yang tertinggi-preseden yang mengatur masing-masing.

Contoh ini, dalam pengaturan pengguna Anda, menambahkan tip string biasa dan tip objek ke rotasi di bawah awalan `Acme tip`:

```json settings.json theme={null}
{
  "spinnerTipsOverride": {
    "label": "Acme tip",
    "tips": [
      "Run /review before opening a PR",
      {
        "id": "gateway-errors",
        "text": "Seeing 5xx errors? Check the gateway status page first",
        "cooldownSessions": 5,
        "priority": 2
      }
    ]
  }
}
```

Setiap field dalam contoh mengubah satu hal tentang cara Claude Code menampilkan tips:

* `label`: Claude Code menampilkan kedua tips sebagai `Acme tip: ...` daripada `Tip: ...`.
* String biasa: Claude Code memberikannya default, jadi dapat muncul lagi di sesi berikutnya.
* `id`: Claude Code mengetik riwayat tampilan tip kedua pada `gateway-errors`, jadi cooldown masih berlaku setelah Anda menambah atau mengurutkan ulang tips.
* `cooldownSessions`: setelah Claude Code menampilkan tip `gateway-errors`, tidak menampilkan tip itu lagi sampai lima sesi kemudian.
* `priority`: ketika tip `gateway-errors` dan tip lain belum ditampilkan untuk jumlah sesi yang sama, misalnya ketika tidak ada yang ditampilkan, Claude Code menampilkan `gateway-errors` terlebih dahulu. String biasa memiliki prioritas default, `0`.

Saat Claude bekerja, Claude Code menampilkan tips Anda dalam spinner dengan awalan Anda, seperti `Acme tip: Run /review before opening a PR`.

<h3 id="spinnerverbs">
  `spinnerVerbs`
</h3>

Saat giliran sedang berlangsung, spinner menampilkan kata kerja yang berputar seperti "Accomplishing", "Architecting", atau "Baking". Gunakan kunci ini untuk menambahkan kata kerja Anda sendiri ke rotasi itu atau ganti daftar bawaan dengan milik Anda.

* **Scope**: [`Any file`](#scopes)
* **Type**: object dengan array `verbs` string dan `mode`, salah satu dari:
  * `"append"`: Claude Code menambahkan kata kerja Anda ke set bawaan
  * `"replace"`: Claude Code hanya menampilkan kata kerja Anda
* **Default**: unset, jadi Claude Code menggunakan kata kerja bawaan

Contoh ini menambahkan dua kata kerja ke set bawaan:

```json settings.json theme={null}
{
  "spinnerVerbs": {
    "mode": "append",
    "verbs": ["Pondering", "Crafting"]
  }
}
```

Dalam mode `"replace"` dengan array `verbs` kosong, Claude Code menjaga kata kerja bawaan.

<h3 id="statusline">
  `statusLine`
</h3>

Jalankan perintah Anda sendiri untuk merender [status line](/docs/id/statusline) di bawah prompt dengan konteks seperti model, biaya, atau cabang git. Field opsional menyesuaikan spasi, menambahkan re-run berkala, dan menyembunyikan indikator mode vim bawaan ketika skrip Anda merender `vim.mode` sendiri.

* **Scope**: [`Any file`](#scopes). Ketika [`allowManagedHooksOnly`](#allowmanagedhooksonly) aktif, atau [`disableAllHooks`](#disableallhooks) diatur di luar pengaturan terkelola, hanya nilai pengaturan terkelola yang berjalan.
* **Type**: object dengan `type` diatur ke `"command"` dan string `command`, ditambah `padding` opsional sebagai jumlah karakter, `refreshInterval` sebagai jumlah detik, minimum `1`, dan `hideVimModeIndicator` sebagai Boolean
* **Default**: unset, jadi tidak ada status line

Contoh ini mencetak nama model dan penggunaan konteks, dan menambahkan dua karakter spasi horizontal:

```json settings.json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
    "padding": 2
  }
}
```

Contoh memerlukan [`jq`](https://jqlang.org/) terinstal dan berjalan dalam shell. Untuk PowerShell dan setara Git Bash, lihat [Windows configuration](/docs/id/statusline#windows-configuration); untuk setup lengkap, lihat [Manually configure a status line](/docs/id/statusline#manually-configure-a-status-line).

<h3 id="subagentstatusline">
  `subagentStatusLine`
</h3>

Ketika Claude menjalankan [subagents](/docs/id/sub-agents), Claude Code mencantumkannya dalam tampilan tugas di bawah prompt, satu baris per subagent menampilkan `name · description · token count`. Kunci ini memungkinkan Anda menjalankan perintah Anda sendiri untuk menulis ulang baris tersebut, misalnya untuk menampilkan penggunaan konteks setiap subagent sebagai persentase. Pada setiap refresh, Claude Code mengirim baris yang terlihat sebagai satu objek JSON di stdin, dengan array `tasks` membawa `id`, `name`, `status`, `model`, `tokenCount`, dan lainnya setiap subagent, dan mengganti baris untuk setiap `id` yang Anda tulis kembali sebagai baris `{"id", "content"}`. Baris yang tidak Anda tulis kembali menjaga rendering default.

* **Scope**: [`Any file`](#scopes). Ketika [`allowManagedHooksOnly`](#allowmanagedhooksonly) aktif, atau [`disableAllHooks`](#disableallhooks) diatur di luar pengaturan terkelola, hanya nilai pengaturan terkelola yang berjalan.
* **Type**: object dengan `type` diatur ke `"command"` dan string `command`
* **Default**: unset, jadi Claude Code merender baris default

```json settings.json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "jq -c '.tasks[] | {id, content: \"\\(.name): \\(.tokenCount) tokens\"}'"
  }
}
```

Lihat [Subagent status lines](/docs/id/statusline#subagent-status-lines).

<h3 id="syntaxhighlightingdisabled">
  `syntaxHighlightingDisabled`
</h3>

Claude Code mewarnai kode berdasarkan bahasa dalam diff, blok kode, dan pratinjau file yang ditampilkan di terminal, dengan highlighter bawaan; tidak ada plugin atau language server yang terlibat. Atur kunci ini ke `true` untuk menampilkannya sebagai teks biasa, misalnya jika warna bertentangan dengan tema terminal Anda atau memperlambat pembaca layar.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code mematikan penyorotan sintaks dalam diff, blok kode, dan pratinjau file
  * `false`: Claude Code menyoroti sintaks
* **Default**: `false`

```json settings.json theme={null}
{
  "syntaxHighlightingDisabled": true
}
```

<h3 id="terminalprogressbarenabled">
  `terminalProgressBarEnabled`
</h3>

Beberapa terminal dapat menampilkan indikator kemajuan di tab atau di taskbar untuk program yang berjalan di dalamnya. Saat Claude bekerja, Claude Code melaporkan status sedang berlangsung ke terminal, sehingga Anda dapat melihat dari tab atau jendela lain apakah sesi masih sibuk. Indikator tetap terlihat setelah giliran berakhir sementara [subagents latar belakang](/docs/id/sub-agents#run-subagents-in-foreground-or-background) atau [dynamic workflows](/docs/id/workflows) masih berjalan, dan jelas setelah sesi idle.

Claude Code melaporkannya hanya di terminal yang mendukung indikator: ConEmu, Ghostty 1.2.0 atau lebih baru, dan iTerm2 3.6.6 atau lebih baru. Atur kunci ini ke `false` untuk menghentikan Claude Code melaporkannya. Muncul di `/config` sebagai **Terminal progress bar**.

* **Scope**: [`Any file`](#scopes). Nilai di `~/.claude.json` dari versi yang lebih lama berlaku ketika tidak ada file pengaturan yang mengaturnya.
* **Type**: Boolean
  * `true`: Anda melihat progress bar terminal di terminal yang mendukungnya
  * `false`: Claude Code menyembunyikan progress bar terminal
* **Default**: `true`

```json settings.json theme={null}
{
  "terminalProgressBarEnabled": false
}
```

<h3 id="terminaltitlefromrename">
  `terminalTitleFromRename`
</h3>

Claude Code mengatur judul tab terminal Anda. Secara default menggunakan judul yang dihasilkan dari percakapan, dan setelah Anda memberi sesi [nama](/docs/id/sessions#name-your-sessions) dengan `/rename` atau `--name`, tab menampilkan nama itu. Atur kunci ini ke `false` untuk menjaga judul yang dihasilkan di tab bahkan setelah Anda memberi nama sesi. Nama itu sendiri masih berlaku, jadi `/resume <name>` dan pemilih sesi menemukannya.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: judul tab terminal menampilkan nama sesi yang Anda atur
  * `false`: tab menjaga judul yang Claude Code hasilkan dari percakapan Anda
* **Default**: `true`

```json settings.json theme={null}
{
  "terminalTitleFromRename": false
}
```

Untuk menghentikan Claude Code memperbarui judul terminal sama sekali, atur [`CLAUDE_CODE_DISABLE_TERMINAL_TITLE`](/docs/id/env-vars) ke `1`.

<h3 id="theme">
  `theme`
</h3>

Pilih tema warna untuk antarmuka. Muncul di `/config` sebagai **Theme**.

* **Scope**: [`Any file`](#scopes). Nilai di `~/.claude.json` dari versi yang lebih lama berlaku ketika tidak ada file pengaturan yang mengaturnya.
* **Type**: string, salah satu dari:
  * `"auto"`: cocok dengan latar belakang terminal Anda yang terang atau gelap
  * `"dark"`: tema gelap
  * `"light"`: tema terang
  * `"dark-daltonized"`: tema gelap dengan warna ramah buta warna
  * `"light-daltonized"`: tema terang dengan warna ramah buta warna
  * `"dark-ansi"`: tema gelap hanya menggunakan palet warna ANSI terminal Anda
  * `"light-ansi"`: tema terang hanya menggunakan palet warna ANSI terminal Anda
  * `"custom:<slug>"` atau `"custom:<plugin-name>:<slug>"`: tema kustom dari `~/.claude/themes/` atau plugin
* **Default**: `"dark"`

```json settings.json theme={null}
{
  "theme": "light-daltonized"
}
```

Lihat [Create a custom theme](/docs/id/terminal-config#create-a-custom-theme).

<h3 id="timeformat">
  `timeFormat`
</h3>

Pilih cara Claude Code menulis waktu yang ditampilkan dalam antarmuka, seperti `done 6:05 PM` di akhir setiap pesan durasi giliran dan stempel waktu dalam [transcript viewer](/docs/id/interactive-mode#transcript-viewer). Untuk memilih preset, jalankan `/config` dan atur **Time format**. Memerlukan Claude Code v2.1.257 atau lebih baru.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, salah satu dari:
  * `"auto"`: sama dengan unset; setiap waktu menjaga format bawaan, yang mengikuti lokal Anda pada pesan durasi giliran
  * `"12-hour"`: jam 12 jam
  * `"24-hour"`: jam 24 jam
  * `"24-hour-utc"`: jam 24 jam di UTC dengan `Z` setelah menit, seperti `18:05Z`; Claude Code mengabaikan [`timeZone`](#timezone) untuk preset ini
  * Pola strftime seperti `"%H:%M"`: Claude Code menulis setiap waktu dengan pola. Nilai apa pun yang berisi `%` adalah pola, dan nilai apa pun di luar preset menghitung sebagai `"auto"`
* **Default**: `"auto"`

```json settings.json theme={null}
{
  "timeFormat": "24-hour"
}
```

`/config` hanya menawarkan preset, jadi untuk menggunakan pola strftime, tambahkan kunci ke file pengaturan. Contoh ini menampilkan setiap waktu sebagai jam 24 jam dua digit:

```json settings.json theme={null}
{
  "timeFormat": "%H:%M"
}
```

Pesan durasi giliran dan transcript viewer kemudian menampilkan waktu seperti `18:05`. Dalam transcript viewer, pola adalah seluruh stempel waktu, jadi tambahkan direktif tanggal ketika Anda menginginkan tanggal di sana. Contoh ini menempatkan tanggal di depan jam:

```json settings.json theme={null}
{
  "timeFormat": "%Y-%m-%d %H:%M"
}
```

Permukaan yang sama kemudian menampilkan waktu seperti `2026-09-01 18:05`.

<h3 id="timezone">
  `timeZone`
</h3>

Tampilkan waktu dalam antarmuka di zona waktu selain sistem Anda. Atur ke [nama zona waktu IANA](https://www.iana.org/time-zones), seperti `"UTC"` atau `"Europe/Dublin"`. Waktu yang [`timeFormat`](#timeformat) kontrol kemudian menampilkan di zona ini. Jika `timeFormat` adalah `"24-hour-utc"`, waktu tetap di UTC dan Claude Code mengabaikan kunci ini. `/config` tidak memiliki baris untuk kunci ini, jadi atur dalam file pengaturan. Memerlukan Claude Code v2.1.257 atau lebih baru.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, nama zona waktu IANA. Ketika Claude Code tidak mengenali nama, menggunakan zona waktu sistem Anda
* **Default**: unset, jadi waktu menampilkan di zona waktu sistem Anda

```json settings.json theme={null}
{
  "timeZone": "Europe/Dublin"
}
```

<h3 id="tui">
  `tui`
</h3>

Pilih renderer UI terminal. Gunakan `"fullscreen"` untuk renderer alt-screen bebas flicker [fullscreen](/docs/id/fullscreen) dengan scrollback virtual, atau `"default"` untuk renderer main-screen klasik. Menjalankan `/tui fullscreen` atau `/tui default` menulis kunci ini untuk Anda.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, salah satu dari:
  * `"default"`: renderer main-screen klasik
  * `"fullscreen"`: renderer alt-screen bebas flicker dengan scrollback virtual
* **Default**: unset, jadi Claude Code [memilih renderer untuk Anda](/docs/id/fullscreen#fullscreen-by-default)
* **Per-session overrides**: [`CLAUDE_CODE_NO_FLICKER`](/docs/id/env-vars) dan [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN`](/docs/id/env-vars) mengambil alih kunci ini untuk satu sesi: `CLAUDE_CODE_NO_FLICKER=1` mengaktifkan fullscreen, dan `CLAUDE_CODE_NO_FLICKER=0` atau `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1` mematikannya; ketika keduanya diatur, Claude Code mematikannya

```json settings.json theme={null}
{
  "tui": "fullscreen"
}
```

Di bawah tmux `-CC` atau melalui SSH ke Windows, Claude Code menjaga renderer klasik kecuali Anda mengatur `CLAUDE_CODE_NO_FLICKER=1`. Sesi latar belakang dibuka dari [agent view](/docs/id/agent-view) selalu menggunakan renderer fullscreen terlepas dari pengaturan ini.

<h3 id="verbose">
  `verbose`
</h3>

Secara default, transkrip meruntuhkan setiap panggilan tool menjadi ringkasan pendek, seperti perintah yang Claude jalankan dan jumlah baris output, dan Anda menekan `Ctrl+O` untuk beralih seluruh transkrip ke tampilan yang diperluas ketika Anda menginginkan detail. Atur kunci ini ke `true` untuk menampilkan input dan output lengkap setiap panggilan tool secara inline saat terjadi, yang berguna ketika Anda men-debug hook, server MCP, atau perintah shell panjang. Muncul di `/config` sebagai **Verbose output**.

* **Scope**: [`Any file`](#scopes). Nilai di `~/.claude.json` dari versi yang lebih lama berlaku ketika tidak ada file pengaturan yang mengaturnya.
* **Type**: Boolean
  * `true`: Anda melihat output tool lengkap
  * `false`: Anda melihat ringkasan terpotong dari output tool
* **Default**: `false`
* **Per-session overrides**: [`--verbose`](/docs/id/cli-reference#cli-flags) mengambil alih kunci ini untuk satu sesi

```json settings.json theme={null}
{
  "verbose": true
}
```

Nilai [`viewMode`](#viewmode) atau pilihan `/focus` lengket menimpa kunci ini setiap sesi.

<h3 id="viewmode">
  `viewMode`
</h3>

Atur tampilan transkrip Claude Code dimulai: `"default"`, `"verbose"`, atau `"focus"`. Ketika diatur, menimpa pilihan `/focus` lengket dan pengaturan [`verbose`](#verbose).

* **Scope**: [`Any file`](#scopes)
* **Type**: string, salah satu dari:
  * `"default"`: transkrip normal dengan output tool terpotong
  * `"verbose"`: transkrip dengan output tool lengkap
  * `"focus"`: hanya prompt terakhir Anda, ringkasan satu baris panggilan tool dengan edit diffstats, dan respons akhir. Focus view memerlukan [renderer fullscreen](#tui)
* **Default**: unset, jadi pengaturan `verbose` dan pilihan `/focus` terakhir Anda berlaku
* **Per-session overrides**: [`--verbose`](/docs/id/cli-reference#cli-flags) mengambil alih kunci ini untuk satu sesi

```json settings.json theme={null}
{
  "viewMode": "focus"
}
```

<h3 id="viminsertmoderemaps">
  `vimInsertModeRemaps`
</h3>

Petakan urutan INSERT-mode dua kunci ke Escape dalam [mode editor vim](/docs/id/interactive-mode#vim-editor-mode). Setiap kunci adalah tepat dua karakter yang dapat dicetak diketik berurutan, dan `"<Esc>"` adalah satu-satunya target yang didukung; Claude Code mengabaikan entri lain. Memerlukan Claude Code v2.1.208 atau lebih baru.

* **Scope**: [`User or managed`](#scopes). Repositori tidak dapat memetakan ulang keystroke Anda.
* **Type**: object memetakan urutan dua karakter ke `"<Esc>"`
* **Default**: unset

```json settings.json theme={null}
{
  "vimInsertModeRemaps": {
    "jj": "<Esc>"
  }
}
```

Tidak berpengaruh kecuali `editorMode` adalah `"vim"`. Lihat [Remap INSERT-mode key sequences](/docs/id/interactive-mode#remap-insert-mode-key-sequences). Memerlukan Claude Code v2.1.208 atau lebih baru.

<h3 id="voice">
  `voice`
</h3>

Aktifkan [voice dictation](/docs/id/voice-dictation) dan pilih cara kunci diksi berperilaku. Claude Code menulis objek ini untuk Anda ketika Anda menjalankan `/voice`.

* **Scope**: [`Any file`](#scopes)
* **Type**: object dengan `enabled` sebagai Boolean, `autoSubmit` sebagai Boolean yang berlaku dalam mode hold saja, dan `mode`, salah satu dari:
  * `"hold"`: Anda menahan kunci diksi saat berbicara dan melepasnya untuk berhenti
  * `"tap"`: Anda mengetuk kunci sekali untuk mulai merekam dan lagi untuk mengirim
* **Default**: unset, jadi diksi mati; ketika `enabled` adalah `true` dan `mode` unset, Claude Code menggunakan `"hold"`

Contoh ini mengaktifkan diksi dan membuat kunci mengetuk sekali untuk mulai merekam dan lagi untuk mengirim:

```json settings.json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

`autoSubmit` mengirim prompt ketika Anda melepaskan kunci dalam mode hold. Voice dictation memerlukan akun claude.ai.

<h3 id="voiceenabled">
  `voiceEnabled`
</h3>

<Warning>
  Tidak direkomendasikan sejak v2.1.92, ketika objek [`voice`](#voice) menggantinya. Claude Code masih membacanya jadi file pengaturan yang lebih lama tetap bekerja, tetapi konfigurasi baru harus mengatur `voice.enabled`.
</Warning>

Aktifkan voice dictation dengan bentuk Boolean tunggal yang mendahului objek `voice`. Ketika keduanya diatur, `voice.enabled` berlaku.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: voice dictation aktif ketika Anda masuk dengan akun claude.ai dan kebijakan organisasi Anda memungkinkan suara, kecuali `voice.enabled` diatur
  * `false`: voice dictation mati, kecuali `voice.enabled` diatur
* **Default**: unset

```json settings.json theme={null}
{
  "voiceEnabled": true
}
```

<h3 id="wheelscrollaccelerationenabled">
  `wheelScrollAccelerationEnabled`
</h3>

Percepat kecepatan scroll roda mouse selama scroll cepat dalam [fullscreen rendering](/docs/id/fullscreen#mouse-wheel-scrolling). Atur ke `false` untuk laju scroll konstan per takik roda.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code mempercepat kecepatan scroll roda mouse selama scroll cepat
  * `false`: Claude Code menggulir pada laju konstan per takik roda
* **Default**: `true`

```json settings.json theme={null}
{
  "wheelScrollAccelerationEnabled": false
}
```

<h2 id="git-and-attribution">
  Git dan atribusi
</h2>

Kontrol atribusi yang ditambahkan Claude Code ke commit dan pull request serta cara kerjanya dengan git.

<span id="attribution-settings" />

<h3 id="attribution">
  `attribution`
</h3>

Sesuaikan atribusi yang ditambahkan Claude Code ke commit git dan pull request. Commit mendapatkan [git trailer](https://git-scm.com/docs/git-interpret-trailers) seperti `Co-Authored-By` secara default; deskripsi pull request mendapatkan teks biasa. Atur setiap bagian secara terpisah dengan sub-kunci di bawah ini.

* **Scope**: [`Any file`](#scopes)
* **Type**: object dengan string `commit` dan `pr` serta Boolean `sessionUrl`, atau `false` untuk menyembunyikan semua atribusi. Nilai `false` memerlukan Claude Code v2.1.281 atau lebih baru; versi sebelumnya menolaknya dan [melewati seluruh file pengaturan pengguna, proyek, atau lokal](/docs/id/settings#fix-a-broken-settings-file) yang menahannya
* **Default**: tidak diatur, jadi Claude Code menggunakan atribusi standar yang ditampilkan di bawah setiap sub-kunci

Untuk menyembunyikan semua atribusi, atur `attribution` ke `false`. Dalam file pengaturan yang juga dibaca versi sebelumnya, atur [`commit`](#attribution-commit) dan [`pr`](#attribution-pr) ke string kosong dan [`sessionUrl`](#attribution-sessionurl) ke `false` sebagai gantinya.

Contoh ini mengganti atribusi commit, menghapus atribusi pull request, dan menghilangkan tautan sesi:

```json settings.json theme={null}
{
  "attribution": {
    "commit": "Generated with AI\n\nCo-Authored-By: AI <ai@example.com>",
    "pr": "",
    "sessionUrl": false
  }
}
```

Setelah Anda mengatur `commit` atau `pr`, Claude Code mengabaikan pengaturan `includeCoAuthoredBy` yang sudah usang dan menggunakan teks defaultnya untuk mana pun dari keduanya yang Anda biarkan tidak diatur.

Claude Code memberitahu Claude bahwa instruksi Anda sendiri tentang atribusi, seperti aturan CLAUDE.md atau [memory](/docs/id/memory), mengambil alih baris commit dan PR ini, kecuali baris diatur dalam [managed settings](/docs/id/managed-settings).

<h3 id="includecoauthoredby">
  `includeCoAuthoredBy`
</h3>

<Warning>
  Sudah usang sejak v2.0.62, ketika [`attribution`](#attribution) menggantinya. Claude Code masih membacanya, tetapi konfigurasi baru harus mengatur `attribution`.
</Warning>

Gunakan [`attribution`](#attribution) sebagai gantinya, yang menggantikan kunci ini dan memungkinkan Anda mengubah atau menyembunyikan trailer commit, teks pull request, dan tautan sesi secara terpisah. Claude Code masih menghormati `includeCoAuthoredBy: false` dari file pengaturan yang mendahului `attribution`, tetapi mengabaikannya setelah Anda mengatur `attribution.commit` atau `attribution.pr`.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: sama dengan tidak diatur; Claude Code menambahkan trailer commit dan teks atribusi pull request
  * `false`: Claude Code menghilangkan trailer commit dan teks atribusi pull request, kecuali `attribution` mengatur `commit` atau `pr`, dalam hal ini aturan [`attribution`](#attribution) berlaku
* **Default**: `true`

```json settings.json theme={null}
{
  "includeCoAuthoredBy": false
}
```

Untuk menyembunyikan semua atribusi, lihat [`attribution`](#attribution).

<h3 id="includegitinstructions">
  `includeGitInstructions`
</h3>

Claude Code memberikan Claude dua bagian konteks terkait git: instruksi bawaan untuk cara menulis commit dan pull request, dalam deskripsi alat Bash, dan snapshot status git dari repositori Anda. Snapshot menyimpan cabang saat ini, cabang utama, output `git status`, dan commit terbaru. Claude Code membacanya ketika percakapan dimulai.

Atur kunci ini ke `false` untuk meninggalkan keduanya, misalnya ketika Anda menggunakan skill alur kerja git Anda sendiri.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code menyertakan instruksi alur kerja commit dan pull request bawaan serta snapshot status git. Sesi cloud tidak pernah menyertakan snapshot
  * `false`: Claude Code meninggalkan keduanya
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS`](/docs/id/env-vars) mengambil alih kunci ini untuk satu sesi

```json settings.json theme={null}
{
  "includeGitInstructions": false
}
```

<h3 id="prurltemplate">
  `prUrlTemplate`
</h3>

Arahkan tautan PR yang dirender Claude Code, di lencana footer dan dalam ringkasan hasil alat, ke alat tinjauan kode internal alih-alih `github.com`. Claude Code mengganti `{host}`, `{owner}`, `{repo}`, `{number}`, dan `{url}` dari URL PR. Tautan [permintaan penggabungan GitLab](/docs/id/interactive-mode#gitlab-merge-requests) di kedua permukaan mempertahankan URL GitLab mereka.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, template URL menggunakan salah satu dari lima placeholder
* **Default**: tidak diatur

```json settings.json theme={null}
{
  "prUrlTemplate": "https://reviews.example.com/{owner}/{repo}/pull/{number}"
}
```

Claude Code menerapkan template hanya ke tautan yang dirender sendiri; nomor PR yang ditulis Claude dalam pesan, seperti `#123`, tetap seperti yang ditulis Claude. URL yang tidak memiliki bentuk `/pull/<number>` dibiarkan tidak berubah.

<h3 id="attribution-commit">
  `attribution.commit`
</h3>

Atur teks atribusi yang ditambahkan Claude Code ke commit git, termasuk trailer apa pun. Atur ke string kosong untuk menyembunyikan atribusi commit.

* **Scope**: [`Any file`](#scopes)
* **Type**: string
* **Default**: tidak diatur, jadi Claude Code menambahkan `Co-Authored-By: <name> <noreply@anthropic.com>`. Nama adalah model aktif sesi, seperti `Claude Sonnet 5`.
  * Ketika Claude Code mengenali model sebagai model Claude tetapi tidak dapat mengkonfirmasi versi pastinya, ia menulis `Claude` saja.
  * Ketika tidak dapat mencocokkan ID model ke model Claude apa pun, seperti model pihak ketiga yang disajikan melalui [`ANTHROPIC_BASE_URL`](/docs/id/env-vars) kustom, ia menulis `Claude Code`.

Contoh ini mengganti trailer default dengan baris kustom dan trailer `Co-Authored-By` kustom:

```json settings.json theme={null}
{
  "attribution": {
    "commit": "Generated with AI\n\nCo-Authored-By: AI <ai@example.com>"
  }
}
```

<h3 id="attribution-pr">
  `attribution.pr`
</h3>

Atur teks atribusi yang ditambahkan Claude Code ke deskripsi pull request. Atur ke string kosong untuk menyembunyikan atribusi pull request.

* **Scope**: [`Any file`](#scopes)
* **Type**: string
* **Default**: tidak diatur, jadi Claude Code menambahkan `🤖 Generated with [Claude Code](https://claude.com/claude-code)`

```json settings.json theme={null}
{
  "attribution": {
    "pr": ""
  }
}
```

<h3 id="attribution-sessionurl">
  `attribution.sessionUrl`
</h3>

Pilih apakah Claude Code menambahkan tautan sesi claude.ai ketika melakukan commit atau membuka pull request dari sesi [cloud](/docs/id/claude-code-on-the-web) atau [Remote Control](/docs/id/remote-control). Claude Code menambahkan tautan sebagai trailer `Claude-Session` pada commit dan sebagai tautan dalam deskripsi pull request. Atur ke `false` untuk menghilangkan tautan.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code menambahkan tautan sesi claude.ai ketika melakukan commit atau membuka pull request dari sesi cloud atau Remote Control
  * `false`: Claude Code menghilangkan tautan
* **Default**: `true`

```json settings.json theme={null}
{
  "attribution": {
    "sessionUrl": false
  }
}
```

<span id="hook-configuration" />

<span id="hook-and-skill-settings" />

<h2 id="hooks-and-automation">
  Hooks dan otomasi
</h2>

Daftarkan hooks, batasi hook mana yang berjalan, dan kontrol alur kerja. Untuk acara hook dan payload, lihat [referensi hooks](/docs/id/hooks).

<h3 id="allowedhttphookurls">
  `allowedHttpHookUrls`
</h3>

Batasi URL mana yang dapat ditargetkan oleh [HTTP hooks](/docs/id/hooks#http-hook-fields). Ketika Anda menentukan kunci ini, Claude Code menjalankan HTTP hook hanya jika URL-nya cocok dengan salah satu pola dan memblokir sisanya tanpa menjalankannya; array kosong memblokir setiap HTTP hook.

* **Scope**: [`Any file`](#scopes). Array digabungkan di seluruh file pengaturan.
* **Type**: array pola URL, dengan `*` sebagai wildcard
* **Default**: tidak diatur, jadi URL apa pun diizinkan

Contoh ini memungkinkan URL apa pun di bawah `https://hooks.example.com/` dan URL `http://localhost` apa pun:

```json settings.json theme={null}
{
  "allowedHttpHookUrls": ["https://hooks.example.com/*", "http://localhost:*"]
}
```

Pencocokan nama host tidak peka huruf besar-kecil dan memperlakukan `hooks.example.com.`, dengan titik trailing yang menandai nama domain yang sepenuhnya memenuhi syarat, sama seperti `hooks.example.com`, yang merupakan cara DNS memperlakukannya. Daftar izin berlaku untuk hooks dari setiap sumber, termasuk pengaturan terkelola.

<h3 id="allowmanagedhooksonly">
  `allowManagedHooksOnly`
</h3>

Batasi eksekusi hook ke hooks yang digunakan organisasi Anda.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: hanya hook terkelola yang berjalan, ditambah hook Agent SDK dan hooks dari plugin yang dipaksa diaktifkan oleh pengaturan terkelola Anda. Lihat [Apa yang berjalan di bawah `allowManagedHooksOnly`](#what-runs-under-allowmanagedhooksonly)
  * `false`: hooks dari setiap scope pengaturan dan plugin berjalan
* **Default**: tidak diatur, jadi hooks dari setiap scope pengaturan dan plugin berjalan

```json managed-settings.json theme={null}
{
  "allowManagedHooksOnly": true
}
```

<h4 id="what-runs-under-allowmanagedhooksonly">
  Apa yang berjalan di bawah `allowManagedHooksOnly`
</h4>

Ketika Anda menyetelnya ke `true`, Claude Code mengubah hook dan perintah seperti hook mana yang dimuat:

* **Hook terkelola dan SDK berjalan**: hooks dari pengaturan terkelola dan hooks yang didaftarkan [Agent SDK](/docs/id/agent-sdk/overview) dalam proses
* **Hook plugin yang dipaksa diaktifkan berjalan**: hooks dari plugin yang dipaksa diaktifkan oleh pengaturan terkelola Anda melalui [`enabledPlugins`](#enabledplugins). Claude Code cocok dengan ID `plugin@marketplace` lengkap, jadi plugin dengan nama yang sama dari marketplace berbeda tetap diblokir. Ini memungkinkan Anda mendistribusikan hooks yang telah diverifikasi melalui marketplace organisasi sambil memblokir segalanya
* **Segalanya yang lain diblokir**: user, project, dan local hooks, hooks dari plugin lain, dan hooks yang dideklarasikan dalam frontmatter agent
* **Plugin bersumber perintah dinonaktifkan**: Claude Code juga menonaktifkan plugin dengan [`command` source](/docs/id/plugins/marketplace-reference#command-plugin-source), termasuk plugin yang dipaksa diaktifkan dalam `enabledPlugins` terkelola, kecuali Anda menyetel [`disableCommandPluginSources`](#disablecommandpluginsources) ke `false` secara eksplisit
* **Perintah marketplace `headersHelper` diblokir**: Claude Code juga memblokir marketplace [`headersHelper` commands](/docs/id/plugins/host-marketplace#authenticate-archive-downloads) kecuali [`disableCommandPluginSources`](#disablecommandpluginsources) secara eksplisit diatur ke `false`, kecuali untuk marketplace yang dideklarasikan oleh pengaturan terkelola itu sendiri. Memerlukan Claude Code v2.1.238 atau lebih baru
* **Status line dan file suggestion menyempit ke pengaturan terkelola**: Claude Code membaca [`statusLine`](/docs/id/statusline), [`fileSuggestion`](#filesuggestion), dan [`subagentStatusLine`](/docs/id/statusline#subagent-status-lines) dari pengaturan terkelola saja, mengikuti [status line dan file suggestion gates](#status-line-and-file-suggestion-gates)

Perintah [`/goal`](/docs/id/goal) tidak dapat berjalan saat kunci ini diatur, karena bergantung pada hooks.

<h3 id="disableallhooks">
  `disableAllHooks`
</h3>

Matikan [hooks](/docs/id/hooks#disable-or-remove-hooks), [status line](/docs/id/statusline) kustom apa pun, dan perintah [file suggestion](#filesuggestion) kustom apa pun. Gunakan untuk mematikan semua ini secara sementara tanpa menghapusnya dari pengaturan Anda.

* **Scope**: [`Any file`](#scopes). Hanya pengaturan terkelola yang dapat menonaktifkan hook terkelola.
* **Type**: Boolean
  * `true`: Claude Code mematikan hooks, status line kustom apa pun, dan perintah file suggestion kustom apa pun
  * `false`: hooks, status line, dan perintah file suggestion berjalan
* **Default**: tidak diatur, jadi hooks berjalan

```json settings.json theme={null}
{
  "disableAllHooks": true
}
```

Jangkauan tergantung pada file mana yang membawa kunci:

* **Dalam pengaturan terkelola**: Claude Code menonaktifkan setiap hook yang dikonfigurasi, termasuk yang terkelola, dan terus menjalankan hooks yang didaftarkan [Agent SDK](/docs/id/agent-sdk/overview) dalam proses
* **Dalam file pengaturan lainnya**: Claude Code menonaktifkan user, project, local, dan plugin hooks; hook terkelola, hook Agent SDK, dan hooks dari plugin yang dipaksa diaktifkan dalam [`enabledPlugins`](#enabledplugins) terkelola terus berjalan

Menjaga hook Agent SDK tetap berjalan ketika pengaturan terkelola menyetel kunci ini memerlukan Claude Code v2.1.242 atau lebih baru.

Perintah [`/goal`](/docs/id/goal) tidak dapat berjalan saat hooks dinonaktifkan, dan menu `/hooks` menampilkan pemberitahuan alih-alih hooks Anda.

<h4 id="status-line-and-file-suggestion-gates">
  Status line dan file suggestion gates
</h4>

Claude Code membuat dua keputusan untuk `statusLine`, `fileSuggestion`, dan `subagentStatusLine`, dalam urutan ini:

* **Matikan sepenuhnya**: ketika pengaturan terkelola menyetel `disableAllHooks`, atau ketika folder tidak dipercaya di bawah [workspace trust rule yang sama dengan hooks dalam file pengaturan](/docs/id/permissions#what-runs-before-you-trust-a-folder)
* **Menyempit ke pengaturan terkelola**: ketika [`allowManagedHooksOnly`](#allowmanagedhooksonly) diatur, ketika `disableAllHooks` adalah `true` di luar pengaturan terkelola setelah [settings precedence](/docs/id/hooks#disable-or-remove-hooks) diterapkan, atau ketika Anda memulai Claude Code dengan `--safe-mode`

Di bawah penyempitan, Claude Code menjalankan nilai terkelola jika satu digunakan. Jika tidak, Claude Code melewati nilai Anda tanpa peringatan: status line dinonaktifkan, dan `@` autocomplete kembali ke file suggestion bawaan.

<h3 id="disableworkflows">
  `disableWorkflows`
</h3>

Matikan [dynamic workflows](/docs/id/workflows#turn-workflows-off) dan perintah workflow bundel untuk semua orang yang dijangkau pengaturan Anda, seperti organisasi melalui pengaturan terkelola. Untuk menyalakan atau mematikan alur kerja hanya untuk diri sendiri, gunakan [`enableWorkflows`](#enableworkflows) sebagai gantinya, yang ditulis oleh toggle **Dynamic workflows** di `/config` ke pengaturan pengguna Anda.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code mematikan dynamic workflows dan perintah workflow bundel untuk semua orang yang dijangkau pengaturan Anda
  * `false`: sama dengan tidak diatur; apakah alur kerja aktif kemudian mengikuti [`enableWorkflows`](#enableworkflows) dan default rencana Anda
* **Default**: `false`
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/id/env-vars) mematikan alur kerja untuk satu sesi; mana pun dari keduanya yang mematikannya, yang lain tidak dapat menyalakannya kembali

```json settings.json theme={null}
{
  "disableWorkflows": true
}
```

<h3 id="enableworkflows">
  `enableWorkflows`
</h3>

Nyalakan atau matikan [dynamic workflows](/docs/id/workflows) untuk diri sendiri ketika default rencana Anda bukan apa yang Anda inginkan. Muncul di `/config` sebagai **Dynamic workflows**, yang menulis kunci ini ke pengaturan pengguna Anda dan menghapusnya lagi ketika Anda beralih kembali ke default rencana Anda. Untuk mematikan alur kerja untuk semua orang dari pengaturan terkelola, gunakan [`disableWorkflows`](#disableworkflows) sebagai gantinya.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code menyalakan dynamic workflows untuk Anda
  * `false`: Claude Code mematikan dynamic workflows untuk Anda
* **Default**: tidak diatur, jadi alur kerja aktif kecuali Anda berada di rencana Pro, di mana alur kerja dimatikan
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/id/env-vars) mematikan alur kerja untuk satu sesi, dan `true` di sini tidak dapat menyalakannya kembali saat diatur

```json settings.json theme={null}
{
  "enableWorkflows": true
}
```

[`disableWorkflows`](#disableworkflows) dan kebijakan alur kerja organisasi Anda juga mengambil alih: `enableWorkflows: true` tidak dapat menyalakan alur kerja kembali saat sumber apa pun mematikan alur kerja. Claude Code menyembunyikan baris `/config` saat sumber selain pengaturan pengguna Anda menyetel `enableWorkflows`, atau menyetel `disableWorkflows` ke `true`.

<h3 id="hooks">
  `hooks`
</h3>

Jalankan perintah, prompt, agent, permintaan HTTP, atau alat MCP Anda sendiri sebagai [hooks](/docs/id/hooks) pada titik dalam siklus hidup Claude Code, seperti sebelum panggilan alat atau ketika sesi dimulai; [referensi hooks](/docs/id/hooks#hook-events) mencantumkan setiap acara, payload-nya, dan kode keluarnya. Setiap acara memetakan ke daftar grup matcher, dan setiap grup mencantumkan handler yang akan dijalankan ketika matcher berlaku.

* **Scope**: [`Any file`](#scopes). Hooks digabungkan di seluruh file daripada menggantikan satu sama lain, dan hooks dari pengaturan terkelola tidak dapat dihapus dari file lain.
* **Type**: object yang dikunci oleh [hook event](/docs/id/hooks#hook-events); setiap nilai adalah array dari grup `{ "matcher", "hooks" }` yang entri `hooks`-nya memiliki `type` dari `"command"`, `"prompt"`, `"agent"`, `"http"`, atau `"mcp_tool"`
* **Default**: tidak diatur, jadi tidak ada hooks yang berjalan

Contoh ini menjalankan skrip sebelum setiap panggilan alat Bash:

```json settings.json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          { "type": "command", "command": "~/.claude/hooks/check-bash.sh" }
        ]
      }
    ]
  }
}
```

Untuk setiap acara, pola matcher, dan bidang handler, lihat [referensi hooks](/docs/id/hooks#configuration). Untuk mematikan hooks, lihat [`disableAllHooks`](#disableallhooks); untuk membatasi hooks ke yang digunakan organisasi Anda, lihat [`allowManagedHooksOnly`](#allowmanagedhooksonly).

<h3 id="httphookallowedenvvars">
  `httpHookAllowedEnvVars`
</h3>

[HTTP hook](/docs/id/hooks#http-hook-fields) dapat menempatkan nilai variabel lingkungan ke dalam header permintaan, misalnya header `Authorization: Bearer $HOOK_TOKEN`, tetapi hanya untuk variabel yang dicantumkan hook dalam `allowedEnvVars`-nya sendiri. Kunci ini menetapkan batas luar pada daftar itu untuk setiap HTTP hook: hook dapat menggunakan variabel hanya jika baik `allowedEnvVars`-nya sendiri maupun kunci ini menamakannya. Gunakan untuk menghentikan hook dari membaca rahasia yang tidak seharusnya, bahkan ketika definisi hook memintanya.

* **Scope**: [`Any file`](#scopes). Array digabungkan di seluruh file pengaturan.
* **Type**: array nama variabel lingkungan
* **Default**: tidak diatur, jadi daftar `allowedEnvVars` hook sendiri berlaku

Contoh ini membatasi interpolasi header ke `MY_TOKEN` dan `HOOK_SECRET`:

```json settings.json theme={null}
{
  "httpHookAllowedEnvVars": ["MY_TOKEN", "HOOK_SECRET"]
}
```

Daftar izin berlaku untuk hooks dari setiap sumber, termasuk pengaturan terkelola.

<h3 id="workflowkeywordtriggerenabled">
  `workflowKeywordTriggerEnabled`
</h3>

Pilih apakah mengetik kata kunci `ultracode` dalam prompt memicu [dynamic workflow](/docs/id/workflows#ask-for-a-workflow-in-your-prompt). Atur ke `false` untuk mengetik kata tanpa memicu satu.

* **Scope**: [`Any file`](#scopes). Muncul di `/config` sebagai **Ultracode keyword trigger**.
* **Type**: Boolean
  * `true`: mengetik `ultracode` dalam prompt memicu dynamic workflow
  * `false`: Anda dapat mengetik kata tanpa memicu satu
* **Default**: `true`

```json settings.json theme={null}
{
  "workflowKeywordTriggerEnabled": false
}
```

Pengaturan upaya `ultracode`, `/workflows`, dan perintah alur kerja yang disimpan tidak terpengaruh.

<h3 id="workflowsizeguideline">
  `workflowSizeGuideline`
</h3>

Atur [jumlah agent yang Claude targetkan](/docs/id/workflows#set-a-size-guideline) dalam dynamic workflows yang ditulisnya. Claude Code mengirimkan nilai ke Claude sebagai saran, bukan batas yang diberlakukan: `"small"` meminta lebih sedikit dari 5 agent, `"medium"` lebih sedikit dari 10, dan `"large"` lebih sedikit dari 50. Pilih `"small"` ketika Anda ingin membatasi apa yang dihabiskan alur kerja. Memerlukan Claude Code v2.1.219 atau lebih baru.

* **Scope**: [`Any file`](#scopes). Nilai di sana mengambil alih pilihan **Dynamic workflow size** di `/config`, yang disimpan Claude Code di `~/.claude.json`, dan Claude Code menyembunyikan baris itu saat file pengaturan menyetel kunci.
* **Type**: string, salah satu dari:
  * `"unrestricted"`: tidak ada panduan, jadi Claude mengukur alur kerja ke tugas
  * `"small"`: Claude menargetkan lebih sedikit dari 5 agent
  * `"medium"`: Claude menargetkan lebih sedikit dari 10 agent
  * `"large"`: Claude menargetkan lebih sedikit dari 50 agent
* **Default**: `"medium"`, atau `"small"` ketika Anda masuk di rencana Pro dengan Claude Code v2.1.271 atau lebih baru

```json settings.json theme={null}
{
  "workflowSizeGuideline": "small"
}
```

Memerlukan Claude Code v2.1.219 atau lebih baru; pada v2.1.202 hingga v2.1.218, atur panduan di `/config` sebagai gantinya.

<span id="plugin-configuration" />

<span id="manage-plugins" />

<span id="plugin-settings" />

<h2 id="plugins-and-skills">
  Plugins dan skills
</h2>

Aktifkan plugins, daftarkan marketplaces, batasi sumber plugin mana yang diizinkan organisasi, dan kontrol skills mana yang dimuat. Untuk menginstal dan membangun plugins, lihat [Plugins](/docs/id/plugins/overview).

<h3 id="disablebundledskills">
  `disableBundledSkills`
</h3>

Matikan [skills](/docs/id/skills) dan workflows yang disertakan dengan Claude Code. Claude Code menghapus skills dan workflows bundel sepenuhnya, sementara perintah bawaan seperti `/init` tetap dapat diketik tetapi disembunyikan dari model.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code menghapus skills dan workflows bundel dan menyembunyikan perintah bawaan seperti `/init` dari model
  * `false`: skills bundel dimuat
* **Default**: unset, jadi skills bundel dimuat
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_BUNDLED_SKILLS`](/docs/id/env-vars) diatur ke `1` mematikan skills bundel untuk satu sesi; mana pun dari keduanya yang mematikannya, yang lain tidak dapat menghidupkannya kembali

```json settings.json theme={null}
{
  "disableBundledSkills": true
}
```

Skills dari plugins, `.claude/skills/`, dan `.claude/commands/` tidak terpengaruh. `/doctor` tetap dapat diketik seperti perintah bawaan; untuk menyembunyikannya, atur [`DISABLE_DOCTOR_COMMAND`](/docs/id/env-vars) sebagai gantinya.

<h3 id="disableskillshellexecution">
  `disableSkillShellExecution`
</h3>

Matikan eksekusi shell inline untuk `` !`...` `` dan ` ```! ` blocks dalam [skills](/id/skills) dan perintah kustom dari sumber pengguna, proyek, plugin, atau direktori tambahan. Claude Code mengganti setiap perintah dengan `[shell command execution disabled by policy]` alih-alih menjalankannya.

* **Scope**: [`Any file`](#scopes). Sebuah `true` dalam managed settings tidak dapat ditimpa oleh `false` di tempat lain.
* **Type**: Boolean
  * `true`: Claude Code mengganti setiap perintah shell inline dengan `[shell command execution disabled by policy]` alih-alih menjalankannya
  * `false`: shell inline berjalan
* **Default**: unset, jadi shell inline berjalan

```json settings.json theme={null}
{
  "disableSkillShellExecution": true
}
```

Skills bundel dan skills yang digunakan melalui managed settings tidak terpengaruh.

<h3 id="skilloverrides">
  `skillOverrides`
</h3>

Sembunyikan atau lipat [skill](/docs/id/skills#override-skill-visibility-from-settings) tanpa mengedit `SKILL.md`-nya. Claude Code menerapkan nilai di bawah nama setiap skill ke daftar skill yang Claude lihat dan ke autocomplete `/` Anda.

* **Scope**: [`Any file`](#scopes). Menu `/skills` menulis ke `.claude/settings.local.json`.
* **Type**: object mapping nama skill ke salah satu dari:
  * `"on"`: Claude melihat skill dan Anda dapat mengetik `/name`
  * `"name-only"`: Claude melihat skill berdasarkan nama tanpa deskripsinya
  * `"user-invocable-only"`: Claude tidak melihat skill, tetapi Anda masih dapat mengetik `/name`
  * `"off"`: Claude tidak melihat skill dan `/name` disembunyikan dari autocomplete
* **Default**: unset, jadi setiap skill adalah `"on"`

Contoh ini mencantumkan `legacy-context` ke Claude hanya berdasarkan nama dan menyembunyikan `deploy` dari Claude dan dari autocomplete `/`:

```json settings.json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Overrides tidak berlaku untuk plugin skills, yang Anda kelola melalui `/plugin`.

Dalam managed settings dan file yang dilewatkan dengan `--settings`, kunci pada alias skill bundel, seperti `checkup` untuk `/doctor`, juga berlaku untuk skill; lihat [bagaimana kunci alias menggabungkan dengan kunci pada nama skill itu sendiri](/docs/id/skills#override-skill-visibility-from-settings).

<h3 id="syncclaudeaiskills">
  `syncClaudeAiSkills`
</h3>

Matikan unduhan [skills yang diaktifkan untuk akun claude.ai Anda](/docs/id/skills#how-synced-skills-behave). Claude Code mengunduhnya ke `~/.claude/skills/synced/` dalam [sesi terminal di mana Anda masuk dengan akun claude.ai Anda](/docs/id/skills#where-synced-skills-load), interaktif atau non-interaktif, dan dalam sesi Cowork dan cloud. Atur `false` untuk menghentikan unduhan itu dan berhenti memuat skills yang sudah disinkronkan. Claude Code hanya menghormati `false`: `true` sama dengan unset dan tidak menghidupkan sinkronisasi di mana sebaliknya mati.

* **Scope**: [`User, local, or managed`](#scopes), dan file yang dilewatkan dengan `--settings`. Repositori tidak dapat mematikannya untuk Anda.
* **Type**: Boolean
  * `false`: Claude Code berhenti mengunduh skills yang disinkronkan dan berhenti memuat yang sudah ada di `~/.claude/skills/synced/`. Dalam user atau managed settings, itu juga memindahkannya ke `~/.claude/skills/.trash/`
  * `true`: sama dengan unset
* **Default**: unset, jadi sesi yang masuk dengan akun claude.ai Anda menyinkronkan skills Anda

Contoh ini mencegah mesin mengunduh skills akun dalam sesi apa pun:

```json settings.json theme={null}
{
  "syncClaudeAiSkills": false
}
```

<h3 id="syncclaudeaiplugins">
  `syncClaudeAiPlugins`
</h3>

Matikan unduhan [plugins yang diaktifkan untuk akun claude.ai Anda](/docs/id/plugins/loading#synced-plugins). Claude Code mengunduhnya ke `~/.claude/plugins/synced/` pada awal sesi terminal di mana Anda masuk dengan akun claude.ai Anda dan dalam sesi Cowork, dan memuat masing-masing sebagai `<name>@synced`. Atur `false` untuk menghentikan unduhan itu dan berhenti memuat plugins yang sudah disinkronkan. Claude Code hanya menghormati `false`: `true` sama dengan unset dan tidak menghidupkan sinkronisasi di mana sebaliknya mati. Memerlukan Claude Code v2.1.273 atau lebih baru.

* **Scope**: [`User, local, or managed`](#scopes), dan file yang dilewatkan dengan `--settings`. Repositori tidak dapat mematikannya untuk Anda.
* **Type**: Boolean
  * `false`: Claude Code berhenti mengunduh plugins yang disinkronkan dan berhenti memuat yang sudah ada di `~/.claude/plugins/synced/`. Dalam user atau managed settings, itu juga memindahkannya ke `~/.claude/plugins/.trash/`
  * `true`: sama dengan unset
* **Default**: unset, jadi sesi yang masuk dengan akun claude.ai Anda menyinkronkan plugins Anda

Untuk mematikan satu plugin yang disinkronkan daripada semuanya, atur `"<name>@synced": false` dalam [`enabledPlugins`](#enabledplugins).

Contoh ini mencegah mesin mengunduh plugins akun dalam sesi apa pun:

```json settings.json theme={null}
{
  "syncClaudeAiPlugins": false
}
```

<h3 id="allowedchannelplugins">
  `allowedChannelPlugins`
</h3>

Pilih [channel](/docs/id/channels) plugins mana yang dapat mendorong pesan ke dalam sesi di organisasi Anda. Ketika Anda mengaturnya, Claude Code menggunakan daftar Anda sebagai pengganti daftar allowlist default Anthropic; setiap entri menamai plugin dan marketplace tempat asalnya.

* **Scope**: [`Managed`](#scopes)
* **Type**: array objek, masing-masing dengan string `marketplace` dan `plugin`. Sebuah entri dapat sebagai gantinya menjadi string `"plugin@marketplace"` seperti `"telegram@claude-plugins-official"`, yang Claude Code perlakukan sebagai objek yang setara. Bentuk string memerlukan Claude Code v2.1.267 atau lebih baru; versi sebelumnya menolak seluruh nilai `allowedChannelPlugins` ketika berisi satu
* **Default**: unset, jadi Claude Code menggunakan daftar allowlist default Anthropic

Contoh ini menghidupkan channels dan hanya mengizinkan plugin Telegram dari marketplace resmi Anthropic:

```json managed-settings.json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" }
  ]
}
```

Array kosong memblokir setiap channel plugin.

Kunci ini berlaku setelah channels melewati gerbang [`channelsEnabled`](#channelsenabled) untuk akun: pada paket Team dan Enterprise, dan pada akun Console dengan managed settings, itu berarti `channelsEnabled: true`. Lihat [Batasi plugins channel mana yang dapat berjalan](/docs/id/channels#restrict-which-channel-plugins-can-run).

<h3 id="blockedmarketplaces">
  `blockedMarketplaces`
</h3>

Blokir sumber marketplace plugin untuk organisasi Anda. Claude Code memeriksa daftar blokir pada penambahan marketplace dan pada instalasi, pembaruan, penyegaran, dan auto-update plugin, jadi marketplace yang ditambahkan seseorang sebelum Anda menetapkan kebijakan tidak dapat digunakan untuk mengambil plugins. Sumber yang diblokir diperiksa sebelum unduhan, jadi mereka tidak pernah menyentuh filesystem.

Jika Anda menetapkan kunci ini dalam [konsol admin claude.ai](/docs/id/server-managed-settings), claude.ai juga menerapkannya ketika siapa pun di organisasi Anda menambahkan marketplace dari repositori git di claude.ai, seperti yang dijelaskan [Bagaimana pembatasan bekerja](/docs/id/plugins/org#restrict-what-users-can-install).

* **Scope**: [`Managed`](#scopes)
* **Type**: array objek sumber marketplace, dalam bentuk yang sama seperti [`strictKnownMarketplaces`](#allowed-source-types)
* **Default**: unset, jadi tidak ada marketplace yang diblokir

Contoh ini memblokir satu repositori GitHub sebagai sumber marketplace:

```json managed-settings.json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted/plugins" }
  ]
}
```

Entri `github` dapat menggunakan [bentuk owner-wildcard](#owner-wildcards) `"owner/*"` untuk memblokir setiap repositori di bawah pemilik GitHub itu, yang memerlukan Claude Code v2.1.223 atau lebih baru. Tambahkan `{ "source": "skills-dir" }` untuk menghentikan Claude Code memuat [`@skills-dir` plugins](/docs/id/plugins/loading#plugins-shared-through-a-repository) dari `~/.claude/skills/` tanpa membatasi marketplace apa pun. Lihat [Pembatasan marketplace yang dikelola](/docs/id/plugins/org#restrict-what-users-can-install).

<h3 id="channelsenabled">
  `channelsEnabled`
</h3>

Izinkan [channels](/docs/id/channels) untuk organisasi Anda. Pada paket Team dan Enterprise claude.ai, Claude Code memblokir channels sampai Anda mengaturnya ke `true`. Untuk akun [Anthropic Console](/docs/id/authentication#claude-console-authentication) yang mengautentikasi dengan kunci API, channels diizinkan secara default. Jika organisasi Anda menerapkan managed settings, Claude Code juga memblokir channels pada akun tersebut sampai Anda mengatur kunci ini ke `true`.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code mengizinkan channels untuk organisasi Anda
  * `false`: sama dengan unset; apakah channels diblokir tergantung pada paket Anda, seperti yang dikatakan Default
* **Default**: unset; channels diblokir pada paket Team dan Enterprise dan pada akun Console dengan managed settings, dan diizinkan pada paket Pro dan Max dan pada akun Console tanpa managed settings

```json managed-settings.json theme={null}
{
  "channelsEnabled": true
}
```

Untuk membatasi plugins mana yang dapat mendaftar sebagai channels setelah diaktifkan, atur [`allowedChannelPlugins`](#allowedchannelplugins). Lihat [Kontrol enterprise](/docs/id/channels#enterprise-controls).

<h3 id="disablecommandpluginsources">
  `disableCommandPluginSources`
</h3>

Blokir [sumber plugin `command`](/docs/id/plugins/marketplace-reference#command-plugin-source), yang menginstal plugin dengan menjalankan perintah yang dideklarasikan marketplace pada mesin pengguna. Ketika Anda mengaturnya ke `true`, Claude Code tidak pernah menjalankan perintah, tidak menginstal atau memperbarui plugins yang bersumber perintah, dan berhenti memuat yang sudah diinstal. Atur ke `false` untuk mengizinkannya secara eksplisit. Kapan pun itu memblokir sumber perintah, apakah Anda mengaturnya ke `true` atau membiarkannya unset di bawah [`allowManagedHooksOnly`](#allowmanagedhooksonly), itu juga memblokir perintah marketplace [`headersHelper`](/docs/id/plugins/host-marketplace#authenticate-archive-downloads), kecuali untuk marketplace yang managed settings sendiri deklarasikan. Memerlukan Claude Code v2.1.229 atau lebih baru, dan blokir `headersHelper` memerlukan v2.1.238 atau lebih baru.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code tidak pernah menjalankan perintah yang dideklarasikan marketplace, tidak menginstal atau memperbarui plugins yang bersumber perintah, dan berhenti memuat yang sudah diinstal
  * `false`: Claude Code mengizinkan plugins yang bersumber perintah secara eksplisit
* **Default**: unset, jadi Claude Code mengikuti [`allowManagedHooksOnly`](#allowmanagedhooksonly): organisasi yang membatasi eksekusi hook ke managed settings juga mendapatkan sumber perintah dinonaktifkan

```json managed-settings.json theme={null}
{
  "disableCommandPluginSources": true
}
```

Memerlukan Claude Code v2.1.229 atau lebih baru.

<h3 id="pluginsuggestionmarketplaces">
  `pluginSuggestionMarketplaces`
</h3>

Namai marketplaces yang plugins-nya dapat muncul sebagai saran instalasi kontekstual, dalam tips spinner dan disematkan di bagian atas tab **Discover** `/plugin`. Tip frontend-design pihak pertama bawaan tidak terpengaruh. Saran berasal dari deklarasi `relevance` setiap plugin dalam entri marketplace-nya.

* **Scope**: [`Managed`](#scopes)
* **Type**: array nama marketplace
* **Default**: unset, jadi tidak ada saran yang dideklarasikan marketplace yang muncul

```json managed-settings.json theme={null}
{
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

Nama berlaku hanya ketika marketplace terdaftar di mesin dan sumber terdaftarnya juga dideklarasikan dalam managed settings yang sama, baik sebagai entri [`extraKnownMarketplaces`](#extraknownmarketplaces) untuk nama itu atau sebagai entri [`strictKnownMarketplaces`](#strictknownmarketplaces). Claude Code mengabaikan marketplace yang terdaftar dari sumber berbeda di bawah nama yang diizinkan. Marketplace resmi dikecualikan dari persyaratan sumber: mengizinkan nama saja sudah cukup, karena nama itu hanya dapat terdaftar dari sumber Anthropic resmi. Lihat [Sarankan plugins berdasarkan konteks](/docs/id/plugins/relevance).

<h3 id="plugintrustmessage">
  `pluginTrustMessage`
</h3>

Tambahkan teks organisasi Anda sendiri ke peringatan kepercayaan plugin yang Claude Code tunjukkan sebelum instalasi, misalnya untuk mengkonfirmasi bahwa plugins dari marketplace internal Anda telah dikurasi.

* **Scope**: [`Managed`](#scopes)
* **Type**: string
* **Default**: unset, jadi Claude Code menunjukkan peringatan standar saja

```json managed-settings.json theme={null}
{
  "pluginTrustMessage": "All plugins from our marketplace are approved by IT"
}
```

<h3 id="strictknownmarketplaces">
  `strictKnownMarketplaces`
</h3>

Batasi sumber marketplace plugin mana yang dapat ditambahkan dan diinstal oleh orang-orang di organisasi Anda. Claude Code memberlakukan daftar allowlist pada penambahan marketplace dan pada instalasi, pembaruan, penyegaran, dan auto-update plugin, sebelum operasi jaringan atau filesystem apa pun, jadi marketplace yang ditambahkan seseorang sebelum Anda menetapkan kebijakan tidak dapat digunakan untuk mengambil plugins setelah sumbernya tidak lagi cocok. Pengguna yang diblokir melihat kesalahan yang menamai kebijakan yang dikelola.

Jika Anda menetapkan kunci ini dalam [konsol admin claude.ai](/docs/id/server-managed-settings), claude.ai juga menerapkannya ketika siapa pun di organisasi Anda menambahkan marketplace dari repositori git di claude.ai, seperti yang dijelaskan [Bagaimana pembatasan bekerja](/docs/id/plugins/org#restrict-what-users-can-install).

* **Scope**: [`Managed`](#scopes)
* **Type**: array objek sumber marketplace; lihat [Tipe sumber yang diizinkan](#allowed-source-types)
* **Default**: unset, jadi pengguna dapat menambahkan marketplace apa pun. Array kosong adalah lockdown lengkap yang memblokir setiap sumber marketplace, termasuk marketplace resmi Anthropic

Contoh ini mengizinkan dua repositori GitHub, satu disematkan ke ref `v2.0`, dan satu URL `marketplace.json` yang dihosting:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/approved-plugins" },
    { "source": "github", "repo": "acme-corp/security-tools", "ref": "v2.0" },
    { "source": "url", "url": "https://plugins.example.com/marketplace.json" }
  ]
}
```

Anda juga dapat menulis kunci ini sebagai `allowedMarketplaces`; [Alias kunci Marketplace](#marketplace-key-aliases) menjelaskan bagaimana Claude Code memperlakukan alias dan versi mana yang menerimanya. Kunci ini adalah gerbang kebijakan: itu mengontrol apa yang dapat ditambahkan pengguna tetapi tidak mendaftarkan apa pun. Untuk membatasi dan pra-daftar dalam satu file, lihat [Gabungkan dengan `extraKnownMarketplaces`](#combine-with-extraknownmarketplaces). Untuk tampilan yang menghadap pengguna, lihat [Pembatasan marketplace yang dikelola](/docs/id/plugins/org#restrict-what-users-can-install).

<h4 id="allowed-source-types">
  Tipe sumber yang diizinkan
</h4>

Setiap entri di bawah menunjukkan satu entri daftar allowlist per tipe sumber dan bidang yang diterimanya. Sebagian besar tipe cocok persis; `hostPattern` dan `pathPattern` cocok dengan regex, dan entri `github` dapat menggunakan [wildcard pemilik](#owner-wildcards).

| Source        | Contoh entri                                                                                                                    | Bidang                                                                                                                                             |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| `github`      | `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main", "path": "marketplace" }`                                     | `repo` diperlukan; `ref` adalah cabang atau tag; `path` adalah subdirektori                                                                        |
| `git`         | `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git", "ref": "production" }`                               | `url` diperlukan; `ref` dan `path` seperti untuk `github`                                                                                          |
| `url`         | `{ "source": "url", "url": "https://plugins.example.com/marketplace.json", "headers": { "Authorization": "Bearer ${TOKEN}" } }` | `url` diperlukan; `headers` menambahkan header HTTP untuk akses yang diautentikasi                                                                 |
| `file`        | `{ "source": "file", "path": "/opt/acme-corp/plugins/marketplace.json" }`                                                       | `path` diperlukan, jalur absolut ke file `marketplace.json`                                                                                        |
| `directory`   | `{ "source": "directory", "path": "/opt/acme-corp/approved-marketplaces" }`                                                     | `path` diperlukan, jalur absolut ke direktori yang berisi `.claude-plugin/marketplace.json`                                                        |
| `hostPattern` | `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`                                                        | `hostPattern` diperlukan, regex yang cocok di mana saja dalam host marketplace; jangkarnya dengan `^` dan `$` untuk mencocokkan seluruh host       |
| `pathPattern` | `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`                                                                 | `pathPattern` diperlukan, regex yang cocok di mana saja dalam `path` dari sumber `file` dan `directory`; mulai dengan `^` untuk menancapkan awalan |
| `skills-dir`  | `{ "source": "skills-dir" }`                                                                                                    | Tidak ada bidang. Memilih kembali pemindaian plugin `~/.claude/skills/`                                                                            |

Tiga tipe sumber membawa aturan di luar tabel:

* **`url`**: marketplace URL hanya mengunduh file `marketplace.json`, dan Claude Code tidak mengambil file plugin dengan jalur relatif dari server itu, jadi plugins-nya harus menggunakan [sumber plugin](/docs/id/plugins/marketplace-reference#plugin-sources) selain jalur relatif, seperti URL arsip, yang dapat berada di host yang sama. Untuk plugins dengan jalur relatif, gunakan marketplace berbasis Git sebagai gantinya. Lihat [Plugins dengan jalur relatif gagal dalam marketplaces berbasis URL](/docs/id/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces).
* **`hostPattern`**: gunakan untuk mengizinkan setiap marketplace di server GitHub Enterprise atau GitLab internal tanpa mencantumkan setiap repositori. Claude Code mencocokkan sumber `github` terhadap `github.com`, mengambil hostname dari sumber `url`, dan mengambilnya dari sumber `git` tergantung pada bentuk [git URL](https://git-scm.com/docs/git-clone#_git_urls):

  * URL dengan skema, seperti `https://` atau `ssh://`: hostname dalam URL.
  * Alamat SSH tanpa skema, dalam bentuk `user@host:path` git, seperti `git@git.example.com:tools/plugins.git`: host antara `@` dan `:`, yang merupakan host yang terhubung git.
  * Bentuk lain apa pun tanpa skema: tidak ada host, jadi tidak ada entri `hostPattern` `strictKnownMarketplaces` yang cocok dengannya. Untuk `hostPattern` `blockedMarketplaces`, Claude Code mengambil host dari set bentuk yang lebih luas, jadi entri daftar blokir masih dapat cocok dengan bentuk seperti itu. Sebelum v2.1.234, `hostPattern` `strictKnownMarketplaces` juga cocok dengan beberapa bentuk yang git tidak perlakukan sebagai alamat SSH.

  Sumber `file` dan `directory` tidak memiliki host dan tidak pernah cocok dengan entri `hostPattern`.
* **`pathPattern`**: gunakan untuk mengizinkan marketplaces filesystem bersama entri `hostPattern` untuk sumber jaringan. `".*"` mengizinkan setiap jalur lokal; pola yang lebih sempit seperti `"^/opt/approved/"` membatasi ke direktori.

Daftar allowlist apa pun, bahkan yang kosong, juga menghentikan Claude Code memuat [`@skills-dir` plugins](/docs/id/plugins/loading#plugins-shared-through-a-repository) dari `~/.claude/skills/`. Tambahkan entri `{ "source": "skills-dir" }` untuk terus memuat mereka; entri tidak memiliki arti di luar kunci ini dan `blockedMarketplaces`.

<h4 id="owner-wildcards">
  Owner wildcards
</h4>

Entri `github` yang nilai `repo`-nya adalah `"<owner>/*"` cocok dengan setiap repositori di bawah pemilik GitHub itu. Wildcard pemilik memerlukan Claude Code v2.1.223 atau lebih baru dan hanya bekerja dalam `strictKnownMarketplaces` dan `blockedMarketplaces`. Di tempat lain sumber `github` muncul, seperti `extraKnownMarketplaces` atau `/plugin marketplace add`, nilai `repo` harus menamai satu repositori. Sebelum v2.1.223, Claude Code membandingkan entri secara harfiah, jadi entri daftar allowlist tidak cocok dengan repositori apa pun dan entri daftar blokir tidak memblokir apa pun; entri repositori tunggal diberlakukan pada setiap versi.

Entri ini mengizinkan repositori marketplace apa pun dalam organisasi `acme-corp`:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/*" }
  ]
}
```

Hanya seluruh posisi nama-repositori yang dapat menjadi wildcard. Claude Code mengabaikan entri seperti `*`, `*/plugins`, atau `acme-corp/tools-*` sebagai tidak valid, jadi mereka tidak cocok dengan repositori apa pun.

Aturan pencocokan berbeda antara dua pengaturan:

| Aturan                  | `strictKnownMarketplaces`                                                                                                                                                           | `blockedMarketplaces`                                                                |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| Pencocokan ejaan sumber | Bentuk `owner/repo` saja. URL git yang mengkloning repositori yang sama tidak cocok                                                                                                 | Ejaan apa pun, termasuk URL git yang diselesaikan ke repositori github.com yang sama |
| Kasus pemilik           | Peka huruf besar-kecil, seperti pencocokan entri yang tepat                                                                                                                         | Tidak peka huruf besar-kecil                                                         |
| `ref`                   | Mengikuti aturan entri yang tepat: entri dengan `ref` hanya cocok dengan sumber dengan ref yang tepat itu, dan entri tanpa satu cocok hanya dengan sumber yang tidak menentukan ref | Entri tanpa `ref` memblokir semua ref dari repositori yang cocok dengannya           |
| `path`                  | Lebih longgar daripada aturan entri yang tepat: entri dengan `path` memerlukan nilai yang tepat itu, sementara entri tanpa satu cocok dengan jalur apa pun di dalam repositori      | Entri tanpa `path` memblokir semua jalur dari repositori yang cocok dengannya        |

<h4 id="exact-matching">
  Exact matching
</h4>

Untuk setiap tipe sumber kecuali entri `github` wildcard pemilik dan entri `hostPattern` dan `pathPattern` yang cocok dengan regex, Claude Code hanya mengizinkan penambahan pengguna ketika sumber marketplace cocok dengan entri secara tepat. Untuk sumber berbasis git `github` dan `git`, pencocokan yang tepat mencakup bidang opsional:

* `repo` atau `url` harus cocok persis
* Bidang `ref` harus cocok persis, atau keduanya harus tidak terdefinisi
* Bidang `path` harus cocok persis, atau keduanya harus tidak terdefinisi

Misalnya, Claude Code memperlakukan setiap pasangan di bawah sebagai dua sumber berbeda:

* `{ "source": "github", "repo": "acme-corp/plugins" }` dan `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main" }`
* `{ "source": "github", "repo": "acme-corp/plugins", "path": "marketplace" }` dan `{ "source": "github", "repo": "acme-corp/plugins" }`

<h4 id="allow-only-the-official-marketplace">
  Allow only the official marketplace
</h4>

Untuk mengizinkan marketplace resmi Anthropic dan tidak ada yang lain, cantumkan repositorinya:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" }
  ]
}
```

Dengan entri ini, Claude Code menjaga marketplace resmi yang sudah terdaftar tetap tersedia dan, di mesin baru, mendaftarkan marketplace secara otomatis pertama kali Anda memulai sesi terminal interaktif. Pendaftaran otomatis paling sering melewatkan:

* Lingkungan non-interaktif yang berjalan sebelum sesi terminal interaktif pertama mesin.
* Mesin di mana Claude Code sudah berjalan melalui ekstensi VS Code.
* Mesin di mana Claude Code sudah menjalankan sesi terminal interaktif di bawah kebijakan yang memblokir marketplace, seperti lockdown array kosong. Claude Code mencatat upaya yang diblokir dan tidak mencoba lagi setelah kebijakan berubah.

Pada mesin ini, tambahkan marketplace ke [`extraKnownMarketplaces`](#extraknownmarketplaces) dalam `managed-settings.json` yang sama sehingga Claude Code mendaftarkannya secara otomatis, atau jalankan `claude plugin marketplace add anthropics/claude-plugins-official`.

<h4 id="combine-with-extraknownmarketplaces">
  Combine with `extraKnownMarketplaces`
</h4>

Dua kunci melakukan pekerjaan berbeda. Tabel ini membandingkannya:

| Aspek              | `strictKnownMarketplaces`                 | `extraKnownMarketplaces`                                                                           |
| ------------------ | ----------------------------------------- | -------------------------------------------------------------------------------------------------- |
| Tujuan             | Penegakan kebijakan organisasi            | Kenyamanan tim                                                                                     |
| File pengaturan    | Hanya managed settings                    | File pengaturan apa pun                                                                            |
| Perilaku           | Memblokir penambahan yang tidak diizinkan | Mendaftarkan marketplace yang hilang                                                               |
| Kapan diberlakukan | Sebelum operasi jaringan dan filesystem   | Segera dari user atau managed settings; setelah dialog kepercayaan workspace untuk file repositori |
| Dapat ditimpa      | Tidak, prioritas tertinggi                | Ya, oleh pengaturan prioritas lebih tinggi                                                         |
| Format sumber      | Objek sumber langsung                     | Marketplace bernama dengan objek `source` bersarang                                                |

Untuk membatasi dan pra-daftar marketplace untuk semua pengguna, atur keduanya dalam `managed-settings.json`:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/plugins" }
  ],
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": { "source": "github", "repo": "acme-corp/plugins" }
    }
  }
}
```

Dengan hanya `strictKnownMarketplaces` yang diatur, pengguna masih dapat menambahkan marketplace yang diizinkan sendiri dengan `/plugin marketplace add`. Marketplace resmi Anthropic adalah satu-satunya yang Claude Code daftarkan secara otomatis, dan hanya ketika daftar allowlist mengizinkannya. [Allow only the official marketplace](#allow-only-the-official-marketplace) mencantumkan mesin yang dilewatkan.

<h3 id="strictpluginonlycustomization">
  `strictPluginOnlyCustomization`
</h3>

Blokir skills, agents, hooks, dan MCP servers dari sumber pengguna dan proyek, jadi mereka hanya dapat berasal dari plugins atau managed settings. Gabungkan dengan [`strictKnownMarketplaces`](#strictknownmarketplaces) untuk mengontrol rantai pasokan kustomisasi penuh: daftar allowlist marketplace mengontrol plugins mana yang dapat diinstal pengguna.

* **Scope**: [`Managed`](#scopes)
* **Type**: `true` untuk mengunci semua empat jenis kustomisasi, atau array yang menamai jenis untuk dikunci, dari `"skills"`, `"agents"`, `"hooks"`, dan `"mcp"`
* **Default**: unset, jadi tidak ada yang dikunci

Contoh ini mengunci skills dan hooks dan membiarkan agents dan MCP servers tidak terkunci:

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills", "hooks"]
}
```

Empat entri sub-kunci di bawah mencantumkan apa yang setiap permukaan blokir dan apa yang masih dimuat. Claude Code mengabaikan nama permukaan yang tidak dikenalinya daripada gagal file pengaturan, jadi Anda dapat menambahkan nama permukaan baru sebelum setiap klien diperbarui.

<h3 id="strictpluginonlycustomization-skills">
  `strictPluginOnlyCustomization.skills`
</h3>

Kunci permukaan `skills`. Claude Code berhenti memuat skills dari `~/.claude/skills/` dan `.claude/skills/`, perintah kustom dari `~/.claude/commands/` dan `.claude/commands/`, skills di bawah direktori `--add-dir`, dan skills yang disinkronkan dari akun claude.ai Anda, dan terus memuat plugin skills, bundled skills, dan skills dalam direktori kebijakan yang dikelola.

* **Scope**: [`Managed`](#scopes)
* **Type**: string `"skills"` dalam array [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: tidak dikunci

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills"]
}
```

<h3 id="strictpluginonlycustomization-agents">
  `strictPluginOnlyCustomization.agents`
</h3>

Kunci permukaan `agents`. Claude Code berhenti memuat agents dari `~/.claude/agents/` dan `.claude/agents/`, dan terus memuat plugin agents, agents bawaan, dan agents dalam direktori kebijakan yang dikelola.

* **Scope**: [`Managed`](#scopes)
* **Type**: string `"agents"` dalam array [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: tidak dikunci

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["agents"]
}
```

<h3 id="strictpluginonlycustomization-hooks">
  `strictPluginOnlyCustomization.hooks`
</h3>

Kunci permukaan `hooks`. Claude Code berhenti menjalankan hooks dari user, project, dan local `settings.json`, dan terus menjalankan plugin hooks dan hooks dalam managed settings.

* **Scope**: [`Managed`](#scopes)
* **Type**: string `"hooks"` dalam array [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: tidak dikunci

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["hooks"]
}
```

<h3 id="strictpluginonlycustomization-mcp">
  `strictPluginOnlyCustomization.mcp`
</h3>

Kunci permukaan `mcp`. Claude Code berhenti memuat MCP servers dari `~/.claude.json` dan `.mcp.json`, dan terus memuat plugin MCP servers, server [`managed-mcp.json`](/docs/id/managed-mcp), dan servers dari [`managedMcpServers`](#managedmcpservers).

* **Scope**: [`Managed`](#scopes)
* **Type**: string `"mcp"` dalam array [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: tidak dikunci

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["mcp"]
}
```

<h3 id="enabledplugins">
  `enabledPlugins`
</h3>

Hidupkan atau matikan [plugins](/docs/id/plugins/overview) individual, dikunci dengan `plugin-name@marketplace-name`. Plugin tanpa entri di scope apa pun kembali ke nilai [`defaultEnabled`](/docs/id/plugins/manifest-reference#fields)-nya. Ketika Anda mengaktifkan atau menonaktifkan plugin dengan `/plugin` atau `claude plugin enable`, Claude Code menulis kunci ini untuk Anda.

* **Scope**: [`Any file`](#scopes)
* **Type**: object mapping `plugin-name@marketplace-name` ke Boolean
* **Default**: unset, jadi setiap plugin mengikuti nilai `defaultEnabled`-nya

Contoh ini mengaktifkan dua plugins dari marketplace `team-tools` dan menonaktifkan satu dari `personal`:

```json settings.json theme={null}
{
  "enabledPlugins": {
    "code-formatter@team-tools": true,
    "deployment-tools@team-tools": true,
    "experimental-features@personal": false
  }
}
```

Setiap scope melayani tujuan berbeda:

* **User settings**: preferensi plugin pribadi Anda
* **Project settings**: plugins yang dibagikan dengan semua orang di repositori
* **Local settings**: overrides per-mesin, gitignored ketika Claude Code menyimpan pengaturan di sana
* **Managed settings**: kebijakan organisasi-lebar. Plugin yang diatur ke `false` di sini diblokir dari instalasi di setiap scope dan disembunyikan dari marketplace

Pengaturan proyek mengambil prioritas atas pengaturan pengguna, jadi mengatur plugin ke `false` dalam `~/.claude/settings.json` tidak menonaktifkan plugin yang `.claude/settings.json` proyek aktifkan. Untuk memilih keluar dari plugin yang diaktifkan proyek di mesin Anda, atur ke `false` dalam `.claude/settings.local.json` sebagai gantinya. Plugins yang dipaksa diaktifkan oleh managed settings tidak dapat dinonaktifkan dengan cara ini, karena managed settings menimpa pengaturan lokal.

Mengaktifkan plugin dari sumber eksternal seperti repositori GitHub atau paket npm dalam `.claude/settings.json` proyek tidak menginstalnya untuk orang lain. Di setiap jalur yang memuat plugins, Claude Code melaporkan plugin sebagai tidak diinstal sampai setiap pengguna [menginstalnya sendiri](/docs/id/plugins/org#require-plugins-per-repository).

<h3 id="extraknownmarketplaces">
  `extraKnownMarketplaces`
</h3>

Daftarkan marketplaces plugin tambahan berdasarkan nama, sehingga orang yang membuka repositori, atau semua orang yang managed settings jangkau, mendapatkan marketplace tanpa menambahkannya sendiri. Claude Code mendaftarkan setiap marketplace yang belum diketahuinya. Apakah plugin yang [`enabledPlugins`](#enabledplugins) namai darinya menginstal tergantung pada sumber plugin dan file mana yang mengaktifkannya; entri itu memiliki aturannya.

* **Scope**: [`Any file`](#scopes). Claude Code menghormati entri dalam `.claude/settings.json` atau `.claude/settings.local.json` repositori hanya setelah Anda menerima dialog kepercayaan workspace untuk folder itu; dalam folder yang belum Anda percayai, termasuk run `-p` di sana, itu mengabaikannya tanpa pesan.
* **Type**: object mapping nama marketplace ke objek dengan objek `source` dan Boolean `autoUpdate` opsional
* **Default**: unset

Contoh ini mendaftarkan marketplace GitHub dan marketplace dari URL git yang dihosting sendiri:

```json settings.json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/claude-plugins"
      }
    },
    "security-plugins": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/security/plugins.git"
      }
    }
  }
}
```

[Apa yang berjalan sebelum Anda mempercayai folder](/docs/id/permissions#what-runs-before-you-trust-a-folder) membandingkan gerbang kepercayaan dengan konten lain yang dapat disediakan repositori. Anda juga dapat menulis kunci ini sebagai `additionalMarketplaces`; lihat [Alias kunci Marketplace](#marketplace-key-aliases).

Atur `"autoUpdate": true` bersama `source` untuk membuat Claude Code menyegarkan marketplace itu dan memperbarui plugins yang diinstal dalam latar belakang setelah startup. Ketika dihilangkan, `claude-plugins-official` dan sebagian besar marketplace resmi Anthropic lainnya default ke `true`, dan marketplace pihak ketiga default ke `false`. Lihat [Konfigurasi auto-updates](/docs/id/plugins/install#keep-plugins-updated).

Ketika lebih dari satu file pengaturan mendefinisikan entri marketplace di bawah nama yang sama, Claude Code menggunakan entri dari file [prioritas tertinggi](/docs/id/settings#settings-precedence) secara keseluruhan. Entri itu menggantikan entri prioritas lebih rendah dan tidak mewarisi bidang apa pun, jadi redefinisi tidak dapat menggabungkan `source.headers` kredensial satu file dengan URL yang dikontrol file lain. Sebelum v2.1.228, Claude Code menggabungkan entri nama-sama bidang demi bidang, jadi entri dalam file prioritas lebih tinggi dapat mewarisi bidang yang tidak diaturnya, termasuk `headers` file lain.

<h4 id="marketplace-source-types">
  Marketplace source types
</h4>

Objek `source` mengambil salah satu bentuk ini:

* **`github`**: repositori GitHub, dengan `repo`
* **`git`**: URL git apa pun, dengan `url`
* **`url`**: URL langsung ke file `marketplace.json`, dengan `url` dan `headers` opsional dan `headersHelper` untuk akses yang diautentikasi. `headersHelper` menamai perintah yang mencetak header yang nilainya terlalu berumur pendek untuk dicantumkan dalam `headers`, dan memerlukan Claude Code v2.1.238 atau lebih baru
* **`file`**: jalur lokal ke file `marketplace.json`, dengan `path`
* **`directory`**: jalur filesystem lokal, dengan `path`, hanya untuk pengembangan
* **`settings`**: marketplace inline yang dideklarasikan langsung dalam file pengaturan tanpa repositori yang dihosting, dengan `name` dan `plugins`

Tipe sumber `git` bekerja dengan layanan hosting git apa pun, termasuk GitLab dan Bitbucket yang dihosting sendiri. Claude Code mengkloning repositori dengan autentikasi yang sama yang `git clone` gunakan di mesin itu: pembantu kredensial yang dikonfigurasi atau kunci SSH. Token penyedia seperti `GITHUB_TOKEN` hanya berlaku melalui pembantu kredensial yang membacanya. Lihat [Private repositories](/docs/id/plugins/host-marketplace#grant-access-to-a-private-marketplace) untuk detail setup.

Untuk sumber `github` dan `git`, Claude Code tidak pernah mengunduh konten [Git LFS](https://git-lfs.com) ketika mengkloning repositori marketplace untuk menambahkan atau memperbaruinya. File yang dilacak LFS diperiksa sebagai file pointer, dan output penambahan atau pembaruan melaporkan berapa banyak.

Bidang `skipLfs` di dalam objek `source` diterima dan tidak memiliki efek. Sebelum v2.1.274, Claude Code mengunduh konten LFS kecuali Anda mengatur `"skipLfs": true`.

Untuk sumber `url`, atur `headersHelper` di dalam objek `source` ketika kredensial dalam `headers` kedaluwarsa dan perintah harus menghasilkan yang baru. Memerlukan Claude Code v2.1.238 atau lebih baru. Untuk apa yang harus dicetak perintah dan di mana Claude Code menjalankannya, lihat [Write the headersHelper command](/docs/id/plugins/host-marketplace#write-the-headershelper-command), dan untuk kasus di mana Claude Code tidak menjalankannya, lihat [When Claude Code skips a headersHelper command](/docs/id/plugins/host-marketplace#when-claude-code-skips-a-headershelper-command-or-drops-its-output). Setelah Anda mengatur `headersHelper` pada URL marketplace `https://`, Claude Code menjalankan perintah di dua titik, menggunakan kembali output satu run selama hingga 60 detik:

* Sebelum setiap pengambilan `marketplace.json` marketplace itu, termasuk penyegaran nanti. Claude Code mengirim header yang dicetak dengan pengambilan itu.
* Sebelum setiap unduhan arsip plugin pada asal URL marketplace, berarti skema, host, dan port yang sama. Claude Code mengirim output dengan unduhan itu, dan tidak ada unduhan lain yang mendapat header.

Claude Code mengabaikan `headersHelper` apa pun yang diatur dalam `.claude/settings.json` atau `.claude/settings.local.json` direktori yang Anda tambahkan dengan [`--add-dir`](/docs/id/permissions#what-runs-before-you-trust-a-folder), pada sumber `url` dan pada entri plugin inline sama-sama, dan hanya mengirim `headers` tetap yang diatur dalam file itu. [How users accept a headersHelper command](/docs/id/plugins/host-marketplace#how-users-accept-a-headershelper-command) mencakup file pengaturan lainnya.

Plugins yang tercantum dalam sumber `settings` harus mereferensikan sumber eksternal seperti GitHub atau npm, dan `name` harus cocok dengan kunci marketplace. Anda masih mengaktifkan setiap plugin secara terpisah dalam `enabledPlugins`. Contoh ini mendeklarasikan satu plugin inline:

```json settings.json theme={null}
{
  "extraKnownMarketplaces": {
    "team-tools": {
      "source": {
        "source": "settings",
        "name": "team-tools",
        "plugins": [
          {
            "name": "code-formatter",
            "source": {
              "source": "github",
              "repo": "acme-corp/code-formatter"
            }
          }
        ]
      }
    }
  }
}
```

Entri plugin di bawah `source: 'settings'` yang `source`-nya sendiri adalah [`archive`](/docs/id/plugins/marketplace-reference#archive-plugin-source) dapat mengatur `headers` untuk unduhan arsip. Jika nilai yang akan Anda masukkan dalam `headers` berumur pendek, seperti token yang registry Anda cetak atas permintaan, atur perintah `headersHelper` sebagai gantinya. Entri dapat mengatur keduanya. Kedua bidang memerlukan Claude Code v2.1.238 atau lebih baru.

Claude Code mengirim `headers` entri, dan apa pun yang dicetak perintah, dengan unduhan arsip plugin itu dan dengan tidak ada unduhan lain. Claude Code menjalankan perintah hanya ketika pengguna [menginstal atau memperbarui plugin itu saja](/docs/id/plugins/host-marketplace#how-users-accept-a-headershelper-command). Tiga aturan lebih lanjut tergantung pada file mana yang memegang entri:

* **`strict`**: tidak seperti entri dalam `marketplace.json` marketplace, entri dalam pengaturan tidak memerlukan `"strict": false`, karena file pengaturan tidak membawa bidang manifest untuk inline. Lihat [Strict mode](/docs/id/plugins/marketplace-reference#strict-mode).
* **Folder trust**: untuk entri dalam `.claude/settings.json` atau `.claude/settings.local.json` proyek, Claude Code menjalankan perintah hanya setelah pengguna juga [mempercayai folder itu](/docs/id/permissions#what-runs-before-you-trust-a-folder).
* **Header filter**: Claude Code menjatuhkan [nama header perutean permintaan dan identitas klien](/docs/id/plugins/host-marketplace#when-claude-code-skips-a-headershelper-command-or-drops-its-output) dari entri dalam `.claude/settings.json` atau `.claude/settings.local.json` proyek, karena repositori dapat menyediakan file itu. Claude Code menerapkan filter yang sama ke entri katalog dan ke entri dalam pengaturan direktori `--add-dir`, dan tidak ada filter ke entri dalam pengaturan pengguna Anda, file `--settings`, atau managed settings.

<h4 id="marketplace-key-aliases">
  Marketplace key aliases
</h4>

Pada Claude Code v2.1.232 atau lebih baru, Anda dapat menulis `extraKnownMarketplaces` sebagai `additionalMarketplaces` dan `strictKnownMarketplaces` sebagai `allowedMarketplaces`. Claude Code memperlakukan setiap alias sebagai berikut:

* Versi sebelumnya mengabaikan alias, jadi pertahankan ejaan kanonik dalam file yang juga dibaca versi lebih lama, seperti file managed settings untuk armada dengan versi Claude Code campuran.
* Dalam file pengaturan apa pun yang menerima kunci kanonik, Claude Code membaca alias persis seperti membaca kunci kanonik.
* Claude Code dapat menulis ulang `additionalMarketplaces` ke `extraKnownMarketplaces` ketika memperbarui file.
* Jika Anda mengatur kedua ejaan dalam satu file, Claude Code menggunakan nilai kanonik dan mengabaikan alias.

<h3 id="pluginconfigs">
  `pluginConfigs`
</h3>

Simpan jawaban non-sensitif yang Anda berikan dialog konfigurasi [`userConfig`](/docs/id/plugins/manifest-reference#user-configuration) plugin, dikunci dengan ID plugin. Claude Code menulis kunci ini ke pengaturan pengguna Anda ketika Anda mengisi dialog, jadi Anda tidak perlu mengeditnya dengan tangan. Claude Code menyimpan opsi sensitif dalam Keychain macOS sebagai gantinya, kembali ke `~/.claude/.credentials.json` ketika Keychain menolak penulisan; pada platform tanpa keychain yang didukung, itu menyimpannya dalam `~/.claude/.credentials.json`.

* **Scope**: [`User or managed`](#scopes)
* **Type**: object mapping ID plugin ke objek dengan bidang `options`, mapping setiap nama opsi ke string, angka, Boolean, atau array string, dan bidang `mcpServers` opsional yang memegang nilai konfigurasi pengguna per-server dalam bentuk yang sama
* **Default**: unset

Contoh ini menyimpan opsi `api_endpoint` untuk plugin `deployer` dari `acme-tools`:

```json settings.json theme={null}
{
  "pluginConfigs": {
    "deployer@acme-tools": {
      "options": {
        "api_endpoint": "https://api.example.com"
      }
    }
  }
}
```

Built-in plugins menyimpan opsi mereka di bawah kunci yang sama dengan sufiks `@builtin`. Misalnya, pengaturan [**Project instructions**](/docs/id/memory#choose-which-instruction-files-load) yang mengontrol apakah Claude Code membaca file `AGENTS.md` adalah `pluginConfigs["agents-md@builtin"].options.instructionFiles`.

Claude Code mengabaikan entri proyek dan lokal karena menggantikan nilai ini ke dalam konfigurasi hook plugin, MCP, dan LSP, dan repositori yang dikloning tidak boleh dapat menyediakannya. Sebelum v2.1.207, pengaturan proyek dan lokal juga dibaca.

<h2 id="mcp">
  MCP
</h2>

Kontrol server MCP mana yang Claude Code terhubung dan mana yang diizinkan organisasi. Lihat [Terhubung ke alat eksternal dengan MCP](/docs/id/mcp) dan [Konfigurasi MCP yang dikelola](/docs/id/managed-mcp).

<h3 id="allowallclaudeaimcps">
  `allowAllClaudeAiMcps`
</h3>

Muat [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) yang Claude Code ambil sendiri bersama dengan `managed-mcp.json` yang diterapkan. Tanpa kunci ini, `managed-mcp.json` mengambil kontrol eksklusif atas server MCP dan menekan konektor tersebut.

* **Scope**: [`Managed`](#scopes). Pengguna tidak dapat mengaktifkan kembali konektor yang kontrol eksklusif tekan.
* **Type**: Boolean
  * `true`: Claude Code memuat konektor claude.ai bersama dengan `managed-mcp.json` yang diterapkan
  * `false`: `managed-mcp.json` yang diterapkan mengambil kontrol eksklusif atas server MCP dan menekan konektor claude.ai [yang Claude Code ambil sendiri](/docs/id/mcp#how-connectors-reach-claude-code)
* **Default**: `false`, jadi `managed-mcp.json` yang diterapkan menekan konektor claude.ai yang Claude Code ambil sendiri

```json managed-settings.json theme={null}
{
  "allowAllClaudeAiMcps": true
}
```

[`allowedMcpServers`](#allowedmcpservers) dan [`deniedMcpServers`](#deniedmcpservers) masih berlaku untuk konektor yang kunci ini muat. Konektor yang dikirimkan ke [sesi cloud](/docs/id/claude-code-on-the-web) yang hostnya membawa `managed-mcp.json`, seperti runner yang di-host sendiri, tetap ditekan. Lihat [Izinkan konektor claude.ai bersama dengan set yang dikelola](/docs/id/managed-mcp#allow-claude-ai-connectors-alongside-the-managed-set).

<h3 id="allowedmcpservers">
  `allowedMcpServers`
</h3>

Daftar putih server MCP yang dapat ditambahkan orang. Claude Code memblokir server apa pun yang tidak cocok dengan entri di mana pun didefinisikan, termasuk server plugin, server yang dilewatkan dengan `--mcp-config`, dan server dari claude.ai.

Server bawaan seperti Claude di Chrome, server `ide` yang Claude Code terhubung dalam [VS Code](/docs/id/vs-code#the-built-in-ide-mcp-server) atau [JetBrains](/docs/id/jetbrains#the-built-in-ide-mcp-server) IDE yang sedang berjalan, dan server yang CLI sendiri konfigurasi dikecualikan dari daftar putih, dan daftar hitam masih berlaku untuk mereka. Server `type: "sdk"` dalam proses dikecualikan dari kedua daftar; [aplikasi yang memulai sesi](/docs/id/mcp#how-connectors-reach-claude-code) mendaftarkan mereka.

Server yang organisasi Anda berikan juga dikecualikan dari daftar putih, dan daftar hitam masih berlaku untuk mereka. Pengecualian mencakup setiap entri [`managedMcpServers`](#managedmcpservers), dan entri [`managed-mcp.json`](/docs/id/managed-mcp#exclusive-control-with-managed-mcp-json) apa pun yang nilainya tidak menggunakan ekspansi `${VAR}`. Lihat [Bagaimana server dievaluasi](/docs/id/managed-mcp#how-a-server-is-evaluated) untuk urutan pemeriksaan lengkap. Sebelum v2.1.259, server dari `managed-mcp.json` juga harus cocok.

* **Scope**: [`Any file`](#scopes). Entri dari setiap file bergabung menjadi satu daftar putih kecuali [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly) diatur. Terapkan di pengaturan yang dikelola untuk memberlakukannya.
* **Type**: array objek, masing-masing dengan tepat satu kunci: `serverName`, string terbatas pada huruf, angka, tanda hubung, dan garis bawah; `serverCommand`, array perintah dan argumennya yang cocok persis; atau `serverUrl`, pola URL dengan wildcard `*`
* **Default**: tidak diatur, jadi setiap server diizinkan; array kosong memblokir setiap server yang ditambahkan pengguna

Contoh ini hanya mengizinkan server stdio yang perintah `npx` yang terdaftar mulai:

```json settings.json theme={null}
{
  "allowedMcpServers": [
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem"] }
  ]
}
```

Entri [`deniedMcpServers`](#deniedmcpservers) memiliki prioritas, jadi server di kedua daftar diblokir. Setelah daftar berisi entri `serverCommand` apa pun, server stdio harus cocok dengan entri `serverCommand`, dan setelah berisi entri `serverUrl` apa pun, server jarak jauh harus cocok dengan entri `serverUrl`: kecocokan `serverName` tidak lagi mengakui jenis server itu. Lihat [Kontrol berbasis kebijakan dengan daftar putih dan daftar hitam](/docs/id/managed-mcp#policy-based-control-with-allowlists-and-denylists).

<h3 id="allowmanagedmcpserversonly">
  `allowManagedMcpServersOnly`
</h3>

Buat daftar putih yang dikelola satu-satunya yang berlaku. Claude Code kemudian membaca [`allowedMcpServers`](#allowedmcpservers) dari pengaturan yang dikelola saja dan mengabaikan daftar putih di pengaturan pengguna, proyek, dan lokal; [`deniedMcpServers`](#deniedmcpservers) masih bergabung dari setiap cakupan pengaturan, jadi pengguna masih dapat memblokir server untuk diri mereka sendiri. Administrator menetapkannya sehingga pengaturan pengguna mereka sendiri tidak dapat memperluas apa yang daftar putih yang dikelola izinkan.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code membaca `allowedMcpServers` dari pengaturan yang dikelola saja dan mengabaikan daftar putih di pengaturan pengguna, proyek, dan lokal
  * `false`: daftar putih dari setiap cakupan pengaturan bergabung
* **Default**: `false`, jadi daftar putih dari setiap cakupan pengaturan bergabung

Contoh ini mengunci daftar putih ke pengaturan yang dikelola dan hanya mengizinkan server bernama `github`:

```json managed-settings.json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverName": "github" }
  ]
}
```

Pengguna masih dapat menambahkan server MCP mereka sendiri; hanya server yang cocok dengan daftar putih yang dikelola yang dimuat. Lihat [Batasi daftar putih ke pengaturan yang dikelola saja](/docs/id/managed-mcp#restrict-the-allowlist-to-managed-settings-only).

<h3 id="deniedmcpservers">
  `deniedMcpServers`
</h3>

Blokir server MCP tertentu. Claude Code menolak untuk memuat server yang cocok di mana pun didefinisikan, termasuk server plugin, server yang dilewatkan dengan `--mcp-config`, server dari `managed-mcp.json`, server dari [`managedMcpServers`](#managedmcpservers), dan konektor claude.ai [yang diambilnya sendiri](/docs/id/mcp#how-connectors-reach-claude-code). Server `type: "sdk"` dalam proses dikecualikan; aplikasi yang memulai sesi mendaftarkan mereka.

* **Scope**: [`Any file`](#scopes). Entri dari setiap file bergabung menjadi satu daftar hitam, dan [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly) tidak mengubah itu. Terapkan di pengaturan yang dikelola untuk memberlakukannya.
* **Type**: array objek, masing-masing dengan tepat satu kunci: `serverName`, string, jadi nama tampilan konektor claude.ai seperti `"claude.ai Slack"` berfungsi; `serverCommand`, array perintah dan argumennya yang cocok persis; atau `serverUrl`, pola URL dengan wildcard `*`
* **Default**: tidak diatur, jadi tidak ada server yang diblokir; array kosong juga tidak memblokir apa pun

```json settings.json theme={null}
{
  "deniedMcpServers": [
    { "serverName": "filesystem" }
  ]
}
```

Daftar hitam memiliki prioritas atas [`allowedMcpServers`](#allowedmcpservers), jadi server di kedua daftar diblokir. Lihat [Kontrol berbasis kebijakan dengan daftar putih dan daftar hitam](/docs/id/managed-mcp#policy-based-control-with-allowlists-and-denylists).

<h3 id="disableclaudeaiconnectors">
  `disableClaudeAiConnectors`
</h3>

Matikan [konektor MCP claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) [yang Claude Code ambil sendiri](/docs/id/mcp#how-connectors-reach-claude-code), sehingga tidak mengambil atau menghubungkan mereka. `true` di file pengaturan apa pun berlaku: file `.claude/settings.json` proyek yang diperiksa dapat menolak repositori dari konektor tersebut, tetapi `false` tingkat proyek tidak dapat mengganti `true` tingkat pengguna atau yang dikelola.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code tidak mengambil atau menghubungkan konektor tersebut
  * `false`: sama dengan tidak diatur; Claude Code mengambil konektor Anda kecuali file pengaturan lain atau `ENABLE_CLAUDEAI_MCP_SERVERS` mematikannya
* **Default**: `false`, jadi Claude Code mengambil konektor Anda
* **Per-session overrides**: [`ENABLE_CLAUDEAI_MCP_SERVERS`](/docs/id/env-vars) diatur ke `false` mematikan konektor untuk satu sesi; mana pun dari keduanya yang mematikannya, yang lain tidak dapat menghidupkannya kembali

```json settings.json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Server yang Anda lewatkan secara eksplisit dengan `--mcp-config` tidak terpengaruh. Untuk memblokir konektor individual alih-alih semuanya, gunakan [`deniedMcpServers`](#deniedmcpservers). Lihat [Nonaktifkan konektor claude.ai](/docs/id/mcp#disable-claude-ai-connectors).

<h3 id="disabledmcpjsonservers">
  `disabledMcpjsonServers`
</h3>

Tolak server tertentu yang didefinisikan dalam file `.mcp.json` proyek sehingga Claude Code tidak pernah menghubungkannya atau meminta Anda menyetujuinya. Penolakan di file pengaturan apa pun berlaku, termasuk `.claude/settings.json` proyek yang diperiksa ke dalam repositori.

* **Scope**: [`Any file`](#scopes)
* **Type**: array string, nama server seperti yang muncul di `.mcp.json`
* **Default**: tidak diatur

```json settings.json theme={null}
{
  "disabledMcpjsonServers": ["filesystem"]
}
```

Claude Code menulis kunci ini ke `.claude/settings.local.json` ketika Anda menolak server di dialog persetujuan. `claude mcp get <name>` menampilkan server yang ditolak sebagai `✘ Rejected (see disabledMcpjsonServers in settings)`. Penolakan memiliki prioritas atas [`enabledMcpjsonServers`](#enabledmcpjsonservers) dan [`enableAllProjectMcpServers`](#enableallprojectmcpservers).

<h3 id="enableallprojectmcpservers">
  `enableAllProjectMcpServers`
</h3>

Setujui setiap server MCP yang didefinisikan dalam file `.mcp.json` proyek tanpa prompt. Claude Code menulis kunci ini ke `.claude/settings.local.json` ketika Anda memilih untuk menyetujui semua server di dialog persetujuan.

* **Scope**: [`Any file`](#scopes). Di folder yang dialog kepercayaan Anda belum terima, Claude Code menghormatinya dari pengaturan pengguna, pengaturan yang dikelola, dan `--settings` dan mengabaikannya di file proyek bersama, baik dalam sesi maupun untuk `claude mcp list` dan `claude mcp get`; [Persetujuan server proyek dan kepercayaan ruang kerja](/docs/id/mcp#project-server-approvals-and-workspace-trust) mengatakan kapan `.claude/settings.local.json` yang tidak dilacak dihitung juga.
* **Type**: Boolean
  * `true`: Claude Code menyetujui setiap server MCP yang didefinisikan dalam file `.mcp.json` proyek tanpa prompt
  * `false`: Claude Code meminta Anda menyetujui setiap server. Di folder yang dipercaya, `false` di file prioritas lebih tinggi mengganti `true` di file prioritas lebih rendah; di folder yang belum Anda percayai, `true` di file yang dihormati apa pun sudah cukup
* **Default**: tidak diatur, jadi Claude Code meminta Anda menyetujui setiap server

```json settings.json theme={null}
{
  "enableAllProjectMcpServers": true
}
```

Entri [`disabledMcpjsonServers`](#disabledmcpjsonservers) masih menolak server.

<h3 id="enabledmcpjsonservers">
  `enabledMcpjsonServers`
</h3>

Setujui server tertentu yang didefinisikan dalam file `.mcp.json` proyek sehingga Claude Code menghubungkannya tanpa bertanya. Claude Code menulis kunci ini ke `.claude/settings.local.json` ketika Anda menyetujui server di dialog persetujuan.

* **Scope**: [`Any file`](#scopes). Di folder yang dialog kepercayaan Anda belum terima, Claude Code menghormatinya dari pengaturan pengguna, pengaturan yang dikelola, dan `--settings` dan mengabaikannya di file proyek bersama, baik dalam sesi maupun untuk `claude mcp list` dan `claude mcp get`; [Persetujuan server proyek dan kepercayaan ruang kerja](/docs/id/mcp#project-server-approvals-and-workspace-trust) mengatakan kapan `.claude/settings.local.json` yang tidak dilacak dihitung juga.
* **Type**: array string, nama server seperti yang muncul di `.mcp.json`
* **Default**: tidak diatur

Contoh ini menyetujui server `memory` dan `github` dari `.mcp.json` proyek:

```json settings.json theme={null}
{
  "enabledMcpjsonServers": ["memory", "github"]
}
```

Entri [`disabledMcpjsonServers`](#disabledmcpjsonservers) masih menolak server.

<h3 id="managedmcpservers">
  `managedMcpServers`
</h3>

Sediakan server MCP jarak jauh untuk setiap pengguna dari pengaturan yang dikelola. Pengguna menyimpan server yang mereka tambahkan sendiri dan tidak dapat mengedit atau menghapus yang Anda sediakan. Memerlukan Claude Code v2.1.259 atau lebih baru.

* **Scope**: [`Managed`](#scopes). Claude Code menghapus kunci dengan peringatan di pengaturan pengguna, proyek, dan lokal, dan tidak membacanya di tab Code aplikasi Claude Desktop pada penerapan pihak ketiga atau di sesi Cowork aplikasi, di mana Claude Desktop menyediakan dan mengunci server MCP sesi tersebut sendiri.
* **Type**: objek yang dikunci berdasarkan nama server. Setiap entri memiliki bentuk `.mcp.json` untuk server `http` atau `sse`: `url` `https://` yang diperlukan, dan secara opsional `headers`, `oauth`, dan opsi HTTP dan SSE lainnya. Claude Code menghapus entri yang gagal validasi, dan [Apa yang dapat berisi entri](/docs/id/managed-mcp#what-an-entry-can-contain) mencantumkan kondisinya
* **Default**: tidak diatur, jadi pengaturan yang dikelola tidak menyediakan server

Contoh ini menyediakan satu server HTTP bernama `search`:

```json managed-settings.json theme={null}
{
  "managedMcpServers": {
    "search": {
      "type": "http",
      "url": "https://search.example.com/mcp"
    }
  }
}
```

Untuk prioritas, bagaimana server yang disediakan bergabung dengan `managed-mcp.json` dan daftar izin dan tolak, dan apa yang dilihat pengguna, lihat [Sediakan server melalui pengaturan yang dikelola](/docs/id/managed-mcp#provide-servers-through-managed-settings).

<h2 id="agents-sessions-and-worktrees">
  Agents, sessions, dan worktrees
</h2>

Atur agent default, kontrol rekan kerja dan pesan lintas-sesi, dan konfigurasi worktrees. Lihat [Subagents](/docs/id/sub-agents) dan [Worktrees](/docs/id/worktrees).

<h3 id="agent">
  `agent`
</h3>

Jalankan thread utama sebagai [subagent](/docs/id/sub-agents#invoke-subagents-explicitly) bernama, sehingga Claude Code menerapkan system prompt, pembatasan tool, dan model subagent tersebut ke sesi Anda. Kunci yang sama menetapkan agent default untuk sesi yang Anda dispatch dari `claude agents`.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, nama agent bawaan atau kustom
* **Default**: unset, sehingga thread utama berjalan sebagai agent default Claude Code
* **Per-session overrides**: `--agent` mengambil alih kunci ini untuk satu sesi

```json settings.json theme={null}
{
  "agent": "code-reviewer"
}
```

`settings.json` plugin sendiri juga dapat menyediakan kunci ini; lihat [Ship default settings with your plugin](/docs/id/plugins/components#default-settings).

<h3 id="crosssessioninbound">
  `crossSessionInbound`
</h3>

Pilih apa yang dilakukan sesi ini dengan [pesan yang tiba dari sesi Claude Code lain Anda](/docs/id/cross-session-messaging#control-inbound-messages). Ketika tidak ada nilai yang berlaku, Claude Code memutuskan per pesan dari kelas permission-mode kedua sesi. Memerlukan Claude Code v2.1.224 atau lebih baru.

* **Scope**: [`Any file`](#scopes). Nilai proyek atau lokal hanya berlaku ketika lebih ketat daripada nilai managed settings, flag `--settings`, atau user settings yang diberikan.
* **Type**: string, salah satu dari:
  * `"accept"`: Claude Code mengirimkan pesan ke Claude
  * `"hold"`: Claude Code menampilkan pemberitahuan untuk pesan tanpa mengirimkannya
  * `"refuse"`: Claude Code menghapus pesan
* **Default**: unset, sehingga Claude Code memutuskan per pesan

```json settings.json theme={null}
{
  "crossSessionInbound": "hold"
}
```

Claude Code membaca managed settings terlebih dahulu, kemudian flag `--settings`, kemudian user settings, dan menerapkan nilai pertama yang ditemukan. `refuse` lebih ketat daripada `hold`, dan `hold` lebih ketat daripada `accept`. Ketika tidak ada sumber terpercaya yang menetapkan nilai, proyek atau `hold` atau `refuse` lokal masih berlaku, menggantikan default per-pesan. Dalam sesi dengan cross-session messaging, kunci ini muncul di `/config` sebagai **Messages from your other sessions**, yang menulisnya ke user settings; baris memerlukan Claude Code v2.1.232 atau lebih baru, dan Claude Code menyembunyikannya saat flag `--settings` atau managed settings menetapkan kunci.

Claude Code [warns](/docs/id/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse) ketika Anda menetapkan nilai yang tidak dikenalinya. Sementara nilai itu ada di file user, project, local, atau `--settings`, Claude Code menahan pesan inbound, bahkan ketika sumber yang mengambil alih menetapkan `accept`. `refuse` yang ditetapkan sumber lain masih berlaku. Perbaiki atau hapus nilai untuk menghapus hold.

Ketika nilai yang tidak dikenali ada di [managed settings](/docs/id/managed-settings), Claude Code malah memperlakukannya sebagai `refuse` sampai administrator memperbaikinya. Sebelum v2.1.248, Claude Code mengabaikan nilai yang tidak dikenali tanpa peringatan.

<h3 id="disableagentview">
  `disableAgentView`
</h3>

Matikan [background agents dan agent view](/docs/id/agent-view): `claude agents`, `--bg`, `/background`, dan supervisor on-demand. Atur di [managed settings](/docs/id/managed-settings) untuk memberlakukannya untuk organisasi.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code mematikan `claude agents`, `--bg`, `/background`, dan supervisor on-demand
  * `false`: agent view tersedia
* **Default**: unset, sehingga agent view tersedia
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_AGENT_VIEW`](/docs/id/env-vars) mematikan agent view untuk satu sesi; mana pun dari keduanya yang mematikannya, yang lain tidak dapat menghidupkannya kembali

```json settings.json theme={null}
{
  "disableAgentView": true
}
```

<h3 id="isolatepeermachines">
  `isolatePeerMachines`
</h3>

Memerlukan persetujuan eksplisit Anda sebelum `SendMessage` Claude mencapai salah satu sesi Anda di luar mesin ini; lihat [Require approval for cross-machine messages](/docs/id/cross-session-messaging#require-approval-for-cross-machine-messages). Prompt persetujuan muncul bahkan dalam [`bypassPermissions` mode](/docs/id/permission-modes#skip-all-checks-with-bypasspermissions-mode).

* **Scope**: [`Any file`](#scopes). `true` dari scope apa pun berlaku, sehingga file proyek yang diperiksa dapat menghidupkan persyaratan tetapi tidak mematikannya.
* **Type**: Boolean
  * `true`: Claude Code meminta persetujuan Anda sebelum `SendMessage` Claude mencapai salah satu sesi Anda di luar mesin ini
  * `false`: pesan lintas-mesin tidak meminta
* **Default**: unset, sehingga pesan lintas-mesin tidak meminta

```json settings.json theme={null}
{
  "isolatePeerMachines": true
}
```

Persetujuan `SendMessage` lintas-mesin memerlukan Claude Code v2.1.224 atau lebih baru.

<h3 id="processwrapper">
  `processWrapper`
</h3>

Di macOS dan Linux, tempatkan perintah launcher korporat di depan [background processes yang dimulai Claude Code](/docs/id/corporate-launcher#what-the-launcher-covers). Claude Code menjalankan launcher dengan baris perintahnya sendiri ditambahkan, sehingga launcher harus exec ke Claude Code; lihat [Run Claude Code behind a corporate launcher](/docs/id/corporate-launcher) untuk kontrak launcher. Memerlukan Claude Code v2.1.210 atau lebih baru.

* **Scope**: [`User or managed`](#scopes)
* **Type**: string, perintah launcher sebagai awalan argv, seperti jalur absolut dengan argumen opsional
* **Default**: unset, sehingga background processes dimulai tanpa wrapper
* **Per-session overrides**: [`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/id/env-vars) mengambil alih kunci ini untuk satu sesi

```json settings.json theme={null}
{
  "processWrapper": "/opt/corp/launcher --profile claude"
}
```

Claude Code mengabaikan launcher di Windows dan memulai setiap proses tanpa wrapper. Memerlukan Claude Code v2.1.210 atau lebih baru.

<h3 id="teammatemode">
  `teammateMode`
</h3>

Pilih di mana Claude Code menampilkan rekan kerja [agent team](/docs/id/agent-teams): di dalam pane terminal utama Anda, atau di pane terpisah ketika terminal Anda mendukungnya. Lihat [Choose a display mode](/docs/id/agent-teams#choose-a-display-mode).

* **Scope**: [`Any file`](#scopes). Claude Code juga membaca nilai yang ditinggalkan di `~/.claude.json` oleh versi yang lebih lama.
* **Type**: string, salah satu dari:
  * `"in-process"`: rekan kerja berjalan di dalam pane terminal utama Anda
  * `"auto"`: pane terpisah ketika Anda menjalankan di dalam tmux, atau di dalam iTerm2 dengan `it2` di `PATH` Anda atau tmux terinstal; in-process sebaliknya
  * `"tmux"`: pane terpisah menggunakan tmux atau iTerm2, terdeteksi dari terminal Anda
  * `"iterm2"`: pane terpisah native iTerm2 melalui CLI `it2`
* **Default**: `"in-process"`
* **Per-session overrides**: `--teammate-mode` mengambil alih kunci ini untuk satu sesi

```json settings.json theme={null}
{
  "teammateMode": "auto"
}
```

<span id="worktree-settings" />

<h3 id="worktree">
  `worktree`
</h3>

Konfigurasi cara Claude Code membuat dan mengelola [git worktrees](/docs/id/worktrees) untuk `--worktree`, tool `EnterWorktree`, dan subagents terisolasi dan background sessions.

* **Scope**: [`Any file`](#scopes)
* **Type**: object dengan `baseRef`, `symlinkDirectories`, `sparsePaths`, dan `bgIsolation`
* **Default**: unset

Contoh ini membuat cabang worktree baru dari `HEAD` Anda saat ini dan membuat symlink `node_modules` ke masing-masing:

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head",
    "symlinkDirectories": ["node_modules"]
  }
}
```

Untuk menyalin file yang diabaikan gitignore seperti `.env` ke worktree baru, tambahkan file [`.worktreeinclude`](/docs/id/worktrees#copy-gitignored-files-into-worktrees) ke root proyek Anda sebagai gantinya dari setting.

<h3 id="worktree-baseref">
  `worktree.baseRef`
</h3>

Pilih ref mana yang dibuat worktree baru. `"fresh"` membuat cabang dari `origin/<default-branch>` untuk pohon bersih yang cocok dengan remote; `"head"` membuat cabang dari `HEAD` lokal Anda saat ini, sehingga commit yang belum dipush dan state feature-branch ada di worktree.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, salah satu dari:
  * `"fresh"`: worktree baru membuat cabang dari `origin/<default-branch>`
  * `"head"`: worktree baru membuat cabang dari `HEAD` lokal Anda saat ini, termasuk commit yang belum dipush
* **Default**: `"fresh"`

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

Di dalam linked worktree, `"head"` menyelesaikan ke `HEAD` worktree itu, bukan checkout utama.

<h3 id="worktree-symlinkdirectories">
  `worktree.symlinkDirectories`
</h3>

Buat symlink direktori dari repository utama ke setiap worktree sehingga Anda tidak menduplikasi direktori besar di disk.

* **Scope**: [`Any file`](#scopes)
* **Type**: array of strings, jalur direktori relatif terhadap root repository
* **Default**: unset, sehingga Claude Code tidak membuat symlink direktori apa pun

Contoh ini membuat symlink `node_modules` dan `.cache` dari repository utama ke setiap worktree baru:

```json settings.json theme={null}
{
  "worktree": {
    "symlinkDirectories": ["node_modules", ".cache"]
  }
}
```

<h3 id="worktree-sparsepaths">
  `worktree.sparsePaths`
</h3>

Periksa hanya direktori yang terdaftar di setiap worktree melalui git sparse-checkout. Claude Code menulis hanya direktori tersebut ditambah file tingkat root ke disk, yang lebih cepat di monorepo besar; lihat [Check out only the directories you need](/docs/id/large-codebases#check-out-only-the-directories-you-need).

* **Scope**: [`Any file`](#scopes)
* **Type**: array of strings, jalur direktori relatif terhadap root repository
* **Default**: unset, sehingga setiap worktree memeriksa seluruh pohon

Contoh ini memeriksa hanya `packages/my-app` dan `shared/utils`, ditambah file tingkat root, di setiap worktree:

```json settings.json theme={null}
{
  "worktree": {
    "sparsePaths": ["packages/my-app", "shared/utils"]
  }
}
```

Sementara sparse worktree ada, git mengaktifkan `extensions.worktreeConfig` di `.git/config` bersama repository.

<h3 id="worktree-bgisolation">
  `worktree.bgIsolation`
</h3>

Pilih cara [background sessions](/docs/id/agent-view#how-file-edits-are-isolated) mengisolasi pengeditan file mereka. Dengan `"worktree"`, Claude Code memblokir `Edit` dan `Write` di checkout utama sampai sesi memanggil `EnterWorktree`; dengan `"none"`, background jobs mengedit working copy secara langsung. Atur `"none"` untuk repository di mana git worktrees tidak praktis.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, salah satu dari:
  * `"worktree"`: Claude Code memblokir `Edit` dan `Write` di checkout utama sampai sesi memanggil `EnterWorktree`
  * `"none"`: background jobs mengedit working copy secara langsung
* **Default**: `"worktree"`

```json settings.json theme={null}
{
  "worktree": {
    "bgIsolation": "none"
  }
}
```

Di luar repository git, [`WorktreeCreate` hook](/docs/id/worktrees#non-git-version-control) yang gagal melepaskan blok sehingga sesi dapat mengedit working directory di tempat; pelepasan itu memerlukan Claude Code v2.1.203 atau lebih baru.

<h2 id="remote-desktop-and-notifications">
  Remote, desktop, dan notifikasi
</h2>

Konfigurasi Remote Control, lingkungan cloud, aplikasi desktop, dan notifikasi yang Claude Code kirimkan ketika membutuhkan Anda. Lihat [Remote Control](/docs/id/remote-control).

<h3 id="agentpushnotifenabled">
  `agentPushNotifEnabled`
</h3>

Izinkan Claude mengirimkan notifikasi push ke ponsel Anda ketika Claude memutuskan bahwa notifikasi tersebut layak dikirim, misalnya ketika tugas yang panjang selesai. Claude Code menyinkronkan pilihan ini ke akun Anda, dan push tiba saat [Remote Control](/docs/id/remote-control) terhubung. Muncul di `/config` sebagai **Push when Claude decides**.

* **Scope**: [`Any file`](#scopes). Claude Code juga membaca nilai yang ditinggalkan di `~/.claude.json` oleh versi yang lebih lama.
* **Type**: Boolean
  * `true`: Claude dapat mengirimkan notifikasi push ke ponsel Anda ketika Claude memutuskan bahwa notifikasi tersebut layak dikirim
  * `false`: Claude tidak mengirimkan notifikasi tersebut
* **Default**: `false`

```json settings.json theme={null}
{
  "agentPushNotifEnabled": true
}
```

Lihat [Mobile push notifications](/docs/id/remote-control#mobile-push-notifications).

<h3 id="awaysummaryenabled">
  `awaySummaryEnabled`
</h3>

Tampilkan ringkasan sesi satu baris ketika Anda kembali ke terminal setelah beberapa menit pergi. Atur ke `false`, atau matikan **Session recap** di `/config`, untuk menghentikan ringkasan.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Anda melihat ringkasan sesi satu baris ketika Anda kembali setelah beberapa menit pergi
  * `false`: Claude Code tidak menampilkan ringkasan
* **Default**: unset, jadi ringkasan aktif
* **Per-session overrides**: [`CLAUDE_CODE_ENABLE_AWAY_SUMMARY`](/docs/id/env-vars) mengambil alih kunci ini untuk satu sesi, dalam kedua arah

```json settings.json theme={null}
{
  "awaySummaryEnabled": false
}
```

Claude Code tidak pernah menampilkan ringkasan dalam mode non-interaktif.

<h3 id="disableartifact">
  `disableArtifact`
</h3>

<Warning>
  Deprecated, dan diganti oleh [`enableArtifact`](#enableartifact). Claude Code masih menghormati `disableArtifact: true` sebagai setara dengan `enableArtifact: false`, dan mengabaikan `disableArtifact: false`.
</Warning>

Gunakan [`enableArtifact`](#enableartifact) sebagai gantinya untuk mematikan alat [Artifact](/docs/id/artifacts), yang menerbitkan output sesi sebagai halaman web pribadi di claude.ai. Ketika Anda mematikan baris **Artifacts** di `/config`, Claude Code menulis `enableArtifact` ke pengaturan pengguna Anda dan menghapus kunci ini.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code mematikan alat Artifact untuk setiap sesi yang berlaku file, dan tidak ada file lain yang menghidupkannya kembali. Sebelum v2.1.242, file dengan prioritas lebih tinggi dapat mengganti `true` file yang lebih rendah daripada kunci bertindak sebagai kunci
  * `false`: diabaikan; untuk membiarkan alat tetap aktif, hapus kunci
* **Default**: unset, jadi alat mengikuti [availability](/docs/id/artifacts#availability) akun Anda
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/id/env-vars) diatur ke `1` mematikan alat untuk satu sesi

```json settings.json theme={null}
{
  "disableArtifact": true
}
```

[Disable artifacts](/docs/id/artifacts#disable-artifacts) mencantumkan setiap cara untuk mematikan alat.

<h3 id="disabledeeplinkregistration">
  `disableDeepLinkRegistration`
</h3>

Hentikan Claude Code dari mendaftarkan penanganan protokol `claude-cli://` dengan sistem operasi, yang sebaliknya dilakukan setelah Anda mengirimkan prompt pertama dari sesi interaktif. [Deep links](/docs/id/deep-links) memungkinkan alat eksternal membuka sesi Claude Code dengan prompt yang sudah diisi. Atur ini di lingkungan di mana pendaftaran penanganan protokol dibatasi atau dikelola secara terpisah.

* **Scope**: [`Any file`](#scopes)
* **Type**: string `"disable"`
* **Default**: unset, jadi Claude Code mendaftarkan penanganan

```json settings.json theme={null}
{
  "disableDeepLinkRegistration": "disable"
}
```

<h3 id="disabledesktoplocalsessions">
  `disableDesktopLocalSessions`
</h3>

Matikan sesi Code yang berjalan di perangkat di [aplikasi desktop](/docs/id/desktop#local-sessions-on-managed-devices), untuk deployment di mana pengembang harus bekerja di mesin jarak jauh melalui SSH. Di tab Code, lingkungan **Local** tetap berada di dropdown lingkungan tetapi berwarna abu-abu dan tidak dapat dipilih, dengan tooltip yang mengatakan organisasi Anda mematikannya; di Windows entri WSL berwarna abu-abu dengan cara yang sama, meskipun apakah sesi WSL berjalan di perangkat yang dikelola sama sekali [diatur secara terpisah](/docs/id/admin-setup#wsl-sessions-in-claude-code-desktop). Sesi baru default ke [koneksi SSH](/docs/id/desktop#ssh-sessions) pertama jika satu dikonfigurasi, dan aplikasi menolak untuk memulai atau melanjutkan sesi di perangkat, termasuk koneksi SSH kembali ke mesin yang sama. Sesi SSH ke host lain dan sesi cloud tidak terpengaruh. Aplikasi desktop membaca kunci ini; CLI terminal mengabaikannya. Memerlukan Claude Desktop v1.37937.0 atau lebih baru.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean; hanya Boolean JSON `true` yang berlaku
  * `true`: aplikasi desktop tidak menawarkan sesi Code di perangkat; sesi lokal yang ada tetap terdaftar tetapi tidak dapat dilanjutkan
  * `false`: sesi lokal tetap tersedia
* **Default**: unset, jadi sesi lokal tersedia

```json managed-settings.json theme={null}
{
  "disableDesktopLocalSessions": true
}
```

Aplikasi desktop mengabaikan nilai lainnya, dan nilai yang bukan Boolean, seperti string `"true"` atau `1`, juga mencatat peringatan. Pasangkan dengan [`sshConfigs`](#sshconfigs) sehingga pengguna mendarat di koneksi yang berfungsi, dan dengan [`sshHostAllowlist`](#sshhostallowlist) untuk membatasi host mana yang dapat mereka jangkau. Lihat [Local sessions on managed devices](/docs/id/desktop#local-sessions-on-managed-devices).

Claude Desktop memasok sesi Code dengan kebijakan yang berasal dari konfigurasi desktop Anda, misalnya daftar egress allowlist, sandbox sistem file, dan pembatasan MCP dalam deployment pihak ketiga. Claude Code mengabaikan pengaturan induk tersebut kapan pun [sumber admin](/docs/id/managed-settings#how-claude-code-combines-managed-sources) hadir: pengaturan yang dikelola server, kebijakan MDM atau tingkat OS, atau file pengaturan yang dikelola. Menerapkan kunci ini melalui salah satu dari mereka di perangkat yang tidak memiliki sebelumnya, seperti dalam deployment pihak ketiga, oleh karena itu menghentikan kebijakan yang berasal dari desktop dari penerapan. [Let an embedding host add policy](/docs/id/managed-settings#let-an-embedding-host-add-policy) mencakup kapan pengaturan induk masih dapat digabung; ini berlaku untuk kunci apa pun yang Anda terapkan dengan cara itu, bukan hanya yang ini.

<h3 id="disableremotecontrol">
  `disableRemoteControl`
</h3>

Matikan [Remote Control](/docs/id/remote-control): Claude Code kemudian menolak `claude remote-control`, flag `--remote-control`, auto-start, dan toggle dalam sesi, dan melaporkan bahwa kebijakan organisasi Anda menonaktifkannya. Tempatkan di [managed settings](/docs/id/managed-settings) untuk penegakan MDM per perangkat.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code menolak `claude remote-control`, flag `--remote-control`, auto-start, dan toggle dalam sesi
  * `false`: Remote Control tetap tersedia
* **Default**: `false`

```json settings.json theme={null}
{
  "disableRemoteControl": true
}
```

<h3 id="enableartifact">
  `enableArtifact`
</h3>

Matikan alat [Artifact](/docs/id/artifacts), yang menerbitkan output sesi sebagai halaman web pribadi di claude.ai. Ketika Anda mematikan baris **Artifacts** di `/config`, Claude Code menulis kunci ini ke pengaturan pengguna Anda, jadi Anda biasanya tidak mengeditnya dengan tangan. Memerlukan Claude Code v2.1.196 atau lebih baru.

* **Scope**: [`Any file`](#scopes). Setiap file dapat mematikan alat, dan tidak ada yang dapat menghidupkannya kembali.
* **Type**: Boolean
  * `false`: Claude Code mematikan alat Artifact untuk setiap sesi yang berlaku file
  * `true`: sama dengan membiarkan kunci unset, karena tidak pernah mengganti `false` dari file lain, dari [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/id/env-vars), atau dari [admin setting](/docs/id/artifacts#manage-artifacts-for-your-organization) organisasi Anda
* **Default**: unset, jadi alat mengikuti [availability](/docs/id/artifacts#availability) akun Anda

```json settings.json theme={null}
{
  "enableArtifact": false
}
```

Sementara sumber selain pengaturan pengguna Anda sendiri membuat alat tetap mati, Claude Code menyembunyikan baris **Artifacts** di `/config`, karena menghidupkannya di sana tidak akan mengubah apa pun. [Disable artifacts](/docs/id/artifacts#disable-artifacts) mencantumkan setiap cara untuk mematikan alat. Sebelum v2.1.242, Claude Code mengabaikan kunci ini dalam pengaturan proyek dan lokal, dan file yang lebih tinggi dalam [stack prioritas](/docs/id/settings#settings-precedence) dapat menghidupkan alat kembali atas file yang lebih rendah.

<h3 id="inputneedednotifenabled">
  `inputNeededNotifEnabled`
</h3>

Dapatkan notifikasi push di ponsel Anda ketika prompt izin atau pertanyaan menunggu input Anda. Claude Code mengirimkan ini hanya saat [Remote Control](/docs/id/remote-control) terhubung. Muncul di `/config` sebagai **Push when actions required**.

* **Scope**: [`Any file`](#scopes). Claude Code juga membaca nilai yang ditinggalkan di `~/.claude.json` oleh versi yang lebih lama.
* **Type**: Boolean
  * `true`: Anda mendapatkan notifikasi push di ponsel Anda ketika prompt izin atau pertanyaan menunggu, saat Remote Control terhubung
  * `false`: Claude Code tidak mengirimkan notifikasi seperti itu
* **Default**: `false`

```json settings.json theme={null}
{
  "inputNeededNotifEnabled": true
}
```

Lihat [Mobile push notifications](/docs/id/remote-control#mobile-push-notifications).

<h3 id="preferrednotifchannel">
  `preferredNotifChannel`
</h3>

Pilih bagaimana Claude Code memberi tahu Anda ketika tugas selesai atau prompt izin menunggu. Muncul di `/config` sebagai **Local notifications**.

* **Scope**: [`Any file`](#scopes). Claude Code juga membaca nilai yang ditinggalkan di `~/.claude.json` oleh versi yang lebih lama.
* **Type**: string, salah satu dari:
  * `"auto"`: Claude Code mengirimkan notifikasi desktop di iTerm2, Ghostty, dan Kitty, membunyikan bel di Terminal.app hanya ketika bel audibel dimatikan, dan tidak melakukan apa pun di tempat lain
  * `"terminal_bell"`: Claude Code membunyikan karakter bel di terminal apa pun
  * `"iterm2"`: Claude Code mengirimkan notifikasi desktop iTerm2
  * `"iterm2_with_bell"`: Claude Code mengirimkan notifikasi desktop iTerm2 dan membunyikan bel
  * `"kitty"`: Claude Code mengirimkan notifikasi desktop Kitty
  * `"ghostty"`: Claude Code mengirimkan notifikasi desktop Ghostty
  * `"notifications_disabled"`: Claude Code tidak mengirimkan notifikasi
* **Default**: `"auto"`

```json settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

Dengan `"auto"`, Claude Code mengirimkan notifikasi desktop di iTerm2, Ghostty, dan Kitty. Di Terminal.app, Claude Code membunyikan karakter bel hanya ketika Anda telah mematikan bel audibel Terminal, dan di terminal lain tidak melakukan apa pun. Atur `"terminal_bell"` untuk membunyikan karakter bel di terminal apa pun. Lihat [Get a terminal bell or notification](/docs/id/terminal-config#get-a-terminal-bell-or-notification).

<h3 id="remote-defaultenvironmentid">
  `remote.defaultEnvironmentId`
</h3>

Pilih [lingkungan cloud](/docs/id/cloud-environments) default untuk sesi cloud yang Anda buat dari CLI, seperti dengan `claude --cloud`. Claude Code menulis kunci ini ke pengaturan pengguna Anda ketika Anda memilih lingkungan dengan [`/remote-env`](/docs/id/cloud-environments#select-an-environment-from-the-cli).

* **Scope**: [`Any file`](#scopes). Untuk ID lingkungan yang di-host sendiri, pengaturan pengguna atau terkelola, atau flag `--settings` saja.
* **Type**: string, ID lingkungan seperti `env_...` atau `ccpool_...`
* **Default**: unset, jadi Claude Code menggunakan lingkungan yang di-host Anthropic ketika daftar Anda memiliki satu, dan sebaliknya lingkungan pertama dalam daftar Anda yang bukan [lingkungan jembatan Remote Control](/docs/id/cloud-environments#the-default-environment), atau lingkungan pertama ketika setiap satu adalah lingkungan jembatan
* **Per-session overrides**: `--environment` mengambil alih kunci ini untuk satu sesi cloud yang dibuatnya

```json settings.json theme={null}
{
  "remote": {
    "defaultEnvironmentId": "env_0123abcd"
  }
}
```

ID lingkungan yang di-host Anthropic, yang dimulai dengan `env_`, mengikuti prioritas pengaturan standar, jadi nilai dalam pengaturan proyek repositori mengganti pilihan tingkat pengguna Anda. ID [lingkungan yang di-host sendiri](/docs/id/self-hosted-environments), yang dimulai dengan `ccpool_`, dihormati hanya dari pengaturan pengguna, pengaturan terkelola, dan flag `--settings`; Claude Code mengabaikan satu dalam pengaturan proyek atau lokal repositori, dan `/remote-env` menunjukkan nilai mana yang diabaikannya, jadi file yang diperiksa tidak dapat mengarahkan sesi ke lingkungan yang di-host sendiri yang tidak Anda pilih.

<h3 id="remotecontrolatstartup">
  `remoteControlAtStartup`
</h3>

Hubungkan [Remote Control](/docs/id/remote-control) secara otomatis ketika setiap sesi interaktif dimulai, alih-alih menunggu `/remote-control`. Atur ke `true` untuk menghidupkan auto-connect, `false` untuk mematikannya. Muncul di `/config` sebagai **Enable Remote Control for all sessions**.

* **Scope**: [`Any file`](#scopes). Claude Code juga membaca nilai yang ditinggalkan di `~/.claude.json` oleh versi yang lebih lama.
* **Type**: Boolean
  * `true`: Claude Code menghubungkan Remote Control secara otomatis ketika setiap sesi interaktif dimulai
  * `false`: Claude Code menunggu `/remote-control`
* **Default**: unset, jadi auto-connect mengikuti default admin organisasi Anda ketika satu diatur, dan sebaliknya default Claude Code saat ini
* **Per-session overrides**: `--remote-control` menghidupkan Remote Control untuk satu sesi bahkan ketika kunci ini `false`, dan tidak ada flag yang mematikannya untuk satu sesi

```json settings.json theme={null}
{
  "remoteControlAtStartup": true
}
```

Claude Code mengabaikan `true` dari pengaturan proyek atau lokal, jadi repositori dapat mematikan auto-connect untuk checkout-nya tetapi tidak dapat menghidupkannya. Untuk perilaku per-scope lengkap, lihat [Enable Remote Control for all sessions](/docs/id/remote-control#enable-remote-control-for-all-sessions) dan [kunci keamanan di mana nilai yang lebih ketat berlaku](/docs/id/settings#security-keys-where-the-stricter-value-applies).

<h3 id="sshconfigs">
  `sshConfigs`
</h3>

Tambahkan koneksi SSH ke dropdown lingkungan [Desktop](/docs/id/desktop#pre-configure-ssh-connections-for-your-team). Administrator menggunakannya untuk mendistribusikan koneksi bersama ke tim. Koneksi yang Anda tentukan dalam pengaturan terkelola ditampilkan sebagai terkelola, jadi pengguna dapat memilihnya tetapi tidak dapat mengedit atau menghapusnya di aplikasi.

* **Scope**: [`User or managed`](#scopes). Aplikasi desktop membaca kunci ini.
* **Type**: array objek, masing-masing dengan `id`, `name`, dan `sshHost` yang diperlukan dan `sshPort` dan `sshIdentityFile` opsional
* **Default**: unset

Contoh ini menambahkan satu koneksi bernama `Dev VM` yang terhubung ke `user@dev.example.com`:

```json settings.json theme={null}
{
  "sshConfigs": [
    {
      "id": "dev-vm",
      "name": "Dev VM",
      "sshHost": "user@dev.example.com"
    }
  ]
}
```

<h3 id="sshhostallowlist">
  `sshHostAllowlist`
</h3>

Batasi host yang dapat terhubung oleh [sesi SSH Desktop](/docs/id/desktop#restrict-which-ssh-hosts-users-can-connect-to). Hanya aplikasi Desktop yang membaca kunci ini; CLI tidak. Pola tidak peka huruf besar-kecil: `*` cocok dengan host apa pun, `*.example.com` cocok dengan `example.com` dan setiap subdomain, dan apa pun yang lain adalah kecocokan tepat terhadap nama host setelah resolusi `~/.ssh/config`. Array kosong mematikan sesi SSH.

* **Scope**: [`Managed`](#scopes)
* **Type**: array pola nama host
* **Default**: unset, jadi host apa pun diizinkan

Contoh ini memungkinkan `devboxes.example.com` dan subdomain-nya, ditambah host tepat `bastion.example.com`:

```json managed-settings.json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

<span id="authentication-and-login" />

<h2 id="authentication-and-providers">
  Autentikasi dan penyedia
</h2>

Berikan kredensial melalui skrip pembantu dan, untuk organisasi, paksa metode login atau organisasi. Lihat [Autentikasi](/docs/id/authentication).

<h3 id="apikeyhelper">
  `apiKeyHelper`
</h3>

Jalankan perintah Anda sendiri untuk menghasilkan kredensial yang Claude Code kirimkan dengan permintaan model. Claude Code menjalankan perintah melalui shell sistem, `/bin/sh` di macOS dan Linux serta `cmd` di Windows, dan mengirimkan outputnya sebagai header `X-Api-Key` dan `Authorization: Bearer`. Gunakan untuk kredensial dinamis atau berputar, seperti token berumur pendek yang diambil dari vault.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, baris perintah shell
* **Default**: tidak diatur, jadi Claude Code tidak menjalankan pembantu

```json settings.json theme={null}
{
  "apiKeyHelper": "/bin/generate_temp_api_key.sh"
}
```

Claude Code menyimpan nilai dalam cache dan menjalankan kembali perintah dalam kasus berikut:

* Setelah masa hidup cache, lima menit secara default atau interval yang Anda atur dengan [`CLAUDE_CODE_API_KEY_HELPER_TTL_MS`](/docs/id/env-vars).
* Ketika permintaan ke API Anthropic, secara langsung atau melalui [gateway LLM](/docs/id/llm-gateway), gagal dengan `401` atau `403`.
* Sebelum mengirimkan permintaan ke API Anthropic, secara langsung atau melalui gateway LLM, ketika output yang disimpan dalam cache adalah JWT yang kedaluwarsa setelah pembantu menghasilkannya. Memerlukan Claude Code v2.1.246 atau lebih baru.

Dua kasus terakhir hanya berlaku ketika output pembantu adalah kredensial yang Claude Code kirimkan dan `ANTHROPIC_AUTH_TOKEN` tidak diatur.

Dalam sesi interaktif, ketika perintah berasal dari pengaturan proyek atau lokal, Claude Code tidak menjalankannya sampai Anda menerima prompt kepercayaan workspace. Lihat [Manajemen kredensial](/docs/id/authentication#credential-management).

<h3 id="awsauthrefresh">
  `awsAuthRefresh`
</h3>

Jalankan perintah Anda sendiri, seperti `aws sso login`, untuk menyegarkan kredensial di direktori `.aws` Anda ketika kredensial yang Claude Code miliki untuk [Amazon Bedrock](/docs/id/amazon-bedrock) berhenti bekerja. Claude Code memeriksa kredensial saat ini terhadap STS terlebih dahulu dan hanya menjalankan perintah ketika pemeriksaan itu gagal, kemudian membaca direktori `.aws` yang telah disegarkan.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, baris perintah shell
* **Default**: tidak diatur, jadi Claude Code tidak menyegarkan kredensial AWS untuk Anda

```json settings.json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile"
}
```

Gunakan kunci ini ketika alur penyegaran Anda menulis ke `.aws`; gunakan [`awsCredentialExport`](#awscredentialexport) ketika alur itu mencetak kredensial sebagai gantinya. Lihat [konfigurasi kredensial lanjutan](/docs/id/amazon-bedrock#advanced-credential-configuration).

<h3 id="awscredentialexport">
  `awsCredentialExport`
</h3>

Jalankan perintah Anda sendiri yang mencetak kredensial AWS sebagai JSON, sehingga Claude Code dapat memanggil [Amazon Bedrock](/docs/id/amazon-bedrock) dengan kredensial yang tidak berada di direktori `.aws` Anda. Claude Code menerima bentuk output `aws sts` dan bentuk datar `aws configure export-credentials`, dan membatasi kredensial ke klien Bedrock-nya sendiri, sehingga perintah shell yang Claude Code jalankan masih melihat kredensial ambient Anda.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, baris perintah shell
* **Default**: tidak diatur, jadi Claude Code menggunakan rantai kredensial AWS ambient

```json settings.json theme={null}
{
  "awsCredentialExport": "/bin/generate_aws_grant.sh"
}
```

Tidak seperti [`awsAuthRefresh`](#awsauthrefresh), Claude Code selalu menjalankan perintah ini ketika diatur, tanpa memeriksa kredensial ambient terlebih dahulu. Lihat [konfigurasi kredensial lanjutan](/docs/id/amazon-bedrock#advanced-credential-configuration).

<h3 id="forceloginmethod">
  `forceLoginMethod`
</h3>

Batasi jenis akun apa yang dapat digunakan orang untuk masuk. Atur `"claudeai"` untuk memungkinkan hanya akun claude.ai, `"console"` untuk memungkinkan hanya akun Claude Console, atau `"gateway"` untuk mengirim orang ke [cloud gateway](/docs/id/claude-apps-gateway) alih-alih login pihak pertama. Administrator mengaturnya dalam pengaturan terkelola dan memasangkannya dengan [`forceLoginOrgUUID`](#forceloginorguuid) untuk menjaga login claude.ai pengembang di dalam satu organisasi. Jika Anda mengaturnya ke `"claudeai"` atau `"console"` di file pengaturan apa pun, Claude Code juga berhenti menawarkan [sign-in Console tanpa kunci](/docs/id/authentication#sign-in-without-an-api-key) dalam sesi yang file itu berlaku.

* **Scope**: [`Any file`](#scopes). Claude Code menghormati `"gateway"` hanya dari sumber terkelola di mesin: `managed-settings.json`, plist macOS atau registri Windows HKLM, atau pembantu kebijakan. Ini memperlakukan `"gateway"` sebagai tidak diatur dalam pengaturan pengguna, proyek, lokal, HKCU, dan server-terkelola, aturan yang sama seperti [`forceLoginGatewayUrl`](#forcelogingatewayurl).
* **Type**: string, salah satu dari:
  * `"claudeai"`: hanya akun claude.ai yang dapat masuk
  * `"console"`: hanya akun Claude Console yang dapat masuk
  * `"gateway"`: Claude Code mengirim orang ke cloud gateway alih-alih login pihak pertama
* **Default**: tidak diatur, jadi orang memilih metode login

```json settings.json theme={null}
{
  "forceLoginMethod": "claudeai"
}
```

Setiap jalur login pihak pertama menerapkan pembatasan, termasuk [ekstensi VS Code](/docs/id/vs-code), Agent SDK, `claude setup-token`, dan `/install-github-app`, kecuali layar login interaktif terminal, yang dicapai oleh `/login` atau onboarding first-run, yang pra-memilih metode tanpa memberlakukannya. Sebelum v2.1.212, hanya login terminal yang menerapkannya. Lihat [Batasi login ke organisasi Anda](/docs/id/authentication#restrict-login-to-your-organization) untuk cara setiap jalur login, kredensial lingkungan, dan penyedia pihak ketiga ditangani.

Ketika sumber terkelola di mesin mengatur `"gateway"`, Claude Code tidak menggunakan login sisa, kunci API, atau kredensial `apiKeyHelper`. Lihat [Administrator policy requires a Cloud gateway sign-in](/docs/id/errors#administrator-policy-requires-a-cloud-gateway-sign-in) untuk pesan yang masing-masing menghasilkan. Jika Anda memilih penyedia cloud melalui `CLAUDE_CODE_USE_BEDROCK` atau variabel lingkungan serupa, sesi tidak memerlukan sign-in gateway. Sebelum v2.1.261, Claude Code menggunakan login sisa di mesin ini.

<h3 id="forcelogingatewayurl">
  `forceLoginGatewayUrl`
</h3>

Atur URL gateway yang layar Cloud gateway `/login` terhubung, sehingga orang mencapai [cloud gateway](/docs/id/claude-apps-gateway) Anda tanpa mengetik alamatnya. Layar tidak memiliki bidang URL: dengan kunci ini diatur, layar menampilkan URL gateway Anda dan terhubung ketika orang menekan Enter; tanpanya, layar memberi tahu mereka untuk menghubungi administrator IT mereka.

Baik kunci ini atau `forceLoginMethod: "gateway"` membuat mesin gateway-only, jadi `/login` membuka di layar Cloud gateway tanpa pemilih metode login. Lihat [Administrator policy requires a Cloud gateway sign-in](/docs/id/errors#administrator-policy-requires-a-cloud-gateway-sign-in) untuk apa yang terjadi pada login pihak pertama atau kunci API yang tersisa. Atur kedua kunci sehingga layar terhubung alih-alih menampilkan kesalahan.

* **Scope**: [`Managed`](#scopes). Baca hanya dari sumber di mesin: `managed-settings.json`, plist macOS atau registri Windows HKLM, atau pembantu kebijakan. Claude Code mengabaikannya dalam pengaturan HKCU dan server-terkelola.
* **Type**: string, URL lengkap termasuk skema
* **Default**: tidak diatur, jadi layar Cloud gateway menampilkan kesalahan yang memberi tahu orang untuk menghubungi administrator IT mereka

```json managed-settings.json theme={null}
{
  "forceLoginGatewayUrl": "https://claude-gateway.example.com"
}
```

Jika nilainya bukan URL yang valid, layar sign-in melaporkannya, dan sisa file pengaturan terkelola masih berlaku. Lihat [Atur URL gateway](/docs/id/claude-apps-gateway#set-the-gateway-url).

<h3 id="forceloginorguuid">
  `forceLoginOrgUUID`
</h3>

Dari sumber terkelola, perlukan login akun claude.ai untuk milik satu organisasi Anthropic, diberikan sebagai UUID tunggal, atau milik salah satu dari beberapa organisasi, diberikan sebagai array. Dari file pengaturan apa pun, Claude Code juga menggunakan UUID tunggal untuk pra-memilih organisasi itu selama login claude.ai atau Claude Console, dan pra-memilih tidak ada untuk array. Jika Anda mengatur kunci di file pengaturan apa pun, Claude Code juga berhenti menawarkan [sign-in Console tanpa kunci](/docs/id/authentication#sign-in-without-an-api-key) dalam sesi yang file itu berlaku dan membuat kunci API sebagai gantinya.

* **Scope**: [`Any file`](#scopes). Hanya sumber terkelola yang memberlakukan pembatasan; UUID tunggal di file pengaturan lain apa pun pra-memilih organisasi selama login tanpa membatasinya.
* **Type**: string, satu UUID, atau array string, beberapa UUID
* **Default**: tidak diatur, jadi organisasi apa pun dapat masuk

Contoh ini menerima login dari salah satu dari dua organisasi tanpa pra-memilih satu:

```json managed-settings.json theme={null}
{
  "forceLoginOrgUUID": ["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", "yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy"]
}
```

Jika sumber terkelola mengatur array kosong, atau nilai yang Claude Code tidak dapat mengurai, Claude Code memblokir setiap login dengan pesan salah konfigurasi.

Lihat [Batasi login ke organisasi Anda](/docs/id/authentication#restrict-login-to-your-organization) untuk cara Claude Code memperlakukan login Claude Console, jalur login lainnya, dan kredensial lingkungan.

<h3 id="gatewayinternalnetworks">
  `gatewayInternalNetworks`
</h3>

Deklarasikan blok IPv4 publik yang organisasi Anda gunakan untuk menomori jaringan internalnya, sehingga `/login` menerima [cloud gateway](/docs/id/claude-apps-gateway) di sana. Memerlukan Claude Code v2.1.268 atau lebih baru.

Tanpa kunci ini, `/login` terhubung ke gateway apa pun pada alamat pribadi dan tidak ada yang lain. Dengan kunci ini, `/login` juga menerima gateway di dalam blok yang terdaftar, hanya melalui koneksi langsung. Alamat mesin itu sendiri pada koneksi itu juga harus berada di dalam blok yang sama.

* **Scope**: [`Managed`](#scopes). Baca hanya dari sumber di mesin: `managed-settings.json`, plist macOS atau registri Windows HKLM, atau pembantu kebijakan. Claude Code mengabaikannya dalam pengaturan HKCU dan server-terkelola.
* **Type**: array string, paling banyak empat blok CIDR IPv4, masing-masing `/8` hingga `/32`, tidak tumpang tindih satu sama lain, dan tidak ada yang tumpang tindih dengan ruang pribadi.
* **Default**: tidak diatur, jadi `/login` hanya menerima gateway pada alamat pribadi

```json managed-settings.json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

Ganti rentang dokumentasi dalam contoh dengan blok Anda sendiri. Claude Code menolak rentang dokumentasi, rentang yang digunakan klien VPN dan NAT64 secara lokal, dan ruang yang dicadangkan yang tidak ada jaringan yang dinomori darinya, seperti multicast.

Jika entri tidak valid, atau nilainya bukan daftar string, `/login` menyebutkan masalahnya dan menolak setiap sign-in gateway baru di mesin sampai Anda memperbaiki nilainya. Sign-in yang ada terus bekerja. Lihat [Izinkan gateway pada ruang alamat publik yang Anda miliki](/docs/id/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) untuk aturan lengkap dan apa yang dilihat pengembang.

<h3 id="gcpauthrefresh">
  `gcpAuthRefresh`
</h3>

Jalankan perintah Anda sendiri untuk menyegarkan Google Cloud Application Default Credentials ketika Claude Code menemukan bahwa kredensial telah kedaluwarsa atau tidak dapat dimuat, sehingga permintaan [Google Cloud's Agent Platform](/docs/id/google-vertex-ai) terus bekerja tanpa Anda perlu melakukan autentikasi ulang secara manual.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, baris perintah shell
* **Default**: tidak diatur, jadi kesalahan kredensial Claude Code memberi tahu Anda untuk menjalankan `gcloud auth application-default login` sendiri

```json settings.json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login"
}
```

Lihat [konfigurasi kredensial lanjutan](/docs/id/google-vertex-ai#advanced-credential-configuration).

<h3 id="otelheadershelper">
  `otelHeadersHelper`
</h3>

Jalankan perintah Anda sendiri untuk menghasilkan header yang Claude Code kirimkan dengan ekspor OpenTelemetry, untuk backend yang tokennya berputar. Claude Code menjalankannya saat startup dan secara berkala setelah itu, dan mengharapkan objek JSON dari nilai header string di stdout.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, jalur yang dapat dieksekusi atau baris perintah shell
* **Default**: tidak diatur, jadi Claude Code tidak menambahkan header yang dihasilkan pembantu

```json settings.json theme={null}
{
  "otelHeadersHelper": "/bin/generate_otel_headers.sh"
}
```

Atur interval penyegaran dengan [`CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`](/docs/id/env-vars). Lihat [Dynamic headers](/docs/id/monitoring-usage#dynamic-headers) untuk persyaratan skrip dan apa yang terjadi ketika pembantu gagal.

<h2 id="updates-and-versioning">
  Pembaruan dan versioning
</h2>

Pilih saluran pembaruan dan, untuk organisasi, tetapkan versi yang dapat dijalankan orang. Lihat [Update Claude Code](/docs/id/setup#update-claude-code).

<h3 id="autoupdateschannel">
  `autoUpdatesChannel`
</h3>

Pilih [saluran rilis](/docs/id/setup#configure-release-channel) mana yang diikuti pembaruan otomatis latar belakang dan `claude update`. Atur `"stable"` untuk versi yang biasanya berusia sekitar satu minggu dan melewati rilis dengan regresi besar, atau `"latest"` untuk rilis paling terbaru.

* **Scope**: [`Any file`](#scopes). Atur di managed settings untuk memberlakukan satu saluran di seluruh organisasi Anda.
* **Type**: string, salah satu dari:
  * `"latest"`: pembaruan mengikuti rilis paling terbaru
  * `"stable"`: pembaruan mengikuti versi yang biasanya berusia sekitar satu minggu dan melewati rilis dengan regresi besar
* **Default**: tidak diatur, jadi Claude Code mengikuti `"latest"`

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

Claude Code menulis `"stable"` ke pengaturan pengguna Anda ketika Anda memilihnya di bawah **Auto-update channel** di `/config`, dan menghapus kunci ketika Anda beralih kembali ke latest di sana. `claude install stable` dan `claude install latest` juga menyimpan saluran yang Anda beri nama. Beralih dari `"latest"` ke `"stable"` di `/config` menanyakan apakah akan mengizinkan downgrade atau tetap di versi saat ini Anda; tetap atur [`minimumVersion`](#minimumversion). Instalasi Homebrew mengabaikan kunci ini: cask `claude-code` melacak stable dan `claude-code@latest` melacak latest, dan `claude update` menunda ke `brew upgrade`. Untuk mematikan pembaruan otomatis sepenuhnya, atur [`DISABLE_AUTOUPDATER`](/docs/id/setup#disable-auto-updates) di `env`.

<h3 id="minimumversion">
  `minimumVersion`
</h3>

Cegah pembaruan otomatis latar belakang dan `claude update` dari menginstal versi apa pun di bawah yang ini, jadi berpindah ke saluran `"stable"` tidak menurunkan Anda dari build `"latest"` yang lebih baru. Claude Code menulis kunci ini untuk Anda ketika Anda memilih untuk tetap di versi saat ini Anda sambil beralih saluran di `/config`, dan menghapusnya ketika Anda beralih kembali ke `"latest"`.

* **Scope**: [`Any file`](#scopes). Atur di managed settings untuk menetapkan minimum di seluruh organisasi yang tidak dapat diturunkan oleh pengaturan pengguna dan proyek.
* **Type**: string, nomor versi seperti `"2.1.100"`; nilai yang bukan versi valid diabaikan
* **Default**: tidak diatur, jadi pembaruan dapat menginstal versi apa pun yang ditawarkan saluran

Contoh ini mengikuti saluran stable dan menolak untuk menginstal versi apa pun di bawah 2.1.100:

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

Kunci ini hanya membatasi pembaruan. Untuk membuat Claude Code menolak memulai di bawah versi, gunakan [`requiredMinimumVersion`](#requiredminimumversion) sebagai gantinya. Lihat [Pin a minimum version](/docs/id/setup#pin-a-minimum-version).

<h3 id="requiredmaximumversion">
  `requiredMaximumVersion`
</h3>

Atur versi Claude Code terbaru yang diizinkan organisasi Anda untuk memulai. Ketika versi yang berjalan lebih baru, Claude Code keluar saat startup dan memberi tahu pengguna untuk menginstal versi yang disetujui melalui metode yang disetujui organisasi Anda; `claude install <version>` mungkin juga berfungsi. Memerlukan Claude Code v2.1.163 atau lebih baru.

* **Scope**: [`Managed`](#scopes). Claude Code tidak memberikan peringatan ketika mengabaikan kunci di tempat lain.
* **Type**: string, nomor versi seperti `"2.1.150"`; nilai yang bukan versi valid diabaikan
* **Default**: tidak diatur, jadi tidak ada batas atas yang berlaku

```json managed-settings.json theme={null}
{
  "requiredMaximumVersion": "2.1.150"
}
```

Pembaruan otomatis latar belakang dan `claude update` melewati versi di atas batas, jadi instalasi dalam rentang tetap berada di dalamnya. `claude update`, `claude install`, dan `claude doctor` terus bekerja di atas batas sehingga pengguna dapat pulih. Pasangkan dengan [`requiredMinimumVersion`](#requiredminimumversion) untuk memberlakukan rentang.

<h3 id="requiredminimumversion">
  `requiredMinimumVersion`
</h3>

Atur versi Claude Code tertua yang diizinkan organisasi Anda untuk memulai. Ketika versi yang berjalan lebih lama, Claude Code keluar saat startup dan memberi tahu pengguna untuk memperbarui melalui metode yang disetujui organisasi Anda. Pemeriksaan berjalan hanya saat startup, jadi sesi yang sudah berjalan terus berlanjut. Memerlukan Claude Code v2.1.163 atau lebih baru.

* **Scope**: [`Managed`](#scopes). Claude Code tidak memberikan peringatan ketika mengabaikan kunci di tempat lain.
* **Type**: string, nomor versi seperti `"2.1.150"`; nilai yang bukan versi valid diabaikan
* **Default**: tidak diatur, jadi tidak ada batas bawah yang berlaku

```json managed-settings.json theme={null}
{
  "requiredMinimumVersion": "2.1.150"
}
```

`claude update`, `claude install`, dan `claude doctor` terus bekerja di bawah batas sehingga pengguna dapat pulih. Tidak seperti [`minimumVersion`](#minimumversion), yang hanya mencegah downgrade, kunci ini memblokir startup. Pasangkan dengan [`requiredMaximumVersion`](#requiredmaximumversion) untuk memberlakukan rentang.

<h2 id="tools">
  Tools
</h2>

Matikan alat tertentu di [aplikasi desktop Claude Code](/docs/id/desktop). CLI terminal mengabaikan kunci-kunci ini. Untuk alat-alat itu sendiri, lihat [Tools available to Claude](/docs/id/tools-reference).

<h3 id="browserexternalpagetools">
  `browserExternalPageTools`
</h3>

Hentikan Claude dari menggunakan alatnya untuk membaca atau bertindak pada halaman eksternal di [Browser pane](/docs/id/desktop#browse-external-sites) aplikasi desktop. Orang-orang di organisasi Anda masih dapat membuka situs eksternal sendiri, dan pratinjau server dev lokal terus bekerja dengan alat Claude. Aplikasi desktop membaca kunci ini; CLI terminal mengabaikannya.

* **Scope**: [`Managed`](#scopes)
* **Type**: string, `"disabled"`; aplikasi desktop juga menerima `"disable"`, dalam kedua kasus
* **Default**: tidak diatur, jadi alat Claude bekerja pada halaman eksternal

```json managed-settings.json theme={null}
{
  "browserExternalPageTools": "disabled"
}
```

Nilai lainnya membiarkan alat Claude tetap aktif, dan string yang tidak kosong yang bukan salah satu dari dua nilai yang diterima mencatat peringatan. Untuk memblokir situs eksternal untuk orang-orang dan Claude, atur [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation) sebagai gantinya. Lihat [Restrict external browsing for your organization](/docs/id/desktop#restrict-external-browsing-for-your-organization).

<h3 id="disablebrowserexternalnavigation">
  `disableBrowserExternalNavigation`
</h3>

Matikan penjelajahan eksternal di [Browser pane](/docs/id/desktop#browse-external-sites) aplikasi desktop untuk orang-orang dan Claude. Pratinjau server dev localhost terus bekerja. Aplikasi desktop membaca kunci ini; CLI terminal mengabaikannya.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean; hanya Boolean JSON `true` yang berlaku
  * `true`: aplikasi desktop mematikan penjelajahan eksternal di Browser pane untuk orang-orang dan Claude; pratinjau localhost terus bekerja
  * `false`: penjelajahan eksternal tetap aktif
* **Default**: tidak diatur, jadi penjelajahan eksternal aktif

```json managed-settings.json theme={null}
{
  "disableBrowserExternalNavigation": true
}
```

Aplikasi desktop mengabaikan nilai lainnya, dan nilai yang bukan Boolean, seperti string `"true"` atau `1`, juga mencatat peringatan. Untuk membiarkan penjelajahan eksternal tetap aktif tetapi menjaga alat Claude tetap mati pada halaman eksternal, atur [`browserExternalPageTools`](#browserexternalpagetools) sebagai gantinya. Lihat [Restrict external browsing for your organization](/docs/id/desktop#restrict-external-browsing-for-your-organization).

<h3 id="disablemobilesimulatortools">
  `disableMobileSimulatorTools`
</h3>

Blokir alat Claude untuk [iOS Simulator pane](/docs/id/desktop-ios-simulator#turn-off-simulator-access) aplikasi desktop. Orang-orang tetap mempertahankan penggunaan manual dari pane; hanya akses Claude yang dihapus, dan tidak ada yang dapat menghidupkannya kembali dari dalam aplikasi. Aplikasi desktop membaca kunci ini; CLI terminal mengabaikannya.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean; hanya Boolean JSON `true` yang berlaku
  * `true`: aplikasi desktop memblokir alat Claude untuk iOS Simulator pane
  * `false`: alat simulator Claude mengikuti pengaturan toggle setiap orang di aplikasi desktop
* **Default**: tidak diatur, jadi alat simulator Claude mengikuti pengaturan toggle setiap orang di aplikasi desktop

```json managed-settings.json theme={null}
{
  "disableMobileSimulatorTools": true
}
```

Aplikasi desktop mengabaikan nilai lainnya, dan nilai yang bukan Boolean, seperti string `"true"` atau `1`, juga mencatat peringatan.

<span id="data-and-privacy" />

<h2 id="privacy-and-telemetry">
  Privasi dan telemetri
</h2>

Kontrol berapa lama Claude Code menyimpan data sesi dan apa yang dikirimnya. Saklar yang mematikan metrik penggunaan dan laporan kesalahan adalah variabel lingkungan, bukan kunci pengaturan: atur `DISABLE_TELEMETRY`, `DISABLE_ERROR_REPORTING`, atau `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` dalam kunci [`env`](#env) atau di shell. [Layanan telemetri](/docs/id/data-usage#telemetry-services) menjelaskan apa yang masing-masing hentikan. Dua pengecualian mematikan dari file pengaturan: [`feedbackDrafts`](#feedbackdrafts) di bawah untuk umpan balik yang dirancang Claude, dan [`feedbackSurveyRate`](#feedbacksurveyrate) di bawah untuk survei sesi.

<h3 id="cleanupperioddays">
  `cleanupPeriodDays`
</h3>

Atur berapa hari Claude Code menyimpan [transkrip sesi dan data aplikasi lainnya](/docs/id/claude-directory#cleaned-up-automatically) sebelum menghapusnya. Claude Code menjalankan penghapusan sebagai sapuan latar belakang setelah sesi dimulai, selama dapat dengan aman menentukan periode retensi.

* **Scope**: [`Any file`](#scopes)
* **Type**: jumlah hari, bilangan bulat, minimum `1`
* **Default**: `30`

```json settings.json theme={null}
{
  "cleanupPeriodDays": 20
}
```

Mengatur `0` gagal validasi, jadi pilih nilai besar seperti `3650` untuk retensi jangka panjang. Untuk menghentikan Claude Code dari penulisan transkrip sama sekali, lihat [Plaintext storage](/docs/id/claude-directory#plaintext-storage).

<h3 id="desktopsessioncleanupperioddays">
  `desktopSessionCleanupPeriodDays`
</h3>

Atur batas usia dalam hari untuk transkrip sesi yang Anda mulai atau lanjutkan terakhir di Claude Desktop atau Cowork. Tanpa kunci ini, Claude Code [menyimpan transkrip tersebut pada usia berapa pun](/docs/id/claude-directory#cleaned-up-automatically). Claude Code menghapus masing-masing setelah lebih tua dari batas ini dan [`cleanupPeriodDays`](#cleanupperioddays), jadi dengan `cleanupPeriodDays` pada default 30, nilai `7` masih menyimpannya 30 hari. Ketika pengaturan terkelola menetapkan `cleanupPeriodDays`, periode itu berlaku sebagai gantinya dan kunci ini diabaikan. Memerlukan Claude Code v2.1.248 atau lebih baru.

* **Scope**: [`User or managed`](#scopes). Claude Code juga membaca kunci dari file yang Anda berikan dengan `--settings`, dan mengabaikannya dalam pengaturan proyek dan lokal.
* **Type**: jumlah hari, bilangan bulat, minimum `0`
* **Default**: `0`, yang tidak menetapkan batas usia

```json settings.json theme={null}
{
  "desktopSessionCleanupPeriodDays": 90
}
```

<h3 id="feedbackdrafts">
  `feedbackDrafts`
</h3>

Kontrol [umpan balik yang dirancang Claude](/docs/id/tools-reference#sendfeedback-tool-behavior): apakah Claude dapat mengantrekan draf umpan balik untuk Anda tinjau, dan apakah Claude Code menampilkan kartu ketika Claude mengantrekan satu.

* **Scope**: [`User or managed`](#scopes)
* **Type**: string, salah satu dari `"notify"`, `"quiet"`, atau `"off"`
  * `"notify"`: Claude Code menampilkan kartu di atas prompt ketika Claude mengantrekan draf, hingga [tiga kartu dalam sesi](/docs/id/tools-reference#what-you-see-when-claude-drafts) secara default
  * `"quiet"`: Claude membuat draf tanpa kartu. Anda melihat jumlah draf yang antri di footer prompt dan meninjau mereka di `/feedback`
  * `"off"`: Claude Code menghapus alat SendFeedback, jadi Claude tidak dapat mengantrekan draf
* **Default**: `"notify"`
* **Per-session overrides**: [`CLAUDE_CODE_SEND_FEEDBACK`](/docs/id/env-vars) diatur ke `0` mematikan fitur untuk satu sesi

```json settings.json theme={null}
{
  "feedbackDrafts": "quiet"
}
```

Muncul di `/config` sebagai **Claude-drafted feedback**, yang menulis kunci ini ke pengaturan pengguna Anda. Anda melihat baris `/config` hanya dalam sesi [di mana Claude dapat membuat draf umpan balik](/docs/id/tools-reference#sessions-without-claude-drafted-feedback); mengatur `"off"` tidak menyembunyikannya, jadi Anda dapat menghidupkan fitur kembali dari baris yang sama. Nilai dalam pengaturan terkelola mengambil alih pengaturan pengguna Anda, jadi ketika administrator menetapkan kunci ini, baris menampilkan nilai terkelola dan mengubahnya tidak berpengaruh. Claude Code mengabaikan kunci ini dalam pengaturan proyek dan lokal.

<h3 id="feedbacksurveyrate">
  `feedbackSurveyRate`
</h3>

Atur probabilitas bahwa [survei kualitas sesi](/docs/id/data-usage#session-quality-surveys) muncul ketika sesi memenuhi syarat untuk itu. Atur `0` untuk mencegah survei muncul.

* **Scope**: [`Any file`](#scopes)
* **Type**: angka antara `0` dan `1`
* **Default**: tidak diatur, jadi Claude Code menggunakan tingkat yang Anthropic atur dari jarak jauh, atau tingkat bawaan `0.005` di Amazon Bedrock, Platform Agen Google Cloud, dan Microsoft Foundry, yang tidak menerima konfigurasi jarak jauh
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY`](/docs/id/env-vars) diatur ke `1` mematikan survei untuk satu sesi apa pun tingkat kunci ini atur

```json settings.json theme={null}
{
  "feedbackSurveyRate": 0.05
}
```

Tingkat yang sama berlaku untuk survei di ekstensi VS Code.

<h3 id="skipwebfetchpreflight">
  `skipWebFetchPreflight`
</h3>

Lewati [pemeriksaan keamanan domain WebFetch](/docs/id/data-usage#webfetch-domain-safety-check), yang mengirim setiap nama host yang diminta ke `api.anthropic.com` sebelum mengambil. Atur `true` di lingkungan yang memblokir lalu lintas ke Anthropic, seperti Amazon Bedrock, Platform Agen Google Cloud, atau penyebaran Microsoft Foundry dengan egress yang ketat.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code melewati pemeriksaan keamanan domain WebFetch
  * `false`: pemeriksaan berjalan sebelum pengambilan pertama ke setiap nama host dalam sesi, dan lagi untuk nama host yang pemeriksaannya sebelumnya diblokir atau gagal
* **Default**: tidak diatur, jadi pemeriksaan berjalan sebelum pengambilan pertama ke setiap nama host dalam sesi

```json settings.json theme={null}
{
  "skipWebFetchPreflight": true
}
```

Dengan pemeriksaan dilewati, WebFetch mencoba URL apa pun tanpa berkonsultasi dengan daftar blokir, jadi pasangkan dengan [aturan izin `WebFetch`](/docs/id/permissions#webfetch) jika Anda perlu membatasi domain mana yang dapat dijangkau Claude.

<span id="managed-policy" />

<h2 id="enterprise-and-managed-settings">
  Pengaturan enterprise dan terkelola
</h2>

Kunci yang digunakan organisasi untuk menghitung, menyegarkan, dan menggabungkan pengaturan terkelola. Lihat [Siapkan pengaturan terkelola](/docs/id/admin-setup).

<h3 id="disablesideloadflags">
  `disableSideloadFlags`
</h3>

Tolak flag CLI `--plugin-dir`, `--plugin-url`, `--agents`, dan `--mcp-config` saat startup, yang dapat digunakan pengguna untuk melewati [`strictKnownMarketplaces`](#strictknownmarketplaces) untuk satu kali jalankan. Claude Code keluar dengan kesalahan yang menyebutkan flag yang ditolak, dan menerapkan pemeriksaan yang sama ke permukaan yang memulai CLI dengan flag ini secara internal, saat ini [Cowork](/docs/id/desktop) sesi lokal di aplikasi desktop. Dalam [sesi cloud](/docs/id/claude-code-on-the-web), Claude Code menghapus server MCP yang dikirimkan server melalui `--mcp-config`, kecuali entri `type: "sdk"` dalam proses, dan memulai sesi. Memerlukan Claude Code v2.1.193 atau lebih baru.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code menolak `--plugin-dir`, `--plugin-url`, `--agents`, dan `--mcp-config` saat startup dan keluar dengan kesalahan yang menyebutkannya, kecuali dalam sesi cloud di mana ia menghapus server MCP yang dikirimkan server melalui `--mcp-config`, kecuali entri `type: "sdk"` dalam proses, dan memulai sesi
  * `false`: Claude Code menerima flag tersebut
* **Default**: `false`

```json managed-settings.json theme={null}
{
  "disableSideloadFlags": true
}
```

Claude Code masih menerima `--mcp-config` yang servernya adalah semua entri `type: "sdk"` dalam proses, sehingga Agent SDK dan ekstensi VS Code tetap berfungsi. Pengguna masih dapat menambahkan server dengan `claude mcp add` atau file `.mcp.json`; untuk kontrol per-server, atur [`allowedMcpServers`](/docs/id/managed-mcp) juga. Memerlukan Claude Code v2.1.193 atau lebih baru.

Pemeriksaan yang sama mencakup folder plugin yang dinamai dalam variabel lingkungan [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/id/env-vars#variables), yang memerlukan Claude Code v2.1.280 atau lebih baru. Ketika variabel menyebutkan folder, Claude Code keluar dengan kesalahan yang sama, dan kesalahan mengatakan untuk membatalkan pengaturan variabel.

Dalam sesi cloud, Claude Code juga mengabaikan pembaruan MCP yang dikirimkan server di tengah sesi, jalur di balik konfigurasi sesi cloud dan SDK `setMcpServers()` pada pekerja jarak jauh. Entri `type: "sdk"` dalam proses tetap dikecualikan di sana juga. Sebelum v2.1.239, `--mcp-config` yang dikirimkan server memblokir sesi cloud dari dimulai.

<h3 id="forceremotesettingsrefresh">
  `forceRemoteSettingsRefresh`
</h3>

Blokir startup CLI hingga Claude Code telah segar mengambil [pengaturan yang dikelola server](/docs/id/server-managed-settings). Jika pengambilan gagal, Claude Code keluar alih-alih melanjutkan dengan pengaturan yang di-cache atau tidak ada. Atur ketika lingkungan Anda tidak dapat menerima bahkan jendela singkat di mana sesi berjalan tanpa kebijakan terkelolanya.

Ketika kunci tidak diatur, Claude Code tidak memblokir startup pada pengambilan, meskipun ketika pengembang masuk saat startup ia menunggu hingga lima detik untuk pengambilan. Sesi gateway Cloud selalu menunggu, dan keluar jika gateway tidak dapat dijangkau.

* **Scope**: [`Managed`](#scopes). Claude Code menghormati `true` dari sumber terkelola yang dikontrol admin apa pun, bahkan yang bukan sumber prioritas tertinggi.
* **Type**: Boolean
  * `true`: Claude Code memblokir startup hingga telah segar mengambil pengaturan yang dikelola server, dan keluar jika pengambilan gagal
  * `false`: Claude Code tidak memblokir startup pada pengambilan, meskipun pada startup masuk ia menunggu hingga lima detik untuk pengambilan
* **Default**: `false`

```json managed-settings.json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Atur dalam profil MDM atau file pengaturan terkelola untuk memberlakukan startup fail-closed sebelum muatan server pertama tiba. Claude Code menerapkan pemeriksaan hanya dalam sesi yang mengambil pengaturan yang dikelola server, jadi sesi yang [tidak mengambilnya](/docs/id/server-managed-settings#platform-availability) dimulai tanpa menunggu. Subperintah `claude auth` dikecualikan, sehingga pengguna dapat mengotentikasi ulang ketika kredensial yang kedaluwarsa adalah alasan pengambilan gagal. Lihat [Berlakukan startup fail-closed](/docs/id/server-managed-settings#enforce-fail-closed-startup).

<h3 id="managedsourcesbehavior">
  `managedSourcesBehavior`
</h3>

Pilih apakah Claude Code menerapkan hanya [sumber terkelola](/docs/id/managed-settings#how-claude-code-combines-managed-sources) prioritas tertinggi yang disampaikan organisasi Anda, atau menggabungkan setiap sumber admin yang disampaikannya. Secara default Claude Code mengambil sumber prioritas tertinggi yang membawa [kunci kebijakan](/docs/id/managed-settings#how-claude-code-combines-managed-sources) dan mengabaikan sisanya. Kunci kebijakan adalah kunci pengaturan apa pun selain yang ini dan `wslInheritsWindowsSettings`. Jadi setelah pengaturan yang dikelola server atau kebijakan MDM memberikan kunci kebijakan, file `managed-settings.json` berkontribusi hanya [kunci yang Claude Code baca dari setiap sumber admin](/docs/id/managed-settings#keys-read-from-every-admin-source). Dengan `"merge"`, setiap sumber admin yang Anda sampaikan berkontribusi kuncinya ke satu kebijakan gabungan. Memerlukan Claude Code v2.1.242 atau lebih baru.

Atur `"merge"` hanya di mana setiap sumber [peringkat](/docs/id/managed-settings#how-claude-code-combines-managed-sources) di bawah yang tertinggi berada di bawah kontrol administrator, karena Claude Code kemudian menambahkan entri dari sumber yang lebih rendah, seperti aturan `permissions.allow`, ke kebijakan.

* **Scope**: [`Managed`](#scopes). Claude Code membaca kunci ini dari sumber prioritas tertinggi yang membawa kunci ini atau kunci kebijakan, dan mengabaikan kunci ini di setiap sumber yang peringkatnya lebih rendah, jadi sumber yang lebih rendah tidak dapat memilih dirinya sendiri untuk menggabungkan dengan sumber di atasnya. Baik registri HKCU Windows maupun [pengaturan induk dari host penyematan](/docs/id/managed-settings#let-an-embedding-host-add-policy) tidak berpartisipasi dalam penggabungan.
* **Type**: string, salah satu dari:
  * `"first-wins"`: sumber prioritas tertinggi yang membawa kunci kebijakan menyediakan kebijakan, dan sumber yang lebih rendah berkontribusi hanya [kunci yang Claude Code baca dari setiap sumber admin](/docs/id/managed-settings#keys-read-from-every-admin-source)
  * `"merge"`: setiap sumber admin yang Anda sampaikan berkontribusi kuncinya, digabungkan oleh aturan di bawah
* **Default**: `"first-wins"`

Sampaikan kunci dalam sumber prioritas tertinggi yang Anda terapkan. Mesin yang tidak pernah menerima pengaturan yang dikelola server memerlukan kunci dalam profil MDM-nya juga, karena Claude Code membaca kunci dari sumber prioritas tertinggi yang membawanya atau kunci kebijakan. File `managed-settings.json` adalah sumber admin dengan peringkat terendah, jadi `"merge"` yang ditetapkan di sana tidak memiliki sumber di bawahnya untuk digabungkan. Dalam pengaturan yang dikelola server, kunci terlihat seperti ini:

```json theme={null}
{
  "managedSourcesBehavior": "merge"
}
```

Di bawah `"merge"`, Claude Code menggabungkan setiap kunci berdasarkan jenisnya. Tabel ini memberikan aturan untuk setiap jenis. Baris daftar pembatasan, nilai-diambil-utuh, dan hanya-sumber-tertinggi menyebutkan setiap kunci yang mereka tutupi, dan baris lainnya memberikan contoh:

| Jenis kunci                                | Bagaimana Claude Code menggabungkannya                                                                                                                                                                                             | Kunci                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :----------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Lists                                      | Menggabungkan entri dari setiap sumber                                                                                                                                                                                             | [`permissions.allow`](#permissions-allow), [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains), dan kunci daftar lainnya                                                                                                                                                                                                                                                                                                                                                                                       |
| Locks                                      | Menerapkan nilai paling ketat yang ditetapkan sumber apa pun. Ketika tidak ada sumber yang menetapkan nilai ketat, menerapkan nilai yang lebih longgar hanya dari sumber tertinggi                                                 | [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly), [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode), dan kunci boolean atau enum lock lainnya                                                                                                                                                                                                                                                                                                                       |
| Restriction allowlists                     | Mengambil daftar utuh dari sumber tertinggi yang menetapkannya, tanpa menambahkan entri dari sumber yang lebih rendah. Ketika sumber tertinggi tidak menetapkannya, mengambilnya utuh dari sumber berikutnya ke bawah              | [`availableModels`](#availablemodels), [`allowedMcpServers`](#allowedmcpservers), [`strictKnownMarketplaces`](#strictknownmarketplaces), [`allowedChannelPlugins`](#allowedchannelplugins), dan rantai [`fallbackModel`](#fallbackmodel)                                                                                                                                                                                                                                                                                       |
| Values taken whole                         | Mengambil nilai utuh dari sumber tertinggi yang menetapkannya, tanpa menggabungkan entri atau bidang dari sumber yang lebih rendah. Ketika sumber tertinggi tidak menetapkannya, mengambilnya utuh dari sumber berikutnya ke bawah | [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs), [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Provided MCP servers                       | Menggabungkan nama server dari setiap sumber. Ketika dua sumber menetapkan nama yang sama, menerapkan entri utuh sumber yang lebih tinggi                                                                                          | [`managedMcpServers`](#managedmcpservers)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Read from the highest-priority source only | Membaca kunci hanya dari sumber prioritas tertinggi yang membawa kunci kebijakan, jadi nilai sumber yang lebih rendah diabaikan bahkan ketika sumber tertinggi tidak menetapkan apa pun                                            | [`apiKeyHelper`](#apikeyhelper), [`awsAuthRefresh`](#awsauthrefresh), [`awsCredentialExport`](#awscredentialexport), [`gcpAuthRefresh`](#gcpauthrefresh), [`otelHeadersHelper`](#otelheadershelper), `proxyAuthHelper`, [`forceLoginOrgUUID`](#forceloginorguuid), nilai `"claudeai"` dan `"console"` dari [`forceLoginMethod`](#forceloginmethod), [`parentSettingsBehavior`](#parentsettingsbehavior), [`modelPicker`](#modelpicker), [`policyHelper`](#policyhelper), [`permissions.defaultMode`](#permissions-defaultmode) |
| `env`                                      | [Menggabungkan per variabel di seluruh sumber admin](/docs/id/managed-settings#keys-read-from-every-admin-source), di bawah `"first-wins"` dan `"merge"`                                                                                | [`env`](#env)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Setiap kunci lainnya                       | Mengambil nilai dari sumber tertinggi yang menetapkannya                                                                                                                                                                           | [`cleanupPeriodDays`](#cleanupperioddays), [`model`](#model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

Mengambil `sandbox.credentials.awsPairs` dan `sandbox.ripgrep` utuh memerlukan Claude Code v2.1.257 atau lebih baru.

Beberapa kunci menambahkan kondisi yang tidak ditunjukkan tabel:

* **[`policyHelper`](#policyhelper)**: Claude Code menghormatinya hanya ketika sumber tertinggi yang membawa kunci kebijakan adalah kebijakan MDM atau file pengaturan terkelola, jadi di bawah pengaturan yang dikelola server tidak berlaku.
* **[`modelOverrides`](#modeloverrides)**: berpasangan dengan `availableModels`. Claude Code mengambil `modelOverrides` dari sumber tertinggi yang menetapkannya, kecuali sumber yang lebih tinggi menetapkan `availableModels` tanpa `modelOverrides`. Dalam hal itu, ia mengabaikan `modelOverrides` dari setiap sumber.
* **[`forceLoginGatewayUrl`](#forcelogingatewayurl), [`gatewayInternalNetworks`](#gatewayinternalnetworks), dan nilai `"gateway"` dari [`forceLoginMethod`](#forceloginmethod)**: Claude Code tidak pernah membaca salah satu dari mereka dari pengaturan yang dikelola server, jadi nilai di sana tidak berlaku atau menyembunyikan yang ditetapkan dalam kebijakan MDM atau file pengaturan terkelola. Di antara sumber admin di mesin, hanya yang tertinggi-peringkat yang membawa kunci kebijakan yang menyediakannya, terlepas dari apakah pengaturan yang dikelola server juga ada.

Untuk mengonfirmasi sumber mana yang digabungkan di mesin, jalankan `/status` dan [baca baris `Setting sources`](/docs/id/managed-settings#read-the-source-in-/status).

<h3 id="parentsettingsbehavior">
  `parentSettingsBehavior`
</h3>

Pilih apakah Claude Code menerapkan pengaturan terkelola yang disediakan oleh proses host penyematan, seperti Agent SDK atau ekstensi IDE, ketika tingkat terkelola yang diterapkan admin juga ada. Dengan `"first-wins"`, Claude Code menghapus pengaturan yang disediakan host; dengan `"merge"`, ia menerapkannya di bawah tingkat admin melalui filter hanya-pembatasan. Atur `"merge"` ketika host perlu melewatkan pembatasannya sendiri ke sesi yang diluncurkannya, misalnya Claude Desktop memberikan daftar izin egress gateway.

* **Scope**: [`Managed`](#scopes). Claude Code membacanya dari sumber terkelola yang dikontrol admin prioritas tertinggi.
* **Type**: string, salah satu dari:
  * `"first-wins"`: Claude Code menghapus pengaturan yang disediakan host ketika tingkat terkelola yang diterapkan admin ada
  * `"merge"`: Claude Code menerapkan pengaturan yang disediakan host di bawah tingkat admin melalui filter hanya-pembatasan
* **Default**: `"first-wins"`

```json managed-settings.json theme={null}
{
  "parentSettingsBehavior": "merge"
}
```

Kunci ini tidak berpengaruh ketika tidak ada tingkat terkelola yang diterapkan admin: pengaturan host kemudian berlaku sebagai satu-satunya tingkat terkelola, masih disaring ke nilai pembatasan. Untuk batasan filter dan bagaimana sumber terkelola berinteraksi, lihat [Pengaturan induk dari host penyematan](/docs/id/managed-settings#parent-settings-from-embedding-hosts) dan [Batasi pengaturan induk](/docs/id/claude-apps-gateway#restrict-parent-settings).

<span id="compute-managed-settings-with-a-policy-helper" />

<h3 id="policyhelper">
  `policyHelper`
</h3>

Jalankan executable yang Anda terapkan yang menghitung pengaturan terkelola saat startup, sehingga Anda dapat menurunkan kebijakan dari postur perangkat, identitas, atau layanan jarak jauh alih-alih file statis. Claude Code menjalankan helper sebelum menerima prompt pertama dan memperlakukan pengaturan yang dipancarkannya sebagai pengaturan terkelola untuk sesi.

* **Scope**: [`Managed`](#scopes). Baca dari plist macOS, registri Windows HKLM, atau file pengaturan terkelola. Claude Code membaca kunci dari sumber terkelola prioritas tertinggi yang membawa [kunci kebijakan](/docs/id/managed-settings#how-claude-code-combines-managed-sources) dan menjalankan helper hanya ketika sumber itu adalah salah satu dari ketiga itu; ia mengabaikan kunci dalam pengaturan yang dikelola server, registri HKCU, dan pengaturan induk yang disediakan host.
* **Type**: object dengan `path`, `timeoutMs`, dan `refreshIntervalMs`
* **Default**: unset, jadi tidak ada helper yang berjalan

Ketika pengaturan yang dikelola server memberikan kebijakan saat peluncuran, mereka mengambil alih dari sumber helper dan helper tidak berjalan.

Jika pengambilan pengaturan yang lebih baru melaporkan pengaturan yang dikelola server dihapus, Claude Code menjalankan helper pada titik itu daripada menunggu peluncuran berikutnya. Outputnya mengatur sisa sesi, dan jalankan yang gagal mengakhiri sesi dengan pesan yang sama seperti [jalankan startup yang gagal](#helper-failures).

Contoh ini menjalankan helper dengan timeout 5 detik dan menjalankannya kembali setiap lima menit:

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy",
    "timeoutMs": 5000,
    "refreshIntervalMs": 300000
  }
}
```

<h4 id="write-the-helper-output">
  Tulis output helper
</h4>

Claude Code menjalankan helper tanpa argumen, menetapkan `CLAUDE_CODE_VERSION` dalam lingkungannya, dan membaca amplop JSON dari stdout, dibatasi pada 1 MiB.

Letakkan pengaturan di bawah kunci `managedSettings`. Objek pengaturan telanjang tanpa kunci `managedSettings` diuraikan dengan `managedSettings` tidak terdefinisi dan tidak menerapkan apa pun, dan Claude Code melaporkan tidak ada kesalahan:

```json theme={null}
{
  "managedSettings": {
    "permissions": { "deny": ["Read(//etc/secrets/**)"] }
  }
}
```

Ketika helper memancarkan `managedSettings`, objek itu menjadi satu-satunya sumber pengaturan terkelola untuk jalankan: Claude Code mengabaikan sumber MDM, file, dan HKCU, membaca [kunci lintas-sumber](/docs/id/managed-settings#keys-read-from-every-admin-source) dari output helper saja, dan tidak pernah menggabungkan [pengaturan induk](/docs/id/managed-settings#parent-settings-from-embedding-hosts).

Pemeriksaan `forceRemoteSettingsRefresh` startup berjalan sebelum helper dan membaca sumber admin apa pun. Helper yang keluar `0` dengan amplop yang menghilangkan `managedSettings` tidak berkontribusi pengaturan terkelola, dan sumber lainnya berlaku seperti biasa.

<h4 id="helper-failures">
  Helper failures
</h4>

Jalankan helper gagal ketika:

* `path` melanggar aturan dalam [`policyHelper.path`](#policyhelper-path).
* Tidak ada file reguler di `path`. Claude Code memeriksa file sebelum memulai helper, dalam anggaran `timeoutMs` yang sama, jadi mount jaringan yang tidak responsif dapat menyebabkan jalankan gagal.
* Helper keluar non-zero, masih berjalan ketika `timeoutMs` berlalu, atau tidak dimulai sama sekali, misalnya karena tidak dapat dieksekusi.
* Helper menulis lebih dari 1 MiB ke stdout atau stderr.
* stdout bukan objek JSON tunggal, atau `managedSettings`-nya memiliki [pelanggaran skema yang tidak dapat diperbaiki Claude Code](/docs/id/managed-settings#find-entries-claude-code-dropped).

Ketika jalankan startup gagal, Claude Code mencetak alasan dan menolak untuk memulai. Setelah keluar non-zero, alasan mencakup stderr helper, atau stdout-nya ketika stderr kosong. Setelah timeout, alasan menyebutkan batas `timeoutMs` dan tidak menyertakan apa pun dari output helper. Penolakan mencakup sesi interaktif, `claude -p`, sesi Agent SDK, [sesi latar belakang](/docs/id/agent-view), dan sebagian besar subperintah.

Penolakan disengaja, jadi helper yang memerlukan ketahanan pemadaman harus melayani dari cache-nya sendiri dan keluar `0`.

Ketika penyegaran latar belakang gagal, Claude Code menjaga kebijakan terakhir yang berhasil berlaku, dan `/status` menunjukkan penyegaran yang gagal dengan alasannya hingga penyegaran berhasil. Setiap penyegaran berjalan di bawah `timeoutMs` dan aturan kegagalan yang sama seperti jalankan startup.

Dengan `--debug`, Claude Code menulis stderr helper dari setiap jalankan ke [log debug](/docs/id/debug-your-config).

Claude Code melaporkan nilai `policyHelper` yang tidak valid sebagai [entri yang dijatuhkan](/docs/id/managed-settings#find-entries-claude-code-dropped) dan memulai sesi pada pengaturan terkelola yang tersisa tanpa menjalankan helper. Nilai yang tidak valid mencakup string path telanjang dan `timeoutMs` di bawah [minimumnya](#policyhelper-timeoutms).

Untuk mematikan helper, hapus kunci dari sumber yang menetapkannya.

<h3 id="policyhelper-path">
  `policyHelper.path`
</h3>

Beri nama executable helper yang Claude Code jalankan. Untuk apa yang terjadi ketika path melanggar aturan di bawah, lihat [Helper failures](#helper-failures).

* **Scope**: [`Managed`](#scopes). Baca dari plist macOS, registri Windows HKLM, atau file pengaturan terkelola, di mana pun [`policyHelper`](#policyhelper) dibaca.
* **Type**: string, jalur absolut dalam bentuk ternormalisasi, tanpa segmen `.` atau `..`; di Windows, jalur huruf drive atau UNC yang berakhir dengan `.exe`
* **Default**: none; diperlukan ketika `policyHelper` diatur

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy"
  }
}
```

<h3 id="policyhelper-timeoutms">
  `policyHelper.timeoutMs`
</h3>

Atur berapa lama Claude Code menunggu helper sebelum memperlakukan jalankan sebagai gagal. Jalankan yang timeout gagal dengan cara yang sama seperti keluar non-zero, jadi saat startup Claude Code menolak untuk memulai.

* **Scope**: [`Managed`](#scopes). Baca dari plist macOS, registri Windows HKLM, atau file pengaturan terkelola, di mana pun [`policyHelper`](#policyhelper) dibaca.
* **Type**: integer, milliseconds, minimum `1000`
* **Default**: `10000`

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy",
    "timeoutMs": 5000
  }
}
```

<h3 id="policyhelper-refreshintervalms">
  `policyHelper.refreshIntervalMs`
</h3>

Buat Claude Code menjalankan kembali helper di latar belakang pada interval sehingga perubahan kebijakan mencapai sesi yang berjalan. Ketika penyegaran berhasil, outputnya menggantikan pengaturan terkelola sebelumnya tanpa restart; ketika penyegaran gagal, Claude Code menjaga kebijakan yang sudah dimilikinya.

* **Scope**: [`Managed`](#scopes). Baca dari plist macOS, registri Windows HKLM, atau file pengaturan terkelola, di mana pun [`policyHelper`](#policyhelper) dibaca.
* **Type**: integer, milliseconds: `0` untuk menonaktifkan penyegaran, jika tidak setidaknya `60000`
* **Default**: unset, jadi Claude Code menjalankan helper sekali saat startup

Contoh ini menjalankan kembali helper setiap lima menit:

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy",
    "refreshIntervalMs": 300000
  }
}
```

<h3 id="wslinheritswindowssettings">
  `wslInheritsWindowsSettings`
</h3>

Buat Claude Code di WSL membaca pengaturan terkelola dari rantai kebijakan Windows, dengan HKLM dan file pengaturan terkelola Windows mengambil prioritas atas `/etc/claude-code` dan HKCU di bawahnya. Sementara rantai aktif, Claude Code membaca `/etc/claude-code` hanya ketika tidak ada file pengaturan terkelola atau drop-in di bawah `C:\Program Files\ClaudeCode\` yang memberikan [kunci kebijakan](/docs/id/managed-settings#how-claude-code-combines-managed-sources). Atur untuk memperluas kebijakan yang sudah Anda terapkan di Windows ke sesi WSL di mesin yang sama, sehingga mereka mengikuti aturan yang sama seperti sesi host. Claude Code menghormatinya hanya ketika diatur dalam kunci registri HKLM atau dalam file pengaturan terkelola atau drop-in di bawah `C:\Program Files\ClaudeCode\`, keduanya memerlukan admin Windows untuk menulis.

* **Scope**: [`Managed`](#scopes). Dalam sumber Windows yang dikontrol admin.
* **Type**: Boolean
  * `true`: Claude Code di WSL membaca pengaturan terkelola dari rantai kebijakan Windows, dan membaca `/etc/claude-code` hanya ketika tidak ada file pengaturan terkelola atau drop-in di bawah `C:\Program Files\ClaudeCode\` yang memberikan [kunci kebijakan](/docs/id/managed-settings#how-claude-code-combines-managed-sources)
  * `false`: WSL membaca hanya `/etc/claude-code`
* **Default**: `false`, jadi WSL membaca hanya `/etc/claude-code`

```json managed-settings.json theme={null}
{
  "wslInheritsWindowsSettings": true
}
```

Setelah sumber admin mengaktifkan rantai, kebijakan HKCU bergabung dengannya di WSL hanya ketika HKCU juga menetapkan kunci ke `true`. Salinan itu tidak mengaktifkan rantai dengan sendirinya. Sumber Windows yang hanya berisi kunci ini tidak dihitung sebagai sumber kebijakan, jadi sumber prioritas yang lebih rendah masih menyediakan kebijakan. Kunci ini tidak berpengaruh pada Windows asli.

<h2 id="global-config-settings">
  Pengaturan konfigurasi global
</h2>

Simpan kunci-kunci ini di `~/.claude.json`, bukan di file pengaturan. Claude Code mengabaikannya di tempat lain. Claude Code dan `/config` menulis sebagian besar dari mereka untuk Anda, dan Anda juga dapat mengeditnya secara manual.

<h3 id="autoconnectide">
  `autoConnectIde`
</h3>

Terhubung ke IDE yang sedang berjalan secara otomatis saat Anda memulai Claude Code dari terminal eksternal. Muncul di `/config` sebagai **Auto-connect to IDE (external terminal)** saat Anda menjalankan Claude Code di luar terminal VS Code atau JetBrains.

* **Scope**: [`Global config`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code terhubung ke IDE yang sedang berjalan secara otomatis saat Anda memulainya dari terminal eksternal
  * `false`: Claude Code tidak terhubung secara otomatis dari terminal eksternal; di dalam terminal VS Code atau JetBrains, atau dengan `--ide`, tetap terhubung
* **Default**: `false`
* **Per-session overrides**: [`CLAUDE_CODE_AUTO_CONNECT_IDE`](/docs/id/env-vars) mengambil alih kunci ini untuk satu sesi, dalam kedua arah

```json ~/.claude.json theme={null}
{
  "autoConnectIde": true
}
```

Claude Code mengabaikan kunci ini di `settings.json`.

<h3 id="autoinstallideextension">
  `autoInstallIdeExtension`
</h3>

Instal ekstensi IDE Claude Code secara otomatis saat Anda menjalankan Claude Code dari terminal VS Code. Muncul di `/config` sebagai **Auto-install IDE extension** saat Anda menjalankan Claude Code di dalam terminal VS Code atau JetBrains.

* **Scope**: [`Global config`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code menginstal ekstensi IDE secara otomatis saat Anda menjalankannya dari terminal VS Code
  * `false`: Claude Code tidak menginstal ekstensi secara otomatis
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/id/env-vars) diatur ke `1` melewati instalasi untuk satu sesi bahkan ketika kunci ini adalah `true`

```json ~/.claude.json theme={null}
{
  "autoInstallIdeExtension": false
}
```

Claude Code mengabaikan kunci ini di `settings.json`.

<h3 id="copyonselect">
  `copyOnSelect`
</h3>

Salin teks ke clipboard Anda secara otomatis saat Anda selesai memilihnya dengan mouse di [fullscreen rendering](/docs/id/fullscreen#use-the-mouse) atau [agent view](/docs/id/agent-view). Muncul di `/config` sebagai **Copy on select** saat fullscreen rendering aktif.

* **Scope**: [`Global config`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code menyalin teks ke clipboard Anda saat Anda selesai memilihnya
  * `false`: memilih teks membiarkan clipboard Anda tidak berubah, dan Anda [menyalin pilihan dengan pintasan keyboard](/docs/id/fullscreen#use-the-mouse) sebagai gantinya
* **Default**: `true`

```json ~/.claude.json theme={null}
{
  "copyOnSelect": false
}
```

Claude Code mengabaikan kunci ini di `settings.json`.

<h3 id="difftool">
  `diffTool`
</h3>

Pilih di mana Claude Code menampilkan diff dari perubahan `Edit` atau `Write` yang diusulkannya saat [VS Code](/docs/id/vs-code) atau [JetBrains](/docs/id/jetbrains#features) IDE terhubung: `"auto"` membukanya di diff viewer IDE, `"terminal"` menyimpannya di terminal. Muncul di `/config` sebagai **Diff tool** hanya saat Claude Code terhubung ke VS Code atau JetBrains IDE.

* **Scope**: [`Global config`](#scopes)
* **Type**: string, salah satu dari:
  * `"auto"`: Claude Code membuka diff di diff viewer IDE saat VS Code atau JetBrains IDE terhubung
  * `"terminal"`: Claude Code menyimpan diff di terminal
* **Default**: `"auto"`

```json ~/.claude.json theme={null}
{
  "diffTool": "terminal"
}
```

Claude Code mengabaikan kunci ini di `settings.json`.

<h3 id="externaleditorcontext">
  `externalEditorContext`
</h3>

Saat Anda menekan `Ctrl+G`, Claude Code membuka prompt yang Anda ketik di [external editor](/docs/id/interactive-mode#general-controls) Anda. Dengan kunci ini aktif, buffer editor dimulai dengan respons sebelumnya Claude sebagai baris komentar `#`, sehingga Anda dapat membacanya saat Anda menulis, dan Claude Code menghapus baris-baris tersebut saat Anda menyimpan. Muncul di `/config` sebagai **Show last response in external editor**.

* **Scope**: [`Global config`](#scopes)
* **Type**: Boolean
  * `true`: buffer editor dimulai dengan respons sebelumnya Claude sebagai baris komentar `#`, yang Claude Code hapus saat Anda menyimpan
  * `false`: buffer editor dibuka hanya dengan prompt Anda
* **Default**: `false`

```json ~/.claude.json theme={null}
{
  "externalEditorContext": true
}
```

Dengan aktif, buffer yang Claude Code buka terlihat seperti ini, dan hanya teks di bawah garis penanda yang dikirim sebagai prompt Anda:

```text theme={null}
# ─── Claude's last response (for reference; removed on save) ───
# I added the retry loop to fetchUser in src/api.ts and a test
# for the timeout case. Want me to wire the same retry into
# fetchOrders?
# ─── Write your reply below this line ──────────────────────────

Yes, and cap it at three attempts.
```

Claude Code menyimpan 50 baris terakhir dari respons dan menandai potongan dengan `# … (earlier output truncated)`.

Claude Code mengabaikan kunci ini di `settings.json`.

<h3 id="permissionexplainerenabled">
  `permissionExplainerEnabled`
</h3>

<Warning>
  Dihapus di v2.1.257, bersama dengan penjelasan perintah `Ctrl+E` pada prompt izin Bash dan PowerShell. Mengaturnya tidak berpengaruh pada versi saat ini.
</Warning>

Melalui v2.1.256, Anda dapat menekan `Ctrl+E` pada prompt izin Bash atau PowerShell untuk melihat penjelasan perintah yang dihasilkan model, dan atur kunci ini ke `false` untuk mematikan pintasan tersebut.

* **Scope**: [`Global config`](#scopes). Pada v2.1.256 dan lebih awal.
* **Type**: Boolean
* **Default**: `true`

<h3 id="teammatedefaultmodel">
  `teammateDefaultModel`
</h3>

<Warning>
  Dihapus di v2.1.234, bersama dengan baris `/config` **Default teammate model**. Mengaturnya tidak berpengaruh pada versi saat ini.
</Warning>

Melalui v2.1.233, Anda mengatur kunci ini ke model untuk rekan tim [agent team](/docs/id/agent-teams#specify-teammates-and-models) yang prompt Anda tidak menamai model untuk: alias seperti `"sonnet"`, atau `null` untuk mengikuti model lead. Untuk model yang Claude Code pilih untuk rekan tim seperti itu sekarang, lihat [specify teammates and models](/docs/id/agent-teams#specify-teammates-and-models).

* **Scope**: [`Global config`](#scopes). Pada v2.1.233 dan lebih awal.
* **Type**: string, alias model atau ID model lengkap, atau `null`
* **Default**: unset

<h2 id="see-also">
  Lihat juga
</h2>

* [Konfigurasi izin](/docs/id/permissions): sintaks aturan, mode izin, dan kepercayaan ruang kerja
* [Variabel lingkungan](/docs/id/env-vars): setiap `CLAUDE_*`, `ANTHROPIC_*`, dan variabel penyedia yang dibaca Claude Code
* [Alat yang tersedia untuk Claude](/docs/id/tools-reference): alat bawaan dan mana yang memerlukan persetujuan
* [Contoh file pengaturan](/docs/id/settings-example): file pribadi, file tim, dan file terkelola organisasi
* [Siapkan pengaturan terkelola](/docs/id/admin-setup): bagaimana organisasi memutuskan apa yang akan diterapkan
* [Terapkan pengaturan terkelola](/docs/id/managed-settings): mekanisme pengiriman, prioritas dalam tingkat terkelola, dan entri tidak valid dalam pengaturan terkelola
* [Debug konfigurasi Anda](/docs/id/debug-your-config): `claude doctor` dan dialog Kesalahan Pengaturan
