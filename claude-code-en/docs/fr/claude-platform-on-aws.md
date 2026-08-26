> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code sur Claude Platform on AWS

> Configurez Claude Code pour utiliser l'API Claude exploitée par Anthropic avec l'authentification AWS, le contrôle d'accès IAM et la facturation AWS Marketplace.

export const ContactSalesCard = ({surface}) => {
  const utm = content => `utm_source=claude_code&utm_medium=docs&utm_content=${surface}_${content}`;
  const iconArrowRight = (size = 13) => <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
      <line x1="5" y1="12" x2="19" y2="12" />
      <polyline points="12 5 19 12 12 19" />
    </svg>;
  const STYLES = `
.cc-cs {
  --cs-slate: #141413;
  --cs-clay: #d97757;
  --cs-clay-deep: #c6613f;
  --cs-gray-000: #ffffff;
  --cs-gray-700: #3d3d3a;
  --cs-border-default: rgba(31, 30, 29, 0.15);
  font-family: inherit;
}
.dark .cc-cs {
  --cs-slate: #f0eee6;
  --cs-gray-000: #262624;
  --cs-gray-700: #bfbdb4;
  --cs-border-default: rgba(240, 238, 230, 0.14);
}
.cc-cs-card {
  display: flex; align-items: center; justify-content: space-between;
  gap: 16px; padding: 14px 16px; margin: 0;
  background: var(--cs-gray-000); border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; flex-wrap: wrap;
}
.cc-cs-text { font-size: 13px; color: var(--cs-gray-700); line-height: 1.5; flex: 1; min-width: 240px; }
.cc-cs-text strong { font-weight: 550; color: var(--cs-slate); }
.cc-cs-actions { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
.cc-cs-btn-clay {
  display: inline-flex; align-items: center; gap: 8px;
  background: var(--cs-clay-deep); color: #fff; border: none;
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
  transition: background-color 0.15s; white-space: nowrap;
}
.cc-cs-btn-clay:hover { background: var(--cs-clay); }
.cc-cs-btn-ghost {
  display: inline-flex; align-items: center; gap: 8px;
  background: transparent; color: var(--cs-gray-700);
  border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
}
.cc-cs-btn-ghost:hover { background: rgba(0, 0, 0, 0.04); }
.dark .cc-cs-btn-ghost:hover { background: rgba(255, 255, 255, 0.04); }
@media (max-width: 720px) {
  .cc-cs-actions { width: 100%; }
}
`;
  return <div className="cc-cs not-prose">
      <style>{STYLES}</style>
      <div className="cc-cs-card">
        <div className="cc-cs-text">
          <strong>Deploying Claude Code across your organization?</strong> Talk to sales about enterprise plans, SSO, and centralized billing.
        </div>
        <div className="cc-cs-actions">
          <a href={`https://claude.com/pricing?${utm('view_plans')}#plans-business`} className="cc-cs-btn-ghost">
            View plans
          </a>
          <a href={`https://claude.com/contact-sales?${utm('contact_sales')}`} className="cc-cs-btn-clay">
            Contact sales {iconArrowRight()}
          </a>
        </div>
      </div>
    </div>;
};

