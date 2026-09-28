> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Descripción general de implementación empresarial

> Aprenda cómo Claude Code puede integrarse con varios servicios de terceros e infraestructura para cumplir con los requisitos de implementación empresarial.

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

Las organizaciones pueden implementar Claude Code directamente a través de Anthropic o a través de un proveedor de nube. Esta página le ayuda a elegir la configuración correcta.

<ContactSalesCard surface="third_party_overview" />

<h2 id="compare-deployment-options">
  Comparar opciones de implementación
</h2>

Para la mayoría de las organizaciones, Claude for Teams o Claude for Enterprise proporciona la mejor experiencia. Los miembros del equipo obtienen acceso tanto a Claude Code como a Claude en la web con una única suscripción, facturación centralizada y sin necesidad de configuración de infraestructura.

**Claude for Teams** es de autoservicio e incluye características de colaboración, herramientas de administración, SSO, gestión de facturación y [configuración administrada por servidor](/docs/es/server-managed-settings) para la configuración de Claude Code en toda la organización. Mejor para equipos más pequeños que necesitan comenzar rápidamente.

**Claude for Enterprise** añade captura de dominio, permisos basados en roles y acceso a API de cumplimiento. Mejor para organizaciones más grandes con requisitos de seguridad y cumplimiento.

Obtenga más información sobre [planes de equipo](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) y [planes empresariales](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan).

Las opciones de implementación comparadas a continuación cubren dónde se ejecuta la inferencia del modelo. Para ejecutar [sesiones en la nube](/docs/es/claude-code-on-the-web) de Claude Code en computación que su organización opera, consulte [entornos autohospedados](/docs/es/self-hosted-environments).

Si su organización tiene requisitos de infraestructura específicos, compare las opciones a continuación:

