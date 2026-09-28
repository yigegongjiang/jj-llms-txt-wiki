> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Legal y cumplimiento

> Acuerdos legales, certificaciones de cumplimiento e información de seguridad para Claude Code.

<h2 id="legal-agreements">
  Acuerdos legales
</h2>

<h3 id="license">
  Licencia
</h3>

Su uso de Claude Code está sujeto a:

* [Términos comerciales](https://www.anthropic.com/legal/commercial-terms) - para usuarios de Team, Enterprise y Claude API
* [Términos de servicio del consumidor](https://www.anthropic.com/legal/consumer-terms) - para usuarios de Free, Pro y Max

<h3 id="commercial-agreements">
  Acuerdos comerciales
</h3>

Ya sea que esté utilizando la Claude API directamente (1P) o accediendo a través de Amazon Bedrock o Google Cloud's Agent Platform (3P), su acuerdo comercial existente se aplicará al uso de Claude Code, a menos que hayamos acordado mutuamente lo contrario.

<h3 id="can-customers-offer-claude-code-in-their-products">
  ¿Pueden los clientes ofrecer Claude Code en sus productos?
</h3>

A menos que hayamos acordado mutuamente lo contrario, preinstalar o ejecutar Claude Code en sus productos o servicios (por ejemplo, en sandboxes alojados u otra infraestructura de agentes) requiere aceptar nuestros [Términos comerciales](https://www.anthropic.com/legal/commercial-terms) y cumplir con las condiciones a continuación:

* **El binario de Claude Code no debe ser modificado.** Claude Code debe instalarse y ejecutarse tal como lo publica Anthropic, y los clientes no pueden eliminar, deshabilitar o restringir ningún método de autenticación integrado en él (incluidos los métodos que permiten iniciar sesión con una cuenta de Claude o la clave API propia del usuario).
* **Los clientes no pueden pagar, revender o intermediar el uso de Claude en nombre de sus usuarios finales.** Cada usuario final debe autenticarse con su propia clave API de Anthropic, credenciales del plan de suscripción de Claude, o credencial del proveedor de inferencia de terceros (Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry). Ese uso se factura directamente al usuario final bajo su propio acuerdo con Anthropic o, para proveedores de inferencia de terceros, con el proveedor aplicable.

**Uso del nombre y logotipo de Claude Code.** Puede decir con precisión, en texto sin formato, que su producto tiene Claude Code preinstalado o que ejecuta Claude Code. Pero no puede usar los nombres o logotipos de Claude Code o Anthropic como parte de su propio nombre de producto, característica o empresa, en su propio logotipo, o de una manera que sugiera que Anthropic construyó, respalda o se asoció con su producto. Cualquier otro uso de los nombres o logotipos de Anthropic se rige por nuestras [Directrices de marca registrada](https://www.anthropic.com/legal/trademark-guidelines) y requiere nuestro permiso escrito.

Claude Code sigue siendo regido por los términos estándar de Anthropic (consulte las secciones de Licencia y Acuerdos comerciales anteriores) independientemente de la plataforma a través de la cual se acceda.

<h2 id="compliance">
  Cumplimiento
</h2>

<h3 id="healthcare-compliance-baa">
  Cumplimiento de atención médica (BAA)
</h3>

Si un cliente ha ejecutado un Acuerdo de Asociado de Negocios (BAA) con Anthropic y tiene [Retención de datos cero (ZDR)](/docs/es/zero-data-retention) habilitada para la organización relevante, ese BAA se extiende al tráfico de API del cliente a través de Claude Code.

<h2 id="usage-policy">
  Política de uso
</h2>

<h3 id="acceptable-use">
  Uso aceptable
</h3>

El uso de Claude Code está sujeto a la [Política de uso de Anthropic](https://www.anthropic.com/legal/aup). Los límites de uso anunciados para los planes Pro y Max asumen el uso ordinario e individual de Claude Code y el Agent SDK.

<h3 id="authentication-and-credential-use">
  Autenticación y uso de credenciales
</h3>

Claude Code se autentica con los servidores de Anthropic utilizando tokens OAuth o claves API. Estos métodos de autenticación sirven para diferentes propósitos:

* **Autenticación OAuth** está destinada exclusivamente a los compradores de planes de suscripción Claude Free, Pro, Max, Team y Enterprise y está diseñada para apoyar el uso ordinario de Claude Code y otras aplicaciones nativas de Anthropic. Para los pasos de inicio de sesión, consulte [Iniciar sesión en su cuenta de Claude](https://support.claude.com/en/articles/13189465-logging-in-to-your-claude-account); para saber cómo Claude Code realiza la autenticación OAuth, consulte [Autenticación](/docs/es/authentication).
* **Los desarrolladores** que crean productos o servicios que interactúan con las capacidades de Claude, incluyendo aquellos que utilizan el [Agent SDK](/docs/es/agent-sdk/overview), deben utilizar autenticación de clave API a través de [Claude Console](https://platform.claude.com/) o un proveedor de nube compatible. Anthropic no permite que desarrolladores de terceros ofrezcan inicio de sesión de Claude.ai en sus propias aplicaciones, ni que enruten solicitudes a través de credenciales de planes Free, Pro o Max en nombre de sus usuarios. Además, los desarrolladores no pueden recopilar, almacenar o intermediar credenciales de Claude.ai o tokens de sesión — el inicio de sesión en una cuenta de Claude debe completarse a través del flujo propio de Anthropic.

Esto no restringe cómo los clientes aprovisionan y administran sus propias claves API o credenciales de proveedores de inferencia de terceros — por ejemplo, configurar una clave API en un entorno de desarrollo, gestor de secretos o imagen de máquina para su uso por los usuarios autorizados del cliente — siempre que el uso resultante se facture al propietario de la clave bajo su acuerdo con Anthropic (o el proveedor aplicable) y no se revenda o intermedie como se describe anteriormente. Tampoco impide que un usuario final inicie sesión en el binario de Claude Code sin modificar con su propia suscripción de Claude, incluyendo cuando una plataforma aloja Claude Code como se describe en *¿Pueden los clientes ofrecer Claude Code en sus productos?* anteriormente.

Anthropic se reserva el derecho de tomar medidas para hacer cumplir estas restricciones y puede hacerlo sin previo aviso.

Para preguntas sobre métodos de autenticación permitidos para su caso de uso, por favor [contacte con ventas](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=legal_compliance_contact_sales).

<h2 id="security-and-trust">
  Seguridad y confianza
</h2>

<h3 id="trust-and-safety">
  Confianza y seguridad
</h3>

Puede encontrar más información en el [Centro de confianza de Anthropic](https://trust.anthropic.com) y [Centro de transparencia](https://www.anthropic.com/transparency).

<h3 id="security-vulnerability-reporting">
  Reporte de vulnerabilidades de seguridad
</h3>

Anthropic gestiona nuestro programa de seguridad a través de HackerOne. [Utilice este formulario para reportar vulnerabilidades](https://hackerone.com/4f1f16ba-10d3-4d09-9ecc-c721aad90f24/embedded_submissions/new).

***

© Anthropic PBC. Todos los derechos reservados. El uso está sujeto a los Términos de servicio de Anthropic aplicables.
