> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 提示詞庫

> 複製貼上提示詞供 Claude Code 使用，按任務和角色標記。

export const PromptLibrary = ({text = {}, labels = {}, tagLabels = {}, phaseLabels = {}, sourceLabels = {}, catLabels = {}}) => {
  const RAW = useMemo(() => [{
    id: 'get-oriented-in-a',
    sdlc: 'discover',
    cat: 'Onboard',
    startN: 1,
    roles: [],
    prompt: 'give me an overview of this codebase: architecture, key directories, and how the pieces connect',
    nextHref: '/en/memory',
    src: 'workflows'
  }, {
    id: 'explain-unfamiliar-code',
    sdlc: 'discover',
    cat: 'Understand',
    roles: [],
    prompt: 'explain what {path} does and how data flows through it. write it up as {format}',
    slots: {
      path: 'src/scheduler/queue.ts',
      format: 'an HTML page with a diagram, then open it in my browser'
    },
    nextHref: '/en/output-styles',
    src: 'workflows'
  }, {
    id: 'find-where-something-happens',
    sdlc: 'discover',
    cat: 'Understand',
    startN: 2,
    roles: [],
    prompt: 'where do we {behavior}?',
    slots: {
      behavior: 'validate uploaded file types'
    },
    src: 'workflows'
  }, {
    id: 'see-what-depends-on',
    sdlc: 'discover',
    cat: 'Understand',
    roles: [],
    prompt: 'what would break if I deleted {target}?',
    slots: {
      target: 'the retryWithBackoff helper'
    },
    src: 'workflows'
  }, {
    id: 'trace-how-code-evolved',
    sdlc: 'discover',
    cat: 'Understand',
    roles: [],
    prompt: 'look through the commit history of {path} and summarize how it evolved and why',
    slots: {
      path: 'internal/auth/session.go'
    },
    src: 'best-practices'
  }, {
    id: 'scope-a-change-before',
    sdlc: 'discover',
    cat: 'Understand',
    roles: ['pm', 'design'],
    prompt: 'which files would I need to touch to {change}?',
    slots: {
      change: 'add a dark mode toggle to settings'
    },
    src: 'teams'
  }, {
    id: 'ask-the-codebase-a',
    sdlc: 'discover',
    cat: 'Understand',
    roles: ['pm'],
    prompt: 'I am a {role}. walk me through what happens when a user {action}, from the UI down to the result',
    slots: {
      role: 'PM',
      action: 'clicks Export to PDF'
    },
    nextHref: '/en/output-styles',
    src: 'teams'
  }, {
    id: 'plan-a-multi-file',
    sdlc: 'design',
    cat: 'Plan',
    roles: ['pm', 'design'],
    prompt: 'plan how to refactor the {target} to {goal}. list the files you would change, but don\'t edit anything yet',
    slots: {
      target: 'payment module',
      goal: 'support multiple currencies'
    },
    src: 'workflows'
  }, {
    id: 'draft-a-spec-by',
    sdlc: 'design',
    cat: 'Plan',
    roles: ['pm'],
    prompt: 'I want to build {feature}. interview me about implementation, UX, edge cases, and tradeoffs until we have covered everything, then write the spec to SPEC.md',
    slots: {
      feature: 'per-workspace rate limits'
    },
    nextHref: '/en/skills',
    src: 'best-practices'
  }, {
    id: 'turn-a-meeting-into',
    sdlc: 'design',
    cat: 'Plan',
    roles: ['pm'],
    prompt: 'read {input} and write up the action items, then create a {tracker} ticket for each with acceptance criteria',
    slots: {
      input: '@meeting-notes.md',
      tracker: 'Linear'
    },
    needs: 'tracker',
    nextHref: '/en/skills',
    src: 'teams'
  }, {
    id: 'map-edge-cases-before',
    sdlc: 'design',
    cat: 'Plan',
    roles: ['design', 'pm'],
    prompt: 'list the error states, empty states, and edge cases for {feature} that the design needs to cover',
    slots: {
      feature: 'the file upload flow'
    },
    src: 'teams'
  }, {
    id: 'turn-a-mockup-into',
    sdlc: 'design',
    cat: 'Prototype',
    roles: ['design', 'pm', 'marketing'],
    paste: 'mockup',
    prompt: 'here is a mockup. build a working prototype I can click through, matching the layout and states shown',
    src: 'teams'
  }, {
    id: 'implement-from-a-screenshot',
    sdlc: 'design',
    cat: 'Prototype',
    roles: ['design'],
    paste: 'design',
    needs: 'browser',
    prompt: 'implement this design, then take a screenshot of the result, compare it to the original, and fix any differences',
    nextHref: '/en/goal',
    src: 'best-practices'
  }, {
    id: 'follow-an-existing-pattern',
    sdlc: 'build',
    cat: 'Implement',
    roles: [],
    prompt: 'look at how {example} is implemented to understand the pattern, then build {new} the same way',
    slots: {
      example: 'the GitHub webhook handler',
      new: 'a Stripe webhook handler'
    },
    nextHref: '/en/memory',
    src: 'best-practices'
  }, {
    id: 'generate-docs-for-code',
    sdlc: 'build',
    cat: 'Implement',
    roles: ['docs'],
    prompt: 'find {scope} without {format} comments and add them, matching the style already used in the file',
    slots: {
      scope: 'the public functions in src/auth/',
      format: 'JSDoc'
    },
    src: 'workflows'
  }, {
    id: 'add-a-small-well',
    sdlc: 'build',
    cat: 'Implement',
    roles: [],
    prompt: 'add a {endpoint} endpoint that returns {payload}',
    slots: {
      endpoint: '/health',
      payload: 'the app version and uptime'
    },
    src: 'workflows'
  }, {
    id: 'build-a-small-internal',
    sdlc: 'build',
    cat: 'Implement',
    roles: ['pm', 'design', 'marketing', 'docs'],
    prompt: 'create a {tool} using HTML, CSS, and vanilla JavaScript, then open it in my browser',
    slots: {
      tool: 'drag-and-drop Kanban board with three columns'
    },
    src: 'teams'
  }, {
    id: 'work-an-issue-end',
    sdlc: 'build',
    cat: 'Implement',
    roles: [],
    prompt: 'read issue #{issue}, implement the fix, and run the tests',
    slots: {
      issue: '312'
    },
    needs: 'gh',
    src: 'workflows'
  }, {
    id: 'find-and-update-copy',
    sdlc: 'build',
    cat: 'Implement',
    roles: ['design', 'docs', 'marketing'],
    prompt: 'find every place we say "{copy}" or a close variant, show me each one in context, then update them all to "{new}". leave tests and the changelog alone',
    slots: {
      copy: 'Sign up free',
      new: 'Start free trial'
    },
    src: 'teams'
  }, {
    id: 'draft-from-past-examples',
    sdlc: 'build',
    cat: 'Implement',
    roles: ['docs', 'marketing', 'pm'],
    prompt: 'read the {examples} in {folder} to learn the structure and voice, then draft a new one for {topic}',
    slots: {
      examples: 'privacy impact assessments',
      folder: 'legal/pia/',
      topic: 'the new analytics integration'
    },
    nextHref: '/en/skills',
    src: 'legal'
  }, {
    id: 'write-tests-run-them',
    sdlc: 'build',
    cat: 'Test',
    startN: 4,
    roles: [],
    prompt: 'write tests for {path}, run them, and fix any failures',
    slots: {
      path: 'app/parsers/feed.py'
    },
    nextHref: '/en/memory',
    src: 'workflows'
  }, {
    id: 'drive-implementation-from-tests',
    sdlc: 'build',
    cat: 'Test',
    roles: [],
    prompt: 'write tests for {feature} first, then implement it until they pass',
    slots: {
      feature: 'the password reset flow'
    },
    src: 'ebook'
  }, {
    id: 'fill-gaps-from-a',
    sdlc: 'build',
    cat: 'Test',
    roles: [],
    prompt: 'read {report} and add tests for the lowest-covered files until each is above {target}%',
    slots: {
      report: 'coverage/coverage-summary.json',
      target: '80'
    },
    nextHref: '/en/goal',
    src: 'workflows'
  }, {
    id: 'migrate-a-pattern-across',
    sdlc: 'build',
    cat: 'Refactor',
    roles: [],
    prompt: 'migrate everything from {from} to {to}: identify every place that needs to change, then make the changes',
    slots: {
      from: 'the old logging API',
      to: 'the structured logger'
    },
    src: 'workflows'
  }, {
    id: 'port-code-between-languages',
    sdlc: 'build',
    cat: 'Refactor',
    roles: [],
    prompt: 'port {source} to {target}, keeping the same {keep}',
    slots: {
      source: 'this Python module',
      target: 'Rust',
      keep: 'public API and test behavior'
    },
    src: 'teams'
  }, {
    id: 'optimize-against-a-measurable',
    sdlc: 'build',
    cat: 'Refactor',
    roles: ['data'],
    prompt: 'optimize {target} to bring {metric} from {current} down to under {goal}',
    slots: {
      target: 'the search query',
      metric: 'p95 latency',
      current: '2s',
      goal: '500ms'
    },
    nextHref: '/en/goal',
    src: 'ebook'
  }, {
    id: 'fix-a-precise-visual',
    sdlc: 'build',
    cat: 'Refactor',
    roles: ['design'],
    prompt: 'the {element} extends {amount} beyond the {container} on {viewport}. fix it.',
    slots: {
      element: 'login button',
      amount: '20px',
      container: 'card border',
      viewport: 'mobile'
    },
    nextHref: '/en/desktop#preview-your-app',
    src: 'ebook'
  }, {
    id: 'review-your-changes-before',
    sdlc: 'build',
    cat: 'Review',
    startN: 5,
    roles: [],
    prompt: 'review my uncommitted changes and flag anything that looks risky before I commit',
    nextHref: '/en/commands',
    src: 'workflows'
  }, {
    id: 'review-a-pull-request',
    sdlc: 'build',
    cat: 'Review',
    roles: [],
    prompt: 'review PR #{pr} and summarize what changed, then list any concerns',
    slots: {
      pr: '247'
    },
    needs: 'gh',
    nextHref: '/en/code-review',
    src: 'workflows'
  }, {
    id: 'review-infrastructure-changes-before',
    sdlc: 'build',
    cat: 'Review',
    roles: ['security', 'ops'],
    paste: 'plan',
    prompt: 'here is my Terraform plan output. what is this going to do, and is anything here going to cause problems?',
    src: 'teams'
  }, {
    id: 'run-a-security-review',
    sdlc: 'build',
    cat: 'Review',
    roles: ['security'],
    prompt: 'use a subagent to review {path} for security issues and report what it finds',
    slots: {
      path: 'src/api/'
    },
    nextHref: '/en/sub-agents',
    src: 'best-practices'
  }, {
    id: 'review-content-before-sending',
    sdlc: 'build',
    cat: 'Review',
    roles: ['marketing', 'docs'],
    prompt: 'review {file} for {concerns} and list anything I should fix before it goes to {reviewer}',
    slots: {
      file: 'launch-post.md',
      concerns: 'unsupported claims, missing attributions, and brand-guideline issues',
      reviewer: 'legal'
    },
    nextHref: '/en/skills',
    src: 'legal'
  }, {
    id: 'course-correct-a-wrong',
    sdlc: 'build',
    cat: 'Steer',
    roles: [],
    prompt: 'that is not right: {feedback}. try a different approach',
    slots: {
      feedback: 'the function signature needs to stay backward-compatible'
    },
    nextHref: '/en/checkpointing',
    src: 'best-practices'
  }, {
    id: 'narrow-the-scope-of',
    sdlc: 'build',
    cat: 'Steer',
    roles: [],
    prompt: 'that is too much. keep only the changes to {scope} and undo your other edits',
    slots: {
      scope: 'the validation logic in src/forms/'
    },
    src: 'best-practices'
  }, {
    id: 'turn-a-correction-into',
    sdlc: 'build',
    cat: 'Steer',
    roles: [],
    prompt: 'you keep {mistake}. add a rule to CLAUDE.md so this stops happening',
    slots: {
      mistake: 'using default exports when this project uses named exports'
    },
    nextHref: '/en/memory',
    src: 'best-practices'
  }, {
    id: 'resolve-merge-conflicts',
    sdlc: 'ship',
    cat: 'Git',
    roles: [],
    prompt: 'resolve the merge conflicts in this branch and explain what you kept from each side',
    src: 'workflows'
  }, {
    id: 'commit-with-a-generated',
    sdlc: 'ship',
    cat: 'Git',
    roles: [],
    prompt: 'commit these changes with a message that summarizes what I did',
    src: 'workflows'
  }, {
    id: 'open-a-pull-request',
    sdlc: 'ship',
    cat: 'Git',
    roles: [],
    prompt: 'find the {tracker} ticket about {topic} and open a PR that implements it',
    slots: {
      tracker: 'Linear',
      topic: 'the login timeout'
    },
    needs: 'tracker',
    src: 'workflows'
  }, {
    id: 'draft-release-notes-from',
    sdlc: 'ship',
    cat: 'Release',
    roles: ['pm', 'docs', 'marketing'],
    prompt: 'compare {from} to {to} and draft release notes grouped by feature, fix, and breaking change',
    slots: {
      from: 'v2.3.0',
      to: 'v2.4.0'
    },
    nextHref: '/en/skills',
    src: 'workflows'
  }, {
    id: 'write-a-ci-workflow',
    sdlc: 'ship',
    cat: 'Release',
    roles: ['ops'],
    prompt: 'write a GitHub Actions workflow that {steps} on every push to {branch}',
    slots: {
      steps: 'runs the tests and deploys to staging',
      branch: 'main'
    },
    src: 'workflows'
  }, {
    id: 'find-and-fix-a',
    sdlc: 'operate',
    cat: 'Debug',
    startN: 3,
    roles: [],
    prompt: 'the {test} test is failing, find out why and fix it',
    slots: {
      test: 'UserAuth'
    },
    src: 'workflows'
  }, {
    id: 'investigate-a-reported-error',
    sdlc: 'operate',
    cat: 'Debug',
    roles: ['ops'],
    prompt: 'users are seeing {symptom} on {where}. investigate and tell me what is going on',
    slots: {
      symptom: '500 errors',
      where: '/api/settings'
    },
    nextHref: '/en/web-quickstart#pre-fill-sessions',
    src: 'workflows'
  }, {
    id: 'fix-a-build-error',
    sdlc: 'operate',
    cat: 'Debug',
    roles: ['ops'],
    paste: 'error',
    prompt: 'here is a build error. fix the root cause and verify the build succeeds',
    src: 'best-practices'
  }, {
    id: 'investigate-a-production-incident',
    sdlc: 'operate',
    cat: 'Incident',
    roles: ['ops', 'security'],
    prompt: '{symptom}. check the logs, recent deploys, and config changes, then tell me the most likely cause',
    slots: {
      symptom: 'the checkout endpoint started returning 500s an hour ago'
    },
    nextHref: '/en/mcp',
    src: 'workflows'
  }, {
    id: 'diagnose-from-a-console',
    sdlc: 'operate',
    cat: 'Incident',
    roles: ['ops', 'data'],
    paste: 'screenshot',
    prompt: 'here is a screenshot of {console}. walk me through why {resource} is failing and give me the exact commands to fix it',
    slots: {
      console: 'the GCP Kubernetes dashboard',
      resource: 'this pod'
    },
    src: 'teams'
  }, {
    id: 'query-logs-in-plain',
    sdlc: 'operate',
    cat: 'Incident',
    roles: ['security', 'ops', 'data'],
    prompt: 'show me all {events} for {scope} over {timeframe}. write the query, run it, and tell me what stands out',
    slots: {
      events: 'failed logins',
      scope: 'the auth service',
      timeframe: 'the past 24 hours'
    },
    needs: 'db',
    src: 'cybersecurity'
  }, {
    id: 'analyze-a-data-file',
    sdlc: 'operate',
    cat: 'Data',
    roles: ['data', 'pm', 'marketing'],
    paste: 'csv',
    prompt: 'read {file}, summarize the key patterns, and write the results to {output}',
    slots: {
      file: '@reports/q1-signups.csv',
      output: 'an HTML page with charts, then open it in my browser'
    },
    nextHref: '/en/mcp',
    src: 'teams'
  }, {
    id: 'generate-variations-from-performance',
    sdlc: 'operate',
    cat: 'Data',
    roles: ['marketing', 'data'],
    paste: 'csv',
    prompt: 'read {file}, find the underperforming {items}, and generate {n} new variations that stay under {limit} characters',
    slots: {
      file: '@ads-performance.csv',
      items: 'headlines',
      n: '20',
      limit: '90'
    },
    nextHref: '/en/mcp',
    src: 'teams'
  }, {
    id: 'turn-a-recurring-task',
    sdlc: 'operate',
    cat: 'Automate',
    roles: [],
    prompt: 'create a /{name} skill for this project that {steps}',
    slots: {
      name: 'ship',
      steps: 'runs the linter and tests, then drafts a commit message'
    },
    src: 'workflows'
  }, {
    id: 'add-a-hook-for',
    sdlc: 'operate',
    cat: 'Automate',
    roles: [],
    prompt: 'write a hook that {action} after every {event}',
    slots: {
      action: 'runs prettier',
      event: 'edit to a .ts or .tsx file'
    },
    src: 'best-practices'
  }, {
    id: 'connect-a-tool-with',
    sdlc: 'operate',
    cat: 'Automate',
    roles: [],
    prompt: 'set up the {server} MCP server so you can read my {data} directly',
    slots: {
      server: 'Sentry',
      data: 'error reports'
    },
    src: 'workflows'
  }, {
    id: 'capture-what-to-remember',
    sdlc: 'operate',
    cat: 'Automate',
    roles: ['pm', 'docs'],
    prompt: 'summarize what we did this session and suggest what to add to CLAUDE.md',
    src: 'teams'
  }], []);
  const PROMPTS = useMemo(() => {
    if (typeof window !== 'undefined') {
      const rawIds = new Set(RAW.map(p => p.id));
      RAW.forEach(p => {
        if (!text[p.id]) console.warn('[prompt-library] no text[] entry for id:', p.id);
      });
      Object.keys(text).forEach(k => {
        if (!rawIds.has(k)) console.warn('[prompt-library] orphaned text[] key:', k);
      });
    }
    return RAW.map(p => ({
      ...p,
      title: p.id,
      teaches: '',
      ...text[p.id] || ({})
    }));
  }, [RAW, text]);
  const L = labels;
  const TL = k => tagLabels[k] || k;
  const CAT_TAG = useMemo(() => ({
    Onboard: 'understand',
    Understand: 'understand',
    Plan: 'plan',
    Prototype: 'prototype',
    Implement: 'build',
    Test: 'test',
    Refactor: 'refactor',
    Review: 'review',
    Steer: 'steer',
    Git: 'git',
    Release: 'release',
    Debug: 'debug',
    Incident: 'debug',
    Data: 'data',
    Automate: 'automate'
  }), []);
  const TAGS = useMemo(() => ['understand', 'plan', 'prototype', 'build', 'test', 'refactor', 'review', 'steer', 'debug', 'git', 'release', 'data', 'automate', 'pm', 'design', 'docs', 'marketing', 'security', 'ops'], []);
  const tagsOf = p => [CAT_TAG[p.cat], ...p.roles || []];
  const doc = useMemo(() => {
    const p = typeof window !== 'undefined' ? window.location.pathname : '';
    const base = p.startsWith('/docs/') ? '/docs' : '';
    const m = p.slice(base.length).match(/^\/([a-z]{2}(?:-[A-Z]{2})?)\//);
    const locale = m ? m[1] : 'en';
    return href => {
      if (!href || href[0] !== '/' || href[1] === '/') return href;
      return base + (href.startsWith('/en/') ? '/' + locale + href.slice(3) : href);
    };
  }, []);
  const linkify = s => {
    const out = [];
    let last = 0;
    const re = /\[([^\]]+)\]\(([^)]+)\)/g;
    for (let m; m = re.exec(s); ) {
      if (m.index > last) out.push(s.slice(last, m.index));
      out.push(<a key={m.index} href={doc(m[2])}>{m[1]}</a>);
      last = re.lastIndex;
    }
    if (last < s.length) out.push(s.slice(last));
    return out;
  };
  const codeify = s => s.split(/(`[^`]+`)/g).map((part, i) => part[0] === '`' ? <code key={i}>{part.slice(1, -1)}</code> : part);
  const SOURCES = useMemo(() => ({
    'workflows': '/en/common-workflows',
    'teams': 'https://claude.com/blog/how-anthropic-teams-use-claude-code',
    'legal': 'https://claude.com/blog/how-anthropic-uses-claude-legal',
    'cybersecurity': 'https://claude.com/blog/how-anthropic-uses-claude-cybersecurity',
    'best-practices': '/en/best-practices',
    'ebook': 'https://resources.anthropic.com/hubfs/Scaling%20agentic%20coding%20across%20your%20organization.pdf'
  }), []);
  const [mounted, setMounted] = useState(false);
  const [q, setQ] = useState('');
  const [start, setStart] = useState(true);
  const [sel, setSel] = useState(null);
  const [openId, setOpenId] = useState(null);
  const [copied, setCopied] = useState(null);
  const [fills, setFills] = useState({});
  const copyTimer = useRef(null);
  useEffect(() => {
    setMounted(true);
    return () => clearTimeout(copyTimer.current);
  }, []);
  const setFill = (id, key, val) => setFills(f => ({
    ...f,
    [id + '.' + key]: val
  }));
  const fillOf = (p, key) => {
    const v = fills[p.id + '.' + key];
    return v !== undefined ? v : p.slots && p.slots[key] !== undefined ? p.slots[key] : '';
  };
  const assemble = p => p.prompt.replace(/\{(\w+)\}/g, (_, k) => fillOf(p, k) || p.slots && p.slots[k] || k);
  const preview = p => p.prompt.replace(/\{(\w+)\}/g, (_, k) => p.slots && p.slots[k] || k);
  const bodyText = p => preview(p) + ' ' + p.teaches.replace(/\[([^\]]+)\]\([^)]+\)/g, '$1') + ' ' + (p.next || '');
  const widthFor = s => (s || '').length + 3 + 'ch';
  const ql = q.trim().toLowerCase();
  const toggleTag = k => {
    setStart(false);
    setSel(s => !ql && s === k ? null : k);
  };
  const clear = () => {
    setStart(false);
    setSel(null);
    setQ('');
  };
  const results = useMemo(() => {
    const list = PROMPTS.filter(p => {
      if (ql) return p.title.toLowerCase().includes(ql) || bodyText(p).toLowerCase().includes(ql);
      if (start) return !!p.startN;
      if (sel) return tagsOf(p).includes(sel);
      return true;
    });
    if (ql) return list;
    if (start) return list.sort((a, b) => a.startN - b.startN);
    if (sel) return list.sort((a, b) => (a.roles || []).length - (b.roles || []).length || (b.sdlc === 'operate') - (a.sdlc === 'operate'));
    return list;
  }, [PROMPTS, ql, start, sel]);
  const matchSnippet = p => {
    if (!ql || p.title.toLowerCase().includes(ql)) return null;
    const txt = bodyText(p);
    const at = txt.toLowerCase().indexOf(ql);
    if (at < 0) return null;
    const lo = Math.max(0, at - 30), hi = Math.min(txt.length, at + ql.length + 50);
    return [lo > 0 ? '…' : '', txt.slice(lo, at), <mark key="m">{txt.slice(at, at + ql.length)}</mark>, txt.slice(at + ql.length, hi), hi < txt.length ? '…' : ''];
  };
  const grouped = useMemo(() => {
    if (start && !q.trim()) return [];
    const g = {};
    for (const p of results) {
      const key = p.sdlc + '|' + p.cat;
      (g[key] = g[key] || ({
        sdlc: p.sdlc,
        cat: p.cat,
        items: []
      })).items.push(p);
    }
    return Object.values(g);
  }, [results, start, q]);
  const copy = async (str, id) => {
    try {
      await navigator.clipboard.writeText(str);
    } catch {
      const ta = document.createElement('textarea');
      ta.value = str;
      ta.setAttribute('readonly', '');
      ta.style.position = 'fixed';
      ta.style.opacity = '0';
      document.body.appendChild(ta);
      ta.select();
      document.execCommand('copy');
      document.body.removeChild(ta);
    }
    clearTimeout(copyTimer.current);
    setCopied(id);
    copyTimer.current = setTimeout(() => setCopied(null), 1600);
  };
  const promptBody = p => {
    if (!p.slots) return <code>{p.prompt}</code>;
    const parts = p.prompt.split(/(\{\w+\})/g);
    return <code>
        {parts.map((part, idx) => {
      const m = part.match(/^\{(\w+)\}$/);
      if (!m) return <span key={idx}>{part}</span>;
      const k = m[1];
      const val = fillOf(p, k);
      return <input key={idx} type="text" className="pl-slot" value={val} placeholder={p.slots[k] || k} aria-label={k} style={{
        width: widthFor(val || p.slots[k])
      }} onChange={e => setFill(p.id, k, e.target.value)} onFocus={e => e.target.select()} onClick={e => e.stopPropagation()} />;
    })}
      </code>;
  };
  const card = p => {
    const open = openId === p.id;
    const srcHref = SOURCES[p.src];
    const srcLabel = sourceLabels[p.src];
    const snip = matchSnippet(p);
    return <div key={p.id} className={'pl-card' + (open ? ' pl-open' : '')}>
        <button type="button" className="pl-head" onClick={() => setOpenId(open ? null : p.id)} aria-expanded={open}>
          <span className="pl-title">{p.title}</span>
          {!!p.startN && <span className="pl-chip">{L.startHere} · {p.startN}</span>}
        </button>
        {snip ? <div className="pl-match">{snip}</div> : <code className="pl-prompt-preview">{preview(p)}</code>}
        {open && <div className="pl-body">
            <div className="pl-label">{p.slots ? L.fillAndCopy : L.copyThis}</div>
            {p.needs && L.needs && L.needs[p.needs] && <div className="pl-hint pl-needs">
                <span className="pl-needs-label">{L.needsLabel}</span> {linkify(L.needs[p.needs])}
              </div>}
            {p.paste && L.paste && L.paste[p.paste] && <div className="pl-hint pl-paste">{L.paste[p.paste]}</div>}
            {p.slots && <div className="pl-hint">
                {L.hintBefore} <span className="pl-hint-chip">{L.hintChip}</span> {L.hintAfter}
              </div>}
            <div className="pl-prompt-box">
              <span className="pl-caret">{'❯'}</span>
              {promptBody(p)}
              <button type="button" className="pl-copy" onClick={() => copy(assemble(p), p.id)}>
                {copied === p.id ? L.copied : L.copy}
              </button>
            </div>
            <div className="pl-label">{L.whyWorks}</div>
            <div className="pl-teaches">{linkify(p.teaches)}</div>
            {p.nextHref && p.next && <div className="pl-next">
                <span className="pl-next-label">{L.makeItStick}</span>
                <a href={doc(p.nextHref)}>{codeify(p.next)} →</a>
              </div>}
            {srcLabel && <div className="pl-src">{L.from} {srcHref ? <a href={doc(srcHref)}>{srcLabel}</a> : srcLabel}</div>}
          </div>}
      </div>;
  };
  const STYLES = useMemo(() => `
.pl {
  --pl-accent: #D97757;
  --pl-accent-bg: rgba(217,119,87,0.07);
  --pl-bg: #fff;
  --pl-surface: #FAFAF7;
  --pl-border: #E8E6DC;
  --pl-border-subtle: rgba(31,30,29,0.08);
  --pl-text: #141413;
  --pl-text-2: #5E5D59;
  --pl-text-3: #73726C;
  --pl-text-4: #9C9A92;
  --pl-mono: var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace);
  font-family: 'Anthropic Sans', -apple-system, BlinkMacSystemFont, sans-serif;
  font-size: 16px; color: var(--pl-text); margin: 8px 0 32px;
}
.dark .pl {
  --pl-bg: #1f1e1d;
  --pl-surface: #262624;
  --pl-border: #3d3d3a;
  --pl-border-subtle: rgba(240,238,230,0.08);
  --pl-text: #f0eee6;
  --pl-text-2: #bfbdb4;
  --pl-text-3: #91908a;
  --pl-text-4: #73726c;
}
.pl *, .pl *::before, .pl *::after { box-sizing: border-box; }
.pl button { font-family: inherit; cursor: pointer; }
.pl a { color: var(--pl-accent); text-decoration: none; }
.pl a:hover { text-decoration: underline; }

.pl-search {
  display: flex; align-items: center; gap: 10px;
  padding: 14px 18px; background: var(--pl-surface);
  border: 1px solid var(--pl-border); border-radius: 12px;
  margin-bottom: 14px;
}
.pl-search input {
  flex: 1; border: none; outline: none; background: transparent;
  font-size: 16px; color: var(--pl-text);
}
.pl-search input::placeholder { color: var(--pl-text-4); }

.pl-tags { display: flex; gap: 8px; flex-wrap: wrap; align-items: center; margin-bottom: 18px; }
.pl-tag {
  padding: 7px 14px; border: 1px solid var(--pl-border); background: var(--pl-bg);
  font-size: 14px; color: var(--pl-text-2); border-radius: 999px;
}
.pl-tag:hover { background: var(--pl-surface); }
.pl-tag.pl-on { background: var(--pl-text); border-color: var(--pl-text); color: var(--pl-bg); }
.pl-tag.pl-start { color: var(--pl-accent); font-weight: 500; }
.pl-tag.pl-start.pl-on { background: var(--pl-accent); border-color: var(--pl-accent); color: #fff; }
.pl-tags.pl-dim .pl-tag { opacity: 0.5; }
.pl-tags.pl-dim .pl-tag:hover { opacity: 1; }
.pl-sep { width: 1px; height: 22px; background: var(--pl-border); margin: 0 4px; }
.pl-clear { border: none; background: none; font-size: 13px; color: var(--pl-text-4); padding: 4px 6px; }
.pl-clear:hover { color: var(--pl-text-2); }
.pl-count { margin-left: auto; font-size: 14px; color: var(--pl-text-4); }

.pl-group-h {
  font-size: 12px; letter-spacing: 0.08em; text-transform: uppercase;
  color: var(--pl-text-4); margin: 24px 0 12px;
}
.pl-group-h .pl-phase { color: var(--pl-text-3); }
.pl-card {
  border: 1px solid var(--pl-border-subtle); border-radius: 10px;
  margin-bottom: 12px; background: var(--pl-bg); overflow: hidden;
  padding: 14px 18px;
}
.pl-card.pl-open { border-color: var(--pl-border); background: var(--pl-surface); }
.pl-head {
  width: 100%; display: flex; align-items: baseline; gap: 12px;
  border: none; background: transparent; text-align: left; padding: 0;
}
.pl-head:focus-visible { outline: 2px solid var(--pl-accent); outline-offset: 2px; border-radius: 6px; }
.pl-title {
  flex: 1; font-size: 17px; font-weight: 500; color: var(--pl-text);
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
.pl-prompt-preview {
  display: block; font-family: var(--pl-mono); font-size: 13.5px; color: var(--pl-text-3);
  margin-top: 6px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
.pl-chip {
  font-size: 11px; letter-spacing: 0.05em; text-transform: uppercase;
  padding: 3px 9px; border-radius: 999px; flex-shrink: 0;
  background: var(--pl-accent-bg); color: var(--pl-accent);
}

.pl-body { margin-top: 14px; padding-top: 14px; border-top: 1px solid var(--pl-border-subtle); }
.pl-label {
  font-size: 11.5px; letter-spacing: 0.08em; text-transform: uppercase;
  color: var(--pl-text-4); margin: 12px 0 8px;
}
.pl-prompt-box {
  display: flex; align-items: center; gap: 10px;
  padding: 14px 16px; background: #141413; color: #f0eee6;
  border-radius: 8px; font-family: var(--pl-mono); font-size: 15px;
}
.pl-caret { color: var(--pl-accent); flex-shrink: 0; }
.pl-prompt-box code { flex: 1; background: none; padding: 0; color: inherit; white-space: pre-wrap; line-height: 1.9; }
.pl-slot {
  font-family: var(--pl-mono); font-size: inherit;
  background: rgba(217,119,87,0.15); color: #f0eee6;
  border: none; border-bottom: 1.5px dashed var(--pl-accent);
  border-radius: 4px 4px 0 0; padding: 2px 6px; margin: 0 1px;
  outline: none; min-width: 6ch; max-width: 100%;
  box-sizing: content-box; cursor: text;
}
.pl-slot:hover { background: rgba(217,119,87,0.22); }
.pl-slot:focus { background: rgba(217,119,87,0.28); border-bottom-style: solid; }
.pl-slot::placeholder { color: rgba(240,238,230,0.4); font-style: italic; }
.pl-hint { font-size: 14px; color: var(--pl-text-3); margin: 0 0 10px; }
.pl-paste { color: var(--pl-text-2); }
.pl-needs { color: var(--pl-text-2); }
.pl-needs-label {
  display: inline-block; font-size: 10.5px; letter-spacing: 0.06em;
  text-transform: uppercase; padding: 2px 7px; margin-right: 6px;
  border-radius: 4px; background: var(--pl-accent-bg); color: var(--pl-accent);
}
.pl-hint-chip {
  font-family: var(--pl-mono); font-size: 0.92em;
  background: var(--pl-accent-bg); color: var(--pl-accent);
  border-bottom: 1.5px dashed var(--pl-accent);
  border-radius: 3px 3px 0 0; padding: 1px 5px;
}
.pl-copy {
  font-size: 12.5px; padding: 6px 12px; border-radius: 6px;
  background: var(--pl-accent); color: #fff; border: none; flex-shrink: 0;
}
.pl-teaches { display: block; font-size: 15.5px; color: var(--pl-text-2); margin: 4px 0 0; line-height: 1.6; }
.pl-match {
  display: block; font-size: 13.5px; color: var(--pl-text-3);
  margin-top: 6px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
.pl-match mark { background: var(--pl-accent-bg); color: var(--pl-text); padding: 1px 2px; border-radius: 3px; }
.pl-next {
  display: flex; align-items: baseline; gap: 10px;
  margin: 14px 0 0; padding: 10px 12px;
  background: var(--pl-accent-bg); border-radius: 8px; font-size: 14.5px;
}
.pl-next-label {
  font-size: 11px; letter-spacing: 0.06em; text-transform: uppercase;
  color: var(--pl-accent); font-weight: 600; flex-shrink: 0;
}
.pl-src { display: block; font-size: 14px; color: var(--pl-text-4); margin: 14px 0 0; }

.pl-show-all {
  display: block; width: 100%; padding: 14px; margin-top: 4px;
  border: 1px dashed var(--pl-border); border-radius: 10px;
  background: transparent; font-size: 15px; color: var(--pl-accent);
  text-align: center;
}
.pl-show-all:hover { background: var(--pl-accent-bg); border-style: solid; }

.pl-empty {
  padding: 32px; text-align: center; color: var(--pl-text-4);
  border: 1px dashed var(--pl-border); border-radius: 10px;
}
`, []);
  if (!mounted) return <div className="pl" style={{
    minHeight: 480
  }} />;
  return <div className="pl">
      <style>{STYLES}</style>

      <div className="pl-search">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" style={{
    color: 'var(--pl-text-4)'
  }}>
          <circle cx="11" cy="11" r="7" /><line x1="21" y1="21" x2="16.65" y2="16.65" />
        </svg>
        <input type="text" placeholder={L.search} value={q} onChange={e => {
    setQ(e.target.value);
    if (e.target.value) setStart(false);
  }} aria-label={L.search} />
      </div>

      <div className={'pl-tags' + (ql ? ' pl-dim' : '')}>
        <button type="button" className={'pl-tag pl-start' + (!ql && start ? ' pl-on' : '')} onClick={() => {
    setQ('');
    setStart(!start);
    if (!start) setSel(null);
  }}>
          ★ {L.startHere}
        </button>
        <span className="pl-sep" />
        {TAGS.map(k => <button key={k} type="button" aria-pressed={!ql && sel === k} className={'pl-tag' + (!ql && sel === k ? ' pl-on' : '')} onClick={() => {
    setQ('');
    toggleTag(k);
  }}>
            {TL(k)}
          </button>)}
        {(start || sel || q) && <button type="button" className="pl-clear" onClick={clear}>{L.clear}</button>}
        <span className="pl-count">{results.length} {results.length === 1 ? L.prompt : L.prompts}</span>
      </div>

      {results.length === 0 ? <div className="pl-empty">
          {L.noMatch} {ql ? <code>{q}</code> : null} <button type="button" className="pl-clear" onClick={clear}>{L.clear}</button>
        </div> : !ql && start ? <div>
          <div className="pl-group-h">{L.startHereHeader}</div>
          {results.map(card)}
          <button type="button" className="pl-show-all" onClick={clear}>
            {L.showAll && L.showAll.replace('{n}', PROMPTS.length)} →
          </button>
        </div> : grouped.map(g => <div key={g.sdlc + '|' + g.cat}>
            <div className="pl-group-h"><span className="pl-phase">{phaseLabels[g.sdlc] || g.sdlc}</span> · {catLabels[g.cat] || g.cat}</div>
            {g.items.map(card)}
          </div>)}
    </div>;
};