<table>
  <thead>
    <tr>
      <th>Característica</th>
      <th>Claude for Teams/Enterprise</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform, formerly Vertex AI</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>Mejor para</td>
      <td>La mayoría de las organizaciones (recomendado)</td>
      <td>Desarrolladores individuales</td>
      <td>Implementaciones nativas de AWS</td>
      <td>Facturación de AWS Marketplace con características de API de Claude</td>
      <td>Implementaciones nativas de GCP</td>
      <td>Implementaciones nativas de Azure</td>
    </tr>

    <tr>
      <td>Facturación</td>
      <td><strong>Teams:</strong> \$150/puesto (Premium) con PAYG disponible<br /><strong>Enterprise:</strong> <a href="https://claude.com/contact-sales?utm_source=claude_code&utm_medium=docs&utm_content=third_party_enterprise">Contactar ventas</a></td>
      <td>PAYG</td>
      <td>PAYG a través de AWS</td>
      <td>PAYG a través de AWS Marketplace</td>
      <td>PAYG a través de GCP</td>
      <td>PAYG a través de Azure</td>
    </tr>

    <tr>
      <td>Regiones</td>
      <td>[Países](https://www.anthropic.com/supported-countries) admitidos</td>
      <td>[Países](https://www.anthropic.com/supported-countries) admitidos</td>
      <td>Múltiples [regiones](https://docs.aws.amazon.com/bedrock/latest/userguide/models-regions.html) de AWS</td>
      <td>Múltiples regiones de AWS</td>
      <td>Múltiples [regiones](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations) de GCP</td>
      <td>Múltiples [regiones](https://azure.microsoft.com/en-us/explore/global-infrastructure/products-by-region/) de Azure</td>
    </tr>

    <tr>
      <td>Prompt caching</td>
      <td>Habilitado de forma predeterminada</td>
      <td>Habilitado de forma predeterminada</td>
      <td>Habilitado de forma predeterminada</td>
      <td>Habilitado de forma predeterminada</td>
      <td>Habilitado de forma predeterminada</td>
      <td>Habilitado de forma predeterminada</td>
    </tr>

    <tr>
      <td>Autenticación</td>
      <td>SSO de claude.ai o correo electrónico</td>
      <td>Clave API o [inicio de sesión en Console sin una](/docs/es/authentication#sign-in-without-an-api-key)</td>
      <td>Clave API o credenciales de AWS</td>
      <td>Clave API o credenciales de AWS</td>
      <td>Credenciales de GCP</td>
      <td>Clave API o Microsoft Entra ID</td>
    </tr>

    <tr>
      <td>Seguimiento de costos</td>
      <td>Panel de uso</td>
      <td>Panel de uso</td>
      <td>AWS Cost Explorer</td>
      <td>AWS Cost Explorer</td>
      <td>Facturación de GCP</td>
      <td>Gestión de costos de Azure</td>
    </tr>

    <tr>
      <td>Incluye Claude en la web</td>
      <td>Sí</td>
      <td>No</td>
      <td>No</td>
      <td>No</td>
      <td>No</td>
      <td>No</td>
    </tr>

    <tr>
      <td>Características empresariales</td>
      <td>Gestión de equipos, SSO, monitoreo de uso</td>
      <td>Ninguno</td>
      <td>Políticas de IAM, CloudTrail</td>
      <td>Políticas de IAM, CloudTrail</td>
      <td>Roles de IAM, registros de auditoría en la nube</td>
      <td>Políticas RBAC, Azure Monitor</td>
    </tr>
  </tbody>
</table>

Para obtener un desglose característica por característica de lo que está disponible en cada opción, consulte [Disponibilidad de características](/docs/es/feature-availability).

Seleccione una opción de implementación para ver las instrucciones de configuración:

* [Claude for Teams o Enterprise](/docs/es/authentication#claude-for-teams-or-enterprise)
* [Anthropic Console](/docs/es/authentication#claude-console-authentication)
* [Claude apps gateway](/docs/es/claude-apps-gateway), una puerta de enlace autohospedada que añade inicio de sesión de IdP frente a Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, Microsoft Foundry o la API de Anthropic
* [Amazon Bedrock](/docs/es/amazon-bedrock)
* [Claude Platform on AWS](/docs/es/claude-platform-on-aws)
* [Google Cloud's Agent Platform](/docs/es/google-vertex-ai)
* [Microsoft Foundry](/docs/es/microsoft-foundry)

Para Amazon Bedrock y Google Vertex AI, también puede ejecutar `claude` y seleccionar **plataforma de terceros** en el mensaje de inicio de sesión para iniciar un asistente de configuración interactivo.

<h2 id="configure-proxies-and-gateways">
  Configurar proxies y gateways
</h2>

La mayoría de las organizaciones pueden usar un proveedor de nube directamente sin configuración adicional. Sin embargo, es posible que deba configurar un proxy corporativo o una puerta de enlace LLM si su organización tiene requisitos específicos de red o gestión. Estas son configuraciones diferentes que se pueden usar juntas:

* **Proxy corporativo**: Enruta el tráfico a través de un proxy HTTP/HTTPS. Úselo si su organización requiere que todo el tráfico saliente pase a través de un servidor proxy para monitoreo de seguridad, cumplimiento o aplicación de políticas de red. Configure con las variables de entorno `HTTPS_PROXY` o `HTTP_PROXY`. Obtenga más información en [Configuración de red empresarial](/docs/es/network-config).
* **LLM Gateway**: Un servicio que se sitúa entre Claude Code y el proveedor de nube para manejar la autenticación y el enrutamiento. Úselo si necesita seguimiento de uso centralizado entre equipos, limitación de velocidad personalizada o presupuestos, o gestión de autenticación centralizada. Configure con las variables de entorno `ANTHROPIC_BASE_URL`, `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_AWS_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, o `ANTHROPIC_FOUNDRY_BASE_URL`. Obtenga más información en [Puertas de enlace LLM](/docs/es/llm-gateway).

Para las variables de entorno por proveedor que enrutan Amazon Bedrock, Microsoft Foundry o la Plataforma de agentes de Google Cloud a través de una puerta de enlace LLM, consulte [enrutar a un proveedor de nube a través de una puerta de enlace](/docs/es/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway). Ejecute `/status` en Claude Code para verificar qué proveedor, URL base y proxy está usando una sesión.

Si su organización usa [claves de cifrado administradas por el cliente](https://platform.claude.com/docs/en/manage-claude/cmek) (CMEK) y enruta Claude Code a través de una puerta de enlace LLM o una `ANTHROPIC_BASE_URL` personalizada, CMEK no se aplica a la telemetría operativa de Claude Code en esas sesiones. Para desactivar la telemetría para cada desarrollador, entregue `DISABLE_TELEMETRY` a través de la configuración administrada como se muestra en [Desactivar la telemetría para su organización](/docs/es/managed-settings#turn-telemetry-off-for-your-organization).

<h2 id="best-practices-for-organizations">
  Mejores prácticas para organizaciones
</h2>

<h3 id="invest-in-documentation-and-memory">
  Invertir en documentación y memoria
</h3>

Le recomendamos encarecidamente que invierta en documentación para que Claude Code comprenda su base de código. Las organizaciones pueden implementar archivos CLAUDE.md en múltiples niveles. Consulte [dónde pueden vivir los archivos CLAUDE.md](/docs/es/memory#choose-where-to-put-claude-md-files) y [cómo implementar un CLAUDE.md en toda la organización](/docs/es/memory#deploy-organization-wide-claude-md).

<h3 id="simplify-deployment">
  Simplificar la implementación
</h3>

Si tiene un entorno de desarrollo personalizado, encontramos que crear una forma de "un clic" para instalar Claude Code es clave para aumentar la adopción en toda una organización.

<h3 id="start-with-guided-usage">
  Comenzar con uso guiado
</h3>

Anime a los nuevos usuarios a probar Claude Code para preguntas sobre la base de código, o en correcciones de errores más pequeñas o solicitudes de características. Pida a Claude Code que haga un plan. Verifique las sugerencias de Claude y proporcione comentarios si se desvía. Con el tiempo, a medida que los usuarios comprendan mejor este nuevo paradigma, serán más efectivos permitiendo que Claude Code se ejecute de manera más agencial.

<h3 id="pin-model-versions-for-cloud-providers">
  Fijar versiones de modelo para proveedores de nube
</h3>

Si implementa a través de [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai), [Microsoft Foundry](/docs/es/microsoft-foundry), o [Claude Platform on AWS](/docs/es/claude-platform-on-aws), fije versiones de modelo específicas usando `ANTHROPIC_DEFAULT_FABLE_MODEL`, `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL`, y `ANTHROPIC_DEFAULT_HAIKU_MODEL`. Sin fijar, los alias de modelo se resuelven al valor predeterminado integrado de Claude Code para ese proveedor, que puede rezagarse con respecto a la versión más reciente y es posible que aún no esté habilitado en su cuenta. Fijar le permite controlar cuándo sus usuarios se mueven a un nuevo modelo. Consulte [Configuración de modelo](/docs/es/model-config#pin-models-for-third-party-deployments) para ver qué hace cada proveedor cuando el valor predeterminado no está disponible.

<h3 id="configure-security-policies">
  Configurar políticas de seguridad
</h3>

Los equipos de seguridad pueden configurar permisos administrados para lo que Claude Code puede y no puede hacer, que no pueden ser sobrescritos por la configuración local. [Obtenga más información](/docs/es/security).

<h3 id="leverage-mcp-for-integrations">
  Usar MCP para integraciones
</h3>

MCP es una excelente manera de dar a Claude Code más información, como conectarse a sistemas de gestión de tickets o registros de errores. Recomendamos que un equipo central configure servidores MCP y verifique una configuración `.mcp.json` en la base de código para que todos los usuarios se beneficien. [Obtenga más información](/docs/es/mcp).

<h2 id="next-steps">
  Próximos pasos
</h2>

Una vez que haya elegido una opción de implementación y configurado el acceso para su equipo:

1. **Implementar en su equipo**: Comparta instrucciones de instalación y haga que los miembros del equipo [instalen Claude Code](/docs/es/setup) y se autentiquen con sus credenciales.
2. **Configurar configuración compartida**: Cree un [archivo CLAUDE.md](/docs/es/memory) en sus repositorios para ayudar a Claude Code a comprender su base de código y estándares de codificación.
3. **Configurar permisos**: Revise [configuración de seguridad](/docs/es/security) para definir qué Claude Code puede y no puede hacer en su entorno.
