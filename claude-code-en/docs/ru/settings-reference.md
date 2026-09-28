> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Все параметры

> Полный справочник по каждому ключу settings.json в Claude Code: где находится каждый ключ, его тип и значение по умолчанию, а также готовый к использованию пример и индекс всех ключей.

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

<BackToIndex href="#all-settings" label="Back to index" />

На этой справочной странице перечислены все ключи, которые Claude Code читает из файла параметров, а также [небольшая группа ключей](#global-config-settings), которые он хранит в `~/.claude.json`. Чтобы выбрать файл или проверить приоритет, начните с [Settings files and precedence](/docs/ru/settings).

<span id="available-settings" />

<span id="scopes" />

<span id="all-settings" />

<h2 id="settings-index">
  Индекс параметров
</h2>

Каждый ключ ниже ссылается на его запись. Область действия указывает на [файлы](/docs/ru/settings#settings-files-and-who-they-affect), в которых он может находиться: `User` — это `~/.claude/settings.json`, `Project` — это `.claude/settings.json`, `Local` — это `.claude/settings.local.json`, а `Managed` — это [то, что развёртывает ваша организация](/docs/ru/managed-settings). `Any file` означает все четыре, а `Global config` означает [`~/.claude.json`](#global-config-settings).

<ReferenceFilter
  noun="settings"
  placeholder="Filter settings by key or purpose"
  facetOrder={{ scope: ["Any file", "User, local, or managed", "User or managed", "Managed", "Global config"] }}
  columnHelp={{
topic: "The section of this page that holds the entry. Use Sort by to group the table by topic.",
scope: "Which settings files can set the key: user (~/.claude/settings.json), project (.claude/settings.json), local (.claude/settings.local.json), or managed (deployed by your organization). Global config keys are in ~/.claude.json instead.",
}}
/>

| Key                                                                                                   | Description                                                                                                                                                                                                                                                             | Topic                              | Scope                   |
| :---------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------- | :---------------------- |
| [`advisorModel`](#advisormodel)                                                                       | Выберите, какая модель отвечает, когда Claude использует [инструмент советника](/docs/ru/advisor)                                                                                                                                                                            | Model and responses                | Any file                |
| [`agent`](#agent)                                                                                     | Начните каждый сеанс как именованный [подагент](/docs/ru/sub-agents) с его подсказкой, инструментами и моделью                                                                                                                                                               | Agents, sessions, and worktrees    | Any file                |
| [`agentPushNotifEnabled`](#agentpushnotifenabled)                                                     | Позвольте Claude отправить [push-уведомление на ваш телефон](/docs/ru/remote-control#mobile-push-notifications), когда он решит это сделать                                                                                                                                  | Remote, desktop, and notifications | Any file                |
| [`allowAllClaudeAiMcps`](#allowallclaudeaimcps)                                                       | Загружайте [разъёмы claude.ai](/docs/ru/mcp), которые Claude Code получает сам, наряду с развёрнутым [`managed-mcp.json`](/docs/ru/managed-mcp#exclusive-control-with-managed-mcp-json)                                                                                           | MCP                                | Managed                 |
| [`allowedChannelPlugins`](#allowedchannelplugins)                                                     | Замените список разрешений по умолчанию для [плагинов канала](/docs/ru/channels#restrict-which-channel-plugins-can-run), которые могут отправлять сообщения                                                                                                                  | Plugins and skills                 | Managed                 |
| [`allowedHttpHookUrls`](#allowedhttphookurls)                                                         | Ограничьте, какие URL-адреса могут использовать [HTTP hooks](/docs/ru/hooks)                                                                                                                                                                                                 | Hooks and automation               | Any file                |
| [`allowedMcpServers`](#allowedmcpservers)                                                             | Список разрешений для [MCP серверов](/docs/ru/mcp), которые пользователи могут добавлять                                                                                                                                                                                     | MCP                                | Any file                |
| [`allowManagedHooksOnly`](#allowmanagedhooksonly)                                                     | Запускайте только [hooks](/docs/ru/hooks), которые развёртывает ваша организация                                                                                                                                                                                             | Hooks and automation               | Managed                 |
| [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly)                                           | Сделайте список разрешений управляемого [MCP](/docs/ru/mcp) единственным применяемым                                                                                                                                                                                         | MCP                                | Managed                 |
| [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly)                                 | Сделайте [управляемые параметры](/docs/ru/managed-settings) единственным источником параметров для [правил разрешений](/docs/ru/permissions#managed-settings)                                                                                                                     | Permission settings                | Managed                 |
| [`alwaysThinkingEnabled`](#alwaysthinkingenabled)                                                     | Отключите [расширенное мышление](/docs/ru/model-config#extended-thinking) для каждого сеанса                                                                                                                                                                                 | Model and responses                | Any file                |
| [`apiKeyHelper`](#apikeyhelper)                                                                       | Создайте [учётные данные API](/docs/ru/authentication#credential-management) с помощью вашей собственной команды                                                                                                                                                             | Authentication and providers       | Any file                |
| [`askUserQuestionTimeout`](#askuserquestiontimeout)                                                   | Позвольте неотвеченному вопросу [автоматически продолжиться](/docs/ru/tools-reference#question-auto-continue-timeout) после времени простоя                                                                                                                                  | Interface and terminal             | User or managed         |
| [`attribution`](#attribution)                                                                         | Настройте атрибуцию, которую Claude Code добавляет к коммитам и pull-запросам                                                                                                                                                                                           | Git and attribution                | Any file                |
| [`attribution.commit`](#attribution-commit)                                                           | Измените или скройте трейлер, который Claude Code добавляет к коммитам                                                                                                                                                                                                  | Git and attribution                | Any file                |
| [`attribution.pr`](#attribution-pr)                                                                   | Измените или скройте строку атрибуции в описаниях pull-запросов                                                                                                                                                                                                         | Git and attribution                | Any file                |
| [`attribution.sessionUrl`](#attribution-sessionurl)                                                   | Опустите ссылку на сеанс claude.ai из коммитов [облака](/docs/ru/claude-code-on-the-web) и [Remote Control](/docs/ru/remote-control)                                                                                                                                              | Git and attribution                | Any file                |
| [`autoCompactEnabled`](#autocompactenabled)                                                           | Отключите или включите [автоматическое сжатие](/docs/ru/context-window)                                                                                                                                                                                                      | Memory and context                 | Any file                |
| [`autoCompactWindow`](#autocompactwindow)                                                             | Установите, насколько полным становится контекст перед [сжатием](/docs/ru/context-window) Claude Code                                                                                                                                                                        | Memory and context                 | Any file                |
| [`autoConnectIde`](#autoconnectide)                                                                   | Автоматически подключайтесь к работающей IDE [VS Code](/docs/ru/vs-code) или [JetBrains](/docs/ru/jetbrains#from-external-terminals) из внешнего терминала                                                                                                                        | Global config settings             | Global config           |
| [`autoContinueAtUsageLimit`](#autocontinueatusagelimit)                                               | Ждите в открытом сеансе и [продолжайте задачу автоматически](/docs/ru/interactive-mode#wait-for-a-usage-limit-to-reset) после сброса лимита использования claude.ai                                                                                                          | Interface and terminal             | User or managed         |
| [`autoInstallIdeExtension`](#autoinstallideextension)                                                 | Отключите автоматическую установку [расширения IDE](/docs/ru/vs-code#install-the-extension) из терминала VS Code                                                                                                                                                             | Global config settings             | Global config           |
| [`autoMemoryDirectory`](#automemorydirectory)                                                         | Сохраняйте [автоматическую память](/docs/ru/memory#auto-memory) в выбранном вами каталоге                                                                                                                                                                                    | Memory and context                 | Any file                |
| [`autoMemoryEnabled`](#automemoryenabled)                                                             | Отключите или включите [автоматическую память](/docs/ru/memory#auto-memory)                                                                                                                                                                                                  | Memory and context                 | Any file                |
| [`autoMode`](#automode)                                                                               | Добавьте свои собственные правила разрешения и запрета к классификатору [автоматического режима](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode)                                                                                                                 | Permission settings                | User or managed         |
| [`autoMode.classifyAllShell`](#automode-classifyallshell)                                             | Отправляйте каждую команду shell через [классификатор автоматического режима](/docs/ru/permission-modes#what-the-classifier-blocks-by-default), даже те, которые соответствуют узкому правилу разрешения                                                                     | Permission settings                | User or managed         |
| [`autoScrollEnabled`](#autoscrollenabled)                                                             | [Следите за новым выводом](/docs/ru/fullscreen#auto-follow) в нижней части при полноэкранном отображении                                                                                                                                                                     | Interface and terminal             | Any file                |
| [`autoUpdatesChannel`](#autoupdateschannel)                                                           | Следуйте стабильному [каналу выпуска](/docs/ru/setup#configure-release-channel) вместо последнего                                                                                                                                                                            | Updates and versioning             | Any file                |
| [`availableModels`](#availablemodels)                                                                 | [Ограничьте, какие модели](/docs/ru/model-config#restrict-model-selection) люди могут выбирать                                                                                                                                                                               | Model and responses                | Any file                |
| [`awaySummaryEnabled`](#awaysummaryenabled)                                                           | Отключите [краткое описание сеанса](/docs/ru/interactive-mode#session-recap), показываемое при возврате в терминал                                                                                                                                                           | Remote, desktop, and notifications | Any file                |
| [`awsAuthRefresh`](#awsauthrefresh)                                                                   | Обновляйте истёкшие [учётные данные Bedrock](/docs/ru/amazon-bedrock#advanced-credential-configuration) в `.aws` с помощью вашей собственной команды                                                                                                                         | Authentication and providers       | Any file                |
| [`awsCredentialExport`](#awscredentialexport)                                                         | Предоставляйте [учётные данные Bedrock](/docs/ru/amazon-bedrock#advanced-credential-configuration) как JSON из вашей собственной команды                                                                                                                                     | Authentication and providers       | Any file                |
| [`axScreenReader`](#axscreenreader)                                                                   | Отображайте [вывод, удобный для программ чтения с экрана](/docs/ru/accessibility)                                                                                                                                                                                            | Interface and terminal             | Any file                |
| [`bashEditDiffEnabled`](#basheditdiffenabled)                                                         | Записывайте [файлы, которые изменила команда Bash](/docs/ru/hooks#bash) в каждом режиме разрешений                                                                                                                                                                           | Interface and terminal             | User or managed         |
| [`bashOutputMaxChars`](#bashoutputmaxchars)                                                           | Установите, сколько [вывода](/docs/ru/tools-reference#output-limits) успешной команды Claude получает встроенным образом                                                                                                                                                     | Memory and context                 | Any file                |
| [`blockedMarketplaces`](#blockedmarketplaces)                                                         | Блокируйте источники [маркетплейса плагинов](/docs/ru/plugins/overview) для вашей организации                                                                                                                                                                                | Plugins and skills                 | Managed                 |
| [`browserExternalPageTools`](#browserexternalpagetools)                                               | Отключите инструменты Claude на внешних страницах в [панели Browser](/docs/ru/desktop) рабочего стола                                                                                                                                                                        | Tools                              | Managed                 |
| [`channelsEnabled`](#channelsenabled)                                                                 | Разрешите [каналы](/docs/ru/channels#enable-channels-for-your-organization) для вашей организации                                                                                                                                                                            | Plugins and skills                 | Managed                 |
| [`claudeMd`](#claudemd)                                                                               | Внедрите инструкции [CLAUDE.md](/docs/ru/memory#deploy-organization-wide-claude-md) на уровне организации из управляемых параметров                                                                                                                                          | Memory and context                 | Managed                 |
| [`claudeMdExcludes`](#claudemdexcludes)                                                               | Пропустите определённые файлы [CLAUDE.md](/docs/ru/memory#exclude-specific-claude-md-files) при загрузке памяти                                                                                                                                                              | Memory and context                 | Any file                |
| [`cleanupPeriodDays`](#cleanupperioddays)                                                             | Выберите, сколько дней Claude Code хранит [стенограммы](/docs/ru/data-usage#data-retention) перед их удалением                                                                                                                                                               | Privacy and telemetry              | Any file                |
| [`companyAnnouncements`](#companyannouncements)                                                       | Показывайте объявления вашей организации при запуске                                                                                                                                                                                                                    | Interface and terminal             | Any file                |
| [`copyOnSelect`](#copyonselect)                                                                       | Отключите автоматическое копирование текста, который вы выбираете мышью при [полноэкранном отображении](/docs/ru/fullscreen#use-the-mouse) и в представлении агента                                                                                                          | Global config settings             | Global config           |
| [`crossSessionInbound`](#crosssessioninbound)                                                         | Выберите, доставляет ли Claude Code [сообщения из ваших других сеансов](/docs/ru/cross-session-messaging#control-inbound-messages), показывает уведомление без доставки или отказывает в них                                                                                 | Agents, sessions, and worktrees    | Any file                |
| [`defaultShell`](#defaultshell)                                                                       | Выберите, запускается ли Bash или PowerShell для команд shell, которые вы вводите с префиксом [`!`](/docs/ru/interactive-mode#shell-mode-with-prefix)                                                                                                                        | Interface and terminal             | Any file                |
| [`deniedMcpServers`](#deniedmcpservers)                                                               | Блокируйте определённые [MCP серверы](/docs/ru/mcp) по URL, команде или имени                                                                                                                                                                                                | MCP                                | Any file                |
| [`desktopSessionCleanupPeriodDays`](#desktopsessioncleanupperioddays)                                 | Установите ограничение по возрасту в днях для [стенограмм Claude Desktop и Cowork](/docs/ru/claude-directory#cleaned-up-automatically)                                                                                                                                       | Privacy and telemetry              | User or managed         |
| [`dialogExpiry`](#dialogexpiry)                                                                       | Установите, как долго Claude Code ждёт ответа [Remote Control](/docs/ru/remote-control) или хоста SDK на переданный диалог перед отменой диалога                                                                                                                             | Interface and terminal             | User or managed         |
| [`diffTool`](#difftool)                                                                               | Выберите, открываются ли предложенные Claude изменения файлов в средстве просмотра diff [VS Code](/docs/ru/vs-code) или [JetBrains](/docs/ru/jetbrains#features) или остаются в терминале                                                                                         | Global config settings             | Global config           |
| [`disableAgentView`](#disableagentview)                                                               | Отключите фоновых агентов и [представление агента](/docs/ru/agent-view)                                                                                                                                                                                                      | Agents, sessions, and worktrees    | Any file                |
| [`disableAllHooks`](#disableallhooks)                                                                 | Отключите [hooks](/docs/ru/hooks), пользовательскую [строку состояния](/docs/ru/statusline) и пользовательскую команду [`@` предложения файла](/docs/ru/interactive-mode#quick-commands) одновременно                                                                                  | Hooks and automation               | Any file                |
| [`disableArtifact`](#disableartifact)                                                                 | Устарело; используйте `enableArtifact` для отключения [инструмента Artifact](/docs/ru/artifacts)                                                                                                                                                                             | Remote, desktop, and notifications | Any file                |
| [`disableAutoMode`](#disableautomode)                                                                 | Удалите [автоматический режим](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode) из цикла режима разрешений                                                                                                                                                        | Permission settings                | Any file                |
| [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation)                               | Ограничьте [панель Browser](/docs/ru/desktop) рабочего стола localhost для людей и Claude                                                                                                                                                                                    | Tools                              | Managed                 |
| [`disableBundledSkills`](#disablebundledskills)                                                       | Отключите [навыки](/docs/ru/skills#bundled-skills) и [рабочие процессы](/docs/ru/workflows), включённые в Claude Code                                                                                                                                                             | Plugins and skills                 | Any file                |
| [`disableClaudeAiConnectors`](#disableclaudeaiconnectors)                                             | Отключите [разъёмы claude.ai](/docs/ru/mcp#disable-claude-ai-connectors), чтобы Claude Code их не получал                                                                                                                                                                    | MCP                                | Any file                |
| [`disableCommandPluginSources`](#disablecommandpluginsources)                                         | Блокируйте [плагины](/docs/ru/plugins/overview), которые устанавливаются путём запуска команды, объявленной маркетплейсом                                                                                                                                                    | Plugins and skills                 | Managed                 |
| [`disableDeepLinkRegistration`](#disabledeeplinkregistration)                                         | Остановите регистрацию Claude Code обработчика [`claude-cli://`](/docs/ru/deep-links)                                                                                                                                                                                        | Remote, desktop, and notifications | Any file                |
| [`disableDesktopLocalSessions`](#disabledesktoplocalsessions)                                         | Отключите [сеансы Desktop Code](/docs/ru/desktop#local-sessions-on-managed-devices), которые работают на устройстве, оставляя SSH для других хостов и облака                                                                                                                 | Remote, desktop, and notifications | Managed                 |
| [`disabledMcpjsonServers`](#disabledmcpjsonservers)                                                   | Отклоняйте определённые серверы из [`.mcp.json`](/docs/ru/mcp#project-scope) проекта                                                                                                                                                                                         | MCP                                | Any file                |
| [`disableMobileSimulatorTools`](#disablemobilesimulatortools)                                         | Блокируйте инструменты Claude в [панели iOS Simulator](/docs/ru/desktop) рабочего стола                                                                                                                                                                                      | Tools                              | Managed                 |
| [`disableRemoteControl`](#disableremotecontrol)                                                       | Отключите [Remote Control](/docs/ru/remote-control) везде, где он может запуститься                                                                                                                                                                                          | Remote, desktop, and notifications | Any file                |
| [`disableSideloadFlags`](#disablesideloadflags)                                                       | Отклоняйте флаги CLI, которые загружают [плагины](/docs/ru/plugins/overview), [подагентов](/docs/ru/sub-agents) и [MCP серверы](/docs/ru/mcp)                                                                                                                                          | Enterprise and managed settings    | Managed                 |
| [`disableSkillShellExecution`](#disableskillshellexecution)                                           | Остановите [навыки](/docs/ru/skills) и пользовательские команды от встроенного выполнения shell                                                                                                                                                                              | Plugins and skills                 | Any file                |
| [`disableWorkflows`](#disableworkflows)                                                               | Отключите [динамические рабочие процессы](/docs/ru/workflows) для всех; используйте `enableWorkflows` для себя                                                                                                                                                               | Hooks and automation               | Any file                |
| [`editorMode`](#editormode)                                                                           | Используйте [сочетания клавиш vim](/docs/ru/interactive-mode#vim-editor-mode) в приглашении ввода                                                                                                                                                                            | Interface and terminal             | Any file                |
| [`effortLevel`](#effortlevel)                                                                         | Установите уровень [усилий](/docs/ru/model-config#adjust-effort-level) по умолчанию для моделей без сохранённого уровня                                                                                                                                                      | Model and responses                | Any file                |
| [`emojiCompletionEnabled`](#emojicompletionenabled)                                                   | Отключите [предложения и замену эмодзи `:shortcode:`](/docs/ru/interactive-mode#emoji-shortcodes) в приглашении ввода                                                                                                                                                        | Interface and terminal             | Any file                |
| [`enableAllProjectMcpServers`](#enableallprojectmcpservers)                                           | Одобрите каждый сервер в файлах проекта [`.mcp.json`](/docs/ru/mcp#project-server-approvals-and-workspace-trust) без подсказки                                                                                                                                               | MCP                                | Any file                |
| [`enableArtifact`](#enableartifact)                                                                   | Отключите [инструмент Artifact](/docs/ru/artifacts) с помощью `false` в любом файле; ни один файл не может его включить обратно                                                                                                                                              | Remote, desktop, and notifications | Any file                |
| [`enabledMcpjsonServers`](#enabledmcpjsonservers)                                                     | Одобрите определённые серверы из [`.mcp.json`](/docs/ru/mcp#project-server-approvals-and-workspace-trust) проекта                                                                                                                                                            | MCP                                | Any file                |
| [`enabledPlugins`](#enabledplugins)                                                                   | Включайте или отключайте отдельные [плагины](/docs/ru/plugins/overview) для каждой области действия                                                                                                                                                                          | Plugins and skills                 | Any file                |
| [`enableWorkflows`](#enableworkflows)                                                                 | Включайте или отключайте [динамические рабочие процессы](/docs/ru/workflows) в зависимости от умолчания вашего плана                                                                                                                                                         | Hooks and automation               | Any file                |
| [`enforceAvailableModels`](#enforceavailablemodels)                                                   | Держите [выбор Default](/docs/ru/model-config#enforce-the-allowlist-for-the-default-model) `/model` внутри вашего списка разрешений `availableModels`                                                                                                                        | Model and responses                | Any file                |
| [`env`](#env)                                                                                         | Установите [переменные окружения](/docs/ru/env-vars#in-settings-files) для каждого сеанса и его подпроцессов                                                                                                                                                                 | Memory and context                 | Any file                |
| [`externalEditorContext`](#externaleditorcontext)                                                     | Показывайте последний ответ Claude как комментарии при нажатии [Ctrl+G](/docs/ru/interactive-mode#general-controls) для редактирования                                                                                                                                       | Global config settings             | Global config           |
| [`extraKnownMarketplaces`](#extraknownmarketplaces)                                                   | Зарегистрируйте [маркетплейсы](/docs/ru/plugins/overview) для репозитория или организации                                                                                                                                                                                    | Plugins and skills                 | Any file                |
| [`fallbackModel`](#fallbackmodel)                                                                     | Назовите [резервные модели](/docs/ru/model-config#fallback-model-chains) на случай перегрузки основной                                                                                                                                                                       | Model and responses                | Any file                |
| [`fastMode`](#fastmode)                                                                               | Включите [быстрый режим](/docs/ru/fast-mode) для сеансов, где он доступен                                                                                                                                                                                                    | Model and responses                | Any file                |
| [`fastModePerSessionOptIn`](#fastmodepersessionoptin)                                                 | Требуйте, чтобы люди включали [быстрый режим](/docs/ru/fast-mode) каждый сеанс                                                                                                                                                                                               | Model and responses                | Any file                |
| [`feedbackDrafts`](#feedbackdrafts)                                                                   | Контролируйте, ставит ли Claude в очередь [черновики обратной связи](/docs/ru/tools-reference#sendfeedback-tool-behavior) для вашего рассмотрения                                                                                                                            | Privacy and telemetry              | User or managed         |
| [`feedbackSurveyRate`](#feedbacksurveyrate)                                                           | Измените частоту появления [опроса качества сеанса](/docs/ru/data-usage#session-quality-surveys)                                                                                                                                                                             | Privacy and telemetry              | Any file                |
| [`fileCheckpointingEnabled`](#filecheckpointingenabled)                                               | Отключите или включите снимки файлов, которые [`/rewind`](/docs/ru/checkpointing) восстанавливает                                                                                                                                                                            | Memory and context                 | Any file                |
| [`fileSuggestion`](#filesuggestion)                                                                   | Предоставляйте [автодополнение файла `@`](/docs/ru/interactive-mode#quick-commands) из вашей собственной команды                                                                                                                                                             | Interface and terminal             | Any file                |
| [`footerLinksRegexes`](#footerlinksregexes)                                                           | Превратите ID проблем или рецензий в выводе в [кликабельные ссылки](/docs/ru/statusline#clickable-links) ниже поля ввода                                                                                                                                                     | Interface and terminal             | User or managed         |
| [`forceLoginGatewayUrl`](#forcelogingatewayurl)                                                       | Установите [URL шлюза](/docs/ru/claude-apps-gateway#set-the-gateway-url), к которому подключается экран входа                                                                                                                                                                | Authentication and providers       | Managed                 |
| [`forceLoginMethod`](#forceloginmethod)                                                               | [Ограничьте вход](/docs/ru/authentication#restrict-login-to-your-organization) на claude.ai, Claude Console или [облачный шлюз](/docs/ru/claude-apps-gateway)                                                                                                                     | Authentication and providers       | Any file                |
| [`forceLoginOrgUUID`](#forceloginorguuid)                                                             | [Привяжите входы claude.ai к вашей организации](/docs/ru/authentication#restrict-login-to-your-organization); только управляемый источник это обеспечивает                                                                                                                   | Authentication and providers       | Any file                |
| [`forceRemoteSettingsRefresh`](#forceremotesettingsrefresh)                                           | Блокируйте запуск до [свежей загрузки параметров, управляемых сервером](/docs/ru/server-managed-settings)                                                                                                                                                                    | Enterprise and managed settings    | Managed                 |
| [`gatewayInternalNetworks`](#gatewayinternalnetworks)                                                 | Позвольте `/login` достичь [облачного шлюза](/docs/ru/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) на общедоступном пространстве IPv4, которое ваша организация использует внутри                                                                    | Authentication and providers       | Managed                 |
| [`gcpAuthRefresh`](#gcpauthrefresh)                                                                   | Обновляйте [учётные данные Google Cloud](/docs/ru/google-vertex-ai#advanced-credential-configuration) с помощью вашей собственной команды                                                                                                                                    | Authentication and providers       | Any file                |
| [`hooks`](#hooks)                                                                                     | Запускайте свои собственные команды как [hooks](/docs/ru/hooks) в точках жизненного цикла Claude Code                                                                                                                                                                        | Hooks and automation               | Any file                |
| [`httpHookAllowedEnvVars`](#httphookallowedenvvars)                                                   | Ограничьте, какие переменные окружения [HTTP hooks](/docs/ru/hooks) могут помещать в заголовки                                                                                                                                                                               | Hooks and automation               | Any file                |
| [`includeCoAuthoredBy`](#includecoauthoredby)                                                         | Устарело; используйте `attribution` для скрытия или изменения атрибуции коммита и PR                                                                                                                                                                                    | Git and attribution                | Any file                |
| [`includeGitInstructions`](#includegitinstructions)                                                   | Удалите встроенные инструкции коммита и PR из контекста Claude                                                                                                                                                                                                          | Git and attribution                | Any file                |
| [`inputNeededNotifEnabled`](#inputneedednotifenabled)                                                 | Получайте [push-уведомление](/docs/ru/remote-control#mobile-push-notifications), когда Claude ждёт вас                                                                                                                                                                       | Remote, desktop, and notifications | Any file                |
| [`isolatePeerMachines`](#isolatepeermachines)                                                         | Спросите вас перед тем, как Claude [отправит сообщение одному из ваших сеансов на другой машине](/docs/ru/cross-session-messaging#require-approval-for-cross-machine-messages)                                                                                               | Agents, sessions, and worktrees    | Any file                |
| [`keybindingFlavor`](#keybindingflavor)                                                               | Устарело и не имеет эффекта; сочетания клавиш для редактирования слов всегда [следуют соглашениям readline](/docs/ru/interactive-mode#make-ctrl-w-delete-back-to-whitespace)                                                                                                 | Interface and terminal             | Any file                |
| [`language`](#language)                                                                               | Попросите Claude отвечать на языке, отличном от английского                                                                                                                                                                                                             | Model and responses                | Any file                |
| [`managedMcpServers`](#managedmcpservers)                                                             | Предоставляйте удалённые [MCP серверы](/docs/ru/managed-mcp#provide-servers-through-managed-settings) каждому пользователю наряду с теми, которые они добавляют                                                                                                              | MCP                                | Managed                 |
| [`managedSourcesBehavior`](#managedsourcesbehavior)                                                   | Составляйте каждый [управляемый источник](/docs/ru/managed-settings#how-claude-code-combines-managed-sources), который вы развёртываете, вместо использования только одного с наивысшим приоритетом                                                                          | Enterprise and managed settings    | Managed                 |
| [`maxEffortLevel`](#maxeffortlevel)                                                                   | Ограничьте [уровень усилий](/docs/ru/model-config#adjust-effort-level) для каждой модели или для каждого поставщика                                                                                                                                                          | Model and responses                | Any file                |
| [`minimumVersion`](#minimumversion)                                                                   | Держите [автоматические обновления](/docs/ru/setup#pin-a-minimum-version) от установки чего-либо ниже версии                                                                                                                                                                 | Updates and versioning             | Any file                |
| [`model`](#model)                                                                                     | Измените [модель](/docs/ru/model-config#set-a-default-model-for-new-sessions), с которой Claude Code начинает                                                                                                                                                                | Model and responses                | Any file                |
| [`modelOverrides`](#modeloverrides)                                                                   | [Сопоставьте ID моделей](/docs/ru/model-config#override-model-ids-per-version) с ID вашего поставщика, такими как ARN Bedrock                                                                                                                                                | Model and responses                | Any file                |
| [`modelPicker`](#modelpicker)                                                                         | Выберите, какие модели [средство выбора `/model`](/docs/ru/model-config#available-models) перечисляет, в вашем собственном порядке и с вашими собственными метками                                                                                                           | Model and responses                | User or managed         |
| [`modelPricing`](#modelpricing)                                                                       | Сообщайте о расходах по контрактным ставкам вашей организации вместо цены списка                                                                                                                                                                                        | Model and responses                | Managed                 |
| [`modelSettings`](#modelsettings)                                                                     | Сохраняйте [уровень усилий](/docs/ru/model-config#adjust-effort-level) для каждой модели или ограничивайте усилия одной модели                                                                                                                                               | Model and responses                | Any file                |
| [`otelHeadersHelper`](#otelheadershelper)                                                             | Создавайте ротирующие заголовки [OpenTelemetry](/docs/ru/monitoring-usage#dynamic-headers) с помощью вашей собственной команды                                                                                                                                               | Authentication and providers       | Any file                |
| [`outputStyle`](#outputstyle)                                                                         | Измените роль, тон и формат вывода Claude с помощью [стиля вывода](/docs/ru/output-styles)                                                                                                                                                                                   | Model and responses                | Any file                |
| [`parentSettingsBehavior`](#parentsettingsbehavior)                                                   | Применяйте или отбрасывайте ограничения, которые [хост SDK или IDE](/docs/ru/managed-settings#let-an-embedding-host-add-policy) передаёт при развёртывании [управляемых параметров](/docs/ru/managed-settings)                                                                    | Enterprise and managed settings    | Managed                 |
| [`permissionExplainerEnabled`](#permissionexplainerenabled)                                           | Удалено в v2.1.257 вместе с объяснением команды `Ctrl+E` на подсказках разрешений shell                                                                                                                                                                                 | Global config settings             | Global config           |
| [`permissions`](#permissions)                                                                         | Установите правила разрешения, запроса и запрета и начальный [режим разрешений](/docs/ru/permission-modes)                                                                                                                                                                   | Permission settings                | Any file                |
| [`permissions.additionalDirectories`](#permissions-additionaldirectories)                             | Дайте Claude доступ к файлам в [каталогах вне текущего](/docs/ru/permissions#working-directories)                                                                                                                                                                            | Permission settings                | Any file                |
| [`permissions.allow`](#permissions-allow)                                                             | Одобрите перечисленные [использования инструментов](/docs/ru/permissions#permission-rule-syntax) без подсказки                                                                                                                                                               | Permission settings                | Any file                |
| [`permissions.ask`](#permissions-ask)                                                                 | Всегда запрашивайте перед перечисленными [использованиями инструментов](/docs/ru/permissions#permission-rule-syntax)                                                                                                                                                         | Permission settings                | Any file                |
| [`permissions.blockReadsOutsideWorkingDirectories`](#permissions-blockreadsoutsideworkingdirectories) | Заставьте инструменты файлов отказывать в чтении вне [рабочих каталогов](/docs/ru/permissions#working-directories) в каждом режиме разрешений                                                                                                                                | Permission settings                | Any file                |
| [`permissions.defaultMode`](#permissions-defaultmode)                                                 | Установите [режим разрешений](/docs/ru/permission-modes#which-mode-a-session-starts-in), в котором начинаются новые сеансы                                                                                                                                                   | Permission settings                | Any file                |
| [`permissions.deny`](#permissions-deny)                                                               | Блокируйте перечисленные [использования инструментов](/docs/ru/permissions#permission-rule-syntax), включая чтение файлов, содержащих секреты                                                                                                                                | Permission settings                | Any file                |
| [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode)               | Предотвратите вход кого-либо в [режим bypassPermissions](/docs/ru/permission-modes#skip-all-checks-with-bypasspermissions-mode)                                                                                                                                              | Permission settings                | Any file                |
| [`plansDirectory`](#plansdirectory)                                                                   | Выберите, где [режим плана](/docs/ru/permission-modes#analyze-before-you-edit-with-plan-mode) записывает файлы плана                                                                                                                                                         | Memory and context                 | Any file                |
| [`pluginConfigs`](#pluginconfigs)                                                                     | Сохраняйте ответы, которые вы дали диалогу конфигурации [плагина](/docs/ru/plugins/overview)                                                                                                                                                                                 | Plugins and skills                 | User or managed         |
| [`pluginSuggestionMarketplaces`](#pluginsuggestionmarketplaces)                                       | Выберите, какие [маркетплейсы](/docs/ru/plugins/org#restrict-what-users-can-install) могут выводить предложения по установке плагинов в `/plugin`                                                                                                                            | Plugins and skills                 | Managed                 |
| [`pluginTrustMessage`](#plugintrustmessage)                                                           | Добавьте свой собственный текст к предупреждению о доверии [плагина](/docs/ru/plugins/overview)                                                                                                                                                                              | Plugins and skills                 | Managed                 |
| [`policyHelper`](#policyhelper)                                                                       | Запустите исполняемый файл, который вычисляет [управляемые параметры](/docs/ru/managed-settings#compute-the-policy-with-a-helper-program) при запуске                                                                                                                        | Enterprise and managed settings    | Managed                 |
| [`policyHelper.path`](#policyhelper-path)                                                             | Назовите [вспомогательный исполняемый файл](/docs/ru/managed-settings#compute-the-policy-with-a-helper-program), который запускает Claude Code                                                                                                                               | Enterprise and managed settings    | Managed                 |
| [`policyHelper.refreshIntervalMs`](#policyhelper-refreshintervalms)                                   | Повторно запустите [помощника](/docs/ru/managed-settings#compute-the-policy-with-a-helper-program) в фоновом режиме по расписанию                                                                                                                                            | Enterprise and managed settings    | Managed                 |
| [`policyHelper.timeoutMs`](#policyhelper-timeoutms)                                                   | Установите, как долго Claude Code ждёт [помощника](/docs/ru/managed-settings#compute-the-policy-with-a-helper-program)                                                                                                                                                       | Enterprise and managed settings    | Managed                 |
| [`preferredNotifChannel`](#preferrednotifchannel)                                                     | Выберите [звонок терминала или уведомление рабочего стола](/docs/ru/terminal-config#get-a-terminal-bell-or-notification) для завершения задачи                                                                                                                               | Remote, desktop, and notifications | Any file                |
| [`prefersReducedMotion`](#prefersreducedmotion)                                                       | [Уменьшите или отключите](/docs/ru/accessibility#accessibility-settings) анимацию спиннера, мерцания и вспышки                                                                                                                                                               | Interface and terminal             | Any file                |
| [`processWrapper`](#processwrapper)                                                                   | Запускайте фоновые процессы Claude Code через [корпоративный запускатель](/docs/ru/corporate-launcher) на macOS и Linux                                                                                                                                                      | Agents, sessions, and worktrees    | User or managed         |
| [`promptCacheTtl`](#promptcachettl)                                                                   | Выберите [время жизни кэша подсказки](/docs/ru/prompt-caching#cache-lifetime) для основного разговора                                                                                                                                                                        | Model and responses                | Any file                |
| [`promptSuggestionEnabled`](#promptsuggestionenabled)                                                 | Скройте серые [предложения подсказок](/docs/ru/interactive-mode#prompt-suggestions) в поле ввода                                                                                                                                                                             | Interface and terminal             | Any file                |
| [`prUrlTemplate`](#prurltemplate)                                                                     | Укажите ссылки PR на внутренний инструмент проверки кода вместо github.com                                                                                                                                                                                              | Git and attribution                | Any file                |
| [`remote.defaultEnvironmentId`](#remote-defaultenvironmentid)                                         | Выберите [облачную среду](/docs/ru/cloud-environments) по умолчанию для `claude --cloud`; самостоятельно размещённый ID `ccpool_` доступен только для чтения из параметров пользователя и управляемых параметров и `--settings`                                              | Remote, desktop, and notifications | Any file                |
| [`remoteControlAtStartup`](#remotecontrolatstartup)                                                   | Подключайте [Remote Control](/docs/ru/remote-control#enable-remote-control-for-all-sessions) автоматически при запуске сеанса                                                                                                                                                | Remote, desktop, and notifications | Any file                |
| [`requiredMaximumVersion`](#requiredmaximumversion)                                                   | [Отказывайте в запуске](/docs/ru/setup#pin-a-minimum-version) на версии новее, чем позволяет ваша организация                                                                                                                                                                | Updates and versioning             | Managed                 |
| [`requiredMinimumVersion`](#requiredminimumversion)                                                   | [Отказывайте в запуске](/docs/ru/setup#pin-a-minimum-version) на версии старше, чем требует ваша организация                                                                                                                                                                 | Updates and versioning             | Managed                 |
| [`respectGitignore`](#respectgitignore)                                                               | Держите файлы, игнорируемые git, вне [средства выбора файла `@`](/docs/ru/interactive-mode#quick-commands)                                                                                                                                                                   | Interface and terminal             | Any file                |
| [`respondToBashCommands`](#respondtobashcommands)                                                     | Остановите Claude от ответа после запуска [команды shell `!`](/docs/ru/interactive-mode#shell-mode-with-prefix)                                                                                                                                                              | Interface and terminal             | Any file                |
| [`sandbox`](#sandbox)                                                                                 | [Изолируйте команды Bash](/docs/ru/sandboxing) от вашей файловой системы и сети на macOS, Linux и WSL2                                                                                                                                                                       | Sandbox settings                   | Any file                |
| [`sandbox.allowAppleEvents`](#sandbox-allowappleevents)                                               | Позвольте [изолированным](/docs/ru/sandboxing) командам отправлять Apple Events на macOS                                                                                                                                                                                     | Sandbox settings                   | User or managed         |
| [`sandbox.allowUnsandboxedCommands`](#sandbox-allowunsandboxedcommands)                               | Позвольте Claude повторить заблокированную команду вне [песочницы](/docs/ru/sandboxing#the-unsandboxed-retry-escape-hatch) или запретить это                                                                                                                                 | Sandbox settings                   | Any file                |
| [`sandbox.autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed)                               | Запускайте [изолированные](/docs/ru/sandboxing#auto-allow-mode) команды без подсказки разрешения                                                                                                                                                                             | Sandbox settings                   | Any file                |
| [`sandbox.bwrapPath`](#sandbox-bwrappath)                                                             | Укажите [песочницу](/docs/ru/sandboxing) на двоичный файл bubblewrap вне `PATH`                                                                                                                                                                                              | Sandbox settings                   | Managed                 |
| [`sandbox.credentials`](#sandbox-credentials)                                                         | Скройте или замаскируйте файлы учётных данных и переменные внутри [песочницы](/docs/ru/sandboxing#protect-credentials)                                                                                                                                                       | Sandbox settings                   | Any file                |
| [`sandbox.credentials.allowPlaintextInject`](#sandbox-credentials-allowplaintextinject)               | Позвольте [замаскированным учётным данным](/docs/ru/sandboxing#mask-credentials) достичь простых HTTP-сервисов в доверенных тестовых сетях                                                                                                                                   | Sandbox settings                   | User or managed         |
| [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs)                                       | Свяжите переменные пользовательского имени ключа AWS в одно учётное данные для [повторного подписания](/docs/ru/sandboxing#re-sign-aws-requests)                                                                                                                             | Sandbox settings                   | User or managed         |
| [`sandbox.credentials.envVars`](#sandbox-credentials-envvars)                                         | Отмените установку или замаскируйте переменную окружения внутри [песочницы](/docs/ru/sandboxing#mask-environment-variables)                                                                                                                                                  | Sandbox settings                   | Any file                |
| [`sandbox.credentials.files`](#sandbox-credentials-files)                                             | Блокируйте или замаскируйте чтение файла учётных данных внутри [песочницы](/docs/ru/sandboxing#mask-credential-files)                                                                                                                                                        | Sandbox settings                   | Any file                |
| [`sandbox.credentials.sigv4`](#sandbox-credentials-sigv4)                                             | Выберите, будут ли потоковые, предписанные или [запросы AWS SigV4A](/docs/ru/sandboxing#re-sign-aws-requests) отклонены или пройдут                                                                                                                                          | Sandbox settings                   | User or managed         |
| [`sandbox.enabled`](#sandbox-enabled)                                                                 | Включите [изоляцию Bash](/docs/ru/sandboxing#get-started) на macOS, Linux и WSL2                                                                                                                                                                                             | Sandbox settings                   | Any file                |
| [`sandbox.enableWeakerNestedSandbox`](#sandbox-enableweakernestedsandbox)                             | Запустите Linux [песочницу](/docs/ru/sandboxing) внутри непривилегированного контейнера                                                                                                                                                                                      | Sandbox settings                   | Any file                |
| [`sandbox.enableWeakerNetworkIsolation`](#sandbox-enableweakernetworkisolation)                       | Позвольте `gh`, `gcloud` и `terraform` проверять TLS за MITM-прокси внутри [песочницы](/docs/ru/sandboxing#troubleshooting) на macOS                                                                                                                                         | Sandbox settings                   | Any file                |
| [`sandbox.excludedCommands`](#sandbox-excludedcommands)                                               | Назовите команды, которые всегда запускаются вне [песочницы](/docs/ru/sandboxing)                                                                                                                                                                                            | Sandbox settings                   | Any file                |
| [`sandbox.failIfUnavailable`](#sandbox-failifunavailable)                                             | Отказывайте в запуске, когда [песочница](/docs/ru/sandboxing) недоступна, вместо запуска без изоляции                                                                                                                                                                        | Sandbox settings                   | Any file                |
| [`sandbox.filesystem`](#sandbox-filesystem)                                                           | Контролируйте, какие пути [изолированные](/docs/ru/sandboxing#filesystem-isolation) команды могут читать и писать                                                                                                                                                            | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly)       | Остановите разработчиков от повторного открытия [путей чтения, которые ваша организация заблокировала](/docs/ru/sandboxing#keep-developers-from-widening-the-policy)                                                                                                         | Sandbox settings                   | Managed                 |
| [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread)                                       | Повторно откройте чтение внутри региона, который [`denyRead`](#sandbox-filesystem-denyread) блокирует                                                                                                                                                                   | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.allowWrite`](#sandbox-filesystem-allowwrite)                                     | Добавьте пути, в которые [изолированные](/docs/ru/sandboxing) команды могут писать                                                                                                                                                                                           | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread)                                         | Блокируйте [изолированные](/docs/ru/sandboxing) команды от чтения определённых путей                                                                                                                                                                                         | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.denyWrite`](#sandbox-filesystem-denywrite)                                       | Блокируйте [изолированные](/docs/ru/sandboxing) команды от записи в определённые пути                                                                                                                                                                                        | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.disabled`](#sandbox-filesystem-disabled)                                         | [Отключите изоляцию файловой системы](/docs/ru/sandboxing#disable-filesystem-isolation) при сохранении изоляции сети                                                                                                                                                         | Sandbox settings                   | User or managed         |
| [`sandbox.ignoreViolations`](#sandbox-ignoreviolations)                                               | Заглушите отчёты о нарушениях для путей, которые команда, как ожидается, будет проверять                                                                                                                                                                                | Sandbox settings                   | Any file                |
| [`sandbox.network`](#sandbox-network)                                                                 | Контролируйте, какие хосты, порты и сокеты [изолированные](/docs/ru/sandboxing#network-isolation) команды достигают                                                                                                                                                          | Sandbox settings                   | Any file                |
| [`sandbox.network.allowAllUnixSockets`](#sandbox-network-allowallunixsockets)                         | Позвольте [изолированным](/docs/ru/sandboxing) командам подключаться к каждому Unix-сокету                                                                                                                                                                                   | Sandbox settings                   | Any file                |
| [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains)                                   | Предварительно разрешите домены, чтобы [изолированные](/docs/ru/sandboxing) команды не запрашивали их                                                                                                                                                                        | Sandbox settings                   | Any file                |
| [`sandbox.network.allowLocalBinding`](#sandbox-network-allowlocalbinding)                             | Позвольте [изолированным](/docs/ru/sandboxing) командам привязываться к портам localhost на macOS                                                                                                                                                                            | Sandbox settings                   | Any file                |
| [`sandbox.network.allowMachLookup`](#sandbox-network-allowmachlookup)                                 | Позвольте инструментам macOS [изолированным](/docs/ru/sandboxing), таким как iOS Simulator или Playwright, достичь их XPC-сервисов                                                                                                                                           | Sandbox settings                   | Any file                |
| [`sandbox.network.allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly)                 | Заблокируйте список разрешений сети на [управляемые параметры](/docs/ru/sandboxing#keep-developers-from-widening-the-policy)                                                                                                                                                 | Sandbox settings                   | Managed                 |
| [`sandbox.network.allowUnixSockets`](#sandbox-network-allowunixsockets)                               | Перечислите пути Unix-сокетов, которые [изолированные](/docs/ru/sandboxing) команды могут использовать на macOS                                                                                                                                                              | Sandbox settings                   | Any file                |
| [`sandbox.network.deniedDomains`](#sandbox-network-denieddomains)                                     | Блокируйте домены для [изолированных](/docs/ru/sandboxing) команд, даже внутри разрешённого подстановочного знака                                                                                                                                                            | Sandbox settings                   | Any file                |
| [`sandbox.network.httpProxyPort`](#sandbox-network-httpproxyport)                                     | Маршрутизируйте [песочницу](/docs/ru/sandboxing#custom-proxy-configuration) HTTP-трафик через ваш собственный прокси                                                                                                                                                         | Sandbox settings                   | Any file                |
| [`sandbox.network.socksProxyPort`](#sandbox-network-socksproxyport)                                   | Маршрутизируйте [песочницу](/docs/ru/sandboxing#custom-proxy-configuration) SOCKS-трафик через ваш собственный прокси                                                                                                                                                        | Sandbox settings                   | Any file                |
| [`sandbox.network.strictAllowlist`](#sandbox-network-strictallowlist)                                 | Отклоняйте хосты вне [списка разрешений](/docs/ru/sandboxing#network-isolation) вместо запроса                                                                                                                                                                               | Sandbox settings                   | User or managed         |
| [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate)                                       | Пусть [песочница](/docs/ru/sandboxing#network-isolation) прокси завершает TLS, чтобы она могла читать HTTPS-запросы                                                                                                                                                          | Sandbox settings                   | User or managed         |
| [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                 | Используйте свой собственный двоичный файл ripgrep внутри [песочницы](/docs/ru/sandboxing)                                                                                                                                                                                   | Sandbox settings                   | User or managed         |
| [`sandbox.socatPath`](#sandbox-socatpath)                                                             | Укажите [песочницу](/docs/ru/sandboxing) прокси на двоичный файл `socat` вне `PATH`                                                                                                                                                                                          | Sandbox settings                   | Managed                 |
| [`showClearContextOnPlanAccept`](#showclearcontextonplanaccept)                                       | Показывайте опцию "очистить контекст" на [экране принятия плана](/docs/ru/permission-modes#review-and-approve-a-plan)                                                                                                                                                        | Interface and terminal             | Any file                |
| [`showThinkingSummaries`](#showthinkingsummaries)                                                     | Смотрите резюме [мышления](/docs/ru/model-config#extended-thinking) Claude вместо свёрнутой заглушки                                                                                                                                                                         | Model and responses                | Any file                |
| [`showTurnDuration`](#showturnduration)                                                               | Скройте длительность "Cooked for" после каждого ответа                                                                                                                                                                                                                  | Interface and terminal             | Any file                |
| [`skillListingBudgetFraction`](#skilllistingbudgetfraction)                                           | Зарезервируйте больше или меньше контекста для [списка навыков](/docs/ru/skills#skill-descriptions-are-cut-short)                                                                                                                                                            | Memory and context                 | Any file                |
| [`skillListingMaxDescChars`](#skilllistingmaxdescchars)                                               | Ограничьте длину описания каждого навыка в [списке навыков](/docs/ru/skills#skill-descriptions-are-cut-short)                                                                                                                                                                | Memory and context                 | Any file                |
| [`skillOverrides`](#skilloverrides)                                                                   | [Скройте или свёрните навык](/docs/ru/skills#override-skill-visibility-from-settings) без редактирования его SKILL.md                                                                                                                                                        | Plugins and skills                 | Any file                |
| [`skipAutoPermissionPrompt`](#skipautopermissionprompt)                                               | Пропустите одноразовое уведомление Claude Code, которое показывается при первом входе в [автоматический режим](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode) самостоятельно, а не через встроенное значение по умолчанию                                       | Permission settings                | User or managed         |
| [`skipDangerousModePermissionPrompt`](#skipdangerousmodepermissionprompt)                             | Пропустите диалог подтверждения перед [режимом bypassPermissions](/docs/ru/permission-modes#skip-all-checks-with-bypasspermissions-mode)                                                                                                                                     | Permission settings                | User, local, or managed |
| [`skipWebFetchPreflight`](#skipwebfetchpreflight)                                                     | Пропустите [проверку имени хоста WebFetch](/docs/ru/tools-reference#webfetch-tool-behavior), когда Anthropic недоступен                                                                                                                                                      | Privacy and telemetry              | Any file                |
| [`spellcheck`](#spellcheck)                                                                           | Подчеркивайте неправильно написанные слова в приглашении ввода с помощью [проверки орфографии](/docs/ru/interactive-mode#check-spelling-as-you-type), которую вы устанавливаете                                                                                              | Interface and terminal             | User or managed         |
| [`spinnerTipsEnabled`](#spinnertipsenabled)                                                           | Скройте советы в спиннере, пока Claude работает                                                                                                                                                                                                                         | Interface and terminal             | Any file                |
| [`spinnerTipsOverride`](#spinnertipsoverride)                                                         | Добавьте свои собственные советы к ротации спиннера или замените встроенные советы                                                                                                                                                                                      | Interface and terminal             | Any file                |
| [`spinnerVerbs`](#spinnerverbs)                                                                       | Добавьте или замените глаголы, показываемые во время выполнения хода                                                                                                                                                                                                    | Interface and terminal             | Any file                |
| [`sshConfigs`](#sshconfigs)                                                                           | Добавьте [SSH-соединения](/docs/ru/desktop#pre-configure-ssh-connections-for-your-team) в раскрывающееся меню среды Desktop                                                                                                                                                  | Remote, desktop, and notifications | User or managed         |
| [`sshHostAllowlist`](#sshhostallowlist)                                                               | Ограничьте, какие хосты [сеансы Desktop SSH](/docs/ru/desktop#restrict-which-ssh-hosts-users-can-connect-to) могут достичь                                                                                                                                                   | Remote, desktop, and notifications | Managed                 |
| [`statusLine`](#statusline)                                                                           | Запустите свою собственную команду для отображения [строки состояния](/docs/ru/statusline) ниже приглашения                                                                                                                                                                  | Interface and terminal             | Any file                |
| [`strictKnownMarketplaces`](#strictknownmarketplaces)                                                 | Список разрешений для источников [маркетплейса](/docs/ru/plugins/overview) для пользователей, которые могут добавлять и устанавливать                                                                                                                                        | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)                                     | Блокируйте [навыки](/docs/ru/skills), [агентов](/docs/ru/sub-agents), [hooks](/docs/ru/hooks) и [MCP серверы](/docs/ru/mcp) из источников пользователя и проекта                                                                                                                            | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.agents`](#strictpluginonlycustomization-agents)                       | Заблокируйте [агентов](/docs/ru/sub-agents) для источников плагинов и управляемых источников                                                                                                                                                                                 | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.hooks`](#strictpluginonlycustomization-hooks)                         | Заблокируйте [hooks](/docs/ru/hooks) для источников плагинов и управляемых источников                                                                                                                                                                                        | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.mcp`](#strictpluginonlycustomization-mcp)                             | Заблокируйте [MCP серверы](/docs/ru/mcp) для источников плагинов и управляемых источников                                                                                                                                                                                    | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.skills`](#strictpluginonlycustomization-skills)                       | Заблокируйте [навыки](/docs/ru/skills) для источников плагинов и управляемых источников                                                                                                                                                                                      | Plugins and skills                 | Managed                 |
| [`subagentPromptCacheTtl`](#subagentpromptcachettl)                                                   | Выберите [время жизни кэша подсказки](/docs/ru/prompt-caching#cache-lifetime) для подагентов и других запросов вне основного разговора                                                                                                                                       | Model and responses                | Any file                |
| [`subagentStatusLine`](#subagentstatusline)                                                           | Переписывайте строки в [отображении задач подагента](/docs/ru/sub-agents) с помощью вашей собственной команды                                                                                                                                                                | Interface and terminal             | Any file                |
| [`switchModelsOnFlag`](#switchmodelsonflag)                                                           | Автоматически переключайте модели или приостанавливайте, когда [классификатор безопасности](/docs/ru/model-config#ask-before-switching) помечает запрос                                                                                                                      | Model and responses                | Any file                |
| [`syncClaudeAiPlugins`](#syncclaudeaiplugins)                                                         | Остановите загрузку [плагинов, включённых в вашу учётную запись claude.ai](/docs/ru/plugins/loading#synced-plugins) и скройте уже синхронизированные                                                                                                                         | Plugins and skills                 | User, local, or managed |
| [`syncClaudeAiSkills`](#syncclaudeaiskills)                                                           | Остановите загрузку [навыков, включённых в вашу учётную запись claude.ai](/docs/ru/skills#how-synced-skills-behave) и скройте уже синхронизированные                                                                                                                         | Plugins and skills                 | User, local, or managed |
| [`syntaxHighlightingDisabled`](#syntaxhighlightingdisabled)                                           | Отключите выделение синтаксиса в дифах и блоках кода                                                                                                                                                                                                                    | Interface and terminal             | Any file                |
| [`taskOutputMaxChars`](#taskoutputmaxchars)                                                           | Удалено в v2.1.277 вместе с инструментом `TaskOutput`, который оно определяло                                                                                                                                                                                           | Memory and context                 | Any file                |
| [`teammateDefaultModel`](#teammatedefaultmodel)                                                       | Удалено в v2.1.234; см. [Укажите товарищей по команде и модели](/docs/ru/agent-teams#specify-teammates-and-models) для того, как Claude Code выбирает модель товарища по команде                                                                                             | Global config settings             | Global config           |
| [`teammateMode`](#teammatemode)                                                                       | Выберите, как [товарищи по команде агентов отображаются](/docs/ru/agent-teams#choose-a-display-mode)                                                                                                                                                                         | Agents, sessions, and worktrees    | Any file                |
| [`terminalProgressBarEnabled`](#terminalprogressbarenabled)                                           | Скройте полосу прогресса терминала в терминалах, которые её поддерживают                                                                                                                                                                                                | Interface and terminal             | Any file                |
| [`terminalTitleFromRename`](#terminaltitlefromrename)                                                 | Остановите [`/rename`](/docs/ru/sessions#name-your-sessions) и `--name` от изменения названия вкладки терминала                                                                                                                                                              | Interface and terminal             | Any file                |
| [`theme`](#theme)                                                                                     | Выберите [цветовую тему](/docs/ru/terminal-config#match-the-color-theme) интерфейса, встроенную или пользовательскую                                                                                                                                                         | Interface and terminal             | Any file                |
| [`timeFormat`](#timeformat)                                                                           | Показывайте время в интерфейсе на 12-часовых или 24-часовых часах, в UTC или с шаблоном strftime                                                                                                                                                                        | Interface and terminal             | Any file                |
| [`timeZone`](#timezone)                                                                               | Показывайте время в интерфейсе в часовом поясе, отличном от вашего системного                                                                                                                                                                                           | Interface and terminal             | Any file                |
| [`tui`](#tui)                                                                                         | Выберите [полноэкранный](/docs/ru/fullscreen) или классический рендерер терминала                                                                                                                                                                                            | Interface and terminal             | Any file                |
| [`ultracode`](#ultracode)                                                                             | Попросите Claude спланировать [рабочий процесс](/docs/ru/workflows#let-claude-decide-with-ultracode) для каждой существенной задачи без запроса                                                                                                                              | Model and responses                | Any file                |
| [`useAutoModeDuringPlan`](#useautomodeduringplan)                                                     | Позвольте классификатору [автоматического режима](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode) проверять команды shell в [режиме плана](/docs/ru/permission-modes#analyze-before-you-edit-with-plan-mode); установите `false` для получения подсказок вместо этого | Permission settings                | User, local, or managed |
| [`verbose`](#verbose)                                                                                 | Показывайте [полный вывод инструмента](/docs/ru/cli-reference#cli-flags) вместо усечённых резюме; `viewMode` имеет приоритет, когда оба установлены                                                                                                                          | Interface and terminal             | Any file                |
| [`viewMode`](#viewmode)                                                                               | Начните каждый сеанс в [представлении по умолчанию, подробном или сфокусированном](/docs/ru/cli-reference#cli-flags)                                                                                                                                                         | Interface and terminal             | Any file                |
| [`vimInsertModeRemaps`](#viminsertmoderemaps)                                                         | Сопоставьте двухклавишную [последовательность режима INSERT](/docs/ru/interactive-mode#remap-insert-mode-key-sequences), такую как `jj`, с Escape                                                                                                                            | Interface and terminal             | User or managed         |
| [`voice`](#voice)                                                                                     | Включите [голосовую диктовку](/docs/ru/voice-dictation) и выберите режим удержания или касания                                                                                                                                                                               | Interface and terminal             | Any file                |
| [`voiceEnabled`](#voiceenabled)                                                                       | Включите [голосовую диктовку](/docs/ru/voice-dictation) с более старой одноклавишной формой                                                                                                                                                                                  | Interface and terminal             | Any file                |
| [`wheelScrollAccelerationEnabled`](#wheelscrollaccelerationenabled)                                   | Отключите [ускорение прокрутки колеса мыши](/docs/ru/fullscreen#mouse-wheel-scrolling) при полноэкранном отображении                                                                                                                                                         | Interface and terminal             | Any file                |
| [`workflowKeywordTriggerEnabled`](#workflowkeywordtriggerenabled)                                     | Позвольте слову `ultracode` в подсказке запустить [рабочий процесс](/docs/ru/workflows); установите `false` для ввода без запуска                                                                                                                                            | Hooks and automation               | Any file                |
| [`workflowSizeGuideline`](#workflowsizeguideline)                                                     | Установите количество агентов, на которое Claude нацелен в [динамических рабочих процессах](/docs/ru/workflows)                                                                                                                                                              | Hooks and automation               | Any file                |
| [`worktree`](#worktree)                                                                               | Настройте, как Claude Code создаёт git [worktrees](/docs/ru/worktrees)                                                                                                                                                                                                       | Agents, sessions, and worktrees    | Any file                |
| [`worktree.baseRef`](#worktree-baseref)                                                               | Ветвите новые [worktrees](/docs/ru/worktrees) из удалённой ветви по умолчанию или вашего локального HEAD                                                                                                                                                                     | Agents, sessions, and worktrees    | Any file                |
| [`worktree.bgIsolation`](#worktree-bgisolation)                                                       | Позвольте фоновым сеансам редактировать рабочую копию без [worktree](/docs/ru/worktrees)                                                                                                                                                                                     | Agents, sessions, and worktrees    | Any file                |
| [`worktree.sparsePaths`](#worktree-sparsepaths)                                                       | Проверьте только необходимые вам каталоги в каждом [worktree](/docs/ru/worktrees)                                                                                                                                                                                            | Agents, sessions, and worktrees    | Any file                |
| [`worktree.symlinkDirectories`](#worktree-symlinkdirectories)                                         | Создавайте символические ссылки на большие каталоги в каждый [worktree](/docs/ru/worktrees) вместо их дублирования                                                                                                                                                           | Agents, sessions, and worktrees    | Any file                |
| [`wslInheritsWindowsSettings`](#wslinheritswindowssettings)                                           | Пусть WSL читает [управляемые параметры](/docs/ru/managed-settings) из цепочки политики Windows                                                                                                                                                                              | Enterprise and managed settings    | Managed                 |

<h2 id="model-and-responses">
  Модель и ответы
</h2>

Выберите, какие модели использует Claude Code и как он отвечает. Информацию о том, как эти параметры взаимодействуют с командой `/model` и переменными окружения, см. в разделе [Конфигурация модели](/docs/ru/model-config).

<h3 id="advisormodel">
  `advisorModel`
</h3>

Выберите, какая модель отвечает, когда Claude вызывает серверный [инструмент advisor](/docs/ru/advisor). Удалите этот параметр, чтобы отключить advisor. Advisor должен быть по крайней мере столь же способным, как ваша основная модель. См. [Выберите модель advisor](/docs/ru/advisor#choose-an-advisor-model) для принятых пар и того, что происходит, когда вы выбираете ту, которая не принята.

Обычно вы не редактируете этот ключ вручную. Запустите `/advisor`, чтобы открыть средство выбора, которое показывает текущий выбор, модели, которые могут давать советы, и **Без advisor**. Claude Code сохраняет ваш выбор в этот ключ в `~/.claude/settings.json`. Если вы выбираете из клиента [Remote Control](/docs/ru/remote-control) или в сеансе, подключённом к удалённому рабочему процессу, выбор применяется только к этому сеансу и не изменяет этот ключ.

Если ваша учётная запись требует [согласия на использование кредитов](/docs/ru/advisor#fable-advisor-and-usage-credits), сначала примите его, запустив `/model fable`. До этого выбор Fable в `/advisor` ничего не сохраняет, и Claude Code говорит вам запустить `/model fable` сначала.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: строка, один из псевдонимов `"fable"`, `"opus"` или `"sonnet"`, которые разрешаются в текущую версию Claude Code этого семейства моделей по умолчанию, или полный ID модели, такой как `"claude-opus-5-5"`
* **По умолчанию**: не установлено, поэтому advisor отключён
* **Переопределения для каждого сеанса**: `--advisor` имеет приоритет над этим ключом для одного сеанса. [`CLAUDE_CODE_DISABLE_ADVISOR_TOOL`](/docs/ru/env-vars) отключает advisor, и этот ключ не может его включить обратно

```json settings.json theme={null}
{
  "advisorModel": "opus"
}
```

Ключ не влияет на поставщиков, где advisor [недоступен](/docs/ru/advisor#requirements), таких как Amazon Bedrock и Claude Platform на AWS. `"fable"` требует [доступа к Fable](/docs/ru/advisor#choose-an-advisor-model).

<h3 id="alwaysthinkingenabled">
  `alwaysThinkingEnabled`
</h3>

Отключите [расширенное мышление](/docs/ru/model-config#extended-thinking) для каждого сеанса, установив это значение на `false`. Мышление включено по умолчанию, поэтому `true` ничего не меняет. Большинство людей устанавливают это через `/config` вместо редактирования файла.

На моделях, которые всегда думают, таких как Opus 5.5 и модели Fable, `false` не имеет эффекта. На [сторонних поставщиках](/docs/ru/third-party-integrations) Claude Code опускает параметр `thinking` вместо отключения мышления, поэтому модели адаптивного рассуждения могут всё ещё думать. При отключении мышления на Anthropic API Claude Code отправляет усилие `high` вместо более высокого уровня моделям, которые, как известно, [не принимают эту комбинацию](/docs/ru/errors#effort-isnt-available-with-thinking-turned-off), таким как Opus 5.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: Boolean
  * `true`: нет эффекта; мышление уже включено
  * `false`: Claude Code отключает расширенное мышление для каждого сеанса
* **По умолчанию**: не установлено, поэтому мышление включено для моделей, которые его поддерживают
* **Переопределения для каждого сеанса**: [`MAX_THINKING_TOKENS`](/docs/ru/env-vars) имеет приоритет над этим ключом для одного сеанса: `0` отключает мышление в соответствии с теми же ограничениями модели и поставщика, что и `false`, а положительное значение включает мышление даже когда этот ключ имеет значение `false`. На моделях адаптивного рассуждения само число игнорируется

```json settings.json theme={null}
{
  "alwaysThinkingEnabled": false
}
```

<h3 id="availablemodels">
  `availableModels`
</h3>

Ограничьте, какие модели люди могут выбирать для основного сеанса, [подагентов](/docs/ru/sub-agents), [skills](/docs/ru/skills) и [advisor](/docs/ru/advisor). Управляемый список ограничивает `/model`, `--model` и ключ `model` в собственных файлах разработчика; модель вне списка не может быть выбрана. Сам по себе это не влияет на опцию Default; объедините его с [`enforceAvailableModels`](#enforceavailablemodels) для этого.

* **Область действия**: [`Any file`](#scopes). Разверните его в управляемых параметрах, чтобы применить его для организации.
* **Тип**: массив псевдонимов моделей или ID
* **По умолчанию**: не установлено, поэтому каждая модель доступна

Этот пример позволяет людям выбирать только модели Sonnet и Haiku:

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

См. [Ограничьте выбор модели](/docs/ru/model-config#restrict-model-selection).

<h3 id="effortlevel">
  `effortLevel`
</h3>

Установите уровень [усилия](/docs/ru/model-config#adjust-effort-level) по умолчанию для моделей, для которых вы не сохранили уровень. Более низкие уровни быстрее и дешевле для простых задач, а более высокие уровни рассуждают глубже для сложных проблем.

Когда вы запускаете `/effort low`, `medium`, `high` или `xhigh` в интерактивном сеансе на вашей машине, Claude Code сохраняет уровень для активной модели под [`modelSettings`](#modelsettings) вместо записи этого ключа. До версии 2.1.251 `/effort` записывал этот ключ.

В одном файле параметров Claude Code использует сохранённый уровень модели вместо этого ключа. [`modelSettings`](#modelsettings) указывает приоритет между файлами.

В сеансе, подключённом к удалённому рабочему процессу, в запуске `-p` и в Agent SDK `/effort` применяется только к этому сеансу. [Отрегулируйте уровень усилия](/docs/ru/model-config#adjust-effort-level) перечисляет интерактивные выборы, которые также применяются только к этому сеансу. Сообщение, которое печатает `/effort`, говорит, что произошло.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: строка, один из:
  * `"low"`: минимальное рассуждение для коротких, ограниченных, чувствительных к задержкам задач, которые не требуют интеллекта
  * `"medium"`: снижает использование токенов для работы, чувствительной к затратам, которая может пожертвовать некоторым интеллектом
  * `"high"`: балансирует использование токенов и интеллект
  * `"xhigh"`: более глубокое рассуждение при более высоких затратах токенов
* **По умолчанию**: не установлено
* **Переопределения для каждого сеанса**: `--effort` имеет приоритет над этим ключом для одного сеанса, и [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/ru/env-vars) имеет приоритет над обоими

```json settings.json theme={null}
{
  "effortLevel": "xhigh"
}
```

В вашем файле параметров пользователя, `~/.claude/settings.json`, этот ключ — это более старая форма, которую `/effort` записывал до того, как он сохранял уровни для каждой модели, и он продолжает применяться там, где он применялся раньше, на Opus 5, Fable 5.1 и более ранних моделях. Opus 5.5 и модели, выпущенные после него, игнорируют его и начинают со своего собственного значения по умолчанию, пока вы не сохраните уровень для них, который `/effort` записывает под [`modelSettings`](#modelsettings). В параметрах проекта, локальных и управляемых, а также с `--settings`, этот ключ применяется к каждой модели.

<h3 id="enforceavailablemodels">
  `enforceAvailableModels`
</h3>

Средство выбора `/model` имеет опцию **Default**, которая разрешается в [модель по умолчанию вашей организации](/docs/ru/model-config#organization-default-model), когда она применяется, и в противном случае в модель по умолчанию вашего типа учётной записи. Список [`availableModels`](#availablemodels) ограничивает модели, которые вы можете назвать, но сам по себе оставляет **Default** в покое, поэтому **Default** всё ещё может разрешиться в модель вне списка. Этот ключ закрывает этот пробел. Требует Claude Code версии 2.1.175 или позже.

Когда ваша организация развёртывает какие-либо управляемые параметры, Claude Code читает этот ключ только из управляемого источника и игнорирует его в ваших других файлах.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: Boolean
  * `true`: когда **Default** разрешится в модель вне `availableModels`, Claude Code разрешает её в первую доступную модель в списке
  * `false`: **Default** разрешается как обычно, даже в модель вне `availableModels`
* **По умолчанию**: `false`

Этот пример ограничивает именованные выборы моделями Sonnet и Haiku и делает **Default** разрешённым в первую из них, которая доступна:

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

Этот ключ не имеет эффекта, когда `availableModels` не установлен или пуст. См. [Применить список разрешений для модели Default](/docs/ru/model-config#enforce-the-allowlist-for-the-default-model). Требует Claude Code версии 2.1.175 или позже.

<h3 id="fallbackmodel">
  `fallbackModel`
</h3>

Назовите резервные модели для Claude Code, чтобы попробовать по порядку, когда ваша основная модель перегружена или недоступна. Claude Code переключается на следующую доступную модель в цепи для остальной части хода и показывает уведомление. Без цепи Claude Code повторяет попытку той же модели, а затем выводит ошибку сервера, и вы повторяете попытку или переключаете модели самостоятельно.

Переключение означает один ход с холодным [кэшем подсказок](/docs/ru/prompt-caching#switching-models) на резервной модели; ваше следующее сообщение сначала пытается основную модель снова.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: массив псевдонимов моделей или ID; `"default"` расширяется в модель по умолчанию
* **По умолчанию**: не установлено, поэтому неудачный запрос не повторяется на другой модели
* **Переопределения для каждого сеанса**: `--fallback-model` имеет приоритет над этим ключом для одного сеанса

Этот пример сначала пытается Sonnet 5, затем Haiku 4.5, когда ваша основная модель не работает:

```json settings.json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

В отличие от большинства параметров массива, этот ключ не объединяется между файлами параметров: файл с наивысшим приоритетом, который его определяет, поставляет всю цепь. Если ваш файл проекта устанавливает `["claude-sonnet-5"]` и ваш файл пользователя устанавливает `["claude-haiku-4-5"]`, цепь — это только `["claude-sonnet-5"]`. Claude Code сохраняет не более трёх различных разрешённых моделей из списка и игнорирует остальные. См. [Цепи резервных моделей](/docs/ru/model-config#fallback-model-chains).

<h3 id="fastmode">
  `fastMode`
</h3>

Включите [быстрый режим](/docs/ru/fast-mode) для сеансов, где он доступен, для интерактивной работы, такой как быстрая итерация или живая отладка, где вы хотите скорость при более высокой стоимости за токен. Обычно вы не редактируете этот ключ вручную: запуск `/fast` записывает `fastMode: true` в `~/.claude/settings.json`, и запуск его снова, чтобы отключить быстрый режим, удаляет ключ. Быстрый режим работает только на Opus 5.5, Opus 5 и Opus 4.8: включение его с другой модели переключает вас на Opus, а переключение на неподдерживаемую модель отключает его. См. [Переключайте модели, пока быстрый режим включён](/docs/ru/fast-mode#switch-models-while-fast-mode-is-on).

* **Область действия**: [`Any file`](#scopes)
* **Тип**: Boolean
  * `true`: Claude Code включает быстрый режим для сеансов, где он доступен
  * `false`: быстрый режим остаётся отключённым
* **По умолчанию**: не установлено, поэтому быстрый режим отключён
* **Переопределения для каждого сеанса**: [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/ru/env-vars) отключает быстрый режим для одного сеанса, и этот ключ не может его включить обратно

```json settings.json theme={null}
{
  "fastMode": true
}
```

<h3 id="fastmodepersessionoptin">
  `fastModePerSessionOptIn`
</h3>

Обычно запуск `/fast` сохраняет [`fastMode`](#fastmode) в параметры пользователя человека, поэтому быстрый режим включён в начале каждого последующего сеанса. Установите этот ключ на `true`, чтобы остановить это: сохранённый `fastMode: true` больше не включает быстрый режим в начале сеанса, и каждый человек должен запустить `/fast` в каждом сеансе, в котором они его хотят. Claude Code оставляет ключ `fastMode` в их файле, поэтому отключение этого ключа восстанавливает старое поведение.

Владельцы планов Team или Enterprise могут развернуть его организационно через [параметры, управляемые сервером](/docs/ru/server-managed-settings). Когда управляемые параметры устанавливают ключ, `/fast on` отклоняется вне интерактивных сеансов терминала и сообщает, что ваша организация отключила быстрый режим. Это охватывает [неинтерактивный режим](/docs/ru/headless), [расширение VS Code](/docs/ru/vs-code) и [облачные сеансы](/docs/ru/claude-code-on-the-web).

* **Область действия**: [`Any file`](#scopes)
* **Тип**: Boolean
  * `true`: сохранённый `fastMode: true` больше не включает быстрый режим в начале сеанса, поэтому каждый человек запускает `/fast` в каждом сеансе, в котором они его хотят; `fastMode: true`, переданный с `--settings`, всё ещё считается для этого сеанса, если только управляемые параметры не устанавливают этот ключ
  * `false`: сохранённый `fastMode: true` включает быстрый режим в начале каждого последующего сеанса
* **По умолчанию**: `false`

```json settings.json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

См. [Требовать согласие для каждого сеанса](/docs/ru/fast-mode#require-per-session-opt-in).

<h3 id="language">
  `language`
</h3>

Попросите Claude отвечать на языке, отличном от английского, по умолчанию. Нет фиксированного списка для ответов: Claude Code добавляет значение дословно в системную подсказку как инструкцию всегда отвечать на этом языке, поэтому любое имя языка, которое Claude может прочитать, работает. Claude Code не проверяет значение, поэтому неправильно написанное имя достигает Claude в том виде, в котором оно написано, а не производит ошибку. То же значение устанавливает язык для [голосовой диктовки](/docs/ru/voice-dictation#change-the-dictation-language), которая имеет фиксированный список [поддерживаемых языков диктовки](/docs/ru/voice-dictation#change-the-dictation-language), и для автоматически созданных названий сеансов.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: строка, любое имя языка, такое как `"japanese"`, `"spanish"` или `"french"`; Claude Code не проверяет его
* **По умолчанию**: не установлено; названия сеансов затем соответствуют языку вашего разговора

```json settings.json theme={null}
{
  "language": "japanese"
}
```

<h3 id="maxeffortlevel">
  `maxEffortLevel`
</h3>

Ограничьте [уровень усилия](/docs/ru/model-config#adjust-effort-level), который может использовать сеанс, оставляя доступными более низкие уровни. Любой более высокий уровень вместо этого работает на пределе, включая один из `/effort`, средства выбора `/model`, `--effort`, [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/ru/env-vars), фронтматтера `effort` skill или subagent, или собственного значения по умолчанию модели. Claude Code применяет предел сам перед каждым запросом, поэтому он действует на каждом поставщике, включая Amazon Bedrock, Google Cloud's Agent Platform и Microsoft Foundry. Требует Claude Code версии 2.1.267 или позже.

* **Область действия**: [`Any file`](#scopes). Разверните его в управляемых параметрах, чтобы применить его для организации. Когда несколько областей устанавливают предел, применяется самый низкий, поэтому предел, установленный в одной области, не может быть повышен из другой
* **Тип**: строка, один из `"low"`, `"medium"`, `"high"`, `"xhigh"` или `"max"`. Значение `"max"` не устанавливает предел
* **По умолчанию**: не установлено, поэтому предел не применяется
* **Эффект на ultracode**: предел ниже `xhigh` делает [ultracode](#ultracode) недоступным на моделях, к которым применяется предел
* **Пределы для каждой модели**: добавьте `maxEffortLevel` в запись [`modelSettings`](#modelsettings) модели. Эта запись заменяет этот ключ только для модели в источнике параметров, который устанавливает оба, такой как ваши параметры пользователя или один [управляемый источник](/docs/ru/managed-settings#how-claude-code-combines-managed-sources). Установите `"max"` там, чтобы освободить модель от предела этого источника; Claude Code всё ещё применяет пределы из других источников

Этот пример ограничивает каждую модель на `medium` и освобождает Sonnet 4.6:

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

Когда ваша организация также устанавливает [ограничение усилия](/docs/ru/model-config#organization-effort-limits) для модели, применяется более низкий из двух пределов.

<h3 id="model">
  `model`
</h3>

Установите модель, которую использует каждый новый сеанс, чтобы вам не пришлось выбирать её с помощью `/model` каждый раз. Установка её здесь не помешает вам переключаться в середине сеанса. Если ваш администратор установил [модель по умолчанию организации](/docs/ru/model-config#organization-default-model) для переопределения выбора пользователя, вы получаете эту модель даже когда вы устанавливаете этот ключ в параметры пользователя, проекта или локальные.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: строка, псевдоним модели или полный ID модели
* **По умолчанию**: не установлено, поэтому Claude Code использует модель по умолчанию вашей учётной записи
* **Переопределения для каждого сеанса**: `--model` имеет приоритет над [`ANTHROPIC_MODEL`](/docs/ru/env-vars), и оба имеют приоритет над этим ключом для одного сеанса, включая управляемый `model`; список [`availableModels`](#availablemodels) всё ещё применяется к выбору

```json settings.json theme={null}
{
  "model": "claude-sonnet-5"
}
```

Значение здесь превосходит [`ANTHROPIC_DEFAULT_MODEL`](/docs/ru/model-config#set-a-default-model-for-new-sessions), которое Claude Code использует только когда ничто другое не выбирает модель.

<h3 id="modeloverrides">
  `modelOverrides`
</h3>

Сопоставьте ID моделей Anthropic с ID моделей, специфичными для поставщика, такими как ARN профилей вывода Amazon Bedrock. Каждая запись средства выбора модели затем использует своё сопоставленное значение при вызове API поставщика. Администраторы используют это на [Amazon Bedrock, Google Cloud's Agent Platform и Microsoft Foundry](/docs/ru/model-config#override-model-ids-per-version) для маршрутизации каждой версии модели к определённому профилю вывода, имени версии или развёртыванию для управления, распределения затрат или региональной маршрутизации.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: объект, сопоставляющий ID модели с ID модели поставщика
* **По умолчанию**: не установлено

Этот пример маршрутизирует каждый вызов для Opus 4.6 к названному профилю вывода Bedrock:

```json settings.json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-6": "arn:aws:bedrock:us-east-1:123456789012:inference-profile/example"
  }
}
```

См. [Переопределите ID моделей для каждой версии](/docs/ru/model-config#override-model-ids-per-version).

<h3 id="modelpicker">
  `modelPicker`
</h3>

Перечислите модели, которые предлагает средство выбора `/model`, в порядке, в котором вы их пишете, и под метками, которые вы выбираете, чтобы средство выбора перечисляло модели, которые использует ваша организация, после встроенного набора или вместо него. `model` каждой строки берётся дословно, поэтому он принимает всё, что принимает `--model`: псевдоним, такой как `opus`, ID модели Anthropic или ID в формате поставщика для Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry или шлюза LLM. Требует Claude Code версии 2.1.242 или позже.

* **Область действия**: [`User or managed`](#scopes). Claude Code читает ключ из управляемых параметров, `--settings` и параметров пользователя, и игнорирует его в параметрах проекта и локальных, поэтому репозиторий, который вы клонируете, не может переименовать средство выбора. Самый высокий из этих трёх, который устанавливает ключ, поставляет весь набор, и Claude Code никогда не объединяет наборы из двух источников.
* **Тип**: объект с массивом `options` строк и необязательным Boolean `replaceBuiltInOptions`
* **По умолчанию**: не установлено, поэтому средство выбора показывает встроенный набор

Этот пример добавляет два развёртывания Bedrock после встроенного набора под именами, которые ваша команда узнаёт:

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
  Поля для `modelPicker`
</h4>

Ключ принимает два поля, одно для самих строк и одно для того, заменяют ли они встроенный набор или добавляют к нему.

| Поле                    | Тип                                                                                   | Что оно делает                                                                                                                                                                                                                                                                                                |
| :---------------------- | :------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `options`               | массив строк, каждая с обязательным `model` и необязательными `label` и `description` | Строки, которые показывает средство выбора, в этом порядке, за исключением того, что затемнённая строка перемещается в конец. Без `label` Claude Code озаглавливает строку встроенным именем для модели, которую он знает, или ID модели в противном случае, и без `description` он пишет общую вторую строку |
| `replaceBuiltInOptions` | Boolean, по умолчанию `false`                                                         | Установите на `true`, чтобы показать только эти строки, **Default** и строку для модели, которую сеанс уже использует. Оставьте не установленным, чтобы добавить эти строки после встроенного набора                                                                                                          |

С включённым `replaceBuiltInOptions` Claude Code скрывает каждую другую строку: встроенный набор, строки, которые он добавляет для записей [`availableModels`](#availablemodels), модели, которые нашла [обнаружение шлюза](/docs/ru/llm-gateway-protocol#model-discovery), и [`ANTHROPIC_CUSTOM_MODEL_OPTION`](/docs/ru/model-config#add-a-custom-model-option). С ним выключенным Claude Code пропускает перечисленную модель, которую встроенный набор уже охватывает. Метка изменяет то, что показывает средство выбора, а не то, какую модель запускает Claude Code.

Список [`availableModels`](#availablemodels) всё ещё применяется к этим строкам. Прежде чем добавить перечисленную модель в список разрешений, прочитайте [Поведение объединения](/docs/ru/model-config#merge-behavior): конкретный ID модели сужает запись подстановочного знака своего семейства. Claude Code также проверяет каждую строку против сеанса перед тем, как показать средство выбора:

* **Удалено**: строка, которую Claude Code не может обслуживать, такая как снятая с производства модель или модель, к которой ваша организация не имеет доступа
* **Затемнено**: строка, которую вы не можете выбрать ещё, показанная с причиной
* **Ни одна строка не выживает**: Claude Code сохраняет встроенный набор, отфильтрованный списком разрешений как обычно

Claude Code удаляет строку, которую он не может разобрать, и сохраняет остальные. См. [Исправьте сломанный файл параметров](/docs/ru/settings#fix-a-broken-settings-file).

<h3 id="modelpricing">
  `modelPricing`
</h3>

Сообщайте о расходах по ставкам, которые платит ваша организация, вместо цены списка. Установите это, когда ваша организация имеет согласованные ставки, поэтому цифры в долларах, которые видят разработчики, соответствуют вашему счёту. Claude Code применяет ставки в `/usage`, [строке состояния](/docs/ru/statusline), `total_cost_usd` Agent SDK, пределе [`--max-budget-usd`](/docs/ru/cli-reference) и [OpenTelemetry](/docs/ru/monitoring-usage) метрике затрат и событиях. Вы поставляете ставки: Claude Code не читает их из вашего контракта или Claude Console. Требует Claude Code версии 2.1.242 или позже.

* **Область действия**: [`Managed`](#scopes). Разверните ключ через параметры, управляемые сервером, политику MDM, файл `managed-settings.json` или [помощник политики](/docs/ru/managed-settings#compute-the-policy-with-a-helper-program). Claude Code игнорирует его в параметрах пользователя, проекта и локальных, в `--settings` и на Windows в пользовательском реестре [HKCU](/docs/ru/managed-settings#where-each-mechanism-stores-the-policy). С параметрами, управляемыми сервером, каждый сеанс сообщает затраты по цене списка до тех пор, пока [выборка параметров](/docs/ru/server-managed-settings#fetch-and-caching-behavior) этого сеанса не подтвердит параметр. Приложение-хост, которое встраивает Claude Code и устанавливает [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/ru/env-vars), может поставить таблицу своего собственного через опцию SDK [`managedSettings`](/docs/ru/agent-sdk/typescript#options), которую Claude Code использует только когда ни один управляемый источник не устанавливает ключ и только в Claude Code версии 2.1.246 или позже.
* **Тип**: объект с необязательным `multiplier` и необязательной картой `overrides`
* **По умолчанию**: не установлено, поэтому Claude Code сообщает цену списка, если приложение-хост не поставляет таблицу

Установите `multiplier` один для плоской скидки или надбавки, `overrides` один для ставок для каждой модели или оба.

Этот пример устанавливает согласованные ставки для Sonnet 4.6 и затем снижает каждую цифру, включая строку Sonnet, на 15%:

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

Установите `multiplier` выше 1, до 10, чтобы отметить каждую цифру. Надбавка требует Claude Code версии 2.1.271 или позже. Более ранние версии игнорируют `multiplier` выше 1 с предупреждением и сохраняют остальную часть параметра.

Для шагов, включая как подтвердить, что ставки действуют, см. [Сообщайте о расходах по вашим согласованным ставкам](/docs/ru/costs#report-spend-at-your-contracted-rates).

<span id="modelpricing-multiplier" />

<span id="modelpricing-overrides" />

<h4 id="fields-for-modelpricing">
  Поля для `modelPricing`
</h4>

| Поле         | Тип                                                                                                    | Что оно делает                                                                                                                                                                                                                                |
| :----------- | :----------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `multiplier` | число больше 0 и не более 10                                                                           | Масштабирует каждую стоимость, которую вычисляет Claude Code, независимо от того, охватывает ли её строка `overrides`. Ниже 1 — скидка, выше 1 — надбавка                                                                                     |
| `overrides`  | карта ID модели к объекту ставки с `input`, `output`, `cacheRead` и `cacheWrite`, каждый от 0 до 10000 | Ставки USD за миллион токенов для этой модели, все четыре обязательны. `cacheWrite` охватывает как пятиминутные, так и одночасовые записи кэша. См. [Какие модели применяет строка modelPricing](#which-models-a-modelpricing-row-applies-to) |

Claude Code использует ставки строки ровно так, как вы их написали, без добавления надбавки быстрого режима или [ставки только для США](https://platform.claude.com/docs/en/about-claude/pricing). Если вы также установите `multiplier`, Claude Code применяет его поверх ставок строки. Claude Code удаляет строку со ставкой, которую он не может разобрать, или `multiplier`, который он не может разобрать, и сохраняет остальные; см. [Исправьте сломанный файл параметров](/docs/ru/settings#fix-a-broken-settings-file).

<h4 id="which-models-a-modelpricing-row-applies-to">
  Какие модели применяет строка `modelPricing`
</h4>

Claude Code решает, какие модели применяет строка, из ключа строки:

* **ID встроенной модели**: ключ, который Claude Code сам использует для встроенной модели, независимо от того, является ли этот ключ собственным ID модели, таким как `claude-sonnet-4-6`, или её ID Bedrock, Agent Platform или Foundry. Claude Code применяет строку к каждому ID снимка с датой и ID, специфичному для поставщика.
* **Любой другой ключ**: ключ, который не является ID встроенной модели, такой как псевдоним модели шлюза. Claude Code применяет строку только к этому одному ID. Когда ID модели точно совпадает с одним из ваших ключей и также попадает под строку, ключ которой является ID встроенной модели, Claude Code использует точное совпадение.
* **Профиль вывода приложения Bedrock**: после того как Claude Code разрешил профиль в модель, к которой он маршрутизирует, через вашу карту [`modelOverrides`](#modeloverrides) или поиск [`bedrock:GetInferenceProfile`](/docs/ru/amazon-bedrock#iam-configuration), Claude Code применяет строку этой модели к профилю.

<h3 id="modelsettings">
  `modelSettings`
</h3>

Сохраните [уровень усилия](/docs/ru/model-config#adjust-effort-level) для каждой модели, которую вы используете. Требует Claude Code версии 2.1.251 или позже.

В интерактивном сеансе на вашей машине, когда вы сохраняете `low`, `medium`, `high` или `xhigh` как свой уровень по умолчанию с помощью `/effort` или ползунка усилия средства выбора `/model`, Claude Code записывает этот уровень здесь под моделью, которую вы используете, поэтому вы редко редактируете этот ключ самостоятельно. Когда вы выбираете один из этих уровней в [средстве выбора модели расширения VS Code](/docs/ru/vs-code#use-the-prompt-box), Claude Code сохраняет его здесь таким же образом. Запись [`effortLevel`](#effortlevel) перечисляет сеансы, где `/effort` применяется только к этому сеансу.

Отредактируйте ключ вручную, чтобы изменить или удалить уровень, который вы сохранили.

`effortLevel` модели здесь имеет приоритет над верхним уровнем [`effortLevel`](#effortlevel) в том же файле параметров. Между файлами Claude Code разрешает каждую модель отдельно: файл [параметров](/docs/ru/settings#settings-precedence) с наивысшим приоритетом, который устанавливает либо `effortLevel` для этой модели, либо верхний уровень `effortLevel`, который [применяется к этой модели](#effortlevel), решает, поэтому `effortLevel` в управляемых параметрах превосходит уровень, который вы сохранили в параметрах пользователя. [Отрегулируйте уровень усилия](/docs/ru/model-config#adjust-effort-level) перечисляет, что ещё может переопределить сохранённый уровень, такой как `--effort` при запуске.

Чтобы ограничить усилие одной модели вместо установки её уровня, добавьте поле [`maxEffortLevel`](#maxeffortlevel) в запись этой модели. Поле требует Claude Code версии 2.1.267 или позже.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: объект, сопоставляющий имя модели с объектом с полем `effortLevel`, один из `"low"`, `"medium"`, `"high"` или `"xhigh"`, полем [`maxEffortLevel`](#maxeffortlevel) или обоими
* **По умолчанию**: не установлено

Claude Code записывает каждую запись под каноническим именем модели, таким как `claude-opus-5-5`, и сопоставляет псевдоним этой модели, с датой, `[1m]` и признанные ID, специфичные для поставщика, с той же записью.

Этот пример сохраняет Opus 5.5 на `high`, пока другие модели используют свои собственные сохранённые или уровни по умолчанию:

```json settings.json theme={null}
{
  "modelSettings": {
    "claude-opus-5-5": {
      "effortLevel": "high"
    }
  }
}
```

Запустите `/effort auto`, чтобы очистить ваш сохранённый уровень для модели, которую вы используете. Claude Code оставляет другие записи и любой верхний уровень `effortLevel` на месте.

<h3 id="outputstyle">
  `outputStyle`
</h3>

Выберите [стиль вывода](/docs/ru/output-styles) по имени. Стиль вывода — это сохранённый набор инструкций, который изменяет роль Claude, тон и формат вывода, такой как встроенные стили Explanatory и Learning или один, который вы написали сами.

Если вы измените этот ключ во время сеанса, Claude использует новый стиль, начиная со следующего сообщения. Для того, что это сообщение стоит в кэшировании подсказок, см. [Изменение стиля вывода](/docs/ru/prompt-caching#changing-output-style). До версии 2.1.251 редактирование применялось только после запуска `/clear` или начала нового сеанса.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: строка, имя [встроенного](/docs/ru/output-styles#built-in-output-styles) или [пользовательского](/docs/ru/output-styles#create-a-custom-output-style) стиля вывода
* **По умолчанию**: не установлено, поэтому Claude Code использует стиль по умолчанию

Этот пример выбирает встроенный стиль Explanatory, который добавляет образовательные идеи между задачами:

```json settings.json theme={null}
{
  "outputStyle": "Explanatory"
}
```

<h3 id="promptcachettl">
  `promptCacheTtl`
</h3>

Выберите, как долго [кэш подсказок](/docs/ru/prompt-caching) держит основной разговор. Этот ключ применяется к вашим интерактивным, `-p` и Agent SDK ходам, вместе с помощниками, которые Claude Code запускает встроенными с ними. Одночасовой срок жизни сохраняет кэш в тепле на более длительные перерывы, и API [выставляет счёт за каждую запись кэша по более высокой ставке](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing), чем при пятиминутном сроке жизни. Требует Claude Code версии 2.1.242 или позже.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: строка, один из:
  * `"5m"`: кэш держится пять минут
  * `"1h"`: кэш держится час
* **По умолчанию**: не установлено, поэтому каждый запрос основного разговора получает [его срок жизни по умолчанию](/docs/ru/prompt-caching#which-ttl-each-request-gets)
* **Переопределения для каждого сеанса**: [`FORCE_PROMPT_CACHING_5M`](/docs/ru/env-vars) имеет приоритет над всем остальным, затем [`CLAUDE_CODE_PROMPT_CACHE_TTL`](/docs/ru/env-vars), затем этот ключ, и последний [`ENABLE_PROMPT_CACHING_1H`](/docs/ru/env-vars)

Этот пример сохраняет основной разговор на одночасовом сроке жизни и оставляет подагентов на пять минут:

```json settings.json theme={null}
{
  "promptCacheTtl": "1h",
  "subagentPromptCacheTtl": "5m"
}
```

Для того, что стоит каждый срок жизни, см. [Срок жизни кэша](/docs/ru/prompt-caching#cache-lifetime).

<h3 id="showthinkingsummaries">
  `showThinkingSummaries`
</h3>

Смотрите резюме [расширенного мышления](/docs/ru/model-config#extended-thinking) Claude в интерактивных сеансах. Установите это, если вы хотите полные резюме, когда вы расширяете мышление с помощью `Ctrl+O`. Когда не установлено или `false`, Anthropic API редактирует блоки мышления и Claude Code показывает свёрнутый заглушку; сторонние поставщики не редактируют.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: Boolean
  * `true`: вы видите полные резюме мышления, когда расширяете мышление с помощью `Ctrl+O`
  * `false`: Anthropic API редактирует блоки мышления и Claude Code показывает свёрнутый заглушку
* **По умолчанию**: `false`

```json settings.json theme={null}
{
  "showThinkingSummaries": true
}
```

Редактирование изменяет только то, что вы видите, а не то, что генерирует модель. Чтобы снизить расходы на мышление, [снизьте бюджет или отключите мышление](/docs/ru/model-config#extended-thinking) вместо этого.

<h3 id="subagentpromptcachettl">
  `subagentPromptCacheTtl`
</h3>

Выберите, как долго [кэш подсказок](/docs/ru/prompt-caching) держит запросы, которые Claude Code делает вне основного разговора. Этот ключ применяется к [подагентам](/docs/ru/sub-agents), [workflows](/docs/ru/workflows) и собственным фоновым и вспомогательным запросам Claude Code, таким как компактирование и названия сеансов. Одночасовой срок жизни сохраняет кэш в тепле на более длительные перерывы, и API [выставляет счёт за каждую запись кэша по более высокой ставке](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing), чем при пятиминутном сроке жизни. Требует Claude Code версии 2.1.242 или позже.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: строка, один из:
  * `"5m"`: кэш держится пять минут
  * `"1h"`: кэш держится час
* **По умолчанию**: не установлено, поэтому каждый из этих запросов получает [его срок жизни по умолчанию](/docs/ru/prompt-caching#which-ttl-each-request-gets)
* **Переопределения для каждого сеанса**: [`FORCE_PROMPT_CACHING_5M`](/docs/ru/env-vars) имеет приоритет над всем остальным, затем [`CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`](/docs/ru/env-vars), затем этот ключ, затем [`ENABLE_PROMPT_CACHING_1H`](/docs/ru/env-vars), который просит одночасовой срок жизни на каждом запросе. Для того, где значение фронтматтера подагента занимает место, см. [Выберите TTL самостоятельно](/docs/ru/prompt-caching#choose-the-ttl-yourself)

Этот пример даёт подагентам и другим запросам вне основного разговора одночасовой срок жизни:

```json settings.json theme={null}
{
  "subagentPromptCacheTtl": "1h"
}
```

Этот ключ охватывает запросы, которые [`promptCacheTtl`](#promptcachettl) не охватывает, поэтому установите оба, чтобы выбрать срок жизни для каждого запроса, который делает Claude Code. Для того, как кэш подагента отличается от основного разговора, см. [Подагенты и кэш](/docs/ru/prompt-caching#subagents-and-the-cache).

<h3 id="switchmodelsonflag">
  `switchModelsOnFlag`
</h3>

Выберите, что происходит, когда [классификатор безопасности помечает запрос](/docs/ru/model-config#automatic-model-fallback): переключиться на резервную модель и продолжить, или приостановиться, чтобы вы могли выбрать между переключением и редактированием подсказки.

* **Область действия**: [`Any file`](#scopes). Появляется в `/config` как **Switch models when a message is flagged**.
* **Тип**: Boolean
  * `true`: Claude Code переключается на резервную модель и продолжает
  * `false`: в интерактивном сеансе Claude Code приостанавливается, чтобы вы могли выбрать между переключением и редактированием подсказки; где диалог не может показаться, такой как запуск `-p`, помеченный запрос заканчивается как ошибка
* **По умолчанию**: `true`, переключаться автоматически

```json settings.json theme={null}
{
  "switchModelsOnFlag": false
}
```

См. [Спросите перед переключением](/docs/ru/model-config#ask-before-switching).

<h3 id="ultracode">
  `ultracode`
</h3>

Начните сеансы с [ultracode](/docs/ru/workflows#let-claude-decide-with-ultracode) включённым. С ним включённым Claude планирует workflow для каждой существенной задачи вместо ожидания, пока вы попросите. Claude планирует workflows только когда [динамические workflows](/docs/ru/workflows) включены для вас, ваша модель поддерживает `xhigh` усилие, и нет [предела усилия](/docs/ru/model-config#organization-effort-limits) ниже `xhigh`. В любом случае `ultracode: true` запускает сеанс на `xhigh` усилии, или на пределе, когда предел усилия ниже. Claude Code читает этот ключ, но никогда не пишет его: `/effort ultracode` включает ultracode только для текущего сеанса.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: Boolean
  * `true`: сеансы начинаются на `xhigh` усилии, с ultracode включённым, когда динамические workflows включены для вас, ваша модель поддерживает `xhigh`, и нет предела усилия ниже `xhigh`
  * `false`: сеансы начинаются с ultracode выключенным
* **По умолчанию**: не установлено, поэтому ultracode выключен
* **Переопределения для каждого сеанса**: `/effort ultracode` включает ultracode для одного сеанса без этого ключа. Флаг `--effort ultracode` также включает его для одного сеанса и требует Claude Code версии 2.1.203 или позже

```json settings.json theme={null}
{
  "ultracode": true
}
```

Ultracode запускает сеанс на `xhigh` усилии и имеет приоритет над `effortLevel` и записями [`modelSettings`](#modelsettings). Если [предел усилия](/docs/ru/model-config#organization-effort-limits) ниже `xhigh` применяется к модели, такой как параметр [`maxEffortLevel`](#maxeffortlevel), сеанс вместо этого запускается на пределе и ultracode остаётся выключенным. Claude затем не планирует workflows самостоятельно, и `/effort` не предлагает `ultracode`. Запрос управления `apply_flag_settings` Agent SDK также принимает ключ.

<h2 id="permission-settings">
  Параметры разрешений
</h2>

Решите, что Claude может делать без запроса, в каком режиме разрешений начинается сеанс и что позволяет классификатор автоматического режима. Синтаксис правил и модель разрешений см. в разделе [Настройка разрешений](/docs/ru/permissions).

<h3 id="allowmanagedpermissionrulesonly">
  `allowManagedPermissionRulesOnly`
</h3>

Сделайте управляемые параметры единственным источником правил разрешений. Claude Code затем игнорирует правила `allow`, `ask` и `deny` в файлах пользователя, проекта, локальных и `--settings`, игнорирует `--allowedTools`, скрывает варианты всегда разрешить в подсказках разрешений и прекращает сохранение новых правил.

Когда применяются [родительские параметры от хоста встраивания](/docs/ru/managed-settings#let-an-embedding-host-add-policy), Claude Code рассматривает их как часть управляемого уровня. Он отбрасывает их правила `allow` и `additionalDirectories`, и сохраняет их правила `deny` и `ask`, кроме правил `Read` и `Edit`, чей шаблон начинается с `!`. Хост не может вырезать пути из управляемых правил с помощью правила `!`, независимо от того, установлен ли этот ключ.

Правила `--disallowedTools` и текущие правила `deny` и `ask` сеанса по-прежнему применяются, включая после перезагрузки параметров Claude Code в середине сеанса. Они только ограничивают, поэтому не могут расширить то, что предоставляют управляемые правила. До версии 2.1.257 Claude Code отбрасывал эти правила командной строки и сеанса при первой перезагрузке параметров.

Для того, что может вырезать шаблон `!` в правиле `--disallowedTools` или сеанса, см. [Правила Read и Edit](/docs/ru/permissions#read-and-edit).

* **Область**: [`Managed`](#scopes)
* **Тип**: Boolean
  * `true`: управляемые параметры становятся единственным источником правил разрешений
  * `false`: Claude Code применяет правила разрешений из файлов пользователя, проекта, локальных и `--settings` в дополнение к управляемым
* **По умолчанию**: не установлено, поэтому Claude Code применяет правила разрешений из параметров пользователя, проекта и локальных, а также из `--settings`, в дополнение к управляемым

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true
}
```

Этот ключ не блокирует список разрешений сервера MCP; для этого установите [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly). См. [Параметры только управляемые](/docs/ru/managed-settings#managed-only-settings).

<h3 id="automode">
  `autoMode`
</h3>

Добавьте свои собственные правила к тому, что блокирует и разрешает классификатор [автоматического режима](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode). Используйте его, чтобы сообщить классификатору, какие репозитории, корзины и домены доверяет ваша организация, чтобы он прекратил блокировать обычные внутренние операции. Классификатор поставляется с [встроенными правилами разрешения и отрицания](/docs/ru/auto-mode-config#inspect-the-defaults-and-your-effective-config). Включите буквальную строку `"$defaults"` в массив, чтобы сохранить эти встроенные правила в этой позиции и добавить свои вокруг них; опустите её, чтобы заменить их своими.

* **Область**: [`User or managed`](#scopes)
* **Тип**: объект с массивами `environment`, `allow`, `soft_deny` и `hard_deny` правил в прозе, плюс Boolean [`classifyAllShell`](#automode-classifyallshell)
* **По умолчанию**: не установлено, поэтому классификатор использует только свои [встроенные правила](/docs/ru/auto-mode-config#inspect-the-defaults-and-your-effective-config)

Этот пример сохраняет встроенные правила `soft_deny` через `"$defaults"` и добавляет ещё одно, которое блокирует `terraform apply`:

```json settings.json theme={null}
{
  "autoMode": {
    "soft_deny": ["$defaults", "Never run terraform apply"]
  }
}
```

Когда более одного из этих файлов устанавливает один и тот же массив, Claude Code объединяет записи. Для формата правила и того, как применяется каждый массив, см. [Настройка автоматического режима](/docs/ru/auto-mode-config).

<h3 id="automode-classifyallshell">
  `autoMode.classifyAllShell`
</h3>

Отправляйте каждую команду Bash и PowerShell через классификатор автоматического режима, пока активен автоматический режим. По умолчанию автоматический режим приостанавливает только правила разрешения, которые могут запускать произвольный код: правила на уровне инструмента и подстановочные знаки, такие как `Bash(*)`, и префиксы интерпретатора или оболочки, такие как `Bash(python *)`. Команда, которая соответствует любому другому правилу разрешения, такому как `Bash(npm test)`, пропускает классификатор, если только она не содержит [разрешённые домены для каждой команды](/docs/ru/sandboxing#per-command-allowed-domains-in-auto-mode). Когда она пропускает, деструктивный аргумент, который префикс правила не предусмотрел, может пройти незамеченным. Установка этого ключа приостанавливает каждое правило разрешения оболочки для сеанса, чтобы классификатор видел каждую команду. Требует Claude Code v2.1.193 или позже.

* **Область**: [`User or managed`](#scopes). Читается везде, где читается [`autoMode`](#automode).
* **Тип**: Boolean
  * `true`: пока активен автоматический режим, Claude Code отправляет каждую команду Bash и PowerShell через классификатор и приостанавливает ваши правила разрешения оболочки; вне автоматического режима правила по-прежнему применяются
  * `false`: автоматический режим приостанавливает только правила разрешения, которые могут запускать произвольный код, такие как `Bash(*)` и `Bash(python *)`; команда, которая соответствует любому другому правилу разрешения, пропускает классификатор, если только она не содержит [разрешённые домены для каждой команды](/docs/ru/sandboxing#per-command-allowed-domains-in-auto-mode), и каждая другая команда оболочки проходит через него
* **По умолчанию**: `false`

```json settings.json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

См. [Маршрутизация всех команд оболочки через классификатор](/docs/ru/auto-mode-config#route-all-shell-commands-through-the-classifier). Требует Claude Code v2.1.193 или позже.

<h3 id="disableautomode">
  `disableAutoMode`
</h3>

Удалите [автоматический режим](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode) из цикла `Shift+Tab`. Любой сеанс, который в противном случае [начинается в автоматическом режиме](/docs/ru/permission-modes#which-mode-a-session-starts-in), будь то из `--permission-mode auto`, файла параметров или встроенного по умолчанию, вместо этого начинается в `default`. Администраторы устанавливают его в управляемых параметрах, чтобы предотвратить использование автоматического режима разработчиками в их организации.

* **Область**: [`Any file`](#scopes). Наиболее полезно в [управляемых параметрах](/docs/ru/managed-settings), где пользователи не могут его переопределить. Также принимается под `permissions` как `permissions.disableAutoMode`.
* **Тип**: строка `"disable"`
* **По умолчанию**: не установлено

```json settings.json theme={null}
{
  "disableAutoMode": "disable"
}
```

<h3 id="permissions">
  `permissions`
</h3>

Контролируйте, какие инструменты Claude может использовать без запроса, какие всегда запрашивают подтверждение, какие заблокированы, и установите [режим разрешений](/docs/ru/permission-modes), в котором начинается сеанс. Каждый ключ `permissions.*` ниже вложен в этот объект.

* **Область**: [`Any file`](#scopes)
* **Тип**: объект с `allow`, `ask`, `deny`, `additionalDirectories`, `blockReadsOutsideWorkingDirectories`, `defaultMode`, `disableBypassPermissionsMode` и `disableAutoMode`
* **По умолчанию**: не установлено

Этот пример одобряет команды `npm run` без запроса, запрашивает подтверждение перед `git push`, блокирует чтение `.env` и запускает сеансы в `acceptEdits`:

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

Три массива правил используют один синтаксис; см. [Синтаксис правила разрешения](#permission-rule-syntax) под `permissions.allow`. Для того, как правила разрешений из разных файлов объединяются, см. [как правила разрешений объединяются в разных областях](/docs/ru/permissions#settings-precedence); для того, как ключи параметров в целом объединяются, см. [Приоритет параметров](/docs/ru/settings#settings-precedence) в руководстве параметров.

<h3 id="useautomodeduringplan">
  `useAutoModeDuringPlan`
</h3>

Выберите, использует ли Claude Code классификатор автоматического режима для проверки команд оболочки в режиме плана. По умолчанию `true` классификатор проверяет каждую команду во время планирования, когда доступен автоматический режим и вы не видите подсказку. Установите `false`, чтобы получить подсказку разрешения для каждой команды вне встроенного набора только для чтения. Появляется в `/config` как **Use auto mode during plan**.

* **Область**: [`User, local, or managed`](#scopes). Репозиторий не может отключить его для вас.
* **Тип**: Boolean
  * `true`: то же самое, что не установлено; когда доступен автоматический режим, классификатор проверяет каждую команду оболочки во время планирования вместо того, чтобы запрашивать вас. `false` в любом из этих файлов по-прежнему отключает его
  * `false`: вы получаете подсказку разрешения для каждой команды вне встроенного набора только для чтения
* **По умолчанию**: `true`

```json settings.json theme={null}
{
  "useAutoModeDuringPlan": false
}
```

<h3 id="permissions-allow">
  `permissions.allow`
</h3>

Перечислите использование инструментов, которые Claude Code одобряет без запроса. В правиле MCP `*` может появляться только в имени инструмента после префикса `mcp__<server>__`, такого как `mcp__github__get_*`; оно не может появляться в имени сервера.

* **Область**: [`Any file`](#scopes)
* **Тип**: массив строк правил разрешения
* **По умолчанию**: не установлено
* **Переопределения для каждого сеанса**: `--allowedTools` добавляет правила разрешения для одного сеанса, и правило отрицания из любого файла параметров по-прежнему блокирует инструмент, который оно называет

Этот пример одобряет `git diff` и позволяет Claude Code читать ваш `.zshrc` без запроса:

```json settings.json theme={null}
{
  "permissions": {
    "allow": ["Bash(git diff *)", "Read(~/.zshrc)"]
  }
}
```

Claude Code применяет правила `allow` из `.claude/settings.json` проекта только после того, как вы примете [диалог доверия рабочей области](/docs/ru/permissions#project-allow-rules-and-workspace-trust) для этой папки.

<h4 id="permission-rule-syntax">
  Синтаксис правила разрешения
</h4>

Правила разрешения следуют формату `Tool` или `Tool(specifier)`. Claude Code оценивает правила `deny` первыми, затем `ask`, затем `allow`, и первое совпадение решает независимо от того, насколько специфично каждое правило; см. [порядок оценки правила разрешения](/docs/ru/permissions#manage-permissions).

Каждая строка показывает одну форму правила и то, что оно соответствует.

| Правило                        | Что оно соответствует             |
| :----------------------------- | :-------------------------------- |
| `Bash`                         | Каждая команда Bash               |
| `Bash(npm run *)`              | Команды, начинающиеся с `npm run` |
| `Read(./.env)`                 | Чтение файла `.env`               |
| `WebFetch(domain:example.com)` | Запросы выборки на example.com    |

Для полного синтаксиса правила, включая поведение подстановочных знаков, шаблоны для конкретных инструментов для Read, Edit, WebFetch, MCP и Agent правил, и ограничения безопасности шаблонов Bash, см. [Синтаксис правила разрешения](/docs/ru/permissions#permission-rule-syntax).

<h3 id="permissions-ask">
  `permissions.ask`
</h3>

Перечислите использование инструментов, которые запрашивают вас на подтверждение даже в режиме разрешений, который в противном случае их одобрил бы, такой как `acceptEdits` или `bypassPermissions`. В режиме `dontAsk` Claude Code отрицает соответствующее использование инструмента вместо того, чтобы запрашивать.

* **Область**: [`Any file`](#scopes)
* **Тип**: массив строк правил разрешения
* **По умолчанию**: не установлено

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

Перечислите использование инструментов, которые блокирует Claude Code. Используйте его для файлов, которые содержат ключи API, секреты или значения окружения: Claude Code исключает соответствующие файлы из обнаружения файлов и результатов поиска, отрицает их чтение и блокирует [инструменты Edit и Write](/docs/ru/permissions#read-and-edit) на соответствующих путях.

Правила отрицания Read и Edit применяются к встроенным инструментам файлов Claude, к командам файлов, которые Claude Code распознаёт в Bash, таким как `cat`, `head`, `tail`, `sed` и `tee`, и к целям Bash [перенаправлений](/docs/ru/permissions#redirections), таких как `> file` и `< file`; они не применяются к команде, которая читает файлы без их названия, такой как `grep -r pattern .`, или к произвольным подпроцессам, поэтому для принудительного применения на уровне ОС [включите песочницу](/docs/ru/sandboxing).

* **Область**: [`Any file`](#scopes)
* **Тип**: массив строк правил разрешения
* **По умолчанию**: не установлено
* **Переопределения для каждого сеанса**: `--disallowedTools` добавляет правила отрицания для одного сеанса рядом с этим ключом

Этот пример отрицает чтение файлов `.env`, каталога `secrets` и файла учётных данных, и блокирует команды `curl`:

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

Имена инструментов принимают шаблоны glob, поэтому `"*"` отрицает каждый инструмент и `"mcp__*"` отрицает каждый инструмент MCP. Claude Code игнорирует правило отрицания для инструмента [`EndConversation`](/docs/ru/tools-reference#endconversation-tool-behavior), пока доступен любой другой инструмент. Правило отрицания `Bash` соответствует команде, как её пишет Claude, поэтому `Bash(curl *)` не останавливает `/usr/bin/curl` или `sh -c 'curl …'`; см. [что правило Bash не соответствует](/docs/ru/permissions#bash-rule-limits). Этот ключ заменяет устаревшую конфигурацию `ignorePatterns`.

<h3 id="permissions-additionaldirectories">
  `permissions.additionalDirectories`
</h3>

Дайте Claude доступ к файлам в каталогах вне того, в котором вы начали, как дополнительные [рабочие каталоги](/docs/ru/permissions#working-directories). Большинство конфигурации `.claude/` [не обнаруживается](/docs/ru/permissions#additional-directories-grant-file-access-not-configuration) из этих каталогов.

* **Область**: [`Any file`](#scopes)
* **Тип**: массив путей каталогов
* **По умолчанию**: не установлено
* **Переопределения для каждого сеанса**: `--add-dir` и `/add-dir` добавляют каталоги для одного сеанса рядом с этим ключом

```json settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": ["../docs/"]
  }
}
```

Как и правила `allow`, записи в `.claude/settings.json` проекта вступают в силу только после того, как вы примете [диалог доверия рабочей области](/docs/ru/permissions#project-allow-rules-and-workspace-trust) для этой папки.

<h3 id="permissions-blockreadsoutsideworkingdirectories">
  `permissions.blockReadsOutsideWorkingDirectories`
</h3>

Остановите Claude от чтения путей вне [рабочих каталогов](/docs/ru/permissions#working-directories) сеанса с помощью инструментов Read, Grep, Glob и LSP, в каждом режиме разрешений, включая `bypassPermissions`. Команда Bash, которая читает соответствующий путь через команду файла, которую Claude Code распознаёт, такую как `cat`, запрашивает вас даже в автоматическом режиме и режиме `bypassPermissions`. Требует Claude Code v2.1.257 или позже.

Команда Bash, которую парсер оболочки не может отследить, такая как та, которая меняет каталог более одного раза или запускает подоболочку, запрашивает вас даже в автоматическом режиме и режиме `bypassPermissions`. Подсказка появляется даже когда команда не называет путь вне рабочих каталогов. Эта подсказка не применяется, когда команда запускается в [песочнице](/docs/ru/sandboxing) и песочница применяет блокировку.

Claude Code также пишет `true` здесь, когда вы выбираете блокировку таких чтений на [подсказке автоматического режима перед первым чтением вне рабочих каталогов](/docs/ru/permission-modes#first-read-outside-the-working-directories).

* **Область**: [`Any file`](#scopes). Если любой источник параметров устанавливает `true`, блокировка применяется, поэтому проверенный файл репозитория может включить блокировку для проекта, но не может снять блокировку, которую вы установили.
* **Тип**: Boolean
  * `true`: чтение файлов вне рабочих каталогов заблокировано
  * `false`: то же самое, что не установлено; `true` в любом другом файле параметров по-прежнему блокирует
* **По умолчанию**: не установлено, поэтому чтение файлов вне рабочих каталогов следует вашему режиму разрешений и правилам

```json settings.json theme={null}
{
  "permissions": {
    "blockReadsOutsideWorkingDirectories": true
  }
}
```

Если только файл параметров репозитория добавляет каталог, блокировка по-прежнему применяется к чтению там. Когда [`autoMemoryDirectory`](#automemorydirectory) поступает из `.claude/settings.json` проекта или из `.claude/settings.local.json` [рассматриваемого как предоставленный репозиторием](/docs/ru/permissions#when-your-local-settings-file-needs-trust), Claude Code не загружает [автоматическую память](/docs/ru/memory#storage-location) из этого каталога и не сохраняет её в него. Файлы, которые нужны самому Claude Code, остаются читаемыми, такие как ваши skills, plugins, правила, agents, команды и файл памяти `CLAUDE.md` под `~/.claude/`.

Когда [песочница](/docs/ru/sandboxing) включена, блокировка также отрицает доступ к чтению для команд в песочнице к домашним каталогам и корням смонтированных томов вне рабочих каталогов. Повторная попытка, которая требует одобрения для [запуска вне песочницы](/docs/ru/sandboxing#the-unsandboxed-retry-escape-hatch), запрашивает вас даже в режиме `bypassPermissions`. Файлы, которые инструмент читает из вашего домашнего каталога, такие как `~/.gitconfig`, отрицаются вместе с остальным; повторно откройте конкретный путь с помощью [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread), когда инструменту это нужно.

Когда рабочий каталог сеанса является связанным [git worktree](/docs/ru/worktrees), включая тот, в который Claude Code вошёл в середине сеанса, общий каталог `.git` репозитория остаётся читаемым и записываемым для команд в песочнице, чтобы git продолжал работать там.

<h3 id="permissions-defaultmode">
  `permissions.defaultMode`
</h3>

Установите [режим разрешений](/docs/ru/permission-modes), в котором начинаются новые сеансы. Когда вы оставляете его не установленным, сеансы начинаются в [встроенном по умолчанию](/docs/ru/permission-modes#which-mode-a-session-starts-in) для вашего плана и поверхности.

* **Область**: [`Any file`](#scopes). `auto` и `bypassPermissions` не вступают в силу из параметров проекта или локальных, поэтому установите их в `~/.claude/settings.json` вместо этого. До версии 2.1.257 `bypassPermissions` вступал в силу из любого файла. Для разговоров, которые запускает расширение VS Code, Claude Code читает только значения пользователя, управляемые и `--settings`.
* **Тип**: строка, одна из:
  * `"default"`: Claude Code запускает только чтение без запроса
  * `"acceptEdits"`: Claude Code также запускает редактирование файлов и общие команды файловой системы, такие как `mkdir` и `mv`, без запроса
  * `"plan"`: Claude Code читает и планирует, но блокирует редактирование до того, как вы одобрите план
  * `"auto"`: Claude Code запускает всё с проверками безопасности в фоне
  * `"dontAsk"`: Claude Code автоматически отрицает каждый вызов, который в противном случае запрашивал бы; чтение, другие действия, которые не требуют одобрения, и предварительно одобренные инструменты по-прежнему запускаются
  * `"bypassPermissions"`: Claude Code запускает всё без запроса
  * `"manual"`: псевдоним для `"default"`, в Claude Code v2.1.200 или позже
* **По умолчанию**: не установлено
* **Переопределения для каждого сеанса**: `--permission-mode` и его эквивалент `--dangerously-skip-permissions` для `bypassPermissions` имеют приоритет над этим ключом для одного сеанса

```json settings.json theme={null}
{
  "permissions": {
    "defaultMode": "acceptEdits"
  }
}
```

Правила разрешений накладываются на каждый режим: правила `deny` блокируют в каждом режиме, включая `bypassPermissions`. См. [Режимы разрешений](/docs/ru/permission-modes). `manual` называет режим разрешений, помеченный Manual в CLI и расширении VS Code; псевдоним требует Claude Code v2.1.200 или позже. В облачных сеансах Claude Code соблюдает только `acceptEdits`, `plan`, `default` и `auto` из этого ключа. Для разговоров, которые запускает расширение VS Code, см. [какой параметр расширение читает для режима разрешений при запуске](/docs/ru/permission-modes#switch-permission-modes).

<h3 id="permissions-disablebypasspermissionsmode">
  `permissions.disableBypassPermissionsMode`
</h3>

Предотвратите вход кого-либо в режим `bypassPermissions`. Claude Code затем отклоняет флаг `--dangerously-skip-permissions` и игнорирует `permissionMode: bypassPermissions` [определения агента](/docs/ru/sub-agents#permission-modes), поэтому подагент запускается с режимом разрешений родительского сеанса.

* **Область**: [`Any file`](#scopes). Обычно устанавливается в [управляемых параметрах](/docs/ru/managed-settings) для применения организационной политики.
* **Тип**: строка `"disable"`
* **По умолчанию**: не установлено
* **Переопределения для каждого сеанса**: этот ключ имеет приоритет над `--dangerously-skip-permissions`, который Claude Code отклоняет, пока ключ установлен

```json settings.json theme={null}
{
  "permissions": {
    "disableBypassPermissionsMode": "disable"
  }
}
```

До версии 2.1.223 Claude Code применял режим разрешений frontmatter даже с отключённым обходом.

<h3 id="skipautopermissionprompt">
  `skipAutoPermissionPrompt`
</h3>

Пропустите одноразовое уведомление, описывающее [автоматический режим](/docs/ru/permission-modes#eliminate-prompts-with-auto-mode), которое Claude Code показывает, когда вы впервые входите в автоматический режим самостоятельно, например через свои собственные параметры или селектор режима, а не когда встроенное по умолчанию запускает сеанс в нём. Claude Code показывает это уведомление один раз, а затем записывает, что оно было показано, поэтому этот ключ имеет значение только там, где уведомление ещё не появилось.

* **Область**: [`User or managed`](#scopes). Репозиторий не может установить его для вас.
* **Тип**: Boolean
  * `true`: Claude Code пропускает уведомление
  * `false`: то же самое, что не установлено; уведомление появляется один раз, если другой из этих файлов не устанавливает `true`
* **По умолчанию**: не установлено, поэтому уведомление появляется один раз

```json settings.json theme={null}
{
  "skipAutoPermissionPrompt": true
}
```

<h3 id="skipdangerousmodepermissionprompt">
  `skipDangerousModePermissionPrompt`
</h3>

Пропустите диалог подтверждения, который Claude Code показывает перед входом сеанса в режим `bypassPermissions`, будь то из `--dangerously-skip-permissions` или из `defaultMode: "bypassPermissions"`. Claude Code пишет `true` здесь в ваши пользовательские параметры, когда вы один раз примете этот диалог.

* **Область**: [`User, local, or managed`](#scopes). Ненадёжный репозиторий не может пропустить диалог для вас.
* **Тип**: Boolean
  * `true`: Claude Code пропускает диалог подтверждения перед входом сеанса в режим `bypassPermissions`
  * `false`: то же самое, что не установлено; диалог появляется, если другой из этих файлов не устанавливает `true`
* **По умолчанию**: не установлено, поэтому диалог появляется

```json settings.json theme={null}
{
  "skipDangerousModePermissionPrompt": true
}
```

<h2 id="sandbox-settings">
  Параметры песочницы
</h2>

Изолируйте команды, которые выполняет Claude, от вашей файловой системы, сети и учетных данных. Информацию о том, как работает песочница и требования к платформе, см. в разделе [Sandboxing](/docs/ru/sandboxing).

<h3 id="sandbox">
  `sandbox`
</h3>

Изолируйте команды Bash, которые выполняет Claude, от вашей файловой системы и сети с помощью [sandboxing](/docs/ru/sandboxing). Включите песочницу с помощью `enabled`, затем сузьте или расширьте то, что могут трогать изолированные команды, с помощью подобъектов `filesystem`, `network` и `credentials`. Песочница работает на macOS, Linux и WSL2.

* **Scope**: [`Any file`](#scopes)
* **Type**: object с `enabled`, `failIfUnavailable`, `autoAllowBashIfSandboxed`, `excludedCommands`, `allowUnsandboxedCommands`, `enableWeakerNestedSandbox`, `enableWeakerNetworkIsolation`, `allowAppleEvents`, `bwrapPath`, `socatPath`, `ignoreViolations` и `ripgrep`, плюс объекты `filesystem`, `network` и `credentials`
* **Default**: не установлено, поэтому Claude Code выполняет команды без песочницы

Это включает песочницу, пропускает подсказки разрешений для изолированных команд, запускает `docker` вне песочницы, открывает два дополнительных пути записи, скрывает файл учетных данных AWS и предварительно разрешает GitHub и npm:

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

Claude Code берет значение логического ключа из области параметров с наивысшим приоритетом, которая его устанавливает, поэтому управляемый `enabled` или `failIfUnavailable` переопределяет все, что устанавливает разработчик. Он объединяет ключи массива во всех областях параметров, которые загружает сеанс, поэтому разработчик может добавлять записи; см. [Keep developers from widening the policy](/docs/ru/sandboxing#keep-developers-from-widening-the-policy) для управляемых блокировок. Чтобы требовать песочницу для организации, см. [Enforce sandboxing with managed settings](/docs/ru/sandboxing#enforce-sandboxing-with-managed-settings).

<h3 id="sandbox-enabled">
  `sandbox.enabled`
</h3>

Включите [sandboxing](/docs/ru/sandboxing) для команд Bash. Когда вы выбираете режим на панели `/sandbox`, Claude Code записывает этот ключ в `.claude/settings.local.json` для текущего проекта; установите его в `~/.claude/settings.json`, чтобы изолировать каждый проект.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code изолирует команды Bash
  * `false`: команды Bash выполняются без изоляции
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true
  }
}
```

На Linux и WSL2 песочница требует `bubblewrap` и `socat`; см. [Set up Linux and WSL2](/docs/ru/sandboxing#set-up-linux-and-wsl2). Когда песочница не может запуститься, Claude Code показывает предупреждение и выполняет команды без изоляции, если вы также не установите [`failIfUnavailable`](#sandbox-failifunavailable).

<h3 id="sandbox-failifunavailable">
  `sandbox.failIfUnavailable`
</h3>

Заставьте Claude Code выйти с ошибкой при запуске, когда `sandbox.enabled` имеет значение `true`, но песочница не может запуститься, потому что отсутствует зависимость или платформа не поддерживается. Без этого Claude Code показывает предупреждение и выполняет команды без изоляции. Используйте это в управляемых параметрах, когда ваша организация требует песочницу как жесткое ограничение.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code выходит с ошибкой при запуске, когда `sandbox.enabled` имеет значение `true`, но песочница не может запуститься
  * `false`: Claude Code показывает предупреждение и выполняет команды без изоляции
* **Default**: `false`

Это заставляет каждую управляемую машину либо изолировать команды, либо отказаться от запуска:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true
  }
}
```

См. [Enforce sandboxing with managed settings](/docs/ru/sandboxing#enforce-sandboxing-with-managed-settings).

<h3 id="sandbox-autoallowbashifsandboxed">
  `sandbox.autoAllowBashIfSandboxed`
</h3>

Позвольте Claude Code выполнять изолированные команды Bash без подсказки разрешения. Команды, которые не могут выполняться в песочнице, по-прежнему проходят обычный поток разрешений, и правила `deny` и правила `ask` с ограничением содержимого, такие как `Bash(git push *)`, по-прежнему применяются; простое правило `Bash` ask пропускается для изолированных команд. Установите значение `false`, чтобы отправить изолированные команды через обычный поток разрешений, который вкладка **Mode** на `/sandbox` называет режимом обычных разрешений.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code выполняет изолированные команды Bash без подсказки разрешения, с учетом правил `deny` и правил `ask` с ограничением содержимого; `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB` отключает автоматическое разрешение
  * `false`: изолированные команды проходят обычный поток разрешений, поэтому ваши правила разрешения и режим разрешения решают. Вкладка **Mode** на `/sandbox` называет это режимом обычных разрешений
* **Default**: `true`

Это сохраняет песочницу включенной и отправляет изолированные команды через обычный поток разрешений:

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": false
  }
}
```

См. [Sandbox modes](/docs/ru/sandboxing#sandbox-modes) для получения информации о том, на что еще запрашивает автоматическое разрешение и как оно ведет себя в режиме плана.

<h3 id="sandbox-excludedcommands">
  `sandbox.excludedCommands`
</h3>

Назовите команды, которые Claude Code выполняет вне песочницы, такие как инструменты, которые не работают под ней. Каждая запись использует тот же синтаксис, что и содержимое правила разрешения `Bash(...)` [permission rule](/docs/ru/permissions#permission-rule-syntax): точная команда, префикс, такой как `docker *`, или шаблон подстановки.

Ваши записи выводят вызов Bash из песочницы только когда они охватывают каждую команду в нем, и некоторые формы вызова остаются изолированными даже тогда. Запись `docker *` одна не выводит `npm ci && docker build .` из песочницы.

* **Scope**: [`Any file`](#scopes)
* **Type**: array of command patterns
* **Default**: не установлено, поэтому ни одна команда не исключена

```json settings.json theme={null}
{
  "sandbox": {
    "excludedCommands": ["docker *"]
  }
}
```

Claude Code сохраняет вызов Bash в изоляции, когда он имеет одну из этих форм, среди прочих:

* Команда, начинающаяся с `sudo`, `eval` или `xargs`
* `cd`, `pushd` или `popd`, где бы они ни появлялись в вызове
* Подстановка команды, подоболочка или блок управления потоком, такой как `if` или `for`
* Перенаправление, такое как `docker build . > build.log`, кроме того, которое только дублирует дескриптор файла, как `2>&1`
* Имя команды, которое поступает из переменной

Например, `cd build && docker compose up` остается изолированным под записью `docker *`, и добавление записи `cd` это не меняет.

Исключенные команды по-прежнему проходят обычный поток разрешений. Исключение — это удобство, а не граница безопасности: предпочитайте [`filesystem.allowWrite`](#sandbox-filesystem-allowwrite), когда инструменту нужно писать только в определенное место. Claude Code объединяет записи во всех областях параметров, которые загружает сеанс, и нет управляемой блокировки для этого списка, поэтому держите управляемый список узким.

<h3 id="sandbox-allowunsandboxedcommands">
  `sandbox.allowUnsandboxedCommands`
</h3>

Позвольте Claude повторить команду вне песочницы с параметром `dangerouslyDisableSandbox` после того, как песочница ее заблокирует. Установите значение `false`, чтобы Claude Code полностью игнорировал этот параметр и каждая команда, которую выполняет Claude, должна быть изолирована или появиться в [`excludedCommands`](#sandbox-excludedcommands). Вкладка **Overrides** на `/sandbox` показывает это состояние как **Strict sandbox mode**. Используйте `false` в управляемых параметрах для политик, требующих строгой изоляции.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude может повторить команду вне песочницы с параметром `dangerouslyDisableSandbox` после того, как песочница ее заблокирует
  * `false`: Claude Code игнорирует этот параметр, поэтому каждая команда, которую выполняет Claude, изолирована или появляется в `excludedCommands`
* **Default**: `true`

Это обеспечивает строгий режим песочницы для всех, кого охватывают управляемые параметры:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowUnsandboxedCommands": false
  }
}
```

Неизолированная повторная попытка проходит обычный поток разрешений с подсказкой в режиме Manual. См. [The unsandboxed retry escape hatch](/docs/ru/sandboxing#the-unsandboxed-retry-escape-hatch).

Чтобы увидеть, когда команды, которые вы вводите сами в [`!` shell-mode prompt](/docs/ru/interactive-mode#shell-mode-with-prefix), выполняются в изоляции, см. [strict sandbox mode](/docs/ru/sandboxing#the-unsandboxed-retry-escape-hatch).

<h3 id="sandbox-filesystem">
  `sandbox.filesystem`
</h3>

Контролируйте, какие пути могут читать и писать изолированные команды. По умолчанию они могут писать в рабочий каталог, временный каталог сеанса и каталоги, которые вы добавляете с помощью `--add-dir`, `/add-dir` или `permissions.additionalDirectories`, и могут читать остальную часть файловой системы, включая файлы учетных данных. Расширьте или сузьте это с помощью четырех списков путей или отключите слой файловой системы с помощью `disabled`. См. [Filesystem isolation](/docs/ru/sandboxing#filesystem-isolation) для получения информации о границах по умолчанию.

* **Scope**: [`Any file`](#scopes)
* **Type**: object с массивами `allowWrite`, `denyWrite`, `denyRead` и `allowRead`, плюс логические значения `allowManagedReadPathsOnly` и `disabled`
* **Default**: не установлено, поэтому применяются границы чтения и записи по умолчанию

Это позволяет изолированным командам писать в каталог сборки и ваш kubeconfig, а также скрывает файл учетных данных AWS:

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

Claude Code применяет эти списки на границе песочницы ОС, поэтому они применяются к каждому подпроцессу, который запускает изолированная команда, такому как `kubectl`, `terraform` или `npm`. Claude Code добавляет ваши [permission rules](/docs/ru/sandboxing#permission-rules) в те же списки: правила `Edit` allow и deny в `allowWrite` и `denyWrite`, правила `Read` deny в `denyRead` и правила `WebFetch(domain:...)` allow и deny в списки доменов [`network`](#sandbox-network).

Если не установлена управляемая блокировка, Claude Code объединяет каждый список во всех файлах параметров, которые загружает сеанс. [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly) ограничивает `allowRead` записями из управляемых параметров, а [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) делает то же самое для разрешенных доменов.

[Configure sandboxing](/docs/ru/sandboxing#configure-sandboxing) охватывает источники, которые вы исключаете с помощью `--setting-sources`. Когда вы редактируете список во время сеанса, Claude Code [применяет изменение к работающему сеансу](/docs/ru/settings#when-edits-take-effect).

<h4 id="sandbox-path-prefixes">
  Префиксы путей песочницы
</h4>

Пути в `allowWrite`, `denyWrite`, `denyRead`, `allowRead` и [`credentials.files`](#sandbox-credentials-files) разрешаются по их префиксу:

| Prefix                | Meaning                                                                                         | Example                                                                    |
| :-------------------- | :---------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------- |
| `/`                   | Абсолютный путь от корня файловой системы                                                       | `/tmp/build` остается `/tmp/build`                                         |
| `~/`                  | Относительно домашнего каталога                                                                 | `~/.kube` становится `$HOME/.kube`                                         |
| `./` или без префикса | Относительно корня проекта для параметров проекта или к `~/.claude` для параметров пользователя | `./output` в `.claude/settings.json` разрешается в `<project-root>/output` |

Префикс `//path` для абсолютных путей также работает. Если вы используете одиночный слэш `/path`, ожидая разрешения относительно проекта, переключитесь на `./path`. Этот синтаксис отличается от [Read and Edit permission rules](/docs/ru/permissions#read-and-edit), которые используют `//path` для абсолютного и `/path` для относительного к проекту: пути файловой системы песочницы используют стандартные соглашения, поэтому `/tmp/build` — это абсолютный путь.

Claude Code удаляет конечный слэш из пути каталога, поэтому `~/.aws` и `~/.aws/` совпадают с одним и тем же каталогом. До v2.1.224 Claude Code передавал конечный слэш в песочницу, и Claude все еще мог читать или писать пути под записью `denyRead` или `denyWrite`, написанной с ним.

Claude Code также удаляет конечный `/**`, поэтому `~/build/**` и `~/build` охватывают один и тот же каталог. Работает ли подстановка, такая как `*`, зависит от того, в каком списке находится запись и от платформы:

* **`allowWrite` и `denyWrite`**: на macOS подстановки работают. На Linux и WSL2 песочница монтирует конкретные пути, поэтому Claude Code пропускает запись, содержащую `*`, `?` или `[`, после удаления конечного `/**`, и эта запись не имеет эффекта. Claude Code добавляет пути из ваших правил разрешения `Edit` в эти списки, поэтому то же ограничение применяется к ним, и вкладка **Config** на `/sandbox` предупреждает о правилах разрешения `Edit` и `Read`, содержащих подстановки.
* **`denyRead` и `allowRead`**: подстановки работают на каждой платформе. На Linux и WSL2 Claude Code расширяет запись чтения до конкретных путей, которые она совпадает, чего он не делает для списков записи.

<h3 id="sandbox-filesystem-allowwrite">
  `sandbox.filesystem.allowWrite`
</h3>

Добавьте пути, где изолированные команды могут писать, помимо рабочего каталога, временного каталога сеанса и каталогов, которые вы добавили с помощью `--add-dir`, `/add-dir` или `permissions.additionalDirectories`. Используйте это, когда подпроцесс, такой как `kubectl` или инструмент сборки, должен писать вне проекта.

* **Scope**: [`Any file`](#scopes)
* **Type**: array of path strings, using the [sandbox path prefixes](#sandbox-path-prefixes)
* **Default**: не установлено, поэтому изолированные команды могут писать в рабочий каталог, временный каталог сеанса, каталоги, которые вы добавили с помощью `--add-dir` или `/add-dir`, и каталоги в [`permissions.additionalDirectories`](#permissions-additionaldirectories)

Это позволяет сборке писать под `/tmp/build` и позволяет `kubectl` обновлять ваш kubeconfig:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"]
    }
  }
}
```

Claude Code объединяет записи во всех областях параметров, которые загружает сеанс: пути пользователя, проекта, локальные и управляемые объединяются, а не заменяют друг друга, и Claude Code добавляет пути из ваших правил разрешения `Edit(...)` allow. Запись `allowWrite` не может снять [protected path](/docs/ru/sandboxing#protected-paths).

<h3 id="sandbox-filesystem-denywrite">
  `sandbox.filesystem.denyWrite`
</h3>

Заблокируйте изолированные команды от записи в определенные пути, включая пути внутри каталога, который в противном случае доступен для записи.

* **Scope**: [`Any file`](#scopes)
* **Type**: array of path strings, using the [sandbox path prefixes](#sandbox-path-prefixes)
* **Default**: не установлено

Это предотвращает изменение системной конфигурации или установку двоичных файлов изолированными командами:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyWrite": ["/etc", "/usr/local/bin"]
    }
  }
}
```

Claude Code объединяет записи во всех областях параметров, которые загружает сеанс, и добавляет пути из ваших правил разрешения `Edit(...)` deny.

<h3 id="sandbox-filesystem-denyread">
  `sandbox.filesystem.denyRead`
</h3>

Заблокируйте изолированные команды от чтения определенных путей, таких как файлы учетных данных, которые политика чтения по умолчанию в противном случае раскрыла бы. Чтобы защитить файл учетных данных и сохранить его пригодным для использования через прокси песочницы, см. [`sandbox.credentials`](#sandbox-credentials) вместо этого.

* **Scope**: [`Any file`](#scopes)
* **Type**: array of path strings, using the [sandbox path prefixes](#sandbox-path-prefixes)
* **Default**: не установлено, поэтому изолированные команды сохраняют [default read access](/docs/ru/sandboxing#filesystem-isolation), который включает файлы учетных данных, такие как `~/.aws/credentials`

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/.aws/credentials"]
    }
  }
}
```

Claude Code объединяет записи во всех областях параметров, которые загружает сеанс, и добавляет пути из ваших правил разрешения `Read(...)` deny. Когда [`filesystem.disabled`](#sandbox-filesystem-disabled) имеет значение `true`, Claude Code не применяет эти записи.

<h3 id="sandbox-filesystem-allowread">
  `sandbox.filesystem.allowRead`
</h3>

Повторно откройте чтение для определенных путей внутри региона, который [`denyRead`](#sandbox-filesystem-denyread) блокирует, чтобы построить доступ только для рабочей области. Точная или подстановочная запись `denyRead` остается заблокированной внутри более широкого `allowRead`, как показывает [overlap table](/docs/ru/sandboxing#configure-sandboxing). Когда подстановочная запись `denyRead`, такая как `~/**/.env`, совпадает с каталогом, Claude Code блокирует чтение его содержимого. До v2.1.236 на macOS Claude Code повторно открывал пути, которые совпадала подстановочная запись `denyRead`, везде, где более широкая запись `allowRead` их охватывала, и оставлял содержимое совпадающего каталога доступным для чтения.

* **Scope**: [`Any file`](#scopes)
* **Type**: array of path strings, using the [sandbox path prefixes](#sandbox-path-prefixes)
* **Default**: не установлено

Это блокирует чтение вашего домашнего каталога, кроме самого проекта:

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

Claude Code разрешает запись `.` в корень проекта в параметрах проекта и в `~/.claude` в параметрах пользователя. Claude Code объединяет записи во всех файлах параметров, которые загружает сеанс, если не установлен [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly).

<h3 id="sandbox-filesystem-allowmanagedreadpathsonly">
  `sandbox.filesystem.allowManagedReadPathsOnly`
</h3>

Соблюдайте только записи [`allowRead`](#sandbox-filesystem-allowread), которые поступают из управляемых параметров, чтобы разработчики не могли повторно открыть доступ для чтения к путям, которые ваша организация заблокировала. Claude Code по-прежнему объединяет записи `denyRead` из каждой области параметров, которые загружает сеанс.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code соблюдает только записи `allowRead` из управляемых параметров
  * `false`: записи `allowRead` объединяются из каждой области параметров, которые загружает сеанс
* **Default**: `false`

Это блокирует чтение домашнего каталога, повторно открывает `~/work` и предотвращает повторное открытие разработчиками чего-либо еще:

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

См. [Keep developers from widening the policy](/docs/ru/sandboxing#keep-developers-from-widening-the-policy).

<h3 id="sandbox-filesystem-disabled">
  `sandbox.filesystem.disabled`
</h3>

Пропустите изоляцию файловой системы, сохраняя изоляцию сети. Изолированные команды получают неограниченный доступ для чтения и записи к файловой системе хоста, и их исходящий трафик сети остается ограниченным [`network.allowedDomains`](#sandbox-network-alloweddomains). Используйте это, когда вы изолируете для контроля того, где подключаются команды, а не что они пишут. Требует Claude Code v2.1.216 или позже.

* **Scope**: [`User or managed`](#scopes). Когда управляемые параметры конфигурируют `sandbox.filesystem` вообще или перечисляют запись `sandbox.credentials.files` с `"mode": "deny"`, только управляемые параметры могут это установить.
* **Type**: Boolean
  * `true`: Claude Code пропускает изоляцию файловой системы и сохраняет изоляцию сети
  * `false`: изоляция файловой системы остается включенной
* **Default**: `false`, поэтому изоляция файловой системы остается включенной

Это оставляет файловую систему открытой и ограничивает исходящий трафик сети GitHub и npm:

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

С отключенным слоем Claude Code не применяет записи `denyRead` или `credentials.files` `deny`, в то время как записи `credentials.envVars` и применяемые записи `mask` продолжают работать. [`autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed) по-прежнему по умолчанию имеет значение `true`, поэтому установите значение `false`, чтобы продолжить запрос. См. [Disable filesystem isolation](/docs/ru/sandboxing#disable-filesystem-isolation) для получения полного списка источников, которые могут это установить, и что изменяется при отключении изоляции. Требует Claude Code v2.1.216 или позже.

<h3 id="sandbox-ignoreviolations">
  `sandbox.ignoreViolations`
</h3>

Подавьте отчеты о нарушениях песочницы для путей, которые вы ожидаете, что команда будет проверять и ей будет отказано, такие как инструмент, который проверяет `/etc/hosts` при запуске, чтобы эти отказы не отображались как нарушения или в том, что видит Claude. Песочница по-прежнему блокирует доступ; подавляется только отчет. Ключи — это подстроки для сопоставления с командой, где `*` совпадает с каждой командой, а значения — это подстроки нарушения, которое нужно игнорировать для этой команды, такие как путь файловой системы.

* **Scope**: [`Any file`](#scopes)
* **Type**: object mapping a command substring to an array of violation substrings, usually paths
* **Default**: не установлено, поэтому каждое нарушение сообщается

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

Запустите песочницу Linux внутри непривилегированного контейнера Docker, где bubblewrap не может смонтировать свежий `/proc`. Вместо этого внутренняя песочница привязывает существующий `/proc` контейнера, который раскрывает информацию о процессе, которую свежее монтирование скрыло бы. Это снижает безопасность; используйте это только когда внешний контейнер уже обеспечивает необходимую вам изоляцию.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: внутренняя песочница привязывает существующий `/proc` контейнера вместо монтирования свежего
  * `false`: песочница монтирует свежий `/proc`, что не работает в непривилегированном контейнере Docker
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNestedSandbox": true
  }
}
```

Только Linux и WSL2. См. [Bubblewrap fails to start inside a container](/docs/ru/sandboxing#troubleshooting).

<h3 id="sandbox-enableweakernetworkisolation">
  `sandbox.enableWeakerNetworkIsolation`
</h3>

Позвольте изолированным командам на macOS достичь системной службы доверия TLS, `com.apple.trustd.agent`. Инструменты на основе Go, такие как `gh`, `gcloud` и `terraform`, нуждаются в этом для проверки сертификатов TLS, когда вы используете [`network.httpProxyPort`](#sandbox-network-httpproxyport) с прокси MITM и пользовательским ЦС. Это снижает безопасность, открывая потенциальный путь утечки данных через службу доверия.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: изолированные команды на macOS могут достичь `com.apple.trustd.agent`
  * `false`: изолированные команды на macOS не могут достичь системной службы доверия TLS
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNetworkIsolation": true
  }
}
```

Если вы не используете прокси MITM, вместо этого перечислите неудачные инструменты в [`excludedCommands`](#sandbox-excludedcommands); см. [Go-based CLIs fail TLS verification on macOS](/docs/ru/sandboxing#troubleshooting).

<h3 id="sandbox-allowappleevents">
  `sandbox.allowAppleEvents`
</h3>

Позвольте изолированным командам на macOS отправлять Apple Events, которые требуют `open`, `osascript` и инструменты, которые открывают URL-адреса в браузере; без этого они не работают с ошибкой `-600`. Это удаляет изоляцию выполнения кода: изолированные команды могут запускать другие приложения без изоляции без подсказки пользователя и могут отправлять команды AppleScript работающим приложениям, таким как Terminal, с учетом подсказки согласия на автоматизацию macOS для каждого приложения (TCC).

* **Scope**: [`User or managed`](#scopes)
* **Type**: Boolean
  * `true`: изолированные команды на macOS могут отправлять Apple Events
  * `false`: изолированные команды на macOS не могут отправлять Apple Events, поэтому `open` и `osascript` не работают с ошибкой `-600`
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowAppleEvents": true
  }
}
```

Чтобы сохранить изоляцию и все еще запустить один такой инструмент, добавьте его в [`excludedCommands`](#sandbox-excludedcommands) вместо этого. См. [Apple Events on macOS](/docs/ru/sandboxing#security-limitations).

<h3 id="sandbox-ripgrep">
  `sandbox.ripgrep`
</h3>

Укажите песочнице на двоичный файл ripgrep вашего собственного вместо того, который использует Claude Code, например, когда ваша платформа требует иначе построенный `rg`.

* **Scope**: [`User or managed`](#scopes)
* **Type**: object с `command`, путем к двоичному файлу ripgrep, и опциональным `args`, массивом аргументов для добавления в начало
* **Default**: не установлено, поэтому песочница использует тот же двоичный файл ripgrep, что и Claude Code. Это встроенный двоичный файл, если вы не установите [`USE_BUILTIN_RIPGREP`](/docs/ru/env-vars) на `0`

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

Укажите песочнице на двоичный файл bubblewrap, установленный вне `PATH`, такой как копия поставщика на хосте без доступа в интернет. Claude Code использует путь как для проверки зависимостей при запуске, так и когда он оборачивает каждую изолированную команду.

* **Scope**: [`Managed`](#scopes). Claude Code читает это только из управляемых параметров, чтобы файл пользователя, проекта или локальный не мог указать песочнице на другой двоичный файл.
* **Type**: string, an absolute path; Claude Code drops a relative path and falls back to `PATH` lookup
* **Default**: не установлено, поэтому Claude Code находит `bwrap` на `PATH`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "bwrapPath": "/opt/admin/bwrap"
  }
}
```

Только Linux и WSL2.

<h3 id="sandbox-socatpath">
  `sandbox.socatPath`
</h3>

Укажите прокси сети песочницы на двоичный файл `socat`, установленный вне `PATH`.

* **Scope**: [`Managed`](#scopes)
* **Type**: string, an absolute path; Claude Code drops a relative path and falls back to `PATH` lookup
* **Default**: не установлено, поэтому Claude Code находит `socat` на `PATH`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "socatPath": "/opt/admin/socat"
  }
}
```

Только Linux и WSL2.

<h3 id="sandbox-credentials">
  `sandbox.credentials`
</h3>

Объявите файлы учетных данных и переменные окружения для [защиты от изолированных команд](/docs/ru/sandboxing#protect-credentials). Каждая запись называет файл `path` или переменную `name` и `mode`: `deny` скрывает учетные данные внутри песочницы, а `mask` показывает изолированным командам заполнитель, в то время как [sandbox proxy](/docs/ru/sandboxing#mask-credentials) подставляет реальное значение в исходящих запросах. Claude Code защищает только записи, которые вы перечисляете; нет встроенного списка отказа учетных данных.

* **Scope**: [`Any file`](#scopes). Claude Code соблюдает записи `mask`, `allowPlaintextInject`, `awsPairs` и `sigv4` только из параметров пользователя, управляемых параметров и флага `--settings`.
* **Type**: object с `files`, `envVars`, `allowPlaintextInject`, `awsPairs` и `sigv4`
* **Default**: не установлено, поэтому никакие учетные данные не защищены

Это скрывает файл учетных данных AWS и удаляет `GITHUB_TOKEN` из изолированных команд:

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

Защита файла `deny` является частью слоя файловой системы, поэтому она не применяется, когда вы [disable filesystem isolation](/docs/ru/sandboxing#disable-filesystem-isolation); защита переменной окружения по-прежнему применяется.

<h4 id="invalid-credential-entries-in-managed-settings">
  Недействительные записи учетных данных в управляемых параметрах
</h4>

Когда запись `sandbox.credentials` в управляемых параметрах не проходит проверку, Claude Code продолжает защищать учетные данные, где может:

* Запись в `files` или `envVars`, которая по-прежнему имеет действительный `path` или `name` и `mode` из `mask` или `deny`, такая как запись, чей шаблон `extract` не имеет группы захвата, деградирует до `mode: "deny"` с предупреждением, поэтому учетные данные остаются заблокированными, а не замаскированными, пока вы не исправите запись. Деградированная запись `files` закрепляет [`filesystem.disabled`](/docs/ru/sandboxing#disable-filesystem-isolation) как явная запись `deny`, и предупреждение отмечает, что ее блок чтения не применяется, если управляемые параметры отключают изоляцию файловой системы.
* Запись с неизвестным `mode` или недействительным `path` или `name` удаляется.
* Каждый случай предупреждает; независимо от того, деградирована ли запись или удалена, оставшиеся действительные записи по-прежнему применяются, и полностью недействительное значение `credentials` удаляется, в то время как остальная часть `sandbox` по-прежнему применяется.

Применяется в v2.1.191 и позже; до v2.1.221 каждая недействительная запись была удалена. Для других управляемых ключей с обработкой для каждого поля см. [Invalid entries in managed settings](/docs/ru/managed-settings#invalid-entries-in-managed-settings).

<h3 id="sandbox-credentials-files">
  `sandbox.credentials.files`
</h3>

Защитите файлы или каталоги учетных данных от изолированных команд. С `"mode": "deny"`, Claude Code блокирует чтение пути внутри песочницы, тот же блок чтения, что и [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread). С `"mode": "mask"`, изолированные команды на Linux и WSL2 читают копию файла-дозорного, и прокси песочницы подставляет реальное значение в исходящих запросах к `injectHosts` этой записи; на macOS файл вместо этого нечитаем внутри песочницы. `"mode": "mask"` требует Claude Code v2.1.221 или позже.

* **Scope**: [`Any file`](#scopes). Claude Code удаляет записи `mask` из проекта `.claude/settings.json` и локального `.claude/settings.local.json`.
* **Type**: array of objects, each with `path` and a `mode` of `"deny"` or `"mask"`, plus the optional [mask fields for files](#mask-fields-for-files)
* **Default**: не установлено, поэтому никакие файлы учетных данных не защищены

Это скрывает файл учетных данных AWS и маскирует файл хостов `gh`, подставляя реальное значение только в запросах к `api.github.com`:

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

Пути используют те же [prefixes](#sandbox-path-prefixes), что и параметры `sandbox.filesystem.*`, и Claude Code объединяет массивы из каждой области параметров, которые загружает сеанс. [Protect credentials](/docs/ru/sandboxing#protect-credentials) охватывает то, что по-прежнему применяется из источников, которые вы исключаете с помощью `--setting-sources`. `mask` записи требуют Claude Code v2.1.221 или позже.

Подстановка `mask` работает только через прокси песочницы, поэтому установите [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate) или [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject) для простых сетей HTTP-тестирования. `mask` применяется к одному файлу, поэтому перечислите каждый файл учетных данных отдельно. Claude Code принимает, но игнорирует поля `mask` в записи `deny`. [Mask credential files](/docs/ru/sandboxing#mask-credential-files) охватывает, какие источники параметров соблюдаются и когда запись возвращается к `deny`.

<span id="sandbox-credentials-files-extract" />

<span id="sandbox-credentials-files-onextractnomatch" />

<span id="sandbox-credentials-files-decode" />

<span id="sandbox-credentials-files-maskclaims" />

<span id="sandbox-credentials-files-maskduplicates" />

<span id="sandbox-credentials-files-injecthosts" />

<h4 id="mask-fields-for-files">
  Поля маски для файлов
</h4>

Запись `mask` принимает эти опциональные поля. Без `extract` или `decode`, Claude Code заменяет все содержимое файла одним дозорным. На macOS с включенной изоляцией файловой системы Claude Code применяет запись `mask` как `deny` перед запуском `extract` или `decode`; см. [Mask credential files](/docs/ru/sandboxing#mask-credential-files).

| Field              | Type                                                                                                               | What it does                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :----------------- | :----------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `extract`          | string, a regular expression with at least one capturing group                                                     | Маскируйте только текст, захваченный группой 1 каждого совпадения, чтобы остальная часть файла оставалась анализируемой. Если также установлен `decode`, Claude Code проверяет каждый захват как возможный JWT вместо прямой замены. Требует v2.1.221 или позже                                                                                                                                                                                                                                                                                                                                                                 |
| `onExtractNoMatch` | `"warn"`, `"deny"` или `"error"`; default `"warn"`                                                                 | Что происходит, когда `extract` или `decode` ничего не находит для маскирования. `warn` оставляет файл доступным для чтения как есть внутри песочницы, `deny` делает его нечитаемым, а `error` останавливает настройку песочницы, пока вы не исправите конфигурацию. Claude Code рассматривает `deny` как `error`, когда блок чтения не будет применяться, потому что вы [disable filesystem isolation](/docs/ru/sandboxing#disable-filesystem-isolation) или запись [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread) повторно открывает путь. Требует v2.1.221 или позже; случай `decode` требует v2.1.224 или позже |
| `decode`           | the string `"jwt"`                                                                                                 | Найдите JSON Web Tokens (JWTs) в файле, со встроенным шаблоном или с `extract`, когда установлен, проверьте каждого кандидата и замените его структурно действительным поддельным токеном, чтобы код внутри песочницы, который декодирует токен, продолжал работать. Когда ни один кандидат не проверяется, `onExtractNoMatch` управляет результатом. Требует v2.1.224 или позже                                                                                                                                                                                                                                                |
| `maskClaims`       | array of strings, at least one claim name; requires `decode`                                                       | Маскируйте только названные утверждения верхнего уровня полезной нагрузки внутри каждого проверенного JWT и перестройте токен вокруг измененной полезной нагрузки, чтобы другие утверждения оставались доступными для чтения. Когда ни одно названное утверждение не совпадает, `onExtractNoMatch` управляет результатом. Требует v2.1.224 или позже                                                                                                                                                                                                                                                                            |
| `maskDuplicates`   | Boolean, default `false`                                                                                           | Также замените дословные копии каждого замаскированного значения в другом месте файла, такие как секрет, вставленный в комментарий. Claude Code совпадает с необработанными подстроками, поэтому зарезервируйте это для длинных, высокоэнтропийных секретов. Консультируется только когда установлены `extract` или `decode`. Требует v2.1.221 или позже                                                                                                                                                                                                                                                                        |
| `injectHosts`      | array of strings, each a host that [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) also admits | Сузьте хосты, где прокси песочницы подставляет реальное значение. Когда не установлено, прокси подставляет его в запросы каждому хосту в `sandbox.network.allowedDomains`. Требует v2.1.221 или позже                                                                                                                                                                                                                                                                                                                                                                                                                           |

Это маскирует только значение `oauth_token` в файле хостов `gh`, заменяет каждую другую копию его в файле, делает файл нечитаемым, если шаблон ничего не совпадает, и подставляет реальный токен только в запросах к `api.github.com`:

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

Защитите переменные окружения от изолированных команд. С `"mode": "deny"`, Claude Code удаляет переменную из окружения изолированных команд. С `"mode": "mask"`, изолированные команды видят значение дозорного для каждого сеанса, и прокси песочницы подставляет реальное значение в исходящих запросах к `injectHosts` этой записи, поэтому инструменты, такие как `gh` и `npm`, продолжают аутентифицироваться без когда-либо удержания реальных учетных данных. `"mode": "mask"` требует Claude Code v2.1.199 или позже.

* **Scope**: [`Any file`](#scopes). Claude Code удаляет записи `mask` из проекта `.claude/settings.json` и локального `.claude/settings.local.json`.
* **Type**: array of objects, each with `name` and a `mode` of `"deny"` or `"mask"`, plus the optional [mask fields for environment variables](#mask-fields-for-environment-variables)
* **Default**: не установлено, поэтому никакие переменные окружения не защищены

Это удаляет `NPM_TOKEN` из изолированных команд и маскирует `GITHUB_TOKEN`, подставляя реальное значение только в запросах к `api.github.com`:

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

`name` должно начинаться с буквы или подчеркивания и содержать только буквы, цифры и подчеркивания. Claude Code объединяет массивы из каждой области параметров, которые загружает сеанс, и применяет `deny`, когда одна и та же переменная появляется с обоими режимами. [Protect credentials](/docs/ru/sandboxing#protect-credentials) охватывает то, что по-прежнему применяется из источников, которые вы исключаете с помощью `--setting-sources`. `mask` записи требуют Claude Code v2.1.199 или позже.

Подстановка `mask` работает только через прокси песочницы, поэтому установите [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate) или [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject) для простых сетей HTTP-тестирования; см. [Mask environment variables](/docs/ru/sandboxing#mask-environment-variables). Claude Code принимает, но игнорирует поля `mask` в записи `deny`.

<span id="sandbox-credentials-envvars-extract" />

<span id="sandbox-credentials-envvars-onextractnomatch" />

<span id="sandbox-credentials-envvars-decode" />

<span id="sandbox-credentials-envvars-maskclaims" />

<span id="sandbox-credentials-envvars-injecthosts" />

<h4 id="mask-fields-for-environment-variables">
  Поля маски для переменных окружения
</h4>

Запись `mask` принимает эти опциональные поля. Без `extract` или `decode`, Claude Code заменяет все значение одним дозорным. `extract` и `decode` не могут быть объединены в одной записи.

| Field              | Type                                                                                                               | What it does                                                                                                                                                                                                                                                                                                                                                                                     |
| :----------------- | :----------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `extract`          | string, a regular expression with at least one capturing group                                                     | Маскируйте только текст, захваченный группой 1 каждого совпадения, такой как пароль внутри строки подключения `DATABASE_URL`, чтобы остальная часть значения оставалась анализируемой. Требует v2.1.224 или позже                                                                                                                                                                                |
| `onExtractNoMatch` | `"warn"`, `"deny"` или `"error"`; default `"warn"`. На записи с `decode` принимается только `"warn"`               | Что происходит, когда `extract` ничего не совпадает. `warn` передает переменную без маскирования, `deny` отменяет ее внутри песочницы, а `error` останавливает настройку песочницы, пока вы не исправите конфигурацию. Требует v2.1.224 или позже                                                                                                                                                |
| `decode`           | the string `"jwt"`                                                                                                 | Проверьте, что все значение является JWT, и замените его структурно действительным поддельным токеном, чтобы код внутри песочницы, который декодирует токен, продолжал работать; прокси подставляет весь реальный токен при выходе. Значение, которое не проверяется, проходит без маскирования с предупреждением. Требует v2.1.224 или позже                                                    |
| `maskClaims`       | array of strings, at least one claim name; requires `decode`                                                       | Маскируйте только названные утверждения верхнего уровня полезной нагрузки внутри декодированного JWT и перестройте токен вокруг измененной полезной нагрузки, чтобы другие утверждения оставались доступными для чтения. Когда ни одно названное утверждение не совпадает, переменная проходит без маскирования с предупреждением. Требует v2.1.224 или позже                                    |
| `injectHosts`      | array of strings, each a host that [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) also admits | Сузьте хосты, где прокси песочницы подставляет реальное значение. Когда не установлено, прокси подставляет его в запросы каждому хосту в `sandbox.network.allowedDomains`. Напишите пункт назначения IPv6 как голый сжатый адрес, такой как `"::1"`, а не форму в скобках; см. [IPv6 destinations in `injectHosts`](/docs/ru/sandboxing#ipv6-destinations-in-injecthosts). Требует v2.1.199 или позже |

Это маскирует только пароль внутри `DATABASE_URL`, отменяет переменную, если шаблон ничего не совпадает, и маскирует JWT в `SERVICE_JWT`, оставляя каждое утверждение, кроме `api_key`, доступным для чтения:

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

Разрешите подстановку `mask` в простых HTTP-запросах, а также в HTTPS с завершением TLS. На простом HTTP восходящая идентичность не проверяется и учетные данные передаются в открытом виде, поэтому оставьте это отключенным вне доверенных тестовых сетей. Требует Claude Code v2.1.199 или позже.

* **Scope**: [`User or managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code разрешает подстановку `mask` в простых HTTP-запросах, а также в HTTPS с завершением TLS
  * `false`: Claude Code разрешает подстановку `mask` только в HTTPS с завершением TLS
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

Требует Claude Code v2.1.199 или позже.

<h3 id="sandbox-credentials-awspairs">
  `sandbox.credentials.awsPairs`
</h3>

Сгруппируйте замаскированные переменные окружения, которые образуют одни учетные данные AWS для [SigV4 re-signing](/docs/ru/sandboxing#re-sign-aws-requests), когда ваши учетные данные находятся в переменных с нестандартными именами. Claude Code автоматически связывает обычный триумвират `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` и `AWS_SESSION_TOKEN`, когда вы маскируете их целые значения, поэтому вам нужен этот ключ только для других имен. Требует Claude Code v2.1.224 или позже.

* **Scope**: [`User or managed`](#scopes)
* **Type**: array of objects, each with `accessKeyIdVar`, `secretAccessKeyVar`, and optionally `sessionTokenVar`, naming `sandbox.credentials.envVars` entries
* **Default**: не установлено, поэтому только обычный триумвират спарен

Это связывает три пользовательские переменные в одни учетные данные AWS для повторного подписания:

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

Каждая названная переменная должна быть записью `mask` целого значения в [`sandbox.credentials.envVars`](#sandbox-credentials-envvars), без `extract` или `decode`, и может заполнять только один слот во всех парах.

<h3 id="sandbox-credentials-sigv4">
  `sandbox.credentials.sigv4`
</h3>

Выберите, что прокси песочницы делает с формами запроса AWS, которые он [не может повторно подписать](/docs/ru/sandboxing#re-sign-aws-requests): `streaming` для потоковых загрузок aws-chunked, `presigned` для предписанных URL-адресов и `sigv4a` для асимметричных подписей SigV4A. Это применяется только к запросам, подписанным с помощью заполнителя ID ключа доступа замаскированной пары. Требует Claude Code v2.1.224 или позже.

* **Scope**: [`User or managed`](#scopes)
* **Type**: object с `streaming`, `presigned` и `sigv4a`, каждый один из:
  * `"deny"`: прокси не выполняет запрос
  * `"passthrough"`: прокси пересылает запрос, подписанный замаскированным заполнителем, поэтому инструмент получает собственный отказ AWS
* **Default**: не установлено, поэтому каждая форма — это `"deny"`

Это пересылает потоковые загрузки вместо их отказа на прокси:

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

С `deny` прокси не выполняет запрос. С `passthrough` прокси пересылает запрос с его подписью, вычисленной из замаскированного заполнителя, поэтому AWS его отклоняет и вызывающий инструмент получает собственный ответ AWS вместо ошибки прокси.

<h3 id="sandbox-network">
  `sandbox.network`
</h3>

Контролируйте, какие хосты, порты и сокеты могут достичь изолированные команды. Песочница маршрутизирует исходящий трафик через прокси, который применяет эти списки; см. [Network isolation](/docs/ru/sandboxing#network-isolation) для получения информации о том, как прокси решает и когда он запрашивает.

* **Scope**: [`Any file`](#scopes). `strictAllowlist`, `allowManagedDomainsOnly` и `tlsTerminate` читаются из меньшего количества источников, как указано в их записях.
* **Type**: object с подключами ниже
* **Default**: не установлено, поэтому никакие домены не предварительно разрешены и песочница запрашивает каждого нового хоста

Это предварительно разрешает GitHub и npm, блокирует `uploads.github.com` и позволяет командам привязываться к localhost:

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

Claude Code объединяет подключи массива во всех областях параметров и дедуплицирует их, поэтому проект может добавлять домены в ваш список пользователей. Правила разрешения `WebFetch(domain:...)` allow и deny [permission rules](/docs/ru/sandboxing#permission-rules) питают те же списки allow и deny.

<h3 id="sandbox-network-allowunixsockets">
  `sandbox.network.allowUnixSockets`
</h3>

Перечислите пути Unix-сокетов, к которым изолированные команды могут подключаться на macOS. Claude Code игнорирует этот список на Linux и WSL2, где фильтр seccomp не может проверять пути сокетов; используйте [`allowAllUnixSockets`](#sandbox-network-allowallunixsockets) вместо этого.

* **Scope**: [`Any file`](#scopes)
* **Type**: array of strings, each a socket path
* **Default**: не установлено, поэтому песочница macOS блокирует каждый Unix-сокет

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowUnixSockets": ["~/.ssh/agent-socket"]
    }
  }
}
```

Путь сокета может предоставить широкий доступ: разрешение `/var/run/docker.sock`, например, позволяет изолированной команде контролировать демон Docker. См. [Security limitations](/docs/ru/sandboxing#security-limitations).

<h3 id="sandbox-network-allowallunixsockets">
  `sandbox.network.allowAllUnixSockets`
</h3>

Позвольте изолированным командам подключаться к каждому Unix-сокету. На Linux и WSL2 [seccomp filter](/docs/ru/sandboxing#set-up-linux-and-wsl2) песочницы блокирует вызовы `socket(AF_UNIX, ...)`, поэтому это единственный способ разрешить Unix-сокеты там. Когда фильтр отсутствует, что `/sandbox` сообщает на вкладке Dependencies, песочница не блокирует вызовы Unix-сокетов. См. [Set up Linux and WSL2](/docs/ru/sandboxing#set-up-linux-and-wsl2) для получения информации о том, откуда берется фильтр.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: изолированные команды могут подключаться к каждому Unix-сокету
  * `false`: песочница блокирует подключения Unix-сокетов: на macOS, кроме путей в `allowUnixSockets`, и на Linux и WSL2 через фильтр seccomp, когда он присутствует
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

На WSL2 `true` также повторно открывает сокет interop, который запускает двоичные файлы Windows, такие как `cmd.exe` и `powershell.exe`.

<h3 id="sandbox-network-allowlocalbinding">
  `sandbox.network.allowLocalBinding`
</h3>

Позвольте изолированным командам привязываться к портам localhost на macOS, например, для запуска сервера разработки.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: изолированные команды могут привязываться к портам localhost на macOS
  * `false`: изолированные команды на macOS не могут привязываться к портам localhost
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

Перечислите дополнительные имена служб XPC и Mach, которые песочница macOS может искать. Инструменты, которые взаимодействуют через XPC, такие как iOS Simulator или Playwright, нуждаются в своих услугах, перечисленных здесь.

* **Scope**: [`Any file`](#scopes)
* **Type**: array of strings, each a service name; a single trailing `*` matches a prefix, and `"*"` alone matches every service
* **Default**: не установлено

Это разрешает каждую услугу под префиксом `com.apple.coresimulator.`:

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

Предварительно разрешите домены для исходящего трафика из изолированных команд, чтобы песочница не запрашивала их. Подстановки, такие как `*.example.com`, совпадают с поддоменами, и опциональный суффикс `:port` ограничивает запись одним портом; запись без порта совпадает с каждым портом.

* **Scope**: [`Any file`](#scopes). Только управляемые параметры, когда установлен [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly).
* **Type**: array of strings, each a domain, wildcard pattern, or IP literal, with an optional `:port` suffix
* **Default**: не установлено, поэтому песочница запрашивает первый раз, когда команда достигает нового хоста

Это предварительно разрешает GitHub на каждом порту, каждый поддомен npm и один хост API только на порту 443:

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org", "api.example.com:443"]
    }
  }
}
```

Напишите литералы IPv6 в скобках, с опциональным портом: `"[::1]"` разрешает каждый порт и `"[::1]:443"` один порт. Форма в скобках требует Claude Code v2.1.229 или позже. См. [IPv6 addresses in domain lists](/docs/ru/sandboxing#ipv6-addresses-in-domain-lists).

<h3 id="sandbox-network-denieddomains">
  `sandbox.network.deniedDomains`
</h3>

Заблокируйте домены для исходящего трафика из изолированных команд, используя тот же синтаксис подстановки, порта и IPv6, что и [`allowedDomains`](#sandbox-network-alloweddomains). Запрещенный домен остается заблокированным даже когда запись `allowedDomains` также совпадает с ним.

* **Scope**: [`Any file`](#scopes)
* **Type**: array of strings, each a domain, wildcard pattern, or IP literal, with an optional `:port` suffix
* **Default**: не установлено

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "deniedDomains": ["sensitive.cloud.example.com"]
    }
  }
}
```

Claude Code объединяет этот список из каждого источника параметров, который загружает сеанс, даже когда установлен `allowManagedDomainsOnly`, поэтому разработчик всегда может затянуть список отказа. Для литералов IPv6 см. [IPv6 addresses in domain lists](/docs/ru/sandboxing#ipv6-addresses-in-domain-lists).

Запись, написанная с конечной точкой, которая отмечает полностью квалифицированное доменное имя, такая как `example.com.`, блокирует те же подключения, что и `example.com`.

<h3 id="sandbox-network-strictallowlist">
  `sandbox.network.strictAllowlist`
</h3>

Запретите изолированным командам доступ к хостам вне списка разрешений вместо запроса одобрения. Список разрешений — это [`allowedDomains`](#sandbox-network-alloweddomains) плюс домены из правил `WebFetch(domain:...)` allow, или только записи управляемых параметров, когда установлен [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly). Требует Claude Code v2.1.219 или позже.

* **Scope**: [`User or managed`](#scopes). Репозиторий не может его включить или отключить.
* **Type**: Boolean
  * `true`: Claude Code запрещает изолированным командам доступ к хостам вне списка разрешений
  * `false`: если другой доверенный файл параметров не установит `true`, Claude Code решает хост вне списка разрешений по режиму разрешения вместо прямого отказа: в режиме auto он проверяет хост против [per-command allowed domains](/docs/ru/sandboxing#per-command-allowed-domains-in-auto-mode) команды, в режиме `dontAsk` отказывает, в режиме `bypassPermissions` и в интерактивных сеансах режима плана терминала, где доступен обход, разрешает, и в противном случае спрашивает вас
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

Claude Code применяет это только для изолированных команд; встроенные инструменты, такие как `WebFetch`, по-прежнему следуют своим [permission rules](/docs/ru/sandboxing#permission-rules). Когда любой из соблюдаемых источников установит это на `true`, оно остается включенным. См. [Network isolation](/docs/ru/sandboxing#network-isolation). Требует Claude Code v2.1.219 или позже.

<h3 id="sandbox-network-allowmanageddomainsonly">
  `sandbox.network.allowManagedDomainsOnly`
</h3>

Заблокируйте список разрешений сети на то, что определяют управляемые параметры. Claude Code затем соблюдает только `allowedDomains` и правила `WebFetch(domain:...)` allow из управляемых параметров, игнорирует домены из параметров пользователя, проекта, локальных и `--settings`, и автоматически блокирует домен, не разрешенный, вместо запроса.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code соблюдает только `allowedDomains` и правила `WebFetch(domain:...)` allow из управляемых параметров и блокирует домен, не разрешенный, вместо запроса
  * `false`: домены из параметров пользователя, проекта, локальных и `--settings` объединяются в список разрешений
* **Default**: `false`

Это блокирует список разрешений на GitHub и npm и игнорирует любые домены, которые добавляют разработчики:

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

Запрещенные домены по-прежнему объединяются из каждого источника, который загружает сеанс. См. [Keep developers from widening the policy](/docs/ru/sandboxing#keep-developers-from-widening-the-policy).

<h3 id="sandbox-network-httpproxyport">
  `sandbox.network.httpProxyPort`
</h3>

Укажите песочнице на ваш собственный HTTP-прокси вместо того, который запускает Claude Code. Организации делают это для проверки трафика HTTPS, применения своих собственных правил фильтрации или регистрации каждого запроса. Когда не установлено, Claude Code запускает свой собственный прокси для трафика HTTP.

* **Scope**: [`Any file`](#scopes)
* **Type**: number, a local TCP port
* **Default**: не установлено, поэтому Claude Code запускает свой собственный прокси

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080
    }
  }
}
```

Также установите [`socksProxyPort`](#sandbox-network-socksproxyport), если ваш прокси должен переносить трафик SOCKS; с установленным только одним из двух, Claude Code по-прежнему запускает свой собственный прокси для другого протокола. См. [Custom proxy configuration](/docs/ru/sandboxing#custom-proxy-configuration).

<h3 id="sandbox-network-socksproxyport">
  `sandbox.network.socksProxyPort`
</h3>

Укажите песочнице на ваш собственный прокси SOCKS5 вместо того, который запускает Claude Code. Когда не установлено, Claude Code запускает свой собственный прокси для трафика SOCKS.

* **Scope**: [`Any file`](#scopes)
* **Type**: number, a local TCP port
* **Default**: не установлено, поэтому Claude Code запускает свой собственный прокси

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "socksProxyPort": 8081
    }
  }
}
```

См. [Custom proxy configuration](/docs/ru/sandboxing#custom-proxy-configuration).

<h3 id="sandbox-network-tlsterminate">
  `sandbox.network.tlsTerminate`
</h3>

Заставьте прокси песочницы завершить TLS, чтобы он мог читать содержимое HTTPS-запросов. Это экспериментально, и подстановка [credential substitution](/docs/ru/sandboxing#mask-credentials) `mask` требует это. Установите `{}` для создания эфемерного центра сертификации для сеанса или установите `caCertPath` и `caKeyPath` для использования вашего собственного.

* **Scope**: [`User or managed`](#scopes). Репозиторий не может его включить или предоставить центр сертификации.
* **Type**: object с опциональными строками `caCertPath` и `caKeyPath`, каждая путь файла
* **Default**: не установлено, поэтому прокси не завершает или не проверяет TLS

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "tlsTerminate": {}
    }
  }
}
```

Когда более одного соблюдаемого источника это установит, Claude Code использует значение из источника с наивысшим приоритетом: управляемые параметры, затем флаг `--settings`, затем параметры пользователя. Требует Claude Code v2.1.199 или позже.

<span id="context-and-memory" />

<h2 id="memory-and-context">
  Память и контекст
</h2>

Контролируйте, что Claude Code загружает в контекст, как он выполняет компактизацию и где хранит память и планы. См. [Управление контекстом](/docs/ru/context-window) и [Память](/docs/ru/memory).

<h3 id="autocompactenabled">
  `autoCompactEnabled`
</h3>

Позвольте Claude Code [автоматически компактизировать беседу](/docs/ru/context-window#when-your-context-fills-up) при приближении контекста к лимиту. Отображается в `/config` как **Auto-compact**, и переключение его там записывает этот ключ в ваши пользовательские параметры.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code автоматически компактизирует беседу при приближении контекста к лимиту
  * `false`: Claude Code не выполняет автоматическую компактизацию
* **Default**: `true`
* **Per-session overrides**: [`DISABLE_AUTO_COMPACT`](/docs/ru/env-vars) отключает автоматическую компактизацию на одну сессию; какой бы из двух параметров её ни отключил, другой не может её включить обратно

```json settings.json theme={null}
{
  "autoCompactEnabled": false
}
```

Команда `/compact` продолжает работать, пока автоматическая компактизация отключена.

<h3 id="autocompactwindow">
  `autoCompactWindow`
</h3>

Установите, насколько полным становится окно контекста перед тем, как Claude Code [автоматически выполняет компактизацию](/docs/ru/context-window#when-your-context-fills-up).

* **Scope**: [`Any file`](#scopes)
* **Type**: количество токенов от `100000` до `1000000`. Claude Code ограничивает значение окном контекста вашей модели; [обзор моделей](https://platform.claude.com/docs/en/about-claude/models/overview) перечисляет окно каждой модели
* **Default**: не установлено, поэтому Claude Code выбирает окно, оптимизированное для вашей модели
* **Per-session overrides**: [`--autocompact`](/docs/ru/cli-reference#cli-flags) имеет приоритет над этим ключом на одну сессию, и [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/ru/env-vars) имеет приоритет над обоими

```json settings.json theme={null}
{
  "autoCompactWindow": 500000
}
```

Установите его с помощью команды [`/autocompact`](/docs/ru/commands#all-commands), которая записывает этот ключ в ваши пользовательские параметры. [Установка окна автоматической компактизации](/docs/ru/model-config#set-the-auto-compact-window) описывает, как команда, флаг, переменная и параметр взаимодействуют.

<h3 id="automemorydirectory">
  `autoMemoryDirectory`
</h3>

Сохраняйте [автоматическую память](/docs/ru/memory#storage-location) в выбранном вами каталоге вместо стандартного каталога для каждого проекта.

* **Scope**: [`Any file`](#scopes)
* **Type**: строка, абсолютный путь каталога или путь с префиксом `~/`
* **Default**: не установлено, поэтому Claude Code использует `~/.claude/projects/<project>/memory/`

```json settings.json theme={null}
{
  "autoMemoryDirectory": "~/my-memory-dir"
}
```

Из параметров проекта или локальных параметров Claude Code соблюдает этот ключ в соответствии с тем же [правилом доверия рабочей области, что и для hooks](/docs/ru/permissions#what-runs-before-you-trust-a-folder), поскольку клонированный репозиторий может предоставить эти файлы.

<h3 id="automemoryenabled">
  `autoMemoryEnabled`
</h3>

Включите или отключите [автоматическую память](/docs/ru/memory#enable-or-disable-auto-memory). Когда `false`, Claude не читает и не записывает в каталог автоматической памяти. Вы также можете переключить её с помощью `/memory` во время сессии, что записывает этот ключ в ваши пользовательские параметры.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: то же самое, что и не установлено; автоматическая память остаётся включённой, если что-то, имеющее приоритет над этим ключом, не отключит её на сессию, например `--bare`, безопасный режим или `CLAUDE_CODE_DISABLE_AUTO_MEMORY`
  * `false`: Claude не читает и не записывает в каталог автоматической памяти
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_AUTO_MEMORY`](/docs/ru/env-vars) имеет приоритет над этим ключом на одну сессию в любом направлении

```json settings.json theme={null}
{
  "autoMemoryEnabled": false
}
```

<h3 id="bashoutputmaxchars">
  `bashOutputMaxChars`
</h3>

Установите, сколько символов вывода успешной команды Bash или PowerShell [Claude получает встроенным образом](/docs/ru/tools-reference#output-limits). Когда вывод превышает лимит, Claude Code сохраняет его в файл, и Claude получает краткий предпросмотр плюс путь к файлу. Увеличьте лимит, когда вывод команды, такой как подробная сборка или полный журнал набора тестов, регулярно превышает значение по умолчанию и вы хотите, чтобы Claude прочитал его без открытия файла. Требуется Claude Code v2.1.261 или позже.

* **Scope**: [`Any file`](#scopes)
* **Type**: количество символов, положительное целое число. Claude Code ограничивает значение диапазоном `4000` на `128000`
* **Default**: не установлено, поэтому Claude получает до 30 000 символов встроенным образом

```json settings.json theme={null}
{
  "bashOutputMaxChars": 100000
}
```

Когда вы устанавливаете этот ключ, Claude Code игнорирует переменную окружения [`BASH_MAX_OUTPUT_LENGTH`](/docs/ru/env-vars).

<h3 id="claudemd">
  `claudeMd`
</h3>

Внедрите инструкции в стиле CLAUDE.md как управляемую организацией память без развёртывания отдельного файла. Claude Code загружает текст как запись управляемой памяти перед файлами CLAUDE.md пользователя и проекта.

* **Scope**: [`Managed`](#scopes)
* **Type**: строка, текст файла CLAUDE.md; напишите его так, как вы бы написали файл, включая Markdown, с разрывами строк как `\n`
* **Default**: не установлено

Этот пример развёртывает два правила как короткий список Markdown:

```json managed-settings.json theme={null}
{
  "claudeMd": "# Engineering rules\n\n- Always run make lint before committing.\n- Never push directly to main."
}
```

См. [Развёртывание CLAUDE.md на уровне организации](/docs/ru/memory#deploy-organization-wide-claude-md).

<h3 id="claudemdexcludes">
  `claudeMdExcludes`
</h3>

Пропускайте определённые файлы `CLAUDE.md` при загрузке Claude Code [памяти](/docs/ru/memory#exclude-specific-claude-md-files). В большом монорепозитории используйте его для пропуска файлов CLAUDE.md из других команд, которые не относятся к вашей работе; [Исключение нерелевантных файлов CLAUDE.md](/docs/ru/large-codebases#exclude-irrelevant-claude-md-files) в руководстве по большим кодовым базам проходит через этот случай. Шаблоны совпадают с абсолютными путями файлов.

* **Scope**: [`Any file`](#scopes)
* **Type**: массив строк, каждая из которых является шаблоном glob или абсолютным путём
* **Default**: не установлено, поэтому Claude Code загружает каждый найденный файл CLAUDE.md

```json settings.json theme={null}
{
  "claudeMdExcludes": ["**/vendor/**/CLAUDE.md"]
}
```

Исключения применяются только к файлам памяти пользователя, проекта и локальной памяти; файлы CLAUDE.md управляемой политики не могут быть исключены.

<span id="environment-variables" />

<h3 id="env">
  `env`
</h3>

Установите переменные окружения для каждой сессии и для подпроцессов, которые Claude Code запускает из неё. Большинство переменных в [справочнике переменных окружения](/docs/ru/env-vars) могут быть здесь, что позволяет применить одну к каждой сессии или развернуть её в вашей команде. Параметры проекта и локальные параметры не могут устанавливать [некоторые из них](#variables-claude-code-ignores-in-env).

* **Scope**: [`Any file`](#scopes)
* **Type**: объект, отображающий имена переменных на строковые значения
* **Default**: не установлено

Этот пример отключает автоматическую компактизацию и маршрутизирует запросы API через прокси:

```json settings.json theme={null}
{
  "env": {
    "DISABLE_AUTO_COMPACT": "1",
    "ANTHROPIC_BASE_URL": "https://proxy.example.com"
  }
}
```

<h4 id="how-env-values-interact-with-your-shell">
  Как значения `env` взаимодействуют с вашей оболочкой
</h4>

* Значение здесь перезаписывает ту же переменную, экспортированную в вашей оболочке, и когда более одного файла параметров устанавливает переменную, применяется [наиболее приоритетный](/docs/ru/settings#settings-precedence). [Переменные, которые Claude Code игнорирует в `env`](#variables-claude-code-ignores-in-env) перечисляет исключения для параметров проекта и локальных параметров.
* Чтобы отменить экспорт оболочки, установите переменную на `""`. Claude Code рассматривает пустое значение как не установленное для выбора поставщика, и подпроцессы наследуют пустое значение.
* `NO_COLOR` и `FORCE_COLOR`, установленные здесь, достигают только подпроцессов. Чтобы изменить цвета собственного интерфейса Claude Code, установите их в вашей оболочке перед запуском `claude`.
* Значения здесь представляют собой простой текст в файле параметров и достигают каждого подпроцесса, который запускает Claude Code. Для токена OTLP, который ротируется, используйте [`otelHeadersHelper`](#otelheadershelper); для учётных данных API используйте [`apiKeyHelper`](#apikeyhelper).

<h4 id="when-claude-code-applies-env-values">
  Когда Claude Code применяет значения `env`
</h4>

* Из пользовательских параметров, `--settings` и управляемых параметров: при запуске и снова в работающей сессии, когда сохранённое изменение изменяет объединённый `env`.
* Из параметров проекта и локальных параметров: после того, как вы доверяете рабочей области, или при запуске в режиме `-p`, который никогда не показывает диалог доверия, и снова, когда сохранённое изменение изменяет объединённый `env`.
* Переменные, которые Claude Code классифицирует как безопасные, такие как выбор модели, тайм-ауты и лимиты, и переключатели функций: при запуске из каждого файла параметров, кроме [переменных, которые параметры проекта и локальные параметры не могут устанавливать](#variables-claude-code-ignores-in-env).
* После того, как вы [переместите сессию с помощью `/cd`](/docs/ru/permissions#move-the-session-to-another-directory) на v2.1.246 или позже: значения `env` нового каталога проекта и локальные, поверх предыдущего каталога.

<h4 id="variables-claude-code-ignores-in-env">
  Переменные, которые Claude Code игнорирует в `env`
</h4>

* Параметры проекта и локальные параметры не могут устанавливать переменные, которые проверенный репозиторий не должен контролировать; установите их в вашей оболочке, пользовательских параметрах или управляемых параметрах вместо этого. Claude Code отбрасывает каждую и регистрирует предупреждение, которое вы можете увидеть с помощью `claude --debug`. Они включают:

  * Переменные, которые выбирают, где Claude Code хранит или записывает свои собственные файлы: `CLAUDE_CONFIG_DIR`, `CLAUDE_CODE_TMPDIR` и переменные каталога операционной системы, такие как `HOME`, `TMPDIR`, `TMP`, `TEMP` и семейство `XDG_*`.
  * Переменные, которые экспортируют содержимое сессии: [`OTEL_LOG_RAW_API_BODIES`](/docs/ru/env-vars#variables) и подробная бета-версия пары трассировки `ENABLE_BETA_TRACING_DETAILED` и `BETA_TRACING_ENDPOINT`.
  * Переменные [OpenTelemetry exporter](/docs/ru/monitoring-usage), которые включают телеметрию, выбирают, куда она идёт, или выбирают, какое содержимое она захватывает:

    * `CLAUDE_CODE_ENABLE_TELEMETRY`, плюс улучшенная телеметрия бета-версия пары `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` и `ENABLE_ENHANCED_TELEMETRY_BETA`
    * Селекторы exporter `OTEL_LOGS_EXPORTER`, `OTEL_METRICS_EXPORTER` и `OTEL_TRACES_EXPORTER`
    * Переменные содержимого `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_ASSISTANT_RESPONSES`, `OTEL_LOG_TOOL_CONTENT` и `OTEL_LOG_TOOL_DETAILS`
    * Переменные `OTEL_EXPORTER_OTLP_*`, чьи имена заканчиваются на `_ENDPOINT`, `_HEADERS`, `_PROTOCOL`, `_CERTIFICATE`, `_CLIENT_KEY` или `_INSECURE`, в общих и по-сигнальных формах, таких как `OTEL_EXPORTER_OTLP_ENDPOINT` и `OTEL_EXPORTER_OTLP_METRICS_HEADERS`
    * `OTEL_EXPORTER_PROMETHEUS_HOST` и `OTEL_EXPORTER_PROMETHEUS_PORT`

    Только эти значения всё ещё применяются из параметров проекта и локальных параметров, потому что они отключают что-то: `none` для трёх селекторов exporter и значение off, такое как `0` для `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_CONTENT` и `OTEL_LOG_TOOL_DETAILS`. Такое значение переопределяет ту же переменную в ваших пользовательских параметрах, но не ту, которую среда, из которой вы запускаете Claude Code, файл `--settings` или управляемые параметры устанавливают.

    Когда файл параметров проекта или локальных параметров устанавливает переменную в этой группе, локальная интерактивная сессия показывает уведомление при запуске. Запустите `/status` или `claude doctor`, чтобы увидеть, какие из них Claude Code игнорировал и какие отключили телеметрию; оба перечисляют имена, никогда значения. Неинтерактивный запуск с `-p` или сессия Agent SDK не показывает уведомление, поэтому проверьте, что ваш сборщик всё ещё получает данные после обновления. Если нет, установите переменные в ваши пользовательские параметры, управляемые параметры, среду задания или файл, который вы передаёте с помощью `--settings`.

    Игнорирование этой группы в параметрах проекта и локальных параметрах требует Claude Code v2.1.282 или позже.
  * Переменные, которые изменяют способ запуска или синхронизации Claude Code, такие как `CLAUDE_CODE_PROCESS_WRAPPER`, `CLAUDE_CODE_SYNC_SKILLS`, `CLAUDE_CODE_SYNC_PLUGINS`, `CLAUDE_CODE_PLUGIN_CACHE_DIR` и `CLAUDE_CODE_PLUGIN_SEED_DIR`.

  До v2.1.251 параметры проекта и локальные параметры могли также устанавливать переменные в этом списке, которые выбирают, где Claude Code записывает свои файлы или которые экспортируют содержимое сессии, кроме `HOME` и `XDG_CONFIG_HOME`.
* Переменные идентификации, которыми владеют среды хостинга Claude Code, такие как `CLAUDE_CODE_REMOTE` и `CLAUDE_CODE_ACCOUNT_UUID`, игнорируются из каждого файла.
* [`CLAUDE_CODE_MESSAGING_SOCKET` и `CLAUDE_CODE_MESSAGING_TOKEN`](/docs/ru/env-vars#variables), которые Claude Code экспортирует сам, игнорируются из каждого файла. Игнорирование переменной сокета требует Claude Code v2.1.224 или позже, а игнорирование токена требует v2.1.228 или позже.
* [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/ru/sessions#name-the-project-directory-yourself), которую Claude Code читает только из среды запуска, игнорируется из каждого файла; требует v2.1.234 или позже.
* [`CLAUDE_CODE_RESTRICTED`](/docs/ru/env-vars#variables), которую Claude Code читает только из среды запуска, игнорируется из каждого файла.

<h3 id="filecheckpointingenabled">
  `fileCheckpointingEnabled`
</h3>

Позвольте Claude Code создавать снимки файлов перед каждым редактированием, чтобы [`/rewind`](/docs/ru/checkpointing) мог их восстановить. Отображается в `/config` как **Rewind code (checkpoints)**, и переключение его там записывает этот ключ в ваши пользовательские параметры.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code создаёт снимки файлов перед каждым редактированием, чтобы `/rewind` мог их восстановить
  * `false`: Claude Code не создаёт снимки файлов, поэтому `/rewind` не может их восстановить
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_FILE_CHECKPOINTING`](/docs/ru/env-vars) отключает checkpointing на одну сессию; какой бы из двух параметров его ни отключил, другой не может его включить обратно

```json settings.json theme={null}
{
  "fileCheckpointingEnabled": false
}
```

В запуске `-p` или сессии Agent SDK Claude Code игнорирует этот ключ. SDK включает checkpointing с помощью своей опции `enableFileCheckpointing`, и простой запуск `-p` требует `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true`. См. [File checkpointing в Agent SDK](/docs/ru/agent-sdk/file-checkpointing).

<h3 id="plansdirectory">
  `plansDirectory`
</h3>

Выберите, где Claude Code хранит файлы плана, которые он записывает в [режиме плана](/docs/ru/permission-modes#analyze-before-you-edit-with-plan-mode). Claude Code разрешает путь относительно корня проекта и сохраняет значение по умолчанию, когда путь разрешается вне его.

* **Scope**: [`Any file`](#scopes)
* **Type**: строка, путь относительно корня проекта
* **Default**: не установлено, поэтому Claude Code использует `~/.claude/plans`

```json settings.json theme={null}
{
  "plansDirectory": "./plans"
}
```

<h3 id="skilllistingbudgetfraction">
  `skillListingBudgetFraction`
</h3>

Каждый ход Claude видит [список ваших skills](/docs/ru/skills#skill-descriptions-are-cut-short) с их описаниями, и Claude Code ограничивает этот список долей окна контекста. Когда список превышает лимит, Claude Code сохраняет имя каждого skill, но отбрасывает описания наименее используемых skills, поэтому Claude всё ещё может вызывать эти skills, но менее вероятно выбрать один самостоятельно. Увеличьте этот ключ, чтобы сохранить больше описаний видимыми за счёт большего контекста за ход.

* **Scope**: [`Any file`](#scopes)
* **Type**: число, доля больше `0` и не более `1`
* **Default**: `0.01`, что резервирует 1% окна контекста

```json settings.json theme={null}
{
  "skillListingBudgetFraction": 0.02
}
```

Чтобы увидеть, сколько контекста использует список и какие skills вносят наибольший вклад, запустите `/doctor`.

<h3 id="skilllistingmaxdescchars">
  `skillListingMaxDescChars`
</h3>

Каждый ход Claude видит [список ваших skills](/docs/ru/skills#skill-descriptions-are-cut-short), который показывает текст `description` и `when_to_use` каждого skill. Этот ключ ограничивает, сколько символов этого текста Claude Code показывает для каждого skill; более длинный текст обрезается на лимите.

* **Scope**: [`Any file`](#scopes)
* **Type**: количество символов, положительное целое число
* **Default**: `1536`

```json settings.json theme={null}
{
  "skillListingMaxDescChars": 2048
}
```

Увеличьте его, чтобы сохранить длинные описания нетронутыми за счёт большего контекста за ход; уменьшите его, чтобы вместить больше skills под [`skillListingBudgetFraction`](#skilllistingbudgetfraction).

<h3 id="taskoutputmaxchars">
  `taskOutputMaxChars`
</h3>

<Warning>
  Удалено в v2.1.277 вместе с инструментом `TaskOutput`, который он определял. Установка этого параметра не влияет на текущие версии. Claude читает [файл вывода](/docs/ru/tools-reference#background-commands) фонового задания с помощью `Read` вместо этого.
</Warning>

До v2.1.276 вы устанавливали этот ключ на количество символов вывода [фонового задания](/docs/ru/tools-reference#background-commands), которое Claude получал встроенным образом, когда читал задание с помощью инструмента `TaskOutput`.

<h2 id="interface-and-terminal">
  Интерфейс и терминал
</h2>

Измените внешний вид и поведение Claude Code в вашем терминале: тему, режим редактора, строку состояния, спиннер, уведомления внутри сеанса и доступность. См. [Конфигурация терминала](/docs/ru/terminal-config).

<h3 id="askuserquestiontimeout">
  `askUserQuestionTimeout`
</h3>

Позвольте диалогу [`AskUserQuestion`](/docs/ru/tools-reference) без ответа автоматически продолжить работу после периода простоя, отправив любые уже выбранные вами параметры. Установите это, когда вы отходите и хотите, чтобы Claude продолжал работу без вас. По умолчанию вопросы ждут, пока вы на них ответите. Требуется Claude Code v2.1.200 или позже.

* **Область действия**: [`Пользователь или управляемый`](#scopes)
* **Тип**: строка, одна из `"60s"`, `"5m"`, `"10m"` или `"never"`
* **По умолчанию**: `"never"`
* **Переопределения для каждого сеанса**: [`CLAUDE_AFK_TIMEOUT_MS`](/docs/ru/env-vars) имеет приоритет над этим ключом для одного сеанса

```json settings.json theme={null}
{
  "askUserQuestionTimeout": "5m"
}
```

Появляется в `/config` как **Question auto-continue timeout**, который записывает этот ключ в пользовательские параметры; Claude Code скрывает строку, пока управляемые параметры или флаг `--settings` устанавливают ключ. Требуется Claude Code v2.1.200 или позже.

<h3 id="autocontinueatusagelimit">
  `autoContinueAtUsageLimit`
</h3>

После того как лимит использования claude.ai остановит ваш сеанс, ждите в открытом сеансе и продолжайте задачу автоматически после сброса. См. [Отключение автоматического продолжения](/docs/ru/interactive-mode#turn-automatic-continue-off). Требуется Claude Code v2.1.234 или позже.

* **Область действия**: [`Пользователь или управляемый`](#scopes). Читается из пользовательских параметров, `--settings` и управляемых параметров только. Когда ни один из них не устанавливает ключ, файл параметров проекта или локальный файл, который его устанавливает, отключает функцию, а не игнорируется.
* **Тип**: Boolean
  * `true`: после того как лимит использования claude.ai остановит ваш сеанс, Claude Code ждет в открытом сеансе и продолжает задачу автоматически после сброса
  * `false`: Claude Code не запускает ожидание самостоятельно. Вы все еще можете [начать ожидание самостоятельно](/docs/ru/interactive-mode#start-a-wait-yourself) из меню параметров лимита использования
* **По умолчанию**: `true`

```json settings.json theme={null}
{
  "autoContinueAtUsageLimit": false
}
```

Появляется в `/config` как **Continue automatically at usage limit**, который записывает этот ключ в пользовательские параметры; Claude Code скрывает строку, пока управляемые параметры или флаг `--settings` устанавливают ключ.

<h3 id="autoscrollenabled">
  `autoScrollEnabled`
</h3>

Следите за новым выводом в конце разговора в [полноэкранном режиме](/docs/ru/fullscreen). Отключите это, чтобы остаться там, где вы прокручивали, пока Claude продолжает работать; подсказки разрешений все еще прокручиваются в поле зрения.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: Boolean
  * `true`: разговор следует за новым выводом в конец
  * `false`: вы остаетесь там, где вы прокручивали, пока Claude продолжает работать; подсказки разрешений все еще появляются ниже стенограммы
* **По умолчанию**: `true`

```json settings.json theme={null}
{
  "autoScrollEnabled": false
}
```

Появляется в `/config` как **Auto-scroll**, когда полноэкранный режим включен, который записывает этот ключ в пользовательские параметры.

<h3 id="axscreenreader">
  `axScreenReader`
</h3>

Отрисовка вывода, удобного для программ чтения с экрана: плоский текст без декоративных границ или анимаций. Режим программы чтения с экрана использует классический рендерер, поэтому параметр `tui` не имеет эффекта, пока он активен; прикрепленные [фоновые сеансы](/docs/ru/agent-view) все еще отрисовываются в полноэкранном режиме.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: Boolean
  * `true`: Claude Code отрисовывает плоский текст без декоративных границ или анимаций, используя классический рендерер
  * `false`: Claude Code отрисовывает нормально
* **По умолчанию**: не установлено, поэтому режим программы чтения с экрана отключен
* **Переопределения для каждого сеанса**: [`--ax-screen-reader`](/docs/ru/cli-reference#cli-flags) имеет приоритет над [`CLAUDE_AX_SCREEN_READER`](/docs/ru/env-vars), и оба имеют приоритет над этим ключом для одного сеанса

```json settings.json theme={null}
{
  "axScreenReader": true
}
```

<h3 id="basheditdiffenabled">
  `bashEditDiffEnabled`
</h3>

Выберите, записывает ли Claude Code, какие файлы изменились в репозитории Git, пока выполняется команда Bash. Когда он их записывает, вы видите их diff в терминале после команды, и ваши [PostToolUse Bash hooks](/docs/ru/hooks#bash) получают список измененных файлов.

Указанный файл не всегда является тем, который команда изменила. Изменение, которое другая программа или другой вызов Bash сделали, пока команда выполнялась, может появиться там тоже.

Установите ключ на `true`, чтобы записывать их в каждом режиме разрешений. Требуется Claude Code v2.1.269 или позже.

* **Область действия**: [`Пользователь или управляемый`](#scopes). `true` считается только из ваших пользовательских параметров, JSON, переданного с `--settings`, или [управляемых параметров](/docs/ru/managed-settings), поэтому `true` в `.claude/settings.json` или `.claude/settings.local.json` репозитория не может включить запись. `false` в любом файле репозитория все еще отключает его, если файл с [более высоким приоритетом](/docs/ru/settings#settings-precedence) не устанавливает `true`.
* **Тип**: Boolean
* **По умолчанию**: не установлено, поэтому Claude Code записывает изменения в режиме auto и режиме `bypassPermissions`, когда он направляет Claude на редактирование файлов через Bash
* **Переопределения для каждого сеанса**: [`CLAUDE_CODE_BASH_EDIT_DIFF`](/docs/ru/env-vars) имеет приоритет над этим ключом для одного сеанса

```json settings.json theme={null}
{
  "bashEditDiffEnabled": true
}
```

<h3 id="companyannouncements">
  `companyAnnouncements`
</h3>

Показывайте объявления вашей организации пользователям при запуске. Когда вы указываете более одного, Claude Code выбирает один случайно для каждого сеанса; при первом запуске человека он показывает первую запись.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: массив строк
* **По умолчанию**: не установлено, поэтому объявление не показывается

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

Выберите, запускается ли Bash или PowerShell команды оболочки, которые вы вводите с префиксом [`!`](/docs/ru/interactive-mode#shell-mode-with-prefix) в поле ввода, те, которые Claude Code запускает напрямую и добавляет в сеанс.

`"powershell"` работает только при включенном [инструменте PowerShell](/docs/ru/tools-reference#powershell-tool). Инструмент включен по умолчанию на Windows без Git Bash и на Windows с Git Bash для учетных записей claude.ai и Console. В сеансах Amazon Bedrock, Google Cloud's Agent Platform и Microsoft Foundry, а также на macOS, Linux и WSL установите `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`, чтобы включить инструмент. Установите эту переменную на `0`, чтобы отключить инструмент.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: строка, одна из:
  * `"bash"`: Claude Code запускает ваши команды `!` в Bash
  * `"powershell"`: Claude Code запускает ваши команды `!` в PowerShell
* **По умолчанию**: `"bash"` или `"powershell"` на Windows, когда Bash недоступен

```json settings.json theme={null}
{
  "defaultShell": "powershell"
}
```

Если оболочка, которую вы назвали, недоступна, Claude Code использует другую: `"powershell"` возвращается к Bash, когда инструмент PowerShell отключен, и `"bash"` возвращается к PowerShell, когда Bash не установлен.

<h3 id="dialogexpiry">
  `dialogExpiry`
</h3>

Установите крайний срок для диалогов, которые Claude Code [пересылает удаленному клиенту](/docs/ru/remote-control#limitations), такому как хост Remote Control или SDK, и для диалога одобрения [удерживаемого кросссеансового сообщения](/docs/ru/cross-session-messaging#control-inbound-messages). На Claude Code v2.1.236 или позже тот же крайний срок ограничивает подсказку согласия на использование кредитов Fable в середине сеанса]\(/ru/model-config#fable-and-usage-credits) в сеансе, в котором может никого не быть у терминала. Когда ответ не поступает до крайнего срока, Claude Code отменяет диалог и продолжает с его действием по умолчанию без действия. Требуется Claude Code v2.1.224 или позже.

* **Область действия**: [`Пользователь или управляемый`](#scopes)
* **Тип**: строка, одна из `"60s"`, `"5m"`, `"10m"` или `"never"`, что отключает крайний срок
* **По умолчанию**: `"5m"`
* **Переопределения для каждого сеанса**: [`CLAUDE_CODE_USER_DIALOG_TIMEOUT_MS`](/docs/ru/env-vars) имеет приоритет над этим ключом для одного сеанса

```json settings.json theme={null}
{
  "dialogExpiry": "10m"
}
```

Подсказки разрешений и вопросы [`AskUserQuestion`](/docs/ru/tools-reference#askuserquestion-tool-behavior) используют свои собственные потоки и не регулируются этим крайним сроком. Появляется в `/config` как **Dialog expiry**, который записывает этот ключ в пользовательские параметры; строка требует Claude Code v2.1.232 или позже, и Claude Code скрывает ее, пока управляемые параметры или флаг `--settings` устанавливают ключ.

<h3 id="editormode">
  `editorMode`
</h3>

Выберите режим сочетаний клавиш для приглашения ввода.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: строка, одна из:
  * `"normal"`: стандартные сочетания клавиш в приглашении ввода
  * `"vim"`: редактирование в стиле vim с режимами NORMAL, INSERT и VISUAL
* **По умолчанию**: `"normal"`

```json settings.json theme={null}
{
  "editorMode": "vim"
}
```

Появляется в `/config` как **Editor mode**, который записывает этот ключ в пользовательские параметры.

<h3 id="emojicompletionenabled">
  `emojiCompletionEnabled`
</h3>

Показывайте предложения эмодзи, когда вы вводите `:` плюс сокращение в приглашение ввода, и замените завершенное сокращение, такое как `:heart:`, его эмодзи. Установите это на `false`, чтобы отключить оба.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: Boolean
  * `true`: Claude Code показывает предложения эмодзи после `:` и заменяет завершенное сокращение его эмодзи
  * `false`: Claude Code ни предлагает эмодзи, ни заменяет сокращения
* **По умолчанию**: `true`

```json settings.json theme={null}
{
  "emojiCompletionEnabled": false
}
```

См. [Сокращения эмодзи](/docs/ru/interactive-mode#emoji-shortcodes). Требуется Claude Code v2.1.217 или позже.

<span id="file-suggestion-settings" />

<h3 id="filesuggestion">
  `fileSuggestion`
</h3>

Запустите вашу собственную команду для предоставления автодополнения пути файла `@` вместо встроенного предложения файла. Встроенное предложение использует быстрый обход файловой системы; большой монорепозиторий может работать лучше с индексированием, специфичным для проекта, таким как предварительно построенный индекс файлов.

* **Область действия**: [`Любой файл`](#scopes). В соответствии с [шлюзами строки состояния и предложения файла](#status-line-and-file-suggestion-gates), Claude Code отключает команду или запускает только управляемое значение и пропускает ваше без предупреждения.
* **Тип**: объект с `type`, всегда `"command"`, и `command`, команда оболочки для запуска
* **По умолчанию**: не установлено, поэтому Claude Code использует встроенное предложение файла

```json settings.json theme={null}
{
  "fileSuggestion": {
    "type": "command",
    "command": "~/.claude/file-suggestion.sh"
  }
}
```

После сохранения введите `@` с последующей частью пути в приглашение: предложения поступают из вывода вашей команды.

<h4 id="command-input-and-output">
  Ввод и вывод команды
</h4>

Claude Code запускает команду с теми же переменными окружения, что и [hooks](/docs/ru/hooks), включая `CLAUDE_PROJECT_DIR`, и прекращает ожидание через пять секунд. Команда получает JSON на stdin с полем `query`, содержащим то, что вы до сих пор вводили:

```json theme={null}
{"query": "src/comp"}
```

Выведите пути файлов, разделенные новой строкой, на stdout. Claude Code показывает максимум 15:

```text theme={null}
src/components/Button.tsx
src/components/Modal.tsx
src/components/Form.tsx
```

Следующий скрипт читает запрос и передает его индексу файлов репозитория:

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

Отрисовка дополнительных кликабельных значков в нижнем колонтитуле под полем ввода, когда регулярное выражение совпадает с выводом хода: результаты инструментов, включая содержимое файлов и полученные страницы, и собственные ответы Claude. Используйте это, чтобы превратить идентификаторы, напечатанные проектными CLI, такие как инструменты проверки и трекеры проблем, в ссылки сеанса.

* **Область действия**: [`Пользователь или управляемый`](#scopes)
* **Тип**: массив объектов, каждый с `type`, установленным на `"regex"`, регулярное выражение `pattern`, шаблон `url` и необязательный `label`; заполнители `{name}` в `url` и `label` заполняются из именованных групп захвата в `pattern`
* **По умолчанию**: не установлено, поэтому значки не отрисовываются

Этот пример совпадает с ключами проблем, такими как `PROJ-1234`, и строит каждую ссылку из захваченного ключа:

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

С этой конфигурацией, когда `PROJ-1234` появляется в результате инструмента или в ответе Claude, значок `PROJ-1234` появляется в нижнем колонтитуле, ссылаясь на `https://issues.example.com/browse/PROJ-1234`.

<h4 id="badge-constraints">
  Ограничения значков
</h4>

Каждая запись URL, метка и количество значков ограничены следующим образом:

| Ограничение        | Поведение                                                                                                                                                                                                                 |
| :----------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Источник URL       | Захваченные значения кодируются в URL, и построенный URL должен совместно использовать буквальный источник шаблона. Захват может заполнить сегмент пути или значение запроса, но не может изменить, куда указывает ссылка |
| Длина URL          | Построенные URL длиннее 2048 символов отбрасываются                                                                                                                                                                       |
| Схема URL          | Должна быть `https`, `http` или признанная схема глубокой ссылки редактора или рабочего пространства: `vscode`, `vscode-insiders`, `cursor`, `windsurf`, `zed`, `jetbrains`, `idea`, `slack`, `linear`, `notion`, `figma` |
| Метка              | По умолчанию совпадает с текстом и усекается до 28 столбцов отображения                                                                                                                                                   |
| Количество значков | Максимум 5 значков отрисовываются. Самый старый вытесняется новыми совпадениями и `/clear` удаляет их                                                                                                                     |

Когда ход завершается, Claude Code совпадает с регулярным выражением `pattern` каждой записи с выводом хода в главном потоке, поэтому медленное регулярное выражение блокирует пользовательский интерфейс до его завершения. Вложенные квантификаторы, такие как `(a+)+$`, могут занимать экспоненциально долго против определенных входов и заморозить сеанс, поэтому держите каждый `pattern` линейным и избегайте вложения `+` или `*`.

Значки нижнего колонтитула отрисовываются рядом с [пользовательской строкой состояния](/docs/ru/statusline), когда она настроена; ни один не заменяет другой. Используйте строку состояния для строки, управляемой скриптом, которая вычисляет свое собственное содержимое из данных сеанса, и значки нижнего колонтитула, чтобы превратить идентификаторы из разговора в ссылки без скрипта.

<h3 id="keybindingflavor">
  `keybindingFlavor`
</h3>

<Warning>
  Устарело с v2.1.261 и не имеет эффекта. Клавиши редактирования слов приглашения всегда [следуют соглашениям readline](/docs/ru/interactive-mode#make-ctrl-w-delete-back-to-whitespace), как в Bash. Claude Code все еще принимает `keybindingFlavor`, поэтому файл параметров, который его устанавливает, остается действительным.
</Warning>

В v2.1.238 через v2.1.260 установка его на `"readline"` заставляла `Ctrl+W` удалять назад к предыдущему пробелу вместо только предыдущего слова.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: строка, `"classic"` или `"readline"`
* **По умолчанию**: не установлено

<h3 id="prefersreducedmotion">
  `prefersReducedMotion`
</h3>

Уменьшите или отключите анимацию интерфейса, такую как спиннер, мерцание и эффекты вспышки. Появляется в `/config` как **Reduce motion**.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: Boolean
  * `true`: Claude Code уменьшает или отключает анимацию интерфейса, такую как спиннер, мерцание и эффекты вспышки
  * `false`: то же самое, что не установлено; Claude Code показывает свои анимации
* **По умолчанию**: `false`

```json settings.json theme={null}
{
  "prefersReducedMotion": true
}
```

<h3 id="promptsuggestionenabled">
  `promptSuggestionEnabled`
</h3>

Показывайте или скрывайте [предложения приглашения](/docs/ru/interactive-mode#prompt-suggestions), затемненные предсказания, которые появляются в вашем приглашении ввода. Установите это на `false` или отключите **Prompt suggestions** в `/config`, чтобы скрыть их.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: Boolean
  * `true`: вы видите предложения приглашения в вашем приглашении ввода
  * `false`: Claude Code скрывает предложения приглашения
* **По умолчанию**: `true`
* **Переопределения для каждого сеанса**: [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/ru/env-vars) имеет приоритет над этим ключом для одного сеанса

```json settings.json theme={null}
{
  "promptSuggestionEnabled": false
}
```

Предложения приглашения требуют учетной записи claude.ai или Console с включенной телеметрией. На Amazon Bedrock, Google Cloud's Agent Platform и Microsoft Foundry, или с отключенной телеметрией, такой как [`DISABLE_TELEMETRY`](/docs/ru/env-vars), этот ключ не имеет эффекта и только `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=1` включает их.

<h3 id="respectgitignore">
  `respectGitignore`
</h3>

Контролируйте, оставляет ли средство выбора файлов `@` файлы, которые совпадают с шаблонами `.gitignore`. Появляется в `/config` как **Respect .gitignore in file picker**.

* **Область действия**: [`Любой файл`](#scopes). Когда ни один файл параметров его не устанавливает, Claude Code возвращается к `respectGitignore` в `~/.claude.json`, который переключатель `/config` записывает.
* **Тип**: Boolean
  * `true`: средство выбора файлов `@` оставляет файлы, которые совпадают с шаблонами `.gitignore`
  * `false`: средство выбора файлов `@` включает файлы, которые совпадают с шаблонами `.gitignore`
* **По умолчанию**: `true`

```json settings.json theme={null}
{
  "respectGitignore": false
}
```

<h3 id="respondtobashcommands">
  `respondToBashCommands`
</h3>

Выберите, отвечает ли Claude после того, как вы запустите команду оболочки с префиксом [`!`](/docs/ru/interactive-mode#shell-mode-with-prefix) в поле ввода. По умолчанию Claude Code добавляет вывод команды в разговор и Claude отвечает на него. Установите этот ключ на `false`, чтобы добавить вывод в контекст без ответа, чтобы вы могли запустить несколько команд и спросить о них вместе.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: Boolean
  * `true`: Claude Code добавляет вывод команды в разговор и Claude отвечает на него
  * `false`: Claude Code добавляет вывод в контекст без ответа
* **По умолчанию**: `true`

```json settings.json theme={null}
{
  "respondToBashCommands": false
}
```

См. [Режим оболочки с префиксом `!`](/docs/ru/interactive-mode#shell-mode-with-prefix).

<h3 id="showclearcontextonplanaccept">
  `showClearContextOnPlanAccept`
</h3>

Когда Claude завершает план в [режиме плана](/docs/ru/permission-modes#review-and-approve-a-plan), он показывает меню одобрения. Планирование может использовать много контекста, поэтому этот ключ добавляет первый вариант в это меню, **Yes, clear context and …**, который одобряет план, очищает контекст разговора и начинает реализацию только из плана. Остальная часть метки называет режим разрешения, в котором продолжается сеанс, и показывает, сколько вашего контекста использовало планирование.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: Boolean
  * `true`: меню одобрения плана получает первый вариант, **Yes, clear context and …**, который одобряет план и очищает контекст разговора
  * `false`: меню одобрения плана не показывает вариант очистки контекста
* **По умолчанию**: `false`

```json settings.json theme={null}
{
  "showClearContextOnPlanAccept": true
}
```

<h3 id="showturnduration">
  `showTurnDuration`
</h3>

Показывайте или скрывайте сообщение о продолжительности хода после каждого ответа, такое как "Cooked for 1m 6s · done 6:05 PM". Часы после "done" показывают, когда ход завершился; [`timeFormat`](#timeformat) и [`timeZone`](#timezone) контролируют его формат и зону. Появляется в `/config` как **Show turn duration**.

* **Область действия**: [`Любой файл`](#scopes). Значение в `~/.claude.json` из более старой версии применяется, когда ни один файл параметров его не устанавливает.
* **Тип**: Boolean
  * `true`: вы видите сообщение о продолжительности хода после каждого ответа
  * `false`: Claude Code скрывает сообщение о продолжительности хода
* **По умолчанию**: `true`

```json settings.json theme={null}
{
  "showTurnDuration": false
}
```

<h3 id="spellcheck">
  `spellcheck`
</h3>

Подчеркивайте неправильно написанные слова в приглашении ввода по мере ввода, используя установленную вами программу проверки орфографии. Claude Code проверяет только текст в поле ввода. [Проверка орфографии по мере ввода](/docs/ru/interactive-mode#check-spelling-as-you-type) охватывает установку aspell, hunspell или ispell и то, что проверяет программа проверки. Требуется Claude Code v2.1.235 или позже.

* **Область действия**: [`Пользователь или управляемый`](#scopes). Блок из самого высокого уровня, который его устанавливает, применяется целиком.
* **Тип**: объект с `enabled` (Boolean), `checker` (`"aspell"`, `"hunspell"`, `"ispell"` или `"auto"`), `language` (строка, передаваемая программе проверки как имя ее словаря) и `color` (строка, имя цвета терминала, `#rrggbb`, `rgb(r,g,b)`, `ansi256(n)` или `ansi:<name>`)
* **По умолчанию**: не установлено, поэтому проверка орфографии отключена; `checker` по умолчанию `"auto"`, первый из трех найденных на `PATH`; `language` по умолчанию собственный словарь программы проверки; `color` по умолчанию цвет ошибки темы

```json settings.json theme={null}
{
  "spellcheck": { "enabled": true, "language": "en_GB" }
}
```

<h3 id="spinnertipsenabled">
  `spinnerTipsEnabled`
</h3>

Пока Claude работает, строка спиннера вращается через короткие советы о функциях Claude Code, такие как "Use Plan Mode to prepare for a complex request before making changes. Press Shift+Tab twice to enable." Установите этот ключ на `false`, чтобы скрыть их. Появляется в `/config` как **Show tips**.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: Boolean
  * `true`: вы видите советы в спиннере, пока Claude работает
  * `false`: Claude Code скрывает советы спиннера
* **По умолчанию**: `true`

```json settings.json theme={null}
{
  "spinnerTipsEnabled": false
}
```

<h3 id="spinnertipsoverride">
  `spinnerTipsOverride`
</h3>

Добавьте свои собственные советы к [советам спиннера](#spinnertipsenabled), которые Claude Code показывает, пока Claude работает, или замените встроенные советы своими. Claude Code помещает ваши советы в ту же ротацию, что и встроенные: он выбирает совет, который дольше всего не показывался, пропускает советы, все еще находящиеся в своем периоде охлаждения, и разрывает связи по приоритету.

Если вы установите [`spinnerTipsEnabled`](#spinnertipsenabled) на `false`, Claude Code скрывает все советы, включая ваши.

* **Область действия**: [`Любой файл`](#scopes). Claude Code соблюдает объекты советов, `tipsFile`, `label` и `excludeDefault` из пользовательских параметров, флага `--settings` и управляемых параметров; из параметров проекта и локальных параметров он читает только простые строковые советы.
* **Тип**: объект с полями `tips`, `tipsFile`, `label` и `excludeDefault`, каждое необязательное
* **По умолчанию**: не установлено, поэтому Claude Code показывает только встроенные советы

Объекты советов, `tipsFile`, `label` и правило в строке Область действия, что параметры проекта и локальные параметры вносят только простые строки, требуют Claude Code v2.1.247 или позже. На более ранних версиях `excludeDefault` файла проекта или локального файла также применяется.

Каждая запись `tips` — это простая строка или объект с этими полями:

| Поле               | Обязательно | Описание                                                                                                                                                                                                                      |
| :----------------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`               | Да          | До 64 букв, цифр, `.`, `_` или `-`. Claude Code ключает историю показа совета на нем, поэтому период охлаждения совета сохраняется при переупорядочении списка. Из двух записей с одинаковым id Claude Code использует первую |
| `text`             | Да          | Совет, одна строка до 500 символов. Claude Code удаляет ANSI-экранирования и управляющие символы и сворачивает пробелы                                                                                                        |
| `cooldownSessions` | Нет         | Сеансы Claude Code ждет перед повторным показом совета, `0` до `1000`, по умолчанию `0`                                                                                                                                       |
| `priority`         | Нет         | Порядок среди советов, которые не показывались одинаково долго, выше первым, `-10` до `10`, по умолчанию `0`                                                                                                                  |

Claude Code читает простую строку как совет с этими значениями по умолчанию и id на основе позиции, поэтому его история показа сбрасывается при переупорядочении списка. Дайте совету `id`, чтобы сохранить его историю при редактировании.

Claude Code читает максимум 200 советов в `tips` и `tipsFile` и отбрасывает неправильную запись с предупреждением отладки вместо отклонения файла параметров.

Используйте оставшиеся поля для именования файла советов, установки префикса и скрытия встроенных советов:

* `tipsFile`: абсолютный или `~/` путь к локальному JSON-файлу, содержащему массив одинаковых записей, или объект с массивом `tips`, до 256 КБ. Claude Code читает файл один раз за процесс, поэтому он загружает ваши правки при следующем запуске. Вы не можете установить его через [управляемые параметры сервера](/docs/ru/server-managed-settings); развертывайте встроенные `tips` там или развертывайте путь в локальном `managed-settings.json`.
* `label`: префикс Claude Code показывает перед советами из пользовательских параметров, `--settings` и управляемых параметров, до 40 символов. По умолчанию `Tip`, тот же префикс, что и встроенные советы, и советы из параметров проекта и локальных параметров всегда его используют.
* `excludeDefault`: установите это на `true`, чтобы скрыть встроенные советы и показать только ваши. Когда Claude Code не может загрузить какой-либо из ваших советов, например потому что `tipsFile` не существует или каждая запись неправильна, он сохраняет встроенную ротацию вместо пустого спиннера.

Когда более одного файла параметров устанавливает ключ, Claude Code показывает советы из всех них и берет `tipsFile`, `label` и `excludeDefault` из того, который из управляемых параметров, флага `--settings` и пользовательских параметров является наивысшим приоритетом, который устанавливает каждый.

Этот пример, в ваших пользовательских параметрах, добавляет простой строковый совет и объектный совет в ротацию под префиксом `Acme tip`:

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

Каждое поле в примере изменяет одно в том, как Claude Code показывает советы:

* `label`: Claude Code показывает оба совета как `Acme tip: ...` вместо `Tip: ...`.
* Простая строка: Claude Code дает ей значения по умолчанию, поэтому она может появиться снова в очень следующем сеансе.
* `id`: Claude Code ключает историю показа второго совета на `gateway-errors`, поэтому его период охлаждения все еще применяется после добавления или переупорядочения советов.
* `cooldownSessions`: после того как Claude Code показывает совет `gateway-errors`, он не показывает этот совет снова до пяти сеансов спустя.
* `priority`: когда совет `gateway-errors` и другой совет не показывались одинаково долго, например когда ни один еще не был показан, Claude Code показывает `gateway-errors` первым. Простая строка имеет приоритет по умолчанию, `0`.

Пока Claude работает, Claude Code показывает ваши советы в спиннере с вашим префиксом, такие как `Acme tip: Run /review before opening a PR`.

<h3 id="spinnerverbs">
  `spinnerVerbs`
</h3>

Пока ход выполняется, спиннер показывает вращающийся глагол, такой как "Accomplishing", "Architecting" или "Baking". Используйте этот ключ, чтобы добавить свои собственные глаголы в эту ротацию или заменить встроенный список своим.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: объект с массивом `verbs` строк и `mode`, одна из:
  * `"append"`: Claude Code добавляет ваши глаголы к встроенному набору
  * `"replace"`: Claude Code показывает только ваши глаголы
* **По умолчанию**: не установлено, поэтому Claude Code использует встроенные глаголы

Этот пример добавляет два глагола к встроенному набору:

```json settings.json theme={null}
{
  "spinnerVerbs": {
    "mode": "append",
    "verbs": ["Pondering", "Crafting"]
  }
}
```

В режиме `"replace"` с пустым массивом `verbs` Claude Code сохраняет встроенные глаголы.

<h3 id="statusline">
  `statusLine`
</h3>

Запустите вашу собственную команду для отрисовки [строки состояния](/docs/ru/statusline) под приглашением с контекстом, таким как модель, стоимость или ветка git. Необязательные поля регулируют интервал, добавляют периодические переустановки и скрывают встроенный индикатор режима vim, когда ваш скрипт отрисовывает `vim.mode` сам.

* **Область действия**: [`Любой файл`](#scopes). Когда [`allowManagedHooksOnly`](#allowmanagedhooksonly) включен или [`disableAllHooks`](#disableallhooks) установлен вне управляемых параметров, запускается только значение управляемых параметров.
* **Тип**: объект с `type`, установленным на `"command"` и строкой `command`, плюс необязательный `padding` как количество символов, `refreshInterval` как количество секунд, минимум `1`, и `hideVimModeIndicator` как Boolean
* **По умолчанию**: не установлено, поэтому нет строки состояния

Этот пример выводит имя модели и использование контекста и добавляет два символа горизонтального интервала:

```json settings.json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
    "padding": 2
  }
}
```

Пример требует установленного [`jq`](https://jqlang.org/) и запускается в оболочке. Для эквивалентов PowerShell и Git Bash см. [Конфигурация Windows](/docs/ru/statusline#windows-configuration); для полной настройки см. [Ручная настройка строки состояния](/docs/ru/statusline#manually-configure-a-status-line).

<h3 id="subagentstatusline">
  `subagentStatusLine`
</h3>

Когда Claude запускает [подагентов](/docs/ru/sub-agents), Claude Code перечисляет их в отображении задач под приглашением, одна строка на подагента, показывающая `name · description · token count`. Этот ключ позволяет вам запустить вашу собственную команду для переписания этих строк, например для показа использования контекста каждого подагента в процентах. При каждом обновлении Claude Code отправляет видимые строки как один JSON-объект на stdin с массивом `tasks`, несущим `id`, `name`, `status`, `model`, `tokenCount` каждого подагента и многое другое, и заменяет строку для каждого `id`, который вы пишете обратно как строку `{"id", "content"}`. Строки, которые вы не пишете обратно, сохраняют отрисовку по умолчанию.

* **Область действия**: [`Любой файл`](#scopes). Когда [`allowManagedHooksOnly`](#allowmanagedhooksonly) включен или [`disableAllHooks`](#disableallhooks) установлен вне управляемых параметров, запускается только значение управляемых параметров.
* **Тип**: объект с `type`, установленным на `"command"` и строкой `command`
* **По умолчанию**: не установлено, поэтому Claude Code отрисовывает строки по умолчанию

```json settings.json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "jq -c '.tasks[] | {id, content: \"\\(.name): \\(.tokenCount) tokens\"}'"
  }
}
```

См. [Строки состояния подагента](/docs/ru/statusline#subagent-status-lines).

<h3 id="syntaxhighlightingdisabled">
  `syntaxHighlightingDisabled`
</h3>

Claude Code раскрашивает код по языку в дифах, блоках кода и предпросмотрах файлов, которые он показывает в терминале, со своим встроенным подсветчиком; никакой плагин или языковой сервер не задействован. Установите этот ключ на `true`, чтобы показать их как простой текст вместо этого, например если цвета конфликтуют с вашей темой терминала или замедляют программу чтения с экрана.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: Boolean
  * `true`: Claude Code отключает подсветку синтаксиса в дифах, блоках кода и предпросмотрах файлов
  * `false`: Claude Code подсвечивает синтаксис
* **По умолчанию**: `false`

```json settings.json theme={null}
{
  "syntaxHighlightingDisabled": true
}
```

<h3 id="terminalprogressbarenabled">
  `terminalProgressBarEnabled`
</h3>

Некоторые терминалы могут показать индикатор прогресса на вкладке или в панели задач для программы, работающей в них. Пока Claude работает, Claude Code сообщает состояние выполнения терминалу, поэтому вы можете видеть с другой вкладки или окна, занят ли сеанс. Индикатор остается видимым после завершения хода, пока [фоновые подагенты](/docs/ru/sub-agents#run-subagents-in-foreground-or-background) или [динамические рабочие процессы](/docs/ru/workflows) все еще работают, и очищается после того, как сеанс неактивен.

Claude Code сообщает это только в терминалах, которые поддерживают индикатор: ConEmu, Ghostty 1.2.0 или позже и iTerm2 3.6.6 или позже. Установите этот ключ на `false`, чтобы остановить Claude Code от сообщения об этом. Появляется в `/config` как **Terminal progress bar**.

* **Область действия**: [`Любой файл`](#scopes). Значение в `~/.claude.json` из более старой версии применяется, когда ни один файл параметров его не устанавливает.
* **Тип**: Boolean
  * `true`: вы видите полосу прогресса терминала в терминалах, которые ее поддерживают
  * `false`: Claude Code скрывает полосу прогресса терминала
* **По умолчанию**: `true`

```json settings.json theme={null}
{
  "terminalProgressBarEnabled": false
}
```

<h3 id="terminaltitlefromrename">
  `terminalTitleFromRename`
</h3>

Claude Code устанавливает название вкладки вашего терминала. По умолчанию он использует название, которое он генерирует из разговора, и как только вы дадите сеансу [имя](/docs/ru/sessions#name-your-sessions) с `/rename` или `--name`, вкладка показывает это имя вместо этого. Установите этот ключ на `false`, чтобы сохранить сгенерированное название на вкладке даже после того, как вы назовете сеанс. Само имя все еще применяется, поэтому `/resume <name>` и средство выбора сеанса его находят.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: Boolean
  * `true`: название вкладки терминала показывает имя сеанса, которое вы установили
  * `false`: вкладка сохраняет название Claude Code генерирует из вашего разговора
* **По умолчанию**: `true`

```json settings.json theme={null}
{
  "terminalTitleFromRename": false
}
```

Чтобы остановить Claude Code от обновления названия терминала вообще, установите [`CLAUDE_CODE_DISABLE_TERMINAL_TITLE`](/docs/ru/env-vars) на `1` вместо этого.

<h3 id="theme">
  `theme`
</h3>

Выберите цветовую тему для интерфейса. Появляется в `/config` как **Theme**.

* **Область действия**: [`Любой файл`](#scopes). Значение в `~/.claude.json` из более старой версии применяется, когда ни один файл параметров его не устанавливает.
* **Тип**: строка, одна из:
  * `"auto"`: совпадает с светлым или темным фоном вашего терминала
  * `"dark"`: темная тема
  * `"light"`: светлая тема
  * `"dark-daltonized"`: темная тема с удобными для дальтоников цветами
  * `"light-daltonized"`: светлая тема с удобными для дальтоников цветами
  * `"dark-ansi"`: темная тема, используя только палитру цветов ANSI вашего терминала
  * `"light-ansi"`: светлая тема, используя только палитру цветов ANSI вашего терминала
  * `"custom:<slug>"` или `"custom:<plugin-name>:<slug>"`: пользовательская тема из `~/.claude/themes/` или плагина
* **По умолчанию**: `"dark"`

```json settings.json theme={null}
{
  "theme": "light-daltonized"
}
```

См. [Создание пользовательской темы](/docs/ru/terminal-config#create-a-custom-theme).

<h3 id="timeformat">
  `timeFormat`
</h3>

Выберите, как Claude Code записывает времена, которые он показывает в интерфейсе, такие как `done 6:05 PM` в конце каждого сообщения о продолжительности хода и временные метки в [средстве просмотра стенограммы](/docs/ru/interactive-mode#transcript-viewer). Чтобы выбрать предустановку, запустите `/config` и установите **Time format**. Требуется Claude Code v2.1.257 или позже.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: строка, одна из:
  * `"auto"`: то же самое, что не установлено; каждое время сохраняет свой встроенный формат, который следует вашей локали в сообщении о продолжительности хода
  * `"12-hour"`: 12-часовые часы
  * `"24-hour"`: 24-часовые часы
  * `"24-hour-utc"`: 24-часовые часы в UTC с `Z` после минут, такие как `18:05Z`; Claude Code игнорирует [`timeZone`](#timezone) для этой предустановки
  * Шаблон strftime, такой как `"%H:%M"`: Claude Code записывает каждое время с шаблоном. Любое значение, содержащее `%`, является шаблоном, и любое другое значение вне предустановок считается `"auto"`
* **По умолчанию**: `"auto"`

```json settings.json theme={null}
{
  "timeFormat": "24-hour"
}
```

`/config` предлагает только предустановки, поэтому для использования шаблона strftime добавьте ключ в файл параметров. Этот пример показывает каждое время как двузначные 24-часовые часы:

```json settings.json theme={null}
{
  "timeFormat": "%H:%M"
}
```

Сообщение о продолжительности хода и средство просмотра стенограммы затем показывают времена, такие как `18:05`. В средстве просмотра стенограммы шаблон — это вся временная метка, поэтому добавьте директивы даты, когда вы хотите дату там. Этот пример ставит дату перед часами:

```json settings.json theme={null}
{
  "timeFormat": "%Y-%m-%d %H:%M"
}
```

Те же поверхности затем показывают времена, такие как `2026-09-01 18:05`.

<h3 id="timezone">
  `timeZone`
</h3>

Показывайте времена в интерфейсе в часовом поясе, отличном от вашего системного. Установите это на [имя часового пояса IANA](https://www.iana.org/time-zones), такое как `"UTC"` или `"Europe/Dublin"`. Времена, которые [`timeFormat`](#timeformat) контролирует, затем показываются в этом поясе. Если `timeFormat` — это `"24-hour-utc"`, времена остаются в UTC и Claude Code игнорирует этот ключ. `/config` не имеет строки для этого ключа, поэтому установите его в файл параметров. Требуется Claude Code v2.1.257 или позже.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: строка, имя часового пояса IANA. Когда Claude Code не распознает имя, он использует ваш системный часовой пояс
* **По умолчанию**: не установлено, поэтому времена показываются в вашем системном часовом поясе

```json settings.json theme={null}
{
  "timeZone": "Europe/Dublin"
}
```

<h3 id="tui">
  `tui`
</h3>

Выберите рендерер пользовательского интерфейса терминала. Используйте `"fullscreen"` для безмерцающего [рендерера alt-screen](/docs/ru/fullscreen) с виртуализированной прокруткой или `"default"` для классического рендерера основного экрана. Запуск `/tui fullscreen` или `/tui default` записывает этот ключ для вас.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: строка, одна из:
  * `"default"`: классический рендерер основного экрана
  * `"fullscreen"`: безмерцающий рендерер alt-screen с виртуализированной прокруткой
* **По умолчанию**: не установлено, поэтому Claude Code [выбирает рендерер для вас](/docs/ru/fullscreen#fullscreen-by-default)
* **Переопределения для каждого сеанса**: [`CLAUDE_CODE_NO_FLICKER`](/docs/ru/env-vars) и [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN`](/docs/ru/env-vars) имеют приоритет над этим ключом для одного сеанса: `CLAUDE_CODE_NO_FLICKER=1` включает полноэкранный режим, и `CLAUDE_CODE_NO_FLICKER=0` или `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1` отключает его; когда оба установлены, Claude Code отключает его

```json settings.json theme={null}
{
  "tui": "fullscreen"
}
```

Под tmux `-CC` или через SSH на Windows Claude Code сохраняет классический рендерер, если вы не установите `CLAUDE_CODE_NO_FLICKER=1`. Фоновые сеансы, открытые из [представления агента](/docs/ru/agent-view), всегда используют полноэкранный рендерер независимо от этого параметра.

<h3 id="verbose">
  `verbose`
</h3>

По умолчанию стенограмма сворачивает каждый вызов инструмента в краткое резюме, такое как команда, которую запустил Claude, и количество строк его вывода, и вы нажимаете `Ctrl+O`, чтобы переключить всю стенограмму в развернутое представление, когда вы хотите детали. Установите этот ключ на `true`, чтобы показать полный ввод и вывод каждого вызова инструмента встроенным по мере его выполнения, что полезно, когда вы отлаживаете hook, MCP-сервер или длинную команду оболочки. Появляется в `/config` как **Verbose output**.

* **Область действия**: [`Любой файл`](#scopes). Значение в `~/.claude.json` из более старой версии применяется, когда ни один файл параметров его не устанавливает.
* **Тип**: Boolean
  * `true`: вы видите полный вывод инструмента
  * `false`: вы видите усеченные резюме вывода инструмента
* **По умолчанию**: `false`
* **Переопределения для каждого сеанса**: [`--verbose`](/docs/ru/cli-reference#cli-flags) имеет приоритет над этим ключом для одного сеанса

```json settings.json theme={null}
{
  "verbose": true
}
```

Значение [`viewMode`](#viewmode) или липкий выбор `/focus` переопределяет этот ключ каждый сеанс.

<h3 id="viewmode">
  `viewMode`
</h3>

Установите представление стенограммы, в котором Claude Code начинает: `"default"`, `"verbose"` или `"focus"`. Когда установлено, оно переопределяет как липкий выбор `/focus`, так и параметр [`verbose`](#verbose).

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: строка, одна из:
  * `"default"`: нормальная стенограмма с усеченным выводом инструмента
  * `"verbose"`: стенограмма с полным выводом инструмента
  * `"focus"`: только ваше последнее приглашение, однострочное резюме вызовов инструментов с редактированием diffstats и финальный ответ. Представление Focus требует [полноэкранного рендерера](#tui)
* **По умолчанию**: не установлено, поэтому параметр `verbose` и ваш последний выбор `/focus` применяются
* **Переопределения для каждого сеанса**: [`--verbose`](/docs/ru/cli-reference#cli-flags) имеет приоритет над этим ключом для одного сеанса

```json settings.json theme={null}
{
  "viewMode": "focus"
}
```

<h3 id="viminsertmoderemaps">
  `vimInsertModeRemaps`
</h3>

Сопоставьте двухклавишные последовательности режима INSERT с Escape в [режиме редактора vim](/docs/ru/interactive-mode#vim-editor-mode). Каждый ключ — это ровно два печатаемых символа, введенные последовательно, и `"<Esc>"` — единственная поддерживаемая цель; Claude Code игнорирует другие записи. Требуется Claude Code v2.1.208 или позже.

* **Область действия**: [`Пользователь или управляемый`](#scopes). Репозиторий не может переназначить ваши нажатия клавиш.
* **Тип**: объект, сопоставляющий двухсимвольную последовательность с `"<Esc>"`
* **По умолчанию**: не установлено

```json settings.json theme={null}
{
  "vimInsertModeRemaps": {
    "jj": "<Esc>"
  }
}
```

Не имеет эффекта, если `editorMode` не `"vim"`. См. [Переназначение последовательностей клавиш режима INSERT](/docs/ru/interactive-mode#remap-insert-mode-key-sequences). Требуется Claude Code v2.1.208 или позже.

<h3 id="voice">
  `voice`
</h3>

Включите [голосовую диктовку](/docs/ru/voice-dictation) и выберите, как ведет себя клавиша диктовки. Claude Code записывает этот объект для вас, когда вы запускаете `/voice`.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: объект с `enabled` как Boolean, `autoSubmit` как Boolean, который применяется только в режиме удержания, и `mode`, одна из:
  * `"hold"`: вы удерживаете клавишу диктовки во время разговора и отпускаете ее, чтобы остановить
  * `"tap"`: вы нажимаете клавишу один раз, чтобы начать запись, и снова, чтобы отправить
* **По умолчанию**: не установлено, поэтому диктовка отключена; когда `enabled` — это `true` и `mode` не установлен, Claude Code использует `"hold"`

Этот пример включает диктовку и делает клавишу нажатием один раз, чтобы начать запись, и снова, чтобы отправить:

```json settings.json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

`autoSubmit` отправляет приглашение, когда вы отпускаете клавишу в режиме удержания. Голосовая диктовка требует учетной записи claude.ai.

<h3 id="voiceenabled">
  `voiceEnabled`
</h3>

<Warning>
  Устарело с v2.1.92, когда объект [`voice`](#voice) его заменил. Claude Code все еще его читает, поэтому более старые файлы параметров продолжают работать, но новые конфигурации должны установить `voice.enabled`.
</Warning>

Включите голосовую диктовку с одной формой Boolean, которая предшествует объекту `voice`. Когда оба установлены, применяется `voice.enabled`.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: Boolean
  * `true`: голосовая диктовка включена, когда вы вошли с учетной записью claude.ai и политика вашей организации позволяет голос, если только `voice.enabled` не установлен
  * `false`: голосовая диктовка отключена, если только `voice.enabled` не установлен
* **По умолчанию**: не установлено

```json settings.json theme={null}
{
  "voiceEnabled": true
}
```

<h3 id="wheelscrollaccelerationenabled">
  `wheelScrollAccelerationEnabled`
</h3>

Ускорьте скорость прокрутки колеса мыши во время быстрой прокрутки в [полноэкранном режиме](/docs/ru/fullscreen#mouse-wheel-scrolling). Установите это на `false` для постоянной скорости прокрутки на каждый щелчок колеса.

* **Область действия**: [`Любой файл`](#scopes)
* **Тип**: Boolean
  * `true`: Claude Code ускоряет скорость прокрутки колеса мыши во время быстрой прокрутки
  * `false`: Claude Code прокручивает с постоянной скоростью на каждый щелчок колеса
* **По умолчанию**: `true`

```json settings.json theme={null}
{
  "wheelScrollAccelerationEnabled": false
}
```

<h2 id="git-and-attribution">
  Git и атрибуция
</h2>

Управляйте атрибуцией, которую Claude Code добавляет к коммитам и pull request'ам, и тем, как она работает с git.

<span id="attribution-settings" />

<h3 id="attribution">
  `attribution`
</h3>

Настройте атрибуцию, которую Claude Code добавляет к git коммитам и pull request'ам. По умолчанию коммиты получают [git trailer](https://git-scm.com/docs/git-interpret-trailers) такой как `Co-Authored-By`; описания pull request'ов получают простой текст. Установите каждую часть отдельно с помощью подключей ниже.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: объект со строками `commit` и `pr` и логическим значением `sessionUrl`, или `false` для скрытия всей атрибуции. Значение `false` требует Claude Code v2.1.281 или позже; более ранние версии его отклоняют и [пропускают весь файл пользователя, проекта или локальных параметров](/docs/ru/settings#fix-a-broken-settings-file), который его содержит
* **По умолчанию**: не установлено, поэтому Claude Code использует стандартную атрибуцию, показанную под каждым подключом

Чтобы скрыть всю атрибуцию, установите `attribution` в `false`. В файле параметров, который также читают более ранние версии, установите [`commit`](#attribution-commit) и [`pr`](#attribution-pr) в пустые строки и [`sessionUrl`](#attribution-sessionurl) в `false` вместо этого.

Этот пример заменяет атрибуцию коммита, удаляет атрибуцию pull request'а и убирает ссылку на сеанс:

```json settings.json theme={null}
{
  "attribution": {
    "commit": "Generated with AI\n\nCo-Authored-By: AI <ai@example.com>",
    "pr": "",
    "sessionUrl": false
  }
}
```

После того как вы установите `commit` или `pr`, Claude Code игнорирует устаревший параметр `includeCoAuthoredBy` и использует текст по умолчанию для того из двух, который вы оставили без установки.

Claude Code сообщает Claude, что ваши собственные инструкции об атрибуции, такие как CLAUDE.md или правило [memory](/docs/ru/memory), имеют приоритет над этими строками коммита и PR, если только строка не установлена в [управляемых параметрах](/docs/ru/managed-settings).

<h3 id="includecoauthoredby">
  `includeCoAuthoredBy`
</h3>

<Warning>
  Устарело с версии v2.0.62, когда [`attribution`](#attribution) его заменила. Claude Code по-прежнему его читает, но новые конфигурации должны устанавливать `attribution`.
</Warning>

Используйте [`attribution`](#attribution) вместо этого, который заменяет этот ключ и позволяет вам изменять или скрывать trailer коммита, текст pull request'а и ссылку на сеанс отдельно. Claude Code по-прежнему соблюдает `includeCoAuthoredBy: false` из файлов параметров, которые предшествуют `attribution`, но игнорирует его после того как вы установите `attribution.commit` или `attribution.pr`.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: логическое значение
  * `true`: то же самое, что и не установлено; Claude Code добавляет trailer коммита и текст атрибуции pull request'а
  * `false`: Claude Code опускает как trailer коммита, так и текст атрибуции pull request'а, если только `attribution` не устанавливает `commit` или `pr`, в этом случае применяются правила [`attribution`](#attribution)
* **По умолчанию**: `true`

```json settings.json theme={null}
{
  "includeCoAuthoredBy": false
}
```

Чтобы скрыть всю атрибуцию, см. [`attribution`](#attribution).

<h3 id="includegitinstructions">
  `includeGitInstructions`
</h3>

Claude Code предоставляет Claude две связанные с git части контекста: встроенные инструкции по написанию коммитов и pull request'ов в описании инструмента Bash и снимок состояния git вашего репозитория. Снимок содержит текущую ветку, основную ветку, вывод `git status` и недавние коммиты. Claude Code читает его при запуске разговора.

Установите этот ключ в `false`, чтобы исключить оба, например когда вы используете свои собственные skills рабочего процесса git.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: логическое значение
  * `true`: Claude Code включает встроенные инструкции рабочего процесса коммита и pull request'а и снимок состояния git. Облачные сеансы никогда не включают снимок
  * `false`: Claude Code исключает оба
* **По умолчанию**: `true`
* **Переопределения для каждого сеанса**: [`CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS`](/docs/ru/env-vars) имеет приоритет над этим ключом для одного сеанса

```json settings.json theme={null}
{
  "includeGitInstructions": false
}
```

<h3 id="prurltemplate">
  `prUrlTemplate`
</h3>

Направьте ссылки PR, которые Claude Code отображает, в значке нижнего колонтитула и в сводках результатов инструментов, на внутренний инструмент проверки кода вместо `github.com`. Claude Code подставляет `{host}`, `{owner}`, `{repo}`, `{number}` и `{url}` из URL PR. Ссылки на [GitLab merge request](/docs/ru/interactive-mode#gitlab-merge-requests) на обеих поверхностях сохраняют свой URL GitLab.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: строка, шаблон URL с использованием любого из пяти заполнителей
* **По умолчанию**: не установлено

```json settings.json theme={null}
{
  "prUrlTemplate": "https://reviews.example.com/{owner}/{repo}/pull/{number}"
}
```

Claude Code применяет шаблон только к ссылкам, которые он отображает сам; номер PR, который Claude пишет в сообщении, такой как `#123`, остается таким, как его написал Claude. URL, который не имеет формы `/pull/<number>`, остается без изменений.

<h3 id="attribution-commit">
  `attribution.commit`
</h3>

Установите текст атрибуции, который Claude Code добавляет к git коммитам, включая любые trailers. Установите его в пустую строку, чтобы скрыть атрибуцию коммита.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: строка
* **По умолчанию**: не установлено, поэтому Claude Code добавляет `Co-Authored-By: <name> <noreply@anthropic.com>`. Имя — это активная модель сеанса, такая как `Claude Sonnet 5`.
  * Когда Claude Code распознает модель как модель Claude, но не может подтвердить её точную версию, он пишет только `Claude`.
  * Когда он не может сопоставить ID модели с какой-либо моделью Claude, такой как модель третьей стороны, обслуживаемая через пользовательский [`ANTHROPIC_BASE_URL`](/docs/ru/env-vars), он пишет `Claude Code`.

Этот пример заменяет trailer по умолчанию пользовательской строкой и пользовательским trailer'ом `Co-Authored-By`:

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

Установите текст атрибуции, который Claude Code добавляет к описаниям pull request'ов. Установите его в пустую строку, чтобы скрыть атрибуцию pull request'а.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: строка
* **По умолчанию**: не установлено, поэтому Claude Code добавляет `🤖 Generated with [Claude Code](https://claude.com/claude-code)`

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

Выберите, добавляет ли Claude Code ссылку на сеанс claude.ai при коммите или открытии pull request'а из [облачного](/docs/ru/claude-code-on-the-web) или [Remote Control](/docs/ru/remote-control) сеанса. Claude Code добавляет ссылку как trailer `Claude-Session` на коммитах и как ссылку в описаниях pull request'ов. Установите его в `false`, чтобы опустить ссылку.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: логическое значение
  * `true`: Claude Code добавляет ссылку на сеанс claude.ai при коммите или открытии pull request'а из облачного или Remote Control сеанса
  * `false`: Claude Code опускает ссылку
* **По умолчанию**: `true`

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
  Hooks и автоматизация
</h2>

Регистрируйте hooks, ограничивайте, какие hooks запускаются, и управляйте рабочими процессами. Для событий hooks и полезных нагрузок см. [справочник hooks](/docs/ru/hooks).

<h3 id="allowedhttphookurls">
  `allowedHttpHookUrls`
</h3>

Ограничьте, какие URL-адреса могут быть целью [HTTP hooks](/docs/ru/hooks#http-hook-fields). Когда вы определяете этот ключ, Claude Code запускает HTTP hook только если его URL соответствует одному из шаблонов и блокирует остальные без их запуска; пустой массив блокирует каждый HTTP hook.

* **Область действия**: [`Any file`](#scopes). Массивы объединяются в файлах параметров.
* **Тип**: массив шаблонов URL-адресов с `*` в качестве подстановочного символа
* **По умолчанию**: не установлено, поэтому разрешены любые URL-адреса

Этот пример разрешает любой URL-адрес под `https://hooks.example.com/` и любой URL-адрес `http://localhost`:

```json settings.json theme={null}
{
  "allowedHttpHookUrls": ["https://hooks.example.com/*", "http://localhost:*"]
}
```

Сопоставление имён хостов не зависит от регистра и рассматривает `hooks.example.com.` с конечной точкой, которая обозначает полное доменное имя, так же, как `hooks.example.com`, что соответствует тому, как это делает DNS. Список разрешений применяется к hooks из каждого источника, включая управляемые параметры.

<h3 id="allowmanagedhooksonly">
  `allowManagedHooksOnly`
</h3>

Ограничьте выполнение hooks только hooks, которые развёртывает ваша организация.

* **Область действия**: [`Managed`](#scopes)
* **Тип**: Boolean
  * `true`: запускаются только управляемые hooks, плюс hooks Agent SDK и hooks из плагинов, которые ваши управляемые параметры принудительно включают. См. [Что запускается под `allowManagedHooksOnly`](#what-runs-under-allowmanagedhooksonly)
  * `false`: запускаются hooks из каждой области параметров и плагинов
* **По умолчанию**: не установлено, поэтому запускаются hooks из каждой области параметров и плагинов

```json managed-settings.json theme={null}
{
  "allowManagedHooksOnly": true
}
```

<h4 id="what-runs-under-allowmanagedhooksonly">
  Что запускается под `allowManagedHooksOnly`
</h4>

Когда вы устанавливаете это значение в `true`, Claude Code изменяет, какие hooks и похожие на hooks команды загружаются:

* **Запускаются управляемые и SDK hooks**: hooks из управляемых параметров и hooks, которые [Agent SDK](/docs/ru/agent-sdk/overview) регистрирует в процессе
* **Запускаются hooks принудительно включённых плагинов**: hooks из плагинов, которые ваши управляемые параметры принудительно включают через [`enabledPlugins`](#enabledplugins). Claude Code сопоставляет полный ID `plugin@marketplace`, поэтому плагин с тем же именем из другого marketplace остаётся заблокированным. Это позволяет вам распространять проверенные hooks через организационный marketplace, блокируя всё остальное
* **Всё остальное блокируется**: пользовательские, проектные и локальные hooks, hooks из других плагинов и hooks, объявленные в frontmatter агента
* **Плагины с источником команды отключены**: Claude Code также отключает плагины с [источником `command`](/docs/ru/plugins/marketplace-reference#command-plugin-source), включая плагины, принудительно включённые в управляемых `enabledPlugins`, если вы не установите [`disableCommandPluginSources`](#disablecommandpluginsources) явно в `false`
* **Команды marketplace `headersHelper` блокируются**: Claude Code также блокирует команды marketplace [`headersHelper`](/docs/ru/plugins/host-marketplace#authenticate-archive-downloads), если [`disableCommandPluginSources`](#disablecommandpluginsources) явно не установлен в `false`, за исключением marketplace, который сами управляемые параметры объявляют. Требуется Claude Code v2.1.238 или позже
* **Строка состояния и предложение файла сужаются до управляемых параметров**: Claude Code читает [`statusLine`](/docs/ru/statusline), [`fileSuggestion`](#filesuggestion) и [`subagentStatusLine`](/docs/ru/statusline#subagent-status-lines) только из управляемых параметров, следуя [шлюзам строки состояния и предложения файла](#status-line-and-file-suggestion-gates)

Команда [`/goal`](/docs/ru/goal) не может запуститься, пока этот ключ установлен, потому что она зависит от hooks.

<h3 id="disableallhooks">
  `disableAllHooks`
</h3>

Отключите [hooks](/docs/ru/hooks#disable-or-remove-hooks), любую пользовательскую [строку состояния](/docs/ru/statusline) и любую пользовательскую команду [предложения файла](#filesuggestion). Используйте это, чтобы временно отключить все эти элементы без удаления их из ваших параметров.

* **Область действия**: [`Any file`](#scopes). Только управляемые параметры могут отключать управляемые hooks.
* **Тип**: Boolean
  * `true`: Claude Code отключает hooks, любую пользовательскую строку состояния и любую пользовательскую команду предложения файла
  * `false`: запускаются hooks, строка состояния и команда предложения файла
* **По умолчанию**: не установлено, поэтому запускаются hooks

```json settings.json theme={null}
{
  "disableAllHooks": true
}
```

Охват зависит от того, какой файл содержит ключ:

* **В управляемых параметрах**: Claude Code отключает каждый настроенный hook, включая управляемые, и продолжает запускать hooks, которые [Agent SDK](/docs/ru/agent-sdk/overview) регистрирует в процессе
* **В любом другом файле параметров**: Claude Code отключает пользовательские, проектные, локальные и плагины hooks; управляемые hooks, hooks Agent SDK и hooks из плагинов, принудительно включённых в управляемых [`enabledPlugins`](#enabledplugins), продолжают запускаться

Сохранение запуска hooks Agent SDK, когда управляемые параметры устанавливают этот ключ, требует Claude Code v2.1.242 или позже.

Команда [`/goal`](/docs/ru/goal) не может запуститься, пока hooks отключены, и меню `/hooks` показывает уведомление вместо ваших hooks.

<h4 id="status-line-and-file-suggestion-gates">
  Шлюзы строки состояния и предложения файла
</h4>

Claude Code принимает два решения для `statusLine`, `fileSuggestion` и `subagentStatusLine` в этом порядке:

* **Полностью отключено**: когда управляемые параметры устанавливают `disableAllHooks`, или когда папка не является доверенной в соответствии с тем же [правилом доверия рабочей области, что и hooks в файлах параметров](/docs/ru/permissions#what-runs-before-you-trust-a-folder)
* **Сужено до управляемых параметров**: когда установлен [`allowManagedHooksOnly`](#allowmanagedhooksonly), когда `disableAllHooks` имеет значение `true` вне управляемых параметров после применения [приоритета параметров](/docs/ru/hooks#disable-or-remove-hooks), или когда вы запускаете Claude Code с `--safe-mode`

При сужении Claude Code запускает управляемое значение, если оно развёрнуто. В противном случае он пропускает ваше значение без предупреждения: строка состояния отключена, и автодополнение `@` возвращается к встроенному предложению файла.

<h3 id="disableworkflows">
  `disableWorkflows`
</h3>

Отключите [динамические рабочие процессы](/docs/ru/workflows#turn-workflows-off) и встроенные команды рабочих процессов для всех, кого охватывают ваши параметры, например организацию через управляемые параметры. Чтобы включить или отключить рабочие процессы только для себя, используйте вместо этого [`enableWorkflows`](#enableworkflows), который переключатель **Dynamic workflows** в `/config` записывает в ваши пользовательские параметры.

* **Область действия**: [`Any file`](#scopes)
* **Тип**: Boolean
  * `true`: Claude Code отключает динамические рабочие процессы и встроенные команды рабочих процессов для всех, кого охватывают ваши параметры
  * `false`: то же самое, что и не установлено; включены ли рабочие процессы, затем следует [`enableWorkflows`](#enableworkflows) и значение по умолчанию вашего плана
* **По умолчанию**: `false`
* **Переопределения для каждой сессии**: [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/ru/env-vars) отключает рабочие процессы на одну сессию; какой бы из двух их ни отключил, другой не может их включить обратно

```json settings.json theme={null}
{
  "disableWorkflows": true
}
```

<h3 id="enableworkflows">
  `enableWorkflows`
</h3>

Включите или отключите [динамические рабочие процессы](/docs/ru/workflows) для себя, когда значение по умолчанию вашего плана не соответствует вашим требованиям. Отображается в `/config` как **Dynamic workflows**, который записывает этот ключ в ваши пользовательские параметры и удаляет его снова, когда вы переключаетесь обратно на значение по умолчанию вашего плана. Чтобы отключить рабочие процессы для всех из управляемых параметров, используйте вместо этого [`disableWorkflows`](#disableworkflows).

* **Область действия**: [`Any file`](#scopes)
* **Тип**: Boolean
  * `true`: Claude Code включает динамические рабочие процессы для вас
  * `false`: Claude Code отключает динамические рабочие процессы для вас
* **По умолчанию**: не установлено, поэтому рабочие процессы включены, если вы не находитесь на плане Pro, где они отключены
* **Переопределения для каждой сессии**: [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/ru/env-vars) отключает рабочие процессы на одну сессию, и `true` здесь не может включить их обратно, пока это установлено

```json settings.json theme={null}
{
  "enableWorkflows": true
}
```

[`disableWorkflows`](#disableworkflows) и политика рабочих процессов вашей организации также имеют приоритет: `enableWorkflows: true` не может включить рабочие процессы обратно, пока какой-либо источник их отключает. Claude Code скрывает строку `/config`, пока источник, отличный от ваших пользовательских параметров, устанавливает `enableWorkflows`, или устанавливает `disableWorkflows` в `true`.

<h3 id="hooks">
  `hooks`
</h3>

Запускайте свои собственные команды, подсказки, агентов, HTTP-запросы или MCP tools как [hooks](/docs/ru/hooks) в точках жизненного цикла Claude Code, например перед вызовом tool или при запуске сессии; [справочник hooks](/docs/ru/hooks#hook-events) перечисляет каждое событие, его полезную нагрузку и коды выхода. Каждое событие соответствует списку групп сопоставления, и каждая группа перечисляет обработчики для запуска, когда применяется сопоставление.

* **Область действия**: [`Any file`](#scopes). Hooks объединяются в файлах, а не заменяют друг друга, и hooks из управляемых параметров не могут быть удалены из других файлов.
* **Тип**: объект, ключи которого — [события hook](/docs/ru/hooks#hook-events); каждое значение — это массив групп `{ "matcher", "hooks" }`, чьи записи `hooks` имеют `type` из `"command"`, `"prompt"`, `"agent"`, `"http"` или `"mcp_tool"`
* **По умолчанию**: не установлено, поэтому hooks не запускаются

Этот пример запускает скрипт перед каждым вызовом tool Bash:

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

Для каждого события, шаблона сопоставления и поля обработчика см. [справочник hooks](/docs/ru/hooks#configuration). Чтобы отключить hooks, см. [`disableAllHooks`](#disableallhooks); чтобы ограничить hooks только теми, которые развёртывает ваша организация, см. [`allowManagedHooksOnly`](#allowmanagedhooksonly).

<h3 id="httphookallowedenvvars">
  `httpHookAllowedEnvVars`
</h3>

[HTTP hook](/docs/ru/hooks#http-hook-fields) может поместить значение переменной окружения в заголовок запроса, например заголовок `Authorization: Bearer $HOOK_TOKEN`, но только для переменных, которые hook перечисляет в своём собственном `allowedEnvVars`. Этот ключ устанавливает внешний предел для этого списка для каждого HTTP hook: hook может использовать переменную только если и его собственный `allowedEnvVars`, и этот ключ её называют. Используйте это, чтобы помешать hook читать секрет, который он не должен, даже когда определение hook просит его.

* **Область действия**: [`Any file`](#scopes). Массивы объединяются в файлах параметров.
* **Тип**: массив имён переменных окружения
* **По умолчанию**: не установлено, поэтому применяется список `allowedEnvVars` каждого hook

Этот пример ограничивает интерполяцию заголовков до `MY_TOKEN` и `HOOK_SECRET`:

```json settings.json theme={null}
{
  "httpHookAllowedEnvVars": ["MY_TOKEN", "HOOK_SECRET"]
}
```

Список разрешений применяется к hooks из каждого источника, включая управляемые параметры.

<h3 id="workflowkeywordtriggerenabled">
  `workflowKeywordTriggerEnabled`
</h3>

Выберите, запускает ли ввод ключевого слова `ultracode` в подсказке [динамический рабочий процесс](/docs/ru/workflows#ask-for-a-workflow-in-your-prompt). Установите его в `false`, чтобы вводить слово без запуска.

* **Область действия**: [`Any file`](#scopes). Отображается в `/config` как **Ultracode keyword trigger**.
* **Тип**: Boolean
  * `true`: ввод `ultracode` в подсказку запускает динамический рабочий процесс
  * `false`: вы можете вводить слово без запуска
* **По умолчанию**: `true`

```json settings.json theme={null}
{
  "workflowKeywordTriggerEnabled": false
}
```

Параметр усилия `ultracode`, `/workflows` и сохранённые команды рабочих процессов не затронуты.

<h3 id="workflowsizeguideline">
  `workflowSizeGuideline`
</h3>

Установите [количество агентов, на которое Claude нацелен](/docs/ru/workflows#set-a-size-guideline) в динамических рабочих процессах, которые он пишет. Claude Code отправляет значение Claude как совет, а не как принудительный предел: `"small"` просит менее 5 агентов, `"medium"` менее 10, и `"large"` менее 50. Выберите `"small"`, когда вы хотите ограничить, что тратит рабочий процесс. Требуется Claude Code v2.1.219 или позже.

* **Область действия**: [`Any file`](#scopes). Значение там имеет приоритет над выбором **Dynamic workflow size** в `/config`, который Claude Code хранит в `~/.claude.json`, и Claude Code скрывает эту строку, пока файл параметров устанавливает ключ.
* **Тип**: строка, одна из:
  * `"unrestricted"`: нет рекомендации, поэтому Claude размер рабочего процесса в соответствии с задачей
  * `"small"`: Claude нацелен на менее 5 агентов
  * `"medium"`: Claude нацелен на менее 10 агентов
  * `"large"`: Claude нацелен на менее 50 агентов
* **По умолчанию**: `"medium"`, или `"small"` когда вы вошли в систему на плане Pro с Claude Code v2.1.271 или позже

```json settings.json theme={null}
{
  "workflowSizeGuideline": "small"
}
```

Требуется Claude Code v2.1.219 или позже; на v2.1.202 через v2.1.218 установите рекомендацию в `/config` вместо этого.

<span id="plugin-configuration" />

<span id="manage-plugins" />

<span id="plugin-settings" />

<h2 id="plugins-and-skills">
  Plugins and skills
</h2>

Включайте плагины, регистрируйте маркетплейсы, ограничивайте источники плагинов, которые разрешены в вашей организации, и контролируйте, какие skills загружаются. Для установки и создания плагинов см. [Plugins](/docs/ru/plugins/overview).

<h3 id="disablebundledskills">
  `disableBundledSkills`
</h3>

Отключите [skills](/docs/ru/skills) и рабочие процессы, включённые в Claude Code. Claude Code полностью удаляет встроенные skills и рабочие процессы, в то время как встроенные команды, такие как `/init`, остаются доступными для ввода, но скрыты от модели.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code удаляет встроенные skills и рабочие процессы и скрывает встроенные команды, такие как `/init`, от модели
  * `false`: встроенные skills загружаются
* **Default**: не установлено, поэтому встроенные skills загружаются
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_BUNDLED_SKILLS`](/docs/ru/env-vars) установлено на `1` отключает встроенные skills на одну сессию; какой бы из двух способов их ни отключил, другой не сможет их включить обратно

```json settings.json theme={null}
{
  "disableBundledSkills": true
}
```

Skills из плагинов, `.claude/skills/` и `.claude/commands/` не затрагиваются. `/doctor` остаётся доступным для ввода, как встроенные команды; чтобы скрыть его, установите [`DISABLE_DOCTOR_COMMAND`](/docs/ru/env-vars) вместо этого.

<h3 id="disableskillshellexecution">
  `disableSkillShellExecution`
</h3>

Отключите встроенное выполнение shell для `` !`...` `` и ` ```! ` блоков в [skills](/ru/skills) и пользовательских команд из источников пользователя, проекта, плагина или дополнительного каталога. Claude Code заменяет каждую команду на `[shell command execution disabled by policy]` вместо её выполнения.

* **Scope**: [`Any file`](#scopes). `true` в управляемых параметрах не может быть переопределён на `false` в другом месте.
* **Type**: Boolean
  * `true`: Claude Code заменяет каждую встроенную shell команду на `[shell command execution disabled by policy]` вместо её выполнения
  * `false`: встроенный shell выполняется
* **Default**: не установлено, поэтому встроенный shell выполняется

```json settings.json theme={null}
{
  "disableSkillShellExecution": true
}
```

Встроенные skills и skills, развёрнутые через управляемые параметры, не затрагиваются.

<h3 id="skilloverrides">
  `skillOverrides`
</h3>

Скройте или свёрните [skill](/docs/ru/skills#override-skill-visibility-from-settings) без редактирования его `SKILL.md`. Claude Code применяет значение под именем каждого skill к списку skills, который видит Claude, и к вашему автодополнению `/`.

* **Scope**: [`Any file`](#scopes). Меню `/skills` записывает в `.claude/settings.local.json`.
* **Type**: объект, отображающий имя skill на одно из:
  * `"on"`: Claude видит skill и вы можете ввести `/name`
  * `"name-only"`: Claude видит skill по имени без его описания
  * `"user-invocable-only"`: Claude не видит skill, но вы всё ещё можете ввести `/name`
  * `"off"`: Claude не видит skill и `/name` скрыт из автодополнения
* **Default**: не установлено, поэтому каждый skill имеет значение `"on"`

Этот пример перечисляет `legacy-context` для Claude только по имени и скрывает `deploy` от Claude и из автодополнения `/`:

```json settings.json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Переопределения не применяются к skills плагинов, которыми вы управляете через `/plugin`.

В управляемых параметрах и файлах, переданных с `--settings`, ключ на псевдониме встроенного skill, такой как `checkup` для `/doctor`, также применяется к skill; см. [как ключи псевдонимов объединяются с ключами на собственном имени skill](/docs/ru/skills#override-skill-visibility-from-settings).

<h3 id="syncclaudeaiskills">
  `syncClaudeAiSkills`
</h3>

Отключите загрузку [skills, включённых для вашей учётной записи claude.ai](/docs/ru/skills#how-synced-skills-behave). Claude Code загружает их в `~/.claude/skills/synced/` в [сеансах терминала, где вы входите с вашей учётной записью claude.ai](/docs/ru/skills#where-synced-skills-load), интерактивных или неинтерактивных, а также в сеансах Cowork и облачных сеансах. Установите `false`, чтобы остановить эту загрузку и остановить загрузку skills, которые она уже синхронизировала. Claude Code учитывает только `false`: `true` то же самое, что и не установлено, и не включает синхронизацию там, где она иначе отключена.

* **Scope**: [`User, local, or managed`](#scopes), и файлы, переданные с `--settings`. Репозиторий не может отключить её для вас.
* **Type**: Boolean
  * `false`: Claude Code прекращает загрузку синхронизированных skills и прекращает загрузку тех, которые уже находятся в `~/.claude/skills/synced/`. В пользовательских или управляемых параметрах он также перемещает их в `~/.claude/skills/.trash/`
  * `true`: то же самое, что и не установлено
* **Default**: не установлено, поэтому сеансы, в которых вы входите с вашей учётной записью claude.ai, синхронизируют ваши skills

Этот пример предотвращает загрузку skills учётной записи машиной в любом сеансе:

```json settings.json theme={null}
{
  "syncClaudeAiSkills": false
}
```

<h3 id="syncclaudeaiplugins">
  `syncClaudeAiPlugins`
</h3>

Отключите загрузку [плагинов, включённых для вашей учётной записи claude.ai](/docs/ru/plugins/loading#synced-plugins). Claude Code загружает их в `~/.claude/plugins/synced/` в начале сеансов терминала, где вы входите с вашей учётной записью claude.ai, и в сеансах Cowork, и загружает каждый как `<name>@synced`. Установите `false`, чтобы остановить эту загрузку и остановить загрузку плагинов, которые она уже синхронизировала. Claude Code учитывает только `false`: `true` то же самое, что и не установлено, и не включает синхронизацию там, где она иначе отключена. Требуется Claude Code v2.1.273 или позже.

* **Scope**: [`User, local, or managed`](#scopes), и файлы, переданные с `--settings`. Репозиторий не может отключить её для вас.
* **Type**: Boolean
  * `false`: Claude Code прекращает загрузку синхронизированных плагинов и прекращает загрузку тех, которые уже находятся в `~/.claude/plugins/synced/`. В пользовательских или управляемых параметрах он также перемещает их в `~/.claude/plugins/.trash/`
  * `true`: то же самое, что и не установлено
* **Default**: не установлено, поэтому сеансы, в которых вы входите с вашей учётной записью claude.ai, синхронизируют ваши плагины

Чтобы отключить один синхронизированный плагин вместо всех, установите `"<name>@synced": false` в [`enabledPlugins`](#enabledplugins).

Этот пример предотвращает загрузку плагинов учётной записи машиной в любом сеансе:

```json settings.json theme={null}
{
  "syncClaudeAiPlugins": false
}
```

<h3 id="allowedchannelplugins">
  `allowedChannelPlugins`
</h3>

Выберите, какие [channel](/docs/ru/channels) плагины могут отправлять сообщения в сеансы в вашей организации. Когда вы это установите, Claude Code использует ваш список вместо списка разрешений Anthropic по умолчанию; каждая запись называет плагин и маркетплейс, из которого он поступает.

* **Scope**: [`Managed`](#scopes)
* **Type**: массив объектов, каждый с строками `marketplace` и `plugin`. Запись может быть вместо этого строкой `"plugin@marketplace"`, такой как `"telegram@claude-plugins-official"`, которую Claude Code рассматривает как эквивалентный объект. Форма строки требует Claude Code v2.1.267 или позже; более ранние версии отклоняют всё значение `allowedChannelPlugins`, когда оно содержит одну
* **Default**: не установлено, поэтому Claude Code использует список разрешений Anthropic по умолчанию

Этот пример включает channels и разрешает только плагин Telegram из официального маркетплейса Anthropic:

```json managed-settings.json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" }
  ]
}
```

Пустой массив блокирует каждый channel плагин.

Этот ключ вступает в силу после того, как channels пройдут ворота [`channelsEnabled`](#channelsenabled) для учётной записи: на планах Team и Enterprise, а также на учётных записях Console с управляемыми параметрами, это означает `channelsEnabled: true`. См. [Ограничьте, какие channel плагины могут работать](/docs/ru/channels#restrict-which-channel-plugins-can-run).

<h3 id="blockedmarketplaces">
  `blockedMarketplaces`
</h3>

Заблокируйте источники маркетплейсов плагинов для вашей организации. Claude Code проверяет список блокировки при добавлении маркетплейса и при установке, обновлении, обновлении и автоматическом обновлении плагина, поэтому маркетплейс, который кто-то добавил до того, как вы установили политику, не может быть использован для получения плагинов. Заблокированные источники проверяются перед загрузкой, поэтому они никогда не касаются файловой системы.

Если вы установите этот ключ в [консоли администратора claude.ai](/docs/ru/server-managed-settings), claude.ai также применит его, когда кто-либо в вашей организации добавит маркетплейс из репозитория git на claude.ai, как описано в [Как работают ограничения](/docs/ru/plugins/org#restrict-what-users-can-install).

* **Scope**: [`Managed`](#scopes)
* **Type**: массив объектов источника маркетплейса в тех же формах, что и [`strictKnownMarketplaces`](#allowed-source-types)
* **Default**: не установлено, поэтому ни один маркетплейс не заблокирован

Этот пример блокирует один репозиторий GitHub как источник маркетплейса:

```json managed-settings.json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted/plugins" }
  ]
}
```

Запись `github` может использовать [форму с подстановочным знаком владельца](#owner-wildcards) `"owner/*"` для блокировки каждого репозитория под этим владельцем GitHub, что требует Claude Code v2.1.223 или позже. Добавьте `{ "source": "skills-dir" }`, чтобы остановить загрузку Claude Code [`@skills-dir` плагинов](/docs/ru/plugins/loading#plugins-shared-through-a-repository) из `~/.claude/skills/` без ограничения какого-либо маркетплейса. См. [Управляемые ограничения маркетплейса](/docs/ru/plugins/org#restrict-what-users-can-install).

<h3 id="channelsenabled">
  `channelsEnabled`
</h3>

Разрешите [channels](/docs/ru/channels) для вашей организации. На планах claude.ai Team и Enterprise Claude Code блокирует channels до тех пор, пока вы не установите это на `true`. Для учётных записей [Anthropic Console](/docs/ru/authentication#claude-console-authentication), которые аутентифицируются с помощью ключа API, channels разрешены по умолчанию. Если ваша организация развёртывает управляемые параметры, Claude Code блокирует channels на этих учётных записях до тех пор, пока вы не установите этот ключ на `true`.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code разрешает channels для вашей организации
  * `false`: то же самое, что и не установлено; блокируются ли channels, зависит от вашего плана, как указано в Default
* **Default**: не установлено; channels блокируются на планах Team и Enterprise и на учётных записях Console с управляемыми параметрами, и разрешены на планах Pro и Max и на учётных записях Console без управляемых параметров

```json managed-settings.json theme={null}
{
  "channelsEnabled": true
}
```

Чтобы ограничить, какие плагины могут регистрироваться как channels после их включения, установите [`allowedChannelPlugins`](#allowedchannelplugins). См. [Элементы управления Enterprise](/docs/ru/channels#enterprise-controls).

<h3 id="disablecommandpluginsources">
  `disableCommandPluginSources`
</h3>

Заблокируйте [источник плагина `command`](/docs/ru/plugins/marketplace-reference#command-plugin-source), который устанавливает плагин, запуская объявленную маркетплейсом команду на машине пользователя. Когда вы установите это на `true`, Claude Code никогда не запускает команду, не устанавливает и не обновляет плагины, полученные из command, и прекращает загрузку уже установленных. Установите на `false`, чтобы явно разрешить их. Всякий раз, когда он блокирует command источники, независимо от того, установили ли вы это на `true` или оставили не установленным под [`allowManagedHooksOnly`](#allowmanagedhooksonly), он также блокирует маркетплейс [`headersHelper` команды](/docs/ru/plugins/host-marketplace#authenticate-archive-downloads), за исключением маркетплейса, который сами управляемые параметры объявляют. Требуется Claude Code v2.1.229 или позже, и блокировка `headersHelper` требует v2.1.238 или позже.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code никогда не запускает объявленную маркетплейсом команду, не устанавливает и не обновляет плагины, полученные из command, и прекращает загрузку уже установленных
  * `false`: Claude Code явно разрешает плагины, полученные из command
* **Default**: не установлено, поэтому Claude Code следует [`allowManagedHooksOnly`](#allowmanagedhooksonly): организация, которая ограничивает выполнение hook только управляемыми параметрами, также получает отключённые command источники

```json managed-settings.json theme={null}
{
  "disableCommandPluginSources": true
}
```

Требуется Claude Code v2.1.229 или позже.

<h3 id="pluginsuggestionmarketplaces">
  `pluginSuggestionMarketplaces`
</h3>

Назовите маркетплейсы, чьи плагины могут появляться как контекстные предложения по установке, в советах спиннера и закреплённые в верхней части вкладки `/plugin` **Discover**. Встроенное предложение первой стороны по дизайну фронтенда не затрагивается. Предложения поступают из объявления `relevance` каждого плагина в его записи маркетплейса.

* **Scope**: [`Managed`](#scopes)
* **Type**: массив имён маркетплейсов
* **Default**: не установлено, поэтому никакие объявленные маркетплейсом предложения не появляются

```json managed-settings.json theme={null}
{
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

Имя вступает в силу только когда маркетплейс зарегистрирован на машине и его зарегистрированный источник также объявлен в тех же управляемых параметрах, либо как запись [`extraKnownMarketplaces`](#extraknownmarketplaces) для этого имени, либо как запись [`strictKnownMarketplaces`](#strictknownmarketplaces). Claude Code игнорирует маркетплейс, зарегистрированный из другого источника под разрешённым именем. Официальный маркетплейс освобождён от требования источника: разрешение только его имени достаточно, так как это имя может регистрироваться только из официального источника Anthropic. См. [Предложите плагины по контексту](/docs/ru/plugins/relevance).

<h3 id="plugintrustmessage">
  `pluginTrustMessage`
</h3>

Добавьте собственный текст вашей организации к предупреждению о доверии плагину, которое Claude Code показывает перед установкой, например, чтобы подтвердить, что плагины из вашего внутреннего маркетплейса проверены.

* **Scope**: [`Managed`](#scopes)
* **Type**: string
* **Default**: не установлено, поэтому Claude Code показывает только стандартное предупреждение

```json managed-settings.json theme={null}
{
  "pluginTrustMessage": "All plugins from our marketplace are approved by IT"
}
```

<h3 id="strictknownmarketplaces">
  `strictKnownMarketplaces`
</h3>

Ограничьте, какие источники маркетплейсов плагинов люди в вашей организации могут добавлять и устанавливать плагины из. Claude Code применяет список разрешений при добавлении маркетплейса и при установке, обновлении, обновлении и автоматическом обновлении плагина, перед любой сетевой или файловой операцией, поэтому маркетплейс, который кто-то добавил до того, как вы установили политику, не может быть использован для получения плагинов после того, как его источник больше не совпадает. Заблокированные пользователи видят ошибку, называющую управляемую политику.

Если вы установите этот ключ в [консоли администратора claude.ai](/docs/ru/server-managed-settings), claude.ai также применит его, когда кто-либо в вашей организации добавит маркетплейс из репозитория git на claude.ai, как описано в [Как работают ограничения](/docs/ru/plugins/org#restrict-what-users-can-install).

* **Scope**: [`Managed`](#scopes)
* **Type**: массив объектов источника маркетплейса; см. [Разрешённые типы источников](#allowed-source-types)
* **Default**: не установлено, поэтому пользователи могут добавлять любой маркетплейс. Пустой массив — это полная блокировка, которая блокирует каждый источник маркетплейса, включая официальный маркетплейс Anthropic

Этот пример разрешает два репозитория GitHub, один закреплённый на ref `v2.0`, и один размещённый URL маркетплейса `marketplace.json`:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/approved-plugins" },
    { "source": "github", "repo": "acme-corp/security-tools", "ref": "v2.0" },
    { "source": "url", "url": "https://plugins.example.com/marketplace.json" }
  ]
}
```

Вы также можете написать этот ключ как `allowedMarketplaces`; [Псевдонимы ключей маркетплейса](#marketplace-key-aliases) описывает, как Claude Code рассматривает псевдоним и какая версия его принимает. Этот ключ — это ворота политики: он контролирует, что пользователи могут добавлять, но ничего не регистрирует. Чтобы ограничить и предварительно зарегистрировать в одном файле, см. [Объедините с `extraKnownMarketplaces`](#combine-with-extraknownmarketplaces). Для представления, обращённого к пользователю, см. [Управляемые ограничения маркетплейса](/docs/ru/plugins/org#restrict-what-users-can-install).

<h4 id="allowed-source-types">
  Allowed source types
</h4>

Каждая запись ниже показывает одну запись списка разрешений на тип источника и поля, которые она принимает. Большинство типов совпадают точно; `hostPattern` и `pathPattern` совпадают по регулярному выражению, и записи `github` могут использовать [подстановочный знак владельца](#owner-wildcards).

| Source        | Example entry                                                                                                                   | Fields                                                                                                                                                          |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `github`      | `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main", "path": "marketplace" }`                                     | `repo` обязателен; `ref` — это ветка или тег; `path` — это подкаталог                                                                                           |
| `git`         | `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git", "ref": "production" }`                               | `url` обязателен; `ref` и `path` как для `github`                                                                                                               |
| `url`         | `{ "source": "url", "url": "https://plugins.example.com/marketplace.json", "headers": { "Authorization": "Bearer ${TOKEN}" } }` | `url` обязателен; `headers` добавляет HTTP заголовки для аутентифицированного доступа                                                                           |
| `file`        | `{ "source": "file", "path": "/opt/acme-corp/plugins/marketplace.json" }`                                                       | `path` обязателен, абсолютный путь к файлу `marketplace.json`                                                                                                   |
| `directory`   | `{ "source": "directory", "path": "/opt/acme-corp/approved-marketplaces" }`                                                     | `path` обязателен, абсолютный путь к каталогу, содержащему `.claude-plugin/marketplace.json`                                                                    |
| `hostPattern` | `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`                                                        | `hostPattern` обязателен, регулярное выражение, совпадающее в любом месте хоста маркетплейса; закрепите его с помощью `^` и `$`, чтобы совпадать со всем хостом |
| `pathPattern` | `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`                                                                 | `pathPattern` обязателен, регулярное выражение, совпадающее в любом месте `path` источников `file` и `directory`; начните его с `^`, чтобы закрепить префикс    |
| `skills-dir`  | `{ "source": "skills-dir" }`                                                                                                    | Нет полей. Включает сканирование плагина `~/.claude/skills/` обратно                                                                                            |

Три типа источников имеют правила, выходящие за рамки таблицы:

* **`url`**: маркетплейс URL загружает только файл `marketplace.json`, и Claude Code не получает файлы плагинов по относительному пути с этого сервера, поэтому его плагины должны использовать [источник плагина](/docs/ru/plugins/marketplace-reference#plugin-sources), отличный от относительного пути, такой как URL архива, который может быть на том же хосте. Для плагинов с относительными путями используйте маркетплейс на основе Git. См. [Плагины с относительными путями не работают в маркетплейсах на основе URL](/docs/ru/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces).
* **`hostPattern`**: используйте его, чтобы разрешить каждый маркетплейс на внутреннем GitHub Enterprise или GitLab сервере без перечисления каждого репозитория. Claude Code совпадает с источниками `github` против `github.com`, берёт имя хоста из источников `url` и берёт его из источников `git` в зависимости от формы [git URL](https://git-scm.com/docs/git-clone#_git_urls):

  * URL со схемой, такой как `https://` или `ssh://`: имя хоста в URL.
  * SSH адрес без схемы, в форме `user@host:path` git, такой как `git@git.example.com:tools/plugins.git`: хост между `@` и `:`, который является хостом, к которому подключается git.
  * Любая другая форма без схемы: нет хоста, поэтому ни одна запись `strictKnownMarketplaces` `hostPattern` не совпадает с ней. Для `blockedMarketplaces` `hostPattern`, Claude Code берёт хост из более широкого набора форм, поэтому запись списка блокировки всё ещё может совпадать с такой формой. До v2.1.234, `strictKnownMarketplaces` `hostPattern` также совпадала с некоторыми формами, которые git не рассматривает как SSH адреса.

  Источники `file` и `directory` не имеют хоста и никогда не совпадают с записью `hostPattern`.
* **`pathPattern`**: используйте его, чтобы разрешить маркетплейсы файловой системы наряду с записями `hostPattern` для сетевых источников. `".*"` разрешает каждый локальный путь; более узкий паттерн, такой как `"^/opt/approved/"`, ограничивает каталогом.

Любой список разрешений, даже пустой, также останавливает загрузку Claude Code [`@skills-dir` плагинов](/docs/ru/plugins/loading#plugins-shared-through-a-repository) из `~/.claude/skills/`. Добавьте запись `{ "source": "skills-dir" }`, чтобы продолжить их загрузку; запись не имеет значения вне этого ключа и `blockedMarketplaces`.

<h4 id="owner-wildcards">
  Owner wildcards
</h4>

Запись `github`, чьё значение `repo` — это `"<owner>/*"`, совпадает с каждым репозиторием под этим владельцем GitHub. Подстановочные знаки владельца требуют Claude Code v2.1.223 или позже и работают только в `strictKnownMarketplaces` и `blockedMarketplaces`. Везде, где источник `github` появляется, такой как `extraKnownMarketplaces` или `/plugin marketplace add`, значение `repo` должно называть один репозиторий. До v2.1.223, Claude Code сравнивал запись буквально, поэтому запись списка разрешений не совпадала ни с одним репозиторием и запись списка блокировки ничего не блокировала; записи с одним репозиторием применяются на каждой версии.

Эта запись разрешает любой маркетплейс репозитория в организации `acme-corp`:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/*" }
  ]
}
```

Только вся позиция имени репозитория может быть подстановочным знаком. Claude Code игнорирует записи, такие как `*`, `*/plugins` или `acme-corp/tools-*`, как недействительные, поэтому они не совпадают ни с одним репозиторием.

Правила сопоставления различаются между двумя параметрами:

| Rule                      | `strictKnownMarketplaces`                                                                                                                                                             | `blockedMarketplaces`                                                                 |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| Matching source spellings | Только форма `owner/repo`. Git URL, который клонирует тот же репозиторий, не совпадает                                                                                                | Любое написание, включая git URL, которые разрешаются в тот же репозиторий github.com |
| Owner case                | Чувствительно к регистру, как точное сопоставление записей                                                                                                                            | Нечувствительно к регистру                                                            |
| `ref`                     | Следует правилам точного сопоставления: запись с `ref` совпадает только с источниками с этим точным ref, и запись без одного совпадает только с источниками, которые не указывают ref | Запись без `ref` блокирует все ref репозиториев, которые она совпадает                |
| `path`                    | Более свободно, чем правила точного сопоставления: запись с `path` требует это точное значение, в то время как запись без одного совпадает с любым путём внутри репозитория           | Запись без `path` блокирует все пути репозиториев, которые она совпадает              |

<h4 id="exact-matching">
  Exact matching
</h4>

Для каждого типа источника, кроме записей `github` с подстановочным знаком владельца и записей `hostPattern` и `pathPattern`, совпадающих по регулярному выражению, Claude Code разрешает добавление пользователя только когда источник маркетплейса точно совпадает с записью. Для источников на основе git `github` и `git`, точное сопоставление включает необязательные поля:

* `repo` или `url` должны совпадать точно
* Поле `ref` должно совпадать точно, или оба должны быть не определены
* Поле `path` должно совпадать точно, или оба должны быть не определены

Например, Claude Code рассматривает каждую пару ниже как два разных источника:

* `{ "source": "github", "repo": "acme-corp/plugins" }` и `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main" }`
* `{ "source": "github", "repo": "acme-corp/plugins", "path": "marketplace" }` и `{ "source": "github", "repo": "acme-corp/plugins" }`

<h4 id="allow-only-the-official-marketplace">
  Allow only the official marketplace
</h4>

Чтобы разрешить только официальный маркетплейс Anthropic, перечислите его репозиторий:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" }
  ]
}
```

С этой записью Claude Code сохраняет уже зарегистрированный официальный маркетплейс доступным и, на свежей машине, регистрирует маркетплейс автоматически при первом запуске интерактивного сеанса терминала. Автоматическая регистрация наиболее часто пропускает:

* Неинтерактивные среды, которые работают до первого интерактивного сеанса терминала машины.
* Машины, где Claude Code работал только через расширение VS Code.
* Машины, где Claude Code уже запустил интерактивный сеанс терминала под политикой, которая блокировала маркетплейс, такой как блокировка пустого массива. Claude Code записывает заблокированную попытку и не повторяет попытку после изменения политики.

На этих машинах добавьте маркетплейс в [`extraKnownMarketplaces`](#extraknownmarketplaces) в том же `managed-settings.json`, чтобы Claude Code зарегистрировал его автоматически, или запустите `claude plugin marketplace add anthropics/claude-plugins-official`.

<h4 id="combine-with-extraknownmarketplaces">
  Combine with `extraKnownMarketplaces`
</h4>

Два ключа выполняют разные работы. Эта таблица их сравнивает:

| Aspect            | `strictKnownMarketplaces`                             | `extraKnownMarketplaces`                                                                                           |
| ----------------- | ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| Purpose           | Применение организационной политики                   | Удобство команды                                                                                                   |
| Settings file     | Только управляемые параметры                          | Любой файл параметров                                                                                              |
| Behavior          | Блокирует добавления, не входящие в список разрешений | Регистрирует отсутствующие маркетплейсы                                                                            |
| When enforced     | Перед сетевыми и файловыми операциями                 | Сразу из пользовательских или управляемых параметров; после диалога доверия рабочей области для файлов репозитория |
| Can be overridden | Нет, наивысший приоритет                              | Да, параметрами с более высоким приоритетом                                                                        |
| Source format     | Прямой объект источника                               | Именованный маркетплейс с вложенным объектом `source`                                                              |

Чтобы ограничить и предварительно зарегистрировать маркетплейс для всех пользователей, установите оба в `managed-settings.json`:

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

Только с установленным `strictKnownMarketplaces`, пользователи всё ещё могут добавить разрешённый маркетплейс сами с помощью `/plugin marketplace add`. Официальный маркетплейс Anthropic — единственный, который Claude Code регистрирует автоматически, и только когда список разрешений его разрешает. [Разрешите только официальный маркетплейс](#allow-only-the-official-marketplace) перечисляет машины, которые он пропускает.

<h3 id="strictpluginonlycustomization">
  `strictPluginOnlyCustomization`
</h3>

Заблокируйте skills, agents, hooks и MCP серверы из источников пользователя и проекта, чтобы они могли поступать только из плагинов или управляемых параметров. Объедините его с [`strictKnownMarketplaces`](#strictknownmarketplaces), чтобы контролировать полную цепочку поставок настройки: список разрешений маркетплейса контролирует, какие плагины пользователи могут устанавливать.

* **Scope**: [`Managed`](#scopes)
* **Type**: `true` для блокировки всех четырёх видов настройки, или массив, называющий виды для блокировки, из `"skills"`, `"agents"`, `"hooks"` и `"mcp"`
* **Default**: не установлено, поэтому ничего не заблокировано

Этот пример блокирует skills и hooks и оставляет agents и MCP серверы разблокированными:

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills", "hooks"]
}
```

Четыре записи подключа ниже перечисляют, что каждая поверхность блокирует и что всё ещё загружается. Claude Code игнорирует имена поверхностей, которые он не распознаёт, вместо того чтобы не пройти файл параметров, поэтому вы можете добавлять новые имена поверхностей перед обновлением каждого клиента.

<h3 id="strictpluginonlycustomization-skills">
  `strictPluginOnlyCustomization.skills`
</h3>

Заблокируйте поверхность `skills`. Claude Code прекращает загрузку skills из `~/.claude/skills/` и `.claude/skills/`, пользовательских команд из `~/.claude/commands/` и `.claude/commands/`, skills под каталогами `--add-dir`, и skills, синхронизированные из вашей учётной записи claude.ai, и продолжает загружать skills плагинов, встроенные skills и skills в каталоге управляемой политики.

* **Scope**: [`Managed`](#scopes)
* **Type**: строка `"skills"` в массиве [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: не заблокировано

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills"]
}
```

<h3 id="strictpluginonlycustomization-agents">
  `strictPluginOnlyCustomization.agents`
</h3>

Заблокируйте поверхность `agents`. Claude Code прекращает загрузку agents из `~/.claude/agents/` и `.claude/agents/`, и продолжает загружать agents плагинов, встроенные agents и agents в каталоге управляемой политики.

* **Scope**: [`Managed`](#scopes)
* **Type**: строка `"agents"` в массиве [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: не заблокировано

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["agents"]
}
```

<h3 id="strictpluginonlycustomization-hooks">
  `strictPluginOnlyCustomization.hooks`
</h3>

Заблокируйте поверхность `hooks`. Claude Code прекращает запуск hooks из пользователя, проекта и локального `settings.json`, и продолжает запускать hooks плагинов и hooks в управляемых параметрах.

* **Scope**: [`Managed`](#scopes)
* **Type**: строка `"hooks"` в массиве [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: не заблокировано

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["hooks"]
}
```

<h3 id="strictpluginonlycustomization-mcp">
  `strictPluginOnlyCustomization.mcp`
</h3>

Заблокируйте поверхность `mcp`. Claude Code прекращает загрузку MCP серверов из `~/.claude.json` и `.mcp.json`, и продолжает загружать MCP серверы плагинов, [`managed-mcp.json`](/docs/ru/managed-mcp) серверы и серверы из [`managedMcpServers`](#managedmcpservers).

* **Scope**: [`Managed`](#scopes)
* **Type**: строка `"mcp"` в массиве [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: не заблокировано

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["mcp"]
}
```

<h3 id="enabledplugins">
  `enabledPlugins`
</h3>

Включайте или отключайте отдельные [plugins](/docs/ru/plugins/overview), обозначенные как `plugin-name@marketplace-name`. Плагин без записи в любой области возвращается к его значению [`defaultEnabled`](/docs/ru/plugins/manifest-reference#fields). Когда вы включаете или отключаете плагин с помощью `/plugin` или `claude plugin enable`, Claude Code записывает этот ключ для вас.

* **Scope**: [`Any file`](#scopes)
* **Type**: объект, отображающий `plugin-name@marketplace-name` на Boolean
* **Default**: не установлено, поэтому каждый плагин следует его значению `defaultEnabled`

Этот пример включает два плагина из маркетплейса `team-tools` и отключает один из `personal`:

```json settings.json theme={null}
{
  "enabledPlugins": {
    "code-formatter@team-tools": true,
    "deployment-tools@team-tools": true,
    "experimental-features@personal": false
  }
}
```

Каждая область служит разной цели:

* **User settings**: ваши личные предпочтения плагинов
* **Project settings**: плагины, общие со всеми в репозитории
* **Local settings**: переопределения для каждой машины, игнорируемые, когда Claude Code сохраняет параметр там
* **Managed settings**: организационная политика. Плагин, установленный на `false` здесь, заблокирован от установки в каждой области и скрыт из маркетплейса

Параметры проекта имеют приоритет над параметрами пользователя, поэтому установка плагина на `false` в `~/.claude/settings.json` не отключает плагин, который `.claude/settings.json` проекта включает. Чтобы отказаться от плагина, включённого проектом, на вашей машине, установите его на `false` в `.claude/settings.local.json` вместо этого. Плагины, принудительно включённые управляемыми параметрами, не могут быть отключены таким образом, так как управляемые параметры переопределяют локальные параметры.

Включение плагина из внешнего источника, такого как репозиторий GitHub или пакет npm, в `.claude/settings.json` проекта не устанавливает его для других людей. На каждом пути, который загружает плагины, Claude Code сообщает о плагине как не установленном до тех пор, пока каждый пользователь [не установит его сам](/docs/ru/plugins/org#require-plugins-per-repository).

<h3 id="extraknownmarketplaces">
  `extraKnownMarketplaces`
</h3>

Регистрируйте дополнительные маркетплейсы плагинов по имени, чтобы люди, которые открывают репозиторий, или все, кого достигают ваши управляемые параметры, получали маркетплейс без добавления его самостоятельно. Claude Code регистрирует каждый маркетплейс, который он ещё не знает. Устанавливается ли плагин, который [`enabledPlugins`](#enabledplugins) называет из него, зависит от источника плагина и какой файл его включает; эта запись имеет правила.

* **Scope**: [`Any file`](#scopes). Claude Code учитывает записи в `.claude/settings.json` или `.claude/settings.local.json` репозитория только после того, как вы примете диалог доверия рабочей области для этой папки; в папке, которую вы не доверяете, включая запуск `-p` там, он игнорирует их без сообщения.
* **Type**: объект, отображающий имя маркетплейса на объект с объектом `source` и необязательным Boolean `autoUpdate`
* **Default**: не установлено

Этот пример регистрирует маркетплейс GitHub и маркетплейс из самостоятельно размещённого git URL:

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

[Что работает перед тем, как вы доверяете папке](/docs/ru/permissions#what-runs-before-you-trust-a-folder), сравнивает ворота доверия с другим содержимым, которое репозиторий может предоставить. Вы также можете написать этот ключ как `additionalMarketplaces`; см. [Псевдонимы ключей маркетплейса](#marketplace-key-aliases).

Установите `"autoUpdate": true` рядом с `source`, чтобы Claude Code обновлял этот маркетплейс и обновлял его установленные плагины в фоне после запуска. Если опущено, `claude-plugins-official` и большинство других официальных маркетплейсов Anthropic по умолчанию имеют `true`, а маркетплейсы третьих сторон по умолчанию имеют `false`. См. [Настройте автоматические обновления](/docs/ru/plugins/install#keep-plugins-updated).

Когда более одного файла параметров определяет запись маркетплейса под одним и тем же именем, Claude Code использует запись из [файла с наивысшим приоритетом](/docs/ru/settings#settings-precedence) целиком. Эта запись заменяет запись с более низким приоритетом и не наследует ни одно из её полей, поэтому переопределение не может объединить `source.headers` учётные данные одного файла с URL, который контролирует другой файл. До v2.1.228, Claude Code объединял записи с одним и тем же именем поле за полем, поэтому запись в файле с более высоким приоритетом могла наследовать поля, которые она не установила, включая `headers` другого файла.

<h4 id="marketplace-source-types">
  Marketplace source types
</h4>

Объект `source` принимает одну из этих форм:

* **`github`**: репозиторий GitHub, с `repo`
* **`git`**: любой git URL, с `url`
* **`url`**: прямой URL к файлу `marketplace.json`, с `url` и необязательными `headers` и `headersHelper` для аутентифицированного доступа. `headersHelper` называет команду, которая печатает заголовки, чьи значения слишком недолговечны для перечисления в `headers`, и требует Claude Code v2.1.238 или позже
* **`file`**: локальный путь к файлу `marketplace.json`, с `path`
* **`directory`**: локальный путь файловой системы, с `path`, только для разработки
* **`settings`**: встроенный маркетплейс, объявленный непосредственно в файле параметров без размещённого репозитория, с `name` и `plugins`

Тип источника `git` работает с любым сервисом размещения git, включая самостоятельно размещённые GitLab и Bitbucket. Claude Code клонирует репозиторий с той же аутентификацией, которую использовал бы `git clone` на этой машине: настроенные помощники учётных данных или ключи SSH. Токен поставщика, такой как `GITHUB_TOKEN`, вступает в силу через помощника учётных данных, который его читает. См. [Приватные репозитории](/docs/ru/plugins/host-marketplace#grant-access-to-a-private-marketplace) для деталей настройки.

Для источников `github` и `git`, Claude Code никогда не загружает содержимое [Git LFS](https://git-lfs.com) при клонировании репозитория маркетплейса для добавления или обновления. Файлы, отслеживаемые LFS, проверяются как файлы указателей, и вывод добавления или обновления сообщает, сколько их.

Поле `skipLfs` внутри объекта `source` принимается и не имеет эффекта. До v2.1.274, Claude Code загружал содержимое LFS, если вы не установили `"skipLfs": true`.

Для источника `url`, установите `headersHelper` внутри объекта `source`, когда учётные данные в `headers` истекают и команда должна произвести свежие. Требуется Claude Code v2.1.238 или позже. Для того, что команда должна печатать и где Claude Code её запускает, см. [Напишите команду headersHelper](/docs/ru/plugins/host-marketplace#write-the-headershelper-command), и для случаев, когда Claude Code её не запускает, см. [Когда Claude Code пропускает команду headersHelper](/docs/ru/plugins/host-marketplace#when-claude-code-skips-a-headershelper-command-or-drops-its-output). Как только вы установите `headersHelper` на URL маркетплейса `https://`, Claude Code запускает команду в двух точках, повторно используя вывод одного запуска в течение до 60 секунд:

* Перед каждой выборкой `marketplace.json` этого маркетплейса, включая более позднее обновление. Claude Code отправляет напечатанные заголовки с этой выборкой.
* Перед каждой загрузкой архива плагина на происхождение URL маркетплейса, означая ту же схему, хост и порт. Claude Code отправляет вывод с этой загрузкой, и никакая другая загрузка не получает заголовки.

Claude Code игнорирует любой `headersHelper`, установленный в `.claude/settings.json` или `.claude/settings.local.json` каталога, который вы добавляете с [`--add-dir`](/docs/ru/permissions#what-runs-before-you-trust-a-folder), на источнике `url` и на встроенной записи плагина, и отправляет только фиксированные `headers`, установленные в этом файле. [Как пользователи принимают команду headersHelper](/docs/ru/plugins/host-marketplace#how-users-accept-a-headershelper-command) охватывает другие файлы параметров.

Плагины, перечисленные в источнике `settings`, должны ссылаться на внешние источники, такие как GitHub или npm, и `name` должен совпадать с ключом маркетплейса. Вы всё ещё включаете каждый плагин отдельно в `enabledPlugins`. Этот пример объявляет один плагин встроенным:

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

Запись плагина под `source: 'settings'`, чей собственный `source` — это [`archive`](/docs/ru/plugins/marketplace-reference#archive-plugin-source), может установить `headers` для загрузки архива. Если значение, которое вы поместили бы в `headers`, недолговечно, такое как токен, который ваш реестр чеканит по запросу, установите вместо этого команду `headersHelper`. Запись может установить оба. Оба поля требуют Claude Code v2.1.238 или позже.

Claude Code отправляет `headers` записи и всё, что печатает команда, с загрузкой архива этого плагина и ни с какой другой загрузкой. Claude Code запускает команду только когда пользователь [устанавливает или обновляет только этот один плагин](/docs/ru/plugins/host-marketplace#how-users-accept-a-headershelper-command). Три дополнительных правила зависят от того, какой файл содержит запись:

* **`strict`**: в отличие от записи в `marketplace.json` маркетплейса, запись в параметрах не нуждается в `"strict": false`, потому что файл параметров не содержит полей манифеста для встраивания. См. [Strict mode](/docs/ru/plugins/marketplace-reference#strict-mode).
* **Folder trust**: для записи в `.claude/settings.json` или `.claude/settings.local.json` проекта, Claude Code запускает команду только после того, как пользователь также [доверил эту папку](/docs/ru/permissions#what-runs-before-you-trust-a-folder).
* **Header filter**: Claude Code удаляет [имена заголовков маршрутизации запросов и идентификации клиента](/docs/ru/plugins/host-marketplace#when-claude-code-skips-a-headershelper-command-or-drops-its-output) из записи в `.claude/settings.json` или `.claude/settings.local.json` проекта, потому что репозиторий может предоставить эти файлы. Claude Code применяет тот же фильтр к записи каталога и к записи в каталоге `--add-dir`, и никакой фильтр к записи в ваши пользовательские параметры, файл `--settings` или управляемые параметры.

<h4 id="marketplace-key-aliases">
  Marketplace key aliases
</h4>

На Claude Code v2.1.232 или позже, вы можете написать `extraKnownMarketplaces` как `additionalMarketplaces` и `strictKnownMarketplaces` как `allowedMarketplaces`. Claude Code рассматривает каждый псевдоним следующим образом:

* Более ранние версии игнорируют псевдоним, поэтому сохраняйте каноническое написание в файле, который также читают более старые версии, такой как файл управляемых параметров для флота со смешанными версиями Claude Code.
* В любом файле параметров, который принимает канонический ключ, Claude Code читает псевдоним точно так же, как читает канонический ключ.
* Claude Code может переписать `additionalMarketplaces` на `extraKnownMarketplaces` при обновлении файла.
* Если вы установите оба написания в одном файле, Claude Code использует каноническое значение и игнорирует псевдоним.

<h3 id="pluginconfigs">
  `pluginConfigs`
</h3>

Сохраняйте нечувствительные ответы, которые вы даёте диалогу конфигурации [`userConfig`](/docs/ru/plugins/manifest-reference#user-configuration) плагина, обозначенные по ID плагина. Claude Code записывает этот ключ в ваши пользовательские параметры при заполнении диалога, поэтому вам не нужно редактировать его вручную. Claude Code сохраняет чувствительные параметры в macOS Keychain вместо этого, возвращаясь к `~/.claude/.credentials.json`, когда Keychain отклоняет запись; на платформах без поддерживаемого keychain, он сохраняет их в `~/.claude/.credentials.json`.

* **Scope**: [`User or managed`](#scopes)
* **Type**: объект, отображающий ID плагина на объект с полем `options`, отображающим каждое имя параметра на строку, число, Boolean или массив строк, и необязательное поле `mcpServers`, содержащее значения конфигурации пользователя для каждого сервера в той же форме
* **Default**: не установлено

Этот пример сохраняет параметр `api_endpoint` для плагина `deployer` из `acme-tools`:

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

Встроенные плагины сохраняют свои параметры под тем же ключом с суффиксом `@builtin`. Например, параметр [**Project instructions**](/docs/ru/memory#choose-which-instruction-files-load), который контролирует, читает ли Claude Code файлы `AGENTS.md`, — это `pluginConfigs["agents-md@builtin"].options.instructionFiles`.

Claude Code игнорирует записи проекта и локальные, потому что он подставляет эти значения в конфигурации hook, MCP и LSP плагина, и клонированный репозиторий не должен быть в состоянии их предоставить. До v2.1.207, параметры проекта и локальные также читались.

<h2 id="mcp">
  MCP
</h2>

Контролируйте, к каким серверам MCP подключается Claude Code и какие разрешает организация. См. [Подключение к внешним инструментам с помощью MCP](/docs/ru/mcp) и [Управляемая конфигурация MCP](/docs/ru/managed-mcp).

<h3 id="allowallclaudeaimcps">
  `allowAllClaudeAiMcps`
</h3>

Загружайте [соединители claude.ai](/docs/ru/mcp#use-mcp-servers-from-claude-ai), которые Claude Code получает сам, наряду с развёрнутым `managed-mcp.json`. Без этого ключа `managed-mcp.json` получает исключительный контроль над серверами MCP и подавляет эти соединители.

* **Scope**: [`Managed`](#scopes). Пользователи не могут повторно включить соединители, которые исключительный контроль подавил.
* **Type**: Boolean
  * `true`: Claude Code загружает соединители claude.ai наряду с развёрнутым `managed-mcp.json`
  * `false`: развёрнутый `managed-mcp.json` получает исключительный контроль над серверами MCP и подавляет соединители claude.ai, [которые Claude Code получает сам](/docs/ru/mcp#how-connectors-reach-claude-code)
* **Default**: `false`, поэтому развёрнутый `managed-mcp.json` подавляет соединители claude.ai, которые Claude Code получает сам

```json managed-settings.json theme={null}
{
  "allowAllClaudeAiMcps": true
}
```

[`allowedMcpServers`](#allowedmcpservers) и [`deniedMcpServers`](#deniedmcpservers) по-прежнему применяются к соединителям, которые загружает этот ключ. Соединители, доставленные в [облачный сеанс](/docs/ru/claude-code-on-the-web), хост которого содержит `managed-mcp.json`, например самостоятельно размещённый runner, остаются подавленными. См. [Разрешить соединители claude.ai наряду с управляемым набором](/docs/ru/managed-mcp#allow-claude-ai-connectors-alongside-the-managed-set).

<h3 id="allowedmcpservers">
  `allowedMcpServers`
</h3>

Создайте список разрешённых серверов MCP, которые люди могут добавлять. Claude Code блокирует любой сервер, который не соответствует записи, где бы он ни был определён, включая серверы плагинов, серверы, переданные с `--mcp-config`, и серверы из claude.ai.

Встроенные серверы, такие как Claude в Chrome, сервер `ide`, к которому Claude Code подключается в работающей IDE [VS Code](/docs/ru/vs-code#the-built-in-ide-mcp-server) или [JetBrains](/docs/ru/jetbrains#the-built-in-ide-mcp-server), и серверы, которые сам CLI настраивает, освобождены от списка разрешений, и список запретов по-прежнему применяется к ним. Внутрипроцессные серверы `type: "sdk"` освобождены от обоих списков; [приложение, которое запустило сеанс](/docs/ru/mcp#how-connectors-reach-claude-code), регистрирует их.

Серверы, которые доставляет ваша организация, также освобождены от списка разрешений, и список запретов по-прежнему применяется к ним. Освобождение охватывает каждую запись [`managedMcpServers`](#managedmcpservers) и любую запись [`managed-mcp.json`](/docs/ru/managed-mcp#exclusive-control-with-managed-mcp-json), значения которой не используют расширение `${VAR}`. См. [Как оценивается сервер](/docs/ru/managed-mcp#how-a-server-is-evaluated) для полного порядка проверки. До версии 2.1.259 серверы из `managed-mcp.json` также должны были совпадать.

* **Scope**: [`Any file`](#scopes). Записи из каждого файла объединяются в один список разрешений, если не установлен [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly). Разверните его в управляемых параметрах, чтобы обеспечить его.
* **Type**: массив объектов, каждый с ровно одним ключом: `serverName`, строка, ограниченная буквами, цифрами, дефисами и подчёркиваниями; `serverCommand`, массив команды и её аргументов, совпадающих точно; или `serverUrl`, шаблон URL с подстановочными знаками `*`
* **Default**: не установлено, поэтому каждый сервер разрешён; пустой массив блокирует каждый сервер, который добавляют пользователи

Этот пример разрешает только сервер stdio, который запускает указанная команда `npx`:

```json settings.json theme={null}
{
  "allowedMcpServers": [
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem"] }
  ]
}
```

Запись [`deniedMcpServers`](#deniedmcpservers) имеет приоритет, поэтому сервер в обоих списках блокируется. Как только список содержит любую запись `serverCommand`, сервер stdio должен совпадать с записью `serverCommand`, и как только он содержит любую запись `serverUrl`, удалённый сервер должен совпадать с записью `serverUrl`: совпадение `serverName` больше не допускает этот вид сервера. См. [Управление на основе политики со списками разрешений и запретов](/docs/ru/managed-mcp#policy-based-control-with-allowlists-and-denylists).

<h3 id="allowmanagedmcpserversonly">
  `allowManagedMcpServersOnly`
</h3>

Сделайте управляемый список разрешений единственным применяемым. Claude Code затем читает [`allowedMcpServers`](#allowedmcpservers) только из управляемых параметров и игнорирует списки разрешений в пользовательских, проектных и локальных параметрах; [`deniedMcpServers`](#deniedmcpservers) по-прежнему объединяется из каждой области параметров, поэтому пользователи могут по-прежнему блокировать серверы для себя. Администраторы устанавливают это так, чтобы собственные параметры пользователя не могли расширить то, что разрешает управляемый список разрешений.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code читает `allowedMcpServers` только из управляемых параметров и игнорирует списки разрешений в пользовательских, проектных и локальных параметрах
  * `false`: списки разрешений из каждой области параметров объединяются
* **Default**: `false`, поэтому списки разрешений из каждой области параметров объединяются

Этот пример блокирует список разрешений для управляемых параметров и разрешает только сервер с именем `github`:

```json managed-settings.json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverName": "github" }
  ]
}
```

Пользователи могут по-прежнему добавлять серверы MCP самостоятельно; загружаются только серверы, которые совпадают с управляемым списком разрешений. См. [Ограничить список разрешений только управляемыми параметрами](/docs/ru/managed-mcp#restrict-the-allowlist-to-managed-settings-only).

<h3 id="deniedmcpservers">
  `deniedMcpServers`
</h3>

Блокируйте определённые серверы MCP. Claude Code отказывается загружать совпадающий сервер, где бы он ни был определён, включая серверы плагинов, серверы, переданные с `--mcp-config`, серверы из `managed-mcp.json`, серверы из [`managedMcpServers`](#managedmcpservers) и соединители claude.ai, [которые он получает сам](/docs/ru/mcp#how-connectors-reach-claude-code). Внутрипроцессные серверы `type: "sdk"` освобождены; приложение, которое запустило сеанс, регистрирует их.

* **Scope**: [`Any file`](#scopes). Записи из каждого файла объединяются в один список запретов, и [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly) это не меняет. Разверните его в управляемых параметрах, чтобы обеспечить его.
* **Type**: массив объектов, каждый с ровно одним ключом: `serverName`, строка, поэтому отображаемое имя соединителя claude.ai, такое как `"claude.ai Slack"`, работает; `serverCommand`, массив команды и её аргументов, совпадающих точно; или `serverUrl`, шаблон URL с подстановочными знаками `*`
* **Default**: не установлено, поэтому ни один сервер не блокируется; пустой массив также ничего не блокирует

```json settings.json theme={null}
{
  "deniedMcpServers": [
    { "serverName": "filesystem" }
  ]
}
```

Список запретов имеет приоритет над [`allowedMcpServers`](#allowedmcpservers), поэтому сервер в обоих списках блокируется. См. [Управление на основе политики со списками разрешений и запретов](/docs/ru/managed-mcp#policy-based-control-with-allowlists-and-denylists).

<h3 id="disableclaudeaiconnectors">
  `disableClaudeAiConnectors`
</h3>

Отключите [соединители MCP claude.ai](/docs/ru/mcp#use-mcp-servers-from-claude-ai), [которые Claude Code получает сам](/docs/ru/mcp#how-connectors-reach-claude-code), чтобы он ни получал, ни подключал их. `true` в любом файле параметров применяется: проверенный в репозитории проект `.claude/settings.json` может отказать репозиторию в этих соединителях, но проектный уровень `false` не может переопределить пользовательский или управляемый уровень `true`.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code ни получает, ни подключает эти соединители
  * `false`: то же самое, что и не установлено; Claude Code получает ваши соединители, если другой файл параметров или `ENABLE_CLAUDEAI_MCP_SERVERS` их не отключает
* **Default**: `false`, поэтому Claude Code получает ваши соединители
* **Per-session overrides**: [`ENABLE_CLAUDEAI_MCP_SERVERS`](/docs/ru/env-vars), установленный на `false`, отключает соединители на один сеанс; какой бы из двух их ни отключил, другой не может их включить обратно

```json settings.json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Серверы, которые вы явно передаёте с `--mcp-config`, не затрагиваются. Чтобы блокировать отдельные соединители вместо всех, используйте [`deniedMcpServers`](#deniedmcpservers). См. [Отключить соединители claude.ai](/docs/ru/mcp#disable-claude-ai-connectors).

<h3 id="disabledmcpjsonservers">
  `disabledMcpjsonServers`
</h3>

Отклоняйте определённые серверы, определённые в файле `.mcp.json` проекта, чтобы Claude Code никогда их не подключал и не просил вас их одобрить. Отклонение в любом файле параметров применяется, включая проект `.claude/settings.json`, проверенный в репозитории.

* **Scope**: [`Any file`](#scopes)
* **Type**: массив строк, имена серверов, как они появляются в `.mcp.json`
* **Default**: не установлено

```json settings.json theme={null}
{
  "disabledMcpjsonServers": ["filesystem"]
}
```

Claude Code записывает этот ключ в `.claude/settings.local.json`, когда вы отклоняете сервер в диалоговом окне одобрения. `claude mcp get <name>` показывает отклонённый сервер как `✘ Rejected (see disabledMcpjsonServers in settings)`. Отклонение имеет приоритет над [`enabledMcpjsonServers`](#enabledmcpjsonservers) и [`enableAllProjectMcpServers`](#enableallprojectmcpservers).

<h3 id="enableallprojectmcpservers">
  `enableAllProjectMcpServers`
</h3>

Одобряйте каждый сервер MCP, определённый в файлах проекта `.mcp.json`, без подсказки. Claude Code записывает этот ключ в `.claude/settings.local.json`, когда вы выбираете одобрение всех серверов в диалоговом окне одобрения.

* **Scope**: [`Any file`](#scopes). В папке, диалоговое окно доверия которой вы не приняли, Claude Code соблюдает его из пользовательских параметров, управляемых параметров и `--settings` и игнорирует его в общем файле проекта, как в сеансе, так и для `claude mcp list` и `claude mcp get`; [Одобрения серверов проекта и доверие рабочей области](/docs/ru/mcp#project-server-approvals-and-workspace-trust) говорит, когда неотслеживаемый `.claude/settings.local.json` также считается.
* **Type**: Boolean
  * `true`: Claude Code одобряет каждый сервер MCP, определённый в файлах проекта `.mcp.json`, без подсказки
  * `false`: Claude Code просит вас одобрить каждый сервер. В доверенной папке `false` в файле с более высоким приоритетом переопределяет `true` в файле с более низким приоритетом; в папке, которой вы не доверяете, `true` в любом соблюдаемом файле достаточно
* **Default**: не установлено, поэтому Claude Code просит вас одобрить каждый сервер

```json settings.json theme={null}
{
  "enableAllProjectMcpServers": true
}
```

Запись [`disabledMcpjsonServers`](#disabledmcpjsonservers) по-прежнему отклоняет сервер.

<h3 id="enabledmcpjsonservers">
  `enabledMcpjsonServers`
</h3>

Одобряйте определённые серверы, определённые в файлах проекта `.mcp.json`, чтобы Claude Code подключал их без вопросов. Claude Code записывает этот ключ в `.claude/settings.local.json`, когда вы одобряете сервер в диалоговом окне одобрения.

* **Scope**: [`Any file`](#scopes). В папке, диалоговое окно доверия которой вы не приняли, Claude Code соблюдает его из пользовательских параметров, управляемых параметров и `--settings` и игнорирует его в общем файле проекта, как в сеансе, так и для `claude mcp list` и `claude mcp get`; [Одобрения серверов проекта и доверие рабочей области](/docs/ru/mcp#project-server-approvals-and-workspace-trust) говорит, когда неотслеживаемый `.claude/settings.local.json` также считается.
* **Type**: массив строк, имена серверов, как они появляются в `.mcp.json`
* **Default**: не установлено

Этот пример одобряет серверы `memory` и `github` из `.mcp.json` проекта:

```json settings.json theme={null}
{
  "enabledMcpjsonServers": ["memory", "github"]
}
```

Запись [`disabledMcpjsonServers`](#disabledmcpjsonservers) по-прежнему отклоняет сервер.

<h3 id="managedmcpservers">
  `managedMcpServers`
</h3>

Предоставляйте удалённые серверы MCP каждому пользователю из управляемых параметров. Пользователи сохраняют серверы, которые они добавляют сами, и не могут редактировать или удалять те, которые вы предоставляете. Требует Claude Code версии 2.1.259 или позже.

* **Scope**: [`Managed`](#scopes). Claude Code удаляет ключ с предупреждением в пользовательских, проектных и локальных параметрах и не читает его в вкладке Code приложения Claude Desktop при развёртывании третьей стороной или в сеансах Cowork приложения, где Claude Desktop сам предоставляет и блокирует серверы MCP этих сеансов.
* **Type**: объект, ключ которого — имя сервера. Каждая запись имеет форму `.mcp.json` для сервера `http` или `sse`: требуемый `https://` `url` и опционально `headers`, `oauth` и другие параметры HTTP и SSE. Claude Code удаляет записи, которые не прошли проверку, и [Что может содержать запись](/docs/ru/managed-mcp#what-an-entry-can-contain) перечисляет условия
* **Default**: не установлено, поэтому управляемые параметры не предоставляют серверы

Этот пример предоставляет один HTTP-сервер с именем `search`:

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

Для приоритета, как предоставленные серверы объединяются с `managed-mcp.json` и списками разрешений и запретов, и что видят пользователи, см. [Предоставлять серверы через управляемые параметры](/docs/ru/managed-mcp#provide-servers-through-managed-settings).

<h2 id="agents-sessions-and-worktrees">
  Агенты, сессии и worktrees
</h2>

Установите агента по умолчанию, управляйте товарищами по команде и обменом сообщениями между сессиями, а также настройте worktrees. См. [Subagents](/docs/ru/sub-agents) и [Worktrees](/docs/ru/worktrees).

<h3 id="agent">
  `agent`
</h3>

Запустите основной поток как именованный [subagent](/docs/ru/sub-agents#invoke-subagents-explicitly), чтобы Claude Code применил системный prompt, ограничения инструментов и модель этого subagent к вашей сессии. Один и тот же ключ устанавливает агента по умолчанию для сессий, которые вы отправляете из `claude agents`.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, имя встроенного или пользовательского агента
* **Default**: не установлено, поэтому основной поток работает как агент Claude Code по умолчанию
* **Per-session overrides**: `--agent` имеет приоритет над этим ключом для одной сессии

```json settings.json theme={null}
{
  "agent": "code-reviewer"
}
```

Собственный `settings.json` плагина также может предоставить этот ключ; см. [Ship default settings with your plugin](/docs/ru/plugins/components#default-settings).

<h3 id="crosssessioninbound">
  `crossSessionInbound`
</h3>

Выберите, что эта сессия делает с [сообщениями, поступающими из ваших других сессий Claude Code](/docs/ru/cross-session-messaging#control-inbound-messages). Когда ни одно значение не применяется, Claude Code решает для каждого сообщения на основе классов режима разрешений двух сессий. Требуется Claude Code v2.1.224 или позже.

* **Scope**: [`Any file`](#scopes). Значение проекта или локальное значение применяется только в том случае, если оно строже, чем значение управляемых параметров, флага `--settings` или пользовательских параметров.
* **Type**: string, один из:
  * `"accept"`: Claude Code доставляет сообщение Claude
  * `"hold"`: Claude Code показывает уведомление о сообщении без его доставки
  * `"refuse"`: Claude Code отбрасывает сообщение
* **Default**: не установлено, поэтому Claude Code решает для каждого сообщения

```json settings.json theme={null}
{
  "crossSessionInbound": "hold"
}
```

Claude Code сначала читает управляемые параметры, затем флаг `--settings`, затем пользовательские параметры и применяет первое найденное значение. `refuse` строже, чем `hold`, а `hold` строже, чем `accept`. Когда ни один из доверенных источников не устанавливает значение, проект или локальное `hold` или `refuse` все еще применяется, заменяя решение по умолчанию для каждого сообщения. В сессиях с обменом сообщениями между сессиями этот ключ появляется в `/config` как **Messages from your other sessions**, который записывает его в пользовательские параметры; строка требует Claude Code v2.1.232 или позже, и Claude Code скрывает её, пока флаг `--settings` или управляемые параметры устанавливают ключ.

Claude Code [предупреждает](/docs/ru/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse) при установке значения, которое он не распознает. Пока это значение присутствует в пользовательском, проектном, локальном или файле `--settings`, Claude Code удерживает входящие сообщения, даже когда источник, который имеет приоритет, устанавливает `accept`. `refuse`, установленный другим источником, все еще применяется. Исправьте или удалите значение, чтобы снять удержание.

Когда неузнанное значение находится в [управляемых параметрах](/docs/ru/managed-settings), Claude Code вместо этого рассматривает его как `refuse` до тех пор, пока администратор не исправит его. До v2.1.248 Claude Code игнорировал неузнанное значение без предупреждения.

<h3 id="disableagentview">
  `disableAgentView`
</h3>

Отключите [фоновых агентов и представление агентов](/docs/ru/agent-view): `claude agents`, `--bg`, `/background` и супервизора по требованию. Установите его в [управляемых параметрах](/docs/ru/managed-settings), чтобы применить его для организации.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code отключает `claude agents`, `--bg`, `/background` и супервизора по требованию
  * `false`: представление агентов доступно
* **Default**: не установлено, поэтому представление агентов доступно
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_AGENT_VIEW`](/docs/ru/env-vars) отключает представление агентов для одной сессии; какой бы из двух его ни отключил, другой не может его снова включить

```json settings.json theme={null}
{
  "disableAgentView": true
}
```

<h3 id="isolatepeermachines">
  `isolatePeerMachines`
</h3>

Требуйте вашего явного одобрения перед тем, как `SendMessage` Claude достигнет одну из ваших сессий за пределами этой машины; см. [Require approval for cross-machine messages](/docs/ru/cross-session-messaging#require-approval-for-cross-machine-messages). Запрос одобрения появляется даже в режиме [`bypassPermissions`](/docs/ru/permission-modes#skip-all-checks-with-bypasspermissions-mode).

* **Scope**: [`Any file`](#scopes). `true` из любой области применяется, поэтому проверенный файл проекта может включить требование, но не отключить его.
* **Type**: Boolean
  * `true`: Claude Code запрашивает ваше одобрение перед тем, как `SendMessage` Claude достигнет одну из ваших сессий за пределами этой машины
  * `false`: сообщения между машинами не вызывают запрос
* **Default**: не установлено, поэтому сообщения между машинами не вызывают запрос

```json settings.json theme={null}
{
  "isolatePeerMachines": true
}
```

Одобрение `SendMessage` между машинами требует Claude Code v2.1.224 или позже.

<h3 id="processwrapper">
  `processWrapper`
</h3>

На macOS и Linux поместите команду корпоративного запуска перед [фоновыми процессами, которые запускает Claude Code](/docs/ru/corporate-launcher#what-the-launcher-covers). Claude Code запускает запуск с добавленной собственной командной строкой, поэтому запуск должен выполнить exec в Claude Code; см. [Run Claude Code behind a corporate launcher](/docs/ru/corporate-launcher) для контракта запуска. Требуется Claude Code v2.1.210 или позже.

* **Scope**: [`User or managed`](#scopes)
* **Type**: string, команда запуска как префикс argv, например абсолютный путь с необязательными аргументами
* **Default**: не установлено, поэтому фоновые процессы запускаются без обёртки
* **Per-session overrides**: [`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/ru/env-vars) имеет приоритет над этим ключом для одной сессии

```json settings.json theme={null}
{
  "processWrapper": "/opt/corp/launcher --profile claude"
}
```

Claude Code игнорирует запуск на Windows и запускает каждый процесс без обёртки. Требуется Claude Code v2.1.210 или позже.

<h3 id="teammatemode">
  `teammateMode`
</h3>

Выберите, где Claude Code показывает товарищей по команде [agent team](/docs/ru/agent-teams): внутри вашей основной панели терминала или в разделённых панелях, когда ваш терминал их поддерживает. См. [Choose a display mode](/docs/ru/agent-teams#choose-a-display-mode).

* **Scope**: [`Any file`](#scopes). Claude Code также читает значение, оставленное в `~/.claude.json` старыми версиями.
* **Type**: string, один из:
  * `"in-process"`: товарищи по команде работают внутри вашей основной панели терминала
  * `"auto"`: разделённые панели, когда вы работаете внутри tmux, или внутри iTerm2 с `it2` на вашем `PATH` или установленным tmux; в противном случае в процессе
  * `"tmux"`: разделённые панели с использованием tmux или iTerm2, обнаруженные из вашего терминала
  * `"iterm2"`: собственные разделённые панели iTerm2 через CLI `it2`
* **Default**: `"in-process"`
* **Per-session overrides**: `--teammate-mode` имеет приоритет над этим ключом для одной сессии

```json settings.json theme={null}
{
  "teammateMode": "auto"
}
```

<span id="worktree-settings" />

<h3 id="worktree">
  `worktree`
</h3>

Настройте, как Claude Code создаёт и управляет [git worktrees](/docs/ru/worktrees) для `--worktree`, инструмента `EnterWorktree` и изолированных subagents и фоновых сессий.

* **Scope**: [`Any file`](#scopes)
* **Type**: object с `baseRef`, `symlinkDirectories`, `sparsePaths` и `bgIsolation`
* **Default**: не установлено

Этот пример ветвит новые worktrees из вашего текущего `HEAD` и создаёт символические ссылки `node_modules` в каждый:

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head",
    "symlinkDirectories": ["node_modules"]
  }
}
```

Чтобы скопировать игнорируемые git файлы, такие как `.env`, в новые worktrees, добавьте файл [`.worktreeinclude`](/docs/ru/worktrees#copy-gitignored-files-into-worktrees) в корень вашего проекта вместо параметра.

<h3 id="worktree-baseref">
  `worktree.baseRef`
</h3>

Выберите, из какого ref ветвятся новые worktrees. `"fresh"` ветвится из `origin/<default-branch>` для чистого дерева, соответствующего удалённому; `"head"` ветвится из вашего текущего локального `HEAD`, поэтому неотправленные коммиты и состояние ветки функции присутствуют в worktree.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, один из:
  * `"fresh"`: новые worktrees ветвятся из `origin/<default-branch>`
  * `"head"`: новые worktrees ветвятся из вашего текущего локального `HEAD`, включая неотправленные коммиты
* **Default**: `"fresh"`

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

Внутри связанного worktree `"head"` разрешается в `HEAD` этого worktree, а не в основной checkout.

<h3 id="worktree-symlinkdirectories">
  `worktree.symlinkDirectories`
</h3>

Создайте символические ссылки на директории из основного репозитория в каждый worktree, чтобы вы не дублировали большие директории на диске.

* **Scope**: [`Any file`](#scopes)
* **Type**: array of strings, пути директорий относительно корня репозитория
* **Default**: не установлено, поэтому Claude Code не создаёт символические ссылки на директории

Этот пример создаёт символические ссылки на `node_modules` и `.cache` из основного репозитория в каждый новый worktree:

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

Проверьте только перечисленные директории в каждом worktree через git sparse-checkout. Claude Code записывает только эти директории плюс файлы на уровне корня на диск, что быстрее в больших монорепозиториях; см. [Check out only the directories you need](/docs/ru/large-codebases#check-out-only-the-directories-you-need).

* **Scope**: [`Any file`](#scopes)
* **Type**: array of strings, пути директорий относительно корня репозитория
* **Default**: не установлено, поэтому каждый worktree проверяет всё дерево

Этот пример проверяет только `packages/my-app` и `shared/utils`, плюс файлы на уровне корня, в каждом worktree:

```json settings.json theme={null}
{
  "worktree": {
    "sparsePaths": ["packages/my-app", "shared/utils"]
  }
}
```

Пока существует разреженный worktree, git включает `extensions.worktreeConfig` в общем `.git/config` репозитория.

<h3 id="worktree-bgisolation">
  `worktree.bgIsolation`
</h3>

Выберите, как [фоновые сессии](/docs/ru/agent-view#how-file-edits-are-isolated) изолируют свои редактирования файлов. С `"worktree"` Claude Code блокирует `Edit` и `Write` в основной checkout до тех пор, пока сессия не вызовет `EnterWorktree`; с `"none"` фоновые задания редактируют рабочую копию напрямую. Установите `"none"` для репозитория, где git worktrees непрактичны.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, один из:
  * `"worktree"`: Claude Code блокирует `Edit` и `Write` в основной checkout до тех пор, пока сессия не вызовет `EnterWorktree`
  * `"none"`: фоновые задания редактируют рабочую копию напрямую
* **Default**: `"worktree"`

```json settings.json theme={null}
{
  "worktree": {
    "bgIsolation": "none"
  }
}
```

Вне git репозитория hook [`WorktreeCreate`](/docs/ru/worktrees#non-git-version-control), который завершается с ошибкой, снимает блокировку, чтобы сессия могла редактировать рабочую директорию на месте; это снятие блокировки требует Claude Code v2.1.203 или позже.

<h2 id="remote-desktop-and-notifications">
  Удалённое управление, настольное приложение и уведомления
</h2>

Настройте удалённое управление, облачные среды, настольное приложение и уведомления, которые Claude Code отправляет, когда вам нужна помощь. См. [Удалённое управление](/docs/ru/remote-control).

<h3 id="agentpushnotifenabled">
  `agentPushNotifEnabled`
</h3>

Разрешить Claude отправлять push-уведомление на ваш телефон, когда он решит, что оно стоит отправить, например, когда завершится длительная задача. Claude Code синхронизирует этот выбор с вашей учётной записью, и уведомления приходят, пока [Удалённое управление](/docs/ru/remote-control) подключено. Отображается в `/config` как **Push when Claude decides**.

* **Scope**: [`Any file`](#scopes). Claude Code также читает значение, оставленное в `~/.claude.json` старыми версиями.
* **Type**: Boolean
  * `true`: Claude может отправлять push-уведомление на ваш телефон, когда он решит, что оно стоит отправить
  * `false`: Claude не отправляет эти уведомления
* **Default**: `false`

```json settings.json theme={null}
{
  "agentPushNotifEnabled": true
}
```

См. [Mobile push notifications](/docs/ru/remote-control#mobile-push-notifications).

<h3 id="awaysummaryenabled">
  `awaySummaryEnabled`
</h3>

Показывать однострочный краткий обзор сеанса, когда вы вернётесь в терминал после нескольких минут отсутствия. Установите значение `false` или отключите **Session recap** в `/config`, чтобы отключить краткий обзор.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: вы видите однострочный краткий обзор сеанса, когда вернётесь после нескольких минут отсутствия
  * `false`: Claude Code не показывает краткий обзор
* **Default**: не установлено, поэтому краткий обзор включён
* **Per-session overrides**: [`CLAUDE_CODE_ENABLE_AWAY_SUMMARY`](/docs/ru/env-vars) имеет приоритет над этим ключом для одного сеанса в любом направлении

```json settings.json theme={null}
{
  "awaySummaryEnabled": false
}
```

Claude Code никогда не показывает краткий обзор в неинтерактивном режиме.

<h3 id="disableartifact">
  `disableArtifact`
</h3>

<Warning>
  Устарело и заменено на [`enableArtifact`](#enableartifact). Claude Code по-прежнему учитывает `disableArtifact: true` как эквивалент `enableArtifact: false` и игнорирует `disableArtifact: false`.
</Warning>

Используйте [`enableArtifact`](#enableartifact) вместо этого, чтобы отключить инструмент [Artifact](/docs/ru/artifacts), который публикует выходные данные сеанса как приватную веб-страницу на claude.ai. Когда вы отключаете строку **Artifacts** в `/config`, Claude Code записывает `enableArtifact` в ваши пользовательские настройки и очищает этот ключ.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code отключает инструмент Artifact для каждого сеанса, к которому применяется файл, и ни один другой файл не включает его обратно. До версии 2.1.242 файл с более высоким приоритетом мог переопределить `true` файла с более низким приоритетом, вместо того чтобы ключ действовал как блокировка
  * `false`: игнорируется; чтобы оставить инструмент включённым, удалите ключ
* **Default**: не установлено, поэтому инструмент следует [доступности](/docs/ru/artifacts#availability) вашей учётной записи
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/ru/env-vars) установленный на `1` отключает инструмент для одного сеанса

```json settings.json theme={null}
{
  "disableArtifact": true
}
```

[Disable artifacts](/docs/ru/artifacts#disable-artifacts) перечисляет все способы отключения инструмента.

<h3 id="disabledeeplinkregistration">
  `disableDeepLinkRegistration`
</h3>

Остановить Claude Code от регистрации обработчика протокола `claude-cli://` в операционной системе, что он иначе делает после отправки первого запроса интерактивного сеанса. [Deep links](/docs/ru/deep-links) позволяют внешним инструментам открывать сеанс Claude Code с предзаполненным запросом. Установите это в окружениях, где регистрация обработчика протокола ограничена или управляется отдельно.

* **Scope**: [`Any file`](#scopes)
* **Type**: строка `"disable"`
* **Default**: не установлено, поэтому Claude Code регистрирует обработчик

```json settings.json theme={null}
{
  "disableDeepLinkRegistration": "disable"
}
```

<h3 id="disabledesktoplocalsessions">
  `disableDesktopLocalSessions`
</h3>

Отключить сеансы Code, которые работают на устройстве в [настольном приложении](/docs/ru/desktop#local-sessions-on-managed-devices), для развёртываний, где разработчики должны работать на удалённых машинах через SSH. На вкладке Code окружение **Local** остаётся в раскрывающемся списке окружений, но отключено и не может быть выбрано, с подсказкой, что ваша организация его отключила; на Windows запись WSL отключена таким же образом, хотя то, работают ли сеансы WSL на управляемом устройстве вообще, [управляется отдельно](/docs/ru/admin-setup#wsl-sessions-in-claude-code-desktop). Новые сеансы по умолчанию используют первое [SSH-соединение](/docs/ru/desktop#ssh-sessions), если оно настроено, и приложение отказывается запускать или возобновлять сеанс на устройстве, включая SSH-соединение обратно на ту же машину. SSH-сеансы на другие хосты и облачные сеансы не затронуты. Настольное приложение читает этот ключ; терминальный CLI его игнорирует. Требуется Claude Desktop v1.37937.0 или позже.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean; только JSON Boolean `true` имеет эффект
  * `true`: настольное приложение не предлагает сеансы Code на устройстве; существующие локальные сеансы остаются в списке, но не могут продолжаться
  * `false`: локальные сеансы остаются доступными
* **Default**: не установлено, поэтому локальные сеансы доступны

```json managed-settings.json theme={null}
{
  "disableDesktopLocalSessions": true
}
```

Настольное приложение игнорирует любое другое значение, и значение, которое не является Boolean, такое как строка `"true"` или `1`, также регистрирует предупреждение. Используйте его вместе с [`sshConfigs`](#sshconfigs), чтобы пользователи попадали на рабочее соединение, и с [`sshHostAllowlist`](#sshhostallowlist), чтобы ограничить, какие хосты они могут достичь. См. [Local sessions on managed devices](/docs/ru/desktop#local-sessions-on-managed-devices).

Claude Desktop предоставляет сеансы Code с политикой, полученной из конфигурации вашего настольного приложения, например список разрешённых исходящих соединений, изоляция файловой системы и ограничения MCP в развёртываниях третьих сторон. Claude Code игнорирует эти родительские настройки всякий раз, когда присутствует [источник администратора](/docs/ru/managed-settings#how-claude-code-combines-managed-sources): управляемые сервером настройки, политика MDM или уровня ОС, или файл управляемых настроек. Развёртывание этого ключа через один из них на устройстве, которое не имело ни одного раньше, как в развёртываниях третьих сторон, поэтому останавливает применение политик, полученных из настольного приложения. [Let an embedding host add policy](/docs/ru/managed-settings#let-an-embedding-host-add-policy) охватывает, когда родительские настройки всё ещё могут объединяться; это применяется к любому ключу, который вы развёртываете таким образом, не только к этому.

<h3 id="disableremotecontrol">
  `disableRemoteControl`
</h3>

Отключить [Удалённое управление](/docs/ru/remote-control): Claude Code затем отказывает в `claude remote-control`, флаге `--remote-control`, автозапуске и переключателе в сеансе, и сообщает, что политика вашей организации это отключила. Поместите это в [управляемые настройки](/docs/ru/managed-settings) для принудительного применения MDM на каждом устройстве.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code отказывает в `claude remote-control`, флаге `--remote-control`, автозапуске и переключателе в сеансе
  * `false`: Удалённое управление остаётся доступным
* **Default**: `false`

```json settings.json theme={null}
{
  "disableRemoteControl": true
}
```

<h3 id="enableartifact">
  `enableArtifact`
</h3>

Отключить инструмент [Artifact](/docs/ru/artifacts), который публикует выходные данные сеанса как приватную веб-страницу на claude.ai. Когда вы отключаете строку **Artifacts** в `/config`, Claude Code записывает этот ключ в ваши пользовательские настройки, поэтому вы обычно не редактируете его вручную. Требуется Claude Code v2.1.196 или позже.

* **Scope**: [`Any file`](#scopes). Каждый файл может отключить инструмент, и ни один не может включить его обратно.
* **Type**: Boolean
  * `false`: Claude Code отключает инструмент Artifact для каждого сеанса, к которому применяется файл
  * `true`: то же самое, что оставить ключ не установленным, потому что он никогда не переопределяет `false` из другого файла, из [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/ru/env-vars) или из [параметра администратора](/docs/ru/artifacts#manage-artifacts-for-your-organization) вашей организации
* **Default**: не установлено, поэтому инструмент следует [доступности](/docs/ru/artifacts#availability) вашей учётной записи

```json settings.json theme={null}
{
  "enableArtifact": false
}
```

Пока источник, отличный от ваших собственных пользовательских настроек, держит инструмент отключённым, Claude Code скрывает строку **Artifacts** в `/config`, потому что включение её там не изменит ничего. [Disable artifacts](/docs/ru/artifacts#disable-artifacts) перечисляет все способы отключения инструмента. До версии 2.1.242 Claude Code игнорировал этот ключ в проектных и локальных настройках, и файл выше в [стеке приоритета](/docs/ru/settings#settings-precedence) мог включить инструмент обратно над отключением файла ниже.

<h3 id="inputneedednotifenabled">
  `inputNeededNotifEnabled`
</h3>

Получить push-уведомление на ваш телефон, когда запрос разрешения или вопрос ожидает вашего ввода. Claude Code отправляет эти уведомления только пока [Удалённое управление](/docs/ru/remote-control) подключено. Отображается в `/config` как **Push when actions required**.

* **Scope**: [`Any file`](#scopes). Claude Code также читает значение, оставленное в `~/.claude.json` старыми версиями.
* **Type**: Boolean
  * `true`: вы получаете push-уведомление на ваш телефон, когда запрос разрешения или вопрос ожидает, пока подключено Удалённое управление
  * `false`: Claude Code не отправляет такие уведомления
* **Default**: `false`

```json settings.json theme={null}
{
  "inputNeededNotifEnabled": true
}
```

См. [Mobile push notifications](/docs/ru/remote-control#mobile-push-notifications).

<h3 id="preferrednotifchannel">
  `preferredNotifChannel`
</h3>

Выберите, как Claude Code вас уведомляет, когда задача завершена или запрос разрешения ожидает. Отображается в `/config` как **Local notifications**.

* **Scope**: [`Any file`](#scopes). Claude Code также читает значение, оставленное в `~/.claude.json` старыми версиями.
* **Type**: строка, одна из:
  * `"auto"`: Claude Code отправляет уведомление рабочего стола в iTerm2, Ghostty и Kitty, звонит в колокол в Terminal.app только когда его звуковой колокол отключён, и ничего не делает в других местах
  * `"terminal_bell"`: Claude Code звонит в символ колокола в любом терминале
  * `"iterm2"`: Claude Code отправляет уведомление рабочего стола iTerm2
  * `"iterm2_with_bell"`: Claude Code отправляет уведомление рабочего стола iTerm2 и звонит в колокол
  * `"kitty"`: Claude Code отправляет уведомление рабочего стола Kitty
  * `"ghostty"`: Claude Code отправляет уведомление рабочего стола Ghostty
  * `"notifications_disabled"`: Claude Code не отправляет уведомление
* **Default**: `"auto"`

```json settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

С `"auto"` Claude Code отправляет уведомление рабочего стола в iTerm2, Ghostty и Kitty. В Terminal.app он звонит в символ колокола только когда вы отключили звуковой колокол Terminal, и в других терминалах ничего не делает. Установите `"terminal_bell"`, чтобы звонить в символ колокола в любом терминале. См. [Get a terminal bell or notification](/docs/ru/terminal-config#get-a-terminal-bell-or-notification).

<h3 id="remote-defaultenvironmentid">
  `remote.defaultEnvironmentId`
</h3>

Выберите [облачное окружение](/docs/ru/cloud-environments) по умолчанию для облачных сеансов, которые вы создаёте из CLI, например с `claude --cloud`. Claude Code записывает этот ключ в ваши пользовательские настройки, когда вы выбираете окружение с помощью [`/remote-env`](/docs/ru/cloud-environments#select-an-environment-from-the-cli).

* **Scope**: [`Any file`](#scopes). Для ID самостоятельно размещённого окружения, пользовательских или управляемых настроек, или флага `--settings` только.
* **Type**: строка, ID окружения, такой как `env_...` или `ccpool_...`
* **Default**: не установлено, поэтому Claude Code использует размещённое Anthropic окружение, когда ваш список имеет одно, и в противном случае первое окружение в вашем списке, которое не является [окружением моста Удалённого управления](/docs/ru/cloud-environments#the-default-environment), или первое окружение, когда каждое является окружением моста
* **Per-session overrides**: `--environment` имеет приоритет над этим ключом для одного облачного сеанса, который он создаёт

```json settings.json theme={null}
{
  "remote": {
    "defaultEnvironmentId": "env_0123abcd"
  }
}
```

ID размещённого Anthropic окружения, который начинается с `env_`, следует стандартному приоритету настроек, поэтому значение в проектных настройках репозитория переопределяет ваш выбор на уровне пользователя. ID [самостоятельно размещённого окружения](/docs/ru/self-hosted-environments), который начинается с `ccpool_`, учитывается только из пользовательских настроек, управляемых настроек и флага `--settings`; Claude Code игнорирует один в проектных или локальных настройках репозитория, и `/remote-env` показывает, какое значение он игнорировал, поэтому проверенный файл не может направить сеансы на самостоятельно размещённое окружение, которое вы не выбрали.

<h3 id="remotecontrolatstartup">
  `remoteControlAtStartup`
</h3>

Подключить [Удалённое управление](/docs/ru/remote-control) автоматически, когда начинается каждый интерактивный сеанс, вместо ожидания `/remote-control`. Установите значение `true`, чтобы включить автоподключение, `false`, чтобы отключить его. Отображается в `/config` как **Enable Remote Control for all sessions**.

* **Scope**: [`Any file`](#scopes). Claude Code также читает значение, оставленное в `~/.claude.json` старыми версиями.
* **Type**: Boolean
  * `true`: Claude Code подключает Удалённое управление автоматически, когда начинается каждый интерактивный сеанс
  * `false`: Claude Code ждёт `/remote-control`
* **Default**: не установлено, поэтому автоподключение следует стандартному значению администратора вашей организации, когда оно установлено, и в противном случае текущему стандартному значению Claude Code
* **Per-session overrides**: `--remote-control` включает Удалённое управление для одного сеанса даже когда этот ключ `false`, и ни один флаг не отключает его для одного сеанса

```json settings.json theme={null}
{
  "remoteControlAtStartup": true
}
```

Claude Code игнорирует `true` из проектных или локальных настроек, поэтому репозиторий может отключить автоподключение для его checkout, но не может включить его. Для полного поведения для каждой области видимости см. [Enable Remote Control for all sessions](/docs/ru/remote-control#enable-remote-control-for-all-sessions) и [ключи безопасности, где применяется более строгое значение](/docs/ru/settings#security-keys-where-the-stricter-value-applies).

<h3 id="sshconfigs">
  `sshConfigs`
</h3>

Добавить SSH-соединения в раскрывающийся список окружения [Desktop](/docs/ru/desktop#pre-configure-ssh-connections-for-your-team). Администраторы используют это для распределения общих соединений команде. Соединения, которые вы определяете в управляемых настройках, отображаются как управляемые, поэтому пользователи могут их выбирать, но не могут редактировать или удалять их в приложении.

* **Scope**: [`User or managed`](#scopes). Настольное приложение читает этот ключ.
* **Type**: массив объектов, каждый с обязательными `id`, `name` и `sshHost` и опциональными `sshPort` и `sshIdentityFile`
* **Default**: не установлено

Этот пример добавляет одно соединение с именем `Dev VM`, которое подключается к `user@dev.example.com`:

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

Ограничить хосты, к которым может подключиться [SSH-сеанс Desktop](/docs/ru/desktop#restrict-which-ssh-hosts-users-can-connect-to). Только приложение Desktop читает этот ключ; CLI не читает. Шаблоны нечувствительны к регистру: `*` соответствует любому хосту, `*.example.com` соответствует `example.com` и каждому поддомену, и всё остальное является точным совпадением с именем хоста после разрешения `~/.ssh/config`. Пустой массив отключает SSH-сеансы.

* **Scope**: [`Managed`](#scopes)
* **Type**: массив шаблонов имён хостов
* **Default**: не установлено, поэтому разрешены любые хосты

Этот пример разрешает `devboxes.example.com` и его поддомены, плюс точный хост `bastion.example.com`:

```json managed-settings.json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

<span id="authentication-and-login" />

<h2 id="authentication-and-providers">
  Аутентификация и поставщики
</h2>

Предоставляйте учетные данные через вспомогательные скрипты и, для организаций, принудительно установите метод входа или организацию. См. [Аутентификация](/docs/ru/authentication).

<h3 id="apikeyhelper">
  `apiKeyHelper`
</h3>

Запустите собственную команду для создания учетных данных, которые Claude Code отправляет с запросами модели. Claude Code запускает команду через системную оболочку, `/bin/sh` на macOS и Linux и `cmd` на Windows, и отправляет её вывод как в заголовках `X-Api-Key`, так и в `Authorization: Bearer`. Используйте это для динамических или ротирующихся учетных данных, таких как краткосрочные токены, полученные из хранилища.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, a shell command line
* **Default**: unset, so Claude Code doesn't run a helper

```json settings.json theme={null}
{
  "apiKeyHelper": "/bin/generate_temp_api_key.sh"
}
```

Claude Code кэширует значение и повторно запускает команду в следующих случаях:

* После истечения времени жизни кэша, пять минут по умолчанию или интервал, который вы установили с помощью [`CLAUDE_CODE_API_KEY_HELPER_TTL_MS`](/docs/ru/env-vars).
* Когда запрос к API Anthropic, напрямую или через [LLM gateway](/docs/ru/llm-gateway), завершается с ошибкой `401` или `403`.
* Перед отправкой запроса к API Anthropic, напрямую или через LLM gateway, когда кэшированный вывод — это JWT, который истек после того, как помощник его создал. Требуется Claude Code v2.1.246 или позже.

Последние два случая применяются только когда вывод помощника — это учетные данные, которые Claude Code отправляет, и `ANTHROPIC_AUTH_TOKEN` не установлен.

В интерактивных сеансах, когда команда поступает из параметров проекта или локальных параметров, Claude Code не запускает её до тех пор, пока вы не примете приглашение доверия рабочей области. См. [Управление учетными данными](/docs/ru/authentication#credential-management).

<h3 id="awsauthrefresh">
  `awsAuthRefresh`
</h3>

Запустите собственную команду, такую как `aws sso login`, для обновления учетных данных в вашем каталоге `.aws` когда те, которые Claude Code имеет для [Amazon Bedrock](/docs/ru/amazon-bedrock), перестают работать. Claude Code сначала проверяет текущие учетные данные против STS и запускает команду только когда эта проверка не удается, затем читает обновленный каталог `.aws`.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, a shell command line
* **Default**: unset, so Claude Code doesn't refresh AWS credentials for you

```json settings.json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile"
}
```

Используйте этот ключ, когда ваш поток обновления записывает в `.aws`; используйте [`awsCredentialExport`](#awscredentialexport) когда он выводит учетные данные вместо этого. См. [расширенную конфигурацию учетных данных](/docs/ru/amazon-bedrock#advanced-credential-configuration).

<h3 id="awscredentialexport">
  `awsCredentialExport`
</h3>

Запустите собственную команду, которая выводит учетные данные AWS в формате JSON, чтобы Claude Code мог вызывать [Amazon Bedrock](/docs/ru/amazon-bedrock) с учетными данными, которые не находятся в вашем каталоге `.aws`. Claude Code принимает форму вывода `aws sts` и плоскую форму `aws configure export-credentials`, и ограничивает учетные данные своим собственным клиентом Bedrock, поэтому команды оболочки, которые запускает Claude Code, по-прежнему видят ваши окружающие учетные данные.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, a shell command line
* **Default**: unset, so Claude Code uses the ambient AWS credential chain

```json settings.json theme={null}
{
  "awsCredentialExport": "/bin/generate_aws_grant.sh"
}
```

В отличие от [`awsAuthRefresh`](#awsauthrefresh), Claude Code всегда запускает эту команду, когда она установлена, без предварительной проверки окружающих учетных данных. См. [расширенную конфигурацию учетных данных](/docs/ru/amazon-bedrock#advanced-credential-configuration).

<h3 id="forceloginmethod">
  `forceLoginMethod`
</h3>

Ограничьте, какой тип учетной записи люди могут использовать для входа. Установите `"claudeai"` чтобы разрешить только учетные записи claude.ai, `"console"` чтобы разрешить только учетные записи Claude Console, или `"gateway"` чтобы отправить людей на [облачный шлюз](/docs/ru/claude-apps-gateway) вместо входа первой стороны. Администраторы устанавливают это в управляемых параметрах и связывают это с [`forceLoginOrgUUID`](#forceloginorguuid) чтобы держать входы claude.ai разработчиков внутри одной организации. Если вы установите это на `"claudeai"` или `"console"` в любом файле параметров, Claude Code также перестает предлагать [вход в Console без ключа](/docs/ru/authentication#sign-in-without-an-api-key) в сеансах, к которым применяется этот файл.

* **Scope**: [`Any file`](#scopes). Claude Code соблюдает `"gateway"` только из управляемого источника на машине: `managed-settings.json`, plist macOS или реестр Windows HKLM, или помощник политики. Он рассматривает `"gateway"` как неустановленный в пользовательских, проектных, локальных, HKCU и управляемых сервером параметрах, то же правило, что и [`forceLoginGatewayUrl`](#forcelogingatewayurl).
* **Type**: string, one of:
  * `"claudeai"`: only claude.ai accounts can log in
  * `"console"`: only Claude Console accounts can log in
  * `"gateway"`: Claude Code sends people to a cloud gateway instead of a first-party login
* **Default**: unset, so people pick a login method

```json settings.json theme={null}
{
  "forceLoginMethod": "claudeai"
}
```

Каждый путь входа первой стороны применяет ограничение, включая [расширение VS Code](/docs/ru/vs-code), Agent SDK, `claude setup-token` и `/install-github-app`, за исключением интерактивного экрана входа терминала, доступного через `/login` или первоначальную настройку, который предварительно выбирает метод без его принудительного применения. До v2.1.212 только входы терминала применяли это. См. [Ограничить вход в вашу организацию](/docs/ru/authentication#restrict-login-to-your-organization) для того, как каждый путь входа, учетные данные окружения и поставщики третьих сторон обрабатываются.

Когда управляемый источник на машине устанавливает `"gateway"`, Claude Code не использует оставшийся вход, API ключ или учетные данные `apiKeyHelper`. См. [Политика администратора требует вход через облачный шлюз](/docs/ru/errors#administrator-policy-requires-a-cloud-gateway-sign-in) для сообщения, которое каждый из них выдает. Если вы выбираете облачного поставщика через `CLAUDE_CODE_USE_BEDROCK` или аналогичную переменную окружения, сеанс не требует входа через шлюз. До v2.1.261 Claude Code использовал оставшийся вход на этих машинах.

<h3 id="forcelogingatewayurl">
  `forceLoginGatewayUrl`
</h3>

Установите URL шлюза, к которому подключается экран `/login` облачного шлюза, чтобы люди достигли вашего [облачного шлюза](/docs/ru/claude-apps-gateway) без ввода его адреса. На экране нет поля URL: с установленным этим ключом он показывает URL вашего шлюза и подключается, когда человек нажимает Enter; без него он говорит им связаться с администратором IT.

Либо этот ключ, либо `forceLoginMethod: "gateway"` делает машину только шлюзом, поэтому `/login` открывается на экране облачного шлюза без средства выбора метода входа. См. [Политика администратора требует вход через облачный шлюз](/docs/ru/errors#administrator-policy-requires-a-cloud-gateway-sign-in) для того, что происходит с оставшимся входом первой стороны или API ключом. Установите оба ключа, чтобы экран подключился вместо показа ошибки.

* **Scope**: [`Managed`](#scopes). Read only from a source on the machine: `managed-settings.json`, the macOS plist or Windows HKLM registry, or a policy helper. Claude Code ignores it in HKCU and server-managed settings.
* **Type**: string, a full URL including the scheme
* **Default**: unset, so the Cloud gateway screen shows an error telling people to contact their IT administrator

```json managed-settings.json theme={null}
{
  "forceLoginGatewayUrl": "https://claude-gateway.example.com"
}
```

Если значение не является действительным URL, экран входа сообщает об этом, и остальная часть файла управляемых параметров по-прежнему применяется. См. [Установить URL шлюза](/docs/ru/claude-apps-gateway#set-the-gateway-url).

<h3 id="forceloginorguuid">
  `forceLoginOrgUUID`
</h3>

Из управляемого источника требуйте, чтобы входы учетной записи claude.ai принадлежали одной организации Anthropic, указанной как один UUID, или любой из нескольких организаций, указанных как массив. Из любого файла параметров Claude Code также использует один UUID для предварительного выбора этой организации во время входа claude.ai или Claude Console, и не предварительно выбирает ничего для массива. Если вы установите ключ в любом файле параметров, Claude Code также перестает предлагать [вход в Console без ключа](/docs/ru/authentication#sign-in-without-an-api-key) в сеансах, к которым применяется этот файл, и вместо этого создает API ключ.

* **Scope**: [`Any file`](#scopes). Only a managed source enforces the restriction; a single UUID in any other settings file pre-selects the organization during login without restricting it.
* **Type**: string, one UUID, or array of strings, several UUIDs
* **Default**: unset, so any organization can log in

Этот пример принимает входы из любой из двух организаций без предварительного выбора одной:

```json managed-settings.json theme={null}
{
  "forceLoginOrgUUID": ["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", "yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy"]
}
```

Если управляемый источник устанавливает пустой массив или значение, которое Claude Code не может разобрать, Claude Code блокирует каждый вход с сообщением о неправильной конфигурации.

См. [Ограничить вход в вашу организацию](/docs/ru/authentication#restrict-login-to-your-organization) для того, как Claude Code обрабатывает входы Claude Console, другие пути входа и учетные данные окружения.

<h3 id="gatewayinternalnetworks">
  `gatewayInternalNetworks`
</h3>

Объявите публичные блоки IPv4, из которых ваша организация нумерует свою внутреннюю сеть, чтобы `/login` принимал [облачный шлюз](/docs/ru/claude-apps-gateway) там. Требуется Claude Code v2.1.268 или позже.

Без этого ключа `/login` подключается к любому шлюзу на приватном адресе и ничему больше. С ним `/login` также принимает шлюз внутри указанного блока, только через прямое соединение. Собственный адрес машины на этом соединении также должен быть внутри того же блока.

* **Scope**: [`Managed`](#scopes). Read only from a source on the machine: `managed-settings.json`, the macOS plist or Windows HKLM registry, or a policy helper. Claude Code ignores it in HKCU and server-managed settings.
* **Type**: array of strings, at most four IPv4 CIDR blocks, each `/8` to `/32`, not overlapping one another, and none overlapping private space.
* **Default**: unset, so `/login` accepts only gateways on private addresses

```json managed-settings.json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

Замените диапазон документации в примере на ваш собственный блок. Claude Code отказывает диапазонам документации, диапазонам, которые используют локально клиенты VPN и NAT64, и зарезервированному пространству, из которого не нумеруется ни одна сеть, такому как многоадресная рассылка.

Если запись недействительна или значение не является списком строк, `/login` называет проблему и отказывает каждому новому входу через облачный шлюз на машине до тех пор, пока вы не исправите значение. Существующие входы продолжают работать. См. [Разрешить шлюз на адресном пространстве общего пользования, которым вы владеете](/docs/ru/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) для полных правил и того, что видят разработчики.

<h3 id="gcpauthrefresh">
  `gcpAuthRefresh`
</h3>

Запустите собственную команду для обновления Google Cloud Application Default Credentials когда Claude Code обнаруживает, что они истекли или не могут быть загружены, чтобы запросы [Google Cloud's Agent Platform](/docs/ru/google-vertex-ai) продолжали работать без повторной аутентификации вручную.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, a shell command line
* **Default**: unset, so Claude Code's credential error tells you to run `gcloud auth application-default login` yourself

```json settings.json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login"
}
```

См. [расширенную конфигурацию учетных данных](/docs/ru/google-vertex-ai#advanced-credential-configuration).

<h3 id="otelheadershelper">
  `otelHeadersHelper`
</h3>

Запустите собственную команду для создания заголовков, которые Claude Code отправляет с экспортами OpenTelemetry, для бэкендов, чьи токены ротируются. Claude Code запускает её при запуске и периодически после этого, и ожидает объект JSON значений строковых заголовков на stdout.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, an executable path or a shell command line
* **Default**: unset, so Claude Code adds no helper-generated headers

```json settings.json theme={null}
{
  "otelHeadersHelper": "/bin/generate_otel_headers.sh"
}
```

Установите интервал обновления с помощью [`CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`](/docs/ru/env-vars). См. [Динамические заголовки](/docs/ru/monitoring-usage#dynamic-headers) для требований скрипта и где Claude Code сообщает о неудачном помощнике.

<h2 id="updates-and-versioning">
  Обновления и версионирование
</h2>

Выберите канал обновления и, для организаций, закрепите версии, которые люди могут запускать. См. [Обновление Claude Code](/docs/ru/setup#update-claude-code).

<h3 id="autoupdateschannel">
  `autoUpdatesChannel`
</h3>

Выберите, какой [канал выпуска](/docs/ru/setup#configure-release-channel) следуют фоновые автоматические обновления и `claude update`. Установите `"stable"` для версии, которая обычно имеет возраст около одной недели и пропускает выпуски с серьёзными регрессиями, или `"latest"` для самого последнего выпуска.

* **Область действия**: [`Any file`](#scopes). Установите это в управляемых параметрах, чтобы применить один канал во всей организации.
* **Тип**: строка, одно из:
  * `"latest"`: обновления следуют самому последнему выпуску
  * `"stable"`: обновления следуют версии, которая обычно имеет возраст около одной недели и пропускает выпуски с серьёзными регрессиями
* **По умолчанию**: не установлено, поэтому Claude Code следует `"latest"`

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

Claude Code записывает `"stable"` в ваши пользовательские параметры, когда вы выбираете это в разделе **Auto-update channel** в `/config`, и удаляет ключ, когда вы переключаетесь обратно на latest там. `claude install stable` и `claude install latest` также сохраняют выбранный вами канал. Переключение с `"latest"` на `"stable"` в `/config` спрашивает, разрешить ли понижение версии или остаться на текущей версии; остаток устанавливает [`minimumVersion`](#minimumversion). Установки Homebrew игнорируют этот ключ: пакет `claude-code` отслеживает stable, а `claude-code@latest` отслеживает latest, и `claude update` полагается на `brew upgrade`. Чтобы полностью отключить автоматические обновления, установите [`DISABLE_AUTOUPDATER`](/docs/ru/setup#disable-auto-updates) в `env`.

<h3 id="minimumversion">
  `minimumVersion`
</h3>

Предотвратите фоновые автоматические обновления и `claude update` от установки любой версии ниже этой, поэтому переход на канал `"stable"` не понизит вас с более новой сборки `"latest"`. Claude Code записывает этот ключ для вас, когда вы выбираете остаться на текущей версии при переключении каналов в `/config`, и очищает его, когда вы переключаетесь обратно на `"latest"`.

* **Область действия**: [`Any file`](#scopes). Установите это в управляемых параметрах, чтобы закрепить минимум на уровне организации, который пользовательские и проектные параметры не могут снизить.
* **Тип**: строка, номер версии, такой как `"2.1.100"`; значение, которое не является допустимой версией, игнорируется
* **По умолчанию**: не установлено, поэтому обновления могут установить любую версию, которую предлагает канал

Этот пример следует стабильному каналу и отказывается устанавливать любую версию ниже 2.1.100:

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

Этот ключ только ограничивает обновления. Чтобы заставить Claude Code отказаться запускаться ниже версии, используйте вместо этого [`requiredMinimumVersion`](#requiredminimumversion). См. [Закрепить минимальную версию](/docs/ru/setup#pin-a-minimum-version).

<h3 id="requiredmaximumversion">
  `requiredMaximumVersion`
</h3>

Установите самую новую версию Claude Code, которую разрешает запускать ваша организация. Когда запущенная версия новее, Claude Code выходит при запуске и сообщает пользователю установить одобренную версию через утверждённый вашей организацией метод; `claude install <version>` также может работать. Требует Claude Code v2.1.163 или позже.

* **Область действия**: [`Managed`](#scopes). Claude Code не выдаёт предупреждение, когда игнорирует ключ в других местах.
* **Тип**: строка, номер версии, такой как `"2.1.150"`; значение, которое не является допустимой версией, игнорируется
* **По умолчанию**: не установлено, поэтому потолок не применяется

```json managed-settings.json theme={null}
{
  "requiredMaximumVersion": "2.1.150"
}
```

Фоновые автоматические обновления и `claude update` пропускают версии выше потолка, поэтому установка внутри диапазона остаётся внутри него. `claude update`, `claude install` и `claude doctor` продолжают работать выше потолка, чтобы пользователи могли восстановиться. Объедините это с [`requiredMinimumVersion`](#requiredminimumversion), чтобы применить диапазон.

<h3 id="requiredminimumversion">
  `requiredMinimumVersion`
</h3>

Установите самую старую версию Claude Code, которую разрешает запускать ваша организация. Когда запущенная версия старше, Claude Code выходит при запуске и сообщает пользователю обновиться через утверждённый вашей организацией метод. Проверка выполняется только при запуске, поэтому уже запущенный сеанс продолжает работу. Требует Claude Code v2.1.163 или позже.

* **Область действия**: [`Managed`](#scopes). Claude Code не выдаёт предупреждение, когда игнорирует ключ в других местах.
* **Тип**: строка, номер версии, такой как `"2.1.150"`; значение, которое не является допустимой версией, игнорируется
* **По умолчанию**: не установлено, поэтому пол не применяется

```json managed-settings.json theme={null}
{
  "requiredMinimumVersion": "2.1.150"
}
```

`claude update`, `claude install` и `claude doctor` продолжают работать ниже пола, чтобы пользователи могли восстановиться. В отличие от [`minimumVersion`](#minimumversion), который только предотвращает понижение версии, этот ключ блокирует запуск. Объедините это с [`requiredMaximumVersion`](#requiredmaximumversion), чтобы применить диапазон.

<h2 id="tools">
  Tools
</h2>

Отключите определённые инструменты в [приложении Claude Code для рабочего стола](/docs/ru/desktop). Терминальный CLI игнорирует эти ключи. Для самих инструментов см. [Инструменты, доступные Claude](/docs/ru/tools-reference).

<h3 id="browserexternalpagetools">
  `browserExternalPageTools`
</h3>

Запретите Claude использовать свои инструменты для чтения или действия на внешних страницах в [панели Browser](/docs/ru/desktop#browse-external-sites) приложения для рабочего стола. Люди в вашей организации по-прежнему могут открывать внешние сайты самостоятельно, а локальные предпросмотры dev-сервера продолжают работать с инструментами Claude. Приложение для рабочего стола читает этот ключ; терминальный CLI игнорирует его.

* **Scope**: [`Managed`](#scopes)
* **Type**: string, `"disabled"`; приложение для рабочего стола также принимает `"disable"`, в любом случае
* **Default**: не установлено, поэтому инструменты Claude работают на внешних страницах

```json managed-settings.json theme={null}
{
  "browserExternalPageTools": "disabled"
}
```

Любое другое значение оставляет инструменты Claude включёнными, и непустая строка, которая не является одним из двух принятых значений, регистрирует предупреждение. Чтобы заблокировать внешние сайты для людей и Claude одновременно, установите [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation) вместо этого. См. [Ограничить внешний просмотр для вашей организации](/docs/ru/desktop#restrict-external-browsing-for-your-organization).

<h3 id="disablebrowserexternalnavigation">
  `disableBrowserExternalNavigation`
</h3>

Отключите внешний просмотр в [панели Browser](/docs/ru/desktop#browse-external-sites) приложения для рабочего стола для людей и Claude одновременно. Предпросмотры localhost dev-сервера продолжают работать. Приложение для рабочего стола читает этот ключ; терминальный CLI игнорирует его.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean; только JSON Boolean `true` вступает в силу
  * `true`: приложение для рабочего стола отключает внешний просмотр в панели Browser для людей и Claude одновременно; предпросмотры localhost продолжают работать
  * `false`: внешний просмотр остаётся включённым
* **Default**: не установлено, поэтому внешний просмотр включён

```json managed-settings.json theme={null}
{
  "disableBrowserExternalNavigation": true
}
```

Приложение для рабочего стола игнорирует любое другое значение, и значение, которое не является Boolean, такое как строка `"true"` или `1`, также регистрирует предупреждение. Чтобы оставить внешний просмотр включённым, но отключить инструменты Claude на внешних страницах, установите [`browserExternalPageTools`](#browserexternalpagetools) вместо этого. См. [Ограничить внешний просмотр для вашей организации](/docs/ru/desktop#restrict-external-browsing-for-your-organization).

<h3 id="disablemobilesimulatortools">
  `disableMobileSimulatorTools`
</h3>

Заблокируйте инструменты Claude для [панели iOS Simulator](/docs/ru/desktop-ios-simulator#turn-off-simulator-access) приложения для рабочего стола. Люди сохраняют ручное использование панели; удаляется только доступ Claude, и никто не может включить его обратно из приложения. Приложение для рабочего стола читает этот ключ; терминальный CLI игнорирует его.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean; только JSON Boolean `true` вступает в силу
  * `true`: приложение для рабочего стола блокирует инструменты Claude для панели iOS Simulator
  * `false`: инструменты симулятора Claude следуют переключателю настроек каждого человека в приложении для рабочего стола
* **Default**: не установлено, поэтому инструменты симулятора Claude следуют переключателю настроек каждого человека в приложении для рабочего стола

```json managed-settings.json theme={null}
{
  "disableMobileSimulatorTools": true
}
```

Приложение для рабочего стола игнорирует любое другое значение, и значение, которое не является Boolean, такое как строка `"true"` или `1`, также регистрирует предупреждение.

<span id="data-and-privacy" />

<h2 id="privacy-and-telemetry">
  Конфиденциальность и телеметрия
</h2>

Контролируйте, как долго Claude Code хранит данные сеанса и что он отправляет. Переключатели, которые отключают метрики использования и отчеты об ошибках, являются переменными окружения, а не ключами параметров: установите `DISABLE_TELEMETRY`, `DISABLE_ERROR_REPORTING` или `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` в ключе [`env`](#env) или в оболочке. [Услуги телеметрии](/docs/ru/data-usage#telemetry-services) описывает, что отключает каждый из них. Два исключения отключаются из файла параметров: [`feedbackDrafts`](#feedbackdrafts) ниже для отзывов, составленных Claude, и [`feedbackSurveyRate`](#feedbacksurveyrate) ниже для опроса сеанса.

<h3 id="cleanupperioddays">
  `cleanupPeriodDays`
</h3>

Установите, сколько дней Claude Code хранит [стенограммы сеансов и другие данные приложения](/docs/ru/claude-directory#cleaned-up-automatically) перед их удалением. Claude Code выполняет удаление как фоновую очистку после запуска сеанса, при условии, что он может безопасно определить период хранения.

* **Область**: [`Any file`](#scopes)
* **Тип**: количество дней, целое число, минимум `1`
* **По умолчанию**: `30`

```json settings.json theme={null}
{
  "cleanupPeriodDays": 20
}
```

Установка `0` не пройдет проверку, поэтому выберите большое значение, например `3650` для длительного хранения. Чтобы остановить Claude Code от записи стенограмм вообще, см. [Хранилище в виде простого текста](/docs/ru/claude-directory#plaintext-storage).

<h3 id="desktopsessioncleanupperioddays">
  `desktopSessionCleanupPeriodDays`
</h3>

Установите ограничение по возрасту в днях для стенограмм сеансов, которые вы запустили или недавно продолжили в Claude Desktop или Cowork. Без этого ключа Claude Code [хранит эти стенограммы в любом возрасте](/docs/ru/claude-directory#cleaned-up-automatically). Claude Code удаляет каждую из них, как только она становится старше как этого ограничения, так и [`cleanupPeriodDays`](#cleanupperioddays), поэтому при `cleanupPeriodDays` по умолчанию 30, значение `7` все еще хранит их 30 дней. Когда управляемые параметры устанавливают `cleanupPeriodDays`, этот период применяется вместо этого, и этот ключ игнорируется. Требуется Claude Code v2.1.248 или позже.

* **Область**: [`User or managed`](#scopes). Claude Code также читает ключ из файла, который вы передаете с `--settings`, и игнорирует его в параметрах проекта и локальных параметрах.
* **Тип**: количество дней, целое число, минимум `0`
* **По умолчанию**: `0`, что не устанавливает ограничение по возрасту

```json settings.json theme={null}
{
  "desktopSessionCleanupPeriodDays": 90
}
```

<h3 id="feedbackdrafts">
  `feedbackDrafts`
</h3>

Контролируйте [отзывы, составленные Claude](/docs/ru/tools-reference#sendfeedback-tool-behavior): может ли Claude ставить в очередь черновики отзывов для вашего рассмотрения и показывает ли Claude Code карточку, когда Claude ставит один в очередь.

* **Область**: [`User or managed`](#scopes)
* **Тип**: строка, одна из `"notify"`, `"quiet"` или `"off"`
  * `"notify"`: Claude Code показывает карточку над приглашением, когда Claude ставит черновик в очередь, до [трех карточек в сеансе](/docs/ru/tools-reference#what-you-see-when-claude-drafts) по умолчанию
  * `"quiet"`: Claude составляет черновики без карточки. Вы видите количество поставленных в очередь черновиков в нижнем колонтитуле приглашения и рассматриваете их в `/feedback`
  * `"off"`: Claude Code удаляет инструмент SendFeedback, поэтому Claude не может ставить черновики в очередь
* **По умолчанию**: `"notify"`
* **Переопределения для каждого сеанса**: [`CLAUDE_CODE_SEND_FEEDBACK`](/docs/ru/env-vars) установленный на `0` отключает функцию для одного сеанса

```json settings.json theme={null}
{
  "feedbackDrafts": "quiet"
}
```

Появляется в `/config` как **Claude-drafted feedback**, который записывает этот ключ в ваши пользовательские параметры. Вы видите строку `/config` только в сеансах [где Claude может составлять отзывы](/docs/ru/tools-reference#sessions-without-claude-drafted-feedback); установка `"off"` не скрывает его, поэтому вы можете снова включить функцию из той же строки. Значение в управляемых параметрах имеет приоритет над вашим пользовательским параметром, поэтому когда администратор устанавливает этот ключ, строка показывает управляемое значение, и его изменение не имеет эффекта. Claude Code игнорирует этот ключ в параметрах проекта и локальных параметрах.

<h3 id="feedbacksurveyrate">
  `feedbackSurveyRate`
</h3>

Установите вероятность того, что [опрос качества сеанса](/docs/ru/data-usage#session-quality-surveys) появится, когда сеанс имеет право на него. Установите `0`, чтобы опрос не появлялся.

* **Область**: [`Any file`](#scopes)
* **Тип**: число между `0` и `1`
* **По умолчанию**: не установлено, поэтому Claude Code использует скорость, которую Anthropic устанавливает удаленно, или встроенную скорость `0.005` на Amazon Bedrock, Google Cloud's Agent Platform и Microsoft Foundry, которые не получают удаленную конфигурацию
* **Переопределения для каждого сеанса**: [`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY`](/docs/ru/env-vars) установленный на `1` отключает опрос для одного сеанса независимо от скорости, которую устанавливает этот ключ

```json settings.json theme={null}
{
  "feedbackSurveyRate": 0.05
}
```

Та же скорость применяется к опросу в расширении VS Code.

<h3 id="skipwebfetchpreflight">
  `skipWebFetchPreflight`
</h3>

Пропустите [проверку безопасности домена WebFetch](/docs/ru/data-usage#webfetch-domain-safety-check), которая отправляет каждое запрашиваемое имя хоста на `api.anthropic.com` перед выборкой. Установите `true` в окружениях, которые блокируют трафик к Anthropic, таких как Amazon Bedrock, Google Cloud's Agent Platform или развертывания Microsoft Foundry с ограничивающим исходящим трафиком.

* **Область**: [`Any file`](#scopes)
* **Тип**: Boolean
  * `true`: Claude Code пропускает проверку безопасности домена WebFetch
  * `false`: проверка выполняется перед первой выборкой для каждого имени хоста в сеансе и снова для имени хоста, чья более ранняя проверка была заблокирована или не удалась
* **По умолчанию**: не установлено, поэтому проверка выполняется перед первой выборкой для каждого имени хоста в сеансе

```json settings.json theme={null}
{
  "skipWebFetchPreflight": true
}
```

С пропущенной проверкой WebFetch пытается использовать любой URL без консультации со списком блокировки, поэтому объедините его с [правилами разрешений `WebFetch`](/docs/ru/permissions#webfetch), если вам нужно ограничить, какие домены может достичь Claude.

<span id="managed-policy" />

<h2 id="enterprise-and-managed-settings">
  Корпоративные и управляемые параметры
</h2>

Ключи, которые организация использует для вычисления, обновления и объединения управляемых параметров. См. [Настройка управляемых параметров](/docs/ru/admin-setup).

<h3 id="disablesideloadflags">
  `disableSideloadFlags`
</h3>

Отклонять флаги CLI `--plugin-dir`, `--plugin-url`, `--agents` и `--mcp-config` при запуске, которые пользователи могут передать для обхода [`strictKnownMarketplaces`](#strictknownmarketplaces) при одном запуске. Claude Code завершает работу с ошибкой, указывающей на отклоненные флаги, и применяет ту же проверку к поверхностям, которые запускают CLI с этими флагами внутри, в настоящее время [Cowork](/docs/ru/desktop) локальные сеансы в приложении для рабочего стола. В [облачных сеансах](/docs/ru/claude-code-on-the-web) Claude Code удаляет MCP серверы, которые сервер доставил через `--mcp-config`, за исключением встроенных записей `type: "sdk"`, и запускает сеанс. Требуется Claude Code v2.1.193 или позже.

* **Область**: [`Managed`](#scopes)
* **Тип**: Boolean
  * `true`: Claude Code отклоняет `--plugin-dir`, `--plugin-url`, `--agents` и `--mcp-config` при запуске и завершает работу с ошибкой, указывающей на них, за исключением облачных сеансов, где он удаляет MCP серверы, которые сервер доставил через `--mcp-config`, за исключением встроенных записей `type: "sdk"`, и запускает сеанс
  * `false`: Claude Code принимает эти флаги
* **По умолчанию**: `false`

```json managed-settings.json theme={null}
{
  "disableSideloadFlags": true
}
```

Claude Code по-прежнему принимает `--mcp-config`, чьи серверы являются встроенными записями `type: "sdk"`, поэтому Agent SDK и расширение VS Code продолжают работать. Пользователи по-прежнему могут добавлять серверы с помощью `claude mcp add` или файла `.mcp.json`; для управления отдельными серверами также установите [`allowedMcpServers`](/docs/ru/managed-mcp). Требуется Claude Code v2.1.193 или позже.

Та же проверка охватывает папки плагинов, названные в переменной окружения [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/ru/env-vars#variables), что требует Claude Code v2.1.280 или позже. Когда переменная называет папку, Claude Code завершает работу с той же ошибкой, и ошибка говорит отменить установку переменной.

В облачных сеансах Claude Code также игнорирует обновления MCP, доставленные сервером в середине сеанса, путь позади конфигурации облачного сеанса и SDK `setMcpServers()` вызовов, которые достигают этих сеансов. Встроенные записи `type: "sdk"` остаются исключенными там же. До v2.1.239 доставленный сервером `--mcp-config` блокировал запуск облачного сеанса.

<h3 id="forceremotesettingsrefresh">
  `forceRemoteSettingsRefresh`
</h3>

Блокировать запуск CLI до тех пор, пока Claude Code не получит свежую выборку [управляемых параметров сервера](/docs/ru/server-managed-settings). Если выборка не удается, Claude Code завершает работу вместо продолжения с кэшированными или отсутствующими параметрами. Установите это, когда ваша среда не может принять даже краткое окно, в котором сеанс работает без своей управляемой политики.

Когда ключ не установлен, Claude Code не блокирует запуск при выборке, хотя когда разработчик входит при запуске, он ждет до пяти секунд для выборки. Сеанс облачного шлюза всегда ждет и завершает работу, если шлюз недоступен.

* **Область**: [`Managed`](#scopes). Claude Code соблюдает `true` из любого управляемого администратором источника, даже если это не источник с наивысшим приоритетом.
* **Тип**: Boolean
  * `true`: Claude Code блокирует запуск до тех пор, пока он не получит свежую выборку управляемых параметров сервера, и завершает работу, если выборка не удается
  * `false`: Claude Code не блокирует запуск при выборке, хотя при запуске входа он ждет до пяти секунд для выборки
* **По умолчанию**: `false`

```json managed-settings.json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Установите это в профиль MDM или файл управляемых параметров, чтобы обеспечить отказоустойчивый запуск до прибытия первого полезного груза сервера. Claude Code применяет проверку только в сеансах, которые получают управляемые параметры сервера, поэтому сеанс, который [их не получает](/docs/ru/server-managed-settings#platform-availability), запускается без ожидания. Подкоманды `claude auth` исключены, поэтому пользователи могут повторно аутентифицироваться, когда истекшие учетные данные являются причиной сбоя выборки. См. [Обеспечение отказоустойчивого запуска](/docs/ru/server-managed-settings#enforce-fail-closed-startup).

<h3 id="managedsourcesbehavior">
  `managedSourcesBehavior`
</h3>

Выберите, применяет ли Claude Code только источник с наивысшим приоритетом [управляемый источник](/docs/ru/managed-settings#how-claude-code-combines-managed-sources), который доставляет ваша организация, или объединяет каждый источник администратора, который она доставляет. По умолчанию Claude Code берет источник с наивысшим приоритетом, который содержит [ключ политики](/docs/ru/managed-settings#how-claude-code-combines-managed-sources), и игнорирует остальное. Ключ политики — это любой ключ параметров, кроме этого и `wslInheritsWindowsSettings`. Таким образом, как только управляемые параметры сервера или политика MDM доставляют ключ политики, файл `managed-settings.json` вносит только [ключи, которые Claude Code читает из каждого источника администратора](/docs/ru/managed-settings#keys-read-from-every-admin-source). С `"merge"` каждый источник администратора, который вы доставляете, вносит свои ключи в одну объединенную политику. Требуется Claude Code v2.1.242 или позже.

Установите `"merge"` только там, где каждый источник [ранжированный](/docs/ru/managed-settings#how-claude-code-combines-managed-sources) ниже вашего наивысшего находится под контролем администратора, потому что Claude Code затем добавляет записи из более низкого источника, такие как правила `permissions.allow`, в политику.

* **Область**: [`Managed`](#scopes). Claude Code читает этот ключ из источника с наивысшим приоритетом, который содержит либо этот ключ, либо ключ политики, и игнорирует этот ключ в каждом источнике, ранжированном ниже, поэтому более низкий источник не может выбрать себя для объединения с источником выше. Ни реестр Windows HKCU, ни [родительские параметры от хоста встраивания](/docs/ru/managed-settings#let-an-embedding-host-add-policy) не участвуют в объединении.
* **Тип**: string, один из:
  * `"first-wins"`: источник с наивысшим приоритетом, который содержит ключ политики, поставляет политику, и более низкие источники вносят только [ключи, которые Claude Code читает из каждого источника администратора](/docs/ru/managed-settings#keys-read-from-every-admin-source)
  * `"merge"`: каждый источник администратора, который вы доставляете, вносит свои ключи, объединенные по правилам ниже
* **По умолчанию**: `"first-wins"`

Доставьте ключ в источник с наивысшим приоритетом, который вы развертываете. Машина, которая никогда не получает управляемые параметры сервера, нуждается в ключе в своем профиле MDM, потому что Claude Code читает ключ из источника с наивысшим приоритетом, который его содержит или ключ политики. Файл `managed-settings.json` является источником администратора с наименьшим рангом, поэтому `"merge"`, установленный там, не имеет источника ниже для объединения. В управляемых параметрах сервера ключ выглядит так:

```json theme={null}
{
  "managedSourcesBehavior": "merge"
}
```

Под `"merge"` Claude Code объединяет каждый ключ по его типу. Эта таблица дает правило для каждого типа. Строки списка ограничений, значения-взятые-целиком и только-наивысший-источник называют каждый ключ, который они охватывают, и другие строки дают примеры:

| Тип ключа                                            | Как Claude Code его объединяет                                                                                                                                                                                                                                    | Ключи                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :--------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Списки                                               | Объединяет записи из каждого источника                                                                                                                                                                                                                            | [`permissions.allow`](#permissions-allow), [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) и другие ключи списков                                                                                                                                                                                                                                                                                                                                                                                      |
| Блокировки                                           | Применяет самое строгое значение, которое устанавливает любой источник. Когда ни один источник не устанавливает строгое значение, применяет более мягкое значение только из источника с наивысшим приоритетом                                                     | [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly), [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode) и другие логические или перечисляемые блокировки                                                                                                                                                                                                                                                                                                            |
| Списки ограничений                                   | Берет список целиком из источника с наивысшим приоритетом, который его устанавливает, без добавления записей из более низких источников. Когда источник с наивысшим приоритетом его не устанавливает, берет его целиком из следующего источника вниз              | [`availableModels`](#availablemodels), [`allowedMcpServers`](#allowedmcpservers), [`strictKnownMarketplaces`](#strictknownmarketplaces), [`allowedChannelPlugins`](#allowedchannelplugins) и цепь [`fallbackModel`](#fallbackmodel)                                                                                                                                                                                                                                                                                        |
| Значения, взятые целиком                             | Берет значение целиком из источника с наивысшим приоритетом, который его устанавливает, без объединения записей или полей из более низких источников. Когда источник с наивысшим приоритетом его не устанавливает, берет его целиком из следующего источника вниз | [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs), [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Предоставленные MCP серверы                          | Объединяет имена серверов из каждого источника. Когда два источника устанавливают одно имя, применяет всю запись источника с более высоким приоритетом                                                                                                            | [`managedMcpServers`](#managedmcpservers)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| Читается только из источника с наивысшим приоритетом | Читает ключ только из источника с наивысшим приоритетом, который содержит ключ политики, поэтому значение более низкого источника игнорируется даже когда источник с наивысшим приоритетом его не устанавливает                                                   | [`apiKeyHelper`](#apikeyhelper), [`awsAuthRefresh`](#awsauthrefresh), [`awsCredentialExport`](#awscredentialexport), [`gcpAuthRefresh`](#gcpauthrefresh), [`otelHeadersHelper`](#otelheadershelper), `proxyAuthHelper`, [`forceLoginOrgUUID`](#forceloginorguuid), значения `"claudeai"` и `"console"` [`forceLoginMethod`](#forceloginmethod), [`parentSettingsBehavior`](#parentsettingsbehavior), [`modelPicker`](#modelpicker), [`policyHelper`](#policyhelper), [`permissions.defaultMode`](#permissions-defaultmode) |
| `env`                                                | [Объединяет переменные для каждого администратора источников](/docs/ru/managed-settings#keys-read-from-every-admin-source), под обоими `"first-wins"` и `"merge"`                                                                                                      | [`env`](#env)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Каждый другой ключ                                   | Берет значение из источника с наивысшим приоритетом, который его устанавливает                                                                                                                                                                                    | [`cleanupPeriodDays`](#cleanupperioddays), [`model`](#model)                                                                                                                                                                                                                                                                                                                                                                                                                                                               |

Взятие `sandbox.credentials.awsPairs` и `sandbox.ripgrep` целиком требует Claude Code v2.1.257 или позже.

Несколько ключей добавляют условие, которое таблица не показывает:

* **[`policyHelper`](#policyhelper)**: Claude Code соблюдает это только когда источник с наивысшим приоритетом, который содержит ключ политики, является политикой MDM или файлом управляемых параметров, поэтому при управляемых параметрах сервера это не применяется.
* **[`modelOverrides`](#modeloverrides)**: связан с `availableModels`. Claude Code берет `modelOverrides` из источника с наивысшим приоритетом, который его устанавливает, если только более высокий источник не устанавливает `availableModels` без `modelOverrides`. В этом случае он игнорирует `modelOverrides` из каждого источника.
* **[`forceLoginGatewayUrl`](#forcelogingatewayurl), [`gatewayInternalNetworks`](#gatewayinternalnetworks) и значение `"gateway"` [`forceLoginMethod`](#forceloginmethod)**: Claude Code никогда не читает ни один из них из управляемых параметров сервера, поэтому значение там ни не применяется, ни не скрывает значение, установленное в политике MDM или файле управляемых параметров. Среди источников администратора на машине только источник с наивысшим рангом, который содержит ключ политики, поставляет их, независимо от того, присутствуют ли также управляемые параметры сервера.

Чтобы подтвердить, какие источники объединены на машине, запустите `/status` и [прочитайте строку `Setting sources`](/docs/ru/managed-settings#read-the-source-in-/status).

<h3 id="parentsettingsbehavior">
  `parentSettingsBehavior`
</h3>

Выберите, применяет ли Claude Code управляемые параметры, поставляемые процессом хоста встраивания, таким как Agent SDK или расширение IDE, когда также присутствует развернутый администратором управляемый уровень. С `"first-wins"` Claude Code отбрасывает параметры, поставляемые хостом; с `"merge"` он применяет их под уровнем администратора через фильтр только для ограничений. Установите `"merge"`, когда хосту нужно передать свои собственные ограничения сеансам, которые он запускает, например Claude Desktop, доставляющий список разрешенных исходящих соединений шлюза.

* **Область**: [`Managed`](#scopes). Claude Code читает это из источника управляемого администратором с наивысшим приоритетом.
* **Тип**: string, один из:
  * `"first-wins"`: Claude Code отбрасывает параметры, поставляемые хостом, когда присутствует развернутый администратором управляемый уровень
  * `"merge"`: Claude Code применяет параметры, поставляемые хостом, под уровнем администратора через фильтр только для ограничений
* **По умолчанию**: `"first-wins"`

```json managed-settings.json theme={null}
{
  "parentSettingsBehavior": "merge"
}
```

Этот ключ не имеет эффекта, когда не существует развернутого администратором управляемого уровня: параметры хоста затем применяются как единственный управляемый уровень, все еще отфильтрованные до значений только для ограничений. Для ограничений фильтра и того, как взаимодействуют управляемые источники, см. [Родительские параметры от хостов встраивания](/docs/ru/managed-settings#parent-settings-from-embedding-hosts) и [Ограничение родительских параметров](/docs/ru/claude-apps-gateway#restrict-parent-settings).

<span id="compute-managed-settings-with-a-policy-helper" />

<h3 id="policyhelper">
  `policyHelper`
</h3>

Запустите исполняемый файл, который вы развертываете, который вычисляет управляемые параметры при запуске, чтобы вы могли получить политику из позиции устройства, идентификации или удаленного сервиса вместо статического файла. Claude Code запускает помощника перед тем, как принять первый запрос, и рассматривает параметры, которые он выдает, как управляемые параметры для сеанса.

* **Область**: [`Managed`](#scopes). Читается из plist macOS, реестра Windows HKLM или файла управляемых параметров. Claude Code читает ключ из источника управляемого администратором с наивысшим приоритетом, который содержит [ключ политики](/docs/ru/managed-settings#how-claude-code-combines-managed-sources), и запускает помощника только когда этот источник является одним из этих трех; он игнорирует ключ в управляемых параметрах сервера, реестре HKCU и родительских параметрах, поставляемых хостом.
* **Тип**: object с `path`, `timeoutMs` и `refreshIntervalMs`
* **По умолчанию**: не установлено, поэтому помощник не запускается

Когда управляемые параметры сервера доставляют политику при запуске, они имеют приоритет над источником помощника и помощник не запускается.

Если более поздняя выборка параметров сообщает, что управляемые параметры сервера удалены, Claude Code запускает помощника в этот момент вместо ожидания следующего запуска. Его выход управляет остальной частью сеанса, и запуск, который не удается, завершает сеанс с тем же сообщением, что и [неудачный запуск при запуске](#helper-failures).

Этот пример запускает помощника с тайм-аутом 5 секунд и повторно запускает его каждые пять минут:

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
  Напишите выход помощника
</h4>

Claude Code запускает помощника без аргументов, устанавливает `CLAUDE_CODE_VERSION` в его окружение и читает конверт JSON из stdout, ограниченный 1 МиБ.

Поместите параметры под ключ `managedSettings`. Объект параметров без ключа `managedSettings` анализируется с `managedSettings` undefined и ничего не применяет, и Claude Code не сообщает об ошибке:

```json theme={null}
{
  "managedSettings": {
    "permissions": { "deny": ["Read(//etc/secrets/**)"] }
  }
}
```

Когда помощник выдает `managedSettings`, этот объект становится единственным источником управляемых параметров для запуска: Claude Code игнорирует источники MDM, файла и HKCU, читает [ключи между источниками](/docs/ru/managed-settings#keys-read-from-every-admin-source) только из выхода помощника и никогда не объединяет [родительские параметры](/docs/ru/managed-settings#parent-settings-from-embedding-hosts).

Проверка `forceRemoteSettingsRefresh` при запуске выполняется перед помощником и читает любой источник администратора. Помощник, который выходит с `0` с конвертом, который опускает `managedSettings`, не вносит управляемые параметры, и другие источники применяются как обычно.

<h4 id="helper-failures">
  Сбои помощника
</h4>

Запуск помощника не удается, когда:

* `path` нарушает правила в [`policyHelper.path`](#policyhelper-path).
* Нет обычного файла в `path`. Claude Code проверяет файл перед запуском помощника, в пределах того же бюджета `timeoutMs`, поэтому неотзывчивое сетевое крепление может привести к сбою запуска.
* Помощник выходит с ненулевым кодом, все еще работает, когда истекает `timeoutMs`, или вообще не запускается, например потому что он не исполняемый.
* Помощник записывает более 1 МиБ в stdout или stderr.
* stdout не является одним объектом JSON, или его `managedSettings` имеет [нарушение схемы, которое Claude Code не может исправить](/docs/ru/managed-settings#find-entries-claude-code-dropped).

Когда запуск при запуске не удается, Claude Code выводит причину и отказывается запускаться. После выхода с ненулевым кодом причина включает stderr помощника или его stdout, когда stderr пуст. После тайм-аута причина называет предел `timeoutMs` и не включает ни один из выходов помощника. Отказ охватывает интерактивные сеансы, `claude -p`, сеансы Agent SDK, [фоновые сеансы](/docs/ru/agent-view) и большинство подкоманд.

Отказ преднамерен, поэтому помощник, который нуждается в устойчивости к сбоям, должен обслуживать из своего собственного кэша и выходить с `0`.

Когда фоновое обновление не удается, Claude Code сохраняет последнюю успешную политику в силе, и `/status` показывает неудачное обновление с его причиной до тех пор, пока обновление не успешно. Каждое обновление выполняется под тем же `timeoutMs` и правилами сбоев, что и запуск при запуске.

С `--debug` Claude Code записывает stderr помощника из каждого запуска в [журнал отладки](/docs/ru/debug-your-config).

Claude Code сообщает о недействительном значении `policyHelper` как о [отброшенной записи](/docs/ru/managed-settings#find-entries-claude-code-dropped) и запускает сеанс на оставшихся управляемых параметрах без запуска помощника. Недействительные значения включают строку пути и `timeoutMs` ниже [его минимума](#policyhelper-timeoutms).

Чтобы отключить помощника, удалите ключ из источника, который его устанавливает.

<h3 id="policyhelper-path">
  `policyHelper.path`
</h3>

Назовите исполняемый файл помощника, который запускает Claude Code. Для того, что происходит, когда путь нарушает правила ниже, см. [Сбои помощника](#helper-failures).

* **Область**: [`Managed`](#scopes). Читается из plist macOS, реестра Windows HKLM или файла управляемых параметров, где читается [`policyHelper`](#policyhelper).
* **Тип**: string, абсолютный путь в нормализованной форме, без сегментов `.` или `..`; в Windows путь с буквой диска или UNC, заканчивающийся на `.exe`
* **По умолчанию**: нет; требуется, когда установлен `policyHelper`

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

Установите, как долго Claude Code ждет помощника перед тем, как рассматривать запуск как неудачный. Истекший по времени запуск не удается так же, как выход с ненулевым кодом, поэтому при запуске Claude Code отказывается запускаться.

* **Область**: [`Managed`](#scopes). Читается из plist macOS, реестра Windows HKLM или файла управляемых параметров, где читается [`policyHelper`](#policyhelper).
* **Тип**: integer, миллисекунды, минимум `1000`
* **По умолчанию**: `10000`

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

Заставьте Claude Code повторно запускать помощника в фоновом режиме по расписанию, чтобы изменения политики достигли работающего сеанса. Когда обновление успешно, его выход заменяет предыдущие управляемые параметры без перезагрузки; когда обновление не удается, Claude Code сохраняет политику, которая у него уже есть.

* **Область**: [`Managed`](#scopes). Читается из plist macOS, реестра Windows HKLM или файла управляемых параметров, где читается [`policyHelper`](#policyhelper).
* **Тип**: integer, миллисекунды: `0` для отключения обновления, иначе по крайней мере `60000`
* **По умолчанию**: не установлено, поэтому Claude Code запускает помощника один раз при запуске

Этот пример повторно запускает помощника каждые пять минут:

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

Заставьте Claude Code на WSL читать управляемые параметры из цепи политики Windows, с HKLM и файлом управляемых параметров Windows, имеющими приоритет над `/etc/claude-code` и HKCU ниже. Пока цепь включена, Claude Code читает `/etc/claude-code` только когда ни один файл управляемых параметров или drop-in под `C:\Program Files\ClaudeCode\` не доставляет [ключ политики](/docs/ru/managed-settings#how-claude-code-combines-managed-sources). Установите это, чтобы расширить политику, которую вы уже развертываете в Windows, на сеансы WSL на той же машине, чтобы они следовали тем же правилам, что и сеансы хоста. Claude Code соблюдает это только когда установлено в ключе реестра HKLM или в файле управляемых параметров или drop-in под `C:\Program Files\ClaudeCode\`, оба из которых требуют администратора Windows для записи.

* **Область**: [`Managed`](#scopes). В источнике Windows, управляемом администратором.
* **Тип**: Boolean
  * `true`: Claude Code на WSL читает управляемые параметры из цепи политики Windows, и читает `/etc/claude-code` только когда ни один файл управляемых параметров или drop-in под `C:\Program Files\ClaudeCode\` не доставляет [ключ политики](/docs/ru/managed-settings#how-claude-code-combines-managed-sources)
  * `false`: WSL читает только `/etc/claude-code`
* **По умолчанию**: `false`, поэтому WSL читает только `/etc/claude-code`

```json managed-settings.json theme={null}
{
  "wslInheritsWindowsSettings": true
}
```

Как только источник администратора включает цепь, политика HKCU присоединяется к ней на WSL только когда HKCU также устанавливает ключ на `true`. Эта копия не включает цепь сама по себе. Источник Windows, который содержит только этот ключ, не считается источником политики, поэтому источник с более низким приоритетом все еще поставляет политику. Этот ключ не имеет эффекта на нативный Windows.

<h2 id="global-config-settings">
  Глобальные параметры конфигурации
</h2>

Сохраняйте эти ключи в `~/.claude.json`, а не в файле параметров. Claude Code игнорирует их везде в других местах. Claude Code и `/config` записывают большинство из них за вас, и вы также можете редактировать их вручную.

<h3 id="autoconnectide">
  `autoConnectIde`
</h3>

Подключайтесь к запущенной IDE автоматически при запуске Claude Code из внешнего терминала. Отображается в `/config` как **Auto-connect to IDE (external terminal)** при запуске Claude Code вне терминала VS Code или JetBrains.

* **Scope**: [`Global config`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code подключается к запущенной IDE автоматически при запуске из внешнего терминала
  * `false`: Claude Code не подключается автоматически из внешнего терминала; внутри терминала VS Code или JetBrains, или с `--ide`, он все еще подключается
* **Default**: `false`
* **Per-session overrides**: [`CLAUDE_CODE_AUTO_CONNECT_IDE`](/docs/ru/env-vars) имеет приоритет над этим ключом для одного сеанса в любом направлении

```json ~/.claude.json theme={null}
{
  "autoConnectIde": true
}
```

Claude Code игнорирует этот ключ в `settings.json`.

<h3 id="autoinstallideextension">
  `autoInstallIdeExtension`
</h3>

Устанавливайте расширение Claude Code IDE автоматически при запуске Claude Code из терминала VS Code. Отображается в `/config` как **Auto-install IDE extension** при запуске Claude Code внутри терминала VS Code или JetBrains.

* **Scope**: [`Global config`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code устанавливает расширение IDE автоматически при запуске из терминала VS Code
  * `false`: Claude Code не устанавливает расширение автоматически
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/ru/env-vars) установленный на `1` пропускает установку для одного сеанса даже когда этот ключ имеет значение `true`

```json ~/.claude.json theme={null}
{
  "autoInstallIdeExtension": false
}
```

Claude Code игнорирует этот ключ в `settings.json`.

<h3 id="copyonselect">
  `copyOnSelect`
</h3>

Копируйте текст в буфер обмена автоматически при завершении выделения текста мышью в [полноэкранном режиме](/docs/ru/fullscreen#use-the-mouse) или [представлении агента](/docs/ru/agent-view). Отображается в `/config` как **Copy on select** во время включения полноэкранного режима.

* **Scope**: [`Global config`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code копирует текст в буфер обмена при завершении выделения
  * `false`: выделение текста оставляет буфер обмена без изменений, и вы [копируете выделение с помощью сочетания клавиш](/docs/ru/fullscreen#use-the-mouse) вместо этого
* **Default**: `true`

```json ~/.claude.json theme={null}
{
  "copyOnSelect": false
}
```

Claude Code игнорирует этот ключ в `settings.json`.

<h3 id="difftool">
  `diffTool`
</h3>

Выбирайте, где Claude Code показывает diff изменения `Edit` или `Write`, которое оно предлагает, когда подключена IDE [VS Code](/docs/ru/vs-code) или [JetBrains](/docs/ru/jetbrains#features): `"auto"` открывает его в средстве просмотра diff IDE, `"terminal"` сохраняет его в терминале. Отображается в `/config` как **Diff tool** только когда Claude Code подключен к IDE VS Code или JetBrains.

* **Scope**: [`Global config`](#scopes)
* **Type**: string, один из:
  * `"auto"`: Claude Code открывает diff в средстве просмотра diff IDE при подключении IDE VS Code или JetBrains
  * `"terminal"`: Claude Code сохраняет diff в терминале
* **Default**: `"auto"`

```json ~/.claude.json theme={null}
{
  "diffTool": "terminal"
}
```

Claude Code игнорирует этот ключ в `settings.json`.

<h3 id="externaleditorcontext">
  `externalEditorContext`
</h3>

При нажатии `Ctrl+G` Claude Code открывает подсказку, которую вы печатаете, в вашем [внешнем редакторе](/docs/ru/interactive-mode#general-controls). Когда этот ключ включен, буфер редактора начинается с предыдущего ответа Claude в виде строк комментариев `#`, чтобы вы могли прочитать его во время написания, и Claude Code удаляет эти строки при сохранении. Отображается в `/config` как **Show last response in external editor**.

* **Scope**: [`Global config`](#scopes)
* **Type**: Boolean
  * `true`: буфер редактора начинается с предыдущего ответа Claude в виде строк комментариев `#`, которые Claude Code удаляет при сохранении
  * `false`: буфер редактора открывается только с вашей подсказкой
* **Default**: `false`

```json ~/.claude.json theme={null}
{
  "externalEditorContext": true
}
```

Когда это включено, буфер, который открывает Claude Code, выглядит так, и только текст ниже строки маркера отправляется как ваша подсказка:

```text theme={null}
# ─── Claude's last response (for reference; removed on save) ───
# I added the retry loop to fetchUser in src/api.ts and a test
# for the timeout case. Want me to wire the same retry into
# fetchOrders?
# ─── Write your reply below this line ──────────────────────────

Yes, and cap it at three attempts.
```

Claude Code сохраняет последние 50 строк ответа и отмечает срез с помощью `# … (earlier output truncated)`.

Claude Code игнорирует этот ключ в `settings.json`.

<h3 id="permissionexplainerenabled">
  `permissionExplainerEnabled`
</h3>

<Warning>
  Удалено в v2.1.257 вместе с объяснением команды `Ctrl+E` на подсказках разрешений Bash и PowerShell. Установка этого параметра не влияет на текущие версии.
</Warning>

До версии v2.1.256 вы могли нажать `Ctrl+E` на подсказке разрешения Bash или PowerShell, чтобы увидеть созданное моделью объяснение команды, и установить этот ключ на `false`, чтобы отключить это сочетание клавиш.

* **Scope**: [`Global config`](#scopes). На v2.1.256 и более ранних версиях.
* **Type**: Boolean
* **Default**: `true`

<h3 id="teammatedefaultmodel">
  `teammateDefaultModel`
</h3>

<Warning>
  Удалено в v2.1.234 вместе с его строкой `/config` **Default teammate model**. Установка этого параметра не влияет на текущие версии.
</Warning>

До версии v2.1.233 вы устанавливали этот ключ на модель для товарищей по команде [агентской команды](/docs/ru/agent-teams#specify-teammates-and-models), для которых ваша подсказка не указала модель: псевдоним, такой как `"sonnet"`, или `null`, чтобы следовать модели лидера. Для модели, которую Claude Code выбирает для таких товарищей по команде сейчас, см. [укажите товарищей по команде и модели](/docs/ru/agent-teams#specify-teammates-and-models).

* **Scope**: [`Global config`](#scopes). На v2.1.233 и более ранних версиях.
* **Type**: string, псевдоним модели или полный ID модели, или `null`
* **Default**: unset

<h2 id="see-also">
  См. также
</h2>

* [Настройка разрешений](/docs/ru/permissions): синтаксис правил, режимы разрешений и доверие рабочей области
* [Переменные окружения](/docs/ru/env-vars): каждая переменная `CLAUDE_*`, `ANTHROPIC_*` и переменная поставщика, которую читает Claude Code
* [Инструменты, доступные Claude](/docs/ru/tools-reference): встроенные инструменты и какие требуют одобрения
* [Примеры файлов настроек](/docs/ru/settings-example): личный файл, командный файл и управляемый файл организации
* [Настройка управляемых параметров](/docs/ru/admin-setup): как организации решают, что применять
* [Развертывание управляемых параметров](/docs/ru/managed-settings): механизмы доставки, приоритет в управляемом уровне и недопустимые записи в управляемых параметрах
* [Отладка конфигурации](/docs/ru/debug-your-config): `claude doctor` и диалог ошибки параметров
