> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Библиотека промптов

> Копируйте и вставляйте промпты для Claude Code, отсортированные по задачам и ролям.

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

Это библиотека промптов для копирования в Claude Code. Используйте её, чтобы изучить новые способы работы или когда вы не знаете, с чего начать.

Промпты собраны из различных руководств Anthropic, включая [Типичные рабочие процессы](/docs/ru/common-workflows), [Лучшие практики](/docs/ru/best-practices) и [Как команды Anthropic используют Claude Code](https://claude.com/blog/how-anthropic-teams-use-claude-code). Это отправные точки, а не готовые скрипты. Откройте **Почему это работает** под любым промптом, чтобы увидеть закономерность, стоящую за ним, и написать свой собственный.

export const labels = {
  startHere: "Начните отсюда",
  startHereHeader: "Пять промптов для начала",
  showAll: "Показать все {n} промптов",
  search: "Поиск промптов…",
  clear: "Очистить",
  prompt: "промпт",
  prompts: "промпты",
  noMatch: "Промпты не найдены",
  fillAndCopy: "Заполните и скопируйте",
  copyThis: "Скопируйте этот промпт",
  hintBefore: "Введите в",
  hintChip: "выделенные",
  hintAfter: "поля для настройки, затем скопируйте.",
  copy: "Копировать",
  copied: "Скопировано",
  whyWorks: "Почему это работает",
  makeItStick: "Закрепите это",
  from: "Из",
  paste: {
    mockup: "Вставьте, перетащите или упомяните вашу макет-картинку с помощью @, затем отправьте это:",
    design: "Вставьте, перетащите или упомяните вашу дизайн-картинку с помощью @, затем отправьте это:",
    screenshot: "Вставьте, перетащите или упомяните ваш скриншот с помощью @, затем отправьте это:",
    plan: "Вставьте вывод вашего плана в промпт сначала, затем отправьте это:",
    error: "Вставьте вывод ошибки в промпт сначала, затем отправьте это:",
    csv: "Перетащите ваш файл в промпт, или замените путь ниже упоминанием вашего файла с помощью @:"
  },
  needsLabel: "Требует",
  needs: {
    tracker: "вашего трекера задач, добавленного как [коннектор claude.ai](/docs/ru/mcp#use-mcp-servers-from-claude-ai) или [MCP сервер](/docs/ru/mcp).",
    gh: "[gh CLI](https://cli.github.com) с аутентификацией, или GitHub, добавленный как [коннектор claude.ai](/docs/ru/mcp#use-mcp-servers-from-claude-ai).",
    browser: "способ для Claude отрендерить и сделать скриншот результата. [Десктопное приложение](/docs/ru/desktop#preview-your-app) имеет это встроенным. В терминале установите [расширение Chrome](/docs/ru/chrome) или Playwright [MCP](/docs/ru/mcp) сервер.",
    db: "вашего хранилища данных или хранилища логов, добавленного как [коннектор claude.ai](/docs/ru/mcp#use-mcp-servers-from-claude-ai) или [MCP сервер](/docs/ru/mcp)."
  }
};

export const tagLabels = {
  understand: "Понять",
  plan: "Спланировать",
  prototype: "Прототип",
  build: "Построить",
  test: "Тестировать",
  refactor: "Рефакторинг",
  review: "Проверить",
  steer: "Управлять",
  debug: "Отладка",
  git: "Git",
  release: "Релиз",
  data: "Данные",
  automate: "Автоматизировать",
  pm: "Продукт",
  design: "Дизайн",
  docs: "Документация",
  marketing: "Маркетинг",
  security: "Безопасность",
  ops: "Дежурство"
};

export const phaseLabels = {
  discover: "Исследование",
  design: "Дизайн",
  build: "Разработка",
  ship: "Выпуск",
  operate: "Эксплуатация"
};

export const sourceLabels = {
  workflows: "Типичные рабочие процессы",
  teams: "Как команды Anthropic используют Claude Code",
  legal: "Как Anthropic использует Claude в Legal",
  cybersecurity: "Как Anthropic использует Claude в Cybersecurity",
  "best-practices": "Лучшие практики",
  ebook: "Руководство по масштабированию агентного кодирования"
};

export const catLabels = {
  Onboard: "Подключение",
  Understand: "Понимание",
  Plan: "Планирование",
  Prototype: "Прототипирование",
  Implement: "Реализация",
  Test: "Тестирование",
  Refactor: "Рефакторинг",
  Review: "Проверка",
  Steer: "Управление",
  Git: "Git",
  Release: "Релиз",
  Debug: "Отладка",
  Incident: "Инцидент",
  Data: "Данные",
  Automate: "Автоматизация"
};

export const text = {
  "get-oriented-in-a": {
    title: "Ориентируйтесь в новом репозитории",
    teaches: "Опишите, что вы хотите узнать, а не какие файлы читать. Claude исследует проект самостоятельно и возвращает сводку того, как он устроен.",
    next: "Запустите `/init`, чтобы установить `CLAUDE.md`, чтобы Claude помнил это в каждой сессии"
  },
  "explain-unfamiliar-code": {
    title: "Объясните незнакомый код",
    teaches: "Назовите файл и скажите, в каком формате вы хотите ответ. Замените HTML-страницу на диаграмму, маркированный список или что-то, что соответствует вашему стилю обучения.",
    next: "Установите стиль вывода, чтобы Claude всегда объяснял в вашем предпочитаемом формате"
  },
  "find-where-something-happens": {
    title: "Найдите, где что-то происходит",
    teaches: "Ищите по поведению, а не по имени файла. Поиск работает даже когда вы не знаете, как называется файл или в каком каталоге он находится."
  },
  "see-what-depends-on": {
    title: "Проверьте, что сломается перед удалением",
    teaches: "Спросите перед удалением чего-либо. Список вызывающих функций и нисходящих эффектов показывает, смотрите ли вы на однострочную очистку или изменение, которое нужно координировать."
  },
  "trace-how-code-evolved": {
    title: "Проследите, как код эволюционировал",
    teaches: "Указывайте на историю коммитов, когда вопрос в том, почему, а не что. Claude читает логи и blame для любой системы контроля версий, которую вы используете, и объясняет решения, стоящие за текущей реализацией."
  },
  "scope-a-change-before": {
    title: "Определите объем изменений перед началом",
    teaches: "Оцените объем работы перед тем, как взять её в дорожную карту. Список файлов показывает, смотрите ли вы на один компонент или сквозное изменение."
  },
  "ask-the-codebase-a": {
    title: "Задайте кодовой базе вопрос о продукте",
    teaches: "Укажите вашу роль, чтобы ответ был на нужном уровне. Claude объясняет, что продукт на самом деле делает из исходного кода, без необходимости его читать.",
    next: "Установите стиль вывода, чтобы Claude всегда давал ответы на этом уровне"
  },
  "plan-a-multi-file": {
    title: "Спланируйте многофайловое изменение перед редактированием кода",
    teaches: "Добавление \"не редактируйте пока\" разделяет исследование от изменений, поэтому вы видите подход перед тем, как код движется. Чтобы сделать план-первый подход по умолчанию для каждого промпта, нажмите Shift+Tab для [режима плана](/docs/ru/permission-modes#analyze-before-you-edit-with-plan-mode)."
  },
  "draft-a-spec-by": {
    title: "Составьте спецификацию путём интервью",
    teaches: "Попросите провести интервью вместо того, чтобы писать спецификацию самостоятельно. Claude задаёт вам структурированные вопросы, пока требования не будут полными, затем записывает результат в файл.",
    next: "Сохраните ваши вопросы интервью как skill `/spec`, чтобы каждая спецификация начиналась одинаково"
  },
  "turn-a-meeting-into": {
    title: "Превратите встречу в задачи",
    teaches: "Пропустите этап транскрипции. Claude извлекает пункты действий из неструктурированного ввода и записывает их прямо в ваш трекер через [MCP](/docs/ru/mcp), поэтому вы проверяете задачи, а не транскрипцию.",
    next: "Сохраните это как skill `/tickets`"
  },
  "map-edge-cases-before": {
    title: "Определите граничные случаи перед разработкой",
    teaches: "Спросите, чего не хватает, а не что есть. Claude перечисляет состояния ошибок, пустые состояния и граничные случаи, которые дизайн счастливого пути обычно пропускает."
  },
  "turn-a-mockup-into": {
    title: "Превратите макет в работающий прототип",
    teaches: "Интерактивный прототип отвечает на вопросы, на которые статический макет не может. Передайте работающий код инженерам вместо объяснения взаимодействий в документе."
  },
  "implement-from-a-screenshot": {
    title: "Реализуйте из скриншота и самопроверьте",
    teaches: "Это даёт Claude цикл проверки: он рендерит, сравнивает с исходным изображением и повторяет без вашего указания на каждый пробел.",
    next: "Используйте `/goal`, чтобы Claude продолжал повторять, пока скриншоты не совпадут"
  },
  "follow-an-existing-pattern": {
    title: "Следуйте существующему паттерну",
    teaches: "Укажите на код, который вам уже нравится. Без ссылки Claude использует общие лучшие практики. С ней он соответствует соглашениям, которые ваша кодовая база на самом деле использует.",
    next: "Попросите Claude написать паттерн, который он следовал, в `CLAUDE.md`, чтобы будущие сессии соответствовали ему без ссылки"
  },
  "add-a-small-well": {
    title: "Добавьте небольшую, хорошо определённую функцию",
    teaches: "Укажите входы и выходы, а не как её построить. Claude находит, где живёт похожий код, и добавляет ваш рядом с ним."
  },
  "build-a-small-internal": {
    title: "Создайте небольшой внутренний инструмент с нуля",
    teaches: "Вам не нужен проект, фреймворк или этап сборки. Опишите инструмент и попросите Claude открыть его, чтобы вы сразу увидели, что он работает."
  },
  "work-an-issue-end": {
    title: "Работайте над задачей от начала до конца",
    teaches: "Дайте номер задачи, а не сводку. Claude читает полный билет сам, поэтому требования, которые вы забыли упомянуть, проходят, и он проверяет изменение перед отчётом."
  },
  "find-and-update-copy": {
    title: "Найдите и обновите текст по всей кодовой базе",
    teaches: "Спросите варианты и скажите, что пропустить. Claude находит фразировки, которые буквальный поиск пропустит, и оставляет тестовые фиксчуры и историю нетронутыми, поэтому вы проверяете только текст, который видят пользователи."
  },
  "draft-from-past-examples": {
    title: "Составьте документ из прошлых примеров",
    teaches: "Укажите на папку готовой работы вместо описания вашего стиля. Claude изучает структуру и голос из того, что вы уже выпустили, поэтому первый черновик читается как один из ваших.",
    next: "Сохраните голос как skill, чтобы каждый черновик начинался там"
  },
  "write-tests-run-them": {
    title: "Напишите тесты, запустите их, исправьте ошибки",
    teaches: "Спросите написать, запустить и исправить вместе, чтобы Claude повторял без остановки для инструкций.",
    next: "Запустите `/init`, чтобы Claude автоматически изучил вашу команду тестирования"
  },
  "drive-implementation-from-tests": {
    title: "Управляйте реализацией из тестов",
    teaches: "Разработка, управляемая тестами: тесты определяют, когда работа завершена, и Claude повторяет реализацию, пока они не пройдут."
  },
  "fill-gaps-from-a": {
    title: "Заполните пробелы из отчёта о покрытии",
    teaches: "Укажите на отчёт о покрытии вместо угадывания того, что не протестировано. Claude читает фактические числа и пишет тесты для файлов, которые нуждаются в них больше всего.",
    next: "Установите это как `/goal`, чтобы Claude продолжал писать тесты, пока покрытие не достигнет целевого показателя"
  },
  "port-code-between-languages": {
    title: "Перенесите код на другой язык",
    teaches: "Скажите, что сохранить, а не только целевой язык. Название API или поведения, которое должно остаться неизменным, даёт Claude контракт для проверки переноса."
  },
  "generate-docs-for-code": {
    title: "Создайте документацию для недокументированного кода",
    teaches: "Назовите область и формат. Claude находит, что не хватает, и соответствует стилю комментариев, уже находящемуся в файле, поэтому новая документация читается как остальная."
  },
  "migrate-a-pattern-across": {
    title: "Перенесите паттерн по всей кодовой базе",
    teaches: "Опишите старый паттерн и новый. Попросить Claude сначала определить каждое место означает, что сайты вызовов перечислены в ответе, поэтому вы можете проверить, что ничего не пропущено."
  },
  "optimize-against-a-measurable": {
    title: "Оптимизируйте против измеримой цели",
    teaches: "Указание метрики и цели даёт Claude чёткое определение завершения.",
    next: "Установите это как `/goal`, чтобы Claude продолжал измерять и повторять, пока не достигнет числа"
  },
  "fix-a-precise-visual": {
    title: "Исправьте точную визуальную ошибку",
    teaches: "Точная визуальная обратная связь получает точное исправление. Укажите точный элемент, измерение и viewport.",
    next: "Добавьте инструмент предпросмотра, чтобы Claude сделал скриншот и проверил исправление сам"
  },
  "review-your-changes-before": {
    title: "Проверьте ваши изменения перед коммитом",
    teaches: "Поймайте проблемы, пока они ещё дешевы в исправлении. Claude читает изменённые файлы полностью, а не только строки diff, поэтому он замечает проблемы, которые быстрая самопроверка пропускает.",
    next: "Запустите `/code-review` для той же проверки в одной команде"
  },
  "review-a-pull-request": {
    title: "Проверьте pull request",
    teaches: "Claude проверяет со всей кодовой базой в контексте, а не только diff. Он читает изменённый код и то, что он вызывает, поэтому он ловит проблемы, которые проверка только diff пропустит.",
    next: "Включите это для каждого PR с Code Review"
  },
  "review-infrastructure-changes-before": {
    title: "Проверьте изменения инфраструктуры перед применением",
    teaches: "Вывод плана плотный и сложно сканировать. Вставка его даёт вам простой текстовый обзор того, что на самом деле будет изменено перед применением."
  },
  "run-a-security-review": {
    title: "Запустите проверку безопасности с подагентом",
    teaches: "[Подагент](/docs/ru/sub-agents) запускает аудит в своём собственном контекстном окне и сообщает обратно сводку, поэтому длинная проверка безопасности не заполняет вашу основную сессию. Встроенный универсальный подагент справляется с этим без дополнительной настройки.",
    next: "Установите выделенный подагент security-review, который может использовать вся ваша команда"
  },
  "review-content-before-sending": {
    title: "Поймайте проблемы перед формальной проверкой",
    teaches: "Получите первый проход перед тем, как человек потратит на это время. Назовите проблемы, которые вы хотите проверить, чтобы проверка была сосредоточена, затем исправьте то, что она находит, и отправьте более чистый черновик.",
    next: "Захватите ваш контрольный список проверки как skill, который может запустить вся ваша команда"
  },
  "course-correct-a-wrong": {
    title: "Исправьте неправильный подход",
    teaches: "Назовите ограничение, которое Claude пропустил, а не просто что это неправильно. Конкретная причина даёт Claude конкретное ограничение для удовлетворения при повторной попытке, вместо угадывания снова.",
    next: "Нажмите `Esc` дважды, чтобы открыть меню перемотки и восстановить код и разговор, чтобы повторная попытка началась чистой"
  },
  "narrow-the-scope-of": {
    title: "Сузьте область изменения",
    teaches: "Когда направление правильное, но изменение стало слишком широким, попросите Claude сохранить часть его, а не перематывать всё. Указанная граница держит небольшое исправление от становления рефакторингом."
  },
  "turn-a-correction-into": {
    title: "Превратите исправление в правило",
    teaches: "Исправление в чате не делится с вашей командой. Правило в [CLAUDE.md](/docs/ru/memory) проекта делится один раз, когда вы его коммитите, и Claude читает его в начале каждой сессии.",
    next: "Откройте `/memory`, чтобы проверить, что Claude написал"
  },
  "resolve-merge-conflicts": {
    title: "Разрешите конфликты слияния",
    teaches: "Скажите, какое состояние вы хотите, а не какие маркеры сохранить. Запрос рассуждения делает слияние проверяемым вместо чёрного ящика."
  },
  "commit-with-a-generated": {
    title: "Коммитьте с сгенерированным сообщением",
    teaches: "Позвольте Claude вывести сообщение из diff. Это соответствует существующему стилю коммитов вашего репозитория."
  },
  "open-a-pull-request": {
    title: "Откройте pull request из задачи",
    teaches: "Пропустите переключение контекста между трекером, редактором и GitHub. Один промпт читает спецификацию, делает изменение и открывает PR."
  },
  "draft-release-notes-from": {
    title: "Составьте примечания к выпуску из истории git",
    teaches: "Дайте две контрольные точки и структуру, которую вы хотите. Claude читает логи коммитов между ними и составляет changelog, который вы можете редактировать.",
    next: "Сохраните это как skill `/changelog`"
  },
  "write-a-ci-workflow": {
    title: "Напишите CI workflow",
    teaches: "Опишите, когда он должен запускаться и что он должен делать; YAML генерируется для вас, соответствуя командам сборки и тестирования вашего проекта."
  },
  "find-and-fix-a": {
    title: "Найдите и исправьте неудачный тест",
    teaches: "Опишите симптом; вам не нужно знать, какой файл сломан. Claude запускает тест, чтобы увидеть ошибку, отслеживает её в исходный код и исправляет её."
  },
  "investigate-a-reported-error": {
    title: "Исследуйте сообщённую ошибку",
    teaches: "Опишите симптом и местоположение; Claude читает соответствующий путь кода и отслеживает вероятные причины. Вставьте трассировки стека или логи, если они у вас есть.",
    next: "Поместите глубокую ссылку в вашу runbook, которая открывает Claude с этим промптом предварительно заполненным"
  },
  "fix-a-build-error": {
    title: "Исправьте ошибку сборки в корне",
    teaches: "Запрос корневой причины и проверки предотвращает поверхностные патчи, которые подавляют ошибку без её исправления."
  },
  "investigate-a-production-incident": {
    title: "Исследуйте инцидент в production",
    teaches: "Перечислите источники доказательств для корреляции, а не шаги для выполнения. Claude читает логи, историю git и конфигурацию вместе, чтобы сузить причину.",
    next: "Подключите Sentry или ваше хранилище логов через MCP"
  },
  "query-logs-in-plain": {
    title: "Запросите логи на простом английском",
    teaches: "Спросите вопрос вместо написания SQL. Claude строит запрос, запускает его против ваших подключённых логов и показывает как запрос, так и результат, чтобы вы могли проверить, что запустилось."
  },
  "diagnose-from-a-console": {
    title: "Диагностируйте из скриншота консоли",
    teaches: "Облачные консоли показывают вам проблему, но не команды для её исправления. Claude читает скриншот и переводит панель управления в команды kubectl, gcloud или aws для запуска."
  },
  "analyze-a-data-file": {
    title: "Проанализируйте файл данных",
    teaches: "Одноразовый вопрос не требует одноразового скрипта. Укажите на файл в папке вашего проекта, и Claude читает его напрямую, находит закономерности и пишет вывод туда, где вы просите.",
    next: "Подключите источник данных через MCP вместо экспорта файлов"
  },
  "generate-variations-from-performance": {
    title: "Создайте варианты из данных производительности",
    teaches: "Укажите ограничение в начале, чтобы генерация оставалась в пределах лимита. Claude читает метрики, выбирает, что заменить, и производит альтернативы, которые подходят.",
    next: "Подключите платформу объявлений через MCP вместо экспорта файла"
  },
  "turn-a-recurring-task": {
    title: "Превратите повторяющуюся задачу в skill",
    teaches: "Назовите шаги один раз; переиспользуйте их как команду. Claude пишет [skill](/docs/ru/skills), который может запустить кто-либо из вашей команды."
  },
  "add-a-hook-for": {
    title: "Добавьте hook для повторяющегося поведения",
    teaches: "Hooks делают поведение автоматическим вместо того, чтобы вы должны были помнить просить об этом. Опишите триггер и действие, и Claude пишет конфигурацию [hook](/docs/ru/hooks)."
  },
  "connect-a-tool-with": {
    title: "Подключите инструмент с MCP",
    teaches: "Подключите источник один раз вместо вставки данных в каждую сессию. После настройки [MCP](/docs/ru/mcp), Claude читает из инструмента напрямую, когда вы спрашиваете об этом."
  },
  "capture-what-to-remember": {
    title: "Захватите, что помнить в следующий раз",
    teaches: "Спросите перед тем, как забыть. Claude знает, что ему пришлось разобраться в этой сессии, и предлагает записи [CLAUDE.md](/docs/ru/memory), чтобы следующая сессия начиналась с этого контекста."
  }
};

<PromptLibrary text={text} labels={labels} tagLabels={tagLabels} phaseLabels={phaseLabels} sourceLabels={sourceLabels} catLabels={catLabels} />

<h2 id="what-makes-these-prompts-work">
  Что делает эти промпты работающими
</h2>

Промпты выше имеют несколько общих закономерностей. Их распознавание помогает вам адаптировать любой промпт здесь к вашей собственной задаче.

**Опишите результат, а не шаги.** Скажите, что вы хотите, и позвольте Claude найти файлы. Промпт ниже работает без указания ни одного пути файла.

```text theme={null}
add rate limiting to the public API and make sure existing tests still pass
```

**Дайте ему способ проверить свою работу.** Спросите запустить, протестировать, сравнить или проверить в одном промпте, чтобы Claude повторял вместо остановки после одной попытки.

```text theme={null}
write the migration, run it against the dev database, and confirm the schema matches
```

**Укажите на ссылку.** Назовите существующий файл, тест или паттерн для соответствия, чтобы новый код был согласован с тем, что у вас уже есть.

```text theme={null}
add a settings page that follows the same layout as the profile page
```

**Укажите измеримую цель.** Когда цель — производительность или покрытие, дайте метрику и пороговое значение, чтобы завершение было однозначным.

```text theme={null}
get the bundle size under 200KB and show me what you removed
```

**Дайте ему артефакт.** Вставьте ошибки, логи, скриншоты и вывод плана прямо в промпт, или введите `@`, чтобы ссылаться на файл. Claude читает источник вместо вашего описания.

```text theme={null}
why is the build failing? @build.log
```

**Скажите, как вы хотите ответ.** Назовите формат, длину или аудиторию, чтобы объяснение соответствовало тому, как вы его будете использовать. Чтобы сделать формат по умолчанию для каждого ответа, установите [стиль вывода](/docs/ru/output-styles).

```text theme={null}
explain how the payment retry logic works as an HTML page with a diagram, then open it in my browser
```

Для подробнее о каждом паттерне, см. [лучшие практики](/docs/ru/best-practices).

<h2 id="where-these-come-from">
  Откуда они берутся
</h2>

Эти промпты основаны на паттернах из опубликованных ресурсов Anthropic. Каждая карточка ссылается на свой источник:

* [Типичные рабочие процессы](/docs/ru/common-workflows): пошаговые руководства для основных задач
* [Лучшие практики](/docs/ru/best-practices): паттерны промптирования и настройка проекта
* [Как команды Anthropic используют Claude Code](https://claude.com/blog/how-anthropic-teams-use-claude-code): реальные рабочие процессы из команд инженерии, продукта, дизайна и данных, с глубокими погружениями в [legal](https://claude.com/blog/how-anthropic-uses-claude-legal), [marketing](https://claude.com/blog/how-anthropic-uses-claude-marketing) и [cybersecurity](https://claude.com/blog/how-anthropic-uses-claude-cybersecurity)
* [Руководство по масштабированию агентного кодирования](https://resources.anthropic.com/hubfs/Scaling%20agentic%20coding%20across%20your%20organization.pdf): руководство по внедрению на уровне предприятия

Для видеопрохождений этих паттернов, см. бесплатный курс [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action) на Anthropic Academy.

<h2 id="related-resources">
  Связанные ресурсы
</h2>

Промпты на этой странице — это отправные точки. Как только один работает для вашего проекта, следующий шаг — сделать его повторяемым: сохраните его как [skill](/docs/ru/skills), чтобы кто-либо из вашей команды мог запустить его как `/command`, и запишите соглашения, которые Claude изучил, в [CLAUDE.md](/docs/ru/memory), чтобы каждая сессия начиналась с этого контекста вместо того, чтобы Claude переучивался. Для более крупных или рискованных изменений [режим плана](/docs/ru/permission-modes#analyze-before-you-edit-with-plan-mode) показывает вам список файлов перед любыми редактированиями.

Если вы вводите Claude Code в команду, см. [администрирование](/docs/ru/admin-setup) для управляемых параметров и политики, и [затраты и использование](/docs/ru/costs) для того, как эта работа выставляется в счёт в вашем плане.