export const Experiment = ({flag, treatment, children}) => {
  const VID_KEY = 'exp_vid';
  const CONSENT_COUNTRIES = new Set(['AT', 'BE', 'BG', 'HR', 'CY', 'CZ', 'DK', 'EE', 'FI', 'FR', 'DE', 'GR', 'HU', 'IE', 'IT', 'LV', 'LT', 'LU', 'MT', 'NL', 'PL', 'PT', 'RO', 'SK', 'SI', 'ES', 'SE', 'RE', 'GP', 'MQ', 'GF', 'YT', 'BL', 'MF', 'PM', 'WF', 'PF', 'NC', 'AW', 'CW', 'SX', 'FO', 'GL', 'AX', 'GB', 'UK', 'AI', 'BM', 'IO', 'VG', 'KY', 'FK', 'GI', 'MS', 'PN', 'SH', 'TC', 'GG', 'JE', 'IM', 'CA', 'BR', 'IN']);
  const fnv1a = s => {
    let h = 0x811c9dc5;
    for (let i = 0; i < s.length; i++) {
      h ^= s.charCodeAt(i);
      h += (h << 1) + (h << 4) + (h << 7) + (h << 8) + (h << 24);
    }
    return h >>> 0;
  };
  const bucket = (seed, vid) => fnv1a(fnv1a(seed + vid) + '') % 10000 < 5000 ? 'control' : 'treatment';
  const [decision] = useState(() => {
    const params = new URLSearchParams(location.search);
    const preBucketed = document.documentElement.dataset['gb_' + flag.replace(/-/g, '_')];
    const force = params.get('gb-force');
    if (force) {
      for (const p of force.split(',')) {
        const [k, v] = p.split(':');
        if (k === flag) return {
          variant: v || 'treatment',
          track: false
        };
      }
    }
    if (navigator.globalPrivacyControl) {
      return {
        variant: 'control',
        track: false
      };
    }
    const prefsMatch = document.cookie.match(/(?:^|; )anthropic-consent-preferences=([^;]+)/);
    if (prefsMatch) {
      try {
        if (JSON.parse(decodeURIComponent(prefsMatch[1])).analytics !== true) {
          return {
            variant: 'control',
            track: false
          };
        }
      } catch {
        return {
          variant: 'control',
          track: false
        };
      }
    } else {
      const country = params.get('country')?.toUpperCase() || (document.cookie.match(/(?:^|; )cf_geo=([A-Z]{2})/) || [])[1];
      if (!country || CONSENT_COUNTRIES.has(country)) {
        return {
          variant: 'control',
          track: false
        };
      }
    }
    let vid;
    try {
      const ajsMatch = document.cookie.match(/(?:^|; )ajs_anonymous_id=([^;]+)/);
      if (ajsMatch) {
        vid = decodeURIComponent(ajsMatch[1]).replace(/^"|"$/g, '');
      } else {
        vid = localStorage.getItem(VID_KEY);
        if (!vid) {
          vid = crypto.randomUUID();
        }
        document.cookie = `ajs_anonymous_id=${vid}; domain=.claude.com; path=/; Secure; SameSite=Lax; max-age=31536000`;
      }
      try {
        localStorage.setItem(VID_KEY, vid);
      } catch {}
    } catch {
      return {
        variant: 'control',
        track: false
      };
    }
    const variant = preBucketed === '1' ? 'treatment' : preBucketed === '0' ? 'control' : bucket(flag, vid);
    return {
      variant,
      track: true,
      vid
    };
  });
  useEffect(() => {
    if (!decision.track) return;
    fetch('https://api.anthropic.com/api/event_logging/v2/batch', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-service-name': 'claude_code_docs'
      },
      body: JSON.stringify({
        events: [{
          event_type: 'GrowthbookExperimentEvent',
          event_data: {
            device_id: decision.vid,
            anonymous_id: decision.vid,
            timestamp: new Date().toISOString(),
            experiment_id: flag,
            variation_id: decision.variant === 'treatment' ? 1 : 0,
            environment: 'production'
          }
        }]
      }),
      keepalive: true
    }).catch(() => {});
  }, []);
  return decision.variant === 'treatment' ? treatment : children;
};

<Experiment flag="docs-contact-sales-cta" treatment={<ContactSalesCard surface="claude_platform_on_aws" />} />

