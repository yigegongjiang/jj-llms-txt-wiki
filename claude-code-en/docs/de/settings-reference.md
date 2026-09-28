> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Alle Einstellungen

> Vollständige Referenz für jeden Claude Code settings.json-Schlüssel: wo jeder hingehört, sein Typ und Standard, sowie ein einsatzbereites Beispiel, mit einem Index aller Schlüssel.

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

<BackToIndex href="#all-settings" label="Zurück zum Index" />

Diese Referenzseite listet jeden Schlüssel auf, den Claude Code aus einer Einstellungsdatei liest, sowie die [kurze Gruppe von Schlüsseln](#global-config-settings), die es stattdessen in `~/.claude.json` speichert. Um eine Datei auszuwählen oder die Priorität zu überprüfen, beginnen Sie mit [Einstellungsdateien und Priorität](/docs/de/settings).

<span id="available-settings" />

<span id="scopes" />

<span id="all-settings" />

<h2 id="settings-index">
  Einstellungsindex
</h2>

Jeder Schlüssel unten verlinkt zu seinem Eintrag. Der Bereich listet die [Dateien](/docs/de/settings#settings-files-and-who-they-affect) auf, in denen er verwendet werden kann: `User` ist `~/.claude/settings.json`, `Project` ist `.claude/settings.json`, `Local` ist `.claude/settings.local.json`, und `Managed` ist [das, was Ihre Organisation bereitstellt](/docs/de/managed-settings). `Any file` bedeutet alle vier, und `Global config` bedeutet [`~/.claude.json`](#global-config-settings).

<ReferenceFilter
  noun="settings"
  placeholder="Filter settings by key or purpose"
  facetOrder={{ scope: ["Any file", "User, local, or managed", "User or managed", "Managed", "Global config"] }}
  columnHelp={{
topic: "The section of this page that holds the entry. Use Sort by to group the table by topic.",
scope: "Which settings files can set the key: user (~/.claude/settings.json), project (.claude/settings.json), local (.claude/settings.local.json), or managed (deployed by your organization). Global config keys are in ~/.claude.json instead.",
}}
/>

| Key                                                                                                   | Description                                                                                                                                                                                                                                                            | Topic                              | Scope                   |
| :---------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------- | :---------------------- |
| [`advisorModel`](#advisormodel)                                                                       | Wählen Sie aus, welches Modell antwortet, wenn Claude das [Advisor-Tool](/docs/de/advisor) verwendet                                                                                                                                                                        | Model and responses                | Any file                |
| [`agent`](#agent)                                                                                     | Starten Sie jede Sitzung als benannter [Subagent](/docs/de/sub-agents) mit seinem Prompt, seinen Tools und seinem Modell                                                                                                                                                    | Agents, sessions, and worktrees    | Any file                |
| [`agentPushNotifEnabled`](#agentpushnotifenabled)                                                     | Lassen Sie Claude eine [Push-Benachrichtigung an Ihr Telefon](/docs/de/remote-control#mobile-push-notifications) senden, wenn es sich dafür entscheidet                                                                                                                     | Remote, desktop, and notifications | Any file                |
| [`allowAllClaudeAiMcps`](#allowallclaudeaimcps)                                                       | Laden Sie die [claude.ai-Konnektoren](/docs/de/mcp), die Claude Code selbst abruft, zusammen mit einem bereitgestellten [`managed-mcp.json`](/docs/de/managed-mcp#exclusive-control-with-managed-mcp-json)                                                                       | MCP                                | Managed                 |
| [`allowedChannelPlugins`](#allowedchannelplugins)                                                     | Ersetzen Sie die Standard-Zulassungsliste der [Channel-Plugins](/docs/de/channels#restrict-which-channel-plugins-can-run), die Nachrichten pushen können                                                                                                                    | Plugins and skills                 | Managed                 |
| [`allowedHttpHookUrls`](#allowedhttphookurls)                                                         | Begrenzen Sie, welche URLs [HTTP-Hooks](/docs/de/hooks) ansteuern können                                                                                                                                                                                                    | Hooks and automation               | Any file                |
| [`allowedMcpServers`](#allowedmcpservers)                                                             | Zulassungsliste, welche [MCP-Server](/docs/de/mcp) Benutzer hinzufügen können                                                                                                                                                                                               | MCP                                | Any file                |
| [`allowManagedHooksOnly`](#allowmanagedhooksonly)                                                     | Führen Sie nur die [Hooks](/docs/de/hooks) aus, die Ihre Organisation bereitstellt                                                                                                                                                                                          | Hooks and automation               | Managed                 |
| [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly)                                           | Machen Sie die verwaltete [MCP](/docs/de/mcp)-Zulassungsliste zur einzigen, die gilt                                                                                                                                                                                        | MCP                                | Managed                 |
| [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly)                                 | Machen Sie [verwaltete Einstellungen](/docs/de/managed-settings) zur einzigen Einstellungsquelle für [Berechtigungsregeln](/docs/de/permissions#managed-settings)                                                                                                                | Permission settings                | Managed                 |
| [`alwaysThinkingEnabled`](#alwaysthinkingenabled)                                                     | Schalten Sie [erweitertes Denken](/docs/de/model-config#extended-thinking) für jede Sitzung aus                                                                                                                                                                             | Model and responses                | Any file                |
| [`apiKeyHelper`](#apikeyhelper)                                                                       | Generieren Sie die [API-Anmeldedaten](/docs/de/authentication#credential-management) mit Ihrem eigenen Befehl                                                                                                                                                               | Authentication and providers       | Any file                |
| [`askUserQuestionTimeout`](#askuserquestiontimeout)                                                   | Lassen Sie eine unbeantwortete Frage [automatisch fortfahren](/docs/de/tools-reference#question-auto-continue-timeout) nach Leerlaufzeit                                                                                                                                    | Interface and terminal             | User or managed         |
| [`attribution`](#attribution)                                                                         | Passen Sie die Zuschreibung an, die Claude Code zu Commits und Pull Requests hinzufügt                                                                                                                                                                                 | Git and attribution                | Any file                |
| [`attribution.commit`](#attribution-commit)                                                           | Ändern oder verbergen Sie den Trailer, den Claude Code zu Commits hinzufügt                                                                                                                                                                                            | Git and attribution                | Any file                |
| [`attribution.pr`](#attribution-pr)                                                                   | Ändern oder verbergen Sie die Zuschreibungszeile in Pull-Request-Beschreibungen                                                                                                                                                                                        | Git and attribution                | Any file                |
| [`attribution.sessionUrl`](#attribution-sessionurl)                                                   | Lassen Sie den claude.ai-Sitzungslink aus [Cloud](/docs/de/claude-code-on-the-web)- und [Remote-Control](/docs/de/remote-control)-Commits weg                                                                                                                                    | Git and attribution                | Any file                |
| [`autoCompactEnabled`](#autocompactenabled)                                                           | Schalten Sie [automatische Komprimierung](/docs/de/context-window) aus oder ein                                                                                                                                                                                             | Memory and context                 | Any file                |
| [`autoCompactWindow`](#autocompactwindow)                                                             | Legen Sie fest, wie voll der Kontext wird, bevor Claude Code [komprimiert](/docs/de/context-window)                                                                                                                                                                         | Memory and context                 | Any file                |
| [`autoConnectIde`](#autoconnectide)                                                                   | Verbinden Sie sich automatisch mit einer laufenden [VS Code](/docs/de/vs-code)- oder [JetBrains](/docs/de/jetbrains#from-external-terminals)-IDE von einem externen Terminal aus                                                                                                 | Global config settings             | Global config           |
| [`autoContinueAtUsageLimit`](#autocontinueatusagelimit)                                               | Warten Sie in der offenen Sitzung und [fahren Sie die Aufgabe automatisch fort](/docs/de/interactive-mode#wait-for-a-usage-limit-to-reset), nachdem ein claude.ai-Nutzungslimit zurückgesetzt wird                                                                          | Interface and terminal             | User or managed         |
| [`autoInstallIdeExtension`](#autoinstallideextension)                                                 | Schalten Sie die automatische Installation der [IDE-Erweiterung](/docs/de/vs-code#install-the-extension) von einem VS Code-Terminal aus                                                                                                                                     | Global config settings             | Global config           |
| [`autoMemoryDirectory`](#automemorydirectory)                                                         | Speichern Sie [automatisches Gedächtnis](/docs/de/memory#auto-memory) in einem Verzeichnis Ihrer Wahl                                                                                                                                                                       | Memory and context                 | Any file                |
| [`autoMemoryEnabled`](#automemoryenabled)                                                             | Schalten Sie [automatisches Gedächtnis](/docs/de/memory#auto-memory) aus oder ein                                                                                                                                                                                           | Memory and context                 | Any file                |
| [`autoMode`](#automode)                                                                               | Fügen Sie Ihre eigenen Allow- und Deny-Regeln zum [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode)-Klassifizierer hinzu                                                                                                                             | Permission settings                | User or managed         |
| [`autoMode.classifyAllShell`](#automode-classifyallshell)                                             | Senden Sie jeden Shell-Befehl durch den [Auto-Modus-Klassifizierer](/docs/de/permission-modes#what-the-classifier-blocks-by-default), auch solche, die eine enge Allow-Regel erfüllen                                                                                       | Permission settings                | User or managed         |
| [`autoScrollEnabled`](#autoscrollenabled)                                                             | [Folgen Sie neuer Ausgabe](/docs/de/fullscreen#auto-follow) zum unteren Ende beim Vollbild-Rendering                                                                                                                                                                        | Interface and terminal             | Any file                |
| [`autoUpdatesChannel`](#autoupdateschannel)                                                           | Folgen Sie dem stabilen [Release-Kanal](/docs/de/setup#configure-release-channel) statt dem neuesten                                                                                                                                                                        | Updates and versioning             | Any file                |
| [`availableModels`](#availablemodels)                                                                 | [Beschränken Sie, welche Modelle](/docs/de/model-config#restrict-model-selection) Personen auswählen können                                                                                                                                                                 | Model and responses                | Any file                |
| [`awaySummaryEnabled`](#awaysummaryenabled)                                                           | Schalten Sie die [Sitzungszusammenfassung](/docs/de/interactive-mode#session-recap) aus, die angezeigt wird, wenn Sie zum Terminal zurückkehren                                                                                                                             | Remote, desktop, and notifications | Any file                |
| [`awsAuthRefresh`](#awsauthrefresh)                                                                   | Aktualisieren Sie abgelaufene [Bedrock-Anmeldedaten](/docs/de/amazon-bedrock#advanced-credential-configuration) in `.aws` mit Ihrem eigenen Befehl                                                                                                                          | Authentication and providers       | Any file                |
| [`awsCredentialExport`](#awscredentialexport)                                                         | Stellen Sie [Bedrock-Anmeldedaten](/docs/de/amazon-bedrock#advanced-credential-configuration) als JSON aus Ihrem eigenen Befehl bereit                                                                                                                                      | Authentication and providers       | Any file                |
| [`axScreenReader`](#axscreenreader)                                                                   | Rendern Sie [bildschirmleserfreundliche Ausgabe](/docs/de/accessibility)                                                                                                                                                                                                    | Interface and terminal             | Any file                |
| [`bashEditDiffEnabled`](#basheditdiffenabled)                                                         | Zeichnen Sie die [Dateien auf, die sich während der Ausführung eines Bash-Befehls geändert haben](/docs/de/hooks#bash) in jedem Berechtigungsmodus                                                                                                                          | Interface and terminal             | User or managed         |
| [`bashOutputMaxChars`](#bashoutputmaxchars)                                                           | Legen Sie fest, wie viel der [Ausgabe](/docs/de/tools-reference#output-limits) eines erfolgreichen Befehls Claude inline erhält                                                                                                                                             | Memory and context                 | Any file                |
| [`blockedMarketplaces`](#blockedmarketplaces)                                                         | Blockieren Sie [Plugin-Marketplace](/docs/de/plugins/overview)-Quellen für Ihre Organisation                                                                                                                                                                                | Plugins and skills                 | Managed                 |
| [`browserExternalPageTools`](#browserexternalpagetools)                                               | Halten Sie Claudes Tools auf externen Seiten im [Desktop](/docs/de/desktop)-Browser-Bereich aus                                                                                                                                                                             | Tools                              | Managed                 |
| [`channelsEnabled`](#channelsenabled)                                                                 | Ermöglichen Sie [Kanäle](/docs/de/channels#enable-channels-for-your-organization) für Ihre Organisation                                                                                                                                                                     | Plugins and skills                 | Managed                 |
| [`claudeMd`](#claudemd)                                                                               | Injizieren Sie organisationsweite [CLAUDE.md](/docs/de/memory#deploy-organization-wide-claude-md)-Anweisungen aus verwalteten Einstellungen                                                                                                                                 | Memory and context                 | Managed                 |
| [`claudeMdExcludes`](#claudemdexcludes)                                                               | Überspringen Sie spezifische [CLAUDE.md](/docs/de/memory#exclude-specific-claude-md-files)-Dateien beim Laden des Gedächtnisses                                                                                                                                             | Memory and context                 | Any file                |
| [`cleanupPeriodDays`](#cleanupperioddays)                                                             | Wählen Sie, wie viele Tage Claude Code [Transkripte](/docs/de/data-usage#data-retention) behält, bevor sie gelöscht werden                                                                                                                                                  | Privacy and telemetry              | Any file                |
| [`companyAnnouncements`](#companyannouncements)                                                       | Zeigen Sie die Ankündigungen Ihrer Organisation beim Start an                                                                                                                                                                                                          | Interface and terminal             | Any file                |
| [`copyOnSelect`](#copyonselect)                                                                       | Schalten Sie das automatische Kopieren von Text aus, den Sie mit der Maus im [Vollbild-Rendering](/docs/de/fullscreen#use-the-mouse) und in der Agent-Ansicht auswählen                                                                                                     | Global config settings             | Global config           |
| [`crossSessionInbound`](#crosssessioninbound)                                                         | Wählen Sie, ob Claude Code [Nachrichten von Ihren anderen Sitzungen](/docs/de/cross-session-messaging#control-inbound-messages) liefert, einen Hinweis anzeigt, ohne sie zu liefern, oder sie ablehnt                                                                       | Agents, sessions, and worktrees    | Any file                |
| [`defaultShell`](#defaultshell)                                                                       | Wählen Sie, ob Bash oder PowerShell die Shell-Befehle ausführt, die Sie mit dem [`!`-Präfix](/docs/de/interactive-mode#shell-mode-with-prefix) eingeben                                                                                                                     | Interface and terminal             | Any file                |
| [`deniedMcpServers`](#deniedmcpservers)                                                               | Blockieren Sie spezifische [MCP-Server](/docs/de/mcp) nach URL, Befehl oder Name                                                                                                                                                                                            | MCP                                | Any file                |
| [`desktopSessionCleanupPeriodDays`](#desktopsessioncleanupperioddays)                                 | Legen Sie ein Alterslimit in Tagen für [Claude Desktop- und Cowork-Transkripte](/docs/de/claude-directory#cleaned-up-automatically) fest                                                                                                                                    | Privacy and telemetry              | User or managed         |
| [`dialogExpiry`](#dialogexpiry)                                                                       | Legen Sie fest, wie lange Claude Code auf eine Antwort von [Remote Control](/docs/de/remote-control) oder einem SDK-Host auf einen weitergeleitet Dialog wartet, bevor der Dialog abgebrochen wird                                                                          | Interface and terminal             | User or managed         |
| [`diffTool`](#difftool)                                                                               | Wählen Sie, ob Claudes vorgeschlagene Dateiänderungen im [VS Code](/docs/de/vs-code)- oder [JetBrains](/docs/de/jetbrains#features)-Diff-Viewer geöffnet werden oder im Terminal bleiben                                                                                         | Global config settings             | Global config           |
| [`disableAgentView`](#disableagentview)                                                               | Schalten Sie Hintergrund-Agenten und [Agent-Ansicht](/docs/de/agent-view) aus                                                                                                                                                                                               | Agents, sessions, and worktrees    | Any file                |
| [`disableAllHooks`](#disableallhooks)                                                                 | Schalten Sie [Hooks](/docs/de/hooks), eine benutzerdefinierte [Statuszeile](/docs/de/statusline) und einen benutzerdefinierten [`@`-Dateivorschlag](/docs/de/interactive-mode#quick-commands)-Befehl auf einmal aus                                                                   | Hooks and automation               | Any file                |
| [`disableArtifact`](#disableartifact)                                                                 | Veraltet; verwenden Sie `enableArtifact`, um das [Artifact-Tool](/docs/de/artifacts) auszuschalten                                                                                                                                                                          | Remote, desktop, and notifications | Any file                |
| [`disableAutoMode`](#disableautomode)                                                                 | Entfernen Sie [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) aus dem Berechtigungsmodus-Zyklus                                                                                                                                                    | Permission settings                | Any file                |
| [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation)                               | Beschränken Sie den [Desktop](/docs/de/desktop)-Browser-Bereich auf localhost für Personen und Claude                                                                                                                                                                       | Tools                              | Managed                 |
| [`disableBundledSkills`](#disablebundledskills)                                                       | Schalten Sie die [Skills](/docs/de/skills#bundled-skills) und [Workflows](/docs/de/workflows) aus, die mit Claude Code enthalten sind                                                                                                                                            | Plugins and skills                 | Any file                |
| [`disableClaudeAiConnectors`](#disableclaudeaiconnectors)                                             | Schalten Sie [claude.ai-Konnektoren](/docs/de/mcp#disable-claude-ai-connectors) aus, damit Claude Code sie nicht abruft                                                                                                                                                     | MCP                                | Any file                |
| [`disableCommandPluginSources`](#disablecommandpluginsources)                                         | Blockieren Sie [Plugins](/docs/de/plugins/overview), die durch Ausführung eines vom Marketplace deklarierten Befehls installiert werden                                                                                                                                     | Plugins and skills                 | Managed                 |
| [`disableDeepLinkRegistration`](#disabledeeplinkregistration)                                         | Verhindern Sie, dass Claude Code den [`claude-cli://`-Handler](/docs/de/deep-links) registriert                                                                                                                                                                             | Remote, desktop, and notifications | Any file                |
| [`disableDesktopLocalSessions`](#disabledesktoplocalsessions)                                         | Schalten Sie [Desktop Code-Sitzungen](/docs/de/desktop#local-sessions-on-managed-devices) aus, die auf dem Gerät ausgeführt werden, und lassen Sie SSH zu anderen Hosts und Cloud                                                                                           | Remote, desktop, and notifications | Managed                 |
| [`disabledMcpjsonServers`](#disabledmcpjsonservers)                                                   | Lehnen Sie spezifische Server aus der [`.mcp.json`](/docs/de/mcp#project-scope) eines Projekts ab                                                                                                                                                                           | MCP                                | Any file                |
| [`disableMobileSimulatorTools`](#disablemobilesimulatortools)                                         | Blockieren Sie Claudes Tools im [Desktop](/docs/de/desktop)-iOS-Simulator-Bereich                                                                                                                                                                                           | Tools                              | Managed                 |
| [`disableRemoteControl`](#disableremotecontrol)                                                       | Schalten Sie [Remote Control](/docs/de/remote-control) überall dort aus, wo es starten kann                                                                                                                                                                                 | Remote, desktop, and notifications | Any file                |
| [`disableSideloadFlags`](#disablesideloadflags)                                                       | Lehnen Sie die CLI-Flags ab, die [Plugins](/docs/de/plugins/overview), [Subagenten](/docs/de/sub-agents) und [MCP-Server](/docs/de/mcp) sideloaden                                                                                                                                    | Enterprise and managed settings    | Managed                 |
| [`disableSkillShellExecution`](#disableskillshellexecution)                                           | Verhindern Sie, dass [Skills](/docs/de/skills) und benutzerdefinierte Befehle Inline-Shell ausführen                                                                                                                                                                        | Plugins and skills                 | Any file                |
| [`disableWorkflows`](#disableworkflows)                                                               | Schalten Sie [dynamische Workflows](/docs/de/workflows) für alle aus; verwenden Sie `enableWorkflows` für sich selbst                                                                                                                                                       | Hooks and automation               | Any file                |
| [`editorMode`](#editormode)                                                                           | Verwenden Sie [vim-Tastenbindungen](/docs/de/interactive-mode#vim-editor-mode) in der Eingabeaufforderung                                                                                                                                                                   | Interface and terminal             | Any file                |
| [`effortLevel`](#effortlevel)                                                                         | Legen Sie eine Standard-[Anstrengungsstufe](/docs/de/model-config#adjust-effort-level) für Modelle ohne eine gespeicherte Stufe fest                                                                                                                                        | Model and responses                | Any file                |
| [`emojiCompletionEnabled`](#emojicompletionenabled)                                                   | Schalten Sie [`:shortcode:`-Emoji-Vorschläge und -Ersetzung](/docs/de/interactive-mode#emoji-shortcodes) in der Eingabeaufforderung aus                                                                                                                                     | Interface and terminal             | Any file                |
| [`enableAllProjectMcpServers`](#enableallprojectmcpservers)                                           | Genehmigen Sie jeden Server in Projekt-[`.mcp.json`](/docs/de/mcp#project-server-approvals-and-workspace-trust)-Dateien ohne Aufforderung                                                                                                                                   | MCP                                | Any file                |
| [`enableArtifact`](#enableartifact)                                                                   | Schalten Sie das [Artifact-Tool](/docs/de/artifacts) mit einem `false` in einer beliebigen Datei aus; keine Datei kann es wieder einschalten                                                                                                                                | Remote, desktop, and notifications | Any file                |
| [`enabledMcpjsonServers`](#enabledmcpjsonservers)                                                     | Genehmigen Sie spezifische Server aus der [`.mcp.json`](/docs/de/mcp#project-server-approvals-and-workspace-trust) eines Projekts                                                                                                                                           | MCP                                | Any file                |
| [`enabledPlugins`](#enabledplugins)                                                                   | Schalten Sie einzelne [Plugins](/docs/de/plugins/overview) pro Bereich ein oder aus                                                                                                                                                                                         | Plugins and skills                 | Any file                |
| [`enableWorkflows`](#enableworkflows)                                                                 | Schalten Sie [dynamische Workflows](/docs/de/workflows) gegen den Standard Ihres Plans ein oder aus                                                                                                                                                                         | Hooks and automation               | Any file                |
| [`enforceAvailableModels`](#enforceavailablemodels)                                                   | Halten Sie die [`/model`-Standardauswahl](/docs/de/model-config#enforce-the-allowlist-for-the-default-model) innerhalb Ihrer `availableModels`-Zulassungsliste                                                                                                              | Model and responses                | Any file                |
| [`env`](#env)                                                                                         | Legen Sie [Umgebungsvariablen](/docs/de/env-vars#in-settings-files) für jede Sitzung und ihre Unterprozesse fest                                                                                                                                                            | Memory and context                 | Any file                |
| [`externalEditorContext`](#externaleditorcontext)                                                     | Zeigen Sie Claudes letzte Antwort als Kommentare an, wenn Sie [Strg+G](/docs/de/interactive-mode#general-controls) drücken, um zu bearbeiten                                                                                                                                | Global config settings             | Global config           |
| [`extraKnownMarketplaces`](#extraknownmarketplaces)                                                   | Registrieren Sie [Marketplaces](/docs/de/plugins/overview) für ein Repository oder eine Organisation                                                                                                                                                                        | Plugins and skills                 | Any file                |
| [`fallbackModel`](#fallbackmodel)                                                                     | Benennen Sie [Backup-Modelle](/docs/de/model-config#fallback-model-chains) für den Fall, dass das primäre überlastet ist                                                                                                                                                    | Model and responses                | Any file                |
| [`fastMode`](#fastmode)                                                                               | Schalten Sie [Fast-Modus](/docs/de/fast-mode) für Sitzungen ein, in denen er verfügbar ist                                                                                                                                                                                  | Model and responses                | Any file                |
| [`fastModePerSessionOptIn`](#fastmodepersessionoptin)                                                 | Erfordern Sie, dass Personen [Fast-Modus](/docs/de/fast-mode) in jeder Sitzung einschalten                                                                                                                                                                                  | Model and responses                | Any file                |
| [`feedbackDrafts`](#feedbackdrafts)                                                                   | Kontrollieren Sie, ob Claude [Feedback-Entwürfe](/docs/de/tools-reference#sendfeedback-tool-behavior) für Sie zur Überprüfung in die Warteschlange einreiht                                                                                                                 | Privacy and telemetry              | User or managed         |
| [`feedbackSurveyRate`](#feedbacksurveyrate)                                                           | Ändern Sie, wie oft die [Sitzungsqualitätsumfrage](/docs/de/data-usage#session-quality-surveys) angezeigt wird                                                                                                                                                              | Privacy and telemetry              | Any file                |
| [`fileCheckpointingEnabled`](#filecheckpointingenabled)                                               | Schalten Sie die Datei-Snapshots aus oder ein, die [`/rewind`](/docs/de/checkpointing) wiederherstellt                                                                                                                                                                      | Memory and context                 | Any file                |
| [`fileSuggestion`](#filesuggestion)                                                                   | Stellen Sie [`@`-Datei-Autovervollständigung](/docs/de/interactive-mode#quick-commands) aus Ihrem eigenen Befehl bereit                                                                                                                                                     | Interface and terminal             | Any file                |
| [`footerLinksRegexes`](#footerlinksregexes)                                                           | Machen Sie Problem- oder Review-IDs in der Ausgabe zu [anklickbaren Links](/docs/de/statusline#clickable-links) unter dem Eingabefeld                                                                                                                                       | Interface and terminal             | User or managed         |
| [`forceLoginGatewayUrl`](#forcelogingatewayurl)                                                       | Legen Sie die [Gateway-URL](/docs/de/claude-apps-gateway#set-the-gateway-url) fest, mit der sich der Anmeldebildschirm verbindet                                                                                                                                            | Authentication and providers       | Managed                 |
| [`forceLoginMethod`](#forceloginmethod)                                                               | [Beschränken Sie die Anmeldung](/docs/de/authentication#restrict-login-to-your-organization) auf claude.ai, Claude Console oder ein [Cloud-Gateway](/docs/de/claude-apps-gateway)                                                                                                | Authentication and providers       | Any file                |
| [`forceLoginOrgUUID`](#forceloginorguuid)                                                             | [Heften Sie claude.ai-Anmeldungen an Ihre Organisation](/docs/de/authentication#restrict-login-to-your-organization); nur eine verwaltete Quelle erzwingt dies                                                                                                              | Authentication and providers       | Any file                |
| [`forceRemoteSettingsRefresh`](#forceremotesettingsrefresh)                                           | Blockieren Sie den Start, bis [Server-verwaltete Einstellungen](/docs/de/server-managed-settings) frisch abgerufen werden                                                                                                                                                   | Enterprise and managed settings    | Managed                 |
| [`gatewayInternalNetworks`](#gatewayinternalnetworks)                                                 | Lassen Sie `/login` ein [Cloud-Gateway](/docs/de/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) auf öffentlichem IPv4-Adressraum erreichen, den Ihre Organisation intern nutzt                                                                        | Authentication and providers       | Managed                 |
| [`gcpAuthRefresh`](#gcpauthrefresh)                                                                   | Aktualisieren Sie [Google Cloud-Anmeldedaten](/docs/de/google-vertex-ai#advanced-credential-configuration) mit Ihrem eigenen Befehl                                                                                                                                         | Authentication and providers       | Any file                |
| [`hooks`](#hooks)                                                                                     | Führen Sie Ihre eigenen Befehle als [Hooks](/docs/de/hooks) an Punkten im Lebenszyklus von Claude Code aus                                                                                                                                                                  | Hooks and automation               | Any file                |
| [`httpHookAllowedEnvVars`](#httphookallowedenvvars)                                                   | Begrenzen Sie, welche Umgebungsvariablen [HTTP-Hooks](/docs/de/hooks) in Header einfügen können                                                                                                                                                                             | Hooks and automation               | Any file                |
| [`includeCoAuthoredBy`](#includecoauthoredby)                                                         | Veraltet; verwenden Sie `attribution`, um Commit- und PR-Zuschreibung zu verbergen oder zu ändern                                                                                                                                                                      | Git and attribution                | Any file                |
| [`includeGitInstructions`](#includegitinstructions)                                                   | Entfernen Sie die integrierten Commit- und PR-Anweisungen aus Claudes Kontext                                                                                                                                                                                          | Git and attribution                | Any file                |
| [`inputNeededNotifEnabled`](#inputneedednotifenabled)                                                 | Erhalten Sie eine [Push-Benachrichtigung](/docs/de/remote-control#mobile-push-notifications), wenn Claude auf Sie wartet                                                                                                                                                    | Remote, desktop, and notifications | Any file                |
| [`isolatePeerMachines`](#isolatepeermachines)                                                         | Fragen Sie, bevor Claude [eine Ihrer Sitzungen auf einem anderen Computer](/docs/de/cross-session-messaging#require-approval-for-cross-machine-messages) benachrichtigt                                                                                                     | Agents, sessions, and worktrees    | Any file                |
| [`keybindingFlavor`](#keybindingflavor)                                                               | Veraltet und hat keine Auswirkung; die Wort-Bearbeitungs-Tastenkombinationen folgen immer [readline-Konventionen](/docs/de/interactive-mode#make-ctrl-w-delete-back-to-whitespace)                                                                                          | Interface and terminal             | Any file                |
| [`language`](#language)                                                                               | Lassen Sie Claude in einer anderen Sprache als Englisch antworten                                                                                                                                                                                                      | Model and responses                | Any file                |
| [`managedMcpServers`](#managedmcpservers)                                                             | Stellen Sie Remote-[MCP-Server](/docs/de/managed-mcp#provide-servers-through-managed-settings) für jeden Benutzer zusammen mit den von ihm hinzugefügten bereit                                                                                                             | MCP                                | Managed                 |
| [`managedSourcesBehavior`](#managedsourcesbehavior)                                                   | Kombinieren Sie jede [verwaltete Quelle](/docs/de/managed-settings#how-claude-code-combines-managed-sources), die Sie bereitstellen, anstatt nur die mit der höchsten Priorität zu verwenden                                                                                | Enterprise and managed settings    | Managed                 |
| [`maxEffortLevel`](#maxeffortlevel)                                                                   | Begrenzen Sie die [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level) für jedes Modell oder pro Modell auf jedem Anbieter                                                                                                                                        | Model and responses                | Any file                |
| [`minimumVersion`](#minimumversion)                                                                   | Halten Sie [Auto-Updates](/docs/de/setup#pin-a-minimum-version) davon ab, etwas unter einer Version zu installieren                                                                                                                                                         | Updates and versioning             | Any file                |
| [`model`](#model)                                                                                     | Ändern Sie das [Modell](/docs/de/model-config#set-a-default-model-for-new-sessions), mit dem Claude Code startet                                                                                                                                                            | Model and responses                | Any file                |
| [`modelOverrides`](#modeloverrides)                                                                   | [Ordnen Sie Modell-IDs](/docs/de/model-config#override-model-ids-per-version) den IDs Ihres Anbieters zu, wie z. B. Bedrock-ARNs                                                                                                                                            | Model and responses                | Any file                |
| [`modelPicker`](#modelpicker)                                                                         | Wählen Sie, welche Modelle der [`/model`-Picker](/docs/de/model-config#available-models) auflistet, in Ihrer eigenen Reihenfolge und mit Ihren eigenen Labels                                                                                                               | Model and responses                | User or managed         |
| [`modelPricing`](#modelpricing)                                                                       | Melden Sie Ausgaben zu den vertraglich vereinbarten Sätzen Ihrer Organisation statt zum Listenpreis                                                                                                                                                                    | Model and responses                | Managed                 |
| [`modelSettings`](#modelsettings)                                                                     | Behalten Sie eine gespeicherte [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level) pro Modell bei, oder begrenzen Sie die Anstrengung eines Modells                                                                                                              | Model and responses                | Any file                |
| [`otelHeadersHelper`](#otelheadershelper)                                                             | Generieren Sie rotierende [OpenTelemetry](/docs/de/monitoring-usage#dynamic-headers)-Header mit Ihrem eigenen Befehl                                                                                                                                                        | Authentication and providers       | Any file                |
| [`outputStyle`](#outputstyle)                                                                         | Ändern Sie Claudes Rolle, Ton und Ausgabeformat mit einem [Ausgabestil](/docs/de/output-styles)                                                                                                                                                                             | Model and responses                | Any file                |
| [`parentSettingsBehavior`](#parentsettingsbehavior)                                                   | Wenden Sie Einschränkungen an oder verwerfen Sie sie, die ein [SDK- oder IDE-Host](/docs/de/managed-settings#let-an-embedding-host-add-policy) übergibt, wenn Sie [verwaltete Einstellungen](/docs/de/managed-settings) bereitstellen                                            | Enterprise and managed settings    | Managed                 |
| [`permissionExplainerEnabled`](#permissionexplainerenabled)                                           | Entfernt in v2.1.257, zusammen mit der `Strg+E`-Befehlserklärung auf Shell-Berechtigungsaufforderungen                                                                                                                                                                 | Global config settings             | Global config           |
| [`permissions`](#permissions)                                                                         | Legen Sie Allow-, Ask- und Deny-Regeln sowie den Start-[Berechtigungsmodus](/docs/de/permission-modes) fest                                                                                                                                                                 | Permission settings                | Any file                |
| [`permissions.additionalDirectories`](#permissions-additionaldirectories)                             | Geben Sie Claude Dateizugriff auf [Verzeichnisse außerhalb des aktuellen](/docs/de/permissions#working-directories)                                                                                                                                                         | Permission settings                | Any file                |
| [`permissions.allow`](#permissions-allow)                                                             | Genehmigen Sie aufgelistete [Tool-Verwendungen](/docs/de/permissions#permission-rule-syntax) ohne Aufforderung                                                                                                                                                              | Permission settings                | Any file                |
| [`permissions.ask`](#permissions-ask)                                                                 | Fragen Sie immer vor aufgelisteten [Tool-Verwendungen](/docs/de/permissions#permission-rule-syntax)                                                                                                                                                                         | Permission settings                | Any file                |
| [`permissions.blockReadsOutsideWorkingDirectories`](#permissions-blockreadsoutsideworkingdirectories) | Machen Sie die Datei-Tools, um Lesevorgänge außerhalb der [Arbeitsverzeichnisse](/docs/de/permissions#working-directories) in jedem Berechtigungsmodus zu verweigern                                                                                                        | Permission settings                | Any file                |
| [`permissions.defaultMode`](#permissions-defaultmode)                                                 | Legen Sie den [Berechtigungsmodus](/docs/de/permission-modes#which-mode-a-session-starts-in) fest, in dem neue Sitzungen starten                                                                                                                                            | Permission settings                | Any file                |
| [`permissions.deny`](#permissions-deny)                                                               | Blockieren Sie aufgelistete [Tool-Verwendungen](/docs/de/permissions#permission-rule-syntax), einschließlich Lesevorgänge von Dateien, die Geheimnisse enthalten                                                                                                            | Permission settings                | Any file                |
| [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode)               | Verhindern Sie, dass jemand den [bypassPermissions-Modus](/docs/de/permission-modes#skip-all-checks-with-bypasspermissions-mode) betritt                                                                                                                                    | Permission settings                | Any file                |
| [`plansDirectory`](#plansdirectory)                                                                   | Wählen Sie, wo [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) Plan-Dateien schreibt                                                                                                                                                         | Memory and context                 | Any file                |
| [`pluginConfigs`](#pluginconfigs)                                                                     | Speichern Sie die Antworten, die Sie dem Konfigurationsdialog eines [Plugins](/docs/de/plugins/overview) gegeben haben                                                                                                                                                      | Plugins and skills                 | User or managed         |
| [`pluginSuggestionMarketplaces`](#pluginsuggestionmarketplaces)                                       | Wählen Sie, welche [Marketplaces](/docs/de/plugins/overview) Plugin-Installationsvorschläge in `/plugin` anzeigen können                                                                                                                                                    | Plugins and skills                 | Managed                 |
| [`pluginTrustMessage`](#plugintrustmessage)                                                           | Fügen Sie Ihren eigenen Text zur [Plugin](/docs/de/plugins/overview)-Vertrauenswarnung hinzu                                                                                                                                                                                | Plugins and skills                 | Managed                 |
| [`policyHelper`](#policyhelper)                                                                       | Führen Sie eine ausführbare Datei aus, die [verwaltete Einstellungen](/docs/de/managed-settings#compute-the-policy-with-a-helper-program) beim Start berechnet                                                                                                              | Enterprise and managed settings    | Managed                 |
| [`policyHelper.path`](#policyhelper-path)                                                             | Benennen Sie die [Helper-Ausführungsdatei](/docs/de/managed-settings#compute-the-policy-with-a-helper-program), die Claude Code ausführt                                                                                                                                    | Enterprise and managed settings    | Managed                 |
| [`policyHelper.refreshIntervalMs`](#policyhelper-refreshintervalms)                                   | Führen Sie den [Helper](/docs/de/managed-settings#compute-the-policy-with-a-helper-program) im Hintergrund in einem Intervall erneut aus                                                                                                                                    | Enterprise and managed settings    | Managed                 |
| [`policyHelper.timeoutMs`](#policyhelper-timeoutms)                                                   | Legen Sie fest, wie lange Claude Code auf den [Helper](/docs/de/managed-settings#compute-the-policy-with-a-helper-program) wartet                                                                                                                                           | Enterprise and managed settings    | Managed                 |
| [`preferredNotifChannel`](#preferrednotifchannel)                                                     | Wählen Sie einen [Terminal-Gong oder Desktop-Benachrichtigung](/docs/de/terminal-config#get-a-terminal-bell-or-notification) für die Aufgabenvollendung                                                                                                                     | Remote, desktop, and notifications | Any file                |
| [`prefersReducedMotion`](#prefersreducedmotion)                                                       | [Reduzieren oder schalten Sie](/docs/de/accessibility#accessibility-settings) Spinner-, Shimmer- und Flash-Animationen aus                                                                                                                                                  | Interface and terminal             | Any file                |
| [`processWrapper`](#processwrapper)                                                                   | Führen Sie die Hintergrundprozesse von Claude Code durch einen [Corporate Launcher](/docs/de/corporate-launcher) auf macOS und Linux aus                                                                                                                                    | Agents, sessions, and worktrees    | User or managed         |
| [`promptCacheTtl`](#promptcachettl)                                                                   | Wählen Sie die [Prompt-Cache-Lebensdauer](/docs/de/prompt-caching#cache-lifetime) für die Hauptkonversation                                                                                                                                                                 | Model and responses                | Any file                |
| [`promptSuggestionEnabled`](#promptsuggestionenabled)                                                 | Verbergen Sie die ausgegraut [Prompt-Vorschläge](/docs/de/interactive-mode#prompt-suggestions) im Eingabefeld                                                                                                                                                               | Interface and terminal             | Any file                |
| [`prUrlTemplate`](#prurltemplate)                                                                     | Zeigen Sie PR-Links auf ein internes Code-Review-Tool statt auf github.com                                                                                                                                                                                             | Git and attribution                | Any file                |
| [`remote.defaultEnvironmentId`](#remote-defaultenvironmentid)                                         | Wählen Sie die Standard-[Cloud-Umgebung](/docs/de/cloud-environments) für `claude --cloud`; eine selbstgehostete `ccpool_`-ID wird nur aus Benutzer- und verwalteten Einstellungen und `--settings` gelesen                                                                 | Remote, desktop, and notifications | Any file                |
| [`remoteControlAtStartup`](#remotecontrolatstartup)                                                   | Verbinden Sie [Remote Control](/docs/de/remote-control#enable-remote-control-for-all-sessions) automatisch, wenn eine Sitzung startet                                                                                                                                       | Remote, desktop, and notifications | Any file                |
| [`requiredMaximumVersion`](#requiredmaximumversion)                                                   | [Weigern Sie sich zu starten](/docs/de/setup#pin-a-minimum-version) auf einer Version, die Ihre Organisation nicht zulässt                                                                                                                                                  | Updates and versioning             | Managed                 |
| [`requiredMinimumVersion`](#requiredminimumversion)                                                   | [Weigern Sie sich zu starten](/docs/de/setup#pin-a-minimum-version) auf einer Version, die älter ist als Ihre Organisation erfordert                                                                                                                                        | Updates and versioning             | Managed                 |
| [`respectGitignore`](#respectgitignore)                                                               | Halten Sie ignorierte Dateien aus dem [`@`-Datei-Picker](/docs/de/interactive-mode#quick-commands)                                                                                                                                                                          | Interface and terminal             | Any file                |
| [`respondToBashCommands`](#respondtobashcommands)                                                     | Verhindern Sie, dass Claude nach einem [`!`-Shell-Befehl](/docs/de/interactive-mode#shell-mode-with-prefix) antwortet                                                                                                                                                       | Interface and terminal             | Any file                |
| [`sandbox`](#sandbox)                                                                                 | [Isolieren Sie Bash-Befehle](/docs/de/sandboxing) von Ihrem Dateisystem und Netzwerk auf macOS, Linux und WSL2                                                                                                                                                              | Sandbox settings                   | Any file                |
| [`sandbox.allowAppleEvents`](#sandbox-allowappleevents)                                               | Lassen Sie [sandboxed](/docs/de/sandboxing)-Befehle Apple Events auf macOS senden                                                                                                                                                                                           | Sandbox settings                   | User or managed         |
| [`sandbox.allowUnsandboxedCommands`](#sandbox-allowunsandboxedcommands)                               | Lassen Sie Claude einen blockierten Befehl außerhalb der [Sandbox](/docs/de/sandboxing#the-unsandboxed-retry-escape-hatch) erneut versuchen, oder verbieten Sie ihn                                                                                                         | Sandbox settings                   | Any file                |
| [`sandbox.autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed)                               | Führen Sie [sandboxed](/docs/de/sandboxing#auto-allow-mode)-Befehle ohne Berechtigungsaufforderung aus                                                                                                                                                                      | Sandbox settings                   | Any file                |
| [`sandbox.bwrapPath`](#sandbox-bwrappath)                                                             | Zeigen Sie die [Sandbox](/docs/de/sandboxing) auf eine Bubblewrap-Binärdatei außerhalb von `PATH`                                                                                                                                                                           | Sandbox settings                   | Managed                 |
| [`sandbox.credentials`](#sandbox-credentials)                                                         | Verbergen oder maskieren Sie Anmeldedatei- und Variablen in der [Sandbox](/docs/de/sandboxing#protect-credentials)                                                                                                                                                          | Sandbox settings                   | Any file                |
| [`sandbox.credentials.allowPlaintextInject`](#sandbox-credentials-allowplaintextinject)               | Lassen Sie [maskierte Anmeldedaten](/docs/de/sandboxing#mask-credentials) Plain-HTTP-Dienste auf vertrauenswürdigen Test-Netzwerken erreichen                                                                                                                               | Sandbox settings                   | User or managed         |
| [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs)                                       | Verknüpfen Sie benutzerdefiniert benannte AWS-Schlüsselvariablen in eine Anmeldedaten für [Neusignierung](/docs/de/sandboxing#re-sign-aws-requests)                                                                                                                         | Sandbox settings                   | User or managed         |
| [`sandbox.credentials.envVars`](#sandbox-credentials-envvars)                                         | Heben Sie die Einstellung auf oder maskieren Sie eine Umgebungsvariable in der [Sandbox](/docs/de/sandboxing#mask-environment-variables)                                                                                                                                    | Sandbox settings                   | Any file                |
| [`sandbox.credentials.files`](#sandbox-credentials-files)                                             | Blockieren oder maskieren Sie Lesevorgänge einer Anmeldedatei in der [Sandbox](/docs/de/sandboxing#mask-credential-files)                                                                                                                                                   | Sandbox settings                   | Any file                |
| [`sandbox.credentials.sigv4`](#sandbox-credentials-sigv4)                                             | Wählen Sie, ob Streaming-, Presigned- oder [SigV4A-AWS-Anfragen](/docs/de/sandboxing#re-sign-aws-requests) fehlschlagen oder durchgehen                                                                                                                                     | Sandbox settings                   | User or managed         |
| [`sandbox.enabled`](#sandbox-enabled)                                                                 | Schalten Sie [Bash-Sandboxing](/docs/de/sandboxing#get-started) auf macOS, Linux und WSL2 ein                                                                                                                                                                               | Sandbox settings                   | Any file                |
| [`sandbox.enableWeakerNestedSandbox`](#sandbox-enableweakernestedsandbox)                             | Führen Sie die Linux-[Sandbox](/docs/de/sandboxing) in einem unprivilegierten Container aus                                                                                                                                                                                 | Sandbox settings                   | Any file                |
| [`sandbox.enableWeakerNetworkIsolation`](#sandbox-enableweakernetworkisolation)                       | Lassen Sie `gh`, `gcloud` und `terraform` TLS hinter einem MITM-Proxy in der [Sandbox](/docs/de/sandboxing#troubleshooting) auf macOS überprüfen                                                                                                                            | Sandbox settings                   | Any file                |
| [`sandbox.excludedCommands`](#sandbox-excludedcommands)                                               | Benennen Sie Befehle, die Claude Code außerhalb der [Sandbox](/docs/de/sandboxing) ausführen kann                                                                                                                                                                           | Sandbox settings                   | Any file                |
| [`sandbox.failIfUnavailable`](#sandbox-failifunavailable)                                             | Weigern Sie sich zu starten, wenn die [Sandbox](/docs/de/sandboxing) nicht kann, anstatt unsandboxed auszuführen                                                                                                                                                            | Sandbox settings                   | Any file                |
| [`sandbox.filesystem`](#sandbox-filesystem)                                                           | Kontrollieren Sie, welche Pfade [sandboxed](/docs/de/sandboxing#filesystem-isolation)-Befehle lesen und schreiben können                                                                                                                                                    | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly)       | Verhindern Sie, dass Entwickler [Lesepfade, die Ihre Organisation blockiert hat,](/docs/de/sandboxing#keep-developers-from-widening-the-policy) erneut öffnen                                                                                                               | Sandbox settings                   | Managed                 |
| [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread)                                       | Öffnen Sie das Lesen erneut in einem Bereich, den [`denyRead`](#sandbox-filesystem-denyread) blockiert                                                                                                                                                                 | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.allowWrite`](#sandbox-filesystem-allowwrite)                                     | Fügen Sie Pfade hinzu, in die [sandboxed](/docs/de/sandboxing)-Befehle schreiben können                                                                                                                                                                                     | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread)                                         | Blockieren Sie [sandboxed](/docs/de/sandboxing)-Befehle vom Lesen spezifischer Pfade                                                                                                                                                                                        | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.denyWrite`](#sandbox-filesystem-denywrite)                                       | Blockieren Sie [sandboxed](/docs/de/sandboxing)-Befehle vom Schreiben in spezifische Pfade                                                                                                                                                                                  | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.disabled`](#sandbox-filesystem-disabled)                                         | [Schalten Sie die Dateisystem-Isolation aus](/docs/de/sandboxing#disable-filesystem-isolation), während Sie die Netzwerk-Isolation beibehalten                                                                                                                              | Sandbox settings                   | User or managed         |
| [`sandbox.ignoreViolations`](#sandbox-ignoreviolations)                                               | Stummschalten Sie Verletzungsberichte für Pfade, die ein Befehl voraussichtlich prüft                                                                                                                                                                                  | Sandbox settings                   | Any file                |
| [`sandbox.network`](#sandbox-network)                                                                 | Kontrollieren Sie, welche Hosts, Ports und Sockets [sandboxed](/docs/de/sandboxing#network-isolation)-Befehle erreichen                                                                                                                                                     | Sandbox settings                   | Any file                |
| [`sandbox.network.allowAllUnixSockets`](#sandbox-network-allowallunixsockets)                         | Lassen Sie [sandboxed](/docs/de/sandboxing)-Befehle sich mit jedem Unix-Socket verbinden                                                                                                                                                                                    | Sandbox settings                   | Any file                |
| [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains)                                   | Genehmigen Sie Domänen im Voraus, damit [sandboxed](/docs/de/sandboxing)-Befehle nicht danach fragen                                                                                                                                                                        | Sandbox settings                   | Any file                |
| [`sandbox.network.allowLocalBinding`](#sandbox-network-allowlocalbinding)                             | Lassen Sie [sandboxed](/docs/de/sandboxing)-Befehle sich an localhost-Ports auf macOS binden                                                                                                                                                                                | Sandbox settings                   | Any file                |
| [`sandbox.network.allowMachLookup`](#sandbox-network-allowmachlookup)                                 | Lassen Sie macOS-[sandboxed](/docs/de/sandboxing)-Tools wie den iOS Simulator oder Playwright ihre XPC-Dienste erreichen                                                                                                                                                    | Sandbox settings                   | Any file                |
| [`sandbox.network.allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly)                 | Sperren Sie die Netzwerk-Zulassungsliste auf [verwaltete Einstellungen](/docs/de/sandboxing#keep-developers-from-widening-the-policy)                                                                                                                                       | Sandbox settings                   | Managed                 |
| [`sandbox.network.allowUnixSockets`](#sandbox-network-allowunixsockets)                               | Listen Sie Unix-Socket-Pfade auf, die [sandboxed](/docs/de/sandboxing)-Befehle auf macOS verwenden können                                                                                                                                                                   | Sandbox settings                   | Any file                |
| [`sandbox.network.deniedDomains`](#sandbox-network-denieddomains)                                     | Blockieren Sie Domänen für [sandboxed](/docs/de/sandboxing)-Befehle, auch innerhalb eines zulässigen Wildcards                                                                                                                                                              | Sandbox settings                   | Any file                |
| [`sandbox.network.httpProxyPort`](#sandbox-network-httpproxyport)                                     | Leiten Sie [Sandbox](/docs/de/sandboxing#custom-proxy-configuration)-HTTP-Verkehr durch Ihren eigenen Proxy                                                                                                                                                                 | Sandbox settings                   | Any file                |
| [`sandbox.network.socksProxyPort`](#sandbox-network-socksproxyport)                                   | Leiten Sie [Sandbox](/docs/de/sandboxing#custom-proxy-configuration)-SOCKS-Verkehr durch Ihren eigenen Proxy                                                                                                                                                                | Sandbox settings                   | Any file                |
| [`sandbox.network.strictAllowlist`](#sandbox-network-strictallowlist)                                 | Lehnen Sie Hosts außerhalb der [Zulassungsliste](/docs/de/sandboxing#network-isolation) ab, anstatt zu fragen                                                                                                                                                               | Sandbox settings                   | User or managed         |
| [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate)                                       | Lassen Sie die [Sandbox](/docs/de/sandboxing#network-isolation) TLS beenden, damit sie HTTPS-Anfragen lesen kann                                                                                                                                                            | Sandbox settings                   | User or managed         |
| [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                 | Verwenden Sie Ihre eigene ripgrep-Binärdatei in der [Sandbox](/docs/de/sandboxing)                                                                                                                                                                                          | Sandbox settings                   | User or managed         |
| [`sandbox.socatPath`](#sandbox-socatpath)                                                             | Zeigen Sie den [Sandbox](/docs/de/sandboxing)-Proxy auf eine `socat`-Binärdatei außerhalb von `PATH`                                                                                                                                                                        | Sandbox settings                   | Managed                 |
| [`showClearContextOnPlanAccept`](#showclearcontextonplanaccept)                                       | Zeigen Sie eine "Kontext löschen"-Option auf dem [Plan-Akzeptanzbildschirm](/docs/de/permission-modes#review-and-approve-a-plan)                                                                                                                                            | Interface and terminal             | Any file                |
| [`showThinkingSummaries`](#showthinkingsummaries)                                                     | Sehen Sie Zusammenfassungen von Claudes [Denken](/docs/de/model-config#extended-thinking) statt eines zusammengeklappten Stubs                                                                                                                                              | Model and responses                | Any file                |
| [`showTurnDuration`](#showturnduration)                                                               | Verbergen Sie die "Gekocht für"-Dauer nach jeder Antwort                                                                                                                                                                                                               | Interface and terminal             | Any file                |
| [`skillListingBudgetFraction`](#skilllistingbudgetfraction)                                           | Reservieren Sie mehr oder weniger Kontext für die [Skill-Auflistung](/docs/de/skills#skill-descriptions-are-cut-short)                                                                                                                                                      | Memory and context                 | Any file                |
| [`skillListingMaxDescChars`](#skilllistingmaxdescchars)                                               | Begrenzen Sie die Beschreibungslänge jedes Skills in der [Skill-Auflistung](/docs/de/skills#skill-descriptions-are-cut-short)                                                                                                                                               | Memory and context                 | Any file                |
| [`skillOverrides`](#skilloverrides)                                                                   | [Verbergen oder reduzieren Sie einen Skill](/docs/de/skills#override-skill-visibility-from-settings), ohne seine SKILL.md zu bearbeiten                                                                                                                                     | Plugins and skills                 | Any file                |
| [`skipAutoPermissionPrompt`](#skipautopermissionprompt)                                               | Überspringen Sie die einmalige Benachrichtigung, die Claude Code anzeigt, wenn Sie selbst zum ersten Mal den [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) betreten, anstatt durch den integrierten Standard                                     | Permission settings                | User or managed         |
| [`skipDangerousModePermissionPrompt`](#skipdangerousmodepermissionprompt)                             | Überspringen Sie das Bestätigungsdialogfeld vor dem [bypassPermissions-Modus](/docs/de/permission-modes#skip-all-checks-with-bypasspermissions-mode)                                                                                                                        | Permission settings                | User, local, or managed |
| [`skipWebFetchPreflight`](#skipwebfetchpreflight)                                                     | Überspringen Sie die [WebFetch-Hostname-Überprüfung](/docs/de/tools-reference#webfetch-tool-behavior), wenn Anthropic nicht erreichbar ist                                                                                                                                  | Privacy and telemetry              | Any file                |
| [`spellcheck`](#spellcheck)                                                                           | Unterstreichen Sie falsch geschriebene Wörter in der Eingabeaufforderung mit einem [Rechtschreibprüfer](/docs/de/interactive-mode#check-spelling-as-you-type), den Sie installieren                                                                                         | Interface and terminal             | User or managed         |
| [`spinnerTipsEnabled`](#spinnertipsenabled)                                                           | Verbergen Sie Tipps im Spinner, während Claude arbeitet                                                                                                                                                                                                                | Interface and terminal             | Any file                |
| [`spinnerTipsOverride`](#spinnertipsoverride)                                                         | Fügen Sie Ihre eigenen Tipps zur Spinner-Rotation hinzu, oder ersetzen Sie die integrierten Tipps                                                                                                                                                                      | Interface and terminal             | Any file                |
| [`spinnerVerbs`](#spinnerverbs)                                                                       | Fügen Sie die Verben hinzu oder ersetzen Sie sie, die während einer Runde angezeigt werden                                                                                                                                                                             | Interface and terminal             | Any file                |
| [`sshConfigs`](#sshconfigs)                                                                           | Fügen Sie [SSH-Verbindungen](/docs/de/desktop#pre-configure-ssh-connections-for-your-team) zum Desktop-Umgebungs-Dropdown hinzu                                                                                                                                             | Remote, desktop, and notifications | User or managed         |
| [`sshHostAllowlist`](#sshhostallowlist)                                                               | Begrenzen Sie, welche Hosts [Desktop-SSH-Sitzungen](/docs/de/desktop#restrict-which-ssh-hosts-users-can-connect-to) erreichen können                                                                                                                                        | Remote, desktop, and notifications | Managed                 |
| [`statusLine`](#statusline)                                                                           | Führen Sie Ihren eigenen Befehl aus, um eine [Statuszeile](/docs/de/statusline) unter der Eingabeaufforderung zu rendern                                                                                                                                                    | Interface and terminal             | Any file                |
| [`strictKnownMarketplaces`](#strictknownmarketplaces)                                                 | Zulassungsliste der [Marketplace](/docs/de/plugins/overview)-Quellen, die Benutzer hinzufügen und installieren können                                                                                                                                                       | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)                                     | Blockieren Sie [Skills](/docs/de/skills), [Agenten](/docs/de/sub-agents), [Hooks](/docs/de/hooks) und [MCP-Server](/docs/de/mcp) aus Benutzer- und Projektquellen                                                                                                                          | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.agents`](#strictpluginonlycustomization-agents)                       | Sperren Sie [Agenten](/docs/de/sub-agents) auf Plugin- und verwaltete Quellen                                                                                                                                                                                               | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.hooks`](#strictpluginonlycustomization-hooks)                         | Sperren Sie [Hooks](/docs/de/hooks) auf Plugin- und verwaltete Quellen                                                                                                                                                                                                      | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.mcp`](#strictpluginonlycustomization-mcp)                             | Sperren Sie [MCP-Server](/docs/de/mcp) auf Plugin- und verwaltete Quellen                                                                                                                                                                                                   | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.skills`](#strictpluginonlycustomization-skills)                       | Sperren Sie [Skills](/docs/de/skills) auf Plugin- und verwaltete Quellen                                                                                                                                                                                                    | Plugins and skills                 | Managed                 |
| [`subagentPromptCacheTtl`](#subagentpromptcachettl)                                                   | Wählen Sie die [Prompt-Cache-Lebensdauer](/docs/de/prompt-caching#cache-lifetime) für Subagenten und andere Anfragen außerhalb der Hauptkonversation                                                                                                                        | Model and responses                | Any file                |
| [`subagentStatusLine`](#subagentstatusline)                                                           | Schreiben Sie Zeilen in der [Subagenten](/docs/de/sub-agents)-Aufgabenanzeige mit Ihrem eigenen Befehl um                                                                                                                                                                   | Interface and terminal             | Any file                |
| [`switchModelsOnFlag`](#switchmodelsonflag)                                                           | Wechseln Sie Modelle automatisch oder pausieren Sie, wenn ein [Sicherheitsklassifizierer](/docs/de/model-config#ask-before-switching) eine Anfrage kennzeichnet                                                                                                             | Model and responses                | Any file                |
| [`syncClaudeAiPlugins`](#syncclaudeaiplugins)                                                         | Beenden Sie das Laden der [auf Ihrem claude.ai-Konto aktivierten Plugins](/docs/de/plugins/overview) und beenden Sie das Herunterladen neuer                                                                                                                                | Plugins and skills                 | User, local, or managed |
| [`syncClaudeAiSkills`](#syncclaudeaiskills)                                                           | Beenden Sie das Laden der [auf Ihrem claude.ai-Konto aktivierten Skills](/docs/de/skills#how-synced-skills-behave) und beenden Sie das Herunterladen neuer                                                                                                                  | Plugins and skills                 | User, local, or managed |
| [`syntaxHighlightingDisabled`](#syntaxhighlightingdisabled)                                           | Schalten Sie die Syntaxhervorhebung in Diffs und Code-Blöcken aus                                                                                                                                                                                                      | Interface and terminal             | Any file                |
| [`taskOutputMaxChars`](#taskoutputmaxchars)                                                           | Entfernt in v2.1.277, zusammen mit dem `TaskOutput`-Tool, das es dimensioniert                                                                                                                                                                                         | Memory and context                 | Any file                |
| [`teammateDefaultModel`](#teammatedefaultmodel)                                                       | Entfernt in v2.1.234; siehe [Geben Sie Teamkollegen und Modelle an](/docs/de/agent-teams#specify-teammates-and-models), wie Claude Code das Modell eines Teamkollegen auswählt                                                                                              | Global config settings             | Global config           |
| [`teammateMode`](#teammatemode)                                                                       | Wählen Sie, wie [Agent-Team-Teamkollegen angezeigt werden](/docs/de/agent-teams#choose-a-display-mode)                                                                                                                                                                      | Agents, sessions, and worktrees    | Any file                |
| [`terminalProgressBarEnabled`](#terminalprogressbarenabled)                                           | Verbergen Sie die Terminal-Fortschrittsleiste in Terminals, die sie unterstützen                                                                                                                                                                                       | Interface and terminal             | Any file                |
| [`terminalTitleFromRename`](#terminaltitlefromrename)                                                 | Verhindern Sie, dass [`/rename`](/docs/de/sessions#name-your-sessions) und `--name` den Terminal-Tab-Titel ändern                                                                                                                                                           | Interface and terminal             | Any file                |
| [`theme`](#theme)                                                                                     | Wählen Sie das Interface-[Farbschema](/docs/de/terminal-config#match-the-color-theme), integriert oder benutzerdefiniert                                                                                                                                                    | Interface and terminal             | Any file                |
| [`timeFormat`](#timeformat)                                                                           | Zeigen Sie die Zeiten in der Benutzeroberfläche auf einer 12-Stunden- oder 24-Stunden-Uhr, in UTC oder mit einem strftime-Muster an                                                                                                                                    | Interface and terminal             | Any file                |
| [`timeZone`](#timezone)                                                                               | Zeigen Sie die Zeiten in der Benutzeroberfläche in einer Zeitzone an, die nicht die Ihres Systems ist                                                                                                                                                                  | Interface and terminal             | Any file                |
| [`tui`](#tui)                                                                                         | Wählen Sie den [Vollbild-](/docs/de/fullscreen) oder klassischen Terminal-Renderer                                                                                                                                                                                          | Interface and terminal             | Any file                |
| [`ultracode`](#ultracode)                                                                             | Lassen Sie Claude einen [Workflow](/docs/de/workflows#let-claude-decide-with-ultracode) für jede wesentliche Aufgabe planen, ohne gefragt zu werden                                                                                                                         | Model and responses                | Any file                |
| [`useAutoModeDuringPlan`](#useautomodeduringplan)                                                     | Lassen Sie den [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode)-Klassifizierer Shell-Befehle im [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) überprüfen; setzen Sie `false`, um stattdessen Aufforderungen zu erhalten | Permission settings                | User, local, or managed |
| [`verbose`](#verbose)                                                                                 | Zeigen Sie [vollständige Tool-Ausgabe](/docs/de/cli-reference#cli-flags) statt verkürzter Zusammenfassungen an; `viewMode` hat Vorrang, wenn beide gesetzt sind                                                                                                             | Interface and terminal             | Any file                |
| [`viewMode`](#viewmode)                                                                               | Starten Sie jede Sitzung in [Standard-, Verbose- oder Focus-Ansicht](/docs/de/cli-reference#cli-flags)                                                                                                                                                                      | Interface and terminal             | Any file                |
| [`vimInsertModeRemaps`](#viminsertmoderemaps)                                                         | Ordnen Sie eine zwei-Tasten-[INSERT-Modus-Sequenz](/docs/de/interactive-mode#remap-insert-mode-key-sequences) wie `jj` zu Escape                                                                                                                                            | Interface and terminal             | User or managed         |
| [`voice`](#voice)                                                                                     | Schalten Sie [Sprachdiktat](/docs/de/voice-dictation) ein und wählen Sie Halte- oder Tap-Modus                                                                                                                                                                              | Interface and terminal             | Any file                |
| [`voiceEnabled`](#voiceenabled)                                                                       | Schalten Sie [Sprachdiktat](/docs/de/voice-dictation) mit der älteren Einzeltasten-Form ein                                                                                                                                                                                 | Interface and terminal             | Any file                |
| [`wheelScrollAccelerationEnabled`](#wheelscrollaccelerationenabled)                                   | Schalten Sie die [Mausrad-Beschleunigung](/docs/de/fullscreen#mouse-wheel-scrolling) beim Vollbild-Rendering aus                                                                                                                                                            | Interface and terminal             | Any file                |
| [`workflowKeywordTriggerEnabled`](#workflowkeywordtriggerenabled)                                     | Lassen Sie das Wort `ultracode` in einer Eingabeaufforderung einen [Workflow](/docs/de/workflows) starten; setzen Sie `false`, um es einzugeben, ohne einen zu starten                                                                                                      | Hooks and automation               | Any file                |
| [`workflowSizeGuideline`](#workflowsizeguideline)                                                     | Legen Sie die Agent-Anzahl fest, auf die Claude in [dynamischen Workflows](/docs/de/workflows) abzielt                                                                                                                                                                      | Hooks and automation               | Any file                |
| [`worktree`](#worktree)                                                                               | Konfigurieren Sie, wie Claude Code Git-[Worktrees](/docs/de/worktrees) erstellt                                                                                                                                                                                             | Agents, sessions, and worktrees    | Any file                |
| [`worktree.baseRef`](#worktree-baseref)                                                               | Verzweigen Sie neue [Worktrees](/docs/de/worktrees) vom Remote-Standard-Branch oder Ihrem lokalen HEAD                                                                                                                                                                      | Agents, sessions, and worktrees    | Any file                |
| [`worktree.bgIsolation`](#worktree-bgisolation)                                                       | Lassen Sie Hintergrund-Sitzungen die Arbeitskopie ohne [Worktree](/docs/de/worktrees) bearbeiten                                                                                                                                                                            | Agents, sessions, and worktrees    | Any file                |
| [`worktree.sparsePaths`](#worktree-sparsepaths)                                                       | Checken Sie nur die Verzeichnisse aus, die Sie in jedem [Worktree](/docs/de/worktrees) benötigen                                                                                                                                                                            | Agents, sessions, and worktrees    | Any file                |
| [`worktree.symlinkDirectories`](#worktree-symlinkdirectories)                                         | Symlinken Sie große Verzeichnisse in jeden [Worktree](/docs/de/worktrees), anstatt sie zu duplizieren                                                                                                                                                                       | Agents, sessions, and worktrees    | Any file                |
| [`wslInheritsWindowsSettings`](#wslinheritswindowssettings)                                           | Lassen Sie WSL [verwaltete Einstellungen](/docs/de/managed-settings) aus der Windows-Richtlinienkette lesen                                                                                                                                                                 | Enterprise and managed settings    | Managed                 |

<h2 id="model-and-responses">
  Modell und Antworten
</h2>

Wählen Sie, welche Modelle Claude Code verwendet und wie es antwortet. Informationen darüber, wie diese Einstellungen mit dem Befehl `/model` und Umgebungsvariablen interagieren, finden Sie unter [Modellkonfiguration](/docs/de/model-config).

<h3 id="advisormodel">
  `advisorModel`
</h3>

Wählen Sie, welches Modell antwortet, wenn Claude das serverseitige [Advisor-Tool](/docs/de/advisor) aufruft. Deaktivieren Sie es, um den Advisor auszuschalten. Der Advisor muss mindestens so leistungsfähig sein wie Ihr Hauptmodell. Informationen zu akzeptierten Kombinationen und was passiert, wenn Sie eine nicht akzeptierte Kombination wählen, finden Sie unter [Wählen Sie ein Advisor-Modell](/docs/de/advisor#choose-an-advisor-model).

Sie bearbeiten diesen Schlüssel normalerweise nicht manuell. Führen Sie `/advisor` aus, um eine Auswahl zu öffnen, die die aktuelle Auswahl, die Modelle, die beraten können, und **Kein Advisor** anzeigt. Claude Code speichert Ihre Auswahl in diesem Schlüssel in `~/.claude/settings.json`. Wenn Sie aus einem [Remote Control](/docs/de/remote-control)-Client oder in einer Sitzung auswählen, die an einen Remote Worker angehängt ist, gilt die Auswahl nur für diese Sitzung und ändert diesen Schlüssel nicht.

Wenn Ihr Konto die [Zustimmung zu Nutzungsguthaben](/docs/de/advisor#fable-advisor-and-usage-credits) erfordert, akzeptieren Sie diese zuerst, indem Sie `/model fable` ausführen. Bis dahin speichert die Auswahl von Fable in `/advisor` nichts und Claude Code teilt Ihnen mit, dass Sie zuerst `/model fable` ausführen sollen.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: String, einer der Aliase `"fable"`, `"opus"` oder `"sonnet"`, die in die aktuelle Standardversion dieser Modellfamilie von Claude Code aufgelöst werden, oder eine vollständige Modell-ID wie `"claude-opus-5-5"`
* **Standard**: nicht gesetzt, daher ist der Advisor ausgeschaltet
* **Sitzungsübergreifende Außerkraftsetzungen**: `--advisor` hat Vorrang vor diesem Schlüssel für eine Sitzung. [`CLAUDE_CODE_DISABLE_ADVISOR_TOOL`](/docs/de/env-vars) schaltet den Advisor aus, und dieser Schlüssel kann ihn nicht wieder einschalten

```json settings.json theme={null}
{
  "advisorModel": "opus"
}
```

Der Schlüssel hat keine Auswirkung auf Provider, bei denen der Advisor [nicht verfügbar](/docs/de/advisor#requirements) ist, wie Amazon Bedrock und Claude Platform auf AWS. `"fable"` erfordert [Fable-Zugriff](/docs/de/advisor#choose-an-advisor-model).

<h3 id="alwaysthinkingenabled">
  `alwaysThinkingEnabled`
</h3>

Deaktivieren Sie [erweitertes Denken](/docs/de/model-config#extended-thinking) für jede Sitzung, indem Sie dies auf `false` setzen. Das Denken ist standardmäßig aktiviert, daher ändert `true` nichts. Die meisten Benutzer stellen dies über `/config` ein, anstatt die Datei zu bearbeiten.

Bei Modellen, die immer denken, wie Opus 5.5 und die Fable-Modelle, hat `false` keine Auswirkung. Bei [Drittanbieter-Providern](/docs/de/third-party-integrations) lässt Claude Code den Parameter `thinking` weg, anstatt das Denken auszuschalten, daher können adaptive Reasoning-Modelle möglicherweise noch denken. Wenn das Denken auf der Anthropic API ausgeschaltet ist, sendet Claude Code stattdessen `high` Aufwand an Modelle, die [diese Kombination nicht akzeptieren](/docs/de/errors#effort-isnt-available-with-thinking-turned-off), wie Opus 5.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: keine Auswirkung; das Denken ist bereits aktiviert
  * `false`: Claude Code deaktiviert erweitertes Denken für jede Sitzung
* **Standard**: nicht gesetzt, daher ist das Denken für Modelle aktiviert, die es unterstützen
* **Sitzungsübergreifende Außerkraftsetzungen**: [`MAX_THINKING_TOKENS`](/docs/de/env-vars) hat Vorrang vor diesem Schlüssel für eine Sitzung: `0` deaktiviert das Denken unter den gleichen Modell- und Provider-Einschränkungen wie `false`, und ein positiver Wert aktiviert das Denken auch wenn dieser Schlüssel `false` ist. Bei adaptive-reasoning-Modellen wird die Zahl selbst ignoriert

```json settings.json theme={null}
{
  "alwaysThinkingEnabled": false
}
```

<h3 id="availablemodels">
  `availableModels`
</h3>

Beschränken Sie, welche Modelle Personen für die Hauptsitzung, [Subagents](/docs/de/sub-agents), [Skills](/docs/de/skills) und den [Advisor](/docs/de/advisor) auswählen können. Eine verwaltete Liste beschränkt `/model`, `--model` und den Schlüssel `model` in den eigenen Dateien eines Entwicklers; ein Modell außerhalb davon kann nicht ausgewählt werden. Dies berührt die Option Standard nicht; kombinieren Sie es mit [`enforceAvailableModels`](#enforceavailablemodels), um das zu tun.

* **Bereich**: [`Beliebige Datei`](#scopes). Stellen Sie es in verwalteten Einstellungen bereit, um es für eine Organisation durchzusetzen.
* **Typ**: Array von Modellaliasen oder IDs
* **Standard**: nicht gesetzt, daher sind alle Modelle verfügbar

Dieses Beispiel ermöglicht es Personen, nur Sonnet- und Haiku-Modelle auszuwählen:

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

Siehe [Modellauswahl einschränken](/docs/de/model-config#restrict-model-selection).

<h3 id="effortlevel">
  `effortLevel`
</h3>

Legen Sie eine Standard-[Aufwandsebene](/docs/de/model-config#adjust-effort-level) für Modelle fest, für die Sie noch keine Ebene gespeichert haben. Niedrigere Ebenen sind schneller und günstiger bei einfachen Aufgaben, höhere Ebenen denken tiefer über komplexe Probleme nach.

Wenn Sie `/effort low`, `medium`, `high` oder `xhigh` in einer interaktiven Sitzung auf Ihrem Computer ausführen, speichert Claude Code die Ebene für das aktive Modell unter [`modelSettings`](#modelsettings) anstatt diesen Schlüssel zu schreiben. Vor v2.1.251 schrieb `/effort` diesen Schlüssel.

Innerhalb derselben Einstellungsdatei verwendet Claude Code die gespeicherte Ebene eines Modells anstatt dieses Schlüssels. [`modelSettings`](#modelsettings) gibt die dateiübergreifende Priorität an.

In einer Sitzung, die an einen Remote Worker angehängt ist, in einem `-p`-Lauf und im Agent SDK gilt `/effort` nur für diese Sitzung. [Aufwandsebene anpassen](/docs/de/model-config#adjust-effort-level) listet die interaktiven Auswahlmöglichkeiten auf, die auch nur für diese Sitzung gelten. Die Nachricht, die `/effort` ausgibt, sagt, was passiert ist.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: String, einer von:
  * `"low"`: das geringste Denken, für kurze, begrenzte, latenzempfindliche Aufgaben, die nicht intelligenzempfindlich sind
  * `"medium"`: reduziert die Token-Nutzung für kostensensitive Arbeiten, die etwas Intelligenz opfern können
  * `"high"`: balanciert Token-Nutzung und Intelligenz
  * `"xhigh"`: tieferes Denken bei höheren Token-Ausgaben
* **Standard**: nicht gesetzt
* **Sitzungsübergreifende Außerkraftsetzungen**: `--effort` hat Vorrang vor diesem Schlüssel für eine Sitzung, und [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/de/env-vars) hat Vorrang vor beiden

```json settings.json theme={null}
{
  "effortLevel": "xhigh"
}
```

In Ihrer Benutzereinstellungsdatei `~/.claude/settings.json` ist dieser Schlüssel die ältere Form, die `/effort` vor dem Speichern von Ebenen pro Modell schrieb, und er wird weiterhin dort angewendet, wo er zuvor angewendet wurde, auf Opus 5, Fable 5.1 und früheren Modellen. Opus 5.5 und später veröffentlichte Modelle ignorieren ihn und beginnen mit ihrem eigenen Standard, bis Sie eine Ebene für sie speichern, die `/effort` unter [`modelSettings`](#modelsettings) schreibt. In Projekt-, Lokal- und verwalteten Einstellungen sowie mit `--settings` gilt dieser Schlüssel für jedes Modell.

<h3 id="enforceavailablemodels">
  `enforceAvailableModels`
</h3>

Die Auswahl `/model` hat eine Option **Standard**, die sich zu Ihrem [Organisations-Standardmodell](/docs/de/model-config#organization-default-model) auflöst, wenn eine gilt, und ansonsten zu Ihrem Kontotyp-Standard. Eine [`availableModels`](#availablemodels)-Zulassungsliste beschränkt die Modelle, die Sie benennen können, aber sie lässt **Standard** allein, daher kann **Standard** sich immer noch zu einem Modell außerhalb der Liste auflösen. Dieser Schlüssel schließt diese Lücke. Erfordert Claude Code v2.1.175 oder später.

Wenn Ihre Organisation verwaltete Einstellungen bereitstellt, liest Claude Code diesen Schlüssel nur aus der verwalteten Quelle und ignoriert ihn in Ihren anderen Dateien.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: wenn sich **Standard** zu einem Modell außerhalb von `availableModels` auflösen würde, löst Claude Code es zum ersten verfügbaren Modell in der Liste auf
  * `false`: **Standard** löst sich wie gewohnt auf, auch zu einem Modell außerhalb von `availableModels`
* **Standard**: `false`

Dieses Beispiel beschränkt benannte Auswahlmöglichkeiten auf Sonnet- und Haiku-Modelle und lässt **Standard** sich zum ersten verfügbaren Modell auflösen:

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

Dieser Schlüssel hat keine Auswirkung, wenn `availableModels` nicht gesetzt oder leer ist. Siehe [Zulassungsliste für das Standardmodell durchsetzen](/docs/de/model-config#enforce-the-allowlist-for-the-default-model). Erfordert Claude Code v2.1.175 oder später.

<h3 id="fallbackmodel">
  `fallbackModel`
</h3>

Benennen Sie Backup-Modelle, die Claude Code der Reihe nach versuchen soll, wenn Ihr primäres Modell überlastet oder nicht verfügbar ist. Claude Code wechselt zum nächsten verfügbaren Modell in der Kette für den Rest des Durchlaufs und zeigt eine Benachrichtigung an. Ohne eine Kette versucht Claude Code das gleiche Modell erneut und zeigt dann den Fehler des Servers an, und Sie versuchen es erneut oder wechseln Modelle selbst.

Ein Wechsel bedeutet einen Durchlauf mit einem kalten [Prompt-Cache](/docs/de/prompt-caching#switching-models) auf dem Fallback-Modell; Ihre nächste Nachricht versucht zuerst das primäre Modell erneut.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Array von Modellaliasen oder IDs; `"default"` wird zum Standardmodell erweitert
* **Standard**: nicht gesetzt, daher wird eine fehlgeschlagene Anfrage nicht auf einem anderen Modell erneut versucht
* **Sitzungsübergreifende Außerkraftsetzungen**: `--fallback-model` hat Vorrang vor diesem Schlüssel für eine Sitzung

Dieses Beispiel versucht zuerst Sonnet 5, dann Haiku 4.5, wenn Ihr primäres Modell fehlschlägt:

```json settings.json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

Im Gegensatz zu den meisten Array-Einstellungen wird dieser Schlüssel nicht über Einstellungsdateien hinweg zusammengeführt: die höchste Prioritätsdatei, die ihn definiert, liefert die ganze Kette. Wenn Ihre Projektdatei `["claude-sonnet-5"]` setzt und Ihre Benutzerdatei `["claude-haiku-4-5"]` setzt, ist die Kette nur `["claude-sonnet-5"]`. Claude Code behält höchstens drei unterschiedliche zulässige Modelle aus der Liste und ignoriert den Rest. Siehe [Fallback-Modellketten](/docs/de/model-config#fallback-model-chains).

<h3 id="fastmode">
  `fastMode`
</h3>

Aktivieren Sie den [Schnellmodus](/docs/de/fast-mode) für Sitzungen, in denen er verfügbar ist, für interaktive Arbeiten wie schnelle Iteration oder Live-Debugging, bei denen Sie Geschwindigkeit zu höheren Kosten pro Token wünschen. Sie bearbeiten diesen Schlüssel normalerweise nicht manuell: Das Ausführen von `/fast` schreibt `fastMode: true` zu `~/.claude/settings.json`, und das erneute Ausführen zum Ausschalten des Schnellmodus entfernt den Schlüssel. Der Schnellmodus läuft nur auf Opus 5.5, Opus 5 und Opus 4.8: Das Aktivieren von einem anderen Modell wechselt Sie zu Opus, und das Wechseln zu einem nicht unterstützten Modell schaltet ihn aus. Siehe [Modelle wechseln, während der Schnellmodus aktiv ist](/docs/de/fast-mode#switch-models-while-fast-mode-is-on).

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code aktiviert den Schnellmodus für Sitzungen, in denen er verfügbar ist
  * `false`: Der Schnellmodus bleibt ausgeschaltet
* **Standard**: nicht gesetzt, daher ist der Schnellmodus ausgeschaltet
* **Sitzungsübergreifende Außerkraftsetzungen**: [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/de/env-vars) schaltet den Schnellmodus für eine Sitzung aus, und dieser Schlüssel kann ihn nicht wieder einschalten

```json settings.json theme={null}
{
  "fastMode": true
}
```

<h3 id="fastmodepersessionoptin">
  `fastModePerSessionOptIn`
</h3>

Normalerweise speichert das Ausführen von `/fast` [`fastMode`](#fastmode) in den Benutzereinstellungen einer Person, daher ist der Schnellmodus am Anfang jeder späteren Sitzung aktiviert. Setzen Sie diesen Schlüssel auf `true`, um das zu stoppen: ein gespeichertes `fastMode: true` aktiviert den Schnellmodus nicht mehr beim Sitzungsstart, und jede Person muss `/fast` in jeder Sitzung ausführen, in der sie es möchte. Claude Code lässt den Schlüssel `fastMode` in ihrer Datei, daher stellt das Ausschalten dieses Schlüssels das alte Verhalten wieder her.

Besitzer in Team- oder Enterprise-Plänen können es organisationsweit durch [serverseitig verwaltete Einstellungen](/docs/de/server-managed-settings) bereitstellen. Wenn verwaltete Einstellungen den Schlüssel setzen, wird `/fast on` außerhalb interaktiver Terminal-Sitzungen abgelehnt und meldet, dass Ihre Organisation den Schnellmodus deaktiviert hat. Das umfasst den [nicht-interaktiven Modus](/docs/de/headless), die [VS Code-Erweiterung](/docs/de/vs-code) und [Cloud-Sitzungen](/docs/de/claude-code-on-the-web).

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: ein gespeichertes `fastMode: true` aktiviert den Schnellmodus nicht mehr beim Sitzungsstart, daher führt jede Person `/fast` in jeder Sitzung aus, in der sie es möchte; ein `fastMode: true` mit `--settings` zählt immer noch für diese Sitzung, es sei denn, verwaltete Einstellungen setzen diesen Schlüssel
  * `false`: ein gespeichertes `fastMode: true` aktiviert den Schnellmodus am Anfang jeder späteren Sitzung
* **Standard**: `false`

```json settings.json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Siehe [Opt-in pro Sitzung erforderlich](/docs/de/fast-mode#require-per-session-opt-in).

<h3 id="language">
  `language`
</h3>

Lassen Sie Claude standardmäßig in einer anderen Sprache als Englisch antworten. Es gibt keine feste Liste für Antworten: Claude Code übergibt den Wert wörtlich an Claude als Anweisung, immer in dieser Sprache zu antworten, daher funktioniert jeder Sprachname, den Claude lesen kann. Claude Code überprüft den Wert nicht, daher erreicht ein falsch geschriebener Name Claude wie geschrieben, anstatt einen Fehler zu erzeugen. Der gleiche Wert setzt die Sprache für [Sprachdiktat](/docs/de/voice-dictation#change-the-dictation-language), das eine feste Liste von [unterstützten Diktiersprachen](/docs/de/voice-dictation#change-the-dictation-language) hat, und für automatisch generierte Sitzungstitel.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: String, jeder Sprachname, wie `"japanese"`, `"spanish"` oder `"french"`; Claude Code überprüft ihn nicht
* **Standard**: nicht gesetzt; Sitzungstitel entsprechen dann der Sprache Ihrer Konversation

```json settings.json theme={null}
{
  "language": "japanese"
}
```

<h3 id="maxeffortlevel">
  `maxEffortLevel`
</h3>

Begrenzen Sie die [Aufwandsebene](/docs/de/model-config#adjust-effort-level), die eine Sitzung verwenden kann, und lassen Sie niedrigere Ebenen verfügbar. Jede höhere Ebene läuft stattdessen bei der Obergrenze, einschließlich einer von `/effort`, der Auswahl `/model`, `--effort`, [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/de/env-vars), der Frontmatter `effort` eines Skills oder Subagents oder dem eigenen Standard des Modells. Claude Code wendet die Obergrenze selbst vor jeder Anfrage an, daher gilt sie auf jedem Provider, einschließlich Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry. Erfordert Claude Code v2.1.267 oder später.

* **Bereich**: [`Beliebige Datei`](#scopes). Stellen Sie es in verwalteten Einstellungen bereit, um es für eine Organisation durchzusetzen. Wenn mehrere Bereiche eine Obergrenze setzen, gilt die niedrigste, daher kann eine in einem Bereich gesetzte Obergrenze nicht von einem anderen erhöht werden
* **Typ**: String, einer von `"low"`, `"medium"`, `"high"`, `"xhigh"` oder `"max"`. Ein Wert `"max"` setzt keine Obergrenze
* **Standard**: nicht gesetzt, daher gilt keine Obergrenze
* **Auswirkung auf Ultracode**: eine Obergrenze unter `xhigh` macht [Ultracode](#ultracode) auf den Modellen, auf die die Obergrenze zutrifft, nicht verfügbar
* **Pro-Modell-Obergrenzen**: fügen Sie `maxEffortLevel` zum Eintrag [`modelSettings`](#modelsettings) eines Modells hinzu. Dieser Eintrag ersetzt diesen Schlüssel nur für das Modell innerhalb der Einstellungsquelle, die beide setzt, wie Ihre Benutzereinstellungen oder eine [verwaltete Quelle](/docs/de/managed-settings#how-claude-code-combines-managed-sources). Setzen Sie dort `"max"`, um das Modell von der Obergrenze dieser Quelle auszunehmen; Claude Code wendet immer noch Obergrenzen von anderen Quellen an

Dieses Beispiel begrenzt jedes Modell auf `medium` und befreit Sonnet 4.6:

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

Wenn Ihre Organisation auch ein [Aufwandslimit](/docs/de/model-config#organization-effort-limits) für ein Modell setzt, gilt die niedrigere der beiden Obergrenzen.

<h3 id="model">
  `model`
</h3>

Legen Sie das Modell fest, das jede neue Sitzung verwendet, damit Sie nicht jedes Mal mit `/model` eines auswählen müssen. Das Setzen hier hindert Sie nicht daran, die Sitzung zu wechseln. Wenn Ihr Administrator ein [Organisations-Standardmodell](/docs/de/model-config#organization-default-model) gesetzt hat, um die Benutzerauswahl zu überschreiben, erhalten Sie dieses Modell auch wenn Sie diesen Schlüssel in Benutzer-, Projekt- oder Lokaleinstellungen setzen.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: String, ein Modellalias oder eine vollständige Modell-ID
* **Standard**: nicht gesetzt, daher verwendet Claude Code das Standardmodell Ihres Kontos
* **Sitzungsübergreifende Außerkraftsetzungen**: `--model` hat Vorrang vor [`ANTHROPIC_MODEL`](/docs/de/env-vars), und beide haben Vorrang vor diesem Schlüssel für eine Sitzung, einschließlich vor einem verwalteten `model`; eine [`availableModels`](#availablemodels)-Liste gilt immer noch für die Auswahl

```json settings.json theme={null}
{
  "model": "claude-sonnet-5"
}
```

Ein Wert hier übertrumpft [`ANTHROPIC_DEFAULT_MODEL`](/docs/de/model-config#set-a-default-model-for-new-sessions), das Claude Code nur verwendet, wenn nichts anderes ein Modell auswählt.

<h3 id="modeloverrides">
  `modelOverrides`
</h3>

Ordnen Sie Anthropic-Modell-IDs Anbieter-spezifischen Modell-IDs zu, wie Amazon Bedrock Inferenz-Profil-ARNs. Jeder Modellauswahl-Eintrag verwendet dann seinen zugeordneten Wert beim Aufrufen der Provider-API. Administratoren verwenden dies auf [Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry](/docs/de/model-config#override-model-ids-per-version), um jede Modellversion zu einem bestimmten Inferenz-Profil, Versionsnamen oder einer Bereitstellung für Governance, Kostenzuteilung oder regionales Routing zu leiten.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Objekt, das Modell-ID zu Anbieter-Modell-ID zuordnet
* **Standard**: nicht gesetzt

Dieses Beispiel leitet jeden Aufruf für Opus 4.6 zum benannten Bedrock-Inferenz-Profil:

```json settings.json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-6": "arn:aws:bedrock:us-east-1:123456789012:inference-profile/example"
  }
}
```

Siehe [Modell-IDs pro Version überschreiben](/docs/de/model-config#override-model-ids-per-version).

<h3 id="modelpicker">
  `modelPicker`
</h3>

Listen Sie die Modelle auf, die die Auswahl `/model` anbietet, in der Reihenfolge, in der Sie sie schreiben, und unter Bezeichnungen, die Sie wählen, damit die Auswahl die Modelle auflistet, die Ihre Organisation ausführt, nach der integrierten Auswahl oder stattdessen. Das `model` jeder Zeile wird wörtlich genommen, daher akzeptiert es alles, was `--model` akzeptiert: ein Alias wie `opus`, eine Anthropic-Modell-ID oder eine Anbieter-Format-ID für Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder ein LLM-Gateway. Erfordert Claude Code v2.1.242 oder später.

* **Bereich**: [`Benutzer oder verwaltet`](#scopes). Claude Code liest den Schlüssel aus verwalteten Einstellungen, `--settings` und Benutzereinstellungen und ignoriert ihn in Projekt- und Lokaleinstellungen, daher kann ein Repository, das Sie klonen, die Auswahl nicht umbenennen. Die höchste dieser drei, die den Schlüssel setzt, liefert die ganze Auswahl, und Claude Code kombiniert Auswahlmöglichkeiten von zwei Quellen nie.
* **Typ**: Objekt mit einem Array `options` von Zeilen und einem optionalen Boolean `replaceBuiltInOptions`
* **Standard**: nicht gesetzt, daher zeigt die Auswahl die integrierte Auswahl

Dieses Beispiel fügt zwei Bedrock-Bereitstellungen nach der integrierten Auswahl hinzu, unter Namen, die Ihr Team erkennt:

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
  Felder für `modelPicker`
</h4>

Der Schlüssel nimmt zwei Felder, eines für die Zeilen selbst und eines dafür, ob sie die integrierte Auswahl ersetzen oder zu ihr hinzufügen.

| Feld                    | Typ                                                                                                    | Was es tut                                                                                                                                                                                                                                                                                                                |
| :---------------------- | :----------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `options`               | Array von Zeilen, jede mit einem erforderlichen `model` und einem optionalen `label` und `description` | Die Zeilen, die die Auswahl zeigt, in dieser Reihenfolge, außer dass eine ausgegraut Zeile nach unten verschoben wird. Ohne `label` betitelt Claude Code die Zeile mit dem integrierten Namen für ein Modell, das es kennt, oder der Modell-ID ansonsten, und ohne `description` schreibt es eine generische zweite Zeile |
| `replaceBuiltInOptions` | Boolean, Standard `false`                                                                              | Setzen Sie es auf `true`, um nur diese Zeilen, **Standard** und eine Zeile für das Modell anzuzeigen, das die Sitzung bereits verwendet. Lassen Sie es ungesetzt, um diese Zeilen nach der integrierten Auswahl hinzuzufügen                                                                                              |

Mit `replaceBuiltInOptions` an, versteckt Claude Code jede andere Zeile: die integrierte Auswahl, die Zeilen, die es für [`availableModels`](#availablemodels)-Einträge hinzufügt, die Modelle, die [Gateway-Erkennung](/docs/de/llm-gateway-protocol#model-discovery) gefunden hat, und [`ANTHROPIC_CUSTOM_MODEL_OPTION`](/docs/de/model-config#add-a-custom-model-option). Mit ihr aus, überspringt Claude Code ein aufgelistetes Modell, das die integrierte Auswahl bereits abdeckt. Ein Label ändert, was die Auswahl zeigt, nicht welches Modell Claude Code ausführt.

Eine [`availableModels`](#availablemodels)-Zulassungsliste gilt immer noch für diese Zeilen. Bevor Sie ein aufgelistetes Modell zur Zulassungsliste hinzufügen, lesen Sie [Zusammenführungsverhalten](/docs/de/model-config#merge-behavior): eine spezifische Modell-ID verengt den Wildcard-Eintrag ihrer Familie. Claude Code überprüft auch jede Zeile gegen die Sitzung, bevor es die Auswahl zeigt:

* **Gelöscht**: eine Zeile, die Claude Code nicht bedienen kann, wie ein veraltetes Modell oder ein Modell, auf das Ihre Organisation keinen Zugriff hat
* **Ausgegraut**: eine Zeile, die Sie noch nicht auswählen können, mit dem Grund angezeigt
* **Keine Zeile überlebt**: Claude Code behält die integrierte Auswahl, gefiltert durch die Zulassungsliste wie gewohnt

Claude Code löscht eine Zeile, die es nicht analysieren kann, und behält den Rest. Siehe [Fehlerhafte Einstellungsdatei beheben](/docs/de/settings#fix-a-broken-settings-file).

<h3 id="modelpricing">
  `modelPricing`
</h3>

Melden Sie Ausgaben zu den Sätzen, die Ihre Organisation zahlt, anstatt zum Listenpreis. Setzen Sie es, wenn Ihre Organisation verhandelte Sätze hat, daher entsprechen die Dollar-Zahlen, die Entwickler sehen, Ihrer Rechnung. Claude Code wendet die Sätze in `/usage`, der [Statuszeile](/docs/de/statusline), dem `total_cost_usd` des Agent SDK, dem Limit [`--max-budget-usd`](/docs/de/cli-reference) und der [OpenTelemetry](/docs/de/monitoring-usage)-Kostenmetrik und Ereignissen an. Sie liefern die Sätze: Claude Code liest sie nicht aus Ihrem Vertrag oder der Claude Console. Erfordert Claude Code v2.1.242 oder später.

* **Bereich**: [`Verwaltet`](#scopes). Stellen Sie den Schlüssel durch serverseitig verwaltete Einstellungen, eine MDM-Richtlinie, eine Datei `managed-settings.json` oder einen [Richtlinien-Helfer](/docs/de/managed-settings#compute-the-policy-with-a-helper-program) bereit. Claude Code ignoriert ihn in Benutzer-, Projekt- und Lokaleinstellungen, in `--settings` und unter Windows in der beschreibbaren [HKCU-Registrierung](/docs/de/managed-settings#where-each-mechanism-stores-the-policy). Mit serverseitig verwalteten Einstellungen meldet jede Sitzung Kosten zum Listenpreis, bis die [Einstellungsabruf](/docs/de/server-managed-settings#fetch-and-caching-behavior) dieser Sitzung die Einstellung bestätigt hat. Eine Host-Anwendung, die Claude Code einbettet und [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/de/env-vars) setzt, kann eine Tabelle ihrer eigenen durch die SDK-Option [`managedSettings`](/docs/de/agent-sdk/typescript#options) liefern, die Claude Code nur verwendet, wenn keine verwaltete Quelle den Schlüssel setzt und nur in Claude Code v2.1.246 oder später.
* **Typ**: Objekt mit einem optionalen `multiplier` und einer optionalen Karte `overrides`
* **Standard**: nicht gesetzt, daher meldet Claude Code Listenpreis, es sei denn, eine Host-Anwendung liefert eine Tabelle

Setzen Sie `multiplier` allein für einen pauschalen Rabatt oder Aufschlag, `overrides` allein für Pro-Modell-Sätze oder beide.

Dieses Beispiel setzt verhandelte Sätze für Sonnet 4.6 und reduziert dann jede Zahl, die Sonnet-Zeile eingeschlossen, um 15%:

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

Setzen Sie `multiplier` über 1, bis zu 10, um jede Zahl zu markieren. Ein Aufschlag erfordert Claude Code v2.1.271 oder später. Frühere Versionen ignorieren einen `multiplier` über 1 mit einer Warnung und behalten den Rest der Einstellung.

Für die Schritte, einschließlich wie Sie bestätigen, dass die Sätze in Kraft sind, siehe [Ausgaben zu Ihren verhandelten Sätzen melden](/docs/de/costs#report-spend-at-your-contracted-rates).

<span id="modelpricing-multiplier" />

<span id="modelpricing-overrides" />

<h4 id="fields-for-modelpricing">
  Felder für `modelPricing`
</h4>

| Feld         | Typ                                                                                                              | Was es tut                                                                                                                                                                                                                                                              |
| :----------- | :--------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `multiplier` | Zahl größer als 0 und höchstens 10                                                                               | Skaliert jede Kosten, die Claude Code berechnet, ob oder ob nicht eine `overrides`-Zeile sie abdeckt. Unter 1 ist ein Rabatt, über 1 ein Aufschlag                                                                                                                      |
| `overrides`  | Karte von Modell-ID zu einem Rateobjekt mit `input`, `output`, `cacheRead` und `cacheWrite`, jeweils 0 bis 10000 | Die USD-pro-Million-Token-Sätze für dieses Modell, alle vier erforderlich. `cacheWrite` deckt sowohl Fünf-Minuten- als auch Einstunden-Cache-Schreibvorgänge ab. Siehe [Welche Modelle eine Zeile `modelPricing` anwendet](#which-models-a-modelpricing-row-applies-to) |

Claude Code verwendet die Sätze einer Zeile genau wie Sie sie schrieben, ohne den Schnellmodus-Aufschlag oder den [nur-US-Inferenz-Satz](https://platform.claude.com/docs/en/about-claude/pricing) hinzuzufügen. Wenn Sie auch `multiplier` setzen, wendet Claude Code ihn auf die Sätze der Zeile an. Claude Code löscht eine Zeile mit einem Satz, den es nicht analysieren kann, oder einen `multiplier`, den es nicht analysieren kann, und behält den Rest; siehe [Fehlerhafte Einstellungsdatei beheben](/docs/de/settings#fix-a-broken-settings-file).

<h4 id="which-models-a-modelpricing-row-applies-to">
  Welche Modelle eine `modelPricing`-Zeile anwendet
</h4>

Claude Code entscheidet, welche Modelle eine Zeile anwendet, vom Schlüssel der Zeile:

* **Die ID eines integrierten Modells**: ein Schlüssel, den Claude Code selbst für ein integriertes Modell verwendet, ob dieser Schlüssel die eigene ID des Modells ist, wie `claude-sonnet-4-6`, oder seine Bedrock-, Agent Platform- oder Foundry-ID. Claude Code wendet die Zeile auf jede datierte Snapshot-ID und Anbieter-spezifische ID dieses Modells an.
* **Jeder andere Schlüssel**: ein Schlüssel, der nicht die ID eines integrierten Modells ist, wie ein Gateway-Modellalias. Claude Code wendet die Zeile auf diese eine ID nur an. Wenn eine Modell-ID genau einem Ihrer Schlüssel entspricht und auch unter eine Zeile fällt, die von einer integrierten Modell-ID gekennzeichnet ist, verwendet Claude Code die genaue Übereinstimmung.
* **Ein Bedrock-Anwendungs-Inferenz-Profil**: sobald Claude Code das Profil zum Modell aufgelöst hat, zu dem es leitet, durch Ihre Karte [`modelOverrides`](#modeloverrides) oder die Suche [`bedrock:GetInferenceProfile`](/docs/de/amazon-bedrock#iam-configuration), wendet Claude Code die Zeile dieses Modells auf das Profil an.

<h3 id="modelsettings">
  `modelSettings`
</h3>

Speichern Sie eine [Aufwandsebene](/docs/de/model-config#adjust-effort-level) für jedes Modell, das Sie verwenden. Erfordert Claude Code v2.1.251 oder später.

In einer interaktiven Sitzung auf Ihrem Computer, wenn Sie `low`, `medium`, `high` oder `xhigh` als Ihren Standard mit `/effort` oder dem Aufwands-Schieberegler der Auswahl `/model` speichern, schreibt Claude Code diese Ebene hier unter dem Modell, das Sie verwenden, daher bearbeiten Sie diesen Schlüssel selten selbst. Wenn Sie einen dieser Ebenen in der [Modellauswahl der VS Code-Erweiterung](/docs/de/vs-code#use-the-prompt-box) auswählen, speichert Claude Code ihn auf die gleiche Weise. Der Eintrag [`effortLevel`](#effortlevel) listet die Sitzungen auf, in denen `/effort` nur für diese Sitzung gilt.

Bearbeiten Sie den Schlüssel manuell, um eine Ebene zu ändern oder zu entfernen, die Sie gespeichert haben.

Ein `effortLevel` eines Modells hier hat Vorrang vor dem Top-Level-[`effortLevel`](#effortlevel) in der gleichen Einstellungsdatei. Über Dateien hinweg löst Claude Code jedes Modell separat auf: die höchste Prioritäts-[Einstellungsdatei](/docs/de/settings#settings-precedence), die entweder einen `effortLevel` für dieses Modell oder einen Top-Level-`effortLevel` setzt, der [auf dieses Modell anwendet](#effortlevel), entscheidet, daher übertrumpft ein `effortLevel` in verwalteten Einstellungen eine Ebene, die Sie in Benutzereinstellungen gespeichert haben. [Aufwandsebene anpassen](/docs/de/model-config#adjust-effort-level) listet auf, was sonst noch eine gespeicherte Ebene überschreiben kann, wie `--effort` beim Start.

Um ein Modell zu begrenzen, anstatt seine Ebene zu setzen, fügen Sie ein Feld [`maxEffortLevel`](#maxeffortlevel) zum Eintrag dieses Modells hinzu. Das Feld erfordert Claude Code v2.1.267 oder später.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Objekt, das einen Modellnamen zu einem Objekt mit einem Feld `effortLevel`, einem von `"low"`, `"medium"`, `"high"` oder `"xhigh"`, einem Feld [`maxEffortLevel`](#maxeffortlevel) oder beiden zuordnet
* **Standard**: nicht gesetzt

Claude Code schreibt jeden Eintrag unter dem kanonischen Namen des Modells, wie `claude-opus-5-5`, und ordnet den Alias, das Datum-Suffix, `[1m]` und erkannte Anbieter-spezifische IDs dieses Modells dem gleichen Eintrag zu.

Dieses Beispiel hält Opus 5.5 auf `high`, während andere Modelle ihre eigenen gespeicherten oder Standard-Ebenen verwenden:

```json settings.json theme={null}
{
  "modelSettings": {
    "claude-opus-5-5": {
      "effortLevel": "high"
    }
  }
}
```

Führen Sie `/effort auto` aus, um Ihre gespeicherte Ebene für das Modell zu löschen, das Sie verwenden. Claude Code lässt die anderen Einträge und jeden Top-Level-`effortLevel` in Kraft.

<h3 id="outputstyle">
  `outputStyle`
</h3>

Wählen Sie einen [Ausgabestil](/docs/de/output-styles) nach Name. Ein Ausgabestil ist ein gespeicherter Satz von Anweisungen, der Claudes Rolle, Ton und Ausgabeformat ändert, wie die integrierten Explanatory- und Learning-Stile oder einen, den Sie selbst geschrieben haben.

Wenn Sie diesen Schlüssel während einer Sitzung ändern, verwendet Claude den neuen Stil ab Ihrer nächsten Nachricht. Für was diese Nachricht im Prompt-Cache kostet, siehe [Ausgabestil ändern](/docs/de/prompt-caching#changing-output-style). Vor v2.1.251 galt die Bearbeitung nur nach dem Ausführen von `/clear` oder dem Starten einer neuen Sitzung.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: String, der Name eines [integrierten](/docs/de/output-styles#built-in-output-styles) oder [benutzerdefinierten](/docs/de/output-styles#create-a-custom-output-style) Ausgabestils
* **Standard**: nicht gesetzt, daher verwendet Claude Code den Standard-Stil

Dieses Beispiel wählt den integrierten Explanatory-Stil, der zwischen Aufgaben pädagogische Einblicke hinzufügt:

```json settings.json theme={null}
{
  "outputStyle": "Explanatory"
}
```

<h3 id="promptcachettl">
  `promptCacheTtl`
</h3>

Wählen Sie, wie lange der [Prompt-Cache](/docs/de/prompt-caching) die Hauptkonversation hält. Dieser Schlüssel gilt für Ihre interaktiven, `-p` und Agent SDK-Durchläufe, zusammen mit den Helfern, die Claude Code inline mit ihnen ausführt. Die Einstunden-Lebensdauer hält den Cache über längere Pausen warm, und die API [berechnet jeden Cache-Schreibvorgang zu einem höheren Satz](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) als bei der Fünf-Minuten-Lebensdauer. Erfordert Claude Code v2.1.242 oder später.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: String, einer von:
  * `"5m"`: der Cache hält für fünf Minuten
  * `"1h"`: der Cache hält für eine Stunde
* **Standard**: nicht gesetzt, daher erhält jede Hauptkonversations-Anfrage [ihre Standard-Lebensdauer](/docs/de/prompt-caching#which-ttl-each-request-gets)
* **Sitzungsübergreifende Außerkraftsetzungen**: [`FORCE_PROMPT_CACHING_5M`](/docs/de/env-vars) hat Vorrang vor allem anderen, dann [`CLAUDE_CODE_PROMPT_CACHE_TTL`](/docs/de/env-vars), dann dieser Schlüssel, und zuletzt [`ENABLE_PROMPT_CACHING_1H`](/docs/de/env-vars)

Dieses Beispiel hält die Hauptkonversation auf der Einstunden-Lebensdauer und lässt Subagents auf fünf Minuten:

```json settings.json theme={null}
{
  "promptCacheTtl": "1h",
  "subagentPromptCacheTtl": "5m"
}
```

Für was jede Lebensdauer kostet, siehe [Cache-Lebensdauer](/docs/de/prompt-caching#cache-lifetime).

<h3 id="showthinkingsummaries">
  `showThinkingSummaries`
</h3>

Sehen Sie Zusammenfassungen von Claudes [erweitertem Denken](/docs/de/model-config#extended-thinking) in interaktiven Sitzungen. Setzen Sie es, wenn Sie die vollständigen Zusammenfassungen möchten, wenn Sie das Denken mit `Ctrl+O` erweitern. Wenn nicht gesetzt oder `false`, redaktioniert die Anthropic API Denk-Blöcke und Claude Code zeigt einen zusammengefassten Stub; Drittanbieter-Provider redaktionieren nicht.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: Sie sehen vollständige Denk-Zusammenfassungen, wenn Sie das Denken mit `Ctrl+O` erweitern
  * `false`: die Anthropic API redaktioniert Denk-Blöcke und Claude Code zeigt einen zusammengefassten Stub
* **Standard**: `false`

```json settings.json theme={null}
{
  "showThinkingSummaries": true
}
```

Redaktion ändert nur, was Sie sehen, nicht was das Modell generiert. Um Denk-Ausgaben zu reduzieren, [senken Sie das Budget oder deaktivieren Sie das Denken](/docs/de/model-config#extended-thinking) stattdessen.

<h3 id="subagentpromptcachettl">
  `subagentPromptCacheTtl`
</h3>

Wählen Sie, wie lange der [Prompt-Cache](/docs/de/prompt-caching) die Anfragen hält, die Claude Code außerhalb der Hauptkonversation macht. Dieser Schlüssel gilt für [Subagents](/docs/de/sub-agents), [Workflows](/docs/de/workflows) und Claudes eigene Hintergrund- und Hilfsanfragen, wie Komprimierung und Sitzungstitel. Die Einstunden-Lebensdauer hält den Cache über längere Pausen warm, und die API [berechnet jeden Cache-Schreibvorgang zu einem höheren Satz](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) als bei der Fünf-Minuten-Lebensdauer. Erfordert Claude Code v2.1.242 oder später.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: String, einer von:
  * `"5m"`: der Cache hält für fünf Minuten
  * `"1h"`: der Cache hält für eine Stunde
* **Standard**: nicht gesetzt, daher erhält jede dieser Anfragen [ihre Standard-Lebensdauer](/docs/de/prompt-caching#which-ttl-each-request-gets)
* **Sitzungsübergreifende Außerkraftsetzungen**: [`FORCE_PROMPT_CACHING_5M`](/docs/de/env-vars) hat Vorrang vor allem anderen, dann [`CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`](/docs/de/env-vars), dann dieser Schlüssel, dann [`ENABLE_PROMPT_CACHING_1H`](/docs/de/env-vars), das die Einstunden-Lebensdauer bei jeder Anfrage anfordert. Für wo der Wert der eigenen Frontmatter eines Subagents rangiert, siehe [Wählen Sie die TTL selbst](/docs/de/prompt-caching#choose-the-ttl-yourself)

Dieses Beispiel gibt Subagents und den anderen Anfragen außerhalb der Hauptkonversation die Einstunden-Lebensdauer:

```json settings.json theme={null}
{
  "subagentPromptCacheTtl": "1h"
}
```

Dieser Schlüssel deckt die Anfragen ab, die [`promptCacheTtl`](#promptcachettl) nicht abdeckt, daher setzen Sie beide, um eine Lebensdauer für jede Anfrage zu wählen, die Claude Code macht. Für wie sich der Cache eines Subagents vom Cache der Hauptkonversation unterscheidet, siehe [Subagents und der Cache](/docs/de/prompt-caching#subagents-and-the-cache).

<h3 id="switchmodelsonflag">
  `switchModelsOnFlag`
</h3>

Wählen Sie, was passiert, wenn ein [Sicherheits-Klassifizierer eine Anfrage kennzeichnet](/docs/de/model-config#automatic-model-fallback): zum Fallback-Modell wechseln und fortfahren, oder pausieren, damit Sie zwischen Wechsel und Bearbeitung der Eingabeaufforderung wählen können.

* **Bereich**: [`Beliebige Datei`](#scopes). Erscheint in `/config` als **Modelle wechseln, wenn eine Nachricht gekennzeichnet ist**.
* **Typ**: Boolean
  * `true`: Claude Code wechselt zum Fallback-Modell und fährt fort
  * `false`: in einer interaktiven Sitzung pausiert Claude Code, damit Sie zwischen Wechsel und Bearbeitung der Eingabeaufforderung wählen können; wo kein Dialog angezeigt werden kann, wie ein `-p`-Lauf, endet die gekennzeichnete Anfrage als Fehler
* **Standard**: `true`, automatisch wechseln

```json settings.json theme={null}
{
  "switchModelsOnFlag": false
}
```

Siehe [Vor dem Wechsel fragen](/docs/de/model-config#ask-before-switching).

<h3 id="ultracode">
  `ultracode`
</h3>

Starten Sie Sitzungen mit [Ultracode](/docs/de/workflows#let-claude-decide-with-ultracode) an. Mit ihm an, plant Claude einen Workflow für jede wesentliche Aufgabe, anstatt auf Sie zu warten, um zu fragen. Claude plant Workflows nur, wenn [dynamische Workflows](/docs/de/workflows) für Sie aktiviert sind, Ihr Modell `xhigh`-Aufwand unterstützt und kein [Aufwandslimit](/docs/de/model-config#organization-effort-limits) unter `xhigh` gilt. Auf jeden Fall läuft `ultracode: true` die Sitzung auf `xhigh`-Aufwand oder bei der Obergrenze, wenn ein Aufwandslimit niedriger ist. Claude Code liest diesen Schlüssel, schreibt ihn aber nie: `/effort ultracode` aktiviert Ultracode nur für die aktuelle Sitzung.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: Sitzungen starten auf `xhigh`-Aufwand, mit Ultracode an, wenn dynamische Workflows für Sie aktiviert sind, Ihr Modell `xhigh` unterstützt und kein Aufwandslimit unter `xhigh` ist
  * `false`: Sitzungen starten mit Ultracode aus
* **Standard**: nicht gesetzt, daher ist Ultracode aus
* **Sitzungsübergreifende Außerkraftsetzungen**: `/effort ultracode` aktiviert Ultracode für eine Sitzung ohne diesen Schlüssel. Das Flag `--effort ultracode` aktiviert es auch für eine Sitzung und erfordert Claude Code v2.1.203 oder später

```json settings.json theme={null}
{
  "ultracode": true
}
```

Ultracode läuft die Sitzung auf `xhigh`-Aufwand und hat Vorrang vor `effortLevel` und [`modelSettings`](#modelsettings)-Einträgen. Wenn ein [Aufwandslimit](/docs/de/model-config#organization-effort-limits) unter `xhigh` auf das Modell anwendet, wie eine [`maxEffortLevel`](#maxeffortlevel)-Einstellung, läuft die Sitzung stattdessen bei der Obergrenze und Ultracode bleibt aus. Claude plant dann keine Workflows von selbst, und `/effort` bietet nicht `ultracode` an. Eine Agent SDK `apply_flag_settings`-Kontrollabfrage akzeptiert auch den Schlüssel.

<h2 id="permission-settings">
  Berechtigungseinstellungen
</h2>

Entscheiden Sie, was Claude ohne Nachfrage tun kann, in welchem Berechtigungsmodus eine Sitzung startet, und was der Klassifizierer des Auto-Modus zulässt. Informationen zur Regelsyntax und zum Berechtigungsmodell finden Sie unter [Berechtigungen konfigurieren](/docs/de/permissions).

<h3 id="allowmanagedpermissionrulesonly">
  `allowManagedPermissionRulesOnly`
</h3>

Machen Sie verwaltete Einstellungen zur einzigen Quelle für Berechtigungsregeln. Claude Code ignoriert dann `allow`-, `ask`- und `deny`-Regeln in Benutzer-, Projekt-, lokalen und `--settings`-Dateien, ignoriert `--allowedTools`, blendet die Optionen zum Immer-Zulassen in Berechtigungsaufforderungen aus und speichert keine neuen Regeln.

Wenn [übergeordnete Einstellungen von einem Embedding-Host](/docs/de/managed-settings#let-an-embedding-host-add-policy) gelten, behandelt Claude Code diese als Teil der verwalteten Ebene. Es verwirft deren `allow`-Regeln und `additionalDirectories` und behält deren `deny`- und `ask`-Regeln bei, außer `Read`- und `Edit`-Regeln, deren Muster mit `!` beginnt. Ein Host kann Pfade aus den verwalteten Regeln nicht mit einer `!`-Regel ausschneiden, unabhängig davon, ob Sie diesen Schlüssel setzen oder nicht.

`--disallowedTools`-Regeln und die `deny`- und `ask`-Regeln der aktuellen Sitzung gelten weiterhin, auch nachdem Claude Code die Einstellungen während der Sitzung neu lädt. Sie beschränken nur, daher können sie nicht erweitern, was die verwalteten Regeln gewähren. Vor v2.1.257 verwarf Claude Code diese Befehlszeilen- und Sitzungsregeln beim ersten Neuladen der Einstellungen.

Informationen dazu, was ein `!`-Muster in einer `--disallowedTools`- oder Sitzungsregel ausschneiden kann, finden Sie unter [Read- und Edit-Regeln](/docs/de/permissions#read-and-edit).

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Boolean
  * `true`: verwaltete Einstellungen werden zur einzigen Quelle für Berechtigungsregeln
  * `false`: Claude Code wendet Berechtigungsregeln aus Benutzer-, Projekt-, lokalen und `--settings`-Dateien zusätzlich zu den verwalteten an
* **Standard**: nicht gesetzt, daher wendet Claude Code Berechtigungsregeln aus Benutzer-, Projekt- und lokalen Einstellungen sowie aus `--settings` zusätzlich zu den verwalteten an

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true
}
```

Dieser Schlüssel sperrt nicht die MCP-Server-Zulassungsliste; verwenden Sie dazu [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly). Siehe [Nur verwaltete Einstellungen](/docs/de/managed-settings#managed-only-settings).

<h3 id="automode">
  `autoMode`
</h3>

Fügen Sie Ihre eigenen Regeln zu dem hinzu, was der [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode)-Klassifizierer blockiert und zulässt. Verwenden Sie es, um dem Klassifizierer mitzuteilen, welche Repos, Buckets und Domains Ihre Organisation vertraut, damit er routinemäßige interne Operationen nicht mehr blockiert. Der Klassifizierer wird mit [integrierten Allow- und Deny-Regeln](/docs/de/auto-mode-config#inspect-the-defaults-and-your-effective-config) ausgeliefert. Fügen Sie die Literalzeichenfolge `"$defaults"` in einem Array ein, um diese integrierten Regeln an dieser Position zu behalten und Ihre Regeln um sie herum hinzuzufügen; lassen Sie sie weg, um sie durch Ihre zu ersetzen.

* **Bereich**: [`User or managed`](#scopes)
* **Typ**: Objekt mit `environment`-, `allow`-, `soft_deny`- und `hard_deny`-Arrays von Prosa-Regeln sowie dem Boolean [`classifyAllShell`](#automode-classifyallshell)
* **Standard**: nicht gesetzt, daher verwendet der Klassifizierer nur seine [integrierten Regeln](/docs/de/auto-mode-config#inspect-the-defaults-and-your-effective-config)

Dieses Beispiel behält die integrierten `soft_deny`-Regeln durch `"$defaults"` und fügt eine weitere hinzu, die `terraform apply` blockiert:

```json settings.json theme={null}
{
  "autoMode": {
    "soft_deny": ["$defaults", "Never run terraform apply"]
  }
}
```

Wenn mehr als eine dieser Dateien dasselbe Array setzt, verkettet Claude Code die Einträge. Informationen zum Regelformat und zur Anwendung der einzelnen Arrays finden Sie unter [Auto-Modus konfigurieren](/docs/de/auto-mode-config).

<h3 id="automode-classifyallshell">
  `autoMode.classifyAllShell`
</h3>

Senden Sie jeden Bash- und PowerShell-Befehl durch den Auto-Modus-Klassifizierer, während der Auto-Modus aktiv ist. Standardmäßig setzt der Auto-Modus nur Allow-Regeln aus, die beliebigen Code ausführen könnten: Tool-weite und Wildcard-Regeln wie `Bash(*)` sowie Interpreter- oder Shell-Wrapper-Präfixe wie `Bash(python *)`. Ein Befehl, der einer anderen Allow-Regel entspricht, wie `Bash(npm test)`, überspringt den Klassifizierer, es sei denn, er trägt [Pro-Befehl zulässige Domains](/docs/de/sandboxing#per-command-allowed-domains-in-auto-mode). Wenn er überspringt, kann ein destruktives Argument, das das Präfix der Regel nicht erwartet hat, ungesehen durchkommen. Das Setzen dieses Schlüssels setzt jede Shell-Allow-Regel für die Sitzung aus, damit der Klassifizierer jeden Befehl sieht. Erfordert Claude Code v2.1.193 oder später.

* **Bereich**: [`User or managed`](#scopes). Lesen Sie überall dort, wo [`autoMode`](#automode) gelesen wird.
* **Typ**: Boolean
  * `true`: Während der Auto-Modus aktiv ist, sendet Claude Code jeden Bash- und PowerShell-Befehl durch den Klassifizierer und setzt Ihre Shell-Allow-Regeln aus; außerhalb des Auto-Modus gelten die Regeln weiterhin
  * `false`: Der Auto-Modus setzt nur Allow-Regeln aus, die beliebigen Code ausführen könnten, wie `Bash(*)` und `Bash(python *)`; ein Befehl, der einer anderen Allow-Regel entspricht, überspringt den Klassifizierer, es sei denn, er trägt [Pro-Befehl zulässige Domains](/docs/de/sandboxing#per-command-allowed-domains-in-auto-mode), und jeder andere Shell-Befehl wird durch ihn geleitet
* **Standard**: `false`

```json settings.json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Siehe [Alle Shell-Befehle durch den Klassifizierer leiten](/docs/de/auto-mode-config#route-all-shell-commands-through-the-classifier). Erfordert Claude Code v2.1.193 oder später.

<h3 id="disableautomode">
  `disableAutoMode`
</h3>

Entfernen Sie den [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) aus dem `Shift+Tab`-Zyklus. Jede Sitzung, die sonst [im Auto-Modus starten würde](/docs/de/permission-modes#which-mode-a-session-starts-in), ob von `--permission-mode auto`, einer Einstellungsdatei oder dem integrierten Standard, startet stattdessen im `default`-Modus. Administratoren setzen es in verwalteten Einstellungen, um zu verhindern, dass Entwickler in ihrer Organisation den Auto-Modus verwenden.

* **Bereich**: [`Any file`](#scopes). Am nützlichsten in [verwalteten Einstellungen](/docs/de/managed-settings), wo Benutzer es nicht überschreiben können. Auch unter `permissions` als `permissions.disableAutoMode` akzeptiert.
* **Typ**: die Zeichenfolge `"disable"`
* **Standard**: nicht gesetzt

```json settings.json theme={null}
{
  "disableAutoMode": "disable"
}
```

<h3 id="permissions">
  `permissions`
</h3>

Steuern Sie, welche Tools Claude ohne Nachfrage verwenden kann, welche immer eine Aufforderung anzeigen, und welche blockiert sind, und legen Sie den [Berechtigungsmodus](/docs/de/permission-modes) fest, in dem eine Sitzung startet. Jeder `permissions.*`-Schlüssel unten verschachtelt sich unter diesem Objekt.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Objekt mit `allow`, `ask`, `deny`, `additionalDirectories`, `blockReadsOutsideWorkingDirectories`, `defaultMode`, `disableBypassPermissionsMode` und `disableAutoMode`
* **Standard**: nicht gesetzt

Dieses Beispiel genehmigt `npm run`-Befehle ohne Nachfrage, fordert vor `git push` auf, blockiert Lesevorgänge von `.env` und startet Sitzungen im `acceptEdits`-Modus:

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

Die drei Regel-Arrays teilen sich eine Syntax; siehe [Berechtigungsregelsyntax](#permission-rule-syntax) unter `permissions.allow`. Informationen dazu, wie Berechtigungsregeln aus verschiedenen Dateien kombiniert werden, finden Sie unter [Wie Berechtigungsregeln über Bereiche hinweg zusammengeführt werden](/docs/de/permissions#settings-precedence); Informationen dazu, wie Einstellungsschlüssel im Allgemeinen kombiniert werden, finden Sie unter [Einstellungspriorität](/docs/de/settings#settings-precedence) im Einstellungshandbuch.

<h3 id="useautomodeduringplan">
  `useAutoModeDuringPlan`
</h3>

Wählen Sie, ob Claude Code den Auto-Modus-Klassifizierer verwendet, um Shell-Befehle im Plan-Modus zu überprüfen. Mit dem Standard `true` überprüft der Klassifizierer jeden Befehl während der Planung, wenn der Auto-Modus verfügbar ist, und Sie sehen keine Aufforderung. Setzen Sie `false`, um für jeden Befehl außerhalb des integrierten schreibgeschützten Satzes eine Berechtigungsaufforderung zu erhalten. Wird in `/config` als **Auto-Modus während Plan verwenden** angezeigt.

* **Bereich**: [`User, local, or managed`](#scopes). Ein Repository kann es nicht für Sie ausschalten.
* **Typ**: Boolean
  * `true`: dasselbe wie nicht gesetzt; wenn der Auto-Modus verfügbar ist, überprüft der Klassifizierer jeden Shell-Befehl während der Planung, anstatt Sie dafür aufzufordern. Ein `false` in einer dieser Dateien schaltet es immer noch aus
  * `false`: Sie erhalten eine Berechtigungsaufforderung für jeden Befehl außerhalb des integrierten schreibgeschützten Satzes
* **Standard**: `true`

```json settings.json theme={null}
{
  "useAutoModeDuringPlan": false
}
```

<h3 id="permissions-allow">
  `permissions.allow`
</h3>

Listen Sie die Tool-Verwendungen auf, die Claude Code ohne Nachfrage genehmigt. In einer MCP-Regel kann `*` nur im Tool-Namen nach dem `mcp__<server>__`-Präfix erscheinen, wie `mcp__github__get_*`; es kann nicht im Server-Namen erscheinen.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Berechtigungsregel-Zeichenfolgen
* **Standard**: nicht gesetzt
* **Sitzungsspezifische Überschreibungen**: `--allowedTools` fügt Allow-Regeln für eine Sitzung hinzu, und eine Deny-Regel aus einer Einstellungsdatei blockiert immer noch ein Tool, das sie benennt

Dieses Beispiel genehmigt `git diff` und lässt Claude Code Ihre `.zshrc` ohne Nachfrage lesen:

```json settings.json theme={null}
{
  "permissions": {
    "allow": ["Bash(git diff *)", "Read(~/.zshrc)"]
  }
}
```

Claude Code wendet `allow`-Regeln aus der `.claude/settings.json` eines Projekts nur an, nachdem Sie den [Workspace-Trust-Dialog](/docs/de/permissions#project-allow-rules-and-workspace-trust) für diesen Ordner akzeptiert haben.

<h4 id="permission-rule-syntax">
  Berechtigungsregelsyntax
</h4>

Berechtigungsregeln folgen dem Format `Tool` oder `Tool(specifier)`. Claude Code wertet `deny`-Regeln zuerst aus, dann `ask`, dann `allow`, und die erste Übereinstimmung entscheidet, unabhängig davon, wie spezifisch jede Regel ist; siehe die [Berechtigungsregel-Evaluierungsreihenfolge](/docs/de/permissions#manage-permissions).

Jede Zeile zeigt eine Regelform und was sie entspricht.

| Regel                          | Was sie entspricht                  |
| :----------------------------- | :---------------------------------- |
| `Bash`                         | Jeder Bash-Befehl                   |
| `Bash(npm run *)`              | Befehle, die mit `npm run` beginnen |
| `Read(./.env)`                 | Lesevorgänge der `.env`-Datei       |
| `WebFetch(domain:example.com)` | Fetch-Anfragen an example.com       |

Für die vollständige Regelsyntax, einschließlich Wildcard-Verhalten, Tool-spezifischer Muster für Read, Edit, WebFetch, MCP und Agent-Regeln sowie der Sicherheitsbeschränkungen von Bash-Mustern, siehe [Berechtigungsregelsyntax](/docs/de/permissions#permission-rule-syntax).

<h3 id="permissions-ask">
  `permissions.ask`
</h3>

Listen Sie die Tool-Verwendungen auf, die Sie zur Bestätigung auffordern, auch in einem Berechtigungsmodus, der sie sonst genehmigen würde, wie `acceptEdits` oder `bypassPermissions`. Im `dontAsk`-Modus verweigert Claude Code eine entsprechende Tool-Verwendung, anstatt aufzufordern.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Berechtigungsregel-Zeichenfolgen
* **Standard**: nicht gesetzt

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

Listen Sie die Tool-Verwendungen auf, die Claude Code blockiert. Verwenden Sie es für Dateien, die API-Schlüssel, Geheimnisse oder Umgebungswerte enthalten: Claude Code schließt entsprechende Dateien aus der Dateiermittlung und Suchergebnissen aus, verweigert Lesevorgänge dafür und blockiert die [Edit- und Write-Tools](/docs/de/permissions#read-and-edit) auf den entsprechenden Pfaden.

Read- und Edit-Deny-Regeln gelten für Claude's integrierte Datei-Tools, für Dateibefehle, die Claude Code in Bash erkennt, wie `cat`, `head`, `tail`, `sed` und `tee`, und für die Ziele von Bash-[Umleitungen](/docs/de/permissions#redirections) wie `> file` und `< file`; sie gelten nicht für einen Befehl, der Dateien liest, ohne sie zu benennen, wie `grep -r pattern .`, oder für beliebige Unterprozesse, daher [aktivieren Sie die Sandbox](/docs/de/sandboxing) für OS-Ebenen-Durchsetzung.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Berechtigungsregel-Zeichenfolgen
* **Standard**: nicht gesetzt
* **Sitzungsspezifische Überschreibungen**: `--disallowedTools` fügt Deny-Regeln für eine Sitzung neben diesem Schlüssel hinzu

Dieses Beispiel verweigert Lesevorgänge von `.env`-Dateien, dem `secrets`-Verzeichnis und einer Credentials-Datei und blockiert `curl`-Befehle:

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

Tool-Namen akzeptieren Glob-Muster, daher verweigert `"*"` jedes Tool und `"mcp__*"` verweigert jedes MCP-Tool. Claude Code ignoriert eine Deny-Regel für das [`EndConversation`](/docs/de/tools-reference#endconversation-tool-behavior)-Tool, solange noch ein anderes Tool für Claude verfügbar ist. Eine `Bash`-Deny-Regel entspricht dem Befehl, wie Claude ihn schreibt, daher stoppt `Bash(curl *)` nicht `/usr/bin/curl` oder `sh -c 'curl …'`; siehe [was eine Bash-Regel nicht entspricht](/docs/de/permissions#bash-rule-limits). Dieser Schlüssel ersetzt die veraltete `ignorePatterns`-Konfiguration.

<h3 id="permissions-additionaldirectories">
  `permissions.additionalDirectories`
</h3>

Geben Sie Claude Dateizugriff auf Verzeichnisse außerhalb desjenigen, in dem Sie gestartet haben, als zusätzliche [Arbeitsverzeichnisse](/docs/de/permissions#working-directories). Die meisten `.claude/`-Konfigurationen werden [nicht ermittelt](/docs/de/permissions#additional-directories-grant-file-access-not-configuration) aus diesen Verzeichnissen.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Verzeichnispfaden
* **Standard**: nicht gesetzt
* **Sitzungsspezifische Überschreibungen**: `--add-dir` und `/add-dir` fügen Verzeichnisse für eine Sitzung neben diesem Schlüssel hinzu

```json settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": ["../docs/"]
  }
}
```

Wie `allow`-Regeln werden Einträge in der `.claude/settings.json` eines Projekts nur wirksam, nachdem Sie den [Workspace-Trust-Dialog](/docs/de/permissions#project-allow-rules-and-workspace-trust) für diesen Ordner akzeptiert haben.

<h3 id="permissions-blockreadsoutsideworkingdirectories">
  `permissions.blockReadsOutsideWorkingDirectories`
</h3>

Verhindern Sie, dass Claude Pfade außerhalb der [Arbeitsverzeichnisse](/docs/de/permissions#working-directories) der Sitzung mit den Read-, Grep-, Glob- und LSP-Tools liest, in jedem Berechtigungsmodus, einschließlich `bypassPermissions`. Ein Bash-Befehl, der einen entsprechenden Pfad durch einen Dateiberfehl liest, den Claude Code erkennt, wie `cat`, fordert Sie auf, auch im Auto-Modus und `bypassPermissions`-Modus. Erfordert Claude Code v2.1.257 oder später.

Ein Bash-Befehl, den der Shell-Parser nicht verfolgen kann, wie einer, der das Verzeichnis mehr als einmal wechselt oder eine Subshell ausführt, fordert Sie auf, auch im Auto-Modus und `bypassPermissions`-Modus. Die Aufforderung wird angezeigt, auch wenn der Befehl keinen Pfad außerhalb der Arbeitsverzeichnisse benennt. Diese Aufforderung gilt nicht, wenn der Befehl in der [Sandbox](/docs/de/sandboxing) ausgeführt wird und die Sandbox die Blockierung durchsetzt.

Claude Code schreibt auch `true` hier, wenn Sie sich entscheiden, solche Lesevorgänge auf [Auto-Modus-Aufforderung vor dem ersten Lesevorgang außerhalb der Arbeitsverzeichnisse](/docs/de/permission-modes#first-read-outside-the-working-directories) zu blockieren.

* **Bereich**: [`Any file`](#scopes). Wenn eine Einstellungsquelle `true` setzt, gilt die Blockierung, daher kann die eingecheckte Datei eines Repositorys die Blockierung für ein Projekt aktivieren, kann aber eine Blockierung, die Sie setzen, nicht aufheben.
* **Typ**: Boolean
  * `true`: Dateilesevorgänge außerhalb der Arbeitsverzeichnisse werden blockiert
  * `false`: dasselbe wie nicht gesetzt; ein `true` in einer anderen Einstellungsdatei blockiert immer noch
* **Standard**: nicht gesetzt, daher folgen Lesevorgänge außerhalb der Arbeitsverzeichnisse Ihrem Berechtigungsmodus und Ihren Regeln

```json settings.json theme={null}
{
  "permissions": {
    "blockReadsOutsideWorkingDirectories": true
  }
}
```

Wenn nur die eingecheckte Einstellungsdatei eines Repositorys ein Verzeichnis hinzufügt, gilt die Blockierung immer noch für Lesevorgänge dort. Wenn [`autoMemoryDirectory`](#automemorydirectory) aus der `.claude/settings.json` des Projekts kommt oder aus einer `.claude/settings.local.json` [als Repository-bereitgestellt behandelt](/docs/de/permissions#when-your-local-settings-file-needs-trust), lädt Claude Code kein [Auto-Memory](/docs/de/memory#storage-location) aus diesem Verzeichnis und speichert keines darin. Dateien, die Claude Code selbst benötigt, bleiben lesbar, wie Ihre Skills, Plugins, Regeln, Agents, Befehle und die `CLAUDE.md`-Speicherdatei unter `~/.claude/`.

Wenn die [Sandbox](/docs/de/sandboxing) aktiviert ist, verweigert die Blockierung auch sandboxierten Befehlen Lesezugriff auf Home-Verzeichnisse und bereitgestellte Volume-Roots außerhalb der Arbeitsverzeichnisse. Ein Wiederholungsversuch, der Genehmigung benötigt, um [außerhalb der Sandbox](/docs/de/sandboxing#the-unsandboxed-retry-escape-hatch) zu laufen, fordert Sie auf, auch im `bypassPermissions`-Modus. Dateien, die ein Tool aus Ihrem Home-Verzeichnis liest, wie `~/.gitconfig`, werden mit dem Rest verweigert; öffnen Sie einen bestimmten Pfad mit [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread) erneut, wenn ein Tool ihn benötigt.

Wenn das Arbeitsverzeichnis der Sitzung ein verknüpftes [Git Worktree](/docs/de/worktrees) ist, einschließlich eines, das Claude Code während der Sitzung eingegeben hat, bleibt das gemeinsame `.git`-Verzeichnis des Repositorys für sandboxierte Befehle lesbar und beschreibbar, damit Git dort funktioniert.

<h3 id="permissions-defaultmode">
  `permissions.defaultMode`
</h3>

Legen Sie den [Berechtigungsmodus](/docs/de/permission-modes) fest, in dem neue Sitzungen starten. Wenn Sie ihn nicht gesetzt lassen, starten Sitzungen im [integrierten Standard](/docs/de/permission-modes#which-mode-a-session-starts-in) für Ihren Plan und Ihre Oberfläche.

* **Bereich**: [`Any file`](#scopes). `auto` und `bypassPermissions` werden nicht aus Projekt- oder lokalen Einstellungen wirksam, daher setzen Sie sie stattdessen in `~/.claude/settings.json`. Vor v2.1.257 wurde `bypassPermissions` aus jeder Datei wirksam. Für Konversationen, die die VS Code-Erweiterung startet, liest Claude Code nur Benutzer-, verwaltete und `--settings`-Werte.
* **Typ**: Zeichenfolge, eine von:
  * `"default"`: Claude Code führt nur Lesevorgänge ohne Nachfrage aus
  * `"acceptEdits"`: Claude Code führt auch Dateibearbeitungen und häufige Dateisystem-Befehle wie `mkdir` und `mv` ohne Nachfrage aus
  * `"plan"`: Claude Code liest und plant, blockiert aber Bearbeitungen, bis Sie einen Plan genehmigen
  * `"auto"`: Claude Code führt alles aus, mit Hintergrund-Sicherheitsprüfungen
  * `"dontAsk"`: Claude Code verweigert automatisch jeden Aufruf, der sonst auffordern würde; Lesevorgänge, andere Aktionen, die keine Genehmigung benötigen, und vorab genehmigte Tools werden weiterhin ausgeführt
  * `"bypassPermissions"`: Claude Code führt alles ohne Nachfrage aus
  * `"manual"`: ein Alias für `"default"`, in Claude Code v2.1.200 oder später
* **Standard**: nicht gesetzt
* **Sitzungsspezifische Überschreibungen**: `--permission-mode` und sein Äquivalent `--dangerously-skip-permissions` für `bypassPermissions` haben Vorrang vor diesem Schlüssel für eine Sitzung

```json settings.json theme={null}
{
  "permissions": {
    "defaultMode": "acceptEdits"
  }
}
```

Berechtigungsregeln überlagern jeden Modus: `deny`-Regeln blockieren in jedem Modus, einschließlich `bypassPermissions`. Siehe [Berechtigungsmodi](/docs/de/permission-modes). `manual` benennt den Berechtigungsmodus mit der Bezeichnung Manual in der CLI und der VS Code-Erweiterung; der Alias erfordert Claude Code v2.1.200 oder später. In Cloud-Sitzungen ehrt Claude Code nur `acceptEdits`, `plan`, `default` und `auto` aus diesem Schlüssel. Für Konversationen, die die VS Code-Erweiterung startet, siehe [welche Einstellung die Erweiterung für den Startberechtigungsmodus liest](/docs/de/permission-modes#switch-permission-modes).

<h3 id="permissions-disablebypasspermissionsmode">
  `permissions.disableBypassPermissionsMode`
</h3>

Verhindern Sie, dass jemand den `bypassPermissions`-Modus betritt. Claude Code lehnt dann das `--dangerously-skip-permissions`-Flag ab und ignoriert eine [Agent-Definition's](/docs/de/sub-agents#permission-modes) `permissionMode: bypassPermissions`, daher wird der Subagent mit dem Berechtigungsmodus der übergeordneten Sitzung ausgeführt.

* **Bereich**: [`Any file`](#scopes). Typischerweise in [verwalteten Einstellungen](/docs/de/managed-settings) gesetzt, um Organisationsrichtlinien durchzusetzen.
* **Typ**: die Zeichenfolge `"disable"`
* **Standard**: nicht gesetzt
* **Sitzungsspezifische Überschreibungen**: Dieser Schlüssel hat Vorrang vor `--dangerously-skip-permissions`, das Claude Code ablehnt, während der Schlüssel gesetzt ist

```json settings.json theme={null}
{
  "permissions": {
    "disableBypassPermissionsMode": "disable"
  }
}
```

Vor v2.1.223 wendete Claude Code den Frontmatter-Berechtigungsmodus auch mit deaktiviertem Bypass an.

<h3 id="skipautopermissionprompt">
  `skipAutoPermissionPrompt`
</h3>

Überspringen Sie die einmalige Mitteilung, die den [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) beschreibt und die Claude Code anzeigt, wenn Sie selbst zum ersten Mal den Auto-Modus betreten, beispielsweise durch Ihre eigenen Einstellungen oder den Modus-Selektor, anstatt wenn der integrierte Standard eine Sitzung darin startet. Claude Code zeigt diese Mitteilung einmal an und zeichnet dann auf, dass sie angezeigt wurde, daher ist dieser Schlüssel nur relevant, wenn die Mitteilung noch nicht angezeigt wurde.

* **Bereich**: [`User or managed`](#scopes). Ein Repository kann es nicht für Sie setzen.
* **Typ**: Boolean
  * `true`: Claude Code überspringt die Mitteilung
  * `false`: dasselbe wie nicht gesetzt; die Mitteilung wird einmal angezeigt, es sei denn, eine andere dieser Dateien setzt `true`
* **Standard**: nicht gesetzt, daher wird die Mitteilung einmal angezeigt

```json settings.json theme={null}
{
  "skipAutoPermissionPrompt": true
}
```

<h3 id="skipdangerousmodepermissionprompt">
  `skipDangerousModePermissionPrompt`
</h3>

Überspringen Sie den Bestätigungsdialog, den Claude Code anzeigt, bevor eine Sitzung den `bypassPermissions`-Modus betritt, ob von `--dangerously-skip-permissions` oder von `defaultMode: "bypassPermissions"`. Claude Code schreibt `true` hier in Ihre Benutzereinstellungen, wenn Sie diesen Dialog einmal akzeptieren.

* **Bereich**: [`User, local, or managed`](#scopes). Ein nicht vertrauenswürdiges Repository kann den Dialog nicht für Sie überspringen.
* **Typ**: Boolean
  * `true`: Claude Code überspringt den Bestätigungsdialog, bevor eine Sitzung den `bypassPermissions`-Modus betritt
  * `false`: dasselbe wie nicht gesetzt; der Dialog wird angezeigt, es sei denn, eine andere dieser Dateien setzt `true`
* **Standard**: nicht gesetzt, daher wird der Dialog angezeigt

```json settings.json theme={null}
{
  "skipDangerousModePermissionPrompt": true
}
```

<h2 id="sandbox-settings">
  Sandbox-Einstellungen
</h2>

Isolieren Sie die Befehle, die Claude ausführt, von Ihrem Dateisystem, Ihrem Netzwerk und Ihren Anmeldedaten. Informationen zur Funktionsweise von Sandboxing und zu Plattformanforderungen finden Sie unter [Sandboxing](/docs/de/sandboxing).

<h3 id="sandbox">
  `sandbox`
</h3>

Isolieren Sie die Bash-Befehle, die Claude ausführt, von Ihrem Dateisystem und Netzwerk mit [Sandboxing](/docs/de/sandboxing). Aktivieren Sie die Sandbox mit `enabled`, und grenzen Sie dann ein oder erweitern Sie, was sandboxed Befehle berühren können, mit den Unterobjekten `filesystem`, `network` und `credentials`. Die Sandbox läuft auf macOS, Linux und WSL2.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Objekt mit `enabled`, `failIfUnavailable`, `autoAllowBashIfSandboxed`, `excludedCommands`, `allowUnsandboxedCommands`, `enableWeakerNestedSandbox`, `enableWeakerNetworkIsolation`, `allowAppleEvents`, `bwrapPath`, `socatPath`, `ignoreViolations` und `ripgrep`, plus die Objekte `filesystem`, `network` und `credentials`
* **Standard**: nicht gesetzt, daher führt Claude Code Befehle ohne Sandbox aus

Dies aktiviert die Sandbox, überspringt Berechtigungsaufforderungen für sandboxed Befehle, führt `docker` außerhalb der Sandbox aus, öffnet zwei zusätzliche Schreibpfade, verbirgt Ihre AWS-Anmeldedatei und erlaubt GitHub und npm vorab:

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

Claude Code nimmt den Wert eines booleschen Schlüssels aus dem Einstellungsbereich mit der höchsten Priorität, der ihn setzt, daher überschreibt ein verwalteter `enabled` oder `failIfUnavailable` alles, was ein Entwickler setzt. Es führt Array-Schlüssel über jeden Einstellungsbereich zusammen, den die Sitzung lädt, daher kann ein Entwickler Einträge anhängen; siehe [Keep developers from widening the policy](/docs/de/sandboxing#keep-developers-from-widening-the-policy) für die verwalteten Sperren. Um Sandboxing für eine Organisation zu erzwingen, siehe [Enforce sandboxing with managed settings](/docs/de/sandboxing#enforce-sandboxing-with-managed-settings).

<h3 id="sandbox-enabled">
  `sandbox.enabled`
</h3>

Aktivieren Sie [Sandboxing](/docs/de/sandboxing) für Bash-Befehle. Wenn Sie einen Modus im `/sandbox`-Panel auswählen, schreibt Claude Code diesen Schlüssel in `.claude/settings.local.json` für das aktuelle Projekt; setzen Sie ihn in `~/.claude/settings.json`, um jedes Projekt zu sandboxen.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolescher Wert
  * `true`: Claude Code sandboxed Bash-Befehle
  * `false`: Bash-Befehle werden unsandboxed ausgeführt
* **Standard**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true
  }
}
```

Auf Linux und WSL2 benötigt die Sandbox `bubblewrap` und `socat`; siehe [Set up Linux and WSL2](/docs/de/sandboxing#set-up-linux-and-wsl2). Wenn die Sandbox nicht starten kann, zeigt Claude Code eine Warnung an und führt Befehle unsandboxed aus, es sei denn, Sie setzen auch [`failIfUnavailable`](#sandbox-failifunavailable).

<h3 id="sandbox-failifunavailable">
  `sandbox.failIfUnavailable`
</h3>

Lassen Sie Claude Code beim Start mit einem Fehler beenden, wenn `sandbox.enabled` `true` ist, aber die Sandbox nicht starten kann, weil eine Abhängigkeit fehlt oder die Plattform nicht unterstützt wird. Ohne dies zeigt Claude Code eine Warnung an und führt Befehle unsandboxed aus. Verwenden Sie es in verwalteten Einstellungen, wenn Ihre Organisation Sandboxing als harte Grenze erfordert.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolescher Wert
  * `true`: Claude Code beendet sich beim Start mit einem Fehler, wenn `sandbox.enabled` `true` ist, aber die Sandbox nicht starten kann
  * `false`: Claude Code zeigt eine Warnung an und führt Befehle unsandboxed aus
* **Standard**: `false`

Dies lässt jeden verwalteten Computer Befehle sandboxen oder sich weigern zu starten:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true
  }
}
```

Siehe [Enforce sandboxing with managed settings](/docs/de/sandboxing#enforce-sandboxing-with-managed-settings).

<h3 id="sandbox-autoallowbashifsandboxed">
  `sandbox.autoAllowBashIfSandboxed`
</h3>

Lassen Sie Claude Code sandboxed Bash-Befehle ohne Berechtigungsaufforderung ausführen. Befehle, die nicht in der Sandbox ausgeführt werden können, durchlaufen weiterhin den regulären Berechtigungsfluss, und `deny`-Regeln und inhaltsbezogene `ask`-Regeln wie `Bash(git push *)` gelten weiterhin; eine bloße `Bash`-Regel wird für sandboxed Befehle übersprungen. Setzen Sie es auf `false`, um sandboxed Befehle auch durch den regulären Berechtigungsfluss zu senden, den die `/sandbox` **Mode**-Registerkarte als regulären Berechtigungsmodus bezeichnet.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolescher Wert
  * `true`: Claude Code führt sandboxed Bash-Befehle ohne Berechtigungsaufforderung aus, vorbehaltlich `deny`-Regeln und inhaltsbezogener `ask`-Regeln; `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB` deaktiviert die automatische Genehmigung
  * `false`: sandboxed Befehle durchlaufen den regulären Berechtigungsfluss, daher entscheiden Ihre Genehmigungsregeln und der Berechtigungsmodus. Die `/sandbox` **Mode**-Registerkarte nennt dies regulären Berechtigungsmodus
* **Standard**: `true`

Dies behält die Sandbox bei und sendet sandboxed Befehle durch den regulären Berechtigungsfluss:

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": false
  }
}
```

Siehe [Sandbox modes](/docs/de/sandboxing#sandbox-modes) für das, worauf der automatische Genehmigungsmodus noch auffordert, und wie er sich im Plan-Modus verhält.

<h3 id="sandbox-excludedcommands">
  `sandbox.excludedCommands`
</h3>

Benennen Sie Befehle, die Claude Code außerhalb der Sandbox ausführt, z. B. Tools, die nicht darunter funktionieren. Jeder Eintrag verwendet die gleiche Syntax wie der Inhalt einer `Bash(...)`-[Berechtigungsregel](/docs/de/permissions#permission-rule-syntax): ein exakter Befehl, ein Präfix wie `docker *` oder ein Wildcard-Muster.

Ihre Einträge nehmen einen Bash-Aufruf aus der Sandbox nur heraus, wenn sie jeden Befehl darin abdecken, und einige Aufrufformen bleiben auch dann sandboxed. Ein `docker *`-Eintrag allein nimmt `npm ci && docker build .` nicht aus der Sandbox heraus.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Befehlsmustern
* **Standard**: nicht gesetzt, daher wird kein Befehl ausgeschlossen

```json settings.json theme={null}
{
  "sandbox": {
    "excludedCommands": ["docker *"]
  }
}
```

Claude Code hält einen Bash-Aufruf sandboxed, wenn er eine dieser Formen hat, unter anderem:

* Ein Befehl, der mit `sudo`, `eval` oder `xargs` beginnt
* Ein `cd`, `pushd` oder `popd`, überall wo es im Aufruf erscheint
* Eine Befehlsersetzung, eine Subshell oder ein Kontrollfluss-Block wie `if` oder `for`
* Eine Umleitung, wie `docker build . > build.log`, außer einer, die nur einen Dateideskriptor dupliziert, wie `2>&1` es tut
* Ein Befehlsname, der aus einer Variable kommt

Zum Beispiel bleibt `cd build && docker compose up` unter einem `docker *`-Eintrag sandboxed, und das Hinzufügen eines `cd`-Eintrags ändert das nicht.

Ausgeschlossene Befehle durchlaufen weiterhin den regulären Berechtigungsfluss. Ausschluss ist eine Bequemlichkeit, keine Sicherheitsgrenze: bevorzugen Sie [`filesystem.allowWrite`](#sandbox-filesystem-allowwrite), wenn ein Tool nur an einer bestimmten Stelle schreiben muss. Claude Code führt Einträge über jeden Einstellungsbereich zusammen, den die Sitzung lädt, und es gibt keine verwaltete Sperre für diese Liste, daher halten Sie eine verwaltete Liste eng.

<h3 id="sandbox-allowunsandboxedcommands">
  `sandbox.allowUnsandboxedCommands`
</h3>

Lassen Sie Claude einen Befehl außerhalb der Sandbox mit dem Parameter `dangerouslyDisableSandbox` erneut versuchen, nachdem die Sandbox ihn blockiert hat. Setzen Sie es auf `false`, damit Claude Code diesen Parameter vollständig ignoriert und jeder Befehl, den Claude ausführt, sandboxed sein oder in [`excludedCommands`](#sandbox-excludedcommands) erscheinen muss. Die `/sandbox` **Overrides**-Registerkarte zeigt diesen Zustand als **Strict sandbox mode** an. Verwenden Sie `false` in verwalteten Einstellungen für Richtlinien, die striktes Sandboxing erfordern.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolescher Wert
  * `true`: Claude kann einen Befehl außerhalb der Sandbox mit dem Parameter `dangerouslyDisableSandbox` erneut versuchen, nachdem die Sandbox ihn blockiert hat
  * `false`: Claude Code ignoriert diesen Parameter, daher ist jeder Befehl, den Claude ausführt, sandboxed oder erscheint in `excludedCommands`
* **Standard**: `true`

Dies erzwingt den strikten Sandbox-Modus für alle, die die verwalteten Einstellungen abdecken:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowUnsandboxedCommands": false
  }
}
```

Ein unsandboxed-Wiederholungsversuch durchläuft den regulären Berechtigungsfluss, mit einer Aufforderung im manuellen Modus. Siehe [The unsandboxed retry escape hatch](/docs/de/sandboxing#the-unsandboxed-retry-escape-hatch).

Um zu sehen, wann Befehle, die Sie selbst an der [`!`-Shell-Modus-Eingabeaufforderung](/docs/de/interactive-mode#shell-mode-with-prefix) eingeben, sandboxed ausgeführt werden, siehe [strict sandbox mode](/docs/de/sandboxing#the-unsandboxed-retry-escape-hatch).

<h3 id="sandbox-filesystem">
  `sandbox.filesystem`
</h3>

Kontrollieren Sie, welche Pfade sandboxed Befehle lesen und schreiben können. Standardmäßig können sie in das Arbeitsverzeichnis, das Sitzungs-Temp-Verzeichnis und Verzeichnisse schreiben, die Sie mit `--add-dir`, `/add-dir` oder `permissions.additionalDirectories` hinzufügen, und können den Rest des Dateisystems lesen, einschließlich Anmeldedateien. Erweitern oder verengen Sie dies mit den vier Pfadlisten, oder schalten Sie die Dateisystem-Schicht mit `disabled` aus. Siehe [Filesystem isolation](/docs/de/sandboxing#filesystem-isolation) für die Standardgrenzen.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Objekt mit `allowWrite`, `denyWrite`, `denyRead` und `allowRead`-Arrays, plus die booleschen Werte `allowManagedReadPathsOnly` und `disabled`
* **Standard**: nicht gesetzt, daher gelten die Standard-Lese- und Schreibgrenzen

Dies lässt sandboxed Befehle in ein Build-Verzeichnis und Ihre kubeconfig schreiben und verbirgt Ihre AWS-Anmeldedatei:

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

Claude Code erzwingt diese Listen an der OS-Sandbox-Grenze, daher gelten sie für jeden Unterprozess, den ein sandboxed Befehl startet, z. B. `kubectl`, `terraform` oder `npm`. Claude Code fügt Ihre [Berechtigungsregeln](/docs/de/sandboxing#permission-rules) zu den gleichen Listen hinzu: `Edit`-Zulassungs- und Ablehnungsregeln zu `allowWrite` und `denyWrite`, `Read`-Ablehnungsregeln zu `denyRead` und `WebFetch(domain:...)`-Zulassungs- und Ablehnungsregeln zu den [`network`](#sandbox-network)-Domänenlisten.

Sofern keine verwaltete Sperre gesetzt ist, führt Claude Code jede Liste über die Einstellungsdateien zusammen, die die Sitzung lädt. [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly) beschränkt `allowRead` auf Einträge aus verwalteten Einstellungen, und [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) macht dasselbe für zulässige Domänen.

[Configure sandboxing](/docs/de/sandboxing#configure-sandboxing) behandelt Quellen, die Sie mit `--setting-sources` ausschließen. Wenn Sie eine Liste während einer Sitzung bearbeiten, [wendet Claude Code die Änderung auf die laufende Sitzung an](/docs/de/settings#when-edits-take-effect).

<h4 id="sandbox-path-prefixes">
  Sandbox-Pfadpräfixe
</h4>

Pfade in `allowWrite`, `denyWrite`, `denyRead`, `allowRead` und [`credentials.files`](#sandbox-credentials-files) werden nach ihrem Präfix aufgelöst:

| Präfix                | Bedeutung                                                                                       | Beispiel                                                              |
| :-------------------- | :---------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------- |
| `/`                   | Absoluter Pfad vom Dateisystem-Root                                                             | `/tmp/build` bleibt `/tmp/build`                                      |
| `~/`                  | Relativ zum Home-Verzeichnis                                                                    | `~/.kube` wird zu `$HOME/.kube`                                       |
| `./` oder kein Präfix | Relativ zum Projekt-Root für Projekteinstellungen oder zu `~/.claude` für Benutzereinstellungen | `./output` in `.claude/settings.json` wird zu `<project-root>/output` |

Das Präfix `//path` für absolute Pfade funktioniert auch. Wenn Sie `/path` erwartend verwenden und projektrelative Auflösung erwarten, wechseln Sie zu `./path`. Diese Syntax unterscheidet sich von [Read and Edit permission rules](/docs/de/permissions#read-and-edit), die `//path` für absolut und `/path` für projektrelativ verwenden: Sandbox-Dateisystempfade verwenden Standardkonventionen, daher ist `/tmp/build` ein absoluter Pfad.

Claude Code entfernt einen nachgestellten Schrägstrich aus einem Verzeichnispfad, daher entsprechen `~/.aws` und `~/.aws/` dem gleichen Verzeichnis. Vor v2.1.224 gab Claude Code den nachgestellten Schrägstrich an die Sandbox weiter, und Claude konnte weiterhin Pfade unter einem `denyRead`- oder `denyWrite`-Eintrag lesen oder schreiben, der mit einem geschrieben wurde.

Claude Code entfernt auch ein nachgestelltes `/**`, daher decken `~/build/**` und `~/build` das gleiche Verzeichnis ab. Ob ein Wildcard wie `*` funktioniert, hängt davon ab, in welcher Liste sich der Eintrag befindet und von der Plattform:

* **`allowWrite` und `denyWrite`**: auf macOS funktionieren Wildcards. Auf Linux und WSL2 mountet die Sandbox konkrete Pfade, daher überspringt Claude Code einen Eintrag, der `*`, `?` oder `[` enthält, sobald das nachgestellte `/**` entfernt ist, und dieser Eintrag hat keine Auswirkung. Claude Code fügt die Pfade aus Ihren `Edit`-Berechtigungsregeln zu diesen Listen hinzu, daher gilt die gleiche Grenze für sie, und die **Config**-Registerkarte von `/sandbox` warnt vor `Edit`- und `Read`-Berechtigungsregeln, die Wildcards enthalten.
* **`denyRead` und `allowRead`**: Wildcards funktionieren auf jeder Plattform. Auf Linux und WSL2 erweitert Claude Code einen Leseeintrag auf die konkreten Pfade, die er abgleicht, was es nicht für die Schreibleisten tut.

<h3 id="sandbox-filesystem-allowwrite">
  `sandbox.filesystem.allowWrite`
</h3>

Fügen Sie Pfade hinzu, in die sandboxed Befehle schreiben können, über das Arbeitsverzeichnis, das Sitzungs-Temp-Verzeichnis und die Verzeichnisse hinaus, die Sie mit `--add-dir`, `/add-dir` oder `permissions.additionalDirectories` hinzugefügt haben. Verwenden Sie es, wenn ein Unterprozess wie `kubectl` oder ein Build-Tool außerhalb des Projekts schreiben muss.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Pfadzeichenfolgen, unter Verwendung der [Sandbox-Pfadpräfixe](#sandbox-path-prefixes)
* **Standard**: nicht gesetzt, daher können sandboxed Befehle in das Arbeitsverzeichnis, das Sitzungs-Temp-Verzeichnis, Verzeichnisse, die Sie mit `--add-dir` oder `/add-dir` hinzugefügt haben, und Verzeichnisse in [`permissions.additionalDirectories`](#permissions-additionaldirectories) schreiben

Dies lässt einen Build unter `/tmp/build` schreiben und lässt `kubectl` Ihre kubeconfig aktualisieren:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"]
    }
  }
}
```

Claude Code führt Einträge über jeden Einstellungsbereich zusammen, den die Sitzung lädt: Benutzer-, Projekt-, lokale und verwaltete Pfade kombinieren sich, anstatt sich gegenseitig zu ersetzen, und Claude Code fügt die Pfade aus Ihren `Edit(...)`-Zulassungsberechtigungsregeln hinzu. Ein `allowWrite`-Eintrag kann einen [geschützten Pfad](/docs/de/sandboxing#protected-paths) nicht aufheben.

<h3 id="sandbox-filesystem-denywrite">
  `sandbox.filesystem.denyWrite`
</h3>

Blockieren Sie sandboxed Befehle vom Schreiben in bestimmte Pfade, einschließlich Pfade in einem Verzeichnis, das ansonsten beschreibbar ist.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Pfadzeichenfolgen, unter Verwendung der [Sandbox-Pfadpräfixe](#sandbox-path-prefixes)
* **Standard**: nicht gesetzt

Dies verhindert, dass sandboxed Befehle Systemkonfiguration ändern oder Binärdateien installieren:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyWrite": ["/etc", "/usr/local/bin"]
    }
  }
}
```

Claude Code führt Einträge über jeden Einstellungsbereich zusammen, den die Sitzung lädt, und fügt die Pfade aus Ihren `Edit(...)`-Ablehnungsberechtigungsregeln hinzu.

<h3 id="sandbox-filesystem-denyread">
  `sandbox.filesystem.denyRead`
</h3>

Blockieren Sie sandboxed Befehle vom Lesen bestimmter Pfade, z. B. Anmeldedateien, die die Standard-Lesrichtlinie ansonsten offenlegen würde. Um eine Anmeldedatei zu schützen und sie durch den Sandbox-Proxy nutzbar zu halten, siehe stattdessen [`sandbox.credentials`](#sandbox-credentials).

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Pfadzeichenfolgen, unter Verwendung der [Sandbox-Pfadpräfixe](#sandbox-path-prefixes)
* **Standard**: nicht gesetzt, daher behalten sandboxed Befehle den [Standard-Lesezugriff](/docs/de/sandboxing#filesystem-isolation), der Anmeldedateien wie `~/.aws/credentials` einschließt

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/.aws/credentials"]
    }
  }
}
```

Claude Code führt Einträge über jeden Einstellungsbereich zusammen, den die Sitzung lädt, und fügt die Pfade aus Ihren `Read(...)`-Ablehnungsberechtigungsregeln hinzu. Wenn [`filesystem.disabled`](#sandbox-filesystem-disabled) `true` ist, erzwingt Claude Code diese Einträge nicht.

<h3 id="sandbox-filesystem-allowread">
  `sandbox.filesystem.allowRead`
</h3>

Öffnen Sie das Lesen für bestimmte Pfade in einer Region erneut, die [`denyRead`](#sandbox-filesystem-denyread) blockiert, um Workspace-only-Lesezugriff zu erstellen. Ein exakter oder Wildcard-`denyRead`-Eintrag bleibt in einem breiteren `allowRead` blockiert, wie die [Überlappungstabelle](/docs/de/sandboxing#configure-sandboxing) zeigt. Wenn ein Wildcard-`denyRead`-Eintrag wie `~/**/.env` ein Verzeichnis abgleicht, blockiert Claude Code auch das Lesen seines Inhalts. Vor v2.1.236 auf macOS öffnete Claude Code die Pfade, die ein Wildcard-`denyRead`-Eintrag abglich, überall dort erneut, wo ein breiterer `allowRead`-Eintrag sie abdeckte, und ließ den Inhalt eines abgeglichenen Verzeichnisses lesbar.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Pfadzeichenfolgen, unter Verwendung der [Sandbox-Pfadpräfixe](#sandbox-path-prefixes)
* **Standard**: nicht gesetzt

Dies blockiert Lesevorgänge Ihres Home-Verzeichnisses außer dem Projekt selbst:

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

Claude Code löst einen `.`-Eintrag zum Projekt-Root in Projekteinstellungen und zu `~/.claude` in Benutzereinstellungen auf. Claude Code führt Einträge über jede Einstellungsdatei zusammen, die die Sitzung lädt, sofern [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly) nicht gesetzt ist.

<h3 id="sandbox-filesystem-allowmanagedreadpathsonly">
  `sandbox.filesystem.allowManagedReadPathsOnly`
</h3>

Beachten Sie nur die [`allowRead`](#sandbox-filesystem-allowread)-Einträge, die aus verwalteten Einstellungen stammen, damit Entwickler den Lesezugriff auf Pfade, die Ihre Organisation blockiert hat, nicht erneut öffnen können. Claude Code führt weiterhin `denyRead`-Einträge aus jedem Einstellungsbereich zusammen, den die Sitzung lädt.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Boolescher Wert
  * `true`: Claude Code beachtet nur die `allowRead`-Einträge aus verwalteten Einstellungen
  * `false`: `allowRead`-Einträge werden aus jedem Einstellungsbereich zusammengeführt, den die Sitzung lädt
* **Standard**: `false`

Dies blockiert Lesevorgänge des Home-Verzeichnisses, öffnet `~/work` erneut und verhindert, dass Entwickler etwas anderes erneut öffnen:

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

Siehe [Keep developers from widening the policy](/docs/de/sandboxing#keep-developers-from-widening-the-policy).

<h3 id="sandbox-filesystem-disabled">
  `sandbox.filesystem.disabled`
</h3>

Überspringen Sie die Dateisystem-Isolierung, während Sie die Netzwerk-Isolierung beibehalten. Sandboxed Befehle erhalten unbeschränkten Lese- und Schreibzugriff auf das Host-Dateisystem, und ihr Netzwerk-Egress bleibt auf [`network.allowedDomains`](#sandbox-network-alloweddomains) beschränkt. Verwenden Sie es, wenn Sie sandboxen, um zu kontrollieren, wo Befehle sich verbinden, anstatt was sie schreiben. Erfordert Claude Code v2.1.216 oder später.

* **Bereich**: [`User or managed`](#scopes). Wenn verwaltete Einstellungen `sandbox.filesystem` überhaupt konfigurieren oder einen `sandbox.credentials.files`-Eintrag mit `"mode": "deny"` auflisten, können nur verwaltete Einstellungen ihn setzen.
* **Typ**: Boolescher Wert
  * `true`: Claude Code überspringt die Dateisystem-Isolierung und behält die Netzwerk-Isolierung bei
  * `false`: Dateisystem-Isolierung bleibt an
* **Standard**: `false`, daher bleibt die Dateisystem-Isolierung an

Dies lässt das Dateisystem offen und beschränkt den Netzwerk-Egress auf GitHub und npm:

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

Mit der Schicht aus erzwingt Claude Code `denyRead`- oder `credentials.files`-`deny`-Einträge nicht, während `credentials.envVars`-Einträge und angewendete `mask`-Einträge weiterhin funktionieren. [`autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed) wird immer noch standardmäßig auf `true` gesetzt, daher setzen Sie es auf `false`, um weiterhin aufzufordern. Siehe [Disable filesystem isolation](/docs/de/sandboxing#disable-filesystem-isolation) für die vollständige Liste der Quellen, die es setzen können, und was sich ändert, wenn die Isolierung aus ist. Erfordert Claude Code v2.1.216 oder später.

<h3 id="sandbox-ignoreviolations">
  `sandbox.ignoreViolations`
</h3>

Unterdrücken Sie Sandbox-Verletzungsberichte für Pfade, bei denen Sie erwarten, dass ein Befehl sie prüft und abgelehnt wird, z. B. ein Tool, das beim Start `/etc/hosts` prüft, damit diese Ablehnungen nicht als Verletzungen angezeigt werden oder in dem, was Claude sieht. Die Sandbox blockiert den Zugriff weiterhin; nur der Bericht wird unterdrückt. Schlüssel sind Teilzeichenfolgen, die mit dem Befehl abgeglichen werden, wobei `*` jeden Befehl abgleicht, und Werte sind Teilzeichenfolgen der Verletzung, die für diesen Befehl ignoriert werden, z. B. ein Dateisystempfad.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Objekt, das eine Befehlsteilzeichenfolge einem Array von Verletzungsteilzeichenfolgen zuordnet, normalerweise Pfade
* **Standard**: nicht gesetzt, daher wird jede Verletzung gemeldet

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

Führen Sie die Linux-Sandbox in einem unprivilegierten Docker-Container aus, wo bubblewrap kein frisches `/proc` mounten kann. Stattdessen bindet die innere Sandbox das vorhandene `/proc` des Containers, das Prozessinformationen offenlegt, die ein frisches Mount verbergen würde. Dies reduziert die Sicherheit; verwenden Sie es nur, wenn der äußere Container bereits die Isolierung bietet, die Sie benötigen.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolescher Wert
  * `true`: die innere Sandbox bindet das vorhandene `/proc` des Containers, anstatt ein frisches zu mounten
  * `false`: die Sandbox mountet ein frisches `/proc`, das in einem unprivilegierten Docker-Container nicht funktioniert
* **Standard**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNestedSandbox": true
  }
}
```

Nur Linux und WSL2. Siehe [Bubblewrap fails to start inside a container](/docs/de/sandboxing#troubleshooting).

<h3 id="sandbox-enableweakernetworkisolation">
  `sandbox.enableWeakerNetworkIsolation`
</h3>

Lassen Sie sandboxed Befehle auf macOS den System-TLS-Vertrauensdienst `com.apple.trustd.agent` erreichen. Go-basierte Tools wie `gh`, `gcloud` und `terraform` benötigen ihn, um TLS-Zertifikate zu überprüfen, wenn Sie [`network.httpProxyPort`](#sandbox-network-httpproxyport) mit einem MITM-Proxy und einer benutzerdefinierten CA verwenden. Dies reduziert die Sicherheit, indem ein potenzieller Datenexfiltrationspfad durch den Vertrauensdienst geöffnet wird.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolescher Wert
  * `true`: sandboxed Befehle auf macOS können `com.apple.trustd.agent` erreichen
  * `false`: sandboxed Befehle auf macOS können den System-TLS-Vertrauensdienst nicht erreichen
* **Standard**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNetworkIsolation": true
  }
}
```

Wenn Sie keinen MITM-Proxy verwenden, listen Sie stattdessen die fehlgeschlagenen Tools in [`excludedCommands`](#sandbox-excludedcommands) auf; siehe [Go-based CLIs fail TLS verification on macOS](/docs/de/sandboxing#troubleshooting).

<h3 id="sandbox-allowappleevents">
  `sandbox.allowAppleEvents`
</h3>

Lassen Sie sandboxed Befehle auf macOS Apple Events senden, die `open`, `osascript` und Tools, die URLs in einem Browser öffnen, benötigen; ohne dies schlagen sie mit Fehler `-600` fehl. Dies entfernt die Code-Ausführungs-Isolierung: sandboxed Befehle können andere Anwendungen unsandboxed ohne Benutzeraufforderung starten und können AppleScript-Befehle an laufende Anwendungen wie Terminal senden, vorbehaltlich der Pro-App-macOS-Automatisierungszustimmungsaufforderung (TCC).

* **Bereich**: [`User or managed`](#scopes)
* **Typ**: Boolescher Wert
  * `true`: sandboxed Befehle auf macOS können Apple Events senden
  * `false`: sandboxed Befehle auf macOS können keine Apple Events senden, daher schlagen `open` und `osascript` mit Fehler `-600` fehl
* **Standard**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowAppleEvents": true
  }
}
```

Um die Isolierung zu behalten und trotzdem ein solches Tool auszuführen, fügen Sie es stattdessen zu [`excludedCommands`](#sandbox-excludedcommands) hinzu. Siehe [Apple Events on macOS](/docs/de/sandboxing#security-limitations).

<h3 id="sandbox-ripgrep">
  `sandbox.ripgrep`
</h3>

Zeigen Sie die Sandbox auf eine ripgrep-Binärdatei Ihrer Wahl, z. B. wenn Ihre Plattform eine anders erstellte `rg` benötigt.

* **Bereich**: [`User or managed`](#scopes)
* **Typ**: Objekt mit `command`, dem Pfad zur ripgrep-Binärdatei, und optional `args`, einem Array von Argumenten zum Voranstellen
* **Standard**: nicht gesetzt, daher verwendet die Sandbox die gleiche ripgrep-Binärdatei wie Claude Code. Das ist die gebündelte Binärdatei, sofern Sie [`USE_BUILTIN_RIPGREP`](/docs/de/env-vars) nicht auf `0` setzen

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

Zeigen Sie die Sandbox auf eine bubblewrap-Binärdatei, die außerhalb von `PATH` installiert ist, z. B. eine Vendor-Kopie auf einem luftgestützten Host. Claude Code verwendet den Pfad sowohl für die Abhängigkeitsprüfung beim Start als auch wenn es jeden sandboxed Befehl umhüllt.

* **Bereich**: [`Managed`](#scopes). Claude Code liest es nur aus verwalteten Einstellungen, damit eine Benutzer-, Projekt- oder lokale Datei die Sandbox nicht auf eine andere Binärdatei zeigen kann.
* **Typ**: Zeichenfolge, ein absoluter Pfad; Claude Code verwirft einen relativen Pfad und fällt auf `PATH`-Suche zurück
* **Standard**: nicht gesetzt, daher findet Claude Code `bwrap` auf `PATH`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "bwrapPath": "/opt/admin/bwrap"
  }
}
```

Nur Linux und WSL2.

<h3 id="sandbox-socatpath">
  `sandbox.socatPath`
</h3>

Zeigen Sie den Sandbox-Netzwerk-Proxy auf eine `socat`-Binärdatei, die außerhalb von `PATH` installiert ist.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Zeichenfolge, ein absoluter Pfad; Claude Code verwirft einen relativen Pfad und fällt auf `PATH`-Suche zurück
* **Standard**: nicht gesetzt, daher findet Claude Code `socat` auf `PATH`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "socatPath": "/opt/admin/socat"
  }
}
```

Nur Linux und WSL2.

<h3 id="sandbox-credentials">
  `sandbox.credentials`
</h3>

Deklarieren Sie die Anmeldedateien und Umgebungsvariablen, um [sie vor sandboxed Befehlen zu schützen](/docs/de/sandboxing#protect-credentials). Jeder Eintrag benennt eine Datei `path` oder eine Variable `name` und einen `mode`: `deny` verbirgt die Anmeldedaten in der Sandbox, und `mask` zeigt sandboxed Befehlen einen Platzhalter, während der [Sandbox-Proxy](/docs/de/sandboxing#mask-credentials) den echten Wert bei ausgehenden Anfragen ersetzt. Claude Code schützt nur die Einträge, die Sie auflisten; es gibt keine integrierte Anmeldedaten-Ablehnungsliste.

* **Bereich**: [`Any file`](#scopes). Claude Code beachtet `mask`-Einträge, `allowPlaintextInject`, `awsPairs` und `sigv4` nur aus Benutzereinstellungen, verwalteten Einstellungen und dem `--settings`-Flag.
* **Typ**: Objekt mit `files`, `envVars`, `allowPlaintextInject`, `awsPairs` und `sigv4`
* **Standard**: nicht gesetzt, daher werden keine Anmeldedaten geschützt

Dies verbirgt Ihre AWS-Anmeldedatei und entfernt `GITHUB_TOKEN` aus sandboxed Befehlen:

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

Der `deny`-Dateischutz ist Teil der Dateisystem-Schicht, daher gilt er nicht, wenn Sie [Dateisystem-Isolierung deaktivieren](/docs/de/sandboxing#disable-filesystem-isolation); der Umgebungsvariablenschutz gilt weiterhin.

<h4 id="invalid-credential-entries-in-managed-settings">
  Ungültige Anmeldedaten-Einträge in verwalteten Einstellungen
</h4>

Wenn ein verwalteter `sandbox.credentials`-Eintrag die Validierung nicht besteht, schützt Claude Code die Anmeldedaten, wo es kann:

* Ein Eintrag in `files` oder `envVars`, der immer noch einen gültigen `path` oder `name` und einen `mode` von `mask` oder `deny` hat, z. B. einer, dessen `extract`-Muster keine Erfassungsgruppe hat, wird mit einer Warnung auf `mode: "deny"` herabgestuft, daher bleibt die Anmeldedaten blockiert, nicht maskiert, bis Sie den Eintrag beheben. Ein herabgestufter `files`-Eintrag fixiert [`filesystem.disabled`](/docs/de/sandboxing#disable-filesystem-isolation) wie ein expliziter `deny`-Eintrag, und die Warnung vermerkt, dass sein Lesblock nicht erzwungen wird, wenn verwaltete Einstellungen die Dateisystem-Isolierung ausschalten.
* Ein Eintrag mit einem unbekannten `mode` oder einem ungültigen `path` oder `name` wird entfernt.
* Jeder Fall warnt; ob ein Eintrag herabgestuft oder entfernt wird, die verbleibenden gültigen Einträge werden weiterhin erzwungen, und ein vollständig ungültiger `credentials`-Wert wird gelöscht, während der Rest von `sandbox` weiterhin gilt.

Gilt in v2.1.191 und später; vor v2.1.221 wurde jeder ungültige Eintrag entfernt. Für die anderen verwalteten Schlüssel mit Pro-Feld-Behandlung siehe [Invalid entries in managed settings](/docs/de/managed-settings#invalid-entries-in-managed-settings).

<h3 id="sandbox-credentials-files">
  `sandbox.credentials.files`
</h3>

Schützen Sie Anmeldedateien oder Verzeichnisse vor sandboxed Befehlen. Mit `"mode": "deny"` blockiert Claude Code Lesevorgänge des Pfads in der Sandbox, der gleiche Lesblock wie [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread). Mit `"mode": "mask"` lesen sandboxed Befehle auf Linux und WSL2 eine Sentinel-Kopie der Datei, und der Sandbox-Proxy ersetzt den echten Wert bei ausgehenden Anfragen an die `injectHosts` dieses Eintrags; auf macOS ist die Datei in der Sandbox stattdessen nicht lesbar. `"mode": "mask"` erfordert Claude Code v2.1.221 oder später.

* **Bereich**: [`Any file`](#scopes). Claude Code lässt `mask`-Einträge aus Projekt `.claude/settings.json` und lokal `.claude/settings.local.json` fallen.
* **Typ**: Array von Objekten, jedes mit `path` und einem `mode` von `"deny"` oder `"mask"`, plus die optionalen [Maskierungsfelder für Dateien](#mask-fields-for-files)
* **Standard**: nicht gesetzt, daher werden keine Anmeldedateien geschützt

Dies verbirgt Ihre AWS-Anmeldedatei und maskiert die `gh`-Hosts-Datei, wobei der echte Wert nur bei Anfragen an `api.github.com` ersetzt wird:

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

Pfade verwenden die gleichen [Präfixe](#sandbox-path-prefixes) wie die `sandbox.filesystem.*`-Einstellungen, und Claude Code führt die Arrays aus jedem Einstellungsbereich zusammen, den die Sitzung lädt. [Protect credentials](/docs/de/sandboxing#protect-credentials) behandelt das, was weiterhin aus Quellen gilt, die Sie mit `--setting-sources` ausschließen. `mask`-Einträge erfordern Claude Code v2.1.221 oder später.

`mask`-Ersetzung läuft nur durch den Sandbox-Proxy, daher setzen Sie [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate) oder [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject) für Plain-HTTP-Test-Netzwerke. `mask` gilt für eine einzelne Datei, daher listen Sie jede Anmeldedatei einzeln auf. Claude Code akzeptiert aber ignoriert die `mask`-Felder auf einem `deny`-Eintrag. [Mask credential files](/docs/de/sandboxing#mask-credential-files) behandelt, welche Einstellungsquellen beachtet werden und wann ein Eintrag auf `deny` zurückfällt.

<span id="sandbox-credentials-files-extract" />

<span id="sandbox-credentials-files-onextractnomatch" />

<span id="sandbox-credentials-files-decode" />

<span id="sandbox-credentials-files-maskclaims" />

<span id="sandbox-credentials-files-maskduplicates" />

<span id="sandbox-credentials-files-injecthosts" />

<h4 id="mask-fields-for-files">
  Maskierungsfelder für Dateien
</h4>

Ein `mask`-Eintrag akzeptiert diese optionalen Felder. Ohne `extract` oder `decode` ersetzt Claude Code den gesamten Dateiinhalt durch einen Sentinel. Auf macOS mit aktivierter Dateisystem-Isolierung wendet Claude Code einen `mask`-Eintrag als `deny` an, bevor `extract` oder `decode` ausgeführt wird; siehe [Mask credential files](/docs/de/sandboxing#mask-credential-files).

| Feld               | Typ                                                                                                                          | Was es tut                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :----------------- | :--------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `extract`          | Zeichenfolge, ein regulärer Ausdruck mit mindestens einer Erfassungsgruppe                                                   | Maskieren Sie nur den Text, der von Gruppe 1 jedes Treffers erfasst wird, damit der Rest der Datei analysierbar bleibt. Wenn `decode` auch gesetzt ist, prüft Claude Code jeden Erfassten als mögliches JWT, anstatt ihn direkt zu ersetzen. Erfordert v2.1.221 oder später                                                                                                                                                                                                                                                                                                                                       |
| `onExtractNoMatch` | `"warn"`, `"deny"` oder `"error"`; Standard `"warn"`                                                                         | Was passiert, wenn `extract` oder `decode` nichts zu maskieren findet. `warn` lässt die Datei in der Sandbox lesbar wie sie ist, `deny` macht sie nicht lesbar, und `error` stoppt das Sandbox-Setup, bis Sie die Konfiguration beheben. Claude Code behandelt `deny` als `error`, wenn der Lesblock nicht erzwungen würde, weil Sie [Dateisystem-Isolierung deaktivieren](/docs/de/sandboxing#disable-filesystem-isolation) oder ein [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread)-Eintrag den Pfad erneut öffnet. Erfordert v2.1.221 oder später; der `decode`-Fall erfordert v2.1.224 oder später |
| `decode`           | die Zeichenfolge `"jwt"`                                                                                                     | Finden Sie JSON Web Tokens (JWTs) in der Datei, mit einem integrierten Muster oder mit `extract`, wenn gesetzt, überprüfen Sie jeden Kandidaten und ersetzen Sie ihn durch ein strukturell gültiges gefälschtes Token, damit Code in der Sandbox, der das Token dekodiert, weiterhin funktioniert. Wenn kein Kandidat überprüft wird, regiert `onExtractNoMatch` das Ergebnis. Erfordert v2.1.224 oder später                                                                                                                                                                                                     |
| `maskClaims`       | Array von Zeichenfolgen, mindestens ein Anspruchsname; erfordert `decode`                                                    | Maskieren Sie nur die benannten Top-Level-Payload-Ansprüche in jedem überprüften JWT und erstellen Sie das Token um die geänderte Payload neu auf, damit die anderen Ansprüche lesbar bleiben. Wenn kein benannter Anspruch übereinstimmt, regiert `onExtractNoMatch` das Ergebnis. Erfordert v2.1.224 oder später                                                                                                                                                                                                                                                                                                |
| `maskDuplicates`   | Boolescher Wert, Standard `false`                                                                                            | Ersetzen Sie auch wörtliche Kopien jedes maskierten Werts anderswo in der Datei, z. B. ein Geheimnis, das in einen Kommentar eingefügt wurde. Claude Code gleicht rohe Teilzeichenfolgen ab, daher reservieren Sie es für lange, hochentropische Geheimnisse. Wird nur berücksichtigt, wenn `extract` oder `decode` gesetzt ist. Erfordert v2.1.221 oder später                                                                                                                                                                                                                                                   |
| `injectHosts`      | Array von Zeichenfolgen, jede ein Host, den [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) auch zulässt | Verengen Sie die Hosts, wo der Sandbox-Proxy den echten Wert ersetzt. Wenn nicht gesetzt, ersetzt der Proxy ihn bei Anfragen an jeden Host in `sandbox.network.allowedDomains`. Erfordert v2.1.221 oder später                                                                                                                                                                                                                                                                                                                                                                                                    |

Dies maskiert nur den `oauth_token`-Wert in der `gh`-Hosts-Datei, ersetzt jede andere Kopie davon in der Datei, macht die Datei nicht lesbar, wenn das Muster nichts abgleicht, und ersetzt das echte Token nur bei Anfragen an `api.github.com`:

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

Schützen Sie Umgebungsvariablen vor sandboxed Befehlen. Mit `"mode": "deny"` entfernt Claude Code die Variable aus der Umgebung von sandboxed Befehlen. Mit `"mode": "mask"` sehen sandboxed Befehle einen pro-Sitzungs-Sentinel-Wert, und der Sandbox-Proxy ersetzt den echten Wert bei ausgehenden Anfragen an die `injectHosts` dieses Eintrags, daher behalten Tools wie `gh` und `npm` die Authentifizierung bei, ohne jemals die echte Anmeldedaten zu halten. `"mode": "mask"` erfordert Claude Code v2.1.199 oder später.

* **Bereich**: [`Any file`](#scopes). Claude Code lässt `mask`-Einträge aus Projekt `.claude/settings.json` und lokal `.claude/settings.local.json` fallen.
* **Typ**: Array von Objekten, jedes mit `name` und einem `mode` von `"deny"` oder `"mask"`, plus die optionalen [Maskierungsfelder für Umgebungsvariablen](#mask-fields-for-environment-variables)
* **Standard**: nicht gesetzt, daher werden keine Umgebungsvariablen geschützt

Dies entfernt `NPM_TOKEN` aus sandboxed Befehlen und maskiert `GITHUB_TOKEN`, wobei der echte Wert nur bei Anfragen an `api.github.com` ersetzt wird:

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

Der `name` muss mit einem Buchstaben oder Unterstrich beginnen und darf nur Buchstaben, Ziffern und Unterstriche enthalten. Claude Code führt die Arrays aus jedem Einstellungsbereich zusammen, den die Sitzung lädt, und wendet `deny` an, wenn die gleiche Variable mit beiden Modi erscheint. [Protect credentials](/docs/de/sandboxing#protect-credentials) behandelt das, was weiterhin aus Quellen gilt, die Sie mit `--setting-sources` ausschließen. `mask`-Einträge erfordern Claude Code v2.1.199 oder später.

`mask`-Ersetzung läuft nur durch den Sandbox-Proxy, daher setzen Sie [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate) oder [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject) für Plain-HTTP-Test-Netzwerke; siehe [Mask environment variables](/docs/de/sandboxing#mask-environment-variables). Claude Code akzeptiert aber ignoriert die `mask`-Felder auf einem `deny`-Eintrag.

<span id="sandbox-credentials-envvars-extract" />

<span id="sandbox-credentials-envvars-onextractnomatch" />

<span id="sandbox-credentials-envvars-decode" />

<span id="sandbox-credentials-envvars-maskclaims" />

<span id="sandbox-credentials-envvars-injecthosts" />

<h4 id="mask-fields-for-environment-variables">
  Maskierungsfelder für Umgebungsvariablen
</h4>

Ein `mask`-Eintrag akzeptiert diese optionalen Felder. Ohne `extract` oder `decode` ersetzt Claude Code den gesamten Wert durch einen Sentinel. `extract` und `decode` können nicht auf dem gleichen Eintrag kombiniert werden.

| Feld               | Typ                                                                                                                          | Was es tut                                                                                                                                                                                                                                                                                                                                                                                                             |
| :----------------- | :--------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `extract`          | Zeichenfolge, ein regulärer Ausdruck mit mindestens einer Erfassungsgruppe                                                   | Maskieren Sie nur den Text, der von Gruppe 1 jedes Treffers erfasst wird, z. B. das Passwort in einer `DATABASE_URL`-Verbindungszeichenfolge, damit der Rest des Werts analysierbar bleibt. Erfordert v2.1.224 oder später                                                                                                                                                                                             |
| `onExtractNoMatch` | `"warn"`, `"deny"` oder `"error"`; Standard `"warn"`. Bei einem Eintrag mit `decode` wird nur `"warn"` akzeptiert            | Was passiert, wenn `extract` nichts abgleicht. `warn` gibt die Variable unmasked durch, `deny` setzt sie in der Sandbox auf unset, und `error` stoppt das Sandbox-Setup, bis Sie die Konfiguration beheben. Erfordert v2.1.224 oder später                                                                                                                                                                             |
| `decode`           | die Zeichenfolge `"jwt"`                                                                                                     | Überprüfen Sie, dass der gesamte Wert ein JWT ist und ersetzen Sie ihn durch ein strukturell gültiges gefälschtes Token, damit Code in der Sandbox, der das Token dekodiert, weiterhin funktioniert; der Proxy ersetzt das ganze echte Token bei Egress. Ein Wert, der nicht überprüft wird, wird unmasked mit einer Warnung durchgegeben. Erfordert v2.1.224 oder später                                              |
| `maskClaims`       | Array von Zeichenfolgen, mindestens ein Anspruchsname; erfordert `decode`                                                    | Maskieren Sie nur die benannten Top-Level-Payload-Ansprüche in dem dekodierten JWT und erstellen Sie das Token um die geänderte Payload neu auf, damit die anderen Ansprüche lesbar bleiben. Wenn kein benannter Anspruch übereinstimmt, wird die Variable unmasked mit einer Warnung durchgegeben. Erfordert v2.1.224 oder später                                                                                     |
| `injectHosts`      | Array von Zeichenfolgen, jede ein Host, den [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) auch zulässt | Verengen Sie die Hosts, wo der Sandbox-Proxy den echten Wert ersetzt. Wenn nicht gesetzt, ersetzt der Proxy ihn bei Anfragen an jeden Host in `sandbox.network.allowedDomains`. Schreiben Sie ein IPv6-Ziel als die bloße komprimierte Adresse, z. B. `"::1"`, nicht die geklammerte Form; siehe [IPv6 destinations in `injectHosts`](/docs/de/sandboxing#ipv6-destinations-in-injecthosts). Erfordert v2.1.199 oder später |

Dies maskiert nur das Passwort in `DATABASE_URL`, setzt die Variable auf unset, wenn das Muster nichts abgleicht, und maskiert ein JWT in `SERVICE_JWT`, während jeder Anspruch außer `api_key` lesbar bleibt:

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

Erlauben Sie `mask`-Ersetzung auf Plain-HTTP-Anfragen sowie TLS-terminiertem HTTPS. Bei Plain-HTTP ist die Upstream-Identität nicht überprüft und die Anmeldedaten reisen im Klartext, daher lassen Sie dies außerhalb vertrauenswürdiger Test-Netzwerke aus. Erfordert Claude Code v2.1.199 oder später.

* **Bereich**: [`User or managed`](#scopes)
* **Typ**: Boolescher Wert
  * `true`: Claude Code erlaubt `mask`-Ersetzung auf Plain-HTTP-Anfragen sowie TLS-terminiertem HTTPS
  * `false`: Claude Code erlaubt `mask`-Ersetzung nur auf TLS-terminiertem HTTPS
* **Standard**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "allowPlaintextInject": true
    }
  }
}
```

Erfordert Claude Code v2.1.199 oder später.

<h3 id="sandbox-credentials-awspairs">
  `sandbox.credentials.awsPairs`
</h3>

Gruppieren Sie maskierte Umgebungsvariablen, die eine AWS-Anmeldedaten für [SigV4-Neusignierung](/docs/de/sandboxing#re-sign-aws-requests) bilden, wenn Ihre Anmeldedaten in Variablen mit nicht standardmäßigen Namen leben. Claude Code verknüpft das konventionelle `AWS_ACCESS_KEY_ID`-, `AWS_SECRET_ACCESS_KEY`- und `AWS_SESSION_TOKEN`-Trio automatisch, wenn Sie ihre ganzen Werte maskieren, daher benötigen Sie diesen Schlüssel nur für andere Namen. Erfordert Claude Code v2.1.224 oder später.

* **Bereich**: [`User or managed`](#scopes)
* **Typ**: Array von Objekten, jedes mit `accessKeyIdVar`, `secretAccessKeyVar` und optional `sessionTokenVar`, benennend `sandbox.credentials.envVars`-Einträge
* **Standard**: nicht gesetzt, daher wird nur das konventionelle Trio gepaart

Dies verknüpft drei benutzerdefiniert benannte Variablen in eine AWS-Anmeldedaten für Neusignierung:

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

Jede benannte Variable muss ein Ganz-Wert-`mask`-Eintrag in [`sandbox.credentials.envVars`](#sandbox-credentials-envvars) sein, ohne `extract` oder `decode`, und kann nur einen Slot über alle Paare hinweg ausfüllen.

<h3 id="sandbox-credentials-sigv4">
  `sandbox.credentials.sigv4`
</h3>

Wählen Sie, was der Sandbox-Proxy mit AWS-Anforderungsformen tut, die er [nicht erneut signieren kann](/docs/de/sandboxing#re-sign-aws-requests): `streaming` für aws-chunked-Streaming-Uploads, `presigned` für vorsignierte URLs und `sigv4a` für SigV4A-asymmetrische Signaturen. Dies gilt nur für Anfragen, die mit einer maskierten Paars Platzhalter-Zugriffschlüssel-ID signiert sind. Erfordert Claude Code v2.1.224 oder später.

* **Bereich**: [`User or managed`](#scopes)
* **Typ**: Objekt mit `streaming`, `presigned` und `sigv4a`, jedes eines von:
  * `"deny"`: der Proxy lehnt die Anfrage ab
  * `"passthrough"`: der Proxy leitet die Anfrage mit dem maskierten Platzhalter signiert weiter, daher erhält das Tool AWS's eigene Ablehnung
* **Standard**: nicht gesetzt, daher ist jede Form `"deny"`

Dies leitet Streaming-Uploads weiter, anstatt sie am Proxy zu fehlschlagen:

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

Mit `deny` lehnt der Proxy die Anfrage ab. Mit `passthrough` leitet der Proxy die Anfrage mit ihrer Signatur weiter, die aus dem maskierten Platzhalter berechnet wird, daher lehnt AWS sie ab und das aufrufende Tool erhält AWS's eigene Antwort, anstatt eines Proxy-Fehlers.

<h3 id="sandbox-network">
  `sandbox.network`
</h3>

Kontrollieren Sie, welche Hosts, Ports und Sockets sandboxed Befehle erreichen können. Die Sandbox leitet ausgehenden Verkehr durch einen Proxy, der diese Listen erzwingt; siehe [Network isolation](/docs/de/sandboxing#network-isolation) für wie der Proxy entscheidet und wann er auffordert.

* **Bereich**: [`Any file`](#scopes). `strictAllowlist`, `allowManagedDomainsOnly` und `tlsTerminate` werden aus weniger Quellen gelesen, wie ihre Einträge sagen.
* **Typ**: Objekt mit den Unterschlüsseln unten
* **Standard**: nicht gesetzt, daher werden keine Domänen vorab erlaubt und die Sandbox fordert für jeden neuen Host auf

Dies erlaubt GitHub und npm vorab, blockiert `uploads.github.com` und lässt Befehle an localhost binden:

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

Claude Code führt die Array-Unterschlüssel über Einstellungsbereiche zusammen und dedupliziert sie, daher kann ein Projekt Domänen zu Ihrer Benutzerliste hinzufügen. `WebFetch(domain:...)`-Zulassungs- und Ablehnungs-[Berechtigungsregeln](/docs/de/sandboxing#permission-rules) speisen die gleichen Zulassungs- und Ablehnungslisten.

<h3 id="sandbox-network-allowunixsockets">
  `sandbox.network.allowUnixSockets`
</h3>

Listen Sie die Unix-Socket-Pfade auf, mit denen sandboxed Befehle auf macOS verbinden können. Claude Code ignoriert diese Liste auf Linux und WSL2, wo der seccomp-Filter Socket-Pfade nicht überprüfen kann; verwenden Sie stattdessen [`allowAllUnixSockets`](#sandbox-network-allowallunixsockets).

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Zeichenfolgen, jede ein Socket-Pfad
* **Standard**: nicht gesetzt, daher blockiert die macOS-Sandbox jeden Unix-Socket

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowUnixSockets": ["~/.ssh/agent-socket"]
    }
  }
}
```

Ein Socket-Pfad kann breiten Zugriff gewähren: Das Zulassen von `/var/run/docker.sock` lässt beispielsweise einen sandboxed Befehl den Docker-Daemon kontrollieren. Siehe [Security limitations](/docs/de/sandboxing#security-limitations).

<h3 id="sandbox-network-allowallunixsockets">
  `sandbox.network.allowAllUnixSockets`
</h3>

Lassen Sie sandboxed Befehle mit jedem Unix-Socket verbinden. Auf Linux und WSL2 blockiert der [seccomp-Filter](/docs/de/sandboxing#set-up-linux-and-wsl2) der Sandbox `socket(AF_UNIX, ...)`-Aufrufe, daher ist dies die einzige Möglichkeit, Unix-Sockets dort zu erlauben. Wenn der Filter fehlt, den `/sandbox` auf seiner Registerkarte Abhängigkeiten meldet, blockiert die Sandbox Unix-Socket-Aufrufe nicht. Siehe [Set up Linux and WSL2](/docs/de/sandboxing#set-up-linux-and-wsl2) für wo der Filter herkommt.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolescher Wert
  * `true`: sandboxed Befehle können mit jedem Unix-Socket verbinden
  * `false`: die Sandbox blockiert Unix-Socket-Verbindungen: auf macOS außer den Pfaden in `allowUnixSockets`, und auf Linux und WSL2 durch den seccomp-Filter, wenn er vorhanden ist
* **Standard**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowAllUnixSockets": true
    }
  }
}
```

Auf WSL2 öffnet `true` auch den Interop-Socket erneut, der Windows-Binärdateien wie `cmd.exe` und `powershell.exe` startet.

<h3 id="sandbox-network-allowlocalbinding">
  `sandbox.network.allowLocalBinding`
</h3>

Lassen Sie sandboxed Befehle auf macOS an localhost-Ports binden, z. B. um einen Dev-Server zu starten.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolescher Wert
  * `true`: sandboxed Befehle können auf macOS an localhost-Ports binden
  * `false`: sandboxed Befehle auf macOS können nicht an localhost-Ports binden
* **Standard**: `false`

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

Listen Sie zusätzliche XPC- und Mach-Servicenamen auf, die die macOS-Sandbox nachschlagen kann. Tools, die über XPC kommunizieren, wie der iOS-Simulator oder Playwright, benötigen ihre Services hier aufgelistet.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Zeichenfolgen, jede ein Servicename; ein einzelner nachgestellter `*` gleicht ein Präfix ab, und `"*"` allein gleicht jeden Service ab
* **Standard**: nicht gesetzt

Dies erlaubt jeden Service unter dem `com.apple.coresimulator.`-Präfix:

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

Erlauben Sie Domänen vorab für ausgehenden Verkehr von sandboxed Befehlen, daher fordert die Sandbox nicht für sie auf. Wildcards wie `*.example.com` gleichen Subdomänen ab, und ein optionales `:port`-Suffix beschränkt einen Eintrag auf einen Port; ein Eintrag ohne Port gleicht jeden Port ab.

* **Bereich**: [`Any file`](#scopes). Nur verwaltete Einstellungen, wenn [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) gesetzt ist.
* **Typ**: Array von Zeichenfolgen, jede eine Domäne, ein Wildcard-Muster oder ein IP-Literal, mit einem optionalen `:port`-Suffix
* **Standard**: nicht gesetzt, daher fordert die Sandbox beim ersten Mal auf, wenn ein Befehl einen neuen Host erreicht

Dies erlaubt GitHub auf jedem Port, jede npm-Subdomain und einen API-Host auf Port 443 nur:

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org", "api.example.com:443"]
    }
  }
}
```

Schreiben Sie IPv6-Literale geklammert, mit einem optionalen Port: `"[::1]"` erlaubt jeden Port und `"[::1]:443"` einen Port. Die geklammerte Form erfordert Claude Code v2.1.229 oder später. Siehe [IPv6 addresses in domain lists](/docs/de/sandboxing#ipv6-addresses-in-domain-lists).

<h3 id="sandbox-network-denieddomains">
  `sandbox.network.deniedDomains`
</h3>

Blockieren Sie Domänen für ausgehenden Verkehr von sandboxed Befehlen, unter Verwendung der gleichen Wildcard-, Port- und IPv6-Syntax wie [`allowedDomains`](#sandbox-network-alloweddomains). Eine blockierte Domäne bleibt blockiert, auch wenn ein `allowedDomains`-Eintrag sie auch abgleicht.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Zeichenfolgen, jede eine Domäne, ein Wildcard-Muster oder ein IP-Literal, mit einem optionalen `:port`-Suffix
* **Standard**: nicht gesetzt

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "deniedDomains": ["sensitive.cloud.example.com"]
    }
  }
}
```

Claude Code führt diese Liste aus jeder Einstellungsquelle zusammen, die die Sitzung lädt, auch wenn `allowManagedDomainsOnly` gesetzt ist, daher kann ein Entwickler immer die Ablehnungsliste verschärfen. Für IPv6-Literale siehe [IPv6 addresses in domain lists](/docs/de/sandboxing#ipv6-addresses-in-domain-lists).

Ein Eintrag, der mit dem nachgestellten Punkt geschrieben ist, der einen vollständig qualifizierten Domänennamen markiert, wie `example.com.`, blockiert die gleichen Verbindungen wie `example.com`.

<h3 id="sandbox-network-strictallowlist">
  `sandbox.network.strictAllowlist`
</h3>

Verweigern Sie sandboxed Befehlen Zugriff auf Hosts außerhalb der Zulassungsliste, anstatt zur Genehmigung aufzufordern. Die Zulassungsliste ist [`allowedDomains`](#sandbox-network-alloweddomains) plus Domänen aus `WebFetch(domain:...)`-Zulassungsregeln, oder nur die verwalteten Einstellungseinträge, wenn [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) gesetzt ist. Erfordert Claude Code v2.1.219 oder später.

* **Bereich**: [`User or managed`](#scopes). Ein Repository kann es nicht ein- oder ausschalten.
* **Typ**: Boolescher Wert
  * `true`: Claude Code verweigert sandboxed Befehlen Zugriff auf Hosts außerhalb der Zulassungsliste
  * `false`: sofern nicht eine andere vertrauenswürdige Einstellungsdatei `true` setzt, entscheidet Claude Code einen Host außerhalb der Zulassungsliste nach Berechtigungsmodus, anstatt ihn direkt zu verweigern: es prüft den Host im Auto-Modus gegen die [per-command allowed domains](/docs/de/sandboxing#per-command-allowed-domains-in-auto-mode) des Befehls, verweigert im `dontAsk`-Modus, erlaubt im `bypassPermissions`-Modus und im Plan-Modus, wenn Bypass verfügbar ist, und fordert Sie ansonsten auf
* **Standard**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "strictAllowlist": true
    }
  }
}
```

Claude Code erzwingt dies nur für sandboxed Befehle; In-Process-Tools wie `WebFetch` folgen weiterhin ihren [Berechtigungsregeln](/docs/de/sandboxing#permission-rules). Wenn eine der beachteten Quellen es auf `true` setzt, bleibt es an. Siehe [Network isolation](/docs/de/sandboxing#network-isolation). Erfordert Claude Code v2.1.219 oder später.

<h3 id="sandbox-network-allowmanageddomainsonly">
  `sandbox.network.allowManagedDomainsOnly`
</h3>

Sperren Sie die Netzwerk-Zulassungsliste auf das, was verwaltete Einstellungen definieren. Claude Code beachtet dann nur `allowedDomains` und `WebFetch(domain:...)`-Zulassungsregeln aus verwalteten Einstellungen, ignoriert Domänen aus Benutzer-, Projekt-, lokalen und `--settings`-Einstellungen und blockiert eine nicht zulässige Domäne automatisch, anstatt aufzufordern.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Boolescher Wert
  * `true`: Claude Code beachtet nur `allowedDomains` und `WebFetch(domain:...)`-Zulassungsregeln aus verwalteten Einstellungen und blockiert eine nicht zulässige Domäne, anstatt aufzufordern
  * `false`: Domänen aus Benutzer-, Projekt-, lokalen und `--settings`-Einstellungen werden in die Zulassungsliste zusammengeführt
* **Standard**: `false`

Dies sperrt die Zulassungsliste auf GitHub und npm und ignoriert alle Domänen, die Entwickler hinzufügen:

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

Blockierte Domänen werden weiterhin aus jeder Quelle zusammengeführt, die die Sitzung lädt. Siehe [Keep developers from widening the policy](/docs/de/sandboxing#keep-developers-from-widening-the-policy).

<h3 id="sandbox-network-httpproxyport">
  `sandbox.network.httpProxyPort`
</h3>

Zeigen Sie die Sandbox auf Ihren eigenen HTTP-Proxy, anstatt auf den, den Claude Code ausführt. Organisationen tun dies, um HTTPS-Verkehr zu überprüfen, ihre eigenen Filterregeln anzuwenden oder jede Anfrage zu protokollieren. Wenn nicht gesetzt, startet Claude Code seinen eigenen Proxy für HTTP-Verkehr.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Zahl, ein lokaler TCP-Port
* **Standard**: nicht gesetzt, daher führt Claude Code seinen eigenen Proxy aus

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080
    }
  }
}
```

Setzen Sie auch [`socksProxyPort`](#sandbox-network-socksproxyport), wenn Ihr Proxy auch SOCKS-Verkehr tragen sollte; mit nur einem der beiden gesetzt, führt Claude Code weiterhin seinen eigenen Proxy für das andere Protokoll aus. Siehe [Custom proxy configuration](/docs/de/sandboxing#custom-proxy-configuration).

<h3 id="sandbox-network-socksproxyport">
  `sandbox.network.socksProxyPort`
</h3>

Zeigen Sie die Sandbox auf Ihren eigenen SOCKS5-Proxy, anstatt auf den, den Claude Code ausführt. Wenn nicht gesetzt, startet Claude Code seinen eigenen Proxy für SOCKS-Verkehr.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Zahl, ein lokaler TCP-Port
* **Standard**: nicht gesetzt, daher führt Claude Code seinen eigenen Proxy aus

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "socksProxyPort": 8081
    }
  }
}
```

Siehe [Custom proxy configuration](/docs/de/sandboxing#custom-proxy-configuration).

<h3 id="sandbox-network-tlsterminate">
  `sandbox.network.tlsTerminate`
</h3>

Lassen Sie den Sandbox-Proxy TLS beenden, damit er den Inhalt von HTTPS-Anfragen lesen kann. Dies ist experimentell, und `mask`-[Anmeldedaten-Ersetzung](/docs/de/sandboxing#mask-credentials) erfordert es. Setzen Sie `{}`, um eine ephemere Zertifizierungsstelle für die Sitzung zu generieren, oder setzen Sie `caCertPath` und `caKeyPath`, um Ihre eigene zu verwenden.

* **Bereich**: [`User or managed`](#scopes). Ein Repository kann es nicht einschalten oder eine Zertifizierungsstelle bereitstellen.
* **Typ**: Objekt mit optionalen `caCertPath`- und `caKeyPath`-Zeichenfolgen, jede ein Dateipfad
* **Standard**: nicht gesetzt, daher beendet oder überprüft der Proxy TLS nicht

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "tlsTerminate": {}
    }
  }
}
```

Wenn mehr als eine beachtete Quelle es setzt, verwendet Claude Code den Wert aus der Quelle mit der höchsten Priorität: verwaltete Einstellungen, dann das `--settings`-Flag, dann Benutzereinstellungen. Erfordert Claude Code v2.1.199 oder später.

<span id="context-and-memory" />

<h2 id="memory-and-context">
  Speicher und Kontext
</h2>

Steuern Sie, was Claude Code in den Kontext lädt, wie es komprimiert wird und wo es Speicher und Pläne speichert. Siehe [Kontext verwalten](/docs/de/context-window) und [Speicher](/docs/de/memory).

<h3 id="autocompactenabled">
  `autoCompactEnabled`
</h3>

Lassen Sie Claude Code [die Konversation automatisch komprimieren](/docs/de/context-window#when-your-context-fills-up), wenn sich der Kontext dem Limit nähert. Erscheint in `/config` als **Auto-compact**, und das Umschalten dort schreibt diesen Schlüssel in Ihre Benutzereinstellungen.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code komprimiert die Konversation automatisch, wenn sich der Kontext dem Limit nähert
  * `false`: Claude Code komprimiert nicht automatisch
* **Standard**: `true`
* **Sitzungsübersteuerungen**: [`DISABLE_AUTO_COMPACT`](/docs/de/env-vars) deaktiviert Auto-Compact für eine Sitzung; welcher der beiden es deaktiviert, der andere kann es nicht wieder aktivieren

```json settings.json theme={null}
{
  "autoCompactEnabled": false
}
```

Der manuelle `/compact`-Befehl funktioniert weiterhin, während Auto-Compact deaktiviert ist.

<h3 id="autocompactwindow">
  `autoCompactWindow`
</h3>

Legen Sie fest, wie voll das Kontextfenster wird, bevor Claude Code [automatisch komprimiert](/docs/de/context-window#when-your-context-fills-up).

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Anzahl der Token, von `100000` bis `1000000`. Claude Code begrenzt den Wert auf das Kontextfenster Ihres Modells; die [Modellübersicht](https://platform.claude.com/docs/en/about-claude/models/overview) listet das Fenster jedes Modells auf
* **Standard**: nicht gesetzt, daher wählt Claude Code ein für Ihr Modell optimiertes Fenster
* **Sitzungsübersteuerungen**: [`--autocompact`](/docs/de/cli-reference#cli-flags) hat Vorrang vor diesem Schlüssel für eine Sitzung, und [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/de/env-vars) hat Vorrang vor beiden

```json settings.json theme={null}
{
  "autoCompactWindow": 500000
}
```

Legen Sie es mit dem [`/autocompact`](/docs/de/commands#all-commands)-Befehl fest, der diesen Schlüssel in Ihre Benutzereinstellungen schreibt. [Auto-Compact-Fenster einstellen](/docs/de/model-config#set-the-auto-compact-window) behandelt, wie der Befehl, das Flag, die Variable und die Einstellung zusammenwirken.

<h3 id="automemorydirectory">
  `autoMemoryDirectory`
</h3>

Speichern Sie [automatischen Speicher](/docs/de/memory#storage-location) in einem Verzeichnis Ihrer Wahl statt in der projektspezifischen Standardeinstellung.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: String, ein absoluter oder mit `~/` präfixierter Verzeichnispfad
* **Standard**: nicht gesetzt, daher verwendet Claude Code `~/.claude/projects/<project>/memory/`

```json settings.json theme={null}
{
  "autoMemoryDirectory": "~/my-memory-dir"
}
```

Aus Projekt- oder lokalen Einstellungen respektiert Claude Code diesen Schlüssel unter der gleichen [Workspace-Vertrauensregel wie Hooks](/docs/de/permissions#what-runs-before-you-trust-a-folder), da ein geklontes Repository diese Dateien bereitstellen kann.

<h3 id="automemoryenabled">
  `autoMemoryEnabled`
</h3>

Schalten Sie [automatischen Speicher](/docs/de/memory#enable-or-disable-auto-memory) ein oder aus. Wenn `false`, liest Claude nicht aus dem automatischen Speicherverzeichnis und schreibt nicht dorthin. Sie können es auch während einer Sitzung mit `/memory` umschalten, was diesen Schlüssel in Ihre Benutzereinstellungen schreibt.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolean
  * `true`: dasselbe wie nicht gesetzt; automatischer Speicher bleibt aktiviert, es sei denn, etwas, das diesen Schlüssel übertrumpft, deaktiviert ihn für die Sitzung, wie `--bare`, sicherer Modus oder `CLAUDE_CODE_DISABLE_AUTO_MEMORY`
  * `false`: Claude liest nicht aus dem automatischen Speicherverzeichnis und schreibt nicht dorthin
* **Standard**: `true`
* **Sitzungsübersteuerungen**: [`CLAUDE_CODE_DISABLE_AUTO_MEMORY`](/docs/de/env-vars) hat Vorrang vor diesem Schlüssel für eine Sitzung, in beide Richtungen

```json settings.json theme={null}
{
  "autoMemoryEnabled": false
}
```

<h3 id="bashoutputmaxchars">
  `bashOutputMaxChars`
</h3>

Legen Sie fest, wie viele Zeichen der [Ausgabe eines erfolgreichen Bash- oder PowerShell-Befehls Claude inline erhält](/docs/de/tools-reference#output-limits). Wenn die Ausgabe das Limit überschreitet, speichert Claude Code sie in einer Datei und Claude erhält eine kurze Vorschau plus den Pfad der Datei. Erhöhen Sie das Limit, wenn die Befehlsausgabe, wie ein ausführlicher Build oder ein vollständiges Test-Suite-Protokoll, routinemäßig das Standard-Limit überschreitet und Sie möchten, dass Claude es liest, ohne die Datei zu öffnen. Erfordert Claude Code v2.1.261 oder später.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Anzahl der Zeichen, eine positive ganze Zahl. Claude Code begrenzt den Wert auf den Bereich `4000` bis `128000`
* **Standard**: nicht gesetzt, daher erhält Claude bis zu 30.000 Zeichen inline

```json settings.json theme={null}
{
  "bashOutputMaxChars": 100000
}
```

Wenn Sie diesen Schlüssel setzen, ignoriert Claude Code die Umgebungsvariable [`BASH_MAX_OUTPUT_LENGTH`](/docs/de/env-vars).

<h3 id="claudemd">
  `claudeMd`
</h3>

Injizieren Sie CLAUDE.md-ähnliche Anweisungen als von der Organisation verwalteter Speicher, ohne eine separate Datei bereitzustellen. Claude Code lädt den Text als verwalteten Speichereintrag vor Benutzer- und Projekt-CLAUDE.md-Dateien.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: String, der Text einer CLAUDE.md-Datei; schreiben Sie ihn wie die Datei, Markdown eingeschlossen, mit Zeilenumbrüchen als `\n`
* **Standard**: nicht gesetzt

Dieses Beispiel stellt zwei Regeln als kurze Markdown-Liste bereit:

```json managed-settings.json theme={null}
{
  "claudeMd": "# Engineering rules\n\n- Always run make lint before committing.\n- Never push directly to main."
}
```

Siehe [Organisationsweite CLAUDE.md bereitstellen](/docs/de/memory#deploy-organization-wide-claude-md).

<h3 id="claudemdexcludes">
  `claudeMdExcludes`
</h3>

Überspringen Sie spezifische `CLAUDE.md`-Dateien, wenn Claude Code [Speicher](/docs/de/memory#exclude-specific-claude-md-files) lädt. In einem großen Monorepo verwenden Sie es, um CLAUDE.md-Dateien von anderen Teams zu überspringen, die für Ihre Arbeit nicht relevant sind; [Irrelevante CLAUDE.md-Dateien ausschließen](/docs/de/large-codebases#exclude-irrelevant-claude-md-files) im Leitfaden für große Codebases führt Sie durch diesen Fall. Muster werden gegen absolute Dateipfade abgeglichen.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Strings, jeweils ein Glob-Muster oder absoluter Pfad
* **Standard**: nicht gesetzt, daher lädt Claude Code jede CLAUDE.md, die es findet

```json settings.json theme={null}
{
  "claudeMdExcludes": ["**/vendor/**/CLAUDE.md"]
}
```

Ausschlüsse gelten nur für Benutzer-, Projekt- und lokale Speicherdateien; verwaltete Richtlinien-CLAUDE.md-Dateien können nicht ausgeschlossen werden.

<span id="environment-variables" />

<h3 id="env">
  `env`
</h3>

Legen Sie Umgebungsvariablen für jede Sitzung und für die Unterprozesse fest, die Claude Code von ihr aus startet. Die meisten Variablen in der [Umgebungsvariablenreferenz](/docs/de/env-vars) können hier gehen, was ist, wie Sie eine auf jede Sitzung anwenden oder sie in Ihrem Team verteilen. Projekt- und lokale Einstellungen können [einige davon](#variables-claude-code-ignores-in-env) nicht setzen.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Objekt, das Variablennamen auf String-Werte abbildet
* **Standard**: nicht gesetzt

Dieses Beispiel deaktiviert die automatische Komprimierung und leitet API-Anfragen durch einen Proxy:

```json settings.json theme={null}
{
  "env": {
    "DISABLE_AUTO_COMPACT": "1",
    "ANTHROPIC_BASE_URL": "https://proxy.example.com"
  }
}
```

<h4 id="how-env-values-interact-with-your-shell">
  Wie `env`-Werte mit Ihrer Shell interagieren
</h4>

* Ein Wert hier überschreibt die gleiche Variable, die in Ihrer Shell exportiert wird, und wenn mehr als eine Einstellungsdatei eine Variable setzt, gilt die [höchste Priorität](/docs/de/settings#settings-precedence). [Variablen, die Claude Code in `env` ignoriert](#variables-claude-code-ignores-in-env) listet die Ausnahmen für Projekt- und lokale Einstellungen auf.
* Um einen Shell-Export zu stornieren, setzen Sie die Variable auf `""`. Claude Code behandelt einen leeren Wert als nicht gesetzt für die Anbieterauswahl, und Unterprozesse erben den leeren Wert.
* `NO_COLOR` und `FORCE_COLOR`, die hier gesetzt sind, erreichen nur Unterprozesse. Um die Farben der Claude Code-Oberfläche selbst zu ändern, setzen Sie sie in Ihrer Shell, bevor Sie `claude` starten.
* Werte hier sind Klartext in der Einstellungsdatei und erreichen jeden Unterprozess, den Claude Code startet. Für ein OTLP-Bearer-Token, das sich dreht, verwenden Sie [`otelHeadersHelper`](#otelheadershelper); für API-Anmeldedaten verwenden Sie [`apiKeyHelper`](#apikeyhelper).

<h4 id="when-claude-code-applies-env-values">
  Wann Claude Code `env`-Werte anwendet
</h4>

* Aus Benutzereinstellungen, `--settings` und verwalteten Einstellungen: beim Start und erneut in der laufenden Sitzung, wenn eine gespeicherte Änderung die zusammengeführte `env` ändert.
* Aus Projekt- und lokalen Einstellungen: nachdem Sie dem Workspace vertrauen, oder beim Start im `-p`-Modus, der niemals den Vertrauensdialog anzeigt, und erneut, wenn eine gespeicherte Änderung die zusammengeführte `env` ändert.
* Variablen, die Claude Code als sicher klassifiziert, wie Modellauswahl, Timeouts und Limits, und Feature-Toggles: beim Start aus jeder Einstellungsdatei, außer den [Variablen, die Projekt- und lokale Einstellungen nicht setzen können](#variables-claude-code-ignores-in-env).
* Nachdem Sie die Sitzung mit `/cd` [verschieben](/docs/de/permissions#move-the-session-to-another-directory) auf v2.1.246 oder später: die `env`-Werte des neuen Verzeichnisses, zusätzlich zu denen des vorherigen Verzeichnisses.

<h4 id="variables-claude-code-ignores-in-env">
  Variablen, die Claude Code in `env` ignoriert
</h4>

* Projekt- und lokale Einstellungen können keine Variablen setzen, die ein ausgechecktes Repository nicht kontrollieren sollte; setzen Sie diese stattdessen in Ihrer Shell, Benutzereinstellungen oder verwalteten Einstellungen. Claude Code löscht jede, außer einigen Werten, die Telemetrie ausschalten, und protokolliert eine Warnung, die Sie mit `claude --debug` sehen können. Sie umfassen:

  * Variablen, die wählen, wo Claude Code seine eigenen Dateien speichert oder schreibt: `CLAUDE_CONFIG_DIR`, `CLAUDE_CODE_TMPDIR` und die Betriebssystem-Verzeichnisvariablen wie `HOME`, `TMPDIR`, `TMP`, `TEMP` und die `XDG_*`-Familie.
  * Variablen, die Sitzungsinhalte exportieren: [`OTEL_LOG_RAW_API_BODIES`](/docs/de/env-vars#variables) und das detaillierte Beta-Tracing-Paar `ENABLE_BETA_TRACING_DETAILED` und `BETA_TRACING_ENDPOINT`.
  * Die [OpenTelemetry-Exporter](/docs/de/monitoring-usage)-Variablen, die Telemetrie aktivieren, wählen, wohin sie geht, oder wählen, welche Inhalte sie erfasst:

    * `CLAUDE_CODE_ENABLE_TELEMETRY`, plus das erweiterte Telemetrie-Beta-Paar `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` und `ENABLE_ENHANCED_TELEMETRY_BETA`
    * Die Exporter-Selektoren `OTEL_LOGS_EXPORTER`, `OTEL_METRICS_EXPORTER` und `OTEL_TRACES_EXPORTER`
    * Die Inhaltsvariablen `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_ASSISTANT_RESPONSES`, `OTEL_LOG_TOOL_CONTENT` und `OTEL_LOG_TOOL_DETAILS`
    * `OTEL_EXPORTER_OTLP_*`-Variablen, deren Namen auf `_ENDPOINT`, `_HEADERS`, `_PROTOCOL`, `_CERTIFICATE`, `_CLIENT_KEY` oder `_INSECURE` enden, in den generischen und pro-Signal-Formen, wie `OTEL_EXPORTER_OTLP_ENDPOINT` und `OTEL_EXPORTER_OTLP_METRICS_HEADERS`
    * `OTEL_EXPORTER_PROMETHEUS_HOST` und `OTEL_EXPORTER_PROMETHEUS_PORT`

    Nur diese Werte gelten weiterhin aus Projekt- und lokalen Einstellungen, da sie etwas ausschalten: `none` für die drei Exporter-Selektoren und ein Aus-Wert wie `0` für `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_CONTENT` und `OTEL_LOG_TOOL_DETAILS`. Ein solcher Wert überschreibt die gleiche Variable in Ihren Benutzereinstellungen, aber nicht eine, die die Umgebung, von der Sie Claude Code starten, eine `--settings`-Datei oder verwaltete Einstellungen setzen.

    Wenn eine Projekt- oder lokale Einstellungsdatei eine Variable in dieser Gruppe setzt, zeigt eine lokale interaktive Sitzung beim Start einen Hinweis. Führen Sie `/status` oder `claude doctor` aus, um zu sehen, welche Claude Code ignoriert hat und welche Telemetrie ausgeschaltet haben; beide listen Namen auf, niemals Werte. Ein nicht-interaktiver Lauf mit `-p` oder eine Agent SDK-Sitzung zeigt keinen Hinweis, daher überprüfen Sie, ob Ihr Collector nach dem Upgrade noch Daten empfängt. Wenn nicht, setzen Sie die Variablen in Ihren Benutzereinstellungen, verwalteten Einstellungen, der Umgebung des Jobs oder einer Datei, die Sie mit `--settings` übergeben.

    Das Ignorieren dieser Gruppe in Projekt- und lokalen Einstellungen erfordert Claude Code v2.1.282 oder später.
  * Variablen, die ändern, wie Claude Code startet oder synchronisiert, wie `CLAUDE_CODE_PROCESS_WRAPPER`, `CLAUDE_CODE_SYNC_SKILLS`, `CLAUDE_CODE_SYNC_PLUGINS`, `CLAUDE_CODE_PLUGIN_CACHE_DIR` und `CLAUDE_CODE_PLUGIN_SEED_DIR`.

  Vor v2.1.251 konnten Projekt- und lokale Einstellungen auch die Variablen in dieser Liste setzen, die wählen, wo Claude Code seine Dateien schreibt oder die Sitzungsinhalte exportieren, außer `HOME` und `XDG_CONFIG_HOME`.
* Identitätsvariablen, die Claude Codes Hosting-Umgebungen besitzen, wie `CLAUDE_CODE_REMOTE` und `CLAUDE_CODE_ACCOUNT_UUID`, werden aus jeder Datei ignoriert.
* [`CLAUDE_CODE_MESSAGING_SOCKET` und `CLAUDE_CODE_MESSAGING_TOKEN`](/docs/de/env-vars#variables), die Claude Code selbst exportiert, werden aus jeder Datei ignoriert. Das Ignorieren der Socket-Variable erfordert Claude Code v2.1.224 oder später, und das Ignorieren des Tokens erfordert v2.1.228 oder später.
* [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/de/sessions#name-the-project-directory-yourself), das Claude Code nur aus der Start-Umgebung liest, wird aus jeder Datei ignoriert; erfordert v2.1.234 oder später.
* [`CLAUDE_CODE_RESTRICTED`](/docs/de/env-vars#variables), das Claude Code nur aus der Start-Umgebung liest, wird aus jeder Datei ignoriert.

<h3 id="filecheckpointingenabled">
  `fileCheckpointingEnabled`
</h3>

Lassen Sie Claude Code Dateien vor jeder Bearbeitung als Snapshot erstellen, damit [`/rewind`](/docs/de/checkpointing) sie wiederherstellen kann. Erscheint in `/config` als **Rewind code (checkpoints)**, und das Umschalten dort schreibt diesen Schlüssel in Ihre Benutzereinstellungen.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code erstellt Snapshots von Dateien vor jeder Bearbeitung, damit `/rewind` sie wiederherstellen kann
  * `false`: Claude Code erstellt keine Snapshots von Dateien, daher kann `/rewind` sie nicht wiederherstellen
* **Standard**: `true`
* **Sitzungsübersteuerungen**: [`CLAUDE_CODE_DISABLE_FILE_CHECKPOINTING`](/docs/de/env-vars) deaktiviert Checkpointing für eine Sitzung; welcher der beiden es deaktiviert, der andere kann es nicht wieder aktivieren

```json settings.json theme={null}
{
  "fileCheckpointingEnabled": false
}
```

In einem `-p`-Lauf oder einer Agent SDK-Sitzung ignoriert Claude Code diesen Schlüssel. Das SDK aktiviert Checkpointing mit seiner `enableFileCheckpointing`-Option, und ein einfacher `-p`-Lauf benötigt `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true`. Siehe [Datei-Checkpointing im Agent SDK](/docs/de/agent-sdk/file-checkpointing).

<h3 id="plansdirectory">
  `plansDirectory`
</h3>

Wählen Sie, wo Claude Code die Plandateien speichert, die es im [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) schreibt. Claude Code löst den Pfad relativ zum Projektstamm auf und behält den Standard bei, wenn der Pfad außerhalb davon aufgelöst wird.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: String, ein Pfad relativ zum Projektstamm
* **Standard**: nicht gesetzt, daher verwendet Claude Code `~/.claude/plans`

```json settings.json theme={null}
{
  "plansDirectory": "./plans"
}
```

<h3 id="skilllistingbudgetfraction">
  `skillListingBudgetFraction`
</h3>

Jede Runde sieht Claude eine [Auflistung Ihrer Skills](/docs/de/skills#skill-descriptions-are-cut-short) mit ihren Beschreibungen, und Claude Code begrenzt diese Auflistung auf einen Anteil des Kontextfensters. Wenn die Auflistung über der Obergrenze liegt, behält Claude Code jeden Skill-Namen, löscht aber die Beschreibungen der am wenigsten verwendeten Skills, damit Claude diese Skills immer noch aufrufen kann, aber weniger wahrscheinlich selbst einen auswählt. Erhöhen Sie diesen Schlüssel, um mehr Beschreibungen sichtbar zu halten, auf Kosten von mehr Kontext pro Runde.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Zahl, ein Bruch größer als `0` und höchstens `1`
* **Standard**: `0.01`, das 1% des Kontextfensters reserviert

```json settings.json theme={null}
{
  "skillListingBudgetFraction": 0.02
}
```

Um zu sehen, wie viel Kontext die Auflistung verwendet und welche Skills am meisten beitragen, führen Sie `/doctor` aus.

<h3 id="skilllistingmaxdescchars">
  `skillListingMaxDescChars`
</h3>

Jede Runde sieht Claude eine [Auflistung Ihrer Skills](/docs/de/skills#skill-descriptions-are-cut-short), die den `description`- und `when_to_use`-Text jedes Skills zeigt. Dieser Schlüssel begrenzt, wie viele Zeichen dieses Textes Claude Code pro Skill anzeigt; längerer Text wird bei der Obergrenze abgeschnitten.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Anzahl der Zeichen, eine positive ganze Zahl
* **Standard**: `1536`

```json settings.json theme={null}
{
  "skillListingMaxDescChars": 2048
}
```

Erhöhen Sie es, um lange Beschreibungen intakt zu halten, auf Kosten von mehr Kontext pro Runde; senken Sie es, um mehr Skills unter [`skillListingBudgetFraction`](#skilllistingbudgetfraction) zu passen.

<h3 id="taskoutputmaxchars">
  `taskOutputMaxChars`
</h3>

<Warning>
  Entfernt in v2.1.277, zusammen mit dem `TaskOutput`-Tool, das es dimensioniert. Das Setzen hat keine Auswirkung auf aktuelle Versionen. Claude liest eine [Ausgabedatei](/docs/de/tools-reference#background-commands) einer Hintergrundaufgabe stattdessen mit `Read`.
</Warning>

Bis v2.1.276 setzen Sie diesen Schlüssel auf die Anzahl der Zeichen einer [Hintergrundaufgabe](/docs/de/tools-reference#background-commands) Ausgabe, die Claude inline erhielt, wenn es die Aufgabe mit dem `TaskOutput`-Tool las.

<h2 id="interface-and-terminal">
  Schnittstelle und Terminal
</h2>

Ändern Sie das Aussehen und Verhalten von Claude Code in Ihrem Terminal: Design, Editor-Modus, Statuszeile, Spinner, Benachrichtigungen innerhalb der Sitzung und Barrierefreiheit. Siehe [Terminal-Konfiguration](/docs/de/terminal-config).

<h3 id="askuserquestiontimeout">
  `askUserQuestionTimeout`
</h3>

Lassen Sie einen unbeantworteten [`AskUserQuestion`](/docs/de/tools-reference)-Dialog nach einer Leerlaufzeit automatisch fortfahren und dabei alle bereits ausgewählten Optionen einreichen. Legen Sie dies fest, wenn Sie sich abmelden und möchten, dass Claude ohne Sie fortfährt. Mit der Standardeinstellung warten Fragen, bis Sie sie beantworten. Erfordert Claude Code v2.1.200 oder später.

* **Bereich**: [`Benutzer oder verwaltet`](#scopes)
* **Typ**: String, einer von `"60s"`, `"5m"`, `"10m"` oder `"never"`
* **Standard**: `"never"`
* **Sitzungsübergreifende Außerkraftsetzungen**: [`CLAUDE_AFK_TIMEOUT_MS`](/docs/de/env-vars) hat Vorrang vor diesem Schlüssel für eine Sitzung

```json settings.json theme={null}
{
  "askUserQuestionTimeout": "5m"
}
```

Erscheint in `/config` als **Frage automatisch fortsetzen Timeout**, das diesen Schlüssel in Benutzereinstellungen schreibt; Claude Code blendet die Zeile aus, während verwaltete Einstellungen oder das Flag `--settings` den Schlüssel setzen. Erfordert Claude Code v2.1.200 oder später.

<h3 id="autocontinueatusagelimit">
  `autoContinueAtUsageLimit`
</h3>

Nachdem ein claude.ai-Nutzungslimit Ihre Sitzung stoppt, warten Sie in der offenen Sitzung und fahren Sie die Aufgabe nach dem Zurücksetzen automatisch fort. Siehe [Automatisches Fortsetzen ausschalten](/docs/de/interactive-mode#turn-automatic-continue-off). Erfordert Claude Code v2.1.234 oder später.

* **Bereich**: [`Benutzer oder verwaltet`](#scopes). Lesen Sie aus Benutzereinstellungen, `--settings` und verwalteten Einstellungen nur. Wenn keiner dieser Einstellungen den Schlüssel setzt, schaltet eine Projekt- oder lokale Einstellungsdatei, die ihn setzt, die Funktion aus, anstatt ignoriert zu werden.
* **Typ**: Boolean
  * `true`: Nachdem ein claude.ai-Nutzungslimit Ihre Sitzung stoppt, wartet Claude Code in der offenen Sitzung und setzt die Aufgabe nach dem Zurücksetzen automatisch fort
  * `false`: Claude Code startet das Warten nicht von selbst. Sie können immer noch [ein Warten selbst starten](/docs/de/interactive-mode#start-a-wait-yourself) aus dem Menü der Nutzungslimit-Optionen
* **Standard**: `true`

```json settings.json theme={null}
{
  "autoContinueAtUsageLimit": false
}
```

Erscheint in `/config` als **Automatisch bei Nutzungslimit fortsetzen**, das diesen Schlüssel in Benutzereinstellungen schreibt; Claude Code blendet die Zeile aus, während verwaltete Einstellungen oder das Flag `--settings` den Schlüssel setzen.

<h3 id="autoscrollenabled">
  `autoScrollEnabled`
</h3>

Folgen Sie neuer Ausgabe zum unteren Ende des Gesprächs in [Vollbilddarstellung](/docs/de/fullscreen). Schalten Sie es aus, um dort zu bleiben, wo Sie gescrollt haben, während Claude weiterarbeitet; Berechtigungsaufforderungen werden immer noch in die Ansicht gescrollt.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: Das Gespräch folgt neuer Ausgabe zum unteren Ende
  * `false`: Sie bleiben dort, wo Sie gescrollt haben, während Claude weiterarbeitet; Berechtigungsaufforderungen werden immer noch unter dem Transkript angezeigt
* **Standard**: `true`

```json settings.json theme={null}
{
  "autoScrollEnabled": false
}
```

Erscheint in `/config` als **Automatisches Scrollen**, wenn Vollbilddarstellung aktiviert ist, das diesen Schlüssel in Benutzereinstellungen schreibt.

<h3 id="axscreenreader">
  `axScreenReader`
</h3>

Rendern Sie bildschirmleserfreundliche Ausgabe: flacher Text ohne dekorative Rahmen oder Animationen. Der Bildschirmlesermodus verwendet den klassischen Renderer, daher hat die Einstellung `tui` keine Auswirkung, während er aktiv ist; angehängte [Hintergrundsitzungen](/docs/de/agent-view) werden immer noch im Vollbild gerendert.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code rendert flachen Text ohne dekorative Rahmen oder Animationen mit dem klassischen Renderer
  * `false`: Claude Code rendert normal
* **Standard**: nicht gesetzt, daher ist der Bildschirmlesermodus aus
* **Sitzungsübergreifende Außerkraftsetzungen**: [`--ax-screen-reader`](/docs/de/cli-reference#cli-flags) hat Vorrang vor [`CLAUDE_AX_SCREEN_READER`](/docs/de/env-vars), und beide haben Vorrang vor diesem Schlüssel für eine Sitzung

```json settings.json theme={null}
{
  "axScreenReader": true
}
```

<h3 id="basheditdiffenabled">
  `bashEditDiffEnabled`
</h3>

Wählen Sie, ob Claude Code die Dateien aufzeichnet, die ein Bash-Befehl in einem Git-Repository ändert. Wenn es sie aufzeichnet, sehen Sie ihren Diff im Terminal nach dem Befehl, und Ihre [PostToolUse Bash Hooks](/docs/de/hooks#bash) erhalten die Liste der geänderten Dateien.

Eine aufgelistete Datei ist nicht immer eine, die der Befehl geändert hat. Eine Änderung, die ein anderes Programm oder ein anderer Bash-Aufruf vorgenommen hat, während der Befehl lief, kann dort auch erscheinen.

Setzen Sie den Schlüssel auf `true`, um sie in jedem Berechtigungsmodus aufzuzeichnen. Erfordert Claude Code v2.1.269 oder später.

* **Bereich**: [`Benutzer oder verwaltet`](#scopes). Ein `true` zählt nur aus Ihren Benutzereinstellungen, JSON, das mit `--settings` übergeben wird, oder [verwalteten Einstellungen](/docs/de/managed-settings), daher kann ein `true` in der `.claude/settings.json` oder `.claude/settings.local.json` eines Repositories die Aufzeichnung nicht einschalten. Ein `false` in einer der Repository-Dateien schaltet es immer noch aus, es sei denn, eine [höher priorisierte](/docs/de/settings#settings-precedence) Datei setzt `true`.
* **Typ**: Boolean
* **Standard**: nicht gesetzt, daher zeichnet Claude Code Änderungen im Auto-Modus und `bypassPermissions`-Modus auf, wenn es Claude anweist, Dateien über Bash zu bearbeiten
* **Sitzungsübergreifende Außerkraftsetzungen**: [`CLAUDE_CODE_BASH_EDIT_DIFF`](/docs/de/env-vars) hat Vorrang vor diesem Schlüssel für eine Sitzung

```json settings.json theme={null}
{
  "bashEditDiffEnabled": true
}
```

<h3 id="companyannouncements">
  `companyAnnouncements`
</h3>

Zeigen Sie die Ankündigungen Ihrer Organisation den Benutzern beim Start an. Wenn Sie mehr als eine auflisten, wählt Claude Code für jede Sitzung zufällig eine aus; beim allerersten Start eines Benutzers wird der erste Eintrag angezeigt.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Array von Strings
* **Standard**: nicht gesetzt, daher wird keine Ankündigung angezeigt

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

Wählen Sie, ob Bash oder PowerShell die Shell-Befehle ausführt, die Sie mit dem Präfix [`!`](/docs/de/interactive-mode#shell-mode-with-prefix) im Eingabefeld eingeben, die Claude Code direkt ausführt und zur Sitzung hinzufügt.

`"powershell"` funktioniert nur, wenn das [PowerShell-Tool](/docs/de/tools-reference#powershell-tool) aktiviert ist. Das Tool ist standardmäßig unter Windows ohne Git Bash aktiviert und unter Windows mit Git Bash für claude.ai- und Console-Konten. In Amazon Bedrock-, Google Cloud Agent Platform- und Microsoft Foundry-Sitzungen sowie unter macOS, Linux und WSL setzen Sie `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`, um das Tool zu aktivieren. Setzen Sie diese Variable auf `0`, um das Tool auszuschalten.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: String, einer von:
  * `"bash"`: Claude Code führt Ihre `!`-Befehle in Bash aus
  * `"powershell"`: Claude Code führt Ihre `!`-Befehle in PowerShell aus
* **Standard**: `"bash"` oder `"powershell"` unter Windows, wenn Bash nicht verfügbar ist

```json settings.json theme={null}
{
  "defaultShell": "powershell"
}
```

Wenn die Shell, die Sie benennen, nicht verfügbar ist, verwendet Claude Code die andere: `"powershell"` fällt auf Bash zurück, wenn das PowerShell-Tool aus ist, und `"bash"` fällt auf PowerShell zurück, wenn Bash nicht installiert ist.

<h3 id="dialogexpiry">
  `dialogExpiry`
</h3>

Legen Sie die Frist für Dialoge fest, die Claude Code [an einen Remote-Client weiterleitet](/docs/de/remote-control#limitations), wie einen Remote Control oder SDK-Host, und für den Genehmigungsdialog für eine [gehaltene sitzungsübergreifende Nachricht](/docs/de/cross-session-messaging#control-inbound-messages). In Claude Code v2.1.236 oder später begrenzt die gleiche Frist die Aufforderung zur Zustimmung für Fable-Nutzungsguthaben in der Mitte der Sitzung]\(/de/model-config#fable-and-usage-credits) in einer Sitzung, in der möglicherweise niemand am Terminal ist. Wenn vor der Frist keine Antwort eintrifft, bricht Claude Code den Dialog ab und fährt mit seinem Standardwert ohne Aktion fort. Erfordert Claude Code v2.1.224 oder später.

* **Bereich**: [`Benutzer oder verwaltet`](#scopes)
* **Typ**: String, einer von `"60s"`, `"5m"`, `"10m"` oder `"never"`, das die Frist deaktiviert
* **Standard**: `"5m"`
* **Sitzungsübergreifende Außerkraftsetzungen**: [`CLAUDE_CODE_USER_DIALOG_TIMEOUT_MS`](/docs/de/env-vars) hat Vorrang vor diesem Schlüssel für eine Sitzung

```json settings.json theme={null}
{
  "dialogExpiry": "10m"
}
```

Berechtigungsaufforderungen und [`AskUserQuestion`](/docs/de/tools-reference#askuserquestion-tool-behavior)-Fragen verwenden ihre eigenen Abläufe und werden nicht durch diese Frist geregelt. Erscheint in `/config` als **Dialog-Ablauf**, das diesen Schlüssel in Benutzereinstellungen schreibt; die Zeile erfordert Claude Code v2.1.232 oder später, und Claude Code blendet sie aus, während verwaltete Einstellungen oder das Flag `--settings` den Schlüssel setzen.

<h3 id="editormode">
  `editorMode`
</h3>

Wählen Sie den Tastenbindungsmodus für die Eingabeaufforderung.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: String, einer von:
  * `"normal"`: Standard-Tastenbindungen in der Eingabeaufforderung
  * `"vim"`: Vim-ähnliche Bearbeitung mit NORMAL-, INSERT- und VISUAL-Modi
* **Standard**: `"normal"`

```json settings.json theme={null}
{
  "editorMode": "vim"
}
```

Erscheint in `/config` als **Editor-Modus**, das diesen Schlüssel in Benutzereinstellungen schreibt.

<h3 id="emojicompletionenabled">
  `emojiCompletionEnabled`
</h3>

Zeigen Sie Emoji-Vorschläge an, wenn Sie `:` plus einen Shortcode in der Eingabeaufforderung eingeben, und ersetzen Sie einen abgeschlossenen Shortcode wie `:heart:` durch sein Emoji. Setzen Sie es auf `false`, um beides auszuschalten.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code zeigt Emoji-Vorschläge nach `:` an und ersetzt einen abgeschlossenen Shortcode durch sein Emoji
  * `false`: Claude Code schlägt weder Emoji vor noch ersetzt Shortcodes
* **Standard**: `true`

```json settings.json theme={null}
{
  "emojiCompletionEnabled": false
}
```

Siehe [Emoji-Shortcodes](/docs/de/interactive-mode#emoji-shortcodes). Erfordert Claude Code v2.1.217 oder später.

<span id="file-suggestion-settings" />

<h3 id="filesuggestion">
  `fileSuggestion`
</h3>

Führen Sie Ihren eigenen Befehl aus, um `@`-Dateipfad-Autovervollständigung anstelle des integrierten Dateivorschlags bereitzustellen. Der integrierte Vorschlag verwendet schnelle Dateisystem-Durchquerung; ein großes Monorepo könnte von projektspezifischer Indizierung wie einem vordefinierten Dateiindex profitieren.

* **Bereich**: [`Beliebige Datei`](#scopes). Unter den [Statuszeilen- und Dateivorschlag-Gates](#status-line-and-file-suggestion-gates) schaltet Claude Code den Befehl aus oder führt nur einen verwalteten Wert aus und überspringt Ihren ohne Warnung.
* **Typ**: Objekt mit `type`, immer `"command"`, und `command`, dem auszuführenden Shell-Befehl
* **Standard**: nicht gesetzt, daher verwendet Claude Code den integrierten Dateivorschlag

```json settings.json theme={null}
{
  "fileSuggestion": {
    "type": "command",
    "command": "~/.claude/file-suggestion.sh"
  }
}
```

Nachdem Sie dies gespeichert haben, geben Sie `@` gefolgt von Teil eines Pfads in der Eingabeaufforderung ein: Die Vorschläge stammen aus der Ausgabe Ihres Befehls.

<h4 id="command-input-and-output">
  Befehlseingabe und -ausgabe
</h4>

Claude Code führt den Befehl mit den gleichen Umgebungsvariablen wie [Hooks](/docs/de/hooks) aus, einschließlich `CLAUDE_PROJECT_DIR`, und stoppt das Warten nach fünf Sekunden. Der Befehl empfängt JSON auf stdin mit einem `query`-Feld, das enthält, was Sie bisher eingegeben haben:

```json theme={null}
{"query": "src/comp"}
```

Geben Sie zeilengetrennte Dateipfade auf stdout aus. Claude Code zeigt höchstens 15:

```text theme={null}
src/components/Button.tsx
src/components/Modal.tsx
src/components/Form.tsx
```

Das folgende Skript liest die Abfrage und übergibt sie an einen Repository-Dateiindex:

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

Rendern Sie zusätzliche anklickbare Abzeichen in der Fußzeile unter dem Eingabefeld, wenn ein Regex die Ausgabe einer Runde abgleicht: Tool-Ergebnisse, einschließlich Dateiinhalte und abgerufene Seiten, und Claudes eigene Antworten. Verwenden Sie es, um IDs, die von Projekt-CLIs wie Review-Tools und Issue-Trackern gedruckt werden, in Sitzungslinks umzuwandeln.

* **Bereich**: [`Benutzer oder verwaltet`](#scopes)
* **Typ**: Array von Objekten, jedes mit `type` auf `"regex"` gesetzt, einem `pattern`-Regex, einer `url`-Vorlage und einem optionalen `label`; `{name}`-Platzhalter in `url` und `label` werden aus benannten Erfassungsgruppen in `pattern` gefüllt
* **Standard**: nicht gesetzt, daher werden keine Abzeichen gerendert

Dieses Beispiel gleicht Issue-Schlüssel wie `PROJ-1234` ab und erstellt jeden Link aus dem erfassten Schlüssel:

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

Mit dieser Konfiguration wird, wenn `PROJ-1234` in einem Tool-Ergebnis oder in Claudes Antwort erscheint, ein `PROJ-1234`-Abzeichen in der Fußzeile angezeigt, das auf `https://issues.example.com/browse/PROJ-1234` verlinkt.

<h4 id="badge-constraints">
  Abzeichen-Einschränkungen
</h4>

Die URL, das Label und die Abzeichen-Anzahl jedes Eintrags sind wie folgt begrenzt:

| Einschränkung    | Verhalten                                                                                                                                                                                                              |
| :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| URL-Ursprung     | Erfasste Werte werden URL-codiert und die konstruierte URL muss den Ursprung der Vorlage teilen. Eine Erfassung kann ein Pfadsegment oder einen Abfragewert ausfüllen, kann aber nicht ändern, wohin der Link verweist |
| URL-Länge        | Konstruierte URLs länger als 2048 Zeichen werden verworfen                                                                                                                                                             |
| URL-Schema       | Muss `https`, `http` oder ein erkanntes Editor- oder Workspace-Deep-Link-Schema sein: `vscode`, `vscode-insiders`, `cursor`, `windsurf`, `zed`, `jetbrains`, `idea`, `slack`, `linear`, `notion`, `figma`              |
| Label            | Standardmäßig der abgeglichene Text und wird auf 28 Anzeigespalten gekürzt                                                                                                                                             |
| Abzeichen-Anzahl | Höchstens 5 Abzeichen werden gerendert. Das älteste wird durch neuere Übereinstimmungen verdrängt und `/clear` entfernt sie                                                                                            |

Wenn eine Runde abgeschlossen ist, gleicht Claude Code jeden `pattern`-Regex des Eintrags gegen die Ausgabe der Runde im Hauptthread ab, daher blockiert ein langsamer Regex die Benutzeroberfläche, bis er fertig ist. Verschachtelte Quantoren wie `(a+)+$` können gegen bestimmte Eingaben exponentiell lange dauern und die Sitzung einfrieren, daher halten Sie jeden `pattern` linear und vermeiden Sie Verschachtelung von `+` oder `*`.

Fußzeilen-Abzeichen werden neben einer [benutzerdefinierten Statuszeile](/docs/de/statusline) gerendert, wenn eine konfiguriert ist; keiner ersetzt den anderen. Verwenden Sie eine Statuszeile für eine skriptgesteuerte Zeile, die ihren eigenen Inhalt aus Sitzungsdaten berechnet, und Fußzeilen-Abzeichen, um IDs aus dem Gespräch in Links umzuwandeln, ohne ein Skript.

<h3 id="keybindingflavor">
  `keybindingFlavor`
</h3>

<Warning>
  Veraltet seit v2.1.261 und hat keine Auswirkung. Die Wort-Bearbeitungstasten der Eingabeaufforderung folgen immer [Readline-Konventionen](/docs/de/interactive-mode#make-ctrl-w-delete-back-to-whitespace), wie in Bash. Claude Code akzeptiert immer noch `keybindingFlavor`, daher bleibt eine Einstellungsdatei, die es setzt, gültig.
</Warning>

In v2.1.238 bis v2.1.260 machte das Setzen auf `"readline"` `Ctrl+W` zum Löschen zurück zum vorherigen Leerzeichen anstatt nur zum vorherigen Wort.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: String, `"classic"` oder `"readline"`
* **Standard**: nicht gesetzt

<h3 id="prefersreducedmotion">
  `prefersReducedMotion`
</h3>

Reduzieren oder schalten Sie Schnittstellen-Animationen wie den Spinner, Shimmer und Flash-Effekte aus. Erscheint in `/config` als **Bewegung reduzieren**.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code reduziert oder schaltet Schnittstellen-Animationen wie den Spinner, Shimmer und Flash-Effekte aus
  * `false`: das gleiche wie nicht gesetzt; Claude Code zeigt seine Animationen
* **Standard**: `false`

```json settings.json theme={null}
{
  "prefersReducedMotion": true
}
```

<h3 id="promptsuggestionenabled">
  `promptSuggestionEnabled`
</h3>

Zeigen Sie [Eingabeaufforderungs-Vorschläge](/docs/de/interactive-mode#prompt-suggestions) an oder verbergen Sie sie, die ausgegraut Vorhersagen, die in Ihrer Eingabeaufforderung erscheinen. Setzen Sie es auf `false` oder schalten Sie **Eingabeaufforderungs-Vorschläge** in `/config` aus, um sie zu verbergen.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: Sie sehen Eingabeaufforderungs-Vorschläge in Ihrer Eingabeaufforderung
  * `false`: Claude Code verbirgt Eingabeaufforderungs-Vorschläge
* **Standard**: `true`
* **Sitzungsübergreifende Außerkraftsetzungen**: [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/de/env-vars) hat Vorrang vor diesem Schlüssel für eine Sitzung

```json settings.json theme={null}
{
  "promptSuggestionEnabled": false
}
```

Eingabeaufforderungs-Vorschläge benötigen ein claude.ai- oder Console-Konto mit aktivierter Telemetrie. In Amazon Bedrock, Google Cloud Agent Platform und Microsoft Foundry oder mit ausgeschalteter Telemetrie, wie durch [`DISABLE_TELEMETRY`](/docs/de/env-vars), hat dieser Schlüssel keine Auswirkung und nur `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=1` schaltet sie ein.

<h3 id="respectgitignore">
  `respectGitignore`
</h3>

Kontrollieren Sie, ob der `@`-Datei-Picker Dateien auslässt, die `.gitignore`-Muster entsprechen. Erscheint in `/config` als **Respektiere .gitignore im Datei-Picker**.

* **Bereich**: [`Beliebige Datei`](#scopes). Wenn keine Einstellungsdatei es setzt, fällt Claude Code auf `respectGitignore` in `~/.claude.json` zurück, das der `/config`-Toggle schreibt.
* **Typ**: Boolean
  * `true`: Der `@`-Datei-Picker lässt Dateien aus, die `.gitignore`-Muster entsprechen
  * `false`: Der `@`-Datei-Picker enthält Dateien, die `.gitignore`-Muster entsprechen
* **Standard**: `true`

```json settings.json theme={null}
{
  "respectGitignore": false
}
```

<h3 id="respondtobashcommands">
  `respondToBashCommands`
</h3>

Wählen Sie, ob Claude antwortet, nachdem Sie einen Shell-Befehl mit dem Präfix [`!`](/docs/de/interactive-mode#shell-mode-with-prefix) im Eingabefeld ausführen. Standardmäßig fügt Claude Code die Ausgabe des Befehls zum Gespräch hinzu und Claude antwortet darauf. Setzen Sie diesen Schlüssel auf `false`, um die Ausgabe zum Kontext hinzuzufügen, ohne eine Antwort zu geben, damit Sie mehrere Befehle ausführen und zusammen darüber sprechen können.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code fügt die Ausgabe des Befehls zum Gespräch hinzu und Claude antwortet darauf
  * `false`: Claude Code fügt die Ausgabe zum Kontext hinzu, ohne eine Antwort zu geben
* **Standard**: `true`

```json settings.json theme={null}
{
  "respondToBashCommands": false
}
```

Siehe [Shell-Modus mit `!`-Präfix](/docs/de/interactive-mode#shell-mode-with-prefix).

<h3 id="showclearcontextonplanaccept">
  `showClearContextOnPlanAccept`
</h3>

Wenn Claude einen Plan im [Plan-Modus](/docs/de/permission-modes#review-and-approve-a-plan) abschließt, zeigt es ein Genehmigungsmenü. Die Planung kann viel Kontext verwenden, daher fügt dieser Schlüssel eine erste Option zu diesem Menü hinzu, **Ja, Kontext löschen und …**, die den Plan genehmigt, den Gesprächskontext löscht und die Implementierung nur aus dem Plan startet. Der Rest des Labels benennt den Berechtigungsmodus, in dem die Sitzung fortgesetzt wird, und zeigt, wie viel Ihres Kontexts die Planung verwendet hat.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: Das Plan-Genehmigungsmenü erhält eine erste Option, **Ja, Kontext löschen und …**, die den Plan genehmigt und den Gesprächskontext löscht
  * `false`: Das Plan-Genehmigungsmenü zeigt keine Clear-Context-Option
* **Standard**: `false`

```json settings.json theme={null}
{
  "showClearContextOnPlanAccept": true
}
```

<h3 id="showturnduration">
  `showTurnDuration`
</h3>

Zeigen Sie die Nachricht zur Rundendauer nach jeder Antwort an oder verbergen Sie sie, wie z. B. „Cooked for 1m 6s · done 6:05 PM". Die Uhr nach „done" zeigt, wann die Runde fertig war; [`timeFormat`](#timeformat) und [`timeZone`](#timezone) kontrollieren ihr Format und ihre Zone. Erscheint in `/config` als **Rundendauer anzeigen**.

* **Bereich**: [`Beliebige Datei`](#scopes). Ein Wert in `~/.claude.json` aus einer älteren Version gilt, wenn keine Einstellungsdatei es setzt.
* **Typ**: Boolean
  * `true`: Sie sehen die Nachricht zur Rundendauer nach jeder Antwort
  * `false`: Claude Code verbirgt die Nachricht zur Rundendauer
* **Standard**: `true`

```json settings.json theme={null}
{
  "showTurnDuration": false
}
```

<h3 id="spellcheck">
  `spellcheck`
</h3>

Unterstreichen Sie falsch geschriebene Wörter in der Eingabeaufforderung, während Sie eingeben, mit einem Rechtschreibprüfer, den Sie installieren. Claude Code prüft nur den Text im Eingabefeld. [Rechtschreibung während der Eingabe prüfen](/docs/de/interactive-mode#check-spelling-as-you-type) behandelt die Installation von aspell, hunspell oder ispell und was der Checker abdeckt. Erfordert Claude Code v2.1.235 oder später.

* **Bereich**: [`Benutzer oder verwaltet`](#scopes). Der Block aus der höchsten Ebene, die ihn setzt, gilt als Ganzes.
* **Typ**: Objekt mit `enabled` (Boolean), `checker` (`"aspell"`, `"hunspell"`, `"ispell"` oder `"auto"`), `language` (String, an den Checker als sein Wörterbuchname übergeben) und `color` (String, ein Terminal-Farbname, `#rrggbb`, `rgb(r,g,b)`, `ansi256(n)` oder `ansi:<name>`)
* **Standard**: nicht gesetzt, daher ist die Rechtschreibprüfung aus; `checker` standardmäßig auf `"auto"`, das erste der drei auf `PATH` gefundenen; `language` standardmäßig auf das eigene Wörterbuch des Checkers; `color` standardmäßig auf die Fehlerfarbe des Designs

```json settings.json theme={null}
{
  "spellcheck": { "enabled": true, "language": "en_GB" }
}
```

<h3 id="spinnertipsenabled">
  `spinnerTipsEnabled`
</h3>

Während Claude arbeitet, rotiert die Spinner-Zeile durch kurze Tipps zu Claude Code-Funktionen, wie z. B. „Use Plan Mode to prepare for a complex request before making changes. Press Shift+Tab twice to enable." Setzen Sie diesen Schlüssel auf `false`, um sie zu verbergen. Erscheint in `/config` als **Tipps anzeigen**.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: Sie sehen Tipps im Spinner, während Claude arbeitet
  * `false`: Claude Code verbirgt Spinner-Tipps
* **Standard**: `true`

```json settings.json theme={null}
{
  "spinnerTipsEnabled": false
}
```

<h3 id="spinnertipsoverride">
  `spinnerTipsOverride`
</h3>

Fügen Sie Ihre eigenen Tipps zu den [Spinner-Tipps](#spinnertipsenabled) hinzu, die Claude Code zeigt, während Claude arbeitet, oder ersetzen Sie die integrierten Tipps durch Ihre. Claude Code setzt Ihre Tipps in die gleiche Rotation wie die integrierten: Es wählt den Tipp, der am längsten nicht angezeigt wurde, überspringt Tipps, die sich noch in ihrer Abklingzeit befinden, und bricht Unentschieden nach Priorität auf.

Wenn Sie [`spinnerTipsEnabled`](#spinnertipsenabled) auf `false` setzen, verbirgt Claude Code alle Tipps, einschließlich Ihrer.

* **Bereich**: [`Beliebige Datei`](#scopes). Claude Code berücksichtigt Tipp-Objekte, `tipsFile`, `label` und `excludeDefault` aus Benutzereinstellungen, dem Flag `--settings` und verwalteten Einstellungen; aus Projekt- und lokalen Einstellungen liest es nur einfache String-Tipps.
* **Typ**: Objekt mit `tips`, `tipsFile`, `label` und `excludeDefault`-Feldern, jeweils optional
* **Standard**: nicht gesetzt, daher zeigt Claude Code nur die integrierten Tipps

Tipp-Objekte, `tipsFile`, `label` und die Regel in der Bereich-Zeile, dass Projekt- und lokale Einstellungen nur einfache Strings beitragen, erfordern Claude Code v2.1.247 oder später. In früheren Versionen gilt auch `excludeDefault` einer Projekt- oder lokalen Datei.

Jeder `tips`-Eintrag ist ein einfacher String oder ein Objekt mit diesen Feldern:

| Feld               | Erforderlich | Beschreibung                                                                                                                                                                                                                                                 |
| :----------------- | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`               | Ja           | Bis zu 64 Buchstaben, Ziffern, `.`, `_` oder `-`. Claude Code schlüsselt die Show-Historie des Tipps darauf auf, daher überlebt die Abklingzeit des Tipps eine Neuordnung der Liste. Von zwei Einträgen mit der gleichen ID verwendet Claude Code den ersten |
| `text`             | Ja           | Der Tipp, eine Zeile mit bis zu 500 Zeichen. Claude Code entfernt ANSI-Escapes und Steuerzeichen und reduziert Leerzeichen                                                                                                                                   |
| `cooldownSessions` | Nein         | Sitzungen, die Claude Code wartet, bevor der Tipp erneut angezeigt wird, `0` bis `1000`, Standard `0`                                                                                                                                                        |
| `priority`         | Nein         | Reihenfolge unter Tipps, die gleich lange nicht angezeigt wurden, höher zuerst, `-10` bis `10`, Standard `0`                                                                                                                                                 |

Claude Code liest einen einfachen String als Tipp mit diesen Standards und einer positionsbasierten ID, daher setzt sich seine Show-Historie zurück, wenn Sie die Liste neu ordnen. Geben Sie einem Tipp eine `id`, um seine Historie über Bearbeitungen hinweg zu behalten.

Claude Code liest höchstens 200 Tipps über `tips` und `tipsFile` und verwirft einen ungültigen Eintrag mit einer Debug-Warnung, anstatt die Einstellungsdatei abzulehnen.

Verwenden Sie die verbleibenden Felder, um eine Tipps-Datei zu benennen, das Präfix zu setzen und die integrierten Tipps zu verbergen:

* `tipsFile`: ein absoluter oder `~/`-Pfad zu einer lokalen JSON-Datei mit einem Array der gleichen Einträge oder ein Objekt mit einem `tips`-Array, bis zu 256 KB. Claude Code liest die Datei einmal pro Prozess, daher lädt es Ihre Bearbeitungen beim nächsten Start. Sie können es nicht durch [server-verwaltete Einstellungen](/docs/de/server-managed-settings) setzen; stellen Sie inline `tips` dort bereit oder stellen Sie den Pfad in einer auf der Festplatte gespeicherten `managed-settings.json` bereit.
* `label`: das Präfix, das Claude Code vor Tipps aus Benutzer-, `--settings`- und verwalteten Einstellungen anzeigt, bis zu 40 Zeichen. Der Standard ist `Tip`, das gleiche Präfix wie die integrierten Tipps, und Tipps aus Projekt- und lokalen Einstellungen verwenden es immer.
* `excludeDefault`: setzen Sie es auf `true`, um die integrierten Tipps zu verbergen und nur Ihre anzuzeigen. Wenn Claude Code keine Ihrer Tipps laden kann, z. B. weil `tipsFile` nicht existiert oder jeder Eintrag ungültig ist, behält es die integrierte Rotation anstelle einer leeren Spinner bei.

Wenn mehr als eine Einstellungsdatei den Schlüssel setzt, zeigt Claude Code Tipps aus allen und nimmt `tipsFile`, `label` und `excludeDefault` von welcher der verwalteten Einstellungen, dem Flag `--settings` und den Benutzereinstellungen die höchste Priorität hat, die jeweils setzt.

Dieses Beispiel in Ihren Benutzereinstellungen fügt einen einfachen String-Tipp und einen Objekt-Tipp zur Rotation unter dem Präfix `Acme tip` hinzu:

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

Jedes Feld im Beispiel ändert eine Sache daran, wie Claude Code die Tipps anzeigt:

* `label`: Claude Code zeigt beide Tipps als `Acme tip: ...` anstelle von `Tip: ...`.
* Der einfache String: Claude Code gibt ihm die Standards, daher kann er in der nächsten Sitzung erneut auftauchen.
* `id`: Claude Code schlüsselt die Show-Historie des zweiten Tipps auf `gateway-errors` auf, daher gilt seine Abklingzeit immer noch nach dem Hinzufügen oder Neuordnen von Tipps.
* `cooldownSessions`: Nachdem Claude Code den `gateway-errors`-Tipp angezeigt hat, zeigt es diesen Tipp nicht erneut an, bis fünf Sitzungen später.
* `priority`: Wenn der `gateway-errors`-Tipp und ein anderer Tipp gleich lange nicht angezeigt wurden, z. B. wenn keiner noch angezeigt wurde, zeigt Claude Code `gateway-errors` zuerst. Der einfache String hat die Standard-Priorität, `0`.

Während Claude arbeitet, zeigt Claude Code Ihre Tipps im Spinner mit Ihrem Präfix an, wie z. B. `Acme tip: Run /review before opening a PR`.

<h3 id="spinnerverbs">
  `spinnerVerbs`
</h3>

Während eine Runde läuft, zeigt der Spinner ein rotierendes Verb wie „Accomplishing", „Architecting" oder „Baking". Verwenden Sie diesen Schlüssel, um Ihre eigenen Verben zu dieser Rotation hinzuzufügen oder die integrierte Liste durch Ihre zu ersetzen.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Objekt mit einem `verbs`-Array von Strings und `mode`, einer von:
  * `"append"`: Claude Code fügt Ihre Verben zum integrierten Satz hinzu
  * `"replace"`: Claude Code zeigt nur Ihre Verben
* **Standard**: nicht gesetzt, daher verwendet Claude Code die integrierten Verben

Dieses Beispiel fügt zwei Verben zum integrierten Satz hinzu:

```json settings.json theme={null}
{
  "spinnerVerbs": {
    "mode": "append",
    "verbs": ["Pondering", "Crafting"]
  }
}
```

Im `"replace"`-Modus mit einem leeren `verbs`-Array behält Claude Code die integrierten Verben.

<h3 id="statusline">
  `statusLine`
</h3>

Führen Sie Ihren eigenen Befehl aus, um eine [Statuszeile](/docs/de/statusline) unter der Eingabeaufforderung mit Kontext wie dem Modell, den Kosten oder dem Git-Branch zu rendern. Optionale Felder passen den Abstand an, fügen periodische Neuausführungen hinzu und verbergen den integrierten Vim-Modus-Indikator, wenn Ihr Skript `vim.mode` selbst rendert.

* **Bereich**: [`Beliebige Datei`](#scopes). Wenn [`allowManagedHooksOnly`](#allowmanagedhooksonly) aktiviert ist oder [`disableAllHooks`](#disableallhooks) außerhalb verwalteter Einstellungen gesetzt ist, wird nur der verwaltete Einstellungswert ausgeführt.
* **Typ**: Objekt mit `type` auf `"command"` gesetzt und einem `command`-String, plus optionales `padding` als Anzahl von Zeichen, `refreshInterval` als Anzahl von Sekunden, Minimum `1`, und `hideVimModeIndicator` als Boolean
* **Standard**: nicht gesetzt, daher keine Statuszeile

Dieses Beispiel druckt den Modellnamen und die Kontextnutzung und fügt zwei Zeichen horizontalen Abstand hinzu:

```json settings.json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
    "padding": 2
  }
}
```

Das Beispiel benötigt [`jq`](https://jqlang.org/) installiert und läuft in einer Shell. Für PowerShell- und Git Bash-Äquivalente siehe [Windows-Konfiguration](/docs/de/statusline#windows-configuration); für die vollständige Einrichtung siehe [Statuszeile manuell konfigurieren](/docs/de/statusline#manually-configure-a-status-line).

<h3 id="subagentstatusline">
  `subagentStatusLine`
</h3>

Wenn Claude [Subagenten](/docs/de/sub-agents) ausführt, listet Claude Code sie in einer Task-Anzeige unter der Eingabeaufforderung auf, eine Zeile pro Subagent mit `name · description · token count`. Dieser Schlüssel lässt Sie Ihren eigenen Befehl ausführen, um diese Zeilen umzuschreiben, z. B. um die Kontextnutzung jedes Subagenten als Prozentsatz anzuzeigen. Bei jeder Aktualisierung sendet Claude Code die sichtbaren Zeilen als ein JSON-Objekt auf stdin mit einem `tasks`-Array, das die `id`, `name`, `status`, `model`, `tokenCount` und mehr jedes Subagenten trägt, und ersetzt die Zeile für jede `id`, die Sie als `{"id", "content"}`-Zeile zurückschreiben. Zeilen, die Sie nicht zurückschreiben, behalten das Standard-Rendering.

* **Bereich**: [`Beliebige Datei`](#scopes). Wenn [`allowManagedHooksOnly`](#allowmanagedhooksonly) aktiviert ist oder [`disableAllHooks`](#disableallhooks) außerhalb verwalteter Einstellungen gesetzt ist, wird nur der verwaltete Einstellungswert ausgeführt.
* **Typ**: Objekt mit `type` auf `"command"` gesetzt und einem `command`-String
* **Standard**: nicht gesetzt, daher rendert Claude Code die Standard-Zeilen

```json settings.json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "jq -c '.tasks[] | {id, content: \"\\(.name): \\(.tokenCount) tokens\"}'"
  }
}
```

Siehe [Subagent-Statuszeilen](/docs/de/statusline#subagent-status-lines).

<h3 id="syntaxhighlightingdisabled">
  `syntaxHighlightingDisabled`
</h3>

Claude Code färbt Code nach Sprache in den Diffs, Code-Blöcken und Datei-Vorschauen, die es im Terminal anzeigt, mit seinem integrierten Highlighter; kein Plugin oder Language Server ist beteiligt. Setzen Sie diesen Schlüssel auf `true`, um sie stattdessen als einfachen Text anzuzeigen, z. B. wenn die Farben mit Ihrem Terminal-Design kollidieren oder einen Bildschirmleser verlangsamen.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code schaltet die Syntaxhervorhebung in Diffs, Code-Blöcken und Datei-Vorschauen aus
  * `false`: Claude Code hebt Syntax hervor
* **Standard**: `false`

```json settings.json theme={null}
{
  "syntaxHighlightingDisabled": true
}
```

<h3 id="terminalprogressbarenabled">
  `terminalProgressBarEnabled`
</h3>

Einige Terminals können einen Fortschrittsindikator auf der Registerkarte oder in der Taskleiste für das darin laufende Programm anzeigen. Während Claude arbeitet, meldet Claude Code einen laufenden Status dem Terminal, damit Sie von einer anderen Registerkarte oder einem anderen Fenster aus sehen können, ob die Sitzung noch beschäftigt ist. Der Indikator bleibt sichtbar, nachdem die Runde endet, während [Hintergrund-Subagenten](/docs/de/sub-agents#run-subagents-in-foreground-or-background) oder [dynamische Workflows](/docs/de/workflows) noch laufen, und wird gelöscht, sobald die Sitzung untätig ist.

Claude Code meldet es nur in Terminals, die den Indikator unterstützen: ConEmu, Ghostty 1.2.0 oder später und iTerm2 3.6.6 oder später. Setzen Sie diesen Schlüssel auf `false`, um Claude Code davon abzuhalten, es zu melden. Erscheint in `/config` als **Terminal-Fortschrittsbalken**.

* **Bereich**: [`Beliebige Datei`](#scopes). Ein Wert in `~/.claude.json` aus einer älteren Version gilt, wenn keine Einstellungsdatei es setzt.
* **Typ**: Boolean
  * `true`: Sie sehen den Terminal-Fortschrittsbalken in Terminals, die ihn unterstützen
  * `false`: Claude Code verbirgt den Terminal-Fortschrittsbalken
* **Standard**: `true`

```json settings.json theme={null}
{
  "terminalProgressBarEnabled": false
}
```

<h3 id="terminaltitlefromrename">
  `terminalTitleFromRename`
</h3>

Claude Code setzt den Titel Ihrer Terminal-Registerkarte. Standardmäßig verwendet es einen Titel, den es aus dem Gespräch generiert, und sobald Sie der Sitzung einen [Namen](/docs/de/sessions#name-your-sessions) mit `/rename` oder `--name` geben, zeigt die Registerkarte stattdessen diesen Namen. Setzen Sie diesen Schlüssel auf `false`, um den generierten Titel auf der Registerkarte zu behalten, auch nachdem Sie die Sitzung benannt haben. Der Name selbst gilt immer noch, daher finden `/resume <name>` und der Sitzungs-Picker ihn.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: Der Terminal-Registerkartentitel zeigt den Sitzungsnamen, den Sie setzen
  * `false`: Die Registerkarte behält den Titel, den Claude Code aus Ihrem Gespräch generiert
* **Standard**: `true`

```json settings.json theme={null}
{
  "terminalTitleFromRename": false
}
```

Um Claude Code davon abzuhalten, den Terminal-Titel überhaupt zu aktualisieren, setzen Sie stattdessen [`CLAUDE_CODE_DISABLE_TERMINAL_TITLE`](/docs/de/env-vars) auf `1`.

<h3 id="theme">
  `theme`
</h3>

Wählen Sie das Farbdesign für die Schnittstelle. Erscheint in `/config` als **Design**.

* **Bereich**: [`Beliebige Datei`](#scopes). Ein Wert in `~/.claude.json` aus einer älteren Version gilt, wenn keine Einstellungsdatei es setzt.
* **Typ**: String, einer von:
  * `"auto"`: passt sich dem hellen oder dunklen Hintergrund Ihres Terminals an
  * `"dark"`: das dunkle Design
  * `"light"`: das helle Design
  * `"dark-daltonized"`: das dunkle Design mit farbenblind-freundlichen Farben
  * `"light-daltonized"`: das helle Design mit farbenblind-freundlichen Farben
  * `"dark-ansi"`: das dunkle Design mit nur Ihrer Terminal-ANSI-Farbpalette
  * `"light-ansi"`: das helle Design mit nur Ihrer Terminal-ANSI-Farbpalette
  * `"custom:<slug>"` oder `"custom:<plugin-name>:<slug>"`: ein benutzerdefiniertes Design aus `~/.claude/themes/` oder einem Plugin
* **Standard**: `"dark"`

```json settings.json theme={null}
{
  "theme": "light-daltonized"
}
```

Siehe [Benutzerdefiniertes Design erstellen](/docs/de/terminal-config#create-a-custom-theme).

<h3 id="timeformat">
  `timeFormat`
</h3>

Wählen Sie, wie Claude Code die Zeiten schreibt, die es in der Schnittstelle anzeigt, wie z. B. die `done 6:05 PM` am Ende jeder Rundendauer-Nachricht und die Zeitstempel im [Transkript-Viewer](/docs/de/interactive-mode#transcript-viewer). Um eine Voreinstellung zu wählen, führen Sie `/config` aus und setzen Sie **Zeitformat**. Erfordert Claude Code v2.1.257 oder später.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: String, einer von:
  * `"auto"`: das gleiche wie nicht gesetzt; jede Zeit behält ihr integriertes Format, das Ihrem Gebietsschema in der Rundendauer-Nachricht folgt
  * `"12-hour"`: eine 12-Stunden-Uhr
  * `"24-hour"`: eine 24-Stunden-Uhr
  * `"24-hour-utc"`: eine 24-Stunden-Uhr in UTC mit `Z` nach den Minuten, wie z. B. `18:05Z`; Claude Code ignoriert [`timeZone`](#timezone) für diese Voreinstellung
  * Ein strftime-Muster wie `"%H:%M"`: Claude Code schreibt jede Zeit mit dem Muster. Jeder Wert, der ein `%` enthält, ist ein Muster, und jeder andere Wert außerhalb der Voreinstellungen zählt als `"auto"`
* **Standard**: `"auto"`

```json settings.json theme={null}
{
  "timeFormat": "24-hour"
}
```

`/config` bietet nur die Voreinstellungen, daher müssen Sie zum Verwenden eines strftime-Musters den Schlüssel zu einer Einstellungsdatei hinzufügen. Dieses Beispiel zeigt jede Zeit als eine zweistellige 24-Stunden-Uhr:

```json settings.json theme={null}
{
  "timeFormat": "%H:%M"
}
```

Die Rundendauer-Nachricht und der Transkript-Viewer zeigen dann Zeiten wie `18:05`. Im Transkript-Viewer ist das Muster der gesamte Zeitstempel, daher fügen Sie Datums-Direktiven hinzu, wenn Sie das Datum dort haben möchten. Dieses Beispiel setzt das Datum vor die Uhr:

```json settings.json theme={null}
{
  "timeFormat": "%Y-%m-%d %H:%M"
}
```

Die gleichen Oberflächen zeigen dann Zeiten wie `2026-09-01 18:05`.

<h3 id="timezone">
  `timeZone`
</h3>

Zeigen Sie die Zeiten in der Schnittstelle in einer anderen Zeitzone als Ihrer Systemzeit an. Setzen Sie es auf einen [IANA-Zeitzonennamen](https://www.iana.org/time-zones), wie z. B. `"UTC"` oder `"Europe/Dublin"`. Die Zeiten, die [`timeFormat`](#timeformat) kontrolliert, werden dann in dieser Zone angezeigt. Wenn `timeFormat` `"24-hour-utc"` ist, bleiben Zeiten in UTC und Claude Code ignoriert diesen Schlüssel. `/config` hat keine Zeile für diesen Schlüssel, daher setzen Sie ihn in einer Einstellungsdatei. Erfordert Claude Code v2.1.257 oder später.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: String, ein IANA-Zeitzonennamen. Wenn Claude Code den Namen nicht erkennt, verwendet es Ihre Systemzeitzone
* **Standard**: nicht gesetzt, daher zeigen Zeiten Ihre Systemzeitzone

```json settings.json theme={null}
{
  "timeZone": "Europe/Dublin"
}
```

<h3 id="tui">
  `tui`
</h3>

Wählen Sie den Terminal-UI-Renderer. Verwenden Sie `"fullscreen"` für den flimmerfreien [Alt-Screen-Renderer](/docs/de/fullscreen) mit virtualisiertem Scrollback oder `"default"` für den klassischen Main-Screen-Renderer. Das Ausführen von `/tui fullscreen` oder `/tui default` schreibt diesen Schlüssel für Sie.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: String, einer von:
  * `"default"`: der klassische Main-Screen-Renderer
  * `"fullscreen"`: der flimmerfreie Alt-Screen-Renderer mit virtualisiertem Scrollback
* **Standard**: nicht gesetzt, daher [wählt Claude Code den Renderer für Sie](/docs/de/fullscreen#fullscreen-by-default)
* **Sitzungsübergreifende Außerkraftsetzungen**: [`CLAUDE_CODE_NO_FLICKER`](/docs/de/env-vars) und [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN`](/docs/de/env-vars) haben Vorrang vor diesem Schlüssel für eine Sitzung: `CLAUDE_CODE_NO_FLICKER=1` schaltet Vollbild ein, und `CLAUDE_CODE_NO_FLICKER=0` oder `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1` schaltet es aus; wenn beide gesetzt sind, schaltet Claude Code es aus

```json settings.json theme={null}
{
  "tui": "fullscreen"
}
```

Unter tmux `-CC` oder über SSH zu Windows behält Claude Code den klassischen Renderer, es sei denn, Sie setzen `CLAUDE_CODE_NO_FLICKER=1`. Hintergrundsitzungen, die aus der [Agent-Ansicht](/docs/de/agent-view) geöffnet werden, verwenden immer den Vollbild-Renderer, unabhängig von dieser Einstellung.

<h3 id="verbose">
  `verbose`
</h3>

Standardmäßig reduziert das Transkript jeden Tool-Aufruf auf eine kurze Zusammenfassung, wie z. B. den Befehl, den Claude ausgeführt hat, und eine Zeilenanzahl seiner Ausgabe, und Sie drücken `Ctrl+O`, um das gesamte Transkript zur erweiterten Ansicht zu wechseln, wenn Sie die Details möchten. Setzen Sie diesen Schlüssel auf `true`, um die vollständige Eingabe und Ausgabe jedes Tool-Aufrufs inline anzuzeigen, während es passiert, was nützlich ist, wenn Sie einen Hook, einen MCP-Server oder einen langen Shell-Befehl debuggen. Erscheint in `/config` als **Ausführliche Ausgabe**.

* **Bereich**: [`Beliebige Datei`](#scopes). Ein Wert in `~/.claude.json` aus einer älteren Version gilt, wenn keine Einstellungsdatei es setzt.
* **Typ**: Boolean
  * `true`: Sie sehen vollständige Tool-Ausgabe
  * `false`: Sie sehen gekürzte Zusammenfassungen der Tool-Ausgabe
* **Standard**: `false`
* **Sitzungsübergreifende Außerkraftsetzungen**: [`--verbose`](/docs/de/cli-reference#cli-flags) hat Vorrang vor diesem Schlüssel für eine Sitzung

```json settings.json theme={null}
{
  "verbose": true
}
```

Ein [`viewMode`](#viewmode)-Wert oder eine klebrige `/focus`-Auswahl überschreibt diesen Schlüssel jede Sitzung.

<h3 id="viewmode">
  `viewMode`
</h3>

Legen Sie die Transkript-Ansicht fest, in der Claude Code startet: `"default"`, `"verbose"` oder `"focus"`. Wenn gesetzt, überschreibt es sowohl die klebrige `/focus`-Auswahl als auch die Einstellung [`verbose`](#verbose).

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: String, einer von:
  * `"default"`: das normale Transkript mit gekürzte Tool-Ausgabe
  * `"verbose"`: das Transkript mit vollständiger Tool-Ausgabe
  * `"focus"`: nur Ihre letzte Eingabeaufforderung, eine einzeilige Zusammenfassung von Tool-Aufrufen mit Edit-Diffstats und die endgültige Antwort. Focus-Ansicht benötigt den [Vollbild-Renderer](#tui)
* **Standard**: nicht gesetzt, daher gelten die Einstellung `verbose` und Ihre letzte `/focus`-Auswahl
* **Sitzungsübergreifende Außerkraftsetzungen**: [`--verbose`](/docs/de/cli-reference#cli-flags) hat Vorrang vor diesem Schlüssel für eine Sitzung

```json settings.json theme={null}
{
  "viewMode": "focus"
}
```

<h3 id="viminsertmoderemaps">
  `vimInsertModeRemaps`
</h3>

Ordnen Sie zwei-Tasten-INSERT-Modus-Sequenzen Escape im [Vim-Editor-Modus](/docs/de/interactive-mode#vim-editor-mode) zu. Jeder Schlüssel ist genau zwei druckbare Zeichen, die nacheinander eingegeben werden, und `"<Esc>"` ist das einzige unterstützte Ziel; Claude Code ignoriert andere Einträge. Erfordert Claude Code v2.1.208 oder später.

* **Bereich**: [`Benutzer oder verwaltet`](#scopes). Ein Repository kann Ihre Tastenanschläge nicht neu zuordnen.
* **Typ**: Objekt, das eine zwei-Zeichen-Sequenz auf `"<Esc>"` abbildet
* **Standard**: nicht gesetzt

```json settings.json theme={null}
{
  "vimInsertModeRemaps": {
    "jj": "<Esc>"
  }
}
```

Hat keine Auswirkung, es sei denn, `editorMode` ist `"vim"`. Siehe [INSERT-Modus-Tastenseqenzen neu zuordnen](/docs/de/interactive-mode#remap-insert-mode-key-sequences). Erfordert Claude Code v2.1.208 oder später.

<h3 id="voice">
  `voice`
</h3>

Schalten Sie [Sprachdiktat](/docs/de/voice-dictation) ein und wählen Sie, wie die Diktat-Taste sich verhält. Claude Code schreibt dieses Objekt für Sie, wenn Sie `/voice` ausführen.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Objekt mit `enabled` als Boolean, `autoSubmit` als Boolean, das nur im Hold-Modus gilt, und `mode`, einer von:
  * `"hold"`: Sie halten die Diktat-Taste gedrückt, während Sie sprechen, und lassen sie los, um zu stoppen
  * `"tap"`: Sie tippen die Taste einmal an, um die Aufzeichnung zu starten, und erneut, um zu senden
* **Standard**: nicht gesetzt, daher ist Diktat aus; wenn `enabled` `true` ist und `mode` nicht gesetzt ist, verwendet Claude Code `"hold"`

Dieses Beispiel schaltet Diktat ein und macht die Taste zu einem Tippen, um die Aufzeichnung zu starten, und erneut zu senden:

```json settings.json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

`autoSubmit` sendet die Eingabeaufforderung, wenn Sie die Taste im Hold-Modus loslassen. Sprachdiktat erfordert ein claude.ai-Konto.

<h3 id="voiceenabled">
  `voiceEnabled`
</h3>

<Warning>
  Veraltet seit v2.1.92, als das Objekt [`voice`](#voice) es ersetzte. Claude Code liest es immer noch, daher funktionieren ältere Einstellungsdateien weiterhin, aber neue Konfigurationen sollten `voice.enabled` setzen.
</Warning>

Schalten Sie Sprachdiktat mit der einzelnen Boolean-Form ein, die dem Objekt `voice` vorausgeht. Wenn beide gesetzt sind, gilt `voice.enabled`.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: Sprachdiktat ist an, wenn Sie mit einem claude.ai-Konto angemeldet sind und die Richtlinie Ihrer Organisation Sprache erlaubt, es sei denn, `voice.enabled` ist gesetzt
  * `false`: Sprachdiktat ist aus, es sei denn, `voice.enabled` ist gesetzt
* **Standard**: nicht gesetzt

```json settings.json theme={null}
{
  "voiceEnabled": true
}
```

<h3 id="wheelscrollaccelerationenabled">
  `wheelScrollAccelerationEnabled`
</h3>

Beschleunigen Sie die Mausrad-Scroll-Geschwindigkeit während schneller Scrolls in [Vollbilddarstellung](/docs/de/fullscreen#mouse-wheel-scrolling). Setzen Sie es auf `false` für eine konstante Scroll-Rate pro Rad-Kerbe.

* **Bereich**: [`Beliebige Datei`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code beschleunigt die Mausrad-Scroll-Geschwindigkeit während schneller Scrolls
  * `false`: Claude Code scrollt mit einer konstanten Rate pro Rad-Kerbe
* **Standard**: `true`

```json settings.json theme={null}
{
  "wheelScrollAccelerationEnabled": false
}
```

<h2 id="git-and-attribution">
  Git und Zuordnung
</h2>

Steuern Sie die Zuordnung, die Claude Code zu Commits und Pull Requests hinzufügt, und wie sie mit Git funktioniert.

<span id="attribution-settings" />

<h3 id="attribution">
  `attribution`
</h3>

Passen Sie die Zuordnung an, die Claude Code zu Git-Commits und Pull Requests hinzufügt. Commits erhalten standardmäßig einen [Git-Trailer](https://git-scm.com/docs/git-interpret-trailers) wie `Co-Authored-By`; Pull-Request-Beschreibungen erhalten Klartext. Legen Sie jeden Teil separat mit den folgenden Unterschlüsseln fest.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Objekt mit `commit`- und `pr`-Zeichenketten und einem `sessionUrl`-Boolean, oder `false`, um alle Zuordnungen auszublenden. Der Wert `false` erfordert Claude Code v2.1.281 oder später; frühere Versionen lehnen ihn ab und [überspringen die gesamte Benutzer-, Projekt- oder lokale Einstellungsdatei](/docs/de/settings#fix-a-broken-settings-file), die ihn enthält
* **Standard**: nicht gesetzt, daher verwendet Claude Code die unter jedem Unterschlüssel angezeigte Standard-Zuordnung

Um alle Zuordnungen auszublenden, setzen Sie `attribution` auf `false`. In einer Einstellungsdatei, die auch frühere Versionen lesen, setzen Sie stattdessen [`commit`](#attribution-commit) und [`pr`](#attribution-pr) auf leere Zeichenketten und [`sessionUrl`](#attribution-sessionurl) auf `false`.

Dieses Beispiel ersetzt die Commit-Zuordnung, entfernt die Pull-Request-Zuordnung und löscht den Sitzungslink:

```json settings.json theme={null}
{
  "attribution": {
    "commit": "Generated with AI\n\nCo-Authored-By: AI <ai@example.com>",
    "pr": "",
    "sessionUrl": false
  }
}
```

Sobald Sie `commit` oder `pr` setzen, ignoriert Claude Code die veraltete Einstellung `includeCoAuthoredBy` und verwendet seinen Standard-Text für denjenigen der beiden, den Sie nicht gesetzt haben.

Claude Code teilt Claude mit, dass Ihre eigenen Anweisungen zur Zuordnung, wie eine CLAUDE.md oder [Memory](/docs/de/memory)-Regel, Vorrang vor diesen Commit- und PR-Zeilen haben, es sei denn, die Zeile ist in [verwalteten Einstellungen](/docs/de/managed-settings) gesetzt.

<h3 id="includecoauthoredby">
  `includeCoAuthoredBy`
</h3>

<Warning>
  Veraltet seit v2.0.62, als [`attribution`](#attribution) es ersetzte. Claude Code liest es immer noch, aber neue Konfigurationen sollten `attribution` setzen.
</Warning>

Verwenden Sie stattdessen [`attribution`](#attribution), das diesen Schlüssel ersetzt und es Ihnen ermöglicht, den Commit-Trailer, den Pull-Request-Text und den Sitzungslink separat zu ändern oder auszublenden. Claude Code respektiert immer noch `includeCoAuthoredBy: false` aus Einstellungsdateien, die `attribution` vorausgehen, ignoriert es aber, sobald Sie `attribution.commit` oder `attribution.pr` setzen.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolean
  * `true`: dasselbe wie nicht gesetzt; Claude Code fügt den Commit-Trailer und den Pull-Request-Zuordnungstext hinzu
  * `false`: Claude Code lässt sowohl den Commit-Trailer als auch den Pull-Request-Zuordnungstext weg, es sei denn, `attribution` setzt `commit` oder `pr`, in welchem Fall die [`attribution`](#attribution)-Regeln gelten
* **Standard**: `true`

```json settings.json theme={null}
{
  "includeCoAuthoredBy": false
}
```

Um alle Zuordnungen auszublenden, siehe [`attribution`](#attribution).

<h3 id="includegitinstructions">
  `includeGitInstructions`
</h3>

Claude Code gibt Claude zwei Git-bezogene Kontextteile: seine integrierten Anweisungen zum Schreiben von Commits und Pull Requests in der Bash-Tool-Beschreibung und einen Git-Status-Snapshot Ihres Repositorys. Der Snapshot enthält den aktuellen Branch, den Haupt-Branch, die Ausgabe von `git status` und aktuelle Commits. Claude Code liest ihn, wenn eine Konversation beginnt.

Setzen Sie diesen Schlüssel auf `false`, um beide auszulassen, zum Beispiel wenn Sie Ihre eigenen Git-Workflow-Skills verwenden.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code enthält seine integrierten Commit- und Pull-Request-Workflow-Anweisungen und den Git-Status-Snapshot. Cloud-Sitzungen enthalten niemals den Snapshot
  * `false`: Claude Code lässt beide aus
* **Standard**: `true`
* **Sitzungsspezifische Überschreibungen**: [`CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS`](/docs/de/env-vars) hat Vorrang vor diesem Schlüssel für eine Sitzung

```json settings.json theme={null}
{
  "includeGitInstructions": false
}
```

<h3 id="prurltemplate">
  `prUrlTemplate`
</h3>

Richten Sie die PR-Links, die Claude Code in der Fußzeilenbadge und in Tool-Ergebnis-Zusammenfassungen rendert, auf ein internes Code-Review-Tool statt auf `github.com` aus. Claude Code ersetzt `{host}`, `{owner}`, `{repo}`, `{number}` und `{url}` aus der PR-URL. [GitLab-Merge-Request](/docs/de/interactive-mode#gitlab-merge-requests)-Links auf beiden Oberflächen behalten ihre GitLab-URL.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Zeichenkette, eine URL-Vorlage mit einem der fünf Platzhalter
* **Standard**: nicht gesetzt

```json settings.json theme={null}
{
  "prUrlTemplate": "https://reviews.example.com/{owner}/{repo}/pull/{number}"
}
```

Claude Code wendet die Vorlage nur auf die Links an, die es selbst rendert; eine PR-Nummer, die Claude in einer Nachricht schreibt, wie `#123`, bleibt so, wie Claude sie geschrieben hat. Eine URL, die nicht die Form `/pull/<number>` hat, wird unverändert gelassen.

<h3 id="attribution-commit">
  `attribution.commit`
</h3>

Legen Sie den Zuordnungstext fest, den Claude Code zu Git-Commits hinzufügt, einschließlich aller Trailer. Setzen Sie ihn auf eine leere Zeichenkette, um die Commit-Zuordnung auszublenden.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Zeichenkette
* **Standard**: nicht gesetzt, daher fügt Claude Code `Co-Authored-By: <name> <noreply@anthropic.com>` hinzu. Der Name ist das aktive Modell der Sitzung, wie `Claude Sonnet 5`.
  * Wenn Claude Code das Modell als Claude-Modell erkennt, aber seine genaue Version nicht bestätigen kann, schreibt es nur `Claude`.
  * Wenn es die Modell-ID nicht mit einem Claude-Modell abgleichen kann, wie ein Drittanbieter-Modell, das über eine benutzerdefinierte [`ANTHROPIC_BASE_URL`](/docs/de/env-vars) bereitgestellt wird, schreibt es `Claude Code`.

Dieses Beispiel ersetzt den Standard-Trailer durch eine benutzerdefinierte Zeile und einen benutzerdefinierten `Co-Authored-By`-Trailer:

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

Legen Sie den Zuordnungstext fest, den Claude Code zu Pull-Request-Beschreibungen hinzufügt. Setzen Sie ihn auf eine leere Zeichenkette, um die Pull-Request-Zuordnung auszublenden.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Zeichenkette
* **Standard**: nicht gesetzt, daher fügt Claude Code `🤖 Generated with [Claude Code](https://claude.com/claude-code)` hinzu

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

Wählen Sie, ob Claude Code den claude.ai-Sitzungslink anhängt, wenn es von einer [Cloud](/docs/de/claude-code-on-the-web)- oder [Remote Control](/docs/de/remote-control)-Sitzung aus committed oder einen Pull Request öffnet. Claude Code fügt den Link als `Claude-Session`-Trailer bei Commits und als Link in Pull-Request-Beschreibungen hinzu. Setzen Sie ihn auf `false`, um den Link auszulassen.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code hängt den claude.ai-Sitzungslink an, wenn es von einer Cloud- oder Remote-Control-Sitzung aus committed oder einen Pull Request öffnet
  * `false`: Claude Code lässt den Link aus
* **Standard**: `true`

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
  Hooks und Automatisierung
</h2>

Registrieren Sie Hooks, beschränken Sie, welche Hooks ausgeführt werden, und kontrollieren Sie Workflows. Für Hook-Ereignisse und Payloads siehe die [Hooks-Referenz](/docs/de/hooks).

<h3 id="allowedhttphookurls">
  `allowedHttpHookUrls`
</h3>

Begrenzen Sie, welche URLs [HTTP-Hooks](/docs/de/hooks#http-hook-fields) ansteuern können. Wenn Sie diesen Schlüssel definieren, führt Claude Code einen HTTP-Hook nur aus, wenn seine URL einem der Muster entspricht, und blockiert die übrigen ohne Ausführung; ein leeres Array blockiert jeden HTTP-Hook.

* **Bereich**: [`Any file`](#scopes). Arrays werden über Einstellungsdateien hinweg zusammengeführt.
* **Typ**: Array von URL-Mustern mit `*` als Platzhalter
* **Standard**: nicht gesetzt, daher ist jede URL zulässig

Dieses Beispiel erlaubt jede URL unter `https://hooks.example.com/` und jede `http://localhost`-URL:

```json settings.json theme={null}
{
  "allowedHttpHookUrls": ["https://hooks.example.com/*", "http://localhost:*"]
}
```

Der Hostname-Abgleich ist nicht case-sensitiv und behandelt `hooks.example.com.` mit dem nachgestellten Punkt, der einen vollständig qualifizierten Domänennamen kennzeichnet, genauso wie `hooks.example.com`, wie DNS es behandelt. Die Zulassungsliste gilt für Hooks aus jeder Quelle, einschließlich verwalteter Einstellungen.

<h3 id="allowmanagedhooksonly">
  `allowManagedHooksOnly`
</h3>

Beschränken Sie die Hook-Ausführung auf Hooks, die Ihre Organisation bereitstellt.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Boolean
  * `true`: nur verwaltete Hooks werden ausgeführt, plus Agent SDK Hooks und Hooks aus Plugins, die Ihre verwalteten Einstellungen erzwingen. Siehe [Was wird unter `allowManagedHooksOnly` ausgeführt](#what-runs-under-allowmanagedhooksonly)
  * `false`: Hooks aus jedem Einstellungsbereich und Plugin werden ausgeführt
* **Standard**: nicht gesetzt, daher werden Hooks aus jedem Einstellungsbereich und Plugin ausgeführt

```json managed-settings.json theme={null}
{
  "allowManagedHooksOnly": true
}
```

<h4 id="what-runs-under-allowmanagedhooksonly">
  Was wird unter `allowManagedHooksOnly` ausgeführt
</h4>

Wenn Sie es auf `true` setzen, ändert Claude Code, welche Hooks und Hook-ähnliche Befehle geladen werden:

* **Verwaltete und SDK-Hooks werden ausgeführt**: Hooks aus verwalteten Einstellungen und Hooks, die das [Agent SDK](/docs/de/agent-sdk/overview) im Prozess registriert
* **Erzwungene Plugin-Hooks werden ausgeführt**: Hooks aus Plugins, die Ihre verwalteten Einstellungen durch [`enabledPlugins`](#enabledplugins) erzwingen. Claude Code gleicht die vollständige `plugin@marketplace`-ID ab, daher bleibt ein Plugin mit demselben Namen aus einem anderen Marketplace blockiert. Dies ermöglicht es Ihnen, überprüfte Hooks über einen Organisations-Marketplace zu verteilen und alles andere zu blockieren
* **Alles andere wird blockiert**: Benutzer-, Projekt- und lokale Hooks, Hooks aus anderen Plugins und Hooks, die in Agent-Frontmatter deklariert sind
* **Command-sourced Plugins sind deaktiviert**: Claude Code deaktiviert auch Plugins mit einer [`command`-Quelle](/docs/de/plugins/marketplace-reference#command-plugin-source), einschließlich Plugins, die in verwalteten `enabledPlugins` erzwungen werden, es sei denn, Sie setzen [`disableCommandPluginSources`](#disablecommandpluginsources) explizit auf `false`
* **Marketplace `headersHelper`-Befehle werden blockiert**: Claude Code blockiert auch Marketplace-[`headersHelper`-Befehle](/docs/de/plugins/host-marketplace#authenticate-archive-downloads), es sei denn, [`disableCommandPluginSources`](#disablecommandpluginsources) ist explizit auf `false` gesetzt, außer für einen Marketplace, den die verwalteten Einstellungen selbst deklarieren. Erfordert Claude Code v2.1.238 oder später
* **Statuszeile und Dateivorschlag werden auf verwaltete Einstellungen beschränkt**: Claude Code liest [`statusLine`](/docs/de/statusline), [`fileSuggestion`](#filesuggestion) und [`subagentStatusLine`](/docs/de/statusline#subagent-status-lines) nur aus verwalteten Einstellungen, gemäß den [Statuszeilen- und Dateivorschlag-Gates](#status-line-and-file-suggestion-gates)

Der [`/goal`](/docs/de/goal)-Befehl kann nicht ausgeführt werden, während dieser Schlüssel gesetzt ist, da er von Hooks abhängt.

<h3 id="disableallhooks">
  `disableAllHooks`
</h3>

Schalten Sie [Hooks](/docs/de/hooks#disable-or-remove-hooks), jede benutzerdefinierte [Statuszeile](/docs/de/statusline) und jeden benutzerdefinierten [Dateivorschlag](#filesuggestion)-Befehl aus. Verwenden Sie dies, um alle diese vorübergehend auszuschalten, ohne sie aus Ihren Einstellungen zu löschen.

* **Bereich**: [`Any file`](#scopes). Nur verwaltete Einstellungen können verwaltete Hooks deaktivieren.
* **Typ**: Boolean
  * `true`: Claude Code schaltet Hooks, jede benutzerdefinierte Statuszeile und jeden benutzerdefinierten Dateivorschlag-Befehl aus
  * `false`: Hooks, die Statuszeile und der Dateivorschlag-Befehl werden ausgeführt
* **Standard**: nicht gesetzt, daher werden Hooks ausgeführt

```json settings.json theme={null}
{
  "disableAllHooks": true
}
```

Die Reichweite hängt davon ab, welche Datei den Schlüssel trägt:

* **In verwalteten Einstellungen**: Claude Code deaktiviert jeden konfigurierten Hook, einschließlich verwalteter, und führt weiterhin die Hooks aus, die das [Agent SDK](/docs/de/agent-sdk/overview) im Prozess registriert
* **In jeder anderen Einstellungsdatei**: Claude Code deaktiviert Benutzer-, Projekt-, lokale und Plugin-Hooks; verwaltete Hooks, Agent SDK Hooks und Hooks aus Plugins, die in verwalteten [`enabledPlugins`](#enabledplugins) erzwungen werden, werden weiterhin ausgeführt

Das Beibehalten von Agent SDK Hooks, wenn verwaltete Einstellungen diesen Schlüssel setzen, erfordert Claude Code v2.1.242 oder später.

Der [`/goal`](/docs/de/goal)-Befehl kann nicht ausgeführt werden, während Hooks deaktiviert sind, und das `/hooks`-Menü zeigt stattdessen einen Hinweis anstelle Ihrer Hooks.

<h4 id="status-line-and-file-suggestion-gates">
  Statuszeilen- und Dateivorschlag-Gates
</h4>

Claude Code trifft zwei Entscheidungen für `statusLine`, `fileSuggestion` und `subagentStatusLine` in dieser Reihenfolge:

* **Vollständig ausgeschaltet**: wenn verwaltete Einstellungen `disableAllHooks` setzen, oder wenn der Ordner nicht vertraut wird gemäß der gleichen [Workspace-Vertrauensregel wie Hooks in Einstellungsdateien](/docs/de/permissions#what-runs-before-you-trust-a-folder)
* **Auf verwaltete Einstellungen beschränkt**: wenn [`allowManagedHooksOnly`](#allowmanagedhooksonly) gesetzt ist, wenn `disableAllHooks` außerhalb verwalteter Einstellungen nach Anwendung der [Einstellungspriorität](/docs/de/hooks#disable-or-remove-hooks) `true` ist, oder wenn Sie Claude Code mit `--safe-mode` starten

Bei Beschränkung führt Claude Code einen verwalteten Wert aus, wenn einer bereitgestellt wird. Andernfalls überspringt es Ihren Wert ohne Warnung: die Statuszeile ist deaktiviert und die `@`-Autovervollständigung fällt auf den integrierten Dateivorschlag zurück.

<h3 id="disableworkflows">
  `disableWorkflows`
</h3>

Schalten Sie [dynamische Workflows](/docs/de/workflows#turn-workflows-off) und die gebündelten Workflow-Befehle für alle aus, die Ihre Einstellungen erreichen, z. B. eine Organisation über verwaltete Einstellungen. Um Workflows nur für sich selbst ein- oder auszuschalten, verwenden Sie stattdessen [`enableWorkflows`](#enableworkflows), das der Schalter **Dynamic workflows** in `/config` in Ihre Benutzereinstellungen schreibt.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code schaltet dynamische Workflows und die gebündelten Workflow-Befehle für alle aus, die Ihre Einstellungen erreichen
  * `false`: dasselbe wie nicht gesetzt; ob Workflows dann aktiviert sind, folgt [`enableWorkflows`](#enableworkflows) und dem Standard Ihres Plans
* **Standard**: `false`
* **Pro-Session-Überschreibungen**: [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/de/env-vars) schaltet Workflows für eine Sitzung aus; welcher der beiden sie ausschaltet, der andere kann sie nicht wieder einschalten

```json settings.json theme={null}
{
  "disableWorkflows": true
}
```

<h3 id="enableworkflows">
  `enableWorkflows`
</h3>

Schalten Sie [dynamische Workflows](/docs/de/workflows) für sich selbst ein oder aus, wenn der Standard Ihres Plans nicht das ist, was Sie möchten. Erscheint in `/config` als **Dynamic workflows**, das diesen Schlüssel in Ihre Benutzereinstellungen schreibt und ihn wieder entfernt, wenn Sie zurück zu Ihrem Plan-Standard umschalten. Um Workflows für alle aus verwalteten Einstellungen auszuschalten, verwenden Sie stattdessen [`disableWorkflows`](#disableworkflows).

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code schaltet dynamische Workflows für Sie ein
  * `false`: Claude Code schaltet dynamische Workflows für Sie aus
* **Standard**: nicht gesetzt, daher sind Workflows aktiviert, es sei denn, Sie sind im Pro-Plan, wo sie deaktiviert sind
* **Pro-Session-Überschreibungen**: [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/de/env-vars) schaltet Workflows für eine Sitzung aus, und `true` hier kann sie nicht wieder einschalten, während es gesetzt ist

```json settings.json theme={null}
{
  "enableWorkflows": true
}
```

[`disableWorkflows`](#disableworkflows) und die Workflows-Richtlinie Ihrer Organisation haben auch Vorrang: `enableWorkflows: true` kann Workflows nicht wieder einschalten, während eine Quelle Workflows ausschaltet. Claude Code verbirgt die `/config`-Zeile, während eine andere Quelle als Ihre Benutzereinstellungen `enableWorkflows` setzt oder `disableWorkflows` auf `true` setzt.

<h3 id="hooks">
  `hooks`
</h3>

Führen Sie Ihre eigenen Befehle, Prompts, Agenten, HTTP-Anfragen oder MCP-Tools als [Hooks](/docs/de/hooks) an Punkten im Lebenszyklus von Claude Code aus, z. B. vor einem Tool-Aufruf oder wenn eine Sitzung startet; die [Hooks-Referenz](/docs/de/hooks#hook-events) listet jedes Ereignis, seine Payload und seine Exit-Codes auf. Jedes Ereignis wird einer Liste von Matcher-Gruppen zugeordnet, und jede Gruppe listet die Handler auf, die ausgeführt werden, wenn der Matcher zutrifft.

* **Bereich**: [`Any file`](#scopes). Hooks werden über Dateien hinweg zusammengeführt, anstatt sich gegenseitig zu ersetzen, und Hooks aus verwalteten Einstellungen können nicht aus anderen Dateien entfernt werden.
* **Typ**: Objekt, das nach [Hook-Ereignis](/docs/de/hooks#hook-events) verschlüsselt ist; jeder Wert ist ein Array von `{ "matcher", "hooks" }`-Gruppen, deren `hooks`-Einträge einen `type` von `"command"`, `"prompt"`, `"agent"`, `"http"` oder `"mcp_tool"` haben
* **Standard**: nicht gesetzt, daher werden keine Hooks ausgeführt

Dieses Beispiel führt ein Skript vor jedem Bash-Tool-Aufruf aus:

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

Für jedes Ereignis, Matcher-Muster und Handler-Feld siehe die [Hooks-Referenz](/docs/de/hooks#configuration). Um Hooks auszuschalten, siehe [`disableAllHooks`](#disableallhooks); um Hooks auf die zu beschränken, die Ihre Organisation bereitstellt, siehe [`allowManagedHooksOnly`](#allowmanagedhooksonly).

<h3 id="httphookallowedenvvars">
  `httpHookAllowedEnvVars`
</h3>

Ein [HTTP-Hook](/docs/de/hooks#http-hook-fields) kann den Wert einer Umgebungsvariablen in einen Request-Header einfügen, z. B. einen `Authorization: Bearer $HOOK_TOKEN`-Header, aber nur für Variablen, die der Hook in seinem eigenen `allowedEnvVars` auflistet. Dieser Schlüssel setzt eine äußere Grenze für diese Liste für jeden HTTP-Hook: Ein Hook kann eine Variable nur verwenden, wenn sowohl sein eigenes `allowedEnvVars` als auch dieser Schlüssel sie benennen. Verwenden Sie dies, um zu verhindern, dass ein Hook ein Geheimnis liest, das es nicht sollte, auch wenn die Hook-Definition danach fragt.

* **Bereich**: [`Any file`](#scopes). Arrays werden über Einstellungsdateien hinweg zusammengeführt.
* **Typ**: Array von Umgebungsvariablennamen
* **Standard**: nicht gesetzt, daher gilt die `allowedEnvVars`-Liste jedes Hooks

Dieses Beispiel begrenzt die Header-Interpolation auf `MY_TOKEN` und `HOOK_SECRET`:

```json settings.json theme={null}
{
  "httpHookAllowedEnvVars": ["MY_TOKEN", "HOOK_SECRET"]
}
```

Die Zulassungsliste gilt für Hooks aus jeder Quelle, einschließlich verwalteter Einstellungen.

<h3 id="workflowkeywordtriggerenabled">
  `workflowKeywordTriggerEnabled`
</h3>

Wählen Sie, ob die Eingabe des Schlüsselworts `ultracode` in einem Prompt einen [dynamischen Workflow](/docs/de/workflows#ask-for-a-workflow-in-your-prompt) auslöst. Setzen Sie es auf `false`, um das Wort einzugeben, ohne einen auszulösen.

* **Bereich**: [`Any file`](#scopes). Erscheint in `/config` als **Ultracode keyword trigger**.
* **Typ**: Boolean
  * `true`: Die Eingabe von `ultracode` in einem Prompt löst einen dynamischen Workflow aus
  * `false`: Sie können das Wort eingeben, ohne einen auszulösen
* **Standard**: `true`

```json settings.json theme={null}
{
  "workflowKeywordTriggerEnabled": false
}
```

Die `ultracode`-Aufwandseinstellung, `/workflows` und gespeicherte Workflow-Befehle sind nicht betroffen.

<h3 id="workflowsizeguideline">
  `workflowSizeGuideline`
</h3>

Legen Sie die [Agentenzahl fest, auf die Claude abzielt](/docs/de/workflows#set-a-size-guideline) in den dynamischen Workflows, die es schreibt. Claude Code sendet den Wert an Claude als Ratschlag, nicht als erzwungene Obergrenze: `"small"` fordert weniger als 5 Agenten an, `"medium"` weniger als 10 und `"large"` weniger als 50. Wählen Sie `"small"`, wenn Sie begrenzen möchten, was ein Workflow ausgibt. Erfordert Claude Code v2.1.219 oder später.

* **Bereich**: [`Any file`](#scopes). Ein Wert dort hat Vorrang vor der Auswahl **Dynamic workflow size** in `/config`, die Claude Code in `~/.claude.json` speichert, und Claude Code verbirgt diese Zeile, während eine Einstellungsdatei den Schlüssel setzt.
* **Typ**: String, einer von:
  * `"unrestricted"`: keine Richtlinie, daher passt Claude den Workflow an die Aufgabe an
  * `"small"`: Claude zielt auf weniger als 5 Agenten ab
  * `"medium"`: Claude zielt auf weniger als 10 Agenten ab
  * `"large"`: Claude zielt auf weniger als 50 Agenten ab
* **Standard**: `"medium"`, oder `"small"` wenn Sie im Pro-Plan mit Claude Code v2.1.271 oder später angemeldet sind

```json settings.json theme={null}
{
  "workflowSizeGuideline": "small"
}
```

Erfordert Claude Code v2.1.219 oder später; auf v2.1.202 bis v2.1.218 legen Sie die Richtlinie stattdessen in `/config` fest.

<span id="plugin-configuration" />

<span id="manage-plugins" />

<span id="plugin-settings" />

<h2 id="plugins-and-skills">
  Plugins und Skills
</h2>

Aktivieren Sie Plugins, registrieren Sie Marketplaces, beschränken Sie, welche Plugin-Quellen eine Organisation zulässt, und kontrollieren Sie, welche Skills geladen werden. Informationen zum Installieren und Erstellen von Plugins finden Sie unter [Plugins](/docs/de/plugins/overview).

<h3 id="disablebundledskills">
  `disableBundledSkills`
</h3>

Deaktivieren Sie die [Skills](/docs/de/skills) und Workflows, die in Claude Code enthalten sind. Claude Code entfernt gebündelte Skills und Workflows vollständig, während integrierte Befehle wie `/init` eingegeben werden können, aber vom Modell verborgen sind.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code entfernt gebündelte Skills und Workflows und verbirgt integrierte Befehle wie `/init` vor dem Modell
  * `false`: gebündelte Skills werden geladen
* **Standard**: nicht gesetzt, daher werden gebündelte Skills geladen
* **Sitzungsübergreifende Außerkraftsetzungen**: [`CLAUDE_CODE_DISABLE_BUNDLED_SKILLS`](/docs/de/env-vars) auf `1` gesetzt deaktiviert gebündelte Skills für eine Sitzung; welcher der beiden sie deaktiviert, der andere kann sie nicht wieder aktivieren

```json settings.json theme={null}
{
  "disableBundledSkills": true
}
```

Skills von Plugins, `.claude/skills/` und `.claude/commands/` sind nicht betroffen. `/doctor` kann wie die integrierten Befehle eingegeben werden; um ihn zu verbergen, setzen Sie stattdessen [`DISABLE_DOCTOR_COMMAND`](/docs/de/env-vars).

<h3 id="disableskillshellexecution">
  `disableSkillShellExecution`
</h3>

Deaktivieren Sie die Inline-Shell-Ausführung für `` !`...` `` und ` ```! ` Blöcke in [Skills](/de/skills) und benutzerdefinierten Befehlen aus Benutzer-, Projekt-, Plugin- oder zusätzlichen Verzeichnisquellen. Claude Code ersetzt jeden Befehl durch `[shell command execution disabled by policy]` statt ihn auszuführen.

* **Bereich**: [`Any file`](#scopes). Ein `true` in verwalteten Einstellungen kann nicht durch `false` an anderer Stelle außer Kraft gesetzt werden.
* **Typ**: Boolean
  * `true`: Claude Code ersetzt jeden Inline-Shell-Befehl durch `[shell command execution disabled by policy]` statt ihn auszuführen
  * `false`: Inline-Shell wird ausgeführt
* **Standard**: nicht gesetzt, daher wird Inline-Shell ausgeführt

```json settings.json theme={null}
{
  "disableSkillShellExecution": true
}
```

Gebündelte Skills und Skills, die über verwaltete Einstellungen bereitgestellt werden, sind nicht betroffen.

<h3 id="skilloverrides">
  `skillOverrides`
</h3>

Verbergen oder reduzieren Sie einen [Skill](/docs/de/skills#override-skill-visibility-from-settings) ohne dessen `SKILL.md` zu bearbeiten. Claude Code wendet den Wert unter jedem Skill-Namen auf die Skill-Liste an, die Claude sieht, und auf Ihre `/` Autovervollständigung.

* **Bereich**: [`Any file`](#scopes). Das `/skills` Menü schreibt in `.claude/settings.local.json`.
* **Typ**: Objekt, das Skill-Namen auf einen der folgenden Werte abbildet:
  * `"on"`: Claude sieht den Skill und Sie können `/name` eingeben
  * `"name-only"`: Claude sieht den Skill nur nach Name ohne seine Beschreibung
  * `"user-invocable-only"`: Claude sieht den Skill nicht, aber Sie können immer noch `/name` eingeben
  * `"off"`: Claude sieht den Skill nicht und `/name` ist in der Autovervollständigung verborgen
* **Standard**: nicht gesetzt, daher ist jeder Skill `"on"`

Dieses Beispiel listet `legacy-context` für Claude nur nach Name auf und verbirgt `deploy` vor Claude und vor der `/` Autovervollständigung:

```json settings.json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Außerkraftsetzungen gelten nicht für Plugin-Skills, die Sie über `/plugin` verwalten.

In verwalteten Einstellungen und Dateien, die mit `--settings` übergeben werden, gilt ein Schlüssel auf einem Alias eines gebündelten Skills, wie `checkup` für `/doctor`, auch für den Skill; siehe [wie Alias-Schlüssel mit Schlüsseln auf dem eigenen Namen des Skills kombiniert werden](/docs/de/skills#override-skill-visibility-from-settings).

<h3 id="syncclaudeaiskills">
  `syncClaudeAiSkills`
</h3>

Deaktivieren Sie den Download der [Skills, die für Ihr claude.ai-Konto aktiviert sind](/docs/de/skills#how-synced-skills-behave). Claude Code lädt sie in `~/.claude/skills/synced/` in [Terminalsitzungen herunter, in denen Sie sich mit Ihrem claude.ai-Konto anmelden](/docs/de/skills#where-synced-skills-load), interaktiv oder nicht-interaktiv, und in Cowork- und Cloud-Sitzungen. Setzen Sie `false`, um diesen Download zu stoppen und das Laden der bereits synchronisierten Skills zu beenden. Claude Code berücksichtigt nur `false`: `true` ist dasselbe wie nicht gesetzt und aktiviert die Synchronisierung nicht, wo sie sonst deaktiviert ist.

* **Bereich**: [`User, local, or managed`](#scopes) und Dateien, die mit `--settings` übergeben werden. Ein Repository kann es für Sie nicht deaktivieren.
* **Typ**: Boolean
  * `false`: Claude Code stoppt das Herunterladen synchronisierter Skills und stoppt das Laden der bereits in `~/.claude/skills/synced/` vorhandenen. In Benutzer- oder verwalteten Einstellungen verschiebt es sie auch in `~/.claude/skills/.trash/`
  * `true`: dasselbe wie nicht gesetzt
* **Standard**: nicht gesetzt, daher synchronisieren Sitzungen, die mit Ihrem claude.ai-Konto angemeldet sind, Ihre Skills

Dieses Beispiel verhindert, dass ein Computer die Skills des Kontos in einer beliebigen Sitzung herunterlädt:

```json settings.json theme={null}
{
  "syncClaudeAiSkills": false
}
```

<h3 id="syncclaudeaiplugins">
  `syncClaudeAiPlugins`
</h3>

Deaktivieren Sie den Download der [Plugins, die für Ihr claude.ai-Konto aktiviert sind](/docs/de/plugins/loading#synced-plugins). Claude Code lädt sie in `~/.claude/plugins/synced/` am Anfang von Terminalsitzungen herunter, in denen Sie sich mit Ihrem claude.ai-Konto anmelden, und in Cowork-Sitzungen, und lädt jedes als `<name>@synced`. Setzen Sie `false`, um diesen Download zu stoppen und das Laden der bereits synchronisierten Plugins zu beenden. Claude Code berücksichtigt nur `false`: `true` ist dasselbe wie nicht gesetzt und aktiviert die Synchronisierung nicht, wo sie sonst deaktiviert ist. Erfordert Claude Code v2.1.273 oder später.

* **Bereich**: [`User, local, or managed`](#scopes) und Dateien, die mit `--settings` übergeben werden. Ein Repository kann es für Sie nicht deaktivieren.
* **Typ**: Boolean
  * `false`: Claude Code stoppt das Herunterladen synchronisierter Plugins und stoppt das Laden der bereits in `~/.claude/plugins/synced/` vorhandenen. In Benutzer- oder verwalteten Einstellungen verschiebt es sie auch in `~/.claude/plugins/.trash/`
  * `true`: dasselbe wie nicht gesetzt
* **Standard**: nicht gesetzt, daher synchronisieren Sitzungen, die mit Ihrem claude.ai-Konto angemeldet sind, Ihre Plugins

Um ein synchronisiertes Plugin auszuschalten, anstatt alle, setzen Sie `"<name>@synced": false` in [`enabledPlugins`](#enabledplugins).

Dieses Beispiel verhindert, dass ein Computer die Plugins des Kontos in einer beliebigen Sitzung herunterlädt:

```json settings.json theme={null}
{
  "syncClaudeAiPlugins": false
}
```

<h3 id="allowedchannelplugins">
  `allowedChannelPlugins`
</h3>

Wählen Sie, in welche [Kanal](/docs/de/channels)-Plugins Nachrichten in Sitzungen in Ihrer Organisation pushen können. Wenn Sie es festlegen, verwendet Claude Code Ihre Liste anstelle der Standard-Anthropic-Zulassungsliste; jeder Eintrag benennt ein Plugin und den Marketplace, aus dem es stammt.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Array von Objekten, jeweils mit `marketplace` und `plugin` Strings. Ein Eintrag kann stattdessen ein `"plugin@marketplace"` String wie `"telegram@claude-plugins-official"` sein, den Claude Code als das äquivalente Objekt behandelt. Die String-Form erfordert Claude Code v2.1.267 oder später; frühere Versionen lehnen den gesamten `allowedChannelPlugins` Wert ab, wenn er einen enthält
* **Standard**: nicht gesetzt, daher verwendet Claude Code die Standard-Anthropic-Zulassungsliste

Dieses Beispiel aktiviert Kanäle und erlaubt nur das Telegram-Plugin aus dem offiziellen Anthropic-Marketplace:

```json managed-settings.json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" }
  ]
}
```

Ein leeres Array blockiert jedes Kanal-Plugin.

Dieser Schlüssel wird wirksam, sobald Kanäle das [`channelsEnabled`](#channelsenabled) Gate für das Konto passieren: auf Team- und Enterprise-Plänen und auf Console-Konten mit verwalteten Einstellungen bedeutet das `channelsEnabled: true`. Siehe [Beschränken Sie, welche Kanal-Plugins ausgeführt werden können](/docs/de/channels#restrict-which-channel-plugins-can-run).

<h3 id="blockedmarketplaces">
  `blockedMarketplaces`
</h3>

Blockieren Sie Plugin-Marketplace-Quellen für Ihre Organisation. Claude Code überprüft die Blockliste beim Hinzufügen von Marketplace und beim Installieren, Aktualisieren, Aktualisieren und automatischen Aktualisieren von Plugins, daher kann ein Marketplace, den jemand hinzugefügt hat, bevor Sie die Richtlinie festgelegt haben, nicht zum Abrufen von Plugins verwendet werden. Blockierte Quellen werden vor dem Download überprüft, daher berühren sie niemals das Dateisystem.

Wenn Sie diesen Schlüssel in der [claude.ai Admin-Konsole](/docs/de/server-managed-settings) festlegen, wendet claude.ai ihn auch an, wenn jemand in Ihrer Organisation einen Marketplace aus einem Git-Repository auf claude.ai hinzufügt, wie [Wie Einschränkungen funktionieren](/docs/de/plugins/org#restrict-what-users-can-install) beschreibt.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Array von Marketplace-Quellobjekten in denselben Formen wie [`strictKnownMarketplaces`](#allowed-source-types)
* **Standard**: nicht gesetzt, daher ist kein Marketplace blockiert

Dieses Beispiel blockiert ein GitHub-Repository als Marketplace-Quelle:

```json managed-settings.json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted/plugins" }
  ]
}
```

Ein `github` Eintrag kann die [Owner-Wildcard-Form](#owner-wildcards) `"owner/*"` verwenden, um jedes Repository unter diesem GitHub-Owner zu blockieren, was Claude Code v2.1.223 oder später erfordert. Fügen Sie `{ "source": "skills-dir" }` hinzu, um Claude Code daran zu hindern, [`@skills-dir` Plugins](/docs/de/plugins/loading#plugins-shared-through-a-repository) aus `~/.claude/skills/` zu laden, ohne einen Marketplace einzuschränken. Siehe [Verwaltete Marketplace-Einschränkungen](/docs/de/plugins/org#restrict-what-users-can-install).

<h3 id="channelsenabled">
  `channelsEnabled`
</h3>

Erlauben Sie [Kanäle](/docs/de/channels) für Ihre Organisation. Bei claude.ai Team- und Enterprise-Plänen blockiert Claude Code Kanäle, bis Sie dies auf `true` setzen. Für [Anthropic Console](/docs/de/authentication#claude-console-authentication) Konten, die sich mit einem API-Schlüssel authentifizieren, sind Kanäle standardmäßig zulässig. Wenn Ihre Organisation verwaltete Einstellungen bereitstellt, blockiert Claude Code Kanäle auf diesen Konten auch, bis Sie diesen Schlüssel auf `true` setzen.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code erlaubt Kanäle für Ihre Organisation
  * `false`: dasselbe wie nicht gesetzt; ob Kanäle blockiert sind, hängt von Ihrem Plan ab, wie der Standard sagt
* **Standard**: nicht gesetzt; Kanäle sind auf Team- und Enterprise-Plänen und auf Console-Konten mit verwalteten Einstellungen blockiert und auf Pro- und Max-Plänen und auf Console-Konten ohne verwaltete Einstellungen zulässig

```json managed-settings.json theme={null}
{
  "channelsEnabled": true
}
```

Um einzuschränken, welche Plugins sich als Kanäle registrieren können, sobald sie aktiviert sind, setzen Sie [`allowedChannelPlugins`](#allowedchannelplugins). Siehe [Enterprise-Kontrollen](/docs/de/channels#enterprise-controls).

<h3 id="disablecommandpluginsources">
  `disableCommandPluginSources`
</h3>

Blockieren Sie die [`command` Plugin-Quelle](/docs/de/plugins/marketplace-reference#command-plugin-source), die ein Plugin durch Ausführung eines von Marketplace deklarierten Befehls auf dem Computer des Benutzers installiert. Wenn Sie es auf `true` setzen, führt Claude Code den Befehl niemals aus, installiert oder aktualisiert keine Befehls-Quellen-Plugins und stoppt das Laden der bereits installierten. Setzen Sie es auf `false`, um sie explizit zuzulassen. Wann immer es Befehlsquellen blockiert, ob Sie es auf `true` setzen oder es unter [`allowManagedHooksOnly`](#allowmanagedhooksonly) nicht gesetzt lassen, blockiert es auch Marketplace [`headersHelper` Befehle](/docs/de/plugins/host-marketplace#authenticate-archive-downloads), außer für einen Marketplace, den verwaltete Einstellungen selbst deklarieren. Erfordert Claude Code v2.1.229 oder später, und der `headersHelper` Block erfordert v2.1.238 oder später.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code führt den von Marketplace deklarierten Befehl niemals aus, installiert oder aktualisiert keine Befehls-Quellen-Plugins und stoppt das Laden der bereits installierten
  * `false`: Claude Code erlaubt Befehls-Quellen-Plugins explizit
* **Standard**: nicht gesetzt, daher folgt Claude Code [`allowManagedHooksOnly`](#allowmanagedhooksonly): eine Organisation, die die Hook-Ausführung auf verwaltete Einstellungen beschränkt, bekommt auch Befehlsquellen deaktiviert

```json managed-settings.json theme={null}
{
  "disableCommandPluginSources": true
}
```

Erfordert Claude Code v2.1.229 oder später.

<h3 id="pluginsuggestionmarketplaces">
  `pluginSuggestionMarketplaces`
</h3>

Benennen Sie die Marketplaces, deren Plugins als kontextuelle Installationsvorschläge erscheinen können, in Spinner-Tipps und oben im `/plugin` **Discover** Tab angeheftet. Der integrierte First-Party-Frontend-Design-Tipp ist nicht betroffen. Vorschläge stammen aus der `relevance` Deklaration jedes Plugins in seinem Marketplace-Eintrag.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Array von Marketplace-Namen
* **Standard**: nicht gesetzt, daher werden keine von Marketplace deklarierten Vorschläge angezeigt

```json managed-settings.json theme={null}
{
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

Ein Name wird nur wirksam, wenn der Marketplace auf dem Computer registriert ist und seine registrierte Quelle auch in denselben verwalteten Einstellungen deklariert ist, entweder als [`extraKnownMarketplaces`](#extraknownmarketplaces) Eintrag für diesen Namen oder als Eintrag von [`strictKnownMarketplaces`](#strictknownmarketplaces). Claude Code ignoriert einen Marketplace, der von einer anderen Quelle unter einem zulässigen Namen registriert ist. Der offizielle Marketplace ist von der Quellanforderung befreit: das Zulassen seines Namens allein genügt, da dieser Name nur von der offiziellen Anthropic-Quelle registriert werden kann. Siehe [Plugins nach Kontext vorschlagen](/docs/de/plugins/relevance).

<h3 id="plugintrustmessage">
  `pluginTrustMessage`
</h3>

Fügen Sie den eigenen Text Ihrer Organisation zur Plugin-Vertrauenswarnung hinzu, die Claude Code vor der Installation anzeigt, um beispielsweise zu bestätigen, dass Plugins aus Ihrem internen Marketplace überprüft werden.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: String
* **Standard**: nicht gesetzt, daher zeigt Claude Code nur die Standardwarnung an

```json managed-settings.json theme={null}
{
  "pluginTrustMessage": "All plugins from our marketplace are approved by IT"
}
```

<h3 id="strictknownmarketplaces">
  `strictKnownMarketplaces`
</h3>

Beschränken Sie, welche Plugin-Marketplace-Quellen Personen in Ihrer Organisation hinzufügen und Plugins installieren können. Claude Code erzwingt die Zulassungsliste beim Hinzufügen von Marketplace und beim Installieren, Aktualisieren, Aktualisieren und automatischen Aktualisieren von Plugins, vor jeder Netzwerk- oder Dateisystemoperation, daher kann ein Marketplace, den jemand hinzugefügt hat, bevor Sie die Richtlinie festgelegt haben, nicht zum Abrufen von Plugins verwendet werden, sobald seine Quelle nicht mehr übereinstimmt. Blockierte Benutzer sehen einen Fehler, der die verwaltete Richtlinie benennt.

Wenn Sie diesen Schlüssel in der [claude.ai Admin-Konsole](/docs/de/server-managed-settings) festlegen, wendet claude.ai ihn auch an, wenn jemand in Ihrer Organisation einen Marketplace aus einem Git-Repository auf claude.ai hinzufügt, wie [Wie Einschränkungen funktionieren](/docs/de/plugins/org#restrict-what-users-can-install) beschreibt.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Array von Marketplace-Quellobjekten; siehe [Zulässige Quellentypen](#allowed-source-types)
* **Standard**: nicht gesetzt, daher können Benutzer jeden Marketplace hinzufügen. Ein leeres Array ist eine vollständige Sperrung, die jede Marketplace-Quelle blockiert, einschließlich des offiziellen Anthropic-Marketplace

Dieses Beispiel erlaubt zwei GitHub-Repositories, eines auf den `v2.0` Ref gepinnt und eines gehostete `marketplace.json` URL:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/approved-plugins" },
    { "source": "github", "repo": "acme-corp/security-tools", "ref": "v2.0" },
    { "source": "url", "url": "https://plugins.example.com/marketplace.json" }
  ]
}
```

Sie können diesen Schlüssel auch als `allowedMarketplaces` schreiben; [Marketplace-Schlüssel-Aliase](#marketplace-key-aliases) beschreibt, wie Claude Code den Alias behandelt und welche Version ihn akzeptiert. Dieser Schlüssel ist ein Richtlinien-Gate: er kontrolliert, was Benutzer hinzufügen dürfen, registriert aber nichts. Um in einer Datei einzuschränken und vorab zu registrieren, siehe [Mit `extraKnownMarketplaces` kombinieren](#combine-with-extraknownmarketplaces). Für die benutzergerichtete Ansicht siehe [Verwaltete Marketplace-Einschränkungen](/docs/de/plugins/org#restrict-what-users-can-install).

<h4 id="allowed-source-types">
  Zulässige Quellentypen
</h4>

Jeder Eintrag unten zeigt einen Zulassungslisten-Eintrag pro Quellentyp und die Felder, die er akzeptiert. Die meisten Typen stimmen genau überein; `hostPattern` und `pathPattern` stimmen per Regex überein, und `github` Einträge können eine [Owner-Wildcard](#owner-wildcards) verwenden.

| Quelle        | Beispiel-Eintrag                                                                                                                | Felder                                                                                                                                                              |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `github`      | `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main", "path": "marketplace" }`                                     | `repo` erforderlich; `ref` ist ein Branch oder Tag; `path` ist ein Unterverzeichnis                                                                                 |
| `git`         | `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git", "ref": "production" }`                               | `url` erforderlich; `ref` und `path` wie für `github`                                                                                                               |
| `url`         | `{ "source": "url", "url": "https://plugins.example.com/marketplace.json", "headers": { "Authorization": "Bearer ${TOKEN}" } }` | `url` erforderlich; `headers` fügt HTTP-Header für authentifizierten Zugriff hinzu                                                                                  |
| `file`        | `{ "source": "file", "path": "/opt/acme-corp/plugins/marketplace.json" }`                                                       | `path` erforderlich, der absolute Pfad zu einer `marketplace.json` Datei                                                                                            |
| `directory`   | `{ "source": "directory", "path": "/opt/acme-corp/approved-marketplaces" }`                                                     | `path` erforderlich, der absolute Pfad zu einem Verzeichnis mit `.claude-plugin/marketplace.json`                                                                   |
| `hostPattern` | `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`                                                        | `hostPattern` erforderlich, ein Regex, das gegen den Marketplace-Host abgeglichen wird; verankern Sie es mit `^` und `$`, um den ganzen Host abzugleichen           |
| `pathPattern` | `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`                                                                 | `pathPattern` erforderlich, ein Regex, das gegen den `path` von `file` und `directory` Quellen abgeglichen wird; beginnen Sie es mit `^`, um ein Präfix zu fixieren |
| `skills-dir`  | `{ "source": "skills-dir" }`                                                                                                    | Keine Felder. Aktiviert den `~/.claude/skills/` Plugin-Scan wieder                                                                                                  |

Drei Quellentypen tragen Regeln über die Tabelle hinaus:

* **`url`**: Ein URL-Marketplace lädt nur die `marketplace.json` Datei herunter, und Claude Code lädt Plugin-Dateien nicht per relativem Pfad von diesem Server herunter, daher müssen seine Plugins eine [Plugin-Quelle](/docs/de/plugins/marketplace-reference#plugin-sources) verwenden, die nicht ein relativer Pfad ist, wie eine Archiv-URL, die auf demselben Host sein kann. Für Plugins mit relativen Pfaden verwenden Sie stattdessen einen Git-basierten Marketplace. Siehe [Plugins mit relativen Pfaden schlagen in URL-basierten Marketplaces fehl](/docs/de/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces).
* **`hostPattern`**: Verwenden Sie es, um jeden Marketplace auf einem internen GitHub Enterprise oder GitLab Server zuzulassen, ohne jedes Repository aufzulisten. Claude Code gleicht `github` Quellen gegen `github.com` ab, nimmt den Hostnamen von `url` Quellen und nimmt ihn von `git` Quellen je nach [git URL](https://git-scm.com/docs/git-clone#_git_urls) Form:

  * Eine URL mit einem Schema, wie `https://` oder `ssh://`: der Hostname in der URL.
  * Eine SSH-Adresse ohne Schema, in Gits `user@host:path` Form, wie `git@git.example.com:tools/plugins.git`: der Host zwischen `@` und `:`, der der Host ist, mit dem Git sich verbindet.
  * Jede andere Form ohne Schema: kein Host, daher stimmt kein `strictKnownMarketplaces` `hostPattern` Eintrag damit überein. Für einen `blockedMarketplaces` `hostPattern` nimmt Claude Code einen Host aus einem breiteren Satz von Formen, daher kann ein Blocklist-Eintrag immer noch mit solch einer Form übereinstimmen. Vor v2.1.234 stimmte ein `strictKnownMarketplaces` `hostPattern` auch mit einigen Formen überein, die Git nicht als SSH-Adressen behandelt.

  `file` und `directory` Quellen haben keinen Host und stimmen niemals mit einem `hostPattern` Eintrag überein.
* **`pathPattern`**: Verwenden Sie es, um Dateisystem-Marketplaces neben `hostPattern` Einträgen für Netzwerkquellen zuzulassen. `".*"` erlaubt jeden lokalen Pfad; ein engeres Muster wie `"^/opt/approved/"` beschränkt auf ein Verzeichnis.

Jede Zulassungsliste, auch eine leere, stoppt auch Claude Code beim Laden von [`@skills-dir` Plugins](/docs/de/plugins/loading#plugins-shared-through-a-repository) aus `~/.claude/skills/`. Fügen Sie den `{ "source": "skills-dir" }` Eintrag hinzu, um sie weiterhin zu laden; der Eintrag hat außerhalb dieses Schlüssels und `blockedMarketplaces` keine Bedeutung.

<h4 id="owner-wildcards">
  Owner-Wildcards
</h4>

Ein `github` Eintrag, dessen `repo` Wert `"<owner>/*"` ist, stimmt mit jedem Repository unter diesem GitHub-Owner überein. Owner-Wildcards erfordern Claude Code v2.1.223 oder später und funktionieren nur in `strictKnownMarketplaces` und `blockedMarketplaces`. Überall sonst, wo eine `github` Quelle erscheint, wie `extraKnownMarketplaces` oder `/plugin marketplace add`, muss der `repo` Wert ein einzelnes Repository benennen. Vor v2.1.223 verglich Claude Code den Eintrag buchstäblich, daher stimmte ein Zulassungslisten-Eintrag mit keinem Repository überein und ein Blocklist-Eintrag blockierte nichts; Einträge für einzelne Repositories werden auf jeder Version erzwungen.

Dieser Eintrag erlaubt jeden Marketplace-Repository in der `acme-corp` Organisation:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/*" }
  ]
}
```

Nur die ganze Repository-Namen-Position kann ein Wildcard sein. Claude Code ignoriert Einträge wie `*`, `*/plugins` oder `acme-corp/tools-*` als ungültig, daher stimmen sie mit keinem Repository überein.

Die Abgleichregeln unterscheiden sich zwischen den beiden Einstellungen:

| Regel                           | `strictKnownMarketplaces`                                                                                                                                                                              | `blockedMarketplaces`                                                                     |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------- |
| Abgleich von Quellschreibweisen | `owner/repo` Form nur. Eine Git-URL, die dasselbe Repository klont, stimmt nicht überein                                                                                                               | Jede Schreibweise, einschließlich Git-URLs, die zum selben github.com Repository auflösen |
| Owner-Fall                      | Groß-/Kleinschreibung beachtet, wie exakter Eintrag-Abgleich                                                                                                                                           | Groß-/Kleinschreibung ignoriert                                                           |
| `ref`                           | Folgt den exakten Eintrag-Regeln: ein Eintrag mit einem `ref` stimmt nur mit Quellen mit diesem exakten Ref überein, und ein Eintrag ohne einen stimmt nur mit Quellen überein, die keinen Ref angeben | Ein Eintrag ohne einen `ref` blockiert alle Refs der Repositories, die er abgleicht       |
| `path`                          | Lockerer als die exakten Eintrag-Regeln: ein Eintrag mit einem `path` erfordert diesen exakten Wert, während ein Eintrag ohne einen jeden Pfad im Repository abgleicht                                 | Ein Eintrag ohne einen `path` blockiert alle Pfade der Repositories, die er abgleicht     |

<h4 id="exact-matching">
  Exakter Abgleich
</h4>

Für jeden Quellentyp außer Owner-Wildcard `github` Einträgen und den Regex-abgeglichenen `hostPattern` und `pathPattern` Einträgen erlaubt Claude Code eine Benutzer-Addition nur, wenn die Marketplace-Quelle genau mit einem Eintrag übereinstimmt. Für die Git-basierten Quellen `github` und `git` umfasst der exakte Abgleich die optionalen Felder:

* Der `repo` oder `url` muss genau übereinstimmen
* Das `ref` Feld muss genau übereinstimmen, oder beide müssen nicht definiert sein
* Das `path` Feld muss genau übereinstimmen, oder beide müssen nicht definiert sein

Zum Beispiel behandelt Claude Code jedes Paar unten als zwei verschiedene Quellen:

* `{ "source": "github", "repo": "acme-corp/plugins" }` und `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main" }`
* `{ "source": "github", "repo": "acme-corp/plugins", "path": "marketplace" }` und `{ "source": "github", "repo": "acme-corp/plugins" }`

<h4 id="allow-only-the-official-marketplace">
  Nur den offiziellen Marketplace zulassen
</h4>

Um den offiziellen Anthropic-Marketplace und nichts anderes zuzulassen, listen Sie sein Repository auf:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" }
  ]
}
```

Mit diesem Eintrag behält Claude Code einen bereits registrierten offiziellen Marketplace bei und registriert den Marketplace beim ersten interaktiven Start von Claude Code automatisch auf einem neuen Computer. Die automatische Registrierung verpasst am häufigsten:

* Nicht-interaktive Umgebungen, die vor dem ersten interaktiven Start des Computers ausgeführt werden.
* Computer, auf denen Claude Code bereits interaktiv unter einer Richtlinie ausgeführt wurde, die den Marketplace blockierte, wie die leere Array-Sperrung. Claude Code zeichnet den blockierten Versuch auf und versucht nicht erneut, nachdem sich die Richtlinie ändert.

Fügen Sie auf diesen Computern den Marketplace zu [`extraKnownMarketplaces`](#extraknownmarketplaces) in derselben `managed-settings.json` hinzu, damit Claude Code ihn automatisch registriert, oder führen Sie `claude plugin marketplace add anthropics/claude-plugins-official` aus.

<h4 id="combine-with-extraknownmarketplaces">
  Mit `extraKnownMarketplaces` kombinieren
</h4>

Die beiden Schlüssel erfüllen unterschiedliche Aufgaben. Diese Tabelle vergleicht sie:

| Aspekt                          | `strictKnownMarketplaces`                | `extraKnownMarketplaces`                                                                                                    |
| ------------------------------- | ---------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| Zweck                           | Durchsetzung der Organisationsrichtlinie | Team-Komfort                                                                                                                |
| Einstellungsdatei               | Nur verwaltete Einstellungen             | Jede Einstellungsdatei                                                                                                      |
| Verhalten                       | Blockiert nicht zulässige Additionen     | Registriert fehlende Marketplaces                                                                                           |
| Wann erzwungen                  | Vor Netzwerk- und Dateisystemoperationen | Sofort aus Benutzer- oder verwalteten Einstellungen; nach dem Workspace-Vertrauensdialog für die Dateien eines Repositories |
| Kann außer Kraft gesetzt werden | Nein, höchste Priorität                  | Ja, durch höher priorisierte Einstellungen                                                                                  |
| Quellenformat                   | Direktes Quellobjekt                     | Benannter Marketplace mit einem verschachtelten `source` Objekt                                                             |

Um einen Marketplace sowohl einzuschränken als auch vorab zu registrieren für alle Benutzer, setzen Sie beide in `managed-settings.json`:

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

Mit nur `strictKnownMarketplaces` gesetzt, können Benutzer einen zulässigen Marketplace immer noch selbst mit `/plugin marketplace add` hinzufügen. Der offizielle Anthropic-Marketplace ist der einzige, den Claude Code automatisch registriert, und nur wenn die Zulassungsliste ihn zulässt. [Nur den offiziellen Marketplace zulassen](#allow-only-the-official-marketplace) listet die Computer auf, die er verpasst.

<h3 id="strictpluginonlycustomization">
  `strictPluginOnlyCustomization`
</h3>

Blockieren Sie Skills, Agents, Hooks und MCP-Server von Benutzer- und Projektquellen, daher können sie nur von Plugins oder verwalteten Einstellungen stammen. Kombinieren Sie es mit [`strictKnownMarketplaces`](#strictknownmarketplaces), um die vollständige Anpassungslieferkette zu kontrollieren: die Marketplace-Zulassungsliste kontrolliert, welche Plugins Benutzer installieren können.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: `true`, um alle vier Arten von Anpassung zu sperren, oder ein Array, das die zu sperrenden Arten benennt, von `"skills"`, `"agents"`, `"hooks"` und `"mcp"`
* **Standard**: nicht gesetzt, daher ist nichts gesperrt

Dieses Beispiel sperrt Skills und Hooks und lässt Agents und MCP-Server entsperrt:

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills", "hooks"]
}
```

Die vier Sub-Schlüssel-Einträge unten listen auf, was jede Oberfläche blockiert und was immer noch geladen wird. Claude Code ignoriert Oberflächennamen, die es nicht erkennt, anstatt die Einstellungsdatei fehlschlagen zu lassen, daher können Sie neue Oberflächennamen hinzufügen, bevor jeder Client aktualisiert hat.

<h3 id="strictpluginonlycustomization-skills">
  `strictPluginOnlyCustomization.skills`
</h3>

Sperren Sie die `skills` Oberfläche. Claude Code stoppt das Laden von Skills aus `~/.claude/skills/` und `.claude/skills/`, benutzerdefinierten Befehlen aus `~/.claude/commands/` und `.claude/commands/`, Skills unter `--add-dir` Verzeichnissen und Skills, die von Ihrem claude.ai-Konto synchronisiert werden, und lädt weiterhin Plugin-Skills, gebündelte Skills und Skills im verwalteten Richtlinienverzeichnis.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: der String `"skills"` im [`strictPluginOnlyCustomization`](#strictpluginonlycustomization) Array
* **Standard**: nicht gesperrt

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills"]
}
```

<h3 id="strictpluginonlycustomization-agents">
  `strictPluginOnlyCustomization.agents`
</h3>

Sperren Sie die `agents` Oberfläche. Claude Code stoppt das Laden von Agents aus `~/.claude/agents/` und `.claude/agents/` und lädt weiterhin Plugin-Agents, integrierte Agents und Agents im verwalteten Richtlinienverzeichnis.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: der String `"agents"` im [`strictPluginOnlyCustomization`](#strictpluginonlycustomization) Array
* **Standard**: nicht gesperrt

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["agents"]
}
```

<h3 id="strictpluginonlycustomization-hooks">
  `strictPluginOnlyCustomization.hooks`
</h3>

Sperren Sie die `hooks` Oberfläche. Claude Code stoppt das Ausführen von Hooks aus Benutzer-, Projekt- und lokalen `settings.json` und führt weiterhin Plugin-Hooks und Hooks in verwalteten Einstellungen aus.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: der String `"hooks"` im [`strictPluginOnlyCustomization`](#strictpluginonlycustomization) Array
* **Standard**: nicht gesperrt

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["hooks"]
}
```

<h3 id="strictpluginonlycustomization-mcp">
  `strictPluginOnlyCustomization.mcp`
</h3>

Sperren Sie die `mcp` Oberfläche. Claude Code stoppt das Laden von MCP-Servern aus `~/.claude.json` und `.mcp.json` und lädt weiterhin Plugin-MCP-Server, [`managed-mcp.json`](/docs/de/managed-mcp) Server und Server von [`managedMcpServers`](#managedmcpservers).

* **Bereich**: [`Managed`](#scopes)
* **Typ**: der String `"mcp"` im [`strictPluginOnlyCustomization`](#strictpluginonlycustomization) Array
* **Standard**: nicht gesperrt

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["mcp"]
}
```

<h3 id="enabledplugins">
  `enabledPlugins`
</h3>

Schalten Sie einzelne [Plugins](/docs/de/plugins/overview) ein oder aus, gekennzeichnet durch `plugin-name@marketplace-name`. Ein Plugin ohne Eintrag in einem beliebigen Bereich fällt auf seinen [`defaultEnabled`](/docs/de/plugins/manifest-reference#fields) Wert zurück. Wenn Sie ein Plugin mit `/plugin` oder `claude plugin enable` aktivieren oder deaktivieren, schreibt Claude Code diesen Schlüssel für Sie.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Objekt, das `plugin-name@marketplace-name` auf einen Boolean abbildet
* **Standard**: nicht gesetzt, daher folgt jedes Plugin seinem `defaultEnabled` Wert

Dieses Beispiel aktiviert zwei Plugins aus dem `team-tools` Marketplace und deaktiviert eines aus `personal`:

```json settings.json theme={null}
{
  "enabledPlugins": {
    "code-formatter@team-tools": true,
    "deployment-tools@team-tools": true,
    "experimental-features@personal": false
  }
}
```

Jeder Bereich dient einem anderen Zweck:

* **Benutzereinstellungen**: Ihre persönlichen Plugin-Voreinstellungen
* **Projekteinstellungen**: Plugins, die mit jedem im Repository geteilt werden
* **Lokale Einstellungen**: Pro-Computer-Außerkraftsetzungen, gitignoriert, wenn Claude Code eine Einstellung dort speichert
* **Verwaltete Einstellungen**: Organisationsrichtlinie. Ein Plugin, das hier auf `false` gesetzt ist, ist von der Installation in jedem Bereich blockiert und im Marketplace verborgen

Projekteinstellungen haben Vorrang vor Benutzereinstellungen, daher deaktiviert das Setzen eines Plugins auf `false` in `~/.claude/settings.json` kein Plugin, das die `.claude/settings.json` des Projekts aktiviert. Um sich von einem von Projekt aktivierten Plugin auf Ihrem Computer abzumelden, setzen Sie es stattdessen auf `false` in `.claude/settings.local.json`. Plugins, die von verwalteten Einstellungen erzwungen aktiviert werden, können auf diese Weise nicht deaktiviert werden, da verwaltete Einstellungen lokale Einstellungen außer Kraft setzen.

Das Aktivieren eines Plugins aus einer externen Quelle wie einem GitHub-Repository oder npm-Paket in der `.claude/settings.json` eines Projekts installiert es nicht für andere Personen. Auf jedem Pfad, der Plugins lädt, meldet Claude Code das Plugin als nicht installiert, bis jeder Benutzer es selbst [installiert](/docs/de/plugins/org#require-plugins-per-repository).

<h3 id="extraknownmarketplaces">
  `extraKnownMarketplaces`
</h3>

Registrieren Sie zusätzliche Plugin-Marketplaces nach Name, damit Personen, die das Repository öffnen, oder jeder, den Ihre verwalteten Einstellungen erreichen, den Marketplace erhalten, ohne ihn selbst hinzuzufügen. Claude Code registriert jeden Marketplace, den es noch nicht kennt. Ob ein Plugin, das [`enabledPlugins`](#enabledplugins) von ihm benennt, installiert wird, hängt von der Plugin-Quelle und welche Datei es aktiviert ab; dieser Eintrag hat die Regeln.

* **Bereich**: [`Any file`](#scopes). Claude Code berücksichtigt Einträge in der `.claude/settings.json` oder `.claude/settings.local.json` eines Repositories nur, nachdem Sie den Workspace-Vertrauensdialog für diesen Ordner akzeptieren; in einem Ordner, dem Sie nicht vertrauen, einschließlich eines `-p` Laufs dort, ignoriert es sie ohne Nachricht.
* **Typ**: Objekt, das einen Marketplace-Namen auf ein Objekt mit einem `source` Objekt und einem optionalen `autoUpdate` Boolean abbildet
* **Standard**: nicht gesetzt

Dieses Beispiel registriert einen GitHub-Marketplace und einen Marketplace von einer selbst gehosteten Git-URL:

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

[Was lädt, bevor Sie einen Ordner vertrauen](/docs/de/permissions#what-runs-before-you-trust-a-folder) vergleicht das Vertrauens-Gate mit dem anderen Inhalt, den ein Repository liefern kann. Sie können diesen Schlüssel auch als `additionalMarketplaces` schreiben; siehe [Marketplace-Schlüssel-Aliase](#marketplace-key-aliases).

Setzen Sie `"autoUpdate": true` neben `source`, um Claude Code zu veranlassen, diesen Marketplace zu aktualisieren und seine installierten Plugins nach dem Start im Hintergrund zu aktualisieren. Wenn weggelassen, standardmäßig `claude-plugins-official` und die meisten anderen offiziellen Anthropic-Marketplaces auf `true`, und Drittanbieter-Marketplaces standardmäßig auf `false`. Siehe [Konfigurieren Sie Auto-Updates](/docs/de/plugins/install#keep-plugins-updated).

Wenn mehr als eine Einstellungsdatei einen Marketplace-Eintrag unter demselben Namen definiert, verwendet Claude Code den Eintrag aus der [höchsten Prioritätsdatei](/docs/de/settings#settings-precedence) ganz. Dieser Eintrag ersetzt den Eintrag mit niedrigerer Priorität und erbt keine seiner Felder, daher kann eine Neudefinition nicht die `source.headers` Anmeldedaten einer Datei mit einer URL kombinieren, die eine andere Datei kontrolliert. Vor v2.1.228 fusionierte Claude Code Einträge mit demselben Namen Feld für Feld, daher konnte ein Eintrag in einer höher priorisierten Datei Felder erben, die er nicht setzte, einschließlich `headers` einer anderen Datei.

<h4 id="marketplace-source-types">
  Marketplace-Quellentypen
</h4>

Das `source` Objekt nimmt eine dieser Formen an:

* **`github`**: ein GitHub-Repository, mit `repo`
* **`git`**: jede Git-URL, mit `url`
* **`url`**: eine direkte URL zu einer `marketplace.json` Datei, mit `url` und optionalen `headers` und `headersHelper` für authentifizierten Zugriff. `headersHelper` benennt einen Befehl, der Header druckt, deren Werte zu kurzlebig sind, um in `headers` aufzulisten, und erfordert Claude Code v2.1.238 oder später
* **`file`**: ein lokaler Pfad zu einer `marketplace.json` Datei, mit `path`
* **`directory`**: ein lokaler Dateisystem-Pfad, mit `path`, nur für Entwicklung
* **`settings`**: ein Inline-Marketplace, der direkt in der Einstellungsdatei ohne ein gehostetes Repository deklariert ist, mit `name` und `plugins`

Der `git` Quellentyp funktioniert mit jedem Git-Hosting-Service, einschließlich selbst gehosteter GitLab und Bitbucket. Claude Code klont das Repository mit derselben Authentifizierung, die `git clone` auf diesem Computer verwenden würde: konfigurierte Credential-Helper oder SSH-Schlüssel. Ein Provider-Token wie `GITHUB_TOKEN` wird nur durch einen Credential-Helper wirksam, der ihn liest. Siehe [Private Repositories](/docs/de/plugins/host-marketplace#grant-access-to-a-private-marketplace) für Setup-Details.

Für `github` und `git` Quellen lädt Claude Code niemals [Git LFS](https://git-lfs.com) Inhalte herunter, wenn es das Marketplace-Repository klont, um es hinzuzufügen oder zu aktualisieren. LFS-verfolgte Dateien werden als Zeiger-Dateien ausgecheckt, und die Ausgabe zum Hinzufügen oder Aktualisieren meldet, wie viele.

Das `skipLfs` Feld im `source` Objekt wird akzeptiert und hat keine Auswirkung. Vor v2.1.274 lud Claude Code LFS-Inhalte herunter, es sei denn, Sie setzen `"skipLfs": true`.

Für eine `url` Quelle setzen Sie `headersHelper` im `source` Objekt, wenn die Anmeldedaten in `headers` ablaufen und ein Befehl eine frische produzieren muss. Erfordert Claude Code v2.1.238 oder später. Für das, was der Befehl drucken muss und wo Claude Code ihn ausführt, siehe [Schreiben Sie den headersHelper-Befehl](/docs/de/plugins/host-marketplace#write-the-headershelper-command), und für die Fälle, in denen Claude Code ihn nicht ausführt, siehe [Wenn Claude Code einen headersHelper-Befehl überspringt oder seine Ausgabe verwirft](/docs/de/plugins/host-marketplace#when-claude-code-skips-a-headershelper-command-or-drops-its-output). Sobald Sie `headersHelper` auf einer `https://` Marketplace-URL setzen, führt Claude Code den Befehl an zwei Punkten aus und verwendet die Ausgabe eines Laufs für bis zu 60 Sekunden erneut:

* Vor jedem Abrufen von diesem Marketplace `marketplace.json`, einschließlich einer späteren Aktualisierung. Claude Code sendet die gedruckten Header mit diesem Abrufen.
* Vor jedem Plugin-Archiv-Download auf dem Ursprung der Marketplace-URL, was dasselbe Schema, Host und Port bedeutet. Claude Code sendet die Ausgabe mit diesem Download, und kein anderer Download erhält die Header.

Claude Code ignoriert jeden `headersHelper`, der in der `.claude/settings.json` oder `.claude/settings.local.json` eines Verzeichnisses gesetzt ist, das Sie mit [`--add-dir`](/docs/de/permissions#what-runs-before-you-trust-a-folder) hinzufügen, auf einer `url` Quelle und auf einem Inline-Plugin-Eintrag gleichermaßen, und sendet nur die festen `headers`, die in dieser Datei gesetzt sind. [Wie Benutzer einen headersHelper-Befehl akzeptieren](/docs/de/plugins/host-marketplace#how-users-accept-a-headershelper-command) behandelt die anderen Einstellungsdateien.

Plugins, die in einer `settings` Quelle aufgelistet sind, müssen externe Quellen wie GitHub oder npm referenzieren, und der `name` muss dem Marketplace-Schlüssel entsprechen. Sie aktivieren immer noch jedes Plugin separat in `enabledPlugins`. Dieses Beispiel deklariert ein Plugin inline:

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

Ein Plugin-Eintrag unter `source: 'settings'`, dessen eigene `source` ein [`archive`](/docs/de/plugins/marketplace-reference#archive-plugin-source) ist, kann `headers` für den Archiv-Download setzen. Wenn der Wert, den Sie in `headers` setzen würden, kurzlebig ist, wie ein Token, den Ihre Registry auf Anfrage prägt, setzen Sie stattdessen einen `headersHelper` Befehl. Ein Eintrag kann beide setzen. Beide Felder erfordern Claude Code v2.1.238 oder später.

Claude Code sendet die `headers` des Eintrags und alles, was der Befehl druckt, mit dem Archiv-Download dieses Plugins und mit keinem anderen Download. Claude Code führt den Befehl nur aus, wenn ein Benutzer [dieses eine Plugin selbst installiert oder aktualisiert](/docs/de/plugins/host-marketplace#how-users-accept-a-headershelper-command). Drei weitere Regeln hängen davon ab, welche Datei den Eintrag hält:

* **`strict`**: anders als ein Eintrag in einem Marketplace `marketplace.json` benötigt ein Eintrag in Einstellungen kein `"strict": false`, weil eine Einstellungsdatei keine Manifest-Felder zum Inline-Einfügen trägt. Siehe [Strict Mode](/docs/de/plugins/marketplace-reference#strict-mode).
* **Ordner-Vertrauen**: für einen Eintrag in der `.claude/settings.json` oder `.claude/settings.local.json` eines Projekts führt Claude Code den Befehl nur aus, nachdem der Benutzer auch [diesen Ordner vertraut hat](/docs/de/permissions#what-runs-before-you-trust-a-folder).
* **Header-Filter**: Claude Code verwirft [Request-Routing- und Client-Identitäts-Header-Namen](/docs/de/plugins/host-marketplace#when-claude-code-skips-a-headershelper-command-or-drops-its-output) aus einem Eintrag in der `.claude/settings.json` oder `.claude/settings.local.json` eines Projekts, weil ein Repository diese Dateien liefern kann. Claude Code wendet denselben Filter auf einen Katalog-Eintrag und auf einen Eintrag in einem `--add-dir` Verzeichnis-Einstellungen an, und keinen Filter auf einen Eintrag in Ihren Benutzereinstellungen, einer `--settings` Datei oder verwalteten Einstellungen.

<h4 id="marketplace-key-aliases">
  Marketplace-Schlüssel-Aliase
</h4>

Auf Claude Code v2.1.232 oder später können Sie `extraKnownMarketplaces` als `additionalMarketplaces` und `strictKnownMarketplaces` als `allowedMarketplaces` schreiben. Claude Code behandelt jeden Alias wie folgt:

* Frühere Versionen ignorieren den Alias, daher behalten Sie die kanonische Schreibweise in einer Datei, die auch ältere Versionen lesen, wie eine verwaltete Einstellungsdatei für eine Flotte mit gemischten Claude Code Versionen.
* In jeder Einstellungsdatei, die den kanonischen Schlüssel akzeptiert, liest Claude Code den Alias genau wie den kanonischen Schlüssel.
* Claude Code kann `additionalMarketplaces` zu `extraKnownMarketplaces` umschreiben, wenn es die Datei aktualisiert.
* Wenn Sie beide Schreibweisen in einer Datei setzen, verwendet Claude Code den kanonischen Wert und ignoriert den Alias.

<h3 id="pluginconfigs">
  `pluginConfigs`
</h3>

Speichern Sie die nicht-sensiblen Antworten, die Sie einem Plugin [`userConfig`](/docs/de/plugins/manifest-reference#user-configuration) Konfigurationsdialog geben, gekennzeichnet durch Plugin-ID. Claude Code schreibt diesen Schlüssel in Ihre Benutzereinstellungen, wenn Sie den Dialog ausfüllen, daher müssen Sie ihn nicht von Hand bearbeiten. Claude Code speichert sensible Optionen stattdessen im macOS Keychain, fällt auf `~/.claude/.credentials.json` zurück, wenn der Keychain den Schreibvorgang ablehnt; auf Plattformen ohne einen unterstützten Keychain speichert es sie in `~/.claude/.credentials.json`.

* **Bereich**: [`User or managed`](#scopes)
* **Typ**: Objekt, das eine Plugin-ID auf ein Objekt mit einem `options` Feld abbildet, das jeden Optionsnamen auf einen String, eine Zahl, einen Boolean oder ein Array von Strings abbildet, und ein optionales `mcpServers` Feld mit Pro-Server-Benutzer-Konfigurationswerten in derselben Form
* **Standard**: nicht gesetzt

Dieses Beispiel speichert die `api_endpoint` Option für das `deployer` Plugin aus `acme-tools`:

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

Integrierte Plugins speichern ihre Optionen unter demselben Schlüssel mit einem `@builtin` Suffix. Zum Beispiel ist die [**Projektanweisungen**](/docs/de/memory#choose-which-instruction-files-load) Einstellung, die kontrolliert, ob Claude Code `AGENTS.md` Dateien liest, `pluginConfigs["agents-md@builtin"].options.instructionFiles`.

Claude Code ignoriert Projekt- und lokale Einträge, weil es diese Werte in Plugin-Hook-, MCP- und LSP-Konfigurationen ersetzt, und ein geklontes Repository darf sie nicht liefern. Vor v2.1.207 wurden auch Projekt- und lokale Einstellungen gelesen.

<h2 id="mcp">
  MCP
</h2>

Steuern Sie, mit welchen MCP-Servern Claude Code sich verbindet, und welche eine Organisation zulässt. Siehe [Mit externen Tools über MCP verbinden](/docs/de/mcp) und [Verwaltete MCP-Konfiguration](/docs/de/managed-mcp).

<h3 id="allowallclaudeaimcps">
  `allowAllClaudeAiMcps`
</h3>

Laden Sie die [claude.ai-Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai), die Claude Code selbst abruft, zusammen mit einer bereitgestellten `managed-mcp.json`. Ohne diesen Schlüssel übernimmt `managed-mcp.json` die ausschließliche Kontrolle über MCP-Server und unterdrückt diese Konnektoren.

* **Bereich**: [`Managed`](#scopes). Benutzer können Konnektoren, deren ausschließliche Kontrolle unterdrückt hat, nicht erneut aktivieren.
* **Typ**: Boolean
  * `true`: Claude Code lädt die claude.ai-Konnektoren zusammen mit einer bereitgestellten `managed-mcp.json`
  * `false`: eine bereitgestellte `managed-mcp.json` übernimmt die ausschließliche Kontrolle über MCP-Server und unterdrückt die claude.ai-Konnektoren, [die Claude Code selbst abruft](/docs/de/mcp#how-connectors-reach-claude-code)
* **Standard**: `false`, daher unterdrückt eine bereitgestellte `managed-mcp.json` die claude.ai-Konnektoren, die Claude Code selbst abruft

```json managed-settings.json theme={null}
{
  "allowAllClaudeAiMcps": true
}
```

[`allowedMcpServers`](#allowedmcpservers) und [`deniedMcpServers`](#deniedmcpservers) gelten weiterhin für die Konnektoren, die dieser Schlüssel lädt. Konnektoren, die an eine [Cloud-Sitzung](/docs/de/claude-code-on-the-web) geliefert werden, deren Host eine `managed-mcp.json` trägt, wie z. B. einen selbstgehosteten Runner, bleiben unterdrückt. Siehe [claude.ai-Konnektoren neben dem verwalteten Satz zulassen](/docs/de/managed-mcp#allow-claude-ai-connectors-alongside-the-managed-set).

<h3 id="allowedmcpservers">
  `allowedMcpServers`
</h3>

Erstellen Sie eine Zulassungsliste der MCP-Server, die Personen hinzufügen können. Claude Code blockiert jeden Server, der nicht mit einem Eintrag übereinstimmt, überall dort, wo er definiert ist, einschließlich Plugin-Server, Server, die mit `--mcp-config` übergeben werden, und Server von claude.ai.

Integrierte Server wie Claude in Chrome, der `ide`-Server, mit dem Claude Code sich in einer laufenden [VS Code](/docs/de/vs-code#the-built-in-ide-mcp-server)- oder [JetBrains](/docs/de/jetbrains#the-built-in-ide-mcp-server)-IDE verbindet, und Server, die die CLI selbst konfiguriert, sind von der Zulassungsliste ausgenommen, und die Ablehnungsliste gilt weiterhin für sie. In-Process-`type: "sdk"`-Server sind von beiden Listen ausgenommen; die [App, die die Sitzung gestartet hat](/docs/de/mcp#how-connectors-reach-claude-code), registriert sie.

Server, die Ihre Organisation bereitstellt, sind auch von der Zulassungsliste ausgenommen, und die Ablehnungsliste gilt weiterhin für sie. Die Ausnahme deckt jeden [`managedMcpServers`](#managedmcpservers)-Eintrag ab, und jeden [`managed-mcp.json`](/docs/de/managed-mcp#exclusive-control-with-managed-mcp-json)-Eintrag, dessen Werte keine `${VAR}`-Erweiterung verwenden. Siehe [Wie ein Server bewertet wird](/docs/de/managed-mcp#how-a-server-is-evaluated) für die vollständige Prüfreihenfolge. Vor v2.1.259 mussten Server aus `managed-mcp.json` auch übereinstimmen.

* **Bereich**: [`Any file`](#scopes). Einträge aus jeder Datei werden in eine Zulassungsliste zusammengeführt, es sei denn, [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly) ist gesetzt. Stellen Sie es in verwalteten Einstellungen bereit, um es durchzusetzen.
* **Typ**: Array von Objekten, jedes mit genau einem Schlüssel: `serverName`, ein String, der auf Buchstaben, Zahlen, Bindestriche und Unterstriche beschränkt ist; `serverCommand`, ein Array des Befehls und seiner Argumente, die genau übereinstimmen; oder `serverUrl`, ein URL-Muster mit `*`-Platzhaltern
* **Standard**: nicht gesetzt, daher ist jeder Server zulässig; ein leeres Array blockiert jeden Server, den Benutzer hinzufügen

Dieses Beispiel erlaubt nur den stdio-Server, den der aufgelistete `npx`-Befehl startet:

```json settings.json theme={null}
{
  "allowedMcpServers": [
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem"] }
  ]
}
```

Ein [`deniedMcpServers`](#deniedmcpservers)-Eintrag hat Vorrang, daher wird ein Server auf beiden Listen blockiert. Sobald die Liste einen `serverCommand`-Eintrag enthält, muss ein stdio-Server mit einem `serverCommand`-Eintrag übereinstimmen, und sobald sie einen `serverUrl`-Eintrag enthält, muss ein Remote-Server mit einem `serverUrl`-Eintrag übereinstimmen: eine `serverName`-Übereinstimmung lässt diese Art von Server nicht mehr zu. Siehe [Richtlinienbasierte Kontrolle mit Zulassungs- und Ablehnungslisten](/docs/de/managed-mcp#policy-based-control-with-allowlists-and-denylists).

<h3 id="allowmanagedmcpserversonly">
  `allowManagedMcpServersOnly`
</h3>

Machen Sie die verwaltete Zulassungsliste zur einzigen, die gilt. Claude Code liest dann [`allowedMcpServers`](#allowedmcpservers) nur aus verwalteten Einstellungen und ignoriert Zulassungslisten in Benutzer-, Projekt- und lokalen Einstellungen; [`deniedMcpServers`](#deniedmcpservers) wird weiterhin aus jedem Einstellungsbereich zusammengeführt, daher können Benutzer weiterhin Server für sich selbst blockieren. Administratoren setzen es so, dass die eigenen Einstellungen eines Benutzers nicht erweitern können, was die verwaltete Zulassungsliste zulässt.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code liest `allowedMcpServers` nur aus verwalteten Einstellungen und ignoriert Zulassungslisten in Benutzer-, Projekt- und lokalen Einstellungen
  * `false`: Zulassungslisten aus jedem Einstellungsbereich werden zusammengeführt
* **Standard**: `false`, daher werden Zulassungslisten aus jedem Einstellungsbereich zusammengeführt

Dieses Beispiel sperrt die Zulassungsliste auf verwaltete Einstellungen und erlaubt nur den Server namens `github`:

```json managed-settings.json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverName": "github" }
  ]
}
```

Benutzer können weiterhin ihre eigenen MCP-Server hinzufügen; nur Server, die mit der verwalteten Zulassungsliste übereinstimmen, werden geladen. Siehe [Zulassungsliste auf verwaltete Einstellungen beschränken](/docs/de/managed-mcp#restrict-the-allowlist-to-managed-settings-only).

<h3 id="deniedmcpservers">
  `deniedMcpServers`
</h3>

Blockieren Sie bestimmte MCP-Server. Claude Code weigert sich, einen übereinstimmenden Server zu laden, überall dort, wo er definiert ist, einschließlich Plugin-Server, Server, die mit `--mcp-config` übergeben werden, Server aus `managed-mcp.json`, Server aus [`managedMcpServers`](#managedmcpservers), und die claude.ai-Konnektoren, [die es selbst abruft](/docs/de/mcp#how-connectors-reach-claude-code). In-Process-`type: "sdk"`-Server sind ausgenommen; die App, die die Sitzung gestartet hat, registriert sie.

* **Bereich**: [`Any file`](#scopes). Einträge aus jeder Datei werden in eine Ablehnungsliste zusammengeführt, und [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly) ändert das nicht. Stellen Sie es in verwalteten Einstellungen bereit, um es durchzusetzen.
* **Typ**: Array von Objekten, jedes mit genau einem Schlüssel: `serverName`, ein String, daher funktioniert der Anzeigename eines claude.ai-Konnektors wie `"claude.ai Slack"`; `serverCommand`, ein Array des Befehls und seiner Argumente, die genau übereinstimmen; oder `serverUrl`, ein URL-Muster mit `*`-Platzhaltern
* **Standard**: nicht gesetzt, daher wird kein Server blockiert; ein leeres Array blockiert auch nichts

```json settings.json theme={null}
{
  "deniedMcpServers": [
    { "serverName": "filesystem" }
  ]
}
```

Die Ablehnungsliste hat Vorrang vor [`allowedMcpServers`](#allowedmcpservers), daher wird ein Server auf beiden Listen blockiert. Siehe [Richtlinienbasierte Kontrolle mit Zulassungs- und Ablehnungslisten](/docs/de/managed-mcp#policy-based-control-with-allowlists-and-denylists).

<h3 id="disableclaudeaiconnectors">
  `disableClaudeAiConnectors`
</h3>

Schalten Sie die [claude.ai-MCP-Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai) aus, [die Claude Code selbst abruft](/docs/de/mcp#how-connectors-reach-claude-code), sodass es sie weder abruft noch verbindet. Ein `true` in einer beliebigen Einstellungsdatei gilt: eine eingecheckte Projekt-`.claude/settings.json` kann ein Repository von diesen Konnektoren abmelden, aber ein Projekt-Level-`false` kann ein Benutzer- oder verwaltetes Level-`true` nicht überschreiben.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code ruft diese Konnektoren weder ab noch verbindet sie
  * `false`: dasselbe wie nicht gesetzt; Claude Code ruft Ihre Konnektoren ab, es sei denn, eine andere Einstellungsdatei oder `ENABLE_CLAUDEAI_MCP_SERVERS` schaltet sie aus
* **Standard**: `false`, daher ruft Claude Code Ihre Konnektoren ab
* **Pro-Sitzungs-Überschreibungen**: [`ENABLE_CLAUDEAI_MCP_SERVERS`](/docs/de/env-vars) auf `false` gesetzt schaltet Konnektoren für eine Sitzung aus; welcher der beiden sie ausschaltet, der andere kann sie nicht wieder einschalten

```json settings.json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Server, die Sie explizit mit `--mcp-config` übergeben, sind nicht betroffen. Um einzelne Konnektoren statt aller zu blockieren, verwenden Sie [`deniedMcpServers`](#deniedmcpservers). Siehe [claude.ai-Konnektoren deaktivieren](/docs/de/mcp#disable-claude-ai-connectors).

<h3 id="disabledmcpjsonservers">
  `disabledMcpjsonServers`
</h3>

Lehnen Sie bestimmte Server ab, die in der `.mcp.json`-Datei eines Projekts definiert sind, damit Claude Code sich nie mit ihnen verbindet oder Sie auffordert, sie zu genehmigen. Eine Ablehnung in einer beliebigen Einstellungsdatei gilt, einschließlich einer Projekt-`.claude/settings.json`, die in das Repository eingecheckt ist.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Strings, die Servernamen, wie sie in `.mcp.json` erscheinen
* **Standard**: nicht gesetzt

```json settings.json theme={null}
{
  "disabledMcpjsonServers": ["filesystem"]
}
```

Claude Code schreibt diesen Schlüssel in `.claude/settings.local.json`, wenn Sie einen Server im Genehmigungsdialog ablehnen. `claude mcp get <name>` zeigt einen abgelehnten Server als `✘ Rejected (see disabledMcpjsonServers in settings)` an. Ablehnung hat Vorrang vor [`enabledMcpjsonServers`](#enabledmcpjsonservers) und [`enableAllProjectMcpServers`](#enableallprojectmcpservers).

<h3 id="enableallprojectmcpservers">
  `enableAllProjectMcpServers`
</h3>

Genehmigen Sie jeden MCP-Server, der in Projekt-`.mcp.json`-Dateien definiert ist, ohne eine Eingabeaufforderung. Claude Code schreibt diesen Schlüssel in `.claude/settings.local.json`, wenn Sie wählen, alle Server im Genehmigungsdialog zu genehmigen.

* **Bereich**: [`Any file`](#scopes). In einem Ordner, dessen Vertrauensdialog Sie nicht akzeptiert haben, berücksichtigt Claude Code ihn aus Benutzereinstellungen, verwalteten Einstellungen und `--settings` und ignoriert ihn in der gemeinsamen Projektdatei, sowohl in der Sitzung als auch für `claude mcp list` und `claude mcp get`; [Projektserver-Genehmigungen und Workspace-Vertrauen](/docs/de/mcp#project-server-approvals-and-workspace-trust) sagt, wann eine nicht nachverfollte `.claude/settings.local.json` auch zählt.
* **Typ**: Boolean
  * `true`: Claude Code genehmigt jeden MCP-Server, der in Projekt-`.mcp.json`-Dateien definiert ist, ohne eine Eingabeaufforderung
  * `false`: Claude Code fordert Sie auf, jeden Server zu genehmigen. In einem vertrauenswürdigen Ordner überschreibt ein `false` in einer Datei mit höherer Priorität ein `true` in einer niedrigeren; in einem Ordner, dem Sie nicht vertrauen, ist ein `true` in einer beliebigen berücksichtigten Datei ausreichend
* **Standard**: nicht gesetzt, daher fordert Claude Code Sie auf, jeden Server zu genehmigen

```json settings.json theme={null}
{
  "enableAllProjectMcpServers": true
}
```

Ein [`disabledMcpjsonServers`](#disabledmcpjsonservers)-Eintrag lehnt einen Server weiterhin ab.

<h3 id="enabledmcpjsonservers">
  `enabledMcpjsonServers`
</h3>

Genehmigen Sie bestimmte Server, die in Projekt-`.mcp.json`-Dateien definiert sind, damit Claude Code sich mit ihnen verbindet, ohne zu fragen. Claude Code schreibt diesen Schlüssel in `.claude/settings.local.json`, wenn Sie einen Server im Genehmigungsdialog genehmigen.

* **Bereich**: [`Any file`](#scopes). In einem Ordner, dessen Vertrauensdialog Sie nicht akzeptiert haben, berücksichtigt Claude Code ihn aus Benutzereinstellungen, verwalteten Einstellungen und `--settings` und ignoriert ihn in der gemeinsamen Projektdatei, sowohl in der Sitzung als auch für `claude mcp list` und `claude mcp get`; [Projektserver-Genehmigungen und Workspace-Vertrauen](/docs/de/mcp#project-server-approvals-and-workspace-trust) sagt, wann eine nicht nachverfollte `.claude/settings.local.json` auch zählt.
* **Typ**: Array von Strings, die Servernamen, wie sie in `.mcp.json` erscheinen
* **Standard**: nicht gesetzt

Dieses Beispiel genehmigt die Server `memory` und `github` aus der `.mcp.json` des Projekts:

```json settings.json theme={null}
{
  "enabledMcpjsonServers": ["memory", "github"]
}
```

Ein [`disabledMcpjsonServers`](#disabledmcpjsonservers)-Eintrag lehnt einen Server weiterhin ab.

<h3 id="managedmcpservers">
  `managedMcpServers`
</h3>

Stellen Sie Remote-MCP-Server aus verwalteten Einstellungen für jeden Benutzer bereit. Benutzer behalten die Server, die sie selbst hinzufügen, und können die Server, die Sie bereitstellen, nicht bearbeiten oder entfernen. Erfordert Claude Code v2.1.259 oder später.

* **Bereich**: [`Managed`](#scopes). Claude Code verwirft den Schlüssel mit einer Warnung in Benutzer-, Projekt- und lokalen Einstellungen und liest ihn nicht in der Code-Registerkarte der Claude Desktop-App bei einer Drittanbieter-Bereitstellung oder in den Cowork-Sitzungen der App, wo Claude Desktop die MCP-Server dieser Sitzungen selbst bereitstellt und sperrt.
* **Typ**: Objekt, das nach Servernamen verschlüsselt ist. Jeder Eintrag hat die `.mcp.json`-Form für einen `http`- oder `sse`-Server: eine erforderliche `https://` `url` und optional `headers`, `oauth` und die anderen HTTP- und SSE-Optionen. Claude Code verwirft Einträge, die die Validierung nicht bestehen, und [Was ein Eintrag enthalten kann](/docs/de/managed-mcp#what-an-entry-can-contain) listet die Bedingungen auf
* **Standard**: nicht gesetzt, daher stellen verwaltete Einstellungen keine Server bereit

Dieses Beispiel stellt einen HTTP-Server namens `search` bereit:

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

Für Vorrang, wie bereitgestellte Server mit `managed-mcp.json` und den Zulassungs- und Ablehnungslisten kombiniert werden, und was Benutzer sehen, siehe [Server durch verwaltete Einstellungen bereitstellen](/docs/de/managed-mcp#provide-servers-through-managed-settings).

<h2 id="agents-sessions-and-worktrees">
  Agenten, Sitzungen und Worktrees
</h2>

Legen Sie den Standard-Agenten fest, steuern Sie Teamkollegen und sitzungsübergreifendes Messaging, und konfigurieren Sie Worktrees. Siehe [Subagenten](/docs/de/sub-agents) und [Worktrees](/docs/de/worktrees).

<h3 id="agent">
  `agent`
</h3>

Führen Sie den Haupt-Thread als benannten [Subagenten](/docs/de/sub-agents#invoke-subagents-explicitly) aus, damit Claude Code den System-Prompt, die Tool-Einschränkungen und das Modell dieses Subagenten auf Ihre Sitzung anwendet. Derselbe Schlüssel legt den Standard-Agenten für Sitzungen fest, die Sie von `claude agents` aus versenden.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: String, der Name eines integrierten oder benutzerdefinierten Agenten
* **Standard**: nicht gesetzt, daher wird der Haupt-Thread als Standard-Agent von Claude Code ausgeführt
* **Sitzungsspezifische Außerkraftsetzungen**: `--agent` hat Vorrang vor diesem Schlüssel für eine Sitzung

```json settings.json theme={null}
{
  "agent": "code-reviewer"
}
```

Die eigene `settings.json` eines Plugins kann diesen Schlüssel auch bereitstellen; siehe [Versenden Sie Standard-Einstellungen mit Ihrem Plugin](/docs/de/plugins/components#default-settings).

<h3 id="crosssessioninbound">
  `crossSessionInbound`
</h3>

Wählen Sie, was diese Sitzung mit [Nachrichten tut, die von Ihren anderen Claude Code-Sitzungen ankommen](/docs/de/cross-session-messaging#control-inbound-messages). Wenn kein Wert zutrifft, entscheidet Claude Code pro Nachricht anhand der Berechtigungsmodus-Klassen der beiden Sitzungen. Erfordert Claude Code v2.1.224 oder später.

* **Bereich**: [`Any file`](#scopes). Ein Projekt- oder lokaler Wert gilt nur, wenn er strenger ist als der Wert, den verwaltete Einstellungen, das Flag `--settings` oder Benutzereinstellungen vorgeben.
* **Typ**: String, einer von:
  * `"accept"`: Claude Code liefert die Nachricht an Claude
  * `"hold"`: Claude Code zeigt einen Hinweis für die Nachricht an, ohne sie zu liefern
  * `"refuse"`: Claude Code verwirft die Nachricht
* **Standard**: nicht gesetzt, daher entscheidet Claude Code pro Nachricht

```json settings.json theme={null}
{
  "crossSessionInbound": "hold"
}
```

Claude Code liest zuerst verwaltete Einstellungen, dann das Flag `--settings`, dann Benutzereinstellungen und wendet den ersten gefundenen Wert an. `refuse` ist strenger als `hold`, und `hold` ist strenger als `accept`. Wenn keine der vertrauenswürdigen Quellen einen Wert setzt, gilt ein Projekt- oder lokales `hold` oder `refuse` immer noch und ersetzt den Standard pro Nachricht. In Sitzungen mit sitzungsübergreifendem Messaging wird dieser Schlüssel in `/config` als **Messages from your other sessions** angezeigt, was ihn in Benutzereinstellungen schreibt; die Zeile erfordert Claude Code v2.1.232 oder später, und Claude Code blendet sie aus, während das Flag `--settings` oder verwaltete Einstellungen den Schlüssel setzen.

Claude Code [warnt](/docs/de/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse), wenn Sie einen Wert setzen, den es nicht erkennt. Während dieser Wert in einer Benutzer-, Projekt-, lokalen oder `--settings`-Datei vorhanden ist, hält Claude Code eingehende Nachrichten, auch wenn eine Quelle mit höherer Priorität `accept` setzt. Ein `refuse`, das eine andere Quelle setzt, gilt immer noch. Beheben oder entfernen Sie den Wert, um den Hold zu löschen.

Wenn der nicht erkannte Wert in [verwalteten Einstellungen](/docs/de/managed-settings) vorhanden ist, behandelt Claude Code ihn stattdessen als `refuse`, bis ein Administrator ihn behebt. Vor v2.1.248 ignorierte Claude Code einen nicht erkannten Wert ohne Warnung.

<h3 id="disableagentview">
  `disableAgentView`
</h3>

Schalten Sie [Hintergrund-Agenten und Agent-Ansicht](/docs/de/agent-view) aus: `claude agents`, `--bg`, `/background` und den On-Demand-Supervisor. Legen Sie es in [verwalteten Einstellungen](/docs/de/managed-settings) fest, um es für eine Organisation zu erzwingen.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code schaltet `claude agents`, `--bg`, `/background` und den On-Demand-Supervisor aus
  * `false`: Agent-Ansicht ist verfügbar
* **Standard**: nicht gesetzt, daher ist Agent-Ansicht verfügbar
* **Sitzungsspezifische Außerkraftsetzungen**: [`CLAUDE_CODE_DISABLE_AGENT_VIEW`](/docs/de/env-vars) schaltet Agent-Ansicht für eine Sitzung aus; welcher der beiden sie ausschaltet, der andere kann sie nicht wieder einschalten

```json settings.json theme={null}
{
  "disableAgentView": true
}
```

<h3 id="isolatepeermachines">
  `isolatePeerMachines`
</h3>

Erfordern Sie Ihre ausdrückliche Genehmigung, bevor Claude's `SendMessage` eine Ihrer Sitzungen über diese Maschine hinaus erreicht; siehe [Genehmigung für sitzungsübergreifende Nachrichten erforderlich](/docs/de/cross-session-messaging#require-approval-for-cross-machine-messages). Die Genehmigungsaufforderung wird auch im [`bypassPermissions`-Modus](/docs/de/permission-modes#skip-all-checks-with-bypasspermissions-mode) angezeigt.

* **Bereich**: [`Any file`](#scopes). Ein `true` aus einem beliebigen Bereich gilt, daher kann eine eingecheckte Projektdatei die Anforderung einschalten, aber nicht ausschalten.
* **Typ**: Boolean
  * `true`: Claude Code fragt nach Ihrer Genehmigung, bevor Claude's `SendMessage` eine Ihrer Sitzungen über diese Maschine hinaus erreicht
  * `false`: sitzungsübergreifende Nachrichten werden nicht angefordert
* **Standard**: nicht gesetzt, daher werden sitzungsübergreifende Nachrichten nicht angefordert

```json settings.json theme={null}
{
  "isolatePeerMachines": true
}
```

Die sitzungsübergreifende `SendMessage`-Genehmigung erfordert Claude Code v2.1.224 oder später.

<h3 id="processwrapper">
  `processWrapper`
</h3>

Auf macOS und Linux platzieren Sie einen Corporate-Launcher-Befehl vor den [Hintergrund-Prozessen, die Claude Code startet](/docs/de/corporate-launcher#what-the-launcher-covers). Claude Code führt den Launcher mit seiner eigenen Befehlszeile angehängt aus, daher muss der Launcher in Claude Code ausgeführt werden; siehe [Führen Sie Claude Code hinter einem Corporate Launcher aus](/docs/de/corporate-launcher) für den Launcher-Vertrag. Erfordert Claude Code v2.1.210 oder später.

* **Bereich**: [`User or managed`](#scopes)
* **Typ**: String, der Launcher-Befehl als argv-Präfix, z. B. ein absoluter Pfad mit optionalen Argumenten
* **Standard**: nicht gesetzt, daher starten Hintergrund-Prozesse unverpackt
* **Sitzungsspezifische Außerkraftsetzungen**: [`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/de/env-vars) hat Vorrang vor diesem Schlüssel für eine Sitzung

```json settings.json theme={null}
{
  "processWrapper": "/opt/corp/launcher --profile claude"
}
```

Claude Code ignoriert den Launcher unter Windows und startet jeden Prozess unverpackt. Erfordert Claude Code v2.1.210 oder später.

<h3 id="teammatemode">
  `teammateMode`
</h3>

Wählen Sie, wo Claude Code [Agent-Team](/docs/de/agent-teams)-Teamkollegen anzeigt: in Ihrem Haupt-Terminal-Bereich oder in geteilten Bereichen, wenn Ihr Terminal diese unterstützt. Siehe [Wählen Sie einen Anzeigemodus](/docs/de/agent-teams#choose-a-display-mode).

* **Bereich**: [`Any file`](#scopes). Claude Code liest auch einen Wert, der von älteren Versionen in `~/.claude.json` hinterlassen wurde.
* **Typ**: String, einer von:
  * `"in-process"`: Teamkollegen werden in Ihrem Haupt-Terminal-Bereich ausgeführt
  * `"auto"`: geteilte Bereiche, wenn Sie sich in tmux befinden, oder in iTerm2 mit `it2` auf Ihrem `PATH` oder tmux installiert; ansonsten in-process
  * `"tmux"`: geteilte Bereiche mit tmux oder iTerm2, erkannt von Ihrem Terminal
  * `"iterm2"`: iTerm2 native geteilte Bereiche über die `it2` CLI
* **Standard**: `"in-process"`
* **Sitzungsspezifische Außerkraftsetzungen**: `--teammate-mode` hat Vorrang vor diesem Schlüssel für eine Sitzung

```json settings.json theme={null}
{
  "teammateMode": "auto"
}
```

<span id="worktree-settings" />

<h3 id="worktree">
  `worktree`
</h3>

Konfigurieren Sie, wie Claude Code [Git-Worktrees](/docs/de/worktrees) für `--worktree`, das Tool `EnterWorktree` und isolierte Subagenten und Hintergrund-Sitzungen erstellt und verwaltet.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Objekt mit `baseRef`, `symlinkDirectories`, `sparsePaths` und `bgIsolation`
* **Standard**: nicht gesetzt

Dieses Beispiel verzweigt neue Worktrees von Ihrem aktuellen `HEAD` und erstellt Symlinks für `node_modules` in jedem:

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head",
    "symlinkDirectories": ["node_modules"]
  }
}
```

Um gitignorierte Dateien wie `.env` in neue Worktrees zu kopieren, fügen Sie stattdessen eine [`.worktreeinclude`-Datei](/docs/de/worktrees#copy-gitignored-files-into-worktrees) zu Ihrem Projekt-Root hinzu.

<h3 id="worktree-baseref">
  `worktree.baseRef`
</h3>

Wählen Sie, von welchem Ref neue Worktrees verzweigt werden. `"fresh"` verzweigt von `origin/<default-branch>` für einen sauberen Baum, der dem Remote entspricht; `"head"` verzweigt von Ihrem aktuellen lokalen `HEAD`, daher sind nicht gepushte Commits und Feature-Branch-Status im Worktree vorhanden.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: String, einer von:
  * `"fresh"`: neue Worktrees verzweigen von `origin/<default-branch>`
  * `"head"`: neue Worktrees verzweigen von Ihrem aktuellen lokalen `HEAD`, einschließlich nicht gepushter Commits
* **Standard**: `"fresh"`

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

Innerhalb eines verknüpften Worktrees wird `"head"` zu diesem Worktree's `HEAD` aufgelöst, nicht zum `HEAD` des Haupt-Checkouts.

<h3 id="worktree-symlinkdirectories">
  `worktree.symlinkDirectories`
</h3>

Erstellen Sie Symlinks für Verzeichnisse aus dem Haupt-Repository in jeden Worktree, damit Sie große Verzeichnisse nicht auf der Festplatte duplizieren.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Strings, Verzeichnispfade relativ zum Repository-Root
* **Standard**: nicht gesetzt, daher erstellt Claude Code keine Symlinks für Verzeichnisse

Dieses Beispiel erstellt Symlinks für `node_modules` und `.cache` aus dem Haupt-Repository in jeden neuen Worktree:

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

Checken Sie nur die aufgelisteten Verzeichnisse in jedem Worktree über Git Sparse-Checkout aus. Claude Code schreibt nur diese Verzeichnisse plus Root-Level-Dateien auf die Festplatte, was in großen Monorepos schneller ist; siehe [Checken Sie nur die Verzeichnisse aus, die Sie benötigen](/docs/de/large-codebases#check-out-only-the-directories-you-need).

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Array von Strings, Verzeichnispfade relativ zum Repository-Root
* **Standard**: nicht gesetzt, daher checkt jeder Worktree den ganzen Baum aus

Dieses Beispiel checkt nur `packages/my-app` und `shared/utils` plus Root-Level-Dateien in jedem Worktree aus:

```json settings.json theme={null}
{
  "worktree": {
    "sparsePaths": ["packages/my-app", "shared/utils"]
  }
}
```

Während ein Sparse-Worktree vorhanden ist, aktiviert Git `extensions.worktreeConfig` in der gemeinsamen `.git/config` des Repositories.

<h3 id="worktree-bgisolation">
  `worktree.bgIsolation`
</h3>

Wählen Sie, wie [Hintergrund-Sitzungen](/docs/de/agent-view#how-file-edits-are-isolated) ihre Datei-Änderungen isolieren. Mit `"worktree"` blockiert Claude Code `Edit` und `Write` im Haupt-Checkout, bis die Sitzung `EnterWorktree` aufruft; mit `"none"` bearbeiten Hintergrund-Jobs die Working Copy direkt. Setzen Sie `"none"` für ein Repository, in dem Git-Worktrees unpraktisch sind.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: String, einer von:
  * `"worktree"`: Claude Code blockiert `Edit` und `Write` im Haupt-Checkout, bis die Sitzung `EnterWorktree` aufruft
  * `"none"`: Hintergrund-Jobs bearbeiten die Working Copy direkt
* **Standard**: `"worktree"`

```json settings.json theme={null}
{
  "worktree": {
    "bgIsolation": "none"
  }
}
```

Außerhalb eines Git-Repositories gibt ein [`WorktreeCreate`-Hook](/docs/de/worktrees#non-git-version-control), der fehlschlägt, die Blockade frei, damit die Sitzung das Working Directory an Ort und Stelle bearbeiten kann; diese Freigabe erfordert Claude Code v2.1.203 oder später.

<h2 id="remote-desktop-and-notifications">
  Remote, Desktop und Benachrichtigungen
</h2>

Konfigurieren Sie Remote Control, Cloud-Umgebungen, die Desktop-App und die Benachrichtigungen, die Claude Code sendet, wenn es Sie braucht. Siehe [Remote Control](/docs/de/remote-control).

<h3 id="agentpushnotifenabled">
  `agentPushNotifEnabled`
</h3>

Erlauben Sie Claude, eine Push-Benachrichtigung an Ihr Telefon zu senden, wenn es entscheidet, dass dies sinnvoll ist, beispielsweise wenn eine lange Aufgabe abgeschlossen ist. Claude Code synchronisiert diese Wahl mit Ihrem Konto, und Push-Benachrichtigungen werden gesendet, während [Remote Control](/docs/de/remote-control) verbunden ist. Wird in `/config` als **Push wenn Claude entscheidet** angezeigt.

* **Bereich**: [`Any file`](#scopes). Claude Code liest auch einen Wert, der von älteren Versionen in `~/.claude.json` hinterlassen wurde.
* **Typ**: Boolean
  * `true`: Claude kann eine Push-Benachrichtigung an Ihr Telefon senden, wenn es entscheidet, dass dies sinnvoll ist
  * `false`: Claude sendet diese Benachrichtigungen nicht
* **Standard**: `false`

```json settings.json theme={null}
{
  "agentPushNotifEnabled": true
}
```

Siehe [Mobile Push-Benachrichtigungen](/docs/de/remote-control#mobile-push-notifications).

<h3 id="awaysummaryenabled">
  `awaySummaryEnabled`
</h3>

Zeigen Sie eine einzeilige Sitzungszusammenfassung an, wenn Sie nach einigen Minuten Abwesenheit zum Terminal zurückkehren. Setzen Sie es auf `false` oder deaktivieren Sie **Sitzungszusammenfassung** in `/config`, um die Zusammenfassung zu deaktivieren.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolean
  * `true`: Sie sehen eine einzeilige Sitzungszusammenfassung, wenn Sie nach einigen Minuten Abwesenheit zurückkehren
  * `false`: Claude Code zeigt keine Zusammenfassung an
* **Standard**: nicht gesetzt, daher ist die Zusammenfassung aktiviert
* **Sitzungsspezifische Außerkraftsetzungen**: [`CLAUDE_CODE_ENABLE_AWAY_SUMMARY`](/docs/de/env-vars) hat Vorrang vor diesem Schlüssel für eine Sitzung, in beide Richtungen

```json settings.json theme={null}
{
  "awaySummaryEnabled": false
}
```

Claude Code zeigt die Zusammenfassung niemals im nicht-interaktiven Modus an.

<h3 id="disableartifact">
  `disableArtifact`
</h3>

<Warning>
  Veraltet und ersetzt durch [`enableArtifact`](#enableartifact). Claude Code berücksichtigt weiterhin `disableArtifact: true` als gleichwertig mit `enableArtifact: false` und ignoriert `disableArtifact: false`.
</Warning>

Verwenden Sie stattdessen [`enableArtifact`](#enableartifact), um das [Artifact](/docs/de/artifacts)-Tool auszuschalten, das Sitzungsausgaben als private Webseite auf claude.ai veröffentlicht. Wenn Sie die Zeile **Artifacts** in `/config` ausschalten, schreibt Claude Code `enableArtifact` in Ihre Benutzereinstellungen und löscht diesen Schlüssel.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code schaltet das Artifact-Tool für jede Sitzung aus, auf die die Datei zutrifft, und keine andere Datei schaltet es wieder ein. Vor v2.1.242 konnte eine Datei mit höherer Priorität eine `true` einer Datei mit niedrigerer Priorität überschreiben, anstatt dass der Schlüssel als Sperre fungiert
  * `false`: ignoriert; um das Tool eingeschaltet zu lassen, entfernen Sie den Schlüssel
* **Standard**: nicht gesetzt, daher folgt das Tool Ihrer Konto-[Verfügbarkeit](/docs/de/artifacts#availability)
* **Sitzungsspezifische Außerkraftsetzungen**: [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/de/env-vars) auf `1` gesetzt schaltet das Tool für eine Sitzung aus

```json settings.json theme={null}
{
  "disableArtifact": true
}
```

[Artifacts deaktivieren](/docs/de/artifacts#disable-artifacts) listet alle Möglichkeiten auf, das Tool auszuschalten.

<h3 id="disabledeeplinkregistration">
  `disableDeepLinkRegistration`
</h3>

Verhindern Sie, dass Claude Code den `claude-cli://`-Protokoll-Handler beim Betriebssystem registriert, was es ansonsten nach dem ersten Prompt einer interaktiven Sitzung tut. [Deep Links](/docs/de/deep-links) ermöglichen es externen Tools, eine Claude Code-Sitzung mit einem vorausgefüllten Prompt zu öffnen. Setzen Sie dies in Umgebungen, in denen die Registrierung von Protokoll-Handlern eingeschränkt oder separat verwaltet wird.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: die Zeichenkette `"disable"`
* **Standard**: nicht gesetzt, daher registriert Claude Code den Handler

```json settings.json theme={null}
{
  "disableDeepLinkRegistration": "disable"
}
```

<h3 id="disabledesktoplocalsessions">
  `disableDesktopLocalSessions`
</h3>

Schalten Sie Code-Sitzungen aus, die auf dem Gerät in der [Desktop-App](/docs/de/desktop#local-sessions-on-managed-devices) ausgeführt werden, für Bereitstellungen, bei denen Entwickler auf Remote-Maschinen über SSH arbeiten sollten. In der Code-Registerkarte bleibt die **Local**-Umgebung in der Umgebungs-Dropdown-Liste, ist aber ausgegraut und kann nicht ausgewählt werden, mit einem Tooltip, das besagt, dass Ihre Organisation sie deaktiviert hat; unter Windows ist der WSL-Eintrag auf die gleiche Weise ausgegraut, obwohl ob WSL-Sitzungen auf einem verwalteten Gerät überhaupt ausgeführt werden, [separat geregelt wird](/docs/de/admin-setup#wsl-sessions-in-claude-code-desktop). Neue Sitzungen werden standardmäßig auf die erste [SSH-Verbindung](/docs/de/desktop#ssh-sessions) gesetzt, falls eine konfiguriert ist, und die App weigert sich, eine Sitzung auf dem Gerät zu starten oder fortzusetzen, einschließlich einer SSH-Verbindung zurück zur gleichen Maschine. SSH-Sitzungen zu anderen Hosts und Cloud-Sitzungen sind nicht betroffen. Die Desktop-App liest diesen Schlüssel; die Terminal-CLI ignoriert ihn. Erfordert Claude Desktop v1.37937.0 oder später.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Boolean; nur der JSON-Boolean `true` hat Auswirkungen
  * `true`: die Desktop-App bietet keine Code-Sitzungen auf dem Gerät an; vorhandene lokale Sitzungen bleiben aufgelistet, können aber nicht fortgesetzt werden
  * `false`: lokale Sitzungen bleiben verfügbar
* **Standard**: nicht gesetzt, daher sind lokale Sitzungen verfügbar

```json managed-settings.json theme={null}
{
  "disableDesktopLocalSessions": true
}
```

Die Desktop-App ignoriert jeden anderen Wert, und ein Wert, der kein Boolean ist, wie die Zeichenkette `"true"` oder `1`, protokolliert auch eine Warnung. Kombinieren Sie ihn mit [`sshConfigs`](#sshconfigs), damit Benutzer auf einer funktionierenden Verbindung landen, und mit [`sshHostAllowlist`](#sshhostallowlist), um zu begrenzen, welche Hosts sie erreichen können. Siehe [Lokale Sitzungen auf verwalteten Geräten](/docs/de/desktop#local-sessions-on-managed-devices).

Claude Desktop versorgt Code-Sitzungen mit einer Richtlinie, die sich aus Ihrer Desktop-Konfiguration ergibt, beispielsweise die Egress-Allowlist, die Dateisystem-Sandbox und die MCP-Einschränkungen in Bereitstellungen von Drittanbietern. Claude Code ignoriert diese übergeordneten Einstellungen, wenn eine [Admin-Quelle](/docs/de/managed-settings#how-claude-code-combines-managed-sources) vorhanden ist: servergesteuerte Einstellungen, eine MDM- oder Betriebssystem-Richtlinie oder eine verwaltete Einstellungsdatei. Die Bereitstellung dieses Schlüssels durch eine dieser Methoden auf einem Gerät, das zuvor keine hatte, wie in Bereitstellungen von Drittanbietern, stoppt daher die Anwendung der Desktop-abgeleiteten Richtlinien. [Lassen Sie einen Embedding-Host eine Richtlinie hinzufügen](/docs/de/managed-settings#let-an-embedding-host-add-policy) behandelt, wann übergeordnete Einstellungen noch zusammengeführt werden können; dies gilt für jeden Schlüssel, den Sie auf diese Weise bereitstellen, nicht nur diesen.

<h3 id="disableremotecontrol">
  `disableRemoteControl`
</h3>

Schalten Sie [Remote Control](/docs/de/remote-control) aus: Claude Code lehnt dann `claude remote-control`, das Flag `--remote-control`, Auto-Start und den In-Session-Toggle ab und meldet, dass die Richtlinie Ihrer Organisation es deaktiviert hat. Platzieren Sie es in [verwalteten Einstellungen](/docs/de/managed-settings) für die MDM-Durchsetzung pro Gerät.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code lehnt `claude remote-control`, das Flag `--remote-control`, Auto-Start und den In-Session-Toggle ab
  * `false`: Remote Control bleibt verfügbar
* **Standard**: `false`

```json settings.json theme={null}
{
  "disableRemoteControl": true
}
```

<h3 id="enableartifact">
  `enableArtifact`
</h3>

Schalten Sie das [Artifact](/docs/de/artifacts)-Tool aus, das Sitzungsausgaben als private Webseite auf claude.ai veröffentlicht. Wenn Sie die Zeile **Artifacts** in `/config` ausschalten, schreibt Claude Code diesen Schlüssel in Ihre Benutzereinstellungen, daher bearbeiten Sie ihn normalerweise nicht von Hand. Erfordert Claude Code v2.1.196 oder später.

* **Bereich**: [`Any file`](#scopes). Jede Datei kann das Tool ausschalten, und keine kann es wieder einschalten.
* **Typ**: Boolean
  * `false`: Claude Code schaltet das Artifact-Tool für jede Sitzung aus, auf die die Datei zutrifft
  * `true`: dasselbe wie das Weglassen des Schlüssels, da es niemals eine `false` aus einer anderen Datei, von [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/de/env-vars) oder von der [Admin-Einstellung](/docs/de/artifacts#manage-artifacts-for-your-organization) Ihrer Organisation überschreibt
* **Standard**: nicht gesetzt, daher folgt das Tool Ihrer Konto-[Verfügbarkeit](/docs/de/artifacts#availability)

```json settings.json theme={null}
{
  "enableArtifact": false
}
```

Während eine andere Quelle als Ihre eigenen Benutzereinstellungen das Tool ausgeschaltet hält, versteckt Claude Code die Zeile **Artifacts** in `/config`, da das Einschalten dort nichts ändern würde. [Artifacts deaktivieren](/docs/de/artifacts#disable-artifacts) listet alle Möglichkeiten auf, das Tool auszuschalten. Vor v2.1.242 ignorierte Claude Code diesen Schlüssel in Projekt- und lokalen Einstellungen, und eine Datei höher im [Prioritätsstapel](/docs/de/settings#settings-precedence) konnte das Tool über eine niedrigere Datei mit Aus wieder einschalten.

<h3 id="inputneedednotifenabled">
  `inputNeededNotifEnabled`
</h3>

Erhalten Sie eine Push-Benachrichtigung auf Ihrem Telefon, wenn eine Berechtigungsaufforderung oder Frage auf Ihre Eingabe wartet. Claude Code sendet diese nur, während [Remote Control](/docs/de/remote-control) verbunden ist. Wird in `/config` als **Push wenn Aktionen erforderlich sind** angezeigt.

* **Bereich**: [`Any file`](#scopes). Claude Code liest auch einen Wert, der von älteren Versionen in `~/.claude.json` hinterlassen wurde.
* **Typ**: Boolean
  * `true`: Sie erhalten eine Push-Benachrichtigung auf Ihrem Telefon, wenn eine Berechtigungsaufforderung oder Frage wartet, während Remote Control verbunden ist
  * `false`: Claude Code sendet keine solchen Benachrichtigungen
* **Standard**: `false`

```json settings.json theme={null}
{
  "inputNeededNotifEnabled": true
}
```

Siehe [Mobile Push-Benachrichtigungen](/docs/de/remote-control#mobile-push-notifications).

<h3 id="preferrednotifchannel">
  `preferredNotifChannel`
</h3>

Wählen Sie, wie Claude Code Sie benachrichtigt, wenn eine Aufgabe abgeschlossen ist oder eine Berechtigungsaufforderung wartet. Wird in `/config` als **Lokale Benachrichtigungen** angezeigt.

* **Bereich**: [`Any file`](#scopes). Claude Code liest auch einen Wert, der von älteren Versionen in `~/.claude.json` hinterlassen wurde.
* **Typ**: Zeichenkette, eine von:
  * `"auto"`: Claude Code sendet eine Desktop-Benachrichtigung in iTerm2, Ghostty und Kitty, läutet die Glocke in Terminal.app nur, wenn die audible Glocke ausgeschaltet ist, und tut nichts anderswo
  * `"terminal_bell"`: Claude Code läutet das Glockensignal in jedem Terminal
  * `"iterm2"`: Claude Code sendet eine iTerm2-Desktop-Benachrichtigung
  * `"iterm2_with_bell"`: Claude Code sendet eine iTerm2-Desktop-Benachrichtigung und läutet die Glocke
  * `"kitty"`: Claude Code sendet eine Kitty-Desktop-Benachrichtigung
  * `"ghostty"`: Claude Code sendet eine Ghostty-Desktop-Benachrichtigung
  * `"notifications_disabled"`: Claude Code sendet keine Benachrichtigung
* **Standard**: `"auto"`

```json settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

Mit `"auto"` sendet Claude Code eine Desktop-Benachrichtigung in iTerm2, Ghostty und Kitty. In Terminal.app läutet es das Glockensignal nur, wenn Sie die audible Glocke von Terminal ausgeschaltet haben, und in anderen Terminals tut es nichts. Setzen Sie `"terminal_bell"`, um das Glockensignal in jedem Terminal zu läuten. Siehe [Erhalten Sie eine Terminal-Glocke oder Benachrichtigung](/docs/de/terminal-config#get-a-terminal-bell-or-notification).

<h3 id="remote-defaultenvironmentid">
  `remote.defaultEnvironmentId`
</h3>

Wählen Sie die Standard-[Cloud-Umgebung](/docs/de/cloud-environments) für Cloud-Sitzungen, die Sie von der CLI aus erstellen, z. B. mit `claude --cloud`. Claude Code schreibt diesen Schlüssel in Ihre Benutzereinstellungen, wenn Sie eine Umgebung mit [`/remote-env`](/docs/de/cloud-environments#select-an-environment-from-the-cli) auswählen.

* **Bereich**: [`Any file`](#scopes). Für eine selbstgehostete Umgebungs-ID nur Benutzer- oder verwaltete Einstellungen oder das Flag `--settings`.
* **Typ**: Zeichenkette, eine Umgebungs-ID wie `env_...` oder `ccpool_...`
* **Standard**: nicht gesetzt, daher verwendet Claude Code die von Anthropic gehostete Umgebung, wenn Ihre Liste eine hat, und ansonsten die erste Umgebung in Ihrer Liste, die keine [Remote Control Bridge-Umgebung](/docs/de/cloud-environments#the-default-environment) ist, oder die erste Umgebung, wenn jede eine Bridge-Umgebung ist
* **Sitzungsspezifische Außerkraftsetzungen**: `--environment` hat Vorrang vor diesem Schlüssel für die eine Cloud-Sitzung, die es erstellt

```json settings.json theme={null}
{
  "remote": {
    "defaultEnvironmentId": "env_0123abcd"
  }
}
```

Eine von Anthropic gehostete Umgebungs-ID, die mit `env_` beginnt, folgt der Standard-Einstellungspriorität, daher überschreibt ein Wert in den Projekteinstellungen eines Repositorys Ihre Auswahl auf Benutzerebene. Eine [selbstgehostete Umgebungs](/docs/de/self-hosted-environments)-ID, die mit `ccpool_` beginnt, wird nur aus Benutzereinstellungen, verwalteten Einstellungen und dem Flag `--settings` berücksichtigt; Claude Code ignoriert eine in den Projekt- oder lokalen Einstellungen eines Repositorys, und `/remote-env` zeigt, welcher Wert ignoriert wurde, daher kann eine eingecheckte Datei Sitzungen nicht auf eine selbstgehostete Umgebung lenken, die Sie nicht ausgewählt haben.

<h3 id="remotecontrolatstartup">
  `remoteControlAtStartup`
</h3>

Verbinden Sie [Remote Control](/docs/de/remote-control) automatisch, wenn jede interaktive Sitzung startet, anstatt auf `/remote-control` zu warten. Setzen Sie es auf `true`, um Auto-Connect einzuschalten, `false`, um es auszuschalten. Wird in `/config` als **Remote Control für alle Sitzungen aktivieren** angezeigt.

* **Bereich**: [`Any file`](#scopes). Claude Code liest auch einen Wert, der von älteren Versionen in `~/.claude.json` hinterlassen wurde.
* **Typ**: Boolean
  * `true`: Claude Code verbindet Remote Control automatisch, wenn jede interaktive Sitzung startet
  * `false`: Claude Code wartet auf `/remote-control`
* **Standard**: nicht gesetzt, daher folgt Auto-Connect dem Admin-Standard Ihrer Organisation, wenn einer gesetzt ist, und ansonsten dem aktuellen Standard von Claude Code
* **Sitzungsspezifische Außerkraftsetzungen**: `--remote-control` schaltet Remote Control für eine Sitzung ein, auch wenn dieser Schlüssel `false` ist, und kein Flag schaltet es für eine Sitzung aus

```json settings.json theme={null}
{
  "remoteControlAtStartup": true
}
```

Claude Code ignoriert eine `true` aus Projekt- oder lokalen Einstellungen, daher kann ein Repository Auto-Connect für seinen Checkout ausschalten, aber nicht einschalten. Für das vollständige Pro-Scope-Verhalten siehe [Remote Control für alle Sitzungen aktivieren](/docs/de/remote-control#enable-remote-control-for-all-sessions) und die [Sicherheitsschlüssel, bei denen der strengere Wert gilt](/docs/de/settings#security-keys-where-the-stricter-value-applies).

<h3 id="sshconfigs">
  `sshConfigs`
</h3>

Fügen Sie SSH-Verbindungen zur [Desktop](/docs/de/desktop#pre-configure-ssh-connections-for-your-team)-Umgebungs-Dropdown hinzu. Administratoren verwenden es, um gemeinsame Verbindungen an ein Team zu verteilen. Verbindungen, die Sie in verwalteten Einstellungen definieren, werden als verwaltet angezeigt, daher können Benutzer sie auswählen, aber nicht bearbeiten oder löschen in der App.

* **Bereich**: [`User or managed`](#scopes). Die Desktop-App liest diesen Schlüssel.
* **Typ**: Array von Objekten, jedes mit erforderlichen `id`, `name` und `sshHost` und optionalen `sshPort` und `sshIdentityFile`
* **Standard**: nicht gesetzt

Dieses Beispiel fügt eine Verbindung namens `Dev VM` hinzu, die sich mit `user@dev.example.com` verbindet:

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

Begrenzen Sie die Hosts, mit denen sich eine [Desktop SSH-Sitzung](/docs/de/desktop#restrict-which-ssh-hosts-users-can-connect-to) verbinden kann. Nur die Desktop-App liest diesen Schlüssel; die CLI nicht. Muster sind Groß-/Kleinschreibung-insensitiv: `*` passt auf jeden Host, `*.example.com` passt auf `example.com` und jede Subdomain, und alles andere ist eine genaue Übereinstimmung mit dem Hostnamen nach `~/.ssh/config`-Auflösung. Ein leeres Array schaltet SSH-Sitzungen aus.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Array von Hostnamen-Mustern
* **Standard**: nicht gesetzt, daher ist jeder Host erlaubt

Dieses Beispiel erlaubt `devboxes.example.com` und seine Subdomains, plus den genauen Host `bastion.example.com`:

```json managed-settings.json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

<span id="authentication-and-login" />

<h2 id="authentication-and-providers">
  Authentifizierung und Anbieter
</h2>

Stellen Sie Anmeldedaten über Hilfsskripte bereit und erzwingen Sie für Organisationen eine Anmeldemethode oder Organisation. Siehe [Authentifizierung](/docs/de/authentication).

<h3 id="apikeyhelper">
  `apiKeyHelper`
</h3>

Führen Sie Ihren eigenen Befehl aus, um die Anmeldedaten zu generieren, die Claude Code mit Modellanfragen sendet. Claude Code führt den Befehl über die System-Shell aus, `/bin/sh` auf macOS und Linux und `cmd` auf Windows, und sendet seine Ausgabe als sowohl `X-Api-Key` als auch `Authorization: Bearer` Header. Verwenden Sie es für dynamische oder rotierende Anmeldedaten, wie kurzlebige Token, die aus einem Vault abgerufen werden.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: string, eine Shell-Befehlszeile
* **Standard**: nicht gesetzt, daher führt Claude Code keinen Helfer aus

```json settings.json theme={null}
{
  "apiKeyHelper": "/bin/generate_temp_api_key.sh"
}
```

Claude Code speichert den Wert zwischen und führt den Befehl in diesen Fällen erneut aus:

* Nach der Cache-Lebensdauer, standardmäßig fünf Minuten oder das Intervall, das Sie mit [`CLAUDE_CODE_API_KEY_HELPER_TTL_MS`](/docs/de/env-vars) festlegen.
* Wenn eine Anfrage an die Anthropic API, direkt oder über ein [LLM-Gateway](/docs/de/llm-gateway), mit `401` oder `403` fehlschlägt.
* Vor dem Senden einer Anfrage an die Anthropic API, direkt oder über ein LLM-Gateway, wenn die zwischengespeicherte Ausgabe ein JWT ist, das nach der Generierung durch den Helfer abgelaufen ist. Erfordert Claude Code v2.1.246 oder später.

Die letzten beiden Fälle gelten nur, wenn die Ausgabe des Helfers die Anmeldedaten sind, die Claude Code sendet, und `ANTHROPIC_AUTH_TOKEN` nicht gesetzt ist.

In interaktiven Sitzungen führt Claude Code den Befehl nicht aus, bis Sie die Eingabeaufforderung zur Workspace-Vertrauenswürdigkeit akzeptieren, wenn der Befehl aus Projekt- oder lokalen Einstellungen stammt. Siehe [Verwaltung von Anmeldedaten](/docs/de/authentication#credential-management).

<h3 id="awsauthrefresh">
  `awsAuthRefresh`
</h3>

Führen Sie Ihren eigenen Befehl aus, z. B. `aws sso login`, um die Anmeldedaten in Ihrem `.aws`-Verzeichnis zu aktualisieren, wenn die Anmeldedaten, die Claude Code für [Amazon Bedrock](/docs/de/amazon-bedrock) hat, nicht mehr funktionieren. Claude Code überprüft zunächst die aktuellen Anmeldedaten gegen STS und führt den Befehl nur aus, wenn diese Überprüfung fehlschlägt, und liest dann das aktualisierte `.aws`-Verzeichnis.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: string, eine Shell-Befehlszeile
* **Standard**: nicht gesetzt, daher aktualisiert Claude Code AWS-Anmeldedaten nicht für Sie

```json settings.json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile"
}
```

Verwenden Sie diesen Schlüssel, wenn Ihr Aktualisierungsablauf in `.aws` schreibt; verwenden Sie [`awsCredentialExport`](#awscredentialexport), wenn er stattdessen Anmeldedaten ausgibt. Siehe [erweiterte Anmeldedatenkonfiguration](/docs/de/amazon-bedrock#advanced-credential-configuration).

<h3 id="awscredentialexport">
  `awsCredentialExport`
</h3>

Führen Sie Ihren eigenen Befehl aus, der AWS-Anmeldedaten als JSON ausgibt, damit Claude Code [Amazon Bedrock](/docs/de/amazon-bedrock) mit Anmeldedaten aufrufen kann, die nicht in Ihrem `.aws`-Verzeichnis vorhanden sind. Claude Code akzeptiert die `aws sts` Ausgabeform und die flache `aws configure export-credentials` Form und beschränkt die Anmeldedaten auf seinen eigenen Bedrock-Client, sodass die Shell-Befehle, die Claude ausführt, immer noch Ihre Umgebungsanmeldedaten sehen.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: string, eine Shell-Befehlszeile
* **Standard**: nicht gesetzt, daher verwendet Claude Code die Umgebungs-AWS-Anmeldedatenkette

```json settings.json theme={null}
{
  "awsCredentialExport": "/bin/generate_aws_grant.sh"
}
```

Im Gegensatz zu [`awsAuthRefresh`](#awsauthrefresh) führt Claude Code diesen Befehl immer aus, wenn er gesetzt ist, ohne zunächst die Umgebungsanmeldedaten zu überprüfen. Siehe [erweiterte Anmeldedatenkonfiguration](/docs/de/amazon-bedrock#advanced-credential-configuration).

<h3 id="forceloginmethod">
  `forceLoginMethod`
</h3>

Beschränken Sie, welche Art von Konto sich Personen anmelden können. Setzen Sie `"claudeai"`, um nur claude.ai-Konten zuzulassen, `"console"`, um nur Claude Console-Konten zuzulassen, oder `"gateway"`, um Personen zu einem [Cloud-Gateway](/docs/de/claude-apps-gateway) statt zu einer First-Party-Anmeldung zu senden. Administratoren legen es in verwalteten Einstellungen fest und koppeln es mit [`forceLoginOrgUUID`](#forceloginorguuid), um die claude.ai-Anmeldungen von Entwicklern in einer Organisation zu halten. Wenn Sie es in einer beliebigen Einstellungsdatei auf `"claudeai"` oder `"console"` setzen, stoppt Claude Code auch das Angebot der [schlüssellosen Console-Anmeldung](/docs/de/authentication#sign-in-without-an-api-key) in den Sitzungen, auf die diese Datei zutrifft.

* **Bereich**: [`Any file`](#scopes). Claude Code berücksichtigt `"gateway"` nur von einer verwalteten Quelle auf dem Computer: `managed-settings.json`, die macOS plist oder Windows HKLM-Registrierung oder ein Policy-Helfer. Es behandelt `"gateway"` als nicht gesetzt in Benutzer-, Projekt-, lokalen, HKCU- und Server-verwalteten Einstellungen, die gleiche Regel wie [`forceLoginGatewayUrl`](#forcelogingatewayurl).
* **Typ**: string, einer von:
  * `"claudeai"`: nur claude.ai-Konten können sich anmelden
  * `"console"`: nur Claude Console-Konten können sich anmelden
  * `"gateway"`: Claude Code sendet Personen zu einem Cloud-Gateway statt zu einer First-Party-Anmeldung
* **Standard**: nicht gesetzt, daher wählen Personen eine Anmeldemethode

```json settings.json theme={null}
{
  "forceLoginMethod": "claudeai"
}
```

Jeder First-Party-Anmeldepfad wendet die Einschränkung an, einschließlich der [VS Code-Erweiterung](/docs/de/vs-code), des Agent SDK, `claude setup-token` und `/install-github-app`, mit Ausnahme des interaktiven Anmeldebildschirms des Terminals, der über `/login` oder das Onboarding beim ersten Start erreichbar ist, das die Methode vorwählt, ohne sie zu erzwingen. Vor v2.1.212 galt dies nur für Terminal-Anmeldungen. Siehe [Anmeldung auf Ihre Organisation beschränken](/docs/de/authentication#restrict-login-to-your-organization), um zu erfahren, wie jeder Anmeldepfad, Umgebungsanmeldedaten und Drittanbieter behandelt werden.

Wenn eine verwaltete Quelle auf dem Computer `"gateway"` setzt, verwendet Claude Code keine verbleibende Anmeldung, keinen API-Schlüssel oder `apiKeyHelper`-Anmeldedaten. Siehe [Administrator-Richtlinie erfordert eine Cloud-Gateway-Anmeldung](/docs/de/errors#administrator-policy-requires-a-cloud-gateway-sign-in) für die Nachricht, die jede erzeugt. Wenn Sie einen Cloud-Anbieter über `CLAUDE_CODE_USE_BEDROCK` oder eine ähnliche Umgebungsvariable auswählen, benötigt die Sitzung keine Gateway-Anmeldung. Vor v2.1.261 verwendete Claude Code eine verbleibende Anmeldung auf diesen Computern.

<h3 id="forcelogingatewayurl">
  `forceLoginGatewayUrl`
</h3>

Legen Sie die Gateway-URL fest, mit der sich der `/login` Cloud-Gateway-Bildschirm verbindet, damit Personen Ihr [Cloud-Gateway](/docs/de/claude-apps-gateway) erreichen, ohne seine Adresse einzugeben. Der Bildschirm hat kein URL-Feld: Mit diesem Schlüssel gesetzt zeigt er Ihre Gateway-URL an und verbindet sich, wenn die Person die Eingabetaste drückt; ohne ihn teilt er ihnen mit, sich an ihren IT-Administrator zu wenden.

Entweder dieser Schlüssel oder `forceLoginMethod: "gateway"` macht den Computer nur Gateway-fähig, daher öffnet `/login` auf dem Cloud-Gateway-Bildschirm ohne Anmeldemethoden-Picker. Siehe [Administrator-Richtlinie erfordert eine Cloud-Gateway-Anmeldung](/docs/de/errors#administrator-policy-requires-a-cloud-gateway-sign-in), um zu erfahren, was mit einer verbleibenden First-Party-Anmeldung oder einem API-Schlüssel geschieht. Legen Sie beide Schlüssel fest, damit sich der Bildschirm verbindet, statt einen Fehler anzuzeigen.

* **Bereich**: [`Managed`](#scopes). Nur von einer Quelle auf dem Computer lesen: `managed-settings.json`, die macOS plist oder Windows HKLM-Registrierung oder ein Policy-Helfer. Claude Code ignoriert es in HKCU- und Server-verwalteten Einstellungen.
* **Typ**: string, eine vollständige URL einschließlich des Schemas
* **Standard**: nicht gesetzt, daher zeigt der Cloud-Gateway-Bildschirm einen Fehler an, der Personen auffordert, sich an ihren IT-Administrator zu wenden

```json managed-settings.json theme={null}
{
  "forceLoginGatewayUrl": "https://claude-gateway.example.com"
}
```

Wenn der Wert keine gültige URL ist, meldet der Anmeldebildschirm dies, und der Rest der verwalteten Einstellungsdatei wird immer noch angewendet. Siehe [Gateway-URL festlegen](/docs/de/claude-apps-gateway#set-the-gateway-url).

<h3 id="forceloginorguuid">
  `forceLoginOrgUUID`
</h3>

Verlangen Sie von einer verwalteten Quelle, dass claude.ai-Kontoanmeldungen zu einer Anthropic-Organisation gehören, angegeben als eine einzelne UUID, oder zu einer von mehreren Organisationen, angegeben als Array. Aus einer beliebigen Einstellungsdatei verwendet Claude Code auch eine einzelne UUID, um diese Organisation während einer claude.ai- oder Claude Console-Anmeldung vorzuwählen, und wählt nichts für ein Array vor. Wenn Sie den Schlüssel in einer beliebigen Einstellungsdatei setzen, stoppt Claude Code auch das Angebot der [schlüssellosen Console-Anmeldung](/docs/de/authentication#sign-in-without-an-api-key) in den Sitzungen, auf die diese Datei zutrifft, und erstellt stattdessen einen API-Schlüssel.

* **Bereich**: [`Any file`](#scopes). Nur eine verwaltete Quelle erzwingt die Einschränkung; eine einzelne UUID in einer beliebigen anderen Einstellungsdatei wählt die Organisation während der Anmeldung vor, ohne sie einzuschränken.
* **Typ**: string, eine UUID, oder Array von Strings, mehrere UUIDs
* **Standard**: nicht gesetzt, daher kann sich jede Organisation anmelden

Dieses Beispiel akzeptiert Anmeldungen von einer von zwei Organisationen, ohne eine vorzuwählen:

```json managed-settings.json theme={null}
{
  "forceLoginOrgUUID": ["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", "yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy"]
}
```

Wenn eine verwaltete Quelle ein leeres Array setzt oder einen Wert, den Claude Code nicht analysieren kann, blockiert Claude Code jede Anmeldung mit einer Fehlkonfigurationsmeldung.

Siehe [Anmeldung auf Ihre Organisation beschränken](/docs/de/authentication#restrict-login-to-your-organization), um zu erfahren, wie Claude Code Claude Console-Anmeldungen, die anderen Anmeldepfade und Umgebungsanmeldedaten behandelt.

<h3 id="gatewayinternalnetworks">
  `gatewayInternalNetworks`
</h3>

Deklarieren Sie die öffentlichen IPv4-Blöcke, aus denen Ihre Organisation ihr internes Netzwerk nummeriert, damit `/login` ein [Cloud-Gateway](/docs/de/claude-apps-gateway) dort akzeptiert. Erfordert Claude Code v2.1.268 oder später.

Ohne diesen Schlüssel verbindet sich `/login` mit jedem Gateway auf einer privaten Adresse und nichts anderem. Mit ihm akzeptiert `/login` auch ein Gateway innerhalb eines aufgelisteten Blocks, nur über eine direkte Verbindung. Die eigene Adresse des Computers auf dieser Verbindung muss sich auch innerhalb desselben Blocks befinden.

* **Bereich**: [`Managed`](#scopes). Nur von einer Quelle auf dem Computer lesen: `managed-settings.json`, die macOS plist oder Windows HKLM-Registrierung oder ein Policy-Helfer. Claude Code ignoriert es in HKCU- und Server-verwalteten Einstellungen.
* **Typ**: Array von Strings, höchstens vier IPv4-CIDR-Blöcke, jeweils `/8` bis `/32`, nicht überlappend miteinander und keine Überlappung mit privatem Raum.
* **Standard**: nicht gesetzt, daher akzeptiert `/login` nur Gateways auf privaten Adressen

```json managed-settings.json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

Ersetzen Sie den Dokumentationsbereich im Beispiel durch Ihren eigenen Block. Claude Code lehnt die Dokumentationsbereiche, die Bereiche, die VPN- und NAT64-Clients lokal verwenden, und reservierten Raum ab, aus dem kein Netzwerk nummeriert wird, wie Multicast.

Wenn ein Eintrag ungültig ist oder der Wert keine Liste von Strings ist, benennt `/login` das Problem und lehnt jede neue Gateway-Anmeldung auf dem Computer ab, bis Sie den Wert korrigieren. Bestehende Anmeldungen funktionieren weiterhin. Siehe [Erlauben Sie ein Gateway auf öffentlichem Adressraum, den Sie besitzen](/docs/de/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) für die vollständigen Regeln und was Entwickler sehen.

<h3 id="gcpauthrefresh">
  `gcpAuthRefresh`
</h3>

Führen Sie Ihren eigenen Befehl aus, um Google Cloud Application Default Credentials zu aktualisieren, wenn Claude Code feststellt, dass sie abgelaufen sind oder nicht geladen werden können, damit [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) Anfragen ohne manuelle Neuzertifizierung funktionieren.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: string, eine Shell-Befehlszeile
* **Standard**: nicht gesetzt, daher teilt Ihnen der Anmeldedatenfehler von Claude Code mit, `gcloud auth application-default login` selbst auszuführen

```json settings.json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login"
}
```

Siehe [erweiterte Anmeldedatenkonfiguration](/docs/de/google-vertex-ai#advanced-credential-configuration).

<h3 id="otelheadershelper">
  `otelHeadersHelper`
</h3>

Führen Sie Ihren eigenen Befehl aus, um die Header zu generieren, die Claude Code mit OpenTelemetry-Exporten sendet, für Backends, deren Token rotieren. Claude Code führt ihn beim Start und danach regelmäßig aus und erwartet ein JSON-Objekt von String-Header-Werten auf stdout.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: string, ein ausführbarer Pfad oder eine Shell-Befehlszeile
* **Standard**: nicht gesetzt, daher fügt Claude Code keine Helfer-generierten Header hinzu

```json settings.json theme={null}
{
  "otelHeadersHelper": "/bin/generate_otel_headers.sh"
}
```

Legen Sie das Aktualisierungsintervall mit [`CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`](/docs/de/env-vars) fest. Siehe [Dynamische Header](/docs/de/monitoring-usage#dynamic-headers) für die Skriptanforderungen und wo Claude Code einen fehlgeschlagenen Helfer meldet.

<h2 id="updates-and-versioning">
  Aktualisierungen und Versionsverwaltung
</h2>

Wählen Sie einen Aktualisierungskanal und fixieren Sie für Organisationen die Versionen, die Personen ausführen können. Siehe [Claude Code aktualisieren](/docs/de/setup#update-claude-code).

<h3 id="autoupdateschannel">
  `autoUpdatesChannel`
</h3>

Wählen Sie, welcher [Release-Kanal](/docs/de/setup#configure-release-channel) Hintergrund-Autoupdates und `claude update` folgen. Setzen Sie `"stable"` für eine Version, die typischerweise etwa eine Woche alt ist und Releases mit großen Regressionen überspringt, oder `"latest"` für die neueste Version.

* **Bereich**: [`Any file`](#scopes). Legen Sie es in verwalteten Einstellungen fest, um einen Kanal in Ihrer gesamten Organisation durchzusetzen.
* **Typ**: string, einer von:
  * `"latest"`: Aktualisierungen folgen der neuesten Version
  * `"stable"`: Aktualisierungen folgen einer Version, die typischerweise etwa eine Woche alt ist und Releases mit großen Regressionen überspringt
* **Standard**: nicht gesetzt, daher folgt Claude Code `"latest"`

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

Claude Code schreibt `"stable"` in Ihre Benutzereinstellungen, wenn Sie es unter **Auto-update channel** in `/config` auswählen, und entfernt den Schlüssel, wenn Sie dort zurück zu latest wechseln. `claude install stable` und `claude install latest` speichern auch den Kanal, den Sie benennen. Das Wechseln von `"latest"` zu `"stable"` in `/config` fragt, ob ein Downgrade erlaubt sein soll oder ob Sie bei Ihrer aktuellen Version bleiben möchten; das Bleiben setzt [`minimumVersion`](#minimumversion). Homebrew-Installationen ignorieren diesen Schlüssel: das `claude-code` Cask verfolgt stable und `claude-code@latest` verfolgt latest, und `claude update` delegiert an `brew upgrade`. Um Autoupdates vollständig auszuschalten, setzen Sie [`DISABLE_AUTOUPDATER`](/docs/de/setup#disable-auto-updates) in `env`.

<h3 id="minimumversion">
  `minimumVersion`
</h3>

Verhindern Sie, dass Hintergrund-Autoupdates und `claude update` eine Version unterhalb dieser installieren, damit ein Wechsel zum `"stable"` Kanal Sie nicht von einem neueren `"latest"` Build herabstuft. Claude Code schreibt diesen Schlüssel für Sie, wenn Sie sich entscheiden, bei Ihrer aktuellen Version zu bleiben, während Sie Kanäle in `/config` wechseln, und löscht ihn, wenn Sie zurück zu `"latest"` wechseln.

* **Bereich**: [`Any file`](#scopes). Legen Sie es in verwalteten Einstellungen fest, um ein organisationsweites Minimum zu fixieren, das Benutzer- und Projekteinstellungen nicht senken können.
* **Typ**: string, eine Versionsnummer wie `"2.1.100"`; ein Wert, der keine gültige Version ist, wird ignoriert
* **Standard**: nicht gesetzt, daher können Aktualisierungen jede Version installieren, die der Kanal anbietet

Dieses Beispiel folgt dem stabilen Kanal und weigert sich, eine Version unterhalb von 2.1.100 zu installieren:

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

Dieser Schlüssel beschränkt nur Aktualisierungen. Um Claude Code zu veranlassen, unterhalb einer Version nicht zu starten, verwenden Sie stattdessen [`requiredMinimumVersion`](#requiredminimumversion). Siehe [Minimum-Version fixieren](/docs/de/setup#pin-a-minimum-version).

<h3 id="requiredmaximumversion">
  `requiredMaximumVersion`
</h3>

Legen Sie die neueste Claude Code-Version fest, die Ihre Organisation starten darf. Wenn die laufende Version neuer ist, beendet Claude Code sich beim Start und teilt dem Benutzer mit, eine genehmigte Version durch die genehmigte Methode Ihrer Organisation zu installieren; `claude install <version>` kann auch funktionieren. Erfordert Claude Code v2.1.163 oder später.

* **Bereich**: [`Managed`](#scopes). Claude Code gibt keine Warnung aus, wenn es den Schlüssel anderswo ignoriert.
* **Typ**: string, eine Versionsnummer wie `"2.1.150"`; ein Wert, der keine gültige Version ist, wird ignoriert
* **Standard**: nicht gesetzt, daher gilt keine Obergrenze

```json managed-settings.json theme={null}
{
  "requiredMaximumVersion": "2.1.150"
}
```

Hintergrund-Autoupdates und `claude update` überspringen Versionen über der Obergrenze, daher bleibt eine Installation innerhalb des Bereichs darin. `claude update`, `claude install` und `claude doctor` funktionieren weiterhin über der Obergrenze, damit Benutzer sich wiederherstellen können. Kombinieren Sie es mit [`requiredMinimumVersion`](#requiredminimumversion), um einen Bereich durchzusetzen.

<h3 id="requiredminimumversion">
  `requiredMinimumVersion`
</h3>

Legen Sie die älteste Claude Code-Version fest, die Ihre Organisation starten darf. Wenn die laufende Version älter ist, beendet Claude Code sich beim Start und teilt dem Benutzer mit, ein Update durch die genehmigte Methode Ihrer Organisation durchzuführen. Die Überprüfung läuft nur beim Start, daher wird eine bereits laufende Sitzung fortgesetzt. Erfordert Claude Code v2.1.163 oder später.

* **Bereich**: [`Managed`](#scopes). Claude Code gibt keine Warnung aus, wenn es den Schlüssel anderswo ignoriert.
* **Typ**: string, eine Versionsnummer wie `"2.1.150"`; ein Wert, der keine gültige Version ist, wird ignoriert
* **Standard**: nicht gesetzt, daher gilt keine Untergrenze

```json managed-settings.json theme={null}
{
  "requiredMinimumVersion": "2.1.150"
}
```

`claude update`, `claude install` und `claude doctor` funktionieren weiterhin unter der Untergrenze, damit Benutzer sich wiederherstellen können. Im Gegensatz zu [`minimumVersion`](#minimumversion), das nur Downgrades verhindert, blockiert dieser Schlüssel den Start. Kombinieren Sie es mit [`requiredMaximumVersion`](#requiredmaximumversion), um einen Bereich durchzusetzen.

<h2 id="tools">
  Werkzeuge
</h2>

Deaktivieren Sie spezifische Werkzeuge in der [Claude Code Desktop-App](/docs/de/desktop). Die Terminal-CLI ignoriert diese Schlüssel. Für die Werkzeuge selbst siehe [Werkzeuge, die Claude zur Verfügung stehen](/docs/de/tools-reference).

<h3 id="browserexternalpagetools">
  `browserExternalPageTools`
</h3>

Verhindern Sie, dass Claude seine Werkzeuge zum Lesen oder Bearbeiten externer Seiten im [Browser-Bereich](/docs/de/desktop#browse-external-sites) der Desktop-App verwendet. Personen in Ihrer Organisation können externe Websites weiterhin selbst öffnen, und lokale Dev-Server-Vorschauversionen funktionieren weiterhin mit Claudes Werkzeugen. Die Desktop-App liest diesen Schlüssel; die Terminal-CLI ignoriert ihn.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: String, `"disabled"`; die Desktop-App akzeptiert auch `"disable"`, in beiden Fällen
* **Standard**: nicht gesetzt, daher funktionieren Claudes Werkzeuge auf externen Seiten

```json managed-settings.json theme={null}
{
  "browserExternalPageTools": "disabled"
}
```

Jeder andere Wert lässt Claudes Werkzeuge eingeschaltet, und ein nicht leerer String, der nicht einer der beiden akzeptierten Werte ist, protokolliert eine Warnung. Um externe Websites für Personen und Claude gleichermaßen zu blockieren, setzen Sie stattdessen [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation). Siehe [Externe Browsing für Ihre Organisation einschränken](/docs/de/desktop#restrict-external-browsing-for-your-organization).

<h3 id="disablebrowserexternalnavigation">
  `disableBrowserExternalNavigation`
</h3>

Deaktivieren Sie das externe Browsing im [Browser-Bereich](/docs/de/desktop#browse-external-sites) der Desktop-App für Personen und Claude gleichermaßen. Localhost Dev-Server-Vorschauversionen funktionieren weiterhin. Die Desktop-App liest diesen Schlüssel; die Terminal-CLI ignoriert ihn.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Boolean; nur der JSON-Boolean `true` hat Auswirkungen
  * `true`: die Desktop-App deaktiviert das externe Browsing im Browser-Bereich für Personen und Claude gleichermaßen; Localhost-Vorschauversionen funktionieren weiterhin
  * `false`: das externe Browsing bleibt aktiviert
* **Standard**: nicht gesetzt, daher ist das externe Browsing aktiviert

```json managed-settings.json theme={null}
{
  "disableBrowserExternalNavigation": true
}
```

Die Desktop-App ignoriert jeden anderen Wert, und ein Wert, der kein Boolean ist, wie der String `"true"` oder `1`, protokolliert auch eine Warnung. Um das externe Browsing aktiviert zu lassen, aber Claudes Werkzeuge auf externen Seiten deaktiviert zu halten, setzen Sie stattdessen [`browserExternalPageTools`](#browserexternalpagetools). Siehe [Externe Browsing für Ihre Organisation einschränken](/docs/de/desktop#restrict-external-browsing-for-your-organization).

<h3 id="disablemobilesimulatortools">
  `disableMobileSimulatorTools`
</h3>

Blockieren Sie Claudes Werkzeuge für den [iOS Simulator-Bereich](/docs/de/desktop-ios-simulator#turn-off-simulator-access) der Desktop-App. Personen behalten die manuelle Nutzung des Bereichs; nur Claudes Zugriff wird entfernt, und niemand kann ihn von innerhalb der App wieder aktivieren. Die Desktop-App liest diesen Schlüssel; die Terminal-CLI ignoriert ihn.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Boolean; nur der JSON-Boolean `true` hat Auswirkungen
  * `true`: die Desktop-App blockiert Claudes Werkzeuge für den iOS Simulator-Bereich
  * `false`: Claudes Simulator-Werkzeuge folgen der Einstellung jeder Person in der Desktop-App
* **Standard**: nicht gesetzt, daher folgen Claudes Simulator-Werkzeuge der Einstellung jeder Person in der Desktop-App

```json managed-settings.json theme={null}
{
  "disableMobileSimulatorTools": true
}
```

Die Desktop-App ignoriert jeden anderen Wert, und ein Wert, der kein Boolean ist, wie der String `"true"` oder `1`, protokolliert auch eine Warnung.

<span id="data-and-privacy" />

<h2 id="privacy-and-telemetry">
  Datenschutz und Telemetrie
</h2>

Steuern Sie, wie lange Claude Code Sitzungsdaten speichert und was es sendet. Die Schalter zum Deaktivieren von Nutzungsmetriken und Fehlerberichten sind Umgebungsvariablen, keine Einstellungsschlüssel: Setzen Sie `DISABLE_TELEMETRY`, `DISABLE_ERROR_REPORTING` oder `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` im Schlüssel [`env`](#env) oder in der Shell. [Telemetrie-Dienste](/docs/de/data-usage#telemetry-services) zeigt, was jeder stoppt. Zwei Ausnahmen werden aus einer Einstellungsdatei deaktiviert: [`feedbackDrafts`](#feedbackdrafts) unten für von Claude entworfenes Feedback und [`feedbackSurveyRate`](#feedbacksurveyrate) unten für die Sitzungsumfrage.

<h3 id="cleanupperioddays">
  `cleanupPeriodDays`
</h3>

Legen Sie fest, wie viele Tage Claude Code [Sitzungstranskripte und andere Anwendungsdaten](/docs/de/claude-directory#cleaned-up-automatically) speichert, bevor sie gelöscht werden. Claude Code führt die Löschung als Hintergrund-Sweep nach dem Start einer Sitzung durch, solange es die Aufbewahrungsfrist sicher bestimmen kann.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Anzahl der Tage, eine ganze Zahl, Minimum `1`
* **Standard**: `30`

```json settings.json theme={null}
{
  "cleanupPeriodDays": 20
}
```

Das Setzen von `0` schlägt bei der Validierung fehl, wählen Sie daher einen großen Wert wie `3650` für lange Aufbewahrung. Um Claude Code davon abzuhalten, Transkripte überhaupt zu schreiben, siehe [Plaintext-Speicherung](/docs/de/claude-directory#plaintext-storage).

<h3 id="desktopsessioncleanupperioddays">
  `desktopSessionCleanupPeriodDays`
</h3>

Legen Sie eine Altersgrenze in Tagen für die Transkripte von Sitzungen fest, die Sie in Claude Desktop oder Cowork gestartet oder zuletzt fortgesetzt haben. Ohne diesen Schlüssel [behält Claude Code diese Transkripte in jedem Alter](/docs/de/claude-directory#cleaned-up-automatically). Claude Code löscht jedes, sobald es älter ist als sowohl diese Grenze als auch [`cleanupPeriodDays`](#cleanupperioddays), also mit `cleanupPeriodDays` auf dem Standard von 30 behält ein Wert von `7` sie immer noch 30 Tage. Wenn verwaltete Einstellungen `cleanupPeriodDays` setzen, gilt dieser Zeitraum stattdessen und dieser Schlüssel wird ignoriert. Erfordert Claude Code v2.1.248 oder später.

* **Bereich**: [`User or managed`](#scopes). Claude Code liest den Schlüssel auch aus einer Datei, die Sie mit `--settings` übergeben, und ignoriert ihn in Projekt- und lokalen Einstellungen.
* **Typ**: Anzahl der Tage, eine ganze Zahl, Minimum `0`
* **Standard**: `0`, was keine Altersgrenze setzt

```json settings.json theme={null}
{
  "desktopSessionCleanupPeriodDays": 90
}
```

<h3 id="feedbackdrafts">
  `feedbackDrafts`
</h3>

Steuern Sie [von Claude entworfenes Feedback](/docs/de/tools-reference#sendfeedback-tool-behavior): ob Claude Feedback-Entwürfe zur Überprüfung in die Warteschlange einreihen kann und ob Claude Code eine Karte anzeigt, wenn Claude einen einreiht.

* **Bereich**: [`User or managed`](#scopes)
* **Typ**: String, einer von `"notify"`, `"quiet"` oder `"off"`
  * `"notify"`: Claude Code zeigt eine Karte über der Eingabeaufforderung an, wenn Claude einen Entwurf einreiht, standardmäßig bis zu [drei Karten in einer Sitzung](/docs/de/tools-reference#what-you-see-when-claude-drafts)
  * `"quiet"`: Claude entwirft ohne Karte. Sie sehen die Anzahl der eingereihten Entwürfe in der Eingabeaufforderungs-Fußzeile und überprüfen sie in `/feedback`
  * `"off"`: Claude Code entfernt das SendFeedback-Tool, sodass Claude keine Entwürfe einreihen kann
* **Standard**: `"notify"`
* **Sitzungsspezifische Außerkraftsetzungen**: [`CLAUDE_CODE_SEND_FEEDBACK`](/docs/de/env-vars) auf `0` gesetzt deaktiviert die Funktion für eine Sitzung

```json settings.json theme={null}
{
  "feedbackDrafts": "quiet"
}
```

Erscheint in `/config` als **Claude-drafted feedback**, das diesen Schlüssel in Ihre Benutzereinstellungen schreibt. Sie sehen die `/config`-Zeile nur in Sitzungen, [in denen Claude Feedback entwerfen kann](/docs/de/tools-reference#sessions-without-claude-drafted-feedback); das Setzen von `"off"` blendet sie nicht aus, sodass Sie die Funktion aus derselben Zeile wieder aktivieren können. Ein Wert in verwalteten Einstellungen hat Vorrang vor Ihrer Benutzereinstellung, sodass wenn ein Administrator diesen Schlüssel setzt, die Zeile den verwalteten Wert anzeigt und das Ändern hat keine Auswirkung. Claude Code ignoriert diesen Schlüssel in Projekt- und lokalen Einstellungen.

<h3 id="feedbacksurveyrate">
  `feedbackSurveyRate`
</h3>

Legen Sie die Wahrscheinlichkeit fest, dass die [Sitzungsqualitätsumfrage](/docs/de/data-usage#session-quality-surveys) angezeigt wird, wenn eine Sitzung für sie berechtigt ist. Setzen Sie `0`, um zu verhindern, dass die Umfrage angezeigt wird.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Zahl zwischen `0` und `1`
* **Standard**: nicht gesetzt, sodass Claude Code die Rate verwendet, die Anthropic remote setzt, oder seine integrierte Rate von `0.005` auf Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry, die keine Remote-Konfiguration erhalten
* **Sitzungsspezifische Außerkraftsetzungen**: [`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY`](/docs/de/env-vars) auf `1` gesetzt deaktiviert die Umfrage für eine Sitzung, unabhängig davon, welche Rate dieser Schlüssel setzt

```json settings.json theme={null}
{
  "feedbackSurveyRate": 0.05
}
```

Die gleiche Rate gilt für die Umfrage in der VS Code-Erweiterung.

<h3 id="skipwebfetchpreflight">
  `skipWebFetchPreflight`
</h3>

Überspringen Sie die [WebFetch-Domänensicherheitsprüfung](/docs/de/data-usage#webfetch-domain-safety-check), die jeden angeforderten Hostnamen an `api.anthropic.com` sendet, bevor sie abruft. Setzen Sie `true` in Umgebungen, die den Datenverkehr zu Anthropic blockieren, wie z. B. Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry-Bereitstellungen mit restriktivem Ausgang.

* **Bereich**: [`Any file`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code überspringt die WebFetch-Domänensicherheitsprüfung
  * `false`: Die Prüfung wird vor dem ersten Abruf zu jedem Hostnamen in einer Sitzung ausgeführt und erneut für einen Hostnamen, dessen frühere Prüfung blockiert oder fehlgeschlagen ist
* **Standard**: nicht gesetzt, sodass die Prüfung vor dem ersten Abruf zu jedem Hostnamen in einer Sitzung ausgeführt wird

```json settings.json theme={null}
{
  "skipWebFetchPreflight": true
}
```

Mit der übersprungenen Prüfung versucht WebFetch jede URL, ohne die Blockliste zu konsultieren, also kombinieren Sie sie mit [`WebFetch`-Berechtigungsregeln](/docs/de/permissions#webfetch), wenn Sie einschränken müssen, welche Domänen Claude erreichen kann.

<span id="managed-policy" />

<h2 id="enterprise-and-managed-settings">
  Enterprise- und verwaltete Einstellungen
</h2>

Schlüssel, die eine Organisation verwendet, um verwaltete Einstellungen zu berechnen, zu aktualisieren und zu kombinieren. Siehe [Verwaltete Einstellungen einrichten](/docs/de/admin-setup).

<h3 id="disablesideloadflags">
  `disableSideloadFlags`
</h3>

Lehnen Sie die CLI-Flags `--plugin-dir`, `--plugin-url`, `--agents` und `--mcp-config` beim Start ab, die Benutzer andernfalls übergeben könnten, um [`strictKnownMarketplaces`](#strictknownmarketplaces) für einen einzelnen Durchlauf zu umgehen. Claude Code wird mit einem Fehler beendet, der die abgelehnten Flags benennt, und wendet die gleiche Überprüfung auf Oberflächen an, die die CLI intern mit diesen Flags starten, derzeit [Cowork](/docs/de/desktop)-Lokalsitzungen in der Desktop-App. In [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) verwirft Claude Code die MCP-Server, die der Server über `--mcp-config` bereitgestellt hat, mit Ausnahme von In-Process-Einträgen vom Typ `type: "sdk"`, und startet die Sitzung. Erfordert Claude Code v2.1.193 oder später.

* **Bereich**: [`Managed`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code lehnt `--plugin-dir`, `--plugin-url`, `--agents` und `--mcp-config` beim Start ab und wird mit einem Fehler beendet, der diese benennt, außer dass es in Cloud-Sitzungen die MCP-Server verwirft, die der Server über `--mcp-config` bereitgestellt hat, mit Ausnahme von In-Process-Einträgen vom Typ `type: "sdk"`, und startet die Sitzung
  * `false`: Claude Code akzeptiert diese Flags
* **Standard**: `false`

```json managed-settings.json theme={null}
{
  "disableSideloadFlags": true
}
```

Claude Code akzeptiert weiterhin ein `--mcp-config`, dessen Server alle In-Process-Einträge vom Typ `type: "sdk"` sind, sodass das Agent SDK und die VS Code-Erweiterung weiterhin funktionieren. Benutzer können weiterhin Server mit `claude mcp add` oder einer `.mcp.json`-Datei hinzufügen; für Pro-Server-Kontrolle setzen Sie auch [`allowedMcpServers`](/docs/de/managed-mcp). Erfordert Claude Code v2.1.193 oder später.

Die gleiche Überprüfung deckt Plugin-Ordner ab, die in der Umgebungsvariablen [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/de/env-vars#variables) benannt sind, was Claude Code v2.1.280 oder später erfordert. Wenn die Variable einen Ordner benennt, wird Claude Code mit dem gleichen Fehler beendet, und der Fehler sagt, die Variable zu deaktivieren.

In Cloud-Sitzungen ignoriert Claude Code auch vom Server bereitgestellte MCP-Updates während der Sitzung, den Pfad hinter der Cloud-Sitzungskonfiguration und SDK `setMcpServers()`-Aufrufe, die diese Sitzungen erreichen. In-Process-Einträge vom Typ `type: "sdk"` bleiben dort auch ausgenommen. Vor v2.1.239 blockierte ein vom Server bereitgestelltes `--mcp-config` den Start einer Cloud-Sitzung.

<h3 id="forceremotesettingsrefresh">
  `forceRemoteSettingsRefresh`
</h3>

Blockieren Sie den CLI-Start, bis Claude Code [vom Server verwaltete Einstellungen](/docs/de/server-managed-settings) neu abgerufen hat. Wenn der Abruf fehlschlägt, wird Claude Code beendet, anstatt mit zwischengespeicherten oder ohne Einstellungen fortzufahren. Setzen Sie dies, wenn Ihre Umgebung nicht einmal ein kurzes Fenster akzeptieren kann, in dem eine Sitzung ohne ihre verwaltete Richtlinie ausgeführt wird.

Wenn der Schlüssel nicht gesetzt ist, blockiert Claude Code den Start nicht beim Abruf, obwohl es beim Anmelden des Entwicklers beim Start bis zu fünf Sekunden auf den Abruf wartet. Eine Cloud-Gateway-Sitzung wartet immer und wird beendet, wenn das Gateway nicht erreichbar ist.

* **Bereich**: [`Managed`](#scopes). Claude Code berücksichtigt ein `true` aus jeder admin-kontrollierten verwalteten Quelle, auch wenn es nicht die höchste Prioritätsquelle ist.
* **Typ**: Boolean
  * `true`: Claude Code blockiert den Start, bis er vom Server verwaltete Einstellungen neu abgerufen hat, und wird beendet, wenn der Abruf fehlschlägt
  * `false`: Claude Code blockiert den Start nicht beim Abruf, obwohl es beim Anmelden des Entwicklers beim Start bis zu fünf Sekunden auf den Abruf wartet
* **Standard**: `false`

```json managed-settings.json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Setzen Sie dies in einem MDM-Profil oder der verwalteten Einstellungsdatei, um einen Fail-Closed-Start durchzusetzen, bevor die erste Server-Nutzlast ankommt. Claude Code wendet die Überprüfung nur in Sitzungen an, die vom Server verwaltete Einstellungen abrufen, sodass eine Sitzung, die [diese nicht abruft](/docs/de/server-managed-settings#platform-availability), ohne Wartezeit startet. Die `claude auth`-Unterbefehle sind ausgenommen, sodass Benutzer sich erneut authentifizieren können, wenn abgelaufene Anmeldedaten der Grund für den fehlgeschlagenen Abruf sind. Siehe [Fail-Closed-Start durchsetzen](/docs/de/server-managed-settings#enforce-fail-closed-startup).

<h3 id="managedsourcesbehavior">
  `managedSourcesBehavior`
</h3>

Wählen Sie, ob Claude Code nur die höchste Priorität [verwaltete Quelle](/docs/de/managed-settings#how-claude-code-combines-managed-sources) anwendet, die Ihre Organisation bereitstellt, oder alle Admin-Quellen kombiniert, die sie bereitstellt. Standardmäßig nimmt Claude Code die höchste Prioritätsquelle, die einen [Richtlinienschlüssel](/docs/de/managed-settings#how-claude-code-combines-managed-sources) trägt, und ignoriert den Rest. Ein Richtlinienschlüssel ist jeder Einstellungsschlüssel außer diesem und `wslInheritsWindowsSettings`. Sobald vom Server verwaltete Einstellungen oder eine MDM-Richtlinie einen Richtlinienschlüssel bereitstellen, trägt eine `managed-settings.json`-Datei nur die [Schlüssel bei, die Claude Code aus jeder Admin-Quelle liest](/docs/de/managed-settings#keys-read-from-every-admin-source). Mit `"merge"` trägt jede Admin-Quelle, die Sie bereitstellen, ihre Schlüssel zu einer kombinierten Richtlinie bei. Erfordert Claude Code v2.1.242 oder später.

Setzen Sie `"merge"` nur dort, wo jede Quelle, die [unter Ihrer höchsten eingestuft](/docs/de/managed-settings#how-claude-code-combines-managed-sources) ist, unter der Kontrolle eines Administrators steht, da Claude Code dann Einträge aus einer niedrigeren Quelle, wie z. B. `permissions.allow`-Regeln, zur Richtlinie hinzufügt.

* **Bereich**: [`Managed`](#scopes). Claude Code liest diesen Schlüssel aus der höchsten Prioritätsquelle, die entweder diesen Schlüssel oder einen Richtlinienschlüssel trägt, und ignoriert diesen Schlüssel in jeder Quelle, die niedriger eingestuft ist, sodass eine niedrigere Quelle sich nicht selbst zum Kombinieren mit der Quelle darüber anmelden kann. Weder die Windows HKCU-Registrierung noch [übergeordnete Einstellungen von einem Embedding-Host](/docs/de/managed-settings#let-an-embedding-host-add-policy) nehmen an der Zusammenführung teil.
* **Typ**: string, einer von:
  * `"first-wins"`: Die höchste Prioritätsquelle, die einen Richtlinienschlüssel trägt, liefert die Richtlinie, und niedrigere Quellen tragen nur die [Schlüssel bei, die Claude Code aus jeder Admin-Quelle liest](/docs/de/managed-settings#keys-read-from-every-admin-source)
  * `"merge"`: Jede Admin-Quelle, die Sie bereitstellen, trägt ihre Schlüssel bei, kombiniert nach den folgenden Regeln
* **Standard**: `"first-wins"`

Stellen Sie den Schlüssel in der höchsten Prioritätsquelle bereit, die Sie bereitstellen. Ein Computer, der niemals vom Server verwaltete Einstellungen erhält, benötigt den Schlüssel auch in seinem MDM-Profil, da Claude Code den Schlüssel aus der höchsten Prioritätsquelle liest, die ihn oder einen Richtlinienschlüssel trägt. Eine `managed-settings.json`-Datei ist die niedrigste Admin-Quelle, daher hat `"merge"` dort keine Quelle darunter, um sich damit zu kombinieren. In vom Server verwalteten Einstellungen sieht der Schlüssel so aus:

```json theme={null}
{
  "managedSourcesBehavior": "merge"
}
```

Unter `"merge"` kombiniert Claude Code jeden Schlüssel nach seiner Art. Diese Tabelle gibt die Regel für jede Art an. Die Zeilen für Einschränkungserlaubnis, Werte-ganz-genommen und nur-höchste-Quelle benennen jeden Schlüssel, den sie abdecken, und die anderen Zeilen geben Beispiele:

| Art des Schlüssels                          | Wie Claude Code ihn kombiniert                                                                                                                                                                                         | Schlüssel                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Listen                                      | Kombiniert Einträge aus jeder Quelle                                                                                                                                                                                   | [`permissions.allow`](#permissions-allow), [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) und andere Listenschlüssel                                                                                                                                                                                                                                                                                                                                                                                          |
| Sperren                                     | Wendet den strengsten Wert an, den eine Quelle setzt. Wenn keine Quelle einen strengen Wert setzt, wendet einen lockereren Wert nur aus der höchsten Quelle an                                                         | [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly), [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode) und andere Boolean- oder Enum-Sperren                                                                                                                                                                                                                                                                                                                               |
| Einschränkungserlaubnisse                   | Nimmt die Liste ganz aus der höchsten Quelle, die sie setzt, ohne Einträge aus niedrigeren Quellen hinzuzufügen. Wenn die höchste Quelle keine setzt, nimmt sie ganz aus der nächsten Quelle darunter                  | [`availableModels`](#availablemodels), [`allowedMcpServers`](#allowedmcpservers), [`strictKnownMarketplaces`](#strictknownmarketplaces), [`allowedChannelPlugins`](#allowedchannelplugins) und die [`fallbackModel`](#fallbackmodel)-Kette                                                                                                                                                                                                                                                                                         |
| Werte ganz genommen                         | Nimmt den Wert ganz aus der höchsten Quelle, die ihn setzt, ohne Einträge oder Felder aus niedrigeren Quellen zu kombinieren. Wenn die höchste Quelle ihn nicht setzt, nimmt ihn ganz aus der nächsten Quelle darunter | [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs), [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Bereitgestellte MCP-Server                  | Kombiniert die Servernamen aus jeder Quelle. Wenn zwei Quellen denselben Namen setzen, wendet den ganzen Eintrag der höheren Quelle an                                                                                 | [`managedMcpServers`](#managedmcpservers)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| Nur aus der höchsten Prioritätsquelle lesen | Liest den Schlüssel nur aus der höchsten Prioritätsquelle, die einen Richtlinienschlüssel trägt, sodass der Wert einer niedrigeren Quelle ignoriert wird, auch wenn die höchste Quelle keinen setzt                    | [`apiKeyHelper`](#apikeyhelper), [`awsAuthRefresh`](#awsauthrefresh), [`awsCredentialExport`](#awscredentialexport), [`gcpAuthRefresh`](#gcpauthrefresh), [`otelHeadersHelper`](#otelheadershelper), `proxyAuthHelper`, [`forceLoginOrgUUID`](#forceloginorguuid), die `"claudeai"`- und `"console"`-Werte von [`forceLoginMethod`](#forceloginmethod), [`parentSettingsBehavior`](#parentsettingsbehavior), [`modelPicker`](#modelpicker), [`policyHelper`](#policyhelper), [`permissions.defaultMode`](#permissions-defaultmode) |
| `env`                                       | [Zusammenführung pro Variable über Admin-Quellen](/docs/de/managed-settings#keys-read-from-every-admin-source), unter sowohl `"first-wins"` als auch `"merge"`                                                              | [`env`](#env)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| Jeder andere Schlüssel                      | Nimmt den Wert aus der höchsten Quelle, die ihn setzt                                                                                                                                                                  | [`cleanupPeriodDays`](#cleanupperioddays), [`model`](#model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |

Das Nehmen von `sandbox.credentials.awsPairs` und `sandbox.ripgrep` ganz erfordert Claude Code v2.1.257 oder später.

Einige Schlüssel fügen eine Bedingung hinzu, die die Tabelle nicht zeigt:

* **[`policyHelper`](#policyhelper)**: Claude Code berücksichtigt ihn nur, wenn die höchste Quelle, die einen Richtlinienschlüssel trägt, eine MDM-Richtlinie oder eine verwaltete Einstellungsdatei ist, sodass unter vom Server verwalteten Einstellungen sie nicht angewendet wird.
* **[`modelOverrides`](#modeloverrides)**: Paare mit `availableModels`. Claude Code nimmt `modelOverrides` aus der höchsten Quelle, die ihn setzt, es sei denn, eine höhere Quelle setzt `availableModels` ohne `modelOverrides`. In diesem Fall ignoriert es `modelOverrides` aus jeder Quelle.
* **[`forceLoginGatewayUrl`](#forcelogingatewayurl), [`gatewayInternalNetworks`](#gatewayinternalnetworks) und der `"gateway"`-Wert von [`forceLoginMethod`](#forceloginmethod)**: Claude Code liest sie nie aus vom Server verwalteten Einstellungen, sodass ein Wert dort weder angewendet wird noch einen in einer MDM-Richtlinie oder verwalteten Einstellungsdatei gesetzten Wert verbirgt. Unter den Admin-Quellen auf dem Computer liefert nur die höchste eingestufte, die einen Richtlinienschlüssel trägt, sie, unabhängig davon, ob vom Server verwaltete Einstellungen auch vorhanden sind.

Um zu bestätigen, welche Quellen auf einem Computer kombiniert wurden, führen Sie `/status` aus und [lesen Sie die Zeile `Setting sources`](/docs/de/managed-settings#read-the-source-in-/status).

<h3 id="parentsettingsbehavior">
  `parentSettingsBehavior`
</h3>

Wählen Sie, ob Claude Code verwaltete Einstellungen anwendet, die von einem Embedding-Host-Prozess bereitgestellt werden, wie z. B. dem Agent SDK oder einer IDE-Erweiterung, wenn auch eine admin-bereitgestellte verwaltete Ebene vorhanden ist. Mit `"first-wins"` verwirft Claude Code die vom Host bereitgestellten Einstellungen; mit `"merge"` wendet es sie unter der Admin-Ebene durch einen restriktiv-nur-Filter an. Setzen Sie `"merge"`, wenn ein Host seine eigenen Einschränkungen an die Sitzungen übergeben muss, die er startet, z. B. Claude Desktop, das die Egress-Erlaubnisliste eines Gateways bereitstellt.

* **Bereich**: [`Managed`](#scopes). Claude Code liest ihn aus der höchsten Priorität admin-kontrollierten verwalteten Quelle.
* **Typ**: string, einer von:
  * `"first-wins"`: Claude Code verwirft die vom Host bereitgestellten Einstellungen, wenn eine admin-bereitgestellte verwaltete Ebene vorhanden ist
  * `"merge"`: Claude Code wendet die vom Host bereitgestellten Einstellungen unter der Admin-Ebene durch einen restriktiv-nur-Filter an
* **Standard**: `"first-wins"`

```json managed-settings.json theme={null}
{
  "parentSettingsBehavior": "merge"
}
```

Dieser Schlüssel hat keine Auswirkung, wenn keine admin-bereitgestellte verwaltete Ebene vorhanden ist: Die Einstellungen des Hosts gelten dann als einzige verwaltete Ebene, immer noch gefiltert auf restriktive Werte. Für die Grenzen des Filters und wie die verwalteten Quellen interagieren, siehe [Übergeordnete Einstellungen von Embedding-Hosts](/docs/de/managed-settings#parent-settings-from-embedding-hosts) und [Übergeordnete Einstellungen einschränken](/docs/de/claude-apps-gateway#restrict-parent-settings).

<span id="compute-managed-settings-with-a-policy-helper" />

<h3 id="policyhelper">
  `policyHelper`
</h3>

Führen Sie eine ausführbare Datei aus, die Sie bereitstellen, die verwaltete Einstellungen beim Start berechnet, sodass Sie Richtlinien von Geräteposition, Identität oder einem Remote-Service ableiten können, anstatt von einer statischen Datei. Claude Code führt den Helper aus, bevor er die erste Eingabeaufforderung akzeptiert, und behandelt die Einstellungen, die er ausgibt, als die verwalteten Einstellungen für die Sitzung.

* **Bereich**: [`Managed`](#scopes). Lesen Sie aus der macOS plist, der Windows HKLM-Registrierung oder der verwalteten Einstellungsdatei. Claude Code liest den Schlüssel aus der höchsten Priorität verwalteten Quelle, die einen [Richtlinienschlüssel](/docs/de/managed-settings#how-claude-code-combines-managed-sources) trägt, und führt den Helper nur aus, wenn diese Quelle eine dieser drei ist; es ignoriert den Schlüssel in vom Server verwalteten Einstellungen, der HKCU-Registrierung und vom Host bereitgestellten übergeordneten Einstellungen.
* **Typ**: Objekt mit `path`, `timeoutMs` und `refreshIntervalMs`
* **Standard**: nicht gesetzt, sodass kein Helper ausgeführt wird

Wenn vom Server verwaltete Einstellungen die Richtlinie beim Start bereitstellen, haben sie Vorrang vor der Quelle des Helpers und der Helper wird nicht ausgeführt.

Wenn ein späterer Einstellungsabruf meldet, dass die vom Server verwalteten Einstellungen entfernt wurden, führt Claude Code den Helper an diesem Punkt aus, anstatt auf den nächsten Start zu warten. Seine Ausgabe regelt den Rest der Sitzung, und ein Durchlauf, der fehlschlägt, beendet die Sitzung mit der gleichen Meldung wie ein [fehlgeschlagener Startup-Durchlauf](#helper-failures).

Dieses Beispiel führt den Helper mit einem 5-Sekunden-Timeout aus und führt ihn alle fünf Minuten erneut aus:

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
  Schreiben Sie die Helper-Ausgabe
</h4>

Claude Code führt den Helper ohne Argumente aus, setzt `CLAUDE_CODE_VERSION` in seiner Umgebung und liest eine JSON-Hülle aus stdout, begrenzt auf 1 MiB.

Legen Sie die Einstellungen unter einen `managedSettings`-Schlüssel. Ein bloßes Einstellungsobjekt ohne `managedSettings`-Schlüssel wird mit `managedSettings` undefined geparst und wendet nichts an, und Claude Code meldet keinen Fehler:

```json theme={null}
{
  "managedSettings": {
    "permissions": { "deny": ["Read(//etc/secrets/**)"] }
  }
}
```

Wenn der Helper `managedSettings` ausgibt, wird dieses Objekt die einzige verwaltete Einstellungsquelle für den Durchlauf: Claude Code ignoriert die MDM-, Datei- und HKCU-Quellen, liest die [quellübergreifenden Schlüssel](/docs/de/managed-settings#keys-read-from-every-admin-source) nur aus der Ausgabe des Helpers und führt niemals [übergeordnete Einstellungen](/docs/de/managed-settings#parent-settings-from-embedding-hosts) zusammen.

Die Startup-Überprüfung `forceRemoteSettingsRefresh` wird vor dem Helper ausgeführt und liest jede Admin-Quelle. Ein Helper, der mit `0` und einer Hülle beendet wird, die `managedSettings` auslässt, trägt keine verwalteten Einstellungen bei, und die anderen Quellen gelten wie üblich.

<h4 id="helper-failures">
  Helper-Fehler
</h4>

Ein Helper-Durchlauf schlägt fehl, wenn:

* `path` bricht die Regeln in [`policyHelper.path`](#policyhelper-path).
* Keine reguläre Datei ist unter `path`. Claude Code prüft auf die Datei, bevor der Helper gestartet wird, innerhalb des gleichen `timeoutMs`-Budgets, sodass eine nicht reagierende Netzwerkbereitstellung den Durchlauf zum Fehlschlag bringen kann.
* Der Helper beendet sich mit nicht-null, läuft noch, wenn `timeoutMs` verstreicht, oder startet überhaupt nicht, z. B. weil er nicht ausführbar ist.
* Der Helper schreibt mehr als 1 MiB zu stdout oder stderr.
* stdout ist kein einzelnes JSON-Objekt, oder sein `managedSettings` hat eine [Schema-Verletzung, die Claude Code nicht reparieren kann](/docs/de/managed-settings#find-entries-claude-code-dropped).

Wenn der Startup-Durchlauf fehlschlägt, druckt Claude Code den Grund und weigert sich zu starten. Nach einem nicht-null-Exit enthält der Grund den stderr des Helpers oder seinen stdout, wenn stderr leer ist. Nach einem Timeout benennt der Grund das `timeoutMs`-Limit und enthält keine Ausgabe des Helpers. Die Weigerung deckt interaktive Sitzungen, `claude -p`, Agent SDK-Sitzungen, [Hintergrund-Sitzungen](/docs/de/agent-view) und die meisten Unterbefehle ab.

Die Weigerung ist absichtlich, sodass ein Helper, der Ausfallresilienz benötigt, aus seinem eigenen Cache bedienen und mit `0` beenden sollte.

Wenn eine Hintergrund-Aktualisierung fehlschlägt, behält Claude Code die letzte erfolgreiche Richtlinie bei, und `/status` zeigt die fehlgeschlagene Aktualisierung mit ihrem Grund an, bis eine Aktualisierung erfolgreich ist. Jede Aktualisierung wird unter den gleichen `timeoutMs`- und Fehlerregeln wie der Startup-Durchlauf ausgeführt.

Mit `--debug` schreibt Claude Code den stderr des Helpers aus jedem Durchlauf in das [Debug-Protokoll](/docs/de/debug-your-config).

Claude Code meldet einen ungültigen `policyHelper`-Wert als einen [gelöschten Eintrag](/docs/de/managed-settings#find-entries-claude-code-dropped) und startet die Sitzung auf den verbleibenden verwalteten Einstellungen ohne Ausführung eines Helpers. Ungültige Werte umfassen einen bloßen Pfad-String und einen `timeoutMs` unter [seinem Minimum](#policyhelper-timeoutms).

Um einen Helper auszuschalten, entfernen Sie den Schlüssel aus der Quelle, die ihn setzt.

<h3 id="policyhelper-path">
  `policyHelper.path`
</h3>

Benennen Sie die ausführbare Datei des Helpers, die Claude Code ausführt. Für das, was passiert, wenn der Pfad die folgenden Regeln bricht, siehe [Helper-Fehler](#helper-failures).

* **Bereich**: [`Managed`](#scopes). Lesen Sie aus der macOS plist, der Windows HKLM-Registrierung oder der verwalteten Einstellungsdatei, wo [`policyHelper`](#policyhelper) gelesen wird.
* **Typ**: string, ein absoluter Pfad in normalisierter Form, ohne `.` oder `..` Segmente; unter Windows ein Laufwerk-Buchstaben- oder UNC-Pfad, der in `.exe` endet
* **Standard**: keine; erforderlich, wenn `policyHelper` gesetzt ist

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

Legen Sie fest, wie lange Claude Code auf den Helper wartet, bevor der Durchlauf als fehlgeschlagen behandelt wird. Ein abgelaufener Durchlauf schlägt auf die gleiche Weise fehl wie ein nicht-null-Exit, sodass Claude Code beim Start sich weigert zu starten.

* **Bereich**: [`Managed`](#scopes). Lesen Sie aus der macOS plist, der Windows HKLM-Registrierung oder der verwalteten Einstellungsdatei, wo [`policyHelper`](#policyhelper) gelesen wird.
* **Typ**: integer, Millisekunden, Minimum `1000`
* **Standard**: `10000`

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

Lassen Sie Claude Code den Helper im Hintergrund in einem Intervall erneut ausführen, sodass Richtlinienänderungen eine laufende Sitzung erreichen. Wenn eine Aktualisierung erfolgreich ist, ersetzt ihre Ausgabe die vorherigen verwalteten Einstellungen ohne einen Neustart; wenn eine Aktualisierung fehlschlägt, behält Claude Code die Richtlinie, die es bereits hat.

* **Bereich**: [`Managed`](#scopes). Lesen Sie aus der macOS plist, der Windows HKLM-Registrierung oder der verwalteten Einstellungsdatei, wo [`policyHelper`](#policyhelper) gelesen wird.
* **Typ**: integer, Millisekunden: `0` zum Deaktivieren der Aktualisierung, andernfalls mindestens `60000`
* **Standard**: nicht gesetzt, sodass Claude Code den Helper einmal beim Start ausführt

Dieses Beispiel führt den Helper alle fünf Minuten erneut aus:

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

Lassen Sie Claude Code auf WSL verwaltete Einstellungen aus der Windows-Richtlinienkette lesen, wobei HKLM und die Windows-verwaltete Einstellungsdatei Vorrang vor `/etc/claude-code` und HKCU darunter haben. Während die Kette aktiv ist, liest Claude Code `/etc/claude-code` nur, wenn keine verwaltete Einstellungsdatei oder Drop-in unter `C:\Program Files\ClaudeCode\` einen [Richtlinienschlüssel](/docs/de/managed-settings#how-claude-code-combines-managed-sources) bereitstellt. Setzen Sie dies, um die Richtlinie, die Sie bereits unter Windows bereitstellen, auf WSL-Sitzungen auf dem gleichen Computer zu erweitern, sodass sie den gleichen Regeln wie Host-Sitzungen folgen. Claude Code berücksichtigt ihn nur, wenn er im HKLM-Registrierungsschlüssel oder in einer verwalteten Einstellungsdatei oder Drop-in unter `C:\Program Files\ClaudeCode\` gesetzt ist, die beide Windows-Admin-Schreibzugriff erfordern.

* **Bereich**: [`Managed`](#scopes). In einer admin-kontrollierten Windows-Quelle.
* **Typ**: Boolean
  * `true`: Claude Code auf WSL liest verwaltete Einstellungen aus der Windows-Richtlinienkette und liest `/etc/claude-code` nur, wenn keine verwaltete Einstellungsdatei oder Drop-in unter `C:\Program Files\ClaudeCode\` einen [Richtlinienschlüssel](/docs/de/managed-settings#how-claude-code-combines-managed-sources) bereitstellt
  * `false`: WSL liest nur `/etc/claude-code`
* **Standard**: `false`, sodass WSL nur `/etc/claude-code` liest

```json managed-settings.json theme={null}
{
  "wslInheritsWindowsSettings": true
}
```

Sobald eine Admin-Quelle die Kette einschaltet, tritt die HKCU-Richtlinie auf WSL nur bei, wenn HKCU auch den Schlüssel auf `true` setzt. Diese Kopie schaltet die Kette nicht von selbst ein. Eine Windows-Quelle, die nur diesen Schlüssel enthält, zählt nicht als Richtlinienquelle, sodass eine niedrigere Prioritätsquelle immer noch die Richtlinie liefert. Dieser Schlüssel hat keine Auswirkung auf natives Windows.

<h2 id="global-config-settings">
  Globale Konfigurationseinstellungen
</h2>

Speichern Sie diese Schlüssel in `~/.claude.json`, nicht in einer Einstellungsdatei. Claude Code ignoriert sie überall sonst. Claude Code und `/config` schreiben die meisten davon für Sie, und Sie können sie auch manuell bearbeiten.

<h3 id="autoconnectide">
  `autoConnectIde`
</h3>

Verbinden Sie sich automatisch mit einer laufenden IDE, wenn Sie Claude Code von einem externen Terminal aus starten. Erscheint in `/config` als **Auto-Verbindung zur IDE (externes Terminal)**, wenn Sie Claude Code außerhalb eines VS Code- oder JetBrains-Terminals ausführen.

* **Bereich**: [`Global config`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code verbindet sich automatisch mit einer laufenden IDE, wenn Sie es von einem externen Terminal aus starten
  * `false`: Claude Code verbindet sich nicht automatisch von einem externen Terminal aus; innerhalb eines VS Code- oder JetBrains-Terminals oder mit `--ide` verbindet es sich trotzdem
* **Standard**: `false`
* **Sitzungsübergreifende Überschreibungen**: [`CLAUDE_CODE_AUTO_CONNECT_IDE`](/docs/de/env-vars) hat Vorrang vor diesem Schlüssel für eine Sitzung, in beide Richtungen

```json ~/.claude.json theme={null}
{
  "autoConnectIde": true
}
```

Claude Code ignoriert diesen Schlüssel in `settings.json`.

<h3 id="autoinstallideextension">
  `autoInstallIdeExtension`
</h3>

Installieren Sie die Claude Code IDE-Erweiterung automatisch, wenn Sie Claude Code von einem VS Code-Terminal aus ausführen. Erscheint in `/config` als **Auto-Install IDE-Erweiterung**, wenn Sie Claude Code innerhalb eines VS Code- oder JetBrains-Terminals ausführen.

* **Bereich**: [`Global config`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code installiert die IDE-Erweiterung automatisch, wenn Sie es von einem VS Code-Terminal aus ausführen
  * `false`: Claude Code installiert die Erweiterung nicht automatisch
* **Standard**: `true`
* **Sitzungsübergreifende Überschreibungen**: [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/de/env-vars) auf `1` gesetzt überspringt die Installation für eine Sitzung, auch wenn dieser Schlüssel `true` ist

```json ~/.claude.json theme={null}
{
  "autoInstallIdeExtension": false
}
```

Claude Code ignoriert diesen Schlüssel in `settings.json`.

<h3 id="copyonselect">
  `copyOnSelect`
</h3>

Kopieren Sie Text automatisch in Ihre Zwischenablage, wenn Sie die Auswahl mit der Maus in der [Vollbilddarstellung](/docs/de/fullscreen#use-the-mouse) oder [Agent-Ansicht](/docs/de/agent-view) beenden. Erscheint in `/config` als **Beim Auswählen kopieren**, während die Vollbilddarstellung aktiviert ist.

* **Bereich**: [`Global config`](#scopes)
* **Typ**: Boolean
  * `true`: Claude Code kopiert Text in Ihre Zwischenablage, wenn Sie die Auswahl beenden
  * `false`: Das Auswählen von Text lässt Ihre Zwischenablage unverändert, und Sie [kopieren die Auswahl stattdessen mit einer Tastenkombination](/docs/de/fullscreen#use-the-mouse)
* **Standard**: `true`

```json ~/.claude.json theme={null}
{
  "copyOnSelect": false
}
```

Claude Code ignoriert diesen Schlüssel in `settings.json`.

<h3 id="difftool">
  `diffTool`
</h3>

Wählen Sie, wo Claude Code den Diff einer `Edit`- oder `Write`-Änderung anzeigt, die es vorschlägt, wenn eine [VS Code](/docs/de/vs-code)- oder [JetBrains](/docs/de/jetbrains#features)-IDE verbunden ist: `"auto"` öffnet es im Diff-Viewer der IDE, `"terminal"` behält es im Terminal. Erscheint in `/config` als **Diff-Tool** nur, wenn Claude Code mit einer VS Code- oder JetBrains-IDE verbunden ist.

* **Bereich**: [`Global config`](#scopes)
* **Typ**: string, einer von:
  * `"auto"`: Claude Code öffnet den Diff im Diff-Viewer der IDE, wenn eine VS Code- oder JetBrains-IDE verbunden ist
  * `"terminal"`: Claude Code behält den Diff im Terminal
* **Standard**: `"auto"`

```json ~/.claude.json theme={null}
{
  "diffTool": "terminal"
}
```

Claude Code ignoriert diesen Schlüssel in `settings.json`.

<h3 id="externaleditorcontext">
  `externalEditorContext`
</h3>

Wenn Sie `Ctrl+G` drücken, öffnet Claude Code die Eingabeaufforderung, die Sie eingeben, in Ihrem [externen Editor](/docs/de/interactive-mode#general-controls). Mit diesem Schlüssel aktiviert, startet der Editor-Puffer mit Claudes vorheriger Antwort als `#`-Kommentarzeilen, damit Sie sie lesen können, während Sie schreiben, und Claude Code entfernt diese Zeilen, wenn Sie speichern. Erscheint in `/config` als **Letzte Antwort im externen Editor anzeigen**.

* **Bereich**: [`Global config`](#scopes)
* **Typ**: Boolean
  * `true`: Der Editor-Puffer startet mit Claudes vorheriger Antwort als `#`-Kommentarzeilen, die Claude Code beim Speichern entfernt
  * `false`: Der Editor-Puffer öffnet sich nur mit Ihrer Eingabeaufforderung
* **Standard**: `false`

```json ~/.claude.json theme={null}
{
  "externalEditorContext": true
}
```

Mit aktiviertem Schlüssel sieht der Puffer, den Claude Code öffnet, so aus, und nur der Text unter der Markierungszeile wird als Ihre Eingabeaufforderung gesendet:

```text theme={null}
# ─── Claudes letzte Antwort (zur Referenz; beim Speichern entfernt) ───
# Ich habe die Wiederholungsschleife zu fetchUser in src/api.ts hinzugefügt
# und einen Test für den Timeout-Fall. Soll ich dasselbe Retry in
# fetchOrders einbauen?
# ─── Schreiben Sie Ihre Antwort unter dieser Zeile ──────────────────────

Ja, und begrenzen Sie es auf drei Versuche.
```

Claude Code behält die letzten 50 Zeilen der Antwort und markiert den Schnitt mit `# … (frühere Ausgabe gekürzt)`.

Claude Code ignoriert diesen Schlüssel in `settings.json`.

<h3 id="permissionexplainerenabled">
  `permissionExplainerEnabled`
</h3>

<Warning>
  Entfernt in v2.1.257, zusammen mit der `Ctrl+E`-Befehlserklärung auf Bash- und PowerShell-Berechtigungsaufforderungen. Das Setzen hat keine Auswirkung auf aktuelle Versionen.
</Warning>

Bis v2.1.256 konnten Sie `Ctrl+E` auf einer Bash- oder PowerShell-Berechtigungsaufforderung drücken, um eine modellgenerierte Erklärung des Befehls zu sehen, und diesen Schlüssel auf `false` setzen, um diese Tastenkombination auszuschalten.

* **Bereich**: [`Global config`](#scopes). Auf v2.1.256 und früher.
* **Typ**: Boolean
* **Standard**: `true`

<h3 id="teammatedefaultmodel">
  `teammateDefaultModel`
</h3>

<Warning>
  Entfernt in v2.1.234, zusammen mit seiner `/config`-Zeile **Standard-Teamkollegen-Modell**. Das Setzen hat keine Auswirkung auf aktuelle Versionen.
</Warning>

Bis v2.1.233 setzen Sie diesen Schlüssel auf das Modell für [Agent-Team](/docs/de/agent-teams#specify-teammates-and-models)-Teamkollegen, für die Ihre Eingabeaufforderung kein Modell benannt hat: ein Alias wie `"sonnet"` oder `null`, um dem Modell des Leads zu folgen. Für das Modell, das Claude Code für solche Teamkollegen jetzt auswählt, siehe [Teamkollegen und Modelle angeben](/docs/de/agent-teams#specify-teammates-and-models).

* **Bereich**: [`Global config`](#scopes). Auf v2.1.233 und früher.
* **Typ**: string, ein Modell-Alias oder vollständige Modell-ID oder `null`
* **Standard**: nicht gesetzt

<h2 id="see-also">
  Siehe auch
</h2>

* [Berechtigungen konfigurieren](/docs/de/permissions): Regelsyntax, Berechtigungsmodi und Workspace-Vertrauen
* [Umgebungsvariablen](/docs/de/env-vars): jede `CLAUDE_*`, `ANTHROPIC_*` und Provider-Variable, die Claude Code liest
* [Für Claude verfügbare Tools](/docs/de/tools-reference): die integrierten Tools und welche Genehmigung benötigen
* [Beispieleinstellungsdateien](/docs/de/settings-example): eine persönliche Datei, eine Team-Datei und eine verwaltete Datei einer Organisation
* [Verwaltete Einstellungen einrichten](/docs/de/admin-setup): wie Organisationen entscheiden, was erzwungen werden soll
* [Verwaltete Einstellungen bereitstellen](/docs/de/managed-settings): Bereitstellungsmechanismen, Vorrang innerhalb der verwalteten Ebene und ungültige Einträge in verwalteten Einstellungen
* [Konfiguration debuggen](/docs/de/debug-your-config): `claude doctor` und der Dialog „Einstellungsfehler"