這是一個提示詞庫，可複製貼上到 Claude Code 中。使用它來探索您未曾嘗試過的工作方式，或當您不確定從何開始時使用。

這些提示詞來自各種 Anthropic 指南，包括[常見工作流程](/docs/zh-TW/common-workflows)、[最佳實踐](/docs/zh-TW/best-practices)和[Anthropic 團隊如何使用 Claude Code](https://claude.com/blog/how-anthropic-teams-use-claude-code)。它們是起點而非腳本。在任何提示詞下打開**為什麼這有效**以查看其背後的模式，這樣您可以編寫自己的提示詞。

export const labels = {
  startHere: "從這裡開始",
  startHereHeader: "五個首先嘗試的提示詞",
  showAll: "顯示全部 {n} 個提示詞",
  search: "搜尋提示詞…",
  clear: "清除",
  prompt: "提示詞",
  prompts: "提示詞",
  noMatch: "沒有提示詞符合",
  fillAndCopy: "填入並複製",
  copyThis: "複製此提示詞",
  hintBefore: "在",
  hintChip: "突出顯示的",
  hintAfter: "欄位中輸入以自訂，然後複製。",
  copy: "複製",
  copied: "已複製",
  whyWorks: "為什麼這有效",
  makeItStick: "使其堅持",
  from: "來自",
  paste: {
    mockup: "貼上、拖曳或 @-提及您的模型圖像，然後發送此內容：",
    design: "貼上、拖曳或 @-提及您的設計圖像，然後發送此內容：",
    screenshot: "貼上、拖曳或 @-提及您的螢幕截圖，然後發送此內容：",
    plan: "首先將您的計畫輸出貼上到提示詞中，然後發送此內容：",
    error: "首先將錯誤輸出貼上到提示詞中，然後發送此內容：",
    csv: "將您的檔案拖入提示詞中，或將下方路徑替換為您自己的 @-提及："
  },
  needsLabel: "需要",
  needs: {
    tracker: "您的問題追蹤器已新增為 [claude.ai 連接器](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai)或 [MCP 伺服器](/docs/zh-TW/mcp)。",
    gh: "[gh CLI](https://cli.github.com) 已驗證，或 GitHub 已新增為 [claude.ai 連接器](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai)。",
    browser: "Claude 可以呈現和截圖結果的方式。[桌面應用程式](/docs/zh-TW/desktop#preview-your-app)內建此功能。在終端中，安裝 [Chrome 擴充功能](/docs/zh-TW/chrome)或 Playwright [MCP](/docs/zh-TW/mcp) 伺服器。",
    db: "您的資料倉庫或日誌存儲已新增為 [claude.ai 連接器](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai)或 [MCP 伺服器](/docs/zh-TW/mcp)。"
  }
};

export const tagLabels = {
  understand: "理解",
  plan: "計畫",
  prototype: "原型",
  build: "建置",
  test: "測試",
  refactor: "重構",
  review: "審查",
  steer: "引導",
  debug: "除錯",
  git: "Git",
  release: "發佈",
  data: "資料",
  automate: "自動化",
  pm: "產品",
  design: "設計",
  docs: "文件",
  marketing: "行銷",
  security: "安全",
  ops: "待命"
};

export const phaseLabels = {
  discover: "探索",
  design: "設計",
  build: "建置",
  ship: "發佈",
  operate: "運營"
};

export const sourceLabels = {
  workflows: "常見工作流程",
  teams: "Anthropic 團隊如何使用 Claude Code",
  legal: "Anthropic 如何在法律中使用 Claude",
  cybersecurity: "Anthropic 如何在網路安全中使用 Claude",
  "best-practices": "最佳實踐",
  ebook: "擴展代理編碼指南"
};

export const catLabels = {
  Onboard: "入職",
  Understand: "理解",
  Plan: "計畫",
  Prototype: "原型",
  Implement: "實施",
  Test: "測試",
  Refactor: "重構",
  Review: "審查",
  Steer: "引導",
  Git: "Git",
  Release: "發佈",
  Debug: "除錯",
  Incident: "事件",
  Data: "資料",
  Automate: "自動化"
};

export const text = {
  "get-oriented-in-a": {
    title: "在新儲存庫中定位",
    teaches: "描述您想了解的內容，而不是要讀取哪些檔案。Claude 自行探索專案並返回其如何組合在一起的摘要。",
    next: "執行 `/init` 以設定 `CLAUDE.md`，以便 Claude 在每個工作階段中記住這一點"
  },
  "explain-unfamiliar-code": {
    title: "解釋不熟悉的程式碼",
    teaches: "命名檔案並說出您想要答案的格式。將 HTML 頁面交換為圖表、項目符號或任何適合您學習方式的內容。",
    next: "設定輸出樣式，以便 Claude 始終以您偏好的格式進行解釋"
  },
  "find-where-something-happens": {
    title: "找到某事發生的位置",
    teaches: "按行為而不是按檔案名稱搜尋。即使您不知道檔案的名稱或它位於哪個目錄，搜尋也能運作。"
  },
  "see-what-depends-on": {
    title: "在刪除前檢查什麼會中斷",
    teaches: "在移除任何內容前先詢問。呼叫者清單和下游效應告訴您是在查看單行清理還是需要協調的變更。"
  },
  "trace-how-code-evolved": {
    title: "追蹤程式碼如何演變",
    teaches: "當問題是為什麼而不是什麼時，指向提交歷史。Claude 讀取您使用的任何版本控制的日誌和責備，並解釋目前實施背後的決策。"
  },
  "scope-a-change-before": {
    title: "在開始前確定變更的範圍",
    teaches: "在將其提交到路線圖前調整工作大小。檔案清單告訴您是在查看一個元件還是跨越式變更。"
  },
  "ask-the-codebase-a": {
    title: "向程式碼庫詢問產品問題",
    teaches: "說明您的角色，以便答案在正確的級別上。Claude 從原始程式碼解釋產品實際執行的內容，無需您讀取它。",
    next: "設定輸出樣式，以便 Claude 始終在此級別上提供答案"
  },
  "plan-a-multi-file": {
    title: "在觸及程式碼前計畫多檔案變更",
    teaches: "新增「暫不編輯」可將探索與變更分開，以便您在任何程式碼移動前看到方法。若要在每個提示詞上將計畫優先設為預設，請按 Shift+Tab 進入[計畫模式](/docs/zh-TW/permission-modes#analyze-before-you-edit-with-plan-mode)。"
  },
  "draft-a-spec-by": {
    title: "透過訪談草擬規格",
    teaches: "要求被訪談而不是自己編寫規格。Claude 詢問您結構化問題，直到需求完整，然後將結果寫入檔案。",
    next: "將您的訪談問題儲存為 `/spec` 技能，以便每個規格都以相同方式開始"
  },
  "turn-a-meeting-into": {
    title: "將會議轉換為工單",
    teaches: "跳過轉錄步驟。Claude 從非結構化輸入中提取行動項目，並透過 [MCP](/docs/zh-TW/mcp) 直接將其寫入您的追蹤器，以便您審查工單而不是轉錄。",
    next: "將此儲存為 `/tickets` 技能"
  },
  "map-edge-cases-before": {
    title: "在建置前對邊界情況進行對應",
    teaches: "詢問缺少什麼，而不是存在什麼。Claude 列出錯誤狀態、空狀態和邊界情況，這些是快樂路徑設計傾向於跳過的。"
  },
  "turn-a-mockup-into": {
    title: "將模型轉換為可運作的原型",
    teaches: "可點擊的原型回答靜態模型無法回答的問題。將可運作的程式碼交給工程部門，而不是在文件中解釋互動。"
  },
  "implement-from-a-screenshot": {
    title: "從螢幕截圖實施並自我檢查",
    teaches: "這為 Claude 提供了驗證迴圈：它呈現、與來源圖像進行比較，並在您指出每個差距前進行迭代。",
    next: "使用 `/goal` 讓 Claude 持續迭代，直到螢幕截圖相符"
  },
  "follow-an-existing-pattern": {
    title: "遵循現有模式",
    teaches: "指向您已經喜歡的程式碼。沒有參考，Claude 預設為一般最佳實踐。有了參考，它就會符合您的程式碼庫實際使用的慣例。",
    next: "要求 Claude 將其遵循的模式寫入 `CLAUDE.md`，以便未來工作階段無需參考即可符合它"
  },
  "add-a-small-well": {
    title: "新增小型、定義明確的功能",
    teaches: "說明輸入和輸出，而不是如何建置它。Claude 找到類似程式碼所在的位置，並將您的程式碼添加到其旁邊。"
  },
  "build-a-small-internal": {
    title: "從頭開始建置小型內部工具",
    teaches: "您不需要專案、框架或建置步驟。描述工具並要求 Claude 打開它，以便您立即看到它運作。"
  },
  "work-an-issue-end": {
    title: "端到端處理問題",
    teaches: "提供問題編號，而不是摘要。Claude 自行讀取完整工單，因此您會忘記提及的需求會通過，並在報告前驗證變更。"
  },
  "find-and-update-copy": {
    title: "在程式碼庫中尋找並更新副本",
    teaches: "詢問變體並說出要跳過的內容。Claude 找到字面搜尋會遺漏的措辭，並保持測試夾具和歷史記錄不變，以便您只審查使用者實際看到的副本。"
  },
  "draft-from-past-examples": {
    title: "從過去的範例草擬文件",
    teaches: "指向已完成工作的資料夾，而不是描述您的風格。Claude 從您已發佈的內容中學習結構和聲音，因此第一稿讀起來像您的其中之一。",
    next: "將聲音儲存為技能，以便每個草稿都從那裡開始"
  },
  "write-tests-run-them": {
    title: "編寫測試、執行測試、修復失敗",
    teaches: "一起要求編寫、執行和修復，以便 Claude 在不停止以獲取指示的情況下進行迭代。",
    next: "執行 `/init` 以便 Claude 自動學習您的測試命令"
  },
  "drive-implementation-from-tests": {
    title: "從測試驅動實施",
    teaches: "測試驅動開發：測試定義工作何時完成，Claude 在實施上進行迭代，直到它們通過。"
  },
  "fill-gaps-from-a": {
    title: "從涵蓋範圍報告填補空白",
    teaches: "指向涵蓋範圍報告，而不是猜測未測試的內容。Claude 讀取實際數字並為最需要的檔案編寫測試。",
    next: "將此設定為 `/goal`，以便 Claude 持續編寫測試，直到涵蓋範圍達到目標"
  },
  "port-code-between-languages": {
    title: "將程式碼移植到另一種語言",
    teaches: "說出要保留的內容，而不僅僅是目標語言。命名必須保持相同的 API 或行為，為 Claude 提供了一份合約來檢查移植。"
  },
  "generate-docs-for-code": {
    title: "為未記錄的程式碼產生文件",
    teaches: "命名範圍和格式。Claude 找到缺少的內容並符合檔案中已有的註解樣式，因此新文件讀起來像其餘部分。"
  },
  "migrate-a-pattern-across": {
    title: "在程式碼庫中遷移模式",
    teaches: "描述舊模式和新模式。要求 Claude 首先識別每個位置意味著呼叫網站在回應中列出，以便您可以檢查是否未遺漏任何內容。"
  },
  "optimize-against-a-measurable": {
    title: "針對可測量目標進行最佳化",
    teaches: "說明指標和目標為 Claude 提供了明確的完成定義。",
    next: "將此設定為 `/goal`，以便 Claude 持續測量和迭代，直到達到該數字"
  },
  "fix-a-precise-visual": {
    title: "修復精確的視覺錯誤",
    teaches: "精確的視覺回饋會得到精確的修復。說明確切的元素、測量和視埠。",
    next: "新增預覽工具，以便 Claude 自行截圖並驗證修復"
  },
  "review-your-changes-before": {
    title: "在提交前審查您的變更",
    teaches: "在問題仍然便宜時捕捉問題。Claude 完整讀取已變更的檔案，而不僅僅是差異行，因此它會發現快速自我審查會遺漏的問題。",
    next: "執行 `/code-review` 以在一個命令中進行相同檢查"
  },
  "review-a-pull-request": {
    title: "審查拉取請求",
    teaches: "Claude 在整個程式碼庫的背景下進行審查，而不僅僅是差異。它讀取已變更的程式碼及其呼叫的內容，因此它會捕捉僅差異審查會遺漏的問題。",
    next: "使用程式碼審查為每個 PR 開啟此功能"
  },
  "review-infrastructure-changes-before": {
    title: "在應用前審查基礎結構變更",
    teaches: "計畫輸出密集且難以掃描。貼上它會在您應用前獲得對實際要變更內容的純文字摘要。"
  },
  "run-a-security-review": {
    title: "使用子代理執行安全審查",
    teaches: "[子代理](/docs/zh-TW/sub-agents)在其自己的背景視窗中執行審計並報告摘要，因此冗長的安全審查不會填滿您的主要工作階段。內建的通用子代理無需額外設定即可處理此問題。",
    next: "設定專用的安全審查子代理，您的整個團隊都可以使用"
  },
  "review-content-before-sending": {
    title: "在正式審查前捕捉問題",
    teaches: "在人類花時間之前進行第一次通過。命名您想檢查的關注點，以便審查集中，然後修復它找到的內容並發送更清潔的草稿。",
    next: "將您的審查檢查清單捕捉為您的整個團隊可以執行的技能"
  },
  "course-correct-a-wrong": {
    title: "糾正錯誤的方法",
    teaches: "命名 Claude 遺漏的約束，而不僅僅是它是錯誤的。具體的原因為 Claude 提供了一個具體的約束來在重試時滿足，而不是再次猜測。",
    next: "按 `Esc` 兩次以打開倒帶菜單並恢復程式碼和對話，以便重試乾淨開始"
  },
  "narrow-the-scope-of": {
    title: "縮小變更的範圍",
    teaches: "當方向正確但變更過於寬泛時，要求 Claude 保留其中一部分，而不是倒帶所有內容。說明的邊界可防止小修復變成重構。"
  },
  "turn-a-correction-into": {
    title: "將更正轉換為規則",
    teaches: "聊天中的更正不會與您的團隊共享。專案 [CLAUDE.md](/docs/zh-TW/memory) 中的規則在您提交後共享，Claude 在每個工作階段開始時讀取它。",
    next: "打開 `/memory` 以審查 Claude 編寫的內容"
  },
  "resolve-merge-conflicts": {
    title: "解決合併衝突",
    teaches: "說出您想要的狀態，而不是要保留哪些標記。要求推理使合併可審查，而不是黑盒。"
  },
  "commit-with-a-generated": {
    title: "使用產生的訊息提交",
    teaches: "讓 Claude 從差異衍生訊息。它符合您儲存庫的現有提交樣式。"
  },
  "open-a-pull-request": {
    title: "從工單打開拉取請求",
    teaches: "跳過追蹤器、編輯器和 GitHub 之間的背景切換。一個提示詞讀取規格、進行變更並打開 PR。"
  },
  "draft-release-notes-from": {
    title: "從 git 歷史草擬發佈說明",
    teaches: "提供兩個參考點和您想要的結構。Claude 讀取它們之間的提交日誌並草擬您可以編輯的變更日誌。",
    next: "將此儲存為 `/changelog` 技能"
  },
  "write-a-ci-workflow": {
    title: "編寫 CI 工作流程",
    teaches: "描述何時應執行以及應執行的操作；YAML 為您產生，符合您專案的建置和測試命令。"
  },
  "find-and-fix-a": {
    title: "尋找並修復失敗的測試",
    teaches: "描述症狀；您不需要知道哪個檔案已損壞。Claude 執行測試以查看失敗，將其追蹤到來源，並修復它。"
  },
  "investigate-a-reported-error": {
    title: "調查報告的錯誤",
    teaches: "描述症狀和位置；Claude 讀取相關程式碼路徑並追蹤可能的原因。如果您有堆疊追蹤或日誌，請貼上它們。",
    next: "在您的執行手冊中放置深層連結，以打開預先填入此提示詞的 Claude"
  },
  "fix-a-build-error": {
    title: "在根本原因處修復建置錯誤",
    teaches: "要求根本原因和驗證可防止表面級修補程式抑制錯誤而不修復它。"
  },
  "investigate-a-production-incident": {
    title: "調查生產事件",
    teaches: "列出要關聯的證據來源，而不是要採取的步驟。Claude 一起讀取日誌、git 歷史和配置以縮小原因。",
    next: "透過 MCP 連接 Sentry 或您的日誌存儲"
  },
  "query-logs-in-plain": {
    title: "用純英文查詢日誌",
    teaches: "詢問問題而不是編寫 SQL。Claude 建置查詢、針對您連接的日誌執行它，並顯示查詢和結果，以便您可以檢查執行的內容。"
  },
  "diagnose-from-a-console": {
    title: "從控制台螢幕截圖診斷",
    teaches: "雲端控制台向您顯示問題，但不顯示修復它的命令。Claude 讀取螢幕截圖並將儀表板轉換為要執行的 kubectl、gcloud 或 aws 命令。"
  },
  "analyze-a-data-file": {
    title: "分析資料檔案",
    teaches: "一次性問題不需要一次性指令碼。指向專案資料夾中的檔案，Claude 直接讀取它、找到模式並在您要求的位置寫入輸出。",
    next: "透過 MCP 連接資料來源，而不是匯出檔案"
  },
  "generate-variations-from-performance": {
    title: "從效能資料產生變體",
    teaches: "在開始時說明約束，以便產生保持在限制內。Claude 讀取指標、選擇要替換的內容，並產生符合的替代方案。",
    next: "透過 MCP 連接廣告平台，而不是匯出檔案"
  },
  "turn-a-recurring-task": {
    title: "將重複任務轉換為技能",
    teaches: "命名步驟一次；將其重複用作命令。Claude 編寫您的團隊中任何人都可以執行的 [技能](/docs/zh-TW/skills)。"
  },
  "add-a-hook-for": {
    title: "為重複行為新增 hook",
    teaches: "Hooks 使行為自動進行，而不是您必須記住要求的內容。描述觸發器和操作，Claude 編寫 [hook](/docs/zh-TW/hooks) 配置。"
  },
  "connect-a-tool-with": {
    title: "使用 MCP 連接工具",
    teaches: "連接來源一次，而不是每個工作階段貼上資料。在 [MCP](/docs/zh-TW/mcp) 設定後，當您詢問它時，Claude 直接從工具讀取。"
  },
  "capture-what-to-remember": {
    title: "捕捉下次要記住的內容",
    teaches: "在您忘記前詢問。Claude 知道它在此工作階段中必須弄清楚的內容，並提議 [CLAUDE.md](/docs/zh-TW/memory) 項目，以便下一個工作階段以該背景開始。"
  }
};

<PromptLibrary text={text} labels={labels} tagLabels={tagLabels} phaseLabels={phaseLabels} sourceLabels={sourceLabels} catLabels={catLabels} />

<h2 id="what-makes-these-prompts-work">
  這些提示詞有效的原因
</h2>

上述提示詞共享一些模式。識別它們有助於您將此處的任何提示詞調整為您自己的任務。

**描述結果，而不是步驟。** 說出您想要的內容，讓 Claude 找到檔案。下面的提示詞無需命名單個檔案路徑即可運作。

```text theme={null}
add rate limiting to the public API and make sure existing tests still pass
```

**給它一種檢查自己工作的方式。** 在同一提示詞中要求執行、測試、比較或驗證，以便 Claude 進行迭代，而不是在一次嘗試後停止。

```text theme={null}
write the migration, run it against the dev database, and confirm the schema matches
```

**指向參考。** 命名現有檔案、測試或模式以符合，以便新程式碼與您已有的內容一致。

```text theme={null}
add a settings page that follows the same layout as the profile page
```

**說明可測量的目標。** 當目標是效能或涵蓋範圍時，提供指標和閾值，以便完成是明確的。

```text theme={null}
get the bundle size under 200KB and show me what you removed
```

**給它工件。** 直接在提示詞中貼上錯誤、日誌、螢幕截圖和計畫輸出，或輸入 `@` 以參考檔案。Claude 讀取來源而不是您對它的描述。

```text theme={null}
why is the build failing? @build.log
```

**說出您想要答案的方式。** 命名格式、長度或受眾，以便解釋適合您使用它的方式。若要為每個回應設定預設格式，請設定 [輸出樣式](/docs/zh-TW/output-styles)。

```text theme={null}
explain how the payment retry logic works as an HTML page with a diagram, then open it in my browser
```

如需每個模式的詳細資訊，請參閱[最佳實踐](/docs/zh-TW/best-practices)。

<h2 id="where-these-come-from">
  這些來自何處
</h2>

這些提示詞基於已發佈的 Anthropic 資源中的模式。每張卡片都連結到其來源：

* [常見工作流程](/docs/zh-TW/common-workflows)：核心任務的逐步指南
* [最佳實踐](/docs/zh-TW/best-practices)：提示詞模式和專案設定
* [Anthropic 團隊如何使用 Claude Code](https://claude.com/blog/how-anthropic-teams-use-claude-code)：來自工程、產品、設計和資料團隊的真實工作流程，深入探討[法律](https://claude.com/blog/how-anthropic-uses-claude-legal)、[行銷](https://claude.com/blog/how-anthropic-uses-claude-marketing)和[網路安全](https://claude.com/blog/how-anthropic-uses-claude-cybersecurity)
* [擴展代理編碼指南](https://resources.anthropic.com/hubfs/Scaling%20agentic%20coding%20across%20your%20organization.pdf)：企業採用指南

如需這些模式的影片演練，請參閱 Anthropic Academy 上的免費 [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action) 課程。

<h2 id="related-resources">
  相關資源
</h2>

此頁面上的提示詞是起點。一旦一個對您的專案有效，下一步是使其可重複：將其儲存為 [技能](/docs/zh-TW/skills)，以便您的團隊中的任何人都可以將其作為 `/command` 執行，並在 [CLAUDE.md](/docs/zh-TW/memory) 中記錄 Claude 學到的慣例，以便每個工作階段都以該背景開始，而不是 Claude 重新學習它。對於更大或更危險的變更，[計畫模式](/docs/zh-TW/permission-modes#analyze-before-you-edit-with-plan-mode)在任何編輯發生前顯示檔案清單。

如果您在整個團隊中引入 Claude Code，請參閱[管理](/docs/zh-TW/admin-setup)以了解受管設定和政策，以及[成本和使用](/docs/zh-TW/costs)以了解此工作在您的計畫上如何計費。
