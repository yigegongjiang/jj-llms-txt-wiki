> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code en Amazon Bedrock

> Aprenda a configurar Claude Code a través de Amazon Bedrock, incluyendo configuración, configuración de IAM y solución de problemas.

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
  Requisitos previos
</h2>

Antes de configurar Claude Code con Amazon Bedrock, asegúrese de tener:

* Una cuenta de AWS con acceso a Amazon Bedrock habilitado
* Acceso a los modelos Claude deseados (por ejemplo, Claude Sonnet 4.6) en Amazon Bedrock
* AWS CLI instalado y configurado (opcional - solo se necesita si no tiene otro mecanismo para obtener credenciales)
* Permisos de IAM apropiados

Para iniciar sesión con sus propias credenciales de Amazon Bedrock, siga [Iniciar sesión con Amazon Bedrock](#sign-in-with-bedrock) a continuación. Para implementar Claude Code en un equipo, utilice los pasos de [configuración manual](#set-up-manually) y [fije las versiones de su modelo](#4-pin-model-versions) antes de implementar.

<h2 id="sign-in-with-bedrock">
  Iniciar sesión con Bedrock
</h2>

Si tiene credenciales de AWS y desea comenzar a usar Claude Code a través de Amazon Bedrock, el asistente de inicio de sesión lo guía a través del proceso. Completa los requisitos previos del lado de AWS una vez por cuenta; el asistente maneja el lado de Claude Code.

<Steps>
  <Step title="Habilitar modelos de Anthropic en su cuenta de AWS">
    En la [consola de Amazon Bedrock](https://console.aws.amazon.com/bedrock/), abra el catálogo de modelos, seleccione un modelo de Anthropic y envíe el formulario de caso de uso. El acceso se otorga inmediatamente después del envío. Vea [Enviar detalles del caso de uso](#1-submit-use-case-details) para AWS Organizations y [configuración de IAM](#iam-configuration) para los permisos que su rol necesita.
  </Step>

  <Step title="Inicie Claude Code y elija Amazon Bedrock">
    Ejecute `claude`. En el mensaje de inicio de sesión, seleccione **3rd-party platform**, luego **Amazon Bedrock**.
  </Step>

  <Step title="Siga los mensajes del asistente">
    Elija cómo se autentica en AWS: un perfil de AWS detectado desde su directorio `~/.aws`, una clave de API de Amazon Bedrock, una clave de acceso y secreto, o credenciales ya en su entorno. El asistente recoge su región, verifica qué modelos de Claude puede invocar su cuenta, y le permite fijarlos. Guarda el resultado en el bloque `env` de su [archivo de configuración de usuario](/docs/es/settings), por lo que no necesita exportar variables de entorno usted mismo.
  </Step>
</Steps>

Después de haber iniciado sesión, ejecute `/setup-bedrock` en cualquier momento para reabrirlo el asistente y cambiar sus credenciales, región o fijaciones de modelo. El paso de fijación de modelo comienza desde sus modelos actualmente fijados. El asistente escribe en `~/.claude/settings.json`, o en `$CLAUDE_CONFIG_DIR/settings.json` cuando [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars#variables) está configurado.

<h2 id="set-up-manually">
  Configurar manualmente
</h2>

Para configurar Amazon Bedrock a través de variables de entorno en lugar del asistente, por ejemplo en CI o una implementación empresarial con script, siga los pasos a continuación.

<h3 id="1-submit-use-case-details">
  1. Enviar detalles del caso de uso
</h3>

Los usuarios por primera vez de modelos de Anthropic deben enviar detalles del caso de uso antes de invocar un modelo. Esto se realiza una vez por cuenta de AWS.

1. Asegúrese de tener los permisos de IAM correctos descritos a continuación
2. Navegue a la [consola de Amazon Bedrock](https://console.aws.amazon.com/bedrock/)
3. Seleccione un modelo de Anthropic del **catálogo de modelos**
4. Complete el formulario de caso de uso. El acceso se otorga inmediatamente después del envío.

Si utiliza AWS Organizations, puede enviar el formulario una vez desde la cuenta de administración utilizando la [API `PutUseCaseForModelAccess`](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_PutUseCaseForModelAccess.html). Esta llamada requiere el permiso de IAM `bedrock:PutUseCaseForModelAccess`. La aprobación se extiende a las cuentas secundarias automáticamente.

<h3 id="2-configure-aws-credentials">
  2. Configurar credenciales de AWS
</h3>

Claude Code utiliza la cadena de credenciales predeterminada del SDK de AWS. Configure sus credenciales utilizando uno de estos métodos:

**Opción A: Configuración de AWS CLI**

```bash theme={null}
aws configure
```

**Opción B: Variables de entorno (clave de acceso)**

```bash theme={null}
export AWS_ACCESS_KEY_ID=your-access-key-id
export AWS_SECRET_ACCESS_KEY=your-secret-access-key
export AWS_SESSION_TOKEN=your-session-token
```

**Opción C: Variables de entorno (perfil SSO)**

Reemplace `your-profile-name` con el nombre de su perfil de AWS antes de ejecutar estos comandos.

```bash theme={null}
aws sso login --profile=your-profile-name

export AWS_PROFILE=your-profile-name
```

Claude Code solicita credenciales de rol desde la región del Centro de Identidades de IAM nombrada por el `sso_region` del perfil, que no necesita coincidir con la región en la que ejecuta Amazon Bedrock. En v2.1.207, la región de Amazon Bedrock anuló `sso_region`, por lo que un perfil cuya instancia del Centro de Identidades de IAM está en una región diferente no se autenticó con un error `Session token not found or invalid`.

**Opción D: Credenciales de la consola de administración de AWS**

```bash theme={null}
aws login
```

[Obtenga más información](https://docs.aws.amazon.com/signin/latest/userguide/command-line-sign-in.html) sobre `aws login`.

**Opción E: Claves de API de Amazon Bedrock**

```bash theme={null}
export AWS_BEARER_TOKEN_BEDROCK=your-bedrock-api-key
```

Las claves de API de Amazon Bedrock proporcionan un método de autenticación más simple sin necesidad de credenciales completas de AWS. [Obtenga más información sobre las claves de API de Amazon Bedrock](https://aws.amazon.com/blogs/machine-learning/accelerate-ai-development-with-amazon-bedrock-api-keys/).

<h4 id="credential-caching-and-resolution-timeout">
  Almacenamiento en caché de credenciales y tiempo de espera de resolución
</h4>

Claude Code resuelve la cadena de proveedores de credenciales predeterminada de AWS una vez y mantiene las credenciales resueltas en memoria. Las reutiliza hasta cinco minutos antes de que expiren, o durante una hora cuando no tienen expiración, por lo que un perfil respaldado por SSO solicita credenciales del Centro de Identidades de IAM aproximadamente una vez por vida útil de credenciales. Un error de credencial de la API borra el caché, y el reintento resuelve credenciales nuevas.

Antes de v2.1.207, Claude Code resolvía la cadena en cada solicitud de API, por lo que un perfil respaldado por SSO solicitaba credenciales nuevas del Centro de Identidades de IAM cada vez y podría ser limitado en implementaciones grandes.

El caché cubre todas las opciones de credenciales anteriores excepto una clave de API de Amazon Bedrock, que no utiliza la cadena de proveedores. Para resolver la cadena en cada solicitud en su lugar, establezca [`CLAUDE_CODE_SKIP_AWS_CRED_CACHE=1`](/docs/es/env-vars).

Cada resolución de la cadena agota el tiempo de espera después de 60 segundos. Si un paso en la cadena se detiene, por ejemplo un asistente `credential_process` que espera entrada que no puede recibir, la solicitud falla con [`AWS default-chain credential resolve timed out`](/docs/es/errors#aws-default-chain-credential-resolve-timed-out). Si su cadena ejecuta un inicio de sesión interactivo que legítimamente necesita más tiempo, como SSO basado en navegador con MFA a través de un contenedor como `aws-vault`, aumente el límite en milisegundos con [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/es/env-vars). Antes de v2.1.207, una resolución de credencial detenida dejaba la solicitud esperando indefinidamente.

<h4 id="advanced-credential-configuration">
  Configuración avanzada de credenciales
</h4>

Claude Code admite la actualización automática de credenciales para AWS SSO y proveedores de identidad corporativos. Agregue estas configuraciones a su archivo de configuración de Claude Code (vea [Configuración](/docs/es/settings) para ubicaciones de archivos).

Estas dos configuraciones tienen diferentes condiciones de activación:

* **`awsAuthRefresh`**: se ejecuta solo cuando Claude Code detecta que sus credenciales de AWS han expirado, ya sea localmente según su marca de tiempo o cuando la API devuelve un error de credencial, luego reintenta la solicitud con credenciales actualizadas.
* **`awsCredentialExport`**: se ejecuta al inicio de la sesión y en cada recarga de credenciales, incluso cuando las credenciales en su cadena de proveedores de credenciales predeterminada de AWS aún son válidas. Utilice esto cuando su cuenta de Amazon Bedrock requiera credenciales entre cuentas que difieran de las que la cadena de proveedores predeterminada resolvería.

<h5 id="example-configuration">
  Configuración de ejemplo
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
  Configuración explicada
</h5>

**`awsAuthRefresh`**: Utilice esto para comandos que modifiquen el directorio `.aws`, como actualizar credenciales, caché de SSO o archivos de configuración. La salida del comando se muestra al usuario, pero la entrada interactiva no es compatible. Esto funciona bien para flujos de SSO basados en navegador donde la CLI muestra una URL o código y usted completa la autenticación en el navegador.

**`awsCredentialExport`**: Solo use esto si no puede modificar `.aws` y debe devolver credenciales directamente. Este comando se ejecuta siempre que sea necesario actualizar las credenciales, no solo cuando las credenciales han expirado. La salida se captura silenciosamente y no se muestra al usuario. El comando debe generar JSON en este formato:

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

A partir de Claude Code v2.1.181, la salida plana de `aws configure export-credentials --format process` también se acepta, con las mismas claves en el nivel superior en lugar de anidadas bajo `Credentials`.

`Expiration` es opcional. A partir de Claude Code v2.1.176, cuando el comando devuelve un `Expiration` válido en ISO 8601, Claude Code almacena en caché las credenciales hasta cinco minutos antes de esa hora. Sin él, o en versiones anteriores, las credenciales se almacenan en caché durante una hora.

Cuando configura `awsCredentialExport` sin `awsAuthRefresh`, Claude Code utiliza las credenciales exportadas directamente y no re-resuelve la cadena de proveedores de credenciales predeterminada de AWS al inicio. Antes de v2.1.206, el inicio también re-resolvía la cadena de proveedores predeterminada, lo que realizaba una llamada SSO o STS en vivo fuera de su configuración de proxy y podría bloquear el primer mensaje durante varios minutos en redes con salida restringida.

<h3 id="3-configure-claude-code">
  3. Configurar Claude Code
</h3>

Establezca las siguientes variables de entorno para habilitar Amazon Bedrock:

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

Al habilitar Amazon Bedrock para Claude Code, tenga en cuenta lo siguiente:

* A partir de v2.1.172, solo necesita establecer `AWS_REGION` para anular la región de su perfil de AWS o cuando su perfil no tiene región. Claude Code resuelve la región en este orden:

  * `AWS_REGION`
  * `AWS_DEFAULT_REGION`
  * la `region` establecida en su perfil de AWS activo, leída primero desde el archivo de credenciales compartidas de AWS y luego desde el archivo de configuración compartida, coincidiendo con la precedencia del SDK de AWS
  * `us-east-1`

  El perfil activo es `AWS_PROFILE` si está establecido, de lo contrario `default`. Establezca `AWS_SHARED_CREDENTIALS_FILE` o `AWS_CONFIG_FILE` para apuntar a rutas de archivo no predeterminadas. Ejecute `/status` para ver la región resuelta. Cuando la región proviene de sus archivos de configuración de AWS o del fallback predeterminado, `/status` también anota la fuente. En v2.1.171 y anteriores, Claude Code no lee los archivos de configuración de AWS, así que establezca `AWS_REGION` explícitamente.
* Cuando se usa Amazon Bedrock, el comando `/logout` no está disponible ya que la autenticación se maneja a través de credenciales de AWS.
* La herramienta WebSearch no está disponible en Amazon Bedrock. Vea [Comportamiento de la herramienta WebSearch](/docs/es/tools-reference#websearch-tool-behavior).
* Puede usar archivos de configuración para variables de entorno como `AWS_PROFILE` que no desea filtrar a otros procesos. Vea [Configuración](/docs/es/settings) para más información.

<h3 id="4-pin-model-versions">
  4. Fijar versiones de modelo
</h3>

<Warning>
  Fije versiones de modelo específicas al implementar para múltiples usuarios. Sin fijar, alias de modelo como `sonnet` y `opus` se resuelven al valor predeterminado integrado de Claude Code para Amazon Bedrock, que puede estar rezagado con respecto a la versión más reciente y puede que aún no esté disponible en su cuenta. Claude Code [retrocede](#startup-model-checks) a una versión anterior o modelo de nivel inferior al inicio cuando el valor predeterminado no está disponible, pero fijar le permite controlar cuándo sus usuarios se mueven a un nuevo modelo.
</Warning>

Establezca estas variables de entorno en IDs de modelo de Amazon Bedrock específicos.

Sin `ANTHROPIC_DEFAULT_OPUS_MODEL`, el alias `opus` en Amazon Bedrock se resuelve a Opus 4.8, y sin `ANTHROPIC_DEFAULT_SONNET_MODEL`, el alias `sonnet` se resuelve a Sonnet 4.5. Este ejemplo fija cada alias a una versión específica:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'
```

Estas variables utilizan IDs de perfil de inferencia entre regiones (con el prefijo `us.`). Si utiliza un prefijo de región diferente o perfiles de inferencia de aplicación, ajuste en consecuencia. En regiones de AWS GovCloud, utilice el prefijo `us-gov.`. Para IDs de modelo actuales y heredados, vea [Descripción general de modelos](https://platform.claude.com/docs/en/about-claude/models/overview). Vea [Configuración de modelo](/docs/es/model-config#pin-models-for-third-party-deployments) para la lista completa de variables de entorno.

Claude Code utiliza estos modelos predeterminados cuando no se establecen variables de fijación:

| Tipo de modelo        | Valor predeterminado                           |
| :-------------------- | :--------------------------------------------- |
| Modelo principal      | `us.anthropic.claude-opus-4-8`                 |
| Modelo pequeño/rápido | `us.anthropic.claude-sonnet-4-5-20250929-v1:0` |

Las tareas en segundo plano, como la generación de títulos de sesión, utilizan el modelo pequeño/rápido, normalmente un modelo de clase Haiku. En Amazon Bedrock, Claude Code utiliza el modelo Sonnet predeterminado para tareas en segundo plano porque Haiku puede no estar habilitado en todas las cuentas o regiones. Dos selecciones cambian qué modelo las realiza:

* Cuando selecciona un modelo principal con `--model`, `ANTHROPIC_MODEL`, o la configuración `model`, las tareas en segundo plano utilizan ese modelo. Establecer `ANTHROPIC_DEFAULT_OPUS_MODEL` sin `ANTHROPIC_DEFAULT_SONNET_MODEL` también cuenta como una selección, porque el modelo Sonnet integrado puede no estar habilitado en una cuenta que dirige su propio Opus.
* Para usar Haiku para tareas en segundo plano, establezca `ANTHROPIC_DEFAULT_HAIKU_MODEL` en un ID de modelo que esté disponible en su cuenta.

<Warning>
  Los modelos Opus tienen un precio por token más alto que los modelos Sonnet, por lo que una implementación que no fija un modelo principal se factura a la tasa de Opus una vez que se actualiza a v2.1.207 o posterior. Para mantener Sonnet 4.5 como el modelo principal, establezca `ANTHROPIC_MODEL` en su ID de modelo completo. Una implementación que dirige el valor predeterminado con `ANTHROPIC_DEFAULT_SONNET_MODEL` y no establece `ANTHROPIC_DEFAULT_OPUS_MODEL` mantiene su modelo Sonnet dirigido como el valor predeterminado.
</Warning>

Antes de v2.1.207, el modelo principal en Amazon Bedrock se establecía de forma predeterminada en Sonnet 4.5, el alias `opus` se resolvía a Opus 4.6, y las tareas en segundo plano siempre utilizaban el modelo principal.

Para personalizar modelos aún más, utilice uno de estos métodos:

```bash theme={null}
# Using inference profile ID
export ANTHROPIC_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'

# Using application inference profile ARN
export ANTHROPIC_MODEL='arn:aws:bedrock:us-east-2:your-account-id:application-inference-profile/your-model-id'

# Optional: Disable prompt caching if needed
export DISABLE_PROMPT_CACHING=1

# Optional: Request 1-hour prompt cache TTL instead of the 5-minute default
export ENABLE_PROMPT_CACHING_1H=1
```

La TTL de caché de 1 hora se factura a una tasa más alta que la predeterminada de 5 minutos. Vea [duración del caché](/docs/es/prompt-caching#cache-lifetime).

<Note>Prompt caching puede no estar disponible en todas las regiones de Amazon Bedrock. Si los recuentos de tokens de caché permanecen en cero, verifique [modelos, regiones y límites compatibles](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) en la documentación de Amazon Bedrock.</Note>

<h4 id="map-each-model-version-to-an-inference-profile">
  Asignar cada versión de modelo a un perfil de inferencia
</h4>

Las variables de entorno `ANTHROPIC_DEFAULT_*_MODEL` configuran un perfil de inferencia por familia de modelo. Si su organización necesita exponer varias versiones de la misma familia en el selector `/model`, cada una enrutada a su propio ARN de perfil de inferencia de aplicación, utilice la configuración `modelOverrides` en su [archivo de configuración](/docs/es/settings#settings-files) en su lugar.

Este ejemplo asigna cuatro versiones de Opus a ARN distintos para que los usuarios puedan cambiar entre ellas sin eludir los perfiles de inferencia de su organización:

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

Cuando un usuario selecciona una de estas versiones en `/model`, Claude Code llama a Amazon Bedrock con el ARN asignado. La misma asignación se aplica cuando pasa el ID de modelo de Anthropic directamente a través de `--model` o `ANTHROPIC_MODEL`. Las versiones sin una anulación se revierten al ID de modelo de Amazon Bedrock integrado o a cualquier perfil de inferencia coincidente descubierto al inicio. Antes de v2.1.200, los valores de `--model` y `ANTHROPIC_MODEL` llegaban a Amazon Bedrock tal cual sin pasar por el mapa de anulación. Vea [Anular IDs de modelo por versión](/docs/es/model-config#override-model-ids-per-version) para detalles sobre cómo las anulaciones interactúan con `availableModels` y otras configuraciones de modelo.

<h2 id="startup-model-checks">
  Verificaciones de modelo al inicio
</h2>

Cuando Claude Code se inicia con Amazon Bedrock configurado, verifica que los modelos que pretende usar sean accesibles en su cuenta.

Si ha fijado una versión de modelo que es más antigua que el valor predeterminado actual de Claude Code, y su cuenta puede invocar la versión más reciente, Claude Code le solicita que actualice la fijación. Aceptar escribe el nuevo ID de modelo en su [archivo de configuración de usuario](/docs/es/settings) y reinicia Claude Code. Rechazar se recuerda hasta el próximo cambio de versión predeterminada. Las fijaciones que apuntan a un [ARN de perfil de inferencia de aplicación](#map-each-model-version-to-an-inference-profile) se omiten, ya que son administradas por su administrador.

Si no ha fijado un modelo y el valor predeterminado actual no está disponible en su cuenta, Claude Code retrocede para la sesión actual y muestra un aviso. Intenta primero versiones anteriores del modelo predeterminado y, cuando el valor predeterminado es un modelo Opus y no hay ninguna versión de Opus disponible, retrocede al modelo Sonnet predeterminado. El retroceso no se persiste. Habilite el modelo más reciente en su cuenta de Amazon Bedrock o [fije una versión](#4-pin-model-versions) para hacer la opción permanente.

<h2 id="iam-configuration">
  Configuración de IAM
</h2>

Cree una política de IAM con los permisos requeridos para Claude Code:

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

Para permisos más restrictivos, puede limitar el Resource a ARN de perfil de inferencia específicos.

`bedrock:GetInferenceProfile` permite que Claude Code resuelva un [ARN de perfil de inferencia de aplicación](#map-each-model-version-to-an-inference-profile) a su modelo de fundación de respaldo, que se utiliza para seleccionar la forma de solicitud correcta para ese modelo.

Si el token carece de este permiso, Claude Code se recupera automáticamente reintentando una vez con la forma alternativa, por lo que las solicitudes aún tienen éxito pero cada nuevo modelo agrega un viaje de ida y vuelta adicional. Otorgar el permiso evita el reintento. Esto se aplica con mayor frecuencia a implementaciones de `AWS_BEARER_TOKEN_BEDROCK`, donde la política del token es típicamente más estrecha que un rol de IAM completo.

Para más detalles, vea [documentación de IAM de Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/security-iam.html).

<Note>
  Cree una cuenta de AWS dedicada para Claude Code para simplificar el seguimiento de costos y el control de acceso.
</Note>

<h2 id="1m-token-context-window">
  Ventana de contexto de 1M de tokens
</h2>

Claude Sonnet 5, Opus 4.6 y posteriores, y Sonnet 4.6 admiten la [ventana de contexto de 1M de tokens](https://platform.claude.com/docs/es/build-with-claude/context-windows#context-window-sizes-by-model) en Amazon Bedrock. Sonnet 5 se sirve a través del [punto de conexión Mantle](#use-the-mantle-endpoint) y siempre se ejecuta con la ventana de 1M, sin variante `[1m]` para seleccionar. Para los otros modelos, Claude Code habilita automáticamente la ventana de contexto extendida cuando selecciona una variante de modelo de 1M.

El [asistente de configuración](#sign-in-with-bedrock) ofrece una opción de contexto de 1M cuando fija modelos. Para habilitarlo para un modelo fijado manualmente en su lugar, agregue `[1m]` al ID del modelo. Vea [Fijar modelos para implementaciones de terceros](/docs/es/model-config#pin-models-for-third-party-deployments) para detalles.

<h2 id="service-tiers">
  Niveles de servicio
</h2>

[Los niveles de servicio de Amazon Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/service-tiers-inference.html) le permiten compensar el costo contra la latencia. Establezca `ANTHROPIC_BEDROCK_SERVICE_TIER` en `default`, `flex` o `priority`:

```bash theme={null}
export ANTHROPIC_BEDROCK_SERVICE_TIER=priority
```

Claude Code envía esto como el encabezado `X-Amzn-Bedrock-Service-Tier` en cada solicitud. La disponibilidad de niveles varía según el modelo y la región. La capacidad reservada utiliza un [ARN de rendimiento aprovisionado](https://docs.aws.amazon.com/bedrock/latest/userguide/prov-throughput.html) como el ID del modelo en lugar de esta configuración.

<h2 id="aws-guardrails">
  AWS Guardrails
</h2>

[Amazon Bedrock Guardrails](https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails.html) le permite implementar filtrado de contenido para Claude Code. Cree un Guardrail en la [consola de Amazon Bedrock](https://console.aws.amazon.com/bedrock/), publique una versión, luego agregue los encabezados de Guardrail a su [archivo de configuración](/docs/es/settings). Habilite la inferencia entre regiones en su Guardrail si está utilizando perfiles de inferencia entre regiones.

Configuración de ejemplo:

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Amzn-Bedrock-GuardrailIdentifier: your-guardrail-id\nX-Amzn-Bedrock-GuardrailVersion: 1"
  }
}
```

<h2 id="use-the-mantle-endpoint">
  Usar el punto final de Mantle
</h2>

Mantle es un punto final de Amazon Bedrock que sirve modelos de Claude a través de la forma de API nativa de Anthropic en lugar de la API de Invoke de Amazon Bedrock. Utiliza las mismas credenciales de AWS, permisos de IAM y configuración de `awsAuthRefresh` descritos anteriormente en esta página.

<h3 id="enable-mantle">
  Habilitar Mantle
</h3>

Con credenciales de AWS ya configuradas, establezca `CLAUDE_CODE_USE_MANTLE` para enrutar solicitudes al punto final de Mantle:

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export AWS_REGION=us-east-1
```

Claude Code construye la URL del punto final desde la región de AWS. A partir de v2.1.172, la región se resuelve con la misma precedencia que [Amazon Bedrock anterior](#3-configure-claude-code); las versiones anteriores utilizan solo `AWS_REGION`. Para anular la URL para un punto final personalizado o puerta de enlace, establezca `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`.

Ejecute `/status` dentro de Claude Code para confirmar. La línea del proveedor muestra `Amazon Bedrock (Mantle)` cuando Mantle está activo.

<h3 id="select-a-mantle-model">
  Seleccionar un modelo de Mantle
</h3>

Mantle utiliza IDs de modelo con prefijo `anthropic.` y sin sufijo de versión, por ejemplo `anthropic.claude-sonnet-5` o `anthropic.claude-haiku-4-5`. Los modelos disponibles para su cuenta dependen de lo que su organización haya sido autorizada; los IDs de modelo adicionales se enumeran en sus materiales de incorporación de AWS. Póngase en contacto con su equipo de cuenta de AWS para solicitar acceso a modelos permitidos.

Establezca el modelo con la bandera `--model` o con `/model` dentro de Claude Code:

```bash theme={null}
claude --model anthropic.claude-haiku-4-5
```

<h3 id="run-mantle-alongside-the-invoke-api">
  Ejecutar Mantle junto con la API de Invoke
</h3>

Los modelos disponibles para usted en Mantle pueden no incluir todos los modelos que usa hoy. Establecer tanto `CLAUDE_CODE_USE_BEDROCK` como `CLAUDE_CODE_USE_MANTLE` permite que Claude Code llame a ambos puntos finales desde la misma sesión. Los IDs de modelo que coinciden con el formato de Mantle se enrutan a Mantle, y todos los demás IDs de modelo van a la API de Invoke de Amazon Bedrock.

```bash theme={null}
export CLAUDE_CODE_USE_BEDROCK=1
export CLAUDE_CODE_USE_MANTLE=1
```

Para mostrar un modelo de Mantle en el selector `/model`, enumere su ID en `availableModels` en su [archivo de configuración](/docs/es/settings). Esta configuración también restringe el selector a las entradas enumeradas. Enumerar `anthropic.claude-haiku-4-5` elimina el alias `haiku` simple del selector, así que también enumere prefijos de versión o IDs completos para las versiones que desee mantener seleccionables. El ID de Mantle y el alias `haiku` se resuelven en la misma familia de modelos, por lo que la fusión mantiene solo la entrada más específica. Vea [Comportamiento de fusión](/docs/es/model-config#merge-behavior):

```json theme={null}
{
  "availableModels": ["opus", "sonnet", "claude-haiku-4-5", "anthropic.claude-haiku-4-5"]
}
```

Las entradas con el prefijo `anthropic.` se agregan como opciones de selector personalizado y se enrutan a Mantle. Reemplace `anthropic.claude-haiku-4-5` con el ID de modelo que su cuenta ha sido autorizada. Vea [Restringir selección de modelo](/docs/es/model-config#restrict-model-selection) para cómo `availableModels` interactúa con otras configuraciones de modelo.

Cuando ambos proveedores están activos, `/status` muestra `Amazon Bedrock + Amazon Bedrock (Mantle)`.

<h3 id="route-mantle-through-a-gateway">
  Enrutar Mantle a través de una puerta de enlace
</h3>

Si su organización enruta el tráfico de modelo a través de una [puerta de enlace LLM](/docs/es/llm-gateway) centralizada que inyecta credenciales de AWS del lado del servidor, deshabilite la autenticación del lado del cliente para que Claude Code envíe solicitudes sin firmas SigV4 o encabezados `x-api-key`:

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export CLAUDE_CODE_SKIP_MANTLE_AUTH=1
export ANTHROPIC_BEDROCK_MANTLE_BASE_URL=https://your-gateway.example.com
```

<h3 id="mantle-environment-variables">
  Variables de entorno de Mantle
</h3>

Estas variables son específicas del punto final de Mantle. Vea [Variables de entorno](/docs/es/env-vars) para la lista completa.

| Variable                                | Propósito                                                                             |
| :-------------------------------------- | :------------------------------------------------------------------------------------ |
| `CLAUDE_CODE_USE_MANTLE`                | Habilitar el punto final de Mantle. Establezca en `1` o `true`.                       |
| `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`     | Anular la URL del punto final de Mantle predeterminada                                |
| `CLAUDE_CODE_SKIP_MANTLE_AUTH`          | Omitir la autenticación del lado del cliente para configuraciones de proxy            |
| `ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION` | Anular la región de AWS para el modelo de clase Haiku (compartido con Amazon Bedrock) |

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="authentication-loop-with-sso-and-corporate-proxies">
  Bucle de autenticación con SSO y proxies corporativos
</h3>

Si las pestañas del navegador se abren repetidamente cuando se usa AWS SSO, elimine la configuración `awsAuthRefresh` de su [archivo de configuración](/docs/es/settings). Esto puede ocurrir cuando las VPN corporativas o los proxies de inspección TLS interrumpen el flujo del navegador SSO. Claude Code trata la conexión interrumpida como un error de autenticación, vuelve a ejecutar `awsAuthRefresh` y entra en un bucle indefinido.

Si su entorno de red interfiere con los flujos de SSO automáticos basados en navegador, use `aws sso login` manualmente antes de iniciar Claude Code en lugar de depender de `awsAuthRefresh`.

<h3 id="region-issues">
  Problemas de región
</h3>

Si encuentra problemas de región:

* Verifique la disponibilidad del modelo: `aws bedrock list-inference-profiles --region your-region`
* Cambie a una región compatible: `export AWS_REGION=us-east-1`
* Considere usar perfiles de inferencia para acceso entre regiones

Si recibe un error "on-demand throughput isn't supported":

* Especifique el modelo como un ID de [perfil de inferencia](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)

Claude Code utiliza la [API de Invoke](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html) de Amazon Bedrock y no admite la API de Converse.

<h3 id="streaming-errors-behind-a-gateway-or-proxy">
  Errores de streaming detrás de una puerta de enlace o proxy
</h3>

Si las solicitudes de streaming fallan con un error que comienza con `Bedrock streaming response has content-type`, una puerta de enlace o proxy entre Claude Code y Amazon Bedrock está transformando la respuesta de streaming. Amazon Bedrock transmite respuestas en un formato de evento binario event-stream con el tipo de contenido `application/vnd.amazon.eventstream`, y Claude Code rechaza una respuesta de streaming exitosa que reporta un tipo de contenido diferente en lugar de decodificar un cuerpo que no puede leer. El error nombra el tipo de contenido que recibió, comúnmente `text/event-stream` de una integración de Amazon API Gateway y Lambda que reemite el stream como eventos enviados por el servidor.

Antes de v2.1.208, la misma configuración incorrecta aparecía como `API Error: Truncated event message received` después de que toda la respuesta había sido almacenada en búfer.

Para solucionarlo, configure la puerta de enlace para pasar el cuerpo de la respuesta `InvokeModelWithResponseStream` y su encabezado `Content-Type` sin modificar. Si la puerta de enlace reescribe solo el encabezado y pasa el cuerpo binario intacto, establezca [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/es/env-vars) para omitir la verificación hasta que se corrija la puerta de enlace. Con la verificación desactivada, un cuerpo de respuesta que fue transformado falla con `Truncated event message received` nuevamente.

<h3 id="zero-token-counts-in-/context">
  Recuentos de tokens cero en /context
</h3>

El comando `/context` cuenta tokens para cada grupo de herramientas enviando los esquemas de herramientas a la API de conteo de tokens de Amazon Bedrock. En versiones de Claude Code anteriores a v2.1.196, Amazon Bedrock rechazó esa solicitud porque los esquemas llevaban campos que su API de conteo de tokens no acepta, por lo que cada grupo de herramientas mostraba 0 tokens. Otras filas en el desglose, como archivos de mensajes y memoria, no se ven afectadas.

Actualice a v2.1.196 o posterior.

<h3 id="mantle-endpoint-errors">
  Errores del punto final de Mantle
</h3>

Si `/status` no muestra `Amazon Bedrock (Mantle)` después de establecer `CLAUDE_CODE_USE_MANTLE`, la variable no está llegando al proceso. Confirme que se exporta en el shell donde lanzó `claude`, o establézcala en el bloque `env` de su [archivo de configuración](/docs/es/settings).

Un `403` del punto final de Mantle con credenciales válidas significa que su cuenta de AWS no ha sido autorizada para acceder al modelo que solicitó. Póngase en contacto con su equipo de cuenta de AWS para solicitar acceso.

Un `400` que nombra el ID del modelo significa que ese modelo no se sirve en Mantle. Mantle tiene su propio catálogo de modelos separado del catálogo estándar de Amazon Bedrock, por lo que los IDs de perfil de inferencia como `us.anthropic.claude-sonnet-4-6` no funcionarán. Utilice un ID de formato de Mantle, o habilite [ambos puntos finales](#run-mantle-alongside-the-invoke-api) para que Claude Code enrute cada solicitud al punto final donde el modelo está disponible.

<h2 id="additional-resources">
  Recursos adicionales
</h2>

* [Documentación de Amazon Bedrock](https://docs.aws.amazon.com/bedrock/)
* [Precios de Amazon Bedrock](https://aws.amazon.com/bedrock/pricing/)
* [Perfiles de inferencia de Amazon Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)
* [Burndown de tokens de Amazon Bedrock y cuotas](https://docs.aws.amazon.com/bedrock/latest/userguide/quotas-token-burndown.html)
* [Claude Code en Amazon Bedrock: Guía de configuración rápida](https://community.aws/content/2tXkZKrZzlrlu0KfH8gST5Dkppq/claude-code-on-amazon-bedrock-quick-setup-guide)
* [Implementación de monitoreo de Claude Code (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md)
