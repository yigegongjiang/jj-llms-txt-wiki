> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Biblioteca de prompts

> Copie y pegue prompts para Claude Code, etiquetados por tarea y rol.

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

Esta es una biblioteca de prompts para copiar en Claude Code. Úsela para explorar formas de trabajo que no ha probado, o cuando no está seguro de dónde comenzar.

Los prompts se recopilan de varias guías de Anthropic, incluyendo [Flujos de trabajo comunes](/docs/es/common-workflows), [Mejores prácticas](/docs/es/best-practices), y [Cómo los equipos de Anthropic usan Claude Code](https://claude.com/blog/how-anthropic-teams-use-claude-code). Son puntos de partida en lugar de scripts. Abra **Por qué funciona esto** bajo cualquier prompt para ver el patrón detrás de él para que pueda escribir el suyo propio.

export const labels = {
  startHere: "Comience aquí",
  startHereHeader: "Cinco prompts para probar primero",
  showAll: "Mostrar todos los {n} prompts",
  search: "Buscar prompts…",
  clear: "Limpiar",
  prompt: "prompt",
  prompts: "prompts",
  noMatch: "Ningún prompt coincide",
  fillAndCopy: "Rellenar y copiar",
  copyThis: "Copiar este prompt",
  hintBefore: "Escriba en el",
  hintChip: "resaltado",
  hintAfter: "campos para personalizar, luego copie.",
  copy: "Copiar",
  copied: "Copiado",
  whyWorks: "Por qué funciona esto",
  makeItStick: "Hacerlo permanente",
  from: "De",
  paste: {
    mockup: "Pegue, arrastre o @-mencione su imagen de maqueta, luego envíe esto:",
    design: "Pegue, arrastre o @-mencione su imagen de diseño, luego envíe esto:",
    screenshot: "Pegue, arrastre o @-mencione su captura de pantalla, luego envíe esto:",
    plan: "Pegue su salida de plan en el prompt primero, luego envíe esto:",
    error: "Pegue la salida de error en el prompt primero, luego envíe esto:",
    csv: "Arrastre su archivo al prompt, o reemplace la ruta a continuación con una @-mención de la suya:"
  },
  needsLabel: "Necesita",
  needs: {
    tracker: "su rastreador de problemas agregado como un [conector de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) o [servidor MCP](/docs/es/mcp).",
    gh: "el [CLI de gh](https://cli.github.com) autenticado, o GitHub agregado como un [conector de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai).",
    browser: "una forma para que Claude renderice y capture una captura de pantalla del resultado. La [aplicación de escritorio](/docs/es/desktop#preview-your-app) tiene esto integrado. En la terminal, instale la [extensión de Chrome](/docs/es/chrome) o un servidor MCP de [Playwright](/docs/es/mcp).",
    db: "su almacén de datos o almacén de registros agregado como un [conector de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) o [servidor MCP](/docs/es/mcp)."
  }
};

export const tagLabels = {
  understand: "Entender",
  plan: "Plan",
  prototype: "Prototipo",
  build: "Construir",
  test: "Prueba",
  refactor: "Refactorizar",
  review: "Revisar",
  steer: "Dirigir",
  debug: "Depurar",
  git: "Git",
  release: "Lanzamiento",
  data: "Datos",
  automate: "Automatizar",
  pm: "Producto",
  design: "Diseño",
  docs: "Documentación",
  marketing: "Marketing",
  security: "Seguridad",
  ops: "En guardia"
};

export const phaseLabels = {
  discover: "Descubrir",
  design: "Diseñar",
  build: "Construir",
  ship: "Enviar",
  operate: "Operar"
};

export const sourceLabels = {
  workflows: "Flujos de trabajo comunes",
  teams: "Cómo los equipos de Anthropic usan Claude Code",
  legal: "Cómo Anthropic usa Claude en Legal",
  cybersecurity: "Cómo Anthropic usa Claude en Ciberseguridad",
  "best-practices": "Mejores prácticas",
  ebook: "Guía de codificación agéntica escalable"
};

export const catLabels = {
  Onboard: "Incorporar",
  Understand: "Entender",
  Plan: "Plan",
  Prototype: "Prototipo",
  Implement: "Implementar",
  Test: "Prueba",
  Refactor: "Refactorizar",
  Review: "Revisar",
  Steer: "Dirigir",
  Git: "Git",
  Release: "Lanzamiento",
  Debug: "Depurar",
  Incident: "Incidente",
  Data: "Datos",
  Automate: "Automatizar"
};

export const text = {
  "get-oriented-in-a": {
    title: "Orientarse en un repositorio nuevo",
    teaches: "Describa lo que desea saber, no qué archivos leer. Claude explora el proyecto por su cuenta y devuelve un resumen de cómo encaja todo.",
    next: "Ejecute `/init` para configurar `CLAUDE.md` para que Claude recuerde esto en cada sesión"
  },
  "explain-unfamiliar-code": {
    title: "Explicar código desconocido",
    teaches: "Nombre el archivo y diga en qué formato desea la respuesta. Cambie la página HTML por un diagrama, puntos de viñeta, o lo que se ajuste a cómo aprende.",
    next: "Establezca un estilo de salida para que Claude siempre explique en su formato preferido"
  },
  "find-where-something-happens": {
    title: "Encontrar dónde sucede algo",
    teaches: "Busque por comportamiento en lugar de por nombre de archivo. La búsqueda funciona incluso cuando no sabe cómo se llama el archivo o en qué directorio vive."
  },
  "see-what-depends-on": {
    title: "Verificar qué se rompe antes de eliminar",
    teaches: "Pregunte antes de eliminar cualquier cosa. La lista de llamadores y efectos posteriores le dice si está mirando una limpieza de una línea o un cambio que necesita coordinar."
  },
  "trace-how-code-evolved": {
    title: "Rastrear cómo evolucionó el código",
    teaches: "Señale el historial de commits cuando la pregunta es por qué, no qué. Claude lee el registro y blame para cualquier control de versiones que use y explica las decisiones detrás de la implementación actual."
  },
  "scope-a-change-before": {
    title: "Definir el alcance de un cambio antes de comenzar",
    teaches: "Dimensione el trabajo antes de comprometerse con una hoja de ruta. La lista de archivos le dice si está mirando un componente o un cambio transversal."
  },
  "ask-the-codebase-a": {
    title: "Hacer una pregunta de producto a la base de código",
    teaches: "Indique su rol para que la respuesta esté al nivel correcto. Claude explica qué hace realmente el producto desde el código fuente, sin que necesite leerlo.",
    next: "Establezca un estilo de salida para que Claude siempre presente respuestas a este nivel"
  },
  "plan-a-multi-file": {
    title: "Planificar un cambio de varios archivos antes de tocar código",
    teaches: "Agregar \"no editar aún\" separa la exploración de los cambios, para que vea el enfoque antes de que se mueva cualquier código. Para hacer que el modo plan sea el predeterminado en cada prompt, presione Shift+Tab para [plan mode](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode)."
  },
  "draft-a-spec-by": {
    title: "Redactar una especificación por entrevista",
    teaches: "Pida ser entrevistado en lugar de escribir la especificación usted mismo. Claude le hace preguntas estructuradas hasta que los requisitos estén completos, luego escribe el resultado en un archivo.",
    next: "Guarde sus preguntas de entrevista como una habilidad `/spec` para que cada especificación comience de la misma manera"
  },
  "turn-a-meeting-into": {
    title: "Convertir una reunión en tickets",
    teaches: "Omita el paso de transcripción. Claude extrae elementos de acción de la entrada no estructurada y los escribe directamente en su rastreador a través de [MCP](/docs/es/mcp), para que revise los tickets, no la transcripción.",
    next: "Guarde esto como una habilidad `/tickets`"
  },
  "map-edge-cases-before": {
    title: "Mapear casos extremos antes de construir",
    teaches: "Pregunte qué falta, no qué hay. Claude enumera los estados de error, estados vacíos y casos extremos que un diseño de ruta feliz tiende a omitir."
  },
  "turn-a-mockup-into": {
    title: "Convertir una maqueta en un prototipo funcional",
    teaches: "Un prototipo interactivo responde preguntas que una maqueta estática no puede. Entregue el código funcional a ingeniería en lugar de explicar las interacciones en un documento."
  },
  "implement-from-a-screenshot": {
    title: "Implementar desde una captura de pantalla y auto-verificar",
    teaches: "Esto le da a Claude un bucle de verificación: renderiza, compara contra la imagen de origen, e itera sin que usted señale cada brecha.",
    next: "Use `/goal` para mantener a Claude iterando hasta que las capturas de pantalla coincidan"
  },
  "follow-an-existing-pattern": {
    title: "Seguir un patrón existente",
    teaches: "Señale código que ya le guste. Sin una referencia, Claude usa las mejores prácticas generales de forma predeterminada. Con una, coincide con las convenciones que su base de código realmente usa.",
    next: "Pida a Claude que escriba el patrón que siguió en `CLAUDE.md` para que las sesiones futuras lo coincidan sin la referencia"
  },
  "add-a-small-well": {
    title: "Agregar una característica pequeña y bien definida",
    teaches: "Indique las entradas y salidas, no cómo construirla. Claude encuentra dónde vive código similar y agrega el suyo junto a él."
  },
  "build-a-small-internal": {
    title: "Construir una pequeña herramienta interna desde cero",
    teaches: "No necesita un proyecto, un marco o un paso de compilación. Describa la herramienta y pida a Claude que la abra para que la vea funcionando inmediatamente."
  },
  "work-an-issue-end": {
    title: "Trabajar un problema de principio a fin",
    teaches: "Dé el número del problema, no un resumen. Claude lee el ticket completo en sí, por lo que los requisitos que olvidaría mencionar se transmiten, y valida el cambio antes de informar."
  },
  "find-and-update-copy": {
    title: "Encontrar y actualizar texto en toda la base de código",
    teaches: "Pida variantes y diga qué omitir. Claude encuentra frases que una búsqueda literal perdería y deja intactos los accesorios de prueba e historial, para que revise solo el texto que los usuarios realmente ven."
  },
  "draft-from-past-examples": {
    title: "Redactar un documento a partir de ejemplos anteriores",
    teaches: "Señale una carpeta de trabajo terminado en lugar de describir su estilo. Claude aprende la estructura y la voz de lo que ya ha enviado, para que el primer borrador se lea como uno de los suyos.",
    next: "Guarde la voz como una habilidad para que cada borrador comience allí"
  },
  "write-tests-run-them": {
    title: "Escribir pruebas, ejecutarlas, corregir fallos",
    teaches: "Pida escribir, ejecutar y corregir juntos para que Claude itere sin detenerse para instrucciones.",
    next: "Ejecute `/init` para que Claude aprenda automáticamente su comando de prueba"
  },
  "drive-implementation-from-tests": {
    title: "Impulsar la implementación desde pruebas",
    teaches: "Desarrollo dirigido por pruebas: las pruebas definen cuándo se completa el trabajo, y Claude itera en la implementación hasta que pasen."
  },
  "fill-gaps-from-a": {
    title: "Llenar brechas de un informe de cobertura",
    teaches: "Señale el informe de cobertura en lugar de adivinar qué no se prueba. Claude lee los números reales y escribe pruebas para los archivos que más los necesitan.",
    next: "Establezca esto como un `/goal` para que Claude siga escribiendo pruebas hasta que la cobertura alcance el objetivo"
  },
  "port-code-between-languages": {
    title: "Portar código a otro idioma",
    teaches: "Diga qué preservar, no solo el idioma de destino. Nombrar la API o el comportamiento que debe permanecer igual le da a Claude un contrato para verificar el puerto."
  },
  "generate-docs-for-code": {
    title: "Generar documentación para código sin documentar",
    teaches: "Nombre el alcance y el formato. Claude encuentra lo que falta y coincide con el estilo de comentario ya en el archivo, para que la nueva documentación se lea como el resto."
  },
  "migrate-a-pattern-across": {
    title: "Migrar un patrón en toda la base de código",
    teaches: "Describa el patrón antiguo y el nuevo. Pedir a Claude que identifique primero cada lugar significa que los sitios de llamada se enumeran en la respuesta, para que pueda verificar que ninguno se perdió."
  },
  "optimize-against-a-measurable": {
    title: "Optimizar contra un objetivo medible",
    teaches: "Indicar la métrica y el objetivo le da a Claude una definición clara de cuándo está hecho.",
    next: "Establezca esto como un `/goal` para que Claude siga midiendo e iterando hasta que alcance el número"
  },
  "fix-a-precise-visual": {
    title: "Corregir un error visual preciso",
    teaches: "La retroalimentación visual precisa obtiene una corrección precisa. Indique el elemento exacto, la medida y la ventana gráfica.",
    next: "Agregue una herramienta de vista previa para que Claude capture una captura de pantalla y verifique la corrección en sí"
  },
  "review-your-changes-before": {
    title: "Revisar sus cambios antes de confirmar",
    teaches: "Detecte problemas mientras aún son baratos de corregir. Claude lee los archivos modificados en su totalidad, no solo las líneas de diferencia, para que detecte problemas que una auto-revisión rápida pierde.",
    next: "Ejecute `/code-review` para la misma verificación en un comando"
  },
  "review-a-pull-request": {
    title: "Revisar una solicitud de extracción",
    teaches: "Claude revisa con toda la base de código en contexto, no solo la diferencia. Lee el código modificado y lo que llama, para que detecte problemas que una revisión solo de diferencia perdería.",
    next: "Active esto para cada PR con Code Review"
  },
  "review-infrastructure-changes-before": {
    title: "Revisar cambios de infraestructura antes de aplicar",
    teaches: "La salida del plan es densa y difícil de escanear. Pegarla le obtiene un resumen en lenguaje simple de lo que realmente va a cambiar antes de aplicarlo."
  },
  "run-a-security-review": {
    title: "Ejecutar una revisión de seguridad con un subagente",
    teaches: "Un [subagente](/docs/es/sub-agents) ejecuta la auditoría en su propia ventana de contexto e informa un resumen, para que una revisión de seguridad larga no llene su sesión principal. El subagente de propósito general integrado maneja esto sin configuración adicional.",
    next: "Configure un subagente dedicado de revisión de seguridad que todo su equipo pueda usar"
  },
  "review-content-before-sending": {
    title: "Detectar problemas antes de la revisión formal",
    teaches: "Obtenga un primer paso antes de que un humano dedique tiempo a ello. Nombre las preocupaciones que desea verificar para que la revisión sea enfocada, luego corrija lo que encuentra y envíe un borrador más limpio.",
    next: "Capture su lista de verificación de revisión como una habilidad que todo su equipo pueda ejecutar"
  },
  "course-correct-a-wrong": {
    title: "Corregir un enfoque incorrecto",
    teaches: "Nombre la restricción que Claude perdió, no solo que está mal. Una razón específica le da a Claude una restricción concreta para satisfacer en el reintento, en lugar de adivinar de nuevo.",
    next: "Presione `Esc` dos veces para abrir el menú de rebobinado y restaurar código y conversación para que el reintento comience limpio"
  },
  "narrow-the-scope-of": {
    title: "Reducir el alcance de un cambio",
    teaches: "Cuando la dirección es correcta pero el cambio fue demasiado amplio, pida a Claude que mantenga parte de él en lugar de rebobinar todo. Un límite establecido evita que una pequeña corrección se convierta en una refactorización."
  },
  "turn-a-correction-into": {
    title: "Convertir una corrección en una regla",
    teaches: "Una corrección en el chat no se comparte con su equipo. Una regla en el [CLAUDE.md](/docs/es/memory) del proyecto se comparte una vez que la confirma, y Claude la lee al inicio de cada sesión.",
    next: "Abra `/memory` para revisar lo que Claude escribió"
  },
  "resolve-merge-conflicts": {
    title: "Resolver conflictos de fusión",
    teaches: "Diga qué estado desea, no qué marcadores mantener. Pedir el razonamiento hace que la fusión sea revisable en lugar de una caja negra."
  },
  "commit-with-a-generated": {
    title: "Confirmar con un mensaje generado",
    teaches: "Deje que Claude derive el mensaje de la diferencia. Coincide con el estilo de commit existente de su repositorio."
  },
  "open-a-pull-request": {
    title: "Abrir una solicitud de extracción desde un ticket",
    teaches: "Omita el cambio de contexto entre rastreador, editor y GitHub. Un prompt lee la especificación, realiza el cambio y abre el PR."
  },
  "draft-release-notes-from": {
    title: "Redactar notas de lanzamiento del historial de git",
    teaches: "Dé dos puntos de referencia y la estructura que desea. Claude lee el registro de commits entre ellos y redacta un registro de cambios que puede editar.",
    next: "Guarde esto como una habilidad `/changelog`"
  },
  "write-a-ci-workflow": {
    title: "Escribir un flujo de trabajo de CI",
    teaches: "Describa cuándo debe ejecutarse y qué debe hacer; el YAML se genera para usted, coincidiendo con los comandos de compilación y prueba de su proyecto."
  },
  "find-and-fix-a": {
    title: "Encontrar y corregir una prueba fallida",
    teaches: "Describa el síntoma; no necesita saber qué archivo está roto. Claude ejecuta la prueba para ver el fallo, lo rastrea en la fuente y lo corrige."
  },
  "investigate-a-reported-error": {
    title: "Investigar un error reportado",
    teaches: "Describa el síntoma y la ubicación; Claude lee la ruta de código relevante y rastrea las causas probables. Pegue seguimientos de pila o registros si los tiene.",
    next: "Ponga un enlace profundo en su runbook que abra Claude con este prompt rellenado previamente"
  },
  "fix-a-build-error": {
    title: "Corregir un error de compilación en la raíz",
    teaches: "Pedir la causa raíz y la verificación evita parches de nivel superficial que suprimen el error sin corregirlo."
  },
  "investigate-a-production-incident": {
    title: "Investigar un incidente de producción",
    teaches: "Enumere las fuentes de evidencia para correlacionar, no los pasos a seguir. Claude lee registros, historial de git y configuración juntos para reducir la causa.",
    next: "Conecte Sentry o su almacén de registros a través de MCP"
  },
  "query-logs-in-plain": {
    title: "Consultar registros en inglés simple",
    teaches: "Haga la pregunta en lugar de escribir el SQL. Claude construye la consulta, la ejecuta contra sus registros conectados y muestra tanto la consulta como el resultado para que pueda verificar qué se ejecutó."
  },
  "diagnose-from-a-console": {
    title: "Diagnosticar desde una captura de pantalla de consola",
    teaches: "Las consolas en la nube le muestran el problema pero no los comandos para corregirlo. Claude lee la captura de pantalla y traduce el panel a los comandos kubectl, gcloud o aws para ejecutar."
  },
  "analyze-a-data-file": {
    title: "Analizar un archivo de datos",
    teaches: "Una pregunta única no necesita un script único. Señale un archivo en su carpeta de proyecto y Claude lo lee directamente, encuentra los patrones y escribe la salida donde le pida.",
    next: "Conecte la fuente de datos a través de MCP en lugar de exportar archivos"
  },
  "generate-variations-from-performance": {
    title: "Generar variaciones a partir de datos de rendimiento",
    teaches: "Indique la restricción al inicio para que la generación se mantenga dentro del límite. Claude lee las métricas, elige qué reemplazar y produce alternativas que se ajusten.",
    next: "Conecte la plataforma de anuncios a través de MCP en lugar de exportar un archivo"
  },
  "turn-a-recurring-task": {
    title: "Convertir una tarea recurrente en una habilidad",
    teaches: "Nombre los pasos una vez; reutilícelos como un comando. Claude escribe una [habilidad](/docs/es/skills) que cualquiera en su equipo pueda ejecutar."
  },
  "add-a-hook-for": {
    title: "Agregar un hook para comportamiento repetido",
    teaches: "Los hooks hacen que un comportamiento sea automático en lugar de algo que tenga que recordar pedir. Describa el disparador y la acción y Claude escribe la configuración del [hook](/docs/es/hooks)."
  },
  "connect-a-tool-with": {
    title: "Conectar una herramienta con MCP",
    teaches: "Conecte la fuente una vez en lugar de pegar datos en cada sesión. Después de la configuración de [MCP](/docs/es/mcp), Claude lee de la herramienta directamente cuando pregunta sobre ella."
  },
  "capture-what-to-remember": {
    title: "Capturar qué recordar para la próxima vez",
    teaches: "Pregunte antes de olvidar. Claude sabe qué tuvo que descubrir en esta sesión y propone entradas de [CLAUDE.md](/docs/es/memory) para que la próxima sesión comience con ese contexto."
  }
};

<PromptLibrary text={text} labels={labels} tagLabels={tagLabels} phaseLabels={phaseLabels} sourceLabels={sourceLabels} catLabels={catLabels} />

<h2 id="what-makes-these-prompts-work">
  Qué hace que estos prompts funcionen
</h2>

Los prompts anteriores comparten algunos patrones. Reconocerlos le ayuda a adaptar cualquier prompt aquí a su propia tarea.

**Describa el resultado, no los pasos.** Diga qué desea y deje que Claude encuentre los archivos. El prompt a continuación funciona sin nombrar una sola ruta de archivo.

```text theme={null}
agregar limitación de velocidad a la API pública y asegurarse de que las pruebas existentes aún pasen
```

**Déle una forma de verificar su propio trabajo.** Pida ejecutar, probar, comparar o verificar en el mismo prompt para que Claude itere en lugar de detenerse después de un intento.

```text theme={null}
escribir la migración, ejecutarla contra la base de datos de desarrollo y confirmar que el esquema coincida
```

**Señale una referencia.** Nombre un archivo, prueba o patrón existente para que coincida para que el nuevo código sea consistente con lo que ya tiene.

```text theme={null}
agregar una página de configuración que siga el mismo diseño que la página de perfil
```

**Indique el objetivo medible.** Cuando el objetivo es rendimiento o cobertura, dé la métrica y el umbral para que la finalización sea inequívoca.

```text theme={null}
obtener el tamaño del paquete por debajo de 200KB y mostrarme qué eliminó
```

**Déle el artefacto.** Pegue errores, registros, capturas de pantalla y salida de plan directamente en el prompt, o escriba `@` para hacer referencia a un archivo. Claude lee la fuente en lugar de su descripción de ella.

```text theme={null}
¿por qué falla la compilación? @build.log
```

**Diga cómo desea la respuesta.** Nombre el formato, la longitud o la audiencia para que la explicación se ajuste a cómo la usará. Para hacer que un formato sea el predeterminado para cada respuesta, establezca un [estilo de salida](/docs/es/output-styles).

```text theme={null}
explicar cómo funciona la lógica de reintento de pago como una página HTML con un diagrama, luego abrirla en mi navegador
```

Para más información sobre cada patrón, consulte [mejores prácticas](/docs/es/best-practices).

<h2 id="where-these-come-from">
  De dónde vienen estos
</h2>

Estos prompts se basan en patrones de recursos publicados de Anthropic. Cada tarjeta vincula a su fuente:

* [Flujos de trabajo comunes](/docs/es/common-workflows): guías paso a paso para las tareas principales
* [Mejores prácticas](/docs/es/best-practices): patrones de prompting y configuración de proyectos
* [Cómo los equipos de Anthropic usan Claude Code](https://claude.com/blog/how-anthropic-teams-use-claude-code): flujos de trabajo reales de equipos de ingeniería, producto, diseño y datos, con análisis profundos sobre [legal](https://claude.com/blog/how-anthropic-uses-claude-legal), [marketing](https://claude.com/blog/how-anthropic-uses-claude-marketing), y [ciberseguridad](https://claude.com/blog/how-anthropic-uses-claude-cybersecurity)
* [Guía de codificación agéntica escalable](https://resources.anthropic.com/hubfs/Scaling%20agentic%20coding%20across%20your%20organization.pdf): la guía de adopción empresarial

Para tutoriales en video de estos patrones, consulte el curso gratuito [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action) en Anthropic Academy.

<h2 id="related-resources">
  Recursos relacionados
</h2>

Los prompts en esta página son puntos de partida. Una vez que uno funciona para su proyecto, el siguiente paso es hacerlo repetible: guárdelo como una [habilidad](/docs/es/skills) para que cualquiera en su equipo pueda ejecutarlo como un `/comando`, y registre las convenciones que Claude aprendió en [CLAUDE.md](/docs/es/memory) para que cada sesión comience con ese contexto en lugar de que Claude lo reaprendan. Para cambios más grandes o más riesgosos, [plan mode](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode) le muestra la lista de archivos antes de que ocurran ediciones.

Si está introduciendo Claude Code en un equipo, consulte [administración](/docs/es/admin-setup) para configuración y política administrada, y [costos y uso](/docs/es/costs) para saber cómo se factura este trabajo en su plan.
