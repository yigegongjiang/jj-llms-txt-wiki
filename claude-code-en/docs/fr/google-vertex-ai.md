> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code sur la Plateforme Agent de Google Cloud

> Découvrez comment configurer Claude Code via la Plateforme Agent de Google Cloud, anciennement Vertex AI, y compris la configuration, la configuration IAM et la résolution des problèmes.

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

<ContactSalesCard surface="vertex" />

<h2 id="prerequisites">
  Conditions préalables
</h2>

Avant de configurer Claude Code avec Google Cloud's Agent Platform, anciennement Vertex AI, assurez-vous que vous disposez de :

* Un compte Google Cloud Platform (GCP) avec facturation activée
* Un projet GCP avec l'API Google Cloud's Agent Platform activée
* Accès aux modèles Claude souhaités (par exemple, Claude Sonnet 4.6)
* Google Cloud SDK (`gcloud`) installé et configuré
* Quota alloué dans la région GCP souhaitée

Pour vous connecter avec vos propres identifiants Google Cloud's Agent Platform, suivez [Se connecter avec Google Cloud's Agent Platform](#sign-in-with-agent-platform) ci-dessous. Pour déployer Claude Code dans une équipe, utilisez les étapes de [configuration manuelle](#set-up-manually) et [épinglez vos versions de modèle](#5-pin-model-versions) avant le déploiement.

<h2 id="sign-in-with-agent-platform">
  Se connecter avec Agent Platform
</h2>

Si vous disposez d'identifiants Google Cloud et souhaitez commencer à utiliser Claude Code via Agent Platform de Google Cloud, l'assistant de connexion vous guide à travers le processus. Vous complétez les conditions préalables du côté GCP une fois par projet ; l'assistant gère le côté Claude Code.

<Steps>
  <Step title="Activer les modèles Claude dans votre projet GCP">
    [Activez l'API Agent Platform de Google Cloud](#1-enable-agent-platform-api) pour votre projet, puis demandez l'accès aux modèles Claude que vous souhaitez dans le [Model Garden d'Agent Platform de Google Cloud](https://console.cloud.google.com/vertex-ai/model-garden). Consultez [Configuration IAM](#iam-configuration) pour les autorisations dont votre compte a besoin.
  </Step>

  <Step title="Démarrer Claude Code et choisir Agent Platform de Google Cloud">
    Exécutez `claude`. À l'invite de connexion, sélectionnez **plateforme tierce**, puis **Google Vertex AI**, l'étiquette que l'invite de connexion utilise toujours pour Agent Platform de Google Cloud. Si vous êtes déjà connecté, exécutez `/login` pour ouvrir le même menu.
  </Step>

  <Step title="Suivre les invites de l'assistant">
    Choisissez comment vous vous authentifiez auprès de Google Cloud : identifiants par défaut de l'application à partir de `gcloud`, fichier de clé de compte de service, ou identifiants déjà dans votre environnement. L'assistant détecte votre projet et votre région, vérifie quels modèles Claude votre projet peut invoquer, et vous permet de les épingler. Il enregistre le résultat dans le bloc `env` de votre [fichier de paramètres utilisateur](/docs/fr/settings), vous n'avez donc pas besoin d'exporter les variables d'environnement vous-même.
  </Step>
</Steps>

Après vous être connecté, exécutez `/setup-vertex` à tout moment pour rouvrir l'assistant et modifier vos identifiants, votre projet, votre région ou vos épingles de modèle. L'étape d'épinglage du modèle commence à partir de vos modèles actuellement épinglés. L'assistant écrit dans `~/.claude/settings.json`, ou dans `$CLAUDE_CONFIG_DIR/settings.json` lorsque [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars#variables) est défini.

<h2 id="region-configuration">
  Configuration de la région
</h2>

Claude Code prend en charge les points de terminaison Google Cloud's Agent Platform [globaux](https://cloud.google.com/blog/products/ai-machine-learning/global-endpoint-for-claude-models-generally-available-on-vertex-ai), multi-régions et régionaux. Définissez `CLOUD_ML_REGION` sur `global`, un emplacement multi-région tel que `eu` ou `us`, ou une région spécifique telle que `us-east5`. Claude Code sélectionne le nom d'hôte Google Cloud's Agent Platform correct pour chaque formulaire, y compris les hôtes `aiplatform.eu.rep.googleapis.com` et `aiplatform.us.rep.googleapis.com` pour les emplacements multi-régions.

<Note>
  Google Cloud's Agent Platform peut ne pas prendre en charge les modèles par défaut de Claude Code sur tous les types de points de terminaison. La disponibilité des modèles varie selon les [régions spécifiques](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations#genai-partner-models), les emplacements multi-régions et les [points de terminaison globaux](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/use-partner-models#supported_models). Vous devrez peut-être basculer vers un emplacement pris en charge ou spécifier un modèle pris en charge.
</Note>

<h2 id="set-up-manually">
  Configuration manuelle
</h2>

Pour configurer Google Cloud's Agent Platform via des variables d'environnement au lieu de l'assistant, par exemple dans CI ou un déploiement d'entreprise scriptée, suivez les étapes ci-dessous.

<h3 id="1-enable-agent-platform-api">
  1. Activer l'API Agent Platform
</h3>

Activez l'API Agent Platform de Google Cloud dans votre projet GCP. Remplacez `YOUR-PROJECT-ID` par votre ID de projet GCP ici et à l'étape de configuration ci-dessous :

```bash theme={null}
# Définissez votre ID de projet
gcloud config set project YOUR-PROJECT-ID

# Activez l'API Agent Platform
gcloud services enable aiplatform.googleapis.com
```

<h3 id="2-request-model-access">
  2. Demander l'accès au modèle
</h3>

Demandez l'accès aux modèles Claude dans Google Cloud's Agent Platform :

1. Accédez au [Google Cloud's Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)
2. Recherchez les modèles « Claude »
3. Demandez l'accès aux modèles Claude souhaités (par exemple, Claude Sonnet 4.6)
4. Attendez l'approbation (peut prendre 24 à 48 heures)

<h3 id="3-configure-gcp-credentials">
  3) Configurer les identifiants GCP
</h3>

Claude Code utilise l'authentification Google Cloud standard.

Pour plus d'informations, consultez la [documentation d'authentification Google Cloud](https://cloud.google.com/docs/authentication).

Claude Code prend en charge la [Fédération d'identité de charge de travail basée sur certificat X.509](https://cloud.google.com/iam/docs/workload-identity-federation-with-x509-certificates) via la même chaîne Application Default Credentials. Définissez `GOOGLE_APPLICATION_CREDENTIALS` sur le chemin de votre fichier de configuration des identifiants.

<Note>
  Claude Code adresse les demandes Google Cloud's Agent Platform au projet dans `ANTHROPIC_VERTEX_PROJECT_ID`, même lorsque `GCLOUD_PROJECT`, `GOOGLE_CLOUD_PROJECT`, ou le fichier d'identifiants référencé par `GOOGLE_APPLICATION_CREDENTIALS` porte un projet différent.
</Note>

<h4 id="advanced-credential-configuration">
  Configuration avancée des identifiants
</h4>

Claude Code prend en charge l'actualisation automatique des identifiants GCP via le paramètre `gcpAuthRefresh`. Ajoutez-le à votre fichier de [paramètres](/docs/fr/settings) Claude Code, par exemple `~/.claude/settings.json`. Lorsque Claude Code détecte que vos identifiants GCP ont expiré ou ne peuvent pas être chargés, il exécute la commande configurée pour obtenir de nouveaux identifiants avant de réessayer la demande.

```json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login",
  "env": {
    "ANTHROPIC_VERTEX_PROJECT_ID": "your-project-id"
  }
}
```

Avant d'exécuter la commande, Claude Code demande un jeton d'accès avec vos identifiants actuels pour confirmer qu'ils sont réellement expirés, et ignore la commande lorsqu'ils fonctionnent toujours.

Si la vérification ne se termine pas dans les cinq secondes, Claude Code ignore également la commande et l'exécute uniquement après l'échec d'une demande avec une erreur d'identifiants. Avant v2.1.261, une vérification qui a expiré était comptée comme un identifiant expiré, donc la commande pouvait ouvrir votre navigateur au démarrage même si vos identifiants étaient toujours valides.

Claude Code vous affiche la sortie de la commande, mais ne peut pas envoyer d'entrée interactive à la commande. Cela fonctionne bien pour les flux d'authentification basés sur navigateur où l'interface de ligne de commande affiche une URL et vous complétez l'authentification dans le navigateur. La commande d'actualisation expire après trois minutes si l'authentification ne se termine pas. Si vous définissez `gcpAuthRefresh` dans les paramètres du projet tels que `.claude/settings.json`, Claude Code l'exécute selon la même [règle de confiance de l'espace de travail que les hooks dans les fichiers de paramètres](/docs/fr/permissions#what-runs-before-you-trust-a-folder), qui inclut les sessions `-p` dans les dossiers que vous n'avez jamais approuvés.

<h3 id="4-configure-claude-code">
  4. Configurer Claude Code
</h3>

Définissez les variables d'environnement suivantes :

```bash theme={null}
# Activez l'intégration Agent Platform
export CLAUDE_CODE_USE_VERTEX=1
export CLOUD_ML_REGION=global
export ANTHROPIC_VERTEX_PROJECT_ID=YOUR-PROJECT-ID

# Optionnel : Remplacez l'URL du point de terminaison Agent Platform pour les points de terminaison personnalisés ou les passerelles
# export ANTHROPIC_VERTEX_BASE_URL=https://aiplatform.googleapis.com

# Quand CLOUD_ML_REGION=global, remplacez la région pour les modèles qui ne prennent pas en charge les points de terminaison globaux
export VERTEX_REGION_CLAUDE_HAIKU_4_5=us-east5
export VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1
```

La plupart des versions de modèle ont une variable `VERTEX_REGION_CLAUDE_*` correspondante. Consultez la [référence des variables d'environnement](/docs/fr/env-vars) pour la liste complète. Vérifiez [Google Cloud's Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) pour déterminer quels modèles prennent en charge les points de terminaison globaux par rapport aux points de terminaison régionaux uniquement.

Si une valeur de région ne ressemble pas à un nom de région ou d'emplacement, Claude Code la traite comme non définie. Par exemple, Claude Code traite une valeur contenant une barre oblique, un point ou un espace comme non définie. Claude Code revient à une source différente pour chaque variable :

* `VERTEX_REGION_CLAUDE_*` : Claude Code revient à `CLOUD_ML_REGION`.
* `CLOUD_ML_REGION` : Claude Code revient à `us-east5`.

[La mise en cache des invites](/docs/fr/prompt-caching) est activée automatiquement. Pour la désactiver, définissez `DISABLE_PROMPT_CACHING=1`. Pour demander une TTL de cache d'1 heure au lieu de la valeur par défaut de 5 minutes, définissez `ENABLE_PROMPT_CACHING_1H=1` ; les écritures de cache avec une TTL d'1 heure sont facturées à un taux plus élevé. Pour définir différentes TTL pour votre conversation principale et pour les demandes que Claude Code effectue en dehors de celle-ci, [choisissez vous-même la TTL](/docs/fr/prompt-caching#choose-the-ttl-yourself).

Pour augmenter vos limites de débit, contactez le support Google Cloud. Lors de l'utilisation de Google Cloud's Agent Platform, la commande `/logout` est indisponible car l'authentification est gérée via les identifiants Google Cloud.

Claude Code décide entre la [recherche d'outils MCP](/docs/fr/mcp#scale-with-mcp-tool-search) et le chargement à l'avance par génération de modèle :

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5 et versions ultérieures** : Claude Code active la recherche d'outils par défaut.
* **Modèles antérieurs, y compris tous les modèles Claude 3.x** : Claude Code charge les définitions d'outils MCP à l'avance, car leurs piles de service Agent Platform rejettent l'en-tête bêta requis. Définir `ENABLE_TOOL_SEARCH=true` ne remplace pas cela.

Définissez `ENABLE_TOOL_SEARCH=false` pour désactiver la recherche d'outils sur tous les modèles. Avant v2.1.221, Claude Code désactivait la recherche d'outils pour tous les modèles sur Google Cloud's Agent Platform sauf si vous définissiez `ENABLE_TOOL_SEARCH=true`.

<h3 id="5-pin-model-versions">
  5. Épingler les versions de modèle
</h3>

<Warning>
  Épinglez les versions de modèle spécifiques lors du déploiement pour plusieurs utilisateurs. Sans épinglage, les alias de modèle tels que `sonnet` et `opus` se résolvent à la valeur par défaut intégrée de Claude Code pour Google Cloud's Agent Platform, qui peut être en retard par rapport à la version la plus récente et peut ne pas encore être activée dans votre projet. Claude Code [revient](#startup-model-checks) à un modèle antérieur ou de niveau inférieur au démarrage lorsque la valeur par défaut n'est pas disponible, mais l'épinglage vous permet de contrôler quand vos utilisateurs passent à un nouveau modèle.
</Warning>

Définissez ces variables d'environnement sur des ID de modèle Google Cloud's Agent Platform spécifiques.

Sans `ANTHROPIC_DEFAULT_OPUS_MODEL`, l'alias `opus` sur Google Cloud's Agent Platform se résout à Opus 5.5, et sans `ANTHROPIC_DEFAULT_SONNET_MODEL`, l'alias `sonnet` se résout à Sonnet 4.5. Cet exemple épingle chaque alias à une version spécifique :

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

Pour les ID de modèle actuels et hérités, consultez [Aperçu des modèles](https://platform.claude.com/docs/en/about-claude/models/overview). Consultez [Configuration du modèle](/docs/fr/model-config#pin-models-for-third-party-deployments) pour la liste complète des variables d'environnement.

Claude Code utilise ces modèles par défaut lorsqu'aucune variable d'épinglage n'est définie :

| Type de modèle      | Valeur par défaut            |
| :------------------ | :--------------------------- |
| Modèle principal    | `claude-opus-5-5`            |
| Modèle petit/rapide | `claude-sonnet-4-5@20250929` |

Les tâches en arrière-plan telles que la génération de titre de session utilisent le modèle petit/rapide, normalement un modèle de classe Haiku. Sur Google Cloud's Agent Platform, Claude Code utilise le modèle Sonnet par défaut pour les tâches en arrière-plan car Haiku peut ne pas être activé dans tous les projets ou régions. Deux sélections changent le modèle qui les exécute :

* Lorsque vous sélectionnez un modèle principal avec `--model`, `ANTHROPIC_MODEL`, ou le paramètre `model`, les tâches en arrière-plan utilisent ce modèle. Lorsque Claude Code démarre la session sur le modèle que vous définissez avec [`ANTHROPIC_DEFAULT_MODEL`](/docs/fr/model-config#set-a-default-model-for-new-sessions), les tâches en arrière-plan utilisent ce modèle aussi. Définir `ANTHROPIC_DEFAULT_OPUS_MODEL` sans `ANTHROPIC_DEFAULT_SONNET_MODEL` compte également comme une sélection, car le modèle Sonnet intégré peut ne pas être activé dans un projet qui oriente son propre Opus.
* Pour utiliser Haiku pour les tâches en arrière-plan, définissez `ANTHROPIC_DEFAULT_HAIKU_MODEL` sur un ID de modèle disponible dans votre projet.

<Warning>
  Les modèles Opus ont un prix par jeton plus élevé que les modèles Sonnet, donc un déploiement qui n'épingle pas un modèle principal est facturé au taux Opus une fois qu'il se met à jour vers v2.1.207 ou ultérieur. Pour conserver Sonnet 4.5 comme modèle principal, définissez `ANTHROPIC_MODEL` sur son ID de modèle complet. Un déploiement qui oriente la valeur par défaut avec `ANTHROPIC_DEFAULT_SONNET_MODEL` et ne définit pas `ANTHROPIC_DEFAULT_OPUS_MODEL` conserve son modèle Sonnet orienté comme valeur par défaut.
</Warning>

Avant v2.1.280, le modèle principal sur Google Cloud's Agent Platform était par défaut Opus 5 et l'alias `opus` se résolvait à Opus 5 à partir de v2.1.219. Sur v2.1.207 à v2.1.218, le modèle principal sur Google Cloud's Agent Platform était par défaut Opus 4.8 et l'alias `opus` se résolvait à Opus 4.8. Avant v2.1.207, le modèle principal était par défaut Sonnet 4.5, l'alias `opus` se résolvait à Opus 4.6, et les tâches en arrière-plan utilisaient toujours le modèle principal.

Pour personnaliser davantage les modèles :

```bash theme={null}
export ANTHROPIC_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

<h3 id="6-verify-your-configuration">
  6. Vérifier votre configuration
</h3>

Démarrez Claude Code et exécutez `/status` pour confirmer la configuration. La ligne `API provider` affiche `Google Vertex AI`, et les lignes `GCP project`, `Default region` et `Model` affichent votre ID de projet, votre région et votre modèle résolu. Si la ligne du fournisseur est manquante, les variables d'environnement n'atteignent pas le processus. Confirmez qu'elles sont exportées dans le shell où vous avez lancé `claude`, ou définissez-les dans le bloc `env` de votre [fichier de paramètres](/docs/fr/settings).

<h2 id="startup-model-checks">
  Vérifications du modèle au démarrage
</h2>

Lorsque Claude Code démarre avec Google Cloud's Agent Platform configuré, il vérifie que les modèles qu'il a l'intention d'utiliser sont accessibles dans votre projet.

Si vous avez épinglé une version de modèle plus ancienne que la valeur par défaut actuelle de Claude Code, et que votre projet peut invoquer la version plus récente, Claude Code vous invite à mettre à jour l'épingle. L'acceptation écrit le nouvel ID de modèle dans votre [fichier de paramètres utilisateur](/docs/fr/settings) et redémarre Claude Code. Le refus est mémorisé jusqu'au prochain changement de version par défaut.

Si vous n'avez pas épinglé un modèle et que la valeur par défaut actuelle n'est pas disponible dans votre projet, Claude Code revient à la version précédente pour la session actuelle et affiche un avis. Il essaie d'abord les versions antérieures du modèle par défaut et, lorsque la valeur par défaut est un modèle Opus et qu'aucune version Opus n'est disponible, revient au modèle Sonnet par défaut. Le retour n'est pas persistant. Activez le modèle plus récent dans [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) ou [épinglez une version](#5-pin-model-versions) pour rendre le choix permanent.

Lorsque vous démarrez la session sur une version spécifique de Sonnet ou Opus, par exemple avec `--model`, `ANTHROPIC_MODEL`, ou le [paramètre `model`](/docs/fr/settings-reference#model), cette version agit comme la valeur par défaut épinglée de la session pour l'alias `sonnet` ou `opus` correspondant. Claude Code ignore la vérification de disponibilité pour la valeur par défaut intégrée que votre modèle remplace et démarre sur le modèle que vous avez configuré, sans avis de retour.

Les alias de modèle tels que `opus` n'agissent pas comme des épingles, et il en va de même pour un ID de modèle que Claude Code ne reconnaît pas.

<h2 id="iam-configuration">
  Configuration IAM
</h2>

Attribuez le rôle `roles/aiplatform.user`, qui inclut les autorisations requises :

* `aiplatform.endpoints.predict` - Requis pour l'invocation de modèle et le comptage des jetons

Pour des autorisations plus restrictives, créez un rôle personnalisé avec uniquement les autorisations ci-dessus.

Pour plus de détails, consultez la [documentation IAM de Google Cloud Vertex AI](https://cloud.google.com/vertex-ai/docs/general/access-control).

<Note>
  Créez un projet GCP dédié pour Claude Code pour simplifier le suivi des coûts et le contrôle d'accès.
</Note>

<h2 id="1m-token-context-window">
  Fenêtre de contexte de 1M de jetons
</h2>

Claude Sonnet 5, Opus 4.6 et versions ultérieures, ainsi que Sonnet 4.6, prennent en charge la [fenêtre de contexte de 1M de jetons](https://platform.claude.com/docs/fr/build-with-claude/context-windows#context-window-sizes-by-model) sur la plateforme Agent de Google Cloud. Sonnet 5 s'exécute toujours avec la fenêtre 1M, sans variante `[1m]` à sélectionner. Pour les autres modèles, Claude Code active automatiquement la fenêtre de contexte étendue lorsque vous sélectionnez une variante de modèle 1M.

L'[assistant de configuration](#sign-in-with-agent-platform) offre une option de contexte 1M lorsqu'il épingle les modèles. Pour l'activer pour un modèle épinglé manuellement à la place, ajoutez `[1m]` à l'ID du modèle. Consultez [Épingler les modèles pour les déploiements tiers](/docs/fr/model-config#pin-models-for-third-party-deployments) pour plus de détails.

<h2 id="troubleshooting">
  Résolution des problèmes
</h2>

Si vous rencontrez des erreurs « Impossible de charger les identifiants par défaut » :

* Exécutez `gcloud auth application-default login` pour configurer les identifiants par défaut de l'application
* Définissez `GOOGLE_APPLICATION_CREDENTIALS` sur le chemin d'un fichier de clé de compte de service
* Consultez [Configurer les identifiants GCP](#3-configure-gcp-credentials) pour toutes les options

Si vous rencontrez des problèmes de quota :

* Vérifiez les quotas actuels ou demandez une augmentation de quota via la [Console Cloud](https://cloud.google.com/docs/quotas/view-manage)

Si vous rencontrez des erreurs « modèle non trouvé » 404 :

* Confirmez que le modèle est activé dans [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)
* Vérifiez que le modèle est disponible dans l'emplacement que vous avez spécifié. Certains modèles ne sont proposés que sur les emplacements `global` ou multi-régions tels que `eu` et `us`, pas dans les régions spécifiques
* Si vous utilisez `CLOUD_ML_REGION=global`, vérifiez que vos modèles prennent en charge les points de terminaison globaux dans [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) sous « Fonctionnalités prises en charge ». Pour les modèles qui ne prennent pas en charge les points de terminaison globaux, soit :
  * Spécifiez un modèle pris en charge via `ANTHROPIC_MODEL` ou `ANTHROPIC_DEFAULT_HAIKU_MODEL`, soit
  * Définissez une région ou un emplacement multi-région à l'aide des variables d'environnement `VERTEX_REGION_<MODEL_NAME>`

Si vous rencontrez des erreurs 429 :

* Pour les points de terminaison régionaux, assurez-vous que le modèle principal et le modèle petit/rapide sont pris en charge dans votre région sélectionnée
* Envisagez de basculer vers `CLOUD_ML_REGION=global` pour une meilleure disponibilité

<h2 id="additional-resources">
  Ressources supplémentaires
</h2>

* [Documentation de la plateforme Agent de Google Cloud](https://cloud.google.com/vertex-ai/docs)
* [Tarification de la plateforme Agent de Google Cloud](https://cloud.google.com/vertex-ai/pricing)
* [Quotas et limites de la plateforme Agent de Google Cloud](https://cloud.google.com/vertex-ai/docs/quotas)
