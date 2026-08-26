> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code en Microsoft Foundry

> Aprende a configurar Claude Code a través de Microsoft Foundry, incluyendo configuración, instalación y solución de problemas.

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

<ContactSalesCard surface="foundry" />

<h2 id="prerequisites">
  Requisitos previos
</h2>

Antes de configurar Claude Code con Microsoft Foundry, asegúrese de que tiene:

* Una suscripción de Azure con acceso a Microsoft Foundry
* Permisos RBAC para crear recursos e implementaciones de Microsoft Foundry
* Azure CLI instalado y configurado (opcional - solo necesario si no tiene otro mecanismo para obtener credenciales)

<Note>
  Si está implementando Claude Code para múltiples usuarios, [fije las versiones de su modelo](#4-pin-model-versions) antes de implementar.
</Note>

<h2 id="setup">
  Configuración
</h2>

<h3 id="1-provision-microsoft-foundry-resource">
  1. Aprovisionar recurso de Microsoft Foundry
</h3>

Primero, cree un recurso de Claude en Azure:

1. Navegue al [portal de Microsoft Foundry](https://ai.azure.com/)
2. Cree un nuevo recurso, anotando el nombre de su recurso
3. Cree implementaciones para los modelos de Claude, anotando el nombre de implementación que asigne a cada uno; establecerá estos nombres como las variables de modelo en el paso 4:
   * Claude Opus
   * Claude Sonnet
   * Claude Haiku

<h3 id="2-configure-azure-credentials">
  2) Configurar credenciales de Azure
</h3>

Claude Code admite tres métodos de autenticación para Microsoft Foundry. Elija el método que mejor se ajuste a sus requisitos de seguridad.

**Opción A: Autenticación por clave API**

1. Navegue a su recurso en el portal de Microsoft Foundry
2. Vaya a la sección **Endpoints and keys**
3. Copie **API Key**
4. Establezca la variable de entorno, reemplazando `your-azure-api-key` con la clave que copió:

```bash theme={null}
export ANTHROPIC_FOUNDRY_API_KEY=your-azure-api-key
```

**Opción B: Autenticación de Microsoft Entra ID**

Cuando ni `ANTHROPIC_FOUNDRY_API_KEY` ni `ANTHROPIC_FOUNDRY_AUTH_TOKEN` están configurados, Claude Code utiliza automáticamente la [cadena de credenciales predeterminada](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview) del SDK de Azure.
Esto admite una variedad de métodos para autenticar cargas de trabajo locales y remotas.

En entornos locales, comúnmente puede usar Azure CLI:

```bash theme={null}
az login
```

**Opción C: Autenticación por token portador**

Claude Code envía el valor de `ANTHROPIC_FOUNDRY_AUTH_TOKEN` en cada solicitud como el encabezado `Authorization: Bearer`. Use esta opción cuando otro proceso, como una aplicación host o un script de inicio de sesión, ya haya obtenido un token de acceso para usted. Requiere Claude Code v2.1.203 o posterior.

Establezca la variable en un token portador que Microsoft Entra ID emitió para su recurso:

```bash theme={null}
export ANTHROPIC_FOUNDRY_AUTH_TOKEN=your-entra-access-token
```

`ANTHROPIC_FOUNDRY_AUTH_TOKEN` tiene prioridad sobre `ANTHROPIC_FOUNDRY_API_KEY` y sobre la cadena de credenciales predeterminada.

<Note>
  Cuando se usa Microsoft Foundry, el comando `/logout` no está disponible ya que la autenticación se maneja a través de credenciales de Azure.
</Note>

<h3 id="3-configure-claude-code">
  3. Configurar Claude Code
</h3>

Establezca las siguientes variables de entorno para habilitar Microsoft Foundry:

```bash theme={null}
# Enable Microsoft Foundry integration
export CLAUDE_CODE_USE_FOUNDRY=1

# Azure resource name (replace {resource} with your resource name)
export ANTHROPIC_FOUNDRY_RESOURCE={resource}
# Or provide the full base URL:
# export ANTHROPIC_FOUNDRY_BASE_URL=https://{resource}.services.ai.azure.com/anthropic
```

<h3 id="4-pin-model-versions">
  4. Fijar versiones de modelo
</h3>

<Warning>
  Fije versiones de modelo específicas para cada implementación. Sin fijar, los alias de modelo como `sonnet` y `opus` se resuelven al valor predeterminado integrado de Claude Code para Microsoft Foundry, que puede estar rezagado con respecto a la versión más reciente y es posible que aún no esté disponible en su cuenta. Microsoft Foundry no tiene verificación de modelo de inicio, por lo que las solicitudes fallan cuando el valor predeterminado no está disponible. Cuando cree implementaciones de Azure, seleccione una versión de modelo específica en lugar de "actualizar automáticamente a la última".
</Warning>

Establezca las variables de modelo para que coincidan con los nombres de implementación que creó en el paso 1.

Sin `ANTHROPIC_DEFAULT_OPUS_MODEL`, el alias `opus` en Microsoft Foundry se resuelve a Opus 4.6. Establézcalo en el ID de Opus 4.8 para usar el modelo más reciente:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5'
```

Las tareas en segundo plano, como la generación de títulos de sesión, utilizan el modelo pequeño/rápido, normalmente un modelo de clase Haiku. En Microsoft Foundry, Claude Code establece por defecto esto al modelo principal porque no todas las cuentas tienen una implementación de Haiku. Para usar Haiku para tareas en segundo plano, establezca `ANTHROPIC_DEFAULT_HAIKU_MODEL` en una implementación de Haiku que esté disponible en su cuenta, como se muestra arriba.

Para los ID de modelos actuales y heredados, consulte [Descripción general de modelos](https://platform.claude.com/docs/en/about-claude/models/overview). Consulte [Configuración de modelo](/docs/es/model-config#pin-models-for-third-party-deployments) para la lista completa de variables de entorno.

[Prompt caching](/docs/es/prompt-caching) está habilitado automáticamente. Para solicitar un TTL de caché de 1 hora en lugar del predeterminado de 5 minutos, establezca la siguiente variable; las escrituras de caché con un TTL de 1 hora se facturan a una tasa más alta:

```bash theme={null}
export ENABLE_PROMPT_CACHING_1H=1
```

<h3 id="5-run-claude-code">
  5. Ejecutar Claude Code
</h3>

Con las variables de entorno configuradas, inicie Claude Code desde su directorio de proyecto:

```bash theme={null}
claude
```

Claude Code lee `CLAUDE_CODE_USE_FOUNDRY` y las otras variables de Microsoft Foundry del entorno y se conecta a su recurso de Azure en el primer mensaje. A diferencia de Amazon Bedrock y Google Cloud's Agent Platform, Microsoft Foundry no tiene un asistente de configuración interactivo, por lo que las variables de entorno en los pasos 3 y 4 son la única ruta de configuración.

Para verificar su configuración, ejecute `/status` dentro de Claude Code. La línea del proveedor de API muestra `Microsoft Foundry`, junto con el nombre del recurso o la URL base que configuró.

<h2 id="azure-rbac-configuration">
  Configuración de RBAC de Azure
</h2>

Los roles predeterminados `Azure AI User` y `Cognitive Services User` incluyen todos los permisos necesarios para invocar modelos de Claude.

Para permisos más restrictivos, cree un rol personalizado con lo siguiente:

```json theme={null}
{
  "permissions": [
    {
      "dataActions": [
        "Microsoft.CognitiveServices/accounts/providers/*"
      ]
    }
  ]
}
```

Para más detalles, consulte la [documentación de RBAC de Microsoft Foundry](https://learn.microsoft.com/en-us/azure/ai-foundry/concepts/rbac-azure-ai-foundry).

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Si recibe un error "Failed to get token from azureADTokenProvider: ChainedTokenCredential authentication failed":

* Configure Entra ID en el entorno, o establezca `ANTHROPIC_FOUNDRY_API_KEY`.

Si las solicitudes fallan con errores de conexión repetidos en el primer mensaje:

* Verifique que `ANTHROPIC_FOUNDRY_RESOURCE` esté configurado con el nombre de recurso real en lugar de un marcador de posición. Claude Code construye la URL del punto de conexión a partir de este valor, por lo que un nombre incorrecto apunta a un host que no existe.

<h2 id="additional-resources">
  Recursos adicionales
</h2>

* [Documentación de Microsoft Foundry](https://learn.microsoft.com/en-us/azure/ai-foundry/what-is-azure-ai-foundry)
* [Modelos de Microsoft Foundry](https://ai.azure.com/explore/models)
* [Precios de Microsoft Foundry](https://azure.microsoft.com/en-us/pricing/details/ai-foundry/)