Claude Platform on AWS est l'API Claude exploitée par Anthropic avec l'authentification AWS, le contrôle d'accès IAM et la facturation AWS Marketplace. Les requêtes atteignent directement l'API d'Anthropic, vous bénéficiez donc des mêmes modèles et fonctionnalités API que l'[API Claude](https://platform.claude.com/docs) selon le même calendrier de publication. Les fonctionnalités côté client que Claude Code active via le service de drapeaux de fonctionnalités d'Anthropic, comme [`/loop` auto-rythme](/docs/fr/scheduled-tasks#let-claude-choose-the-interval), sont désactivées par défaut, et l'[outil conseiller](/docs/fr/advisor) n'est pas disponible. Consultez la [matrice de disponibilité des fonctionnalités](/docs/fr/feature-availability#summary-by-provider) pour la liste complète. Vous vous authentifiez avec les identifiants AWS ou une clé API d'espace de travail, et vous payez via AWS Marketplace.

Utilisez ce guide pour pointer Claude Code vers un espace de travail que vous avez déjà provisionné via Claude Platform on AWS. Pour l'abonnement AWS et la configuration de l'espace de travail qui précèdent cela, consultez la [documentation Claude Platform on AWS](https://platform.claude.com/docs/en/build-with-claude/claude-platform-on-aws).

<Note>
  L'abonnement via AWS Marketplace provisionne une nouvelle organisation Anthropic liée à votre compte AWS. Cette organisation est distincte de toute organisation que vous avez déjà avec Anthropic, et les identifiants ne se transfèrent pas entre elles. Utilisez l'ID d'espace de travail et les clés API de l'organisation liée à AWS, et non d'un compte Claude Console préexistant.
</Note>

<h2 id="prerequisites">
  Prérequis
</h2>

Avant de configurer Claude Code, vous avez besoin de :

* Un abonnement actif à Claude Platform on AWS via AWS Marketplace
* Un espace de travail dans votre organisation Anthropic liée à AWS, avec son ID d'espace de travail
* Un principal IAM avec la permission d'invoquer le service Anthropic, ou une clé API limitée à l'espace de travail
* Des identifiants AWS dans votre environnement, dans `~/.aws/credentials`, ou à partir d'un rôle IAM attaché si vous souhaitez l'authentification SigV4. L'AWS CLI n'est requis que pour le flux de connexion SSO.

<h2 id="setup">
  Configuration
</h2>

<h3 id="1-configure-aws-credentials">
  1. Configurez les identifiants AWS
</h3>

Claude Code prend en charge deux méthodes d'authentification pour Claude Platform on AWS. Choisissez la méthode qui correspond à la façon dont votre équipe gère l'accès.

**Option A : Identifiants AWS avec SigV4**

Claude Code signe les requêtes avec SigV4 en utilisant la chaîne d'identifiants AWS standard : variables d'environnement, identifiants partagés dans `~/.aws/credentials`, rôles IAM, sessions AWS SSO et toute autre source que le SDK AWS prend en charge.

Pour une utilisation locale, connectez-vous avec l'AWS CLI avant de démarrer Claude Code. L'exemple ci-dessous utilise un profil SSO, mais toute méthode qui produit des identifiants aux emplacements standard fonctionne.

```bash theme={null}
aws sso login --profile my-profile
export AWS_PROFILE=my-profile
```

Pour l'intégration continue et l'automatisation, donnez au runner un rôle IAM avec la permission d'invoquer le service Anthropic et définissez `AWS_REGION`. La chaîne d'identifiants récupère automatiquement le rôle.

Si vos identifiants SSO expirent en cours de session, configurez [`awsAuthRefresh`](/docs/fr/amazon-bedrock#advanced-credential-configuration) pour que Claude Code réexécute votre commande de connexion et réessaie au lieu d'échouer. L'actualisation automatique sur Claude Platform on AWS nécessite Claude Code v2.1.198 ou version ultérieure ; les versions antérieures s'arrêtent avec une invite pour exécuter `/login`, qui ne peut pas actualiser les identifiants AWS. Ajoutez la commande à votre `settings.json` :

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile my-profile"
}
```

Avec `awsAuthRefresh` configuré, `/login` affiche une option **Claude Platform on AWS · actualiser les identifiants** sous **Utilisation de plateformes tierces**. La sélectionner exécute la commande configurée et relit vos identifiants AWS sans redémarrer Claude Code.

**Option B : Clé API d'espace de travail**

Une clé API d'espace de travail est un secret de longue durée, utile lorsque vous ne souhaitez pas gérer les identifiants AWS fédérés. Générez-en une dans la console AWS sous **Claude Platform on AWS → API keys** et définissez-la comme `ANTHROPIC_AWS_API_KEY` :

```bash theme={null}
export ANTHROPIC_AWS_API_KEY=sk-ant-xxxxx
```

La clé est envoyée en tant que `x-api-key` et prend la priorité sur SigV4, donc tous les identifiants AWS dans votre environnement sont ignorés. Les clés API d'une organisation Claude Console distincte ne fonctionneront pas ici.

Traitez les clés API d'espace de travail comme toute autre identifiant de production. Le bloc `env` du [fichier de paramètres utilisateur](/docs/fr/settings) est un moyen pratique de limiter la clé à votre machine sans l'exporter globalement.

<Note>
  Les commandes `/login` et `/logout` ne vous connectent pas à un abonnement Claude.ai pour Claude Platform on AWS. L'authentification s'effectue via vos identifiants AWS ou votre clé API d'espace de travail. L'exception est l'option **actualiser les identifiants** que `/login` affiche lorsque `awsAuthRefresh` est configuré, qui relit vos identifiants AWS comme décrit ci-dessus.
</Note>

<h3 id="2-configure-claude-code">
  2. Configurez Claude Code
</h3>

Définissez les variables d'environnement qui acheminent Claude Code via Claude Platform on AWS au lieu de l'API Anthropic par défaut.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export AWS_REGION=us-east-1
```

`ANTHROPIC_AWS_WORKSPACE_ID` est obligatoire et est envoyé à chaque requête en tant qu'en-tête `anthropic-workspace-id`. L'URL de base est calculée à partir de `AWS_REGION` comme `https://aws-external-anthropic.{region}.api.aws`. Pour remplacer l'URL directement, définissez `ANTHROPIC_AWS_BASE_URL`.

Claude Platform on AWS est optionnel même lorsque des identifiants AWS sont présents dans votre environnement. Amazon Bedrock et Microsoft Foundry prennent la priorité dans l'acheminement des fournisseurs, donc désactivez `CLAUDE_CODE_USE_BEDROCK` et `CLAUDE_CODE_USE_FOUNDRY` s'ils sont définis.

<h3 id="3-pin-model-versions">
  3. Épinglez les versions de modèle
</h3>

Claude Platform on AWS utilise les mêmes ID de modèle que l'API Claude directe.

Les alias par défaut `fable`, `opus`, `sonnet` et `haiku` se résolvent aux valeurs par défaut intégrées de Claude Code pour Claude Platform on AWS, qui peuvent être en retard par rapport à la version la plus récente. Sans `ANTHROPIC_DEFAULT_OPUS_MODEL`, l'alias `opus` se résout à Opus 4.8. Avant la v2.1.207, il se résolvait à Opus 4.7.

Si vous déployez Claude Code pour une équipe, épinglez explicitement les ID de modèle pour qu'une nouvelle version ne déplace pas tout le monde à la fois :

```bash theme={null}
export ANTHROPIC_DEFAULT_FABLE_MODEL=claude-fable-5
export ANTHROPIC_DEFAULT_OPUS_MODEL=claude-opus-4-8
export ANTHROPIC_DEFAULT_SONNET_MODEL=claude-sonnet-5
export ANTHROPIC_DEFAULT_HAIKU_MODEL=claude-haiku-4-5
```

Pour la liste complète des ID de modèle et des alias, consultez [Aperçu des modèles](https://platform.claude.com/docs/en/about-claude/models/overview). Pour d'autres variables liées aux modèles, consultez [Configuration du modèle](/docs/fr/model-config).

[La mise en cache des invites](/docs/fr/prompt-caching) est activée automatiquement. Pour demander un TTL de cache d'1 heure au lieu de la valeur par défaut de 5 minutes, définissez `ENABLE_PROMPT_CACHING_1H=1`. L'API facture les écritures en cache d'1 heure à un taux plus élevé. Consultez [tarification de la mise en cache des invites](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) pour connaître les tarifs.

<h2 id="use-the-agent-sdk">
  Utilisez le SDK Agent
</h2>

Le [SDK Agent](/docs/fr/agent-sdk/overview) lit les mêmes variables d'environnement que l'interface de ligne de commande, donc tout programme qui lance le sous-processus Claude Code peut cibler Claude Platform on AWS en exportant `CLAUDE_CODE_USE_ANTHROPIC_AWS`, `ANTHROPIC_AWS_WORKSPACE_ID` et soit `ANTHROPIC_AWS_API_KEY` soit les identifiants AWS avant l'appel.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

process.env.CLAUDE_CODE_USE_ANTHROPIC_AWS = "1";
process.env.ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN";
process.env.AWS_REGION = "us-east-1";

for await (const msg of query({ prompt: "What's in this repo?" })) {
  console.log(msg);
}
```

Cet exemple s'appuie sur la chaîne d'identifiants AWS ambiante pour SigV4. Pour vous authentifier avec une clé API d'espace de travail à la place, définissez `ANTHROPIC_AWS_API_KEY` de la même manière. Pour la surface plus large du SDK Agent, consultez [Aperçu du SDK Agent](/docs/fr/agent-sdk/overview).

<h2 id="route-through-a-corporate-proxy">
  Acheminez via un proxy d'entreprise
</h2>

Pour acheminer le trafic via un proxy ou une [passerelle LLM](/docs/fr/llm-gateway), définissez `ANTHROPIC_AWS_BASE_URL` sur l'adresse du proxy. Claude Code envoie les requêtes à cette URL avec les mêmes en-têtes d'espace de travail et d'authentification, donc toute passerelle qui les transfère inchangés fonctionne.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

Si votre passerelle signe elle-même les requêtes, définissez `CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1` pour que Claude Code envoie des requêtes non signées et laisse la passerelle ajouter les en-têtes SigV4 avant de les transférer à AWS. Si la passerelle nécessite son propre jeton, définissez-le dans `ANTHROPIC_AUTH_TOKEN`.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

<h2 id="troubleshooting">
  Dépannage
</h2>

Exécutez `/status` pour voir le fournisseur résolu et tout ID d'espace de travail explicitement configuré, région, remplacement d'URL de base et paramètre d'authentification ignorée. C'est le moyen le plus rapide de confirmer que Claude Code cible Claude Platform on AWS.

<h3 id="403-forbidden-or-accessdenied-on-every-request">
  `403 Forbidden` ou `AccessDenied` à chaque requête
</h3>

Le principal IAM que Claude Code a résolu manque probablement de la permission d'invoquer le service Anthropic dans votre espace de travail. Vérifiez le rôle attaché à votre profil AWS ou au runner qui a démarré Claude Code, et vérifiez qu'il dispose des actions `aws-external-anthropic` documentées dans la [référence des actions IAM](https://platform.claude.com/docs/fr/api/claude-platform-on-aws-iam-actions).

Si vous avez défini `ANTHROPIC_AWS_API_KEY`, la clé prend la priorité sur SigV4 et une clé obsolète produit la même erreur. Régénérez la clé dans la console AWS sous **Claude Platform on AWS → API keys** ou désactivez la variable pour revenir à vos identifiants AWS.

<h3 id="requests-fail-with-a-missing-workspace-error">
  Les requêtes échouent avec une erreur d'espace de travail manquant
</h3>

`ANTHROPIC_AWS_WORKSPACE_ID` est probablement non défini ou vide. Chaque requête Claude Platform on AWS doit inclure l'ID d'espace de travail. Il n'est pas implicite par vos identifiants AWS. Trouvez l'ID sous **Workspaces** sur la page du service de la console AWS et exportez-le avant de démarrer Claude Code.

<h3 id="requests-still-go-to-api-anthropic-com">
  Les requêtes vont toujours à `api.anthropic.com`
</h3>

`CLAUDE_CODE_USE_ANTHROPIC_AWS` est probablement non défini ou défini sur une valeur qui ne s'analyse pas comme vraie. Définissez-le sur `1` et exécutez `/status` pour confirmer le fournisseur résolu. Si `CLAUDE_CODE_USE_BEDROCK` ou `CLAUDE_CODE_USE_FOUNDRY` est également défini, ceux-ci prennent la priorité sur Claude Platform on AWS.

<h2 id="additional-resources">
  Ressources supplémentaires
</h2>

L'abonnement Claude Platform on AWS, la configuration de l'espace de travail et d'IAM qui précèdent la configuration de Claude Code sont couverts dans la documentation de la plateforme :

* [Aperçu de Claude Platform on AWS](https://platform.claude.com/docs/fr/build-with-claude/claude-platform-on-aws) : abonnement, configuration de l'espace de travail et référence du produit
* [Référence des actions IAM](https://platform.claude.com/docs/fr/api/claude-platform-on-aws-iam-actions) : permissions et politiques gérées
