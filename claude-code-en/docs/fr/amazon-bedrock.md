> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code sur Amazon Bedrock

> Découvrez comment configurer Claude Code via Amazon Bedrock, y compris la configuration, la configuration IAM et le dépannage.

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

<ContactSalesCard surface="bedrock" />

<h2 id="prerequisites">
  Prérequis
</h2>

Avant de configurer Claude Code avec Amazon Bedrock, assurez-vous que vous disposez de :

* Un compte AWS avec accès à Amazon Bedrock activé
* Accès aux modèles Claude souhaités (par exemple, Claude Sonnet 4.6) dans Amazon Bedrock
* AWS CLI installé et configuré (facultatif - nécessaire uniquement si vous n'avez pas d'autre mécanisme pour obtenir les identifiants)
* Autorisations IAM appropriées

Pour vous connecter avec vos propres identifiants Amazon Bedrock, suivez [Se connecter avec Amazon Bedrock](#sign-in-with-bedrock) ci-dessous. Pour déployer Claude Code dans une équipe, utilisez les étapes de [configuration manuelle](#set-up-manually) et [épinglez vos versions de modèle](#4-pin-model-versions) avant le déploiement.

<h2 id="sign-in-with-bedrock">
  Se connecter avec Bedrock
</h2>

Si vous disposez d'identifiants AWS et souhaitez commencer à utiliser Claude Code via Amazon Bedrock, l'assistant de connexion vous guide à travers le processus. Vous complétez les prérequis côté AWS une seule fois par compte ; l'assistant gère le côté Claude Code.

<Steps>
  <Step title="Activer les modèles Anthropic dans votre compte AWS">
    Dans la [console Amazon Bedrock](https://console.aws.amazon.com/bedrock/), ouvrez le catalogue de modèles, sélectionnez un modèle Anthropic et soumettez le formulaire de cas d'usage. L'accès est accordé immédiatement après la soumission. Consultez [Soumettre les détails du cas d'usage](#1-submit-use-case-details) pour AWS Organizations et [Configuration IAM](#iam-configuration) pour les autorisations dont votre rôle a besoin.
  </Step>

  <Step title="Démarrer Claude Code et choisir Amazon Bedrock">
    Exécutez `claude`. À l'invite de connexion, sélectionnez **plateforme tierce**, puis **Amazon Bedrock**. Si vous êtes déjà connecté et voyez l'invite de chat à la place, exécutez `/setup-bedrock` pour ouvrir l'assistant. Jusqu'à ce que `CLAUDE_CODE_USE_BEDROCK=1` soit défini, Claude Code [masque la commande du menu de commandes](/docs/fr/commands#how-the-command-menu-matches-what-you-type) ; tapez-la en entier.
  </Step>

  <Step title="Suivre les invites de l'assistant">
    Choisissez comment vous vous authentifiez auprès d'AWS : un profil AWS détecté à partir de votre répertoire `~/.aws`, une clé API Amazon Bedrock, une clé d'accès et un secret, ou des identifiants déjà dans votre environnement. L'assistant vous demande votre région, vérifie quels modèles Claude votre compte peut invoquer, et vous permet de les épingler. Il enregistre le résultat dans le bloc `env` de votre [fichier de paramètres utilisateur](/docs/fr/settings), afin que vous n'ayez pas besoin d'exporter vous-même les variables d'environnement.
  </Step>
</Steps>

Après vous être connecté, exécutez `/setup-bedrock` à tout moment pour rouvrir l'assistant et modifier vos identifiants, votre région ou vos épingles de modèle. L'étape d'épingle de modèle commence à partir de vos modèles actuellement épinglés. L'assistant écrit dans `~/.claude/settings.json`, ou dans `$CLAUDE_CONFIG_DIR/settings.json` lorsque [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars#variables) est défini.

<h2 id="set-up-manually">
  Configuration manuelle
</h2>

Pour configurer Amazon Bedrock via des variables d'environnement au lieu de l'assistant, par exemple dans une intégration continue ou un déploiement d'entreprise scriptisé, suivez les étapes ci-dessous.

<h3 id="1-submit-use-case-details">
  1. Soumettre les détails du cas d'usage
</h3>

Avant d'invoquer un modèle Anthropic pour la première fois, soumettez les détails du cas d'usage. Vous faites cela une fois par compte AWS.

1. Assurez-vous que vous disposez des bonnes permissions IAM décrites ci-dessous
2. Accédez à la [console Amazon Bedrock](https://console.aws.amazon.com/bedrock/)
3. Sélectionnez un modèle Anthropic dans le **Catalogue de modèles**
4. Complétez le formulaire de cas d'usage. L'accès est accordé immédiatement après la soumission.

Si vous utilisez AWS Organizations, vous pouvez soumettre le formulaire une fois à partir du compte de gestion en utilisant l'[API `PutUseCaseForModelAccess`](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_PutUseCaseForModelAccess.html). Cet appel nécessite la permission IAM `bedrock:PutUseCaseForModelAccess`. L'approbation s'étend automatiquement aux comptes enfants.

<h3 id="2-configure-aws-credentials">
  2. Configurer les identifiants AWS
</h3>

Claude Code utilise la chaîne d'identifiants par défaut du SDK AWS. Configurez vos identifiants en utilisant l'une de ces méthodes :

**Option A : Configuration AWS CLI**

```bash theme={null}
aws configure
```

**Option B : Variables d'environnement (clé d'accès)**

```bash theme={null}
export AWS_ACCESS_KEY_ID=your-access-key-id
export AWS_SECRET_ACCESS_KEY=your-secret-access-key
export AWS_SESSION_TOKEN=your-session-token
```

**Option C : Variables d'environnement (profil SSO)**

Remplacez `your-profile-name` par le nom de votre profil AWS avant d'exécuter ces commandes.

```bash theme={null}
aws sso login --profile=your-profile-name

export AWS_PROFILE=your-profile-name
```

Claude Code demande les identifiants de rôle à la région du Centre d'identité IAM nommée par le `sso_region` du profil, qui n'a pas besoin de correspondre à la région dans laquelle vous exécutez Amazon Bedrock. En v2.1.207, la région Amazon Bedrock a remplacé `sso_region`, donc un profil dont l'instance du Centre d'identité IAM se trouve dans une région différente n'a pas pu s'authentifier avec une erreur `Session token not found or invalid`.

**Option D : Identifiants de la console de gestion AWS**

```bash theme={null}
aws login
```

[En savoir plus](https://docs.aws.amazon.com/signin/latest/userguide/command-line-sign-in.html) sur `aws login`.

**Option E : Clés API Amazon Bedrock**

```bash theme={null}
export AWS_BEARER_TOKEN_BEDROCK=your-bedrock-api-key
```

Les clés API Amazon Bedrock fournissent une méthode d'authentification plus simple sans avoir besoin d'identifiants AWS complets. [En savoir plus sur les clés API Amazon Bedrock](https://aws.amazon.com/blogs/machine-learning/accelerate-ai-development-with-amazon-bedrock-api-keys/).

<h4 id="credential-caching-and-resolution-timeout">
  Mise en cache des identifiants et délai d'expiration de la résolution
</h4>

Claude Code résout la chaîne du fournisseur d'identifiants par défaut AWS une fois et conserve les identifiants résolus en mémoire. Il les réutilise jusqu'à cinq minutes avant leur expiration, ou pendant une heure quand ils n'ont pas d'expiration, donc un profil soutenu par SSO demande des identifiants au Centre d'identité IAM environ une fois par durée de vie des identifiants. Une erreur d'identifiant de l'API efface le cache, et la nouvelle tentative résout les identifiants frais. Nécessite Claude Code v2.1.207 ou ultérieur.

Le cache couvre chaque option d'identifiant ci-dessus sauf une clé API Amazon Bedrock, qui n'utilise pas la chaîne du fournisseur. Pour résoudre la chaîne à chaque demande à la place, définissez [`CLAUDE_CODE_SKIP_AWS_CRED_CACHE=1`](/docs/fr/env-vars).

Chaque résolution de la chaîne expire après 60 secondes. Si une étape de la chaîne s'arrête, par exemple un assistant `credential_process` qui attend une entrée qu'il ne peut pas recevoir, la demande échoue avec [`AWS default-chain credential resolve timed out`](/docs/fr/errors#aws-default-chain-credential-resolve-timed-out). Si votre chaîne exécute une connexion interactive qui a légitimement besoin de plus de temps, comme SSO basé sur navigateur avec MFA via un wrapper comme `aws-vault`, augmentez la limite en millisecondes avec [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/fr/env-vars). Avant v2.1.207, une résolution d'identifiant bloquée laissait la demande en attente indéfiniment.

Sauf quand vous vous authentifiez avec une clé API Amazon Bedrock, l'[assistant de configuration](#sign-in-with-bedrock) applique la même limite à chaque appel AWS qu'il effectue lors de la vérification de vos identifiants, et à la recherche d'identifiants avant chaque vérification de modèle. Lors de la vérification des identifiants, une vérification qui dépasse la limite échoue avec [`Timed out after 60s waiting for AWS`](/docs/fr/errors#bedrock-setup-verification-timed-out-waiting-for-aws).

<h4 id="advanced-credential-configuration">
  Configuration avancée des identifiants
</h4>

Claude Code prend en charge l'actualisation automatique des identifiants pour AWS SSO et les fournisseurs d'identité d'entreprise. Ajoutez ces paramètres à votre fichier de paramètres Claude Code (voir [Paramètres](/docs/fr/settings) pour les emplacements des fichiers).

Ces deux paramètres ont des conditions de déclenchement différentes :

* **`awsAuthRefresh`** : s'exécute uniquement quand Claude Code détecte que vos identifiants AWS ont expiré, soit localement en fonction de leur horodatage, soit quand l'API retourne une erreur d'identifiant, puis réessaie la demande avec des identifiants actualisés.
* **`awsCredentialExport`** : s'exécute au démarrage de la session et à chaque rechargement d'identifiant, même quand les identifiants dans votre chaîne du fournisseur d'identifiants par défaut AWS sont toujours valides. Utilisez ceci quand votre compte Amazon Bedrock nécessite des identifiants inter-comptes qui diffèrent de ceux que la chaîne du fournisseur par défaut résoudrait.

Avant d'exécuter la commande `awsAuthRefresh`, Claude Code effectue un appel STS `GetCallerIdentity` pour confirmer que vos identifiants ont réellement expiré, et ignore la commande quand ils fonctionnent toujours. Claude Code envoie cette vérification via votre [configuration de proxy](/docs/fr/network-config#proxy-configuration), en respectant `HTTPS_PROXY` et `NO_PROXY`. Avant v2.1.239, Claude Code envoyait cette vérification directement et s'arrêtait au démarrage sur les réseaux qui ne permettent la sortie que via un proxy.

<h5 id="example-configuration">
  Exemple de configuration
</h5>

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile",
  "env": {
    "AWS_PROFILE": "myprofile"
  }
}
```

<h5 id="configuration-settings-explained">
  Paramètres de configuration expliqués
</h5>

**`awsAuthRefresh`** : Utilisez ceci pour les commandes qui modifient le répertoire `.aws`, comme la mise à jour des identifiants, du cache SSO ou des fichiers de configuration. La sortie de la commande est affichée à l'utilisateur, mais l'entrée interactive n'est pas prise en charge. Cela fonctionne bien pour les flux SSO basés sur navigateur où l'interface de ligne de commande affiche une URL ou un code et vous complétez l'authentification dans le navigateur.

**`awsCredentialExport`** : Utilisez ceci uniquement si vous ne pouvez pas modifier `.aws` et devez retourner directement les identifiants. La sortie est capturée silencieusement et non affichée à l'utilisateur. La commande doit afficher JSON dans ce format :

```json theme={null}
{
  "Credentials": {
    "AccessKeyId": "value",
    "SecretAccessKey": "value",
    "SessionToken": "value",
    "Expiration": "2026-01-01T00:00:00Z"
  }
}
```

La sortie plate de `aws configure export-credentials --format process` est également acceptée, avec les mêmes clés au niveau supérieur au lieu d'être imbriquées sous `Credentials`.

`Expiration` est optionnel. Quand la commande retourne un `Expiration` ISO 8601 valide, Claude Code met en cache les identifiants jusqu'à cinq minutes avant cette heure. Sans cela, les identifiants sont mis en cache pendant une heure.

Quand vous configurez `awsCredentialExport` sans `awsAuthRefresh`, Claude Code utilise les identifiants exportés directement et ne ré-résout pas la chaîne du fournisseur d'identifiants par défaut AWS au démarrage. Nécessite Claude Code v2.1.206 ou ultérieur.

<h3 id="3-configure-claude-code">
  3. Configurer Claude Code
</h3>

Définissez les variables d'environnement suivantes pour activer Amazon Bedrock :

```bash theme={null}
# Enable Bedrock integration
export CLAUDE_CODE_USE_BEDROCK=1
export AWS_REGION=us-east-1  # optional if your AWS profile already sets a region

# Optional: Override the AWS region for the small/fast model (Bedrock and Mantle).
# On Bedrock, has no effect without ANTHROPIC_DEFAULT_HAIKU_MODEL
# or the deprecated ANTHROPIC_SMALL_FAST_MODEL set.
export ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION=us-west-2

# Optional: Override the Bedrock endpoint URL for custom endpoints or gateways
# export ANTHROPIC_BEDROCK_BASE_URL=https://bedrock-runtime.us-east-1.amazonaws.com
```

Lors de l'activation d'Amazon Bedrock pour Claude Code, gardez à l'esprit les points suivants :

* Vous n'avez besoin de définir `AWS_REGION` que pour remplacer la région de votre profil AWS ou quand votre profil n'a pas de région. Claude Code résout la région dans cet ordre :

  * `AWS_REGION`
  * `AWS_DEFAULT_REGION`
  * la `region` définie sur votre profil AWS actif, lue d'abord à partir du fichier des identifiants partagés AWS puis du fichier de configuration partagée, correspondant à la précédence du SDK AWS
  * `us-east-1`

  Si une valeur de l'une de ces sources n'a pas la forme d'un nom de région, Claude Code la traite comme non définie et continue dans l'ordre. Par exemple, Claude Code traite une valeur contenant une barre oblique, un point ou un espace comme non définie.

  Le profil actif est `AWS_PROFILE` s'il est défini, sinon `default`. Définissez `AWS_SHARED_CREDENTIALS_FILE` ou `AWS_CONFIG_FILE` pour pointer vers des chemins de fichiers non par défaut.

  Exécutez `/status` pour voir la région résolue. Quand la région provient de vos fichiers de configuration AWS ou du secours par défaut, Claude Code note également la source dans la sortie `/status`.
* Lors de l'utilisation d'Amazon Bedrock, la commande `/logout` n'est pas disponible puisque l'authentification est gérée via les identifiants AWS.
* L'outil WebSearch n'est pas disponible sur Amazon Bedrock. Voir [Comportement de l'outil WebSearch](/docs/fr/tools-reference#websearch-tool-behavior).
* Vous pouvez utiliser des fichiers de paramètres pour les variables d'environnement comme `AWS_PROFILE` que vous ne voulez pas divulguer à d'autres processus. Voir [Paramètres](/docs/fr/settings) pour plus d'informations.

<h3 id="4-pin-model-versions">
  4. Épingler les versions de modèle
</h3>

<Warning>
  Épinglez les versions de modèle spécifiques lors du déploiement pour plusieurs utilisateurs. Sans épinglage, les alias de modèle tels que `sonnet` et `opus` se résolvent à la valeur par défaut intégrée de Claude Code pour Amazon Bedrock, qui peut être en retard sur la version la plus récente et peut ne pas encore être disponible dans votre compte. Claude Code [revient](#startup-model-checks) à un modèle antérieur ou de niveau inférieur au démarrage quand la valeur par défaut n'est pas disponible, mais l'épinglage vous permet de contrôler quand vos utilisateurs passent à un nouveau modèle.
</Warning>

Définissez ces variables d'environnement sur des ID de modèle Amazon Bedrock spécifiques.

Sans `ANTHROPIC_DEFAULT_OPUS_MODEL`, l'alias `opus` sur Amazon Bedrock se résout à Opus 5.5, et sans `ANTHROPIC_DEFAULT_SONNET_MODEL`, l'alias `sonnet` se résout à Sonnet 4.5. Cet exemple épingle chaque alias à une version spécifique :

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'
```

Ces ID utilisent le préfixe du profil d'inférence inter-régions `us.`. Si vous utilisez un préfixe de région différent ou des profils d'inférence d'application, ajustez en conséquence. Dans les régions AWS GovCloud, utilisez le préfixe `us-gov.`.

Pour conserver les modèles par défaut intégrés et modifier uniquement leur préfixe préféré, définissez [`ANTHROPIC_BEDROCK_REGION_PREFIX`](#cross-region-inference-profile-prefixes) au lieu d'épingler. La différence s'affiche dans ce que l'alias `opus` se résout à :

| Vous définissez                                               | L'alias `opus` se résout à                                                               |
| :------------------------------------------------------------ | :--------------------------------------------------------------------------------------- |
| `ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'` | `us.anthropic.claude-opus-4-8`, l'ID exact que vous avez épinglé                         |
| `ANTHROPIC_BEDROCK_REGION_PREFIX=eu`                          | `eu.anthropic.claude-opus-5-5`, la valeur par défaut intégrée avec votre préfixe préféré |

Pour les ID de modèle actuels et hérités, voir [Aperçu des modèles](https://platform.claude.com/docs/en/about-claude/models/overview). Pour la liste complète des variables d'environnement d'épinglage, voir [Configuration du modèle](/docs/fr/model-config#pin-models-for-third-party-deployments).

Claude Code utilise ces modèles par défaut quand aucune variable d'épinglage n'est définie :

| Type de modèle      | Modèle par défaut                                                                             |
| :------------------ | :-------------------------------------------------------------------------------------------- |
| Modèle principal    | Opus 5.5, par exemple `us.anthropic.claude-opus-5-5` dans une région `us-*`                   |
| Modèle petit/rapide | Sonnet 4.5, par exemple `us.anthropic.claude-sonnet-4-5-20250929-v1:0` dans une région `us-*` |

Les tâches de fond telles que la génération de titre de session utilisent le modèle petit/rapide, normalement un modèle de classe Haiku. Sur Amazon Bedrock, Claude Code utilise le modèle Sonnet par défaut pour les tâches de fond car Haiku peut ne pas être activé dans chaque compte ou région. Deux sélections changent quel modèle les porte :

* Quand vous sélectionnez un modèle principal avec `--model`, `ANTHROPIC_MODEL`, ou le paramètre `model`, les tâches de fond utilisent ce modèle. Quand Claude Code démarre la session sur le modèle que vous avez défini avec [`ANTHROPIC_DEFAULT_MODEL`](/docs/fr/model-config#set-a-default-model-for-new-sessions), les tâches de fond utilisent ce modèle aussi. Définir `ANTHROPIC_DEFAULT_OPUS_MODEL` sans `ANTHROPIC_DEFAULT_SONNET_MODEL` compte aussi comme une sélection, car le modèle Sonnet intégré peut ne pas être activé dans un compte qui dirige son propre Opus.
* Pour utiliser Haiku pour les tâches de fond, définissez `ANTHROPIC_DEFAULT_HAIKU_MODEL` sur un ID de modèle qui est disponible dans votre compte.

<Warning>
  Les modèles Opus ont un prix par jeton plus élevé que les modèles Sonnet, donc un déploiement qui n'épingle pas un modèle principal est facturé au tarif Opus une fois qu'il se met à jour vers v2.1.207 ou ultérieur. Pour conserver Sonnet 4.5 comme modèle principal, définissez `ANTHROPIC_MODEL` sur son ID de modèle complet. Un déploiement qui dirige la valeur par défaut avec `ANTHROPIC_DEFAULT_SONNET_MODEL` et ne définit pas `ANTHROPIC_DEFAULT_OPUS_MODEL` conserve son modèle Sonnet dirigé comme valeur par défaut.
</Warning>

Avant v2.1.280, le modèle principal sur Amazon Bedrock était par défaut Opus 5 et l'alias `opus` se résolvait à Opus 5 à partir de v2.1.219. Sur v2.1.207 à v2.1.218, le modèle principal sur Amazon Bedrock était par défaut Opus 4.8 et l'alias `opus` se résolvait à Opus 4.8. Avant v2.1.207, le modèle principal était par défaut Sonnet 4.5, l'alias `opus` se résolvait à Opus 4.6, et les tâches de fond utilisaient toujours le modèle principal.

Pour personnaliser davantage les modèles, utilisez l'une de ces méthodes :

```bash theme={null}
# Using inference profile ID
export ANTHROPIC_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'

# Using application inference profile ARN
export ANTHROPIC_MODEL='arn:aws:bedrock:us-east-2:your-account-id:application-inference-profile/your-model-id'

# Optional: Disable prompt caching if needed
# export DISABLE_PROMPT_CACHING=1

# Optional: Request 1-hour prompt cache TTL instead of the 5-minute default
# export ENABLE_PROMPT_CACHING_1H=1
```

La TTL du cache d'une heure est facturée à un tarif plus élevé que la valeur par défaut de 5 minutes. Voir [durée de vie du cache](/docs/fr/prompt-caching#cache-lifetime). Pour définir des TTL différentes pour votre conversation principale et pour les demandes que Claude Code effectue en dehors de celle-ci, [choisissez la TTL vous-même](/docs/fr/prompt-caching#choose-the-ttl-yourself).

<Note>La mise en cache des invites peut ne pas être disponible dans toutes les régions Amazon Bedrock. Si les nombres de jetons du cache restent à zéro, vérifiez les [modèles, régions et limites pris en charge](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) dans la documentation Amazon Bedrock.</Note>

<h4 id="map-each-model-version-to-an-inference-profile">
  Mapper chaque version de modèle à un profil d'inférence
</h4>

Les variables d'environnement `ANTHROPIC_DEFAULT_*_MODEL` configurent un profil d'inférence par famille de modèles. Si votre organisation a besoin d'exposer plusieurs versions de la même famille dans le sélecteur `/model`, chacune acheminée vers son propre ARN de profil d'inférence d'application, utilisez le paramètre `modelOverrides` dans votre [fichier de paramètres](/docs/fr/settings#where-settings-live) à la place.

Cet exemple mappe quatre versions d'Opus à des ARN distincts afin que les utilisateurs puissent basculer entre elles sans contourner les profils d'inférence de votre organisation :

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-47-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-opus-4-5-20251101": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-45-prod",
    "claude-opus-4-1-20250805": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-41-prod"
  }
}
```

Quand un utilisateur sélectionne l'une de ces versions dans `/model`, Claude Code appelle Amazon Bedrock avec l'ARN mappé. Le même mappage s'applique quand vous passez l'ID de modèle Anthropic directement via `--model` ou `ANTHROPIC_MODEL`. Les versions sans remplacement reviennent à l'ID de modèle Amazon Bedrock intégré ou à tout profil d'inférence correspondant découvert au démarrage. Avant v2.1.200, les valeurs `--model` et `ANTHROPIC_MODEL` atteignaient Amazon Bedrock telles quelles sans passer par la carte de remplacement. Voir [Remplacer les ID de modèle par version](/docs/fr/model-config#override-model-ids-per-version) pour les détails sur la façon dont les remplacements interagissent avec `availableModels` et d'autres paramètres de modèle.

<h2 id="startup-model-checks">
  Vérifications du modèle au démarrage
</h2>

Lorsque Claude Code démarre avec Amazon Bedrock configuré, il vérifie que les modèles qu'il a l'intention d'utiliser sont accessibles dans votre compte.

Si vous avez épinglé une version de modèle plus ancienne que la valeur par défaut actuelle de Claude Code, et que votre compte peut invoquer la version plus récente, Claude Code vous invite à mettre à jour l'épingle. L'acceptation écrit le nouvel ID de modèle dans votre [fichier de paramètres utilisateur](/docs/fr/settings) et redémarre Claude Code. Le refus est mémorisé jusqu'au prochain changement de version par défaut. Les épingles qui pointent vers un [ARN de profil d'inférence d'application](#map-each-model-version-to-an-inference-profile) sont ignorées, car celles-ci sont gérées par votre administrateur.

Si vous n'avez pas épinglé de modèle et que la valeur par défaut actuelle n'est pas disponible dans votre compte, Claude Code bascule pour la session actuelle et affiche un avis. Il essaie d'abord les versions antérieures du modèle par défaut et, lorsque la valeur par défaut est un modèle Opus et qu'aucune version Opus n'est disponible, bascule vers le modèle Sonnet par défaut. Le basculement n'est pas persistant. Activez le modèle plus récent dans votre compte Amazon Bedrock ou [épinglez une version](#4-pin-model-versions) pour rendre le choix permanent.

Lorsque vous démarrez la session sur une version spécifique de Sonnet ou Opus, par exemple avec `--model`, `ANTHROPIC_MODEL`, ou le [paramètre `model`](/docs/fr/settings-reference#model), cette version agit comme la valeur par défaut épinglée de la session pour l'alias `sonnet` ou `opus` correspondant. Claude Code ignore la vérification de disponibilité pour la valeur par défaut intégrée que votre modèle remplace et démarre sur le modèle que vous avez configuré, sans avis de basculement.

Les alias de modèle tels que `opus` n'agissent pas comme des épingles, et il en va de même pour un ID de modèle que Claude Code ne reconnaît pas, comme un ARN de profil d'inférence d'application.

<h2 id="cross-region-inference-profile-prefixes">
  Préfixes de profil d'inférence inter-régions
</h2>

Sur l'[API Invoke](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html) d'Amazon Bedrock, Claude Code résout ses modèles par défaut intégrés en [ID de profil d'inférence inter-régions](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html) ; pour acheminer les versions de modèles via vos propres profils d'inférence, consultez [Mapper chaque version de modèle à un profil d'inférence](#map-each-model-version-to-an-inference-profile). Ce tableau montre le préfixe que Claude Code préfère pour chaque région AWS résolue :

| Région AWS                | Préfixe   |
| :------------------------ | :-------- |
| `us-gov-*` (AWS GovCloud) | `us-gov.` |
| `us-*`                    | `us.`     |
| `eu-*`                    | `eu.`     |
| `ap-*`                    | `apac.`   |
| Toutes les autres régions | `global.` |

Définissez `ANTHROPIC_BEDROCK_REGION_PREFIX` pour choisir le préfixe que Claude Code essaie en premier ; lorsque Claude Code peut vérifier la disponibilité du profil et ne trouve aucun profil correspondant pour un modèle, il bascule comme décrit dans l'ordre de résolution ci-dessous. Les valeurs valides sont `us`, `eu`, `apac`, `jp`, `au` et `global`. Par exemple, définissez-le sur `global` lorsque votre compte a des profils `global.` activés mais que Claude Code en dériverait un spécifique à une géographie à partir de votre région AWS. Nécessite Claude Code v2.1.224 ou ultérieur.

Cet exemple achemine les modèles par défaut via des profils `global.` :

```bash theme={null}
export ANTHROPIC_BEDROCK_REGION_PREFIX=global
# Dans une région us-*, le modèle principal se résout maintenant en
# global.anthropic.claude-opus-5-5 au lieu de us.anthropic.claude-opus-5-5
```

Le préfixe préféré est une préférence, pas une garantie, qu'il provienne de votre région ou de la variable. La façon dont Claude Code l'applique dépend de sa capacité à vérifier la disponibilité du profil dans votre compte :

* Lorsque Claude Code peut [lister les profils d'inférence](#iam-configuration) dans votre compte, il résout chaque modèle dans cet ordre :
  1. Le profil avec votre préfixe préféré.
  2. Tout profil correspondant, pour un modèle qui n'a pas de profil avec ce préfixe.
  3. L'ID de modèle intégré avec votre préfixe préféré, pour un modèle qui n'a pas de profil correspondant du tout. Claude Code applique cet ID sans vérifier la disponibilité à cette étape ; les [vérifications du modèle au démarrage](#startup-model-checks) couvrent toujours les modèles par défaut de la session.
* Lorsque la découverte de profil n'est pas disponible, Claude Code applique le préfixe sans vérifier la disponibilité. Si votre compte n'a pas de profils d'inférence avec ce préfixe activés, les demandes échouent avec une erreur 400.

Claude Code ne réécrit pas les ID de profil d'inférence Amazon Bedrock ou les ARN que vous configurez vous-même, ou les valeurs [`modelOverrides`](#map-each-model-version-to-an-inference-profile) ; les ID de modèle au format Anthropic se résolvent via [le même mappage que le sélecteur `/model`](#map-each-model-version-to-an-inference-profile). Claude Code ignore également la variable dans deux cas :

* Dans les régions AWS GovCloud, Claude Code utilise toujours `us-gov.`, le seul préfixe qui achemine au sein de la partition GovCloud.
* Lorsque vous définissez une valeur qui n'est pas l'une des valeurs valides, Claude Code bascule vers le préfixe préféré dérivé de la région.

<h2 id="iam-configuration">
  Configuration IAM
</h2>

Créez une politique IAM avec les autorisations requises pour Claude Code :

```json theme={null}
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AllowModelAndInferenceProfileAccess",
      "Effect": "Allow",
      "Action": [
        "bedrock:InvokeModel",
        "bedrock:InvokeModelWithResponseStream",
        "bedrock:ListInferenceProfiles",
        "bedrock:GetInferenceProfile"
      ],
      "Resource": [
        "arn:aws:bedrock:*:*:inference-profile/*",
        "arn:aws:bedrock:*:*:application-inference-profile/*",
        "arn:aws:bedrock:*:*:foundation-model/*"
      ]
    },
    {
      "Sid": "AllowMarketplaceSubscription",
      "Effect": "Allow",
      "Action": [
        "aws-marketplace:ViewSubscriptions",
        "aws-marketplace:Subscribe"
      ],
      "Resource": "*",
      "Condition": {
        "StringEquals": {
          "aws:CalledViaLast": "bedrock.amazonaws.com"
        }
      }
    }
  ]
}
```

Pour des autorisations plus restrictives, vous pouvez limiter la ressource à des ARN de profil d'inférence spécifiques.

`bedrock:GetInferenceProfile` permet à Claude Code de résoudre un [ARN de profil d'inférence d'application](#map-each-model-version-to-an-inference-profile) vers son modèle de fondation de support, qui est utilisé pour sélectionner la forme de requête correcte pour ce modèle.

Si le jeton ne dispose pas de cette autorisation, Claude Code se rétablit automatiquement en réessayant une fois avec la forme alternative, de sorte que les requêtes réussissent toujours mais chaque nouveau modèle ajoute un aller-retour supplémentaire. L'octroi de l'autorisation évite la nouvelle tentative. Cela s'applique le plus souvent aux déploiements `AWS_BEARER_TOKEN_BEDROCK`, où la politique du jeton est généralement plus étroite qu'un rôle IAM complet.

Pour plus de détails, voir [Documentation IAM Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/security-iam.html).

<Note>
  Créez un compte AWS dédié pour Claude Code pour simplifier le suivi des coûts et le contrôle d'accès.
</Note>

<h2 id="1m-token-context-window">
  Fenêtre de contexte de 1M de jetons
</h2>

Claude Sonnet 5, Opus 4.6 et versions ultérieures, ainsi que Sonnet 4.6, prennent en charge la [fenêtre de contexte de 1M de jetons](https://platform.claude.com/docs/fr/build-with-claude/context-windows#context-window-sizes-by-model) sur Amazon Bedrock. Sonnet 5 s'exécute toujours avec la fenêtre 1M sur l'API Invoke et le [point de terminaison Mantle](#use-the-mantle-endpoint), sans variante `[1m]` à sélectionner. Pour les autres modèles sur l'API Invoke, Claude Code active automatiquement la fenêtre de contexte étendue lorsque vous sélectionnez une variante de modèle 1M.

L'[assistant de configuration](#sign-in-with-bedrock) offre une option de contexte 1M lorsqu'il épingle les modèles. Pour l'activer pour un modèle épinglé manuellement à la place, ajoutez `[1m]` à l'ID du modèle. Voir [Épingler les modèles pour les déploiements tiers](/docs/fr/model-config#pin-models-for-third-party-deployments) pour plus de détails.

<h2 id="service-tiers">
  Niveaux de service
</h2>

[Les niveaux de service Amazon Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/service-tiers-inference.html) vous permettent de faire un compromis entre le coût et la latence. Définissez `ANTHROPIC_BEDROCK_SERVICE_TIER` sur `default`, `flex` ou `priority` :

```bash theme={null}
export ANTHROPIC_BEDROCK_SERVICE_TIER=priority
```

Claude Code envoie ceci comme en-tête `X-Amzn-Bedrock-Service-Tier` sur chaque demande. La disponibilité des niveaux varie selon le modèle et la région. La capacité réservée utilise un ARN de [débit provisionné](https://docs.aws.amazon.com/bedrock/latest/userguide/prov-throughput.html) comme ID de modèle au lieu de ce paramètre.

<h2 id="aws-guardrails">
  Garde-fous AWS
</h2>

[Les garde-fous Amazon Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails.html) vous permettent de mettre en œuvre le filtrage du contenu pour Claude Code. Créez un garde-fou dans la [console Amazon Bedrock](https://console.aws.amazon.com/bedrock/), publiez une version, puis ajoutez les en-têtes du garde-fou à votre [fichier de paramètres](/docs/fr/settings). Activez l'inférence inter-régions sur votre garde-fou si vous utilisez des profils d'inférence inter-régions.

Exemple de configuration :

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Amzn-Bedrock-GuardrailIdentifier: your-guardrail-id\nX-Amzn-Bedrock-GuardrailVersion: 1"
  }
}
```

Si votre organisation fournit les en-têtes du garde-fou via une politique de [passerelle d'applications Claude](/docs/fr/claude-apps-gateway), ils comptent comme des [paramètres nécessitant une approbation](/docs/fr/server-managed-settings#environment-variables-and-the-approval-dialog).

<h2 id="use-the-mantle-endpoint">
  Utiliser le point de terminaison Mantle
</h2>

Mantle est un point de terminaison Amazon Bedrock qui sert les modèles Claude via la forme API Anthropic native plutôt que l'API Invoke Bedrock. Il utilise les mêmes [identifiants AWS](#2-configure-aws-credentials) et configuration [`awsAuthRefresh`](#advanced-credential-configuration).

Mantle possède ses propres actions IAM sous le préfixe `bedrock-mantle:`, donc les actions `bedrock:` dans la [configuration IAM](#iam-configuration) ne le couvrent pas. Accordez à votre identité IAM `bedrock-mantle:CreateInference` pour l'inférence et `bedrock-mantle:CountTokens` pour le comptage des jetons. Consultez [Effectuer des demandes d'inférence](https://docs.aws.amazon.com/bedrock/latest/userguide/inference.html) et [Compter les jetons](https://docs.aws.amazon.com/bedrock/latest/userguide/count-tokens.html) dans la documentation AWS, et la [référence d'autorisation de service](https://docs.aws.amazon.com/service-authorization/latest/reference/list_amazonbedrockpoweredbyawsmantle.html) pour chaque action Mantle.

<h3 id="enable-mantle">
  Activer Mantle
</h3>

Avec les identifiants AWS déjà configurés, définissez `CLAUDE_CODE_USE_MANTLE` pour acheminer les demandes vers le point de terminaison Mantle :

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export AWS_REGION=us-east-1
```

Claude Code construit l'URL du point de terminaison à partir de la région AWS, résolue avec la même priorité que [Bedrock ci-dessus](#3-configure-claude-code). Pour remplacer l'URL pour un point de terminaison personnalisé ou une passerelle, définissez `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`.

Exécutez `/status` dans Claude Code pour confirmer. La ligne du fournisseur affiche `Amazon Bedrock (Mantle)` lorsque Mantle est actif.

<h3 id="select-a-mantle-model">
  Sélectionner un modèle Mantle
</h3>

Mantle utilise des ID de modèle préfixés avec `anthropic.` et sans suffixe de version, par exemple `anthropic.claude-sonnet-5` ou `anthropic.claude-haiku-4-5`. Les modèles disponibles pour votre compte dépendent de ce que votre organisation a reçu ; les ID de modèle supplémentaires sont répertoriés dans vos documents d'intégration d'AWS. Contactez votre équipe de compte AWS pour demander l'accès aux modèles autorisés.

Définissez le modèle avec l'indicateur `--model` ou avec `/model` dans Claude Code :

```bash theme={null}
claude --model anthropic.claude-haiku-4-5
```

<h3 id="run-mantle-alongside-the-invoke-api">
  Exécuter Mantle aux côtés de l'API Invoke
</h3>

Les modèles disponibles pour vous sur Mantle peuvent ne pas inclure tous les modèles que vous utilisez aujourd'hui. La définition de `CLAUDE_CODE_USE_BEDROCK` et `CLAUDE_CODE_USE_MANTLE` permet à Claude Code d'appeler les deux points de terminaison à partir de la même session. Les ID de modèle qui correspondent au format Mantle sont acheminés vers Mantle, et tous les autres ID de modèle vont à l'API Invoke Bedrock.

```bash theme={null}
export CLAUDE_CODE_USE_BEDROCK=1
export CLAUDE_CODE_USE_MANTLE=1
```

Pour afficher un modèle Mantle dans le sélecteur `/model`, répertoriez son ID dans `availableModels` dans votre [fichier de paramètres](/docs/fr/settings). Ce paramètre restreint également le sélecteur aux entrées répertoriées. Répertorier `anthropic.claude-haiku-4-5` supprime l'alias `haiku` nu du sélecteur, donc répertoriez également les préfixes de version ou les ID complets pour les versions que vous souhaitez garder sélectionnables. L'ID Mantle et l'alias `haiku` se résolvent à la même famille de modèles, donc la fusion conserve uniquement l'entrée plus spécifique. Voir [Comportement de fusion](/docs/fr/model-config#merge-behavior) :

```json theme={null}
{
  "availableModels": ["opus", "sonnet", "claude-haiku-4-5", "anthropic.claude-haiku-4-5"]
}
```

Les entrées avec le préfixe `anthropic.` sont ajoutées en tant qu'options de sélecteur personnalisées et acheminées vers Mantle. Remplacez `anthropic.claude-haiku-4-5` par l'ID de modèle que votre compte a reçu. Voir [Restreindre la sélection du modèle](/docs/fr/model-config#restrict-model-selection) pour savoir comment `availableModels` interagit avec d'autres paramètres de modèle.

Lorsque les deux fournisseurs sont actifs, `/status` affiche `Amazon Bedrock + Amazon Bedrock (Mantle)`.

<h3 id="route-mantle-through-a-gateway">
  Acheminer Mantle via une passerelle
</h3>

Si votre organisation achemine le trafic du modèle via une [passerelle LLM](/docs/fr/llm-gateway) centralisée qui injecte les identifiants AWS côté serveur, désactivez l'authentification côté client afin que Claude Code envoie les demandes sans signatures SigV4 ou en-têtes `x-api-key` :

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export CLAUDE_CODE_SKIP_MANTLE_AUTH=1
export ANTHROPIC_BEDROCK_MANTLE_BASE_URL=https://your-gateway.example.com
```

<h3 id="mantle-environment-variables">
  Variables d'environnement Mantle
</h3>

Ces variables sont spécifiques au point de terminaison Mantle. Voir [Variables d'environnement](/docs/fr/env-vars) pour la liste complète.

| Variable                                | Objectif                                                                      |
| :-------------------------------------- | :---------------------------------------------------------------------------- |
| `CLAUDE_CODE_USE_MANTLE`                | Activer le point de terminaison Mantle. Définissez sur `1` ou `true`.         |
| `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`     | Remplacer l'URL du point de terminaison Mantle par défaut                     |
| `CLAUDE_CODE_SKIP_MANTLE_AUTH`          | Ignorer l'authentification côté client pour les configurations de proxy       |
| `ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION` | Remplacer la région AWS pour le modèle de classe Haiku (partagé avec Bedrock) |

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="authentication-loop-with-sso-and-corporate-proxies">
  Boucle d'authentification avec SSO et proxies d'entreprise
</h3>

Si des onglets de navigateur s'ouvrent à plusieurs reprises lors de l'utilisation d'AWS SSO, supprimez le paramètre `awsAuthRefresh` de votre [fichier de paramètres](/docs/fr/settings). Cela peut se produire lorsque les VPN d'entreprise ou les proxies d'inspection TLS interrompent le flux SSO du navigateur. Claude Code traite la connexion interrompue comme un échec d'authentification, réexécute `awsAuthRefresh` et boucle indéfiniment.

Si votre environnement réseau interfère avec les flux SSO automatiques basés sur un navigateur, utilisez `aws sso login` manuellement avant de démarrer Claude Code au lieu de vous fier à `awsAuthRefresh`.

<h3 id="certificate-errors-behind-a-tls-inspecting-proxy">
  Erreurs de certificat derrière un proxy d'inspection TLS
</h3>

Claude Code applique votre configuration du [magasin de certificats CA](/docs/fr/network-config#ca-certificate-store) à ses demandes à AWS, y compris :

* Découverte de modèle
* Comptage des jetons
* Les appels de rôle STS et SSO qui résolvent vos identifiants AWS
* La vérification des identifiants et les vérifications de modèle de l'[assistant de configuration](#sign-in-with-bedrock)

Pour ces demandes, un certificat racine d'entreprise dans votre magasin de confiance du système d'exploitation ou le bundle `NODE_EXTRA_CA_CERTS` ne nécessite aucune configuration spécifique à Amazon Bedrock.

Avant la v2.1.260, Claude Code appliquait votre configuration CA à ces demandes uniquement lorsqu'elles passaient par un proxy configuré, et sur une connexion directe, elles ne faisaient confiance qu'au magasin de certificats par défaut du runtime.

Avant la v2.1.261, la recherche d'identifiants derrière les vérifications de modèle de l'assistant de configuration avec l'option **Utiliser les identifiants déjà dans mon environnement** ne faisait toujours confiance qu'au magasin de certificats par défaut du runtime. Derrière un proxy d'inspection TLS dont le certificat racine se trouve uniquement dans le magasin du système d'exploitation, les demandes affectées ont échoué avec `unable to get local issuer certificate`, ou l'assistant a montré les modèles comme `unreachable`, tandis que les demandes d'inférence ont réussi. Mettez à jour vers la v2.1.261 ou une version ultérieure.

<h3 id="region-issues">
  Problèmes de région
</h3>

Si vous rencontrez des problèmes de région :

* Vérifiez la disponibilité du modèle : `aws bedrock list-inference-profiles --region your-region`
* Basculez vers une région prise en charge : `export AWS_REGION=us-east-1`
* Envisagez d'utiliser des profils d'inférence pour l'accès inter-régions

Si vous recevez une erreur « on-demand throughput isn't supported » :

* Spécifiez le modèle comme ID de [profil d'inférence](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)

Claude Code utilise l'API Amazon Bedrock [Invoke](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html) et ne prend pas en charge l'API Converse.

<h3 id="streaming-errors-behind-a-gateway-or-proxy">
  Erreurs de streaming derrière une passerelle ou un proxy
</h3>

Amazon Bedrock diffuse les réponses `InvokeModelWithResponseStream` dans un format d'événement binaire event-stream avec l'en-tête `Content-Type: application/vnd.amazon.eventstream`. Une passerelle ou un proxy entre Claude Code et Amazon Bedrock doit transmettre le corps de la réponse et ses en-têtes, y compris `Content-Type`, tels qu'Amazon Bedrock les a envoyés.

Si la passerelle réécrit `Content-Type` en une autre valeur, Claude Code rejette la réponse avec une erreur qui commence par `Bedrock streaming response has content-type`, en nommant la valeur qu'il a reçue. La réécriture courante est `text/event-stream`, provenant d'une intégration qui réemet le flux sous forme d'événements envoyés par le serveur.

Si la passerelle supprime ou efface l'en-tête à la place, Claude Code suppose que le corps est le flux d'événements d'Amazon Bedrock et le décode, donc un corps que la passerelle a transmis sans modification continue à diffuser.

Si une passerelle qui supprime l'en-tête réemet également le flux sous forme d'événements envoyés par le serveur, Claude Code ne peut pas décoder le corps et revient à un chemin plus lent sans streaming à chaque tour : chaque réponse n'apparaît qu'une fois qu'elle est complète au lieu de diffuser. Dans ce cas, définissez [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_DEFAULT=1`](/docs/fr/env-vars) afin que Claude Code lise le corps sous forme d'événements envoyés par le serveur à la place.

Pour corriger l'erreur ou le repli, configurez la passerelle pour transmettre le corps de la réponse `InvokeModelWithResponseStream` et son en-tête `Content-Type` sans modification.

Une passerelle qui convertit le flux en événements envoyés par le serveur ne sert plus l'API Amazon Bedrock. Si elle accepte également les demandes de l'API Messages Anthropic, connectez-vous à elle en tant que [passerelle LLM](/docs/fr/llm-gateway-connect) avec `ANTHROPIC_BASE_URL` au lieu de `CLAUDE_CODE_USE_BEDROCK`.

<h3 id="zero-token-counts-in-/context">
  Comptages de jetons zéro dans /context
</h3>

La commande `/context` compte les jetons pour chaque groupe d'outils en envoyant les schémas d'outils à l'API count-tokens d'Amazon Bedrock. Sur les versions de Claude Code antérieures à v2.1.196, Amazon Bedrock a rejeté cette demande car les schémas contenaient des champs que son API count-tokens n'accepte pas, donc chaque groupe d'outils affichait 0 jetons. Les autres lignes de la ventilation, telles que les messages et les fichiers de mémoire, ne sont pas affectées.

Mettez à jour vers v2.1.196 ou une version ultérieure.

<h3 id="mantle-endpoint-errors">
  Erreurs du point de terminaison Mantle
</h3>

Si `/status` n'affiche pas `Amazon Bedrock (Mantle)` après avoir défini `CLAUDE_CODE_USE_MANTLE`, la variable n'atteint pas le processus. Confirmez qu'elle est exportée dans le shell où vous avez lancé `claude`, ou définissez-la dans le bloc `env` de votre [fichier de paramètres](/docs/fr/settings).

Ce qu'un `403` du point de terminaison Mantle signifie dépend de si l'erreur nomme une action IAM :

* Si l'erreur nomme une action `bedrock-mantle:`, accordez cette action à votre identité IAM.
* Si l'erreur ne nomme aucune action et que vos identifiants sont valides, votre compte AWS n'a pas reçu l'accès au modèle que vous avez demandé. Contactez votre équipe de compte AWS pour demander l'accès.

Un `400` qui nomme l'ID du modèle signifie que ce modèle n'est pas servi sur Mantle. Mantle a sa propre gamme de modèles distincte du catalogue Amazon Bedrock standard, donc les ID de profil d'inférence tels que `us.anthropic.claude-sonnet-4-6` ne fonctionneront pas. Utilisez un ID au format Mantle, ou activez [les deux points de terminaison](#run-mantle-alongside-the-invoke-api) afin que Claude Code achemine chaque demande vers le point de terminaison où le modèle est disponible.

<h2 id="additional-resources">
  Ressources supplémentaires
</h2>

* [Documentation Amazon Bedrock](https://docs.aws.amazon.com/bedrock/)
* [Tarification Amazon Bedrock](https://aws.amazon.com/bedrock/pricing/)
* [Profils d'inférence Amazon Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)
* [Quota de jetons Amazon Bedrock et réduction des jetons](https://docs.aws.amazon.com/bedrock/latest/userguide/quotas-token-burndown.html)
* [Claude Code sur Amazon Bedrock : Guide de configuration rapide](https://builder.aws.com/content/2tXkZKrZzlrlu0KfH8gST5Dkppq/claude-code-on-amazon-bedrock-quick-setup-guide)
* [Implémentation de la surveillance de Claude Code (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md)
